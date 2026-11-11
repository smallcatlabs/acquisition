//! The job verbs' text: submitting a job and waiting on it, its progress
//! lines, its report and outcome, and the jobs table, live and persisted
//! (C45).
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.

use std::io::{IsTerminal as _, Write as _};
use std::time::{Duration, Instant};

use acquisition_client::client::{Client, Observed, Signal, Subscription};
use acquisition_protocol::job::{FailureKind, JobInfo, JobState, JobTimes, Outcome};
use acquisition_protocol::protocol::ListState;
use acquisition_verbs::jobs::{job_info, result_live};
use acquisition_verbs::{
    ChildTally, FailedChild, Fault, Job, JobCounts, JobResult, Jobs, Listed, Progress,
};
use anyhow::{Result, bail};

use crate::run::{Ctx, attach};
use crate::store_cmd;
use crate::terminal::{AlreadyReported, Terminal, plural};

/// `jobs` with no daemon (C45): `running: false` and the persisted queue
/// — the same table as the live list, without ETAs, under a line that
/// says how old the picture is and what restores it.
pub(crate) fn print_persisted_queue(jobs: &Jobs, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(jobs)?);
        return Ok(());
    }
    print!("{}", persisted_queue_text(jobs)?);
    Ok(())
}

/// The persisted queue as text (C45): what `jobs` says with no daemon,
/// and the web UI's jobs page with it.
pub(crate) fn persisted_queue_text(jobs: &Jobs) -> Result<String> {
    let Jobs::Absent { persisted } = jobs else {
        return Ok(String::new());
    };
    let spawns = "a job command or `acq auth` starts one";
    Ok(match persisted {
        None => format!("daemon is not running; no queue on disk ({spawns})\n"),
        Some(p) if p.total == 0 => {
            format!("daemon is not running; nothing waits on disk ({spawns})\n")
        }
        Some(p) => {
            let jobs = p
                .jobs
                .iter()
                .map(job_info)
                .collect::<Result<Vec<_>, Fault>>()?;
            let mut text = format!(
                "daemon is not running; on disk: {} waiting, {} recorded running, written {} — \
                 a job command spawns a successor that restores them, and `acq cancel <id>` \
                 then acts on them\n{}{}",
                p.waiting,
                p.recorded_running,
                store_cmd::ago(acquisition_store::now(), p.written_at),
                rows_text(&jobs),
                summary_text(&p.counts(), Counted::Ledger)
            );
            if p.left_out > 0 {
                text.push_str(&format!(
                    "{} more waiting or recorded running on disk not listed (`acq jobs --all`)\n",
                    p.left_out
                ));
            }
            text
        }
    })
}

/// `status <id>` with no daemon: the row as recorded, in `jobs`' absent
/// shape — `running: false` beside what the ledger holds, and `next`.
pub(crate) fn print_persisted_job(id: u64, job: &Job, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(job)?);
        return Ok(());
    }
    let Job::Absent { persisted, .. } = job else {
        return Ok(());
    };
    println!(
        "daemon is not running; job {id} as the ledger on disk records it, written {}",
        store_cmd::ago(acquisition_store::now(), Some(persisted.updated_at)),
    );
    let job = job_info(persisted)?;
    print_table(std::slice::from_ref(&job));
    if let Some(words) = times_said(&job.times) {
        println!("job {id}: {words}");
    }
    Ok(())
}

/// C3 and C4: submit a job as this process, the account flag passed to
/// the daemon unresolved.
pub(crate) async fn submit(
    cx: &Ctx,
    client: &mut Client,
    kind: String,
    params: serde_json::Value,
    priority: u8,
) -> Result<u64> {
    Ok(acquisition_verbs::jobs::submit(
        &cx.frontend,
        client,
        &kind,
        params,
        priority,
        cx.account.as_deref(),
    )
    .await?)
}

/// `jobs --watch`: the subscriber's sequence under C85, as the reference
/// consumer. Subscribe first, then take the snapshot over a request
/// connection to the same daemon instance (a restart landing between the
/// two connections starts the sequence over); a job that changes between
/// subscribing and reading is an event, never a gap. Every event is an
/// invalidation, so the job is re-read over the request connection before
/// its line is printed — an event never stands in for the read, and a
/// read that fails restarts the sequence rather than trusting the hint.
/// On `resync_required` the lagged subscription is dropped — its queued
/// events predate any snapshot taken now — and the sequence starts over
/// with a fresh subscription and snapshot; when the daemon goes, observe
/// it again — an observer never spawns — and subscribe and snapshot
/// afresh if it is back. Pinned by `tests/watch_recovery.rs`.
pub(crate) async fn watch_jobs(cx: &Ctx, json: bool, view: ListState, all: bool) -> Result<()> {
    // each snapshot the `jobs` document (C149), the newest ten open and
    // ten finished beside the counts of every job, every one under
    // `--all` (C53, C166)
    let print_snapshot = |listed: Listed| -> Result<()> {
        if json {
            println!("{}", serde_json::to_string_pretty(&Jobs::Live(listed))?);
        } else {
            print!("{}", jobs_text(&listed));
        }
        Ok(())
    };
    loop {
        let mut subscription = match Subscription::observe_at(&cx.frontend.root).await? {
            Observed::Compatible(subscription) => subscription,
            Observed::Absent => bail!("daemon stopped (it spawns on demand for job commands)"),
            Observed::Incompatible(found) => bail!("{found}"),
            Observed::Unresponsive(found) => bail!("{found}"),
        };
        let mut client = attach(cx).await?;
        if client.daemon().pid() != subscription.daemon().pid() {
            // A restart between the two connections: the subscription is
            // to a daemon that is gone. Start over.
            continue;
        }
        print_snapshot(Listed::read(&mut client, view, all).await?)?;
        loop {
            let signal = match subscription.next().await {
                Ok(signal) => signal,
                // A transport failure mid-stream is a disconnect; anything
                // else is a protocol violation worth stopping on.
                Err(e) if e.downcast_ref::<std::io::Error>().is_some() => None,
                Err(e) => return Err(e),
            };
            match signal {
                Some(Signal::Event(hint)) => {
                    // The read, never the hint, is what gets printed; a
                    // read that fails restarts the sequence.
                    let job = match client.status(hint.id).await {
                        Ok(job) => job,
                        Err(e) => {
                            if !json {
                                println!(
                                    "re-reading job {} failed ({e:#}); subscribing and reading again",
                                    hint.id
                                );
                            }
                            break;
                        }
                    };
                    if json {
                        // its times said, as every job document says them (C165)
                        let job = acquisition_verbs::jobs::aged(job, acquisition_store::now());
                        println!("{}", serde_json::to_string(&job)?);
                    } else {
                        println!("job {:>3}  {:<8} -> {}", job.id, job.kind, job.state);
                    }
                }
                Some(Signal::ResyncRequired { missed }) => {
                    if !json {
                        println!("missed {missed} event(s); subscribing and reading again");
                    }
                    break;
                }
                None => {
                    if !json {
                        println!("daemon went away; watching for it");
                    }
                    break;
                }
            }
        }
    }
}

/// Default CLI mode: block with progress until the job finishes, then print
/// its result. This is where "rate limited, retrying in 4m37s..." UX lives.
pub(crate) async fn block_on_job(client: &mut Client, id: u64, json: bool) -> Result<()> {
    wait_for_job(client, id, json).await?;
    print_result(client, id, json, true).await
}

/// Progress is redrawn in place only on a tty; captured output (the
/// driver's `.out` files, an agent's shell) gets a plain line per change,
/// at most every this often — a 13-minute apply is not 1,500 lines.
const PROGRESS_EVERY: Duration = Duration::from_secs(10);

/// Block until the job is terminal and return its outcome, through the
/// layer's wait (C4). Progress goes to stderr: stdout is the answer and
/// nothing else, in text mode as in JSON, so what a caller captures is
/// what the job returned. `quiet` skips the progress line.
pub(crate) async fn wait_for_job(client: &mut Client, id: u64, quiet: bool) -> Result<Outcome> {
    let mut progress = Progressor::new();
    let mut report = |p: &Progress| progress.report(p);
    let outcome = acquisition_verbs::jobs::wait(
        client,
        id,
        (!quiet).then_some(&mut report as &mut (dyn FnMut(&Progress) + Send)),
    )
    .await?;
    Ok(outcome)
}

/// The progress line's state: redrawn in place on a tty, a plain line per
/// change elsewhere, at most every [`PROGRESS_EVERY`].
pub(crate) struct Progressor {
    tty: bool,
    last: String,
    last_print: Option<Instant>,
}

impl Progressor {
    pub(crate) fn new() -> Progressor {
        Progressor {
            tty: std::io::stderr().is_terminal(),
            last: String::new(),
            last_print: None,
        }
    }

    /// One reading of the job: printed when it changed and is due, and a
    /// tty's line ended once the job is terminal.
    pub(crate) fn report(&mut self, p: &Progress) {
        let terminal = p.job.state.is_terminal();
        let mut line = progress_line(p);
        // a job's end says its times (C165), said now from the daemon's
        if terminal {
            let times =
                acquisition_verbs::jobs::said(p.job.times.clone(), acquisition_store::now());
            if let Some(words) = times_said(&times) {
                line = format!("{line}; {words}");
            }
        }
        let due = self
            .last_print
            .is_none_or(|t| t.elapsed() >= PROGRESS_EVERY);
        if line != self.last && (self.tty || terminal || due) {
            if self.tty {
                eprint!("\r\x1b[2K{line}");
                std::io::stderr().flush().ok();
            } else {
                eprintln!("{line}");
            }
            self.last = line;
            self.last_print = Some(Instant::now());
        }
        if terminal && self.tty {
            eprintln!();
        }
    }
}

/// One line of progress: a parent reports its children (`30/112 done, 82
/// waiting, next in ~343s (limiter hold on stash)`), any other job its
/// own state and ETA.
fn progress_line(p: &Progress) -> String {
    let job = &p.job;
    // The account the job runs as, on every line: with several accounts
    // known a job verb takes the one logged in (C51), and the caller
    // should see which that was before reading what came back.
    let id = match &job.account {
        Some(account) => format!("{} ({account})", job.id),
        None => job.id.to_string(),
    };
    if let Some(t) = p.children.as_ref().filter(|t| t.total > 0) {
        return children_line(&id, t);
    }
    match (job.state, job.eta_seconds) {
        (JobState::Waiting, Some(eta)) if eta > 0 && job.retries > 0 => {
            format!("job {id}: got a 429, retry {} in ~{eta}s...", job.retries)
        }
        (JobState::Waiting, Some(eta)) if eta > 0 => {
            format!("job {id}: rate limited, starting in ~{eta}s...")
        }
        (state, _) => format!("job {id}: {state}"),
    }
}

/// A parent's line: its children counted, the next limiter hold named.
fn children_line(id: &str, t: &ChildTally) -> String {
    let mut line = format!("job {id}: {}/{} done", t.done, t.total);
    if t.failed > 0 {
        line.push_str(&format!(", {} failed", t.failed));
    }
    if t.cancelled > 0 {
        line.push_str(&format!(", {} cancelled", t.cancelled));
    }
    if t.running > 0 {
        line.push_str(&format!(", {} running", t.running));
    }
    if t.waiting > 0 {
        line.push_str(&format!(", {} waiting", t.waiting));
    }
    if let Some((eta, kind)) = &t.next {
        line.push_str(&format!(", next in ~{eta}s (limiter hold on {kind})"));
    }
    line
}

/// A finished job in text (rule 4 of the legibility ruling), the one
/// renderer every text path calls: an apply's success as one line with
/// its child range; a failure in the daemon's own words, then one row
/// per failed child the layer detailed (C23) — its id, kind and target,
/// its own error — and, when the tally holds more ids than rows, where
/// every id is listed. The CLI composes no count of its own and fetches
/// nothing: the sentence is the daemon's, the rows the layer's.
pub(crate) fn job_report(id: u64, outcome: &Outcome, failed: &[FailedChild]) -> String {
    match outcome {
        Outcome::Success { payload } => {
            let requests = payload["requests"].as_u64().unwrap_or(0);
            let done = payload["children"]["done"].as_u64().unwrap_or(0);
            let ids: Vec<u64> = payload["child_jobs"]
                .as_array()
                .map(|a| a.iter().filter_map(serde_json::Value::as_u64).collect())
                .unwrap_or_default();
            let range = match (ids.iter().min(), ids.iter().max()) {
                (Some(a), Some(b)) if a == b => format!(" (job {a})"),
                (Some(a), Some(b)) => format!(" (jobs {a}–{b})"),
                _ => String::new(),
            };
            format!(
                "job {id} done: {requests} request{}, {done} done{range}\n",
                if requests == 1 { "" } else { "s" }
            )
        }
        Outcome::Failure {
            error,
            children,
            payload,
            ..
        } => {
            let mut out = format!("job {id} failed: {error}\n");
            // C144: a failure that kept what its job fetched says where.
            if payload.is_some() {
                out.push_str(&format!(
                    "  what it fetched is kept with the result: `acq result {id} --json` shows it\n"
                ));
            }
            let job = |r: &FailedChild| format!("job {}", r.job_id);
            let label = |r: &FailedChild| format!("{} {}", r.kind, r.target).trim().to_string();
            let w1 = failed.iter().map(|r| job(r).len()).max().unwrap_or(0);
            let w2 = failed
                .iter()
                .map(|r| label(r).chars().count())
                .max()
                .unwrap_or(0);
            for r in failed {
                out.push_str(&format!(
                    "  {:<w1$}  {:<w2$}  {}\n",
                    job(r),
                    label(r),
                    r.error
                ));
            }
            let all = children.as_ref().map_or(0, |c| c.failed_ids.len());
            if all > failed.len() {
                out.push_str(&format!(
                    "  and {} more failed; `acq result {id} --json` lists every id\n",
                    all - failed.len()
                ));
            }
            out
        }
        Outcome::Cancelled => format!("job {id} was cancelled\n"),
    }
}

/// `waited`: the job's last progress line said its times (C165), so the
/// text says them no second time.
pub(crate) async fn print_result(
    client: &mut Client,
    id: u64,
    json: bool,
    waited: bool,
) -> Result<()> {
    let result = result_live(client, id).await?;
    // An apply's success has its summary line; any other outcome prints
    // through the one renderer, whatever the job's kind. Status may be
    // gone after a daemon restart; then an apply's success prints its
    // payload.
    if !json
        && matches!(result.outcome, Outcome::Success { .. })
        && client.status(id).await.is_ok_and(|job| job.kind == "apply")
    {
        print!("{}", job_report(id, &result.outcome, &[]));
        if !waited && let Some(words) = times_said(&result.times) {
            eprintln!("job {id}: {words}");
        }
        return Ok(());
    }
    print_outcome(&result, json, waited)
}

/// A terminal job's outcome, from the daemon or from the ledger on disk.
/// JSON is the layer's document, the outcome with the job's id, account
/// and failed children beside it (additive, C53): a blocking verb has no
/// other way to say which job it was or whose. Only a success exits 0 —
/// a failed or cancelled job left nothing on stdout to use, so the exit
/// says so in both modes (C11), and the text of one is [`job_report`]'s
/// on stderr; the JSON outcome is the report, and main adds no second
/// message. Its times in words (C165) follow on stderr, a success's on a
/// line of their own beside the payload, unless `waited`: the job's last
/// progress line said them.
pub(crate) fn print_outcome(result: &JobResult, json: bool, waited: bool) -> Result<()> {
    // C165 and C143: a failure's way on as this terminal's command lines
    let result = &result.clone().offered(&Terminal::writing());
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
        return match result.outcome {
            Outcome::Success { .. } => Ok(()),
            _ => Err(AlreadyReported.into()),
        };
    }
    let times = times_said(&result.times).filter(|_| !waited);
    match &result.outcome {
        Outcome::Success { payload } => {
            println!("{}", serde_json::to_string_pretty(payload)?);
            if let Some(words) = times {
                eprintln!("job {}: {words}", result.job_id);
            }
            Ok(())
        }
        outcome => {
            let mut report = job_report(result.job_id, outcome, &result.failed_children);
            report.push_str(&way_on_line(result));
            if let Some(words) = times {
                report.push_str(&format!("  {words}\n"));
            }
            bail!("{}", report.trim_end())
        }
    }
}

/// C165: a failed job's way on as one line of text, from its document
/// (C53): the words, and the first offer's command line, where the kind
/// offers one; the rest counted. Nothing for a body not recorded, whose
/// line [`job_report`] prints already, or a failure with no kind.
pub(crate) fn way_on_line(result: &JobResult) -> String {
    let Some(way) = &result.way_on else {
        return String::new();
    };
    if matches!(
        result.outcome,
        Outcome::Failure {
            kind: Some(FailureKind::NotRecorded),
            ..
        }
    ) {
        return String::new();
    }
    match result.offers.first().and_then(|o| o["request"].as_str()) {
        Some(line) => {
            let more = match result.offers.len() - 1 {
                0 => String::new(),
                n => format!(", and {n} more in `--json`'s offers"),
            };
            format!("  {}: `{line}`{more}\n", way.words)
        }
        None => format!("  {}\n", way.words),
    }
}

/// C165: a job's times in words, as its document says them (C53, text a
/// function of JSON): `submitted 3s ago, started 2s ago, finished 0s
/// ago`, each that happened; `None` where none is said.
pub(crate) fn times_said(times: &JobTimes) -> Option<String> {
    let said: Vec<String> = [
        ("submitted", &times.submitted_at_age),
        ("started", &times.started_at_age),
        ("finished", &times.finished_at_age),
    ]
    .into_iter()
    .filter_map(|(what, age)| Some(format!("{what} {}", age.as_deref()?)))
    .collect();
    (!said.is_empty()).then(|| said.join(", "))
}

/// C165: the latest of a job's times in words, the jobs table's `when`:
/// its finish, else its start, else its submission.
fn latest_said(times: &JobTimes) -> String {
    [
        ("finished", &times.finished_at_age),
        ("started", &times.started_at_age),
        ("submitted", &times.submitted_at_age),
    ]
    .into_iter()
    .find_map(|(what, age)| Some(format!("{what} {}", age.as_deref()?)))
    .unwrap_or_default()
}

/// `jobs` as text, a function of its document (C53, C149): the rows,
/// the counts by state read from the document, and what it left out —
/// the earliest open and finished past the newest ten of each — with
/// `--all`, which lists every one. The web UI's jobs page states
/// it word for word.
pub(crate) fn jobs_text(listed: &Listed) -> String {
    let mut out = String::new();
    if !listed.rows.is_empty() {
        out.push_str(&rows_text(&listed.rows));
    }
    out.push_str(&summary_text(&listed.counts, Counted::Live));
    if listed.open_left_out > 0 {
        out.push_str(&format!(
            "{} earlier open jobs not listed (`acq jobs --all`)\n",
            listed.open_left_out
        ));
    }
    if listed.finished_left_out > 0 {
        out.push_str(&format!(
            "{} earlier finished jobs not listed (`acq jobs --all`)\n",
            listed.finished_left_out
        ));
    }
    out
}

pub(crate) fn print_table(jobs: &[JobInfo]) {
    print!("{}", table_text(jobs));
}

fn table_text(jobs: &[JobInfo]) -> String {
    if jobs.is_empty() {
        return "no jobs\n".to_string();
    }
    rows_text(jobs) + &summary_text(&JobCounts::of(jobs), Counted::Selected)
}

fn rows_text(jobs: &[JobInfo]) -> String {
    // Widths from the data: a route label or an account name must never
    // push the columns out of line or be cut.
    let rows: Vec<[String; 10]> = jobs
        .iter()
        .map(|job| {
            [
                job.id.to_string(),
                job.parent.map(|p| p.to_string()).unwrap_or_default(),
                job.kind.clone(),
                job.target(),
                if job.retries > 0 {
                    format!("{} ↻{}", job.state, job.retries)
                } else {
                    job.state.to_string()
                },
                job.priority.to_string(),
                job.account.clone().unwrap_or_else(|| "-".into()),
                job.submitted_by.clone(),
                latest_said(&job.times),
                job.eta_seconds
                    .map(|s| format!("~{s}s"))
                    .unwrap_or_default(),
            ]
        })
        .collect();
    let heads = [
        "id", "parent", "kind", "target", "state", "prio", "account", "by", "when", "eta",
    ];
    let mut w = [0usize; 10];
    for (i, h) in heads.iter().enumerate() {
        w[i] = h.len();
    }
    for r in &rows {
        for (i, cell) in r.iter().enumerate() {
            w[i] = w[i].max(cell.chars().count());
        }
    }
    let line = |cells: [&str; 10]| {
        format!(
            "{:>w0$}  {:>w1$}  {:<w2$}  {:<w3$}  {:<w4$}  {:>w5$}  {:<w6$}  {:<w7$}  {:<w8$}  {}",
            cells[0],
            cells[1],
            cells[2],
            cells[3],
            cells[4],
            cells[5],
            cells[6],
            cells[7],
            cells[8],
            cells[9],
            w0 = w[0],
            w1 = w[1],
            w2 = w[2],
            w3 = w[3],
            w4 = w[4],
            w5 = w[5],
            w6 = w[6],
            w7 = w[7],
            w8 = w[8],
        )
    };
    let mut out = format!("{}\n", line(heads).trim_end());
    for r in &rows {
        out.push_str(&format!(
            "{}\n",
            line([
                &r[0], &r[1], &r[2], &r[3], &r[4], &r[5], &r[6], &r[7], &r[8], &r[9]
            ])
            .trim_end()
        ));
    }
    out
}

/// Where a job table's counts were taken (C146: a count says what it
/// counts): one summary line serves three reads, each saying its own.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Counted {
    /// The live list: the jobs this daemon holds, those submitted to it
    /// and those it restored from the ledger alike.
    Live,
    /// The ledger read with no daemon: what is on disk, its `running`
    /// recorded rather than current, and no `done` recorded at all.
    Ledger,
    /// The jobs a command selected: those it shows.
    Selected,
}

/// The last line of a job table: the counts by state, where they were
/// taken ([`Counted`]), and for what waits when the next starts and when
/// the last does — a start, never an end: how long a request then takes
/// is not predicted.
fn summary_text(counts: &JobCounts, counted: Counted) -> String {
    if counts.total == 0 {
        return "no jobs\n".to_string();
    }
    let by = &counts.by_state;
    let running = match counted {
        Counted::Ledger => format!("{} recorded running", by.running),
        Counted::Live | Counted::Selected => format!("{} running", by.running),
    };
    let mut parts = vec![running];
    let waiting = by.waiting;
    parts.push(
        match (counts.next_starts_in_seconds, counts.last_starts_in_seconds) {
            (Some(next), Some(last)) if last > next => {
                format!("{waiting} waiting (next starts in ~{next}s, the last in ~{last}s)")
            }
            (Some(next), _) => format!("{waiting} waiting (next starts in ~{next}s)"),
            _ => format!("{waiting} waiting"),
        },
    );
    if counted == Counted::Ledger {
        return format!(
            "{} on disk: {}; done not recorded\n",
            plural(counts.total, "job", "jobs"),
            parts.join(", ")
        );
    }
    parts.push(format!("{} done", by.done));
    if by.failed > 0 {
        parts.push(format!("{} failed", by.failed));
    }
    if by.cancelled > 0 {
        parts.push(format!("{} cancelled", by.cancelled));
    }
    let held = match counted {
        Counted::Live => " this daemon holds (its own and the restored)",
        Counted::Ledger | Counted::Selected => "",
    };
    format!(
        "{}{held}: {}\n",
        plural(counts.total, "job", "jobs"),
        parts.join(", ")
    )
}

/// Re-render the job table once a second until every listed job is
/// terminal. The list is read as pages, newest first, until one reaches
/// below the earliest of `ids` (C166).
pub(crate) async fn watch_table_until_done(client: &mut Client, ids: &[u64]) -> Result<()> {
    let mut printed_lines = 0usize;
    let earliest = ids.iter().copied().min().unwrap_or(0);
    loop {
        let below = |jobs: &[JobInfo]| jobs.last().is_some_and(|job| job.id < earliest);
        let (mut jobs, _) = acquisition_verbs::jobs::pages(client, ListState::All, below).await?;
        jobs.reverse();
        // Move the cursor up over the previous render and redraw in place.
        if printed_lines > 0 {
            print!("\x1b[{printed_lines}A");
        }
        let mine: Vec<JobInfo> = jobs.into_iter().filter(|j| ids.contains(&j.id)).collect();
        for _ in 0..printed_lines {
            println!("\x1b[2K");
        }
        if printed_lines > 0 {
            print!("\x1b[{printed_lines}A");
        }
        print_table(&mine);
        printed_lines = mine.len() + 1;
        if mine.iter().all(|j| j.state.is_terminal()) {
            let done = mine.iter().filter(|j| j.state == JobState::Done).count();
            println!("\nall jobs finished ({done}/{} done)", mine.len());
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acquisition_protocol::protocol::ListCounts;

    /// C53 and C149: `jobs`' text is a function of its document — the
    /// counts line read from it over every job, and what it left out,
    /// open and finished apart, each with the flag that lists every one
    /// (the brief's second review, finding 2: the open jobs are cut too);
    /// C166: the counts are the daemon's, over jobs the rows leave out.
    #[test]
    fn c149_the_jobs_text_counts_every_job_and_says_what_it_left_out() {
        let job = |id: u64, state: JobState| JobInfo {
            id,
            kind: "sleep".into(),
            state,
            priority: 0,
            submitted_by: "cli:1".into(),
            eta_seconds: (state == JobState::Waiting).then_some(id),
            parent: None,
            retries: 0,
            account: None,
            params: serde_json::json!({}),
            times: Default::default(),
        };
        // the daemon's whole list: 15 done, 13 waiting, the soonest
        // waiting starting in 16 s and the latest in 28 s
        let counts = ListCounts {
            total: 28,
            waiting: 13,
            done: 15,
            soonest_eta_seconds: Some(16),
            latest_eta_seconds: Some(28),
            ..ListCounts::default()
        };
        // the first pages: the newest ten of each state
        let newest: Vec<JobInfo> = (6..=15)
            .map(|id| job(id, JobState::Done))
            .chain((19..=28).map(|id| job(id, JobState::Waiting)))
            .collect();
        let text = jobs_text(&Listed::of(newest, &counts, ListState::All));
        assert_eq!(text.lines().filter(|l| l.contains(" sleep ")).count(), 20);
        assert!(
            text.contains(
                "28 jobs this daemon holds (its own and the restored): 0 running, 13 waiting (next starts in ~16s, the last in ~28s), 15 done\n"
            ),
            "{text}"
        );
        assert!(
            text.ends_with(
                "3 earlier open jobs not listed (`acq jobs --all`)\n5 earlier finished jobs not listed (`acq jobs --all`)\n"
            ),
            "{text}"
        );
        let every: Vec<JobInfo> = (1..=15)
            .map(|id| job(id, JobState::Done))
            .chain((16..=28).map(|id| job(id, JobState::Waiting)))
            .collect();
        let all = jobs_text(&Listed::of(every, &counts, ListState::All));
        assert_eq!(all.lines().filter(|l| l.contains(" sleep ")).count(), 28);
        assert!(!all.contains("not listed"), "{all}");
        // a view narrowed to the open jobs leaves out none of the finished
        let open: Vec<JobInfo> = (19..=28).map(|id| job(id, JobState::Waiting)).collect();
        let narrowed = jobs_text(&Listed::of(open, &counts, ListState::Open));
        assert!(
            narrowed.ends_with("3 earlier open jobs not listed (`acq jobs --all`)\n"),
            "{narrowed}"
        );
        // a selected job's table keeps its own context: the jobs it shows
        let one = table_text(&[job(3, JobState::Waiting)]);
        assert!(
            one.ends_with("1 job: 0 running, 1 waiting (next starts in ~3s), 0 done\n"),
            "{one}"
        );
    }

    /// A parent's result as the layer gives it: the daemon's sentence,
    /// a tally of `ids` failed, and one detailed child.
    fn parent_failed(error: &str, ids: Vec<u64>) -> JobResult {
        JobResult {
            job_id: 4,
            account: None,
            outcome: Outcome::Failure {
                error: error.into(),
                kind: None,
                children: Some(acquisition_protocol::job::Children {
                    done: 2,
                    failed: ids.len() as u64,
                    cancelled: 0,
                    failed_ids: ids,
                }),
                payload: None,
            },
            times: Default::default(),
            failed_children: vec![FailedChild {
                job_id: 7,
                kind: "stash".into(),
                target: "Standard/t9".into(),
                error: "GET /stash/Standard/t9 returned 404".into(),
            }],
            offers: Vec::new(),
            way_on: None,
        }
    }

    fn text_of(r: &JobResult) -> String {
        job_report(r.job_id, &r.outcome, &r.failed_children)
    }

    /// C165 and invariant 3 (never retry through a block): a blocked
    /// fetch's result offers nothing at this terminal, and its one line
    /// of text says not to submit it again, naming no command.
    #[test]
    fn c165_a_blocked_fetch_offers_nothing_and_says_not_to_retry() {
        let outcome = Outcome::failure(
            FailureKind::Blocked,
            "GET /stash/Standard/t1 returned 403 Forbidden; NOT retrying",
        );
        let params = serde_json::json!({ "league": "Standard", "id": "t1" });
        let way_on = acquisition_verbs::jobs::way_on(
            &outcome,
            5,
            Some("alice"),
            Some(("stash", &params)),
            &[],
        );
        let result = JobResult {
            job_id: 5,
            account: Some("alice".into()),
            outcome,
            times: Default::default(),
            failed_children: Vec::new(),
            offers: Vec::new(),
            way_on,
        }
        .offered(&Terminal::writing());
        assert!(result.offers.is_empty(), "{:?}", result.offers);
        let line = way_on_line(&result);
        assert!(line.contains("do not submit it again"), "{line}");
        assert!(
            !line.contains('`'),
            "a blocked fetch names no command: {line}"
        );
    }

    /// The review's case: a restart's deferred failure (the sentence the
    /// daemon installs mid fan-out) with a failed child keeps its warning
    /// and its remedy, and the child's row follows it.
    #[test]
    fn a_deferred_parent_failure_keeps_its_words_beside_a_failed_childs_row() {
        let error = "interrupted by a daemon restart mid fan-out: the children submitted \
                     before the restart ran (their responses are in the store), but the full \
                     child set is unknown — resubmit to complete it";
        let text = text_of(&parent_failed(error, vec![7]));
        assert!(
            text.starts_with(&format!("job 4 failed: {error}\n")),
            "{text}"
        );
        assert!(text.contains("resubmit"), "{text}");
        assert!(
            text.contains("job 7  stash Standard/t9  GET /stash/Standard/t9 returned 404"),
            "{text}"
        );
    }

    /// The ordinary case: the daemon's count sentence once, then the rows;
    /// the CLI composes no count of its own.
    #[test]
    fn a_failed_apply_prints_the_daemons_sentence_once_and_the_rows() {
        let text = text_of(&parent_failed("1 of 3 child jobs failed, 2 done", vec![7]));
        assert_eq!(
            text,
            "job 4 failed: 1 of 3 child jobs failed, 2 done\n  \
             job 7  stash Standard/t9  GET /stash/Standard/t9 returned 404\n"
        );
        assert_eq!(text.matches("child jobs failed").count(), 1);
    }

    /// More failed ids than detailed rows: the rest are counted, and the
    /// line names where every id is listed — the JSON's tally, which a
    /// ledger read has too, never a live-only listing.
    #[test]
    fn failed_children_past_the_rows_are_counted_with_where_every_id_is() {
        let text = text_of(&parent_failed(
            "3 of 5 child jobs failed, 2 done",
            vec![7, 8, 9],
        ));
        assert!(
            text.ends_with("  and 2 more failed; `acq result 4 --json` lists every id\n"),
            "{text}"
        );
    }
}
