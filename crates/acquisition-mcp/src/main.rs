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
//!   sorted by one function (`failed`) over the shared layer's one fault
//!   type (`acquisition_verbs::Fault`, C46): its kind and fields, the
//!   remedy in this adapter's words, `arguments`, and `refused` for the
//!   rest (the release seat's rerun); a JSON-RPC error is kept for a call
//!   the server could not serve — a daemon's own refusal with its closed
//!   kind (C85), a door that did not open, another or an unresponsive
//!   daemon, the transport.
//! - **The corpus is held across calls** (C98, the consumer holds the
//!   corpus and checks before every answer; #82): `search` opens the
//!   account's store and intent per call, as before, and asks the search
//!   crate's `Holder` — the one `acq web` holds — for the corpus, which
//!   is reused while its scope is the request's and `is_current` says the
//!   basis still holds. The first call after a refresh, a price or policy
//!   write, or a call of another account, realm or membership pays the
//!   load; every other pays the evaluation alone. `show` reads one item
//!   and holds nothing.
//!
//! **Every text the server serves speaks its own words** (C143, the
//! shared words first and an adapter's spelling after them; the readiness
//! seat, `audits/readiness-2026-10/seat/`, calls 5, 6, 12, 13, 35, 47):
//! the shared crates' words are the CLI's, and wherever a refusal,
//! `describe` or an answer's `why` or `hint` speaks a CLI command or flag,
//! `spoken` appends `— here,` and what each is here (`acq search` is
//! `search`, `--realm` is `realm`), from one table, `SPOKEN`. A person's
//! command (`acq auth check`) is left as the person's; an entry's name,
//! how it is asked for, keeps the CLI's spelling and its `what` carries
//! the spelling; every other string of an answer is data, sent verbatim.
//! The rewrite is on the serialized text, so the document's key order
//! stands (C115). Pinned by `tests/spoken.rs`.
//!
//! An account that cannot be picked is refused in the protocol's words
//! (C121): `not_logged_in` when no account's store is found (naming the
//! directory looked in) or the selector names none, `ambiguous_account`
//! when it names several or none among several, each with `accounts`,
//! the names known or to pick from. Arguments a tool does not take are
//! this adapter's own refusal (`arguments`).
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
//! document, `{"error": …}` in the CLI's words (C11), this server's
//! spelling after them. Pricing reads
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

use acquisition_client::client::{Client, ConnectError, DaemonError, NotSpawned};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshPlan};
use acquisition_protocol::protocol::{ErrorKind, Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Asked, QueryInput};
use acquisition_search::{Holder, SearchError};
use acquisition_store::{Annotations, Provenance, Store};
use acquisition_verbs::fault::door_word;
use acquisition_verbs::receipt;
use acquisition_verbs::refusal::{self, Arguments, Way};
use acquisition_verbs::{Account, Fault, Frontend};
use anyhow::Result;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router};
use serde::Deserialize;
use serde_json::{Value, json};

/// A call the server could not serve, as a JSON-RPC error with the full
/// context chain.
fn err(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(format!("{e:#}"), None)
}

/// A layer fault in this adapter's words (C11, C116): the cause the fault
/// says, and the remedy in the MCP's own verbs — the argument that
/// selects an account (`named`: `account`, `scope.account` on search),
/// `set_sync_policy`, `refresh_plan`, the person's command for a daemon.
/// One arm per fault, so a cause the layer adds cannot reach an agent
/// without its remedy.
fn fault_words(fault: &Fault, named: &str) -> String {
    match fault {
        Fault::Environment(_)
        | Fault::Unopened(_)
        | Fault::Plan(_)
        | Fault::Policy(_)
        | Fault::Door(_)
        | Fault::Unresponsive(_)
        | Fault::Daemon(_)
        | Fault::Transport(_)
        | Fault::Other(_)
        | Fault::Spend(_) => fault.to_string(),
        // No login for the caller to run (the agent's seat, F10: a
        // server pointed at the wrong root answered every call with
        // `acq auth`, the one act an agent cannot perform).
        Fault::NoAccounts { .. } => format!(
            "{fault}; ACQ_STORE_DIR sets the root this server looks under, and a store is made \
             by a login, which is a human's, from a terminal"
        ),
        Fault::UnknownAccount { .. } => format!(
            "{fault}: `{named}` takes a username, with or without its #discriminator, or a uuid"
        ),
        Fault::AmbiguousAccount { .. } => format!("{fault}; `{named}` selects one"),
        Fault::NoUuid { .. } => format!("{fault}; a human can fix it with one `acq auth` login"),
        Fault::NoSyncPolicy { .. } => {
            format!("{fault} — declare one with set_sync_policy first")
        }
        // A merge reads the stored row to merge into, so the retry is a
        // whole write naming the revision.
        Fault::UnreadablePolicy { revision, .. } => format!(
            "{fault} — `replace: true` with `if_revision: {revision}` writes a whole policy over \
             it deliberately; sync_policy reads it"
        ),
        // Absent by this server's own policy in real mode: the remedy is
        // the human's, and the words say so rather than promising a spawn.
        Fault::NoDaemon {
            because: Some(NotSpawned::Policy),
            next,
        } if acquisition_protocol::provider::ggg_mode() => format!(
            "no daemon running in real-GGG mode, and this server never starts one there — \
             {next}: {fault}"
        ),
        Fault::NoDaemon {
            because: Some(_),
            next,
        } => format!("no daemon running — {next}: {fault}"),
        Fault::NoDaemon {
            because: None,
            next,
        } => format!("no daemon running — {next}"),
        Fault::NoJobOnDisk { next, .. } => format!("{fault} — {next}"),
        Fault::Unfinished { next, .. } => {
            format!("{fault}: a daemon resumes it — {next}")
        }
        Fault::AnotherDaemon(found) => format!(
            "{found}; this server never replaces a daemon — resolve it with the CLI (`acq daemon stop`{})",
            if found.world_matches() {
                ""
            } else {
                ", or point ACQ_STORE_DIR at its world"
            }
        ),
    }
}

/// The failure's words: its chain as anyhow prints it, a layer fault in
/// this adapter's words where the chain holds one.
fn failure_words(e: &anyhow::Error, named: &str) -> String {
    let mut parts = Vec::new();
    for link in e.chain() {
        if let Some(fault) = link.downcast_ref::<Fault>() {
            parts.push(fault_words(fault, named));
            return parts.join(": ");
        }
        parts.push(link.to_string());
    }
    format!("{e:#}")
}

/// A tool's failure, sorted by the one rule every tool shares, so no
/// tool decides it alone (one match over the layer's fault). A call the
/// server could not serve is a JSON-RPC error: a daemon's own refusal
/// with its closed kind ([`daemon_error`], C85), a door that did not
/// open ([`connect_error`]), another or an unresponsive daemon, the
/// conversation with the daemon failing. Everything else is a refusal —
/// the request was wrong, or the store or policy cannot serve it, or no
/// daemon runs — a result marked isError holding the document
/// [`refusal()`] words. The release seat's rerun met an unknown account
/// and an uncovered league as JSON-RPC errors where the instructions
/// promise results (mcp-opus-2 calls 50, 46).
fn failed(e: anyhow::Error) -> Told {
    let words = spelled(&failure_words(&e, "account"));
    match e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        Some(Fault::Daemon(refused)) => Err(daemon_error(refused.kind, words)),
        Some(Fault::Door(door)) => Err(connect_error(door, words)),
        Some(Fault::AnotherDaemon(_) | Fault::Unresponsive(_) | Fault::Transport(_)) => {
            Err(ErrorData::internal_error(words, None))
        }
        _ => Ok(refusing(refusal(&e, "account"))),
    }
}

/// A refusal's document (C11, C121; round 4 of the 2g build, the one
/// refusal shape): the verbs' one sorter
/// (`acquisition_verbs::refusal::document`), its words this server's,
/// `named` the argument that selects an account, and its offers spelled
/// as this server's requests ([`Spoken`]).
fn refusal(e: &anyhow::Error, named: &str) -> Value {
    refusal::document(e, &Spoken { named })
}

tokio::task_local! {
    /// The call being served, as sent: what a refusal's offer to send it
    /// again as another account is made from ([`Spoken`]). Set around
    /// every call by `call_tool`, read by nothing else.
    static CALLED: Called;
}

/// A tool's name and its arguments, as the call sent them.
#[derive(Clone)]
struct Called {
    tool: String,
    arguments: serde_json::Map<String, Value>,
}

/// This server's side of the one refusal shape: its words
/// ([`failure_words`]) and its requests — an offer is the arguments a
/// tool takes, naming the tool where it is another than the one refused;
/// the same call again is its arguments as sent with the account alone
/// changed (`scope.account` for `search`, inside the object that holds
/// the request where it came in one).
struct Spoken<'a> {
    named: &'a str,
}

impl refusal::Surface for Spoken<'_> {
    fn words(&self, e: &anyhow::Error) -> String {
        failure_words(e, self.named)
    }

    fn repeats_from_its_start(&self) -> bool {
        CALLED
            .try_with(|c| {
                c.tool == "search"
                    && refusal::again(&Value::Object(c.arguments.clone()), "")
                        .is_some_and(|(_, restarted)| restarted)
            })
            .unwrap_or(false)
    }

    fn request(&self, way: &Way) -> Option<(Value, Option<&'static str>)> {
        let called = CALLED.try_with(Called::clone).ok();
        let tool = called.as_ref().map(|c| c.tool.clone());
        let other = |name: &'static str| (tool.as_deref() != Some(name)).then_some(name);
        match way {
            // the arguments as sent, only the account changed: for
            // `search` the verbs' one rule over a request read whole
            // (`refusal::again`), a continuation from its start
            Way::Again { account } => {
                let mut arguments = called?.arguments;
                if tool.as_deref() == Some("search") {
                    let (envelope, _) = refusal::again(&Value::Object(arguments), account)?;
                    return Some((envelope, None));
                }
                arguments.insert("account".into(), json!(account));
                Some((Value::Object(arguments), None))
            }
            Way::Search(request) => Some((serde_json::to_value(request).ok()?, other("search"))),
            Way::Tabs {
                account,
                realm,
                league,
            } => Some((
                json!({ "account": account, "realm": realm.as_str(), "league": league }),
                other("tabs"),
            )),
            Way::PriceShow {
                target,
                account,
                realm,
                league,
            } => Some((
                json!({
                    "target": target, "account": account, "realm": realm.as_str(), "league": league,
                }),
                other("price_show"),
            )),
            Way::PriceStatus {
                account,
                realm,
                league,
            } => Some((
                json!({
                    "account": account, "realm": realm.as_str(), "league": league, "expand": true,
                }),
                other("price_status"),
            )),
            // a receipt's undo: the write tool's own arguments, the
            // amount and currency only where the type carries a price,
            // the revision only where it names one (a create names none)
            Way::PriceSet {
                target,
                account,
                kind,
                amount,
                currency,
                if_revision,
            } => {
                let mut arguments = serde_json::Map::new();
                arguments.insert("target".into(), json!(target));
                arguments.insert("type".into(), json!(kind));
                if let Some(amount) = amount {
                    arguments.insert("amount".into(), json!(amount));
                }
                if let Some(currency) = currency {
                    arguments.insert("currency".into(), json!(currency));
                }
                if let Some(revision) = if_revision {
                    arguments.insert("if_revision".into(), json!(revision));
                }
                arguments.insert("account".into(), json!(account));
                Some((Value::Object(arguments), other("price_set")))
            }
            Way::PriceClear {
                target,
                account,
                if_revision,
            } => Some((
                json!({ "target": target, "if_revision": if_revision, "account": account }),
                other("price_clear"),
            )),
            Way::PolicySet {
                account,
                value,
                if_revision,
            } => Some((
                json!({
                    "value": value, "if_revision": if_revision, "replace": true, "account": account,
                }),
                other("set_sync_policy"),
            )),
        }
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
    ErrorData::internal_error(message, Some(json!({ "connect": door_word(door) })))
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

// ---- the item search: adapters of `acquisition-search` ----

/// What a tool of the search returns: the document the CLI's `--json`
/// prints — an answer, or a failure in a result marked an error.
type Told = Result<CallToolResult, ErrorData>;

/// The document as a tool's result. The text block is serialized from the
/// document itself, so its keys come in the order the document declares
/// them (C115); the structured content, a JSON value, holds them in its
/// own order, and an agent reads the text.
fn told<T: serde::Serialize>(document: &T) -> Told {
    told_as(document, Prose { all: false })
}

/// [`told`], the document's prose as [`Prose`] says: each string
/// [`spoken`] for this server, and the structured content read back from
/// the text.
fn told_as<T: serde::Serialize>(document: &T, prose: Prose) -> Told {
    let text = spoken(
        &serde_json::to_string(document).map_err(|e| err(e.into()))?,
        prose,
    );
    // C149: every answer is one JSON object; a bare array, a bare null or
    // a scalar is this server's fault, never served (held here, the one
    // door every answer passes, so no tool can regress alone)
    if !text.starts_with('{') {
        return Err(err(anyhow::anyhow!(
            "an answer must be one JSON object, and this tool's is not"
        )));
    }
    let mut result = serde_json::from_str(&text)
        .map(CallToolResult::structured)
        .map_err(|e| err(e.into()))?;
    result.content = vec![rmcp::model::ContentBlock::text(text)];
    Ok(result)
}

/// A refusal the search crate made (C11), in the one shape: `error`,
/// `kind`, `readings`, payload, a scope refusal's in the request's words
/// as a language refusal's are the query's, and `offers`.
fn refused(e: impl Into<SearchError>) -> CallToolResult {
    refusing(refusal::of_search(&e.into(), &Spoken { named: "account" }))
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
    /// The arguments, or [`Arguments`], refused as `arguments`.
    fn parse(self) -> Result<T> {
        serde_json::from_value(Value::Object(self.0))
            .map_err(|e| Arguments(format!("these are not arguments the tool takes: {e}")).into())
    }

    /// The arguments, or their refusal document.
    fn read(self) -> std::result::Result<T, CallToolResult> {
        self.parse().map_err(|e| refusing(refusal(&e, "account")))
    }
}

/// The channel every intent write from this binary is stamped with (C65).
const WRITTEN_VIA: &str = "mcp";

/// One request to the daemon: its answer, a daemon refusal as its closed
/// kind (C85), or the transport's failure, not served.
async fn ask(client: &mut Client, request: &Request) -> Result<Response, Fault> {
    match client.request(request).await.map_err(Fault::from_client)? {
        Response::Error { kind, message } => Err(Fault::Daemon(DaemonError { kind, message })),
        other => Ok(other),
    }
}

/// An answer of a shape the request does not take: not served.
fn unexpected(response: Response) -> Fault {
    Fault::Transport(anyhow::anyhow!("unexpected response: {response:?}"))
}

/// A `realm` tool parameter: pc when omitted (as on the wire), else one
/// of the documented realms in the layer's one sentence; anything else is
/// refused as an argument.
fn realm_param(realm: Option<&str>) -> Result<Realm> {
    match realm {
        None => Ok(Realm::DEFAULT),
        Some(s) => acquisition_verbs::realm(s).map_err(|why| Arguments(why).into()),
    }
}

/// Every tool's `account`, one sentence (#83: undescribed on seven).
const ACCOUNT: &str = "A username, with or without its #discriminator, or a uuid; needed only when several are known.";

#[derive(Deserialize, schemars::JsonSchema)]
struct TabsParams {
    /// League name; defaults to "Standard".
    league: Option<String>,
    /// List this many tabs in place of ten: the most items first where
    /// some are left out, else in the listing's order.
    limit: Option<usize>,
    /// List every tab of the league, or of the selection.
    expand: Option<bool>,
    /// Only tabs of this type, as GGG names it (UniqueStash, MapStash,
    /// PremiumStash …; `by_type` lists them).
    r#type: Option<String>,
    /// Only tabs never fetched.
    never_fetched: Option<bool>,
    /// Realm: pc (default), xbox, or sony.
    realm: Option<String>,
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct CharactersParams {
    /// Restrict to one league; omitted lists every league.
    league: Option<String>,
    /// Restrict to one realm (pc, xbox, sony, poe2); omitted lists every realm.
    realm: Option<String>,
    /// List this many characters in place of ten, in the store's order.
    limit: Option<usize>,
    /// List every character of the selection.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
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
                    "description": "What is searched: {\"account\": needed only where several could be meant, \"realm\": pc, xbox, sony, poe2 or all, needed only where the store holds several, \"membership\": \"live\" (the default), \"all\", the removed items too, each marked with when it left, or \"removed\"}."
                },
                "query": {
                    "type": "object",
                    "description": "{\"text\": a query in the language `describe` prints} or {\"tree\": the tree an answer returned}, never both; omitted, every item in scope. Or {\"trade\": a trade site search's URL, id or query JSON}, translated with no network; `describe` with names [\"trade\"] says what it becomes and reports."
                },
                "view": {
                    "type": "object",
                    "description": "{\"rows\": {\"limit\": N, \"sort\": a value, \"desc\": true}}, the default (ten rows); {\"counts\": {\"keys\": [a key, …], \"sum\": a value}}; {\"cross\": {\"keys\": [two keys]}}; or {\"explain\": {\"path\": a node's path}}; \"routes\": true beside any. `describe` with names [\"answer\"] says what each gives."
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
                    "description": "In place of the fields above: a request as an answer holds one — a count's, `view.rows.next`, an offer — sent whole, as printed."
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

/// What the job tools answer, read once by name (`describe` with names
/// `["daemon"]`, #83): the words their texts carried, cut. The document is
/// the verbs crate's; the words are this adapter's, since what this
/// server does about a daemon (never spawning one in real mode, never
/// replacing one) is its own (C13).
const DAEMON: [(&str, &str); 8] = [
    (
        "real mode",
        "in real-GGG mode the daemon must already be running: a human starts it from the CLI, which also does the login, and this server never spawns or replaces one there — it submits, applies and quotes only through one a human started; in mock mode it starts one into an empty socket",
    ),
    (
        "jobs",
        "an API request is a job: submit_job returns its id at once, and apply_plan the parent's when it submits one (an empty or fully spent plan submits none, `applied: false`); poll job_status, then read job_result. A waiting job with a large ETA is the rate limiter holding, not a hang: holds can reach about 5 minutes",
    ),
    (
        "no daemon",
        "list_jobs, job_status, job_result and daemon_status observe only: with no daemon running none of them spawns one, and each answers from the ledger on disk; list_jobs, job_status and daemon_status say `running: false` — list_jobs `persisted`, the open rows as the last daemon left them (without ETAs), and `written_at`, how old that picture is; job_status the job's row and `next`, the command a person runs to start a daemon; daemon_status `persisted` (waiting, recorded_running, written_at) and `log`, the last daemon's log file in this world. `persisted` is null when this world has no queue yet; job_result answers a finished job's outcome as a running daemon would",
    ),
    (
        "job counts",
        "list_jobs with a daemon running: `running: true`, `total` every job it lists, `by_state` them by state (running, waiting, done, failed, cancelled, a zero included), and `next_starts_in_seconds` and `last_starts_in_seconds` the soonest and the latest predicted start among the waiting (a start, never an end; absent when none is predicted); then `rows` in id order, the first ten open jobs by id (not by when they run: priority decides that, so a running job can be left out) and the ten most recent finished, `open_left_out` and `finished_left_out` counting the rest, which `expand` lists. With none running, `persisted` holds ten of the ledger's open rows as `jobs`, with `total`, `waiting`, `recorded_running` and `left_out` over every one",
    ),
    (
        "ledger",
        "a finished job is answered from the ledger on disk while retention keeps it, whichever daemon ran it, across restarts; its outcome there is final, and an unfinished one is refused as wrong_state with `next`; a job the ledger does not hold is refused as unknown_job with `next`. A job command from the CLI spawns a successor that restores the open ones",
    ),
    (
        "outcome",
        "job_result holds the payload or the error, with `job_id` and `account` beside it; a parent's failure carries its children's tally as `children` (done, failed, cancelled, failed_ids) and the first ten failed children with their kind, target and error as `failed_children`. Payloads can be large: the store tools read the same data",
    ),
    (
        "another daemon",
        "daemon_status `running: true, compatible: false`: a daemon of another contract, artifact, provider or world, reported by its identity alone — contract_matches, artifact_matches, provider_matches, world_matches; artifact_relation same_file, same_bytes for another copy, different, unhashable, no_sibling or unreported, against the acqd beside this server; `wanted` this server's own — and never replaced by this server. A compatible one adds its vitals: provider, uptime, idle_shutdown_seconds, connections, queue counts, rate-limit policies learned, the rails state with the journal path, keyring health and `log`, the file it opened",
    ),
    (
        "wedged",
        "daemon_status `running: true, responsive: false`, with `socket` and `lock_holder`, the pid the world's lock records: something accepted the connection and did not answer hello within 5 s — a wedged daemon, for a human to stop by hand",
    ),
];

/// What `store_status`, `tabs`, `characters` and `item_events` count, and
/// what an age's words beside a time say on every tool that carries them,
/// read once by name (`describe` with names `["store"]`, or `["ages"]`
/// alone, the store's own words): C146, amended 2026-10-27 — on the
/// MCP a count's gloss stands in the `describe` entry the tool's
/// description names. The words are the ones those descriptions carried,
/// whole, and the CLI footers' (C149: the counts a text prints stand in
/// its JSON under one set of words).
const STORE: [(&str, &str); 5] = [
    (
        "store counts",
        "store_status: the store file's path, size and row counts. `leagues` counts the league listing's rows and `league_list` says whether that listing was ever fetched (`acq leagues` or a `leagues` job fetches it); characters and tabs carry their league names whether or not it was. `refused_bodies` counts bodies the store would not take, which `acq store refused` lists. `withheld_responses` counts fetches of tabs or characters the latest list no longer named and `withheld_items` the items they carried: kept as evidence, in no search or count, nothing to do unless a list names the place again. `unlifted_items` counts item-shaped objects in characters' data under a key this build does not read as items, so no search counts them: GGG added a kind of item list, which a newer build reads. `granted_skills` counts the skill entries PoE2 characters' items grant — GGG shows a granted skill once in each place it appears, the item's sockets and the character's skills, so one skill can be two entries — kept with their characters and not counted as items, having no id of their own",
    ),
    (
        "tab counts",
        "tabs, counted first: how many tabs, fetched, never fetched and folders, how many items, and the same by tab type (`by_type`, the most items first); then `rows`, the tabs listed — the ten with the most items, `limit` more or fewer, `expand` every one, in the listing's order where none is left out (`listed`: `all` or `most_items`). `left_out` counts the tabs not listed. `type` and `never_fetched: true` select the tabs listed — the never fetched of one type by name, which a refresh takes — under the league's counts, `selected` counting them",
    ),
    (
        "character counts",
        "characters, counted first over every character the selection holds (`realm` and `league` as asked, absent when not): `characters` how many; `fetched` those a full fetch (equipment and inventory) has filled; `empty_bodies` the fetched whose last full fetch carried no items (`fetched_items` 0); `never_fetched` those listed and never fetched, whose items no answer knows; `items` the live items held at every one. Then `rows`, the characters listed — ten in the store's order (by realm, league, then level, highest first), `limit` more or fewer, `expand` every one — and `left_out`, the characters not listed. On a row, `item_count` is the live items the store holds at the character now and `fetched_items` what the last full fetch's body carried (equipment and inventory, socketed gems not counted), null until one",
    ),
    (
        "event counts",
        "item_events reads the newest events of the last `hours`, at most `limit` (ten when not given): `returned` counts the events in `rows`, and `limit_reached` says it returned as many as `limit`, so the window may hold more (a larger `limit` reads them); the window's whole is not counted. With `summary: true` the same events by location: `events` counts the events summarized, `locations` every location they touched (a move touches two), `by_kind` the added, changed, moved and removed over every location; `rows` lists ten locations, the newest activity first, `expand` every one, and `left_out` counts the locations not listed",
    ),
    ("ages", acquisition_store::age::SAID),
];

/// The names `describe` answers beside the search crate's sections: what
/// the pricing tools, the job tools and the store's counts answer, which
/// are not the search's (C1: the search links no plan).
const OWN_SECTIONS: [&str; 3] = ["pricing", "daemon", "store"];

/// The own sections' entries, in [`OWN_SECTIONS`]' order.
fn own_sections() -> [Vec<Entry>; 3] {
    let entries = |table: &[(&'static str, &'static str)]| {
        table
            .iter()
            .map(|&(name, what)| Entry { name, what })
            .collect()
    };
    [
        acquisition_plan::pricing::described()
            .into_iter()
            .map(|d| Entry {
                name: d.name,
                what: d.what,
            })
            .collect(),
        entries(&DAEMON),
        entries(&STORE),
    ]
}

/// One entry of an own section: a name and what it says.
#[derive(serde::Serialize)]
struct Entry {
    name: &'static str,
    what: &'static str,
}

/// `describe`'s answer: the search crate's document for the names it
/// knows, beside the pricing model and the daemon entries where named or
/// where nothing is named. Flattened, so the language's keys keep the
/// order the crate declares (C115).
#[derive(serde::Serialize)]
struct Described {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    language: Option<acquisition_search::describe::Describe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pricing: Option<Vec<Entry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    daemon: Option<Vec<Entry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    store: Option<Vec<Entry>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct DescribeParams {
    #[schemars(description = describe_names_doc())]
    names: Option<Vec<String>>,
}

/// `describe`'s `names`: the sections as the answer's own keys
/// (`describe::SECTIONS`, which the search crate pins against the answer).
fn describe_names_doc() -> String {
    format!(
        "Sections, each by the key it answers under: {}, {} — or a name within one (league, \
total_res, revision). Omitted: everything, over 100 KB.",
        acquisition_search::describe::SECTIONS.join(", "),
        OWN_SECTIONS.join(", ")
    )
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
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct StoreParams {
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct EventsParams {
    /// How far back to look (default 24 hours).
    hours: Option<f64>,
    /// The most events read, the newest first (default ten).
    limit: Option<usize>,
    /// The events counted by location in place of listed: ten locations.
    summary: Option<bool>,
    /// With `summary`, every location.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

/// `submit_job`'s kind: the protocol's list, the words the daemon's
/// refusal of an unknown kind prints (C127), and what `apply` takes.
fn submit_kind_doc() -> String {
    use acquisition_protocol::job::{MOCK_ONLY_KINDS, client_kinds};
    format!(
        "Job kind, one a client may submit: {}. {} run against the mock daemon alone and are \
refused in real mode. apply_plan sends a reviewed plan, with the checks an apply here skips.",
        client_kinds(false),
        MOCK_ONLY_KINDS.join(", ")
    )
}

#[derive(Deserialize, schemars::JsonSchema)]
struct SubmitParams {
    #[schemars(description = submit_kind_doc())]
    kind: String,
    /// Kind-specific params, e.g. {"league": "Standard", "id": "..."} for
    /// stash. Passed verbatim; visible to every connected client.
    params: Option<Value>,
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
    /// Higher runs sooner (default 0).
    priority: Option<u8>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct AccountsParams {
    /// List every account, not ten.
    expand: Option<bool>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct ListJobsParams {
    /// List every job, open and finished, not ten of each.
    expand: Option<bool>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct JobParams {
    /// The job id, as returned by submit_job.
    id: u64,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct SetPolicyParams {
    /// The policy, e.g.
    /// {"version":3,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","characters":"all","max_age_seconds":3600}}}}}:
    /// per league `tabs` and/or `characters`, "all" or an id list; a realm
    /// named with no leagues is removed. Validated strictly, landing whole
    /// or not at all.
    value: Value,
    /// The revision this write replaces, from sync_policy: required when a
    /// policy exists, `replace` or not; omitted only to create the first.
    if_revision: Option<i64>,
    /// true writes the whole policy: every realm `value` does not name is
    /// removed. Default false: the realms `value` names, the rest kept.
    replace: Option<bool>,
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
struct PlanParams {
    /// League name; defaults to "Standard".
    league: Option<String>,
    /// Realm: pc (default), xbox, sony, or poe2 (a poe2 plan can carry
    /// character work only).
    realm: Option<String>,
    /// An explicit selection, compiled in place of the stored policy:
    /// {"tabs": "all" or [tab ids], "characters": [character ids],
    /// "deep": true to fetch a named map or unique tab's pages on record
    /// with it}, any of them left out; ids as `tabs` and `characters` list
    /// them.
    #[schemars(with = "Option<Value>")]
    selection: Option<acquisition_plan::PlanSelection>,
    #[schemars(description = ACCOUNT)]
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
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct PriceStatusParams {
    /// The league; Standard when omitted.
    league: Option<String>,
    /// pc (the default), xbox, sony or poe2.
    realm: Option<String>,
    /// List every row of `rows.unmatched`, `rows.unreadable` and
    /// `rows.other_league`, past the first ten.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
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
    /// List this many of each item set in place of ten, the first in
    /// listing order.
    limit: Option<usize>,
    /// List every item of both sets.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
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
    /// List this many items in place of ten, the first in listing order;
    /// `total` says how many the selection holds.
    limit: Option<usize>,
    /// List every item of the selection.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
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
    /// List every entry of `posted`, `left_out` and `freshness`'s lists,
    /// past the first ten, the items nothing applies to included in
    /// `left_out` (a league holds thousands); they are counted either way.
    expand: Option<bool>,
    #[schemars(description = ACCOUNT)]
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
    #[schemars(description = ACCOUNT)]
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
    #[schemars(description = ACCOUNT)]
    account: Option<String>,
}

/// The CLI's words the shared layer speaks, each with what it is here:
/// the commands, longest first where one begins another, then the flags
/// (the shared crates' words are the CLI's, which were first). A phrase
/// follows the word in the spelling (`acq search` is `search`); an empty
/// phrase is a person's command, which the words already give a person
/// at a terminal, and which no tool here performs. Drawn from the words
/// the readiness seat met (`audits/readiness-2026-10/seat/`: calls 5, 6,
/// 12, 13, 35, 47), a grep of the shared crates' texts for `acq ` and
/// `--`, and every flag the search crate's command builder prints
/// (`Request::command`, `Next::command`, `opening`: `--account`,
/// `--realm`, `--membership`, `--trade`, `--next`, `--count`, `--cross`,
/// `--sum`, `--limit`, `--sort`, `--desc`, `--explain`, `--routes`,
/// `--context`, `--league`).
const SPOKEN: [(&str, &str); 55] = [
    ("acq price show", "is `price_show`"),
    ("acq price status", "is `price_status`"),
    ("acq price list", "is `price_list`"),
    ("acq price set", "is `price_set`"),
    ("acq price clear", "is `price_clear`"),
    ("acq shop render", "is `shop_render`"),
    ("acq reference currency", "is `reference_currency`"),
    ("acq store status", "is `store_status`"),
    ("acq store events", "is `item_events`"),
    ("acq store characters", "is `characters`"),
    ("acq store refused", "has no tool here: a person runs it"),
    ("acq policy set", "is `set_sync_policy`"),
    ("acq policy", "is `sync_policy`"),
    ("acq refresh", "is `refresh_plan`, then `apply_plan`"),
    ("acq search", "is `search`"),
    ("acq show", "is `show`"),
    ("acq tabs", "is `tabs`"),
    ("acq accounts", "is `accounts`"),
    ("acq leagues", "is `submit_job` with kind `leagues`"),
    ("acq submit", "is `submit_job`"),
    ("acq jobs", "is `list_jobs`"),
    ("acq status", "is `job_status`"),
    ("acq result", "is `job_result`"),
    ("acq cancel", "is `cancel_job`"),
    ("acq daemon status", "is `daemon_status`"),
    ("acq daemon stop", ""),
    ("acq daemon reset-tripwire", ""),
    ("acq auth check", ""),
    ("acq auth status", ""),
    ("acq auth", ""),
    ("acq profile", ""),
    ("--if-revision", "is `if_revision`"),
    ("--league", "is `league`"),
    ("--realm", "is `realm` (`scope.realm` in a search request)"),
    (
        "--account",
        "is `account` (`scope.account` in a search request)",
    ),
    ("--in", "is `in`"),
    ("--covered-by", "is `covered_by`"),
    ("--expand", "is `expand`"),
    ("--replace", "is `replace`"),
    (
        "--limit",
        "is `limit` (`view.rows.limit` in a search request)",
    ),
    ("--count", "is `view.counts.keys`"),
    ("--cross", "is `view.cross.keys`"),
    ("--sum", "is `sum` in `view.counts` or `view.cross`"),
    ("--sort", "is `view.rows.sort`"),
    ("--desc", "is `view.rows.desc`"),
    ("--explain", "is `view.explain.path`"),
    ("--context", "is `context`"),
    ("--routes", "is `view.routes`"),
    ("--against", "is `show`'s `against`"),
    ("--describe", "is `describe`"),
    ("--membership", "is `scope.membership`"),
    ("--trade", "is `query.trade`"),
    (
        "--next",
        "is an answer's `next`, sent back to `search` as printed",
    ),
    (
        "--dir",
        "has no spelling here: ACQ_STORE_DIR sets this server's store",
    ),
    (
        "--fields",
        "has no spelling here, being built on neither surface",
    ),
];

/// The flags built on neither surface beside `--fields`, spelled as it is.
const UNBUILT: [&str; 2] = ["--rebind", "--basis"];

/// Where `word` stands in `words` as a word: not inside a longer one.
fn stands(words: &str, word: &str) -> bool {
    words.match_indices(word).any(|(at, _)| {
        let before = words[..at].chars().next_back();
        let after = words[at + word.len()..].chars().next();
        !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            && !after.is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    })
}

/// What this server says after shared words that speak the CLI (C143:
/// the shared words first, an adapter's spelling after them): `— here,`
/// and each command or flag they speak with what it is here, or nothing
/// when they speak none. A command begun by a longer one the words also
/// speak is the longer one's (`acq auth` in `acq auth check`).
fn spelling(words: &str) -> String {
    let unbuilt = UNBUILT
        .iter()
        .map(|&flag| (flag, "has no spelling here, being built on neither surface"));
    let spoken: Vec<(&str, &str)> = SPOKEN.iter().copied().chain(unbuilt).collect();
    let said: Vec<String> = spoken
        .iter()
        .filter(|(word, _)| stands(words, word))
        .filter(|(word, _)| {
            !spoken.iter().any(|(longer, _)| {
                longer.len() > word.len() && longer.starts_with(word) && stands(words, longer)
            })
        })
        .filter(|(_, here)| !here.is_empty())
        .map(|(word, here)| format!("`{word}` {here}"))
        .collect();
    if said.is_empty() {
        String::new()
    } else {
        format!(" — here, {}", said.join(", "))
    }
}

/// Shared words, then this server's [`spelling`] of the CLI they speak.
fn spelled(words: &str) -> String {
    format!("{words}{}", spelling(words))
}

/// The keys of an answer whose strings are the tool's own words, spelled
/// where they speak the CLI; every other string of an answer is data,
/// sent verbatim. A refusal's and `describe`'s strings are all words.
const WORDS: [&str; 2] = ["why", "hint"];

/// The keys whose string names an entry, asked by that name: never
/// rewritten, their spelling carried by the entry's `what` instead.
const NAMES: [&str; 2] = ["name", "construct"];

/// The keys whose value is payload, not prose: what an agent sends back
/// as printed (an offer, a request, its query and params, a
/// continuation, a count's route) or types as written (an example of
/// the language). Nothing under one is spelled, at any depth: the
/// spelling is prose's, and a request it touched would no longer run as
/// printed (the readiness seat's fix-round review, P2). A refusal's
/// `readings` are payload on every refusal: how what was typed could be
/// read, in the request's words (C143); and a refusal's `current` and
/// `accounts` are data, a stored row and names, carried verbatim as the
/// read that prints them does (round 4's review, P3); a receipt's `undo`
/// is an offer, sent back as printed (round 5 of the 2g build).
const PAYLOAD: [&str; 11] = [
    "offers", "request", "query", "params", "next", "route", "examples", "readings", "current",
    "accounts", "undo",
];

/// Which strings of a document are the tool's words.
#[derive(Clone, Copy)]
struct Prose {
    /// Every string not under [`PAYLOAD`] (a refusal, `describe`); else
    /// those under [`WORDS`] alone (an answer).
    all: bool,
}

/// Every text this server serves, spelled (C143): `text`, a serialized
/// JSON document, with [`spelling`] appended inside each string literal
/// that is the tool's words ([`Prose`]), never one under [`PAYLOAD`],
/// and an entry's name's spelling appended to its `what`. Done on the
/// text, so the document's key order (C115) stands; the structured
/// content is read back from it.
fn spoken(text: &str, prose: Prose) -> String {
    enum Frame {
        Object {
            key: Option<String>,
            expecting_key: bool,
            name: Option<String>,
            payload: bool,
        },
        Array {
            key: Option<String>,
            payload: bool,
        },
    }
    let payload_key = |key: Option<&str>| key.is_some_and(|k| PAYLOAD.contains(&k));
    // a value is payload when its frame is, or the key it stands under is
    let payload_here = |frames: &[Frame]| match frames.last() {
        Some(Frame::Object { key, payload, .. }) | Some(Frame::Array { key, payload }) => {
            *payload || payload_key(key.as_deref())
        }
        None => false,
    };
    let mut out = String::with_capacity(text.len());
    let mut frames: Vec<Frame> = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((at, c)) = chars.next() {
        match c {
            '"' => {
                // the literal whole, escapes included
                let mut end = at + 1;
                let mut escaped = false;
                for (i, c) in chars.by_ref() {
                    end = i + c.len_utf8();
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        break;
                    }
                }
                let literal = &text[at..end];
                let value: String = serde_json::from_str(literal).unwrap_or_default();
                match frames.last_mut() {
                    Some(Frame::Object {
                        key, expecting_key, ..
                    }) if *expecting_key => {
                        *key = Some(value);
                        out.push_str(literal);
                        continue;
                    }
                    _ => {}
                }
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Object { key, .. }) | Some(Frame::Array { key, .. }) => key.clone(),
                    None => None,
                };
                let key = key.as_deref().unwrap_or("");
                let mut more = String::new();
                if payload {
                } else if NAMES.contains(&key) {
                    if let Some(Frame::Object { name, .. }) = frames.last_mut() {
                        *name = Some(value);
                    }
                } else if prose.all || WORDS.contains(&key) {
                    let named = match frames.last_mut() {
                        Some(Frame::Object { name, .. }) if key == "what" => name.take(),
                        _ => None,
                    };
                    let said = match named {
                        Some(name) => format!("{name} {value}"),
                        None => value,
                    };
                    more = spelling(&said);
                }
                if more.is_empty() {
                    out.push_str(literal);
                } else {
                    let escaped = serde_json::to_string(&more).unwrap_or_default();
                    out.push_str(&literal[..literal.len() - 1]);
                    out.push_str(&escaped[1..escaped.len() - 1]);
                    out.push('"');
                }
            }
            '{' => {
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Array { key, .. }) => key.clone(),
                    _ => None,
                };
                frames.push(Frame::Object {
                    key,
                    expecting_key: true,
                    name: None,
                    payload,
                });
                out.push(c);
            }
            '[' => {
                let payload = payload_here(&frames);
                let key = match frames.last() {
                    Some(Frame::Object { key, .. }) | Some(Frame::Array { key, .. }) => key.clone(),
                    None => None,
                };
                frames.push(Frame::Array { key, payload });
                out.push(c);
            }
            '}' | ']' => {
                frames.pop();
                out.push(c);
            }
            ':' => {
                if let Some(Frame::Object { expecting_key, .. }) = frames.last_mut() {
                    *expecting_key = false;
                }
                out.push(c);
            }
            ',' => {
                if let Some(Frame::Object { expecting_key, .. }) = frames.last_mut() {
                    *expecting_key = true;
                }
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

/// A refusal's document as a tool's result, marked an error: its prose
/// [`spoken`] for this server, its readings and offers payload.
fn refusing(doc: Value) -> CallToolResult {
    let text = spoken(&doc.to_string(), Prose { all: true });
    let mut result = CallToolResult::structured_error(serde_json::from_str(&text).unwrap_or(doc));
    result.content = vec![rmcp::model::ContentBlock::text(text)];
    result
}

/// A pricing verb's answer as a tool's result: the document the CLI's
/// `--json` prints, or its failure document, `{"error": …}`, in a result
/// marked an error, its words [`spelled`] for this server.
fn priced<T: serde::Serialize>(answer: Result<T>) -> Told {
    match answer {
        Ok(document) => told(&document),
        Err(e) => Ok(refusing(refusal(&e, "account"))),
    }
}

/// A pricing tool's `realm`, read as the CLI's `--realm` is; a word that
/// is no realm is refused as any argument the tool does not take.
fn price_realm(realm: Option<&str>) -> std::result::Result<Option<Realm>, CallToolResult> {
    realm
        .map(|s| {
            acquisition_verbs::realm(s)
                .map_err(|why| refusing(refusal(&Arguments(why).into(), "account")))
        })
        .transpose()
}

/// The server: one frontend for its lifetime (C46), an agent's door
/// (C13), the account resolved per call.
struct AcqMcp {
    frontend: Frontend,
    /// The one corpus `search` answers over (C98).
    held: Holder,
}

impl AcqMcp {
    /// C121: the account a call's selector names.
    fn account(&self, selector: Option<&str>) -> Result<Account, Fault> {
        self.frontend.account(selector)
    }

    /// The selected account's facts, as the CLI's store verbs open them.
    fn facts(&self, selector: Option<&str>) -> Result<Store, Fault> {
        self.account(selector)?.facts()
    }

    /// The selected account's facts and its intent file, as `acq search`
    /// opens them: the search reads both and writes neither. A failure is
    /// a refusal document, `named` the argument that selects an account.
    fn open_search(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<(Store, Annotations), CallToolResult> {
        self.account(selector)
            .and_then(|account| account.open())
            .map(|opened| (opened.facts, opened.intent))
            .map_err(|e| refusing(refusal(&e.into(), named)))
    }

    /// [`Self::open_search`] with the account's name, for a refusal that
    /// prints a command to run as that account (a second account known,
    /// a command naming none is ambiguous).
    fn open_priced(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<(Store, Annotations, String), CallToolResult> {
        self.account(selector)
            .and_then(|account| account.open())
            .map(|opened| (opened.facts, opened.intent, opened.account.entry.username))
            .map_err(|e| refusing(refusal(&e.into(), named)))
    }

    /// The selected account's intent file alone, for a verb that reads
    /// and writes nothing else — the MCP opens for a verb what the CLI
    /// opens for it, so a facts file the CLI's verb never opens cannot
    /// refuse the tool's — with the account's name, for the commands its
    /// receipt and refusals print as that account.
    fn open_account_intent(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<(Annotations, String), CallToolResult> {
        self.account(selector)
            .and_then(|account| Ok((account.intent()?, account.entry.username)))
            .map_err(|e| refusing(refusal(&e.into(), named)))
    }
}

#[tool_router]
impl AcqMcp {
    // ---- store reads: no daemon, no GGG traffic ----

    #[tool(
        description = "Accounts this machine has logged into (from the store's non-secret index; no daemon), each with its store file: `total` counts every one, `accounts` lists ten, `expand` every one, and `left_out` counts the rest. Each age is in words beside its time (`describe` with names [\"ages\"])."
    )]
    fn accounts(&self, Parameters(p): Parameters<Args<AccountsParams>>) -> Told {
        answer(|| {
            let expand = p.parse()?.expand.unwrap_or(false);
            Ok(self.frontend.accounts()?.bounded(expand))
        })
    }

    #[tool(
        description = "The stash tabs of a league, counted first, from the store (no daemon), then listed — the ten with the most items, `limit` more or fewer, `expand` every one — each with its type, item count and fetch time, its age in words beside it; `type` and `never_fetched` select the tabs listed. `describe` with names [\"store\"] says what each count counts, and [\"ages\"] what an age's words say. A name ending ` (Remove-only)` is GGG's mark on a tab whose items can only be taken out."
    )]
    fn tabs(&self, Parameters(p): Parameters<Args<TabsParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = realm_param(p.realm.as_deref())?;
            let store = self.facts(p.account.as_deref())?;
            let league = p
                .league
                .as_deref()
                .unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
            let rows = store.tabs(realm.as_str(), league)?;
            Ok(acquisition_search::tabs::Tabs::of(
                realm.as_str(),
                league,
                rows,
                &acquisition_search::tabs::Selection {
                    kind: p.r#type,
                    never_fetched: p.never_fetched.unwrap_or(false),
                },
                p.limit,
                p.expand.unwrap_or(false),
                acquisition_store::now(),
            ))
        })
    }

    #[tool(
        description = "Characters known to the store (no daemon), counted first — how many, fetched, fetched empty, never fetched, their items — then listed: ten, `limit` more or fewer, `expand` every one, each with class, level, league, `item_count` (the live items held at it now) and `fetched_items` (what the last full fetch carried, null until one), and when it was listed and fetched, each age in words beside its time. `describe` with names [\"store\"] says what each count counts, and [\"ages\"] what an age's words say. Fresh data is a `characters` or `character` job."
    )]
    fn characters(&self, Parameters(p): Parameters<Args<CharactersParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let realm = p
                .realm
                .as_deref()
                .map(|r| realm_param(Some(r)))
                .transpose()?;
            let store = self.facts(p.account.as_deref())?;
            let realm = realm.map(Realm::as_str);
            Ok(acquisition_verbs::Characters::of(
                realm,
                p.league.as_deref(),
                store.characters(realm, p.league.as_deref())?,
                p.limit,
                p.expand.unwrap_or(false),
                acquisition_store::now(),
            ))
        })
    }

    #[tool(
        description = "Item search over the store (no daemon): a query in the language `describe` prints, asked of every item in scope. The arguments are the request: {} is every item; {\"query\": {\"text\": \"class:ring rarity=rare\"}} is a query, and {\"query\": {\"trade\": a trade site URL}} a site search translated to one. The answer is one JSON document — `query`, `scope`, `total`, `rows`, `view`, `terms`, `basis`; `describe` with names [\"answer\"] says what each key holds and what each count counts. Any request an answer holds is sent back as printed."
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
            match self.open_search(asked.request.scope.account.as_deref(), "scope.account") {
                Ok(opened) => opened,
                Err(refusal) => return Ok(refusal),
            };
        match self
            .held
            .corpus(&asked.request, &store, &intent)
            .and_then(|corpus| asked.answer(&corpus))
        {
            Ok(answer) => told(&answer),
            Err(e) => Ok(refused(e)),
        }
    }

    #[tool(
        description = "What the tools answer and the language `search` takes, read by name (no store read): answer, every key of a search answer; trade, what a site search becomes and its report; fields, what a query may ask; pricing, the listing model and its revision rule; daemon, what the job tools answer without one; store, what store_status and tabs count. The whole is over 100 KB: computed is some 80 KB, fields and trade some 20 KB each, categories some 10 KB, answer and pricing some 5 KB each, and the rest each under 4 KB. A name it does not know is refused, and the refusal lists every name it knows."
    )]
    fn describe(&self, Parameters(p): Parameters<Args<DescribeParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let names = p.names.unwrap_or_default();
        let sections = own_sections();
        // an own section answers to its name, and each of its entries to
        // its own, as the language's entries do (C97: every word an entry
        // is named by can be asked for alone)
        let asked = |n: &str, section: usize, entry: &Entry| {
            n.eq_ignore_ascii_case(OWN_SECTIONS[section]) || n.eq_ignore_ascii_case(entry.name)
        };
        let own_name = |n: &str| {
            OWN_SECTIONS.iter().any(|o| o.eq_ignore_ascii_case(n))
                || sections
                    .iter()
                    .flatten()
                    .any(|e| e.name.eq_ignore_ascii_case(n))
        };
        let language: Vec<String> = names.iter().filter(|n| !own_name(n)).cloned().collect();
        // the whole, or the language's names alone: an own name asked
        // alone is never the whole language
        let language = if names.is_empty() || !language.is_empty() {
            match acquisition_search::describe(&language) {
                Ok(described) => Some(described),
                Err(mut e) => {
                    // every name this tool knows, its own among them
                    if e.kind == acquisition_search::error::ErrorKind::UnknownName {
                        let own: Vec<&str> = OWN_SECTIONS
                            .iter()
                            .copied()
                            .chain(sections.iter().flatten().map(|e| e.name))
                            .collect();
                        e.message.push_str(&format!(", {}", own.join(", ")));
                    }
                    return Ok(refused(e));
                }
            }
        } else {
            None
        };
        let mut chosen = sections.into_iter().enumerate().map(|(section, entries)| {
            let kept: Vec<Entry> = entries
                .into_iter()
                .filter(|e| names.is_empty() || names.iter().any(|n| asked(n, section, e)))
                .collect();
            (!kept.is_empty()).then_some(kept)
        });
        told_as(
            &Described {
                language,
                pricing: chosen.next().flatten(),
                daemon: chosen.next().flatten(),
                store: chosen.next().flatten(),
            },
            Prose { all: true },
        )
    }

    #[tool(
        description = "One item as the search derives it, live or removed (no daemon): its place, its fields, class, category and price, and every line with its kind, template and numbers. `against` adds why the item does or does not match a query, each node's outcome (`describe` with names [\"against\"]); `body` adds the stored body, verbatim. Each age is in words beside its time (`describe` with names [\"ages\"])."
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
                // the language's own refusal of a tree it cannot read
                return Ok(refused(acquisition_search::LanguageError {
                    kind: acquisition_search::ErrorKind::Tree,
                    message: format!(
                        "`against` is a query as a request holds one, {{\"text\": …}} or {{\"tree\": …}}: {e}"
                    ),
                    span: None,
                    readings: Vec::new(),
                }));
            }
        };
        // an authoring error is said before the store is read
        if let Some(Err(e)) = against.as_ref().map(|q| q.read(None)) {
            return Ok(refused(e));
        }
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
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
        description = "Store file path, size, and row counts (no daemon); `describe` with names [\"store\"] says what each count counts."
    )]
    fn store_status(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| self.facts(p.parse()?.account.as_deref())?.status())
    }

    #[tool(
        description = "Item events (added/moved/changed/removed) from recent ingests (no daemon): the newest ten, `limit` more or fewer, beside how many were returned and whether the limit was reached; `summary: true` counts them by location instead, ten locations, `expand` every one. `describe` with names [\"store\"] says what each count counts."
    )]
    fn item_events(&self, Parameters(p): Parameters<Args<EventsParams>>) -> Told {
        use acquisition_verbs::events::{EventSummary, Events, summary};
        answer(|| -> Result<Value> {
            let p = p.parse()?;
            let store = self.facts(p.account.as_deref())?;
            let events = Events::read(
                &store,
                acquisition_store::now(),
                p.hours.unwrap_or(24.0),
                p.limit,
            )?;
            Ok(if p.summary.unwrap_or(false) {
                serde_json::to_value(EventSummary::of(
                    &events,
                    summary(&store, &events.rows),
                    p.expand.unwrap_or(false),
                ))?
            } else {
                serde_json::to_value(events)?
            })
        })
    }

    // ---- the intent/plan slice (tracer step 8): policy → plan → apply ----

    #[tool(
        description = "The account's sync policy (declared coverage and freshness) with its revision — what refresh_plan compiles (no daemon). `policy` is null when none is set."
    )]
    fn sync_policy(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| {
            Ok(self
                .account(p.parse()?.account.as_deref())?
                .policy_shown()?)
        })
    }

    #[tool(
        description = "Write the account's sync policy, validated strictly, compare-and-swap on `if_revision`. Local only: it sends nothing; the network cost appears only when a plan is applied. Each realm the value names replaces that realm alone, the others stay; `replace: true` writes the whole policy. Returns the stored row with the receipt — `target`, `written`, `prior`, `undo` (a request that puts `prior` back) or `no_undo` (why none) — and `league_notes` for a league the fetched league list lacks, with `list_fetched_at` (null if never fetched; a `leagues` job fetches it). Nothing refuses on a note."
    )]
    fn set_sync_policy(&self, Parameters(p): Parameters<Args<SetPolicyParams>>) -> Told {
        let written = (|| -> Result<_> {
            let p = p.parse()?;
            let write = if p.replace.unwrap_or(false) {
                PolicyWrite::Replace
            } else {
                PolicyWrite::Merge
            };
            // C52: naming no revision creates, never replaces.
            let written = self.account(p.account.as_deref())?.set_policy(
                &p.value,
                Expect::or(p.if_revision, Expect::Create),
                write,
            )?;
            Ok(written)
        })();
        match written {
            Ok(w) => told(&receipt::policy(&w, &Spoken { named: "account" })),
            Err(e) => failed(e),
        }
    }

    #[tool(
        description = "Compile a refresh plan for one realm and league, offline — sends nothing, spends nothing: from the stored sync policy, or with `selection` from that explicit selection, which needs no policy. The plan is the bounded action set applying would execute, with reasons, skips, unknown ids and a wire estimate; a running daemon adds its read-only quote, never spawned for it (`quote_note` says why one is absent). A policy plan's `notes` names a league the fetched league list lacks, or a covered league with no character listed. Review it, then hand `plan` to apply_plan."
    )]
    async fn refresh_plan(&self, Parameters(p): Parameters<Args<PlanParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let realm = realm_param(p.realm.as_deref())?;
                let mut opened = self.account(p.account.as_deref())?.open()?;
                let league = p
                    .league
                    .as_deref()
                    .unwrap_or(acquisition_plan::DEFAULT_LEAGUE);
                Ok(match &p.selection {
                    Some(selection) => opened.plan_selection(realm, league, selection).await?,
                    None => opened.plan(realm, league).await?,
                })
            }
            .await,
        )
    }

    #[tool(
        description = "Execute a reviewed plan, its actions and nothing else, as one `apply` parent job the daemon admits or refuses whole. Refused offline if the stored policy revision moved since the plan (replan with refresh_plan). A plan spends each request once: an action fetched since the plan, or held by the daemon, is listed in `spent` and not sent, so a replay answers `applied: false` with `remaining_requests`, what a plan compiled now would fetch. Returns `job_id`, `account`, `requests`, `spent` and `skipped`; poll job_status, then job_result."
    )]
    async fn apply_plan(&self, Parameters(p): Parameters<Args<ApplyParams>>) -> Told {
        settled(
            async {
                let p = p.parse()?;
                let mut opened = self.account(p.account.as_deref())?.open()?;
                let plan = RefreshPlan::from_value(&p.plan).map_err(Fault::from)?;
                Ok(opened.apply(&plan, p.max_requests, &mut None).await?)
            }
            .await,
        )
    }

    // ---- pricing: adapters of acquisition-plan's pricing verbs ----

    #[tool(
        description = "The listing state of one league, counted (no daemon): what every item, tab and character is priced as — by hand, in game, how the two stand — as the counts and the row accounting, without the listings themselves; `describe` with names [\"pricing\"] says what each count counts. price_list names the items; price_show is one target."
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
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let league = p
            .league
            .as_deref()
            .unwrap_or(acquisition_plan::pricing::DEFAULT_LEAGUE);
        priced(
            acquisition_plan::pricing::report(&store, &intent, realm, league)
                .map(|r| r.status(p.expand.unwrap_or(false))),
        )
    }

    #[tool(
        description = "The items of one league that are priced, each with both sides of its listing (no daemon): `items`, ten by default (`limit` more, `expand` all), each with its `subject`, the hand row that applies (`manual`), the game's reading (`game`), the `relation` and the `effective` price; `containers`, the tabs, substashes and characters they sit under; and the counts `describe` with names [\"pricing\"] explains. With neither `relation` nor `effective`, items nothing prices are left out, except unresolved ones."
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
        let (store, intent, account) = match self.open_priced(p.account.as_deref(), "account") {
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
                expand: p.expand.unwrap_or(false),
            },
            &account,
            acquisition_search::answer::shell_quoted,
        ))
    }

    #[tool(
        description = "One target's listing (no daemon): `listing`, both sides with their causes — the hand row that applies (`manual`, its `revision` and the target it is on, `from`), the note and the tab name verbatim beside what each reads as — the relation, the effective price and why; `container`, the one an item sits in; for a container, `items_here` (in it) and `items_covered_below` (through its children), ten each beside its counts, `limit` more, `expand` all (`describe` with names [\"pricing\"]). Read it before price_set or price_clear on a target with its own row: they name the revision they replace."
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
        let (store, intent, account) = match self.open_priced(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        priced(acquisition_plan::pricing::show(
            &store,
            &intent,
            &p.target,
            realm,
            p.league.as_deref(),
            acquisition_plan::pricing::Bound {
                limit: p.limit,
                expand: p.expand.unwrap_or(false),
            },
            &account,
            acquisition_search::answer::shell_quoted,
        ))
    }

    #[tool(
        description = "The forum shop pages for one league, for a person to paste by hand; this server posts nothing and reads no forum (no daemon). One link code per hand-priced item under its price's spoiler, in pages of at most `size` characters: `pages` (each `text` is what to paste), `posted`, `left_out`, `counts`, `policy` and `freshness`, whether the sync policy covers the posted items — reported, never enforced. `describe` with names [\"pricing\"] says what each count counts."
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
        let (store, intent) = match self.open_search(p.account.as_deref(), "account") {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let rendered = acquisition_plan::pricing::shop_render(
            &store,
            &intent,
            self.frontend.provider,
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
        description = "The currency table this build ships (no store, no daemon): every row — `tag`, the word a price cites; `emit`, the word the game writes in a note; `aliases`, legacy words that resolve to it; `display`; `retired` where the game no longer writes it; `evidence` — with the table's version. `word` resolves one word instead."
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
        description = "Set one target's price by hand: one row on an item, tab, substash or character, which what it covers inherits unless a row nearer the item says otherwise. Local only: it sends nothing and changes nothing in game. Over a row the target already has, `if_revision` names that row's revision (price_show reads it); with none, omit it to create one. Returns the receipt — `target`, `written`, `prior`, `revision` (the next write's), `undo` (a request that puts `prior` back) or `no_undo` (why none), `in_facts`; `describe` with names [\"pricing\"] gives the revision rule and the undo."
    )]
    fn price_set(&self, Parameters(p): Parameters<Args<PriceSetParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let (store, mut intent, username) = match self.open_priced(p.account.as_deref(), "account")
        {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let written = acquisition_plan::pricing::set(
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
            &Provenance::via(self.frontend.via),
            &username,
            acquisition_search::answer::shell_quoted,
        );
        match written {
            Ok(r) => told(&receipt::price_set(
                &r,
                &username,
                &Spoken { named: "account" },
            )),
            Err(e) => priced::<()>(Err(e)),
        }
    }

    #[tool(
        description = "Remove one target's own price row; what it covered falls back to the next row up, or to the game's price. Local only: it sends nothing. `if_revision` names the revision of the row it removes (price_show reads it). Returns the receipt — `target`, `written` null, `prior`, the row removed, `revision`, the one the clear left, and `undo`, the price_set that puts it back, a create naming no revision, sent as printed (or `no_undo`, why none)."
    )]
    fn price_clear(&self, Parameters(p): Parameters<Args<PriceClearParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let (mut intent, username) = match self.open_account_intent(p.account.as_deref(), "account")
        {
            Ok(opened) => opened,
            Err(refusal) => return Ok(refusal),
        };
        let written = acquisition_plan::pricing::clear(
            &mut intent,
            &p.target,
            acquisition_plan::pricing::Expect::or(
                p.if_revision,
                acquisition_plan::pricing::Expect::Create,
            ),
            &Provenance::via(self.frontend.via),
            &username,
            acquisition_search::answer::shell_quoted,
        );
        match written {
            Ok(w) => told(&receipt::price_clear(
                &w,
                &username,
                &Spoken { named: "account" },
            )),
            Err(e) => priced::<()>(Err(e)),
        }
    }

    // ---- daemon jobs ----

    #[tool(
        description = "Submit a job to the daemon and return its id at once; poll job_status, then job_result. `describe` with names [\"daemon\"] says what the job tools answer."
    )]
    async fn submit_job(&self, Parameters(p): Parameters<SubmitParams>) -> Told {
        settled(
            async {
                let mut client = self.frontend.connect().await?;
                let id = acquisition_verbs::jobs::submit(
                    &self.frontend,
                    &mut client,
                    &p.kind,
                    p.params.unwrap_or_else(|| json!({})),
                    p.priority.unwrap_or(0),
                    p.account.as_deref(),
                )
                .await?;
                Ok(json!({ "job_id": id }))
            }
            .await,
        )
    }

    #[tool(
        description = "Jobs the daemon lists, with states and ETAs, counted first over every job (`total`, `by_state`), then listed: ten open and the ten most recent finished, `expand` every one. Observes only, never spawning a daemon: with none running it answers from the ledger (`running: false`, `persisted`); `describe` with names [\"daemon\"] says what each holds and counts."
    )]
    async fn list_jobs(&self, Parameters(p): Parameters<Args<ListJobsParams>>) -> Told {
        let expand = match p.parse() {
            Ok(p) => p.expand.unwrap_or(false),
            Err(e) => return failed(e),
        };
        settled(async { Ok(self.frontend.jobs(expand).await?) }.await)
    }

    #[tool(
        description = "One job's state, ETA and retries. A waiting job with a large ETA is the rate limiter holding, not a hang (holds can reach about 5 minutes). Observes only: a finished job, or any job with no daemon running, is answered from the ledger on disk (`describe` with names [\"daemon\"])."
    )]
    async fn job_status(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(async { Ok(self.frontend.job(p.id).await?) }.await)
    }

    #[tool(
        description = "A finished job's outcome — payload or error — with its job_id and account, from the daemon or, with none running, the ledger on disk; an unfinished one is refused. A parent's failure carries `children` and `failed_children`. Payloads can be large — prefer the store tools for reading data."
    )]
    async fn job_result(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(async { Ok(self.frontend.result(p.id).await?) }.await)
    }

    #[tool(description = "Cancel a waiting or running job; cascades to its descendants.")]
    async fn cancel_job(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(
            async {
                let mut client = self.frontend.attach().await?;
                match ask(&mut client, &Request::Cancel { id: p.id }).await? {
                    Response::Ack { .. } => Ok(json!({ "job_id": p.id, "cancel_requested": true })),
                    other => Err(unexpected(other).into()),
                }
            }
            .await,
        )
    }

    #[tool(
        description = "The daemon running: its identity — contract, executable and hash, the world it serves, its socket — which of the four match this server (`contract_matches`, `artifact_matches`, `provider_matches`, `world_matches`; `wanted` is this server's own) and, when compatible, its vitals. Observes only: `running: false` with no daemon, `compatible: false` for one of another contract, artifact, provider or world (never replaced here), `responsive: false` for a wedged one; `describe` with names [\"daemon\"] says what each holds."
    )]
    async fn daemon_status(&self) -> Told {
        settled(async { Ok(self.frontend.daemon().await?) }.await)
    }
}

#[tool_handler]
impl ServerHandler for AcqMcp {
    /// Every call, served with the call itself at hand ([`CALLED`]), so
    /// a refusal can offer it again as each account known.
    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, ErrorData> {
        let called = Called {
            tool: request.name.to_string(),
            arguments: request.arguments.clone().unwrap_or_default(),
        };
        let call = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        CALLED.scope(called, Self::tool_router().call(call)).await
    }

    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(format!(
            "Acquisition (provider: {}).\n\
             - price_set, price_clear and set_sync_policy write locally and send nothing.\n\
             - submit_job, apply_plan and cancel_job talk to the daemon; in real mode a human \
             starts it and this server never spawns one.\n\
             - list_jobs, job_status, job_result and daemon_status observe, and answer without \
             one: the jobs from the ledger on disk.\n\
             - Every other tool reads locally, no daemon; refresh_plan adds a running daemon's \
             quote, never starting one.\n\
             - Begin with search and no arguments; where the store holds several realms, name \
             scope.realm (the refusal offers each).\n\
             - describe with names explains the language and the answers: fields first, then \
             answer, trade, pricing, daemon, store; the whole is over 100 KB.\n\
             - A refusal is marked isError: error, kind, and readings, next and offers where \
             it has them; any request an offer or an answer holds is sent back as printed, to \
             the tool it names or the same one.\n\
             - No tool says what an item is worth: a price is the owner's own listing.",
            self.frontend.provider,
        ));
        info
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // C88: refused before serving — the host shows the line.
    let frontend = match Frontend::agent(WRITTEN_VIA) {
        Ok(frontend) => frontend,
        Err(e) => {
            eprintln!("acq-mcp: {e}");
            std::process::exit(2);
        }
    };
    let (stdio, writer) = transport::stdio();
    // The writer is awaited however the service ends, so a reply queued
    // before it failed (a line answered before initialisation, then
    // stdin closed) still reaches stdout.
    let served: Result<()> = async {
        AcqMcp {
            frontend,
            held: Holder::new(),
        }
        .serve(stdio)
        .await?
        .waiting()
        .await?;
        Ok(())
    }
    .await;
    let _ = writer.await;
    served
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The unreadable policy's remedy names both arguments the retry
    /// needs: the merge would read the row it cannot.
    #[test]
    fn the_unreadable_policy_remedy_names_a_whole_write_at_its_revision() {
        let words = fault_words(
            &Fault::UnreadablePolicy {
                revision: 3,
                detail: "newer".into(),
            },
            "account",
        );
        assert!(
            words.contains("`replace: true` with `if_revision: 3`"),
            "{words}"
        );
        assert!(words.contains("sync_policy"), "{words}");
    }
}
