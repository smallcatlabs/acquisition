# h1 last — findings 1 and 3 of the tail's review, one rule each

The last round of step h1, over the Codex review of the tail round,
saved verbatim at `roadmap/reviews/h1-tail-064c1412.md`. The owner's
word, 2026-10-26: "one last round over findings 1 and 3 by one rule
each, no further Codex review; file finding 2 as an issue; then close
h1". Finding 2 is issue #74. No review follows this round; the h1
close commit follows it.

## Goal

A keyring name is registered in the index before a credential is
saved under it, so no sequence of renames under a failing index leaves
a saved credential the index does not know. Every load result at
restore reaches the words: a held name whose load fails is reported,
never read as "no entry" with the keyring called healthy.

## Read

- `roadmap/reviews/h1-tail-064c1412.md`, findings 1 and 3, with their
  inputs.
- `h1-tail.md`; C144 (the daemon reports no success it did not earn).
- `crates/acquisition-daemon/src/daemon/refresh.rs` around 150 to 190
  (`persist_credentials_locked`: today the save precedes the index
  write that holds the name), `daemon/start.rs` around 670 to 700
  (`load_reached`, `restore_sessions`), the `c144_` tests in
  `daemon/tests/refresh.rs`, `tests/sessions.rs` and
  `tests/login.rs`, and `acquisition-store/src/index.rs`
  (`settle_keyring_names`, `record_login`).

## Input

- Finding 1: refresh A→B then B→C while the index directory is
  unwritable and keyring saves succeed; restore writability; logout C.
  Today B's credential is left behind and the logout answers
  `cleared`.
- Finding 3: persisted B holds old name A; loading B returns no entry;
  loading A fails with access denied. Today restore logs "no entry" and
  reports keyring health fine.

## Do

1. **One rule: the index knows a name before the keyring does.** The
   persist writes the new name into the entry's held names (one index
   write) before the credential is saved under it; if that write fails,
   the save does not happen and the refresh's warning says so (the
   token is installed in the session as before; persistence is what
   failed). After the save, the clears and the release of cleared
   names proceed as now. Pin finding 1's input: after the two renames
   and the logout, no credential remains under A, B or C, or the logout
   does not answer `cleared`.
2. **One rule: every load result reaches the words.** `load_reached`
   returns the first held-name error when no token is recovered, and
   restore reports it as it reports the username's own load error,
   with keyring health following. Pin finding 3's input.
3. One commit per rule, each naming this brief; the second deletes
   `roadmap/reviews/h1-tail-064c1412.md` and cites it whole at
   `a519fb02`.

## Done when

- Two tests named for the two faults, each staging the review's input,
  green on the tree, and the message says whether each ran red on
  `064c1412` (a scratch worktree under your scratch for the red run).
- The review file deleted and cited.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- Finding 2 (#74). Any surface change. Anything beyond the two rules.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree for the build; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h1-last/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`.

## Report

`TEMPLATE.md`, as text.
