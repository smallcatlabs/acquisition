# Trial: a second build lane from a copy-on-write clone

The owner's standing word (2026-11-23) is no worktree unless two
writers build at once, because each worktree built a cold `target/` of
8 to 30 GB and fifteen of them reached 154 GB. So the project has had
one build lane. This trial asks what a second lane costs if its build
cache is cloned instead of built. `scripts/lane.sh`, run once on
2026-11-25 on the M4 with nothing else building; the tree was at
`3a47098a` and `target/debug` held 5.5 GB.

| Step | Time | New disk |
| --- | --- | --- |
| `cp -cR` of the tree and `target/debug` into a scratch directory (APFS clone) | 2 s | 0 |
| `cargo build --workspace --offline` there | 26 s | 1.1 GB |
| `cargo nextest run --workspace --all-targets --no-run` there | 35 s | 1.3 GB more |
| `cargo build --workspace` after one edit to a CLI file | 3 s | |
| `rm -rf` of the clone | | all returned |

The clone keeps every third-party crate's artifacts and rebuilds the
ten workspace crates, since a path crate's identity holds its path.

## What it does not show

- The test build ended in an error of the trial's own making: the
  clone left out `search/`, `runs/` and `audits/`, and the search
  crate's tests read files under `search/`. A real lane clones the
  whole tree. The 35 s is therefore a lower bound.
- No test ran. Two lanes running the suite at once on ten cores is
  untried, and C185's reason for one measurement at a time stands.
- The gate's `artifacts` and `tree` stages in a clone, and whether the
  release cache clones as well, are untried.
- How a lane's commits come home. A clone has its own `.git`; its
  commits are fetched into the main tree and rebased, never merged
  (no merge commits on `main`). A `git worktree` with a cloned
  `target/` is the other form and was not tried.

## What it changes

A second lane is a minute and about 2.5 GB, removed in one command.
That is cheap enough that "two writers build at once" can be planned
for instead of avoided, where their crates do not overlap.
