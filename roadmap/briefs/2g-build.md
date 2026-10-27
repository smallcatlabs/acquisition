# 2g — The read economy and the refusal and receipt bundle: the build brief, in rounds

Drawn from `2g-design.md` (`e11768e2`) after the owner ruled on it in
full (2026-10-27, "yes to all", at `ea0b0255`): C53 amended three ways
(Q1 the bound of ten rows beside counts with `limit` and `expand`, held
once; Q2 on the MCP the JSON is the text and no second rendering; Q4
ages in words beside epochs), C149 new (every answer and refusal one
JSON object, lists counted beside rows, the one non-additive change),
C121 kept (no `unknown_account`). Q5's refusal shape and Q6's receipt
shape are delegated to this build, recorded in the commits that build
them and listed for the close; Q2's second copy is this build's one
host measurement; Q7's constraints bind. Issues #9 and #10.

Re-cut from the cold review at `roadmap/reviews/2g-brief-3691766e.md`
(the first draft stays in history at `3691766e`): the one Do list is
now eight rounds, each one concern, gated, committed and reviewed on
its own, in an order where no later round is needed by an earlier one,
so the owner can stop after any round with the tree consistent. Each
of the review's sixteen findings is taken into a round or named as not
taken, under "The review's findings, taken into the rounds". Every
citation below was checked against the tree at `1a3ec2d6`; where the
review's or the design's memory of the tree differs, the tree's line
stands here and the difference is named in place.

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
  #82, #83 and #84; under "Open", "For 4-full, from 4b" (the web
  phrasing for the next actions the pages print as command lines; this
  step does not close it, round 4 says why).
- `roadmap/reviews/2g-brief-3691766e.md` whole: the sixteen findings
  this cut answers and the consumer inventory round 3 carries.
- `roadmap/briefs/TEMPLATE.md` (the form; a fix round's done-when);
  `roadmap/briefs/2g-design.md` whole (the source: Q1 to Q8, "The
  tests that pin today's shapes", "Not in scope"); `p83-surface-trimmed.md`
  Do 7 and Do 8 (the second copy weighed; the measurement's form).
- `AGENTS.md`: the build-order rule is one sentence (a build before
  anything that starts a daemon; the gate's `artifacts` stage holds the
  rest); C1 (the edges), C47 (no `unwrap` in the store, plan, protocol
  and search crates), C116 (no id in user-facing text), C122.
- `decisions/frontends.md`: C53 as amended (line 11; read it whole, it
  is the contract every round meets), C149 (line 12, "Unbuilt: step
  2g", "Pinned: nothing yet": round 3 names the pin), C11 (the failure
  document; `kind: "conflict"` with `revision` as a field, null when no
  row was ever written), C52, C115, C116, C121 (kept), C143 (a shared
  refusal speaks the request's words; the adapter adds its spelling),
  C146 (a count says what it counts; a JSON field's name is the
  contract).
- `decisions/search.md`: C98, C100 (one answer shape; a continuation
  names its basis), C118 (a row's age beside `place`).
- `decisions/pricing.md`: C69, C74, C78 (a single write returns the
  prior so a hand can undo it). `decisions/plans.md`: C44, C126 (the
  policy receipt's `league_notes`). `decisions/daemon.md`: C85 (the
  closed error kinds), C84 (one tree writes one artifact).
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets",
  its rows "A sibling sentence left standing" (the three helpers that
  sort refusals, named), "A document unbounded by default" and "A
  printed command that does not run as printed".
- `audits/readiness-2026-10/seat/calls.csv` (the sizes by call; rows
  28, 29 and 31 are the price writes that precede call 35, cleared at
  call 51), `REPORT.md` sections 2, 3, 5 and 6, `DEBRIEF.md` answers 2,
  5 and 10, `README.md` (the pristine world and how a rerun is made:
  `setup --store pristine/store` into a fresh world).
- The makers, as the tree has them:
  - the search crate: `crates/acquisition-search/src/answer.rs`
    (`DEFAULT_LIMIT` 20 at line 341; `LISTED` 10 and `SUGGESTED` 5 at
    344 and 345; `Next` and `RowsOut` at 1428 to 1450: `limit`,
    `returned`, `after`, `left_out`, `next`), `tabs.rs` (`LISTED_WHOLE`
    30 and `MOST_ITEMS` 10 at lines 35 and 37; `Tabs` at 41 with
    `by_type`, `listed`, `rows`, `left_out`, `left_out_needs`),
    `describe.rs` line 243 (the `answer` section's "20 where no limit
    is given"), `corpus.rs` line 284 (`Place`, its `fetched_at` at
    316), `error.rs` (`Offered` at 161: `reading`, `request`, `realm`;
    `offering_realms` at 200, the reading `--realm {realm}`; `to_json`
    at 253: `readings` from the offers' readings, `offers` as
    `{request}`).
  - the planner: `crates/acquisition-plan/src/listing.rs`
    (`RowAccounting` at 482: `unmatched`, `unreadable`, `other_league`,
    each a whole `Vec`; `ShowView` at 618: `items_here`,
    `items_covered_below`, each a whole `Vec<Listing>`; `list_view` at
    686, whose default leaves out the items nothing applies to),
    `shop.rs` (`LEFT_OUT_LISTED` 50 at 155; `ShopRender` at 497:
    `posted`, `left_out`, `left_out_total`, `off_page_listed`,
    `freshness`; `Freshness` at 441: `uncovered`, `stale`,
    `position_before_listing`, each unbounded; `RenderOptions::expand`),
    `pricing.rs` (`Refused` at 71, `kind` and `message` alone;
    `RefusedKind` at 78, `Conflict { revision: Option<i64> }` and
    `NotFound` carrying no target; `Refused::fields` at 100, which
    writes `revision: null` when no row was ever written (C11);
    `refused_write` at 509, which takes `current: Option<i64>` and reads
    the tombstone through `Annotations::cleared`; `on_conflict` at 547,
    which holds the store's `current: Option<Box<AnnotationRow>>` and
    reduces it to the revision; `SetReceipt` at 565; `described()` at
    778, the `revision` and `undo` entries at 798 and 802), `price.rs`
    (`PriceWrite` at 639: `target`, `written`, `prior`; the clear at
    763 calling `Annotations::delete`), `lib.rs` (`put_sync_policy` at
    887: under `Merge` it reads the stored row and refuses with
    `current: Some(Box::new(row))` on a mismatch; under `Replace` it
    reads nothing before the put; `league_notes` at 742).
  - the store: `crates/acquisition-store/src/lib.rs` (`CharacterRow`
    at 754: `listed_at`, `fetched_at`, `item_count`, `fetched_items`;
    `Store::characters` at 2010, a bare `Vec`; `Store::events_since` at
    2175, rows under an SQL `LIMIT` and no count of the window; `TabRow`
    at 734; `refused_list`), `annotations.rs` (`put` at 880: creating
    over a tombstone continues its revision sequence and names no
    revision; `delete` at 1006 returns `()`; `cleared` at 1070).
  - the verbs: `crates/acquisition-verbs/src/jobs.rs` (`Jobs` at 127:
    `Live(Vec<JobInfo>)` serialized as the bare array at 230, `Absent`
    an object; `FAILED_CHILDREN_DETAILED` 10 at 167 with the tally's
    ids whole, C23), `fault.rs` (`UnknownAccount { selector, known }`
    at 40; `Fault::kind` at 111; `Fault::fields` at 138: `accounts`,
    `next`), `frontend.rs` (`Accounts` at 95, an object with `provider`
    and `accounts`; `set_policy` at 338: the stored row is read under
    `Expect::Current` alone; `PolicyWritten` at 391: the row flattened
    and `league_notes`), `events.rs` (`summary` at 28, a bare
    `Vec<LocationChanges>`), `refresh.rs` (`age` at 355: `45s`, `3m`,
    `5h`, `2d`; the verbs', not the plan's as the first draft said).
  - the CLI: `crates/acquisition-cli/src/store_cmd.rs` (`accounts` at
    93, its `--json` the bare `accounts.accounts` array at 96 while the
    MCP's `accounts` answers the object; `ago` at 128: `never`, `{n}s
    ago`, `{n}m ago`, `{n}h ago`, `{n}d ago`; `characters` at 404, its
    JSON the bare `Vec` at 408 and its text footer at 455 to 475;
    `refused` at 575, a bare array under `limit`; `events` at 626: its
    `--json` a bare `Vec` of summary rows at 642 or of events at 707,
    its text at 663 counting `ev.len()` and saying "(limit reached)"),
    `search_cmd.rs` (the row's place age at 724, the scope's oldest and
    newest at 922, `list_seen` at 931, `removed` at 1015 and 1896;
    `shown_text` at 1882 prints no `first_seen`), `price_cmd.rs`
    (`LIST_UP_TO` 10 at 50; `show`'s text at 790 to 840: each set's
    relation breakdown from the whole set at 799, the deferral decided
    from the set's size at 818, the `next` line naming `acq price list
    --in` at 834; `undo_words` at 938, `render_write` at 956 whose
    clear case at 1002 says a create names no revision), `shop_cmd.rs`
    (`LIST_UP_TO` 10 at 37, a second copy), `plan_cmd.rs` (ages at
    156, 232, 248, 1656, 1684), `main.rs` (ages at 112, 131, 1938;
    `PolicyCmd` at 759: `Show` and `Set`, no clear; `error_report` at
    998, the CLI's one failure path; `Cmd::Jobs` at 1493; `watch_jobs`
    at 2113, each snapshot printed as a bare array at 2116;
    `FINISHED_LISTED` 10 at 2412; `jobs_text` at 2424 listing every
    open job and ten finished; `summary_text` at 2543), `dash.rs`
    (`fmt_ago` at 763 over seconds the daemon reports: a tty view with
    no JSON, outside round 6), `web/mod.rs` (startup resolves the
    account at 153 before any page is served; the refresh page's
    document at 528, `{"tabs", "characters"}`, and its `no_tabs_text`
    prose at 518; the jobs page at 780 on `Jobs::Live` through
    `jobs_text` at 808; the failure page at 322).
  - the MCP: `crates/acquisition-mcp/src/main.rs` (`failed` at 264,
    `refusal` at 281, `told` at 358, `told_as` at 371, `refused` at 387
    with its `readings = matches!(e, SearchError::Scope { .. })`, `Args`
    at 403, `TabsParams` at 466 with `expand` and no `limit`,
    `CharactersParams` 484, `EventsParams` 685 with `limit` "default
    200", `PriceStatusParams` 776 with no `expand`, `PriceShowParams`
    787 with neither `limit` nor `expand` (the first draft's citation
    of `MCP-REFERENCE.md` 94 was `price_list`'s `limit`, the review's
    finding 16), `PriceListParams` 806, `ShopRenderParams` 835 with
    `expand`; `PAYLOAD` at 1052, `Prose` at 1058 with its `readings`
    field, `spoken` at 1075 with the clause `(k == "readings" &&
    !prose.readings)` at 1089, `refusing` at 1208, `refusing_as` at
    1219, `priced` at 1229; the tools `tabs` 1329, `characters` 1355,
    `item_events` 1509 with `limit.unwrap_or(200)`, `price_show` 1650,
    `shop_render` 1677, `list_jobs` 1803 taking no arguments). The test
    `crates/acquisition-mcp/tests/spoken.rs`: its `PAYLOAD` copy at 156
    and the clause `(k == "readings" && !fields.contains_key("offers"))`
    at 176 are the test-side twin of the prose-or-payload line; its
    assertion at 231 that the text and the structured content are one
    document is the one read of the structured copy that is a contract,
    not a convenience.
  - the drivers that read these answers, under `tools/` and the seat
    skill: `tracer-rung.sh` (351 to 382 `accounts --json` through jq
    and python; 692 `jobs --json` saved; 760 `store characters --json`;
    797 `store events --json` with `jq 'length'`), `persist-check.sh`
    (149 `accounts --json`; 190 and 216 `jobs --json` through `.[]`),
    `.claude/skills/seat/seat-mcp.py` 275 and `seat.py` 283 (opaque
    text; no migration).
  - the daemon's bound: `crates/acquisition-protocol/src/protocol.rs`
    `MAX_FRAME_BYTES`, the wire's, untouched (#56).
- `MCP-REFERENCE.md` lines 94 (`price_list`'s `limit`, "some 1.3 KB an
  item"), 168 (`shop_render`'s `expand`, "past the first 50", "some 200
  bytes an item"), 207 and 210 (`tabs`, "thirty or fewer, else the
  ten"); `CLI-REFERENCE.md` for the same verbs. Both are goldens of
  `crates/acquisition-mcp/tests/mcp_reference.rs` and
  `crates/acquisition-cli/tests/cli_reference.rs`.
- The seat's door, `.claude/skills/seat/seat-mcp.py` (its header: a
  call's `text_bytes` is the result's text block, `result_bytes` the
  whole JSON-RPC line; `--host-config NAME` starts the server as a host
  would), and `seat.py` (`setup --store DIR`): round 8's driver.
- The tests that pin today's shapes: every test the design lists under
  "The tests that pin today's shapes" exists at `1a3ec2d6` under the
  name it gives (each checked), with these corrections: the web goldens
  are five files under `crates/acquisition-cli/tests/goldens/web/`
  (`console_realm`, `jobs_halted`, `moved_world`, `refresh_loop`,
  `search_pages`), held by `every_search_page_is_a_wording_golden`,
  `the_refresh_loop_runs_through_the_pages` and
  `the_jobs_page_opens_with_the_halt`, not seven; the structured copy is
  read by 24 lines across six test files and the harness
  (`grep -rn structured crates/acquisition-mcp/tests/`), not fifteen; a
  request with `limit: 20` stands in `crates/acquisition-search/tests/rows.rs`
  alone, not in `explain.rs`;
  `c100_a_rows_evidence_is_bounded_and_says_what_it_left_out`
  (`surface_faults.rs` 165) pins a row's per-term evidence over forty
  matching lines of one item, not the rows default (the review's
  finding 14; round 1 adds the test that does). The goldens that print
  "ago" are 203 lines across 28 files under
  `crates/acquisition-cli/tests/goldens/`; text goldens do not change
  under round 6 (the text already prints words), and a golden changes
  only where it holds JSON that gains a field or a wrapped array, or
  where a round names a text change.

## Input

Fixed before the work starts; nothing here is measured by the build
except where a round says so.

- The failing inputs the tests stage, each the readiness seat's call
  by number (`calls.csv`; text and whole bytes), and the round that
  stages it:
  - call 11, `characters` {}: 65 rows, a bare array, no counts; 15,805
    text, 33,671 whole. Round 3: a `characters` answer is one object
    whose counts are the CLI footer's (the total, fetched, with empty
    bodies, never fetched, items) and whose rows hold ten with
    `left_out`, more on `limit`, all on `expand`.
  - call 27, `price_show` of a tab of 39 items: 28,857 text, 62,615
    whole; every item whole under `items_here`. Round 2: each of the
    two sets is counted, broken down by relation as the text prints
    it, and lists ten; `expand` lists the whole.
  - call 35, `shop_render` {}: 34,779 text, 75,401 whole, for a
    2,800-character page: 50 `left_out` entries and 38 ids under
    `freshness` before the page's `text`. Round 2: `posted`, `left_out`
    and each list of `freshness` hold ten and are counted; the pages
    are whole.
  - calls 2, 8, 18 (`accounts` 237, `search` 1,465, `show` 3,547 text):
    `last_login`, `oldest_fetch` and `newest_fetch` as epochs alone
    (`show`'s age in the text is a removal's, not `first_seen`). Round
    6: each epoch the text turns into an age has its words beside it.
  - call 46, `tabs` with `GERWARIK#7694`: `not_logged_in` with
    `accounts` (175 text, 476 whole). Round 4: the refusal carries the
    offer that is the same call with each known name; the kind stays
    (C121).
  - call 47, `price_show` of `tab/pc/ffffffffff`: `not_found` with no
    `offers` and no `next` (184 text, 486 whole). Round 4: the refusal
    carries offers naming `search` and `tabs` as requests.
  - call 49, `price_set` at a stale `if_revision`: `conflict` with
    `revision: 4` and no current value (158 text, 436 whole). Round 4:
    `current` stands beside `revision`, the row as `price_show`'s
    `manual` prints it.
  - calls 28 to 31, 36, 37 (`price_set`, `price_clear`): receipts
    `target`, `written`, `prior`, `in_facts`; a clear's `written` null
    and its tombstone's revision nowhere. Round 5: every receipt
    carries `revision`; a clear's is its tombstone's, returned by the
    delete itself; `undo` is an offer, and after a clear it is a create
    naming no revision, as the store requires.
  - call 14, `search` with `limit: 30` of 187: the audit view already
    works on `limit`; stays true.
- The sizes measured at #83 (`d2a588bc`): a result on the wire is a
  little over twice its document (`search` {} 7,431 text, 16,126
  whole; `describe` ["trade"] 24,359 text, 51,396 whole), because
  `told_as` sends the document as the text block and again as
  structured content. Round 7 measures what the host hands the model.
- The before measurement (round 8) is taken from an `acq-mcp` built at
  the commit that lands this brief (the parent of round 1's first
  commit), in a scratch worktree, never from a binary copied aside: so
  no round waits on a capture, and the baseline is reproducible from
  the sha alone.
- The bound's number: ten, C53's own, held once. Its home: the store
  crate, the one crate every maker links (C1's edges: the search links
  the store and the planner, not the protocol crate; the planner links
  the protocol crate and the store; the verbs link all of them; the
  store links nothing of the workspace). A constant and the one helper
  that applies it live there; the store itself cuts nothing (a store
  read returns what was asked for, bounded by its caller as today).

## The rounds

Eight rounds, each one concern with its own commits, gate, review and
done-when (`roadmap/README.md`, "A step": a round's review runs over
its commits as one range; the owner reads before any fix is built).
The order is a dependency order and nothing more: round 2 and round 3
need round 1's helper, round 5 needs round 4's offer shape, and rounds
6, 7 and 8 need nothing but the tree they run on. **Round 1 is the
first to build.** Rounds 1 to 5 are the contract and each is about
half a day; round 3 is the compatibility break (C149) and the largest,
three commits that may take a day. **Round 6 (the ages) is ruled and
must land, and may follow the others in any order.** **Rounds 7 and 8
are evidence rounds the owner may defer**; round 8 is the step's
before-and-after measurement, so the step's ledger line rides in the
last round he takes, and if that is not round 8 the ledger line says
the sizes are unmeasured.

What every round shares: one commit per concern; a reference
(`MCP-REFERENCE.md`, `CLI-REFERENCE.md`) regenerated in the commit that
changes its source, under `ACQ_UPDATE_FIXTURES=1` on the one test named
in a shell where the variable is otherwise unset, every changed line
read as an agent would and explained in that commit's message; a tool
description, parameter doc, `describe` entry or README line that
states a size, a number or a list rule re-read in the round that
changes the rule it states (the first draft's Do 8 and Do 9 are not
rounds: a sweep rides with the contract it describes, and a web page
golden is the check of the round that changed the document it
renders). The CLI's text stays a function of the JSON (C53) and prints
what it prints today unless a round names the change; where the JSON
gains a count the text already prints, the text reads it from the
document. Each agent-facing decision (a field's name, a count's words,
an offer's shape, the undo's shape, the age key) is recorded in the
commit that builds it and listed in the round's report for the close.

### Round 1 — The bound's home and the search crate

The number held once, and the first maker under it.

1. In the store crate one `pub const` holding ten (the build names it;
   the doc comment headed by C53 says what it is: the number of like
   entities a list holds by default on every surface) and one helper
   that takes a list and an asked bound (`limit` named, or `expand`
   meaning the whole) and gives back the rows kept and the count left
   out, so every maker applies one arithmetic and no maker owns a
   second number. A unit test in the store crate pins the helper (ten
   by default; `limit` names more or fewer, zero included; `expand` is
   the whole; `left_out` is the difference).
2. `DEFAULT_LIMIT` (`answer.rs` 341) becomes a reading of the store's
   constant: the default moves from 20 to ten, a default and not a
   shape (`RowsOut` already carries `limit`, `returned`, `after`,
   `left_out`, `next`); `LISTED` (344), already ten, becomes a reading
   too; `SUGGESTED` (345) stands (five suggestions are offers beside a
   term, not a list of entities; the list inventory says so). A new
   test in `crates/acquisition-search/tests/` (named for the fact: an
   unlimited request over more than ten matching items returns ten,
   counts the rest and continues) pins the default at the store's
   constant, never at a literal;
   `c100_a_rows_evidence_is_bounded_and_says_what_it_left_out` stays
   as it is (it pins a row's per-term evidence, a different boundary;
   the review's finding 14). `rows.rs`'s `limit: 20` request reads the
   number.
3. `tabs` folds its "thirty or fewer whole, else the ten with the most
   items" into the one rule: `LISTED_WHOLE` goes, `MOST_ITEMS` becomes
   a reading of the constant; `rows` holds ten with the most items by
   default, `limit` (added to `TabsParams` and the CLI's `tabs`) names
   more, `expand` the whole; `listed`, `left_out`, `left_out_needs` and
   `by_type` stand as they are (C53, additive; C146, a field's name is
   the contract; `by_type` is a counts view, not a list of entities).
   The tests `tabs_counts_first_and_lists_the_ten_with_the_most_items`,
   `tabs_says_its_counts_first_and_counts_a_large_league` and
   `tabs_lists_the_never_fetched_of_one_type` read the ten over a league
   of thirty or fewer too and keep their names. The `answer` section's
   sentence at `describe.rs` 243 says ten; the MCP's `tabs` description
   and `TabsParams::expand` doc, `MCP-REFERENCE.md` 207 and 210 and
   `CLI-REFERENCE.md`'s `tabs` say the one rule; `surface_size.rs`
   holds. Both references regenerated here.

Done when: a constant in the store crate holds the bound and one
helper applies it, pinned by a unit test there; `search` lists ten by
default, pinned by the new test; `tabs` lists ten with `limit` and
`expand` and no thirty; `DEFAULT_LIMIT`, `LISTED`, `LISTED_WHOLE` and
`MOST_ITEMS` are gone or readings of the constant (the commit message
names each); both references regenerated and each changed line
explained; no test deleted and none renamed; the gate green on the
round's tree. Measurement: none. Must not touch: the planner's
documents, the verbs' documents, any bare array, any refusal or
receipt, the CLI's `LIST_UP_TO` and `FINISHED_LISTED` (rounds 2 and 3
touch them with the documents they bound).

### Round 2 — The planner's documents under the bound

`price_show`, `shop_render` and `price_list`, each counted whole
before it lists ten.

1. `ShowView`'s two sets (`listing.rs` 625, 629): `items_here` and
   `items_covered_below` each list ten by default, and beside each set
   stand the counts the text prints from the whole set today
   (`price_cmd.rs` 799 and 818): the set's total, how many are listed
   and left out (the search answer's words, `returned` and `left_out`,
   with `total` as `price_list` already says it), and the relation
   breakdown by count, made in the planner from the whole set before
   it is cut, so the text's "items here: 39 items — …" line reads the
   aggregates and never describes the sample as the whole (the review's
   finding 6). `limit` and `expand` are added to `price_show` on both
   surfaces (`PriceShowParams` has neither today; the review's finding
   16): `limit` names more, `expand` the whole. The route to the rest
   is `price_show`'s own `expand` (the review's finding 7: `price list
   --in` leaves out descendants and, by default, the items nothing
   applies to), so the text's `next` line at 834 names `--expand`
   first and `acq price list --in` after it as the filtered route, in
   those words; a test runs the audit request against a container
   whose two sets exceed ten and finds every omitted item in it. This
   is the one text change this round names: the `next` line's words
   and the deferral read from the document's counts; the `price show`
   goldens that hold the old line regenerate on their named test with
   the change explained.
2. `ShopRender` (`shop.rs` 497): `posted` and `left_out` hold ten
   (`LEFT_OUT_LISTED` 50 retired for the constant; `left_out_total`
   stands and `posted` gains its count beside it in the same words),
   and each list of `Freshness` (`uncovered`, `stale`,
   `position_before_listing`) holds ten and is counted; `expand` means
   every list whole; the pages' `text` is what the answer is for and is
   never cut (C53). `shop_cmd.rs`'s `LIST_UP_TO` (37) becomes a reading
   of the constant. `c74_every_item_is_posted_omitted_blocked_or_off_the_page_and_counted`
   is extended, never renamed; `the_shop_document_bounds_what_it_leaves_out_alike_on_both_surfaces`
   and `shop_render_json_is_the_document_the_text_and_the_page_read`
   read the shape (C122).
3. `price_list`'s default `limit` is the constant where it has none
   today; `price_cmd.rs`'s `LIST_UP_TO` (50) becomes a reading of it.
   `price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many`
   and `c53_price_list_groups_by_container_and_lists_ten_or_fewer`
   read the default from the constant.
4. `MCP-REFERENCE.md` 94 and 168 re-read: a size a description states
   ("some 1.3 KB an item", "some 200 bytes an item", "past the first
   50") is remeasured in the commit or dropped. Both references
   regenerated here. `price status`'s `RowAccounting` lists stay whole
   (the list inventory names the exemption and its trigger).

Done when: a container's `price_show` lists ten of each set with the
set's total, listed, left out and relation counts beside it, `limit`
and `expand` on both surfaces, pinned at the planner and read by
`price_json.rs` and `pricing_tools.rs`; `shop_render`'s `posted`,
`left_out` and each `freshness` list hold ten and are counted, the
pages whole; `price_list` defaults to ten; `LEFT_OUT_LISTED` and both
`LIST_UP_TO` are gone or readings of the constant; the audit request
recovers every omitted item of a show, pinned; the one named text
change explained; references regenerated; the gate green on the
round's tree. Measurement: none (round 8 measures calls 27 and 35).
Must not touch: the verbs' documents, any bare array, any refusal or
receipt, the search crate.

### Round 3 — C149: no bare array (the compatibility break)

The one change that is not additive, four commits, each migrating
every consumer of the answer it changes in the same commit, so the
tree is consistent after each. A document is made in the verbs crate
so both frontends and the web read one maker; the counts' words follow
C146 (each says what it counts, the same words on every surface, the
`describe` entry the tool's description names in one line; the CLI's
text reads them from the document, so the footer is a function of the
JSON). The rows of each document hold ten by round 1's helper with
`limit` and `expand`, except where the list inventory names an
exemption.

1. **`characters`.** One document: the rows under one key with the
   counts the CLI's footer prints at `store_cmd.rs` 455 to 475 (the
   total, fetched, with empty bodies, never fetched, the items), the
   rows ten by default with `limit` and `expand` added to
   `CharactersParams` and the CLI's `store characters`.
2. **`events` / `item_events`.** One document for the raw rows and
   one for the summary rows (`events.rs` `summary`), each counted with
   what the text prints at `store_cmd.rs` 663: the rows returned, the
   limit, and whether the limit was reached; the window's total is not
   promised (the read at `lib.rs` 2175 has none, and the text never
   printed one: the review's finding 3, taken by narrowing, not by a
   count read; the inventory names the trigger that adds one). The
   default `limit` becomes the constant (the MCP's `unwrap_or(200)` and
   the CLI's `default_value_t = 200` read it); the summary's location
   rows hold ten by the helper, and the summary says which events it
   summarizes (the rows returned). A test stages more than ten events
   and `limit: 0`.
3. **`jobs` / `list_jobs`.** `Jobs::Live` serializes as an object: the
   counts `summary_text` prints (running, waiting, done, failed,
   cancelled) beside the rows; the rows are what `jobs_text` lists
   today, every open job and the ten most recent finished with the
   earlier finished counted as `jobs_text` counts them (the open jobs
   whole: the list inventory names the exemption); `expand` is added
   to `list_jobs` (today it takes no arguments; the review's finding 5)
   and means every finished job, as the CLI's `--all` does;
   `FINISHED_LISTED` becomes a reading of the constant; `Jobs::Absent`
   stands as the object it is with its persisted rows whole (the
   ledger's open rows, the same exemption). `acq jobs --watch --json`
   prints the same document per snapshot (`watch_jobs` at 2116 prints
   a bare array today; the review's finding 4).
4. **`accounts --json`.** The CLI prints the verbs' `Accounts` object
   (`provider`, `accounts`) as the MCP already does, in place of the
   inner array at `store_cmd.rs` 96; no bound (the list inventory says
   why). This commit also writes C149's pin: one test at the verbs
   crate (`documents.rs`) that every document a verb returns serializes
   to a JSON object, and one at each frontend that reads the shape of
   each answer changed in this round;
   `decisions/frontends.md` C149's "Pinned: nothing yet" becomes the
   tests' names and "Unbuilt: step 2g" goes (the registry's pin line is
   the build's to fill; the ruling's words are not touched). The pin
   lands last, when its promised coverage exists.

The consumer inventory, from the review's finding 1, each consumer
checked against the tree and each migrated in the commit named (a
driver under `tools/` is a live driver and is edited, never run, in
this round; the owner's next live run is its check):

| Consumer | Where | Migrated in |
| --- | --- | --- |
| CLI `store characters` text and `--json` | `store_cmd.rs` 404 to 475 | 3.1 |
| MCP `characters` | `main.rs` 1355 | 3.1 |
| Web refresh page's document (`{"tabs", "characters"}`) and its characters table | `web/mod.rs` 517 to 528, 624 | 3.1 (the refresh page, not the tabs page as the first draft said) |
| `apply_loop.rs` 128, `characters_wire.rs` 153 (both require an array) | CLI tests | 3.1 |
| `new_league.rs` 148 (the text "no characters known in …") | CLI tests | 3.1, if the footer's words move; else untouched |
| `tracer-rung.sh` 760 (`characters.json` saved and read) | live driver | 3.1 |
| `tool_refusals.rs` 75 (`characters` refused, the refusal shape) | MCP tests | untouched: the refusal's shape does not change here |
| CLI `store events` raw and summary, text and `--json` | `store_cmd.rs` 626 to 730 | 3.2 |
| MCP `item_events` | `main.rs` 1509 | 3.2 |
| `search_tools.rs` 1060 (`item_events_reads_back_its_hours_and_defaults_to_a_day`, reads the array through `expect_ok`) | MCP tests | 3.2 |
| `harness/mod.rs` 268 (`call`, takes either shape) | MCP harness | untouched |
| `tracer-rung.sh` 797 (`jq 'length'` over `store-events.json`) | live driver | 3.2 |
| CLI `jobs` text and `--json`, `--all` | `main.rs` 1493, 2424, 2543 | 3.3 |
| CLI `jobs --watch --json` snapshots | `main.rs` 2113 to 2125 | 3.3 |
| `watch_recovery.rs` 63 (frames a snapshot on `[]`) | CLI tests | 3.3 |
| MCP `list_jobs` | `main.rs` 1803 | 3.3 |
| Web jobs page (`Jobs::Live` through `jobs_text`; its `?json`) | `web/mod.rs` 780 to 812 | 3.3 |
| `seat_faults.rs` 588 (`jobs_lists_what_is_open_and_counts_the_finished_it_leaves_out`), 887 and 979 (two apply-loop helpers reading `--all --json` as an array) | CLI tests | 3.3 |
| `web.rs` 670, 709, 870 (jobs read as arrays; 870 the account-selection test, not a golden) | CLI tests | 3.3 |
| `daemon.rs` 249 (verbs), `daemon_observe.rs` 55, `world.rs` 379, `persisted_queue.rs` 58 (the absent queue) | tests | untouched: `Jobs::Absent` is an object today and stays |
| `tracer-rung.sh` 692 (`jobs-$tag.json` saved), `persist-check.sh` 190 and 216 (`.[]` over `jobs --json`) | live drivers | 3.3; the saved jobs file asks `--all`, the complete readback |
| CLI `accounts --json` | `store_cmd.rs` 93 to 97 | 3.4 |
| `tracer-rung.sh` 351, 360, 366, 382 and `persist-check.sh` 149 (`.[]` over `accounts --json`) | live drivers | 3.4 |
| The seat doors `seat-mcp.py` 275, `seat.py` 283 | opaque text | untouched |
| `store refused --json` (`store_cmd.rs` 575) | CLI | not migrated: #18 redesigns the refused list; the inventory names the exemption |

Done when: no tool and no `--json` of `characters`, `events`,
`jobs` (its watch snapshots included) or `accounts` prints a bare
array; each answers an object with the counts its text prints, the
text reading them from the document; every consumer in the table is
migrated in the commit the table names, and `git grep` for the old
array readings (`as_array` on these answers in the test trees; `.[]`
and `length` over these files in `tools/`) finds none; C149's pin
named in `decisions/frontends.md` and "Unbuilt" gone; the web goldens
green or regenerated with each changed line explained; `readme_tour.rs`
green; references regenerated; the gate green on the round's tree.
Measurement: none (round 8 measures call 11). Must not touch: the
planner's or the search's documents, any refusal or receipt, the ages,
`store refused`.

### Round 4 — The refusal shape (Q5, delegated)

One document on every surface, in the CLI's `--json` as on the MCP:
`error`, the words (C116, C143); `kind`, one word from the closed lists
as they stand (the protocol's, the verbs' `Fault::kind`, the planner's
`Refused`, the search's), no new word; and at most four optional keys,
each meaning one thing wherever it stands: `offers`, requests an agent
sends back as printed, each `{request, says}` and naming `tool` when
it is another tool's; `readings`, how what was typed could be read,
payload on every refusal; `next`, one sentence or one command that is
a person's act, never an agent's; and the kind's own fields
(`accounts`; `revision` with `current`, the row as `price_show`'s
`manual` prints it). A key this round adds is absent when it has
nothing to say, never null; a field that stands today keeps its
contract, so a conflict with no row ever written still says
`revision: null` (C11; the review's finding 9).

1. **The context an offer needs** (the review's finding 8). An offer
   is a request, so the maker of the refusal must hold what the request
   was: `RefusedKind::NotFound` gains the target it did not find (the
   `PriceTarget`, structured), so the offers that would find it (a
   `tabs` request for the target's realm and league; a `search` request
   for its id) can be made; and the one shaping function (item 4)
   takes, beside the fault, the request as the adapter holds it (the
   MCP: the tool's name and its arguments; the CLI: the command words),
   so an offer on `not_logged_in` with names known is the same request
   with each known name set. The adapter supplies the request and its
   spelling and nothing else decides (C143: the shared refusal speaks
   the request's words; the adapter adds its spelling after them). A
   test resubmits each offer and it runs: with a non-default realm and
   league, and with two accounts known.
2. **The search crate's scope refusal.** `Offered` for a realm
   (`error.rs` 200) reads in the request's own words (C143), the
   terminal's `--realm pc` added by the CLI's adapter after them as
   `failed` adds a flag's name; with that a scope refusal's `readings`
   are payload like a language refusal's, and the one place the
   spelling depended on the error type goes: `Prose::readings` and its
   clause in `spoken`'s `payload_key` (`main.rs` 1089), `refused`'s
   `matches!` (387), `refusing_as` where nothing else calls it, and the
   test sweep's `contains_key("offers")` clause (`tests/spoken.rs` 176)
   with the header's sentence at its line 20. `to_json`'s `offers` gain
   `says` (the reading) beside `request`; `readings` stay.
3. **`conflict` with `current`.** The row `on_conflict` (`pricing.rs`
   547) has in hand before it reduces it to the revision is passed
   through to `refused_write`; the tombstone case has no row and says so
   by `current`'s absence. `not_logged_in` with names known
   (`Fault::fields`, `UnknownAccount`): the `error`'s shape is the
   design's Q5 text ("no login made a store for `GERWARIK#7694`; the
   names known: GERWARIC#7694"); the kind stays (C121, kept).
4. **The sorters.** The MCP's `refusal`, `refused` and `priced` and the
   CLI's `error_report` (`main.rs` 998) produce this shape from one
   function at the lowest crate that sees every kind (the verbs', since
   the search's and the planner's refusals both reach it; the build
   names it), so no tool decides alone and no sibling sentence is left
   standing (`REVIEW-CHECKLIST.md`, the row). The web's failure page
   (`web/mod.rs` 322) renders the one shape. The web phrasing item
   under "Open" ("For 4-full, from 4b") stays open: offers as requests
   are the shape a page can render, but the refresh page still composes
   command prose at 518, the jobs page prints CLI text at 808, and the
   web resolves an account at startup (153) before any page, so no
   page renders an offer yet (the review's finding 13); the close says
   what remains.

Pinned once at the verbs (the shape over every kind, the four added
keys absent-never-null, `revision: null` kept on the no-row conflict)
and once per source of a new field (the search's `says`, the planner's
`current` and the `not_found` target and offers), the frontends'
refusal tests under "The tests that pin today's shapes" reading the
shape and keeping their names and fact words;
`every_text_the_server_sends_spells_the_cli_it_speaks` sweeps
`readings` as payload now. `MCP-REFERENCE.md`'s instructions line on
refusals (`readings` and `offers`) re-read; both references
regenerated here.

Done when: every refusal on both surfaces is `error`, `kind` and at
most `offers` (`{request, says, tool?}`), `readings` (payload), `next`
and the kind's own fields, the added keys absent never null and
`revision: null` kept where C11 has it; `not_found` and `not_logged_in`
carry offers that run as resubmitted, pinned with a non-default realm
and league and two accounts; `conflict` carries `current`; the scope
refusal's readings speak the request's words in the search crate with
the CLI's flag added by the adapter; `Prose::readings` and its clauses
in `spoken` and `tests/spoken.rs` are gone; the refusal tests keep
their names and fact words; the Open item's remainder named in the
round's report; references regenerated; the gate green on the round's
tree. Measurement: none. Must not touch: any receipt, any list bound,
the ages, a page's own next-action prose.

### Round 5 — The receipt shape (Q6, delegated)

Every write (`price_set`, `price_clear`, `set_sync_policy`) answers one
shape: `target` (a policy write's: the account and the realms);
`written`, the row now standing, null after a clear; `prior`, the row
before, null on a create; `revision`, the revision now current; `undo`,
the request that restores `prior`, an offer in round 4's shape (the CLI
prints it as the command `render_write` prints today, which runs as
printed; the MCP sends it as a request naming its tool); and the
write's own facts (`in_facts` on a price write; `league_notes` on a
policy write, C126). Additive on both surfaces.

1. **A clear's revision, from the delete itself** (the review's
   finding 2). `Annotations::delete` returns the tombstone's revision
   in place of `()` (one function's return; its one production caller
   is the clear at `price.rs` 763, the rest are the store's own
   tests), so a clear's receipt reports what its own transaction wrote
   and never a read taken after it (`refused_write`'s `cleared()` read
   stays for refusals, where no transaction of ours wrote). What the
   number means is said in the field's doc and the `describe` entry:
   the generation the target's row sequence has reached, which a later
   conflict names; the next write after a clear is a create and names
   no revision (`annotations.rs` 880, the store's rule), so a clear's
   `undo` is that create, exactly as `render_write` prints it at 1002,
   and a test runs it as printed.
2. **When there is no undo** (the review's finding 10). `undo` is
   present where `undo_words`' rule gives one and absent otherwise, and
   beside its absence stands one sibling key the build names (recorded
   in the message) holding the reason in words: a prior this build
   cannot read, a prior naming a retired currency (C67's writer's
   rule), and a policy create, since the CLI's `PolicyCmd` has `Show`
   and `Set` and no clear, so nothing restores a policy's absence (not
   a verb this round adds: the inventory under "Not in scope"). The
   done-when below reads these fields, not a universal `undo`.
3. **The policy receipt's `prior`.** `set_policy` (`frontend.rs` 338)
   reads the stored row once before every put (today it does so under
   `Expect::Current` alone, and `put_sync_policy` does so again under
   `Merge`); the put's compare-and-swap vouches for the read (a put
   that lands named the revision the read saw, or named none and found
   none), so `PolicyWritten` gains `prior` on every policy write, null
   on a create, with no second read after the write. If the build finds
   this reasoning wrong against the store it stops and says so in the
   report rather than carrying a `prior` it cannot vouch for.
4. The `describe` pricing entries `revision` and `undo` (`pricing.rs`
   798, 802) say the shape in the same words. A write naming a
   container's revision is refused as a conflict naming the target's
   own revision or none (today's behaviour; a test pins it as the
   teaching refusal). Pinned once at the planner
   (`c78_a_write_returns_the_prior_value_so_a_hand_can_undo_it`
   extended to `revision` and `undo`, never renamed) and once at the
   verbs for the policy receipt; the receipt tests under "The tests
   that pin today's shapes" read the shape
   (`every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known`
   runs the printed `undo` as before, the post-clear create among
   them). Both references regenerated here.

Done when: every receipt carries `revision`, a clear's returned by the
delete; every receipt carries `undo` or the named reason key, never
both and never neither; the post-clear `undo` is a create naming no
revision and runs as printed, pinned; the policy receipt carries
`prior` on every write or the report says why not; the container-
revision conflict pinned; references regenerated; the gate green on
the round's tree. Measurement: none. Must not touch: the refusal shape
beyond using it, any list bound, a policy clear verb.

### Round 6 — Ages in words beside epochs, from one function

Ruled (C53 as amended), independent of the other rounds, lands when
the owner takes it. One function for the vocabulary the CLI prints
today (`ago` at `store_cmd.rs` 128: `never`, `{n}s ago`, `{n}m ago`,
`{n}h ago`, `{n}d ago`), at the lowest crate the text and the JSON
share (the store, beside `now()`); the verbs' `age` at `refresh.rs` 355
(`45s`, `3m`: the refresh text's form, by one argument or one wrapper,
never a second table) and the CLI's `ago` become readings of it. The
rule, one sentence: an age the text prints, the JSON carries in words
beside the epoch it is made from, made at the answer's own time, under
a sibling key the build names once and uses everywhere (recorded in
the message).

The inventory the first draft recalled from memory is replaced by the
call sites in the tree (`grep -n 'ago(' crates/acquisition-cli/src`;
the review's finding 11), each with its source epoch; the basis is the
answer's own `now` in every case:

| Text site | Source epoch | Document that gains the words |
| --- | --- | --- |
| `store_cmd.rs` 112 | `AccountRow.last_login` | `accounts` |
| `store_cmd.rs` 248; `web/mod.rs` 571 | `TabRow.fetched_at` | `tabs` rows |
| `store_cmd.rs` 451, 452; `web/mod.rs` 624 | `CharacterRow.fetched_at`, `listed_at` | `characters` rows (round 3's document) |
| `store_cmd.rs` 565, 585 | the refused rows' `fetched_at` | `store refused` (text only until #18; no JSON change here) |
| `store_cmd.rs` 700, 727 | `LocationChanges.last`; `EventRow.at` | `events` summary and raw rows |
| `search_cmd.rs` 724 | `Place.fetched_at` (C118) | `search` rows' `place` |
| `search_cmd.rs` 922, 923, 931 | the scope's `oldest_fetch`, `newest_fetch`, `coverage.list_seen` | `search`'s scope |
| `search_cmd.rs` 1015, 1336, 1896; `web/page.rs` 300 | `removed_at` | `search` rows and `show` (no `first_seen`: the text never printed one) |
| `price_cmd.rs` 368, 376, 705, 769, 770, 777 | the bids' `fetched_at`, a history row's `taken_at`, `manual.updated_at`, a response's `at` | `price_show` |
| `shop_cmd.rs` 190 | `freshness.oldest_stale_seconds` (already a span; words beside it) | `shop_render` |
| `plan_cmd.rs` 156 | the policy row's `updated_at` | `sync_policy` / `policy show` |
| `plan_cmd.rs` 232, 248 | a plan's `at` and a listing's `fetched_at` | `refresh_plan` |
| `plan_cmd.rs` 1656, 1684 | `quote.observed_at` | `quote` |
| `main.rs` 112, 131, 1938 | a persisted job's `written_at`, `updated_at` | `jobs` absent and `job` |
| `dash.rs` 763 | seconds the daemon reports, a tty view with no JSON | none; outside this round |

Pinned once at the store (the vocabulary) and once per maker that
gains the words, the frontends' tests reading the shape
(`a_rows_place_carries_when_its_location_was_fetched` reads the words
beside the epoch). A golden holding JSON regenerates on its named test
alone; the text goldens do not change. Two commits if the build wants
them: the function and the rows the seats met (`accounts`, `tabs`,
`characters`, `search`, `show`), then the rest of the table.

Done when: every text site in the table prints through the one
function, the CLI's `ago` and the verbs' `age` readings of it; every
document in the table carries the words beside the epoch under the one
key; `dash.rs` untouched; pinned at the store and per maker; references
regenerated where a field doc changed; the gate green on the round's
tree. Measurement: none. Must not touch: any list bound, any document
shape beyond the added key, any refusal or receipt.

### Round 7 — The second copy, measured once on the host at hand (Q2, delegated; evidence, deferrable)

Fixed, the recipe (the review's finding 15): a file under your scratch
`2g/host/mcp.json` holding `{"mcpServers": {"acq": {"command":
"<abs path>/target/debug/acq-mcp", "env": {"ACQ_PROVIDER": "mock",
"ACQ_STORE_DIR": "<your 2g/host/store>"}}}}` over a world made from
the pristine store; the run `claude -p --mcp-config <that file>
--strict-mcp-config --allowedTools mcp__acq__characters --output-format
stream-json --verbose --max-turns 2 'Call the characters tool with no
arguments, then reply with the one word done.'` from a directory under
your scratch that holds no `.claude/` and no `CLAUDE.md` (so neither
the repository's configuration nor your home's rides in); the reading,
the `user` event whose `message.content[0].type` is `tool_result`: by
the stream's own contract a `user` message is what the host sends to
the model as its next input, so that block's `content` is the model's
input as observed at the host, and the measurement says so in those
words rather than claiming to have seen the API request. What is
recorded in the commit message: the host's version (`claude
--version`), the text block's bytes, the whole result's bytes on the
wire (from `seat-mcp.py` over the same world), the `tool_result`
content's bytes and its shape (one text block, or the document twice),
and the first 200 bytes of it verbatim; the whole stream saved under
`runs/2g-host/` (gitignored, the evidence home of a run) for the owner
to read.

The rule for what it decides: if the host hands the model both the
text block and the structured content (the content's bytes near the
result's whole, or the document twice in it), the structured copy goes
in a commit of its own: `told_as` sends the text block alone,
`CallToolResult::structured` is not used, and the 24 reads of
`structuredContent` in the MCP's tests and harness are migrated by
purpose, inventoried in the message: a convenience read (the test
wanted the document) reads the text; the harness's `call` reads the
text; the one contract read, `spoken.rs` 231 ("the text and the
structured content are one document"), becomes its inverse (no
structured content is sent) rather than a vacuous comparison. If the
host hands the text block alone, both copies stay, the copy being the
host's and costing the model nothing, and the measurement is stated as
the reason in a commit that changes no code (or in the ledger line, if
no commit is owed). No configuration of the repository or of your home
changes for this.

Done when: the measurement is in a commit message with the host's
version, the four byte counts, the shape and the verbatim head; the
structured copy is gone or kept by the rule's outcome; if gone, every
former read of it is migrated under its named purpose and `spoken.rs`
231 asserts the absence; the gate green on the round's tree. Must not
touch: any document shape, any list bound.

### Round 8 — The replay, before and after (evidence, deferrable; the step's measurement)

The three calls (11, 27, 35) through `seat-mcp.py` `call` over a fresh
world made from the pristine store (`~/seat-worlds/readiness-2026-10/pristine/`,
read and never written; `setup --store pristine/store` into a world
under your scratch `2g/replay/`), twice: with an `acq-mcp` built at the
commit that lands this brief (checked out into a scratch worktree
under `2g/replay/before/`, `cargo build -p acquisition-mcp` there),
and with the `acq-mcp` of the round's HEAD; alone on the machine (no
gate, build or driver beside a measurement). Calls 11 and 27 reproduce
the recorded input state (the seat's price writes begin at call 28);
call 35 does not: calls 28, 29 and 31 wrote prices the render posted,
cleared at call 51, so over the pristine store its before-size is a
new baseline and not a reproduction of the recorded 34,779 text and
75,401 whole (the review's finding 12). The measurement says so, and
stages nothing: the baseline is the pristine world for both binaries,
the same input to each. Recorded in the commit message as a table,
text and whole bytes per call, before and after, with the two shas
the binaries were built from, the pristine store's path and the line
"call 35 is a new baseline over the pristine store, not the seat's
recorded state", and one line stating what the three cost together
before and after.

This round's commit carries the step's ledger line and a "Current
step" that says the review is pending; if the owner takes the step's
close before this round, the last round he takes carries the ledger
line with "the sizes unmeasured; round 8 open". A worktree made for
the before-build is removed when the measurement is read.

Done when: the table is in the commit message with both shas and the
baseline line; the ledger line and "Current step" written; nothing
under `crates/` changed by this round; `git diff --check`,
`tools/docs-check.sh` and `python3 -I tools/bothy.py check` green (no
Rust changes, so the gate's Rust stages are not owed; run the gate if
any earlier round's tree is doubted). Must not touch: any code.

## The list inventory

Every list a frontend answers (the review's finding 5), its maker, the
count that stands beside it, its default before and after this step,
its audit argument, and the exemption where one is justified. A row
with a round number is that round's; a row marked "stands" changes
nothing and is here so the next reviewer need not find it again.

| List | Maker | Count beside it | Default (before → after) | Audit argument | Exemption, if any |
| --- | --- | --- | --- | --- | --- |
| `search` `rows` | search `RowsOut` | `limit`, `returned`, `left_out`, `next` | 20 → 10 (round 1) | `limit`, `after`/`next` | |
| a row's per-term `shows` | search (`LISTED`) | says what it left out | 10, reads the constant (round 1) | `show <id>` is the whole | |
| `search` suggestions | search (`SUGGESTED`) | its own count per suggestion | 5, stands | | offers beside a term, not a list of entities |
| `search` counts views (`by_type`-like keys) | search | they are counts | stands | | a counts view, not rows |
| `tabs` `rows` | search `Tabs` | `listed`, `left_out`, `left_out_needs`, `tabs` | whole ≤ 30 else 10 → 10 (round 1) | `limit` (added), `expand` | |
| `tabs` `by_type` | search `Tabs` | they are counts | stands | | a counts view; a league has a handful of types |
| `price_list` items | planner `ListView` | `total`, `listed` | none → 10 (round 2) | `limit` | |
| `price_show` `items_here`, `items_covered_below` | planner `ShowView` | per set: total, listed, left out, by relation (added) | whole → 10 (round 2) | `limit`, `expand` (both added) | |
| `price_status` `RowAccounting.unmatched`, `unreadable`, `other_league` | planner | `total`, `applied`, `other_realm` | whole, stands | CLI `--expand` exists; MCP has none | the audit of where every row went, read nowhere else (its doc); bounded when a runner meets one large, as an issue |
| `shop_render` `posted`, `left_out`, `freshness.*` | planner `ShopRender` | `left_out_total`; `posted`'s count (added); each freshness list's (added) | 50 / whole → 10 (round 2) | `expand` | the pages' `text`: what the answer is for, never cut |
| `characters` rows | verbs (round 3) | total, fetched, empty bodies, never fetched, items | whole → 10 | `limit`, `expand` (both added) | |
| `events` / `item_events` rows and summary rows | verbs (round 3) | returned, limit, limit reached; the summary's events | 200 → 10 | `limit` | no window total: the read has none and the text never printed one; a count read waits for a runner who asks for it (an issue if one does) |
| `jobs` / `list_jobs` open rows | verbs `Jobs::Live` (round 3) | running, waiting, done, failed, cancelled | every open job, stands | | what the answer is for ("what runs, what waits"), as the text has listed them since C53's text rule; the owner may strike this reading at round 3's launch |
| `jobs` finished rows | verbs (round 3) | the earlier finished counted | 10 (`FINISHED_LISTED`, reads the constant) | `expand` (added to `list_jobs`), CLI `--all` | |
| `Jobs::Absent` persisted rows | verbs | the ledger's counts | whole, stands | | the ledger's open rows, the same reading as the open jobs |
| `job` result `failed_children` / `failed_ids` | verbs `JobResult` | the tally | 10 detailed, every id, stands | | C23: the tally lists every id by design |
| `accounts` | verbs `Accounts` | none | whole, stands (the CLI's `--json` wrapped in round 3) | | a handful of names; the selector's domain |
| `store refused --json` | store `refused_list` | none | `limit` as typed, a bare array, stands | `--limit` | #18 redesigns the refused list; C149's pin covers the verbs' documents and this CLI maintenance read is named here as the one `--json` array left |
| `describe` sections, a plan's actions, a `show` body, a page's `text`, a submitted job's parameters | each maker | | whole | | C53: what an answer is for is never a list and never cut; C38: a plan's actions are the authorization |
| the daemon's `List` and `Quote` over the frame | daemon | | | | #56, a ruling, not this step's |

## The review's findings, taken into the rounds

One line each; the number is the review's.

1. Consumer inventory: taken into round 3 as the table, every consumer
   checked against the tree and migrated in the commit named; the
   drivers under `tools/` edited there.
2. A clear's revision and the post-clear create: taken into round 5
   (the delete returns the tombstone's revision; the `undo` after a
   clear is a create naming no revision, run as printed).
3. The event window total: taken by narrowing, round 3.2 (the count is
   the rows returned and "limit reached", which is what the text
   prints; no count read added; the inventory names the trigger).
4. "No `--json` prints a bare array" wider than three: taken into round
   3 (watch snapshots and `accounts --json` migrated; `store refused`
   named as the one exemption, #18's) and the done-when narrowed to the
   inventory.
5. The list inventory and jobs' audit route: taken as the table above
   and round 3.3's `expand` on `list_jobs`; `jobs.rs` 167 is
   `FAILED_CHILDREN_DETAILED`, already ten with the tally whole by C23,
   inventoried as an exemption rather than migrated.
6. `price_show`'s aggregates from the whole set: taken into round 2.1
   (per-set totals and relation counts made before the cut; the one
   text change named).
7. The route to a show's omitted items: taken into round 2.1 (the
   audit route is `price_show`'s own `expand`, tested against the
   omitted sets; `price list --in` named as the filtered route).
8. The context an offer needs: taken into round 4.1 (`NotFound` gains
   its target; the shaping function takes the request from the
   adapter).
9. "Absent, never null" against C11's `revision: null`: taken into
   round 4 (the rule applies to the keys the round adds; `revision`
   keeps its contract).
10. Universal `undo` against its exceptions: taken into round 5.2 (an
    `undo` or a named reason key, never both or neither; a policy
    create has no undo and says so; no clear verb added).
11. The age inventory: taken into round 6 as the table from the tree's
    call sites; `first_seen` struck.
12. Call 35's input state: taken into round 8 (calls 11 and 27
    reproduce; 35 is a new baseline over the pristine store, said so;
    the before-binary built from a sha, so no capture precedes round
    1).
13. The web phrasing item: taken into round 4.4 (the Open item stays
    open; the close names the three renderers that remain).
14. The default-limit pin: taken into round 1.2 (a new test over more
    than ten matching items; the C100 test kept as it is).
15. The host experiment's recipe and the test migration by purpose:
    taken into round 7 (the exact command, the isolation, the reading
    stated as observed host output, `spoken.rs` 231 inverted rather
    than made vacuous).
16. The `price_show` citation: taken (Read corrected; `limit` and
    `expand` are additions to `price_show`, round 2.1).

Not taken: none outright. Two remedies were taken narrower than the
review proposed, 3 (no count read) and 5's `jobs.rs` 167 (an exemption,
not a migration), each for the reason in its line.

## Not in scope

- A new kind in the closed set (`unknown_account` or any other; C121
  kept, the owner's to reopen), or a kind standing on one surface.
- Renaming a JSON field that stands today (C53, additive): `listed`,
  `left_out_total`, `by_type`, `items_here`, `items_covered_below`
  stay; a field added takes the search answer's words.
- A policy clear verb (round 5 names the missing undo instead); a
  window count for events (round 3.2 names the trigger); a bound on
  `price_status`'s row accounting (the inventory names the trigger).
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
  naming it stay for the web UI); the time seam (4-0); the pages'
  next-action prose (round 4.4 leaves the Open item open).
- A second rendering of a result on the MCP (C53, no second rendering).
- A schema for any document; the seat skill's wording and
  `SEAT-LEDGER.md` (the owner's); the next seat's brief (the seat
  skill's, written by the session that sits it, pointed at these
  shapes by the close).
- Running a live driver (`tracer-rung.sh`, `persist-check.sh`): round
  3 edits their readings; the owner's next live run is their check.

## Rules

`TEMPLATE.md`, with these differences: one round per session, alone in
the main tree on `main`, no worktree except round 8's scratch worktree
for the before-build; scratch under your session's scratchpad in a
subdirectory named `2g`, every `ACQ_STORE_DIR` under it, every
`--mcp-config` file and every seat world made for a measurement under
it; the pristine store under `~/seat-worlds/` is read and never
written; commit on `main`, never push; the `Co-Authored-By` line for
your model last; long commands as tracked background tasks;
`tools/gate.sh`, and `cargo build --workspace` before any daemon (C84,
the artifact mismatch; no build is owed after a `--workspace` test
run); a measurement runs alone on the machine, with no gate, build or
other driver beside it; an edit made for a measurement is reverted
before the next and never committed; regenerate a golden or a
reference only with `ACQ_UPDATE_FIXTURES=1` on the one test named, in a
shell where the variable is otherwise unset; a round's review runs over
its commits as one range and a fix round's done-when is the
template's; a round reads this brief, its own section, the rounds
before it as landed (their commit messages name the fields they
added), and nothing of a later round's work.

## Report

`TEMPLATE.md`, per round; item 1 lists the agent-facing decisions for
the close beside the commits; round 7's item 3 carries the host
measurement and round 8's the three calls' sizes before and after.

## The rulings this brief rests on

C53 as amended at `ea0b0255` (the bound of ten held once, with `limit`
and `expand`; on the MCP the JSON is the text and no second rendering;
ages in words beside epochs), C149 (every answer and refusal one JSON
object, lists counted beside rows; the one non-additive change, taken
at the alpha), C121 kept (an account that cannot be picked is
`not_logged_in` or `ambiguous_account` with `accounts`), C11 (the
failure document; `kind: "conflict"` with `revision`), C143 (a shared
refusal speaks the request's words; the adapter adds its spelling),
C146 (a count says what it counts; a field's name is the contract),
C115, C116, C122, C1, C47, C84; C100 and C118 (the search's answer
and a row's place age); C74, C78 (a write returns the prior), C126
(the policy receipt's notes), C38 (a plan's actions are the
authorization, never cut); C35 (intent under compare-and-swap, which
round 5's `prior` and the delete's return rest on). Q5's refusal shape
and Q6's receipt shape are the build's under the owner's delegation,
recorded in the commits that build them and listed for the close.

## What still needs the owner's word

Nothing, on this reading: the design was ruled in full and every
round builds under a ruling or a delegation named above. Two readings
he may want to glance at before the round that carries them, neither a
ruling: in round 3, the open jobs are listed whole as "what the answer
is for" (the text has listed every open job since C53's text rule; the
inventory marks it and he may strike it at the round's launch, in
which case the open rows take the ten-and-counted rule with `expand`);
and in round 5, `Annotations::delete` gains a return value, a change
inside the store crate's own door and not to its contract (C35 holds;
the revision sequence is untouched). Round 7's full capture goes under
gitignored `runs/`, the existing home of a run's evidence, so no new
document is made.
