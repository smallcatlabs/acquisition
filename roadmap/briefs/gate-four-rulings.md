# gate-four-rulings — What the local gate and the release run

## Goal
A session pushes on a local green (C171, commit and push on a green
gate), so the local gate holds what a push can turn red and only a
local tree can see. When this step is done the full gate runs `cargo
doc` as its `doc` stage; its `trade` stage runs the class, defence and
forms tables' `--check` wherever the `poe1` clone sits at its pin; the
`artifacts` stage stands, recorded as kept; and the macOS Keychain round
trip has one definition, called by the release and by the weekly run,
and has run once on its own. Closes #257 and #225.

## Inputs
- The owner's rulings of 2026-11-23: the last comment of each issue
  (`gh issue view 257 --comments`, `gh issue view 225 --comments`), his
  words, the four recommendations as put and the costs each rested on;
  #220 names them (`gh issue view 220 --comments`).
- What they reverse: his ruling of 2026-10-29, `git show 4b54e8c7` (doc
  out of the full gate) and `roadmap/briefs/t2-gate-trims.md` (the
  step, which also retired `artifacts`).
- C170 (the gate's stages are the owner's), C175 (a gate or workflow
  round is reviewed cold), C154 (a retirement names its holder), C192
  (an entry states a property), C84 (the artifact mismatch). The entry
  form is `tools/bothy.py`'s docstring; the next id is the highest
  `python3 -I tools/bothy.py list` prints, plus one.
- Code: `tools/gate.sh` (header, `names`, `ONLY`, `usage`,
  `run_stage`), `tools/gate-breakers.sh`, `tools/trade-check.sh` and
  `tools/trade-check-breakers.sh`, `tools/game-diff.py` (`check`: the
  drift line's three outcomes), `tools/sources.py` (`require`, `PINS`),
  `.github/workflows/release.yml`, `weekly.yml`, `gate-linux.yml`.
- The design session's dispatch of the release workflow on `main` at
  `51d99a3d`, the Keychain job's first run: `gh run view 45329267444`,
  its `keyring-macos` job's conclusion read and said in the report.
- CI: `gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  the same for `Gate (Windows)`; both were running on `51d99a3d` when
  this was written. A red run is this step's first concern.

## Decisions
The design session's (C172, who designs), settled with the owner:
1. **`doc` is a stage of the full gate,** after `winclippy` and before
   `fmt`, in the table and the record. `--docs` stays the five stages
   that build no Rust; `--only doc` still names it. `AGENTS.md`'s stage
   list and "Quality gate" commands gain it, and its sentence that CI
   alone runs `cargo doc` goes. Every standing text that tells a session
   to run `--only doc` by hand loses that line; a brief or a record
   already written is history and is not edited. The gate's breakers
   stage a broken intra-doc link the stage must refuse.
2. **`artifacts` is unchanged; the record changes:** the registry says
   the stage is kept and why, in the owner's words of 2026-11-23.
3. **The three `--check`s run in `tools/trade-check.sh`, as one stage
   of it.** The clone beside the tree at its pin: they run and must
   pass. No clone: skipped, with one line saying so, as the drift line
   is, so CI's run is unchanged. A clone off its pin: the stage fails
   and says so, since that is a bump in progress, which regenerates its
   tables. The header's "are not here" is rewritten to say where they
   run. `tools/trade-check-breakers.sh` stages a table that has left
   its generator.
4. **One definition of the Keychain job:** a reusable workflow of its
   own (`on: workflow_call` and `workflow_dispatch`), called by
   `release.yml`, whose `draft` still needs it, and by `weekly.yml`; no
   copy of its steps in two files. Its doc comment names the three
   places it runs.
5. **Rulings first:** the build's first commit records the four in
   `decisions/process.md`, each in the entry form with the owner's words
   quoted and dated, before any code that depends on them. An entry that
   already rules the same thing is amended, never contradicted by a
   second. (The entry form's rule 5 refuses a date or a quote in an
   entry: raised with the design session with this brief; build what it
   answers.)
6. **Closes #257 and #225;** a comment on #220 says the four are ruled
   and built, so its close no longer waits on them.

## Find first
- `git grep -n -e 'only doc' -e 'cargo doc' -e 'doc stage' -e 'doc job' -- AGENTS.md .claude roadmap/briefs/TEMPLATE.md roadmap/briefs/REVIEW-PROMPT.md roadmap/briefs/BRIEF-REVIEW-PROMPT.md REVIEW-CHECKLIST.md tools .github`:
  every standing text about the doc stage, CI's doc job among them.
- `git grep -n -e 'build test clippy' -e 'Its stages' -e "sed -n '4,10p'" -- AGENTS.md tools`:
  every list of the stages, the check holding `AGENTS.md`'s to `names`,
  and the usage the script prints by line number.
- `git grep -n -i -e retire -e artifacts -- decisions AGENTS.md .claude REVIEW-CHECKLIST.md GLOSSARY.md`
  and `python3 -I tools/bothy.py list | grep -i -e gate -e stage`:
  what the record says of `artifacts`, and the entries that rule the
  stages, to amend rather than contradict.
- `git grep -n -e 'not here' -e poe1 -e drift -- tools/trade-check.sh tools/trade-check-breakers.sh`
  and `git grep -n 'require(' -- tools/class-table.py tools/defence-table.py tools/trade-forms-table.py`:
  the header to rewrite, the drift line's skip as the model, and the
  clone each generator requires.
- `git grep -n -e keyring-macos -e keyring_round_trip -- .github crates tools roadmap/README.md`:
  every place the job or its test is named.
- `git grep -n -i workflow -- tools`: every check that reads a workflow
  file, which a moved job must still satisfy.

## Done when
- `decisions/process.md` records the four, in the first commit, before
  any code; `python3 -I tools/bothy.py check` passes.
- `tools/gate.sh`: `names` holds `doc` between `winclippy` and `fmt`;
  `--only doc` and `--docs` select as before; the header says the full
  gate runs it; `tools/gate.sh --help` prints the usage whole.
- `tools/gate-breakers.sh`: a case stages a broken intra-doc link and
  the doc stage refuses it by name; the case that holds `doc` out of the
  full list is changed, or retired by a `retired X; held by Y` line.
- `AGENTS.md`'s stage list and "Quality gate" name `doc`, and no
  sentence says CI alone runs it (`tools/docs-check.sh` passes). The
  first Find-first command finds no standing instruction to run
  `--only doc` by hand.
- `tools/trade-check.sh` runs the three `--check`s as one stage: `ok`
  here, with the clone at its pin; one line saying they were skipped,
  exit unchanged, with no clone; a failure naming the pin with the clone
  off it. Its header says where they run. `tools/trade-check-breakers.sh`
  refuses a table that has left its generator by name. Both breaker
  suites pass here.
- The Keychain job's steps stand in one file
  (`git grep -n keyring_round_trip -- .github`); `release.yml`'s
  `draft` needs its caller; `weekly.yml` calls it;
  `tools/gate.sh --only features` passes.
- The reusable workflow has run once by its own dispatch on the pushed
  `main` and passed, named by run id (`gh run view <id>`); a refused
  `gh workflow run` is said, and the design session dispatches it.
- #257 and #225 are closed by the close commit; #220 has the comment.
- `tools/gate.sh` exits 0 on the close's tree; its table shows `doc`,
  and the `trade` stage's output shows the three checks run, not
  skipped.

## Not in scope
- Any other stage of the gate; a macOS gate on every push; the
  mutation run; #312.
- The category table's `--tags`, which also reads the clone: not ruled.
- Editing a brief or a record already written.

## Rules
`TEMPLATE.md`, less its line on `--only doc` once this step removes
it. A `gh workflow run` stands alone in its command, after the push
that carries the file. A choice of shape with two defensible answers
goes to the design session; the rest carries on.

## Report
`TEMPLATE.md`.
