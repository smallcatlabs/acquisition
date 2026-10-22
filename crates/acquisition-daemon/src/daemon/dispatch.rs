//! The dispatcher and a job's life: picking what runs, running it,
//! concluding it, and finishing its parent.
//!
//! Rulings: `decisions/daemon.md`, `decisions/network.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! # Decisions as recorded
//!
//! ## C23 — Work that needs many requests is a parent job that submits child jobs; a parent finishe…
//!
//! **Work that needs many requests is a parent job that submits child jobs; a parent finishes when its last descendant does, gives up its dispatcher task and scheduling key while waiting, and cancels its descendants when cancelled.** Rationale: the queue, dispatcher, priorities, ETAs, and events already work per job, so children get all of it for free; a job-internal loop would need its own scheduler and hide the requests from every tool. Observed API shapes (2026-08-20): folder children are in the stash list (a folder holds tabs only — never items, never another folder; confirmed against GGG patch notes 2026-08-24); map/unique substashes only appear on fetching the tab (one map tab listed 234); substash stubs carry `metadata.items` counts. Following substashes is opt-in per tab. Children exist only once their parent runs, so a progress denominator grows while a client watches ("0/1", then "8/8", and again when each map/unique tab lands): any progress UI must expect the tree to widen (frontend boundary finding, 2026-08-24).
//!

use super::*;

/// What `execute` hands back to `process`.
pub(super) enum Exec {
    Done(Outcome),
    RateLimited(String),
    /// C25: a halt turned the job away after the pick; nothing of it
    /// reached GGG. It returns to waiting with no retry spent.
    Parked(String),
}

pub(super) fn may_requeue_429(retries: u32, cancel_requested: bool) -> bool {
    retries < MAX_429_RETRIES && !cancel_requested
}

impl Daemon {
    // ---- dispatcher -----------------------------------------------------

    /// Starts at most one task per scheduling key. N4's gate, not this
    /// dispatcher, owns actual-send capacity. Woken by submits, completions,
    /// and reprioritization.
    pub(super) async fn dispatcher(self: Arc<Self>) {
        loop {
            let picks = self.pick_runnable();
            for id in picks {
                tokio::spawn(self.clone().run_active(id));
            }
            self.work.notified().await;
        }
    }

    /// Waiting jobs, in dispatch order, that can start a task right now.
    pub(super) fn pick_runnable(&self) -> Vec<JobId> {
        let mut s = self.shared.lock().unwrap();
        // A failed queue means no new dispatch at all: every transition
        // a job makes from here would widen the memory/disk divergence.
        if self.queue_failed().is_some() {
            return Vec::new();
        }
        let mut busy: HashSet<String> = s.active_jobs.values().cloned().collect();
        let halted = self.rails().halted().is_some();
        let mut picks = Vec::new();
        // Whether a route's probe is pending, asked once per route: the
        // question scans every job, and a queue waiting on one probe
        // would otherwise scan it once per job.
        let mut probing: HashMap<String, bool> = HashMap::new();
        for id in s.queue_order() {
            let entry = &s.jobs[&id];
            let route = self.keyed_route_for(
                &entry.info.kind,
                &entry.params,
                entry.info.account.as_deref(),
            );
            // A halted daemon (the rails, `rails.rs`) sends nothing:
            // network jobs wait, on disk, for `reset-tripwire`, and so do
            // the kinds with no route that may still send (a probe's HEAD,
            // `whoami`'s refresh). A job a halt turned away after its pick
            // (C25) is so not picked again at once.
            if halted && (route.is_some() || Self::sends_unrouted(&entry.info.kind)) {
                continue;
            }
            // A job whose route is still being probed has nothing to do yet.
            if let Some((route, _)) = route
                && self.choke.endpoint_state(&route) == EndpointState::Unknown
                && *probing
                    .entry(route)
                    .or_insert_with_key(|route| Self::probe_pending(&s, route))
            {
                continue;
            }
            let key = self.serial_key(entry);
            if busy.contains(&key) {
                continue;
            }
            busy.insert(key.clone());
            picks.push((id, key));
        }
        for (id, key) in &picks {
            s.active_jobs.insert(*id, key.clone());
        }
        picks.into_iter().map(|(id, _)| id).collect()
    }

    async fn run_active(self: Arc<Self>, id: JobId) {
        self.process(id).await;
        self.shared.lock().unwrap().active_jobs.remove(&id);
        self.work.notify_one();
    }

    async fn process(&self, id: JobId) {
        let ready = self.choke.now();
        let (route, account, kind) = {
            let s = self.shared.lock().unwrap();
            match s.jobs.get(&id) {
                Some(e) => (
                    self.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref()),
                    e.info.account.clone(),
                    e.info.kind.clone(),
                ),
                None => return,
            }
        };

        // L0 rail 1: a halted daemon sends nothing — no probe, no pacing
        // wait, no permit. The job stays waiting (the dispatcher will not
        // pick it again until the reset); its key is given back. A probe
        // and `whoami` (`sends_unrouted`) are held the same way.
        if self.rails().halted().is_some() && (route.is_some() || Self::sends_unrouted(&kind)) {
            return;
        }

        // A route we've never heard from gets a probe first (N16, N24) —
        // unless HEAD is known not to be accepted there; a degraded one
        // (N20) fails its jobs cleanly until the cooldown ends.
        if let Some((route, url)) = &route {
            match self.choke.endpoint_state(route) {
                EndpointState::Unknown if Self::route_probes(route) => {
                    self.ensure_probe(route, url, account);
                    return; // scheduling key released; the probe outranks us
                }
                EndpointState::Degraded { until, reason } => {
                    let left = until.saturating_duration_since(self.choke.now()).as_secs();
                    let error = format!("route {route} is degraded for another {left}s: {reason}");
                    self.note_error(&format!("job {id}: {error}"));
                    self.start_and_finish(id, Outcome::failure(error));
                    return;
                }
                // A no-probe route's first GET is what teaches the limiter.
                EndpointState::Unknown | EndpointState::Policy(_) | EndpointState::Policyless => {}
            }
        }

        // Dispatcher pacing remains while the job is still `waiting`, in
        // short slices so cancellation and reprioritization stay responsive.
        // This is only a scheduling hint: after authentication, ChokePoint
        // repeats the final limiter check under the actual-send permit.
        if let Some((route, _)) = &route {
            loop {
                let step = {
                    let s = self.shared.lock().unwrap();
                    let Some(me) = s.jobs.get(&id) else { return };
                    if me.info.state != JobState::Waiting {
                        return; // cancelled out from under us
                    }
                    // A higher-priority job on the same key may have arrived;
                    // give the key back so the dispatcher picks it instead.
                    let my_key = self.serial_key(me);
                    let outranked = s
                        .queue_order()
                        .into_iter()
                        .take_while(|&q| q != id)
                        .any(|q| s.jobs.get(&q).is_some_and(|e| self.serial_key(e) == my_key));
                    if outranked {
                        return;
                    }
                    self.choke.check(route)
                };
                match step {
                    Ok(()) => break,
                    Err(d) => self.choke.sleep(d.min(Duration::from_secs(1))).await,
                }
            }
        }

        let job = {
            let mut s = self.shared.lock().unwrap();
            let Some(entry) = s.jobs.get_mut(&id) else {
                return;
            };
            if entry.info.state != JobState::Waiting {
                return;
            }
            entry.info.state = JobState::Running;
            if !self.persist(entry) {
                // Disk still says waiting: after a restart this run would
                // be invisible (a no-probe job would replay blind). A send
                // the queue cannot see must not happen — revert, don't run.
                entry.info.state = JobState::Waiting;
                return;
            }
            (entry.info.clone(), entry.params.clone())
        };
        let (info, params) = job;
        self.emit(info);

        let route = route.map(|(route, _)| route);
        let outcome = match self
            .execute(id, &kind, params, route, account.as_deref(), ready)
            .await
        {
            Exec::Done(outcome) => outcome,
            Exec::RateLimited(evidence) => {
                // P-A: a 429 is recovered from, not surfaced — unless it keeps
                // happening. The limiter already holds the policy for
                // Retry-After + bucket (N19); putting the job back to waiting
                // (it keeps its place: order is priority, then id) makes it
                // go out after that hold, with the ETA visible meanwhile.
                let requeued = {
                    let mut s = self.shared.lock().unwrap();
                    let Some(entry) = s.jobs.get_mut(&id) else {
                        return;
                    };
                    if may_requeue_429(entry.info.retries, entry.cancel_requested) {
                        entry.info.retries += 1;
                        entry.info.state = JobState::Waiting;
                        self.persist(entry);
                        Some(entry.info.clone())
                    } else {
                        None
                    }
                };
                match requeued {
                    Some(info) => {
                        self.note_error(&format!(
                            "job {id} ({kind}): rate limited (429), re-queued behind the limiter's hold (retry {}/{MAX_429_RETRIES}): {evidence}",
                            info.retries
                        ));
                        self.emit(info);
                        return; // scheduling key released; re-pick after the hold
                    }
                    None => Outcome::failure(format!(
                        "rate limited (429) {MAX_429_RETRIES} times; giving up: {evidence}"
                    )),
                }
            }
            // C25: turned away by a halt after the pick. Back to waiting,
            // persisted, no retry spent; the key is given back on return,
            // and the pick passes it over until the reset. A cancel that
            // landed while it ran is honoured here, under the lock `cancel`
            // takes: the job concludes cancelled instead.
            Exec::Parked(cause) => {
                let parked = {
                    let mut s = self.shared.lock().unwrap();
                    let Some(entry) = s.jobs.get_mut(&id) else {
                        return;
                    };
                    if entry.cancel_requested {
                        None
                    } else {
                        entry.info.state = JobState::Waiting;
                        self.persist(entry);
                        Some(entry.info.clone())
                    }
                };
                match parked {
                    Some(info) => {
                        self.log(&format!(
                            "job {id} ({kind}): turned away by a halt, back to waiting with no retry spent: {cause}"
                        ));
                        self.emit(info);
                        return;
                    }
                    None => Outcome::Cancelled,
                }
            }
        };
        if let Outcome::Failure { error, .. } = &outcome {
            self.note_error(&format!("job {id} ({kind}): {error}"));
        }
        self.conclude(id, outcome);
    }

    /// A finished job's landing. A job that spawned children holds its
    /// own result until they're all done (its scheduling key is already
    /// given back so children can run); anything else finishes. A
    /// cancellation that landed during the fan-out is honored here, under
    /// the lock: the held result becomes `Cancelled`, so a parent
    /// cancelled after its last child was submitted can never report
    /// success.
    pub(super) fn conclude(&self, id: JobId, outcome: Outcome) {
        let has_children = {
            let mut s = self.shared.lock().unwrap();
            let spawned = s.jobs.values().any(|e| e.info.parent == Some(id));
            if spawned
                && let Some(entry) = s.jobs.get_mut(&id)
                && entry.info.state == JobState::Running
            {
                entry.deferred = Some(if entry.cancel_requested {
                    Outcome::Cancelled
                } else {
                    outcome.clone()
                });
                self.persist(entry);
            }
            spawned
        };
        if has_children {
            self.maybe_finish_parent(id);
        } else {
            self.finish(id, outcome);
        }
    }

    /// If `pid` is waiting on children and none are left running, finish it
    /// with its held-back result plus a summary of what the children did.
    pub(super) fn maybe_finish_parent(&self, pid: JobId) {
        let final_outcome = {
            let mut s = self.shared.lock().unwrap();
            let Some(parent) = s.jobs.get(&pid) else {
                return;
            };
            if parent.deferred.is_none() {
                return;
            }
            let children: Vec<&Entry> = s
                .jobs
                .values()
                .filter(|e| e.info.parent == Some(pid))
                .collect();
            if children.iter().any(|e| !e.info.state.is_terminal()) {
                return;
            }
            let (mut done, mut failed, mut cancelled) = (0, 0, 0);
            let mut failed_ids = Vec::new();
            for c in &children {
                match c.info.state {
                    JobState::Done => done += 1,
                    JobState::Failed => {
                        failed += 1;
                        failed_ids.push(c.info.id);
                    }
                    _ => cancelled += 1,
                }
            }
            let total = children.len();
            // Sorted for every reader: the ids are the remedy's handles
            // (`acq result <id>`), and a HashMap's order is noise.
            failed_ids.sort_unstable();
            let tally = Children {
                done,
                failed,
                cancelled,
                failed_ids,
            };
            let deferred = s.jobs.get_mut(&pid).unwrap().deferred.take().unwrap();
            // C23: every parent failure carries the tally as a field, the
            // sentence kept for a reader; a success carries it in its
            // payload, as it always has.
            match deferred {
                Outcome::Success { mut payload } if failed == 0 && cancelled == 0 => {
                    payload["children"] = json!(tally);
                    Outcome::Success { payload }
                }
                // Cancelled children mean the work was not completed; a
                // parent must not call that success.
                Outcome::Success { .. } if failed == 0 => Outcome::Failure {
                    error: format!("{cancelled} of {total} child jobs were cancelled"),
                    children: Some(tally),
                },
                // The counts in words; the ids ride in the tally alone,
                // never as prose a client reads back (C23).
                Outcome::Success { .. } => Outcome::Failure {
                    error: format!(
                        "{failed} of {total} child jobs failed, {done} done{}",
                        if cancelled > 0 {
                            format!(", {cancelled} cancelled")
                        } else {
                            String::new()
                        }
                    ),
                    children: Some(tally),
                },
                // A held failure (the interrupted fan-out a restart
                // installs, or the parent's own): its message stands.
                Outcome::Failure { error, .. } => Outcome::Failure {
                    error,
                    children: Some(tally),
                },
                Outcome::Cancelled => Outcome::Cancelled,
            }
        };
        self.finish(pid, final_outcome);
    }

    /// The one place a job becomes terminal (`start_and_finish` and
    /// `maybe_finish_parent` both land here). A pending cancellation is
    /// arbitrated under this final lock: if `cancel` set the flag at any
    /// point before terminalization — including between a caller
    /// computing this outcome and the lock below — the job finishes
    /// `Cancelled`, never with the stale outcome. Cancellation wins the
    /// job's terminal surface; work already done may have sent regardless
    /// (sends are committed once dispatched), and storable API responses
    /// were recorded as they landed — though not every outcome is stored
    /// separately (probes, mock fetches, and failures are not).
    pub(super) fn finish(&self, id: JobId, outcome: Outcome) {
        let info = {
            let mut s = self.shared.lock().unwrap();
            let Some(entry) = s.jobs.get_mut(&id) else {
                return;
            };
            let outcome = if entry.cancel_requested {
                Outcome::Cancelled
            } else {
                outcome
            };
            entry.info.state = match &outcome {
                Outcome::Success { .. } => JobState::Done,
                Outcome::Failure { .. } => JobState::Failed,
                Outcome::Cancelled => JobState::Cancelled,
            };
            entry.outcome = Some(outcome);
            self.persist(entry);
            entry.info.clone()
        };
        let parent = info.parent;
        self.emit(info);
        if let Some(pid) = parent {
            self.maybe_finish_parent(pid);
        }
    }

    /// For jobs that fail before they ever run: pass through `running` so
    /// subscribers see the same state sequence as every other job.
    pub(super) fn start_and_finish(&self, id: JobId, outcome: Outcome) {
        let info = {
            let mut s = self.shared.lock().unwrap();
            let Some(entry) = s.jobs.get_mut(&id) else {
                return;
            };
            if entry.info.state != JobState::Waiting {
                return;
            }
            entry.info.state = JobState::Running;
            self.persist(entry);
            entry.info.clone()
        };
        self.emit(info);
        self.finish(id, outcome);
    }

    /// `ready` is when the dispatcher picked the job: every send it makes
    /// journals its wait from that instant.
    pub(super) async fn execute(
        &self,
        id: JobId,
        kind: &str,
        params: Value,
        route: Option<String>,
        account: Option<&str>,
        ready: Instant,
    ) -> Exec {
        // Network kinds bubble a 429 up as `Exec::RateLimited`; everything
        // else is an ordinary outcome.
        match self
            .execute_inner(id, kind, params, route, account, ready)
            .await
        {
            Ok(outcome) => Exec::Done(outcome),
            Err(ApiError::RateLimited(evidence)) => Exec::RateLimited(evidence),
            Err(ApiError::Protocol(error)) => Exec::Done(Outcome::failure(error)),
            Err(ApiError::Other(error)) => Exec::Done(Outcome::failure(error)),
            Err(ApiError::Halted(cause)) => Exec::Parked(cause),
        }
    }
}
