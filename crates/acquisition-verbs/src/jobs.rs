//! The job verbs (C4, API requests are jobs): submit, wait, and the
//! queue's reads, live or from the ledger on disk when no daemon answers
//! (C45, the ledger answers). An observer never spawns (C10); a job verb
//! that submits passes its account selector through to the daemon
//! unresolved, so a job verb with no login still spawns and is refused by
//! the daemon (C3).

use std::time::Duration;

use acquisition_client::client::{Client, DaemonError};
use acquisition_protocol::job::{JobInfo, JobState, Outcome};
use acquisition_protocol::protocol::{Request, Response};
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
            let children = if matches!(job.kind.as_str(), "apply" | "refresh")
                && job.state != JobState::Waiting
            {
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
/// rows (C45). `persisted` is `None` when this world has no queue yet.
#[derive(Clone, Debug)]
pub enum Jobs {
    Live(Vec<JobInfo>),
    Absent { persisted: Option<Persisted> },
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
}

impl Serialize for Jobs {
    /// Live: the job list as the daemon gives it. Absent: `running:
    /// false` and the persisted queue.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Jobs::Live(jobs) => jobs.serialize(serializer),
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
    Ok(JobResult {
        job_id: id,
        account,
        outcome,
    })
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
    /// ledger on disk when no daemon answers; never spawns.
    pub async fn jobs(&self) -> Result<Jobs, Fault> {
        match self.observe().await? {
            Some(mut client) => Ok(Jobs::Live(list(&mut client).await?)),
            None => Ok(Jobs::Absent {
                persisted: self.persisted()?,
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
        Ok(JobResult {
            job_id: id,
            account: record.job.account,
            outcome,
        })
    }

    /// C45: the ledger's open rows in this world, `None` when it has no
    /// queue yet. Read-only, never while a daemon answers.
    pub fn persisted(&self) -> Result<Option<Persisted>, Fault> {
        Persisted::observe(self.provider).map_err(Fault::Unopened)
    }

    fn record(&self, id: u64) -> Result<PersistedRecord, Fault> {
        Persisted::observe_job(self.provider, id)
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
