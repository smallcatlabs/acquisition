//! The persisted job queue and its admission: the writes to `daemon.db`,
//! the restore, submit, cancel and priority.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! # Decisions as recorded
//!
//! ## C6 — The job queue persists: a `jobs` table in a per-daemon `daemon.db` (SQLite, in the prov…
//!
//! **The job queue persists: a `jobs` table in a per-daemon `daemon.db` (SQLite, in the provider's store directory beside the account files), written through at every state change and read back at start.** Memory stays the runtime source of truth — the table is a mirror, "the HashMap, but it survives" — and the one thing read from it while the daemon runs is `result` for an id this lifetime never held (history, not state). On restore only open jobs are loaded (terminal rows carry bodies; a week of them does not belong in memory): `waiting` jobs resume; a `running` job is **re-queued** where the replay premise holds — every network kind is an idempotent GET and the restart probe reads GGG's current counters before it sends, so the duplicate costs one seen hit. Two exceptions (2026-08-30 review): on a declared **no-probe route** the premise fails — a replay would go out against an empty limiter — so the job fails as interrupted instead; and a running **parent whose children exist is mid-fan-out** (its held result was not yet written) — re-running it would submit a duplicate child set, so it holds for the children it has and then finishes as **interrupted, never success**: how many children were never submitted is unknowable, so a partial fan-out must not claim completeness (the children that did run recorded their responses; resubmitting completes the set). A parent whose held result was written resumes holding it; `probe` rows are dropped (one HEAD per lifetime, N16); ids continue from where they were (`AUTOINCREMENT`, never reused, so a stale `acq result <id>` can never name a different job) — a daemon that cannot open or read `daemon.db` refuses to start rather than risk reissuing them, and a queue **write** failure at runtime is sticky: a submit whose insert fails is refused with its id rolled back (a job exists only once its row does), later submits are refused outright, and the dispatcher stops picking while running jobs finish — **ids** never run ahead of disk. Completions are the accepted residual, stated plainly: a job already running when the flag trips finishes in memory but its outcome write fails, so disk still says running and the next daemon replays it (probed route: one seen duplicate hit; no-probe: fails as interrupted) — the send already happened, so refusing to finish it would record nothing at all. The same teeth apply per transition: a `waiting→running` write that fails reverts the job instead of running it (a send the queue cannot see must not happen), `cancel`/`set-priority` report a failed write instead of claiming success (the cancel still wins the job's terminal surface in this lifetime, though an already-running job may still complete its in-flight send — sends are committed once dispatched), and a `result` read failure is an error, never "no job". `submit_child` is refused once its parent is terminal or asked to cancel, under the lock `cancel` takes, so cancellation cannot race an active fan-out into submitting unseen children; a stopped fan-out finishes cancelled or failed, never as success over a partial set; a cancellation that lands after the last child is honored when the held result is installed, cancelled children never count toward a parent's success, and `finish` arbitrates a pending `cancel_requested` under the final lock — a cancel can land at any instant before terminalization and still win. Terminal rows stay so `acq result` has a memory across restarts (`acq jobs` lists live jobs only), pruned at start by age — `ACQ_JOB_RETENTION_DAYS` (default 7) for done/cancelled and `ACQ_FAILED_JOB_RETENTION_DAYS` (default 30) for failed, misread values logged as `CONFIG` errors like the rails knobs. Outcomes are stored verbatim, bodies included (a full refresh is ~50 MB, bounded by retention); compression was considered and deferred — it costs a crate and makes the column unreadable in `sqlite3`, and compressing one column later is a local change. **One daemon per store directory is the world lock (C83)**: since the daemon split's step 5 the daemon takes an exclusive lock on its world's root (`<root>/daemon.lock`, `acquisition_store::world::Lock`) before `daemon.db` opens and refuses to start naming the holder; two daemons on one `daemon.db` would each restore and run the same queue. Parallel mock daemons are parallel worlds (`ACQ_STORE_DIR=<scratch>`, whose socket derives from it; the mock-session skill). Rationale: the queue was the one thing a restart lost once results moved to the store (2026-08-29); a mirror written under the same lock as the memory change keeps disk equal to memory at the `process::exit` the daemon leaves by (up to the declared write-failure residual above); SQLite because it is the crate's one persistence idiom, debuggable with `sqlite3`, and readable by a frontend without a daemon. Decided 2026-08-30.
//!

use super::*;

impl Daemon {
    /// Mirror one job to `daemon.db`. Called with the `Shared` lock held,
    /// so the table sees changes in memory's order. A failed write trips
    /// the sticky queue failure (logged here — safe, `log` takes no lock
    /// but its own — and refused loudly at the next submit): disk has
    /// diverged from memory, so no new work may be taken.
    pub(super) fn persist(&self, entry: &Entry) -> bool {
        match self
            .jobs_db
            .lock()
            .unwrap()
            .upsert(&entry.row(self.unix_now()))
        {
            Ok(()) => true,
            Err(e) => {
                let mut failure = self.queue_failure.lock().unwrap();
                if failure.is_none() {
                    *failure = Some(format!("{e:#}"));
                    self.log(&format!(
                        "JOBS QUEUE FAILED persisting job {}: {e:#}; refusing new jobs until restart",
                        entry.info.id
                    ));
                }
                false
            }
        }
    }

    /// The sticky queue-write failure, if one has happened.
    pub(super) fn queue_failed(&self) -> Option<String> {
        self.queue_failure.lock().unwrap().clone()
    }

    /// Take the previous lifetime's open jobs from `daemon.db` (C6):
    /// waiting jobs resume; a running one is re-queued only where the
    /// replay premise holds — a probe will read GGG's current counters
    /// before the send. Two exceptions: a running job on a declared
    /// no-probe route would send blind, so it fails as interrupted
    /// instead; and a running parent whose children exist is mid-fan-out
    /// (its held result was not yet written) — re-running it would submit
    /// a duplicate child set, so it
    /// is held for its existing children with a synthetic result. A parent
    /// whose held result was written keeps holding it; probes are per
    /// lifetime and dropped. An open parent's finished children come back
    /// with it (its summary counts them). Ids continue from the table's
    /// sequence. Other terminal rows stay in the table for `result`,
    /// pruned by age first.
    /// A read failure is fatal to startup, like an unopenable file: a
    /// daemon that cannot see the previous lifetime's rows would reissue
    /// ids from 1 against them.
    pub(super) fn restore_jobs(&self, retention: Retention) -> Result<()> {
        let db = &self.jobs_db;
        let (rows, next_id, pruned) = {
            let db = db.lock().unwrap();
            let pruned = db.prune(retention, self.unix_now()).unwrap_or_else(|e| {
                self.log(&format!("JOBS: prune failed: {e:#}"));
                0
            });
            match (db.load_open(), db.next_id()) {
                (Ok(rows), Ok(next)) => (rows, next, pruned),
                (Err(e), _) | (_, Err(e)) => {
                    anyhow::bail!("could not read {}: {e:#}", db.path().display());
                }
            }
        };
        let entries: Vec<Entry> = rows.into_iter().filter_map(Entry::from_row).collect();
        let has_children: HashSet<JobId> = entries.iter().filter_map(|e| e.info.parent).collect();
        let (mut requeued, mut held, mut not_replayed, mut dropped) = (0, 0, 0, 0);
        let mut finish_parents = Vec::new();
        let mut restored = Vec::new();
        {
            let mut s = self.shared.lock().unwrap();
            s.next_id = next_id;
            for mut entry in entries {
                if entry.info.kind == "probe" {
                    dropped += 1;
                    let _ = db.lock().unwrap().delete(entry.info.id);
                    continue;
                }
                if entry.info.state == JobState::Running {
                    let no_probe_route = self
                        .keyed_route_for(
                            &entry.info.kind,
                            &entry.params,
                            entry.info.account.as_deref(),
                        )
                        .is_some_and(|(route, _)| !Self::route_probes(&route));
                    if entry.cancel_requested {
                        entry.info.state = JobState::Cancelled;
                        entry.outcome = Some(Outcome::Cancelled);
                    } else if entry.deferred.is_some() {
                        // Holding its result for children: no task to
                        // give it; it finishes when they do.
                        finish_parents.push(entry.info.id);
                    } else if has_children.contains(&entry.info.id) {
                        // Mid-fan-out: children were submitted but the
                        // held result was not yet written. Re-running the
                        // parent would submit them all again, so it holds
                        // for the set it has — but how many children never
                        // got submitted is unknowable, so once they land
                        // it finishes as interrupted, never as success.
                        entry.deferred = Some(Outcome::failure(
                            "interrupted by a daemon restart mid fan-out: the children \
                                    submitted before the restart ran (their responses are in \
                                    the store), but the full child set is unknown — resubmit \
                                    to complete it",
                        ));
                        held += 1;
                        finish_parents.push(entry.info.id);
                    } else if no_probe_route {
                        // The replay premise — a probe reads the previous
                        // lifetime's hits before anything sends — does not
                        // hold on a no-probe route; a replay would go out
                        // against an empty limiter.
                        not_replayed += 1;
                        entry.info.state = JobState::Failed;
                        entry.outcome = Some(Outcome::failure(format!(
                            "interrupted by a daemon restart and not replayed: \
{} sends without a probe, so a replay could not learn the previous lifetime's hits first; \
resubmit if still wanted",
                            entry.info.kind
                        )));
                    } else {
                        entry.info.state = JobState::Waiting;
                        requeued += 1;
                    }
                    self.persist(&entry);
                }
                restored.push(entry.info.clone());
                s.jobs.insert(entry.info.id, entry);
            }
        }
        if !restored.is_empty() || dropped > 0 {
            self.log(&format!(
                "JOBS: restored {} from {} ({requeued} re-queued from running, {held} parents held mid-fan-out, {not_replayed} not replayed on no-probe routes, {dropped} probes dropped, {pruned} old rows pruned); next id {next_id}",
                restored.len(),
                db.lock().unwrap().path().display()
            ));
        }
        for info in restored {
            self.emit(info);
        }
        // A parent whose last child finished in the instant before the
        // previous daemon died is finished now.
        for pid in finish_parents {
            self.maybe_finish_parent(pid);
        }
        self.work.notify_one();
        Ok(())
    }

    /// The previous lifetime's result for a job this one never held.
    /// `Ok(None)` is genuinely no such job; a queue that cannot be read
    /// is an error, never mistaken for "no job".
    pub(super) fn stored_outcome(&self, id: JobId) -> Result<Option<Outcome>, Refusal> {
        match self.jobs_db.lock().unwrap().get(id) {
            Ok(row) => Ok(row.and_then(Entry::from_row).and_then(|e| e.outcome)),
            Err(e) => Err(Refusal::new(
                ErrorKind::QueueFailed,
                format!("could not read the persisted queue: {e:#}"),
            )),
        }
    }

    pub(super) fn submit(
        &self,
        kind: String,
        params: Value,
        priority: Priority,
        submitted_by: String,
        account: Option<String>,
    ) -> Result<JobId, Refusal> {
        // An `apply` is admitted or refused whole, before a job id exists
        // (C43): vocabulary and budget checked
        // here, so a refusal admits nothing. A realm a kind's family does
        // not take is refused the same way, before an id exists.
        if kind == "apply" {
            validate_apply(&params)?;
        }
        admit_realm(&kind, &params)?;
        admit_params_size(&params)?;
        admit_submitter(&submitted_by)?;
        self.submit_with_parent(kind, params, priority, submitted_by, account, None)
    }

    /// A child inherits its parent's priority, submitter, and account.
    /// The cancellation guard lives in `submit_with_parent`, inside the
    /// same critical section as the insert.
    pub(super) fn submit_child(
        &self,
        parent: JobId,
        kind: &str,
        params: Value,
    ) -> Result<JobId, Refusal> {
        let (priority, by, account) = {
            let s = self.shared.lock().unwrap();
            let p = s.jobs.get(&parent).ok_or_else(|| parent_gone(parent))?;
            (
                p.info.priority,
                p.info.submitted_by.clone(),
                p.info.account.clone(),
            )
        };
        self.submit_with_parent(kind.into(), params, priority, by, account, Some(parent))
    }

    /// A job exists only once its row does: an insert that fails rolls the
    /// id back and refuses the submit, so memory can never run ahead of
    /// the ids disk has seen.
    pub(super) fn submit_with_parent(
        &self,
        kind: String,
        params: Value,
        priority: Priority,
        submitted_by: String,
        account: Option<String>,
        parent: Option<JobId>,
    ) -> Result<JobId, Refusal> {
        if let Some(e) = self.queue_failed() {
            return Err(Refusal::queue_failed(&e));
        }
        let now = self.choke.now();
        let info = {
            let mut s = self.shared.lock().unwrap();
            // The parent guard sits here, inside the same critical section
            // as the insert: `cancel` takes this lock to enumerate
            // children, so a child can never be inserted after a
            // cancellation has swept and missed it.
            if let Some(pid) = parent {
                let p = s.jobs.get(&pid).ok_or_else(|| parent_gone(pid))?;
                if p.info.state.is_terminal() || p.cancel_requested {
                    return Err(Refusal::new(
                        ErrorKind::Refused,
                        format!("parent job {pid} was cancelled"),
                    ));
                }
            }
            s.last_activity = now;
            let id = s.next_id;
            s.next_id += 1;
            let info = JobInfo {
                id,
                kind,
                state: JobState::Waiting,
                priority,
                submitted_by,
                eta_seconds: None,
                parent,
                retries: 0,
                account,
                params: params.clone(),
            };
            let entry = Entry {
                info: info.clone(),
                params,
                outcome: None,
                cancel_requested: false,
                deferred: None,
                submitted_at: self.unix_now(),
            };
            if !self.persist(&entry) {
                s.next_id -= 1;
                let e = self.queue_failed().unwrap_or_default();
                return Err(Refusal::queue_failed(&e));
            }
            s.jobs.insert(id, entry);
            info
        };
        let id = info.id;
        self.emit(info);
        self.work.notify_one();
        Ok(id)
    }

    pub(super) fn cancel(&self, id: JobId) -> Result<(), Refusal> {
        // Cancelling a parent cancels everything under it: waiting
        // descendants immediately, running ones at their next slice.
        let mut emits = Vec::new();
        let mut persist_failed = false;
        {
            let mut s = self.shared.lock().unwrap();
            let entry = s.jobs.get(&id).ok_or_else(|| Refusal::no_job(id))?;
            if entry.info.state.is_terminal() {
                return Err(Refusal::new(
                    ErrorKind::WrongState,
                    format!("job {id} already {}", entry.info.state),
                ));
            }
            let mut targets = vec![id];
            let mut i = 0;
            while i < targets.len() {
                let p = targets[i];
                targets.extend(
                    s.jobs
                        .values()
                        .filter(|e| e.info.parent == Some(p))
                        .map(|e| e.info.id),
                );
                i += 1;
            }
            for t in targets {
                let entry = s.jobs.get_mut(&t).unwrap();
                match entry.info.state {
                    JobState::Waiting => {
                        entry.info.state = JobState::Cancelled;
                        entry.outcome = Some(Outcome::Cancelled);
                        emits.push(entry.info.clone());
                    }
                    // A parent waiting on children isn't on any worker;
                    // cancel it outright.
                    JobState::Running if entry.deferred.is_some() => {
                        entry.deferred = None;
                        entry.info.state = JobState::Cancelled;
                        entry.outcome = Some(Outcome::Cancelled);
                        emits.push(entry.info.clone());
                    }
                    JobState::Running => entry.cancel_requested = true,
                    _ => continue,
                }
                persist_failed |= !self.persist(entry);
            }
        }
        // A cancelled child may have been the last thing its parent was
        // holding for; without this the parent would wait forever.
        let parents: HashSet<JobId> = emits
            .iter()
            .filter(|i| i.state.is_terminal())
            .filter_map(|i| i.parent)
            .collect();
        for info in emits {
            self.emit(info);
        }
        for pid in parents {
            self.maybe_finish_parent(pid);
        }
        if persist_failed {
            // The cancellation holds for this lifetime — nothing here will
            // send — but disk may still say waiting/running, so a restart
            // can revive the job. Say so instead of claiming success.
            return Err(Refusal::new(
                ErrorKind::QueueFailed,
                "cancelled for this daemon's lifetime, but the queue write failed — the \
                 cancellation may not survive a restart; the daemon refuses new jobs until then",
            ));
        }
        Ok(())
    }

    pub(super) fn set_priority(&self, id: JobId, priority: Priority) -> Result<(), Refusal> {
        let mut s = self.shared.lock().unwrap();
        let entry = s.jobs.get_mut(&id).ok_or_else(|| Refusal::no_job(id))?;
        if entry.info.state != JobState::Waiting {
            return Err(Refusal::new(
                ErrorKind::WrongState,
                format!("job {id} is {}, not waiting", entry.info.state),
            ));
        }
        let before = entry.info.priority;
        entry.info.priority = priority;
        if !self.persist(entry) {
            entry.info.priority = before;
            return Err(Refusal::new(
                ErrorKind::QueueFailed,
                "priority unchanged: the queue write failed; the daemon refuses new work until restart",
            ));
        }
        Ok(())
    }

    pub(super) fn cancelled(&self, id: JobId) -> bool {
        let s = self.shared.lock().unwrap();
        s.jobs.get(&id).map(|e| e.cancel_requested).unwrap_or(true)
    }
}

/// Realm admission: a kind in a realm family (`acquisition_protocol::realm`) must name a
/// realm that family takes — or none, meaning pc. Runs at submit for
/// every kind and per tuple inside `validate_apply`, so a job that would
/// render a stash URL under `poe2` never gets an id (C59: no code path
/// renders an unobserved URL shape). Kinds outside the families ignore
/// the param. A realm that is not a string of at most `PREVIEW_BYTES` is
/// refused here first, by `describe`, so the protocol's realm refusal,
/// which quotes the value whole, only ever quotes a short one.
pub(super) fn admit_realm(kind: &str, params: &Value) -> Result<(), Refusal> {
    let family = match kind {
        "characters" | "character" => Family::Characters,
        "stashes" | "stash" | "refresh" => Family::Stashes,
        _ => return Ok(()),
    };
    match params.get("realm") {
        None | Some(Value::Null) => {}
        Some(Value::String(realm)) if realm.len() <= PREVIEW_BYTES => {}
        Some(other) => {
            return Err(Refusal::new(
                ErrorKind::Refused,
                format!(
                    "realm must be a string naming one of {}, not {}",
                    Realm::ALL.map(Realm::as_str).join(", "),
                    describe(other)
                ),
            ));
        }
    }
    family
        .realm_of(params)
        .map(|_| ())
        .map_err(|e| Refusal::new(ErrorKind::Refused, e))
}

/// C127 — Where a kind may be submitted from, and in which provider mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Door {
    /// A client submits it in either mode.
    Client,
    /// A client submits it against the mock alone: it sends nothing real,
    /// or sends only to the mock's fake endpoints.
    MockOnly,
    /// Only the daemon submits it, for itself.
    DaemonOnly,
}

/// A kind's further check of its params: the refusal's words, or nothing.
pub(super) type ParamCheck = fn(&Value) -> Result<(), String>;

/// C127 — One row of the kind table: a kind, its door, the params that
/// become path segments of its URL, and any further check of its params.
pub(super) struct KindRow {
    pub(super) kind: &'static str,
    pub(super) door: Door,
    /// Params each rendered as exactly one path segment (`route_for`).
    pub(super) segments: &'static [&'static str],
    /// The segment params without which no URL renders.
    pub(super) required: &'static [&'static str],
    /// A further check of the params, run at submit.
    pub(super) check: Option<ParamCheck>,
}

/// The longest `sleep` the mock takes, in seconds: an hour is longer than
/// any demonstration or test wants, and a bound keeps the timer finite.
pub(super) const SLEEP_MAX_SECONDS: f64 = 3600.0;

/// C127 — The kind table: every kind the daemon runs, in one place. The
/// client's door (`admit_client`), apply's tuples (`validate_apply`, by
/// `admit_segments`), the unknown-kind failure and the refusal texts read
/// it; a kind that is not a row is refused at the door.
pub(super) const KINDS: &[KindRow] = &[
    KindRow {
        kind: "profile",
        door: Door::Client,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "characters",
        door: Door::Client,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "character",
        door: Door::Client,
        segments: &["name"],
        required: &["name"],
        check: None,
    },
    KindRow {
        kind: "leagues",
        door: Door::Client,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "stashes",
        door: Door::Client,
        segments: &["league"],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "stash",
        door: Door::Client,
        segments: &["league", "id", "sub"],
        required: &["id"],
        check: None,
    },
    KindRow {
        kind: "refresh",
        door: Door::Client,
        segments: &["league"],
        required: &[],
        check: Some(check_refresh),
    },
    KindRow {
        kind: "apply",
        door: Door::Client,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "sleep",
        door: Door::MockOnly,
        segments: &[],
        required: &[],
        check: Some(check_sleep),
    },
    KindRow {
        kind: "fetch",
        door: Door::MockOnly,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "whoami",
        door: Door::MockOnly,
        segments: &[],
        required: &[],
        check: None,
    },
    KindRow {
        kind: "probe",
        door: Door::DaemonOnly,
        segments: &[],
        required: &[],
        check: None,
    },
];

/// The kinds a client may name in this provider mode, comma-separated,
/// for the refusal texts: never a daemon-only kind, and in real mode no
/// mock-only one, since the door would refuse each of them.
pub(super) fn client_kinds(real: bool) -> String {
    KINDS
        .iter()
        .filter(|row| match row.door {
            Door::Client => true,
            Door::MockOnly => !real,
            Door::DaemonOnly => false,
        })
        .map(|row| row.kind)
        .collect::<Vec<_>>()
        .join(", ")
}

/// The longest path param a segment takes, in UTF-8 bytes. The longest
/// legitimate values are a few dozen bytes: a stash or substash id is 10
/// hex characters, a character's id 64, a character name a couple of
/// dozen, and a league name with an event or private suffix about 40
/// ("Ruthless Hardcore SSF Mercenaries (PL55)"). 255 is several times
/// that, and keeps a three-segment URL under a few kilobytes encoded.
pub(super) const SEGMENT_MAX_BYTES: usize = 255;

/// C127 — Whether a param is exactly one path segment: at most
/// `SEGMENT_MAX_BYTES`; no separator (`/`, or `\`, which a URL parser
/// reads as one); no control character (a URL parser drops tabs and
/// newlines, which can make a dot segment of what was not one); and its
/// head, the part before any `;`, names something other than dots and
/// spaces. A server that strips `;` path parameters (Tomcat, Jetty) reads
/// `..;x` as `..`, and one that trims trailing dots and spaces (IIS) reads
/// `.. .` as `..`; an empty head would leave an empty segment. Anything
/// else is legal and is percent-encoded where it is rendered (a space in a
/// league name, `#` in an event's).
pub(super) fn is_one_segment(value: &str) -> bool {
    let head = value.split(';').next().unwrap_or(value);
    value.len() <= SEGMENT_MAX_BYTES
        && !head.chars().all(|c| c == '.' || c.is_whitespace())
        && !value.contains(['/', '\\'])
        && !value.chars().any(char::is_control)
}

/// C127 — The kind's segment params, judged: each one the row requires is
/// present, and each one present is a string that is exactly one path
/// segment. `route_for` renders no URL for params this refuses, and a
/// job's failure then says these words.
pub(super) fn admit_segments(kind: &str, params: &Value) -> Result<(), String> {
    let Some(row) = KINDS.iter().find(|row| row.kind == kind) else {
        return Ok(());
    };
    for name in row.segments {
        match params.get(*name) {
            None | Some(Value::Null) if row.required.contains(name) => {
                let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                return Err(format!("{kind} needs {article} {name}"));
            }
            None | Some(Value::Null) => {}
            Some(Value::String(value)) if is_one_segment(value) => {}
            Some(Value::String(value)) => {
                return Err(format!(
                    "{kind}: `{name}` {} is not one path segment — it must name something \
other than dots and spaces before any `;`, be at most {SEGMENT_MAX_BYTES} bytes, and hold no \
slash, backslash or control character",
                    describe_str(value)
                ));
            }
            Some(other) => {
                return Err(format!(
                    "{kind}: `{name}` must be a string that is one path segment, not {}",
                    describe(other)
                ));
            }
        }
    }
    Ok(())
}

/// C127 — `sleep`'s `seconds`, when given: a number from 0 to the bound.
/// The door runs it at submit and the runner again before its timer, so a
/// row that never passed the door fails in the same words.
pub(super) fn check_sleep(params: &Value) -> Result<(), String> {
    match params.get("seconds") {
        None | Some(Value::Null) => Ok(()),
        Some(seconds) => match seconds.as_f64() {
            Some(s) if s.is_finite() && (0.0..=SLEEP_MAX_SECONDS).contains(&s) => Ok(()),
            _ => Err(format!(
                "sleep: `seconds` must be a number from 0 to {SLEEP_MAX_SECONDS}, not {}",
                describe(seconds)
            )),
        },
    }
}

/// The longest JSON text of a client's value a refusal quotes whole, and
/// the length of the head it quotes of a longer one, in bytes.
pub(super) const PREVIEW_BYTES: usize = 48;

/// A client's refused value in words: its JSON text whole when that is at
/// most `PREVIEW_BYTES`; otherwise the head of its JSON text, then its
/// JSON type and its size. A refusal never echoes a client's value whole:
/// a response frame has the request's bound (`MAX_FRAME_BYTES`), and each
/// escape grows the text, so a value that fits a request may not fit
/// the refusal that quotes it, and the refusal would be lost. A string's
/// size is its UTF-8 bytes, the measure a segment's bound uses; any other
/// value's is the bytes of its JSON text.
pub(super) fn describe(value: &Value) -> String {
    if let Value::String(text) = value {
        return describe_str(text);
    }
    let what = match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    };
    preview(value, |json_bytes| {
        format!("{what} of {json_bytes} bytes as JSON")
    })
}

/// `describe` for a value that is a string already (a kind, an account
/// selector), without building a `Value` of it.
pub(super) fn describe_str(text: &str) -> String {
    preview(text, |_| format!("a string of {} bytes", text.len()))
}

/// The JSON text of `value` whole if it is short; else its head, then
/// `size` given the text's whole length. One pass that keeps only the
/// head, so a large value is never copied to describe it.
fn preview<T: serde::Serialize + ?Sized>(value: &T, size: impl FnOnce(usize) -> String) -> String {
    /// A writer that counts everything and keeps the first bytes.
    struct Head {
        kept: Vec<u8>,
        total: usize,
    }
    impl std::io::Write for Head {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let room = (PREVIEW_BYTES + 1).saturating_sub(self.kept.len());
            self.kept.extend_from_slice(&buf[..room.min(buf.len())]);
            self.total += buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut head = Head {
        kept: Vec::new(),
        total: 0,
    };
    // Serializing a `Value` or a `str` fails only on the writer, which
    // never does.
    let _ = serde_json::to_writer(&mut head, value);
    let end = head.kept.len().min(PREVIEW_BYTES);
    let kept = &head.kept[..end];
    let text = match std::str::from_utf8(kept) {
        Ok(text) => text,
        // Cut inside a character: keep what precedes it.
        Err(e) => std::str::from_utf8(&kept[..e.valid_up_to()]).unwrap_or(""),
    };
    if head.total <= PREVIEW_BYTES {
        text.to_string()
    } else {
        format!("{text}… ({})", size(head.total))
    }
}

/// The longest message of an error a refusal quotes, in bytes: the
/// longest the decoder writes for a value it names by type (an unknown
/// `req`, which lists the fifteen this version reads) is about 300.
pub(super) const MESSAGE_MAX_BYTES: usize = 512;

/// C127 — An error's message, bounded: whole when it is at most
/// `MESSAGE_MAX_BYTES`; else its head, cut on a character boundary, its
/// whole length, and `tail` (where the cut text said where it went
/// wrong). The decoder's message for a frame that is not a request quotes
/// the mistyped value whole, Debug-escaped, so a value that fits the
/// request frame made a refusal over the response frame, and the refusal
/// was lost. Its message has no one shape to find the value in (a string
/// is quoted, a variant or a field is in backticks), so the whole text is
/// bounded, and its head keeps the decoder's words and the value's type.
/// One pass that keeps only the head.
pub(super) fn bounded_message(error: &dyn std::fmt::Display, tail: &str) -> String {
    /// A writer that counts everything and keeps the head whole.
    struct Head {
        kept: String,
        cut: bool,
        total: usize,
    }
    impl std::fmt::Write for Head {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.total += s.len();
            if !self.cut {
                let room = MESSAGE_MAX_BYTES - self.kept.len();
                let mut end = room.min(s.len());
                while !s.is_char_boundary(end) {
                    end -= 1;
                }
                self.kept.push_str(&s[..end]);
                self.cut = end < s.len();
            }
            Ok(())
        }
    }
    let mut head = Head {
        kept: String::new(),
        cut: false,
        total: 0,
    };
    // `Head` never fails; a `Display` that does leaves what it wrote.
    let _ = std::fmt::write(&mut head, format_args!("{error}"));
    if head.cut {
        format!(
            "{}… (a message of {} bytes, cut){tail}",
            head.kept, head.total
        )
    } else {
        head.kept
    }
}

/// The most bytes a submit's `params` may take as JSON. They are stored
/// and echoed as submitted (C7), so without a bound one submit of junk
/// made `list` larger than the response frame, and the queue kept it.
/// The largest legitimate params are an `apply`'s, one tuple per action
/// of a plan, and a plan is one (realm, league): the owner's Standard,
/// 3,347 tabs (3,009 of them map and unique substashes, N47) and 59
/// characters, is 3,408 actions, about 370 KB, or 560 KB with a 64-byte
/// league name. 8 MiB is over twenty times that, and an eighth of the
/// frame.
pub(super) const PARAMS_MAX_BYTES: usize = 8 << 20;

/// The most actions one `apply` may carry. Each becomes a child job, so
/// the count, not the bytes, is what grows the queue: tiny tuples under
/// `PARAMS_MAX_BYTES` reached about 381,000 children, enough to put
/// `list` past its frame. Measured at this bound, one apply's `list` is
/// about 9 MiB with tiny tuples, 15 MiB with realistic ones, and 28 MiB
/// (44% of the frame) at worst: a 128-byte submitter and params just
/// under `PARAMS_MAX_BYTES`, which the parent's own entry carries whole.
/// Several applies together can still outgrow the frame (issue #56).
/// The bound guards the frame, not a plan's size: the planner has no size limit
/// of its own, and a plan is one (realm, league), the largest observed,
/// the owner's Standard, at most 3,408 actions (two listings, 3,347 tabs
/// by N47, the realm's 59 characters by the sample of 2026-09-02 in
/// `RUN-LEDGER.md`). A heavy account, with a remove-only map tab from
/// each of some 35 leagues, is estimated at 10,000 to 12,000; 50,000 is
/// four to five times that.
pub(super) const APPLY_MAX_ACTIONS: usize = 50_000;

/// C127 — A submit's `params` are at most `PARAMS_MAX_BYTES` as JSON,
/// judged after the checks that name a param, so a value one of them
/// refuses is refused in its words. The refusal names the size, never
/// the value.
pub(super) fn admit_params_size(params: &Value) -> Result<(), Refusal> {
    /// A writer that only counts.
    struct Count(usize);
    impl std::io::Write for Count {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0 += buf.len();
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut count = Count(0);
    // Serializing a `Value` fails only on the writer, which never does.
    let _ = serde_json::to_writer(&mut count, params);
    if count.0 <= PARAMS_MAX_BYTES {
        return Ok(());
    }
    Err(Refusal::new(
        ErrorKind::Refused,
        format!(
            "params of {} bytes as JSON exceed the bound of {PARAMS_MAX_BYTES} bytes; \
submit the work in parts",
            count.0
        ),
    ))
}

/// The most bytes a submit's `submitted_by` may take. It is stored and
/// echoed like its params; a real one is a frontend's name and its pid
/// (`cli:12345`, `acquisition-verbs`' `jobs.rs`), a dozen bytes.
pub(super) const SUBMITTER_MAX_BYTES: usize = 128;

/// C127 — A submit's `submitted_by` is at most `SUBMITTER_MAX_BYTES`; the
/// refusal describes it, never echoes it.
pub(super) fn admit_submitter(submitted_by: &str) -> Result<(), Refusal> {
    if submitted_by.len() <= SUBMITTER_MAX_BYTES {
        return Ok(());
    }
    Err(Refusal::new(
        ErrorKind::Refused,
        format!(
            "submitted_by must be at most {SUBMITTER_MAX_BYTES} bytes, not {}",
            describe_str(submitted_by)
        ),
    ))
}

/// C127 — A refresh's selection, the one rule the door and the run-time
/// arm (`kinds.rs`) both read: `all` true, or the tab ids `tabs` lists
/// (its string elements). There is no default, so a refresh that selects
/// nothing is refused at submit: it would still render a URL, and on a
/// route not yet probed its probe would spend a token before the job
/// failed. A restored row from an older build meets the same words when
/// it runs.
pub(super) fn refresh_selection(params: &Value) -> Result<(bool, Vec<String>), String> {
    let all = params.get("all").and_then(Value::as_bool).unwrap_or(false);
    let tabs: Vec<String> = params
        .get("tabs")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .filter(|tab| !tab.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    if !all && tabs.is_empty() {
        return Err(
            "refresh needs a selection, and there is no default: `all` true, or `tabs` \
listing at least one tab id"
                .into(),
        );
    }
    Ok((all, tabs))
}

/// `refresh`'s selection, judged at submit.
fn check_refresh(params: &Value) -> Result<(), String> {
    refresh_selection(params).map(|_| ())
}

/// C127 — Admission at the client's door (`Request::Submit`, in
/// `server.rs`), before account resolution and before an id exists: the
/// kind is a row of `KINDS` a client may name in this provider mode, and
/// its params pass the row's checks. A daemon-only kind is one the daemon
/// submits for itself: `probe`, whose route and URL are the daemon's to
/// choose (`ensure_probe`, one HEAD per route, N16); a client's pair would
/// carry the bearer to any host and install that host's policy on the
/// route. The daemon's own submissions call `submit` directly and never
/// pass this door; `submit` still judges an `apply`'s tuples.
pub(super) fn admit_client(kind: &str, params: &Value, real: bool) -> Result<(), Refusal> {
    let refused = |message: String| Refusal::new(ErrorKind::Refused, message);
    let Some(row) = KINDS.iter().find(|row| row.kind == kind) else {
        return Err(refused(format!(
            "unknown job kind {}; a client may submit: {}",
            describe_str(kind),
            client_kinds(real)
        )));
    };
    match row.door {
        Door::DaemonOnly => {
            return Err(refused(format!(
                "a {kind} is queued by the daemon itself before a route's first use; \
a client cannot submit one"
            )));
        }
        Door::MockOnly if real => {
            return Err(refused(format!(
                "{kind} is a mock-only kind and is refused in real mode"
            )));
        }
        Door::MockOnly | Door::Client => {}
    }
    admit_segments(kind, params).map_err(refused)?;
    if let Some(check) = row.check {
        check(params).map_err(refused)?;
    }
    Ok(())
}

/// A child submission's parent is no longer there to take it.
fn parent_gone(parent: JobId) -> Refusal {
    Refusal::new(ErrorKind::Refused, format!("parent job {parent} is gone"))
}
