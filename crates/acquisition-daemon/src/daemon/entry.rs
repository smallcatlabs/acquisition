//! #261, one ending: a job in memory ([`Entry`]) and the one function
//! that makes it terminal ([`Daemon::end`]), held together in this module
//! so the compiler holds the rule. `Entry`'s fields are private here:
//! another module reads a job through [`Entry::info`] and the readers
//! beside it, changes an open job only by a named transition (a start, a
//! re-queue, a park, a priority, a cancel request, a held result), and
//! can make one terminal only by calling [`Daemon::end`], which carries
//! every duty an ending has. Adding a site that sets a terminal state or
//! an outcome by hand fails to build.
//!
//! Rulings: `decisions/daemon.md`; the cross-cutting ones are `AGENTS.md`'s.

use super::*;

/// A job [`Daemon::end`] made terminal: its info, for the event sent
/// after the lock, and whether its row's write landed (C6).
pub(super) struct Ended {
    pub(super) info: JobInfo,
    pub(super) landed: bool,
}

/// One job in memory. Its params are `info.params`, the one copy (C7,
/// they travel on `JobInfo` as submitted).
pub(super) struct Entry {
    info: JobInfo,
    outcome: Option<Outcome>,
    cancel_requested: bool,
    /// A parent's own result, held back until its descendants finish. Set
    /// means "running, waiting on children, with no active dispatcher task".
    deferred: Option<Outcome>,
}

impl Entry {
    /// A job as submitted: waiting, nothing held, no cancel asked.
    pub(super) fn waiting(info: JobInfo) -> Entry {
        Entry {
            info,
            outcome: None,
            cancel_requested: false,
            deferred: None,
        }
    }

    /// A test's job in a given state with a given outcome: a fixture,
    /// never an ending (no stamp, no write, no eviction, no event).
    #[cfg(test)]
    pub(super) fn fixture(info: JobInfo, outcome: Option<Outcome>) -> Entry {
        Entry {
            info,
            outcome,
            cancel_requested: false,
            deferred: None,
        }
    }

    /// The job as a client reads it.
    pub(super) fn info(&self) -> &JobInfo {
        &self.info
    }

    /// Its outcome, once it has ended.
    pub(super) fn outcome(&self) -> Option<&Outcome> {
        self.outcome.as_ref()
    }

    /// Whether a cancel was asked of it while it ran.
    pub(super) fn cancel_requested(&self) -> bool {
        self.cancel_requested
    }

    /// A parent's result, held until its descendants finish.
    pub(super) fn deferred(&self) -> Option<&Outcome> {
        self.deferred.as_ref()
    }

    /// C165: the job's transition into running, the one place a start is
    /// set: its first, kept by a 429's re-queue and a restart. A record
    /// that never had one (a running row of schema v1, a parent restored
    /// still running) keeps none: a persist is no start (the review of
    /// round E, finding 4).
    pub(super) fn start(&mut self, now: i64) {
        self.info.state = JobState::Running;
        self.info.times.started_at.get_or_insert(now);
    }

    /// A start the queue could not write, undone: back to waiting, with
    /// the start it had before (C165), so no start that did not happen
    /// is kept.
    pub(super) fn revert_start(&mut self, started_at: Option<i64>) {
        self.info.state = JobState::Waiting;
        self.info.times.started_at = started_at;
    }

    /// P-A: a 429 recovered from: back to waiting behind the limiter's
    /// hold, its retry counted.
    pub(super) fn requeue_429(&mut self) {
        self.info.retries += 1;
        self.info.state = JobState::Waiting;
    }

    /// C25: turned away by a halt after the pick: back to waiting, no
    /// retry spent.
    pub(super) fn park(&mut self) {
        self.info.state = JobState::Waiting;
    }

    /// C6: a job a restart found running, re-queued to run again.
    pub(super) fn requeue(&mut self) {
        self.info.state = JobState::Waiting;
    }

    /// Its priority, changed while it waits.
    pub(super) fn set_priority(&mut self, priority: Priority) {
        self.info.priority = priority;
    }

    /// A cancel asked of it while it runs, honoured at its end.
    pub(super) fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    /// A parent's own result, held until its descendants finish; or, with
    /// `None`, the held result dropped.
    pub(super) fn hold(&mut self, deferred: Option<Outcome>) {
        self.deferred = deferred;
    }

    /// The held result, taken as the parent concludes.
    pub(super) fn take_deferred(&mut self) -> Option<Outcome> {
        self.deferred.take()
    }

    /// C165: the finish this state makes true, set once when the job is
    /// terminal and never moved after. [`Daemon::persist`] calls it before
    /// every write, so memory and the row agree. The submission is set at
    /// submit, the start by [`Self::start`].
    pub(super) fn stamp(&mut self, now: i64) {
        if self.info.state.is_terminal() {
            self.info.times.finished_at.get_or_insert(now);
        }
    }

    /// The row `daemon.db` holds for this job (C6): `JobInfo` column for
    /// column plus the fields a restart needs. `eta_seconds` is a
    /// prediction, never stored.
    pub(super) fn row(&self, now: i64) -> JobRow {
        let json = |o: &Outcome| serde_json::to_value(o).unwrap_or(Value::Null);
        JobRow {
            id: self.info.id,
            kind: self.info.kind.clone(),
            state: self.info.state.to_string(),
            priority: self.info.priority,
            submitted_by: self.info.submitted_by.clone(),
            parent: self.info.parent,
            retries: self.info.retries,
            account: self.info.account.clone(),
            params: self.info.params.clone(),
            outcome: self.outcome.as_ref().map(json),
            deferred: self.deferred.as_ref().map(json),
            cancel_requested: self.cancel_requested,
            submitted_at: self.info.times.submitted_at.unwrap_or(now),
            updated_at: now,
            started_at: self.info.times.started_at,
            finished_at: self.info.times.finished_at,
        }
    }

    /// [`Entry::from_row`], a state word this daemon does not know
    /// refused rather than read as no row.
    pub(super) fn from_row_read(row: JobRow) -> Result<Option<Entry>, Refusal> {
        match job_state(&row.state) {
            Some(_) => Ok(Entry::from_row(row)),
            None => Err(Refusal::unknown_state(row.id, &row.state)),
        }
    }

    /// A restored row. What a restart does with `running` (re-queue) or
    /// `probe` (drop) is decided by the caller; this is the plain mapping.
    pub(super) fn from_row(row: JobRow) -> Option<Entry> {
        let state = job_state(&row.state)?;
        let outcome = |v: Option<Value>| v.and_then(|v| serde_json::from_value(v).ok());
        Some(Entry {
            info: JobInfo {
                id: row.id,
                kind: row.kind,
                state,
                priority: row.priority,
                submitted_by: row.submitted_by,
                eta_seconds: None,
                parent: row.parent,
                retries: row.retries,
                account: row.account,
                params: row.params,
                times: JobTimes {
                    submitted_at: Some(row.submitted_at),
                    started_at: row.started_at,
                    finished_at: row.finished_at,
                    ..JobTimes::default()
                },
            },
            outcome: outcome(row.outcome),
            cancel_requested: row.cancel_requested,
            deferred: outcome(row.deferred),
        })
    }
}

impl Daemon {
    /// #261: the one ending, the only code that makes a job terminal.
    /// Under `shared`, in order: a pending cancel is arbitrated (if
    /// `cancel` set the flag at any point before this, including between a
    /// caller computing `outcome` and the lock, the job ends `Cancelled`,
    /// never with the stale outcome; work already sent may have landed
    /// regardless, since sends are committed once dispatched); the state
    /// and the outcome are set; activity is stamped (C3 as amended, #208:
    /// under the lock the idle verdict reads it under, by the signature,
    /// so no verdict sees the job ended with the hour spent); the row is
    /// written, and whether it landed is returned (C6); the concluded tree
    /// leaves memory (`evict_tree`). What must happen after the lock, the
    /// event and the parent's tally (C23), is returned for the caller
    /// ([`Self::after_end`]). `None` for a job memory does not hold.
    pub(super) fn end(&self, s: &mut Shared, id: JobId, outcome: Outcome) -> Option<Ended> {
        let entry = s.jobs.get_mut(&id)?;
        self.mark_activity(self.choke.now());
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
        let landed = self.persist(entry);
        let info = entry.info.clone();
        self.evict_tree(s, id);
        Some(Ended { info, landed })
    }
}
