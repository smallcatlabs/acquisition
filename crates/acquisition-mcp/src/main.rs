//! `acq-mcp` — MCP server frontend (tracer bullet).
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! A fourth thin client: it consumes exactly the two frontend surfaces —
//! the daemon protocol (via `acquisition_client`) and the shared
//! store's read API — and renders them as MCP tools over stdio.
//!
//! Two structural rules distinguish it from the CLI:
//!
//! - **It never kills or replaces a daemon** (`ConnectOptions::autonomous`).
//!   A contract, artifact or provider mismatch (C84) may be a human's live
//!   GGG run; the MCP reports it and stops. It lazy-spawns only in mock
//!   mode, into an empty socket.
//! - **It never spawns a daemon in real-GGG mode.** A real-mode daemon is
//!   a human's act, via the CLI (it needs the keychain and the browser);
//!   the MCP talks to the one that is running or reports that none is.
//!   Agent traffic against GGG itself is allowed (C14, the owner's ruling
//!   of 2026-09-01): humans, scripts and agents are all clients of the one
//!   daemon, and the daemon is the single gate that enforces GGG's rules.
//!
//! The refresh tracer's plan slice (step 8) is exposed as tools —
//! `sync_policy` / `set_sync_policy` (intent), `refresh_plan`
//! (derivation), `apply_plan` (effect) — through the same shared
//! semantics the CLI uses (C46; `acquisition-plan`: validate-then-CAS policy
//! writes, the validating plan parse, the step-7 staleness gate).
//! Mode rules for the slice: intent reads/writes and offline plan
//! compilation send nothing and work in either mode (a policy write must
//! name the revision it replaces — an agent never clobbers intent it has
//! not read); `apply_plan` and `submit_job` spend through the running
//! daemon in either mode; quote enrichment asks the running daemon in
//! either mode (read-only, never spawning).
//!
//! # As built: the item search (the build plan, step 11b)
//!
//! `search`, `describe` and `show` are adapters of `acquisition-search`,
//! as `acq search` and `acq show` are (C46, C89): what a tool returns is
//! the document the CLI's `--json` prints, and nothing here evaluates.
//! The rulings are `search/LEDGER.md`, "11 · the opening":
//!
//! - **H1 — the tool takes the request itself.** The arguments of
//!   `search` are handed whole to the one reader of a request
//!   (`Asked::read`, what `--request` reads): a request, or what an
//!   answer holds one in — a count with its route, a `next`, an offer —
//!   sent back as printed. No token: a token is a terminal's encoding
//!   of the request a `next` already holds.
//! - **H2 — `scope.account` selects the store**, where a terminal's
//!   `--account` does; every request an answer holds names it.
//! - **H3 — an omitted realm is the terminal's rule** (C96): resolved
//!   over a store of one realm, refused over several.
//! - **H4 — the answer's JSON whole, no text.** The result's structured
//!   content is the answer, and its one content block is that same
//!   document serialized, for a host that reads no structured content.
//! - **H5 — an offer that is a command is a request too**: a refusal's
//!   `offers`, made in the search crate (`error::Offered`).
//! - **A trade site search is a request's query** (C113):
//!   `{"trade": …}`, read and translated by the same reader, so nothing
//!   here translates; `show`'s `against` takes it as a query too.
//! - **A refusal is a result marked `isError`** holding the CLI's
//!   failure document — `error`, `kind`, `readings`, `offers` — never a
//!   JSON-RPC error, whose `data` a host need not show the model (the
//!   owner, 2026-10-10). Every other tool's refusal is the same form,
//!   sorted by one function (`failed`): the account's kinds, `arguments`,
//!   `store`, `no_daemon`, and `refused` for the rest, in the words the
//!   failure has (the release seat's rerun); a JSON-RPC error is kept
//!   for a call the server could not serve — a daemon's own refusal with
//!   its closed kind (C85), a door that did not open, the transport.
//!
//! Three refusals are this adapter's own and no document of the CLI's,
//! a terminal's words for them being no agent's: no account's store
//! (`no_accounts`, naming the directory looked in), an account not
//! named or not known (`account_needed`, `account_unknown`, the known
//! names as readings), and arguments a tool does not take
//! (`arguments`).
//!
//! Pinned by `tests/search_tools.rs`: AQ1–AQ5 through the tools over a
//! fixture, each answer and each of the search crate's refusals held
//! equal to the CLI's document.
//!
//! # As built: pricing (the release, step 1)
//!
//! `price_status`, `price_list`, `price_show`, `shop_render`,
//! `reference_currency`, `price_set` and `price_clear` are adapters of
//! `acquisition_plan::pricing`, the functions `acq price`, `acq shop
//! render` and `acq reference currency` call (C46): a tool returns the
//! document the CLI's `--json` prints for the same request, and a
//! refusal is a result marked `isError` holding the CLI's failure
//! document, `{"error": …}` in the CLI's words (C11). Pricing reads
//! intent and facts through the store: no daemon, no network (C64). The
//! account is selected as the search's tools select it, with their three
//! refusals; arguments a tool does not take are refused as theirs are. A
//! tool opens what the CLI's verb opens: `price_clear` the intent file
//! alone (`open_account_intent`), the rest the facts beside it.
//!
//! One rule is this adapter's and stricter than the CLI's: a write that
//! names no revision is a create (C52; `pricing::Expect::Create`), so a
//! set or a clear over a row of the target's own must name the revision
//! it replaces and is otherwise refused naming the current one; the
//! CLI's default replaces whatever is stored. The shop template is the
//! page's text here, a file there. Pinned by `tests/pricing_tools.rs`.
//!
//! # Decisions as recorded
//!
//! The rulings are the decision registry — `decisions/frontends.md` for this
//! area, `AGENTS.md` for the cross-cutting ones (`C<n>`); what follows is each
//! entry's full text as recorded there, moved here on 2026-09-02 because
//! the mechanism it describes is this module's. The registry is current;
//! this is the mechanism as decided, kept beside the code that implements it.
//!
//! ## C13 — The MCP server is a fourth thin client (`acquisition-mcp`, binary `acq-mcp`, official `…
//!
//! **The MCP server is a fourth thin client (`acquisition-mcp`, binary `acq-mcp`, official `rmcp` SDK over stdio), never in-process with the daemon.** Same reasoning that moved reads to the store: daemon-hosted queries make the daemon an application server. Two structural rules in the rail-6 mold: it never kills or replaces a daemon (autonomous connect policy above), and, while the agent-traffic deferral stood (2026-08-30 → 2026-09-01), it refused `submit_job` in real-GGG mode — store reads and observing a live daemon were always allowed, they send nothing. In real mode it still never spawns a daemon (a human's act: keychain, browser); it talks to the one that is running. It lazy-spawns only in mock mode; login stays human, via the CLI. The tracer is the consumer that validates the protocol: when it has proven the shape, the protocol gets pinned — the GUI arrives to a pinned boundary and proposes changes against it, rather than reopening the question. Decided 2026-08-30.
//!
//! ## C14 — Agent traffic against GGG is allowed; the daemon is the single gate.
//!
//! **Agent traffic against GGG is allowed; the daemon is the single gate.** Owner ruling 2026-09-01 on outside information: GGG permits agent use of the API as long as the API rules are respected. A CLI is already agent-drivable, and so increasingly is a desktop app, so the distinction between human, script and agent clients was never enforceable — what is enforceable is one gate that every client's traffic passes through, and that is the daemon (invariant 1). Consequences: the agent-traffic deferral is lifted; `acq-mcp` submits, applies and quotes in either mode against a running daemon; `quote` over MCP in real mode is simply allowed (it sends nothing). What stays: the MCP never spawns or replaces a daemon in real mode (login and the keychain are human), the live-test rails stay what the live-run skill says, and every client — human or agent — is paced, journaled and halted by the same code. Decided 2026-09-01.

mod transport;

use std::path::PathBuf;

use acquisition_client::client::{
    Client, ConnectError, ConnectOptions, DaemonError, NotSpawned, Observed,
};
use acquisition_plan::{
    PlanError, PolicyWrite, RefreshPlan, apply_params_of, plan_refresh, put_sync_policy,
};
use acquisition_protocol::protocol::{ErrorKind, QuoteJob, Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_search::SearchError;
use acquisition_search::answer::{Asked, QueryInput};
use acquisition_store::{
    AccountEntry, Annotations, Index, Provenance, Resolve, SYNC_POLICY_KEY, SYNC_POLICY_KIND,
    SYNC_POLICY_SCOPE, Store, account_path, store_dir,
};
use anyhow::Result;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router};
use serde::Deserialize;
use serde_json::{Value, json};

/// The provider this process wants (`ACQ_PROVIDER`, C88): the store directory, the
/// plan's provider field and the daemon's `hello` all name the same one.
fn provider() -> &'static str {
    acquisition_protocol::provider::wanted()
}

/// A call the server could not serve, as a JSON-RPC error with the full
/// context chain.
fn err(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(format!("{e:#}"), None)
}

/// A tool's failure, sorted by the one rule every tool shares, so no
/// tool decides it alone. A call the server could not serve is a
/// JSON-RPC error: a daemon's own refusal with its closed kind
/// ([`daemon_error`], C85), a door that did not open
/// ([`connect_error`]), the conversation with the daemon failing
/// ([`Unserved`]). Everything else is a refusal — the request was wrong,
/// or the store or policy cannot serve it, or no daemon runs — a result
/// marked isError holding the document [`refusal`] words. The release
/// seat's rerun met an unknown account and an uncovered league as
/// JSON-RPC errors where the instructions promise results (mcp-opus-2
/// calls 50, 46).
fn failed(e: anyhow::Error) -> Told {
    if let Some(refused) = DaemonError::find(&e) {
        return Err(daemon_error(refused.kind, format!("{e:#}")));
    }
    // no daemon is a door that did not open, and a step for the person
    if e.downcast_ref::<NoDaemon>().is_none() {
        if let Some(door) = e.chain().find_map(|c| c.downcast_ref::<ConnectError>()) {
            return Err(connect_error(door, format!("{e:#}")));
        }
        if e.downcast_ref::<Unserved>().is_some() {
            return Err(err(e));
        }
    }
    Ok(CallToolResult::structured_error(refusal(&e, "account")))
}

/// A refusal's document, in the words the failure already has: `error`
/// and a `kind` — `no_daemon` with `next`, the person's command; the
/// account's (`no_accounts`; `account_unknown` and `account_needed`,
/// the known names as `readings`, `named` the argument that selects
/// one); `arguments` for a parameter the tool cannot read; `store` for
/// a store that will not open; `refused` for the rest.
fn refusal(e: &anyhow::Error, named: &str) -> Value {
    let error = format!("{e:#}");
    if e.downcast_ref::<NoDaemon>().is_some() {
        return json!({ "error": error, "kind": "no_daemon", "next": start_one() });
    }
    if e.downcast_ref::<NoAccounts>().is_some() {
        return json!({ "error": error, "kind": "no_accounts" });
    }
    let known = || {
        Index::load(&store_dir(provider()))
            .map(|index| {
                index
                    .entries()
                    .iter()
                    .map(|e| e.username.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    match e.downcast_ref::<Resolve>() {
        Some(Resolve::NotFound(selector)) => {
            return json!({
                "error": format!(
                    "no account matches `{selector}`: `{named}` takes a username, with or \
                     without its #discriminator, or a uuid"
                ),
                "kind": "account_unknown",
                "readings": known(),
            });
        }
        Some(Resolve::Ambiguous(names)) => {
            return json!({
                "error": format!("{} accounts are known and `{named}` selects one", names.len()),
                "kind": "account_needed",
                "readings": names,
            });
        }
        _ => {}
    }
    let kind = if e.downcast_ref::<BadArguments>().is_some() {
        "arguments"
    } else if e.downcast_ref::<Unopened>().is_some() {
        "store"
    } else {
        "refused"
    };
    json!({ "error": error, "kind": kind })
}

/// Arguments a tool cannot read: refused as `arguments`.
#[derive(Debug)]
struct BadArguments(String);

/// A store, intent or index file that will not open: refused as `store`,
/// in its own words.
#[derive(Debug)]
struct Unopened(String);

/// The conversation with the daemon failed — a door that did not open
/// in words of its own, the transport, an answer of the wrong shape: the
/// call was not served, a JSON-RPC error.
#[derive(Debug)]
struct Unserved(String);

impl std::fmt::Display for BadArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::fmt::Display for Unopened {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::fmt::Display for Unserved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BadArguments {}
impl std::error::Error for Unopened {}
impl std::error::Error for Unserved {}

/// A file's failure to open, marked so [`refusal`] calls it `store`.
fn unopened(e: anyhow::Error) -> anyhow::Error {
    Unopened(format!("{e:#}")).into()
}

/// A failure of the daemon conversation, marked so [`failed`] keeps it
/// a JSON-RPC error; a daemon refusal, a door or no daemon is already
/// typed and passes as it is.
fn unserved(e: anyhow::Error) -> anyhow::Error {
    let typed = DaemonError::find(&e).is_some()
        || e.downcast_ref::<NoDaemon>().is_some()
        || e.chain()
            .any(|c| c.downcast_ref::<ConnectError>().is_some());
    if typed {
        e
    } else {
        Unserved(format!("{e:#}")).into()
    }
}

/// A tool's answer: the document, or its failure as [`failed`] sorts it.
fn settled<T: serde::Serialize>(answer: Result<T>) -> Told {
    match answer {
        Ok(document) => told(&document),
        Err(e) => failed(e),
    }
}

/// [`settled`] over a tool's body, for a tool that never awaits.
fn answer<T: serde::Serialize>(body: impl FnOnce() -> Result<T>) -> Told {
    settled(body())
}

/// A door that did not open, as a JSON-RPC error: the server could not
/// serve the call (`internal_error`), and `data.connect` names which
/// door failed — `absent`, `incompatible`, `other_world`,
/// `spawn_failed`, `unresponsive`, `transport` — so an agent can branch
/// on it the way it branches on a daemon error's kind.
fn connect_error(door: &ConnectError, message: String) -> ErrorData {
    let connect = match door {
        ConnectError::Absent { .. } => "absent",
        ConnectError::Incompatible { .. } => "incompatible",
        ConnectError::OtherWorld { .. } => "other_world",
        ConnectError::SpawnFailed { .. } => "spawn_failed",
        ConnectError::Unresponsive(_) => "unresponsive",
        ConnectError::Transport(_) => "transport",
    };
    ErrorData::internal_error(message, Some(json!({ "connect": connect })))
}

/// A daemon refusal as a JSON-RPC error (C85): the code follows the kind
/// — the request was wrong (`invalid_request`), its parameters were
/// (`invalid_params`), it named nothing (`resource_not_found`), or the
/// daemon could not serve it (`internal_error`) — and the kind itself
/// rides in `data` so an agent can branch on the closed set.
fn daemon_error(kind: ErrorKind, message: String) -> ErrorData {
    let data = Some(json!({ "kind": kind }));
    match kind {
        ErrorKind::BadRequest => ErrorData::invalid_request(message, data),
        ErrorKind::Refused | ErrorKind::AmbiguousAccount | ErrorKind::WrongState => {
            ErrorData::invalid_params(message, data)
        }
        ErrorKind::UnknownJob => ErrorData::resource_not_found(message, data),
        ErrorKind::NotLoggedIn
        | ErrorKind::QueueFailed
        | ErrorKind::Upstream
        | ErrorKind::Internal => ErrorData::internal_error(message, data),
    }
}

/// Open one account's store file, resolved against the non-secret index —
/// no daemon involved (the store is the second frontend surface).
fn open_store(account: Option<&str>) -> Result<Store> {
    let (dir, entry) = resolve(account)?;
    Store::open(&account_path(&dir, &entry.username)).map_err(unopened)
}

/// No account is known: the error names the directory looked in, and
/// no login for the caller to run (the agent's seat, F10: a server
/// pointed at the wrong root answered every call with `acq auth`, the
/// one act an agent cannot perform).
#[derive(Debug)]
struct NoAccounts(PathBuf);

impl std::fmt::Display for NoAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "no account's store in {} (provider {}; ACQ_STORE_DIR sets the root this server \
             looks under): a store is made by a login, which is a human's, from a terminal",
            self.0.display(),
            provider()
        )
    }
}

impl std::error::Error for NoAccounts {}

fn resolve(account: Option<&str>) -> Result<(PathBuf, AccountEntry)> {
    let dir = store_dir(provider());
    let index = Index::load(&dir).map_err(unopened)?;
    match index.resolve(account) {
        Ok(entry) => Ok((dir, entry.clone())),
        Err(Resolve::Empty) => Err(NoAccounts(dir).into()),
        Err(e) => Err(e.into()),
    }
}

// ---- the item search: adapters of `acquisition-search` ----

/// What a tool of the search returns: the document the CLI's `--json`
/// prints — an answer, or a failure in a result marked an error.
type Told = Result<CallToolResult, ErrorData>;

/// The document as a tool's result. The text block is serialized from the
/// document itself, so its keys come in the order the document declares
/// them (C115); the structured content, a JSON value, holds them in its
/// own order, and an agent reads the text.
fn told<T: serde::Serialize>(document: &T) -> Told {
    let text = serde_json::to_string(document).map_err(|e| err(e.into()))?;
    let mut result = serde_json::to_value(document)
        .map(CallToolResult::structured)
        .map_err(|e| err(e.into()))?;
    result.content = vec![rmcp::model::ContentBlock::text(text)];
    Ok(result)
}

/// A refusal, as the search crate words it (C11): `error`, `kind`,
/// `readings`, `offers`.
fn refused(e: impl Into<SearchError>) -> CallToolResult {
    CallToolResult::structured_error(e.into().to_json())
}

/// The selected account's facts and its intent file, as `acq search`
/// opens them: the search reads both and writes neither. Built on
/// [`open_account_intent`], whose refusals it shares; a facts file that
/// will not open is refused as `store`. `named` is the argument that
/// selects an account, for the refusal that needs one.
fn open_search(
    account: Option<&str>,
    named: &str,
) -> std::result::Result<(Store, Annotations), CallToolResult> {
    let (dir, entry, intent) = open_account_intent(account, named)?;
    let store = Store::open(&account_path(&dir, &entry.username))
        .map_err(|e| CallToolResult::structured_error(refusal(&unopened(e), named)))?;
    Ok((store, intent))
}

/// The selected account's intent file alone, for a verb that reads and
/// writes nothing else — the MCP opens for a verb what the CLI opens for
/// it, so a facts file the CLI's verb never opens cannot refuse the
/// tool's. A failure is a refusal document ([`refusal`]): no account's
/// store, an account not known or not named, the file (`store`).
fn open_account_intent(
    account: Option<&str>,
    named: &str,
) -> std::result::Result<(PathBuf, AccountEntry, Annotations), CallToolResult> {
    open_intent(account).map_err(|e| CallToolResult::structured_error(refusal(&e, named)))
}

/// A tool's arguments, read here and not by the SDK, whose refusal of
/// arguments it cannot read is a line of text and no document: what
/// the tool does not take is refused as any refusal is, and a key it
/// does not know is never read past. The schema is `T`'s.
struct Args<T>(serde_json::Map<String, Value>, std::marker::PhantomData<T>);

impl<'de, T> Deserialize<'de> for Args<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|map| Args(map, std::marker::PhantomData))
    }
}

impl<T: schemars::JsonSchema> schemars::JsonSchema for Args<T> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        T::schema_name()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        T::json_schema(generator)
    }
}

impl<T: serde::de::DeserializeOwned> Args<T> {
    /// The arguments, or [`BadArguments`].
    fn parse(self) -> Result<T> {
        serde_json::from_value(Value::Object(self.0)).map_err(|e| {
            BadArguments(format!("these are not arguments the tool takes: {e}")).into()
        })
    }

    /// The arguments, or their refusal document.
    fn read(self) -> std::result::Result<T, CallToolResult> {
        self.parse()
            .map_err(|e| CallToolResult::structured_error(refusal(&e, "account")))
    }
}

/// The selected account's provider directory, index entry, and annotations
/// file — the latter addressed by the uuid the index maps the account to
/// (an entry without one predates uuid-at-login; intent cannot be bound
/// to it). Same shape as the CLI's `open_intent`.
fn open_intent(account: Option<&str>) -> Result<(PathBuf, AccountEntry, Annotations)> {
    let (dir, entry) = resolve(account)?;
    let Some(uuid) = entry.uuid.as_deref() else {
        anyhow::bail!(
            "account {} has no recorded uuid (a login predating uuid-at-login); \
             a human can fix it with one `acq auth` login",
            entry.username
        );
    };
    let annotations = Annotations::open_for(&dir, uuid).map_err(|e| unopened(e.into()))?;
    Ok((dir, entry, annotations))
}

/// The channel every intent write from this binary is stamped with (C65).
const WRITTEN_VIA: &str = "mcp";

/// The quote attempt is bounded and best-effort, same as the CLI's: the
/// offline plan is the deliverable, and a wedged daemon must not keep it
/// from returning.
const QUOTE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Try to enrich the plan with a *running* daemon's read-only quote, in
/// either mode (owner ruling 2026-09-01: a quote sends nothing, and agent
/// use of the gate is allowed). The connection never spawns or replaces a
/// daemon — the plan promises to spend nothing, and a kill-and-respawn is
/// a spend (the successor resumes the persisted queue).
async fn try_quote(plan: RefreshPlan) -> (RefreshPlan, Option<String>) {
    let Some(account) = plan.account_name.clone() else {
        return (
            plan,
            Some("no quote: the facts record no account name to quote as".into()),
        );
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
            Ok(Observed::Absent) => return Err("no daemon running".to_string()),
            Ok(Observed::Incompatible(found)) => return Err(found.to_string()),
            Ok(Observed::Unresponsive(found)) => return Err(found.to_string()),
            Err(e) => return Err(format!("{e:#}")),
        };
        client
            .quote(jobs, Some(account))
            .await
            .map_err(|e| format!("{e:#}"))
    };
    match tokio::time::timeout(QUOTE_TIMEOUT, attempt).await {
        Err(_) => (
            plan,
            Some(format!(
                "no quote: the daemon did not answer within {QUOTE_TIMEOUT:?} — plan \
                 compiled offline"
            )),
        ),
        Ok(Err(why)) => (
            plan,
            Some(format!("no quote: {why} — plan compiled offline")),
        ),
        Ok(Ok(quote)) => match plan.clone().with_quote(quote) {
            Ok(enriched) => (enriched, None),
            Err(e) => (plan, Some(format!("daemon quote rejected: {e}"))),
        },
    }
}

/// A spending tool's connect, under the autonomous policy: never kill or
/// replace; lazy-spawn only in mock mode (spawning a real-GGG daemon is
/// the human's act, via the CLI).
/// What a refusal for want of a daemon tells the agent to have the person
/// do: the README's own line ("Your agent"), so the remedy is one command
/// in every mode. The release seat's MCP runs (mcp-opus calls 40 to 42,
/// mcp-sonnet 42 to 45) were told what was wrong and not this. The line
/// runs as printed in this server's world: it carries the provider when
/// it is not the default and the store root when one is set, quoted for
/// a shell — typed bare beside a mock server with its store elsewhere,
/// it started a daemon for another provider and store (the rerun,
/// mcp-opus-2 calls 45, 47).
fn start_one() -> String {
    let mut world = String::new();
    if !acquisition_protocol::provider::ggg_mode() {
        world.push_str(&format!(
            "{}={} ",
            acquisition_protocol::provider::KNOB,
            provider()
        ));
    }
    if std::env::var_os("ACQ_STORE_DIR").is_some() {
        world.push_str(&format!(
            "ACQ_STORE_DIR={} ",
            acquisition_search::answer::shell_quoted(
                &acquisition_store::world::intended_root()
                    .display()
                    .to_string()
            )
        ));
    }
    format!(
        "a person starts one from a terminal: `{world}ACQ_IDLE_SHUTDOWN=3600 acq auth check` \
         (the README, \"Your agent\")"
    )
}

/// No daemon to ask: the one failure a daemon tool answers as a refusal
/// the agent can read — a result marked isError, `kind: "no_daemon"`,
/// `next` the person's command — rather than a protocol error, since the
/// remedy is a step in the agent's conversation, not a broken call (the
/// release seat; the owner's triage, 2026-10-19).
#[derive(Debug)]
struct NoDaemon {
    real: bool,
}

impl std::fmt::Display for NoDaemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.real {
            write!(
                f,
                "no daemon running in real-GGG mode, and this server never starts one there — {}",
                start_one()
            )
        } else {
            write!(f, "no daemon running — {}", start_one())
        }
    }
}

impl std::error::Error for NoDaemon {}

async fn connect(spawn: bool) -> Result<Client> {
    let real = acquisition_protocol::provider::ggg_mode();
    let spawn = spawn && !real;
    match Client::connect(ConnectOptions::autonomous(spawn)).await {
        Ok(client) => Ok(client),
        // Absent by this server's own policy: the remedy is the human's,
        // and the message says so rather than promising a spawn.
        Err(
            e @ ConnectError::Absent {
                because: NotSpawned::Policy,
            },
        ) if real => Err(anyhow::Error::from(e).context(NoDaemon { real: true })),
        Err(e @ ConnectError::Absent { .. }) => {
            Err(anyhow::Error::from(e).context(NoDaemon { real: false }))
        }
        Err(e) => Err(e.into()),
    }
}

/// The running daemon, for a tool that observes it or acts on it (C10):
/// never spawns or replaces; absence and a mismatch are errors that say
/// which.
async fn attach() -> Result<Client> {
    client_of(observe().await?)
}

/// One look at the daemon, never spawning; its failure is the door's.
async fn observe() -> Result<Observed> {
    Client::observe().await.map_err(unserved)
}

/// One request to the daemon: its answer, a daemon refusal as its closed
/// kind (C85), or the transport's failure, not served.
async fn ask(client: &mut Client, request: &Request) -> Result<Response> {
    match client.request(request).await.map_err(unserved)? {
        Response::Error { kind, message } => Err(DaemonError { kind, message }.into()),
        other => Ok(other),
    }
}

/// An answer of a shape the request does not take: not served.
fn unexpected(response: Response) -> anyhow::Error {
    Unserved(format!("unexpected response: {response:?}")).into()
}

/// The queue on disk, for a tool with no daemon to ask (C45): this
/// server's world and provider, `None` when neither has been created.
fn persisted_queue() -> Result<Option<acquisition_store::jobs::Persisted>> {
    acquisition_store::jobs::Persisted::observe(acquisition_protocol::provider::wanted())
        .map_err(unopened)
}

/// The client an observation yields, or the error that says why not.
fn client_of(observed: Observed) -> Result<Client> {
    let door = match observed {
        Observed::Compatible(client) => return Ok(client),
        Observed::Absent => return Err(NoDaemon { real: false }.into()),
        Observed::Unresponsive(found) => format!("{found}"),
        Observed::Incompatible(found) => format!(
            "{found}; this server never replaces a daemon — resolve it with the CLI (`acq daemon stop`{})",
            if found.world_matches() {
                ""
            } else {
                ", or point ACQ_STORE_DIR at its world"
            }
        ),
    };
    Err(Unserved(door).into())
}

/// A `realm` tool parameter: pc when omitted (as on the wire), else one
/// of the documented realms; anything else is refused as an argument.
fn realm_param(realm: Option<&str>) -> Result<Realm> {
    match realm {
        None => Ok(Realm::DEFAULT),
        Some(s) => Realm::parse(s).ok_or_else(|| {
            BadArguments(format!(
                "unknown realm {s:?} (one of {})",
                Realm::ALL.map(Realm::as_str).join(", ")
            ))
            .into()
        }),
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
struct TabsParams {
    /// League name; defaults to "Standard".
    league: Option<String>,
    /// List every tab of a league of more than thirty, or of the
    /// selection, in place of the ten with the most items.
    expand: Option<bool>,
    /// Only tabs of this type, as GGG names it (UniqueStash, MapStash,
    /// PremiumStash …; `by_type` lists them).
    r#type: Option<String>,
    /// Only tabs never fetched.
    never_fetched: Option<bool>,
    /// Realm: pc (default), xbox, or sony.
    realm: Option<String>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct CharactersParams {
    /// Restrict to one league; omitted lists every league.
    league: Option<String>,
    /// Restrict to one realm (pc, xbox, sony, poe2); omitted lists every realm.
    realm: Option<String>,
    account: Option<String>,
}

/// The arguments of `search`: the request, read by the search crate's
/// own reader and by nothing here (H1). The schema says what a request
/// holds; the reader refuses what is none.
#[derive(Deserialize)]
#[serde(transparent)]
struct RequestParams(serde_json::Map<String, Value>);

impl schemars::JsonSchema for RequestParams {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SearchRequest".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "properties": {
                "scope": {
                    "type": "object",
                    "description": "What is searched: {\"account\": a username or uuid, needed only when several accounts are known; \"realm\": pc, xbox, sony, poe2 or all, needed only when the store holds several; \"membership\": \"live\" (the default); \"all\", which adds removed items, each row marked with when it left; or \"removed\", the removed items alone: what has left the stash and the characters}. Under all and removed the answer's scope counts the live items as `items` and the removed as `removed`, whichever were searched."
                },
                "query": {
                    "type": "object",
                    "description": "{\"text\": a query in the language `describe` prints} or {\"tree\": the tree an answer returned}, never both. Omitted: every item in scope. A refinement is the old text in parentheses and a new term. Or {\"trade\": a trade site search}: its URL, its id or its query JSON as a string, or the query JSON as an object (PoE1), translated with no network to the text a person would type; the answer's `query.trade` holds every clause with its status — exact, ambiguous, unsupported, listing_only, disabled; an exact one with a term of its own its `at`, `term` and `path` alone, any other — a sum's filter, a group, a clause not exact — the site's filter and what the catalogue says of it — the `remainder` that did not run, and `fidelity`, how far the text is the site's search: exact, ambiguous where a clause ran as the site may not read it, or partial where the remainder is not empty, the answer then not the site's search. An ambiguous clause whose other reading is known carries `other`: that reading, and the items in scope it `adds` and `drops` against the query as run. Never how far the items are the site's: the answer is over the items the store has fetched, priced or not — what it never fetched is `scope.never_fetched` — and the site shows public listings, a tab's being `tab.public`. A URL's realm is the scope, so `scope.realm` beside one is the URL's or nothing; an id or JSON takes a realm as a text does. A `text` sent beside `trade` must be the one it translates to, as `view.rows.next` sends it: an edited text is refused — drop `trade` to run the edit as your own query. `describe` with names [\"trade\"] says what each clause becomes."
                },
                "view": {
                    "type": "object",
                    "description": "One of: {\"rows\": {\"limit\": N, \"sort\": a value, \"desc\": true}}, the default, 20 rows where no limit is given; {\"counts\": {\"keys\": [a key, …], \"sum\": a value, \"limit\": N}}, a table for each key and no rows; {\"cross\": {\"keys\": [two keys]}}, one crossed table; {\"explain\": {\"path\": a node's path, a term's as `terms[].path` gives it}}, the total with that node forced true and forced false. Beside any of them, \"routes\": true gives every count its request, a terminal's --routes; alone, {\"routes\": true} is the default rows. `describe` with names [\"counts\"] lists the keys. A sort's value is a field (reqlevel), a computed value (pseudo.total_res), a line's number (line(\"# to maximum Life\").arg1) or a sum of one (`sum(\"# to maximum Life\")`), spanning every source unless `source=` names one; an item that lacks it is last, ascending or descending."
                },
                "context": {
                    "type": "string",
                    "description": "\"matches\" (the default), or \"corpus\": the view over every item in scope, the query set aside."
                },
                "basis": {
                    "type": "object",
                    "description": "Carried by a continuation, as an answer's `next` holds it; never written by hand."
                },
                "request": {
                    "type": "object",
                    "description": "In place of the fields above: a request as an answer holds one — a count (`count`, `request`, and `denominator`, what the request searches: the scope, never the share the count is of), `view.rows.next`, or an offer — sent whole, as printed."
                },
                "count": {
                    "type": "integer",
                    "description": "Beside `request`, the count it was printed with: the answer then says under `before` where the count has moved."
                },
                "counted_at": {
                    "type": "object",
                    "description": "Beside `request` and `count`, the `basis` of the answer the count came from: the answer then says under `before` wherever the basis has moved."
                }
            }
        })
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct DescribeParams {
    /// Only these entries: a section — `fields`, `line`, `linked`,
    /// `values`, `composition`, `operators`, `slots`, `computed`,
    /// `counts`, `explain` — or a name within one: `league`,
    /// `total_res`. Omitted: the whole language.
    names: Option<Vec<String>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct ShowParams {
    /// An item's id, whole, as an answer printed it.
    id: String,
    /// Also return the body as the store holds it, verbatim.
    body: Option<bool>,
    /// Why the item does or does not match a query: `{"text": …}`,
    /// `{"tree": …}` or `{"trade": …}`, as a request's `query`.
    against: Option<Value>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct StoreParams {
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct EventsParams {
    /// How far back to look (default 24 hours).
    hours: Option<f64>,
    /// Maximum rows (default 200).
    limit: Option<usize>,
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct SubmitParams {
    /// Job kind: profile, characters, character, leagues, stashes, stash,
    /// refresh (mock daemon also: sleep, fetch, whoami).
    kind: String,
    /// Kind-specific params, e.g. {"league": "Standard", "id": "..."} for
    /// stash. Passed verbatim; visible to every connected client.
    params: Option<Value>,
    /// Account to run as; required when several sessions are live.
    account: Option<String>,
    /// Higher runs sooner (default 0).
    priority: Option<u8>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct JobParams {
    /// The job id, as returned by submit_job.
    id: u64,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct SetPolicyParams {
    /// The sync-policy value, e.g.
    /// {"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}
    /// — per league, `tabs` and/or `characters` ("all" or an id list;
    /// absent means no coverage of that facet; an entry covering neither is
    /// refused). `tabs` is refused under realm poe2 (stashes are PoE1
    /// only); `characters` is taken under every realm. Older values still
    /// parse: a version-1 top-level `leagues` map as realm pc, and version
    /// 1/2 as tab coverage only.
    /// Validated strictly before anything lands — a typo'd field is
    /// refused, never half-honored. Each realm it names replaces that
    /// realm's stored policy, every league of it; a realm it does not
    /// name stays, and a realm named with no leagues
    /// ({"poe2":{"leagues":{}}}) is removed.
    value: Value,
    /// The revision this write replaces, from sync_policy. Required when a
    /// policy exists (replacing intent you have not read is refused as a
    /// conflict naming the current revision), with `replace` too; omit
    /// only to create the first policy.
    if_revision: Option<i64>,
    /// true writes the whole policy: every realm `value` does not name is
    /// removed. Default false: the realms `value` names, the rest kept.
    replace: Option<bool>,
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct PlanParams {
    /// League name; defaults to "Standard".
    league: Option<String>,
    /// Realm: pc (default), xbox, sony, or poe2 (a poe2 plan can carry
    /// character work only).
    realm: Option<String>,
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct ApplyParams {
    /// The plan envelope exactly as refresh_plan returned it (the `plan`
    /// field). Re-validated here; a tampered or hand-built envelope will
    /// not parse.
    plan: Value,
    /// Refuse at admission — before any child job exists — if the plan
    /// authorizes more logical requests than this.
    max_requests: Option<u64>,
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceStatusParams {
    /// The league; Standard when omitted.
    league: Option<String>,
    /// pc (the default), xbox, sony or poe2.
    realm: Option<String>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceShowParams {
    /// One address: `item/<id>`, `character/<id>`, `tab/<realm>/<id>` or
    /// `substash/<realm>/<parent>/<id>`. An answer holds a target as an
    /// object, `{scope, realm, parent, id}`: the address is its parts in
    /// that order joined by `/`, the absent ones left out.
    target: String,
    /// The league to read the target in; omitted, the league the facts
    /// hold it in, so an address an answer printed reads back as printed.
    league: Option<String>,
    /// The realm of an item or character target (pc when omitted); a tab
    /// or substash address carries its own, and a realm given beside it
    /// must agree.
    realm: Option<String>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceListParams {
    /// The league; omitted, the league of the container `in` or
    /// `covered_by` names, else Standard.
    league: Option<String>,
    /// pc (the default), xbox, sony or poe2; a tab or substash address
    /// given to `in` or `covered_by` carries its own.
    realm: Option<String>,
    /// Only items in this relation: none, manual_only, game_only, agree or
    /// conflict.
    relation: Option<String>,
    /// Only items whose price this side decides: game, manual, none or
    /// unresolved.
    effective: Option<String>,
    /// Only items physically in this container: a tab, substash or
    /// character address.
    #[serde(rename = "in")]
    location: Option<String>,
    /// Only items a row on this target would cover: a folder's tabs'
    /// items, a map or unique tab's substashes' items too.
    covered_by: Option<String>,
    /// List at most this many items, the first in listing order (some
    /// 1.3 KB an item); `total` says how many the selection holds.
    limit: Option<usize>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct ShopRenderParams {
    /// The league; Standard when omitted.
    league: Option<String>,
    /// pc (the default), xbox, sony or poe2.
    realm: Option<String>,
    /// Characters per page, the template included: the forum's hard limit
    /// when omitted.
    size: Option<usize>,
    /// The page's text, holding `[items]` exactly once, which each page
    /// replaces with its items; omitted, a page is the items alone.
    template: Option<String>,
    /// One page alone, by its number from 1: that page's record in place
    /// of the whole render.
    page: Option<usize>,
    /// List every item left off in `left_out`, past the first 50 and the
    /// items nothing applies to included (some 200 bytes an item; a
    /// league holds thousands); they are counted either way.
    expand: Option<bool>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReferenceCurrencyParams {
    /// A tag, the game's word, or a legacy alias to resolve, exact and
    /// case-sensitive; omitted, the whole table.
    word: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceSetParams {
    /// One address: `item/<id>`, `character/<id>`, `tab/<realm>/<id>` or
    /// `substash/<realm>/<parent>/<id>`. An answer holds a target as an
    /// object, `{scope, realm, parent, id}`: the address is its parts in
    /// that order joined by `/`, the absent ones left out.
    target: String,
    /// exact, negotiable, no_price or skip (the game's `price`, `b/o` and
    /// `~skip` work too).
    r#type: String,
    /// For exact and negotiable: a decimal of up to four places (`12.5`),
    /// or a `wanted/lot` ratio (`1/5`), as a string.
    amount: Option<String>,
    /// For exact and negotiable: a tag of reference_currency.
    currency: Option<String>,
    /// The revision of the target's own row this write replaces: price_show's
    /// `listing.manual.revision`, when `listing.manual.from` is the target
    /// itself, or the revision a refusal names. Required when the target
    /// has a row of its own; omit it only to create one.
    if_revision: Option<i64>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceClearParams {
    /// The address, as for price_set.
    target: String,
    /// The revision of the target's own row this clear removes, as for
    /// price_set. Required: a clear without it is refused naming the
    /// current revision.
    if_revision: Option<i64>,
    /// Account selector (username with or without `#discriminator`, or
    /// uuid); required only when several accounts are known.
    account: Option<String>,
}

/// A pricing verb's answer as a tool's result: the document the CLI's
/// `--json` prints, or its failure document, `{"error": …}`, in a result
/// marked an error.
fn priced<T: serde::Serialize>(answer: Result<T>) -> Told {
    match answer {
        Ok(document) => told(&document),
        Err(e) => {
            let mut report = json!({ "error": format!("{e:#}") });
            if let Some(refused) = acquisition_plan::pricing::Refused::find(&e) {
                report.as_object_mut().unwrap().extend(refused.fields());
            }
            Ok(CallToolResult::structured_error(report))
        }
    }
}

/// A pricing tool's `realm`, read as the CLI's `--realm` is; a word that
/// is no realm is refused as any argument the tool does not take.
fn price_realm(realm: Option<&str>) -> std::result::Result<Option<Realm>, CallToolResult> {
    realm
        .map(|s| {
            Realm::parse(s).ok_or_else(|| {
                CallToolResult::structured_error(json!({
                    "error": format!(
                        "unknown realm {s:?} (one of {})",
                        Realm::ALL.map(Realm::as_str).join(", ")
                    ),
                    "kind": "arguments",
                }))
            })
        })
        .transpose()
}

struct AcqMcp;

#[tool_router]
impl AcqMcp {
    // ---- store reads: no daemon, no GGG traffic ----

    #[tool(
        description = "Accounts this machine has logged into (from the store's non-secret index; no daemon)."
    )]
    fn accounts(&self) -> Told {
        answer(|| {
            let index = Index::load(&store_dir(provider())).map_err(unopened)?;
            let rows: Vec<Value> = index
                .entries()
                .iter()
                .map(|e| {
                    json!({
                        "username": e.username,
                        "last_login": e.last_login,
                        "persisted": e.persisted,
                        "uuid": e.uuid,
                    })
                })
                .collect();
            Ok(json!({ "provider": provider(), "accounts": rows }))
        })
    }

    #[tool(
        description = "The stash tabs of a league, counted first, from the shared store (no daemon, no network): how many tabs, fetched, never fetched and folders, how many items, and the same by tab type (`by_type`, the most items first); then `rows`, the tabs listed — every one where the league holds thirty or fewer, else the ten with the most items (`listed`: `all` or `most_items`) — each with its type, item count and fetch time, a substash naming its `parent`. `left_out` counts the tabs not listed; `expand: true` lists them all (some 220 bytes a tab). `type` and `never_fetched: true` select the tabs listed — the never fetched of one type by name, which a refresh takes — under the league's counts, `selected` counting them. A name ending ` (Remove-only)` is GGG's mark on a tab whose items can only be taken out; a search asks it as `tab.remove_only`."
    )]
    fn tabs(&self, Parameters(p): Parameters<Args<TabsParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = realm_param(p.realm.as_deref())?;
            let store = open_store(p.account.as_deref())?;
            let league = p.league.as_deref().unwrap_or("Standard");
            let rows = store.tabs(realm.as_str(), league)?;
            Ok(acquisition_search::tabs::Tabs::of(
                realm.as_str(),
                league,
                rows,
                &acquisition_search::tabs::Selection {
                    kind: p.r#type,
                    never_fetched: p.never_fetched.unwrap_or(false),
                },
                p.expand.unwrap_or(false),
            ))
        })
    }

    #[tool(
        description = "Characters known to the store, with class, level, league, and two counts — `item_count`, the live items the store holds at the character now, and `fetched_items`, what the last full fetch's body carried (equipment + inventory, socketed gems not counted), null until one — (no daemon, no network). Fresh data is a `characters` or `character` job."
    )]
    fn characters(&self, Parameters(p): Parameters<Args<CharactersParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = p
                .realm
                .as_deref()
                .map(|r| realm_param(Some(r)))
                .transpose()?;
            let store = open_store(p.account.as_deref())?;
            store.characters(realm.map(Realm::as_str), p.league.as_deref())
        })
    }

    #[tool(
        description = "Item search over the shared store (no daemon, no network): a query in the language `describe` prints, asked of every item in scope. The arguments are the request: {} is every item; {\"query\": {\"text\": \"class:ring rarity=rare\"}} is a query, and {\"query\": {\"trade\": a trade site URL}} a site search translated to one. The answer is one JSON document, its keys in this order: `query` (the text as understood, and its tree), `scope` (account, realm, `items` every live item in scope before any term, what was and was not fetched: `never_fetched` counts the tabs, their pages and the characters a listing names and no fetch has filled, and no answer says what they hold, a count of none or of many; `never_fetched_by_type` is the same by tab type, a character's `character`, each counted as the `tabs` tool counts a type, and `tabs` lists a league's tabs, `never_fetched_leagues` naming the leagues that hold a never-fetched tab, the most first), `total`, `rows` — id, name, base, rarity, place by name and id — a tab's page with no name of its own has `name` null and its tab in `parent` — with the cell the item sits in (`place.cell`, GGG's x and y, in a tab or an inventory) and when that place was last fetched (`place.fetched_at`), and the lines each term touched with their values, the term named by its `path` in `terms`, never the item's body, which is `show` — then `view`, and last `terms` (for each term how many items matched, failed, lacked the thing or were undecided — each term alone over every item in scope, never over the matches, which are `total`; a pattern term's `resolved` is what it matched there, its `rest.denominator` `scope`) and `basis` (what the answer was computed from). Rows past the limit are counted in `view.rows.left_out`, and `view.rows.next` is the request for the rows after: send it back whole. A count holds the request that returns exactly its members where a next step hangs on it — what is undecided, what holds only together, what a key could not place (`none`, `undecided`), what an ambiguous trade clause's other reading adds or drops, and each term's matched where nothing matched — and every other count is its number alone: `view.routes: true` asks for every count's. A zero total offers requests under `zero.offers`: any request an answer holds is sent back as printed. A count-first look is {\"view\": {\"counts\": {\"keys\": [\"tab\", \"rarity\"]}}}; the vocabulary of lines is the key `line`, or `line:<text>` narrowed. A key is one entry of `keys`: a terminal's list `line:resist,life` is [\"line:resist\", \"line:life\"], and a comma inside an entry is part of its text. A time is seconds since 1970, UTC. `describe` and the messages name a terminal's flags: --count, --cross, --sum, --sort, --desc, --limit, --explain and --routes are `view`'s fields, --context is `context`, --realm and --membership are `scope`'s, and --next is `view.rows.next` sent back. A refusal is a result marked isError: `error`, `kind`, `readings` (what may have been meant, as a terminal would type it: a query text, a name, a flag or a whole `acq` command) and, where a way on is a search, `offers` (the same as requests to send instead)."
    )]
    fn search(&self, Parameters(RequestParams(args)): Parameters<RequestParams>) -> Told {
        let asked = match Asked::read(&Value::Object(args)) {
            Ok(asked) => asked,
            Err(e) => return Ok(refused(e)),
        };
        // an authoring error is said before the store is read
        if let Err(e) = asked.request.check() {
            return Ok(refused(e));
        }
        let (store, intent) =
            match open_search(asked.request.scope.account.as_deref(), "scope.account") {
                Ok(opened) => opened,
                Err(refusal) => return Ok(refusal),
            };
        match asked
            .request
            .corpus(&store, &intent)
            .and_then(|corpus| asked.answer(&corpus))
        {
            Ok(answer) => told(&answer),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "The query language `search` takes, as this build knows it (no store read): fields, what a line has, operators, closed value sets, slots, the computed values with their definitions, what a count takes, what a trade site search's clauses become, what is not built and the limits stated — or only the entries named. The whole is over 100 KB, most of it under `computed`: name what is needed instead. The sections are fields (some 20 KB), line, linked, values, composition, operators, slots, counts and explain (each under 4 KB), trade (some 20 KB) and computed; a single name — league, total_res — returns its own entry. A name it does not know is refused, and the refusal lists every name it knows."
    )]
    fn describe(&self, Parameters(p): Parameters<Args<DescribeParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        match acquisition_search::describe(&p.names.unwrap_or_default()) {
            Ok(described) => told(&described),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "One item as the search derives it, live or removed (no daemon, no network): its place, its fields, class, category and price, and every line with its kind, template and numbers. `against` adds why the item does or does not match a query: the root's outcome and each node's — a term's `matched`, `failed`, `lacked` or `undecided`, a group's `match`, `no_match` or `undecided`. `body` adds the stored body, verbatim. An id that is a tab's or a character's is refused and offers the search that lists it. A refusal is a result marked isError, as `search`'s is."
    )]
    fn show(&self, Parameters(p): Parameters<Args<ShowParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let against = match p.against.map(serde_json::from_value::<QueryInput>) {
            None => None,
            Some(Ok(query)) => Some(query),
            Some(Err(e)) => {
                return Ok(CallToolResult::structured_error(json!({
                    "error": format!(
                        "`against` is a query as a request holds one, {{\"text\": …}} or {{\"tree\": …}}: {e}"
                    ),
                    "kind": "tree",
                })));
            }
        };
        // an authoring error is said before the store is read
        if let Some(Err(e)) = against.as_ref().map(|q| q.read(None)) {
            return Ok(refused(e));
        }
        let (store, intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let body = p.body.unwrap_or(false);
        match match &against {
            Some(query) => acquisition_search::show::against(&store, &intent, &p.id, body, query),
            None => acquisition_search::show(&store, &intent, &p.id, body),
        } {
            Ok(shown) => told(&shown),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "Store file path, size, and row counts (no daemon, no network). `leagues` counts the league listing's rows and `league_list` says whether that listing was ever fetched (`acq leagues` or a `leagues` job fetches it); characters and tabs carry their league names whether or not it was. `refused_bodies` counts bodies the store would not take, which `acq store refused` lists."
    )]
    fn store_status(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| open_store(p.parse()?.account.as_deref())?.status())
    }

    #[tool(
        description = "Item events (added/moved/changed/removed) from recent ingests, newest first (no daemon, no network); `hours` how far back, `limit` how many of the newest."
    )]
    fn item_events(&self, Parameters(p): Parameters<Args<EventsParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let store = open_store(p.account.as_deref())?;
            let since = acquisition_store::now() - (p.hours.unwrap_or(24.0) * 3600.0) as i64;
            store.events_since(since, p.limit.unwrap_or(200))
        })
    }

    // ---- the intent/plan slice (tracer step 8): policy → plan → apply ----

    #[tool(
        description = "The per-account sync policy (declared coverage + freshness, an annotation) with its revision — the intent refresh_plan compiles. Reads the store; no daemon, no network. `policy` is null when none is set."
    )]
    fn sync_policy(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| {
            let (_, _, annotations) = open_intent(p.parse()?.account.as_deref())?;
            let row = annotations.get(SYNC_POLICY_SCOPE, SYNC_POLICY_KEY, SYNC_POLICY_KIND)?;
            Ok(json!({ "policy": row }))
        })
    }

    #[tool(
        description = "Write the per-account sync policy (validated strictly, compare-and-swap on if_revision). Sends nothing — intent is local; the network cost appears only when a plan is applied. Each realm the value names replaces that realm alone; the others stay, a realm named with no leagues is removed, and `replace: true` writes the whole policy instead. Replacing an existing policy requires if_revision (from sync_policy) either way; a stale or missing revision is a conflict, never a clobber. Returns the stored row, the whole policy as it now stands."
    )]
    fn set_sync_policy(&self, Parameters(p): Parameters<Args<SetPolicyParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let (_, _, mut annotations) = open_intent(p.account.as_deref())?;
            let write = if p.replace.unwrap_or(false) {
                PolicyWrite::Replace
            } else {
                PolicyWrite::Merge
            };
            Ok(put_sync_policy(
                &mut annotations,
                &p.value,
                p.if_revision,
                write,
                &Provenance::via(WRITTEN_VIA),
            )?)
        })
    }

    #[tool(
        description = "Compile the stored sync policy + facts on record into a RefreshPlan for one (realm, league): the explicit, bounded action set applying would execute — stash listing and tab fetches, the realm's character listing and character fetches, each facet as the policy covers it — with per-action reasons, skipped tabs and characters, unknown ids, and a coarse wire estimate. Offline — sends nothing, spends nothing. A running daemon adds its read-only quote (ETA + rate-limit headroom; never spawned for this); `quote_note` says why one is absent. Review the plan, then hand `plan` to apply_plan."
    )]
    async fn refresh_plan(&self, Parameters(p): Parameters<Args<PlanParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let realm = realm_param(p.realm.as_deref())?;
                let (dir, entry, annotations) = open_intent(p.account.as_deref())?;
                let store = Store::open(&account_path(&dir, &entry.username)).map_err(unopened)?;
                let snapshot = store.refresh_snapshot(
                    realm.as_str(),
                    p.league.as_deref().unwrap_or("Standard"),
                    &annotations,
                )?;
                let plan = match plan_refresh(provider(), &snapshot, acquisition_store::now()) {
                    Err(PlanError::NoSyncPolicy) => anyhow::bail!(
                        "no sync policy is set for {} — declare one with set_sync_policy first",
                        entry.username
                    ),
                    other => other?,
                };
                let (plan, quote_note) = try_quote(plan).await;
                let mut out = json!({ "plan": plan });
                if let Some(note) = quote_note {
                    out["quote_note"] = json!(note);
                }
                Ok(out)
            }
            .await,
        )
    }

    #[tool(
        description = "Execute a reviewed plan: its actions and nothing else, as one `apply` parent job the daemon admits or refuses whole at submit (single-request vocabulary + the max_requests budget). Refused offline — before any daemon contact — if the stored sync-policy revision moved since the plan (replan with refresh_plan). In real-GGG mode the daemon must already be running (a human starts it; this server never spawns one there). An empty plan is a no-op with no daemon contact. A plan spends each request once: an action whose target was fetched after the plan was compiled, or whose request the daemon already holds, is listed in `spent` (with `fetched_at`, or the `job_id` holding it) and not sent again, so replaying a plan answers `applied: false` and sends nothing. Returns the parent job id and the `requests` sent; poll job_status, then job_result."
    )]
    async fn apply_plan(&self, Parameters(p): Parameters<Args<ApplyParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let (dir, entry, annotations) = open_intent(p.account.as_deref())?;
                let plan = RefreshPlan::from_value(&p.plan)?;
                plan.check_spendable(provider(), entry.uuid.as_deref(), &annotations)?;
                if plan.actions.is_empty() {
                    // A strict subset of zero actions is satisfied by doing
                    // nothing; the plan's own skipped/unknown reporting says why
                    // it is empty.
                    return Ok(json!({
                        "applied": false,
                        "requests": 0,
                        "note": "nothing to do: the plan authorizes no requests",
                    }));
                }
                // C111: a plan spends each request once — the store says what was
                // fetched after the plan's snapshot, the daemon what it already
                // holds; only the rest is submitted. The facts are read again
                // after the list, so a fetch that finishes between the two reads
                // is in one of them.
                let store =
                    Store::open(&account_path(&dir, &entry.username)).map_err(unopened)?;
                let facts =
                    store.refresh_snapshot(plan.realm.as_str(), &plan.league, &annotations)?;
                let (mut unspent, mut spent) = plan.unspent(&facts, &[], &entry.username);
                let mut connected = None;
                if !unspent.is_empty() {
                    let mut client = connect(true).await?;
                    if let Response::Jobs { jobs } = ask(&mut client, &Request::List).await? {
                        let facts = store.refresh_snapshot(
                            plan.realm.as_str(),
                            &plan.league,
                            &annotations,
                        )?;
                        (unspent, spent) = plan.unspent(&facts, &jobs, &entry.username);
                    }
                    connected = Some(client);
                }
                let Some(mut client) = connected.filter(|_| !unspent.is_empty()) else {
                    return Ok(json!({
                        "applied": false,
                        "requests": 0,
                        "note": "nothing to do: every request of this plan is spent — fetched after it was compiled, or already waiting or running (each entry of `spent` says which)",
                        "spent": spent,
                    }));
                };
                let submit = Request::Submit {
                    kind: "apply".into(),
                    params: apply_params_of(&unspent, p.max_requests),
                    priority: 0,
                    submitted_by: format!("mcp:{}", std::process::id()),
                    account: Some(entry.username),
                };
                match ask(&mut client, &submit).await? {
                    Response::Submitted { id } => Ok(json!({
                        "job_id": id,
                        "requests": unspent.len(),
                        "spent": spent,
                    })),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    // ---- pricing: adapters of acquisition-plan's pricing verbs ----

    #[tool(
        description = "The listing state of one league, counted (no daemon, no network): what every item, tab and character is priced as — by hand (a row this server or the CLI wrote), in game (an item's note, then its tab's name, read only where the tab is public), how the two stand — as the counts and the row accounting, without the listings themselves. `counts.by_effective` says who decides each item's price (game, manual, none, unresolved — a row that cannot be read could decide), `counts.by_relation` how the two sides stand (none, manual_only, game_only, agree, conflict); `rows` accounts for every hand-written row: how many apply here, which name nothing in the facts (`unmatched`, each row whole — `value`, `from`, `revision`, `updated_at`, `written_via` — since no listing carries it), which cannot be read, which belong to other leagues or realms. The document `acq price status --json` prints. price_list names the items; price_show is one target."
    )]
    fn price_status(&self, Parameters(p): Parameters<Args<PriceStatusParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm.unwrap_or(Realm::DEFAULT),
            Err(refusal) => return Ok(refusal),
        };
        let (store, intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let league = p
            .league
            .as_deref()
            .unwrap_or(acquisition_plan::pricing::DEFAULT_LEAGUE);
        priced(
            acquisition_plan::pricing::report(&store, &intent, realm, league).map(|r| r.summary()),
        )
    }

    #[tool(
        description = "The items of one league that are priced, each with both sides of its listing (no daemon, no network): `items`, each with its `subject` (an item address in `subject.target`), the hand row that applies (`manual`: the value, the row's own target in `from`, `inherited`, its `revision`), the game's reading (`game`: the note and the tab name verbatim beside what each reads as, the tab and whether it is public), the `relation` and the `effective` price; `containers`, the tabs, substashes and characters those items sit under, each with its own listing; `items_on_record`, how many items the league holds, so an empty answer says which nothing; `total`, how many the selection holds, and `listed`, how many `items` lists — fewer only under `limit`, the first in listing order, `containers` then being theirs alone. With neither `relation` nor `effective`, items nothing prices are left out, except unresolved ones, which are always listed. A container the facts do not hold is refused (`kind: \"not_found\"`), never an empty selection. The document `acq price list --json` prints; a refusal is a result marked isError holding `error`, whose words name the CLI's flags (--league is `league`, --in is `in`)."
    )]
    fn price_list(&self, Parameters(p): Parameters<Args<PriceListParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm,
            Err(refusal) => return Ok(refusal),
        };
        let (store, intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::list(
            &store,
            &intent,
            realm,
            p.league.as_deref(),
            &acquisition_plan::pricing::ListArgs {
                relation: p.relation,
                effective: p.effective,
                location: p.location,
                covered_by: p.covered_by,
                limit: p.limit,
            },
        ))
    }

    #[tool(
        description = "One target's listing (no daemon, no network): `listing`, both sides with their causes — the hand row that applies with its `revision` and the target it is on (`manual.from`; the target's own row when that is the target itself, a container's when `inherited`), the note and the tab name verbatim beside what each reads as — the relation, the effective price and why; `container`, the one an item sits in; for a tab, substash or character, `items_here` (physically in it) and `items_covered_below` (covered through its children). A bare id is refused with its two readings, `item/<id>` and `character/<id>`; a target the facts do not hold is refused (`kind: \"not_found\"`) naming the league that holds it, if any, and saying so when it has a row of its own, which price_status lists whole. The document `acq price show --json` prints. Read it before price_set or price_clear on a target that has a row of its own: they name the revision they replace."
    )]
    fn price_show(&self, Parameters(p): Parameters<Args<PriceShowParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm,
            Err(refusal) => return Ok(refusal),
        };
        let (store, intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::show(
            &store,
            &intent,
            &p.target,
            realm,
            p.league.as_deref(),
        ))
    }

    #[tool(
        description = "The forum shop pages for one league, rendered for a person to paste by hand; this server posts nothing and reads no forum (no daemon, no network). One link code per hand-priced item under its price's spoiler, in pages of at most `size` characters, each page labelled n of N: `pages` (each `text` is what to paste), `posted` (the items on them), `left_out` (the first 50 items left off for a reason, the blocked first, each with its `cell` — what the game already lists or a skip is omitted, a case the forum has not been observed for is blocked — and `left_out_total` counting them all; `expand: true` lists every one, the items nothing applies to too, as `off_page_listed` says), `counts` (every item by its cell; the items nothing applies to in `off_page`), `policy` (each cell's rule and count) and `freshness`: whether the sync policy covers the posted items and which of them are older than its window, with the refresh that would renew them — reported, never enforced. `page` returns one page's record alone. The document `acq shop render --json` prints."
    )]
    fn shop_render(&self, Parameters(p): Parameters<Args<ShopRenderParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let realm = match price_realm(p.realm.as_deref()) {
            Ok(realm) => realm.unwrap_or(Realm::DEFAULT),
            Err(refusal) => return Ok(refusal),
        };
        let (store, intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let rendered = acquisition_plan::pricing::shop_render(
            &store,
            &intent,
            provider(),
            realm,
            p.league
                .as_deref()
                .unwrap_or(acquisition_plan::pricing::DEFAULT_LEAGUE),
            &acquisition_plan::pricing::ShopArgs {
                size: p.size.unwrap_or(acquisition_plan::shop::DEFAULT_PAGE_SIZE),
                template: p.template.as_deref(),
                now: acquisition_store::now(),
                expand: p.expand.unwrap_or(false),
            },
        );
        match (rendered, p.page) {
            (Ok(rendered), Some(n)) => {
                priced(acquisition_plan::pricing::shop_page(&rendered, n).cloned())
            }
            (rendered, _) => priced(rendered),
        }
    }

    #[tool(
        description = "The currency table this build ships (no store, no daemon, no network): every row — `tag`, the word a price cites; `emit`, the word the game writes in a note; `aliases`, legacy words that resolve to it; `display`; `retired` where the game no longer writes it; `evidence` — with the table's version and sources. `word` resolves one word instead, exact and case-sensitive: `{word, version, currency}`, or a refusal naming the word and the version. The document `acq reference currency --json` prints."
    )]
    fn reference_currency(&self, Parameters(p): Parameters<Args<ReferenceCurrencyParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::currency_document(
            p.word.as_deref(),
        ))
    }

    #[tool(
        description = "Set one target's price by hand: one row on an item, tab, substash or character, which what it covers inherits (a tab's items, a folder's tabs, a parent tab's substashes, a character's items) unless a row nearer the item says otherwise. Intent only: it sends nothing and changes nothing in game. Stricter than `acq price set`: a target that already has a row of its own is replaced only when `if_revision` names that row's revision, and without it the write is refused naming the current revision — `kind: \"conflict\"`, the revision in `revision`, as for a stale one — read the target with price_show first; with no row, omit `if_revision` to create one. A clear takes a revision too, so the next write's revision continues from it. Returns the receipt: `target`, `written` (the row now, with its revision), `prior` (the row replaced, or null after a create) — the undo after a create is price_clear with `if_revision` the written revision, and after a replace price_set with the prior's value and `if_revision` the written revision, unless the prior is a value this build cannot read (a newer build's, replaced only by naming its revision), which it cannot write back — and `in_facts`, false for a target the facts do not hold (not fetched yet, or a typo: the row stands). The document `acq price set --json` prints."
    )]
    fn price_set(&self, Parameters(p): Parameters<Args<PriceSetParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let (store, mut intent) = match open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::set(
            &store,
            &mut intent,
            &p.target,
            &p.r#type,
            p.amount.as_deref(),
            p.currency.as_deref(),
            acquisition_plan::pricing::Expect::or(
                p.if_revision,
                acquisition_plan::pricing::Expect::Create,
            ),
            &Provenance::via(WRITTEN_VIA),
        ))
    }

    #[tool(
        description = "Remove one target's own price row; what it covered falls back to the next row up, or to the game's price. Intent only: it sends nothing. Stricter than `acq price clear`: `if_revision` must name the revision of the row it removes, and without it the clear is refused naming the current revision (or saying the target has no row of its own) — `kind: \"conflict\"`, the revision in `revision` — read the target with price_show first. A clear takes a revision too, so the next write's revision continues from it. Returns the receipt: `target`, `written` null, `prior` (the row removed); price_set with the prior's value puts it back. The document `acq price clear --json` prints."
    )]
    fn price_clear(&self, Parameters(p): Parameters<Args<PriceClearParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let (_, _, mut intent) = match open_account_intent(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::clear(
            &mut intent,
            &p.target,
            acquisition_plan::pricing::Expect::or(
                p.if_revision,
                acquisition_plan::pricing::Expect::Create,
            ),
            &Provenance::via(WRITTEN_VIA),
        ))
    }

    // ---- daemon jobs ----

    #[tool(
        description = "Submit a job to the daemon and return its id immediately; poll job_status then fetch job_result. In real-GGG mode the daemon must already be running (a human starts it; this server never spawns one there)."
    )]
    async fn submit_job(&self, Parameters(p): Parameters<SubmitParams>) -> Told {
        settled(
            async {
                let mut client = connect(true).await?;
                let submit = Request::Submit {
                    kind: p.kind,
                    params: p.params.unwrap_or_else(|| json!({})),
                    priority: p.priority.unwrap_or(0),
                    submitted_by: format!("mcp:{}", std::process::id()),
                    account: p.account,
                };
                match ask(&mut client, &submit).await? {
                    Response::Submitted { id } => Ok(json!({ "job_id": id })),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    #[tool(
        description = "Jobs the daemon knows about this lifetime, with states and ETAs. Observes only: with no daemon running it never spawns one and answers from the ledger on disk instead — running: false, persisted: the non-terminal rows as the last daemon left them (jobs, without ETAs) and written_at, how old that picture is; persisted is null when this shell's world has no queue yet. A job command from the CLI spawns a successor that restores them."
    )]
    async fn list_jobs(&self) -> Told {
        settled(
            async {
                let mut client = match observe().await? {
                    Observed::Absent => {
                        let persisted = persisted_queue()?;
                        return Ok(json!({ "running": false, "persisted": persisted }));
                    }
                    other => client_of(other)?,
                };
                match ask(&mut client, &Request::List).await? {
                    Response::Jobs { jobs } => Ok(serde_json::to_value(jobs)?),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    #[tool(
        description = "One job's state, ETA, and retries. A waiting job with a large ETA is the rate limiter holding, not a hang (holds can reach ~5 minutes)."
    )]
    async fn job_status(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(
            async {
                let mut client = attach().await?;
                match ask(&mut client, &Request::Status { id: p.id }).await? {
                    Response::Status { job } => Ok(job),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    #[tool(
        description = "A finished job's payload or error; answered across daemon restarts (the queue persists). Payloads can be large — prefer the store tools for reading data."
    )]
    async fn job_result(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(
            async {
                let mut client = attach().await?;
                match ask(&mut client, &Request::Result { id: p.id }).await? {
                    Response::Result { outcome, .. } => Ok(outcome),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    #[tool(description = "Cancel a waiting or running job; cascades to its descendants.")]
    async fn cancel_job(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(
            async {
                let mut client = attach().await?;
                match ask(&mut client, &Request::Cancel { id: p.id }).await? {
                    Response::Ack => Ok(json!({ "job_id": p.id, "cancel_requested": true })),
                    other => Err(unexpected(other)),
                }
            }
            .await,
        )
    }

    #[tool(
        description = "The daemon running: its identity — contract revision, the executable it runs from and its hash (artifact), the world it serves (its canonical store root), the socket it was reached on (derived from the world), how that file relates to the acqd beside this server (artifact_relation: same_file, same_bytes for another copy, different, unhashable, no_sibling, unreported), which of the four dimensions match (contract_matches, artifact_matches, provider_matches, world_matches), and under wanted this server's own version, contract, provider and world with that sibling — all from the one look that judged the daemon; and, when compatible, its vitals: provider, uptime, idle_shutdown_seconds (the idle exit it was started with), connections, queue counts, rate-limit policies learned, rails state (with the journal path), keyring health, and log — the log file the daemon opened, as it reports it. Observes only: running=false when no daemon is up, with persisted — what the ledger on disk holds: waiting, recorded_running, written_at; null when this shell's world has no queue yet; running=true, compatible=false for a daemon of another contract, artifact, provider or world, reported by its identity alone and never replaced by this server; running=true, responsive=false (with socket and lock_holder, the pid the world's lock records) for something that accepted the connection and did not answer hello within 5s — a wedged daemon, for a human to stop by hand."
    )]
    async fn daemon_status(&self) -> Told {
        settled(
            async {
                let mut client = match observe().await? {
                    Observed::Compatible(c) => c,
                    Observed::Absent => {
                        let persisted = persisted_queue()?.map(|p| {
                            json!({
                                "waiting": p.waiting(),
                                "recorded_running": p.recorded_running(),
                                "written_at": p.written_at,
                            })
                        });
                        return Ok(json!({ "running": false, "persisted": persisted }));
                    }
                    Observed::Incompatible(found) => {
                        let mut report = found.report();
                        report["running"] = json!(true);
                        report["compatible"] = json!(false);
                        return Ok(report);
                    }
                    Observed::Unresponsive(found) => {
                        return Ok(json!({
                            "running": true,
                            "responsive": false,
                            "socket": found.at.socket,
                            "lock_holder": found.holder,
                        }));
                    }
                };
                let found = client.daemon().clone();
                let resp = ask(&mut client, &Request::DaemonStatus).await?;
                let mut report = serde_json::to_value(resp)?;
                report["running"] = json!(true);
                report["compatible"] = json!(true);
                // The identity keys the incompatible report has (C84): the
                // daemon's contract and artifact, the artifact's relation to this
                // server's sibling, and the sibling under `wanted`.
                let identity = found.report();
                for key in [
                    "contract",
                    "artifact",
                    "world",
                    "socket",
                    "contract_matches",
                    "artifact_matches",
                    "artifact_relation",
                    "provider_matches",
                    "world_matches",
                    "wanted",
                ] {
                    report[key] = identity[key].clone();
                }
                Ok(report)
            }
            .await,
        )
    }
}

#[tool_handler]
impl ServerHandler for AcqMcp {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(format!(
            "Acquisition (provider: {}). Items: search answers a query over \
             every item the store holds — the total, then rows or tables, then counts \
             for each term — describe prints its query language, and show is one item whole. \
             Their arguments and answers are JSON; a count a next step hangs on — what is \
             undecided, what a key could not place — holds the request that returns its \
             members, sent back as printed, view.routes asks every count's, and a refusal is a result marked isError with readings and offers to go on \
             from. A call nested deeper than {} JSON levels is not read: it is refused \
             as arguments. Begin with search with no arguments, for what the store holds, or with \
             scope.realm named where the store holds several realms (a PoE2 character beside \
             the PoE1 stashes), since a search over several realms is refused and its offers \
             are the request in each realm: send one back. Then describe with names \
             [\"fields\"], for what a query may ask: describe whole is over 100 KB. A \
             trade site search — its URL, id or query JSON — is the query \
             {{\"trade\": …}}, translated to the text a person would type. No \
             tool says what an item is worth: a price is the owner's own listing. \
             Pricing: price_status counts a league's listing state, price_list names \
             the priced items and price_show is one target, each with its price by hand \
             and in game; price_set and price_clear write one hand row (intent only, \
             sending nothing), and over a row the target already has they name the \
             revision they replace; shop_render renders the forum pages for a person \
             to paste, posting nothing; reference_currency is the currency table a \
             price cites. The store tools (search, show, accounts, tabs, characters, \
             store_status, item_events) and the pricing tools read the shared SQLite \
             store directly — no daemon, no network. \
             The refresh slice: sync_policy / \
             set_sync_policy declare intent (local, sends nothing; replacing a policy \
             must name the revision it replaces), refresh_plan compiles it offline into \
             the explicit bounded action set for review, and apply_plan spends exactly \
             that plan as one parent job. Job tools talk to the local daemon; API \
             requests are jobs (submit_job returns an id; poll job_status, then \
             job_result). Rate-limit holds can reach ~5 minutes — a waiting job is the \
             limiter working. Login is done by the human via the `acq` CLI; this server \
             never spawns or replaces a daemon in real-GGG mode — there it submits, applies \
             and quotes only through one a human already started.",
            provider(),
            transport::MAX_DEPTH,
        ));
        info
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // C88: refused before serving — the host shows the line.
    if let Err(e) = acquisition_protocol::provider::check_environment() {
        eprintln!("acq-mcp: {e}");
        std::process::exit(2);
    }
    let (stdio, writer) = transport::stdio();
    // The writer is awaited however the service ends, so a reply queued
    // before it failed (a line answered before initialisation, then
    // stdin closed) still reaches stdout.
    let served: Result<()> = async {
        AcqMcp.serve(stdio).await?.waiting().await?;
        Ok(())
    }
    .await;
    let _ = writer.await;
    served
}
