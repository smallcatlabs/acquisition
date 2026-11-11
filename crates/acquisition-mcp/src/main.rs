//! `acq-mcp` — MCP server frontend (tracer bullet).
//!
//! Rulings: `decisions/frontends.md`; the cross-cutting ones are `AGENTS.md`'s.
//!
//! This file is the entry and its wiring. `server.rs` holds the tool
//! table and the handler, `params.rs` every tool's arguments, `told.rs`
//! how an answer and a refusal are told, `prose.rs` the text block they
//! are spoken as, and `describe.rs` the server's own index sections.
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
//!   of the request a `next` already holds. The one exception is
//!   `saved`, which this adapter takes out and resolves before the reader
//!   (the saved search, below): nothing else of the arguments is read
//!   here.
//! - **H2 — `scope.account` selects the store**, where a terminal's
//!   `--account` does; every request an answer holds names it.
//! - **H3 — an omitted realm is the terminal's rule** (C96): resolved
//!   over a store of one realm, refused over several.
//! - **H4 — the answer's JSON whole, no text.** The result's one content
//!   block is the answer, its document serialized with its keys in the
//!   order it declares them (C115), and no structured content is sent:
//!   on the MCP the JSON is the text, with no second rendering (C53).
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
//! every text the shared crates make — a refusal, `describe`, an
//! answer's `why` and `hint` — speaks the request's words, and its ways
//! on are values this server sends as its tools' arguments (C143 as
//! amended; #97, the harness's `c143_a_text_speaks_no_frontend`). Where
//! a daemon's text still speaks a CLI command or flag (`acq accounts`,
//! `--account`), `spoken` appends `— here,` and what each is here, from
//! one table, `SPOKEN`. A person's command (`acq auth check`) is left as
//! the person's; every other string of an answer is data, sent verbatim.
//! The rewrite is on the serialized text, so the document's key order
//! stands (C115). Pinned by `tests/spoken.rs`.
//!
//! An account that cannot be picked is refused in the protocol's words
//! (C121): `not_logged_in` when no account's store is found (naming the
//! directory looked in), `unknown_account` when the selector names none
//! the index ever held, `ambiguous_account` when it names several or
//! none among several, each with `accounts`, the names known or to pick
//! from. Arguments a tool does not take are this adapter's own refusal
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
//! # As built: the saved search (4-full part 1, step 2)
//!
//! `saved_searches`, `saved_search_set` and `saved_search_clear` are
//! adapters of `acquisition_verbs::saved`, the functions `acq saved`
//! calls (C46), and `search` takes `saved`, which `acq search --saved`
//! runs: a tool returns the document the CLI's `--json` prints for the
//! same request, a refusal in a result marked `isError`. The saved
//! searches are the world's, in its user file (C167): no daemon, no
//! network, and no account but where a create resolves its realm.
//!
//! - **`saved_searches`** answers `saved::list` (`limit`, `expand`), or
//!   with `saved`, an id or a name, `saved::show`'s row; `saved` beside
//!   `limit` or `expand` is refused as `arguments` naming the pair (a row
//!   is never a list, C53).
//! - **`saved_search_set`** reads its arguments into the verbs' one
//!   `Set`, its revision through `Expect::or(if_revision, Create)` as
//!   `price_set` reads one: no `id`, a create under a minted id; `id` and
//!   `if_revision`, a write over the row at exactly that revision, the
//!   fields not named kept; `id` and no `if_revision` (or 0), a create at
//!   the id, the whole value, landing only where no live row stands (a
//!   clear's undo). Every other pair is the verbs' `arguments`. `account`
//!   is read only where a create names no realm, to resolve it.
//! - **`saved_search_clear`** takes `id` and `if_revision`, both required,
//!   read as `Expect::Exactly`: a clear is no create, and 0 names a
//!   revision no row stands at, answered as `conflict`.
//! - **`search` with `saved`**: `saved` is taken out, `saved::request_of`
//!   resolves it (as `scope.account`, where that is a string), and the
//!   row's `query.text`, `scope.realm` and `scope.membership` are set into
//!   the arguments as sent, so every other key reaches `Asked::read` and
//!   is refused there as it is without `saved`. Beside it stand
//!   `scope.account`, `view` and `context`; `query`, `scope.realm`,
//!   `scope.membership`, `request`, `basis`, `count` and `counted_at` are
//!   refused naming the word, in the kind and the sentence of the CLI's
//!   refusal of a flag beside `--saved`. A refusal of the resolution
//!   names `scope.account`.
//! - **Every way spoken**: `Spoken` sends each saved way as one of these
//!   tools' arguments, so a receipt's undo and a refusal's offers are sent
//!   back as printed; a run as another account is the arguments as sent
//!   with `scope.account` set, nothing restarted.
//!
//! The stricter rule is the pricing section's: a write that names no
//! revision is a create (C52), and no form here writes at the revision
//! read, the CLI's default. Pinned by `tests/saved_tools.rs`.
//!
//! # Rulings held here
//!
//! - **C13** (the MCP is a thin client): in real mode it talks to the
//!   daemon that is running and never spawns one.
//! - **C14** (agent traffic, one gate): `acq-mcp` submits, applies and
//!   quotes in either mode against a running daemon; `quote` in real mode
//!   sends nothing.

mod describe;
mod params;
mod prose;
mod server;
mod told;
mod transport;

use acquisition_search::Holder;
use acquisition_verbs::Frontend;
use anyhow::Result;
use rmcp::ServiceExt;

use crate::server::AcqMcp;

/// The channel every intent write from this binary is stamped with (C65).
const WRITTEN_VIA: &str = "mcp";

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
