# 2g — The read economy and the refusal and receipt bundle: the build brief

Drawn from `2g-design.md` (`e11768e2`) after the owner ruled on it in
full (2026-10-27, "yes to all", at `ea0b0255`): C53 amended three ways
(Q1 the bound of ten rows beside counts with `limit` and `expand`, held
once; Q2 on the MCP the JSON is the text and no second rendering; Q4
ages in words beside epochs), C149 new (every answer and refusal one
JSON object, lists counted beside rows, the one non-additive change),
C121 kept (no `unknown_account`). Q5's refusal shape and Q6's receipt
shape are delegated to this build, recorded in the commits that build
them and listed for the close; Q2's second copy is this build's one
host measurement; Q7's constraints bind. Issues #9 and #10. Every
citation below was checked against the tree at `7b2e0801`; where the
design's memory of the tree differs, the tree's line stands here and
the difference is named in place.

## Goal

An agent's call pays for what the next decision needs: every list in a
JSON answer holds ten of like entities by default beside its counts,
more on `limit`, all on `expand` where the tool has it, with how many
it left out and, where the maker continues, the request that
continues it (C53, the read economy); no tool answers a bare array
(C149); an age the text prints stands in words beside its epoch (C53,
ages); every refusal is one shape carrying what an agent acts on next
(`kind`, `offers`, `readings`, `next`, the kind's own fields) and
every receipt one shape carrying the revision the next write names
and the undo as an offer. Each rule is held once, at the lowest crate
that holds it (C122, a behaviour pinned once), so the seat's three
largest answers fall to a few KB each and the next seat meets no
answer it must read twice and no count it must make by eye.

## Read

- `roadmap/README.md`: "A step"; "Current step"; the ledger rows for
  #82, #83 and #84; under "Open", "For 4-full, from 4b" (lines 273 to
  282: the web phrasing for the next actions the pages print as command
  lines, which Q5's offers and Q6's `undo` take up).
- `roadmap/briefs/TEMPLATE.md` (the form; a fix round's done-when);
  `roadmap/briefs/2g-design.md` whole (the source: Q1 to Q8, "The
  tests that pin today's shapes", "Not in scope"); `p83-surface-trimmed.md`
  Do 7 and Do 8 (the second copy weighed; the measurement's form).
- `AGENTS.md` as it stands at `a6a3b698`: the build-order rule is one
  sentence now (a build before anything that starts a daemon; no build
  is owed after a `--workspace` test run, the gate's `artifacts` stage
  holds it); C1 (the edges), C47 (no `unwrap` in the store, plan,
  protocol and search crates), C116 (no id in user-facing text), C122.
- `decisions/frontends.md`: C53 as amended (the three sentences the
  owner took, at lines 11; read them whole, they are the contract this
  build meets), C149 (line 12, "Unbuilt: step 2g", "Pinned: nothing
  yet": this build names the pin), C11 (the failure document; `kind:
  "conflict"` with `revision`), C52, C115, C116, C121 (kept), C143 (a
  shared refusal speaks the request's words; the adapter adds its
  spelling), C146 (a count says what it counts; a JSON field's name is
  the contract).
- `decisions/search.md`: C98, C100 (one answer shape; a continuation
  names its basis), C118 (a row's age beside `place`).
- `decisions/pricing.md`: C69, C74, C78 (a single write returns the
  prior so a hand can undo it). `decisions/plans.md`: C44, C126 (the
  policy receipt's `league_notes`). `decisions/daemon.md`: C85 (the
  closed error kinds), C84 (one tree writes one artifact).
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets"
  (line 98 on), and its row "A sibling sentence left standing" (line
  104: the three helpers that sort refusals, named).
- `audits/readiness-2026-10/seat/calls.csv` (the sizes by call; row
  `n`'s text at `calls/NNN.json` under `~/seat-worlds/readiness-2026-10/mcp-opus/`),
  `REPORT.md` sections 2, 3, 5 and 6, `DEBRIEF.md` answers 2, 5 and 10,
  `README.md` (the pristine world and how a rerun is made: `setup
  --store pristine/store` into a fresh world).
- The makers, as the tree has them:
  - the search crate: `crates/acquisition-search/src/answer.rs`
    (`DEFAULT_LIMIT` 20 at line 341; `Next` and `RowsOut` at 1428 to
    1450: `limit`, `returned`, `after`, `left_out`, `next`), `tabs.rs`
    (`LISTED_WHOLE` 30 and `MOST_ITEMS` 10 at lines 35 and 37; `Tabs`
    with `listed`, `rows`, `left_out`, `left_out_needs`), `describe.rs`
    line 243 (the `answer` section's "20 where no limit is given"),
    `corpus.rs` line 284 (`Place`, its `fetched_at` at 316), `error.rs`
    (`Offered` at 161: `reading`, `request`, `realm`; `offering_realms`
    at 200, the reading `--realm {realm}`; `asked_by` after it;
    `to_json` at 253: `readings` from the offers' readings, `offers` as
    `{request}`); its callers `corpus.rs` 948, `answer.rs` 1651,
    `show.rs` 397.
  - the planner: `crates/acquisition-plan/src/listing.rs` (`ShowView`
    at 618: `items_here`, `items_covered_below`, each a whole
    `Vec<Listing>`), `shop.rs` (`LEFT_OUT_LISTED` 50 at 155;
    `ShopRender` at 497: `posted`, `left_out`, `left_out_total`,
    `off_page_listed`, `freshness`; `Freshness` at 441: `uncovered`,
    `stale`, `position_before_listing`, each unbounded;
    `RenderOptions::expand`), `pricing.rs` (`Refused` and `RefusedKind`
    at 71 to 83; `Refused::fields` at 100; `refused_write` at 509, which
    takes `current: Option<i64>`; `on_conflict` at 547, which holds the
    store's `current: Option<Box<AnnotationRow>>` and reduces it to the
    revision before calling `refused_write` — the design said the live
    row is in `refused_write`'s hand; it is in `on_conflict`'s;
    `SetReceipt` at 565: `write` flattened, `in_facts`; `described()`
    at 778, the `revision` and `undo` entries), `price.rs`
    (`PriceWrite` at 639: `target`, `written`, `prior`), `lib.rs`
    (`put_sync_policy` at 887: reads the stored row and refuses with
    `current: Some(Box::new(row))` on a revision mismatch; `league_notes`
    at 742).
  - the store's rows: `crates/acquisition-store/src/lib.rs`
    (`CharacterRow` at 754: `listed_at`, `fetched_at`, `item_count`,
    `fetched_items`; `Store::characters` at 2010, a bare `Vec`;
    `Store::events_since` at 2175, a bare `Vec<EventRow>` under a
    `limit`; `TabRow` at 734).
  - the verbs: `crates/acquisition-verbs/src/jobs.rs` (`Jobs` at 127:
    `Live(Vec<JobInfo>)` serialized as the bare array at 230, `Absent`
    an object), `fault.rs` (`Fault::kind` at 111, `Fault::fields` at
    138: `accounts`, `next`), `frontend.rs` (`set_policy` at 338: the
    stored row is read under `Expect::Current` alone; `PolicyWritten`
    at 391: the row flattened and `league_notes`), `events.rs`
    (`summary` at 28, a bare `Vec<LocationChanges>`), `refresh.rs`
    (`age` at 355: `45s`, `3m`, `5h`, `2d`).
  - the CLI: `crates/acquisition-cli/src/store_cmd.rs` (`ago` at 128:
    `never`, `{d}s ago`, `{m}m ago`, `{h}h ago`, `{d}d ago`;
    `characters` at 404, its JSON the bare `Vec` at 408 and its text
    footer at 455 to 475: fetched, with empty bodies, never fetched,
    the total, the items; `events` at 626, its JSON a bare `Vec` at
    642), `search_cmd.rs` (the row's place age at 724, the scope's
    oldest and newest at 922, `list_seen` at 931, `removed` at 1015,
    all through `ago`), `price_cmd.rs` (`LIST_UP_TO` 10 at 50; `show`'s
    text bound at 818; `render_write` at 956 and `undo_words` at 938,
    the printed undo), `main.rs` (`error_report` at 998, the CLI's one
    failure path; `FINISHED_LISTED` 10 at 2412; `jobs_text` at 2424
    and `summary_text` at 2543: running, waiting, done, failed,
    cancelled, the hidden finished), `web/mod.rs` (the tabs page's
    document at 528, `{"tabs", "characters"}`; the jobs page at 780 on
    `Jobs::Live` through `jobs_text`).
  - the MCP: `crates/acquisition-mcp/src/main.rs` (`failed` at 264,
    `refusal` at 281, `told` at 358, `told_as` at 371, `refused` at
    387 with its `readings = matches!(e, SearchError::Scope { .. })`,
    `Args` at 403, `PAYLOAD` at 1052, `Prose` at 1058 with its
    `readings` field, `spoken` at 1075 with the clause `(k ==
    "readings" && !prose.readings)` at 1089, `refusing` at 1208,
    `refusing_as` at 1219, `priced` at 1229; the tools `tabs` 1329,
    `characters` 1355, `item_events` 1509 with `limit.unwrap_or(200)`,
    `price_show` 1650, `shop_render` 1677, `list_jobs` 1803; the
    params `TabsParams` 466, `CharactersParams` 484, `EventsParams`
    685, `PriceShowParams` 787, `PriceListParams` 806, `ShopRenderParams`
    835). The design's "`spoken.rs`" is the test
    `crates/acquisition-mcp/tests/spoken.rs`: its `PAYLOAD` copy at
    156 and the clause `(k == "readings" && !fields.contains_key("offers"))`
    at 176 are the test-side twin of the prose-or-payload line.
  - the daemon's bound: `crates/acquisition-protocol/src/protocol.rs`
    `MAX_FRAME_BYTES`, the wire's, untouched (#56).
- `MCP-REFERENCE.md` lines 94 (`price_show`'s `limit`, "some 1.3 KB an
  item"), 168 (`shop_render`'s `expand`, "past the first 50", "some 200
  bytes an item"), 207 and 210 (`tabs`, "thirty or fewer, else the
  ten"); `CLI-REFERENCE.md` for the same verbs. Both are goldens of
  `crates/acquisition-mcp/tests/mcp_reference.rs` and
  `crates/acquisition-cli/tests/cli_reference.rs`.
- The seat's door, `.claude/skills/seat/seat-mcp.py` (its header: a
  call's `text_bytes` is the result's text block, `result_bytes` the
  whole JSON-RPC line; `--host-config NAME` starts the server as a host
  would), and `seat.py` (`setup --store DIR`): the measurement's driver.
- The tests that pin today's shapes: every test the design lists under
  "The tests that pin today's shapes" exists at `7b2e0801` under the
  name it gives (each checked), with these corrections: the web goldens
  are five files under `crates/acquisition-cli/tests/goldens/web/`
  (`console_realm`, `jobs_halted`, `moved_world`, `refresh_loop`,
  `search_pages`), held by `every_search_page_is_a_wording_golden`,
  `the_refresh_loop_runs_through_the_pages` and
  `the_jobs_page_opens_with_the_halt`, not seven; the structured copy is
  read by 24 lines across six test files and the harness
  (`grep -rn structured crates/acquisition-mcp/tests/`), not fifteen; a
  request with `limit: 20` stands in `crates/acquisition-search/tests/rows.rs`
  alone, not in `explain.rs`. The goldens that print "ago" are 203
  lines across 28 files under `crates/acquisition-cli/tests/goldens/`;
  text goldens do not change under this step (the text already prints
  words), and a golden changes only where it holds JSON that gains a
  field or a wrapped array.

## Input

Fixed before the work starts; nothing here is measured by the build
except where the line says so.

- The failing inputs the tests stage, each the readiness seat's call
  by number (`calls.csv`; text and whole bytes):
  - call 11, `characters` {}: 65 rows, a bare array, no counts; 15,805
    text, 33,671 whole. The test: a `characters` answer is one object
    whose counts are the CLI footer's (the total, fetched, with empty
    bodies, never fetched, items) and whose rows hold ten with
    `left_out`, more on `limit`, all on `expand`.
  - call 27, `price_show` of a tab of 39 items: 28,857 text, 62,615
    whole; every item whole under `items_here`. The test: each of the
    two sets is counted and lists ten.
  - call 35, `shop_render` {}: 34,779 text, 75,401 whole, for a
    2,800-character page: 50 `left_out` entries and 38 ids under
    `freshness` before the page's `text`. The test: `posted`,
    `left_out` and each list of `freshness` hold ten and are counted;
    the pages are whole.
  - calls 2, 8, 18 (`accounts` 237, `search` 1,465, `show` 3,547 text):
    `last_login`, `oldest_fetch` and `newest_fetch`, `first_seen` as
    epochs alone. The test: each epoch the text turns into an age has
    its words beside it.
  - call 46, `tabs` with `GERWARIK#7694`: `not_logged_in` with
    `accounts` (175 text, 476 whole). The test: the refusal carries the
    offer that is the same call with each known name; the kind stays
    (C121).
  - call 47, `price_show` of `tab/pc/ffffffffff`: `not_found` with no
    `offers` and no `next` (184 text, 486 whole). The test: the refusal
    carries offers naming `search` and `tabs` as requests.
  - call 49, `price_set` at a stale `if_revision`: `conflict` with
    `revision: 4` and no current value (158 text, 436 whole). The test:
    `current` stands beside `revision`, the row as `price_show`'s
    `manual` prints it.
  - calls 28 to 31, 36, 37 (`price_set`, `price_clear`): receipts
    `target`, `written`, `prior`, `in_facts`; a clear's `written` null
    and its tombstone's revision nowhere. The test: every receipt
    carries `revision` and `undo`; a clear's `revision` is the
    tombstone's.
  - call 14, `search` with `limit: 30` of 187: the audit view already
    works on `limit`; stays true.
- The sizes measured at #83 (`d2a588bc`): a result on the wire is a
  little over twice its document (`search` {} 7,431 text, 16,126
  whole; `describe` ["trade"] 24,359 text, 51,396 whole), because
  `told_as` sends the document as the text block and again as
  structured content.
- The host measurement Q2 asks (Do 10): what Claude Code, the host the
  seats run under, hands the model of a result that carries both.
  Fixed: the method, a headless `claude -p` run with `--mcp-config`
  naming a file under your scratch (the repository's configuration
  untouched) and `--output-format stream-json --verbose`, whose
  `tool_result` event is what the model was handed; the server
  `target/debug/acq-mcp` in a mock world under your scratch; one call,
  `characters` {} over the pristine world's store. The build measures
  the bytes.
- The before measurement: the three calls above over
  `~/seat-worlds/readiness-2026-10/pristine/` (the owner's items as the
  mock serves them; the store is read, never written: a rerun is `setup
  --store pristine/store` into a fresh world under your scratch), by
  `seat-mcp.py` `call`, HEAD's `acq-mcp` copied aside before the first
  edit, alone on the machine.
- The bound's number: ten, C53's own, held once. Its home: the store
  crate, the one crate every maker links (C1's edges: the search links
  the store and the planner, not the protocol crate; the planner links
  the protocol crate and the store; the verbs link all of them; the
  store links nothing of the workspace). A constant and the one helper
  that applies it live there; the store itself cuts nothing (a store
  read returns what was asked for, bounded by its caller as today).

## Do

Numbered, in order, one commit per concern. Each agent-facing decision
(a field's name, a count's words, an offer's shape, the undo's shape)
is recorded in the commit that builds it and listed in the report for
the close. A reference (`MCP-REFERENCE.md`, `CLI-REFERENCE.md`) is
regenerated in the commit that changes its source, under
`ACQ_UPDATE_FIXTURES=1` on the one test named in a shell where the
variable is otherwise unset, and every changed line is read as an
agent would and explained in that commit's message. The CLI's text
stays a function of the JSON (C53) and prints what it prints today
unless a line below says otherwise; where the JSON gains a count the
text already prints, the text reads it from the document.

1. **The bound's home.** In the store crate one `pub const` holding
   ten (the build names it; the doc comment headed by C53 says what it
   is: the number of like entities a list holds by default on every
   surface) and one helper that takes a list and an asked bound
   (`limit` named, or `expand` meaning the whole) and gives back the
   rows kept and the count left out, so every maker applies one
   arithmetic and no maker owns a second number. A unit test in the
   store crate pins the helper (ten by default; `limit` names more or
   fewer, zero included; `expand` is the whole; `left_out` is the
   difference). The CLI's `LIST_UP_TO` (`price_cmd.rs` 50) and
   `FINISHED_LISTED` (`main.rs` 2412) and the search's `MOST_ITEMS`
   (`tabs.rs` 37) become readings of that constant in the commits that
   touch them, never a second literal. Nothing is cut yet; this commit
   adds the home and its test.

2. **The search crate under the bound.** `DEFAULT_LIMIT` (`answer.rs`
   341) becomes the store's constant: the default moves from 20 to
   ten, a default and not a shape (`RowsOut` already carries `limit`,
   `returned`, `after`, `left_out`, `next`). `tabs` folds its "thirty
   or fewer whole, else the ten with the most items" into the one
   rule: `LISTED_WHOLE` goes; `rows` holds ten with the most items by
   default, `limit` names more, `expand` the whole; `listed`, `left_out`
   and `left_out_needs` stand as they are (C53, additive; C146, a
   field's name is the contract). The `answer` section's sentence at
   `describe.rs` 243 says ten. The tests named under "The tests that
   pin today's shapes" for the search crate change only where they
   read the number (`rows.rs`'s `limit: 20`; `tabs_counts_first_and_lists_the_ten_with_the_most_items`
   and `tabs_says_its_counts_first_and_counts_a_large_league` read the
   ten over a league of thirty or fewer too); `c100_a_rows_evidence_is_bounded_and_says_what_it_left_out`
   pins the default at the store's constant, never at a literal. The
   MCP's `tabs` description and `TabsParams::expand` doc, and
   `MCP-REFERENCE.md` 207 and 210, say the one rule; `surface_size.rs`
   holds. Both references regenerated here.

3. **The planner's makers under the bound.** `ShowView`'s two sets
   (`listing.rs` 625, 629): `items_here` and `items_covered_below`
   each list ten, with a count beside each in the search answer's
   words (the build names the two count fields, one per set, after
   `total` and `left_out`; the field names recorded in the message),
   `limit` on `price_show` (the MCP's `PriceShowParams` and the CLI's
   flag) naming more and `expand` the whole; the description says
   `price_list` with `in` is the route to the rest. `ShopRender`
   (`shop.rs` 497): `posted` and `left_out` hold ten (`LEFT_OUT_LISTED`
   50 retired for the constant; `left_out_total` stands and `posted`
   gains its count beside it in the same words), and each list of
   `Freshness` (`uncovered`, `stale`, `position_before_listing`) holds
   ten and is counted; `expand` means every list whole; the pages'
   `text` is what the answer is for and is never cut (C53). `price_list`'s
   default `limit` is the constant where it has none today. The CLI's
   text reads the counts from the document: `show`'s bound at
   `price_cmd.rs` 818 and the shop's at `shop_cmd.rs` read the
   constant, and the text that defers a long set to `list` stands.
   The planner's own tests pin each shape (`c74_every_item_is_posted_omitted_blocked_or_off_the_page_and_counted`,
   `c53_at_real_scale_the_report_round_trips_and_every_container_shows_its_two_sets`
   extended, never renamed); the CLI's `price_json.rs` and
   `shop_json.rs` tests and the MCP's `pricing_tools.rs` tests read
   the shape alone (C122). `MCP-REFERENCE.md` 94 and 168 re-read:
   a size a description states ("some 1.3 KB an item", "some 200
   bytes an item", "past the first 50") is remeasured or dropped.
   Both references regenerated here.

4. **C149: no bare array; the verbs' three documents counted.** One
   document per bare array, made in the verbs crate so both frontends
   and the web read one maker: `characters` (the rows under one key
   with the counts the CLI's footer prints at `store_cmd.rs` 455 to
   475: the total, fetched, with empty bodies, never fetched, the
   items; the rows ten by default with `limit` and `expand`, as Do 1's
   helper gives them); `item_events` / `acq events` (the total in the
   window and the rows returned; the MCP's `limit.unwrap_or(200)` and
   the CLI's `events` limit read the constant as the default, `limit`
   names more; the CLI's `--json` of the summary rows, `events.rs`
   `summary`'s `Vec<LocationChanges>`, is wrapped the same way);
   `list_jobs` / `acq jobs` (`Jobs::Live` serialized as an object:
   the counts `summary_text` prints — running, waiting, done, failed,
   cancelled — beside the rows, the finished past the listed ones
   counted as `jobs_text` counts them; `Jobs::Absent` stands). The
   counts' words follow C146 (each says what it counts, the same words
   on every surface, the `describe` entry the tool's description names
   in one line; the CLI's text reads them from the document, so the
   footer is a function of the JSON). The pin C149 lacks: one test at
   the verbs crate (`documents.rs`) that every document a verb returns
   serializes to a JSON object, and one at each frontend that reads
   the shape (`characters_query_lists_live_rows_with_item_counts` in
   the store stays the store's; `jobs_lists_what_is_open_and_counts_the_finished_it_leaves_out`,
   `item_events_reads_back_its_hours_and_defaults_to_a_day` read the
   object). `acq web` renders the verbs' documents: the tabs page
   (`web/mod.rs` 517 to 528, `characters` in its JSON document) and
   the jobs page (780 on, `jobs_text`) are rebuilt with them, and the
   goldens under `tests/goldens/web/` are the check; a golden
   regenerates only where a changed line is read and explained.
   `decisions/frontends.md` C149's "Pinned: nothing yet" becomes the
   test's name and "Unbuilt: step 2g" goes, in this commit (the
   registry's pin line is the build's to fill; the ruling's words are
   not touched). Both references regenerated here.

5. **Ages in words beside epochs, from one function.** One function
   for the vocabulary the CLI prints today (`ago` at `store_cmd.rs`
   128: `never`, `{n}s ago`, `{n}m ago`, `{n}h ago`, `{n}d ago`), at
   the lowest crate the text and the JSON share (the store, beside
   `now()`; the verbs' `age` at `refresh.rs` 355 and the CLI's `ago`
   become readings of it, the plan's `45s` form kept where a plan's
   text uses it, by one argument or one wrapper, never a second
   table). The rule, one sentence: an age the text prints, the JSON
   carries in words beside the epoch it is made from, made at the
   answer's own time, under a sibling key the build names once and
   uses everywhere (recorded in the message). Applied where the text
   prints an age today: a `characters` row's `fetched_at` and
   `listed_at`; a search row's `place.fetched_at` (`search_cmd.rs`
   724; C118); the scope's `oldest_fetch` and `newest_fetch` (922)
   and `list_seen` (931); a row's `removed` (1015); `accounts`'
   `last_login`; `show`'s `first_seen`; a tab's age in `tabs`. Pinned
   once at the store (the vocabulary) and once per maker that gains
   the words, the frontends' tests reading the shape
   (`a_rows_place_carries_when_its_location_was_fetched` reads the
   words beside the epoch). A golden holding JSON regenerates on its
   named test alone.

6. **The refusal shape (Q5, delegated).** One document on every
   surface, in the CLI's `--json` as on the MCP: `error`, the words
   (C116, C143); `kind`, one word from the closed lists as they stand
   (the protocol's, the verbs' `Fault::kind`, the planner's `Refused`,
   the search's), no new word; and at most four optional keys, each
   meaning one thing wherever it stands, absent when it has nothing
   to say, never null: `offers`, requests an agent sends back as
   printed, each `{request, says}` and naming `tool` when it is
   another tool's; `readings`, how what was typed could be read,
   payload on every refusal; `next`, one sentence or one command that
   is a person's act, never an agent's; and the kind's own fields
   (`accounts`; `revision` with `current`, the row as `price_show`'s
   `manual` prints it). Built as:
   - the search crate: `Offered` for a realm (`error.rs` 200) reads in
     the request's own words (C143), the terminal's `--realm pc` added
     by the CLI's adapter after them as `failed` adds a flag's name;
     with that, a scope refusal's `readings` are payload like a
     language refusal's, and the one place the spelling depended on
     the error type goes: `Prose::readings` and its clause in
     `spoken`'s `payload_key` (`main.rs` 1089), `refused`'s `matches!`
     (387), `refusing_as` where nothing else calls it, and the test
     sweep's `contains_key("offers")` clause (`tests/spoken.rs` 176)
     with the header's sentence at its line 20. `to_json`'s `offers`
     gain `says` (the reading) beside `request`; `readings` stay.
   - `not_found` (`Refused::fields`, `pricing.rs` 100) carries offers:
     a `search` and a `tabs` request that would find the target, as
     requests (the MCP's tool named; the CLI prints each as the
     command it is).
   - `not_logged_in` with names known (`Fault::fields`, `fault.rs`
     138, `UnknownAccount`): the offers are the same call with each
     known name; the kind stays (C121, kept; the design's Q5 text:
     "no login made a store for `GERWARIK#7694`; the names known:
     GERWARIC#7694" is the `error`'s shape).
   - `conflict`: `current` beside `revision`, the row `on_conflict`
     (`pricing.rs` 547) has in hand before it reduces it to the
     revision — pass the row through to `refused_write`, whose
     tombstone case has no row and says so by `current`'s absence.
   - the sorters: the MCP's `refusal`, `refused` and `priced` and the
     CLI's `error_report` (`main.rs` 998) produce this shape from one
     function at the lowest crate that sees every kind (the verbs',
     since the search's and the planner's refusals both reach it; the
     build names it), so no tool decides alone and no sibling sentence
     is left standing (`REVIEW-CHECKLIST.md` line 104).
   Pinned once at the verbs (the shape over every kind, the four keys
   absent-never-null) and once per source of a new field (the search's
   `says`, the planner's `current` and the `not_found` offers), the
   frontends' refusal tests under "The tests that pin today's shapes"
   reading the shape and keeping their names and fact words;
   `every_text_the_server_sends_spells_the_cli_it_speaks` sweeps
   `readings` as payload now. `MCP-REFERENCE.md`'s instructions line on
   refusals (`readings` and `offers`) re-read; both references
   regenerated here.

7. **The receipt shape (Q6, delegated).** Every write (`price_set`,
   `price_clear`, `set_sync_policy`) answers one shape: `target` (a
   policy write's: the account and the realms); `written`, the row
   now standing, null after a clear; `prior`, the row before, null on
   a create; `revision`, the revision now current, which the next
   write on this target names (a clear's is its tombstone's, read as
   `refused_write` reads it through `Annotations::cleared`, so
   `written: null` stands and the number is there); `undo`, the
   request that restores `prior`, an offer in Do 6's shape (the CLI
   prints it as the command `render_write` prints today, which runs
   as printed; the MCP sends it as a request naming its tool;
   `undo_words`' rule decides whether there is one, and a prior this
   build cannot read has no `undo` and says why in `says`); and the
   write's own facts (`in_facts` on a price write; `league_notes` on a
   policy write, C126). Additive on both surfaces. For the policy
   write, verify first whether the prior row is at hand: `set_policy`
   (`frontend.rs` 338) reads the stored row under `Expect::Current`
   alone, and `put_sync_policy` (`lib.rs` 887) reads it for the
   compare-and-swap and returns the new row; if the prior can be
   carried out of that read without a second read, `PolicyWritten`
   gains `prior`; if not, `prior` is a price receipt's alone and the
   report says so. The `describe` pricing entries `revision` and
   `undo` (`pricing.rs` 778 on) say the shape in the same words. A
   write naming a container's revision is refused as a conflict
   naming the target's own revision or none (today's behaviour; a
   test pins it as the teaching refusal). Pinned once at the planner
   (`c78_a_write_returns_the_prior_value_so_a_hand_can_undo_it`
   extended to `revision` and `undo`) and once at the verbs for the
   policy receipt; the receipt tests under "The tests that pin today's
   shapes" read the shape (`every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known`
   runs the printed `undo` as before). Both references regenerated
   here.

8. **The descriptions and `describe` entries re-read.** Every tool
   description, parameter doc, `describe` entry and README line that
   states a size, a number or a list rule (`DEFAULT_LIMIT`'s 20, the
   shop's 50, `tabs`' thirty, `item_events`' 200, "some 1.3 KB an
   item") says the one rule or nothing; `surface_size.rs`'s bounds
   hold. Folded into the commits above where the source changed; this
   item is the sweep that finds what they missed, by grepping the
   references and `describe`'s text for each old number, in a commit
   of its own only if the sweep finds something.

9. **The web pages.** `acq web` renders the verbs' documents and is
   rebuilt with them in Do 4 and Do 6 (the web's failure page,
   `web/mod.rs` 322, reads the one refusal shape); the goldens under
   `tests/goldens/web/` and `readme_tour.rs` are the check, and no
   page-side rule is added (Q7: the offers and
   `undo` as requests are the one phrasing every renderer reads, so
   4b's "web phrasing for the next actions" under "Open" is answered
   by the verb change and the close says so). Nothing here is a
   commit of its own unless a page's golden changes for a reason the
   commits above did not explain.

10. **The second copy, measured once on the host at hand (Q2,
    delegated).** By the method under Input: a headless Claude Code
    run against `target/debug/acq-mcp` in a mock world, one
    `characters` call, the `tool_result` event's content read whole.
    The rule for what it decides: if the host hands the model both the
    text block and the structured content (the content's bytes near
    the result's whole, or the document twice in it), the structured
    copy goes — `told_as` sends the text block alone,
    `CallToolResult::structured` is not used, and the 24 lines that
    read `structuredContent` in the MCP's tests and harness read the
    text, one line each; if the host hands the text block alone (the
    content's bytes near the text's), both copies stay, the copy being
    the host's and costing the model nothing, and the measurement is
    stated as the reason. Either way the measurement is in the commit
    message (the text's bytes, the whole result's, the content handed,
    the host's version), and a decision to drop the copy is its own
    commit. No configuration of the repository or of your home changes
    for this: the `--mcp-config` file lives under your scratch.

11. **The measurement, before and after, and the close's inputs.** The
    three calls (11, 27, 35) through `seat-mcp.py` over a fresh world
    made from the pristine store, HEAD's `acq-mcp` copied aside before
    the first edit and the step's last `acq-mcp` after, alone on the
    machine (no gate, build or driver beside it; the before-run before
    the first edit): text and whole bytes per call, before and after,
    in the last build commit's message as a table, with one line
    stating what the three cost together before and after. The last
    build commit carries the step's ledger line and a "Current step"
    that says the review is pending; the report lists the agent-facing
    decisions (every field name added, the counts' words, the offer and
    undo shapes, the age key) for the close.

## Done when

- A constant in the store crate holds the bound and one helper applies
  it, pinned by a unit test there; no maker holds a second literal for
  a list's default (`DEFAULT_LIMIT`, `LEFT_OUT_LISTED`, `LISTED_WHOLE`,
  `MOST_ITEMS`, `LIST_UP_TO`, `FINISHED_LISTED` and the `200` of
  `item_events` are gone or readings of the constant; the commit
  message names each).
- `search` lists ten by default; `tabs` lists ten with `limit` and
  `expand` and no thirty; a container's `price_show` lists ten of each
  set and counts both; `shop_render`'s `posted`, `left_out` and each
  `freshness` list hold ten and are counted, the pages whole;
  `price_list` defaults to ten; each pinned at its own crate and read
  by the frontends' tests, no test deleted and none renamed.
- No tool and no `--json` prints a bare array: `characters`,
  `item_events` / `events`, `list_jobs` / `jobs` answer objects with
  the counts the CLI's text prints, pinned at the verbs crate and
  named in C149's "Pinned" line in `decisions/frontends.md`.
- Every epoch the text turns into an age has its words beside it in
  the JSON, from one function in the store crate; the CLI's `ago` and
  the verbs' `age` are readings of it.
- Every refusal on both surfaces is `error`, `kind` and at most
  `offers` (`{request, says, tool?}`), `readings` (payload), `next`
  and the kind's own fields, absent never null; `not_found` and
  `not_logged_in` carry offers; `conflict` carries `current`; the
  scope refusal's readings speak the request's words in the search
  crate with the CLI's flag added by the adapter; `Prose::readings`
  and its clauses in `spoken` and `tests/spoken.rs` are gone; the
  refusal tests named under Read keep their names and fact words.
- Every receipt carries `revision` and `undo` (an offer), a clear's
  `revision` its tombstone's; the printed `undo` runs as printed on
  the CLI and the MCP's is a request; the policy receipt's `prior`
  built or the report says why not.
- `MCP-REFERENCE.md` and `CLI-REFERENCE.md` regenerated in each commit
  that changed their source, every changed line explained in that
  commit's message; no description states a number the one rule
  retired.
- The web goldens green or regenerated with each changed line
  explained; `readme_tour.rs` green.
- The host measurement is in a commit message with the rule's outcome,
  and the structured copy is gone or kept by that outcome.
- The last build commit's message carries the before-and-after table
  for calls 11, 27 and 35 (text and whole bytes), the step's ledger
  line and "Current step" saying the review is pending.
- The gate green on the step's own tree (`tools/gate.sh`'s exit
  status), the stage table in the report.

## Not in scope

- A new kind in the closed set (`unknown_account` or any other; C121
  kept, the owner's to reopen), or a kind standing on one surface.
- Renaming a JSON field that stands today (C53, additive): `listed`,
  `left_out_total`, `by_type`, `items_here`, `items_covered_below`
  stay; a field added takes the search answer's words.
- A filter per tool for what a count names (`characters` by
  `never_fetched`, `price_list` by `invalid_notes` or `residue`): #11
  holds the pricing ones; a filter waits for the runner who asks
  twice.
- `store_refused`, the refused bodies listed (#18); the invalid notes
  named and `basis.intent` explained (#11); the daemon's `List` and
  `Quote` over the frame (#56, a ruling); the first turn's schemas
  (#83's "Open"); `realm` as a counts key (the seat's fault 10, the
  search's if a second runner asks).
- Rows carrying fields, the cards and the dense table beyond the
  window (4-full; `view.rows.fields` and the `search` description
  naming it stay for the web UI); the time seam (4-0).
- A second rendering of a result on the MCP (C53, no second rendering).
- A schema for any document; the seat skill's wording and
  `SEAT-LEDGER.md` (the owner's); the next seat's brief (the seat
  skill's, written by the session that sits it, pointed at these
  shapes by the close).

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `2g`, every `ACQ_STORE_DIR` under it, every
`--mcp-config` file and every seat world made for the measurement
under it; the pristine store under `~/seat-worlds/` is read and never
written; commit on `main`, never push; the `Co-Authored-By` line for
your model last; long commands as tracked background tasks;
`tools/gate.sh`, and `cargo build --workspace` before any daemon (C84,
the artifact mismatch; no build is owed after a `--workspace` test
run); a measurement runs alone on the machine, with no gate, build or
other driver beside it, and the before-run is taken from HEAD's binary
copied aside before the first edit; an edit made for a measurement is
reverted before the next and never committed; regenerate a golden or
a reference only with `ACQ_UPDATE_FIXTURES=1` on the one test named,
in a shell where the variable is otherwise unset; a fix round's
done-when is the template's.

## Report

`TEMPLATE.md`; item 3 carries the three calls' sizes before and after
and the host measurement; item 1 lists the agent-facing decisions for
the close beside the commits.
