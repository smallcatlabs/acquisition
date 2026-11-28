# next-b6 — A renamed account keeps its history

## Goal

When GGG reports an account's uuid under a new username, at a login
or at a token refresh, every door goes on reading and writing the
facts file the account already has. When each item was first seen,
its events and the items since removed stay in reach (C201, the
history is kept state). Today the index entry follows the new name,
every opener computes a file from that name, and the old file, the
only copy of the history, is left behind. It is the one fault of the
whole-project review that cannot be undone by a refresh
(`audits/whole-review-2026-11/NEXT.md`, B6).

## Inputs

- C201 (the history is kept state), C50 (the account's key is its
  uuid), C144 (no keyring entry out of the index's reach: the shape
  to copy, names held in the entry), C148 (no index I/O under the
  daemon's shared lock), C193 (a frontend reads the facts through a
  type that cannot write them), C124 (a read never migrates), C47
  (bad input is an error, never a panic).
- `crates/acquisition-store/src/index.rs`: `AccountEntry`,
  `account_path`, `record_login` (its doc says what a rename does
  today), `hold_keyring_name`.
- `crates/acquisition-daemon/src/daemon/refresh.rs`: a refresh whose
  answer renames; `daemon/kinds.rs`, `with_store` and `admit_listed`,
  which open a facts file by a job's account name; `daemon.rs`,
  `Sessions::rename`.
- `audits/whole-review-2026-11/findings/persistence.md`, "After the
  skeptic", thesis 9: the states a transition must hold.
- `audits/whole-review-2026-11/issues/a-rename-may-double-send.md`:
  an inference never run.
- The last push's CI on `main`: Gate (Linux) 46160188835 and Gate
  (Windows) 46160188728, on `931d931d` (documents only), in progress
  as this is written; the last finished, on `656902d4`, were green
  (46153980080, 46153980070).

## Decisions

1. **No file is ever moved.** SQLite's database, `-wal` and `-shm`
   cannot be renamed safely while any process holds them, and a
   frontend reads while the daemon writes. This rules out moving the
   file at the rename, and a copy.
2. **The index entry names its facts file.** An entry carries the
   name its facts file has, written only where that differs from
   what its username gives, so an index of accounts never renamed is
   byte for byte what it is today (as `keyring_names` is absent when
   empty). When `record_login` finds the uuid under another
   username, the entry keeps the file it had: the name is set from
   the old username unless one is already held. Only a uuid tells a
   rename (C50); an entry with no uuid is linked to nothing, as now.
3. **One maker of an account's facts path, from its entry.** No
   production source and no example outside the store's index
   module computes a facts path from a username. `account_path`
   stays public for the tests that seed and read a world by name,
   which is most of its callers, so a line of `tools/arch-check.sh`
   holds the rule, with its breaker case. A world file's escape
   (`account_path`'s reserved stems) stays inside that maker.
4. **Two entries never name one file.** A new entry, or a rename's
   target, whose username gives a file another entry holds takes a
   name of its own, made from its uuid. Else a second account that
   takes a freed username would ingest into the first one's history.
5. **A job lands in its account's one file whatever name it was
   queued under**, or is refused before any send; it is never sent
   and then found unrecordable. How the daemon resolves a job's
   account name to the entry is yours, under C148: the index is read
   where `index_apply` reads it, never under the shared lock.
6. **The double send is shown first.** Write the test of the issue's
   inference before any fix: a refresh plan for the uuid, a job
   queued under the old username, a rename, then the plan's spent
   check. If the job is not counted as held, the rule is that what
   the daemon or the plan keys by username follows a rename where
   the session does (`Sessions::rename`). If that fix would change
   the protocol or the choke point, stop and report; do not build it.

## Find first

- `git grep -n 'account_path' -- 'crates/*/src/*' 'crates/*/examples/*'`
  : every place production names a facts file from a username; the
  same over `crates` shows the tests, which stay.
- `git grep -n 'record_login(\|\.upsert(\|hold_keyring_name(' -- crates/acquisition-daemon/src crates/acquisition-store/src`
  : every write of an entry's name.
- `git grep -n 'renamed_answer\|a_renamed_username' -- crates` : the
  tests that already script a rename, and their harness.
- `git grep -n 'fn unspent' -- crates/acquisition-plan/src` and the
  job fields it compares.
- `git grep -n 'with_store(' -- crates/acquisition-daemon/src` : the
  landings and the judge that open a store by name.

## Done when

- A rehearsal in the daemon's tests, named for C201: a login, a
  stash fetched, a refresh that renames the account, a restart, the
  same stash fetched again. After it the world holds one facts file;
  an item fetched before the rename has the `first_seen` it had; its
  events from before the rename are read through the frontends' door
  under the new name; a removed item is still listed as removed.
- A test that two accounts, the second holding the first's former
  username, never open one file.
- A test that an index holding no renamed account is written byte
  for byte as the tree before this step writes it.
- `acq store backup` and `acq store restore` round-trip a renamed
  account's history (the verbs' tests).
- Decision 6's test, and what it showed in the commit message.
- `tools/arch-check.sh` refuses `account_path` named under any
  `crates/*/src` or `crates/*/examples` outside the store's index
  module and its `lib.rs` export, and its breaker shows it refuse.
- The doc comments that say a rename orphans the facts file
  (`git grep -n 'orphan' -- crates/acquisition-store/src`) say what
  is now true, and C201's *Pinned* names the rehearsal.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- A facts file an earlier build already left behind at a rename:
  nothing here finds or merges it. Say in your report what a user
  in that state would see.
- Naming new accounts' files by uuid; any change to the annotation
  files, the keyring names or the protocol.
- `acq web`'s pages (parked), and anything of the road beyond this
  step.

## Rules

`TEMPLATE.md`. Every `acq` you run is in a mock world (the
mock-session skill); never a bare `acq`, never the owner's store.
Commit and push are yours on a green gate (C171); the close is the
orchestrator's.

## Report

`TEMPLATE.md`.
