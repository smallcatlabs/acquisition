# governance-tools-2 — the checks say what the process rulings say

## Goal
The remaining checks match the rulings of 2026-11-09: no byte budget
on `AGENTS.md` or on a registry entry, a cap that fails on the road's
state file, a gate stage list that cannot drift from the script, a
commit hook that refuses an agent's commit naming no assistant, and a
gate that keeps a record of its own verdicts, so a stage can later be
retired on evidence.

## Inputs
- `decisions/process.md`: C180 (the state file and its cap), C184 (the
  trailer the hook holds), C182 (a fault becomes structure), and "Not
  doing" (a byte budget is refused).
- `governance-tools-1.md` has landed (`git log --oneline
  --grep='governance-tools-1'` shows its commits): `tools/bothy.py`
  holds the entry's shape, so the entry-length note here has a holder.
- `tools/docs-check.sh` and `tools/docs-check-breakers.sh`;
  `tools/gate.sh` and `tools/gate-breakers.sh`;
  `.githooks/commit-msg`; `.github/workflows/gate-linux.yml`, the
  `breakers` job.
- `AGENTS.md`, "Quality gate": the sentence "Its stages, in order:"
  and the ticked names after it.
- An agent's shell sets the environment variable `CLAUDECODE`; the
  owner's own terminal does not.
- CI: read both gates' last runs on `main` before starting, as
  `TEMPLATE.md` says, and name them in the report.

## Decisions
1. **docs-check's byte budget moves to the state file.** Check 1
   fails when `roadmap/README.md` is over 6,000 bytes and says to
   move what is not "now", "next" or "waiting" out of it. It prints
   nothing about `AGENTS.md`'s bytes.
2. **The entry-length note leaves check 2.** The count of
   cross-cutting rulings in `AGENTS.md` stays as it is.
3. **The uncited scan reads `.github/`** as the commit scan does, so
   a ruling cited only from a workflow is cited.
4. **A new check compares the stage list** in `AGENTS.md` with
   `names=(…)` in `tools/gate.sh`, in order, and fails naming both
   when they differ.
5. **docs-check's header** lists the checks it runs, by number, with
   no count that can go stale.
6. **The hook refuses.** After its rewrite, when `CLAUDECODE` is set
   and the message has no line beginning `Assisted-by:`, it exits 1.
   The refusal says a commit from an agent's shell names its
   assistant, and prints the line to add:
   `Co-Authored-By: Claude <model> <noreply@anthropic.com>`. A merge
   or a revert made in an agent's shell is a commit like any other.
   With `CLAUDECODE` unset the hook behaves as it does today.
7. **The gate records its verdicts.** After its table, every run
   appends one tab-separated line per stage to
   `runs/gate/verdicts.tsv` (under the gitignored `runs/`): the UTC
   time, the short hash of `HEAD`, `clean` or `dirty`, the selection
   as typed (`full`, `--docs`, `--only <stage>`), the stage, its exit
   status and its seconds. A failure to write the record prints one
   line to stderr and never changes the gate's exit status.
8. **Two stale sentences go.** `tools/gate.sh`'s header says once
   which stage it ends with, as its stage list has it.
   `tools/tracer-rung.sh` prints that ground truth is authored here,
   as `AGENTS.md`, "Where things live", says.

## Find first
- The breaker cases that stage what decisions 1 and 2 remove:
  `grep -n -i -E 'budget|entry limit|ENTRY_LIMIT' tools/docs-check-breakers.sh`
- Every reader of the gate's table or its stage names:
  `git grep -n -E 'gate\.sh|names=\(' -- tools .github .claude AGENTS.md`
- Everything that commits, or names the variable, outside a person's
  hands:
  `git grep -n -E 'git (commit|merge|revert)|CLAUDECODE' -- tools .github .claude .githooks`
- The breaker scripts the CI job runs: `.github/workflows/gate-linux.yml`,
  the `breakers` job.
- Whether any test or build reads what this step changes (a comment
  that names a file is not a read):
  `git grep -n -E '(include_str|read_to_string|File::open|Command::new)[^;]*(docs-check|gate\.sh|commit-msg|verdicts)' -- 'crates/*.rs' 'crates/*/build.rs'`
- Every sentence that says what the hook does today, to make true:
  `git grep -n -i -E 'Assisted-by|commit-msg' -- AGENTS.md CONTRIBUTING.md .claude decisions roadmap/briefs/TEMPLATE.md`

## Done when
- `tools/docs-check.sh` exits 0 on the tree, prints no byte line for
  `AGENTS.md` and no entry-length note, and no longer reports as
  uncited a ruling that a workflow cites.
- `tools/docs-check-breakers.sh` exits 0 with a case that fails
  docs-check for each of: the state file over its cap; the stage list
  in `AGENTS.md` differing from the script's. Each retired case has
  its line in the commit message, `retired X; held by Y`: the
  entry-length cases held by the shape cases of
  `tools/bothy-breakers.sh`, the `AGENTS.md` budget cases by the
  check on the count of cross-cutting rulings.
- A breaker suite the CI job runs proves the hook in a scratch
  repository: with `CLAUDECODE` set, a commit with no assistant line
  is refused and the refusal prints the line; with the
  `Co-Authored-By: Claude` line, the commit carries `Assisted-by:`;
  with `CLAUDECODE` unset, a commit with no line succeeds.
- A `--docs` run appends five lines to `runs/gate/verdicts.tsv`, and
  `tools/gate-breakers.sh` has a case showing the exit status is
  unchanged when the record cannot be written.
- C180's and C184's `*Pinned:*` in `decisions/process.md` name the
  check and the hook in place of `Unbuilt, #226`, and `AGENTS.md`
  ("Once per clone") and the session-close skill say that the hook
  refuses. No other word of either entry changes.
- `tools/gate-breakers.sh` exits 0, and the full `tools/gate.sh`
  exits 0 on the step's tree.

## Not in scope
- `tools/bothy.py` and the registry's entries: the sibling step.
- `tools/arch-check.sh`, and any workflow change beyond adding a new
  breaker script to the `breakers` job's list if one is created.
- Reading or pruning the verdict record: #220 reads it.

## Rules
`TEMPLATE.md`. For this step: the full gate runs once, on the last
tree, because `tools/gate.sh` itself changes. Your own commits must
pass the hook you are changing. Order the step's cold review yourself
when it is committed (the cold-review skill). Do not push while #227
is open: the push waits on the owner's read of the process rulings.

## Report
`TEMPLATE.md`.
