# r1 — The Windows gate decides again

Readiness step 1 (`roadmap/README.md`, row r), after p0. Closes #77.
About a day.

## Goal

Both Windows reds are fixed as the test defects they are, the Windows
and Linux gates run clippy, fmt and the script checks even when a test
step fails, and the gate decides on both platforms again. #70's watch
(a week of pushes without a re-run) resumes from the first green push.

## Read

- #77 and its 2026-10-26 comment (the diagnosis); #70 (what was held:
  nextest groups, the two trade tests) and `roadmap/TIME-SEAM.md`
  section 3.
- C137 (a deadline's contract is its expiry behaviour; no wall-clock
  bound a slow machine can miss), C83 (the world is canonical), C147
  (a door takes the world it acts on).
- `crates/acquisition-client/src/client.rs`, the test
  `a_test_that_moves_the_environment_restores_it_before_its_scratch_goes`
  (around line 2233) and the `Moved` helper and `ENV` lock beside it;
  `crates/acquisition-client/tests/harness/mod.rs`, `isolation()`, the
  one list of the temp variables it moves (`TMPDIR`, `TEMP`, `TMP`,
  `XDG_RUNTIME_DIR`).
- `crates/acquisition-verbs/tests/daemon.rs`, the test
  `c147_a_frontend_over_another_world_acts_in_its_world_not_the_environments`
  (around line 580) and its assertion comparing `client.daemon().world()`
  with `b.canonicalize()` (around line 597);
  `crates/acquisition-store/src/world.rs`, where a canonical root is
  made and the `\\?\` prefix is handled (`REVIEW-CHECKLIST.md`, "A Unix
  mechanism assumed in product code": paths kept `\\?\`).
- `.github/workflows/gate-windows.yml` and `gate-linux.yml`: the step
  order (Build, Test, Clippy, …).
- The last red Windows run's log: `gh run list --workflow
  gate-windows.yml --limit 3`, then `gh run view <id> --log-failed`.

## Input

The two failing assertions as the last red run printed them, quoted in
the commit message. No other input: the fix must be judged from the
assertions, as the diagnosis was.

## Do

1. The environment-moving test: move the same variables the harness's
   isolation list moves, taken from one list (name it once, in the
   harness or beside `Moved`), so a test never again knows less about
   the platform than the harness does.
2. The c147 pin: compare the daemon's reported world with the form the
   product promises (C83's canonical root as `world.rs` makes it, the
   prefix handled there), through the one function that makes it, not
   a bare `canonicalize()` in the test. If the product's promise is
   itself unclear on Windows, that is a finding for the report, not a
   second guess in the test.
3. Both workflows: `if: ${{ !cancelled() }}` on every step after the
   test step (clippy, fmt, the four script checks, docs), so one red
   test step cannot hide the only lint of `cfg(windows)` code; the test
   step's failure still fails the job.
4. Push once; read both gates. A red that is not these two is a
   finding, reported with its log, never re-run.

## Done when

- Both tests pass on Windows and on Unix (the Unix run in the gate;
  the Windows run in CI, its run id in the message).
- The Windows gate is green on the step's push, and the Linux gate too.
- Every step after the test step in both workflows carries the
  condition, visible in the diff.
- The commit message quotes the two failing assertions and names the
  product form the c147 pin now compares.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- The older ten-second waits in `contract.rs` (under "Open").
- nextest test groups and the two trade tests (held per
  `TIME-SEAM.md` section 3 until the week of pushes is read).
- Any change to the C137 knobs or the isolation list's values.
- The rust-cache's `CACHE_ON_FAILURE` setting.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `r1`; commit on `main`; this step pushes once, at
the owner's word, since its done-when is a CI run; the
`Co-Authored-By` line for your model last; long commands in the
background; `tools/gate.sh` if p0 has landed, else the gate bare by
each stage's own exit status.

## Report

`TEMPLATE.md`, as text.
