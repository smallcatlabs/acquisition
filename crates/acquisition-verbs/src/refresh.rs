//! The refresh flow as every frontend asks it (C46): compile the stored
//! policy into a plan (C39, offline), quote it if a daemon runs (C10, an
//! observation that never spawns), apply it once (C38, plans are
//! binding; C44 and C111, the spend gates) and say what landed. Each
//! answer is one type both surfaces serialise: the CLI's `--json` prints
//! it, the MCP returns it, the CLI's text is a function of it.

use std::collections::BTreeSet;
use std::time::Duration;

use acquisition_client::client::{Client, Observed};
use acquisition_plan::{RefreshPlan, Spent, SpentBy, apply_params_of, plan_refresh};
use acquisition_protocol::job::{JobInfo, Outcome};
use acquisition_protocol::protocol::{QuoteJob, Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_store::{RefreshSnapshot, Status, Store};
use serde::Serialize;

use crate::fault::Fault;
use crate::frontend::Opened;
use crate::jobs::{Progress, submit, wait};

/// The quote attempt's bound: the plan in hand is the deliverable, and a
/// wedged daemon or socket peer must not keep an offline plan from
/// returning.
pub const QUOTE_TIMEOUT: Duration = Duration::from_secs(5);

/// A compiled plan, with why the daemon's quote is absent when it is
/// (C10): the plan is the deliverable, the quote decoration.
#[derive(Clone, Debug, Serialize)]
pub struct Planned {
    pub plan: RefreshPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_note: Option<QuoteNote>,
}

/// How many tabs and characters the plan skipped: every apply answer
/// says so, so an empty or spent plan's nothing says which nothing (C53).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Skipped {
    pub tabs: usize,
    pub characters: usize,
}

impl Skipped {
    /// C53 (every nothing says which nothing): the plan's skipped tabs
    /// and characters, counted.
    pub fn of(plan: &RefreshPlan) -> Skipped {
        Skipped {
            tabs: plan.skipped_tabs.len(),
            characters: plan.skipped_characters.len(),
        }
    }
}

/// What applying a plan did (C38, C111), one arm per outcome; each
/// frontend's renderer matches it whole.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Apply {
    /// The plan authorizes no requests: no daemon was contacted.
    Nothing {
        applied: bool,
        requests: u64,
        note: String,
        skipped: Skipped,
    },
    /// Every request of the plan is spent (C111): fetched after it was
    /// compiled, or held by a job. `remaining_requests` is what a plan
    /// compiled now would fetch, `None` while a queued request has not
    /// landed (or the compile failed).
    Spent {
        applied: bool,
        requests: u64,
        note: String,
        spent: Vec<Spent>,
        skipped: Skipped,
        remaining_requests: Option<u64>,
    },
    /// The unspent actions went to the daemon as one `apply` job.
    Submitted(Submitted),
}

/// An apply job the daemon admitted, and what the process that submitted
/// it needs to say what landed: the store's status and the clock at the
/// submit, since what changed is a difference of two statuses.
#[derive(Debug, Serialize)]
pub struct Submitted {
    pub job_id: u64,
    pub account: String,
    pub requests: usize,
    pub spent: Vec<Spent>,
    pub skipped: Skipped,
    #[serde(skip)]
    pub before: Status,
    #[serde(skip)]
    pub started: i64,
}

/// What an apply did once its job finished: the outcome, the store's
/// changes since the submit, and what a plan compiled now would still
/// fetch (C53: the next action).
#[derive(Debug, Serialize)]
pub struct Landed {
    #[serde(flatten)]
    pub outcome: Outcome,
    pub job_id: u64,
    pub account: String,
    pub store_changes: StoreChanges,
    pub remaining_requests: Option<u64>,
    pub spent: Vec<Spent>,
}

/// What the store recorded between an apply's submit and its end: the
/// answer to "what changed", read from the facts rather than inferred
/// from the job.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct StoreChanges {
    pub responses: i64,
    pub added: usize,
    pub changed: usize,
    pub moved: usize,
    pub removed: usize,
    pub locations: usize,
}

impl StoreChanges {
    /// C34 (facts through the store): the difference between `before`
    /// and the store now, and the item events since `started`.
    pub fn since(store: &Store, before: &Status, started: i64) -> Result<StoreChanges, Fault> {
        let after = store.status().map_err(Fault::Other)?;
        let events = store
            .events_since(started, 1_000_000)
            .map_err(Fault::Other)?;
        let mut out = StoreChanges {
            responses: after.responses - before.responses,
            ..Default::default()
        };
        let mut locations = BTreeSet::new();
        for e in &events {
            match e.kind.as_str() {
                "added" => out.added += 1,
                "changed" => out.changed += 1,
                "moved" => out.moved += 1,
                "removed" => out.removed += 1,
                _ => {}
            }
            for loc in [&e.from_location, &e.to_location].into_iter().flatten() {
                locations.insert(loc.clone());
            }
        }
        out.locations = locations.len();
        Ok(out)
    }
}

/// C39 (the planner, offline) and C46: the one compile every frontend
/// calls, from a snapshot. A snapshot with no policy is
/// [`Fault::NoSyncPolicy`] naming the account.
pub fn compile(
    provider: &str,
    snapshot: &RefreshSnapshot,
    username: &str,
    now: i64,
) -> Result<RefreshPlan, Fault> {
    plan_refresh(provider, snapshot, now).map_err(|e| match Fault::from(e) {
        Fault::NoSyncPolicy { .. } => Fault::NoSyncPolicy {
            username: username.to_string(),
        },
        other => other,
    })
}

/// C10 (an observation never spawns) and C46: ask a running daemon to
/// quote the plan, within [`QUOTE_TIMEOUT`]. Never fails: the plan is
/// the deliverable, and `quote_note` says why a quote is absent.
pub async fn quote(plan: RefreshPlan) -> Planned {
    quote_within(plan, QUOTE_TIMEOUT).await
}

/// C10 (an observation never spawns): [`quote`] under another bound,
/// for a test that stages a wedged peer.
pub async fn quote_within(plan: RefreshPlan, limit: Duration) -> Planned {
    let Some(account) = plan.account_name.clone() else {
        // `with_quote` requires the quote to name exactly the plan's
        // account; with no name on record there is nothing to quote as.
        return Planned {
            plan,
            quote_note: Some(QuoteNote::NoAccountName),
        };
    };
    let jobs: Vec<QuoteJob> = plan
        .actions
        .iter()
        .map(|action| {
            let (kind, params) = action.job();
            QuoteJob {
                kind: kind.into(),
                params,
            }
        })
        .collect();
    let attempt = async {
        let mut client = match Client::observe().await {
            Ok(Observed::Compatible(client)) => client,
            Ok(Observed::Absent) => return Err(QuoteNote::NoDaemon),
            Ok(Observed::Incompatible(found)) => return Err(QuoteNote::Other(found.to_string())),
            Ok(Observed::Unresponsive(found)) => return Err(QuoteNote::Other(found.to_string())),
            Err(e) => return Err(QuoteNote::Other(format!("{e:#}"))),
        };
        client
            .quote(jobs, Some(account))
            .await
            .map_err(|e| QuoteNote::Other(format!("{e:#}")))
    };
    let miss = match tokio::time::timeout(limit, attempt).await {
        Err(_) => QuoteNote::Timeout(limit),
        Ok(Err(miss)) => miss,
        Ok(Ok(quote)) => match plan.clone().with_quote(quote) {
            Ok(enriched) => {
                return Planned {
                    plan: enriched,
                    quote_note: None,
                };
            }
            // The daemon answered about something other than this plan —
            // a planner/daemon disagreement worth seeing, not hiding.
            Err(e) => QuoteNote::Rejected(e.to_string()),
        },
    };
    Planned {
        plan,
        quote_note: Some(miss),
    }
}

/// Why a plan's quote is absent (C10: the plan is the deliverable, the
/// quote decoration). Each frontend reads the cause from the variant and
/// words its own view; `Display` is the note's one sentence per cause,
/// and the JSON `quote_note` is that sentence on every surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuoteNote {
    /// The facts record no account name, and a quote names exactly the
    /// plan's account.
    NoAccountName,
    /// No daemon runs: the common case, without an OS error — a plan
    /// never spawns a daemon, so whether spawning is allowed is beside
    /// the point.
    NoDaemon,
    /// The daemon did not answer within the bound.
    Timeout(Duration),
    /// The daemon answered about something other than this plan.
    Rejected(String),
    /// Another daemon, an unresponsive peer, a failed conversation: its
    /// words.
    Other(String),
}

impl std::fmt::Display for QuoteNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteNote::NoAccountName => {
                f.write_str("no quote: the facts record no account name to quote as")
            }
            QuoteNote::NoDaemon => f.write_str(
                "no quote: no daemon running (the plan needs none; a running daemon adds its ETA)",
            ),
            QuoteNote::Timeout(limit) => write!(
                f,
                "no quote: the daemon did not answer within {limit:?} — plan compiled offline"
            ),
            QuoteNote::Rejected(e) => write!(f, "daemon quote rejected: {e}"),
            QuoteNote::Other(why) => write!(f, "no quote: {why} — plan compiled offline"),
        }
    }
}

impl Serialize for QuoteNote {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// C111, in words both readers accept: how many of a plan's requests
/// were spent before this apply and by what — fetched since (with the
/// latest's age), or held by the daemon (naming the jobs). No command:
/// each frontend adds its own way to wait.
pub fn spent_phrase(spent: &[Spent], now: i64) -> String {
    let fetched: Vec<i64> = spent
        .iter()
        .filter_map(|s| match s.by {
            SpentBy::Fetched { fetched_at } => Some(fetched_at),
            SpentBy::Queued { .. } => None,
        })
        .collect();
    let owners = holding_jobs(spent);
    let queued = spent
        .iter()
        .filter(|s| matches!(s.by, SpentBy::Queued { .. }))
        .count();
    let mut parts = Vec::new();
    if let Some(latest) = fetched.iter().max() {
        parts.push(format!(
            "{} fetched after it was compiled, the latest {} ago",
            fetched.len(),
            age((now - latest).max(0))
        ));
    }
    if !owners.is_empty() {
        let names: Vec<String> = owners.iter().map(u64::to_string).collect();
        parts.push(format!(
            "{queued} waiting or running in {} {}",
            if owners.len() == 1 { "job" } else { "jobs" },
            names.join(", ")
        ));
    }
    parts.join(", ")
}

/// C111 (a request spent once): the jobs holding a plan's queued
/// requests, a parent in its children's place, sorted — what a frontend
/// names as the job to wait for.
pub fn holding_jobs(spent: &[Spent]) -> Vec<u64> {
    let mut owners: Vec<u64> = spent
        .iter()
        .filter_map(|s| match s.by {
            SpentBy::Queued { job_id, parent } => Some(parent.unwrap_or(job_id)),
            SpentBy::Fetched { .. } => None,
        })
        .collect();
    owners.sort_unstable();
    owners.dedup();
    owners
}

/// `45s`, `3m`, `5h`, `2d`: an age as the plan's text writes one.
fn age(seconds: i64) -> String {
    let s = seconds.max(0);
    if s < 90 {
        format!("{s}s")
    } else if s < 5400 {
        format!("{}m", s / 60)
    } else if s < 172_800 {
        format!("{}h", s / 3600)
    } else {
        format!("{}d", s / 86400)
    }
}

/// The note of an apply with nothing to send.
pub const NOTHING_TO_DO: &str = "nothing to do: the plan authorizes no requests";

impl Opened {
    fn snapshot(&self, realm: Realm, league: &str) -> Result<RefreshSnapshot, Fault> {
        self.facts
            .refresh_snapshot(realm.as_str(), league, &self.intent)
            .map_err(Fault::Other)
    }

    /// C39 and C46: the stored policy and the facts on record compiled
    /// into a plan for one (realm, league), offline.
    pub fn compile(&self, realm: Realm, league: &str, now: i64) -> Result<RefreshPlan, Fault> {
        let snapshot = self.snapshot(realm, league)?;
        compile(
            self.account.frontend.provider,
            &snapshot,
            &self.account.entry.username,
            now,
        )
    }

    /// C39 then C10: compile, then ask a running daemon's quote.
    pub async fn plan(&mut self, realm: Realm, league: &str) -> Result<Planned, Fault> {
        let plan = self.compile(realm, league, acquisition_store::now())?;
        // The quote observes the environment's world.
        self.account.frontend.same_world();
        Ok(quote(plan).await)
    }

    /// What a plan compiled now over the facts would fetch: `None` when
    /// the compile fails, the apply's own answer standing without it.
    fn remaining(&self, plan: &RefreshPlan) -> Option<u64> {
        self.snapshot(plan.realm, &plan.league)
            .ok()
            .and_then(|snapshot| {
                plan_refresh(
                    self.account.frontend.provider,
                    &snapshot,
                    acquisition_store::now(),
                )
                .ok()
            })
            .map(|next| next.logical_requests)
    }

    /// C38 (plans are binding), C44 (the spend gate) and C111 (a request
    /// spent once): apply a reviewed plan's unspent actions as one
    /// `apply` job. The gate runs before any daemon contact and an empty
    /// plan contacts none. What was fetched after the plan's snapshot is
    /// read from the store; only when something is left is the daemon
    /// asked what it holds, on the connection the submit needs, and the
    /// facts read again after that list, so a fetch finishing between the
    /// two reads is in one of them.
    ///
    /// `client` is the connection: one the caller holds is used, and one
    /// this verb opens through the frontend's use door is left there, so
    /// the caller can say a daemon was started (C3) and wait on it.
    pub async fn apply(
        &mut self,
        plan: &RefreshPlan,
        max_requests: Option<u64>,
        client: &mut Option<Client>,
    ) -> Result<Apply, Fault> {
        let frontend = self.account.frontend.clone();
        let username = self.account.entry.username.clone();
        plan.check_spendable(
            frontend.provider,
            self.account.entry.uuid.as_deref(),
            &self.intent,
        )?;
        let skipped = Skipped::of(plan);
        if plan.actions.is_empty() {
            // A strict subset of zero actions is satisfied by doing
            // nothing; the plan's own skipped and unknown reporting says
            // why it is empty.
            return Ok(Apply::Nothing {
                applied: false,
                requests: 0,
                note: NOTHING_TO_DO.into(),
                skipped,
            });
        }
        let facts = self.snapshot(plan.realm, &plan.league)?;
        let (mut unspent, mut spent) = plan.unspent(&facts, &[], &username);
        if !unspent.is_empty() {
            if client.is_none() {
                *client = Some(frontend.connect().await?);
            }
            if let Some(connected) = client.as_mut() {
                let jobs = list(connected).await?;
                let facts = self.snapshot(plan.realm, &plan.league)?;
                (unspent, spent) = plan.unspent(&facts, &jobs, &username);
            }
        }
        let connected = match client.as_mut() {
            Some(connected) if !unspent.is_empty() => connected,
            _ => {
                // Queued requests have not landed, so what a plan compiled
                // now would fetch still counts them: no remaining count.
                let queued = spent.iter().any(|s| matches!(s.by, SpentBy::Queued { .. }));
                let remaining_requests = if queued { None } else { self.remaining(plan) };
                return Ok(Apply::Spent {
                    applied: false,
                    requests: 0,
                    note: format!(
                        "nothing to do: every request of this plan is spent — {}",
                        spent_phrase(&spent, acquisition_store::now())
                    ),
                    spent,
                    skipped,
                    remaining_requests,
                });
            }
        };
        let before = self.facts.status().map_err(Fault::Other)?;
        let started = acquisition_store::now();
        let job_id = submit(
            &frontend,
            connected,
            "apply",
            apply_params_of(&unspent, max_requests),
            0,
            Some(&username),
        )
        .await?;
        Ok(Apply::Submitted(Submitted {
            job_id,
            account: username,
            requests: unspent.len(),
            spent,
            skipped,
            before,
            started,
        }))
    }

    /// C53 (the next action) and C34: wait for a submitted apply, then
    /// say what landed — the outcome, the store's changes since the
    /// submit, and what the plan's (realm, league) still wants.
    pub async fn landed(
        &mut self,
        client: &mut Client,
        plan: &RefreshPlan,
        submitted: Submitted,
        progress: Option<&mut (dyn FnMut(&Progress) + Send)>,
    ) -> Result<Landed, Fault> {
        let outcome = wait(client, submitted.job_id, progress).await?;
        let store_changes = StoreChanges::since(&self.facts, &submitted.before, submitted.started)?;
        Ok(Landed {
            outcome,
            job_id: submitted.job_id,
            account: submitted.account,
            store_changes,
            remaining_requests: self.remaining(plan),
            spent: submitted.spent,
        })
    }
}

/// The daemon's job list, this lifetime's.
pub(crate) async fn list(client: &mut Client) -> Result<Vec<JobInfo>, Fault> {
    match client
        .request(&Request::List)
        .await
        .map_err(Fault::from_client)?
    {
        Response::Jobs { jobs } => Ok(jobs),
        Response::Error { kind, message } => {
            Err(Fault::Daemon(acquisition_client::client::DaemonError {
                kind,
                message,
            }))
        }
        other => Err(Fault::Transport(anyhow::anyhow!(
            "unexpected response: {other:?}"
        ))),
    }
}
