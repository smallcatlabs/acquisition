//! The daemon: one job queue, a dispatcher running at most one task per
//! scheduling key, and a JSON-lines server on a Unix socket or, on
//! Windows, a named pipe (C112). It owns the shared state and clients
//! reach it over local IPC (C2); every API request is a job, never a call
//! (C4).
//!
//! Lifecycle follows the gpg-agent model: clients spawn it on demand, it exits
//! on its own after a stretch with no connections and no live jobs.
//!
//! This file holds the daemon's state and what every part of it calls:
//! the knobs, `Entry`, `Refusal`, the session layer, `Shared`, the
//! `Daemon` struct, the credential store and the basics. Its children,
//! under `daemon/`, one line each:
//!
//! - `endpoint.rs`: the socket or named pipe on each platform.
//! - `routes.rs`: the routes, the per-route knowledge and the probes.
//! - `queue.rs`: the persisted queue and its admission.
//! - `dispatch.rs`: the dispatcher and a job's life.
//! - `kinds.rs`: what each job kind does and what it records.
//! - `login.rs`: the login flow, logout and forget.
//! - `refresh.rs`: the single-flight refresh and the dead bearer.
//! - `quote.rs`: the quote.
//! - `server.rs`: the connection, the requests, the idle watchdog and the exit.
//! - `start.rs`: `run`, the locks, the log and the knobs read at start.
//!
//! The tests are `tests.rs` and the files under `tests/`.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/daemon.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is each
//! entry's full text as recorded there, moved here on 2026-09-02 because
//! the mechanism it describes is this module's. The registry is current;
//! this is the mechanism as decided, kept beside the code that implements it.
//!
//! ## C31 — Multi-account is one daemon holding many sessions, not one daemon per account.
//!
//! **Multi-account is one daemon holding many sessions, not one daemon per account.** The Cloudflare bound (`SendGate`, 2 live sends) is a per-IP property (P-B, ground truth §1) held as per-process state; two daemons on one machine make it a 4-wide burst that neither sees, with separate tripwires — enforced among real-mode daemons for one OS user by the C83 lock (`ggg.lock` in a private `run` directory of the user's own data directory, from the home the system records and never a variable, C132; taken in `run` before the log directory is made, the log rotated or opened, or anything sends, whatever the root — a refused contender may append its refusal to an existing log afterward; amended 2026-09-11, the split's step 5); other users and processes remain external concurrency no local lock can coordinate, and the tripwire bounds further sends after any resulting visible violation. Rung 11 (2026-08-30) showed the other half: `Account` rules count per account on GGG's side, so two accounts never contend on layer 2 — the only thing they share is layer 1 and the `Ip`-scoped token endpoint, which is exactly what the single gate exists for. Built in two halves with different blast radii (option C): **account as first-class identity first** (store path, job field, keyring key — leaves), then **many live sessions** (a refactor confined to the session layer) — both built by 2026-08-30; every persisted account is restored as a live session at start. Limiter and probe scope keying — `(account, policy)` for `Account` rules, policy alone for `Ip` rules, scope learned from `X-Rate-Limit-Rules` — is a **precondition of the session map, not an optimization**: with two live sessions on one policy each response would overwrite shared state with a different account's counters, and the next send from the other account floods (a 429 path; the "over-waits, never floods" reading only held for rung 11's sequential switch). Decided 2026-08-29, amended 2026-08-30 after review across sessions; design below in "Multi-account design"; built 2026-08-30 through step (6) — step (7)'s live samples are in `RUN-LEDGER.md`.
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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
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
use crate::rails::{BlockShape, Rails, RailsConfig, Verdict, verify_state};
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

mod dispatch;
mod endpoint;
mod kinds;
mod login;
mod queue;
mod quote;
mod refresh;
mod routes;
mod server;
mod start;
#[cfg(test)]
mod tests;

pub use start::{DIAGNOSTIC_CAP_BYTES, rotate_if_over, run};

use dispatch::RUNNING_JOB;
use endpoint::{Accept, BoundSocket, Listener, Stream};
use kinds::{ApiError, validate_apply};
use queue::{
    APPLY_MAX_ACTIONS, DAEMON_SUBMITTER, admit_client, admit_client_submitter, admit_realm,
    admit_segments, bounded_message, check_sleep, client_kinds, describe, describe_str,
};
use refresh::AccessTokenResult;
use routes::why_no_url;
use start::idle_shutdown_from_env;

const IDLE_SHUTDOWN: Duration = Duration::from_secs(60);

const IDLE_POLL: Duration = Duration::from_secs(5);

/// C133: how long the exit waits, on wall-clock (tokio) time, for the
/// jobs past admission, the token refreshes and a login's code exchange
/// to land and be recorded. The client's `STOPPED_WITHIN` is this plus
/// five seconds.
const EXIT_BOUND: Duration = Duration::from_secs(15);

/// How often the exit's drain looks again at what is still in flight.
const DRAIN_POLL: Duration = Duration::from_millis(10);

/// C133: how long past the bound the exit may take to leave (the
/// checkpoints, the socket) before the bound's own thread ends the
/// process at once. Under the five seconds a client's stop waits past the
/// bound.
const LEAVE_GRACE: Duration = Duration::from_secs(3);

/// C133: what ends the process at once, with no lock and no checkpoint:
/// a line for the log and the exit code. `run` installs one that writes
/// through a clone of the log's file, removes the socket and exits; a
/// harness daemon has none, or a test's recorder.
pub(super) type EndAtOnce = Arc<dyn Fn(&str, i32) + Send + Sync>;

/// C133: the exit's own state, outside every lock, so a cause that
/// begins the exit (a stop, the idle verdict, a signal) arms its bound
/// before it can wait on one. The drain's bound runs on tokio time, never
/// the daemon's `Clock`: the harness's `ManualClock` advances on every
/// sleep, and a drain bounded on it would cut at once. The bound is hard
/// on a plain OS thread of its own: a keyring prompt under `shared` can
/// block every tokio worker, and then no tokio timer runs.
struct Exit {
    begun: AtomicBool,
    deadline: OnceLock<tokio::time::Instant>,
    bound: Duration,
    grace: Duration,
    end: OnceLock<EndAtOnce>,
}

impl Exit {
    fn with_bound(bound: Duration) -> Exit {
        Exit {
            begun: AtomicBool::new(false),
            deadline: OnceLock::new(),
            bound,
            grace: LEAVE_GRACE,
            end: OnceLock::new(),
        }
    }

    fn begun(&self) -> bool {
        self.begun.load(Ordering::SeqCst)
    }

    /// Begin the exit, once, taking no lock; `false` if it had begun
    /// already. Arms the drain's deadline and, when an end is installed,
    /// the thread that ends the process at once if the exit has not left
    /// by the bound and its grace, whatever holds a lock or a worker.
    fn arm(&self, why: &str) -> bool {
        if self.begun.swap(true, Ordering::SeqCst) {
            return false;
        }
        let _ = self.deadline.set(tokio::time::Instant::now() + self.bound);
        if let Some(end) = self.end.get() {
            let end = end.clone();
            let at = std::time::Instant::now() + self.bound + self.grace;
            let line = format!(
                "the exit ({why}) had not left {}s past its {}s bound: exiting at once, with no checkpoint",
                self.grace.as_secs_f64(),
                self.bound.as_secs_f64()
            );
            let _ = std::thread::Builder::new()
                .name("acqd-exit-bound".into())
                .spawn(move || {
                    std::thread::sleep(at.saturating_duration_since(std::time::Instant::now()));
                    end(&line, 1);
                });
        }
        true
    }

    /// End the process at once: the installed end, or a bare exit. A
    /// second signal is its one caller, so it is Unix's alone.
    #[cfg(unix)]
    fn end_at_once(&self, why: &str, code: i32) -> ! {
        if let Some(end) = self.end.get() {
            end(why, code);
        }
        std::process::exit(code)
    }
}

impl Default for Exit {
    fn default() -> Exit {
        Exit::with_bound(EXIT_BOUND)
    }
}
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
/// | `refuse_while_stopping` (submit, once the exit has begun, C133) | `the daemon is stopping; try again in a moment` | `refused` |
/// | `validate_apply`, `admit_realm`, `admit_params_size`, `admit_submitter` (submit) | `apply needs a jobs array …`, `action i: …`, `max_requests must be …`, `plan exceeds the budget …`, a realm refusal, `params of N bytes as JSON exceed …`, `submitted_by must be at most …` | `refused` |
/// | `admit_client_submitter` (a client's submit, C136) | `this submitted_by is reserved for the daemon's own work …` | `refused` |
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
/// | `logout`, forgetting a known account that is not live: no index (harness daemons), an unreadable index | `no account index …`, `accounts index: …` | `internal` |
/// | `logout`, forgetting (C144): a keyring clear failed, or the index could not mark the account not persisted — each said, never an `ack` (a keyring turned off with the mark written is the `ack` whose `keyring` is `off`) | `forgot account … (not restored at start), but its keyring entry could not be cleared: …; … tries again`, `… but the keyring is off (…), so no keyring entry was cleared; …`, `… but the account index could not mark it not persisted (…), so a start may still try to restore it` | `internal` |
/// | `logout`, forgetting: the index knows no such account / several | `no account matches …` / the index's message | `not_logged_in` / `ambiguous_account` |
/// | `logout` (C144): after the session was dropped, a keyring clear failed, or the index could not mark the account not persisted — the answer, never an `ack` (as the forget's row) | `logged out … (session dropped, not restored at start), but …`, in the forget's words | `internal` |
///
/// Not a kind of its own: a rails halt never refuses a submit — a halted
/// send waits, and `quote` names the halt in its body. The one request it
/// reaches the wire through is `auth_check`, whose forced refresh the
/// choke point refuses under the halt: that is `upstream`, the provider
/// not answered, with the session still standing. To a job, the same
/// refusal carries `halted`: the job is turned away, not failed (C25).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub kind: ErrorKind,
    pub message: String,
    /// C25: the refresh was refused by a halt, or answered by the block
    /// that sets one; GGG saw nothing. Never on the wire (the kind is
    /// `upstream`); a job that meets it returns to waiting and spends no
    /// retry.
    pub halted: bool,
    /// C25: the refresh was called off before it was sent, for the job
    /// that owned its flight was cancelled. Never on the wire: a caller
    /// still wanting the refresh asks again (`valid_access_token`), and a
    /// cancelled job that meets it concludes cancelled.
    pub cancelled: bool,
}

impl Refusal {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Refusal {
            kind,
            message: message.into(),
            halted: false,
            cancelled: false,
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
/// serving its jobs untouched. C144 — the staged access token serves the
/// login's own profile job alone, the one `profile_job` names
/// (`staged_profile_token`): a client's profile job for the same account
/// in the window runs on the account's session, or fails `not_logged_in`
/// when there is none. A login whose profile fetch fails drops this
/// slot: it fails whole.
struct Staging {
    /// The flow that staged this; stale flows can neither complete nor
    /// abort it.
    flow: u64,
    /// The login's own profile job, marked where it is inserted
    /// (`submit_marked`), so no worker sees it unmarked. `None` only in
    /// the instant between the stage and that submit. The exit's drain
    /// counts the login while this job is terminal and the slot is not
    /// yet cleared (C133: a pending login has the bound to land).
    profile_job: Option<JobId>,
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
    /// Login code exchanges on the wire or landing: a spawned task's,
    /// neither a job nor a refresh, counted here so the exit's drain
    /// sees them (C133).
    exchanges: usize,
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
        format!("no session for {} — run `acq auth`", describe_str(selector)),
    )
}

impl Sessions {
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

    /// The session an operation is for: the named account's, or the sole
    /// one. No selector with several live is refused — the daemon does not
    /// guess whose stash to spend sends on.
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
                    "account {} names several live sessions ({}); use the full name",
                    describe_str(selector),
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
        let entry = self.jobs.get(&id)?;
        let mut info = entry.info.clone();
        info.eta_seconds = if info.state == JobState::Waiting {
            self.waiting_etas(daemon, Some(id)).get(&id).copied()
        } else {
            None
        };
        Some(info)
    }

    /// Every job, by id, each with its eta. The queue is ordered and each
    /// route simulated once per call, never once per job: a per-job
    /// snapshot walked the whole queue for every waiting job, all under
    /// `shared`, and 3,408 jobs held the dispatcher for a second.
    fn list(&self, daemon: &Daemon) -> Vec<JobInfo> {
        let etas = self.waiting_etas(daemon, None);
        let mut jobs: Vec<JobInfo> = self
            .jobs
            .values()
            .map(|e| {
                let mut info = e.info.clone();
                info.eta_seconds = if info.state == JobState::Waiting {
                    etas.get(&info.id).copied()
                } else {
                    None
                };
                info
            })
            .collect();
        jobs.sort_by_key(|info| info.id);
        jobs
    }

    /// The eta, in seconds, of each waiting job (only `only`'s, when
    /// given). Only same-route jobs ahead of a job compete for its
    /// policy, so its estimate is its route's forward simulation at the
    /// count of those; a job whose kind sends nothing waits for nothing.
    fn waiting_etas(&self, daemon: &Daemon, only: Option<JobId>) -> HashMap<JobId, u64> {
        let mut etas = HashMap::new();
        // Per route: how many of its jobs the walk has passed, and the
        // waiting jobs it must estimate, with their counts ahead.
        let mut routes: HashMap<String, (usize, Vec<(JobId, usize)>)> = HashMap::new();
        for id in self.queue_order() {
            let e = &self.jobs[&id];
            let wanted = only.is_none_or(|o| o == id);
            match daemon.keyed_route_for(&e.info.kind, &e.params, e.info.account.as_deref()) {
                Some((route, _)) => {
                    let (passed, wanted_here) = routes.entry(route).or_default();
                    if wanted {
                        wanted_here.push((id, *passed));
                    }
                    *passed += 1;
                }
                None if wanted => {
                    etas.insert(id, 0);
                }
                None => {}
            }
            if only == Some(id) {
                break;
            }
        }
        for (route, (_, wanted)) in routes {
            let Some(&(_, deepest)) = wanted.last() else {
                continue;
            };
            let by_position = daemon.choke.etas_for(&route, deepest + 1);
            for (id, ahead) in wanted {
                etas.insert(id, by_position[ahead].as_secs());
            }
        }
        etas
    }
}

/// The lock order. A thread that holds one of these takes only those
/// after it: `store` → `shared` → `jobs_db` → `queue_failure` → `log`.
/// Under `shared`, the choke point's `limiter` (and under it the gate's
/// `state`), the gate's `state`, the choke point's `sends` and the rails'
/// `state` are leaves: none takes a daemon lock. `log` is innermost. No
/// std guard is held across an `.await` (clippy's `await_holding_lock`,
/// denied by the gate). A guard in a `match` scrutinee lives through
/// every arm, so a result that is reported is bound to a local first
/// (`leave`).
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
    /// C133: the exit, begun at most once. Outside every lock.
    exit: Exit,
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
    /// A test's pause where a refresh's landing releases `shared`, run
    /// once (C144: the landing is one critical section).
    #[cfg(test)]
    refresh_landed_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

/// Where a refresh token is kept (invariant 5): the OS keyring in a
/// running daemon. `Any` lets the in-process harness name the test store
/// its daemons default to.
/// C144: every call's result reaches its caller and the words the user
/// reads; restore reads through `load`, so a test can stage a store that
/// fails at any of the three.
trait CredentialStore: Send + Sync + std::any::Any {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String>;
    /// The stored refresh token; `Ok(None)` when the store works and holds
    /// no entry for the account.
    fn load(&self, service: &str, username: &str) -> Result<Option<String>, String>;
    fn clear(&self, service: &str, username: &str) -> Result<(), auth::NotCleared>;
}

struct OsCredentialStore;

impl CredentialStore for OsCredentialStore {
    fn save(&self, service: &str, refresh_token: &str, username: &str) -> Result<(), String> {
        auth::keyring_save(service, refresh_token, username)
    }

    fn load(&self, service: &str, username: &str) -> Result<Option<String>, String> {
        auth::keyring_load(service, username)
    }

    fn clear(&self, service: &str, username: &str) -> Result<(), auth::NotCleared> {
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

    /// Takes only `log`, the innermost lock: the uptime stamp reads the
    /// clock, not `shared`, so it may be called under any other lock.
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

    fn rails(&self) -> &Arc<Rails> {
        self.choke.rails()
    }

    /// Log a rails trip once, the first time any send path notices it,
    /// and every write of the rails state that failed.
    fn announce_trip(&self) {
        for error in self.rails().take_unwritten() {
            self.note_error(&format!("RAILS STATE NOT WRITTEN: {error}"));
        }
        if let Some(cause) = self.rails().take_unannounced_trip() {
            if cause.starts_with("Cloudflare answered") {
                self.note_error(&format!(
                    "SENDS STOPPED: {cause}; {}",
                    crate::rails::BLOCK_REMEDY
                ));
                return;
            }
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
                    format!(
                        "account {}: not logged in — run `acq auth`",
                        describe_str(req)
                    ),
                )),
                None => Err(Refusal::new(
                    ErrorKind::NotLoggedIn,
                    format!(
                        "account {} is not logged in (live: {}); log in as it first",
                        describe_str(req),
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
}
