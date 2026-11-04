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
his review. To be reviewed cold before a worker reads it.

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
README's new-league line names that read (#157); `store status`'s
league-list count says which list it counts and how old it is; and
C126's league match is per realm where the list carries one. #157 and
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
- `crates/acquisition-store/src/lib.rs:793–812` (`LeagueList`: the
  latest leagues response's ids and `fetched_at`; `league_list_in`
  reads the latest 2xx response's envelope), `:2046` (`Store::league_list`),
  `:1418–1430` (the `leagues` table, an upsert union across responses),
  `:578` (`store status`'s `league list N` counts that union),
  `:671–693` and `:696–715` (`TabRow`, `CharacterRow`: realm and
  league per row; `Store::tabs(realm, league)`, `characters`).
- `crates/acquisition-cli/src/web/mod.rs:356–372` (the web's
  `leagues(realm)`: the stored list, each id marked with whether the
  realm has tabs of it on disk: the one existing read, by a frontend,
  which C46 says belongs in the verbs crate), `:414–421`, `:539` (the
  two pages' league controls).
- `crates/acquisition-cli/src/main.rs:1672` (`acq leagues` submits a
  job; the list is read back by `job_result`), `:594–602` (`price
  status`'s `--league`, Standard by default), `:841` (the policy
  flags' words). `crates/acquisition-mcp/src/main.rs` (the tools
  table: `tabs` at 359, `price_status` at 391, `set_sync_policy` at
  435; `sync_policy`; the `leagues` job through `submit_job`; the
  `describe` entries).
- `crates/acquisition-cli/src/price_cmd.rs:201` (`rows: … N for other
  leagues, M for other realms`: the count exists, the leagues are not
  named), `:278–300` (`render_status`'s head); `shop_cmd.rs:135–215`
  (the freshness lines, the `facts:` line first since round 4);
  `store_cmd.rs:296–330` (`tabs`' head and `no_tabs_text`);
  `crates/acquisition-search/src/tabs.rs:44`, `:139`, `:172` (one
  league per listing). `policy show`: `crates/acquisition-verbs/src/frontend.rs:500–525`
  (`PolicyShown`, the row whole) and its CLI and MCP renderings (grep
  `PolicyShown`).
- The plan: `crates/acquisition-verbs/src/refresh.rs:35–50`
  (`Planned`: `plan`, `ages`, `quote_note`, `notes`: the wrapper, C38
  keeps the envelope); `crates/acquisition-plan/src/lib.rs:655–665`
  (`covers_tab`), `:2240–2248` (compile keeps the covered tabs),
  `crates/acquisition-store/src/snapshot.rs:140` (`RefreshSnapshot.tabs`,
  the league's live tabs whole); `crates/acquisition-cli/src/plan_cmd.rs`
  (the summary's `skipped` line; grep `skipped_tabs`).
- `README.md:226–240` ("Standard unless you name a league"; "A new
  league": `acq leagues` names it the day it opens) and
  `crates/acquisition-cli/tests/readme_tour.rs:52–137` (every tour
  line resolves under clap).
- The tests and goldens that pin these texts, each changed by this
  step: `goldens/seat_faults/price_status_other_leagues.txt` and
  `tests/seat_faults.rs:142`; `goldens/shop_json/shop_render.txt`,
  `shop_json.rs:158`; `goldens/seat_faults/tabs_counts_first.txt`,
  `tabs_pointer_names_its_league.txt` and `tests/tabs.rs`;
  `goldens/new_league/launch_morning.txt` and `new_league.rs:44`
  (the morning: the first policy, the first plans); `goldens/journey/a_first_session.txt`;
  `plan_json.rs:104–120` (`policy show`'s row); `goldens/seat_faults/store_status_counts.txt`;
  `crates/acquisition-cli/src/price_cmd.rs:1234`, `:1701`,
  `shop_cmd.rs:528` (inline positional tests); `crates/acquisition-mcp/tests/pricing_tools.rs`,
  `search_tools.rs` (the `tabs` tool), `mcp_reference.rs:77`
  (`MCP-REFERENCE.md`); `crates/acquisition-verbs/tests/documents.rs:1234`
  (the walk over every document gains the new one); and the grep by
  command (`price status`, `shop render`, `tabs`, `policy show`,
  `leagues`, `store status`) over `crates/*/tests` and `crates/*/src`.
- `REVIEW-CHECKLIST.md`, "Store and plans", "Pricing and intent",
  "Frontends".

## Input

- The rulings, above; the owner's answer to question 6 (a). No new
  ruling. C126's words stand; its match gains the realm where the
  stored envelope carries one (the worker reads a stored `leagues`
  envelope in the mock world's fixtures, `acquisition-daemon`'s mock
  provider, to see whether each league entry names its realm; where
  it does not, the match stays by id and the brief's Do 6 says so in
  the note's gloss).
- The shapes, the session's design:
  - the store read, neutral (C39): `Store::leagues_held(realm) ->
    Vec<LeagueHeld { league, tabs, characters }>`, one query over the
    live tabs and characters of the realm, league-less characters
    counted under a `None` league, in the store's order; a unit test
    over a fixture of rows.
  - the verbs document fragment `LeaguesHeld { realm, leagues:
    [{ league, tabs, characters, covered }] }`, `covered` from the
    policy as `covers_tab`'s realm-and-league lookup says, `false`
    with no policy; made once in the verbs crate and carried by
    `price status`, `shop render` and `tabs` under the key
    `leagues_held`, and by `policy show` under the same key, per realm
    the store holds, the realms the policy does not name included.
  - the text, one line after the head on each of the three: `leagues:
    Standard (this); also held: Allflame (3 tabs, covered), Rehearsal
    (12 tabs, 2 characters, not covered)`, every league of the realm
    on disk but the one read, each with its counts and `covered` or
    `not covered` (`no policy` where none is set), `none other` where
    the realm holds one league; on `policy show`: `held: pc Standard
    (covered), Allflame (not covered); xbox Standard (no policy for
    xbox)`.
  - `acq leagues --stored` and the MCP tool `leagues` (new, no
    daemon, `stored` its only argument, true): `LeagueListShown {
    fetched_at, fetched_at_age, leagues: [{ id, realm?, covered,
    held: { tabs, characters } }] }` from `Store::league_list` and
    `leagues_held`; never fetched: `{ fetched_at: null,
    fetched_at_age: "never", leagues: [] }` and the text `no league
    list on record; \`acq leagues\` fetches it` (the job as today,
    `--stored` absent). The text: `league list fetched 2d ago: Standard
    (covered, 405 tabs on disk), Allflame (not covered, 3 tabs),
    Rehearsal (not covered, nothing on disk)`.
  - the plan wrapper: `notes.uncovered_tabs { count, of }`, the
    league's non-folder tabs the policy does not cover against all its
    non-folder tabs, from the snapshot the planner already holds
    (`snapshot.tabs` and `covers_tab`), beside the covered set; the
    summary's text gains `outside the policy: 3,343 of 3,347 tabs
    (\`acq policy set …\` names them)` after the skipped line, absent
    where the count is 0.
  - `store status`: `league list N leagues` becomes `league list: N
    leagues in the latest fetch, 2d ago` from `LeagueList` (the latest
    response), `never fetched` where none; the JSON's `league_list`
    counts that list and gains `league_list_fetched_at(_age)`; the
    union table stays what it is and is no longer printed as a count.
  - the README's "A new league" line opens with `acq leagues
    --stored` (what the store holds and the policy covers, no daemon)
    and keeps `acq leagues` for the fetch; the tour line resolves.
- The failing inputs the step's tests stage, in the verbs tests'
  worlds and the CLI's seeded worlds: a realm with three leagues on
  disk, a policy covering one: `price status` without `--league`
  names the other two with counts and coverage; the same for `shop
  render` and `tabs`; a realm with one league: `none other`; no
  policy: `no policy`; `policy show` with a policy naming one realm
  over a store holding two: the second realm's leagues named with `no
  policy for <realm>`; `acq leagues --stored` with a fetched list, a
  policy and tabs on disk; with no list ever fetched; the MCP tool the
  same two; a plan over a league of 10 non-folder tabs and 2 folders
  with a policy naming 2: `uncovered_tabs { count: 8, of: 10 }` and
  the line; a policy naming every tab: no line; `store status` with
  and without a list; the C126 note on a stored envelope with realms
  (or the gloss, Input's first bullet).
- The last push's CI, on `main` at `c80caee6`: the runs named at the
  cut that starts the build (this brief's commit is before them); a
  red run after a push is the group's first concern.

## Do

1. The store read `leagues_held` with its unit test; the verbs crate's
   `LeaguesHeld` fragment with the policy's coverage, and `LeagueListShown`
   from the stored list, each a document the walk in `documents.rs`
   covers; the web's `leagues(realm)` replaced by the verbs crate's
   read (C46: the frontend had opened a store read of its own).
2. `price status`, `shop render` and `tabs`: `leagues_held` in the
   JSON, the `leagues:` line in the text; `policy show`: the same
   beside the row.
3. `acq leagues --stored` (clap: the flag beside the job's verb; the
   verb without it submits the job as today) and the MCP tool
   `leagues` with `stored: true`; its `describe` entry and tool text;
   `MCP-REFERENCE.md` and `CLI-REFERENCE.md` regenerated.
4. The plan wrapper's `uncovered_tabs` and the summary's line; the
   plan's goldens read.
5. `store status`'s league-list line and fields.
6. C126's match per realm where the envelope carries one, else the
   gloss saying the match is by id across realms; the note's test.
7. The README's new-league line and the tour; the glosses (no id in
   any, C116); the goldens regenerated under `ACQ_UPDATE_FIXTURES=1`
   once after Do 1 to 6 and the `# holds` edits, every changed line
   read: a `leagues:`, `held:`, `league list` or `outside the policy`
   line, a `# holds` line, an added JSON key, the two references'
   prose, the README's line.
8. The gate; the commit naming this brief, the changed lines by kind,
   and `retired nothing` (the web's own read replaced, not retired: a
   C46 edge restored, named in the message). #157 and #22 close.

## Done when

- `Store::leagues_held` exists with its unit test; `LeaguesHeld` and
  `LeagueListShown` are verbs documents the `documents.rs` walk covers;
  `grep -n 'league_list' crates/acquisition-cli/src/web/mod.rs` finds
  no store read of the web's own.
- The three surfaces and `policy show` carry `leagues_held` and print
  the line; `acq leagues --stored` and the MCP `leagues` tool answer
  the two cases; the plan wrapper carries `uncovered_tabs` and the
  summary prints the line when nonzero; `store status` says which list
  and how old.
- Each failing input of Input is a test; the README's tour line
  resolves; both references regenerated.
- Every regenerated line is one of Do 7's kinds.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- A sticky default realm or league (#19's own body, phase 4).
- An all-leagues `tabs` view: the `leagues:` line names the leagues to
  ask for; one league per listing stays.
- The union `leagues` table's retirement (it feeds nothing printed
  after Do 5; its removal is a schema change, ruled first).
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
