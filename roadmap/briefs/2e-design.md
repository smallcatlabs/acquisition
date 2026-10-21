# 2e — The game-diff tool, a new-league rehearsal and the README line (design)

A Fable design for step 2e of `roadmap/README.md`, drawn from three Opus
surveys of the tree at `36f5e5fb` (the generators, the gate and the
export's local history; the new-league path run by hand in a mock world;
the README, the live-run skill and the ground truth). Every `file:line`
is at `36f5e5fb`. The step's source is the assessment's report 6,
section 5 (`STATE-2026-10-19.md:900-923`): the first, fourth, fifth and
sixth rows of its checklist. The build brief, `2e-build.md`, follows the
design's review; the owner's word is section 8. Nothing here is built.

## 1. The problem, measured against the tree

### 1.1 The tables are frozen at one export, and nothing says when the game moves

Every game-shaped table the search ships was drawn from the owner's
`poe1` clone at `poe1@e2bd511a` (game 3.29.3.3) or from the site captures of
2026-09-12, and all six generator checks pass today (measured
2026-10-21, `python3 -I tools/<x>.py --check`, each exit 0: class 0.27 s,
defence 0.09 s, totals 0.02 s, category 0.02 s, trade-stats 0.09 s,
trade-forms 8.5 s). That settles the first of report 6's unverified
items. But the tree holds no tool that says what changed in the game
between two exports; the gate's only generator check is the category
table's (`tools/trade-check.sh:38-41`), which reads committed files
alone; and the regeneration procedure is stated in eight places and
whole in none (`tools/sources.py:48-50`, `search/repoe/MANIFEST.md:3`,
three generator docstrings, `SURFACES.md:28`, the search crate's
admission test `lib.rs:313-315`, `class.rs:14`), none naming which
generators run or in what order, though order matters: the category
table reads the class table, the forms table reads the stat table.

Four traps wait on the first pin bump. `tools/trade-forms-table.py:244`
types `poe1 e2bd511a (3.29.3.3)` into its table's `source` instead of
reading `PINS` and `version.txt` as `class-table.py:58` and
`defence-table.py:156` do, so after a bump its header would lie and its
`--check` would pass. `search/repoe/MANIFEST.md:13` says
`exported-version.txt` holds a schema commit; that is `schema.txt`'s
content, and `exported-version.txt` holds the game version last
exported, the fork's own lag marker (`../poe1/.github/workflows/export.yml:30,88`:
an export runs only when `version.txt` differs from it; the file exists
since `poe1@3e1570f5`, 2026-08-19). `search/repoe/README.md:20-29` names four
scripts deleted at `c756bd38`, among them `base-defences.py`, whose
output `base-defences.csv` the defence table still reads
(`defence-table.py:104`): a pull cannot regenerate that extract from
this tree. And the pin itself, `poe1@e2bd511a`, is a schema-poll commit five
days after the export it carries (`poe1@bc729bff`): a pin can move with no
data behind it, and, as this evening showed, data can move under a pin.

**This evening's event.** At 19:30 on 2026-10-21 the owner's clone was
pulled from `poe1@e2bd511a` to `poe1@ee2084de` (`git -C ../poe1 reflog`: a
fast-forward pull), an export of 2026-10-19 still at game 3.29.3.3. The
data differs by twelve lines of `stat_translations.json`: two trade
stat ids joined to existing translations (`fractured.stat_2445634879`,
`implicit.stat_3485231932`). Measured after the pull: `class-table.py
--check` refuses, naming the clone's commit and the pin (exit 1),
as do the defence and forms checks, while the gate stays green because
none of the three is in it. That is the drift case the step is for,
met before the step exists: a clone past its pin, tables that may now
be stale, and no line that says so.

### 1.2 The export's whole history is local, and the league transition is in it

The clone holds 590 commits back to 2019; `git show <commit>:data/<file>`
streams any export with no second checkout (base_items 7.5 MB in 0.01 s,
mods 33 MB in 0.13 s, gems 40 MB in 0.14 s; a five-file two-pin diff
ran in about 1.5 s). The last league start is there as a fixed
rehearsal input: `poe1@220a34f7` (Export 3.28.0.16, 2026-07-21) to `poe1@d400e21c`
(Export 3.29.0.2, 2026-07-25). Measured between them: bases +102 and
0 removed with 1,326 changed (1,283 in `properties`, 38 in implicits,
11 in tags, 8 in name); classes +2 (`DeepwaterChart` "Chart",
`CocoonedItem` "Enshrouded Items", both of which `category-table.py:228-230`
would have refused as unplaced); gems +24 with 253 changed; mods +1,044
and 2 removed with 978 changed; stat translations +171 and 3 removed
with 54 English strings changed. Renames happen in place, the same
metadata id with a new name ("Dark Pact" to "Dark Bargain", "Omen of
Blanching" to "Omen of Trichromatism"): a diff keyed by name reports a
removal and an addition where a diff keyed by id reports a rename.
Neither `.0.1` launch version was ever exported; the fork's first
export came 8.5 h after 3.29.0.1 was seen and 53 h after 3.28.0.1
(report F6's median lag 0.5 h and maximum 298 h reproduced).

### 1.3 The new-league path works on the mock, and the mock lies about it

Run by hand in a mock world (`ACQ_PROVIDER=mock`, a fictional league
`Rehearsal`, Standard synced first as the owner's store will be): the
policy written with the realm's leagues whole, then plan and apply
three rounds (1, 7, 7 requests) to "nothing to do"; `tabs`, `search`,
`price status` and `stashes` all read the new league; the journal
records the league only in `path` (`/stash/Rehearsal/maps`), never in
`route` (`stash@ExileTester`), and no new probe, as a new string on a
contacted route needs none (`routes.rs:20-29`). The faults, each a
fact:

1. **The mock moves the fixture's items between leagues.**
   `mock_stash_list` echoes any league name onto the same fifteen tabs
   (`mockggg.rs:476-501`) and `mock_stash` ignores the league
   (`:504-522`), so fetching Rehearsal served Standard's 37 items under
   the same ids, and the store, stamping the request's league and
   recording a same-id item seen under another league as a move
   (`acquisition-store/src/lib.rs:2524-2531`), printed ">18 moved" and
   ">19 moved". Afterwards `tabs` said Standard held 0 items in every
   tab and the planner called it fresh. Launch day cannot do this; a
   rehearsal test on today's mock would pin a false move.
2. **The next step drops the league.** After `refresh --apply --league
   Rehearsal`, `remaining_line` (`plan_cmd.rs:393-401`) prints a bare
   `next: acq refresh --plan`; followed as printed, it plans Standard
   and says "nothing to do" naming no league, because the empty-plan
   renderer (`plan_cmd.rs:755-790`) names none where a non-empty plan's
   header does. The sibling at `plan_cmd.rs:686` prints `--league
   {plan.league}` unquoted where `store_cmd.rs:350` and
   `price_cmd.rs:218` quote.
3. **A wrong reason for no characters.** `store characters --league
   Rehearsal` said "no characters known in …/ExileTester.db (run `acq
   characters` or cover them in the sync policy)" (`store_cmd.rs:396-399`)
   with two characters known in Standard and the policy covering
   Rehearsal's; and the Rehearsal plan, characters covered and none
   listed in the league, printed no characters line at all.
4. **No league name is read against the league list.** `policy set`,
   the plan and the apply said nothing of a league `acq leagues` does
   not list; the store writes the `leagues` table (`lib.rs:1176-1185`)
   and only counts it (`:1788-1792`). A typo'd league sends a stash
   list for a league that does not exist; what GGG answers is
   unrecorded, and if it is a 4xx it spends the undocumented
   invalid-request budget (ground truth N27).

Also measured: `acq leagues` and `acq stashes` print GGG's JSON raw in
text mode (`main.rs:1054-1058`); the mock's league list is Standard and
Hardcore, no challenge league (`mockggg.rs:758-762`); a league with a
space reaches the wire as `%20` and the mock echoes it undecoded, which
the store files correctly; two full syncs within five minutes through
the ad-hoc door met the real-shaped stash hold (313 s, no 429), which
is what launch morning looks like with Standard and the new league
both covered.

### 1.4 The README invites the launch-day mistake, and the live side says nothing of the day

`policy set` replaces a whole realm (`CLI-REFERENCE.md:433`, C36 the
sync policy's replace rule), so a policy naming only the new league
drops Standard. The tour's comment says "the realms named, the rest
kept" (`README.md:160`), read easily as "the leagues named"; the owner's
own driver writes a one-league realm (`tools/tracer-rung.sh:449-456`),
and `RUN-LEDGER.md:67` records the Allflame run doing exactly this and
the owner restoring a merged policy by hand. The "Standard unless you
name a league" bullet (`README.md:207-211`) lists six verbs and leaves
out `refresh --apply`, `refresh --tabs` and `stash`, which default to
Standard too (`CLI-REFERENCE.md:352,386`); the tour lines for the last
two hide `--league` (`README.md:151-152`). The Standard default is a
constant, not a ruling (`acquisition-plan/src/lib.rs:260-262`,
`pricing.rs:123-125`; daemon fallbacks `routes.rs:58,71`,
`kinds.rs:328,440`; five clap defaults and five CLI fallbacks; four MCP
sites); #19 holds the sticky default for phase 4. The live-run skill
never mentions a league or launch day; no ground-truth claim covers
launch-day load (N19 and Q9 are the nearest); and nothing records the
real `/account/leagues` response's fields (N39 records its policy and
counted HEAD alone).

## 2. The shape

Three deliverables, one rule each, and nothing built that phase 3b (the
regeneration) or phase 3 (the live league) delivers.

### 2.1 `tools/game-diff.py`: two refs of the export, keyed by id

One script in `tools/`, read-only, no network ever, reading the `poe1`
clone where `sources.py` finds it (`CLONES`, `ACQ_CLONES`) through
`git show <ref>:<path>`, never a checkout and never `require`, whose
pin check would refuse the very run the tool is for.

- **Refs.** `--old` defaults to `PINS["poe1"]`, `--new` to the clone's
  `HEAD`; any ref the clone resolves is accepted (`origin/master` after
  an owner's `git fetch`, so the working tree can stay at the pin while
  the owner reads what is coming). Both refs are printed resolved, with
  each ref's `version.txt` and, where the file exists, its
  `exported-version.txt`; when the two differ at a ref the tool says
  the fork is between a game version and its export (the lag).
- **Files and keys.** The five files the tables draw on, each keyed by
  the export's own id: `base_items.json`, `gems.json`, `mods.json` and
  `item_classes.json` by their object keys; `stat_translations.json` by
  the sorted tuple of each entry's `ids`. Per file: added, removed,
  renamed (same key, a new `name` or display name), changed (the
  top-level fields that differ, counted by field); under stat
  translations also the `trade_stats` ids added and removed, since the
  forms table joins on them. Classes report both the key and the
  display name, since the class table and the category table read the
  name.
- **The reading for the tables.** After the counts, one fixed block per
  shipped table says what the diff means for it and which generator
  regenerates it, in the order that holds: `base_items`, `gems` and
  `item_classes` feed `classes-v1` (`class-table.py`, first) and
  `base_items` feeds `defences-v1`; the class table feeds
  `categories-v1` (`category-table.py`, after the class table; it
  refuses a class it cannot place, which a new class is); the site's
  stat capture, the owner's, feeds `trade-stats-v1` and that with
  `stat_translations` feeds `trade-game-forms-v1` (`trade-forms-table.py`,
  after the stat table); `mods` feeds no table directly and a new mod
  text counts toward no total (`totals.rs:20-27`), which is said, not
  hidden. This block is the regeneration procedure's one home; the
  eight places that state fragments of it point here afterwards.
- **Output.** Text by default: the refs and versions, one block per
  file with counts first and then names, each list bounded to twenty
  with the rest counted and `--all` lifting the bound; `--json` is the
  whole document. Nothing is written anywhere.
- **`--self-test`.** The tool builds two snapshots of the five files in
  a temporary git repository, as `sources.py --self-test` builds its
  temporary repos (`sources.py:139-200`), staging one addition, one
  removal, one in-place rename, one changed field and one trade-stat
  id added, and asserts the counts and the names. It reads no clone,
  so it runs wherever the gate runs.
- **`--check`, the drift line.** Reads the shipped headers that name an
  export (`classes-v1.toml:25-26`, `defences-v1.toml:34-35`,
  `trade-game-forms-v1.toml` once its generator stamps them read, not
  typed) and compares them with `PINS["poe1"]` and with the clone. It
  passes when no clone is present and says so on its one line
  ("drift: no poe1 clone beside the tree; tables at 3.29.3.3 from
  e2bd511a, unread against the game"); it passes when the clone is at
  the pin and every header names the pin and the clone's version; it
  fails naming what to do when the clone's HEAD is not the pin ("the
  clone is past the pin: run game-diff, regenerate
  in this order, bump PINS and the manifest in one commit, or check the
  pin out again"), when a header's pin or game is not the clone's, or
  when `version.txt` and `exported-version.txt` differ at the pin (the
  tables were drawn from a lagging export, said as such). Both lines
  join the loop at `tools/trade-check.sh:38-41`, the gate's existing
  home for the sitting's tools.

Measured today, the drift line would fail on the owner's machine until
the clone is back at its pin or the pin moves (section 7, the first
item); on CI it passes stating no clone.

### 2.2 The mock: three leagues, and a league that starts empty

The stand-in keeps serving the owner's fixture (2c's ruling, the
committed fixture), and gains what launch day has that it lacks:

- `/account/leagues` lists three: Standard and Hardcore as today, and
  `Rehearsal`, the fixture's challenge league, in the same shape
  (`id`, `realm`, `description`, `category.id`), named so nobody can
  mistake it for a real one.
- The stash list of Standard is today's tree with today's items. The
  stash list of Hardcore and of Rehearsal is the same tree of names,
  types and the folder, with ids distinct per league (a short league
  prefix on each fixture id, `rh-cur1`) and every tab and substash
  empty: the account's purchased tabs, present and bare, as a league
  is on its first morning. No item body changes, so none says a league
  it is not in.
- A league the list does not hold is answered as the mock answers a
  tab it does not hold: 404 on the stash list. That is the mock's
  convention and is documented as such on the mock; what GGG answers a
  stash list for a league that does not exist is an open claim, never
  to be provoked (N27).
- The characters stay two in Standard; a plan covering Rehearsal's
  characters finds none, which is launch morning before the first
  character is made. `whoami`'s hard-coded league is untouched. The
  daemon's fixture test extends: three leagues listed, each non-Standard
  league's tab ids distinct from Standard's and every one empty.

Nothing varies at run time: no knob adds a league. The daemon's
Standard fallbacks stay; the mock never relies on them.

### 2.3 The rehearsal pinned: a second journey, over two leagues

A test beside the journey in the CLI's one integration binary
(`crates/acquisition-cli/tests/journey.rs`, under C122 one harness),
reusing `start_daemon_with` with the journal on, `login_as_mock`,
`Transcript` and its golden, and the one fact table
(`acquisition-client/tests/harness/journey.rs`), which gains the new
league's numbers. Its steps, in the owner's order on launch day:

1. The journey's Standard sync, so the store is his.
2. `acq leagues`: three leagues, Rehearsal among them.
3. `acq policy show`, then `acq policy set` with the pc realm written
   back whole, Standard and Rehearsal; the receipt.
4. `acq refresh --plan --league Rehearsal` (one request, the list; the
   characters line says none are listed in Rehearsal), apply, plan,
   apply, plan, until "nothing to do in Rehearsal"; each `next:` line
   carries `--league Rehearsal`.
5. `acq tabs --league Rehearsal`: every tab fetched, 0 items; `acq tabs`:
   Standard's 37 items still there and no move printed anywhere, the
   fact that pins fault 1.
6. `acq search --count league`, `acq price status --league Rehearsal`,
   `acq store characters --league Rehearsal` (its line names Rehearsal).
7. Bare `acq refresh --plan`: "nothing to do in Standard".
8. The negative case: `acq policy set` naming a league the list does
   not hold (`Typo`), the receipt's note; `acq refresh --plan --league
   Typo` with the note on the plan; the apply's failure naming the
   league and the mock's 404 (never GGG's answer).
9. The journal holds: the harness's `assert_journal_holds` gains a
   per-league tally read from `path`, since `route` hides the league,
   and the GETs are counted per (route, league): Rehearsal's listing
   GETs and tab GETs, no HEAD for the new string, no 429.

The MCP twin (`acquisition-mcp/tests/journey.rs`) checks two fields of
the same table, `refresh_plan`'s `league` and `logical_requests` for
Rehearsal and the `set_sync_policy` receipt's note, and no more: the
semantics are the shared layer's, pinned once below. The one-way check
(`journey.rs:207-221`, every verb run is a tour line) covers the
rehearsal's verbs too, which is what ties the README line to the test.

### 2.4 The words: one rule, six sites

**A line about a league's work names the league, and a printed command
carries `--league` whenever the plan's league is not Standard, quoted
when the name needs it.** The sites: `remaining_line` takes the plan's
league (`plan_cmd.rs:393-401`); the empty plan says "nothing to do in
<league>" (`plan_cmd.rs:755-790`); `plan_cmd.rs:686` quotes as its two
siblings do; `store characters --league X` with none says "no
characters known in X" and keeps the remedy (`store_cmd.rs:396-399`);
a plan covering characters where the listing holds none in the league
says so; and, under ruling R2, the policy receipt and the plan carry a
note when a league the fetched list does not hold is named ("Typo is
not among the leagues fetched 3 m ago (`acq leagues`)", or "the league
list was never fetched"). `acq leagues` renders a list in text mode (id,
realm, category; `--json` unchanged), taken on the owner's word
(section 8), since the README line sends a person there; `acq stashes` stays raw,
not this step's. Every wording is a golden under C122, regenerated under
`ACQ_UPDATE_FIXTURES=1` and read as a diff.

### 2.5 The README, the driver and the skill

- `README.md`, "Before the first command", gains one bullet after the
  Standard one: **A new league.** `acq leagues` names it the day it
  opens; add it to the policy by writing the realm back with every
  league it keeps (a policy naming only the new league drops Standard);
  then `acq refresh --plan --league <name>` and `--apply --league <name>`
  until the plan says nothing to do; a new league's first plan lists
  every tab you own, so read the ceiling first. The Standard bullet
  names every verb that defaults (adding `refresh --apply`, `refresh
  --tabs`, `stash`); the tour line at `README.md:160` says "a realm is
  written whole: name every league it keeps". No new tour line: the
  verbs are there, and docs-check rule 4 holds a tour line to one verb.
- `tools/tracer-rung.sh:442-456` writes the realm back with its league
  added, reading `policy show --json` first, so the owner's phase-3 run
  keeps Standard; with no policy stored it writes as today.
- `.claude/skills/live-run/SKILL.md` gains one paragraph, "A new league
  on a contacted route": no first contact in C59's sense, the ceiling
  from the plan as the rung derives it, `acq leagues` first (its HEAD
  counts, N39), and the record afterwards: a ledger row and a claim on
  the league list's fields, which nothing records today.
- The small corrections the survey found: `trade-forms-table.py` stamps
  the pin and the game from `PINS` and the clone, never typed;
  `search/repoe/MANIFEST.md:13` says what `exported-version.txt` holds;
  `search/repoe/README.md:20-29` marks the four scripts deleted at
  `c756bd38`, so a reader of 3b meets the gap before the pull.

## 3. What changes, by crate

| Crate or place | Changes |
| --- | --- |
| `tools/` | `game-diff.py` (the diff, the reading, `--self-test`, `--check`); `trade-check.sh` two loop entries; `trade-forms-table.py` the header read; `tracer-rung.sh` the realm written back |
| `acquisition-daemon` | the mock's three leagues, per-league empty trees, 404 for an unlisted league; the fixture test extended |
| `acquisition-store` | a read of the `leagues` table's ids and age for the planner and the receipt (one read, the store's read API) |
| `acquisition-plan` | the plan's note for a league the list does not hold (R2); the characters-none line's fact |
| `acquisition-verbs` | the policy receipt's note (R2), so both doors say it once |
| `acquisition-cli` | the six wording sites (2.4); `leagues` rendered; the rehearsal module and golden; the harness tally |
| `acquisition-mcp` | the twin's two field checks; a golden if the receipt's note reaches a tool text |
| `acquisition-client` (tests) | the fact table's league numbers; `assert_journal_holds` per (route, league) |
| documents | `decisions/search.md` R1 and `decisions/plans.md` R2; `README.md` (2.5); the live-run skill; `search/repoe/MANIFEST.md` and `README.md`; `REVIEW-CHECKLIST.md` one row for the per-realm replace, the fault that came back (the driver, the Allflame row, the tour comment); `CLI-REFERENCE.md` regenerated for `leagues` |

## 4. The rulings this needs

**R1 — the tables and the pin.** For `decisions/search.md`, the owner's
word (it binds the gate and the owner's clone):

> - **C<next> — A table drawn from the game's export names the export it
>   was drawn from, read from the clone and never typed, and the gate
>   refuses a clone that has moved past the pin.** `tools/game-diff.py
>   --check` fails when the `poe1` clone beside the tree is at another
>   commit than `PINS`, when a shipped header's pin or game version is
>   not the clone's, or when the pinned export lags the game it names;
>   it passes, stating so, where no clone is present. A pull is a
>   regeneration in the same commit as the pin and the manifest. *Why:*
>   the tables went stale silently at every patch, and a clone pulled
>   under a pin (2026-10-21) left three generators refusing and the gate
>   green. *Pinned:* `game-diff.py --self-test` and `--check` in
>   `trade-check.sh`; the class table's test on `game()`.

**R2 — a league the list does not hold.** For `decisions/plans.md`:

> - **C<next> — A policy may name a league the account's fetched league
>   list does not hold; the policy receipt and the plan say so, and
>   nothing refuses.** The note names the list's age, or that it was
>   never fetched. *Why:* the list is a snapshot, stale on launch
>   morning before `acq leagues` and empty on a fresh store, and a
>   refusal would block the user the day the league is new; the note is
>   what keeps a typo from spending the invalid-request budget (N27).
>   *Pinned:* the rehearsal's negative case.

**No ruling needed:** the mock's three leagues and empty trees (2c's
fixture ruling covers what the stand-in serves; the 404 convention is a
mechanism on the mock's doc); the wording rule (agent-facing, under
delegation, listed here for review); the README bullet and the skill
paragraph; the driver; the tests. `AGENTS.md` does not change.

## 5. Alternatives weighed and declined

- **A second checkout or worktree of `poe1` for the old pin.** `git show`
  streams any export in under a second; a worktree is a second thing to
  pin.
- **Keying the diff by name.** Renames in place would read as removal
  plus addition; the id is the export's own identity.
- **A drift line that only warns (exit 0).** The gate commits on exit
  statuses alone (`AGENTS.md`, the gate decides); a warning no status
  carries is a line nobody reads.
- **Reading the clone in CI.** No workflow checks one out and
  `SURFACES.md` has the owner clone and pull; the line states absence
  instead.
- **Bumping the pin to `poe1@ee2084de` now.** That is 3b's regeneration,
  with its named inputs; 2e builds the tool that makes 3b a reading.
- **A knob for the mock's league list** (`ACQ_MOCK_LEAGUES`). Declined
  as 2c declined a fixture path: a knob for a need no test has;
  `ACQ_MOCK_DEGRADED_HEAD` stages a fault, not data.
- **The mock echoing any league, as today.** It is the false move.
- **Refusing a league the list does not hold.** The list is stale by
  design on the one morning it matters (R2's why).
- **A `--self-test` over the real transition.** It needs the clone; the
  3.28-to-3.29 numbers are a measurement in the build's commit message
  and a row in the track's README instead.
- **A full MCP twin of the rehearsal.** The semantics are pinned once
  below the doors; the twin checks two fields.
- **Rendering `acq stashes` as well.** A different document; nothing in
  the README sends a person to it.
- **A tour line for the new league.** A tour line is one verb (docs
  check rule 4); the sequence is a bullet.

## 6. Risks and what holds them

- **The gate goes red on the owner's machine.** The clone is past the
  pin tonight; `--check` fails there until the pin is checked out again
  or 3b bumps it. Held by the line saying which; raised first in
  section 7.
- **The rehearsal's request counts.** They depend on the mock's tree
  and the stash windows left by the Standard sync; the fact table fixes
  them in the build, run ten times and diffed before the golden lands,
  as 2c did.
- **The journal tally reads `path`.** A change to the path's form breaks
  it; the harness already pins the route form and gains the path's.
- **The mock's 404 is not GGG's answer.** Said on the mock and in the
  test's name; a claim when a live run ever meets it, never provoked.
- **`exported-version.txt` is absent before `poe1@3e1570f5`.** The tool falls
  back to `version.txt` and the commit subject and says which it read.
- **A league diff is wide.** 1,044 mods at 3.29; the output bounds every
  list and `--all` lifts the bound.
- **Launch-day load is unmodelled** (no Cloudflare, no 503 in the mock;
  the rails read strings alone, `rails.rs:216-253`). Not this step's;
  the README's clock sentence and the skill's ceilings are what hold it.
- **The twelve lines under the pin.** A regeneration at `poe1@ee2084de` would
  change the forms table; 3b's reviewed diff is where that is read.

## 7. Raised for the owner, each with its home

1. **The clone.** Checked out at the pin by the owner, 2026-10-21
   (measured: `rev-parse HEAD` is `poe1@e2bd511a`, `class-table.py
   --check` 0). Resolved.
2. **R1 and R2**, section 4: approved 2026-10-21. The build writes
   them to the registry before the code.
3. **`acq leagues` rendered** in text mode: the owner asked for advice
   as a primary user of the CLI; advised taken (the README line sends
   a person there to copy one name; the JSON stays for agents) and
   taken.
4. **The driver's merge**: advised taken (the Allflame run's hand
   restore; one read and one insert; the test does not depend on it)
   and taken.
5. **The Standard default is unruled** (section 1.4). #19 holds the
   sticky default for phase 4; nothing now unless he wants a line.
6. **`base-defences.py` is gone** and 3b cannot regenerate
   `base-defences.csv`: an issue at the close, `area:search`,
   `trigger:live`, for 3b's brief.
7. **What GGG answers a stash list for a league that does not exist**,
   and **the real `/account/leagues` fields**: two open claims for
   phase 3's live run, recorded in the skill's after-list.
8. **`bothy pack --path README.md`** returns the search and trade
   rulings (`tools/bothy.py:284-305` matches the basename against
   intros naming `search/README.md`): tooling, no step named, for the
   roadmap's "Open".
9. **The assessment's "league is opaque (C61)"** at `STATE-2026-10-19.md:880`
   names no such ruling; dated evidence, left standing, noted here.

## 8. The owner's word

On 2026-10-21, after reading the brief: R1 and R2, "approved"; the
clone, "i have checked out e2bd511a"; on the two delegated items,
"what do you advise for these as one of the primary users of the
cli?", and both taken on the advice in section 7; the rhythm, "commit,
then update, then commit after updating and start a review." Earlier
the same day, opening the step: "a Fable brief first, surveys before
the brief, nothing built until I say so."

## Appendix A — the rehearsal by hand (2026-10-21, mock world, `acq 1.0.0-alpha.1`)

| Step | Command | Exit | Said |
| --- | --- | --- | --- |
| 1 | `acq leagues` | 0 | raw JSON: Standard, Hardcore |
| 2 | `acq policy set` (Standard + Rehearsal, the realm whole) | 0 | revision 2; no note on Rehearsal |
| 3 | `acq refresh --plan --league Rehearsal` | 0 | 1 request, list stashes; no characters line |
| 4 | apply | 0 | "next: `acq refresh --plan` — 7 requests more" (league dropped) |
| 5 | plan, apply | 0 | 7 tabs; "changed: >18 moved at 8 locations" |
| 6 | plan, apply | 0 | 7 substashes; ">19 moved at 14 locations"; "left: nothing" |
| 7 | plan | 0 | nothing to do; no league named |
| 8 | `acq tabs --league Rehearsal` / `acq tabs` | 0 | 37 items / 0 items in every tab, fresh |
| 9 | `acq store characters --league Rehearsal` | 0 | "no characters known in …db (run `acq characters` …)" |
| 10 | `acq search --count league` | 0 | 37 Rehearsal, 12 Standard (the characters') |
| 11 | `acq show <id> --body` | 0 | place Rehearsal; body `"league":"Standard"`: the request wins |
| 12 | `acq price status --league Rehearsal` | 0 | 37 items in Rehearsal; next line keeps the league |
| 13 | `acq refresh --plan --league Hardcore` (not in the policy) | 1 | "the sync policy does not cover league Hardcore on realm pc" |
| 14 | `acq refresh --all --league Rehearsal` | done | held ~313 s on the stash window (14 + 14 + 2 GETs in 300 s); no 429 |
| 15 | `acq stashes --league 'Hardcore Rehearsal'` | 0 | wire `/stash/Hardcore%20Rehearsal`; the mock echoed `%20`; the store filed the name |

Journal: 54 sends; league in `path` alone; one HEAD per route during
the Standard sync, none for Rehearsal; three token POSTs.

## Appendix B — the export's history, measured

| From | To | Bases | Classes | Gems | Mods | Translations |
| --- | --- | --- | --- | --- | --- | --- |
| `poe1@220a34f7` 3.28.0.16 | `poe1@d400e21c` 3.29.0.2 | +102 −0, 1,326 changed | +2 | +24, 253 changed | +1,044 −2, 978 changed | +171 −3, 54 changed |
| `poe1@926da1c0` 3.29.3.2 | `poe1@e2bd511a` 3.29.3.3 | 0 | 0 | 0 | +1, 1 changed | 0 |
| `poe1@e2bd511a` 3.29.3.3 | `poe1@ee2084de` 3.29.3.3 (2026-10-19) | 0 | 0 | 0 | 0 | 2 trade-stat ids added |

Files at `poe1@e2bd511a`: `base_items.json` 8.0 MB, 5,461 keys;
`item_classes.json` 17.7 KB, 103; `gems.json` 40.9 MB, 1,458;
`stat_translations.json` 12.6 MB, 11,257 entries; `mods.json` 34.3 MB,
40,355. `json.load` under `python3 -I`: 0.03, 0.00, 0.17, 0.04, 0.15 s.
The fork polls every four hours (`export.yml:8`) and exports only on a
version change (`:30`).

## Appendix C — the citations the step edits

`tools/trade-check.sh:38-41`; `tools/trade-forms-table.py:225,244`;
`tools/tracer-rung.sh:442-456`; `search/repoe/MANIFEST.md:13`;
`search/repoe/README.md:20-29`; `crates/acquisition-daemon/src/mockggg.rs:476-522,758-762`;
`crates/acquisition-cli/src/plan_cmd.rs:393-401,686,755-790`;
`crates/acquisition-cli/src/store_cmd.rs:396-399`;
`crates/acquisition-cli/src/main.rs:1054-1058`;
`crates/acquisition-client/tests/harness/journey.rs` (the table) and
`harness/mod.rs:630-697` (`assert_journal_holds`);
`crates/acquisition-cli/tests/journey.rs:207-221`; `README.md:151-152,160,207-211`;
`.claude/skills/live-run/SKILL.md`; `decisions/search.md`,
`decisions/plans.md`, `REVIEW-CHECKLIST.md`. Tests that assert on the
words this step changes, found by grepping the old words: the empty
plan's "nothing to do" and `remaining_line`'s text in
`crates/acquisition-cli/tests/goldens/` and `apply_loop.rs`; the
characters line in `seat_faults.rs`; the build brief lists each by
file once the words are fixed.
