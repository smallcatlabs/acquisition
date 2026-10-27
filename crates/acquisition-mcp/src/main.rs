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

use acquisition_client::client::{Client, ConnectError, DaemonError, NotSpawned};
use acquisition_plan::pricing::Expect;
use acquisition_plan::{PolicyWrite, RefreshPlan};
use acquisition_protocol::protocol::{ErrorKind, Request, Response};
use acquisition_protocol::realm::Realm;
use acquisition_search::SearchError;
use acquisition_search::answer::{Asked, QueryInput};
use acquisition_store::{Annotations, Provenance, Store};
use acquisition_verbs::fault::door_word;
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
/// [`refusal`] words. The release seat's rerun met an unknown account
/// and an uncovered league as JSON-RPC errors where the instructions
/// promise results (mcp-opus-2 calls 50, 46).
fn failed(e: anyhow::Error) -> Told {
    let words = failure_words(&e, "account");
    match e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        Some(Fault::Daemon(refused)) => Err(daemon_error(refused.kind, words)),
        Some(Fault::Door(door)) => Err(connect_error(door, words)),
        Some(Fault::AnotherDaemon(_) | Fault::Unresponsive(_) | Fault::Transport(_)) => {
            Err(ErrorData::internal_error(words, None))
        }
        _ => Ok(CallToolResult::structured_error(refusal(&e, "account"))),
    }
}

/// A refusal's document (C11, C121): `error` in the words the failure
/// has and a `kind` — a layer fault's kind and fields (`accounts`, the
/// names known or to pick from; `next`, the person's command), with
/// `named` the argument that selects an account; `arguments` for a
/// parameter the tool cannot read; `refused` for the rest.
fn refusal(e: &anyhow::Error, named: &str) -> Value {
    let mut doc = serde_json::Map::new();
    doc.insert("error".into(), json!(failure_words(e, named)));
    if let Some(fault) = e.chain().find_map(|c| c.downcast_ref::<Fault>()) {
        doc.extend(fault.fields());
    } else if e.downcast_ref::<BadArguments>().is_some() {
        doc.insert("kind".into(), json!("arguments"));
    } else {
        doc.insert("kind".into(), json!("refused"));
    }
    Value::Object(doc)
}

/// Arguments a tool cannot read: refused as `arguments`.
#[derive(Debug)]
struct BadArguments(String);

impl std::fmt::Display for BadArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BadArguments {}

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
        Some(s) => acquisition_verbs::realm(s).map_err(|why| BadArguments(why).into()),
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
                    "description": "{\"text\": a query in the language `describe` prints} or {\"tree\": the tree an answer returned}, never both. Omitted: every item in scope. A refinement is the old text in parentheses and a new term. Or {\"trade\": a trade site search}: its URL, its id or its query JSON as a string, or the query JSON as an object (PoE1), translated with no network to the text a person would type; the answer's `query.trade` holds every clause with its status — exact, ambiguous, unsupported, listing_only, disabled; an exact one with a term of its own its `at`, `term` and `path` alone, any other — a sum's filter, a group, a clause not exact — the site's filter and what the catalogue says of it — the `remainder` that did not run, and `fidelity`, how far the text is the site's search: exact, ambiguous where a clause ran as the site may not read it, or partial where the remainder is not empty, the answer then not the site's search. An ambiguous clause whose other reading is known carries `other`: that reading, and the items in scope it `adds` and `drops` against the query as run. Never how far the items are the site's: the answer is over the items the store has fetched, priced or not — what it never fetched is `scope.never_fetched` — and the site shows public listings, a tab's being `tab.public`. A URL's realm is the scope, so `scope.realm` beside one is the URL's or nothing; an id or JSON takes a realm as a text does, and a `league` beside the JSON's `query` is a term, as a URL's league is; `defences: \"displayed\"` beside it reads the four defence filters as the item stands, where the site reads them at 20 % quality. A `text` sent beside `trade` must be the one it translates to, as `view.rows.next` sends it: an edited text is refused — drop `trade` to run the edit as your own query. `describe` with names [\"trade\"] says what each clause becomes."
                },
                "view": {
                    "type": "object",
                    "description": "One of: {\"rows\": {\"limit\": N, \"sort\": a value, \"desc\": true}}, the default, 20 rows where no limit is given; {\"counts\": {\"keys\": [a key, …], \"sum\": a value, \"limit\": N}}, a table for each key and no rows, a sum's `value` over the items that have the value and its `lacking` counting the items without it, which add nothing; {\"cross\": {\"keys\": [two keys]}}, one crossed table; {\"explain\": {\"path\": a node's path, a term's as `terms[].path` gives it}}, the total with that node forced true and forced false. Beside any of them, \"routes\": true gives every count its request, a terminal's --routes; alone, {\"routes\": true} is the default rows. `describe` with names [\"counts\"] lists the keys. A sort's value is a field (reqlevel), a computed value (pseudo.total_res), a line's number (line(\"# to maximum Life\").arg1) or a sum of one (`sum(\"# to maximum Life\")`), spanning every source unless `source=` names one; an item that lacks it is last, ascending or descending."
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
    #[schemars(description = describe_names_doc())]
    names: Option<Vec<String>>,
}

/// `describe`'s `names`: the sections as the answer's own keys
/// (`describe::SECTIONS`, which the search crate pins against the answer).
fn describe_names_doc() -> String {
    format!(
        "Only these entries: a section, by the answer's own key — {} — or a name within one: \
`league`, `total_res`. Omitted: the whole language.",
        acquisition_search::describe::SECTIONS
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<_>>()
            .join(", ")
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

/// `submit_job`'s kind: the protocol's list, the words the daemon's
/// refusal of an unknown kind prints (C127), and what `apply` takes.
fn submit_kind_doc() -> String {
    use acquisition_protocol::job::{MOCK_ONLY_KINDS, client_kinds};
    format!(
        "Job kind, one a client may submit: {}. {} run against the mock daemon alone and are \
refused in real mode. apply takes {{\"jobs\": [{{\"kind\", \"params\"}}, …]}}, the single \
requests a plan authorizes (stashes, stash, characters, character); apply_plan sends a reviewed \
plan after the checks this skips.",
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
    /// policy exists (replacing a policy you have not read is refused as a
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
    /// An explicit selection, compiled in place of the stored policy, in
    /// the shape a selection plan's `basis.selection` carries:
    /// {"tabs": "all" or [tab ids], "characters": [character ids],
    /// "deep": true to fetch a named map or unique tab's pages on record
    /// with it}, any of them left out. Ids are as `tabs` and `characters`
    /// list them; one the facts do not hold is reported, never fetched.
    #[schemars(with = "Option<Value>")]
    selection: Option<acquisition_plan::PlanSelection>,
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
            acquisition_verbs::realm(s).map_err(|why| {
                CallToolResult::structured_error(json!({ "error": why, "kind": "arguments" }))
            })
        })
        .transpose()
}

/// The server: one frontend for its lifetime (C46), an agent's door
/// (C13), the account resolved per call.
struct AcqMcp {
    frontend: Frontend,
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
            .map_err(|e| CallToolResult::structured_error(refusal(&e.into(), named)))
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
            .map_err(|e| CallToolResult::structured_error(refusal(&e.into(), named)))
    }

    /// The selected account's intent file alone, for a verb that reads
    /// and writes nothing else — the MCP opens for a verb what the CLI
    /// opens for it, so a facts file the CLI's verb never opens cannot
    /// refuse the tool's.
    fn open_account_intent(
        &self,
        selector: Option<&str>,
        named: &str,
    ) -> std::result::Result<Annotations, CallToolResult> {
        self.account(selector)
            .and_then(|account| account.intent())
            .map_err(|e| CallToolResult::structured_error(refusal(&e.into(), named)))
    }
}

#[tool_router]
impl AcqMcp {
    // ---- store reads: no daemon, no GGG traffic ----

    #[tool(
        description = "Accounts this machine has logged into (from the store's non-secret index; no daemon), each with its store file."
    )]
    fn accounts(&self) -> Told {
        answer(|| Ok(self.frontend.accounts()?))
    }

    #[tool(
        description = "The stash tabs of a league, counted first, from the shared store (no daemon, no network): how many tabs, fetched, never fetched and folders, how many items, and the same by tab type (`by_type`, the most items first); then `rows`, the tabs listed — every one where the league holds thirty or fewer, else the ten with the most items (`listed`: `all` or `most_items`) — each with its type, item count and fetch time, a substash naming its `parent`. `left_out` counts the tabs not listed; `expand: true` lists them all (some 220 bytes a tab). `type` and `never_fetched: true` select the tabs listed — the never fetched of one type by name, which a refresh takes — under the league's counts, `selected` counting them. A name ending ` (Remove-only)` is GGG's mark on a tab whose items can only be taken out; a search asks it as `tab.remove_only`."
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
            let store = self.facts(p.account.as_deref())?;
            store.characters(realm.map(Realm::as_str), p.league.as_deref())
        })
    }

    #[tool(
        description = "Item search over the shared store (no daemon, no network): a query in the language `describe` prints, asked of every item in scope. The arguments are the request: {} is every item; {\"query\": {\"text\": \"class:ring rarity=rare\"}} is a query, and {\"query\": {\"trade\": a trade site URL}} a site search translated to one. The answer is one JSON document, its keys in this order: `query` (the text as understood, and its tree), `scope` (account, realm, `items` every live item in scope before any term, what was and was not fetched: `fetched` counts the tabs, their pages and the characters fetched, `oldest_fetch` and `newest_fetch` the earliest and the latest of their last fetches, each one location's, where a row's `place.fetched_at` is its own; `never_fetched` counts the tabs, their pages and the characters a listing names and no fetch has filled, and no answer says what they hold, a count of none or of many; `never_fetched_by_type` is the same by tab type, a character's `character`: each type that has a location never fetched, its `tabs` every location of the type and `fetched` how many of them were, so `tabs` minus `fetched` were never fetched, and `items` the live items in the fetched ones, counted as the `tabs` tool counts a type, over the scope's realm in every league whatever the query asks, since what a never-fetched location holds is unknown, and `tabs` lists a league's tabs, `never_fetched_leagues` naming the leagues that hold a never-fetched tab, the most first), `total`, `rows` — id, name, base, rarity, place by name and id — a tab's page with no name of its own has `name` null and its tab in `parent` — with the cell the item sits in (`place.cell`, GGG's x and y, in a tab or an inventory) and when that place was last fetched (`place.fetched_at`), and the lines each term touched with their values, the term named by its `path` in `terms`, never the item's body, which is `show` — then `view`, and last `terms` (for each term how many items matched, failed, lacked the thing or were undecided — each term alone over every item in scope, never over the matches, which are `total`; a pattern term's `resolved` is what it matched there, its `rest.denominator` `scope`) and `basis` (what the answer was computed from: its `intent` is the revision of the account's hand-written prices and sync policy, which every write to either moves). Rows past the limit are counted in `view.rows.left_out`, and `view.rows.next` is the request for the rows after: send it back whole. A count holds the request that returns exactly its members where a next step hangs on it — what is undecided, what holds only together, what a key could not place (`none`, `undecided`), what an ambiguous trade clause's other reading adds or drops, and each term's matched where nothing matched — and every other count is its number alone: `view.routes: true` asks for every count's. A zero total offers requests under `zero.offers`: any request an answer holds is sent back as printed. A count-first look is {\"view\": {\"counts\": {\"keys\": [\"tab\", \"rarity\"]}}}; the vocabulary of lines is the key `line`, or `line:<text>` narrowed. A key is one entry of `keys`: a terminal's list `line:resist,life` is [\"line:resist\", \"line:life\"], and a comma inside an entry is part of its text. A time is seconds since 1970, UTC. The messages name the request's own fields, and `describe` names a terminal's flags: --count, --cross, --sum, --sort, --desc, --limit, --explain and --routes are `view`'s fields, --context is `context`, --realm and --membership are `scope`'s, and --next is `view.rows.next` sent back. A refusal is a result marked isError: `error`, `kind`, `readings` (what may have been meant, as a terminal would type it: a query text, a name, a flag or a whole `acq` command) and, where a way on is a search, `offers` (the same as requests to send instead)."
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
        description = "The query language `search` takes, as this build knows it (no store read): fields, what a line has, operators, closed value sets, slots, the computed values with their definitions, what a count takes, what a trade site search's clauses become, what is not built and the limits stated — or only the entries named. The whole is over 100 KB, most of it under `computed`: name what is needed instead. Of the sections `names` lists, computed is some 80 KB, fields and trade some 20 KB each, categories some 10 KB, and the rest each under 4 KB; a single name — league, total_res — returns its own entry. A name it does not know is refused, and the refusal lists every name it knows."
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
        description = "Store file path, size, and row counts (no daemon, no network). `leagues` counts the league listing's rows and `league_list` says whether that listing was ever fetched (`acq leagues` or a `leagues` job fetches it); characters and tabs carry their league names whether or not it was. `refused_bodies` counts bodies the store would not take, which `acq store refused` lists. `withheld_responses` counts fetches of tabs or characters the latest list no longer named and `withheld_items` the items they carried: kept as evidence, in no search or count, nothing to do unless a list names the place again. `unlifted_items` counts item-shaped objects in characters' data under a key this build does not read as items, so no search counts them: GGG added a kind of item list, which a newer build reads. `granted_skills` counts the skill entries PoE2 characters' items grant — GGG shows a granted skill once in each place it appears, the item's sockets and the character's skills, so one skill can be two entries — kept with their characters and not counted as items, having no id of their own."
    )]
    fn store_status(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| self.facts(p.parse()?.account.as_deref())?.status())
    }

    #[tool(
        description = "Item events (added/moved/changed/removed) from recent ingests, newest first (no daemon, no network); `hours` how far back, `limit` how many of the newest."
    )]
    fn item_events(&self, Parameters(p): Parameters<Args<EventsParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let store = self.facts(p.account.as_deref())?;
            let since = acquisition_store::now() - (p.hours.unwrap_or(24.0) * 3600.0) as i64;
            store.events_since(since, p.limit.unwrap_or(200))
        })
    }

    // ---- the intent/plan slice (tracer step 8): policy → plan → apply ----

    #[tool(
        description = "The per-account sync policy (declared coverage + freshness, an annotation) with its revision — what refresh_plan compiles. Reads the store; no daemon, no network. `policy` is null when none is set."
    )]
    fn sync_policy(&self, Parameters(p): Parameters<Args<StoreParams>>) -> Told {
        answer(|| {
            let row = self.account(p.parse()?.account.as_deref())?.policy()?;
            Ok(json!({ "policy": row }))
        })
    }

    #[tool(
        description = "Write the per-account sync policy (validated strictly, compare-and-swap on if_revision). Sends nothing — the policy is local; the network cost appears only when a plan is applied. Each realm the value names replaces that realm alone; the others stay, a realm named with no leagues is removed, and `replace: true` writes the whole policy instead. Replacing an existing policy requires if_revision (from sync_policy) either way; a stale or missing revision is a conflict, never a clobber. Returns the stored row, the whole policy as it now stands, and `league_notes` when the value names a league the account's fetched league list does not hold: each such league with `list_fetched_at`, when that list was fetched, null when it never was (a `leagues` job fetches it). Nothing refuses on a note."
    )]
    fn set_sync_policy(&self, Parameters(p): Parameters<Args<SetPolicyParams>>) -> Told {
        answer(|| {
            let p = p.parse()?;
            let write = if p.replace.unwrap_or(false) {
                PolicyWrite::Replace
            } else {
                PolicyWrite::Merge
            };
            // C52: naming no revision creates, never replaces.
            Ok(self.account(p.account.as_deref())?.set_policy(
                &p.value,
                Expect::or(p.if_revision, Expect::Create),
                write,
            )?)
        })
    }

    #[tool(
        description = "Compile a RefreshPlan for one (realm, league) from the stored sync policy and the facts on record: stash listing and tab fetches, the realm's character listing and character fetches, each facet as the policy covers it. With `selection`, compile that explicit selection instead: the listings it selects from and each named tab or character however fresh, needing no policy and carrying no policy revision, so apply_plan checks none. Either plan is the explicit, bounded action set applying would execute, with per-action reasons, skipped tabs and characters, unknown ids, and a coarse wire estimate. Offline — sends nothing, spends nothing. A running daemon adds its read-only quote (ETA + rate-limit headroom; never spawned for this); `quote_note` says why one is absent. Only policy plans can carry `notes`; selection plans carry none: `notes.league` names the plan's league when the account's fetched league list does not hold it (with `list_fetched_at`, when that list was fetched; nothing when no list was ever fetched, which set_sync_policy's receipt says), and `notes.no_characters_listed` says the policy covers the league's characters and the realm's character listing names none in it. Review the plan, then hand `plan` to apply_plan."
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
        description = "Execute a reviewed plan: its actions and nothing else, as one `apply` parent job the daemon admits or refuses whole at submit (single-request vocabulary + the max_requests budget). Refused offline — before any daemon contact — if the stored sync-policy revision moved since the plan (replan with refresh_plan). In real-GGG mode the daemon must already be running (a human starts it; this server never spawns one there). An empty plan is a no-op with no daemon contact (`applied: false`, `note`, `skipped`). A plan spends each request once: an action whose target was fetched after the plan was compiled, or whose request the daemon already holds, is listed in `spent` (with `fetched_at`, or the `job_id` holding it) and not sent again, so replaying a plan answers `applied: false`, sends nothing, and gives `remaining_requests`, what a plan compiled now would fetch (null while a queued request has not landed). Returns the parent `job_id`, its `account`, the `requests` sent, `spent` and `skipped`; poll job_status, then job_result."
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
        description = "The listing state of one league, counted (no daemon, no network): what every item, tab and character is priced as — by hand (a row this server or the CLI wrote), in game (an item's note, then its tab's name, read only where the tab is public), how the two stand — as the counts and the row accounting, without the listings themselves. `counts.by_effective` says who decides each item's price (game, manual, none, unresolved — a row that cannot be read could decide), `counts.by_relation` how the two sides stand (none, manual_only, game_only, agree, conflict), so the items priced by hand are its manual_only, agree and conflict together; `counts.game_priced` the items the game prices; `counts.residue` the items carrying price text the trade site cannot see — a note on a character's item, which decides no price until the item is moved into a public stash tab, or a note or tab name in a tab that is not public, which decides none until the tab is made public in the game; `counts.invalid_notes` the notes that start like a price and do not read as one (no effect: the tab's name applies); `rows` accounts for every hand-written row: how many apply here, which name nothing in the facts (`unmatched`, each row whole — `value`, `from`, `revision`, `updated_at`, `written_via` — since no listing carries it), which cannot be read, which belong to other leagues or realms. The document `acq price status --json` prints. price_list names the items; price_show is one target."
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
            },
            &account,
            acquisition_search::answer::shell_quoted,
        ))
    }

    #[tool(
        description = "One target's listing (no daemon, no network): `listing`, both sides with their causes — the hand row that applies with its `revision` and the target it is on (`manual.from`; the target's own row when that is the target itself, a container's when `inherited`), the note and the tab name verbatim beside what each reads as — the relation, the effective price and why; `container`, the one an item sits in; for a tab, substash or character, `items_here` (physically in it) and `items_covered_below` (covered through its children). A bare id is refused with its two readings, `item/<id>` and `character/<id>`; a target the facts do not hold is refused (`kind: \"not_found\"`) naming the league that holds it, if any, and saying so when it has a row of its own and naming what displays it: price_status lists a row the facts hold nowhere whole, and price_show in the league that holds the target shows its row. The document `acq price show --json` prints. Read it before price_set or price_clear on a target that has a row of its own: they name the revision they replace."
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
            &account,
            acquisition_search::answer::shell_quoted,
        ))
    }

    #[tool(
        description = "The forum shop pages for one league, rendered for a person to paste by hand; this server posts nothing and reads no forum (no daemon, no network). One link code per hand-priced item under its price's spoiler, in pages of at most `size` characters, each page labelled n of N: `pages` (each `text` is what to paste, and its `tabs` name each `Stash<n>` its links write — the tab's rank among the tabs the website lists — by the tab's own `name`, with how many `items` link to it), `posted` (the items on them), `left_out` (the first 50 items left off for a reason, the blocked first, each with its `cell` — what the game already lists or a skip is omitted, a case the forum has not been observed for is blocked — and `left_out_total` counting them all; `expand: true` lists every one, the items nothing applies to too, as `off_page_listed` says), `counts` (every item by its cell; the items nothing applies to in `off_page`), `policy` (each cell's rule and count) and `freshness`: whether the sync policy covers the posted items and which of them are older than its window, with the refresh that would renew them — reported, never enforced. `page` returns one page's record alone. The document `acq shop render --json` prints."
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
        description = "Set one target's price by hand: one row on an item, tab, substash or character, which what it covers inherits (a tab's items, a folder's tabs, a parent tab's substashes, a character's items) unless a row nearer the item says otherwise. Local only: it sends nothing and changes nothing in game. Stricter than `acq price set`: a target that already has a row of its own is replaced only when `if_revision` names that row's revision, and without it the write is refused naming the current revision — `kind: \"conflict\"`, the revision in `revision`, as for a stale one — read the target with price_show first; with no row, omit `if_revision` to create one. A clear takes a revision too, so the next write's revision continues from it. Returns the receipt: `target`, `written` (the row now, with its revision), `prior` (the row replaced, or null after a create) — the undo after a create is price_clear with `if_revision` the written revision, and after a replace price_set with the prior's value and `if_revision` the written revision, unless the prior is a value this build cannot read (a newer build's, replaced only by naming its revision), which it cannot write back — and `in_facts`, false for a target the facts do not hold (not fetched yet, or a typo: the row stands). The document `acq price set --json` prints."
    )]
    fn price_set(&self, Parameters(p): Parameters<Args<PriceSetParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let (store, mut intent) = match self.open_search(p.account.as_deref(), "account") {
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
            &Provenance::via(self.frontend.via),
        ))
    }

    #[tool(
        description = "Remove one target's own price row; what it covered falls back to the next row up, or to the game's price. Local only: it sends nothing. Stricter than `acq price clear`: `if_revision` must name the revision of the row it removes, and without it the clear is refused naming the current revision (or saying the target has no row of its own) — `kind: \"conflict\"`, the revision in `revision` — read the target with price_show first. A clear takes a revision too, so the next write's revision continues from it. Returns the receipt: `target`, `written` null, `prior` (the row removed); price_set with the prior's value puts it back. The document `acq price clear --json` prints."
    )]
    fn price_clear(&self, Parameters(p): Parameters<Args<PriceClearParams>>) -> Told {
        let p = match p.read() {
            Ok(p) => p,
            Err(refusal) => return Ok(refusal),
        };
        let mut intent = match self.open_account_intent(p.account.as_deref(), "account") {
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
            &Provenance::via(self.frontend.via),
        ))
    }

    // ---- daemon jobs ----

    #[tool(
        description = "Submit a job to the daemon and return its id immediately; poll job_status then fetch job_result. In real-GGG mode the daemon must already be running (a human starts it; this server never spawns one there)."
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
        description = "Jobs the daemon lists, with states and ETAs: the open ones, and the finished ones of this daemon's work and of what it restored from the daemon before it, read from the ledger on disk. Observes only: with no daemon running it never spawns one and answers from the ledger on disk instead — running: false, persisted: the non-terminal rows as the last daemon left them (jobs, without ETAs) and written_at, how old that picture is; persisted is null when this shell's world has no queue yet. A job command from the CLI spawns a successor that restores them."
    )]
    async fn list_jobs(&self) -> Told {
        settled(async { Ok(self.frontend.jobs().await?) }.await)
    }

    #[tool(
        description = "One job's state, ETA, and retries. A waiting job with a large ETA is the rate limiter holding, not a hang (holds can reach ~5 minutes). A finished job is answered from the ledger on disk while retention keeps it, whichever daemon ran it. Observes only: with no daemon running it answers from the ledger on disk — running: false, persisted: the job's row as the last daemon recorded it, and next, the command a person runs to start a daemon; a job the ledger does not hold is refused as unknown_job with next."
    )]
    async fn job_status(&self, Parameters(p): Parameters<JobParams>) -> Told {
        settled(async { Ok(self.frontend.job(p.id).await?) }.await)
    }

    #[tool(
        description = "A finished job's outcome — payload or error — with its job_id and account beside it; answered across daemon restarts (the queue persists), and with no daemon running read from the ledger on disk, where a finished job's outcome is final (an unfinished one is refused as wrong_state with next). A parent's failure carries its children's tally as `children` (done, failed, cancelled, failed_ids), and the first ten failed children with their kind, target and error as `failed_children`. Payloads can be large — prefer the store tools for reading data."
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
        description = "The daemon running: its identity — contract revision, the executable it runs from and its hash (artifact), the world it serves (its canonical store root), the socket it was reached on (derived from the world), how that file relates to the acqd beside this server (artifact_relation: same_file, same_bytes for another copy, different, unhashable, no_sibling, unreported), which of the four dimensions match (contract_matches, artifact_matches, provider_matches, world_matches), and under wanted this server's own version, contract, provider and world with that sibling — all from the one look that judged the daemon; and, when compatible, its vitals: provider, uptime, idle_shutdown_seconds (the idle exit it was started with), connections, queue counts, rate-limit policies learned, rails state (with the journal path), keyring health, and log — the log file the daemon opened, as it reports it. Observes only: running=false when no daemon is up, with persisted — what the ledger on disk holds: waiting, recorded_running, written_at; null when this shell's world has no queue yet — and log, the last daemon's log file in this world when one exists; running=true, compatible=false for a daemon of another contract, artifact, provider or world, reported by its identity alone and never replaced by this server; running=true, responsive=false (with socket and lock_holder, the pid the world's lock records) for something that accepted the connection and did not answer hello within 5s — a wedged daemon, for a human to stop by hand."
    )]
    async fn daemon_status(&self) -> Told {
        settled(async { Ok(self.frontend.daemon().await?) }.await)
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
             and in game; price_set and price_clear write one hand row (local only, \
             sending nothing), and over a row the target already has they name the \
             revision they replace; shop_render renders the forum pages for a person \
             to paste, posting nothing; reference_currency is the currency table a \
             price cites. The store tools (search, show, accounts, tabs, characters, \
             store_status, item_events) and the pricing tools read the shared SQLite \
             store directly — no daemon, no network. \
             The refresh slice: sync_policy / \
             set_sync_policy declare what to keep fresh (local, sends nothing; replacing a policy \
             must name the revision it replaces), refresh_plan compiles it offline into \
             the explicit bounded action set for review, or compiles a `selection` of tabs \
             and characters named in its place, and apply_plan spends exactly that plan as \
             one parent job. Job tools talk to the local daemon; API \
             requests are jobs (submit_job returns an id; poll job_status, then \
             job_result). Rate-limit holds can reach ~5 minutes — a waiting job is the \
             limiter working. Login is done by the human via the `acq` CLI; this server \
             never spawns or replaces a daemon in real-GGG mode — there it submits, applies \
             and quotes only through one a human already started.",
            self.frontend.provider,
            transport::MAX_DEPTH,
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
        AcqMcp { frontend }.serve(stdio).await?.waiting().await?;
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
