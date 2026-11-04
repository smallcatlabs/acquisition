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
Cut three times: the first draft (`36c4dc9b`) and the re-cut
(`9080b9cf`) were each reviewed cold
(`roadmap/reviews/words-extents-brief-<sha>.md`, the prompts beside:
nine findings then four, two nitpicks in all); this cut answers the
second review's four, listed at the foot under "Taken from the
reviews" with the first's nine; not reviewed a third time, the
roadmap's rule.

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
  1. The events summary: `crates/acquisition-verbs/src/events.rs:74`
     (`limit_reached` is set on equality, so a window of exactly the
     limit's events sets it too: a limit met is not proof of more),
     `:128–145` (`events: events.returned`, `limit_reached`;
     `left_out` counts location rows, not events) and
     `crates/acquisition-cli/src/store_cmd.rs:738–745` (`--expand`
     alone selects the raw event list, 739; `--summary --expand`
     expands the summary's location rows, 744; the event limit stays
     under both),
     `:787` (the raw list's `(limit reached)`), `:788–812` (the
     summary's sentence) and `:879` (the footer "`--expand` lists
     every event", which is not what `--expand` does): the counts are
     the returned rows', said as the window's.
  2. `daemon status`: `crates/acquisition-cli/src/main.rs:2526`
     (`policies learned: N`) and `:2557` (`sends N`, beside a journal
     path that spans lifetimes): this daemon's lifetime, bare
     (`crates/acquisition-daemon/src/rails.rs:33`, the limiter is
     never persisted).
  3. `jobs`: `crates/acquisition-verbs/src/jobs.rs:30`, `:140`,
     `:175` (`done` counted) and the ledger read's text `3 jobs: 1
     running, 2 waiting, 0 done`
     (`goldens/persisted_queue/the_queue_read_from_the_ledger.txt:12`):
     `done` is never read from the ledger, and the ledger's `running`
     is recorded, not current; the live count is the jobs this daemon
     holds, this lifetime's and the restored ones alike
     (`crates/acquisition-daemon/src/daemon/queue.rs:390–415`); the
     one `summary_text` serves the live read, the ledger read and a
     selected job (`crates/acquisition-cli/src/main.rs:122`, `:2982`,
     `:3060`), so each gets its own context rather than one changed
     sentence.
  4. `accounts`: `crates/acquisition-store/src/index.rs:246`, `:259`,
     `:332` (`last_login` written by a login and by the refresh's
     persist alike); the text `last login 19d ago`
     (`documents.rs:219`, `c53_an_account_row_says_its_last_login_as_an_age`).
  5. `price status`: `crates/acquisition-cli/src/price_cmd.rs:464`
     (`N items skipped (~skip)`), where C81 makes a nearer hand row
     apply over the tab's `~skip`, and where the count is
     `by_game_statement`'s, the game-side statements in public tabs
     alone (`crates/acquisition-plan/src/listing.rs:525–527`; a
     non-public tab's price text is residue).
  6. `store refused`: `store_cmd.rs:664–672` and `:687–693` (`N
     refused body(ies)`, `returned` printed as the count, never
     whether more exist; the only next step offered is one body in
     full); `store status`'s `refused_bodies` is the whole
     (`lib.rs:846–897`).
  7. A price receipt's `in_facts`: `crates/acquisition-plan/src/pricing.rs:897–903`
     ("whether the facts hold the target at all, in any league") and
     its gloss at `:1134`; `price status`'s `name nothing in the
     facts` (`price_cmd.rs:213`), which counts rows whose address is
     not in this league's facts: two phrases for two extents. The
     status phrase has three producers that must move together: the
     summary line (`:213`), the expanded list's heading `rows naming
     nothing in the facts` (`:229`) and the receipt's quotation of it
     (`:1173`), and the status gloss (`pricing.rs:1140`).
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
      and the id is shown; `tab.remove_only` is read from the raw
      name at `:499` and stays so, C117), the scope line from the
      store's `FetchedLocation::of` (`coverage.rs:35–44`: the mark
      kept as the name): one tab, two names. The scope's ages are in
      the JSON alone since round 2 (`tests/seat_faults.rs:1500–1523`),
      so the two names meet in the scope's JSON, the total's JSON and
      the total line and the rows' text.
  14. Raised at round 4's close: `price_cmd.rs:366–368` (`nothing on
      record for <place>: no stash listing, no character listing`,
      printed because both listings are absent, beside a `facts:` line
      that may count fetched locations: the contradiction round 4's
      worker saw, pinned as it stands at `:1421–1427` over four
      fetched locations) and `:370–376` (`no items on record for
      <place>: N listed, none fetched with items`, ambiguous beside `M
      fetched` where the fetched locations are empty).
- The glosses and bounds: the MCP's `describe` entries for each
  changed count (grep each key in `crates/acquisition-search/src/describe.rs`
  and `crates/acquisition-mcp/src/main.rs`), `surface_size.rs:32`
  (the first turn's bounds: 600, 300 and 1,000 bytes for a tool's
  description, its parameters and the instructions) and
  `search_tools.rs` (the `describe` sections' bounds: 7,000 answer,
  8,000 pricing, 6,000 store, 4,000 daemon).
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
  a seeded fixture with a remove-only tab, extended from the one
  `crates/acquisition-search/tests/seat_faults.rs:1160` uses (the
  unique page's test; there is no CLI golden of that name), with
  `goldens/seat_faults/place_and_its_age.txt` changing only if its
  fixture changes; `price_cmd.rs:1414`, `:1426`, `:1867` and
  `goldens/new_league/launch_morning.txt:174` (the zero-item status's
  sentences); `store_cmd.rs:879` and `:787` (the events footer and the
  raw list's words); the type rows' holders for item 10:
  `crates/acquisition-search/tests/tabs.rs:133` (whole `by_type` rows
  compared as JSON) and `crates/acquisition-store/unit/tests/coverage.rs:98`,
  `:165`, `:278`, `:296` (`TabType` expectations) and `:432` (the
  serialized coverage text holding a type row), each gaining `never`
  before regeneration, with a zero and a positive `never` pinned on
  the `tabs` and search surfaces; for item 13:
  `crates/acquisition-search/tests/seat_faults.rs:1160`
  (`a_unique_tabs_page_with_no_name_is_named_by_its_id`, extended to
  the scope's and the total's JSON) and one new seeded CLI transcript
  with its own named golden under `seat_faults` for the total line
  and a row's text (extending `a_rows_place_says_how_long_ago…` would
  change `place_and_its_age.txt` instead), the C117 field test
  keeping its raw-name assertions; `seat_faults.rs:597` (the jobs literal);
  `goldens/web/refresh_loop.txt:36`, `:44` and `web/jobs_halted.txt:10`
  (the rails and jobs text on the web); `crates/acquisition-plan/unit/tests/ages.rs:116`
  (the plan note's age, its direct pin) and
  `goldens/new_league/launch_morning.txt:49`, `:78`, `:113` (its
  text); `price_json.rs:361`, `:816`, `price_cmd.rs:1352`, `:1375`,
  `:1869`, `:1878`, `:1929`, `seat_faults.rs:162` and
  `goldens/seat_faults/price_status_other_leagues.txt` (the "names
  nothing" phrase's holders). The worker lists every holder it finds
  in its report.
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets"
  (a claim wider than its evidence; a sibling sentence left standing).

## Input

- The rulings, above; no new ruling. The owner's answers in hand:
  C146 unchanged (question 5); #12 closed on the sentence (question
  7, the close's action, not the worker's).
- The words, the session's design, each a function of the document's
  own fields (C53):
  1. Events: when `limit_reached`, `the newest N events at M locations
     in the last H h (the window may hold more; a larger \`--limit\`
     reads more events): …`, N the document's `events` and H its
     `hours`; the existing sentence when not; the footer at `:879`
     says `\`acq store events --summary --expand …\` lists every
     location the returned events touched`, the printed command
     carrying the request's account, hours and limit as the refusals
     print theirs (C156), pinned from the default invocation; the raw
     list's `(limit reached)` at `:787` gains the same clause;
     the JSON unchanged (`limit_reached` and `left_out` already say
     it); the gloss says the counts are the returned events' and
     `left_out` the locations'.
  2. Daemon status: `policies learned: N since this daemon started`
     and `sends N since this daemon started`, the journal path's
     clause unchanged; the glosses the same.
  3. Jobs: on the live read, `N jobs this daemon holds (its own and
     the restored): …`; on the ledger read, `3 jobs on disk: 1
     recorded running, 2 waiting; done not recorded`, `done` not
     printed; a selected job's summary keeps its own context; the
     JSON's `done` kept and glossed as the live count; three contexts
     for one `summary_text`, each said where it is printed.
  4. Accounts: `last sign-in or token refresh 19d ago`; the key
     `last_login` kept (C146 bars a rename), its gloss saying both
     writers.
  5. Price status: `N items with a game-side ~skip statement in a
     public tab (a hand row nearer the item still applies)`; the
     gloss the same; a non-public tab's skip text stays residue.
  6. Store refused: `N refused bodies shown of M on record; \`acq store
     refused --account <name> --limit M\` lists them all` where N < M,
     the account resolved and shell-quoted as every printed command
     carries it (C156; two accounts without a selector is the
     standard ambiguity, `documents.rs:203`), `N refused bodies on
     record` where N is M, the one-body-in-full action kept after it;
     the JSON gains `total` beside `returned` and `limit` (additive),
     both counts from `RefusedBodies`; the printed command is run in
     its test over a two-account store.
  7. The receipt's gloss says `in_facts` is "the address exists in
     these facts in any league, removed included"; the status's phrase
     becomes `name nothing in this league's facts` in all three
     producers (the summary line, the expanded list's heading, the
     receipt's quotation) and the status gloss; the JSON key
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
  12. C77's pin: the plan crate's fresh-facts fixture
      (`unit/tests/lib.rs:350`) extended with a second compile at `T +
      cycle`: the plan at `T` skips the fresh fact and the plan at `T +
      cycle` fetches it (`lib.rs:2197`, `:2409–2420` judge against the
      supplied compile time and the window; nothing in the planner
      changes); C77's *Pinned:* names the test (the build replaces
      `nothing yet`, as every build does). This pin covers behaviour
      already correct, so it is the one of Do 1 that need not start
      red.
  13. `FetchedLocation::of` names a location as the row's `Place`
      does: a name that is the remove-only mark alone, or empty, is
      none and the id is shown, the raw name still deciding
      `tab.remove_only` (C117); the one rule in the store, the
      search's `Place` reading it from there if that is simpler than
      two matching rules; pinned on a seeded fixture with such a tab
      in the scope's JSON, the total's JSON, the total line and a
      row's text, all four naming it alike.
  14. The zero-item status, three branches whose guards exclude one
      another: neither listing and `facts.places` 0: `nothing on
      record for <place>: no stash listing, no character listing`;
      neither listing and `facts.places` > 0: `no items on record for
      <place>: N locations, M fetched, none with items; no stash
      listing, no character listing`; a listing exists: `no items on
      record for <place>: N listed, M fetched, none with items`; M
      from `facts.fetched`, `none fetched` where M is 0; the `facts:`
      line stays after each. Five cases staged and their opening
      sentence asserted beside the `facts:` line (`price_cmd.rs:1414`,
      `:1426` cover two, and `:1438` filters the opening sentence out
      today): no location and no listing; locations and no listing
      with M 0 and with M > 0; a listing with M 0 and with M > 0.
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

1. The pins first, red (item 12's excepted, a pin of behaviour
   already right): one test per failing input, in the crate that
   holds the behaviour (C122), the words asserted as goldens where a
   golden holds them; item 1's three cases (a window of exactly the
   limit's events, a non-default limit and window, a limit of zero);
   item 5's two (a public and a non-public tab's skip text).
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
  'lists every event' crates/acquisition-cli/src` finds nothing, and
  every `limit reached` in the CLI's sources carries its clause.
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

## Taken from the reviews

The second cold review, of `9080b9cf`
(`roadmap/reviews/words-extents-brief-9080b9cf.md`): four findings,
no nitpick, each answered in this cut.

1. The events footer names `--summary --expand`, what expands the
   locations, as a printed command carrying the request's account,
   hours and limit, pinned from the default invocation.
2. The refused list's continuation carries the resolved account and
   is run in its test over two accounts.
3. The zero-item status's three branches have exclusive guards and
   five staged cases with the opening sentence asserted.
4. The type rows' five more holders and the remove-only pin's own
   named golden are named.

The first cold review, of `36c4dc9b`
(`roadmap/reviews/words-extents-brief-36c4dc9b.md`): nine findings,
two nitpicks, each answered in the second cut.

1. The events sentence uses the document's own counts and hours and
   promises what a larger limit does, not what `--expand` does; the
   footer and the raw list's words follow; three cases staged.
2. The jobs count names what it holds, this lifetime's and the
   restored, the ledger's running is "recorded", and each of the
   three contexts of one summary gets its own words.
3. The zero-item status's no-listing branch, already pinned
   contradictory, is included, "nothing on record" reserved for no
   location, the fetched count from the facts block.
4. The skip count names the public tabs' game-side statements.
5. The "names nothing" phrase moves in its three producers and the
   gloss, their holders named.
6. The refused list offers the continuation that lists the rest.
7. The remove-only pin names the surfaces as they are since round 2
   and a fixture that exists; the raw name keeps `tab.remove_only`.
8. C77's pin is exempt from starting red, extending the fresh-facts
   fixture with a later compile.
9. The holders missed are named: the events footer and raw list, the
   jobs literal, the web goldens, the plan note's direct pin and its
   texts, the zero-item status's assertions.
