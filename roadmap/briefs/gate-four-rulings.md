# gate-four-rulings — What the local gate and the release run

## Goal
A session pushes on a local green (C171), so the local gate holds what
a push can turn red and only a local tree can see. Done, the full gate
runs `cargo doc` as its `doc` stage; its `trade` stage runs the class,
defence and forms tables' `--check` wherever the `poe1` clone sits at
its pin; `artifacts` stands, recorded as part of the gate; and the macOS
Keychain round trip has one definition, called by the release and the
weekly run, and has run once on its own. Closes #257 and #225.

## Inputs
- The owner's rulings of 2026-11-23 and what they answer: comments
  7122394416 (#257) and 7122396497 (#225), read by `gh api repos/smallcatlabs/acquisition/issues/comments/<id>`.
- What they reverse: his ruling of 2026-10-29 on
  `roadmap/briefs/t2-gate-trims.md`, which took `doc` out
  (`git show 4b54e8c7`) and ruled `artifacts` retired, a stage the
  step's close kept (`grep -n '^| t2,' roadmap/LEDGER.md`).
- C170 (the gate's stages are the owner's), C154 (a retirement names
  its holder), C192 (an entry states a property), C84 (the artifact
  mismatch). The entry form is `tools/bothy.py`'s docstring; the next
  id is the highest `python3 -I tools/bothy.py list` prints, plus one.
- The Keychain job's first run, dispatched on `51d99a3d`:
  `gh run view 45329267444`, its `keyring-macos` conclusion reported.
- CI: `gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows; a red run is this step's first concern.

## Decisions
The design session's (C172, who designs), settled with the owner:
1. **`doc` is a stage of the full gate,** after `winclippy`, before
   `fmt`, in the table and the record. `--docs` stays the five stages
   that build no Rust; `--only doc` still names it. `AGENTS.md`'s stage
   list and "Quality gate" gain it; its sentence that CI alone runs
   `cargo doc` goes. Every standing text telling a session to run
   `--only doc` by hand loses that line; a written brief or record is
   history, not edited. The gate's breakers stage a broken intra-doc
   link the stage must refuse.
2. **`artifacts` is unchanged.** Its entry says the stage is part of
   the gate and what it holds that `features` does not; its retirement
   and return are the commit message's and the issues', not the entry's.
3. **The three `--check`s run in `tools/trade-check.sh`, as one stage
   of it.** The clone where `tools/sources.py` places it (`CLONES`,
   which `ACQ_CLONES` moves) at its pin: they run and must pass. No
   clone: skipped, one line saying so, as the drift line is, so CI's run
   is unchanged. A clone off its pin: the stage fails and says so (a
   bump in progress regenerates its tables). Every text saying the gate
   never runs them says where they run. `tools/trade-check-breakers.sh`
   follows the same rule: with the clone at its pin, a case stages a
   table that has left its generator and the stage refuses it; with
   none, that case is skipped with a line and the suite passes; no
   fixture export.
4. **One definition of the Keychain job:** a reusable workflow of its
   own (`on: workflow_call` and `workflow_dispatch`), called by
   `release.yml`, whose `draft` still needs it, and by `weekly.yml`; its
   steps in one file, its `ACQ_KEYRING_ROUNDTRIP=1` and test selector
   as they are. Its doc comment names the three places it runs.
5. **Rulings first:** the build's first commit records the four in
   `decisions/process.md`, each a property in the entry form with its
   why; an entry that already rules the same thing is amended, never
   contradicted by a second. No date or quote in an entry: the owner's
   words, "I agree with each of your recommendations" and "Yes, build
   them", 2026-11-23, are quoted in that commit's message, with the
   one-line recommendation each answers.
6. **Closes #257 and #225;** a comment on #220 says the four are ruled
   and built.

## Find first
- `git grep -n -e 'only doc' -e 'cargo doc' -e 'doc stage' -e 'doc job' -- AGENTS.md .claude roadmap/briefs/TEMPLATE.md roadmap/briefs/REVIEW-PROMPT.md roadmap/briefs/BRIEF-REVIEW-PROMPT.md REVIEW-CHECKLIST.md tools .github`:
  every standing text about the doc stage, CI's doc job among them.
- `git grep -n -e 'build test clippy' -e 'Its stages' -e 'DOCS=(' -e 'ONLY=(' -e 'selected=' -e "sed -n '4,10p'" -- AGENTS.md tools`:
  the stage lists, their selection logic, the check and breaker holding
  `AGENTS.md`'s to `names`, and the usage printed by line number.
- `git grep -n -i -e retire -e artifacts -- decisions AGENTS.md .claude REVIEW-CHECKLIST.md GLOSSARY.md`
  and `python3 -I tools/bothy.py list | grep -i -e gate -e stage`:
  the record of `artifacts`, and the entries ruling the stages.
- `git grep -n -e 'not here' -e 'never runs' -e poe1 -e drift -e 'def check' -e CLONES -e '^POE1' -e '^EXPORT' -e 'require(' -- tools/trade-check.sh tools/trade-check-breakers.sh tools/game-diff.py tools/sources.py tools/class-table.py tools/defence-table.py tools/trade-forms-table.py`:
  the texts to rewrite, the drift line's skip as the model, where the
  clone is found, and what each generator requires.
- `git grep -n -e keyring-macos -e keyring_round_trip -e ACQ_KEYRING_ROUNDTRIP -- .github crates tools roadmap/README.md`:
  every place the job, its opt-in or its test is named.
- `git grep -n -i workflow -- tools`: the checks reading a workflow.

## Done when
- `decisions/process.md` records the four, in the first commit, before
  any code; `python3 -I tools/bothy.py check` passes.
- `tools/gate.sh`: `names` holds `doc` between `winclippy` and `fmt`;
  `--only doc` and `--docs` select as before; the header says the full
  gate runs it; `tools/gate.sh --help` prints the usage whole.
- `tools/gate-breakers.sh`: a case stages a broken intra-doc link and
  the doc stage refuses it by name; the `--only doc` case stays, its
  comment true.
- `AGENTS.md`'s stage list and "Quality gate" name `doc`; no sentence
  says CI alone runs it (`tools/docs-check.sh` passes); the first
  Find-first command finds no standing instruction to run `--only doc`
  by hand; `tools/docs-check-breakers.sh` passes on the new list.
- `tools/trade-check.sh`: the three checks `ok` with the clone at its
  pin, here and with `ACQ_CLONES` naming another folder holding it;
  skipped with a line, exit unchanged, with none; failing, naming the
  pin, with the clone off it. `tools/trade-check-breakers.sh` passes
  here (the staged table refused by name) and with `ACQ_CLONES` at an
  empty folder (the case skipped with a line).
- The Keychain steps stand in one file
  (`git grep -n keyring_round_trip -- .github`); `release.yml`'s
  `draft` needs its caller, `weekly.yml` calls it, and
  `tools/gate.sh --only features` passes.
- The reusable workflow has run once by its own dispatch on the pushed
  `main` and passed; the close names the run id and its commit, and
  `gh run view <id> --log | grep keyring_round_trip` shows the test
  passed, with no "keyring round trip skipped" line. A refused
  `gh workflow run` is said, and the design session dispatches it.
- #257 and #225 are closed by the close commit; #220 has the comment.
- `tools/gate.sh` exits 0 on the close's tree; its table shows `doc`,
  and the `trade` stage's output shows the three checks run.

## Not in scope
- Any other stage; a macOS gate per push; the mutation run; #312.
- The category table's `--tags`, which reads the clone: not ruled.
- A pin bump, a table's regeneration, a generator's rules: a table that
  fails its check when the stage first runs goes to the design session.
- The Keychain test's reach (an unsigned binary, the runner's login
  keychain), signing, item ACLs, the daemon's auth.

## Rules
`TEMPLATE.md`, less its line on `--only doc` once removed. A `gh
workflow run` stands alone in its command, after the push carrying the
file. A choice of shape with two defensible answers goes to the design
session; the rest carries on.

## Report
`TEMPLATE.md`.
