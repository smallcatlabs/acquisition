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
/// the param.
pub(super) fn admit_realm(kind: &str, params: &Value) -> Result<(), Refusal> {
    let family = match kind {
        "characters" | "character" => Family::Characters,
        "stashes" | "stash" | "refresh" => Family::Stashes,
        _ => return Ok(()),
    };
    family
        .realm_of(params)
        .map(|_| ())
        .map_err(|e| Refusal::new(ErrorKind::Refused, e))
}

/// A child submission's parent is no longer there to take it.
fn parent_gone(parent: JobId) -> Refusal {
    Refusal::new(ErrorKind::Refused, format!("parent job {parent} is gone"))
}
