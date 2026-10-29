# 53 — the defence extract's generator, restored

Issue #53. The first of the league group's three in "Pull-forward"
after #63's close (`roadmap/README.md`): in the tree before the
post-launch pin. The rhythm is "A step".

## Goal

`tools/defence-table.py` writes the shipped defence table
(`crates/acquisition-search/reference/defences-v1.toml`) from
`base_items.json` in the `poe1` clone, and at every run refuses unless
its ranges are, row for row, those of the committed extract
`search/repoe/data/base-defences.csv`. That extract's generator,
`base-defences.py`, was deleted at `c756bd38` (step 11f, `search/`
shrunk to what the build names) with three other scripts of the repoe
track. So phase 3b, the regeneration against the post-launch export,
cannot remake the extract from this tree: once the pin is bumped, a new
or changed armour base stops the defence table until someone rewrites
the extract by hand. When this step is done the generator is back under
`search/repoe/scripts/`, reading `base_items.json` from the clone at its
pin through `tools/sources.py`, its output at the pin identical to the
committed extract (a `diff` whose command and empty output the commit
message quotes), with a `--check` that says so; the regeneration
procedure's one home (`tools/game-diff.py`, the table block) names it
before the defence table; and the repoe track's README no longer marks
it deleted. Nothing bumps the pin, regenerates a table, or touches the
daemon, GGG or the owner's store.

## Read

- `AGENTS.md`; `roadmap/README.md` ("A step", "Pull-forward", phase 3b
  in "The phases": the pin bumped, the generators run, every
  generator's `--check` green).
- Issue #53 and its comment (`gh issue view 53 --json body,comments`):
  the two ways it closes; the 3b note, the pin bumped before the
  generators run (C125).
- `search/README.md`, "Rules in force", rule 4: `data/` is regenerable,
  its first line names the script and the input; `scripts/` survives in
  place only if the build names it or a kept table needs it. And
  "Traps": a CSV written with `lineterminator="\n"`.
- `search/repoe/README.md`: line 22, the paragraph naming the four
  scripts deleted at `c756bd38` and `base-defences.csv` as having no
  generator in the tree; line 31, the outputs table's row for it (476
  rows, `base-defences.py`); F8, its findings. The paragraph cites each
  script by today's directory at the parent of `c756bd38`, a path that
  commit never held: `c756bd38` precedes the root swap (`e7b813b8`;
  `git merge-base --is-ancestor c756bd38 e7b813b8` exits 0), and
  `git ls-tree -r --name-only c756bd38^ | grep repoe/scripts/base-defences`
  prints `spikes/rust-playground/search/repoe/scripts/base-defences.py`.
- `search/repoe/MANIFEST.md`: the clone table (`poe1@e2bd511a`,
  every script refusing another commit) and line 21, `base_items.json`
  read by `base-defences.py` (already named; it stands).
- The deleted generator:
  `git show c756bd38^:spikes/rust-playground/search/repoe/scripts/base-defences.py`
  (85 lines; its own `pinned()` pin check, its clone path five
  directories up, its header typing `@ e2bd511a`), and `git log -1
  c756bd38` for why it went: the census kept what a crate, test, tool,
  skill, ruling or record opens, and nothing opened the script.
- `search/repoe/scripts/trade-stat-map.py` and `base-taxonomy.py`: the
  kept generators' current form (`sys.path` to `tools/`, `from sources
  import POE1, require`, `require(...)` in `main()` before the first
  read, a `HEADER` first line).
- `tools/sources.py`: `PINS["poe1"]` (`e2bd511a0133bbe6c1ab548ef1285cb99f3cf0e9`),
  `POE1`, `require` (a clone off its pin, or absent, is a refusal and
  nothing is written).
- `tools/defence-table.py`: the docstring's inputs (line 20, the
  extract, "read only to show that the ranges written here are that
  file's, row for row"), `extract()` (115), the comparison in `main()`
  (160 to 164) that refuses at every run, writing or `--check`.
- `tools/game-diff.py`: `TABLES` (67 to 80) and `PROCEDURE` (81 to 83),
  "the regeneration procedure's one home"; the defences row (70 to 72)
  says the extract's generator "is not in the tree (deleted at
  c756bd38), so a change to a range stops it". The self-test asserts
  the table order and `bump PINS` (lines 393 to 398), not the row's
  words.
- `tools/trade-check.sh`: the gate stage that reads `search/`; it runs
  `category-table.py --check`, the sources and game-diff self-tests and
  `game-diff.py --check`, and no generator that reads the clone
  (`grep -n defence tools/*.sh` prints nothing).
- `decisions/search.md`: C125 (a table drawn from the export names the
  export it was drawn from, read from the clone and never typed; a pull
  is a regeneration in one commit with the pin and the manifest, the
  pin bumped first); C106 and C68 (reference data from a registered
  surface through reviewed tables); C101 (the percentile a derived
  field over reference data).
- `roadmap/briefs/2e-design.md`, section 1.1: the survey that found the
  gap, and the trap beside it, a generator typing the pin into its
  output so that after a bump its header lies.
- `REVIEW-CHECKLIST.md`, "Search": "A claim the code did not make";
  "Evidence read off whatever is on disk" (row 93: every input declared
  and held to its record before any is read).
- The extract's other readers, which the second way of closing would
  move: `search/pseudo-stats/scripts/percentile.py` (line 84, `BASES`),
  `percentile-expected.py` and `percentile-shapes.py` (through
  `percentile.py`'s `BASES`).

## Input

Measured on this tree (`a1d65095`), with the clone beside it:

- `git -C ../poe1 rev-parse HEAD` prints
  `e2bd511a0133bbe6c1ab548ef1285cb99f3cf0e9`, the pin.
- The committed extract: `wc -l search/repoe/data/base-defences.csv`
  prints 478 (the header naming the script, the column row, 476 rows);
  `wc -c` prints 47390; its first line is
  `# generated by scripts/base-defences.py from poe1/data/base_items.json @ e2bd511a`.
- The deleted script, run unchanged against the clone at the pin with
  only its clone and output paths pointed at the scratch
  (`sed` over `git show c756bd38^:…`), printed `export base entries:
  5461; with a non-null defence or block: 476 (474 distinct names)`,
  and `diff search/repoe/data/base-defences.csv <its output>` printed
  nothing and exited 0. So the restored script's proof is expected
  empty; if it is not, the difference is the finding (Do 5).
- `python3 -I tools/defence-table.py --check` and
  `python3 -I tools/game-diff.py --check` each print `ok` and exit 0.

## Do

The choice, the step's own as agent-facing design: **restore the
generator**, the issue's first way. Why: it is the smaller change (one
script back, the defence table's code and its shipped file untouched);
the extract has three other readers under `search/pseudo-stats/scripts/`
that the second way would move or break; the defence table's row-for-row
comparison stays as the check 3b's run meets (it is not a gate stage:
nothing in `tools/` runs `defence-table.py`, since no CI holds the
clone); and 3b's procedure is a generator per input, each then with
`--check`.

1. **Restore** `search/repoe/scripts/base-defences.py` from
   `c756bd38^`, its rule unchanged (every entry carrying a defence
   range or block, ordered by name then id; the fourteen columns; the
   printed counts), adapted to the tree in three ways only:
   - the clone through `tools/sources.py`: `POE1`, and `require(POE1)`
     in `main()` before the first read, in place of its own `pinned()`
     and a path five directories up (the kept generators' form; the
     checklist's row 93);
   - its header's pin read from `PINS["poe1"]`, never typed (C125's
     rule; 2e's trap), which at the pin writes the same first line;
   - `--check`: write nothing, and exit 1 naming the file when the
     committed extract differs from what the clone gives, else print
     one `ok` line with the counts. Design: the proof becomes a
     command anyone reruns, the form `tools/defence-table.py` has, and
     3b's "every generator's `--check` green" covers the extract.
   The docstring says its inputs and outputs as the kept generators'
   do, and why it is back (issue #53, phase 3b).
2. **Run** it with `python3 -I` against the clone at the pin, writing
   in place, then prove the output: `git diff --exit-code --
   search/repoe/data/base-defences.csv` prints nothing and exits 0, and
   `--check` prints `ok`. The commit message quotes both commands and
   their output.
3. **The procedure's one home names it.** `tools/game-diff.py`'s
   defences row says to remake the extract first with
   `search/repoe/scripts/base-defences.py`, which reads the same
   `base_items.json`, so that a moved range is a reviewed diff of the
   extract and the table, never a stop. `tools/defence-table.py`'s
   docstring names the extract's generator beside the extract, as it
   names `trade-stat-map.py` beside its CSV. Nothing else in either
   script changes.
4. **The track's README.** `search/repoe/README.md` line 22: three
   scripts gone, `base-defences.py` restored by this step; each
   deleted script cited at its real path at `c756bd38^`
   (`spikes/rust-playground/search/repoe/scripts/<name>`); `scripts/`
   holds three. The outputs table and F8 stand. No other document
   names the generators' set: `MANIFEST.md` line 21 already names the
   script, and the dated briefs (`2e-design.md`, `2e-build.md`) and
   `STATE-2026-10-19.md` are evidence, never rewritten.
5. **If the proof is not empty**, the extract is not edited to match:
   the differing rows are the finding, reported, and the step stops
   before any commit of the script.
6. The full gate (`tools/gate.sh`) in the background with its output
   under the session's scratch; one commit per concern (1 and 2 as one,
   the script and its proof; 3 and 4 as one, the documents naming it),
   each message naming this brief. Then "Current step" as built, review
   pending, and the Codex review over the range.

## Done when

- `search/repoe/scripts/base-defences.py` exists, calls
  `require(POE1)` before reading, and reads its header's pin from
  `PINS`: `grep -n "e2bd511a" search/repoe/scripts/base-defences.py`
  finds nothing.
- The build commit's message quotes `python3 -I
  search/repoe/scripts/base-defences.py` run in place followed by
  `git diff --exit-code -- search/repoe/data/base-defences.csv` with
  empty output, and the commit does not touch
  `search/repoe/data/base-defences.csv` (`git show --stat` of it lists
  the script alone).
- `python3 -I search/repoe/scripts/base-defences.py --check` exits 0 on
  the step's tree, and exits 1 on an extract one cell different
  (reported, the build measures it on a scratch copy).
- `grep -n "not in the tree" tools/game-diff.py search/repoe/README.md`
  finds nothing; the defences row of `TABLES` names the generator.
- `search/repoe/README.md` cites no `search/repoe/scripts/<name>` at
  `c756bd38^`.
- `PINS`, `search/repoe/MANIFEST.md`'s clone row and every file under
  `crates/acquisition-search/reference/` are unchanged by the range.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- Bumping the `poe1` pin, or regenerating any table or extract: 3b's,
  after the league's export (C125: the pin, the manifest and the
  tables in one commit).
- The second way of closing (the defence table reading the export and
  the extract retired): declined above.
- The other deleted scripts (`hash-check.py`,
  `template-vs-translation.py`, `mod-stat-index.py`): no kept table
  reads their outputs as an input checked at regeneration.
- A gate stage for the clone-reading generators' `--check`: CI holds no
  clone, and `game-diff.py --check` is the gate's drift line by C125's
  design.
- The typed `@ e2bd511a` in the kept generators' headers
  (`trade-stat-map.py`, `base-taxonomy.py`): the report raises it; 3b
  meets it.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; scratch
under this session's scratchpad in a subdirectory named `53`; kill only
the processes you started. No `acq` runs. Python runs as `python3 -I`.

## Report

`TEMPLATE.md`.
