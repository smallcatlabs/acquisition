//! The daemon: one job queue, a dispatcher running at most one task per
//! scheduling key, and a JSON-lines server on a Unix socket or, on
//! Windows, a named pipe (C112). It owns the shared state and clients
//! reach it over local IPC (C2); every API request is a job, never a call
//! (C4).
//!
//! Lifecycle follows the gpg-agent model: clients spawn it on demand, it exits
//! on its own after a stretch with no connections and no live jobs.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/daemon.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is each
//! entry's full text as recorded there, moved here on 2026-09-02 because
//! the mechanism it describes is this module's. The registry is current;
//! this is the mechanism as decided, kept beside the code that implements it.
//!
//! ## C6 — The job queue persists: a `jobs` table in a per-daemon `daemon.db` (SQLite, in the prov…
//!
//! **The job queue persists: a `jobs` table in a per-daemon `daemon.db` (SQLite, in the provider's store directory beside the account files), written through at every state change and read back at start.** Memory stays the runtime source of truth — the table is a mirror, "the HashMap, but it survives" — and the one thing read from it while the daemon runs is `result` for an id this lifetime never held (history, not state). On restore only open jobs are loaded (terminal rows carry bodies; a week of them does not belong in memory): `waiting` jobs resume; a `running` job is **re-queued** where the replay premise holds — every network kind is an idempotent GET and the restart probe reads GGG's current counters before it sends, so the duplicate costs one seen hit. Two exceptions (2026-08-30 review): on a declared **no-probe route** the premise fails — a replay would go out against an empty limiter — so the job fails as interrupted instead; and a running **parent whose children exist is mid-fan-out** (its held result was not yet written) — re-running it would submit a duplicate child set, so it holds for the children it has and then finishes as **interrupted, never success**: how many children were never submitted is unknowable, so a partial fan-out must not claim completeness (the children that did run recorded their responses; resubmitting completes the set). A parent whose held result was written resumes holding it; `probe` rows are dropped (one HEAD per lifetime, N16); ids continue from where they were (`AUTOINCREMENT`, never reused, so a stale `acq result <id>` can never name a different job) — a daemon that cannot open or read `daemon.db` refuses to start rather than risk reissuing them, and a queue **write** failure at runtime is sticky: a submit whose insert fails is refused with its id rolled back (a job exists only once its row does), later submits are refused outright, and the dispatcher stops picking while running jobs finish — **ids** never run ahead of disk. Completions are the accepted residual, stated plainly: a job already running when the flag trips finishes in memory but its outcome write fails, so disk still says running and the next daemon replays it (probed route: one seen duplicate hit; no-probe: fails as interrupted) — the send already happened, so refusing to finish it would record nothing at all. The same teeth apply per transition: a `waiting→running` write that fails reverts the job instead of running it (a send the queue cannot see must not happen), `cancel`/`set-priority` report a failed write instead of claiming success (the cancel still wins the job's terminal surface in this lifetime, though an already-running job may still complete its in-flight send — sends are committed once dispatched), and a `result` read failure is an error, never "no job". `submit_child` is refused once its parent is terminal or asked to cancel, under the lock `cancel` takes, so cancellation cannot race an active fan-out into submitting unseen children; a stopped fan-out finishes cancelled or failed, never as success over a partial set; a cancellation that lands after the last child is honored when the held result is installed, cancelled children never count toward a parent's success, and `finish` arbitrates a pending `cancel_requested` under the final lock — a cancel can land at any instant before terminalization and still win. Terminal rows stay so `acq result` has a memory across restarts (`acq jobs` lists live jobs only), pruned at start by age — `ACQ_JOB_RETENTION_DAYS` (default 7) for done/cancelled and `ACQ_FAILED_JOB_RETENTION_DAYS` (default 30) for failed, misread values logged as `CONFIG` errors like the rails knobs. Outcomes are stored verbatim, bodies included (a full refresh is ~50 MB, bounded by retention); compression was considered and deferred — it costs a crate and makes the column unreadable in `sqlite3`, and compressing one column later is a local change. **One daemon per store directory is the world lock (C83)**: since the daemon split's step 5 the daemon takes an exclusive lock on its world's root (`<root>/daemon.lock`, `acquisition_store::world::Lock`) before `daemon.db` opens and refuses to start naming the holder; two daemons on one `daemon.db` would each restore and run the same queue. Parallel mock daemons are parallel worlds (`ACQ_STORE_DIR=<scratch>`, whose socket derives from it; the mock-session skill). Rationale: the queue was the one thing a restart lost once results moved to the store (2026-08-29); a mirror written under the same lock as the memory change keeps disk equal to memory at the `process::exit` the daemon leaves by (up to the declared write-failure residual above); SQLite because it is the crate's one persistence idiom, debuggable with `sqlite3`, and readable by a frontend without a daemon. Decided 2026-08-30.
//!
//! ## C23 — Work that needs many requests is a parent job that submits child jobs; a parent finishe…
//!
//! **Work that needs many requests is a parent job that submits child jobs; a parent finishes when its last descendant does, gives up its dispatcher task and scheduling key while waiting, and cancels its descendants when cancelled.** Rationale: the queue, dispatcher, priorities, ETAs, and events already work per job, so children get all of it for free; a job-internal loop would need its own scheduler and hide the requests from every tool. Observed API shapes (2026-08-20): folder children are in the stash list (a folder holds tabs only — never items, never another folder; confirmed against GGG patch notes 2026-08-24); map/unique substashes only appear on fetching the tab (one map tab listed 234); substash stubs carry `metadata.items` counts. Following substashes is opt-in per tab. Children exist only once their parent runs, so a progress denominator grows while a client watches ("0/1", then "8/8", and again when each map/unique tab lands): any progress UI must expect the tree to widen (frontend boundary finding, 2026-08-24).
//!
//! ## C31 — Multi-account is one daemon holding many sessions, not one daemon per account.
//!
//! **Multi-account is one daemon holding many sessions, not one daemon per account.** The Cloudflare bound (`SendGate`, 2 live sends) is a per-IP property (P-B, ground truth §1) held as per-process state; two daemons on one machine make it a 4-wide burst that neither sees, with separate tripwires — enforced among real-mode daemons for one OS user by the C83 lock (`<runtime>/ggg.lock`, taken in `run` before the log directory is made, the log rotated or opened, or anything sends, whatever the root — a refused contender may append its refusal to an existing log afterward; amended 2026-09-11, the split's step 5); other users and processes remain external concurrency no local lock can coordinate, and the tripwire bounds further sends after any resulting visible violation. Rung 11 (2026-08-30) showed the other half: `Account` rules count per account on GGG's side, so two accounts never contend on layer 2 — the only thing they share is layer 1 and the `Ip`-scoped token endpoint, which is exactly what the single gate exists for. Built in two halves with different blast radii (option C): **account as first-class identity first** (store path, job field, keyring key — leaves), then **many live sessions** (a refactor confined to the session layer) — both built by 2026-08-30; every persisted account is restored as a live session at start. Limiter and probe scope keying — `(account, policy)` for `Account` rules, policy alone for `Ip` rules, scope learned from `X-Rate-Limit-Rules` — is a **precondition of the session map, not an optimization**: with two live sessions on one policy each response would overwrite shared state with a different account's counters, and the next send from the other account floods (a 429 path; the "over-waits, never floods" reading only held for rung 11's sequential switch). Decided 2026-08-29, amended 2026-08-30 after review across sessions; design below in "Multi-account design"; built 2026-08-30 through step (6) — step (7)'s live samples are in `RUN-LEDGER.md`.
//!
//! ## C32 — Per-route knowledge about GGG that headers cannot teach lives in one place (`Daemon::de…
//!
//! **Per-route knowledge about GGG that headers cannot teach lives in one place (`Daemon::declare_route_knowledge`), and strict observation is the default everywhere else.** `GET /profile` (first contact 2026-08-30) answers 200 with no `X-Rate-Limit-*` headers at all and 403 to HEAD, which strict observation ("every endpoint has a policy", post-N33) classed as a protocol failure and discarded. Now: a route *declared* policyless accepts a 2xx with **no** rate-limit header (a partial set is still a failure; a policy that later appears is learned strictly), becomes `EndpointState::Policyless`, and is paced by nothing but the send gate; a declared no-probe route goes straight to its GET. Only `/profile` is declared, and it is called at most once per login. Not generalised on purpose: "any headerless 2xx is fine" reopens the blind spot strict observation closed. Owner decision 2026-08-30; GGG confirmed the same day (Q12/N38): `/profile` is not rate limited at present, so the declaration is confirmed and stays until headers ever appear — strict observation covers that arm.
//!
//! ## C43 — Apply is its own pure fan-out parent job kind (`apply`), never the `refresh` parent.
//!
//! **Apply is its own pure fan-out parent job kind (`apply`), never the `refresh` parent.** The refresh parent re-lists by construction — it fans out from the listing it just fetched — which contradicts "executes exactly the listed actions": a plan's listing is an optional action and its fetches derive from reviewed facts. So `apply`'s params carry the plan's actions as explicit `(kind, params)` child tuples; the parent performs no send of its own, submits exactly one child per tuple, and holds for them. "Never expands" is structural, not disciplinary: the daemon stays plan-blind (it cannot link the plan crate, and it reads no intent — C34; the store it links only to write facts and keep its queue), so what it admits is **vocabulary, not meaning** — only single-request kinds (`stashes`, or `stash` with `deep` false), each of which submits no children. Admission is at submit, before a job id exists: a malformed tuple list, an empty one, or a logical bound over the caller's `max_requests` refuses the submit whole — the mid-fan-out terminalization path is never the budget's normal mechanism (D8). Plan validation, the staleness check, and rendering actions to tuples are the frontend's (`acq refresh --apply`, through the planner's validating parse). The ad-hoc `refresh` kind (`--all`/`--tabs`) stays untouched as the explicit client-stated-selection surface; whether it retires rides on step 9's friction notes. Decided 2026-09-01.
//!
//! ## C49–C51 — Multi-account design as recorded (2026-08-30)
//!
//! **Complexity rule:** the only code that interprets accounts is the
//! session layer. Everywhere else account is data — a field on the job, a
//! path segment for the store, an opaque key component for the limiter
//! (which never reads it; scope comes from `X-Rate-Limit-Rules`). An
//! `if account == …` outside the session layer is the smell.
//!
//! - **Identity: the stable account key is the profile `uuid`, fetched at
//!   login and required** (amended 2026-08-31; was username-only with
//!   opportunistic uuid). After token exchange the daemon submits a profile
//!   job — causal service of the client's `acq auth` — and the session is
//!   registered, the keyring written, and `accounts.json` updated only when
//!   the uuid lands; a login whose profile fetch fails **fails whole**: no
//!   provisional identity, no minted keys, no rename-repair machinery — if
//!   `/profile` is broken, something is broken and login says so. A retry
//!   repeats the token exchange, paced by the `Ip`-scoped token policy, so
//!   a retry loop is already bounded. `accounts.json` maps
//!   username/discriminator/provider → uuid; a rename is a mapping update
//!   with intent untouched. The token response's `username`
//!   (`name#discriminator`) stays the display name and selector; fact files
//!   stay username-named (refetchable; rename-orphaning tolerable);
//!   annotation files are uuid-named. Entries without a uuid: one re-auth,
//!   no migration. The mock serves deterministic per-username uuids.
//! - **No daemon-side default account; stateless selection.** Every submit
//!   carries `account`. Omitted, it resolves only when exactly one session
//!   exists; otherwise the daemon refuses with the list. While the daemon
//!   holds one session, a submitted `account` is validated against it and
//!   refused on mismatch (so the selector is testable before the session
//!   map exists). The CLI resolves `--account` / `ACQ_ACCOUNT` client-side
//!   against a non-secret index file, `store/<provider>/accounts.json`
//!   (username, uuid when known, last login), so reads never spawn a daemon.
//!   Matching is exact — name with or without discriminator, or uuid —
//!   never by prefix. GUI/MCP hold their own selection and pass it.
//! - One-off (non-persisted) sessions are accounts: listed, selectable,
//!   marked "not persisted".
//! - A job has exactly one account; no cross-account `refresh --all`.
//!   Cross-account work is a frontend loop.
//! - `account` is a protocol field on `Submit`/`JobInfo`, not a params
//!   entry; fixed at submit (resolved against the live session, refused
//!   before a job exists otherwise), checked again at the moment a token is
//!   taken (a mismatch fails the job with no send), and it selects the store
//!   file — never the session at landing time. Shown in `jobs`, `dash`, the
//!   daemon log, and the journal (the `route` field is the endpoint key,
//!   `stash@Alice#1234`).
//! - **Limiter keying as built (step 5):** the endpoint key is
//!   `route@account`; a policy's state is keyed `name@account` only when
//!   *every* rule of the policy is `Account`-scoped and the send had an
//!   account — `Ip` rules, mixed scopes, and accountless sends share the
//!   bare name (over-waits at worst). One notch more conservative than
//!   "Account rules per account" if GGG ever mixes scopes in one policy.
//!   The token route (`oauth-token`) is deliberately accountless: it is
//!   `Ip`-scoped and has no probe, so an accounted key would be unpaced on
//!   an account's first login.
//! - **The free HEAD probe (N24) is per endpoint, not an API property.**
//!   First contact 2026-08-30: `HEAD /account/leagues` is answered 200 and
//!   counted as a hit (the free HEADs answer 204); `HEAD /profile` is 403.
//!   Both routes are declared no-probe in `Daemon::declare_route_knowledge`
//!   and taught by their first GET; pacing was never wrong (headers are
//!   post-increment and trusted), a probe there is just a wasted hit.
//! - Store: `store/<provider>/<account>.db`, opened lazily on first record;
//!   `tabs`/`items`/`store` take the selector and never span accounts.
//!   Keyring: one entry per account; the index file is how the daemon knows
//!   which entries to restore (the keyring crate cannot enumerate). Restore
//!   continues past a dead grant — the terminal-grant mark is per session.
//!   The existing single keyring entry is orphaned, not migrated: one
//!   re-auth.
//! - The `jobs` table lives in a per-daemon `daemon.db`, not inside an
//!   account file, and carries the account column (persistence decision above).
//! - Mock: the login page accepts any username and policies count per
//!   username (the access token carries it, `at-<user>-<rand>`), so
//!   two-account tests can distinguish per-account from shared counting —
//!   the property rung 11 established for GGG.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use acquisition_store::jobs::{JobDb, JobRow, Retention};
use acquisition_store::world::{Lock, World, app_runtime_dir};
use acquisition_store::{Endpoint, Index, Store, account_matches, account_path};
use anyhow::Result;
use serde_json::{Value, json};
use tokio::io::{AsyncWriteExt, BufReader, WriteHalf};
use tokio::net::TcpListener;
use tokio::sync::{Notify, broadcast, watch};

use std::collections::VecDeque;

use crate::frame::{Frame, read_frame};
use crate::provider::{CALLBACK_PATH, Provider, SCOPES};
use crate::rails::{BlockShape, Rails, RailsConfig, verify_state};
use crate::ratelimit::{
    Bearer, ChokePoint, Clock, EndpointState, ProbeError, RetryAfter, SendError, SystemClock,
    url_path,
};
use crate::ratelimit::{endpoint_key, split_endpoint_key};
use acquisition_protocol::artifact::Artifact;
use acquisition_protocol::job::{
    Children, JobId, JobInfo, JobState, MAX_429_RETRIES, Outcome, Priority, target_of,
};
use acquisition_protocol::protocol::{
    Bootstrap, BootstrapReply, ErrorKind, ErrorRecord, MAX_FRAME_BYTES, Quote, QuoteJob,
    QuoteScope, Request, Response, SessionStatus,
};
use acquisition_protocol::provider::ggg_mode;
use acquisition_protocol::realm::{Family, Realm};

use crate::{auth, mockggg};

const IDLE_SHUTDOWN: Duration = Duration::from_secs(60);

/// `ACQ_IDLE_SHUTDOWN=<secs>` overrides the idle exit (L0 rail 8) so a
/// live-test rung or soak runs on one daemon instead of a respawn cycle.
fn idle_shutdown_from_env() -> Duration {
    std::env::var("ACQ_IDLE_SHUTDOWN")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map_or(IDLE_SHUTDOWN, Duration::from_secs)
}
const IDLE_POLL: Duration = Duration::from_secs(5);
const ERROR_HISTORY: usize = 50;
/// Probes outrank everything: every job on that route is waiting on one.
const PROBE_PRIORITY: Priority = u8::MAX;

/// The login's own profile job: interactive, and nothing else on the
/// account can run before the login completes anyway.
const LOGIN_PRIORITY: Priority = u8::MAX - 1;

/// How long a login waits for its profile job before failing whole. Room
/// for a full token-endpoint hold; a rails-halted daemon fails the login
/// rather than pinning the flow forever.
const LOGIN_PROFILE_TIMEOUT: Duration = Duration::from_secs(120);
/// The bound on each diagnostic file — the daemon log and the default
/// send journal (C83: "bounded diagnostics elsewhere"). Past it, the
/// file is rotated once at daemon start ([`rotate_if_over`]): the
/// current generation becomes `<name>.1`, replacing the previous one,
/// so a file never holds more than the cap plus one lifetime, and the
/// two generations together never more than about twice it. Sixteen
/// mebibytes: a full-league refresh is about 150 KB of either file, so a
/// season of daily refreshes fits in one generation, `tail` and an
/// editor open it instantly, and the previous generation is still there
/// when a rotation lands mid-investigation. A journal `ACQ_JOURNAL`
/// points elsewhere (a live run's evidence directory) is the caller's
/// file and is never rotated.
pub const DIAGNOSTIC_CAP_BYTES: u64 = 16 << 20;

/// Bound `path` at `cap` bytes: over it, rotate to `<path>.1`; when the
/// rename fails, truncate in place — the previous generation is lost,
/// the bound holds (review 2026-09-11: a failure that merely logged and
/// kept appending left the file unbounded). `Ok(Some)` says what was
/// done, for the log; `Ok(None)` when nothing was; `Err` when the file
/// could be neither rotated nor truncated, and the daemon must not
/// start over an unbounded diagnostic. Called only by the daemon that
/// holds the world lock, so an incumbent's live file is never moved from
/// under it.
pub fn rotate_if_over(path: &std::path::Path, cap: u64) -> Result<Option<String>, String> {
    let len = match std::fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(_) => return Ok(None),
    };
    if len <= cap {
        return Ok(None);
    }
    let mut previous = path.as_os_str().to_owned();
    previous.push(".1");
    let previous = PathBuf::from(previous);
    match std::fs::rename(path, &previous) {
        Ok(()) => Ok(Some(format!(
            "rotated {} ({len} bytes, over the {cap}-byte cap) to {}",
            path.display(),
            previous.display()
        ))),
        Err(rename) => match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(_) => Ok(Some(format!(
                "truncated {} in place ({len} bytes, over the {cap}-byte cap; the previous generation is lost: it could not be renamed to {}: {rename})",
                path.display(),
                previous.display()
            ))),
            Err(truncate) => Err(format!(
                "{} is {len} bytes, over the {cap}-byte cap, and could be neither rotated ({rename}) nor truncated ({truncate})",
                path.display()
            )),
        },
    }
}

/// What a network call can fail with; 429 is its own arm so the job can
/// be re-queued rather than failed.
#[derive(Debug)]
enum ApiError {
    RateLimited(String),
    Protocol(String),
    Other(String),
}

/// How a GET authenticates (C87): through the account's session, whose
/// token is read after gate admission and told of a 401; through a
/// login's staged tokens, which no session holds yet (a 401 fails the
/// login whole, C50); or not at all (the mock's fake data endpoint).
enum Auth {
    None,
    Session { token: String, username: String },
    Staged(String),
}

/// C87 — the session's token, as the choke point reads it after admission.
/// `expected` is the token the job learned before it waited; what goes out
/// is whatever the session holds *now*: the same token, or the one a
/// refresh installed meanwhile, or nothing — the token was rejected while
/// this send waited, and the send does not go out. A rejection reaches
/// the session through `rejected`, under the rejecting send's permit.
struct SessionBearer<'a> {
    daemon: &'a Daemon,
    username: &'a str,
    expected: &'a str,
    /// Set when `rejected` cleared the session's token: the job's error
    /// says so.
    cleared: std::sync::atomic::AtomicBool,
}

impl<'a> SessionBearer<'a> {
    fn new(daemon: &'a Daemon, username: &'a str, expected: &'a str) -> Self {
        SessionBearer {
            daemon,
            username,
            expected,
            cleared: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn was_cleared(&self) -> bool {
        self.cleared.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Bearer for SessionBearer<'_> {
    fn token(&self) -> Option<String> {
        let s = self.daemon.shared.lock().unwrap();
        let session = s.auth.find(self.username)?;
        let current = session.access_token.clone()?;
        if current != self.expected {
            self.daemon.log(&format!(
                "send for {} carries the token a refresh installed while it waited",
                self.username
            ));
        }
        Some(current)
    }

    fn rejected(&self, token: &str) {
        if self.daemon.reject_access_token(token).is_some() {
            self.cleared
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// A login's staged access token (C50): held outside the session map, so
/// there is nothing to re-read or to forget — a 401 fails the login whole.
struct StagedBearer(String);

impl Bearer for StagedBearer {
    fn token(&self) -> Option<String> {
        Some(self.0.clone())
    }

    fn rejected(&self, _token: &str) {}
}

/// What `execute` hands back to `process`.
enum Exec {
    Done(Outcome),
    RateLimited(String),
}

fn may_requeue_429(retries: u32, cancel_requested: bool) -> bool {
    retries < MAX_429_RETRIES && !cancel_requested
}

struct Entry {
    info: JobInfo,
    params: Value,
    outcome: Option<Outcome>,
    cancel_requested: bool,
    /// A parent's own result, held back until its descendants finish. Set
    /// means "running, waiting on children, with no active dispatcher task".
    deferred: Option<Outcome>,
    /// Unix seconds; the persisted row's `submitted_at`.
    submitted_at: i64,
}

impl Entry {
    /// The row `daemon.db` holds for this job (C6): `JobInfo` column for
    /// column plus the fields a restart needs. `eta_seconds` is a
    /// prediction, never stored.
    fn row(&self, now: i64) -> JobRow {
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
            params: self.params.clone(),
            outcome: self.outcome.as_ref().map(json),
            deferred: self.deferred.as_ref().map(json),
            cancel_requested: self.cancel_requested,
            submitted_at: self.submitted_at,
            updated_at: now,
        }
    }

    /// A restored row. What a restart does with `running` (re-queue) or
    /// `probe` (drop) is decided by the caller; this is the plain mapping.
    fn from_row(row: JobRow) -> Option<Entry> {
        let state = match row.state.as_str() {
            "waiting" => JobState::Waiting,
            "running" => JobState::Running,
            "done" => JobState::Done,
            "failed" => JobState::Failed,
            "cancelled" => JobState::Cancelled,
            _ => return None,
        };
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
                params: row.params.clone(),
            },
            params: row.params,
            outcome: outcome(row.outcome),
            cancel_requested: row.cancel_requested,
            deferred: outcome(row.deferred),
            submitted_at: row.submitted_at,
        })
    }
}

/// `ACQ_JOB_RETENTION_DAYS` / `ACQ_FAILED_JOB_RETENTION_DAYS`, each read
/// like a rails knob: absent means the default, a misread value is
/// reported (the second return) and the default stays.
fn retention_from_env() -> (Retention, Vec<String>) {
    let mut r = Retention::default();
    let mut problems = Vec::new();
    for (var, slot) in [
        ("ACQ_JOB_RETENTION_DAYS", &mut r.done_days),
        ("ACQ_FAILED_JOB_RETENTION_DAYS", &mut r.failed_days),
    ] {
        if let Ok(v) = std::env::var(var) {
            match v.trim().parse::<u32>() {
                Ok(days) => *slot = days,
                Err(_) => {
                    problems.push(format!("{var}={v:?} is not a number of days; using {slot}"))
                }
            }
        }
    }
    (r, problems)
}

type AccessTokenResult = Result<(String, String), Refusal>;

const SESSION_CHANGED_DURING_REFRESH: &str =
    "authentication session changed while token refresh was in progress";
const REFRESH_OWNER_ABANDONED: &str = "token refresh owner was abandoned before producing a result";

/// A request the daemon will not perform, classified where it is made
/// (C47; the wire shape is C85's `Response::Error`): the [`ErrorKind`] a
/// frontend branches on and the message a person reads, which stays
/// useful on its own. Every site that makes one names its kind here, so
/// nothing reaches the wire unclassified — the audit of 2026-09-10 that
/// fixed the vocabulary, site by site:
///
/// | Site | Message | Kind |
/// | --- | --- | --- |
/// | `handle_conn`: a frame that is not JSON/UTF-8, or not a request of this version | `bad request: …` | `bad_request` |
/// | `handle_conn`: a frame over `MAX_FRAME_BYTES` | `frame exceeds …` | `bad_request` |
/// | `handle_conn`: a versioned request before `hello`, or after `subscribed` | `the connection has not opened with hello …` / `this connection is a subscription …` | `bad_request` |
/// | `Sessions::get`, `get_mut`: no session at all | `not logged in — run acq auth` | `not_logged_in` |
/// | `Sessions::get`, `get_mut`, `canonical_account`: none for the selector | `no session for … — run acq auth` | `not_logged_in` |
/// | `Sessions::get`: several live, none named | `several accounts are logged in (…)` | `ambiguous_account` |
/// | `Sessions::matching` (submit, quote, auth check, logout): a selector two or more live sessions match | `account … names several live sessions (…)` | `ambiguous_account` |
/// | `resolve_account`: a selector with no session | `account …: not logged in` / `… is not logged in (live: …)` | `not_logged_in` |
/// | `validate_apply`, `admit_realm` (submit) | `apply needs a jobs array …`, `action i: …`, `max_requests must be …`, `plan exceeds the budget …`, a realm refusal | `refused` |
/// | `submit_with_parent`: parent gone or cancelled (children only; becomes the parent's outcome) | `parent job N is gone` / `was cancelled` | `refused` |
/// | `submit_with_parent`: the sticky queue failure, or the insert failing | `the persisted queue failed (…)` | `queue_failed` |
/// | `Status`, `Result`, `cancel`, `set_priority`: no such id | `no job N` | `unknown_job` |
/// | `Result` before terminal; `cancel` after; `set_priority` off waiting | `job N is still …` / `already …` / `is …, not waiting` | `wrong_state` |
/// | `stored_outcome`: the queue unreadable | `could not read the persisted queue: …` | `queue_failed` |
/// | `cancel`, `set_priority`: the queue write failed | `… the queue write failed …` | `queue_failed` |
/// | `auth_start`: the loopback listener or the authorize URL | `could not bind loopback listener: …` | `internal` |
/// | `valid_access_token`: no refresh token; a grant the provider rejected, now or before | `no refresh token for … — run acq auth`, `token refresh disabled for …`, `token refresh failed: …` (4xx) | `not_logged_in` |
/// | `valid_access_token`: the refresh failed on transport, a 5xx, exhausted 429s, or the rails halt | `token refresh failed: …` | `upstream` |
/// | `valid_access_token`: the session changed under the refresh | `authentication session changed …` | `not_logged_in` |
/// | `valid_access_token`: the refresh owner was dropped | `token refresh owner was abandoned …` | `internal` |
/// | `forget_account`: no index (harness daemons), an unreadable index, a keyring clear that failed | `no account index …`, `accounts index: …`, the keyring's error | `internal` |
/// | `forget_account`: the index knows no such account / several | the index's message | `not_logged_in` / `ambiguous_account` |
/// | `logout`: the keyring clear failed (logged, never on the wire) | the keyring's error | `internal` |
///
/// Not a kind of its own: a rails halt never refuses a submit — a halted
/// send waits, and `quote` names the halt in its body. The one request it
/// reaches the wire through is `auth_check`, whose forced refresh the
/// choke point refuses under the halt: that is `upstream`, the provider
/// not answered, with the session still standing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub kind: ErrorKind,
    pub message: String,
}

impl Refusal {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Refusal {
            kind,
            message: message.into(),
        }
    }

    fn no_job(id: JobId) -> Self {
        Refusal::new(ErrorKind::UnknownJob, format!("no job {id}"))
    }

    fn queue_failed(cause: &str) -> Self {
        Refusal::new(
            ErrorKind::QueueFailed,
            format!(
                "the persisted queue failed ({cause}); the daemon refuses new jobs — restart it once daemon.db is writable"
            ),
        )
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl PartialEq<&str> for Refusal {
    fn eq(&self, other: &&str) -> bool {
        self.message == *other
    }
}

impl From<Refusal> for Response {
    fn from(r: Refusal) -> Response {
        Response::Error {
            kind: r.kind,
            message: r.message,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AuthGenerations {
    session: u64,
    access_token: u64,
    refresh_token: u64,
}

struct RefreshFlight {
    id: u64,
    generations: AuthGenerations,
    result: watch::Sender<Option<AccessTokenResult>>,
}

/// A login whose token exchange succeeded but whose profile fetch has not
/// landed the uuid yet (C50). Held *outside* the
/// session map: the previously live session for the same account keeps
/// serving its jobs untouched, and no client lookup can reach these tokens
/// — only the profile route hands them out (`staged_profile_token`). A
/// login whose profile fetch fails drops this slot: it fails whole.
struct Staging {
    /// The flow that staged this; stale flows can neither complete nor
    /// abort it.
    flow: u64,
    username: String,
    access_token: String,
    access_expires_at: SystemTime,
    refresh_token: String,
}

#[derive(Default)]
struct AuthSession {
    access_token: Option<String>,
    /// Wall-clock, not `Instant`: on macOS a monotonic clock does not advance
    /// while the machine sleeps, so an `Instant` deadline would wake believing
    /// an expired token still has hours left and send 401s until it caught up.
    access_expires_at: Option<SystemTime>,
    refresh_token: Option<String>,
    username: Option<String>,
    /// The account uuid the login's profile fetch delivered (restored
    /// sessions read it from the index). A refresh that renames the account
    /// uses it to update the index mapping in place rather than minting a
    /// uuid-less twin entry.
    uuid: Option<String>,
    /// "ok" or an error description shown in `auth status`.
    keyring: String,
    generations: AuthGenerations,
    refresh_flight: Option<RefreshFlight>,
    next_refresh_flight: u64,
}

/// Every live session, by account. One daemon, many sessions (C31): the
/// daemon holds no default (C51) — a caller names the account, or there
/// is exactly one.
#[derive(Default)]
struct Sessions {
    by_account: HashMap<String, AuthSession>,
    /// A login flow waiting on the browser (its flow generation).
    pending: Option<u64>,
    next_flow: u64,
    /// A login past token exchange, waiting on its profile job's uuid.
    staging: Option<Staging>,
    /// How the most recent login flow ended: the username it registered, or
    /// the error it failed with. Cleared when a new flow starts; this is
    /// what `acq auth` reports, so a failed login is never mistaken for a
    /// different account's live session.
    flow_result: Option<Result<String, String>>,
    /// The account of the most recent login — reported, never used to pick.
    last_login: Option<String>,
    /// Keyring health from restore, shown when there is no session to ask.
    keyring: String,
}

/// The selector names no live session: `not_logged_in`, remedy `acq auth`.
fn no_session(selector: &str) -> Refusal {
    Refusal::new(
        ErrorKind::NotLoggedIn,
        format!("no session for {selector} — run `acq auth`"),
    )
}

impl Sessions {
    /// The session an operation is for: the named account's, or the sole
    /// one. No selector with several live is refused — the daemon does not
    /// guess whose stash to spend sends on.
    /// Invariant: a session's map key is its `username`. Every insert goes
    /// through `replace`, and a refresh that renames goes through
    /// `rename`, so lookups can trust the key.
    fn find(&self, username: &str) -> Option<&AuthSession> {
        self.by_account.get(username)
    }

    fn find_mut(&mut self, username: &str) -> Option<&mut AuthSession> {
        self.by_account.get_mut(username)
    }

    /// Move a session to a new name (a refresh reported a different
    /// username). The session keeps its state; only the key changes. A
    /// session already under the new name is replaced.
    fn rename(&mut self, from: &str, to: &str) {
        if from == to {
            return;
        }
        if let Some(mut session) = self.by_account.remove(from) {
            session.username = Some(to.to_string());
            self.by_account.insert(to.to_string(), session);
            if self.last_login.as_deref() == Some(from) {
                self.last_login = Some(to.to_string());
            }
        }
    }

    /// C87 — an access token the provider rejected with a 401 is dead: the
    /// session that holds it forgets it, so its next token lookup refreshes
    /// before any send. Keyed by the token, not the account: the only
    /// session touched is the one still holding *this* token, so a 401
    /// that lands after a refresh already replaced it changes nothing
    /// (`None`), and no code outside the session layer names an account
    /// (C49). The refresh token and the generations are untouched — a
    /// refresh already in flight installs over the cleared slot as it
    /// would have. Returns the username whose token was cleared.
    fn reject_access_token(&mut self, token: &str) -> Option<String> {
        let session = self
            .by_account
            .values_mut()
            .find(|session| session.access_token.as_deref() == Some(token))?;
        session.access_token = None;
        session.access_expires_at = None;
        session.username.clone()
    }

    fn get(&self, account: Option<&str>) -> Result<&AuthSession, Refusal> {
        match account {
            Some(account) => self.find(account).ok_or_else(|| no_session(account)),
            None => match self.by_account.len() {
                0 => Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    "not logged in — run `acq auth`",
                )),
                1 => Ok(self.by_account.values().next().expect("one")),
                _ => Err(Refusal::new(
                    ErrorKind::AmbiguousAccount,
                    format!(
                        "several accounts are logged in ({}); pick one with --account",
                        self.usernames().join(", ")
                    ),
                )),
            },
        }
    }

    fn get_mut(&mut self, account: Option<&str>) -> Result<&mut AuthSession, Refusal> {
        match account {
            Some(account) => self.find_mut(account).ok_or_else(|| no_session(account)),
            None => {
                self.get(None)?;
                Ok(self.by_account.values_mut().next().expect("one"))
            }
        }
    }

    /// Live usernames, sorted.
    fn usernames(&self) -> Vec<String> {
        let mut v: Vec<String> = self.by_account.keys().cloned().collect();
        v.sort();
        v
    }

    /// The live session a selector names (username, name without
    /// discriminator, or uuid), if any. A name without its discriminator
    /// that two live sessions share is refused as `ambiguous_account`
    /// (C51: matching is exact, and the daemon never guesses) — review
    /// finding 2026-09-10, round 1: `find` over a map picked one. A staged
    /// login is not a session and cannot match: it lives outside this map.
    fn matching(&self, selector: &str) -> Result<Option<&AuthSession>, Refusal> {
        let mut hits = self.by_account.values().filter(|s| {
            s.username
                .as_deref()
                .is_some_and(|u| account_matches(selector, u, s.uuid.as_deref()))
        });
        let first = hits.next();
        let mut rest = hits.peekable();
        if first.is_some() && rest.peek().is_some() {
            let mut names: Vec<String> = first
                .into_iter()
                .chain(rest)
                .filter_map(|s| s.username.clone())
                .collect();
            names.sort();
            return Err(Refusal::new(
                ErrorKind::AmbiguousAccount,
                format!(
                    "account {selector:?} names several live sessions ({}); use the full name",
                    names.join(", ")
                ),
            ));
        }
        Ok(first)
    }

    /// Insert or replace an account's session. Replacing advances the old
    /// session's generations so an in-flight refresh for it lands stale.
    fn replace(&mut self, mut session: AuthSession) -> &mut AuthSession {
        let key = session.username.clone().unwrap_or_default();
        if let Some(old) = self.by_account.remove(&key) {
            session.generations = AuthGenerations {
                session: old.generations.session.wrapping_add(1),
                access_token: old.generations.access_token.wrapping_add(1),
                refresh_token: old.generations.refresh_token.wrapping_add(1),
            };
            session.next_refresh_flight = old.next_refresh_flight;
        }
        self.last_login = Some(key.clone());
        self.by_account.entry(key).or_insert(session)
    }

    #[cfg(test)]
    fn with(session: AuthSession) -> Sessions {
        let mut s = Sessions {
            keyring: "ok".into(),
            ..Sessions::default()
        };
        s.replace(session);
        s
    }

    /// Test helper: the sole session.
    #[cfg(test)]
    fn one(&self) -> &AuthSession {
        assert_eq!(self.by_account.len(), 1, "exactly one session expected");
        self.by_account.values().next().expect("one")
    }

    #[cfg(test)]
    fn one_mut(&mut self) -> &mut AuthSession {
        assert_eq!(self.by_account.len(), 1, "exactly one session expected");
        self.by_account.values_mut().next().expect("one")
    }
}

impl AuthSession {
    fn advance_access_token(&mut self) {
        self.generations.access_token = self.generations.access_token.wrapping_add(1);
    }

    fn advance_refresh_token(&mut self) {
        self.generations.refresh_token = self.generations.refresh_token.wrapping_add(1);
    }
}

struct Shared {
    jobs: HashMap<JobId, Entry>,
    next_id: JobId,
    auth: Sessions,
    connections: usize,
    last_activity: Instant,
    /// Recent errors for the dashboard, newest last (bounded ring). Every
    /// entry is also in the log; this is the structured, queryable subset.
    errors: VecDeque<(Instant, String)>,
    /// Jobs with an active dispatcher task → their scheduling key.
    ///
    /// This is ordering ownership, not HTTP capacity: N4's send gate owns the
    /// actual-send bound. One active task per key keeps same-policy priority
    /// and FIFO stable without letting auth or pacing waits block other keys.
    active_jobs: HashMap<JobId, String>,
}

impl Shared {
    /// Waiting jobs in dispatch order: highest priority first, then FIFO.
    fn queue_order(&self) -> Vec<JobId> {
        let mut waiting: Vec<&Entry> = self
            .jobs
            .values()
            .filter(|e| e.info.state == JobState::Waiting)
            .collect();
        waiting.sort_by_key(|e| (std::cmp::Reverse(e.info.priority), e.info.id));
        waiting.iter().map(|e| e.info.id).collect()
    }

    /// Snapshot with eta filled in for waiting jobs. The daemon can predict
    /// because it sees the whole queue and the limiter.
    fn snapshot(&self, daemon: &Daemon, id: JobId) -> Option<JobInfo> {
        let queue = self.queue_order();
        let entry = self.jobs.get(&id)?;
        let mut info = entry.info.clone();
        info.eta_seconds = if info.state == JobState::Waiting {
            match daemon.keyed_route_for(&info.kind, &entry.params, info.account.as_deref()) {
                Some((route, _)) => {
                    // Only same-route jobs ahead of us compete for the same
                    // policy; counting them is what the estimate needs.
                    let ahead = queue
                        .iter()
                        .take_while(|&&q| q != id)
                        .filter(|q| {
                            self.jobs
                                .get(q)
                                .and_then(|e| {
                                    daemon.keyed_route_for(
                                        &e.info.kind,
                                        &e.params,
                                        e.info.account.as_deref(),
                                    )
                                })
                                .is_some_and(|(r, _)| r == route)
                        })
                        .count();
                    Some(daemon.choke.eta_for(&route, ahead as u32).as_secs())
                }
                None => Some(0),
            }
        } else {
            None
        };
        Some(info)
    }

    fn list(&self, daemon: &Daemon) -> Vec<JobInfo> {
        let mut ids: Vec<JobId> = self.jobs.keys().copied().collect();
        ids.sort();
        ids.into_iter()
            .filter_map(|id| self.snapshot(daemon, id))
            .collect()
    }
}

/// A connection to the daemon's endpoint (C112): a Unix socket, or on
/// Windows the server end of one instance of the world's named pipe.
#[cfg(unix)]
type Stream = tokio::net::UnixStream;
#[cfg(windows)]
type Stream = tokio::net::windows::named_pipe::NamedPipeServer;

/// The daemon's end of its endpoint (C112): bound once at start, then
/// one connection per `accept`.
///
/// - **Unix**: a socket in the private per-user runtime directory. A
///   socket file left by a crash is probed — something answering is a
///   live daemon and the start is refused — and removed; the bound
///   file's identity is kept ([`BoundSocket`]) so the exit path unlinks
///   only its own.
/// - **Windows**: the world's named pipe, every instance created under a
///   DACL granting this user alone (`world_windows`). The first instance
///   is created as such, so a name another process holds refuses the
///   start; each accepted connection is replaced by a fresh instance
///   before it is handed on. A pipe vanishes with its server: there is
///   no stale file to probe and nothing to unlink.
#[cfg(unix)]
struct Listener(tokio::net::UnixListener);
#[cfg(windows)]
struct Listener {
    name: PathBuf,
    next: Stream,
}

impl Listener {
    #[cfg(unix)]
    async fn bind(path: &Path) -> Result<(Listener, BoundSocket)> {
        if path.exists() {
            // Live daemon or stale socket from a crash?
            if tokio::net::UnixStream::connect(path).await.is_ok() {
                anyhow::bail!("daemon already running on {}", path.display());
            }
            std::fs::remove_file(path)?;
        }
        let listener = tokio::net::UnixListener::bind(path)
            .map_err(|e| anyhow::anyhow!("could not bind {}: {e}", path.display()))?;
        let bound = BoundSocket::of(path.to_path_buf()).map_err(|e| {
            anyhow::anyhow!("identifying the socket {} just bound: {e}", path.display())
        })?;
        Ok((Listener(listener), bound))
    }

    #[cfg(windows)]
    async fn bind(path: &Path) -> Result<(Listener, BoundSocket)> {
        let first = Self::instance(path, true).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                anyhow::anyhow!(
                    "the pipe {} is already held by another process; this daemon will not serve beside it",
                    path.display()
                )
            } else {
                anyhow::anyhow!("could not create the pipe {}: {e}", path.display())
            }
        })?;
        Ok((
            Listener {
                name: path.to_path_buf(),
                next: first,
            },
            BoundSocket,
        ))
    }

    /// One instance of the pipe at `name`, this user's alone.
    #[cfg(windows)]
    fn instance(name: &Path, first: bool) -> std::io::Result<Stream> {
        acquisition_store::world_windows::with_user_only_attributes(|attrs| {
            // SAFETY: `attrs` is a SECURITY_ATTRIBUTES that lives until
            // `with_user_only_attributes` returns, after this call.
            unsafe {
                tokio::net::windows::named_pipe::ServerOptions::new()
                    .first_pipe_instance(first)
                    .reject_remote_clients(true)
                    .create_with_security_attributes_raw(name, attrs)
            }
        })
    }

    /// The next connection.
    async fn accept(&mut self) -> std::io::Result<Stream> {
        #[cfg(unix)]
        {
            Ok(self.0.accept().await?.0)
        }
        #[cfg(windows)]
        {
            self.next.connect().await?;
            let fresh = Self::instance(&self.name, false)?;
            Ok(std::mem::replace(&mut self.next, fresh))
        }
    }
}

/// The socket a daemon bound, with the file's identity at the time, so
/// the exit path removes that file and not a successor's at the same
/// path. Unix only: a Windows pipe leaves nothing behind.
#[cfg(unix)]
struct BoundSocket {
    path: PathBuf,
    dev: u64,
    ino: u64,
}
#[cfg(windows)]
struct BoundSocket;

impl BoundSocket {
    /// Identify the file just bound at `path`.
    #[cfg(unix)]
    fn of(path: PathBuf) -> std::io::Result<BoundSocket> {
        use std::os::unix::fs::MetadataExt;
        let meta = std::fs::metadata(&path)?;
        Ok(BoundSocket {
            dev: meta.dev(),
            ino: meta.ino(),
            path,
        })
    }

    /// Remove the socket file if it is still the one bound; leave a
    /// successor's alone.
    fn remove_if_still_ours(&self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let Ok(meta) = std::fs::symlink_metadata(&self.path)
                && meta.dev() == self.dev
                && meta.ino() == self.ino
            {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }
}

pub struct Daemon {
    shared: Mutex<Shared>,
    /// On `Daemon`, not `Shared`, so `log`'s uptime prefix needs no lock:
    /// `persist` logs failures while holding `shared`.
    started: Instant,
    events: broadcast::Sender<JobInfo>,
    work: Notify,
    log: Mutex<std::fs::File>,
    /// The single rate-limit choke point (CONTEXT invariant 1). It owns the
    /// HTTP client, so all outbound requests — OAuth included — consult the
    /// header-driven limiter and feed their responses back to it.
    choke: ChokePoint,
    /// Real GGG by default; the in-process mock when the daemon was
    /// started under `ACQ_PROVIDER=mock` (C88).
    provider: Provider,
    /// The executable this daemon runs from, identified and hashed once
    /// at startup (C84, `artifact.rs`); what `hello` reports. `None` in
    /// the in-process harness, which runs from no executable of its own.
    artifact: Option<Artifact>,
    credential_store: Arc<dyn CredentialStore>,
    /// The world this daemon serves (C83): its canonical root, as `hello`
    /// names it. `None` in the in-process harness, which serves no world.
    world: Option<String>,
    /// The log file this daemon opened, as `daemon_status` reports it —
    /// the path as opened, exactly (a log root that is not valid UTF-8 is
    /// refused at start), never recomputed by a client from its own
    /// environment (review 2026-09-11). `None` in the in-process harness.
    log_path: Option<String>,
    /// The socket this daemon bound (C83: derived from its world into the
    /// runtime directory), removed when it exits — only while the path
    /// still names the file this daemon bound: a successor on the same
    /// world may have replaced it while this daemon was on its way out,
    /// and its rendezvous must not be unlinked from under it (seen in a
    /// rehearsal, 2026-09-11). `None` in the in-process harness, which
    /// binds nothing.
    socket_path: Option<BoundSocket>,
    /// The provider's store directory (`acquisition-store`): one file per
    /// account plus the account index. `None` in tests: nothing recorded.
    store_dir: Option<PathBuf>,
    /// The open store for the current session's account, opened on first
    /// record and reopened when the session's username changes. The daemon
    /// only ever writes it.
    store: Mutex<Option<(String, Store)>>,
    /// The persisted queue (`daemon.db`), mirrored from memory at every
    /// state change and read at start. Always present: `run` refuses to
    /// start without it, and test daemons get a throwaway in-memory one —
    /// so every test exercises the mirror.
    jobs_db: Mutex<JobDb>,
    /// Set (sticky) by the first failed queue write. A daemon whose
    /// mirror diverges from memory must not take or dispatch new work:
    /// finished jobs would replay and ids could repeat after a restart.
    /// Running jobs finish (their sends are committed); everything else
    /// refuses until a restart finds a working `daemon.db`.
    queue_failure: Mutex<Option<String>>,
}

struct RefreshOwnerGuard<'a> {
    daemon: &'a Daemon,
    account: String,
    id: u64,
    generations: AuthGenerations,
    result: Option<watch::Sender<Option<AccessTokenResult>>>,
}

impl RefreshOwnerGuard<'_> {
    fn finish(mut self, refresh: Result<auth::TokenResponse, Refusal>) -> AccessTokenResult {
        let result = self.result.as_ref().expect("refresh owner result exists");
        let outcome =
            self.daemon
                .finish_refresh(&self.account, self.id, self.generations, refresh, result);
        self.result = None;
        outcome
    }
}

impl Drop for RefreshOwnerGuard<'_> {
    fn drop(&mut self) {
        let Some(result) = self.result.take() else {
            return;
        };
        if result.borrow().is_some() {
            return;
        }
        {
            let mut s = self.daemon.shared.lock().unwrap();
            if let Some(session) = s.auth.find_mut(&self.account) {
                let owns_current_flight = session.generations == self.generations
                    && session.refresh_flight.as_ref().is_some_and(|flight| {
                        flight.id == self.id && flight.generations == self.generations
                    });
                if owns_current_flight {
                    session.refresh_flight = None;
                }
            }
        }
        result.send_replace(Some(Err(Refusal::new(
            ErrorKind::Internal,
            REFRESH_OWNER_ABANDONED,
        ))));
    }
}

trait CredentialStore: Send + Sync {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String>;
    fn clear(&self, service: &str, username: &str) -> Result<(), String>;
}

struct OsCredentialStore;

impl CredentialStore for OsCredentialStore {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
        auth::keyring_save(service, refresh_token, username)
    }

    fn clear(&self, service: &str, username: &str) -> Result<(), String> {
        auth::keyring_clear(service, username)
    }
}

impl Daemon {
    /// Seconds since the Unix epoch on the daemon's clock, for persisted
    /// rows — the same wall face the journal stamps, so a scenario's rows
    /// and its journal agree.
    fn unix_now(&self) -> i64 {
        self.choke
            .wall()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// Seconds this daemon has run, on its clock.
    fn uptime_seconds(&self) -> u64 {
        self.choke
            .now()
            .saturating_duration_since(self.started)
            .as_secs()
    }

    // Takes the shared lock for the uptime stamp — never call while holding it.
    fn log(&self, msg: &str) {
        let uptime = self.uptime_seconds();
        let mut f = self.log.lock().unwrap();
        let _ = writeln!(f, "[{uptime:>5}s] {msg}");
    }

    fn emit(&self, info: JobInfo) {
        self.log(&format!("job {} -> {}", info.id, info.state));
        let _ = self.events.send(info);
    }

    /// Log an error and keep it in the dashboard's recent-errors ring.
    fn note_error(&self, msg: &str) {
        {
            let now = self.choke.now();
            let mut s = self.shared.lock().unwrap();
            if s.errors.len() >= ERROR_HISTORY {
                s.errors.pop_front();
            }
            s.errors.push_back((now, msg.to_string()));
        }
        self.log(msg);
    }

    /// Mirror one job to `daemon.db`. Called with the `Shared` lock held,
    /// so the table sees changes in memory's order. A failed write trips
    /// the sticky queue failure (logged here — safe, `log` takes no lock
    /// but its own — and refused loudly at the next submit): disk has
    /// diverged from memory, so no new work may be taken.
    fn persist(&self, entry: &Entry) -> bool {
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
    fn queue_failed(&self) -> Option<String> {
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
    fn restore_jobs(&self, retention: Retention) -> Result<()> {
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
    fn stored_outcome(&self, id: JobId) -> Result<Option<Outcome>, Refusal> {
        match self.jobs_db.lock().unwrap().get(id) {
            Ok(row) => Ok(row.and_then(Entry::from_row).and_then(|e| e.outcome)),
            Err(e) => Err(Refusal::new(
                ErrorKind::QueueFailed,
                format!("could not read the persisted queue: {e:#}"),
            )),
        }
    }

    /// Apply one change to the account index, reporting failure to the
    /// caller. Safe under the shared lock (it never calls `note_error`).
    /// No-op in tests (no store directory).
    fn index_apply(&self, f: impl FnOnce(&mut Index) -> anyhow::Result<()>) -> Result<(), String> {
        let Some(dir) = &self.store_dir else {
            return Ok(());
        };
        Index::load(dir)
            .and_then(|mut index| f(&mut index))
            .map_err(|e| format!("accounts index: {e:#}"))
    }

    /// Apply one change to the account index, logging (never failing) on
    /// error. Not for callers holding the shared lock (`note_error` takes
    /// it) — those use `index_apply`.
    fn with_index(&self, f: impl FnOnce(&mut Index) -> anyhow::Result<()>) {
        if let Err(e) = self.index_apply(f) {
            self.note_error(&e);
        }
    }

    /// Hand a successful API body to the shared store. The daemon's whole
    /// involvement: endpoint + params + body; what is inside is the store's
    /// business. The result is classified: a body the store refuses as
    /// malformed (`acquisition_store::MalformedBody` — the response itself
    /// is bad, and ingesting it would have poisoned facts) comes back as
    /// the job's `Outcome::Failure`, so the caller returns it instead of
    /// reporting success; the store keeps that body verbatim in its
    /// `refused` table and the failure names the row, so the finding is
    /// readable without a re-fetch. Genuine persistence trouble (an unopenable file,
    /// a write error) stays logged-and-absorbed — the send happened and
    /// the payload still reaches the client that asked. The list-shaped
    /// jobs also pre-check their top-level array (a nicer early failure,
    /// and `refresh` needs the list anyway); entry-level judgment lives in
    /// the store alone and propagates from here.
    fn record(
        &self,
        account: Option<&str>,
        kind: &str,
        params: &Value,
        body: &Value,
    ) -> Result<(), Outcome> {
        let Some(dir) = &self.store_dir else {
            return Ok(());
        };
        let Some(endpoint) = Endpoint::from_job(kind, params) else {
            return Ok(());
        };
        // The job's account — fixed at submit — selects the file, never the
        // session at landing time: a login as B while A's refresh is still
        // landing must not file A's tabs under B.
        let Some(account) = account else {
            self.note_error(&format!(
                "store: {kind} landed with no account; not recorded"
            ));
            return Ok(());
        };
        let mut guard = self.store.lock().unwrap();
        if guard.as_ref().is_none_or(|(u, _)| u != account) {
            let path = account_path(dir, account);
            match Store::open(&path) {
                Ok(store) => {
                    self.log(&format!("store: {} opened for {account}", path.display()));
                    *guard = Some((account.to_string(), store));
                }
                Err(e) => {
                    self.note_error(&format!("store: could not open {}: {e:#}", path.display()));
                    return Ok(());
                }
            }
        }
        let (_, store) = guard.as_mut().expect("store opened above");
        let result = store.record(&endpoint, params, 200, body, acquisition_store::now());
        match result {
            Ok(ingest) => {
                self.log(&format!(
                    "store: {kind} {} -> response {} | {} items (+{} ~{} >{} -{}){}",
                    target_of(kind, params),
                    ingest.response_id,
                    ingest.items,
                    ingest.added,
                    ingest.changed,
                    ingest.moved,
                    ingest.removed,
                    match ingest.withheld {
                        Some(n) => format!(
                            " | WITHHELD: the location was retired by a listing; {n} item fact(s) kept on the response row, nothing landed"
                        ),
                        None => String::new(),
                    }
                ));
                Ok(())
            }
            Err(e) => match e.downcast_ref::<acquisition_store::MalformedBody>() {
                Some(malformed) => Err(Outcome::failure(malformed.to_string())),
                None => {
                    self.note_error(&format!("store: recording {kind} failed: {e:#}"));
                    Ok(())
                }
            },
        }
    }

    fn rails(&self) -> &Arc<Rails> {
        self.choke.rails()
    }

    /// Log a rails trip once, the first time any send path notices it.
    fn announce_trip(&self) {
        if let Some(cause) = self.rails().take_unannounced_trip() {
            let until = if cause.starts_with("ceiling:") {
                "for the rest of this daemon's lifetime"
            } else {
                "until `acq daemon reset-tripwire`"
            };
            self.note_error(&format!(
                "LIVE-TEST RAILS TRIPPED: {cause} — sends refused {until}"
            ));
        }
    }

    /// The route label and URL a job sends on, if it sends at all. Routes,
    /// not URLs, key the limiter: every league shares `stash-list`. A
    /// realm other than pc is a segment on the URL *and* on the label
    /// (`stash-list/xbox`): a realm's URL shape gets its own free HEAD
    /// probe before its first counted send, and whether it shares the pc
    /// policy is learned from its headers (same-name policies already
    /// share state, N6). pc adds nothing to either, so every pc URL and
    /// journal route is byte-identical to the pre-realm ones. `fetch` is a
    /// fake data endpoint that exists only on the mock.
    fn route_for(&self, kind: &str, params: &Value) -> Option<(String, String)> {
        let base = &self.provider.api_base;
        // Admission (`admit_realm`) refused anything a family does not
        // take before a job existed; a row that still fails here (a
        // persisted job from a build with a different table) sends nothing.
        let realm = |family: Family| family.realm_of(params).ok().map(Realm::infix);
        match kind {
            "characters" => {
                let r = realm(Family::Characters)?;
                Some((format!("character-list{r}"), format!("{base}/character{r}")))
            }
            // One character with its inventory/equipment: its own policy.
            "character" => {
                let r = realm(Family::Characters)?;
                let name = params.get("name").and_then(Value::as_str)?;
                Some((
                    format!("character{r}"),
                    format!("{base}/character{r}/{name}"),
                ))
            }
            // The account's leagues: `GET /account/leagues` (account:leagues).
            // `/league` is the public league list and needs `service:leagues`,
            // which the registration does not have (first contact
            // 2026-08-30: 403 `insufficient_scope`). Not realm-aware: no
            // consumer asks, and the plan never lists leagues.
            "leagues" => Some(("league".into(), format!("{base}/account/leagues"))),
            // The account profile (account:profile).
            "profile" => Some(("profile".into(), format!("{base}/profile"))),
            "stashes" | "refresh" => {
                let r = realm(Family::Stashes)?;
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard");
                Some((
                    format!("stash-list{r}"),
                    format!("{base}/stash{r}/{league}"),
                ))
            }
            // One tab, or one substash of a map/unique tab: same route, same
            // policy (stash-request-limit), one probe for all of them.
            "stash" => {
                let r = realm(Family::Stashes)?;
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard");
                let id = params.get("id").and_then(Value::as_str)?;
                let url = match params.get("sub").and_then(Value::as_str) {
                    Some(sub) => format!("{base}/stash{r}/{league}/{id}/{sub}"),
                    None => format!("{base}/stash{r}/{league}/{id}"),
                };
                Some((format!("stash{r}"), url))
            }
            "fetch" if !self.provider.is_real() => Some(("fetch".into(), format!("{base}/fetch"))),
            _ => None,
        }
    }

    /// `route_for`, keyed by the job's account: the limiter paces
    /// `Account`-scoped policies per account (rung 11) and serializes per
    /// `(account, policy)`; probes are per `(account, route)`.
    fn keyed_route_for(
        &self,
        kind: &str,
        params: &Value,
        account: Option<&str>,
    ) -> Option<(String, String)> {
        let (route, url) = self.route_for(kind, params)?;
        Some((endpoint_key(&route, account), url))
    }

    /// Whether a route needs the bearer token. The mock's `fetch` is open;
    /// everything on the real API is not.
    fn needs_auth(&self, route: &str) -> bool {
        self.provider.is_real() || route != "fetch"
    }

    /// What a job must not overlap with: probes with each other (N18),
    /// network jobs with anything under the same policy, everything else
    /// only with itself.
    fn serial_key(&self, e: &Entry) -> String {
        if e.info.kind == "probe" {
            return "probe".into();
        }
        match self.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref()) {
            Some((route, _)) => self.choke.serial_key(&route),
            None => format!("solo:{}", e.info.id),
        }
    }

    fn probe_pending(s: &Shared, route: &str) -> bool {
        s.jobs.values().any(|e| {
            e.info.kind == "probe"
                && !e.info.state.is_terminal()
                && e.params.get("route").and_then(Value::as_str) == Some(route)
        })
    }

    /// Per-route knowledge about GGG that headers cannot teach, kept in
    /// one place (`RUN-LEDGER.md`, 2026-08-30):
    /// - `/profile`: HEAD is answered 403, and GET answers 200 with no
    ///   `X-Rate-Limit-*` headers at all — not probed, may be policyless.
    /// - `/account/leagues`: HEAD is answered 200 and **counted** as a hit
    ///   (state `1:10:0` after the probe; N24's free HEAD is per endpoint),
    ///   so a probe costs what the GET costs — not probed; the GET teaches.
    ///
    /// Every other route is probed (HEAD uncounted) and observed strictly.
    /// Owner decisions 2026-08-30, pending GGG's word on both endpoints.
    fn declare_route_knowledge(choke: &ChokePoint) {
        choke.declare_policyless("profile");
    }

    /// Routes whose first use is not preceded by a HEAD probe (see
    /// `declare_route_knowledge`).
    const NO_PROBE_ROUTES: &'static [&'static str] = &["profile", "league"];

    /// Whether a route's first use is preceded by a HEAD probe. `route` is
    /// the endpoint key (`profile@user`).
    fn route_probes(route: &str) -> bool {
        !Self::NO_PROBE_ROUTES.contains(&crate::ratelimit::split_endpoint_key(route).0)
    }

    /// Make sure a probe for `route` is queued or running; submit one if not.
    /// One probe per route per daemon lifetime in the normal case — the
    /// sanctioned "one HEAD at startup" (N16), sent lazily on first use.
    fn ensure_probe(&self, route: &str, url: &str, account: Option<String>) {
        let pending = Self::probe_pending(&self.shared.lock().unwrap(), route);
        if !pending {
            self.log(&format!(
                "route {route} unknown; probing {} first",
                url_path(url)
            ));
            if let Err(e) = self.submit(
                "probe".into(),
                json!({ "route": route, "url": url }),
                PROBE_PRIORITY,
                "daemon".into(),
                account,
            ) {
                // Queue failed: the dispatcher has stopped picking, so the
                // jobs behind this probe wait rather than spin.
                self.log(&format!("JOBS: could not queue a probe for {route}: {e}"));
            }
        }
    }

    /// Turn a client's account selector into the account a job runs as:
    /// the selector must name a live session, or be absent with exactly
    /// one live; anything else is refused at submit, before a job exists.
    fn resolve_account(
        &self,
        kind: &str,
        requested: Option<&str>,
    ) -> Result<Option<String>, Refusal> {
        let s = self.shared.lock().unwrap();
        match requested {
            Some(req) => match s.auth.matching(req)? {
                Some(session) => Ok(session.username.clone()),
                None if s.auth.by_account.is_empty() => Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!("account {req:?}: not logged in — run `acq auth`"),
                )),
                None => Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!(
                        "account {req:?} is not logged in (live: {}); log in as it first",
                        s.auth.usernames().join(", ")
                    ),
                )),
            },
            // A job that will need a token must not exist without an
            // account: it would slip past the token-time check. With
            // several sessions live, nothing is guessed.
            None if !self.kind_needs_account(kind) => Ok(None),
            None => s.auth.get(None).map(|session| session.username.clone()),
        }
    }

    /// A client's selector (username, name without discriminator, or uuid)
    /// as the live session's canonical username; `None` stays `None` (the
    /// sole session, or a refusal downstream when several are live).
    fn canonical_account(&self, selector: Option<&str>) -> Result<Option<String>, Refusal> {
        let Some(sel) = selector else { return Ok(None) };
        let s = self.shared.lock().unwrap();
        s.auth
            .matching(sel)?
            .and_then(|x| x.username.clone())
            .map(Some)
            .ok_or_else(|| no_session(sel))
    }

    /// Every kind that sends with a token. `sleep` never sends; the mock's
    /// `fetch` is open.
    fn kind_needs_account(&self, kind: &str) -> bool {
        match kind {
            "sleep" => false,
            "fetch" => self.provider.is_real(),
            _ => true,
        }
    }

    fn submit(
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
    fn submit_child(&self, parent: JobId, kind: &str, params: Value) -> Result<JobId, Refusal> {
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
    fn submit_with_parent(
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

    fn cancel(&self, id: JobId) -> Result<(), Refusal> {
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

    fn set_priority(&self, id: JobId, priority: Priority) -> Result<(), Refusal> {
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

    fn cancelled(&self, id: JobId) -> bool {
        let s = self.shared.lock().unwrap();
        s.jobs.get(&id).map(|e| e.cancel_requested).unwrap_or(true)
    }

    // ---- dispatcher -----------------------------------------------------

    /// Starts at most one task per scheduling key. N4's gate, not this
    /// dispatcher, owns actual-send capacity. Woken by submits, completions,
    /// and reprioritization.
    async fn dispatcher(self: Arc<Self>) {
        loop {
            let picks = self.pick_runnable();
            for id in picks {
                tokio::spawn(self.clone().run_active(id));
            }
            self.work.notified().await;
        }
    }

    /// Waiting jobs, in dispatch order, that can start a task right now.
    fn pick_runnable(&self) -> Vec<JobId> {
        let mut s = self.shared.lock().unwrap();
        // A failed queue means no new dispatch at all: every transition
        // a job makes from here would widen the memory/disk divergence.
        if self.queue_failed().is_some() {
            return Vec::new();
        }
        let mut busy: HashSet<String> = s.active_jobs.values().cloned().collect();
        let halted = self.rails().halted().is_some();
        let mut picks = Vec::new();
        for id in s.queue_order() {
            let entry = &s.jobs[&id];
            if let Some((route, _)) = self.keyed_route_for(
                &entry.info.kind,
                &entry.params,
                entry.info.account.as_deref(),
            ) {
                // A halted daemon (the rails, `rails.rs`) sends nothing:
                // network jobs wait, on disk, for `reset-tripwire`.
                if halted {
                    continue;
                }
                // A job whose route is still being probed has nothing to do yet.
                if self.choke.endpoint_state(&route) == EndpointState::Unknown
                    && Self::probe_pending(&s, &route)
                {
                    continue;
                }
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
        let (route, account) = {
            let s = self.shared.lock().unwrap();
            match s.jobs.get(&id) {
                Some(e) => (
                    self.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref()),
                    e.info.account.clone(),
                ),
                None => return,
            }
        };

        // L0 rail 1: a halted daemon sends nothing — no probe, no pacing
        // wait, no permit. The job stays waiting (the dispatcher will not
        // pick it again until the reset); its key is given back.
        if route.is_some() && self.rails().halted().is_some() {
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
        let kind = info.kind.clone();
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
    fn conclude(&self, id: JobId, outcome: Outcome) {
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
    fn maybe_finish_parent(&self, pid: JobId) {
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
    fn finish(&self, id: JobId, outcome: Outcome) {
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
    fn start_and_finish(&self, id: JobId, outcome: Outcome) {
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
    async fn execute(
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
        }
    }

    async fn execute_inner(
        &self,
        id: JobId,
        kind: &str,
        params: Value,
        route: Option<String>,
        account: Option<&str>,
        ready: Instant,
    ) -> Result<Outcome, ApiError> {
        Ok(match kind {
            // The one real API call: GET {api_base}/character. Same code in
            // both modes; only the provider's base URL differs. The choke
            // point classifies and observes every response; the dispatcher
            // requeues only acceptable 429s, while Cloudflare-shaped blocks
            // are never retried.
            "characters" => {
                let (token, username) = match self.valid_access_token(account, false).await {
                    Ok(pair) => pair,
                    Err(error) => {
                        return Ok(Outcome::failure(error.to_string()));
                    }
                };
                let Some((_, url)) = self.route_for(kind, &params) else {
                    return Ok(Outcome::failure("characters: unrenderable realm"));
                };
                let route = route.as_deref().expect("characters is a network kind");
                let auth = Auth::Session {
                    token: token.clone(),
                    username: username.clone(),
                };
                let (v, rate) = self.api_get(route, &url, auth, ready).await?;
                // A 2xx without the `characters` array is a malformed
                // response, not an empty account: the job fails and
                // nothing is recorded (the store would refuse it too —
                // ingesting it as empty would retire every character).
                let Some(characters) = v.get("characters").and_then(Value::as_array) else {
                    return Ok(Outcome::failure(
                        "characters response without a `characters` array",
                    ));
                };
                let characters = json!(characters);
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "characters": characters,
                        "rate_limit": rate,
                    }),
                }
            }
            "character" | "leagues" | "profile" => {
                // A profile job during a login runs on the staged tokens
                // (`staged_profile_token`): the account may have no
                // registered session yet, or a dead one being replaced —
                // the new tokens are the ones being proven.
                let staged = (kind == "profile")
                    .then(|| self.staged_profile_token(account))
                    .flatten();
                let (auth, username) = match staged {
                    Some((token, username)) => (Auth::Staged(token), username),
                    None => match self.valid_access_token(account, false).await {
                        Ok((token, username)) => (
                            Auth::Session {
                                token,
                                username: username.clone(),
                            },
                            username,
                        ),
                        Err(error) => {
                            return Ok(Outcome::failure(error.to_string()));
                        }
                    },
                };
                let Some((_, url)) = self.route_for(kind, &params) else {
                    return Ok(Outcome::failure(format!("{kind} needs a name")));
                };
                let route = route.as_deref().expect("network kind");
                let (v, rate) = self.api_get(route, &url, auth, ready).await?;
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                // The profile's uuid is the stable account identity,
                // required at login (`complete_login` writes the index
                // entry). Recording it whenever a later profile lands too
                // keeps the mapping fresh across renames and backfills
                // pre-uuid entries.
                if kind == "profile"
                    && let (Some(account), Some(uuid)) =
                        (account, v.get("uuid").and_then(Value::as_str))
                {
                    {
                        let mut s = self.shared.lock().unwrap();
                        if let Some(session) = s.auth.find_mut(account) {
                            session.uuid = Some(uuid.to_string());
                        }
                    }
                    self.with_index(|index| index.set_uuid(account, uuid));
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "params": params,
                        "body": v,
                        "rate_limit": rate,
                    }),
                }
            }
            // The stash list: one request under stash-list-request-limit, the
            // second real policy. Tab contents are a later step.
            "stashes" => {
                let (token, username) = match self.valid_access_token(account, false).await {
                    Ok(pair) => pair,
                    Err(error) => {
                        return Ok(Outcome::failure(error.to_string()));
                    }
                };
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard")
                    .to_string();
                let Some((_, url)) = self.route_for(kind, &params) else {
                    return Ok(Outcome::failure("stashes: unrenderable realm"));
                };
                let route = route.as_deref().expect("stashes is a network kind");
                let auth = Auth::Session {
                    token: token.clone(),
                    username: username.clone(),
                };
                let (v, rate) = self.api_get(route, &url, auth, ready).await?;
                // A 2xx without the `stashes` array is a malformed
                // response, not an empty account: the job fails and
                // nothing is recorded — ingested as empty it would retire
                // every tab and mint a false listing basis.
                let Some(stashes) = v.get("stashes").and_then(Value::as_array) else {
                    return Ok(Outcome::failure(
                        "stashes response without a `stashes` array",
                    ));
                };
                let stashes = json!(stashes);
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "username": username,
                        "league": league,
                        "stashes": stashes,
                        "rate_limit": rate,
                    }),
                }
            }
            "stash" => {
                let (token, username) = match self.valid_access_token(account, false).await {
                    Ok(pair) => pair,
                    Err(error) => {
                        return Ok(Outcome::failure(error.to_string()));
                    }
                };
                let Some((_, url)) = self.route_for("stash", &params) else {
                    return Ok(Outcome::failure("stash needs an id"));
                };
                let route = route.as_deref().expect("stash is a network kind");
                let auth = Auth::Session {
                    token: token.clone(),
                    username: username.clone(),
                };
                let (v, rate) = self.api_get(route, &url, auth, ready).await?;
                if let Err(failure) = self.record(account, kind, &params, &v) {
                    return Ok(failure);
                }
                let stash = v.get("stash").cloned().unwrap_or(v);
                // Map/unique tabs carry their substashes as stubs; following
                // them is opt-in per tab (--deep) because one map tab can
                // hold hundreds. Each substash becomes a child job.
                let deep = params.get("deep").and_then(Value::as_bool).unwrap_or(false);
                let children = stash
                    .get("children")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let mut submitted = Vec::new();
                if deep {
                    let realm = Realm::from_params(&params).unwrap_or(Realm::DEFAULT);
                    let league = params.get("league").cloned().unwrap_or(json!("Standard"));
                    let tab = params.get("id").cloned().unwrap_or(Value::Null);
                    for child in &children {
                        let Some(sub) = child.get("id").and_then(Value::as_str) else {
                            continue;
                        };
                        match self.submit_child(
                            id,
                            "stash",
                            json!({ "realm": realm, "league": league, "id": tab, "sub": sub, "deep": false }),
                        ) {
                            Ok(cid) => submitted.push(cid),
                            Err(e) => {
                                return Ok(fan_out_stopped(
                                    self.cancelled(id),
                                    submitted.len(),
                                    &e.message,
                                ));
                            }
                        }
                    }
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "stash": stash,
                        "substashes_listed": children.len(),
                        "substash_jobs": submitted,
                        "rate_limit": rate,
                    }),
                }
            }
            // A refresh: one stash-list request, then one `stash` child per
            // selected tab. Folder children come straight from the list
            // (folders themselves are never fetched); map/unique substashes
            // only if `deep`. Selection is explicit — there is no default.
            "refresh" => {
                let (token, username) = match self.valid_access_token(account, false).await {
                    Ok(pair) => pair,
                    Err(error) => {
                        return Ok(Outcome::failure(error.to_string()));
                    }
                };
                let league = params
                    .get("league")
                    .and_then(Value::as_str)
                    .unwrap_or("Standard")
                    .to_string();
                let deep = params.get("deep").and_then(Value::as_bool).unwrap_or(false);
                let all = params.get("all").and_then(Value::as_bool).unwrap_or(false);
                let wanted: Vec<String> = params
                    .get("tabs")
                    .and_then(Value::as_array)
                    .map(|a| {
                        a.iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default();
                if !all && wanted.is_empty() {
                    return Ok(Outcome::failure("refresh needs --all or --tabs <id,...>"));
                }
                let realm = Realm::from_params(&params).unwrap_or(Realm::DEFAULT);
                let Some((_, url)) = self.route_for(kind, &params) else {
                    return Ok(Outcome::failure("refresh: unrenderable realm"));
                };
                let route = route.as_deref().expect("refresh is a network kind");
                let auth = Auth::Session {
                    token: token.clone(),
                    username: username.clone(),
                };
                let (v, rate) = self.api_get(route, &url, auth, ready).await?;
                // A malformed listing fails the refresh whole, before the
                // store sees it: converted to an empty list it would
                // "succeed" with zero children over a retired tab set.
                let Some(listed) = v.get("stashes").and_then(Value::as_array).cloned() else {
                    return Ok(Outcome::failure(
                        "stashes response without a `stashes` array",
                    ));
                };
                // The list a refresh fetches is the same response `stashes`
                // records; the store wants it under that endpoint.
                if let Err(failure) = self.record(
                    account,
                    "stashes",
                    &json!({ "realm": realm, "league": params.get("league").cloned().unwrap_or(json!("Standard")) }),
                    &v,
                ) {
                    return Ok(failure);
                }
                // Flatten: top-level tabs plus folder children; skip folders.
                let mut tabs: Vec<(String, String, String)> = Vec::new();
                for t in &listed {
                    let ty = t.get("type").and_then(Value::as_str).unwrap_or("");
                    if ty == "Folder" {
                        for c in t
                            .get("children")
                            .and_then(Value::as_array)
                            .into_iter()
                            .flatten()
                        {
                            if let Some(cid) = c.get("id").and_then(Value::as_str) {
                                tabs.push((
                                    cid.into(),
                                    c.get("name").and_then(Value::as_str).unwrap_or("").into(),
                                    c.get("type").and_then(Value::as_str).unwrap_or("").into(),
                                ));
                            }
                        }
                    } else if let Some(tid) = t.get("id").and_then(Value::as_str) {
                        tabs.push((
                            tid.into(),
                            t.get("name").and_then(Value::as_str).unwrap_or("").into(),
                            ty.into(),
                        ));
                    }
                }
                let selected: Vec<&(String, String, String)> = tabs
                    .iter()
                    .filter(|(tid, _, _)| all || wanted.contains(tid))
                    .collect();
                let unknown: Vec<&String> = wanted
                    .iter()
                    .filter(|w| !tabs.iter().any(|(tid, _, _)| tid == *w))
                    .collect();
                let mut submitted = Vec::new();
                for (tid, _, ty) in &selected {
                    let follow = deep && matches!(ty.as_str(), "MapStash" | "UniqueStash");
                    match self.submit_child(
                        id,
                        "stash",
                        json!({ "realm": realm, "league": league, "id": tid, "deep": follow }),
                    ) {
                        Ok(cid) => submitted.push(cid),
                        Err(e) => {
                            return Ok(fan_out_stopped(
                                self.cancelled(id),
                                submitted.len(),
                                &e.message,
                            ));
                        }
                    }
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "league": league,
                        "tabs_listed": tabs.len(),
                        "tabs_selected": selected.iter().map(|(i, n, t)| json!({ "id": i, "name": n, "type": t })).collect::<Vec<_>>(),
                        "unknown_tab_ids": unknown,
                        "deep": deep,
                        "tab_jobs": submitted,
                        "rate_limit": rate,
                    }),
                }
            }
            // A plan's action set, executed exactly (tracer step 7): a pure
            // fan-out parent. Each admitted tuple becomes one child job; the
            // parent sends nothing itself, so it has no route and no token —
            // children authenticate and pace themselves. Admission validated
            // the vocabulary at submit; it is checked again here because a
            // restored `daemon.db` row was admitted by an earlier lifetime,
            // possibly an earlier build.
            "apply" => {
                if let Err(error) = validate_apply(&params) {
                    return Ok(Outcome::failure(error.to_string()));
                }
                let jobs = params
                    .get("jobs")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let mut submitted = Vec::new();
                for job in &jobs {
                    let kind = job.get("kind").and_then(Value::as_str).unwrap_or_default();
                    let child_params = job.get("params").cloned().unwrap_or_else(|| json!({}));
                    match self.submit_child(id, kind, child_params) {
                        Ok(cid) => submitted.push(cid),
                        Err(e) => {
                            return Ok(fan_out_stopped(
                                self.cancelled(id),
                                submitted.len(),
                                &e.message,
                            ));
                        }
                    }
                }
                Outcome::Success {
                    payload: json!({
                        "provider": self.provider.name,
                        "requests": submitted.len(),
                        "child_jobs": submitted,
                    }),
                }
            }
            "sleep" => {
                // Real seconds by design, not the daemon's clock: the mock's
                // demo and the watch tests want a job that visibly takes time.
                let seconds = params.get("seconds").and_then(Value::as_f64).unwrap_or(3.0);
                let deadline = Instant::now() + Duration::from_secs_f64(seconds);
                while Instant::now() < deadline {
                    if self.cancelled(id) {
                        return Ok(Outcome::Cancelled);
                    }
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
                Outcome::Success {
                    payload: json!({ "slept_seconds": seconds }),
                }
            }
            "fetch" => {
                // A real request to the mock's fake data endpoint, so the
                // limiter learns a second policy from real headers. Never
                // sent to GGG: real mode has no such endpoint.
                if self.provider.is_real() {
                    return Ok(Outcome::failure(
                        "fetch is a mock-only kind; real mode has no fake data endpoint",
                    ));
                }
                let url = format!("{}/fetch", self.provider.api_base);
                let route = route.as_deref().expect("mock fetch is a network kind");
                let (v, rate) = self.api_get(route, &url, Auth::None, ready).await?;
                Outcome::Success {
                    payload: json!({
                        "note": "fake data from the in-process mock",
                        "params": params,
                        "items": v.get("items").cloned().unwrap_or(v),
                        "rate_limit": rate,
                    }),
                }
            }
            "whoami" => {
                // The mock-only auth-exercising kind: access-token expiry and
                // silent refresh through the daemon-owned session. Fake data,
                // so in real mode it must not cost a token POST (L0 rail 6).
                if self.provider.is_real() {
                    return Ok(Outcome::failure(
                        "whoami is a mock-only job kind; not run against real GGG",
                    ));
                }
                let (token, username) = match self.valid_access_token(account, false).await {
                    Ok(pair) => pair,
                    Err(error) => {
                        return Ok(Outcome::failure(error.to_string()));
                    }
                };
                tokio::time::sleep(Duration::from_millis(300)).await;
                if self.cancelled(id) {
                    return Ok(Outcome::Cancelled);
                }
                Outcome::Success {
                    payload: json!({
                        "note": "fake data — the mock never talks to GGG",
                        "username": username,
                        "league": "Standard",
                        "characters": [
                            { "name": "StashHoarder", "class": "Scion", "level": 97 },
                            { "name": "MuleQuadTab", "class": "Witch", "level": 12 },
                        ],
                        "authorized_with": format!("{}…", token.chars().take(11).collect::<String>()),
                    }),
                }
            }
            // The HEAD probe (N16): discovers an endpoint's policy and the
            // account's current counters before the first real send, without
            // counting against the policy (N24). Submitted by the daemon
            // itself; visible like any other job so it can be inspected.
            "probe" => {
                let (Some(route), Some(url)) = (
                    params
                        .get("route")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    params
                        .get("url")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                ) else {
                    return Ok(Outcome::failure("probe needs a route and a url"));
                };
                let mut session = if self.needs_auth(&route) {
                    match self.valid_access_token(account, false).await {
                        Ok(pair) => Some(pair),
                        Err(error) => {
                            // Close the endpoint too, or the waiting job would
                            // just ask for another probe; login reopens it.
                            self.choke.degrade(&route, &error.message);
                            return Ok(Outcome::failure(error.message));
                        }
                    }
                } else {
                    None
                };
                let mut retried = false;
                let (probed, cleared_for) = loop {
                    let bearer = session
                        .as_ref()
                        .map(|(token, username)| SessionBearer::new(self, username, token));
                    let probed = self
                        .choke
                        .head(
                            &route,
                            &url,
                            bearer.as_ref().map(|b| b as &dyn Bearer),
                            ready,
                        )
                        .await;
                    let cleared_for = bearer
                        .as_ref()
                        .filter(|b| b.was_cleared())
                        .map(|b| b.username.to_string());
                    match probed {
                        // C87: the token this probe was to carry was rejected
                        // while it waited for the exclusive permit. Refresh
                        // outside it and try admission once more.
                        Err(ProbeError::StaleBearer) if !retried => {
                            retried = true;
                            let (_, username) = session
                                .as_ref()
                                .expect("only a session's bearer can go stale");
                            match self.valid_access_token(Some(username), false).await {
                                Ok(pair) => session = Some(pair),
                                Err(error) => {
                                    self.choke.degrade(&route, &error.message);
                                    return Ok(Outcome::failure(error.message));
                                }
                            }
                        }
                        other => break (other, cleared_for),
                    }
                };
                self.announce_trip();
                match probed {
                    Ok((status, policy, headers)) => {
                        let name = policy.name;
                        self.log(&format!(
                            "HEAD {} -> {status} | policy {name} | {headers}",
                            url_path(&url)
                        ));
                        Outcome::Success {
                            payload: json!({
                                "route": route,
                                "endpoint": url_path(&url),
                                "status": status.as_u16(),
                                "policy": name,
                                "rate_limit": headers,
                            }),
                        }
                    }
                    Err(error) => {
                        let mut message = format!("HEAD {}: {error}", url_path(&url));
                        if let Some(username) = cleared_for {
                            message.push_str(&format!(
                                " — the access token for {username} was rejected and is forgotten; the next send refreshes it first"
                            ));
                            // The limiter closed the route with the bare
                            // HEAD status as its reason; the job waiting
                            // on this probe reports that reason, so it
                            // is re-closed with the whole story.
                            self.choke.degrade(&route, &message);
                        }
                        Outcome::failure(message)
                    }
                }
            }
            other => Outcome::failure(format!(
                "unknown job kind '{other}' (kinds: sleep, fetch, whoami, profile, characters, character, leagues, stashes, stash, refresh, apply, probe)"
            )),
        })
    }

    /// One rate-limited GET: sends with the receipt, logs the rate headers,
    /// and turns non-2xx into a typed error. A 429 is distinguishable so the
    /// caller can re-queue the job behind the limiter's hold (P-A); a
    /// Cloudflare-shaped 403/503 is never retried (invariant 3).
    async fn api_get(
        &self,
        route: &str,
        url: &str,
        mut auth: Auth,
        ready: Instant,
    ) -> Result<(Value, Value), ApiError> {
        let mut retried = false;
        loop {
            let (sent, cleared_for) = match &auth {
                Auth::Session { token, username } => {
                    let bearer = SessionBearer::new(self, username, token);
                    let sent = self.choke.get_bearer(route, url, &bearer, ready).await;
                    (sent, bearer.was_cleared().then(|| username.clone()))
                }
                Auth::Staged(token) => (
                    self.choke
                        .get_bearer(route, url, &StagedBearer(token.clone()), ready)
                        .await,
                    None,
                ),
                Auth::None => (self.choke.get(route, url, ready).await, None),
            };
            let response = match sent {
                // C87: the token this send was to carry was rejected while
                // it waited at the gate. The permit is released; refresh
                // outside it and try admission once more.
                Err(SendError::StaleBearer) if !retried => {
                    retried = true;
                    let Auth::Session { username, .. } = &auth else {
                        unreachable!("only a session's bearer can go stale")
                    };
                    match self.valid_access_token(Some(username), false).await {
                        Ok((token, username)) => {
                            auth = Auth::Session { token, username };
                            continue;
                        }
                        Err(refusal) => {
                            return Err(ApiError::Other(format!(
                                "GET {}: {}",
                                url_path(url),
                                refusal.message
                            )));
                        }
                    }
                }
                Err(SendError::StaleBearer) => {
                    return Err(ApiError::Other(format!(
                        "GET {}: {} — twice; giving up",
                        url_path(url),
                        SendError::StaleBearer
                    )));
                }
                Err(SendError::Protocol(error)) => {
                    return Err(ApiError::Protocol(format!(
                        "GET {}: rate-limit protocol failure: {error}",
                        url_path(url)
                    )));
                }
                Err(SendError::Transport(error)) => {
                    return Err(ApiError::Other(format!("GET {url} failed: {error}")));
                }
                Err(SendError::Halted(cause)) => {
                    return Err(ApiError::Other(format!(
                        "GET {} refused: {cause}",
                        url_path(url)
                    )));
                }
                Ok(response) => response,
            };
            self.announce_trip();
            let status = response.status;
            let rate = response.rate;
            let retry_after = response.retry_after;
            let path = url_path(url);
            self.log(&format!("GET {path} -> {status} | rate headers: {rate}"));
            let classified = classify_api_body(status, &retry_after, &path, rate, response.body);
            if let Some(username) = cleared_for {
                return classified.map_err(|error| match error {
                    ApiError::Other(evidence) => ApiError::Other(format!(
                        "{evidence} — the access token for {username} was rejected and is forgotten; the next send refreshes it first"
                    )),
                    other => other,
                });
            }
            return classified;
        }
    }

    // ---- auth -----------------------------------------------------------

    /// Kick off a login: bind a loopback redirect listener, build the
    /// authorize URL, and spawn a task that waits for the browser callback
    /// and exchanges the code. Returns the URL for the user to open.
    async fn auth_start(self: &Arc<Self>) -> Result<String, Refusal> {
        let internal = |e: &dyn std::fmt::Display| Refusal::new(ErrorKind::Internal, e.to_string());
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| internal(&format!("could not bind loopback listener: {e}")))?;
        let port = listener.local_addr().map_err(|e| internal(&e))?.port();
        // The registered callback path; GGG accepts any loopback port.
        let redirect_uri = format!("http://127.0.0.1:{port}{CALLBACK_PATH}");
        let (verifier, challenge) = auth::pkce_pair();
        let state = auth::random_token("st");
        let mut authorize_url =
            url::Url::parse(&self.provider.authorize_url).map_err(|e| internal(&e))?;
        authorize_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", self.provider.client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256")
            .append_pair("scope", &SCOPES.join(" "));
        let authorize_url = authorize_url.to_string();
        let flow_generation = self.begin_auth_flow();
        self.log(&format!("auth flow started (callback on port {port})"));

        let daemon = self.clone();
        tokio::spawn(async move {
            let callback =
                tokio::time::timeout(Duration::from_secs(300), wait_callback(listener, &state))
                    .await;
            match callback {
                Ok(Ok(code)) => {
                    match auth::exchange_code(
                        &daemon.choke,
                        &daemon.provider,
                        &code,
                        &verifier,
                        &redirect_uri,
                    )
                    .await
                    {
                        Ok(tokens) => match daemon.stage_auth_flow(flow_generation, tokens) {
                            Some(staged) => {
                                daemon.login_with_profile(flow_generation, staged).await;
                            }
                            None => daemon.log("stale auth flow completion ignored"),
                        },
                        Err(e) => daemon.finish_auth_flow_error(
                            flow_generation,
                            &format!("token exchange failed: {e}"),
                        ),
                    }
                }
                Ok(Err(e)) => daemon
                    .finish_auth_flow_error(flow_generation, &format!("auth callback failed: {e}")),
                Err(_) => {
                    daemon.finish_auth_flow_error(flow_generation, "auth flow timed out after 300s")
                }
            }
        });
        Ok(authorize_url)
    }

    /// A login flow is daemon-wide (one browser at a time), not a session:
    /// it disturbs no live session until its tokens arrive and name one.
    fn begin_auth_flow(&self) -> u64 {
        let mut s = self.shared.lock().unwrap();
        s.auth.next_flow = s.auth.next_flow.wrapping_add(1);
        let generation = s.auth.next_flow;
        s.auth.pending = Some(generation);
        // A fresh flow owns the login state: an older flow's staged tokens
        // and result must not leak into it.
        s.auth.staging = None;
        s.auth.flow_result = None;
        generation
    }

    /// Token exchange succeeded: hold the tokens in the staging slot —
    /// outside the session map, so clients cannot reach them and a live
    /// session for the same account keeps serving its jobs untouched. The
    /// flow stays pending; nothing is registered until
    /// [`Self::complete_login`] has the uuid. `None`: the flow is stale (a
    /// newer login superseded it).
    fn stage_auth_flow(&self, generation: u64, tokens: auth::TokenResponse) -> Option<String> {
        let mut s = self.shared.lock().unwrap();
        if s.auth.pending != Some(generation) {
            return None;
        }
        let username = tokens.username.clone();
        s.auth.staging = Some(Staging {
            flow: generation,
            username: username.clone(),
            access_token: tokens.access_token,
            access_expires_at: self.choke.wall() + Duration::from_secs(tokens.expires_in),
            refresh_token: tokens.refresh_token,
        });
        Some(username)
    }

    /// The staged login's access token, handed out only for the profile
    /// route of the account being logged in: the login's own profile job
    /// must use the *new* tokens (the account may have no registered
    /// session yet, or a dead one being replaced), and no other work can
    /// reach them.
    fn staged_profile_token(&self, account: Option<&str>) -> Option<(String, String)> {
        let s = self.shared.lock().unwrap();
        s.auth
            .staging
            .as_ref()
            .filter(|st| Some(st.username.as_str()) == account)
            .map(|st| (st.access_token.clone(), st.username.clone()))
    }

    /// The second half of a login (C50): submit a
    /// profile job as the staged account — causal service of the client's
    /// `acq auth` — and register the session only when the uuid lands.
    /// Any failure fails the login whole: the staged tokens are dropped and
    /// nothing was written anywhere.
    async fn login_with_profile(&self, generation: u64, username: String) {
        let uuid = match self.submit(
            "profile".into(),
            json!({}),
            LOGIN_PRIORITY,
            "daemon".into(),
            Some(username.clone()),
        ) {
            Err(e) => Err(format!("could not queue the profile fetch: {e}")),
            Ok(id) => match self.wait_job_terminal(id, LOGIN_PROFILE_TIMEOUT).await {
                None => {
                    let _ = self.cancel(id);
                    Err(format!(
                        "profile fetch (job {id}) did not finish within {}s",
                        LOGIN_PROFILE_TIMEOUT.as_secs()
                    ))
                }
                Some(Outcome::Success { payload }) => payload
                    .get("body")
                    .and_then(|b| b.get("uuid"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| "profile response carried no uuid".to_string()),
                Some(Outcome::Failure { error, .. }) => {
                    Err(format!("profile fetch failed: {error}"))
                }
                Some(Outcome::Cancelled) => Err("profile fetch was cancelled".into()),
            },
        };
        match uuid {
            Ok(uuid) => {
                self.complete_login(generation, &username, &uuid);
            }
            Err(error) => self.abort_login(generation, &username, &error),
        }
    }

    /// The uuid landed: register the login in dependency order — keyring,
    /// then the index mapping (a login the index cannot record **fails
    /// whole**, keyring rolled back: a session without its uuid mapping
    /// could neither be restored nor find its annotation file), and only
    /// then the visible session and the flow's terminal result. A stale
    /// flow registers nothing.
    fn complete_login(&self, generation: u64, username: &str, uuid: &str) -> bool {
        enum Done {
            Stale,
            Failed(String),
            Registered { warning: Option<String> },
        }
        let done = {
            let mut s = self.shared.lock().unwrap();
            if s.auth.pending != Some(generation) {
                Done::Stale
            } else if let Some(staging) = s
                .auth
                .staging
                .take_if(|st| st.flow == generation && st.username == username)
            {
                // Keyring and index writes happen under the auth lock so
                // they are ordered with logout and re-login.
                let saved = self.credential_store.save(
                    self.provider.keyring_service,
                    &staging.refresh_token,
                    username,
                );
                let (keyring, warning) = match &saved {
                    Ok(()) => ("ok".to_string(), None),
                    Err(e) => (
                        format!("unavailable: {e}"),
                        Some(format!(
                            "keyring save failed: {e} (session is in-memory only)"
                        )),
                    ),
                };
                let persisted = saved.is_ok();
                let indexed = self.index_apply(|index| {
                    index.record_login(username, uuid, persisted, acquisition_store::now())
                });
                match indexed {
                    Err(e) => {
                        let mut error =
                            format!("could not record the login in the account index: {e}");
                        if persisted
                            && let Err(clear) = self
                                .credential_store
                                .clear(self.provider.keyring_service, username)
                        {
                            error = format!(
                                "{error}; the keyring entry could not be rolled back either: {clear}"
                            );
                        }
                        s.auth.pending = None;
                        s.auth.flow_result = Some(Err(error.clone()));
                        Done::Failed(error)
                    }
                    Ok(()) => {
                        // A re-login replaces the account's old session here
                        // — not before — so generations advance and an
                        // in-flight refresh for it lands stale.
                        let session = s.auth.replace(AuthSession {
                            username: Some(username.to_string()),
                            uuid: Some(uuid.to_string()),
                            keyring,
                            ..AuthSession::default()
                        });
                        session.access_token = Some(staging.access_token);
                        session.access_expires_at = Some(staging.access_expires_at);
                        session.refresh_token = Some(staging.refresh_token);
                        session.advance_access_token();
                        session.advance_refresh_token();
                        s.auth.pending = None;
                        s.auth.flow_result = Some(Ok(username.to_string()));
                        Done::Registered { warning }
                    }
                }
            } else {
                let error = "the staged tokens disappeared before the profile landed".to_string();
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.clone()));
                Done::Failed(error)
            }
        };
        match done {
            Done::Stale => {
                self.log("stale login completion ignored");
                false
            }
            Done::Failed(error) => {
                self.note_error(&format!("login failed for {username}: {error}"));
                false
            }
            Done::Registered { warning } => {
                // Only a completed login clears the dead-grant mark: until
                // here the keyring still held the rejected token.
                self.rails().clear_refresh_failed(username);
                // A probe that failed for lack of a session would succeed
                // now; don't make the user sit out the cooldown.
                self.choke.forget_degraded();
                self.log(&format!("logged in as {username} ({uuid})"));
                if let Some(warning) = warning {
                    self.note_error(&warning);
                }
                true
            }
        }
    }

    /// A login failed after token exchange: drop the staged tokens (it
    /// fails whole — no provisional identity) and, if the flow is still
    /// current, close it with the error as its terminal result.
    fn abort_login(&self, generation: u64, username: &str, error: &str) {
        let current = {
            let mut s = self.shared.lock().unwrap();
            if s.auth
                .staging
                .as_ref()
                .is_some_and(|st| st.flow == generation)
            {
                s.auth.staging = None;
            }
            if s.auth.pending == Some(generation) {
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.to_string()));
                true
            } else {
                false
            }
        };
        if current {
            self.note_error(&format!("login failed for {username}: {error}"));
        }
    }

    /// Wait until job `id` is terminal, returning its outcome (`None` on
    /// timeout). Daemon-internal; clients wait through the protocol.
    async fn wait_job_terminal(&self, id: JobId, timeout: Duration) -> Option<Outcome> {
        let mut rx = self.events.subscribe();
        tokio::time::timeout(timeout, async {
            loop {
                let outcome = {
                    let s = self.shared.lock().unwrap();
                    s.jobs
                        .get(&id)
                        .filter(|e| e.info.state.is_terminal())
                        .and_then(|e| e.outcome.clone())
                };
                if let Some(outcome) = outcome {
                    return outcome;
                }
                match rx.recv().await {
                    Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => std::future::pending().await,
                }
            }
        })
        .await
        .ok()
    }

    fn finish_auth_flow_error(&self, generation: u64, error: &str) {
        let current = {
            let mut s = self.shared.lock().unwrap();
            if s.auth.pending == Some(generation) {
                s.auth.pending = None;
                s.auth.flow_result = Some(Err(error.to_string()));
                true
            } else {
                false
            }
        };
        if current {
            self.note_error(error);
        }
    }

    /// Store fresh tokens in memory and mirror the refresh token (which the
    /// provider rotates on every grant) into the keyring. The caller holds
    /// the auth-state lock so keyring mutation is ordered with logout and
    /// re-authentication — and updates the account index *after* releasing
    /// it (`with_index` reports errors through `note_error`, which takes
    /// the same lock).
    fn install_tokens_locked(
        &self,
        session: &mut AuthSession,
        tokens: auth::TokenResponse,
    ) -> Option<String> {
        let (keyring, warning) = match self.credential_store.save(
            self.provider.keyring_service,
            &tokens.refresh_token,
            &tokens.username,
        ) {
            Ok(()) => ("ok".to_string(), None),
            Err(e) => {
                let warning = format!("keyring save failed: {e} (session is in-memory only)");
                (format!("unavailable: {e}"), Some(warning))
            }
        };
        session.access_token = Some(tokens.access_token);
        session.access_expires_at =
            Some(self.choke.wall() + Duration::from_secs(tokens.expires_in));
        session.refresh_token = Some(tokens.refresh_token);
        session.username = Some(tokens.username);
        session.keyring = keyring;
        session.advance_access_token();
        session.advance_refresh_token();
        session.refresh_flight = None;
        warning
    }

    /// A 401 landed on a send that carried `token` (C87): the session
    /// forgets it and the log says so. Reached from the choke point through
    /// `Bearer::rejected`, while the rejecting send still holds its permit,
    /// so every send admitted after it reads the cleared session. The job
    /// that met the 401 fails as it did; the next send for that account is
    /// the refresh (`TESTING-NOTES.md`, the C87 scenarios). Returns the
    /// account whose token was cleared, or `None` when no session held it
    /// any more.
    fn reject_access_token(&self, token: &str) -> Option<String> {
        let username = self
            .shared
            .lock()
            .unwrap()
            .auth
            .reject_access_token(token)?;
        self.log(&format!(
            "access token for {username} rejected (401); forgotten — the next send refreshes first"
        ));
        Some(username)
    }

    /// Current access token, refreshing through the provider if it is
    /// expired (or about to be). Jobs call this; clients never see tokens.
    /// `force_refresh` skips the cached token so the provider round-trip is
    /// guaranteed — that's what makes `auth check` an actual proof.
    /// `account` is the job's account (fixed at submit): the token handed
    /// back is that account's session's, looked up at the moment the token
    /// is taken — a logout of A after A's job was submitted fails A's job
    /// here, before any send ("no session for A"). `None` is the sole
    /// session (auth verbs), refused when several are live.
    async fn valid_access_token(
        &self,
        account: Option<&str>,
        force_refresh: bool,
    ) -> Result<(String, String), Refusal> {
        enum Decision {
            Owner {
                id: u64,
                generations: AuthGenerations,
                refresh_token: String,
                result: watch::Sender<Option<AccessTokenResult>>,
            },
            Waiter(watch::Receiver<Option<AccessTokenResult>>),
        }

        let (username, decision) = {
            let mut s = self.shared.lock().unwrap();
            let wall = self.choke.wall();
            let session = s.auth.get_mut(account)?;
            let username = session.username.clone().unwrap_or_default();
            if !force_refresh
                && let (Some(token), Some(expires)) =
                    (&session.access_token, session.access_expires_at)
                && expires
                    .duration_since(wall)
                    .is_ok_and(|left| left > Duration::from_secs(5))
            {
                return Ok((token.clone(), username));
            }
            let refresh_token = match &session.refresh_token {
                Some(rt) => rt.clone(),
                None => {
                    return Err(Refusal::new(
                        ErrorKind::NotLoggedIn,
                        format!("no refresh token for {username} — run `acq auth`"),
                    ));
                }
            };
            // A grant the provider already rejected is terminal (C24):
            // it is not sent again until login or logout replaces it.
            if let Some(cause) = self.rails().refresh_failed(&username) {
                return Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!("token refresh disabled for {username}: {cause}; run `acq auth`"),
                ));
            }
            let generations = session.generations;
            let decision = if let Some(flight) = &session.refresh_flight
                && flight.generations == generations
            {
                Decision::Waiter(flight.result.subscribe())
            } else {
                let id = session.next_refresh_flight;
                session.next_refresh_flight = session.next_refresh_flight.wrapping_add(1);
                let (result, _) = watch::channel(None);
                session.refresh_flight = Some(RefreshFlight {
                    id,
                    generations,
                    result: result.clone(),
                });
                Decision::Owner {
                    id,
                    generations,
                    refresh_token,
                    result,
                }
            };
            (username, decision)
        };
        match decision {
            Decision::Waiter(result) => wait_for_refresh(result).await,
            Decision::Owner {
                id,
                generations,
                refresh_token,
                result,
            } => {
                let owner = RefreshOwnerGuard {
                    daemon: self,
                    account: username.clone(),
                    id,
                    generations,
                    result: Some(result),
                };
                // May wait on the token endpoint's limiter; the shared lock is
                // not held here, so every concurrent caller can join this owner.
                let refresh = auth::refresh(&self.choke, &self.provider, &refresh_token).await;
                self.announce_trip();
                // L0 rail 2: mark a rejected grant before the flight is
                // closed, so no caller can pass the fast path and re-send the
                // dead token in between; and only while this flight is still
                // the current one, so a stale flight cannot disable a session
                // that re-authenticated meanwhile. The persisted cause is the
                // status plus a fixed reason, never the response body.
                if let Some(error) = refresh.as_ref().err().filter(|e| e.is_rejected_grant()) {
                    let current = {
                        let s = self.shared.lock().unwrap();
                        s.auth.find(&username).is_some_and(|session| {
                            session.generations == generations
                                && session.refresh_flight.as_ref().is_some_and(|f| f.id == id)
                        })
                    };
                    let cause = format!(
                        "refresh token rejected with HTTP {}",
                        error.status.unwrap_or_default()
                    );
                    if current && self.rails().mark_refresh_failed(&username, &cause) {
                        self.note_error(&format!(
                            "AUTH: {username}: {cause}; further refreshes disabled until `acq auth` ({error})"
                        ));
                    }
                }
                // A rejected grant is a dead session (`not_logged_in`: the
                // remedy is login); anything else — transport, a 5xx,
                // exhausted 429s, a rails halt — is the provider not
                // answering (`upstream`), with the session still standing.
                let refresh = refresh.map_err(|e| {
                    let kind = if e.is_rejected_grant() {
                        ErrorKind::NotLoggedIn
                    } else {
                        ErrorKind::Upstream
                    };
                    Refusal::new(kind, format!("token refresh failed: {e}"))
                });
                owner.finish(refresh)
            }
        }
    }

    fn finish_refresh(
        &self,
        account: &str,
        id: u64,
        generations: AuthGenerations,
        refresh: Result<auth::TokenResponse, Refusal>,
        sender: &watch::Sender<Option<AccessTokenResult>>,
    ) -> AccessTokenResult {
        let mut warning = None;
        let mut renamed = None;
        let mut index_update = None;
        let outcome = {
            let mut s = self.shared.lock().unwrap();
            let Some(session) = s.auth.find_mut(account) else {
                return finish_stale(sender);
            };
            let owns_current_flight = session
                .refresh_flight
                .as_ref()
                .is_some_and(|flight| flight.id == id && flight.generations == generations);
            if !owns_current_flight || session.generations != generations {
                Err(session_changed())
            } else {
                session.refresh_flight = None;
                match refresh {
                    Ok(tokens) => {
                        // GGG returns the same account on a refresh; if the
                        // name ever differs (a rename), the session follows
                        // it — lookups go by the session's username.
                        if tokens.username != account {
                            renamed = Some(tokens.username.clone());
                        }
                        let result = (tokens.access_token.clone(), tokens.username.clone());
                        let username = tokens.username.clone();
                        warning = self.install_tokens_locked(session, tokens);
                        index_update = Some((username, session.uuid.clone(), warning.is_none()));
                        Ok(result)
                    }
                    Err(error) => Err(error),
                }
            }
        };
        // With the session's uuid, a refresh that renamed the account
        // updates the existing index mapping in place (`record_login`
        // follows the uuid) instead of minting a uuid-less twin entry; a
        // pre-uuid session falls back to plain bookkeeping.
        if let Some((username, uuid, persisted)) = index_update {
            self.with_index(|index| match &uuid {
                Some(uuid) => index.record_login(&username, uuid, persisted, self.unix_now()),
                None => index.upsert(&username, persisted, self.unix_now()),
            });
        }
        if let Some(new_name) = &renamed {
            self.shared.lock().unwrap().auth.rename(account, new_name);
        }
        sender.send_replace(Some(outcome.clone()));
        if outcome.is_ok() {
            self.log(&format!("access token refreshed for {account}"));
        }
        if let Some(new_name) = renamed {
            self.note_error(&format!(
                "AUTH: refresh for {account} returned username {new_name}; session follows the new name"
            ));
        }
        if let Some(warning) = warning {
            self.note_error(&warning);
        }
        outcome
    }

    /// Drop a session (the named account's, or the sole one), its keyring
    /// entry, and its dead-grant mark. Other sessions are untouched.
    fn logout(&self, account: Option<&str>) -> Result<(), Refusal> {
        let (username, clear) = {
            let mut s = self.shared.lock().unwrap();
            let username = s.auth.get(account)?.username.clone().unwrap_or_default();
            s.auth
                .by_account
                .retain(|_, x| x.username.as_deref() != Some(username.as_str()));
            if s.auth.last_login.as_deref() == Some(username.as_str()) {
                s.auth.last_login = None;
            }
            let clear = self
                .credential_store
                .clear(self.provider.keyring_service, &username);
            (username, clear)
        };
        self.rails().clear_refresh_failed(&username);
        self.with_index(|index| index.set_persisted(&username, false));
        self.log(&format!("logged out {username}"));
        clear.map_err(|e| Refusal::new(ErrorKind::Internal, e))
    }

    /// Drop a *non-live* account's keyring entry and mark it not persisted.
    /// Nothing about the live session changes.
    fn forget_account(&self, selector: &str) -> Result<(), Refusal> {
        let Some(dir) = &self.store_dir else {
            return Err(Refusal::new(
                ErrorKind::Internal,
                "no account index in this daemon",
            ));
        };
        let index = Index::load(dir)
            .map_err(|e| Refusal::new(ErrorKind::Internal, format!("accounts index: {e:#}")))?;
        let entry = index
            .resolve(Some(selector))
            .map_err(|e| {
                let kind = match e {
                    acquisition_store::Resolve::Ambiguous(_) => ErrorKind::AmbiguousAccount,
                    _ => ErrorKind::NotLoggedIn,
                };
                Refusal::new(kind, e.to_string())
            })?
            .clone();
        // Nothing to clear for a session that was never in the keyring.
        let cleared = if entry.persisted {
            self.credential_store
                .clear(self.provider.keyring_service, &entry.username)
        } else {
            Ok(())
        };
        self.with_index(|index| index.set_persisted(&entry.username, false));
        self.log(&format!(
            "forgot account {} (keyring entry cleared)",
            entry.username
        ));
        cleared.map_err(|e| Refusal::new(ErrorKind::Internal, e))
    }

    fn session_statuses(&self, s: &Shared) -> Vec<SessionStatus> {
        let wall = self.choke.wall();
        let mut v: Vec<SessionStatus> = s
            .auth
            .by_account
            .values()
            .map(|session| SessionStatus {
                username: session.username.clone().unwrap_or_default(),
                access_expires_in_seconds: session
                    .access_expires_at
                    .map(|t| t.duration_since(wall).unwrap_or_default().as_secs()),
                keyring: session.keyring.clone(),
            })
            .collect();
        v.sort_by(|a, b| a.username.cmp(&b.username));
        v
    }

    /// The account reported as "the" username: the most recent login while
    /// it is live, else the sole session. Informational only.
    fn headline_session<'a>(&self, s: &'a Shared) -> Option<&'a AuthSession> {
        s.auth
            .last_login
            .as_ref()
            .and_then(|u| s.auth.find(u))
            .or_else(|| s.auth.get(None).ok())
    }

    fn keyring_summary(&self, s: &Shared) -> String {
        self.headline_session(s)
            .map(|session| session.keyring.clone())
            .unwrap_or_else(|| s.auth.keyring.clone())
    }

    fn auth_status(&self) -> Response {
        let s = self.shared.lock().unwrap();
        let head = self.headline_session(&s);
        Response::Auth {
            logged_in: !s.auth.by_account.is_empty(),
            pending: s.auth.pending.is_some(),
            login_ok: s
                .auth
                .flow_result
                .as_ref()
                .and_then(|r| r.as_ref().ok().cloned()),
            login_error: s
                .auth
                .flow_result
                .as_ref()
                .and_then(|r| r.as_ref().err().cloned()),
            username: head.and_then(|h| h.username.clone()),
            access_expires_in_seconds: head.and_then(|h| h.access_expires_at).map(|t| {
                t.duration_since(self.choke.wall())
                    .unwrap_or_default()
                    .as_secs()
            }),
            keyring: self.keyring_summary(&s),
            provider: self.provider.name.to_string(),
            accounts: self.session_statuses(&s),
        }
    }

    // ---- connection handling -------------------------------------------

    /// One connection, frame by frame (C85). Every frame is read under
    /// the bound, then tried on the bootstrap plane first — `hello` and
    /// `daemon_stop` are answered on any connection in any state, so a
    /// client of another revision can identify this daemon and a human
    /// can stop it — and only then parsed as a versioned request. A frame
    /// that is oversize, not JSON, or not a request of this version is
    /// answered `bad_request` and the connection stays; so is a
    /// versioned request on a connection that has subscribed. Frames are
    /// answered in order, one at a time; a subscribed connection also
    /// carries `event` frames, and `resync_required` when the event
    /// channel overran it.
    async fn handle_conn(self: Arc<Self>, stream: Stream) {
        {
            let now = self.choke.now();
            let mut s = self.shared.lock().unwrap();
            s.connections += 1;
            s.last_activity = now;
        }

        let (read, mut write) = tokio::io::split(stream);
        let mut reader = BufReader::new(read);
        let mut events: Option<broadcast::Receiver<JobInfo>> = None;
        // Every connection begins with the hello exchange (C85): a
        // versioned request before it is refused, so a peer that never
        // identified itself performs nothing (review round 1).
        let mut greeted = false;

        loop {
            tokio::select! {
                frame = read_frame(&mut reader, MAX_FRAME_BYTES) => {
                    let frame = match frame {
                        Ok(Frame::Closed) | Err(_) => break,
                        Ok(frame) => frame,
                    };
                    let bytes = match frame {
                        Frame::Oversize => {
                            let response = Refusal::new(
                                ErrorKind::BadRequest,
                                format!("frame exceeds {MAX_FRAME_BYTES} bytes; discarded through its newline"),
                            );
                            if write_line(&mut write, &Response::from(response)).await.is_err() {
                                break;
                            }
                            continue;
                        }
                        Frame::Line(bytes) => bytes,
                        Frame::Closed => unreachable!("handled above"),
                    };
                    if bytes.iter().all(u8::is_ascii_whitespace) {
                        continue;
                    }
                    match Bootstrap::read(&bytes) {
                        Some(Bootstrap::Hello {
                            version,
                            contract,
                            world,
                        }) => {
                            // The daemon names itself in every dimension
                            // (C84, C83) and decides nothing: the client
                            // judges staleness and replaces, refuses or
                            // reports (C10). What the client said about
                            // itself is logged when it differs, so a
                            // mismatch is visible from this side too.
                            if contract != acquisition_protocol::CONTRACT_REVISION {
                                self.log(&format!(
                                    "contract mismatch: client {contract} (version {version}), daemon {} (version {})",
                                    acquisition_protocol::CONTRACT_REVISION,
                                    acquisition_protocol::VERSION
                                ));
                            }
                            let own_world = self
                                .world
                                .clone()
                                .unwrap_or_else(|| acquisition_protocol::protocol::UNKNOWN.to_string());
                            if world != own_world {
                                self.log(&format!(
                                    "world mismatch: client on {world}, daemon on {own_world} — the client refuses this daemon"
                                ));
                            }
                            let hello = BootstrapReply::Hello {
                                version: acquisition_protocol::VERSION.to_string(),
                                contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                                artifact: self.artifact.clone(),
                                pid: std::process::id(),
                                provider: self.provider.name.to_string(),
                                world: own_world,
                            };
                            if write_line(&mut write, &hello).await.is_err() {
                                break;
                            }
                            greeted = true;
                            continue;
                        }
                        Some(Bootstrap::DaemonStop) => {
                            let _ = write_line(&mut write, &BootstrapReply::Stopping).await;
                            self.exit_process("stop requested; exiting");
                        }
                        None => {}
                    }
                    let response = if !greeted {
                        Refusal::new(
                            ErrorKind::BadRequest,
                            "the connection has not opened with hello; every connection begins with the hello exchange",
                        )
                        .into()
                    } else if events.is_some() {
                        Refusal::new(
                            ErrorKind::BadRequest,
                            "this connection is a subscription: after `subscribed` it carries events only; requests go on a connection of their own",
                        )
                        .into()
                    } else {
                        match serde_json::from_slice::<Request>(&bytes) {
                            Ok(req) => self.handle_request(req, &mut events).await,
                            Err(e) => Refusal::new(ErrorKind::BadRequest, format!("bad request: {e}")).into(),
                        }
                    };
                    if write_line(&mut write, &response).await.is_err() {
                        break;
                    }
                }
                signal = recv_event(&mut events) => {
                    let response = match signal {
                        Signal::Event(job) => Response::Event { job },
                        Signal::Lagged(missed) => Response::ResyncRequired { missed },
                    };
                    if write_line(&mut write, &response).await.is_err() {
                        break;
                    }
                }
            }
        }

        let now = self.choke.now();
        let mut s = self.shared.lock().unwrap();
        s.connections -= 1;
        s.last_activity = now;
    }

    /// The `quote` request (C40, decided 2026-08-31): a read-only,
    /// non-reserving projection of what the quoted work would meet at the
    /// choke point right now. Reads the limiter and the queue, sends
    /// nothing, reserves nothing, remembers nothing — two quotes about the
    /// same work may disagree because the world moved. Account selection
    /// follows `Submit`'s rules exactly (resolved here, refused when
    /// ambiguous), so the quote keys the same limiter state a submit would.
    fn quote(&self, jobs: &[QuoteJob], account: Option<&str>) -> Result<Quote, Refusal> {
        // The selector is judged before anything is projected — an unknown
        // or ambiguous account refuses the quote whole even when the job
        // list is empty, exactly as a submit would refuse it. Omitted with
        // several sessions live is ambiguous too: a quote is one projection
        // under one headline account, and the daemon never guesses whose.
        // The judged selector is what each job resolves against, so a
        // job the quote projects keys exactly what the same submit would key (an
        // omitted selector still runs accountless kinds accountless).
        let account = match account {
            Some(_) => self.canonical_account(account)?,
            None => {
                let s = self.shared.lock().unwrap();
                if !s.auth.by_account.is_empty() {
                    s.auth.get(None)?;
                }
                None
            }
        };
        // Group the work by scheduling scope: the policy state key once the
        // route is learned (same-name policies share counters, N6; `Account`
        // rules count per account, rung 11), else the endpoint key itself.
        let mut scopes: BTreeMap<String, (BTreeSet<String>, u64)> = BTreeMap::new();
        let mut sends_nothing: BTreeMap<String, u64> = BTreeMap::new();
        let mut resolved_account: Option<String> = account.clone();
        let mut needs_token = false;
        for job in jobs {
            let acct = self.resolve_account(&job.kind, account.as_deref())?;
            if let Some(a) = &acct {
                resolved_account.get_or_insert_with(|| a.clone());
            }
            match self.keyed_route_for(&job.kind, &job.params, acct.as_deref()) {
                None => *sends_nothing.entry(job.kind.clone()).or_insert(0) += 1,
                Some((endpoint, _)) => {
                    needs_token |= self.needs_auth(split_endpoint_key(&endpoint).0);
                    let slot = scopes.entry(self.choke.serial_key(&endpoint)).or_default();
                    slot.0.insert(endpoint);
                    slot.1 += 1;
                }
            }
        }
        // Jobs already here compete for the same scopes; the estimate puts
        // them ahead of the quoted work. Probes stay out (their own key, and
        // named under `not_covered` instead), and so does a parent holding a
        // deferred result: its own request already happened — it is only
        // waiting for its children, who count for themselves.
        let mut queued: HashMap<String, u64> = HashMap::new();
        {
            let s = self.shared.lock().unwrap();
            for e in s.jobs.values() {
                if e.info.state.is_terminal() || e.info.kind == "probe" || e.deferred.is_some() {
                    continue;
                }
                if let Some((endpoint, _)) =
                    self.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref())
                {
                    *queued.entry(self.choke.serial_key(&endpoint)).or_insert(0) += 1;
                }
            }
        }
        let mut unprobed: BTreeSet<String> = BTreeSet::new();
        let mut out = Vec::new();
        for (key, (endpoints, requests)) in scopes {
            // Endpoints share a scope only through a learned common policy,
            // so any one of them names the scope's state. The projection is
            // taken under one limiter lock, so its rules, observation age,
            // and ETA describe the same instant.
            let sample = endpoints.iter().next().cloned().unwrap_or_default();
            let queued_ahead = queued.get(&key).copied().unwrap_or(0);
            let ahead = (queued_ahead + requests - 1) as u32;
            let projection = self.choke.project(&sample, ahead);
            let (policy, eta_seconds, notes) = match projection.state {
                EndpointState::Policy(name) => {
                    let eta = projection.eta.unwrap_or_default().as_secs();
                    (Some(name), Some(eta), Vec::new())
                }
                EndpointState::Policyless => (
                    None,
                    Some(0),
                    vec![
                        "declared policyless: paced by the send gate alone; headers, if they \
                         ever appear, are learned strictly"
                            .into(),
                    ],
                ),
                EndpointState::Unknown => {
                    let route = split_endpoint_key(&sample).0.to_string();
                    let note = if Self::route_probes(&sample) {
                        unprobed.insert(route.clone());
                        format!(
                            "policy not yet learned: a HEAD probe (its own send) precedes the \
                             first request on {route}"
                        )
                    } else {
                        format!(
                            "policy not yet learned: {route} is a declared no-probe route — \
                             its first GET teaches the limiter"
                        )
                    };
                    (None, None, vec![note])
                }
                EndpointState::Degraded { until, reason } => {
                    // The cooldown expires into `Unknown`, so the eventual
                    // replacement probe is a future send too.
                    let route = split_endpoint_key(&sample).0.to_string();
                    if Self::route_probes(&sample) {
                        unprobed.insert(route);
                    }
                    (
                        None,
                        None,
                        vec![format!(
                            "endpoint closed by a failed probe for another {}s: {reason}",
                            until.saturating_duration_since(self.choke.now()).as_secs()
                        )],
                    )
                }
            };
            out.push(QuoteScope {
                key,
                endpoints: endpoints.into_iter().collect(),
                requests,
                queued_ahead,
                policy,
                rules: projection.rules,
                observed_seconds_ago: projection.observed_seconds_ago,
                eta_seconds,
                notes,
            });
        }
        let total: u64 = out.iter().map(|s| s.requests).sum();
        let mut not_covered = Vec::new();
        if !unprobed.is_empty() {
            not_covered.push(format!(
                "a HEAD probe before the first request on {} — its own send, outside \
                 every estimate",
                unprobed.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
        if needs_token {
            not_covered.push(
                "an OAuth token refresh if the access token expires first — paced by \
                 the token policy, not quoted"
                    .into(),
            );
        }
        if total > 0 {
            not_covered.push(format!(
                "429 re-sends (up to {MAX_429_RETRIES} per request) — possible, never predicted"
            ));
        }
        for (kind, n) in sends_nothing {
            not_covered.push(format!(
                "{n} `{kind}` job(s) send nothing and are outside every scope"
            ));
        }
        Ok(Quote {
            observed_at: self
                .choke
                .wall()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            provider: self.provider.name.to_string(),
            account: resolved_account,
            halted: self.rails().halted(),
            // Echoed verbatim: the quote's verifiable basis — a carrier
            // checks these tuples name exactly the work it cares about.
            work: jobs.to_vec(),
            scopes: out,
            not_covered,
        })
    }

    async fn handle_request(
        self: &Arc<Self>,
        req: Request,
        events: &mut Option<broadcast::Receiver<JobInfo>>,
    ) -> Response {
        match req {
            Request::Submit {
                kind,
                params,
                priority,
                submitted_by,
                account,
            } => match self
                .resolve_account(&kind, account.as_deref())
                .and_then(|account| self.submit(kind, params, priority, submitted_by, account))
            {
                Ok(id) => Response::Submitted { id },
                Err(refusal) => {
                    self.note_error(&format!("submit refused: {refusal}"));
                    refusal.into()
                }
            },
            Request::Status { id } => match self.shared.lock().unwrap().snapshot(self, id) {
                Some(job) => Response::Status { job },
                None => Refusal::no_job(id).into(),
            },
            Request::Result { id } => {
                let held = {
                    let s = self.shared.lock().unwrap();
                    s.jobs.get(&id).map(|e| (e.info.state, e.outcome.clone()))
                };
                match held {
                    Some((_, Some(outcome))) => Response::Result { id, outcome },
                    Some((state, None)) => {
                        Refusal::new(ErrorKind::WrongState, format!("job {id} is still {state}"))
                            .into()
                    }
                    // Not this lifetime's: a previous daemon's result, if
                    // retention still has it.
                    None => match self.stored_outcome(id) {
                        Ok(Some(outcome)) => Response::Result { id, outcome },
                        Ok(None) => Refusal::no_job(id).into(),
                        Err(refusal) => {
                            self.note_error(&format!("result {id}: {refusal}"));
                            refusal.into()
                        }
                    },
                }
            }
            Request::Cancel { id } => match self.cancel(id) {
                Ok(()) => Response::Ack,
                Err(refusal) => refusal.into(),
            },
            Request::SetPriority { id, priority } => match self.set_priority(id, priority) {
                Ok(()) => {
                    self.work.notify_one();
                    Response::Ack
                }
                Err(refusal) => refusal.into(),
            },
            Request::List => Response::Jobs {
                jobs: self.shared.lock().unwrap().list(self),
            },
            Request::Subscribe => {
                *events = Some(self.events.subscribe());
                Response::Subscribed
            }
            Request::AuthStart => match self.auth_start().await {
                Ok(authorize_url) => Response::AuthUrl { authorize_url },
                Err(refusal) => refusal.into(),
            },
            Request::AuthStatus => self.auth_status(),
            Request::AuthCheck { account } => match self.canonical_account(account.as_deref()) {
                Err(refusal) => refusal.into(),
                Ok(account) => match self.valid_access_token(account.as_deref(), true).await {
                    Ok(_) => self.auth_status(),
                    Err(refusal) => {
                        self.note_error(&format!("auth check failed: {refusal}"));
                        refusal.into()
                    }
                },
            },
            Request::AuthLogout { account } => {
                // A live session (named, or the sole one) logs out; a known
                // but not live account only loses its keyring entry.
                let live = {
                    let s = self.shared.lock().unwrap();
                    match &account {
                        None => s
                            .auth
                            .get(None)
                            .map(|x| x.username.clone().unwrap_or_default()),
                        Some(req) => s.auth.matching(req).and_then(|found| {
                            found
                                .map(|x| x.username.clone().unwrap_or_default())
                                .ok_or_else(|| no_session(req))
                        }),
                    }
                };
                match (live, &account) {
                    (Ok(username), _) => {
                        if let Err(e) = self.logout(Some(&username)) {
                            self.log(&format!("keyring clear failed: {e}"));
                        }
                        Response::Ack
                    }
                    (Err(_), Some(req)) => match self.forget_account(req) {
                        Ok(()) => Response::Ack,
                        Err(refusal) => refusal.into(),
                    },
                    (Err(refusal), None) => refusal.into(),
                }
            }
            Request::Quote { jobs, account } => match self.quote(&jobs, account.as_deref()) {
                Ok(quote) => Response::Quote { quote },
                Err(refusal) => refusal.into(),
            },
            Request::DaemonStatus => {
                let s = self.shared.lock().unwrap();
                let (in_flight, max_in_flight) = self.choke.actual_send_occupancy();
                let (waiting, running) =
                    s.jobs
                        .values()
                        .fold((0, 0), |(w, r), e| match e.info.state {
                            JobState::Waiting => (w + 1, r),
                            JobState::Running => (w, r + 1),
                            _ => (w, r),
                        });
                Response::DaemonStatus {
                    pid: std::process::id(),
                    version: acquisition_protocol::VERSION.to_string(),
                    contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                    provider: self.provider.name.to_string(),
                    uptime_seconds: self.uptime_seconds(),
                    idle_shutdown_seconds: idle_shutdown_from_env().as_secs(),
                    connections: s.connections,
                    jobs_waiting: waiting,
                    jobs_running: running,
                    policies_known: self.choke.policy_statuses().len(),
                    in_flight,
                    max_in_flight,
                    rails: self.rails().status(),
                    keyring: self.keyring_summary(&s),
                    log: self.log_path.clone(),
                }
            }
            Request::ResetTripwire => {
                self.reset_rails();
                Response::Ack
            }
            Request::Dashboard => {
                let now = self.choke.now();
                let s = self.shared.lock().unwrap();
                let (in_flight, max_in_flight) = self.choke.actual_send_occupancy();
                Response::Dashboard {
                    pid: std::process::id(),
                    version: acquisition_protocol::VERSION.to_string(),
                    contract: acquisition_protocol::CONTRACT_REVISION.to_string(),
                    provider: self.provider.name.to_string(),
                    uptime_seconds: self.uptime_seconds(),
                    connections: s.connections,
                    logged_in: !s.auth.by_account.is_empty(),
                    username: self.headline_session(&s).and_then(|h| h.username.clone()),
                    access_expires_in_seconds: self
                        .headline_session(&s)
                        .and_then(|h| h.access_expires_at)
                        .map(|t| {
                            t.duration_since(self.choke.wall())
                                .unwrap_or_default()
                                .as_secs()
                        }),
                    keyring: self.keyring_summary(&s),
                    in_flight,
                    max_in_flight,
                    policies: self.choke.policy_statuses(),
                    policyless_endpoints: self.choke.policyless_endpoints(),
                    degraded_endpoints: self.choke.degraded_endpoints(),
                    jobs: s.list(self),
                    sends: self.choke.recent_sends(),
                    rails: self.rails().status(),
                    errors: s
                        .errors
                        .iter()
                        .rev()
                        .map(|(at, message)| ErrorRecord {
                            seconds_ago: now.saturating_duration_since(*at).as_secs_f64(),
                            message: message.clone(),
                        })
                        .collect(),
                }
            }
        }
    }

    /// Clear the rails halt and wake the jobs that were waiting on it.
    fn reset_rails(&self) {
        self.rails().reset_tripwire();
        self.log("live-test rails reset by request");
        self.work.notify_one();
    }

    /// The one way out. The daemon leaves by `process::exit`, so nothing
    /// is dropped: the store's and the queue's write-ahead logs are
    /// checkpointed here first (`Store::checkpoint`: an exit without it
    /// left every fact since the last automatic checkpoint in a `-wal`
    /// beside the file — the price-notes run, 2026-09-04; plan step 7,
    /// item 4), then the socket is removed, then the process exits. A
    /// checkpoint a reader keeps from finishing is logged, never waited
    /// on. Pinned by `tests/daemon_stop_checkpoint.rs` (acquisition-cli).
    fn exit_process(&self, why: &str) -> ! {
        self.log(why);
        {
            let guard = self.store.lock().unwrap();
            if let Some((account, store)) = guard.as_ref() {
                match store.checkpoint() {
                    Ok(c) => self.log(&format!(
                        "store: {account}'s facts checkpointed on exit ({} of {} WAL pages{})",
                        c.checkpointed,
                        c.wal_pages,
                        if c.busy {
                            "; a reader kept it from finishing"
                        } else {
                            ""
                        }
                    )),
                    Err(e) => self.note_error(&format!(
                        "store: checkpoint of {account}'s facts on exit failed: {e:#}"
                    )),
                }
            }
        }
        match self.jobs_db.lock().unwrap().checkpoint() {
            Ok(c) => self.log(&format!(
                "jobs: queue checkpointed on exit ({} of {} WAL pages{})",
                c.checkpointed,
                c.wal_pages,
                if c.busy {
                    "; a reader kept it from finishing"
                } else {
                    ""
                }
            )),
            Err(e) => self.note_error(&format!("jobs: queue checkpoint on exit failed: {e:#}")),
        }
        if let Some(socket) = &self.socket_path {
            socket.remove_if_still_ours();
        }
        std::process::exit(0);
    }

    /// The idle verdict (C3; the halted arm is C25): no connection, no
    /// live job, the last activity at least `idle_shutdown` ago, and no
    /// limiter history worth keeping — a pending wait or counted hits
    /// inside a policy window, which a daemon respawned a minute later
    /// would otherwise have to assume the worst about. Every clause reads
    /// the daemon's clock, so a test advances a manual clock past the
    /// knob instead of waiting. Pinned: `tests/idle.rs`'s `idle_verdict_*`.
    fn is_idle(&self, idle_shutdown: Duration) -> bool {
        let now = self.choke.now();
        let idle = {
            let s = self.shared.lock().unwrap();
            let parked = self.rails().halted().is_some() || self.queue_failed().is_some();
            let live_jobs = Self::has_live_jobs(&s, parked);
            s.connections == 0
                && !live_jobs
                && now.saturating_duration_since(s.last_activity) >= idle_shutdown
        };
        idle && !self.choke.is_live()
    }

    /// Polls the verdict on tokio time, deliberately not on the daemon's
    /// `Clock`: the test `ManualClock` advances on every `sleep`, so a
    /// poller on it would drive the clock forward `IDLE_POLL` per yield.
    /// The verdict is the tested part; this loop is never spawned under a
    /// manual clock (the harness constructors spawn nothing).
    async fn idle_watchdog(self: Arc<Self>) {
        let idle_shutdown = idle_shutdown_from_env();
        loop {
            tokio::time::sleep(IDLE_POLL).await;
            if self.is_idle(idle_shutdown) {
                self.exit_process("idle timeout; exiting");
            }
        }
    }
}

/// The outcome of a fan-out whose child submission was refused: the
/// parent was cancelled, or the queue failed. Already-submitted children
/// run either way (their sends are theirs); the parent never claims
/// success over a partial set.
/// Realm admission: a kind in a realm family (`acquisition_protocol::realm`) must name a
/// realm that family takes — or none, meaning pc. Runs at submit for
/// every kind and per tuple inside `validate_apply`, so a job that would
/// render a stash URL under `poe2` never gets an id (C59: no code path
/// renders an unobserved URL shape). Kinds outside the families ignore
/// the param.
fn admit_realm(kind: &str, params: &Value) -> Result<(), Refusal> {
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

/// The `apply` admission check (C43 — vocabulary, not meaning; C42 — the
/// logical budget at admission; decided 2026-09-01): the vocabulary a
/// plan-blind daemon can enforce. `params.jobs` must be a
/// non-empty array of `(kind, params)` tuples in which every kind is a
/// single-request one that submits no children — `stashes`, `stash` with
/// `deep` absent/false, `characters`, or `character` with a name
/// (characters joined 2026-09-02) — so no child can expand the reviewed set;
/// and when the caller declares `max_requests`, the tuple count must not
/// exceed it. Runs at submit, before a job id exists, so a refusal
/// admits nothing (mid-fan-out terminalization is never the budget's
/// mechanism); a misread budget refuses too — a limit half-honored by
/// ignoring it would spend exactly what the caller tried to cap.
fn validate_apply(params: &Value) -> Result<(), Refusal> {
    let refused = |message: String| Refusal::new(ErrorKind::Refused, message);
    let Some(jobs) = params.get("jobs").and_then(Value::as_array) else {
        return Err(refused(
            "apply needs a `jobs` array of {kind, params} tuples (a plan's actions)".into(),
        ));
    };
    if jobs.is_empty() {
        return Err(refused(
            "apply with an empty `jobs` array: a plan with no actions has nothing to execute"
                .into(),
        ));
    }
    for (i, job) in jobs.iter().enumerate() {
        let kind = job.get("kind").and_then(Value::as_str).unwrap_or("");
        let params = job.get("params").cloned().unwrap_or(Value::Null);
        admit_realm(kind, &params).map_err(|e| refused(format!("action {i}: {e}")))?;
        match kind {
            "stashes" => {}
            "stash" => {
                if params.get("deep").and_then(Value::as_bool).unwrap_or(false) {
                    return Err(refused(format!(
                        "action {i}: a plan's stash fetch never fans out (`deep` must be false)"
                    )));
                }
                if params.get("id").and_then(Value::as_str).is_none() {
                    return Err(refused(format!("action {i}: stash needs an id")));
                }
            }
            "characters" => {}
            "character" => {
                if params.get("name").and_then(Value::as_str).is_none() {
                    return Err(refused(format!("action {i}: character needs a name")));
                }
            }
            other => {
                return Err(refused(format!(
                    "action {i}: kind {other:?} is not in the plan vocabulary \
                     (stashes, stash, characters, character)"
                )));
            }
        }
    }
    if let Some(max) = params.get("max_requests") {
        let Some(max) = max.as_u64() else {
            return Err(refused(format!(
                "max_requests must be a non-negative integer, not {max}"
            )));
        };
        if jobs.len() as u64 > max {
            return Err(refused(format!(
                "plan exceeds the budget: {} logical request(s) against max_requests {max} — nothing was submitted",
                jobs.len()
            )));
        }
    }
    Ok(())
}

fn fan_out_stopped(cancelled: bool, submitted: usize, why: &str) -> Outcome {
    if cancelled {
        Outcome::Cancelled
    } else {
        Outcome::failure(format!("fan-out stopped after {submitted} children: {why}"))
    }
}

impl Daemon {
    /// Whether any job keeps the daemon up. A halted daemon's waiting jobs
    /// do not: they are on disk, and its successor holds them until the
    /// reset (C25). A parent holding its result runs no task of its own.
    fn has_live_jobs(s: &Shared, halted: bool) -> bool {
        s.jobs.values().any(|e| match e.info.state {
            JobState::Running => e.deferred.is_none() || !halted,
            JobState::Waiting => !halted,
            _ => false,
        })
    }
}

fn classify_api_body(
    status: reqwest::StatusCode,
    retry_after: &RetryAfter,
    path: &str,
    rate: Value,
    body: Result<String, String>,
) -> Result<(Value, Value), ApiError> {
    // Status owns non-2xx outcomes even if reading the error body also failed
    // (D8). The body error remains in the evidence rather than becoming fake
    // empty content.
    if !status.is_success() {
        let body = match body {
            Ok(body) => body.chars().take(300).collect(),
            Err(error) => format!("<body read transport failure: {error}>"),
        };
        let evidence = format!("GET {path} returned {status}; rate headers {rate}; body: {body}");
        return Err(match status.as_u16() {
            429 if retry_after.is_acceptable() => ApiError::RateLimited(evidence),
            429 => ApiError::Other(format!(
                "terminal rate limit: {evidence}; {retry_after}; NOT retrying"
            )),
            403 | 503 => ApiError::Other(format!(
                "{evidence} — {}; NOT retrying (invariant 3)",
                BlockShape::of(&body).describe()
            )),
            _ => ApiError::Other(evidence),
        });
    }

    let body = body.map_err(|error| {
        ApiError::Other(format!(
            "GET {path} body read transport failure after {status}: {error}"
        ))
    })?;
    serde_json::from_str::<Value>(&body)
        .map(|value| (value, rate))
        .map_err(|error| ApiError::Other(format!("bad JSON from {path}: {error}")))
}

/// Wait for the browser to hit the loopback redirect with an auth code.
/// Ignores stray requests (favicons etc.) until `/callback` arrives.
async fn wait_callback(listener: TcpListener, expected_state: &str) -> Result<String, String> {
    loop {
        let (mut stream, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let Some(req) = mockggg::read_request(&mut stream).await else {
            continue;
        };
        if req.method != "GET" || req.path != CALLBACK_PATH {
            mockggg::respond(&mut stream, "404 Not Found", "text/plain", "not found").await;
            continue;
        }
        match (req.query.get("code"), req.query.get("state")) {
            (Some(code), Some(state)) if state == expected_state => {
                mockggg::respond(
                    &mut stream,
                    "200 OK",
                    "text/html",
                    "<h1>Logged in</h1><p>You can close this tab and return to the terminal.</p>",
                )
                .await;
                return Ok(code.clone());
            }
            _ => {
                mockggg::respond(
                    &mut stream,
                    "400 Bad Request",
                    "text/plain",
                    "state mismatch",
                )
                .await;
                return Err("callback state mismatch".into());
            }
        }
    }
}

/// What a subscribed connection is told next: an event, or that it
/// missed some (C85).
enum Signal {
    Event(JobInfo),
    /// The broadcast channel overran this receiver; `missed` events are
    /// gone. The subscriber snapshots again.
    Lagged(u64),
}

async fn recv_event(rx: &mut Option<broadcast::Receiver<JobInfo>>) -> Signal {
    match rx {
        Some(rx) => match rx.recv().await {
            Ok(info) => Signal::Event(info),
            // A loss the client cannot observe cannot be recovered from,
            // so it is reported, never skipped (C85); the receiver has
            // already advanced past the gap. Closed can't happen while
            // the daemon holds the sender.
            Err(broadcast::error::RecvError::Lagged(missed)) => Signal::Lagged(missed),
            Err(broadcast::error::RecvError::Closed) => std::future::pending().await,
        },
        None => std::future::pending().await,
    }
}

/// A refresh whose session vanished (logged out) while it was in flight.
/// The session a refresh was for is gone or replaced: whoever waited on
/// it has no session to use — `not_logged_in`.
fn session_changed() -> Refusal {
    Refusal::new(ErrorKind::NotLoggedIn, SESSION_CHANGED_DURING_REFRESH)
}

/// A child submission's parent is no longer there to take it.
fn parent_gone(parent: JobId) -> Refusal {
    Refusal::new(ErrorKind::Refused, format!("parent job {parent} is gone"))
}

fn finish_stale(sender: &watch::Sender<Option<AccessTokenResult>>) -> AccessTokenResult {
    let outcome: AccessTokenResult = Err(session_changed());
    sender.send_replace(Some(outcome.clone()));
    outcome
}

async fn wait_for_refresh(
    mut result: watch::Receiver<Option<AccessTokenResult>>,
) -> AccessTokenResult {
    loop {
        if let Some(outcome) = result.borrow().clone() {
            return outcome;
        }
        if result.changed().await.is_err() {
            return Err(Refusal::new(ErrorKind::Internal, REFRESH_OWNER_ABANDONED));
        }
    }
}

async fn write_line<T: serde::Serialize>(
    write: &mut WriteHalf<Stream>,
    response: &T,
) -> std::io::Result<()> {
    let mut line = serde_json::to_string(response).expect("response serializes");
    line.push('\n');
    write.write_all(line.as_bytes()).await
}

/// Run the daemon until stopped or idle-timed-out. Never returns Ok while the
/// socket is healthy; returns Err early if another daemon already owns the
/// world, the real-mode lock or the socket.
pub async fn run() -> Result<()> {
    // The world first (C83): the use path creates the root and
    // canonicalises it — the frontend that spawned this daemon did the
    // same a moment ago, so this is the second look at an existing
    // directory. Without a world there is no log directory to write a
    // refusal to; stderr is all there is (a driver captures it).
    let world = World::create().map_err(|e| anyhow::anyhow!("{e}"))?;
    let provider_name = acquisition_protocol::provider::wanted();
    let log_path = world.log_path(provider_name);
    // The path crosses the wire exactly (`daemon_status`), so it must be
    // valid UTF-8; `ACQ_LOG_DIR` is the only way it would not be.
    let log_name = log_path
        .to_str()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "the log directory is not valid UTF-8 (ACQ_LOG_DIR): {}",
                log_path.display()
            )
        })?
        .to_string();
    // The locks first (review 2026-09-11): nothing in the log directory
    // is made, rotated or opened until this daemon holds the world, so a
    // contender that loses cannot rotate the incumbent's live log or
    // rewrite its marker. A refused contender appends its refusal to
    // the log as it stands, creating nothing — a lazy-spawned daemon's
    // stderr goes to null and the log is where the CLI reads a spawn
    // failure from; with no log there yet, stderr is all there is.
    let locks = match take_locks(&world) {
        Ok(locks) => locks,
        Err(e) => {
            // Appended to a log that exists; never created (review
            // 2026-09-11, round 21: a contender under another log
            // directory must make no file there either).
            if let Ok(log) = std::fs::OpenOptions::new().append(true).open(&log_path) {
                writeln!(&log, "STARTUP: {e:#}").ok();
            }
            return Err(e);
        }
    };
    // Holding the world: the log directory and its marker, then the
    // bound — the log is rotated here, before it opens, when the last
    // lifetime left it over the cap.
    if let Some(dir) = log_path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| anyhow::anyhow!("creating the log directory {}: {e}", dir.display()))?;
    }
    let _ = std::fs::write(world.log_marker_path(), format!("{}\n", world.name()));
    let rotated = match rotate_if_over(&log_path, DIAGNOSTIC_CAP_BYTES) {
        Ok(rotated) => rotated,
        Err(why) => {
            let e = anyhow::anyhow!("the log cannot be bounded: {why}");
            if let Ok(log) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
            {
                writeln!(&log, "STARTUP: {e:#}").ok();
            }
            return Err(e);
        }
    };
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| anyhow::anyhow!("opening the log {}: {e}", log_path.display()))?;
    if let Some(what) = rotated {
        writeln!(&log, "log: {what}").ok();
    }
    let result = run_with_log(world, locks, log_name, log.try_clone()?).await;
    if let Err(e) = &result {
        writeln!(&log, "STARTUP: {e:#}").ok();
    }
    result
}

/// The world lock and, in real mode, the real-mode lock (C83), taken
/// before the log directory is made, the log rotated or opened, or
/// anything sends; held for the daemon's lifetime by the caller,
/// released by the kernel however the process ends. A
/// second daemon on this root, or a second real-mode daemon for this OS
/// user whatever its root, refuses here naming the holder.
fn take_locks(world: &World) -> Result<(Lock, Option<Lock>)> {
    let world_lock = Lock::acquire(&world.lock_path()).map_err(|e| {
        anyhow::anyhow!(
            "another daemon holds this world: {e} — one daemon per world; `acq daemon status` or `acq daemon stop` against its socket"
        )
    })?;
    // Per OS user, whatever the root: two live-test roots cannot make
    // two GGG gates (C31).
    let real_mode_lock = if ggg_mode() {
        let path = acquisition_store::world::real_mode_lock_path()
            .map_err(|e| anyhow::anyhow!("resolving the real-mode lock: {e}"))?;
        Some(Lock::acquire(&path).map_err(|e| {
            anyhow::anyhow!(
                "another real-mode daemon is running for this OS user: {e} — one real-mode daemon per OS user, whatever its world; `acq daemon stop` it first"
            )
        })?)
    } else {
        None
    };
    Ok((world_lock, real_mode_lock))
}

async fn run_with_log(
    world: World,
    locks: (Lock, Option<Lock>),
    log_path: String,
    log: std::fs::File,
) -> Result<()> {
    // The locks live here, as long as the daemon does.
    let (_world_lock, real_mode_lock) = locks;
    let provider_name = acquisition_protocol::provider::wanted();
    // The rendezvous (C112): the world's endpoint, derived from the world
    // and the user. On Unix a socket in the private per-user runtime
    // directory, made here — the daemon is the one side that creates it;
    // a client only verifies it. On Windows the directory holds only the
    // real-mode lock and the endpoint is a pipe.
    app_runtime_dir().map_err(|e| anyhow::anyhow!("the runtime directory: {e}"))?;
    let path = world
        .socket_path()
        .map_err(|e| anyhow::anyhow!("this world's socket: {e}"))?;
    // The rails state lives in the world (C83), read strictly before the
    // rails are built: a state that cannot be carried into this lifetime
    // refuses the start (fail closed, review 2026-09-11).
    let rails_state = world.rails_state_path(provider_name);
    verify_state(&rails_state)?;

    let (mut listener, bound) = Listener::bind(&path).await?;

    // Who this daemon is (C84): the executable it runs from, identified
    // and hashed once, before the journal opens — its header names the
    // hash — and before anything can send. A daemon that cannot read its
    // own file does not start: no client could use it.
    let artifact = crate::artifact::of_current_exe()?;

    // Real GGG by default (C88); under `ACQ_PROVIDER=mock` the in-process
    // mock, which in real mode is never even started.
    let provider = if ggg_mode() {
        Provider::ggg()
    } else {
        Provider::mock(&mockggg::start().await?)
    };
    // Same limiter in both modes: empty until responses teach it policies.
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    // The default journal is bounded like the log; one `ACQ_JOURNAL`
    // names is the caller's and is left alone.
    let default_journal = world.journal_path(provider.name);
    let journal_rotated = if std::env::var_os("ACQ_JOURNAL").is_none() {
        rotate_if_over(&default_journal, DIAGNOSTIC_CAP_BYTES)
            .map_err(|why| anyhow::anyhow!("the journal cannot be bounded: {why}"))?
    } else {
        None
    };
    let mut rails_config = RailsConfig::from_env(&rails_state, &default_journal);
    rails_config.daemon_artifact = Some(artifact.sha256.clone());
    let rails = Arc::new(Rails::with_config_and_clock(rails_config, clock.clone()));
    let choke = ChokePoint::with_clock_and_rails(clock, rails);
    Daemon::declare_route_knowledge(&choke);
    let now = choke.now();

    // Sessions survive daemon restarts through the keyring, one entry per
    // account; the account index says which entries to look for, and every
    // persisted account comes back as a live session (C31). One unreadable
    // entry never blocks the rest.
    let dir = world.provider_dir(provider.name);
    let mut sessions = Sessions {
        keyring: match auth::keyring_disabled() {
            Some(why) => format!("unavailable: {why}"),
            None => "ok".into(),
        },
        ..Sessions::default()
    };
    match Index::load(&dir) {
        Ok(index) => {
            for entry in index.persisted() {
                match auth::keyring_load(provider.keyring_service, &entry.username) {
                    Ok(Some(refresh_token)) => {
                        sessions.replace(AuthSession {
                            refresh_token: Some(refresh_token),
                            username: Some(entry.username.clone()),
                            uuid: entry.uuid.clone(),
                            keyring: "ok".into(),
                            ..AuthSession::default()
                        });
                    }
                    Ok(None) => {
                        writeln!(
                            &log,
                            "ACCOUNTS: index lists {} as persisted but the keyring has no entry",
                            entry.username
                        )
                        .ok();
                    }
                    Err(e) => {
                        sessions.keyring = format!("unavailable: {e}");
                        writeln!(
                            &log,
                            "ACCOUNTS: keyring read for {} failed: {e}",
                            entry.username
                        )
                        .ok();
                    }
                }
            }
            // `persisted()` is newest first; `replace` marked the last one
            // inserted as the most recent login, which is the oldest.
            sessions.last_login = index.persisted().first().map(|e| e.username.clone());
        }
        Err(e) => {
            writeln!(&log, "ACCOUNTS: could not read {}: {e:#}", dir.display()).ok();
        }
    }

    // A daemon that cannot read its queue must not run: it would restart
    // ids at 1 and reuse them against the table's history the moment the
    // file comes back. Refusing to start is the safe failure; the log
    // names the file to repair or remove.
    let jobs_db = match JobDb::open(&acquisition_store::jobs::daemon_db_path(&dir)) {
        Ok(db) => Mutex::new(db),
        Err(e) => {
            anyhow::bail!(
                "could not open {}: {e:#} (repair or remove it; the daemon will not run without its queue)",
                acquisition_store::jobs::daemon_db_path(&dir).display()
            );
        }
    };
    let daemon = Arc::new(Daemon {
        shared: Mutex::new(Shared {
            jobs: HashMap::new(),
            next_id: 1,
            auth: sessions,
            connections: 0,
            last_activity: now,
            errors: VecDeque::new(),
            active_jobs: HashMap::new(),
        }),
        started: now,
        events: broadcast::channel(256).0,
        work: Notify::new(),
        log: Mutex::new(log),
        choke,
        provider,
        artifact: Some(artifact),
        credential_store: Arc::new(OsCredentialStore),
        world: Some(world.name()),
        log_path: Some(log_path.clone()),
        socket_path: Some(bound),
        store_dir: Some(dir.clone()),
        store: Mutex::new(None),
        jobs_db,
        queue_failure: Mutex::new(None),
    });

    {
        let artifact = daemon.artifact.as_ref().expect("run sets the artifact");
        daemon.log(&format!(
            "daemon {} contract {} artifact {} ({}) listening on {} (pid {})",
            acquisition_protocol::VERSION,
            acquisition_protocol::CONTRACT_REVISION,
            artifact.sha256,
            artifact.file.path,
            path.display(),
            std::process::id()
        ));
        daemon.log(&format!(
            "world: {} (lock {}{}) | log: {} | rails state: {}",
            world.name(),
            world.lock_path().display(),
            match &real_mode_lock {
                Some(lock) => format!("; real-mode lock {}", lock.path().display()),
                None => String::new(),
            },
            log_path,
            rails_state.display(),
        ));
        if let Some(what) = journal_rotated {
            daemon.log(&format!("journal: {what}"));
        }
    }
    let (keyring, username) = {
        let s = daemon.shared.lock().unwrap();
        let names = s.auth.usernames();
        (
            daemon.keyring_summary(&s),
            (!names.is_empty()).then(|| names.join(", ")),
        )
    };
    let provider_desc = if daemon.provider.is_real() {
        "ggg (REAL GGG — requests leave this machine)".to_string()
    } else {
        format!("mock at {}", daemon.provider.api_base)
    };
    daemon.log(&format!(
        "provider: {provider_desc} | keyring: {keyring} | session: {}",
        username.as_deref().unwrap_or("none"),
    ));
    let rails = daemon.rails().status();
    daemon.log(&format!(
        "rails: tripwire {} | halted: {} | refresh-failed: {} | ceiling: {} | journal: {} | idle exit after {}s",
        if rails.tripwire_enabled { "ON" } else { "off" },
        rails.halted.as_deref().unwrap_or("no"),
        rails.refresh_failed.as_deref().unwrap_or("no"),
        rails.max_sends.map_or("none".to_string(), |n| n.to_string()),
        rails.journal.as_deref().unwrap_or("off"),
        idle_shutdown_from_env().as_secs(),
    ));
    for warning in daemon.rails().startup_warnings() {
        daemon.note_error(&format!("RAILS CONFIG: {warning}"));
    }
    let (retention, problems) = retention_from_env();
    for problem in problems {
        daemon.note_error(&format!("JOBS CONFIG: {problem}"));
    }
    if let Err(e) = daemon.restore_jobs(retention) {
        anyhow::bail!(
            "restore of the persisted queue failed: {e:#} (repair or remove it; the daemon will not run without its queue)"
        );
    }

    tokio::spawn(daemon.clone().dispatcher());
    tokio::spawn(daemon.clone().idle_watchdog());

    loop {
        let stream = listener.accept().await?;
        tokio::spawn(daemon.clone().handle_conn(stream));
    }
}

#[cfg(test)]
mod tests;
