# h3 last — the fix round's three findings, one rule each; #75

The last round of step h3, over the Codex review of the fix round,
saved verbatim at `roadmap/reviews/h3-fixes-6015d64d.md`. The owner's
word, 2026-10-26: "one last round over all three findings, one rule
each, no further Codex review, closing #75; then close h3". No review
follows; the h3 close commit follows.

## Goal

The client's startup budget is a liveness deadline with a knob, like
its siblings, so the isolation list widens it and no lifecycle pin can
fail for want of CPU (#75); a spawn begins with a fresh budget; and
the new unit test that mutates the environment does so under the lock
every sibling holds, restoring what it changed before it deletes its
scratch.

## Read

- `roadmap/reviews/h3-fixes-6015d64d.md`, whole; issue #75.
- C137 (a deadline's contract is its behaviour at expiry; a liveness
  deadline widens through a named knob set by the isolation list; the
  knob at zero forces the expiry for its pin), C86 (`ACQ_HELLO_WITHIN`
  as the model), C122 (every test process takes the harness's one
  isolation list).
- `crates/acquisition-client/src/client.rs`: `STARTUP_POLLS` and the
  `polled` counter in `open_door` (around 1278 to 1319), `hello_within`
  and `parse_hello_within` as the knob pattern; the unit tests around
  1940 (`c147_a_winner_found_on_the_last_poll_is_handshaken`) and the
  sibling `a_stop_the_daemon_did_not_acknowledge_is_not_a_stop`, and
  the `ENV` lock they share.
- `crates/acquisition-client/tests/harness/mod.rs`: the isolation list
  (where `ACQ_HELLO_WITHIN` and `ACQ_ANNOTATION_WAIT` are set), and
  `README.md`'s knob table (docs-check: every `ACQ_*` the crates read
  has a README row).

## Input

- The review's three schedules: the parent exhausting its polls before
  the child gets CPU; ninety-nine polls spent, an incompatible winner
  stopped, the replacement given one poll; the sibling test reading the
  environment while the new test mutates it and deletes its scratch.

## Do

1. **The startup budget is a knob** (finding 1, #75): the budget reads
   `ACQ_STARTUP_WITHIN` (a duration, as `ACQ_HELLO_WITHIN` is), the
   isolation list sets it to its ceiling beside its siblings, the knob
   at zero forces the expiry, and the README's knob table gains its
   row. Pin the expiry by forcing it; re-read the race and reap pins
   and say in the message what, if anything, in them still depends on
   the budget after the list sets it.
2. **A spawn begins fresh** (finding 2): the budget resets whenever a
   new child is spawned; pin the review's replacement schedule.
3. **The test holds the lock and restores its world** (finding 3): the
   sibling stop test takes `ENV` for its whole run; the new test
   restores every variable it changed before removing its scratch
   directory; the doc comment's safety claim is rewritten to what is
   now true.
4. One commit per rule (three), each naming this brief; the first says
   "Closes #75"; the last deletes `roadmap/reviews/h3-fixes-6015d64d.md`
   and cites it whole at `a0bb6ab1`.

## Done when

- Three tests named for the three faults, each staging the review's
  schedule, green on the tree; the message says which ran red on
  `6015d64d`.
- `ACQ_STARTUP_WITHIN` in the isolation list and the README's knob
  table; docs-check's knob line green.
- The review file deleted and cited.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- The older ten-second waits elsewhere in `contract.rs`: they predate
  h3 and the fix brief's done-when line was wider than its rule; the
  close says so.
- Any other knob.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree for the build; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h3-last/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`. A red run in a scratch worktree under
your scratch with its own target dir.

## Report

`TEMPLATE.md`, as text.
