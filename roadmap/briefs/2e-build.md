# 2e — The game-diff tool, a new-league rehearsal and the README line (build)

The build brief for the design in `2e-design.md`, which the worker
reads first and whole. A worker reads `roadmap/README.md`, then this
brief, then what it names, and nothing else unless the work turns it
up.

## Goal

A maintainer can read what changed in the game between two exports and
what each shipped table does about it, and the gate refuses a clone
that has moved past its pin; the mock lists a challenge league whose
tabs exist and are empty, and a test walks launch morning over two
leagues the way the owner will on 2026-11-13; every line about a
league's work names the league; and the README says how to start a
new league without dropping Standard. Phase 3 runs the league with
these in hand; phase 3b regenerates the tables with the tool in hand.

## Read

- `2e-design.md` whole: section 2 is the shape (2.1 the tool, 2.2 the
  mock, 2.3 the rehearsal, 2.4 the words, 2.5 the documents), section
  3 the changes by crate, section 4 the two rulings as drafted, 6 the
  risks, 7 what is resolved and what is not yours, appendix A the
  rehearsal by hand, B the export's measured history, C the citations.
- `AGENTS.md`: the build order (build before any daemon, again after
  tests), the gate, "Rulings first, findings after", "No id in
  user-facing text" (C116). The commit order below is the gate's, every
  time.
- `decisions/search.md` (R1's home; its intro and the entry rule in
  `tools/bothy.py`'s docstring, 800 bytes a bullet) and
  `decisions/plans.md` (R2's home; C36, the sync policy's replace rule,
  the shape the receipt reports under). `decisions/pricing.md` C68,
  reference data as a fifth input, which R1 extends.
- `tools/sources.py` whole: `PINS`, `CLONES`, `absent`, `require`, and
  `--self-test`'s temporary repositories (`:139-200`), the shape
  `game-diff.py --self-test` copies. `tools/trade-check.sh:36-46`, the
  loop the two new lines join (one token per flag).
- `tools/class-table.py:50-120` and `tools/trade-forms-table.py:150-260`:
  how a generator reads the clone and stamps its header; the forms
  generator's typed pin at `:225` and `:244` is what you replace.
- `search/repoe/MANIFEST.md` and `search/repoe/README.md:1-40`: the
  clone row, the mislabelled line (`MANIFEST.md:13`) and the four
  scripts deleted at `c756bd38`.
- `crates/acquisition-daemon/src/mockggg.rs`: the tab table and its doc
  (`:214-235`), `mock_stash_list` (`:476-501`), `mock_stash` (`:504-522`),
  the league list (`:758-762`), `classify_data_path` (`:336-390`), the
  fixture tests (`:1108`, `:1197`, `:1219`). 2c's ruling on what the
  stand-in serves, in `decisions/daemon.md`.
- `crates/acquisition-store/src/schema.sql:10-22,41-46` and
  `lib.rs:1170-1192,1785-1795`: the responses envelope, the `leagues`
  table that upserts, the status counts. C108, the store's read API,
  for the shape a new read takes.
- `crates/acquisition-plan/src/lib.rs:1123-1160` (`RefreshPlan`), the
  coverage ruling's doc (`:142`) and `SkipReason` (`:885-902`): where a
  plan's note lives and why an empty substash is skipped.
- `crates/acquisition-verbs/src/frontend.rs:300-360` (`set_policy`, the
  receipt both doors print).
- `crates/acquisition-cli/src/plan_cmd.rs:380-410,680-690,750-795`,
  `store_cmd.rs:390-400`, `main.rs:1050-1060`: the six wording sites
  and the `leagues` verb.
- `crates/acquisition-client/tests/harness/mod.rs:140-165` (the
  isolation list), `:397` (`start_daemon_with`), `:630-697`
  (`assert_journal_holds`); `harness/journey.rs` whole (the fact
  table); `crates/acquisition-cli/tests/journey.rs` whole (`step`,
  `Transcript`, `golden`, the one-way tour check at `:207-221`) and its
  golden `goldens/journey/a_first_session.txt`;
  `crates/acquisition-mcp/tests/journey.rs` (the twin).
- `README.md:138-184` (the tour), `:194-222` ("Before the first
  command"); `crates/acquisition-cli/tests/readme_tour.rs:53-181` (how a
  tour line is found and held); `tools/docs-check.sh:257-294` (rule 4,
  the tour's form and the knobs).
- `tools/tracer-rung.sh:430-460` (the policy the driver writes).
- `.claude/skills/live-run/SKILL.md` whole (the paragraph you add sits
  after "The standing rule: first contact").
- `REVIEW-CHECKLIST.md`, "Store and plans" (the row you add).
- Tests that assert on the words this step changes, found by grepping
  the old words: "more are planned now" and "nothing to do" in
  `acquisition-cli/tests/journey.rs`, `seat_faults.rs`, `plan_json.rs`
  and the goldens `journey/a_first_session.txt`,
  `seat_faults/a_spent_plan_applied_again.txt`,
  `seat_faults/an_apply_ends_on_what_is_still_planned.txt`,
  `seat_faults/the_second_round_leaves_nothing_planned.txt`;
  `acquisition-verbs/tests/documents.rs` ("nothing to do" from
  `refresh.rs`); "no characters known" has no test. The search crate's
  `surface_faults.rs` and the daemon's `dispatch.rs` say "nothing to
  do" of other things and are not touched.

## Input

- The tree at `cb7db91c`. Every line range in this brief and the design
  is at that commit or the design's `36f5e5fb`; confirm each before you
  edit and name any the design misplaces in your report.
- The `poe1` clone at its pin `poe1@e2bd511a` (the owner checked it out
  2026-10-21; `python3 -I tools/class-table.py --check` exits 0 there).
  The clone is read by `git show`, never checked out, never fetched.
- The export's two rehearsal inputs, both in the clone's local history,
  with the counts the design measured (appendix B): `poe1@220a34f7` to
  `poe1@d400e21c` (bases +102 and 0 removed, classes +2, gems +24, mods
  +1,044 and 2 removed, translations +171 and 3 removed), and
  `poe1@e2bd511a` to `poe1@ee2084de` (two trade-stat ids added, nothing
  else). The tool's run over each is a measurement in commit 2's
  message, never a test.
- The failing inputs `--self-test` stages: one base added, one
  removed, one renamed in place (same key, new name), one with a
  changed field, one trade-stat id added under an existing
  translation; and for `--check`: a clone at another commit than
  `PINS`, a header whose game is not the clone's, a clone absent
  (`ACQ_CLONES` to an empty directory), each exercised in the self-test
  through the temporary repository.
- The mock's facts the rehearsal fixes once measured: the challenge
  league `Rehearsal`; its tab tree the fifteen of Standard with a
  league prefix on every id, every tab empty, no substash stub; the
  rounds to "nothing to do" (expected two after the listing: the
  listing, then the fourteen non-folder tabs; measured in the build and
  written to the fact table); Standard's 37 items untouched after.
- The rehearsal's negative case: a policy naming `Typo`; the receipt's
  note; the plan's note; the apply's failure naming the league on the
  mock's 404.

## Do

In this order, one commit each unless a line says otherwise; the gate
before each.

1. **The rulings.** Write R1 to `decisions/search.md` and R2 to
   `decisions/plans.md` as the design drafts them (section 4), each a
   bullet in the entry rule's form with the next free id, *Why:* and
   *Pinned:* naming the files below by a path from `crates/` or
   `tools/`. `bothy check` green. Commit: "2e rulings".
2. **The tool.** `tools/game-diff.py` as section 2.1: `--old`, `--new`,
   the five files keyed by id, added, removed, renamed, changed by
   field, the trade-stat ids; the versions and the lag line; the fixed
   "for the tables" block in generator order, which is the procedure's
   one home; text bounded to twenty a list with `--all`; `--json`;
   `--self-test`; `--check` with its three outcomes and the recovery
   order (bump `PINS` and the manifest, regenerate, commit together; or
   check the pin out). Two lines in `trade-check.sh`'s loop:
   `gamediff tools/game-diff.py --self-test` and
   `drift tools/game-diff.py --check`. `trade-forms-table.py` stamps
   the pin and the game read from `PINS` and the clone, its `--check`
   still green on the committed table (the header text changes only
   if the bytes it reads differ: say which in the message).
   `search/repoe/MANIFEST.md:13` corrected; `search/repoe/README.md:20-29`
   marks the four scripts gone at `c756bd38` and that `base-defences.csv`
   has no generator in the tree. The eight places that state a fragment
   of the procedure (design 1.1) each gain one clause pointing at the
   tool, no more. Run the tool over the two rehearsal inputs and quote
   its counts in the message beside the design's. Commit: "2e
   game-diff".
3. **The mock.** Section 2.2: three leagues listed; per-league empty
   trees with prefixed ids and no substash stubs; 404 for a league not
   listed, said on the mock's doc as the mock's convention and not
   GGG's; the fixture test extended (three leagues; each non-Standard
   league's ids distinct, empty, stub-free). The existing journey must
   pass unchanged: Standard is untouched. Commit: "2e mock leagues".
4. **The words and the note.** Section 2.4: the store's one read (the
   ids and fetch time of the latest successful leagues response, from
   its envelope; unit-tested on a league that left the list and a list
   fetched empty); the plan's note and the receipt's note under R2
   (the verbs crate, so both doors say it once; the MCP's
   `set_sync_policy` and `refresh_plan` carry the same field); the six
   wording sites; `acq leagues` rendered in text (id, realm, category;
   `--json` unchanged). Regenerate the goldens the Read names under
   `ACQ_UPDATE_FIXTURES=1`, read each diff, and name in the message
   every line that changed and the source change that explains it;
   regenerate `CLI-REFERENCE.md` and `MCP-REFERENCE.md` the same way.
   Commit: "2e the league named".
5. **The rehearsal.** Section 2.3: a module `new_league` in the CLI's
   one test binary (`tests/main.rs`), its golden under
   `goldens/new_league/`, the fact table's new constants, the harness's
   per-(route, league) tally read from `path`, the MCP twin's two
   fields. Run the rehearsal ten times and diff the transcripts before
   the golden is committed; a difference is a finding in the report,
   never a normaliser rule. The one-way tour check covers the
   rehearsal's verbs. Commit: "2e rehearsal".
6. **The documents and the driver.** Section 2.5: the README bullet,
   the Standard bullet's three verbs, the tour comment at `:160`;
   `tracer-rung.sh` writing the realm back whole; the live-run skill's
   paragraph with its two open claims; the `REVIEW-CHECKLIST.md` row
   ("A realm written one league at a time", the driver, the Allflame
   row, the tour comment; held by the README bullet and the rehearsal's
   policy step). This commit carries the step's ledger line and a
   "Current step" saying the review is pending (`roadmap/README.md`,
   "A step"). Commit: "2e documents, ledger".
7. **The review.** Launch Codex over `<commit 1>^..<commit 6>` as
   `roadmap/README.md` says, save its findings verbatim at
   `roadmap/reviews/2e-<sha>.md`, commit that file alone under
   `git diff --check`, and quote the findings in your report. Fix
   nothing before the owner has read them.

## Done when

- `python3 -I tools/game-diff.py --self-test` exits 0 and prints its
  five staged differences and the three `--check` outcomes by name;
  `tools/trade-check.sh` runs both new lines and exits 0 on the step's
  tree with the clone at its pin, and `ACQ_CLONES=<empty dir>
  tools/trade-check.sh` exits 0 with the drift line stating no clone.
- `python3 -I tools/game-diff.py --old poe1@220a34f7 --new poe1@d400e21c`
  (the refs bare, the `poe1@` form is the citation's) prints the
  design's counts; commit 2's message quotes them and the e2bd511a to
  ee2084de run.
- `grep -n 'e2bd511a\|3\.29\.3\.3' tools/trade-forms-table.py` finds
  nothing but a comment that cites, never a value the header takes.
- `decisions/search.md` and `decisions/plans.md` each hold one new
  bullet; `bothy check` green; the registry count in its output is
  126.
- The daemon's fixture test asserts three leagues listed and, for each
  non-Standard league, ids distinct from Standard's, every tab empty,
  no substash; `cargo nextest run -p acquisition-daemon` green.
- The store's unit tests `a_league_that_left_the_list_is_not_held` and
  `a_list_fetched_empty_is_a_fetched_list` (or names as plain) exist
  and pass.
- `grep -rn 'next: `acq refresh --plan`' crates/acquisition-cli/src`
  finds the line that takes the league; the goldens the Read names
  differ only where commit 4's message says; `acq store characters
  --league X` with none says the league; `acq leagues` text is a list,
  held by the rehearsal's golden.
- `crates/acquisition-cli/tests/new_league.rs` exists, is a `mod` of
  `tests/main.rs`, and its golden shows: three leagues; the receipt
  for a policy with both leagues; plans naming Rehearsal to "nothing
  to do in Rehearsal" with every `next:` line carrying the league;
  `tabs --league Rehearsal` with every non-folder tab fetched and 0
  items; `tabs` with 37 items and no "moved" anywhere; the bare plan
  "nothing to do in Standard"; the Typo receipt's note, plan note and
  the apply's failure naming Typo. The journal assertion counts GETs
  per (route, league).
- The MCP twin compares `refresh_plan`'s `league` and
  `logical_requests` for Rehearsal and the receipt's note field to the
  fact table.
- `README.md` holds the new bullet, the Standard bullet names
  `refresh --apply`, `refresh --tabs` and `stash`, and line 160's
  comment says a realm is written whole; `readme_tour.rs` green;
  `docs-check.sh` green.
- `tools/tracer-rung.sh --mock` on a world holding a Standard policy
  leaves Standard in the policy after writing its league (quoted in
  commit 6's message from `acq policy show`).
- The live-run skill has the paragraph and the two open claims;
  `REVIEW-CHECKLIST.md` the row; `roadmap/README.md` the ledger line
  and "Current step".
- The gate green at every commit on the step's own tree; the Windows
  gate's statuses in the report, or that the guest was down.

## Not in scope

- A pin bump or any regeneration: 3b's, with its named inputs. The
  clone stays at `poe1@e2bd511a`.
- A generator for `base-defences.csv` (an issue at the close).
- Any Standard default, in the CLI, the MCP, the plan crate or the
  daemon (#19, phase 4).
- The mock's `whoami` league, the characters' leagues, a knob for the
  league list, any fault injection (Cloudflare, 503) in the mock.
- `acq stashes` rendered; the ad-hoc refresh door.
- What GGG answers a stash list for a league that does not exist: the
  mock's 404 is its own; never provoke it live.
- A normaliser rule for anything the ten rehearsal runs vary on.
- The regeneration of `trade-game-forms-v1.toml`, even though
  `poe1@ee2084de` would change it.
- `AGENTS.md`; `HISTORY.md` (the phase close's).

## Rules

`TEMPLATE.md`, with these differences:

- This step runs `acq` in a mock world under the harness's isolation
  list and by hand under the mock-session skill; the main tree and
  your own scratch subdirectory, named at launch.
- Commit 2's measurements over the real clone are run with the clone at
  its pin and read by `git show`; if `git -C ../poe1 rev-parse HEAD` is
  not `e2bd511a0133bbe6c1ab548ef1285cb99f3cf0e9`, stop and say so.
- The design's drafted rulings are taken verbatim unless the tree
  disproves a sentence; the id is the next free one at commit time.
- A golden's diff is read line by line before it is committed; a line
  no source change in the commit explains is a finding.
- zsh: `${c}:path`, never `$c:path`, in a `git show` loop (the `:e`
  modifier bit the survey).

## Report

`TEMPLATE.md`'s, with, under 3 (measurements): the tool's counts over
the two rehearsal inputs beside the design's; the rehearsal's round
count and request counts; the ten-run diff; the time `--self-test` and
`--check` add to `trade-check.sh`.
