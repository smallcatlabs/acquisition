# 157-leagues-held — what the store holds beyond the league an answer took (#157, #22)

Round 5 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§3 Root 2 and §10 step S5), written by the orchestrating session on
2026-11-04 against the tree at `c80caee6` (round 4 closed; every line
number below from it). Its rulings are C118 as amended and C162
(`f3db3427`), C126 (a policy may name a league the list does not
hold; the plan says so) and C146 (a count says what it counts); the
owner accepted the map's §11 question 6, (a): the same verb and tool
read the stored league list offline when asked, each league beside
whether the policy covers it. The JSON shapes and the text's words
below are the session's agent-facing design, listed at the close for
his review. Cut three times: the first draft (`dae7aa60`) and the
re-cut (`d38e3762`) were each reviewed cold
(`roadmap/reviews/157-leagues-held-brief-<sha>.md`, the prompts
beside: eight findings then six, three nitpicks in all); this cut
answers the second review's six, listed at the foot under "Taken from
the reviews" with the first's eight; not reviewed a third time, the
roadmap's rule. Round 3 (`briefs/every-epoch-words.md`) and this
round build in either order, one at a time; whichever lands second
reconciles the other's changes to the policy document's assertions
and the `documents.rs` walk, named in its report.

## Goal

When this step is done, a surface that takes one league says what
else the store holds: `price status`, `shop render` and `tabs` each
carry a `leagues_held` block (every league of the realm with tabs or
characters on disk, its counts, and whether the policy covers it) and
print one line naming the league they read and the others held;
`policy show` carries the same beside the policy's leagues; the plan's
wrapper counts the league's tabs the policy does not name beside the
covered set (#22); `acq leagues --stored` and the MCP's `leagues` tool
read the stored league list without a daemon, each league beside
whether the policy covers it and what the store holds of it, and the
README's new-league line names that read (#157); `store status`
gains the latest list's count and age beside the fields it has; and
C126's league match is by realm and league where the list's entry
carries a realm, by id where it does not. #157 and
#22 close by their own lines; the league seat's comment on #19 is
answered, #19 itself (phase 4's sticky default) stays open.

Why it matters: the league seat's runners inferred the new league from
its category because no read of the stored list exists (#157, both
MCP runs); `price status` and `shop render` without `--league` read
Standard and named no other league, so Allflame's unreadable notes
appeared only when asked by name (the CLI-2 seat's fault 6, #19's
comment); a plan's `skipped_tabs: []` beside 3,347 tabs read as full
coverage (#22). The search answer's scope block already says all of
this (`realms_held`, `leagues`, the pointer); the league-taking
surfaces say none of it.

## Read

- The map §3 Root 2 (the idea, what it settles, the gaps no issue
  names: `shop render` without `--league`, `policy show` with a
  policy, `store status`'s union), §10 S5, §11 question 6.
- `decisions/plans.md`: C126 (the list is the latest successful
  leagues response, never an accumulated table; the note names its age
  or that it was never fetched) and its mechanism
  `crates/acquisition-plan/src/lib.rs:722–760` (`LeagueNote`,
  `league_notes`: the match is by id over the list's ids, realm-blind
  at 748); C36, C37 (the policy's shape). `decisions/frontends.md`:
  C146, C53, C149, C162. `decisions/store.md`: C39 (a neutral read),
  C108. `AGENTS.md`: C1, C46 (every frontend is an adapter over the
  verbs crate's door), C116, C122, C154.
- `crates/acquisition-store/src/lib.rs:793–828` (`LeagueList`: the
  latest leagues response's `fetched_at` and `ids`; `league_list_in`
  reads the latest 2xx response's envelope and keeps each entry's `id`
  alone, 820–827, where the real endpoint's entry and the mock's carry
  a `realm` too: `crates/acquisition-daemon/src/mockggg.rs:325–336`),
  `:2046` (`Store::league_list`), `:846–856` (`Status`: `leagues` a
  count of the union table, `league_list` a string, `fetched` or
  `never fetched`, both kept: JSON is additive, C53), `:1418–1430`
  (the `leagues` table, an upsert union across responses), `:578–582`
  (`store status` prints the count beside the string), `:671–693` and
  `:696–715` (`TabRow`, `CharacterRow`: realm and league per row;
  `Store::tabs(realm, league)`, `characters`). `crates/acquisition-store/src/bound.rs`
  (`LISTED`, the one bound every list takes, C53).
- `crates/acquisition-cli/src/web/mod.rs:356–372` (the web's
  `leagues(realm)`: the stored list, each id marked with whether the
  realm has tabs of it on disk: the one existing read, by a frontend,
  which C46 says belongs in the verbs crate), `:414–421`, `:539` (the
  two pages' league controls).
- `crates/acquisition-cli/src/main.rs:1672` (`acq leagues` submits a
  job in its own arm, before `connect`: the `--stored` branch returns
  before it), `:594–602` (`price status`'s `--league`, Standard by
  default), `:841` (the policy flags' words). `crates/acquisition-mcp/src/main.rs`
  (the tools table: `tabs` at 359, `price_status` at 391,
  `set_sync_policy` at 435 are the refusal-request mappings; the
  tools themselves are `tabs` at 1500, `set_sync_policy` at 1753,
  `price_status` at 1817, and the policy read `sync_policy` at 1742,
  which takes `StoreParams` (832–838: the standard optional `account`
  and nothing else) and gains `expand`; `AcqMcp::account`/`facts`,
  1428–1438; the `leagues` job through `submit_job`; the `describe`
  entries); `crates/acquisition-cli/src/main.rs:833` (`PolicyCmd::Show`
  takes no argument and gains `--expand`); `crates/acquisition-mcp/tests/mcp_reference.rs`
  (a new tool reaches `MCP-REFERENCE.md` through the generator) and
  `surface_size.rs:32` (the first turn's texts held to their bounds:
  the new tool's text counts).
- `crates/acquisition-cli/src/price_cmd.rs:201` (`rows: … N for other
  leagues, M for other realms`: the count exists, the leagues are not
  named), `:338` (`render_status`'s head, since round 4);
  `shop_cmd.rs:135–215` (the freshness lines, the `facts:` line first
  since round 4; nothing printed with nothing posted, 138, and the
  early return with no items, 253–260: on those paths the `leagues:`
  line follows the head); `store_cmd.rs:296–330` (`tabs`' head and
  `no_tabs_text`), `:578–582` (`store status`'s text);
  `crates/acquisition-cli/src/diagnose_cmd.rs:71` (reuses the status
  text, so `diagnose` prints the new words too);
  `crates/acquisition-search/src/tabs.rs:44`, `:139`, `:172` (one
  league per listing). `policy show`: `crates/acquisition-verbs/src/frontend.rs:500–525`
  (`PolicyShown`, the row whole) and its CLI and MCP renderings (grep
  `PolicyShown`).
- The plan: `crates/acquisition-verbs/src/refresh.rs:35–50`
  (`Planned`: `plan`, `ages`, `quote_note`, `notes`: the wrapper, C38
  keeps the envelope; `notes` is the planner's `PlanNotes`,
  `crates/acquisition-plan/src/lib.rs:775–800`, so the new observation
  is a field of the verbs' wrapper, not of `PlanNotes`), `:449–458`
  (`plan`: the policy plan from `self.snapshot`, the notes from the
  same snapshot) and `:465–490` (`compile_selection`,
  `plan_selection`: a selection plan with no stored policy, which
  carries no such observation; a plan read from a file carries none
  either); `crates/acquisition-plan/src/lib.rs:655–665` (`covers_tab`
  over a `Selection`, a league entry's tab selection) and `:712–713`
  (`SyncPolicy::league(realm, league)`: whether the policy names the
  league); `:2240–2248` (compile keeps the covered tabs);
  `crates/acquisition-store/src/snapshot.rs:140` (`RefreshSnapshot.tabs`,
  the league's live tabs whole); `crates/acquisition-cli/src/plan_cmd.rs:343–355`
  (under `--json` stdout is the envelope alone, the notes on stderr)
  and `:879–890` (the text: a plan with no actions prints its notes
  too; the new line prints on every path).
- `README.md:226–240` ("Standard unless you name a league"; "A new
  league": `acq leagues` names it the day it opens) and
  `crates/acquisition-cli/tests/readme_tour.rs:52–137` (every tour
  line resolves under clap).
- The tests and goldens that pin these texts, each changed by this
  step: `goldens/seat_faults/price_status_other_leagues.txt` and
  `tests/seat_faults.rs:142`; `goldens/price_json/list_show_and_refusals.txt`
  (`price_json.rs:278` records a status answer); `goldens/shop_json/shop_render.txt`,
  `shop_json.rs:158`; `goldens/seat_faults/tabs_counts_first.txt`,
  `tabs_pointer_names_its_league.txt`, `tabs_never_fetched_of_a_type.txt`
  (`seat_faults.rs:841`) and `crates/acquisition-search/tests/tabs.rs`
  (the document's shape; there is no CLI `tests/tabs.rs`);
  `goldens/new_league/launch_morning.txt` and `new_league.rs:44`
  (the morning: the first policy, the first plans);
  `goldens/journey/a_first_session.txt`; `plan_json.rs:104–120`
  (`policy show`'s row); `goldens/seat_faults/store_status_counts.txt`;
  the inline positional tests `price_cmd.rs:1234` (its assertion at
  1316: `facts:` is line 1, so the `leagues:` line is line 2 and the
  `sides:` line moves to 3), `shop_cmd.rs:528` (its assertion at 571:
  `facts:` first among the freshness lines, `leagues:` second, the
  coverage line moves down); `price_cmd.rs:1701` is an item-show
  assertion this step does not touch;
  `crates/acquisition-mcp/tests/pricing_tools.rs`, `search_tools.rs`
  (the `tabs` tool), `surface_size.rs:32`, `mcp_reference.rs:77`
  (`MCP-REFERENCE.md`); `crates/acquisition-verbs/tests/documents.rs:1234`
  (the walk over every document gains the new ones) and `:1106`,
  with `crates/acquisition-cli/tests/store_answers.rs:288` (each
  asserts the policy answer equals `{"policy": null}`: it now carries
  `leagues_held` beside `policy`, so each asserts the new shape);
  `crates/acquisition-cli/tests/diagnose.rs:173` and
  `goldens/diagnose/no_daemon.txt:32` (the status text through
  `diagnose`); the `LeagueList` constructors in
  `crates/acquisition-store/unit/tests/lib.rs:58`, `:94`, `:111` and
  `crates/acquisition-plan/unit/tests/ages.rs:10` (each gains
  `entries`); and the grep by
  command (`price status`, `shop render`, `tabs`, `policy show`,
  `leagues`, `store status`) over `crates/*/tests` and `crates/*/src`,
  the empty-scope paths included (a render with nothing posted, a
  status with no items): the `leagues:` line prints on them too.
- `REVIEW-CHECKLIST.md`, "Store and plans", "Pricing and intent",
  "Frontends".

## Input

- The rulings, above; the owner's answer to question 6 (a). No new
  ruling. C126's words stand; its match gains the realm: the real
  endpoint's entry and the mock's carry `realm`
  (`mockggg.rs:325–336`), the store's read keeps it (the shapes
  below), `league_notes` and `SyncPolicy::leagues` carry `(realm,
  league)` and match an entry by both where it has a realm, by id
  where it has none (a stored list from before this step).
- The shapes, the session's design:
  - the store read, neutral (C39): `Store::leagues_held(realm) ->
    Vec<LeagueHeld { league: Option<String>, tabs, characters }>`, one
    query over the live tabs and characters of the realm, league-less
    characters under a `None` league, in the store's order; a unit
    test over a fixture of rows. its callers count the whole before the bound. `LeagueList` gains `entries:
    Vec<LeagueEntry { id, realm: Option<String> }>` read from each
    entry's `id` and `realm` (`ids` stays, additive), `None` where an
    entry carries no realm.
  - the verbs fragment `LeaguesHeld { realm, policy, leagues: [{
    league, tabs, characters, named }], left_out, left_out_needs }`:
    `policy` is `none`, `unreadable` or `set` (with its revision);
    `named` is whether `SyncPolicy::league(realm, league)` is `Some`,
    the policy names the league, whatever its entry covers (a league
    entry may cover characters alone or some tabs; `named` says only
    that it is in the policy); `leagues_total` counts every league of
    the realm on disk, the list is bounded by `LISTED` with `left_out`
    counted and `left_out_needs: expand` (C53, C149), the league read
    first, in one of the ten slots, and the rest in the store's order,
    `expand` listing all (`--expand` on `price status`, `shop render`
    and `tabs` as today, and on `policy show` new; `expand` on the
    MCP's `price_status`, `shop_render`, `tabs` as today and on
    `sync_policy` new); made once in the verbs crate and carried by `price
    status`, `shop render` and `tabs` under `leagues_held`, and by
    `policy show` under the same key once per realm the store holds,
    the realms the policy does not name included.
  - the text, one line after the `facts:` line on `price status` and
    `shop render` and after the head on `tabs`: `leagues: Standard
    (this); also held: Allflame (3 tabs, named), Rehearsal (12 tabs, 2
    characters, not named), no league (4 characters)`, ten named and
    the rest counted (`… and 3 more: \`--expand\` lists them`), `none
    other` where the realm holds one league; `no policy` or `policy
    unreadable` in place of `named`/`not named` as `policy` says; on
    `policy show`: `held: pc Standard (named), Allflame (not named);
    xbox Standard (no policy for xbox)`.
  - `acq leagues --stored` (a flag on the job's verb, branching before
    `connect` in its arm, `--expand` beside it) and the MCP tool
    `leagues` (new, no daemon; arguments `stored: true` required, the
    standard optional `account`, and `expand`): `LeagueListShown {
    fetched_at, fetched_at_age, policy, leagues_total, leagues: [{ id,
    realm, named, held: { tabs, characters } }], left_out,
    left_out_needs }`, bounded as above, `policy` the same state as
    `LeaguesHeld` carries, `named` whether the policy names the entry
    in its realm, or in any realm where the entry has none, and
    `false` throughout where `policy` is `none` or `unreadable`,
    `leagues_total` the list's length, from `Store::league_list` and
    `leagues_held` (a league's `held` in its own realm where the entry
    has one, else in every realm, summed); never fetched: `{
    fetched_at: null, fetched_at_age: "never", policy, leagues_total:
    0, leagues: [] }` and the text `no league
    list on record; \`acq leagues\` fetches it` (the job as today,
    `--stored` absent). The text: `league list fetched 2d ago: Standard
    (pc, named, 405 tabs on disk), Allflame (pc, not named, 3 tabs),
    Rehearsal (pc, not named, nothing on disk)`, ten and the rest
    counted; the web's league control keeps its complete list through
    the same read with `expand`.
  - the plan wrapper: `Planned.uncovered_tabs: Option<{ count, of }>`
    (the verbs' own field, beside `notes`, never in the envelope, C38;
    not in the planner's `PlanNotes`): the league's non-folder tabs
    the policy's entry does not cover (`covers_tab` on the entry's tab
    selection, none covered where the entry has no tab selection)
    against all its non-folder tabs, from the same snapshot `plan`
    compiles over (`refresh.rs:449–458`); `None` on a selection plan
    and on a plan read from a file. The text: `outside the policy:
    3,343 of 3,347 tabs (\`acq policy set …\` names them)` after the
    skipped line, on every text path including a plan with no actions
    (`plan_cmd.rs:879–890`), absent where the count is 0; under
    `--json` it goes to stderr with the notes (`:343–355`), stdout
    stays the envelope.
  - `store status`: `leagues` and `league_list` stay as they are;
    the JSON gains `league_list_latest { count, fetched_at,
    fetched_at_age }` from `LeagueList` (null and `never` where none);
    the text's `league list N leagues` becomes `league ids seen across
    every league fetch: N (the latest fetch: 3 leagues, 2d ago)` or
    `… (never fetched)`, so the kept count names its set (C146); the
    same words reach `diagnose`.
  - the README's "A new league" line opens with `acq leagues
    --stored` (what the store holds and the policy names, no daemon)
    and keeps `acq leagues` for the fetch; the tour line resolves.
- The failing inputs the step's tests stage, in the verbs tests'
  worlds and the CLI's seeded worlds: a realm with three leagues on
  disk, a policy covering one: `price status` without `--league`
  names the other two with counts and coverage; the same for `shop
  render` and `tabs`; a realm with one league: `none other`; no
  policy: `no policy`; an unreadable policy: `policy unreadable`; a
  realm with twelve leagues on disk: ten named, two counted,
  `leagues_total` 12, `expand` listing all on each of the four
  surfaces and on the MCP's four tools; league-less characters: the
  `no league` row; `policy
  show` with a policy naming one realm over a store holding two: the
  second realm's leagues with `no policy for <realm>`; a policy
  entry covering characters alone: `named`; `acq leagues --stored`
  with a fetched list, a policy and tabs on disk; with no list ever
  fetched; a list of twelve: bounded, `leagues_total` 12; a list
  entry with no realm and a policy naming its league under one realm:
  `named`; no policy and an unreadable policy: `policy` says so and
  `named` is `false`; the MCP tool the same and
  a two-account store with and without `account` (the standard
  ambiguity refusal); a plan over a league of 10 non-folder tabs and
  2 folders with a policy naming 2: `uncovered_tabs { count: 8, of:
  10 }` and the line, on a plan with actions and on one with none; a
  policy naming every tab: no line; a selection plan and a plan from a
  file: no field; `store status` with and without a list; C126's
  match: a policy naming `Standard` under `xbox` where the list holds
  `Standard` for `pc` alone notes it; a list entry with no realm
  matches by id.
- The last push's CI, on `main` at `c80caee6`: the runs named at the
  cut that starts the build (this brief's commit is before them); a
  red run after a push is the group's first concern.

## Do

1. The store read `leagues_held` and `LeagueList.entries`, each with
   its unit test (the constructors in the store's and the planner's
   unit tests gaining `entries`); the verbs crate's `LeaguesHeld`
   fragment with the policy's `named` and state and its whole count,
   and `LeagueListShown` from the stored list, both bounded, each a
   document the walk in `documents.rs` covers; the web's
   `leagues(realm)` replaced by the verbs crate's read with `expand`
   (C46: the frontend had opened a store read of its own).
2. `price status`, `shop render` and `tabs`: `leagues_held` in the
   JSON, the `leagues:` line in the text after the `facts:` line, or
   after the head where no `facts:` line prints; `policy show`: the
   same beside the row, `--expand` on the CLI and `expand` on the
   MCP's `sync_policy` listing the leagues whole.
3. `acq leagues --stored` (clap: the flag and `--expand` beside the
   job's verb, branching before `connect`; the verb without it submits
   the job as today) and the MCP tool `leagues` with `stored`,
   `account` and `expand`; its `describe` entry and tool text within
   the first turn's bound.
4. The plan wrapper's `uncovered_tabs` on the policy plan alone, and
   the line on every text path, stderr under `--json`.
5. `store status`'s `league_list_latest` and the words beside the
   count it keeps.
6. C126's match by `(realm, league)` where the entry carries a realm,
   by id where it does not; the note's tests.
7. The README's new-league line and the tour; the glosses (no id in
   any, C116); the holders of Read moved to their new facts; then,
   once, `ACQ_UPDATE_FIXTURES=1` over the workspace, the two
   references regenerated in that same run, every changed line read:
   a `leagues:`, `held:`, `league list` or `outside the policy` line,
   a `# holds` line, an added JSON key or subtree, the two references'
   prose for the new tool and flag, the README's line.
8. The gate; the commit naming this brief, the changed lines by kind,
   and `retired nothing` (the web's own read replaced, not retired: a
   C46 edge restored, named in the message). #157 and #22 close.

## Done when

- `Store::leagues_held` and `LeagueList.entries` exist with their
  unit tests; `LeaguesHeld` and `LeagueListShown` are bounded verbs
  documents the `documents.rs` walk covers; `grep -n 'league_list'
  crates/acquisition-cli/src/web/mod.rs` finds no store read of the
  web's own.
- The three surfaces and `policy show` carry `leagues_held` and print
  the line, the empty-scope paths too; `acq leagues --stored` and the
  MCP `leagues` tool answer the cases of Input, the account selector
  among them; `Planned.uncovered_tabs` is on the policy plan alone and
  the line prints on every text path when nonzero; `store status`
  keeps `leagues` and `league_list` and gains `league_list_latest`.
- Each failing input of Input is a test; the README's tour line
  resolves; both references regenerated once, in Do 7; the new tool's
  text within `surface_size.rs`'s bound.
- Every regenerated line is one of Do 7's kinds; `git diff` of
  `Status`'s `leagues` and `league_list` fields is empty.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- A sticky default realm or league (#19's own body, phase 4).
- An all-leagues `tabs` view: the `leagues:` line names the leagues to
  ask for; one league per listing stays.
- The union `leagues` table and `Status.leagues`, which stay as they
  are: their reader is kept (additive JSON), and a retirement is a
  schema change, ruled first.
- The words round's sweep (S6) and the named-paths pin (round 3).

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names; never while another round's worker builds.
- `ACQ_UPDATE_FIXTURES=1` runs once, after Do 1 to 6 and the `# holds`
  edits, every changed line read before the commit.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the regenerated files by directory, the
changed lines by kind, the two references' size before and after,
and the test count before and after.

## Taken from the reviews

The second cold review, of `d38e3762`
(`roadmap/reviews/157-leagues-held-brief-d38e3762.md`): six findings,
two nitpicks, each answered in this cut.

- A. `policy show --expand` and `expand` on the MCP's policy read;
  both documents carry their whole count; the league read takes one
  of the ten slots.
- B. The stored-list document carries the policy's state; `named` is
  defined for a realm-less entry and under no or an unreadable policy.
- C. The policy answer's two equality assertions, `diagnose`'s reuse
  of the status text with its golden, and the `LeagueList`
  constructors are named with their new facts.
- D. The kept status count is named for its set, league ids seen
  across every fetch.
- E. The `leagues:` line follows the head where no `facts:` line
  prints.
- F. The citations corrected: the MCP tools' lines, `render_status`'s,
  the item-show assertion dropped, the status text's file.

The first cold review, of `dae7aa60`
(`roadmap/reviews/157-leagues-held-brief-dae7aa60.md`): eight
findings, one a nitpick, each answered in the second cut.

1. `store status` keeps `leagues` and `league_list` as they are and
   gains `league_list_latest`; the union's reader is kept.
2. The store's list read keeps each entry's realm, and C126's match
   is by `(realm, league)` where an entry has one, by id where not.
3. The predicate is `SyncPolicy::league`, the boolean is `named`, the
   fragment carries the policy's state, and the characters-only entry
   is staged.
4. Both league lists are bounded by the one bound with their counts
   and `expand`; the web's control keeps the whole list.
5. The MCP tool takes the standard `account` and `expand`; the
   two-account case is staged.
6. The observation is the verbs wrapper's own field, from the policy
   plan's snapshot with the entry's tab selection, absent on selection
   and file plans, printed on every text path, stderr under `--json`.
7. The holders are complete: the status answer in the price golden,
   the never-fetched-type golden, the search crate's `tabs.rs`, the
   first-turn bound, the positional assertions at their lines and
   the lines they move, the empty-scope paths.
8. Regeneration happens once, in Do 7.
