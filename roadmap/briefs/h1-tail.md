# h1 tail — two rules: the uuid-keyed names, and the logout answer

The tail round of step h1, over the Codex review of the fix round,
saved verbatim at `roadmap/reviews/h1-fixes-afeab845.md`. The owner's
rulings, 2026-10-26: "the logout answer carries the keyring's outcome
(cleared, off, failed) as an additive field; exit 0 when the session
dropped and nothing failed, 1 otherwise"; and "build the tail round as
two commits, the uuid-keyed entry carrying every name in one index
write, and the logout answer's keyring field with exit 0 when nothing
failed". "A step" (`roadmap/README.md`) holds.

## Goal

Findings 1 to 4 of the review are one fault in four shapes: the index
entry is addressed by username where the stable key is the uuid, and
the names a keyring entry may live under are a single slot written
apart from the username. Finding 5 is the CLI reading the daemon's
sentence for a JSON field, because the logout answer carries no data.
When this round is done, an index entry is found by the session's uuid
and carries every name whose keyring entry may exist, written with the
username in one index operation and cleared whole under the one lock;
and the logout answer says what the keyring did as data, with the exit
status following it.

## Read

- `roadmap/reviews/h1-fixes-afeab845.md`, whole.
- `h1-fixes.md` and `h1-build.md`; C144 (the daemon reports no success
  it did not earn) in `decisions/daemon.md`; C50 (the uuid is the
  stable key; `accounts.json` maps username to uuid); C35 and C47 in
  `AGENTS.md` (structured errors, no `unwrap` in the protocol and
  store crates); C84 (the contract revision moves on a protocol
  change: expected here); C11 (`--json` on every command, the error
  path included).
- The fix round's commits, `bd768467^..afeab845`, with `git show`: the
  persist (`daemon/refresh.rs` around 150 to 180,
  `persist_credentials_locked`), logout and forget (`daemon/login.rs`
  around 400 to 530, `clear_account_entries`, `settle`), the index
  (`acquisition-store/src/index.rs`: `AccountEntry`, `keyring_name`,
  `record_login`, `hold_keyring_name`, `set_persisted`), restore
  (`daemon/start.rs`), the CLI's logout (`acquisition-cli/src/main.rs`
  around 1083), the protocol's logout answer
  (`acquisition-protocol/src/protocol.rs`: the `Ack` logout answers,
  the `Response` enum), the wire test
  (`acquisition-protocol/tests/wire.rs`), the `c144_` tests in
  `daemon/tests/login.rs`, `refresh.rs`, and the CLI's `error_kind.rs`.
- Every reader of `keyring_name` and of `persisted`, found by grep
  before the edit (restore, forget, logout, the index's tests, any
  renderer).

## Input

- The review's five inputs, each staged as written: a refresh A→B
  whose index write fails, then logout B; a crash between the rename
  and the held name; forget racing a login under the lock; two renames
  whose clears both fail, then logout; the mock username
  `keyring entry cleared` under a keyring-off logout.
- The owner's two rulings above.

## Do

1. **One rule, the uuid-keyed names.** `AccountEntry` carries
   `keyring_names: Vec<String>` (serde default, skipped when empty, so
   no existing file changes; `keyring_name` is read once for migration
   if any file carries it, then dropped). `record_login` takes the
   username and the names to hold and writes both in one save. A name
   enters the list when a credential is saved under it and leaves it
   only when its clear succeeds; a name never leaves on a failed clear.
   Logout and forget find the entry by the session's uuid (the
   session holds it; forget resolves the selector through the index to
   a uuid first), clear every name the entry carries plus the current
   username, mark the entry not persisted, and write the index, all
   under one hold of `shared`, with the live/non-live decision made
   inside the hold. Restore reads the current username's entry and, if
   its load fails, tries the held names in order, so a rotated token
   under an old name is still reached. Pin the four inputs by forcing
   them (C137, no wall-clock bound); say which ran red on `afeab845`.
2. **One rule, the logout answer.** The protocol's logout answer gains
   `keyring: "cleared" | "off" | "failed"` as an additive field
   (`serde(default)` for an old daemon's bare ack, read as unknown and
   rendered as such), and the index write's outcome beside it if the
   words need it. A logout whose session dropped and whose clears and
   index write succeeded, or whose keyring is off, is a success: exit
   0, the words saying what the keyring did ("keyring off, nothing
   cleared"). A failed clear or a failed index write stays the error
   answer: exit 1. The CLI reads the field, never the sentence; `--json`
   carries it. Forget takes the same shape. Pin in
   `acquisition-protocol/tests/wire.rs` (the new field round-trips; a
   bare ack reads), in the daemon's tests and in the CLI's
   `error_kind.rs` (the mock username `keyring entry cleared` under a
   keyring-off logout answers `keyring: "off"`). The `web.rs` logout
   assertion returns to success. The contract revision moves: C84
   working, not a finding.
3. Two commits, one per rule, each naming this brief; the second
   deletes `roadmap/reviews/h1-fixes-afeab845.md` and cites it whole at
   `65b18c90`. Regenerate `CLI-REFERENCE.md` if the logout help
   changes, and read the diff.

## Done when

- Five tests named for the five faults, each staging the review's
  input, green on the tree; the message says which ran red on
  `afeab845` and how.
- A keyring-off logout exits 0 with words saying nothing was cleared;
  a failed clear exits 1; `--json` carries `keyring` from the field.
- No reader of the logout sentence remains in the CLI.
- The review file deleted and cited.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- A repair verb for a stuck keyring entry; a second installation's
  keyring; #55.
- Any other protocol field.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h1-tail/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`. A red run against `afeab845` uses a
scratch worktree under your scratch (`git worktree add`), never a
stash in the main tree; remove it when done.

## Report

`TEMPLATE.md`, as text.
