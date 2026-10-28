//! The job verbs (C4, API requests are jobs): submit, wait, and the
//! queue's reads, live or from the ledger on disk when no daemon answers
//! (C45, the ledger answers). An observer never spawns (C10); a job verb
//! that submits passes its account selector through to the daemon
//! unresolved, so a job verb with no login still spawns and is refused by
//! the daemon (C3).

use std::time::Duration;

use acquisition_client::client::{Client, DaemonError};
use acquisition_protocol::job::{JobInfo, JobState, Outcome, target_of};
use acquisition_protocol::protocol::{ErrorKind, Request, Response};
use acquisition_store::bound::bounded;
use acquisition_store::jobs::{Persisted, PersistedJob, PersistedRecord};
use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;

use crate::fault::Fault;
use crate::frontend::Frontend;
use crate::refresh::list;

/// How often [`wait`] reads the job while it runs.
const POLL: Duration = Duration::from_millis(500);

/// What the children of a parent job are doing right now.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ChildTally {
    pub total: usize,
    pub done: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub waiting: usize,
    pub running: usize,
    /// The soonest predicted start among waiting children, with its kind.
    pub next: Option<(u64, String)>,
}

/// One reading of a job being waited for, for a frontend's progress
/// line: the job, and for a parent that runs its children's tally (C23).
#[derive(Clone, Debug)]
pub struct Progress {
    pub job: JobInfo,
    pub children: Option<ChildTally>,
}

/// C4 and C3: submit one job and return its id. `account` is the
/// selector as the caller gave it, passed through unresolved; the
/// submitter is this frontend's channel and process (C65).
pub async fn submit(
    frontend: &Frontend,
    client: &mut Client,
    kind: &str,
    params: Value,
    priority: u8,
    account: Option<&str>,
) -> Result<u64, Fault> {
    let request = Request::Submit {
        kind: kind.into(),
        params,
        priority,
        submitted_by: format!("{}:{}", frontend.via, std::process::id()),
        account: account.map(str::to_string),
    };
    match client.request(&request).await.map_err(Fault::from_client)? {
        Response::Submitted { id } => Ok(id),
        Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
        other => Err(unexpected(other)),
    }
}

/// C4 (a rate-limit wait is a job state, never a blocked call): read the
/// job until it is terminal and return its outcome. `progress` is given
/// each reading, a parent's with its children's tally (C23).
pub async fn wait(
    client: &mut Client,
    id: u64,
    mut progress: Option<&mut (dyn FnMut(&Progress) + Send)>,
) -> Result<Outcome, Fault> {
    loop {
        let job = client.status(id).await.map_err(Fault::from_client)?;
        let terminal = job.state.is_terminal();
        if let Some(report) = progress.as_deref_mut() {
            let children = if job.kind == "apply" && job.state != JobState::Waiting {
                Some(tally(client, job.id).await?)
            } else {
                None
            };
            report(&Progress { job, children });
        }
        if terminal {
            return outcome_live(client, id).await;
        }
        tokio::time::sleep(POLL).await;
    }
}

/// A parent's children, counted from the daemon's list.
async fn tally(client: &mut Client, parent: u64) -> Result<ChildTally, Fault> {
    let mut tally = ChildTally::default();
    for job in list(client).await? {
        if job.parent != Some(parent) {
            continue;
        }
        tally.total += 1;
        match job.state {
            JobState::Done => tally.done += 1,
            JobState::Failed => tally.failed += 1,
            JobState::Cancelled => tally.cancelled += 1,
            JobState::Running => tally.running += 1,
            JobState::Waiting => {
                tally.waiting += 1;
                if let Some(eta) = job.eta_seconds
                    && eta > 0
                    && tally.next.as_ref().is_none_or(|(e, _)| eta < *e)
                {
                    tally.next = Some((eta, job.kind.clone()));
                }
            }
        }
    }
    Ok(tally)
}

/// The queue: the daemon's list, or with no daemon the ledger's open
/// rows (C45), each one document counted beside its rows (C149, every
/// answer one object; C53, the read economy). `persisted` is `None` when
/// this world has no queue yet.
#[derive(Clone, Debug)]
pub enum Jobs {
    Live(Listed),
    Absent { persisted: Option<PersistedJobs> },
}

/// Jobs by state, every state named, a zero included.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ByState {
    pub running: usize,
    pub waiting: usize,
    pub done: usize,
    pub failed: usize,
    pub cancelled: usize,
}

/// The counts `acq jobs` prints on its last line, made over every job a
/// list holds before any cut: how many, by state, and of the waiting
/// jobs with a predicted start the soonest and the latest, in seconds
/// from now — a start, never an end.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct JobCounts {
    /// Every job the list holds.
    pub total: usize,
    pub by_state: ByState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_starts_in_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_starts_in_seconds: Option<u64>,
}

impl JobCounts {
    /// The counts of `jobs`, whole.
    pub fn of(jobs: &[JobInfo]) -> JobCounts {
        let count = |s: JobState| jobs.iter().filter(|j| j.state == s).count();
        let etas = || {
            jobs.iter()
                .filter(|j| j.state == JobState::Waiting)
                .filter_map(|j| j.eta_seconds)
                .filter(|e| *e > 0)
        };
        JobCounts {
            total: jobs.len(),
            by_state: ByState {
                running: count(JobState::Running),
                waiting: count(JobState::Waiting),
                done: count(JobState::Done),
                failed: count(JobState::Failed),
                cancelled: count(JobState::Cancelled),
            },
            next_starts_in_seconds: etas().min(),
            last_starts_in_seconds: etas().max(),
        }
    }
}

/// C149 and C53: the daemon's job list as one document — its counts over
/// every job ([`JobCounts`]), then `rows`, in the daemon's id order: the
/// open jobs (running or waiting), the first ten by id — not by when they
/// run, which priority decides, so a running job can be among those left
/// out — and the ten most recent finished; every job under
/// `expand` (the CLI's `--all`). What is left out is counted, open and
/// finished apart (the brief's second review, finding 2: no exemption
/// for the open jobs).
#[derive(Clone, Debug, Serialize)]
pub struct Listed {
    #[serde(flatten)]
    pub counts: JobCounts,
    pub rows: Vec<JobInfo>,
    /// Open jobs not in `rows`.
    pub open_left_out: usize,
    /// Finished jobs not in `rows`: the earliest.
    pub finished_left_out: usize,
    /// What lists them all: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

impl Listed {
    /// The daemon's list, counted whole and then cut by the one rule.
    pub fn of(jobs: Vec<JobInfo>, expand: bool) -> Listed {
        let counts = JobCounts::of(&jobs);
        let (open, finished): (Vec<JobInfo>, Vec<JobInfo>) =
            jobs.into_iter().partition(|j| !j.state.is_terminal());
        let (open, open_left_out) = bounded(open, None, expand);
        // the most recent finished are the last by id: cut from the end
        let mut finished = finished;
        finished.reverse();
        let (mut finished, finished_left_out) = bounded(finished, None, expand);
        finished.reverse();
        let mut rows = open;
        rows.extend(finished);
        rows.sort_by_key(|j| j.id);
        Listed {
            counts,
            rows,
            open_left_out,
            finished_left_out,
            left_out_needs: (open_left_out + finished_left_out > 0).then_some("expand"),
        }
    }
}

/// C45 and C149: the ledger's open rows as one document — `jobs` (the
/// store's rows, ten, every one under `expand`), `written_at`, and the
/// ledger's counts over every open row: `total`, `waiting`,
/// `recorded_running`, the words `daemon_status` uses, and `left_out`.
#[derive(Clone, Debug, Serialize)]
pub struct PersistedJobs {
    pub jobs: Vec<PersistedJob>,
    pub written_at: Option<i64>,
    /// Every open row the ledger holds.
    pub total: usize,
    pub waiting: usize,
    pub recorded_running: usize,
    /// The open rows not in `jobs`.
    pub left_out: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

impl PersistedJobs {
    /// The ledger's open rows, counted whole and then cut by the one rule.
    pub fn of(persisted: Persisted, expand: bool) -> PersistedJobs {
        let (total, waiting, recorded_running) = (
            persisted.jobs.len(),
            persisted.waiting(),
            persisted.recorded_running(),
        );
        let (jobs, left_out) = bounded(persisted.jobs, None, expand);
        PersistedJobs {
            jobs,
            written_at: persisted.written_at,
            total,
            waiting,
            recorded_running,
            left_out,
            left_out_needs: (left_out > 0).then_some("expand"),
        }
    }

    /// The ledger's counts in the live list's shape, for one summary
    /// line: what was recorded running counts as running.
    pub fn counts(&self) -> JobCounts {
        JobCounts {
            total: self.total,
            by_state: ByState {
                running: self.recorded_running,
                waiting: self.waiting,
                ..ByState::default()
            },
            ..JobCounts::default()
        }
    }
}

/// One job: the daemon's reading, or with no daemon the ledger's row as
/// recorded (C45), with `next`, the command that starts a daemon.
#[derive(Clone, Debug)]
pub enum Job {
    Live(JobInfo),
    Absent {
        persisted: PersistedJob,
        next: String,
    },
}

/// A terminal job's outcome with the job's id and, where known, its
/// account beside it (C53, additive): from the daemon, or from the
/// ledger on disk, which is final once the job is.
#[derive(Clone, Debug, Serialize)]
pub struct JobResult {
    pub job_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(flatten)]
    pub outcome: Outcome,
    /// C23 (a parent's failure carries its children's tally): for a
    /// parent whose children failed, the first
    /// [`FAILED_CHILDREN_DETAILED`] of them with their kind, target and
    /// own error, read by the same reader as the parent (the daemon, or
    /// the ledger when none answers, C45), so one call answers what
    /// failed and why. The tally's `failed_ids` stays whole beside it:
    /// every id, details for the first ten. Empty, and absent from the
    /// JSON, for any other outcome.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failed_children: Vec<FailedChild>,
}

/// How many of a parent's failed children a result details; the tally
/// lists every id (C23, the one exemption from the read economy's
/// bound). The bound's own number, held once (C53).
pub const FAILED_CHILDREN_DETAILED: usize = acquisition_store::bound::LISTED;

/// One failed child of a parent, as the parent's result details it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FailedChild {
    pub job_id: u64,
    /// Its kind and target, empty when no record of the child can be
    /// read (live, a daemon that no longer holds it after a restart).
    pub kind: String,
    pub target: String,
    /// Its own error; or, where it cannot be read, what happened — that
    /// no result is retained, or why the read failed.
    pub error: String,
}

/// The words for a child confirmed absent — the daemon does not know
/// its id, or the ledger holds no row for it — and for nothing else: a
/// read that failed for another reason says why (`child_error`).
pub const NO_RESULT: &str = "no result retained for this job";

/// The failed ids a result details, the first ten of the tally's.
fn detailed(outcome: &Outcome) -> &[u64] {
    match outcome {
        Outcome::Failure {
            children: Some(children),
            ..
        } => {
            let ids = &children.failed_ids;
            &ids[..ids.len().min(FAILED_CHILDREN_DETAILED)]
        }
        _ => &[],
    }
}

/// A failed child's error from the read of its outcome, never a guess:
/// its own error; a tally that disagrees with the child's own outcome,
/// said; [`NO_RESULT`] for confirmed absence alone (the daemon's
/// `unknown_job`, the ledger's missing row); any other failed read with
/// its cause.
fn child_error(read: Result<Outcome, Fault>) -> String {
    match read {
        Ok(Outcome::Failure { error, .. }) => error,
        Ok(Outcome::Success { .. }) => {
            "the parent's tally lists it failed, but its own outcome is a success".into()
        }
        Ok(Outcome::Cancelled) => {
            "the parent's tally lists it failed, but its own outcome is a cancellation".into()
        }
        Err(Fault::Daemon(e)) if e.kind == ErrorKind::UnknownJob => NO_RESULT.into(),
        Err(Fault::NoJobOnDisk { .. }) => NO_RESULT.into(),
        Err(fault) => format!("its result could not be read: {fault}"),
    }
}

/// A ledger row's outcome as the child reader takes it: none recorded,
/// or one this build cannot parse, each said.
fn disk_outcome(outcome: Option<Value>) -> Result<Outcome, String> {
    let Some(outcome) = outcome else {
        return Err("its row on disk records no outcome yet".into());
    };
    serde_json::from_value(outcome).map_err(|e| format!("its outcome on disk does not parse: {e}"))
}

impl Serialize for Jobs {
    /// Live: `running: true` beside the listed document ([`Listed`]).
    /// Absent: `running: false` and the persisted queue.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Jobs::Live(listed) => {
                #[derive(Serialize)]
                struct Live<'a> {
                    running: bool,
                    #[serde(flatten)]
                    listed: &'a Listed,
                }
                Live {
                    running: true,
                    listed,
                }
                .serialize(serializer)
            }
            Jobs::Absent { persisted } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("running", &false)?;
                map.serialize_entry("persisted", persisted)?;
                map.end()
            }
        }
    }
}

impl Serialize for Job {
    /// Live: the job as the daemon reads it. Absent: `running: false`,
    /// the ledger's row and `next`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Job::Live(job) => job.serialize(serializer),
            Job::Absent { persisted, next } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("running", &false)?;
                map.serialize_entry("persisted", persisted)?;
                map.serialize_entry("next", next)?;
                map.end()
            }
        }
    }
}

/// C45 and C47 (no panic on input): a ledger row in the live list's
/// shape, for one table — no ETA (nothing is predicting), the state as
/// recorded. A state this build does not know is a fault naming the row.
pub fn job_info(job: &PersistedJob) -> Result<JobInfo, Fault> {
    let state = match job.state.as_str() {
        "waiting" => JobState::Waiting,
        "running" => JobState::Running,
        "done" => JobState::Done,
        "failed" => JobState::Failed,
        "cancelled" => JobState::Cancelled,
        other => {
            return Err(Fault::Other(anyhow::anyhow!(
                "job {} on disk has state `{other}`, which this build does not know",
                job.id
            )));
        }
    };
    Ok(JobInfo {
        id: job.id,
        kind: job.kind.clone(),
        state,
        priority: job.priority,
        submitted_by: job.submitted_by.clone(),
        eta_seconds: None,
        parent: job.parent,
        retries: job.retries,
        account: job.account.clone(),
        params: job.params.clone(),
    })
}

/// C4: a terminal job's outcome from the daemon, with its account where
/// the daemon still lists the job.
pub async fn result_live(client: &mut Client, id: u64) -> Result<JobResult, Fault> {
    let outcome = outcome_live(client, id).await?;
    // A previous lifetime's job has no status; its outcome stands alone.
    let account = client.status(id).await.ok().and_then(|job| job.account);
    let failed_children = failed_children_live(client, &outcome).await;
    Ok(JobResult {
        job_id: id,
        account,
        outcome,
        failed_children,
    })
}

/// C23 and C45: a parent failure's first failed children, read through
/// the daemon, never the ledger while it answers — their results across
/// restarts, their kind and target while it still holds them. Empty for
/// any other outcome.
pub async fn failed_children_live(client: &mut Client, outcome: &Outcome) -> Vec<FailedChild> {
    let mut rows = Vec::new();
    for &cid in detailed(outcome) {
        let error = child_error(outcome_live(client, cid).await);
        let (kind, target) = match client.status(cid).await {
            Ok(job) => (job.kind.clone(), job.target()),
            Err(_) => (String::new(), String::new()),
        };
        rows.push(FailedChild {
            job_id: cid,
            kind,
            target,
            error,
        });
    }
    rows
}

/// C4: a terminal job's outcome from the daemon alone; a job not yet
/// terminal is the daemon's `wrong_state` refusal (C85).
pub async fn outcome_live(client: &mut Client, id: u64) -> Result<Outcome, Fault> {
    match client
        .request(&Request::Result { id })
        .await
        .map_err(Fault::from_client)?
    {
        Response::Result { outcome, .. } => Ok(outcome),
        Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
        other => Err(unexpected(other)),
    }
}

impl Frontend {
    /// C45 (the ledger answers) and C10: the queue, live, or from the
    /// ledger on disk when no daemon answers; never spawns. Ten open and
    /// ten finished rows beside the counts of every job, every one under
    /// `expand` (C53, C149).
    pub async fn jobs(&self, expand: bool) -> Result<Jobs, Fault> {
        match self.observe().await? {
            Some(mut client) => Ok(Jobs::Live(Listed::of(list(&mut client).await?, expand))),
            None => Ok(Jobs::Absent {
                persisted: self.persisted()?.map(|p| PersistedJobs::of(p, expand)),
            }),
        }
    }

    /// C45 and C10: one job, live, or the ledger's row with `next` when
    /// no daemon answers; [`Fault::NoJobOnDisk`] when neither holds it.
    pub async fn job(&self, id: u64) -> Result<Job, Fault> {
        match self.observe().await? {
            Some(mut client) => Ok(Job::Live(
                client.status(id).await.map_err(Fault::from_client)?,
            )),
            None => self.job_on_disk(id),
        }
    }

    /// C45: one job as the ledger on disk records it.
    pub fn job_on_disk(&self, id: u64) -> Result<Job, Fault> {
        let record = self.record(id)?;
        Ok(Job::Absent {
            persisted: record.job,
            next: self.next(),
        })
    }

    /// C45 and C10: a terminal job's outcome, live, or from the ledger
    /// when no daemon answers, with its id and account beside it.
    pub async fn result(&self, id: u64) -> Result<JobResult, Fault> {
        match self.observe().await? {
            Some(mut client) => result_live(&mut client, id).await,
            None => self.result_on_disk(id),
        }
    }

    /// C45: a terminal job's outcome as the ledger on disk holds it,
    /// which is final; an unfinished job is [`Fault::Unfinished`].
    pub fn result_on_disk(&self, id: u64) -> Result<JobResult, Fault> {
        let record = self.record(id)?;
        let Some(outcome) = record.outcome else {
            return Err(Fault::Unfinished {
                id,
                state: record.job.state,
                next: self.next(),
            });
        };
        let outcome: Outcome = serde_json::from_value(outcome).map_err(|e| {
            Fault::Other(anyhow::anyhow!(
                "job {id}'s outcome on disk does not parse: {e}"
            ))
        })?;
        let failed_children = detailed(&outcome)
            .iter()
            .map(|&cid| match self.record(cid) {
                Ok(child) => FailedChild {
                    job_id: cid,
                    target: target_of(&child.job.kind, &child.job.params),
                    kind: child.job.kind,
                    error: match disk_outcome(child.outcome) {
                        Ok(outcome) => child_error(Ok(outcome)),
                        Err(said) => said,
                    },
                },
                Err(fault) => FailedChild {
                    job_id: cid,
                    kind: String::new(),
                    target: String::new(),
                    error: child_error(Err(fault)),
                },
            })
            .collect();
        Ok(JobResult {
            job_id: id,
            account: record.job.account,
            outcome,
            failed_children,
        })
    }

    /// C45: the ledger's open rows in this world, `None` when it has no
    /// queue yet. Read-only, never while a daemon answers.
    pub fn persisted(&self) -> Result<Option<Persisted>, Fault> {
        Persisted::observe_at(&self.root, self.provider).map_err(Fault::Unopened)
    }

    fn record(&self, id: u64) -> Result<PersistedRecord, Fault> {
        Persisted::observe_job_at(&self.root, self.provider, id)
            .map_err(Fault::Unopened)?
            .ok_or_else(|| Fault::NoJobOnDisk {
                id,
                next: self.next(),
            })
    }
}

/// An answer of a shape the request does not take.
pub(crate) fn unexpected(response: Response) -> Fault {
    Fault::Transport(anyhow::anyhow!("unexpected response: {response:?}"))
}
