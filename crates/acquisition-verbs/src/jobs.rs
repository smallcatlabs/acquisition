//! The job verbs (C4, API requests are jobs): submit, wait, and the
//! queue's reads, live or from the ledger on disk when no daemon answers
//! (C45, the ledger answers). An observer never spawns (C10); a job verb
//! that submits passes its account selector through to the daemon
//! unresolved, so a job verb with no login still spawns and is refused by
//! the daemon (C3).

use std::time::Duration;

use acquisition_client::client::{Client, DaemonError};
use acquisition_protocol::job::{
    CLIENT_KINDS, FailureKind, JobInfo, JobState, JobTimes, MOCK_ONLY_KINDS, Outcome, target_of,
};
use acquisition_protocol::protocol::{
    ErrorKind, LIST_LIMIT_MAX, ListCounts, ListState, Request, Response,
};
use acquisition_store::bound::{LISTED, bounded};
use acquisition_store::jobs::{Persisted, PersistedJob, PersistedRecord};
use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};
use serde_json::Value;

use crate::fault::Fault;
use crate::frontend::Frontend;
use crate::refusal::{Surface, Way, offer};

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

/// C166: one page of the daemon's job list, as it answered.
#[derive(Clone, Debug)]
pub struct Page {
    /// Newest first.
    pub jobs: Vec<JobInfo>,
    /// The id after which the next page starts; `None` on the last.
    pub next: Option<u64>,
    /// Over every job of the list, whatever the page's state.
    pub counts: ListCounts,
}

/// C166: one page of `state`, at most `limit` jobs below `after`.
pub async fn page(
    client: &mut Client,
    state: ListState,
    limit: usize,
    after: Option<u64>,
) -> Result<Page, Fault> {
    let request = Request::List {
        state,
        limit,
        after,
    };
    match client.request(&request).await.map_err(Fault::from_client)? {
        Response::Jobs { jobs, next, counts } => Ok(Page { jobs, next, counts }),
        Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
        other => Err(unexpected(other)),
    }
}

/// C166, the page loop: the jobs of `state`, newest first, read a page
/// of the most the daemon serves at a time, each resumed after the last,
/// until the last page or until `enough` says the jobs read so far
/// suffice; with the counts of the last page read. Every reader that
/// needs more than one first page reads through here.
pub async fn pages(
    client: &mut Client,
    state: ListState,
    mut enough: impl FnMut(&[JobInfo]) -> bool,
) -> Result<(Vec<JobInfo>, ListCounts), Fault> {
    let mut jobs = Vec::new();
    let mut after = None;
    loop {
        let page = page(client, state, LIST_LIMIT_MAX, after).await?;
        jobs.extend(page.jobs);
        match page.next {
            Some(next) if !enough(&jobs) => after = Some(next),
            _ => return Ok((jobs, page.counts)),
        }
    }
}

/// C166: every job of `state`, newest first ([`pages`] to the last).
pub async fn every(client: &mut Client, state: ListState) -> Result<Vec<JobInfo>, Fault> {
    Ok(pages(client, state, |_| false).await?.0)
}

/// A parent's children, counted from the daemon's list. A child's id is
/// above its parent's (it was submitted after it), so the pages, newest
/// first, are read until one reaches below the parent.
async fn tally(client: &mut Client, parent: u64) -> Result<ChildTally, Fault> {
    let mut tally = ChildTally::default();
    let below = |jobs: &[JobInfo]| jobs.last().is_some_and(|job| job.id < parent);
    for job in pages(client, ListState::All, below).await?.0 {
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

/// C165 and C163 (every time a document carries has its words beside
/// it): a job's times, each that happened said at `now`, the answer's
/// time, by the store's one formatter, as [`PersistedJob::aged`] says a
/// ledger row's. The daemon sends the times; a frontend says them.
pub fn said(mut times: JobTimes, now: i64) -> JobTimes {
    let words = |at: Option<i64>| at.map(|at| acquisition_store::age::ago(now, Some(at)));
    times.submitted_at_age = words(times.submitted_at);
    times.started_at_age = words(times.started_at);
    times.finished_at_age = words(times.finished_at);
    times
}

/// A job as a document carries it: its times said at `now` ([`said`]).
pub fn aged(mut job: JobInfo, now: i64) -> JobInfo {
    job.times = said(job.times, now);
    job
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
    /// The finished jobs a daemon holds, a live count: the ledger records
    /// none (C146).
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
    /// Every job the list holds: on the live list, every job this daemon
    /// holds, those submitted to it and those it restored alike.
    pub total: usize,
    pub by_state: ByState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_starts_in_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_starts_in_seconds: Option<u64>,
}

impl From<&ListCounts> for JobCounts {
    /// C166: the daemon's counts over its whole list, as the document
    /// carries them.
    fn from(counts: &ListCounts) -> JobCounts {
        let n = |count: u64| usize::try_from(count).unwrap_or(usize::MAX);
        JobCounts {
            total: n(counts.total),
            by_state: ByState {
                running: n(counts.running),
                waiting: n(counts.waiting),
                done: n(counts.done),
                failed: n(counts.failed),
                cancelled: n(counts.cancelled),
            },
            next_starts_in_seconds: counts.soonest_eta_seconds,
            last_starts_in_seconds: counts.latest_eta_seconds,
        }
    }
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
/// every job ([`JobCounts`], the daemon's, C166), then `rows`, in id
/// order: the ten newest open jobs (running or waiting) and the ten
/// newest finished, or of one of them when the view is narrowed to it;
/// every job of the view under `expand` (the CLI's `--all`). What the
/// view leaves out is counted, open and finished apart (the brief's
/// second review, finding 2: no exemption for the open jobs): the
/// earliest of each.
#[derive(Clone, Debug, Serialize)]
pub struct Listed {
    #[serde(flatten)]
    pub counts: JobCounts,
    pub rows: Vec<JobInfo>,
    /// Open jobs of the view not in `rows`: the earliest.
    pub open_left_out: usize,
    /// Finished jobs of the view not in `rows`: the earliest.
    pub finished_left_out: usize,
    /// What lists them all: the request's `expand`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_out_needs: Option<&'static str>,
}

impl Listed {
    /// C166: the view of the daemon's list `view` names (`all`, both
    /// open and finished; `open` or `finished` alone), read as pages: the
    /// first page of ten of each state the view holds, or under `expand`
    /// every page of the view.
    pub async fn read(client: &mut Client, view: ListState, expand: bool) -> Result<Listed, Fault> {
        if expand {
            let (rows, counts) = pages(client, view, |_| false).await?;
            return Ok(Listed::of(rows, &counts, view));
        }
        let mut rows = Vec::new();
        let mut counts = ListCounts::default();
        for state in [ListState::Open, ListState::Finished] {
            if view == state || view == ListState::All {
                let page = page(client, state, LISTED, None).await?;
                rows.extend(page.jobs);
                counts = page.counts;
            }
        }
        Ok(Listed::of(rows, &counts, view))
    }

    /// The document of `rows` read from a view of the list and the
    /// daemon's `counts` beside them: the rows by id, each one's times
    /// said now ([`aged`], C165), and what the view holds that they do
    /// not, by the counts.
    pub fn of(rows: Vec<JobInfo>, counts: &ListCounts, view: ListState) -> Listed {
        let now = acquisition_store::now();
        let mut rows: Vec<JobInfo> = rows.into_iter().map(|job| aged(job, now)).collect();
        rows.sort_by_key(|j| j.id);
        let counts = JobCounts::from(counts);
        let by = &counts.by_state;
        let shown = |finished: bool| {
            rows.iter()
                .filter(|j| j.state.is_terminal() == finished)
                .count()
        };
        let left_out = |state: ListState, held: usize, finished: bool| {
            if view == state || view == ListState::All {
                held.saturating_sub(shown(finished))
            } else {
                0
            }
        };
        let open_left_out = left_out(ListState::Open, by.running + by.waiting, false);
        let finished_left_out = left_out(
            ListState::Finished,
            by.done + by.failed + by.cancelled,
            true,
        );
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
/// store's rows, ten, every one under `expand`, each row's times in words
/// as one job's are, C163), `written_at`, and the
/// ledger's counts over every open row: `total`, `waiting`,
/// `recorded_running`, the words `daemon_status` uses, and `left_out`.
#[derive(Clone, Debug, Serialize)]
pub struct PersistedJobs {
    pub jobs: Vec<PersistedJob>,
    pub written_at: Option<i64>,
    /// C53: `written_at` in words at the answer's time (`3m ago`, `never`
    /// for a ledger with no rows): how old this picture is.
    pub written_at_age: String,
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
    /// The ledger's open rows, counted whole and then cut by the one rule,
    /// its last write and each listed row's times said as ages at `now`,
    /// the answer's time (C53, C163).
    pub fn of(persisted: Persisted, expand: bool, now: i64) -> PersistedJobs {
        let (total, waiting, recorded_running) = (
            persisted.jobs.len(),
            persisted.waiting(),
            persisted.recorded_running(),
        );
        let (jobs, left_out) = bounded(persisted.jobs, None, expand);
        PersistedJobs {
            jobs: jobs.into_iter().map(|job| job.aged(now)).collect(),
            written_at: persisted.written_at,
            written_at_age: acquisition_store::age::ago(now, persisted.written_at),
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
    /// C165: when the job was submitted, started and finished, each with
    /// its words (C163); absent where the daemon or the row has none.
    #[serde(flatten)]
    pub times: JobTimes,
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
    /// C165 and C143: a failure's way on as the surface that answers
    /// sends it back, each `{request, says}` ([`JobResult::offered`]);
    /// none for a kind whose way on sends nothing, or a failure from
    /// before the kinds.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<Value>,
    /// The way on before a surface spells it ([`way_on`]).
    #[serde(skip)]
    pub way_on: Option<WayOn>,
}

impl JobResult {
    /// The result with its way on spelled by `surface` as `offers`.
    pub fn offered(mut self, surface: &dyn Surface) -> JobResult {
        if let Some(way_on) = &self.way_on {
            self.offers = way_on
                .ways
                .iter()
                .filter_map(|(way, says)| offer(way, says, surface))
                .collect();
        }
        self
    }
}

/// C165: a failed job's way on — the words every surface shares, and
/// the ways each spells in its own terms (C143).
#[derive(Clone, Debug, PartialEq)]
pub struct WayOn {
    /// What the failure leaves to do, in words: the reason the ways are
    /// offered, or, where none is, why.
    pub words: String,
    pub ways: Vec<(Way, String)>,
}

/// C165 (a failed job says what kind of failure it is): a failure's way
/// on by its kind, from the failed job's own `kind` and `params` where
/// the reader holds them. A 404's is the fetch of the listing that names
/// what was asked for; a refusal or fault of GGG's, a 429 that ended the
/// job and an interruption, the same job again; a block, and a 403 or
/// 503 never retried, nothing that sends (invariant 3); a body not
/// recorded, the result
/// that keeps it; a parent, each failed child's result; a session that
/// cannot give a token, a login, a person's. None for a failure with no
/// kind (one from before the field), a success or a cancellation.
pub fn way_on(
    outcome: &Outcome,
    id: u64,
    account: Option<&str>,
    job: Option<(&str, &Value)>,
    failed: &[FailedChild],
) -> Option<WayOn> {
    let Outcome::Failure {
        kind: Some(kind), ..
    } = outcome
    else {
        return None;
    };
    let submit = |kind: &str, params: Value| Way::Submit {
        account: account.map(str::to_string),
        kind: kind.to_string(),
        params,
    };
    // the same job again, where it is one a client may submit
    let again = |says: &str| -> Vec<(Way, String)> {
        job.filter(|(kind, _)| CLIENT_KINDS.contains(kind) || MOCK_ONLY_KINDS.contains(kind))
            .map(|(kind, params)| vec![(submit(kind, params.clone()), says.to_string())])
            .unwrap_or_default()
    };
    // the listing a 404's target is named in, its realm and league kept
    let listing = || -> Vec<(Way, String)> {
        let Some((kind, params)) = job else {
            return Vec::new();
        };
        let kept = |keys: &[&str]| -> Value {
            Value::Object(
                keys.iter()
                    .filter_map(|k| Some((k.to_string(), params.get(*k)?.clone())))
                    .collect(),
            )
        };
        match kind {
            // a client's stash params carry `sub: null` for a tab
            "stash" if params.get("sub").is_some_and(|sub| !sub.is_null()) => vec![(
                submit("stash", kept(&["realm", "league", "id"])),
                "the parent tab's fetch, which lists its substashes again".into(),
            )],
            "stash" => vec![(
                submit("stashes", kept(&["realm", "league"])),
                "the league's listing, which names its tabs again".into(),
            )],
            "character" => vec![(
                submit("characters", kept(&["realm"])),
                "the character list, which names the characters again".into(),
            )],
            _ => Vec::new(),
        }
    };
    let (words, ways) = match kind {
        FailureKind::NotFound => (
            "GGG has nothing there now; fetch what lists it again",
            listing(),
        ),
        FailureKind::Upstream => (
            "GGG's answer could not be used; submit the same job again later",
            again("the same job again, later"),
        ),
        FailureKind::RateLimited => (
            "a rate limit ended it; submit the same job again once the limiter's hold has passed",
            again("the same job again, once the limiter's hold has passed"),
        ),
        FailureKind::Blocked => (
            "do not submit it again, nor work like it: Cloudflare answered in GGG's place, \
             which is never retried, and the daemon sends nothing until a person clears it",
            Vec::new(),
        ),
        FailureKind::NotRetried => (
            "GGG answered 403 or 503, which is never retried: do not submit it again until a \
             person has looked; a 403 that names an authorization error asks for a login, a \
             person's (`acq auth`)",
            Vec::new(),
        ),
        FailureKind::NotRecorded => (
            "what it fetched is kept with its result",
            vec![(
                Way::Result { id },
                "the result, which keeps what it fetched".into(),
            )],
        ),
        FailureKind::Children => (
            "its children failed; each failed child's own result says why",
            failed
                .iter()
                .map(|child| {
                    (
                        Way::Result { id: child.job_id },
                        format!("failed child {}'s own result", child.job_id),
                    )
                })
                .collect(),
        ),
        FailureKind::NotLoggedIn => (
            "its account has no session to send with: a login, which is a person's (`acq auth`)",
            Vec::new(),
        ),
        FailureKind::Interrupted => (
            "a daemon's end stopped it; submit the same job again",
            again("the same job again"),
        ),
        FailureKind::Internal => ("the daemon's own fault; nothing is offered", Vec::new()),
    };
    Some(WayOn {
        words: words.to_string(),
        ways,
    })
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
        times: times_of(job),
    })
}

/// C165: a ledger row's times as a job's record carries them, with the
/// words the row was said with.
fn times_of(job: &PersistedJob) -> JobTimes {
    JobTimes {
        submitted_at: Some(job.submitted_at),
        submitted_at_age: job.submitted_at_age.clone(),
        started_at: job.started_at,
        started_at_age: job.started_at_age.clone(),
        finished_at: job.finished_at,
        finished_at_age: job.finished_at_age.clone(),
    }
}

/// C4: a terminal job's outcome from the daemon, with its account where
/// the daemon still lists the job.
pub async fn result_live(client: &mut Client, id: u64) -> Result<JobResult, Fault> {
    let (outcome, times) = answer_live(client, id).await?;
    // A previous lifetime's job has no status; its outcome stands alone.
    let job = client.status(id).await.ok();
    let account = job.as_ref().and_then(|job| job.account.clone());
    let failed_children = failed_children_live(client, &outcome).await;
    let way_on = way_on(
        &outcome,
        id,
        account.as_deref(),
        job.as_ref().map(|job| (job.kind.as_str(), &job.params)),
        &failed_children,
    );
    Ok(JobResult {
        job_id: id,
        account,
        outcome,
        times: said(times, acquisition_store::now()),
        failed_children,
        offers: Vec::new(),
        way_on,
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
    Ok(answer_live(client, id).await?.0)
}

/// [`outcome_live`] with the job's times as the daemon answers them
/// (C165), unsaid.
async fn answer_live(client: &mut Client, id: u64) -> Result<(Outcome, JobTimes), Fault> {
    match client
        .request(&Request::Result { id })
        .await
        .map_err(Fault::from_client)?
    {
        Response::Result { outcome, times, .. } => Ok((outcome, times)),
        Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
        other => Err(unexpected(other)),
    }
}

/// A cancel the daemon took (C4, jobs not calls): the job, and that its
/// cancel is asked; a running job stops at its next check, which its
/// result says. The one document both frontends answer a cancel with.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CancelRequested {
    pub job_id: u64,
    pub cancel_requested: bool,
}

impl Frontend {
    /// C10: a cancel sent to the daemon running, cascading to the job's
    /// descendants; with none running, [`Fault::NoDaemon`].
    pub async fn cancel(&self, id: u64) -> Result<CancelRequested, Fault> {
        let mut client = self.attach().await?;
        match client
            .request(&Request::Cancel { id })
            .await
            .map_err(Fault::from_client)?
        {
            Response::Ack { .. } => Ok(CancelRequested {
                job_id: id,
                cancel_requested: true,
            }),
            Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
            other => Err(unexpected(other)),
        }
    }

    /// C45 (the ledger answers) and C10: the queue, live, or from the
    /// ledger on disk when no daemon answers; never spawns. The ten
    /// newest open and ten newest finished rows of the view beside the
    /// counts of every job, every one of the view under `expand` (C53,
    /// C149, C166). The ledger holds open rows alone, so with no daemon
    /// the view does not narrow them.
    pub async fn jobs(&self, view: ListState, expand: bool) -> Result<Jobs, Fault> {
        match self.observe().await? {
            Some(mut client) => Ok(Jobs::Live(Listed::read(&mut client, view, expand).await?)),
            None => Ok(Jobs::Absent {
                persisted: self
                    .persisted()?
                    .map(|p| PersistedJobs::of(p, expand, acquisition_store::now())),
            }),
        }
    }

    /// C45 and C10: one job, live, or the ledger's row with `next` when
    /// no daemon answers; [`Fault::NoJobOnDisk`] when neither holds it.
    pub async fn job(&self, id: u64) -> Result<Job, Fault> {
        match self.observe().await? {
            Some(mut client) => Ok(Job::Live(aged(
                client.status(id).await.map_err(Fault::from_client)?,
                acquisition_store::now(),
            ))),
            None => self.job_on_disk(id),
        }
    }

    /// C45: one job as the ledger on disk records it, its last write said
    /// as an age at the answer's time (C53), as the text says it.
    pub fn job_on_disk(&self, id: u64) -> Result<Job, Fault> {
        let record = self.record(id)?;
        Ok(Job::Absent {
            persisted: record.job.aged(acquisition_store::now()),
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
        let failed_children: Vec<FailedChild> = detailed(&outcome)
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
        let job = record.job.aged(acquisition_store::now());
        let way_on = way_on(
            &outcome,
            id,
            job.account.as_deref(),
            Some((job.kind.as_str(), &job.params)),
            &failed_children,
        );
        Ok(JobResult {
            job_id: id,
            outcome,
            times: times_of(&job),
            account: job.account,
            failed_children,
            offers: Vec::new(),
            way_on,
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
