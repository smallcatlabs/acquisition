# words-extents — a count or an age names the extent it is taken over

Round 6 of the answer-facts group, the words round
(`roadmap/reviews/answer-facts-map.md` §3 Root 3 and §10 steps S6 to
S10, with two items raised at rounds 2 and 4's closes), written by the
orchestrating session on 2026-11-04 against the tree at `24ad0bd3`
(round 5 closed; every line number below from it; round 3, which
adds JSON words and changes no text, lands before or after this one
without touching what this round changes). No new ruling: every item
is C146 read precisely (a count says what it counts, where it came
from and what the reader can do next), C53 (the decision view; one
clock per age) and C116 (no id in user-facing text); the owner's
answers to the map's §11 questions 5 (C146 unchanged) and 7 (#12
closed on the terms block's sentence) are in hand. The words are the
session's agent-facing design, listed at the close for his review.
To be reviewed cold before a worker reads it.

## Goal

When this step is done, every count and age the map's third root
found names the extent it is taken over, in the text and in its
gloss: a lifetime, a window, a limit's rows, a file, a league, a
realm; the plan has one measure for its character listing's age; C77
is pinned; each never-fetched type row says its never-fetched count;
the total line names a remove-only tab as its rows do; and a status
with no items but locations says so without contradiction. #105
closes on its third item; #12 closes at the round's close on the
terms block's sentence (the owner's, question 7). No JSON key is
renamed (C146); every JSON change is additive.

Why it matters: the league seat read `sends 17` as the day's count,
`0 done` as a fact, "skipped" as the game's word, and the events
summary's window as its count; a reader takes a bare number for the
whole, and the words beside it are the only place the extent can
live.

## Read

- The map §3 Root 3 (the cases, each with its section) and §10 S6 to
  S10; the closes of rounds 2 and 4 (`LEDGER.md`, their "Raised and
  left" cells: the remove-only name, the zero-item status line).
- `decisions/frontends.md`: C146, C53, C149; `decisions/plans.md`:
  C77 (freshness is judged at compile against the window; *Pinned:*
  nothing yet), C3 (the idle default, one hour, `decisions/daemon.md`);
  `decisions/pricing.md`: C81 (a nearer hand row beats a tab's
  `~skip`), C78; `AGENTS.md`: C116, C122, C154.
- The items, each with its maker and its words today:
  1. The events summary: `crates/acquisition-verbs/src/events.rs:128–145`
     (`events: events.returned`, `limit_reached`) and
     `crates/acquisition-cli/src/store_cmd.rs:788–812` (`N events at M
     locations in the last 24 h` with `(limit reached)`): the counts
     are the returned rows', said as the window's.
  2. `daemon status`: `crates/acquisition-cli/src/main.rs:2526`
     (`policies learned: N`) and `:2557` (`sends N`, beside a journal
     path that spans lifetimes): this daemon's lifetime, bare
     (`crates/acquisition-daemon/src/rails.rs:33`, the limiter is
     never persisted).
  3. `jobs`: `crates/acquisition-verbs/src/jobs.rs:30`, `:140`,
     `:175` (`done` counted) and the ledger read's text `3 jobs: 1
     running, 2 waiting, 0 done`
     (`goldens/persisted_queue/the_queue_read_from_the_ledger.txt:12`):
     `done` is never read from the ledger, and `N jobs` is this
     daemon's lifetime from the first id it issued (`queue.rs:397–415`).
  4. `accounts`: `crates/acquisition-store/src/index.rs:246`, `:259`,
     `:332` (`last_login` written by a login and by the refresh's
     persist alike); the text `last login 19d ago`
     (`documents.rs:219`, `c53_an_account_row_says_its_last_login_as_an_age`).
  5. `price status`: `crates/acquisition-cli/src/price_cmd.rs:464`
     (`N items skipped (~skip)`), where C81 makes a nearer hand row
     apply over the tab's `~skip`.
  6. `store refused`: `store_cmd.rs:664–672` (`N refused body(ies)`,
     `returned` printed as the count, never whether more exist);
     `store status`'s `refused_bodies` is the whole (`lib.rs:846–897`).
  7. A price receipt's `in_facts`: `crates/acquisition-plan/src/pricing.rs:897–903`
     ("whether the facts hold the target at all, in any league") and
     its gloss at `:1134`; `price status`'s `name nothing in the
     facts` (`price_cmd.rs:213`), which counts rows whose address is
     not in this league's facts: two phrases for two extents.
  8. `crates/acquisition-protocol/src/protocol.rs:594–598` (the idle
     knob's doc says `default 60`) against
     `crates/acquisition-daemon/src/daemon.rs:172` (`IDLE_SHUTDOWN`
     3600, C3).
  9. The `never_fetched_by_type` gloss: `crates/acquisition-search/src/describe.rs:238–240`
     ("`items` the live items in the fetched ones") against the count
     (`crates/acquisition-store/src/coverage.rs:144–151`, `:168–175`:
     the live items at every location of the type).
  10. S10: `TabType` (`coverage.rs:144–151`): `tabs`, `fetched`,
      `items`; the seat subtracted by hand.
  11. S7, the plan's two measures of one age:
      `crates/acquisition-plan/src/lib.rs:837–845` (`PlanNotes.character_listing_fetched_at_age`,
      made at `now`, `:866–886`) and `crates/acquisition-verbs/src/refresh.rs:113`,
      `:133` (`PlanAges.character_listing_fetched_at_age`, measured
      `before generated_at`): one key name, two clocks.
  12. S8: C77's property with no pin: `crates/acquisition-plan/src/lib.rs:1–60`
      (the module doc: a plan does not pre-fetch what is fresh at
      compile; the policy's window), the compile's freshness test
      (grep `max_age_seconds` in `lib.rs`), `unit/tests/lib.rs`.
  13. Raised at round 2's close: the total line names a location from
      the item's `Place` (`crates/acquisition-search/src/corpus.rs:291–325`:
      a name that is the ` (Remove-only)` mark alone is none, 301,
      and the id is shown), the scope line from the store's
      `FetchedLocation::of` (`coverage.rs:35–44`: the mark kept as the
      name): one tab, two names.
  14. Raised at round 4's close: `price_cmd.rs:370–376` (the zero-item
      line `no items on record for <place>: N listed, none fetched
      with items`) beside the `facts:` line that follows it (`N
      locations, M fetched`): "none fetched with items" reads against
      "M fetched" where the fetched locations are empty.
- The glosses and bounds: the MCP's `describe` entries for each
  changed count (grep each key in `crates/acquisition-search/src/describe.rs`
  and `crates/acquisition-mcp/src/main.rs`), `surface_size.rs:32`
  (the first turn's bounds) and `search_tools.rs` (the `describe`
  sections' bounds).
- The holders, each changed by this step, found by grep on each
  item's words over `crates/*/tests` and `crates/*/src`: the events
  goldens and `store_answers.rs`; `goldens/persisted_queue/*` and
  the daemon-status text's tests (`acquisition-mcp/tests/daemon_status.rs`,
  identity fields alone; `main.rs`'s inline tests); `documents.rs:219`;
  `price_json.rs` and `price_cmd.rs:1388`'s inline literal;
  `store_answers.rs:289` (`store refused`); `pricing_tools.rs:545`,
  `:1012` (the receipt's gloss); `seat_faults.rs:1375` and
  `goldens/seat_faults/scope_line_by_tab_type.txt` (the type rows);
  `documents.rs:429`, `:483` and `plan_json.rs` (the plan's ages);
  `goldens/seat_faults/place_and_its_age.txt` and a fixture with a
  remove-only tab (`goldens/seat_faults/a_unique_tabs_page_with_no_name…`,
  `seat_faults.rs:1160`); `price_cmd.rs:1234` (the zero-item
  positional test, round 4's line 1). The worker lists every holder
  it finds in its report.
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets"
  (a claim wider than its evidence; a sibling sentence left standing).

## Input

- The rulings, above; no new ruling. The owner's answers in hand:
  C146 unchanged (question 5); #12 closed on the sentence (question
  7, the close's action, not the worker's).
- The words, the session's design, each a function of the document's
  own fields (C53):
  1. Events: `the newest 10 events (of more; \`--expand\` lists all) at
     M locations in the last 24 h: …` when `limit_reached`, the
     existing sentence when not; the JSON unchanged (`limit_reached`
     and `left_out` already say it); the gloss says the counts are the
     returned rows'.
  2. Daemon status: `policies learned: N since this daemon started`
     and `sends N since this daemon started`, the journal path's
     clause unchanged; the glosses the same.
  3. Jobs: `N jobs since this daemon started: …` on the live read; on
     the ledger read, `done` is not printed (`3 jobs on disk: 1
     running, 2 waiting; done not recorded`), the JSON's `done` kept
     and glossed as the live count.
  4. Accounts: `last sign-in or token refresh 19d ago`; the key
     `last_login` kept (C146 bars a rename), its gloss saying both
     writers.
  5. Price status: `N items marked ~skip in game (a hand row nearer
     the item still applies)`; the gloss the same.
  6. Store refused: `N refused bodies shown of M on record` (M from
     the status's whole), `of M` absent where N is M; the JSON gains
     `total` beside `returned` and `limit` (additive).
  7. The receipt's gloss says `in_facts` is "the address exists in
     these facts in any league, removed included"; the status's phrase
     becomes `name nothing in this league's facts`; the JSON key
     unchanged.
  8. The protocol doc says `default 3600, one hour (C3)`; a test holds
     the doc's number to the daemon's constant (the words step's
     pattern for a knob: grep `IDLE_SHUTDOWN` in the daemon's tests).
  9. The gloss says `items` is "the live items at every location of
     the type, fetched or not".
  10. `TabType` gains `never: usize` (`tabs - fetched`), additive, on
      every row of `never_fetched_by_type` and `tabs`' `by_type`; the
      text unchanged; the gloss names it.
  11. The plan note's `character_listing_fetched_at_age` is measured
      `before generated_at` as the wrapper's is, the key name kept;
      the note's text says `before the plan` where it said `ago`.
  12. C77's pin: a unit test in the plan crate compiling a plan at `T`
      over a fact fresh at `T` and past the window at `T + cycle`:
      the plan at `T` does not fetch it and a plan at `T + cycle`
      does; C77's *Pinned:* names the test (the build replaces
      `nothing yet`, as every build does).
  13. `FetchedLocation::of` names a location as the row's `Place`
      does: a name that is the remove-only mark alone, or empty, is
      none and the id is shown; the one rule in the store, the search's
      `Place` reading it from there if that is simpler than two
      matching rules; the scope line and the total line agree.
  14. The zero-item status line says `no items on record for <place>:
      N listed, M fetched, none with items` where M > 0, and `none
      fetched` where M is 0; the `facts:` line stays.
- Not taken, with reasons, listed here so the worker does not take
  them: thousands separators (every count prints bare, one rule);
  `policy show` answering on a malformed row (its answer is the row;
  a design call, left); narrowing `last_login`'s writer (a behaviour
  change to the index; the words say both writers instead).
- The failing inputs, each a test staged before its words change:
  an events summary over the limit; a daemon status after seventeen
  sends; a ledger read with no `done`; an account whose last write
  was a refresh; a status with a `~skip` tab and a nearer hand row;
  a refused list shorter than the whole; a receipt for a target in
  another league beside a status counting it; a type row's `never`;
  a plan note and wrapper measuring one listing age alike; C77's two
  compiles; a remove-only tab as a match's place and as the scope's
  oldest; a zero-item status over fetched empty locations.
- The last push's CI, on `main` at `24ad0bd3`: the runs named at the
  cut that starts the build; a red run after a push is the group's
  first concern.

## Do

1. The pins first, red: one test per failing input, in the crate that
   holds the behaviour (C122), the words asserted as goldens where a
   golden holds them.
2. The words and the one measure, items 1 to 14, each where its maker
   is; the glosses beside them; no key renamed, every JSON change
   additive; the C77 test and C77's *Pinned:* line.
3. The holders moved to their new facts; `ACQ_UPDATE_FIXTURES=1` once
   over the workspace, every changed line read: a changed count or
   age sentence of items 1 to 14, a `# holds` line, an added JSON key
   (`total`, `never`), a gloss, the references' prose; the MCP's
   bounds measured after the gloss edits, none raised without a
   measured reason named in the report.
4. The gate; the commit naming this brief, the changed lines by kind,
   and `retired nothing`. #105 closes on its third item.

## Done when

- Each of the fourteen items prints the words of Input on the surface
  that holds it, pinned by a test or a golden fact; `grep -rn
  '(limit reached)' crates/acquisition-cli/src` finds nothing.
- `TabType.never` exists on every type row; the plan note's and the
  wrapper's character-listing ages are equal on one fixture; C77's
  test exists and its *Pinned:* names it; `FetchedLocation::of` and
  `Place` name a remove-only tab alike, pinned on both lines.
- No JSON key renamed: `git diff` shows no removed `pub` field in any
  serialized struct.
- Every regenerated line is one of Do 3's kinds; the MCP's bounds
  hold or their raise is named with its measure.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- Round 3's words beside every epoch (its own brief); the
  subject blocks (done); thousands separators; `policy show`'s
  refusal; `last_login`'s writer; the union `leagues` table.
- The seat that follows the group: the orchestrator's, after this
  round and round 3 both land.

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names; never while another round's worker builds.
- `ACQ_UPDATE_FIXTURES=1` runs once, in Do 3, every changed line read
  before the commit.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the regenerated files by directory, the
changed lines by kind, the MCP's bounds before and after, and the
test count before and after.
