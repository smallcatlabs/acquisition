# h1 fixes — the review's seven findings, each by one rule

The fix round of step h1 (`h1-build.md`), over the Codex review saved
verbatim at `roadmap/reviews/h1-ab67bc54.md`. The owner read the
findings on 2026-10-25 and ruled: "Yes to all with the smaller shape
for 3". The rhythm of "A step" (`roadmap/README.md`) holds.

## Goal

Each finding is fixed by one rule, never per case, and pinned by a
test named for the fault. When this round is done, the credential
store, the index and the session move together under the one lock; a
disabled keyring says so instead of claiming a clear; no keyring entry
is ever out of the index's reach; every index result reaches the
words; an absent character item array reconciles as an empty one; and
a printed retry command runs as printed.

## Read

- `roadmap/reviews/h1-ab67bc54.md`, whole: the seven findings, their
  concrete inputs and the reviewer's smallest fixes.
- `h1-build.md` for the step's inputs; C144 (the daemon reports no
  success it did not earn) and C145 (a present key of the wrong type
  is malformed; an absent key is the documented empty) in
  `decisions/daemon.md` and `decisions/store.md`.
- The four commits of the build, `fdae88e1^..ab67bc54`, read with
  `git show`; their tests under
  `crates/acquisition-daemon/src/daemon/tests/` and
  `crates/acquisition-store/src/lib.rs` carry the `c144_` and `c145_`
  prefixes.
- The sites the review names: `daemon/refresh.rs` (the index update
  after the lock, around line 380; the rename clear, around 103),
  `daemon/login.rs` (`complete_login` around 246, forget's words around
  493, the retry command around 520), `auth.rs` (`keyring_clear` under
  `ACQ_NO_KEYRING`, around 376), `acquisition-store/src/lib.rs` (the
  character arrays, around 1460), `acquisition-store/src/index.rs`.
- Issue #16 (printed commands are POSIX-quoted): finding 7 is its
  first instance; the issue stays open for the rest.

## Input

- The review's seven failing inputs, each to be staged as written
  there; the schedule of finding 1 (refresh saves and releases `shared`;
  logout drops the session, fails its clear, records not persisted;
  the refresh's index write lands after) forced in a test, never timed.
- The owner's ruling on finding 3: the smaller shape, below.

## Do

1. **Findings 1 and 4, one critical section.** A refresh's token
   install, its index write and its session rename happen under one
   hold of `shared`, as login's do; `complete_login` applies the same
   save-failure and rename cleanup the refresh applies, found once and
   called from both. Pin finding 1's schedule and finding 4's two
   inputs (a re-login whose save fails; a re-login under a renamed
   username).
2. **Finding 3, the smaller shape.** The account index records the
   name the keyring entry is held under (a field beside `persisted`);
   logout and forget clear that name; a rename moves the field only
   after the old entry's clear succeeds, and a failed clear leaves the
   field on the old name with the account still persisted there, so
   nothing the index knows is out of reach. No retry handle, no new
   verb. Pin: a rename whose old-name clear fails, then a logout that
   clears it.
3. **Findings 2 and 5, the words say what happened.** Under
   `ACQ_NO_KEYRING` a clear is not a success: the result says the
   keyring is off and nothing was cleared, and the CLI's logout and
   forget words carry that, text and `--json`. Every index write's
   result reaches the caller: forget says "not restored at start" only
   when its index write succeeded, and logout's absorbed index error
   reaches its words the same way. Pin each with the review's inputs.
4. **Finding 6, absent is empty.** A character fetch whose item arrays
   are all absent reconciles its location as a fetch with empty arrays
   does: previously live items at that character are retired. Pin
   against previously populated facts (record `c` with `i`, then record
   `{"character":{"id":"c"}}`, and `i` is retired).
5. **Finding 7, the printed command.** The retry command quotes its
   account argument for a POSIX shell (`Alice Smith`), and the test
   parses the printed command back through clap. Say "part of #16" in
   the message.
6. One commit per rule above (five), each naming this brief; the last
   deletes `roadmap/reviews/h1-ab67bc54.md` and cites it whole at
   `0e7ed91d`. Update C144's *Pinned:* only if a new file carries a pin;
   the entry is at 795 of 800 bytes, so route any new words to a doc
   comment, not the entry.

## Done when

- Seven tests named for the seven faults, each staging the review's
  input, each green on the tree and red on `ab67bc54` (say in the
  message which you ran red; a test that cannot be run red says why).
- The review file deleted and cited at its commit.
- No golden changed except where the logout or forget words changed,
  each read and named in the commit.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- #16 beyond finding 7's one command.
- The remaining hygiene rows of #63; #55 (the access token in the
  keyring).
- A retry handle or a repair verb for a stuck keyring entry (the
  owner declined the larger shape).

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h1-fixes/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). The
subagent has no task list: do not call `update_plan`.

## Report

`TEMPLATE.md`, as text; item 1 names which tests you ran red on
`ab67bc54`.
