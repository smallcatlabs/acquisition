# h3 fixes — the review's four findings, one rule each

The fix round of step h3 (`h3-build.md`), over the Codex review saved
verbatim at `roadmap/reviews/h3-cc6f8dc5.md`. The owner's word,
2026-10-26: "build the h3 fix round over all four findings, one rule
each, then a Codex pass over the round". "A step" (`roadmap/README.md`)
holds.

## Goal

The look for a spawn race's winner respects its deadline and hands
the winner it finds to the handshake; the printed start command always
names the frontend's world; and the lifecycle pins are deterministic,
with hang protection left to the runner. When this round is done each
of the four findings is pinned by a test named for its fault, and the
spawn-race pin cannot fail on a starved runner.

## Read

- `roadmap/reviews/h3-cc6f8dc5.md`, whole.
- `h3-build.md`; C147 (a door and a ledger read take the world they
  act on) in `decisions/daemon.md`; C86 (the handshake deadline: late
  success rejected); C137 (a deadline's contract is its behaviour at
  expiry; no test asserts a wall-clock bound a slow machine can miss);
  the #70 record in `roadmap/README.md` "Open" (the Windows gate's
  watch) and `roadmap/TIME-SEAM.md` section 3, for what a starved
  runner did to deadline-shaped tests.
- The two commits `37fae0e5^..cc6f8dc5` with `git show`; the sites the
  review names: `acquisition-client/src/client.rs:1633`
  (`winner_listens`), `:1271` (the look on the outer loop's last
  iteration), `acquisition-verbs/src/frontend.rs:223` (the printed
  start command), `acquisition-client/tests/contract.rs:699`, `:719`,
  `:824` (the race and reap pins); the handshake's own deadline code
  (`hello_within`, `timeout_at`) as the model for finding 1.

## Input

- The review's four inputs, each staged as written: a connect that
  succeeds after the look's deadline; a mock frontend over
  `default_root()` with `ACQ_STORE_DIR` naming scratch world A; the
  winner delayed past the lookup deadline after the loser is reaped;
  the loser's exit observed on the polling loop's final iteration.

## Do

1. **The look is bounded whole** (finding 1): `winner_listens` wraps
   its reach in the deadline (`timeout_at`, as the handshake does) and
   a success after expiry is rejected. Pin by forcing expiry through
   the knob (`ACQ_HELLO_WITHIN` at zero, or an injected instant), never
   by timing.
2. **The printed command names the world** (finding 2): the start
   command the frontend prints always carries the captured root as
   `ACQ_STORE_DIR`, the default root included. Pin with the review's
   input in the verbs crate's tests, reading the printed command.
3. **The lifecycle pins are deterministic** (finding 3): the race and
   reap tests synchronise the loser's exit, the reaper's wait and the
   winner's start through explicit seams (a `cfg(test)` hook or a
   channel the harness controls), so no step waits on a wall clock;
   the sixty-second polls go, with hang termination left to nextest's
   profile. Say in the message what each test now waits on and why a
   starved runner cannot fail it.
4. **A found winner reaches the handshake** (finding 4): the look's
   reached connection is carried into the handshake path directly,
   independent of the startup polling budget; the loop's exhaustion
   after a found winner is impossible by construction. Pin by forcing
   the final-iteration schedule.
5. One commit per rule (four), each naming this brief; the last
   deletes `roadmap/reviews/h3-cc6f8dc5.md` and cites it whole at
   `684262bb`.

## Done when

- Four tests named for the four faults, each staging the review's
  input, green on the tree; the message says which ran red on
  `cc6f8dc5` (a scratch worktree for the red run).
- No test in `contract.rs` waits on a wall-clock bound for a
  lifecycle step; the hang bound is the runner's.
- The review file deleted and cited.
- The gate green on the round's own tree, every stage's exit status
  shown; `tools/arch-check.sh` with no new edge.

## Not in scope

- The provider as a per-frontend value (the build's note; C147 rules
  the world alone).
- A shorter wait in place of a synchronisation: that is the fault
  staged nearer, not the named one (checklist shape 2).

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree for the build; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h3-fixes/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`. A red run in a scratch worktree under
your scratch with its own target dir.

## Report

`TEMPLATE.md`, as text.
