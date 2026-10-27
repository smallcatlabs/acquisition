# p82 — The held corpus in `acq-mcp`: one load, reused while the basis holds

Pull-forward item 1 (`roadmap/README.md`, "Pull-forward"), issue #82.
The owner ruled the move on 2026-10-19 (`roadmap/GUI-NOTES.md`,
section 8.2, move 2: "the MCP holds its corpus and reuses it while
`Corpus::is_current` says so"); it is unbuilt. It is the first stone
of 4-full's backend. About a day, the measurements included.

## Goal

An agent's `search` through `acq-mcp` costs the evaluation alone
between refreshes: the server loads the corpus once and answers every
later call over it while C98's check says the basis still holds (the
consumer holds the corpus and checks before every answer), and loads
again the moment a fact or an intent write has moved it. The shape is
not new: `acq web` already holds one corpus this way, in a private
struct of the CLI crate that no test pins. This step lifts that one
shape into the search crate, puts the web on it and the MCP beside it,
and pins it once at the crate (C122, one pin at the lowest crate): one
holder, two resident frontends, and a GUI's server will be the third.
No answer is ever served from a corpus whose basis is not the store's
(C48, a stale result taken for current truth is the failure a cache
reintroduces): the hold is keyed on the store's revision and the
intent's, never on time.

## Read

- #82 (the evidence and the pin in one paragraph). Its "`search`,
  `show` and count call" is wider than the tree: `show` derives one
  item in one pass of the read (`show.rs`, "As built") and loads no
  corpus; a count is a `search` view. This step is `search` alone.
- `roadmap/GUI-NOTES.md` section 8.1 (where the 480 ms goes: the
  derivation, not the database; 214 MB peak held, the steady figure
  unmeasured) and 8.2, moves 1 to 3 (what this step measures for move
  1 as a by-product, and what it leaves to move 3, #32). Dated
  evidence, never rewritten.
- `decisions/search.md`: C98 (no search service; a held corpus is
  reused only while every component of its basis matches, checked
  before every answer; a write after the basis is chosen does not
  unmake a labelled answer), C89 (the crate's edges: no async runtime,
  no file of its own), C96 (the realm is the scope), C100 (one answer
  shape, the basis in it). `decisions/store.md` C108 (the revision
  advances in the transaction that changes items, locations or
  membership). `AGENTS.md`: C12 (two surfaces), C46 (shared semantics
  in Rust, every frontend an adapter), C47 (no `unwrap` in the search
  crate), C48, C122.
- `crates/acquisition-store/src/corpus.rs`, the module doc: the
  revision is the highest `responses.id` with its time and the facts
  schema (`Revision`); every path through `Store::record` that commits
  inserts one response, so a refresh that writes a body, a listing or
  a withheld fetch moves it; `Store::revision` reads it alone.
- `crates/acquisition-search/src/corpus.rs`: the module doc's basis
  bullet; `Basis::of` (the store named by a digest of its path, the
  account uuid, the build, the facts revision, every table's version,
  the intent revision); `Corpus::load_of` and `read` (three reads then
  `moved`); `is_current` (a whole-header read, `RealmScope::All`, then
  `Basis` equality) and `still_at` (the revision and the intent alone).
- `crates/acquisition-search/src/answer.rs`: `Request::corpus` (the
  request's realm and membership choose the load), `Request::check`,
  `Request.scope` (`account`, `realm`, `membership`), `answer`.
- `crates/acquisition-cli/src/web/mod.rs`: the "One corpus, held"
  bullet; `Web.held: Mutex<Option<Held>>`, `Held` (realm, membership,
  corpus), `Web::answer` (check, open, lock, reuse where current and of
  the request's scope, else load and swap, then answer under the lock).
  This is the shape, built once; it moves down.
- `crates/acquisition-mcp/src/main.rs`: the module doc's "As built: the
  item search"; `AcqMcp` (one `Frontend`, the account resolved per call,
  C121); `open_search` (facts and intent opened per call through
  `Account::open`); the `search` tool (read, check, open, `corpus`,
  `answer`); `show` (not this step's). The tools are sync `fn`s under
  `rmcp`'s router; a load blocks the call that pays it, as today.
- `crates/acquisition-verbs/src/lib.rs` (the search's verbs stay in the
  search crate; the verbs crate holds the openers) and `frontend.rs`:
  `Account::open`, `Opened`.
- `crates/acquisition-cli/src/search_cmd.rs`: a process per ask; it
  calls `request.corpus` then `answer`, which is what the holder calls
  on a miss. It gains nothing and changes nothing.
- Tests that assert on MCP search answers today: `tests/journey.rs`
  (two tests; the searches come after the applies, none after
  `price_set`), `tests/search_tools.rs` (every `Seat` test;
  `the_requests_account_selects_the_store` runs two accounts through
  one server, which the hold must keep apart;
  `the_search_tool_is_acq_search_request` holds the tool to the CLI's
  document), `tests/tool_refusals.rs` (one search), `tests/mcp_reference.rs`
  (the tool texts: none changes here). The search crate's own:
  `tests/answer.rs`, `c98_a_held_corpus_answers_at_its_basis_and_the_check_sees_the_change`
  and `c98_the_basis_names_the_store_and_the_check_tells_two_files_apart`,
  the fixture helpers they use (`worked_in`, `fetch_tab`, `load_with`).
  The web's: `crates/acquisition-cli/tests/web.rs`,
  `every_search_page_is_a_wording_golden`, `the_fold_answers_as_acq_search_trade_does`,
  `the_moved_world_renders_with_a_badge`,
  `the_refresh_loop_runs_through_the_pages` (no search page after its
  apply: the web's hold has no invalidation pin).
- The harness: `crates/acquisition-mcp/tests/harness/mod.rs` (`Mcp`,
  one server over many calls, newline-delimited JSON-RPC over stdio:
  the shape the measuring script copies), `start_daemon_with`,
  `login_as_mock`, `finish_job`;
  `crates/acquisition-client/tests/harness/journey.rs` (the journey
  world's facts: `POLICY`, `PLAN_REQUESTS`, `STASH_ITEMS` 37,
  `RING_QUERY`, `LIFE_QUERY`, `CURRENCY_QUERY`).
- `.claude/skills/mock-session/SKILL.md` (the isolated world, the
  scripted login); `.claude/skills/seat/seat-mcp.py` (an MCP door that
  starts one server per call, so it cannot see a hold: the measurement
  needs its own driver).
- `search/MEASUREMENTS.md`: M1 (the read alone), M3 (the ask over the
  owner-shaped copy, every release ask under 500 ms), M5 (a resident
  consumer pays one whole load per committed response), M7 (214 MB
  peak, the steady figure not measured apart). The owner-shaped copy
  is `search/item-facts/raw/m3/store` (gitignored, local to the
  owner's machine: a store directory under `mock/`, 22,721 items,
  wrapped by `crates/acquisition-search/examples/copy-as-store.rs`);
  `search/item-facts/scripts/m3-ask.py` for how a measurement was run
  and reported.

## Input

- Ruled: C98 and the owner's word of 2026-10-19. The step needs no
  ruling: the hold is in one process, checked before every answer
  against the store's own revision and the intent's (the shape C98
  ruled, the service C48 refuses), the MCP still consumes the store's
  read API and nothing else (C12), and no answer is kept across
  processes. One slot, not one per account, is this step's own
  decision (Do 2).
- The baseline, measured by the worker before any edit, on a release
  build of HEAD's `acq-mcp` copied aside: Do 1.
- Fixed inputs the tests stage: `search_tools.rs`'s five-item seed and a
  third tab recorded between two calls on one server; a `price_set`
  between two calls on one server; two accounts through one server
  (the existing test); in the search crate, `worked_in`'s store with a
  `fetch_tab` between two asks, a second store, and a second scope.

## Do

1. **Measure before.** A driver script under your scratch (Python,
   modelled on the harness's `Mcp`: `initialize`, `notifications/initialized`,
   then `tools/call` lines, one server over every call; wall time per
   call from the request's write to its response line; the server's
   resident size from `ps -o rss= -p <pid>` after `initialize`, after
   the first call and after the last). Over the journey's world, built
   as `journey.rs` builds it (a mock daemon, the scripted login,
   `set_sync_policy`, plan and apply until the plan names nothing, the
   daemon stopped: the search tools need none), three requests: `{}`
   with `view.rows.limit` 0, `RING_QUERY`, and the currency count view
   (`view.counts`, keys `base`, sum `stack`). One cold call then ten
   warm, each; the first and the median of ten. Then the same over a
   copy of the owner-shaped store (`cp -R search/item-facts/raw/m3/store`
   under your scratch, `ACQ_STORE_DIR` at the copy; never the directory
   under `raw/` itself, since the MCP opens the intent file beside the
   facts), where that directory exists; where it does not, the row says
   "not measured" and nothing is inferred. Release builds
   (`cargo build --release -p acquisition-mcp`); the HEAD binary copied
   aside before any edit, so the after-run uses the step's binary and
   the before-run the same copy. The measurement runs alone on the
   machine: no gate, no build, no other driver while it runs.
2. **The holder, in the search crate.** One type beside `is_current`
   in `corpus.rs` (or a sibling module; the name is yours, not `Held`,
   which the crate uses for an item): a `Mutex<Option<…>>` slot holding
   the scope it was loaded for (the request's `scope.realm` and
   `scope.membership`, as the web keys today) and the corpus in an
   `Arc`. One method does what `Web::answer` does today: under the
   lock, reuse where the scope matches and `is_current(store, intent)`
   says the basis holds, else `Request::corpus` and swap; hand back the
   `Arc<Corpus>` and release the lock, so the evaluation runs outside
   it and a concurrent call waits for a load, never for an answer. One
   slot: the account is already in the basis (the store's digest and
   the uuid), so a call for another account, another realm or another
   membership loads anew and replaces; two accounts alternating cost
   what every call costs today, no worse, and the server's resident
   size is one corpus's. A per-account map waits for a seat that
   alternates accounts (an issue, not this step). A poisoned lock is
   taken as the web takes it (`into_inner`), never a panic (C47).
3. **The web on it.** `Web.held` and `Held` go; `Web::answer` calls the
   holder. Every page renders as before: the web goldens are the check,
   and the module doc's "One corpus, held" bullet names the shared type.
4. **The MCP on it.** `AcqMcp` gains the holder beside `frontend`; the
   `search` tool opens as today (`open_search`, per call, which is what
   keeps the account right under C121) and asks the holder for the
   corpus, then `asked.answer`. `show`, `describe` and every other tool
   are untouched. The module doc's "As built: the item search" gains
   one bullet: the corpus is held across calls and checked before every
   answer, the first call after a refresh or a price write pays the
   load. No tool description changes, so `MCP-REFERENCE.md` does not
   move and `mcp_reference.rs` stays green unchanged.
5. **The pins**, each named for what it holds, none on a clock:
   - the crate's, in `tests/answer.rs` beside the `c98_` tests: two asks
     over an unchanged store hand back one corpus (`Arc::ptr_eq`); a
     `fetch_tab` between two asks hands back another, whose basis's
     `snapshot.response` is higher and whose answer counts the new item;
     a second scope (another membership) loads anew; a second store
     (`c98_the_basis_names_the_store…`'s shape) is never answered from
     the first's corpus.
   - the MCP's, in `tests/search_tools.rs`: one server; `search` `{}`
     answers 5; a third tab recorded through `record` as `seed` records
     it, the store opened by the test through `Store::open` while the
     server runs (the server opens per call and holds no handle
     between); `search` `{}` answers 6 with `basis.snapshot.response`
     higher; then `price_set` on one item; the next `search`'s
     `basis.intent` higher. One test, both writes: the fact and the
     intent each move the basis, by the one check.
   - `the_requests_account_selects_the_store` green unchanged: it is
     the pin that two accounts through one server never share a corpus.
6. **`is_current`'s cost, only if the after-measurement shows it.**
   `is_current` reads the whole header (every location and the realms:
   6 ms by M1's header row, 13 ms with the realms read by the store's
   `corpus.rs` doc, on the owner's copy) where `still_at`
   reads the revision and the intent alone. If the warm call over the
   owner-shaped copy is more than a few ms over the evaluation M3 puts
   at about 29 ms for the dearest query, make `is_current` compare the
   store's revision, the intent's revision and the store's digest, the
   basis's other components being constants of a file (the account
   uuid) and of a process (the build and the table versions), and say
   so in its doc; measure again. If not, leave it and say the number.
7. **Measure after** with the same script, the same worlds, the same
   requests, alone on the machine; the resident size after the tenth
   call is GUI-NOTES move 1's "steady resident size after the load's
   transients drop", measured here for the first time. The commit
   message carries both tables and the corpus build's share: the
   before-median minus the after-warm-median, over the before-median,
   per request and world; the after-cold call shows the load paid once.
   One row, M8, in `search/MEASUREMENTS.md` in the form of M7: the
   command, the worlds, the numbers, and that the driver's text is in
   the step's report, not the tree.
8. **The registry's mechanism fields only.** C98's *Pinned:* gains the
   crate's new test; no sentence of any ruling changes. `REVIEW-CHECKLIST.md`
   gains a row only if the build meets a fault that came back.

## Done when

- `crates/acquisition-search/src/` holds the one holder type;
  `crates/acquisition-cli/src/web/mod.rs` has no `struct Held` and no
  `Mutex<Option<` of its own; `acq-mcp`'s `search` answers through the
  holder (the tree).
- The crate's pin and the MCP's pin exist by the names the commit gives
  them, each green; `the_requests_account_selects_the_store`, both
  journey tests, `the_search_tool_is_acq_search_request`,
  `mcp_reference.rs` and the web goldens green with no edit to any of
  them (the tree and `git diff --stat` of the step's range).
- No test asserts a wall-clock bound; the measurement is the commit
  message's and M8's, never a test's.
- The commit message carries the before and after tables for the
  journey world (release), the owner-shaped copy's or "not measured"
  with why, the resident sizes, and the build's share per request; M8
  is in `search/MEASUREMENTS.md`.
- Do 6 either changed `is_current` with its number or left it with its
  number, said in the message.
- Every build commit names this brief and says "part of #82"; the
  close commit, not the worker's, closes the issue.
- `tools/gate.sh` green on the step's own tree, every stage's exit
  status in the report.

## Not in scope

- The incremental refresh of a held corpus over `item_events` (#32;
  GUI-NOTES move 3): a moved basis costs one whole load here, as M5
  measured, and that is the number this step reports.
- 4-full's subscriber, the live loop and the time seam (4-0,
  `roadmap/TIME-SEAM.md`); rows carrying the item's fields (section
  8.2's move 4) and the dense table (section 6); the memory diet (move
  5); a faster derivation (move 6); the persisted projection (move 7;
  C98's "Not doing" line, its trigger unfired).
- #83, the MCP surface trimmed, and #84's seat: no tool text changes
  here.
- `show` over the held corpus: its read is one item's today; measure
  it before moving it (an issue if a seat feels it).
- A per-account map of corpora (Do 2 says why one slot).
- The CLI: a process per ask, nothing to hold.
- Any change to what the store's revision counts (C108) or to
  `Basis`'s components.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `p82`, every `ACQ_STORE_DIR` under it (the
owner-shaped copy copied there, never opened under `raw/`); commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands as tracked background tasks; `tools/gate.sh`, and
`cargo build --workspace` before any daemon and after any test run
(C84, the artifact mismatch); a measurement runs alone on the machine,
with no gate, build or other driver beside it, and the before-run is
taken from HEAD's binary before the first edit.

## Report

`TEMPLATE.md`, as text; the driver script's full text under
"Measurements", since it is not committed.
