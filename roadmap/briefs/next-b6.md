# next-b6 — A renamed account keeps its history

## Goal

When GGG reports an account's uuid under a new username, at a login
or at a token refresh, every door goes on reading and writing the
facts file the account already has: when each item was first seen,
its events and the items since removed stay in reach (C201, the
history is kept state). Today the index entry follows the new name,
every opener computes a file from that name, and the old file, the
only copy of the history, is left behind
(`audits/whole-review-2026-11/NEXT.md`, B6).

## Inputs

- C201 (above), C50 (the account's key is its uuid, known only from
  a profile response), C144 (no keyring entry out of the index's
  reach), C148 (no index I/O under the daemon's shared lock), C193
  (a frontend cannot write the facts), C124 (a read never migrates).
- `crates/acquisition-store/src/index.rs`: `AccountEntry`,
  `account_path`, `record_login`. `crates/acquisition-daemon/src/daemon/`:
  `kinds.rs` (`record`, `with_store`, `admit_listed`: a job's
  username, fixed at submit, selects the file), `login.rs`
  (`login_with_profile`: the profile job lands before the uuid is
  known), `refresh.rs` (`finish_refresh`: the session is renamed
  before the index is written, and that write can fail);
  `daemon.rs`, `Sessions::rename`.
- `audits/whole-review-2026-11/findings/persistence.md`, thesis 9
  under "After the skeptic";
  `audits/whole-review-2026-11/issues/a-rename-may-double-send.md`.
- `roadmap/reviews/next-b6-brief-f62b8db0.md`: the cases this brief
  was rewritten to hold.
- CI on `main`, each gate's last run:
  `gh run list --branch main --workflow "Gate (Linux)" --limit 1`
  and the same for `"Gate (Windows)"`. A red run is your first
  concern.

## Decisions

1. **No file is ever moved or copied.** SQLite's database, `-wal`
   and `-shm` cannot be renamed safely while a process holds them,
   and a frontend reads while the daemon writes.
2. **The index entry names its facts file, and keeps it.** An entry
   carries its file's name, written only where it differs from what
   its username gives, so an index of accounts never renamed is byte
   for byte today's. One pure function of the index, given a uuid
   and a username, says the file: the one the uuid's entry has;
   else what the username gives; else, where another entry's file
   already has that name, a name made from the uuid. `record_login`
   writes what it says. An entry that has a file never changes it.
3. **The daemon binds each username it meets to a file, in memory,
   and a landing asks that binding, never the index.** The binding
   is made in three places and never unmade in a lifetime: at
   restore, each index entry's username to its file; at a profile
   response's landing, which carries the uuid, the job's username to
   the file decision 2's function gives (the index read there, where
   the landing already does I/O, never under the shared lock);
   in `Sessions::rename`, the new name to the old name's file, the
   old name kept. `with_store` and `admit_listed` resolve a job's
   username through it, and a name never bound gets what the
   username gives, as today. So a job in flight across a rename, a
   job submitted before the index is written, and a login under a
   new name all land in the one file, with no provisional identity
   (C50).
4. **A frontend opens the file its entry names**, through one
   method of the entry. No production source and no example outside
   the store's index module names `account_path`; tests keep it.
   `tools/arch-check.sh` holds that, by its own exclusion of tests.
5. **The double send is shown first.** Before any fix, write the
   test of the issue's inference: a plan for the uuid, a job queued
   under the old username, a rename, the plan's spent check. If the
   job is not counted as held, the rule is that what is keyed by
   username follows a rename where the session does. If that would
   change the protocol or the choke point, stop and report.

## Find first

Each finds candidates; class them in your report.

- `git grep -n 'account_path' -- 'crates/*/src/*' 'crates/*/examples/*'`
  : callers that name a facts file, the daemon's tests among them.
- `git grep -n 'record_login(\|index\.upsert(\|hold_keyring_name(' -- crates`
  : writes of an entry.
- `git grep -n -i 'renam' -- crates/acquisition-daemon/src crates/acquisition-store`
  : the tests and helpers that script a rename.
- `git grep -n 'fn unspent' -- crates/acquisition-plan/src`, and
  the job fields it compares.
- `git grep -n 'orphan' -- crates/acquisition-store/src` : the
  comments that say what a rename does today.

## Done when

Each a test, in the daemon's tests unless named; after each the
world holds one facts file for the account.

- The rehearsal, named for C201: a login, a stash fetched, a
  refresh that renames, a restart, the stash fetched again. An item
  from before the rename has the `first_seen` it had, its events are
  read through the frontends' door under the new name, and a removed
  item is still listed as removed.
- A new login under a new name for a known uuid: its profile
  response lands in the file the account had.
- A body that lands after its job's account was renamed.
- A job submitted after the session is renamed, with the index
  write failing.
- Two accounts, the second taking the first's former username,
  never share a file, and the first keeps its own (the store's
  tests, over decision 2's function).
- An index holding no renamed account is written byte for byte as
  before this step.
- `acq store backup` and `restore` round-trip a renamed account's
  history (the verbs' tests).
- Decision 5's test; the commit message says what it showed.
- `tools/arch-check-breakers.sh` shows the new line refuse a
  production caller and pass a test's.
- The comments found by the `orphan` search say what is now true,
  and C201's *Pinned* names the rehearsal.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- A facts file an earlier build left behind at a rename: nothing
  finds or merges it. Say in your report what that user would see.
- A second account that takes a renamed account's old name while
  the first's jobs under that name are still in flight: not held.
- Naming files by uuid beyond decision 2's collision; the
  annotation files, the keyring names, the protocol, the job rows.
- `acq web`'s pages, and the road beyond this step.

## Rules

`TEMPLATE.md`. Every `acq` you run is in a mock world (the
mock-session skill); never a bare `acq`, never the owner's store.
Commit and push are yours on a green gate (C171); the close is the
orchestrator's.

## Report

`TEMPLATE.md`.
