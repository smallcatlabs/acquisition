# store-backup — A backup a person can make, and put back

## Goal
A person can keep what no refresh brings back: an account's prices
and sync policy, the world's saved searches, and the history in the
account's facts (when each item was first seen, its events, the
items since removed). One verb writes a backup to a place they name,
one puts it back, and the two refusals that tell a person to remove a
facts file stop promising that a refresh restores it. Issues 329 and
330.

## Inputs
- C201 (the history in an account's facts is state the tool keeps,
  the owner's ruling of 2026-11-27 on #351), C35 (annotations are
  irreplaceable; backup is a store-managed snapshot), C167 (saved searches are the world's, in `user.db`),
  C193 (a frontend's write of facts goes through one verbs door,
  which refuses while a daemon is live), C46 (semantics in Rust, the
  CLI an adapter), C53 and C149 (text is a function of one JSON
  object), C146 (a count says what it counts), C116 (no id in
  user-facing text), C124 (a read opens read-only and never
  migrates).
- `Annotations::export` (`crates/acquisition-store/src/annotations.rs`)
  and its doc: a checked snapshot published where no file stands. Its
  module doc: a raw copy of a file under WAL is no backup.
- `Seeded::write` (`crates/acquisition-client/tests/harness/seeded.rs`):
  a world on disk with no daemon.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Backup and restore are one operation pointed two ways: a checked
   snapshot of a file, published where no file stands.** `export` is
   that operation for annotations and saved searches already. The
   store gains the same for a facts file, and restore is the export
   of a backup's file into the store. Neither direction overwrites:
   a backup never replaces a backup, and a restore never replaces a
   file the store holds. No merge exists.
2. **The verbs are `acq store backup <dir>` and `acq store restore
   <dir>`**, CLI only, their semantics in the verbs crate. `<dir>`
   is made by the backup and must not exist. It holds the account's
   annotations, the world's saved searches, the account's facts, and
   one manifest: the account's name and uuid, when it was made, the
   build, and for each file what it holds, counted (prices, the
   policy, saved searches, items, events, removed items).
3. **Restore puts back each file the store lacks and names each it
   left alone.** It refuses a backup made for another account, by the
   uuid. It refuses while a daemon is live and says to stop it
   (C193). A file that stands is not an error of the whole: the
   answer says which were restored, which stand, and that a standing
   file is moved aside by the person, with no process running,
   before it can be restored.
4. **The answers.** Each verb answers one document (C149) whose text
   leads with what was kept or put back, counted in a reader's words
   (C146), and ends with the next act. The help says what a backup
   holds and what it does not: the login, the job queue, and what a
   refresh fetches again.
5. **Issue 330's two refusals say what is lost, and say move, not
   delete.** `FACTS_BELOW_FLOOR_REMEDY` and the refusal of a reserved
   file that holds an account's facts tell a person to move the file
   and its `-wal` and `-shm` aside, say that the next refresh fetches
   what the account holds now, and say that the history the file
   held is not fetched again. Where a backup can help, the refusal
   names the verb. The store's module docs that still call
   annotations the only irreplaceable state say what C35 and C201
   say now, and C201's `Pinned` names the rehearsal.
6. **A person whose whole store is gone logs in first, then
   restores, before the first refresh.** The help says so. Restore
   writes no index entry.

## Find first
- `git grep -n -E '\.export\(' -- 'crates/*/src'`: the exports'
  callers, none outside tests today.
- `git grep -n 'VACUUM INTO\|quick_check' -- crates/acquisition-store/src`:
  the snapshot's parts, to share and not to copy.
- `git grep -n -i 'refetchable\|only irreplaceable' -- crates decisions AGENTS.md README.md`:
  every sentence decision 5 sorts; the report lists each with its
  verdict.
- `git grep -n 'facts are refetchable\|holds the facts of an account' -- 'crates/*/tests'`:
  the tests and goldens that hold the two refusals' words.
- `git grep -n -i 'daemon is live\|fn live\|daemon.lock' -- crates/acquisition-verbs/src crates/acquisition-client/src crates/acquisition-store/src/world.rs`:
  how a verb learns a daemon is live, to reuse.
- `sed -n 540,560p tools/arch-check.sh`: the door grep the new verbs
  must pass.
- `python3 -I tools/bothy.py pack --path crates/acquisition-store/src/annotations.rs crates/acquisition-store/src/lib.rs crates/acquisition-verbs/src/frontend.rs crates/acquisition-cli/src/store_cmd.rs`.

## Done when
- A rehearsal over `Seeded::write`'s world, with no daemon: back up;
  remove the three files with their `-wal` and `-shm`; restore; and
  the prices, the policy, the saved searches, `store events` and the
  removed items read as before, compared as documents.
- Tests pin each refusal of decisions 1 and 3: a backup directory
  that exists, a file that stands, another account's backup, a live
  daemon (in a mock world).
- A backup made while a mock daemon ingests opens whole.
- `CLI-REFERENCE.md` holds both verbs, regenerated, and the README's
  tour one line for them.
- Neither refusal of decision 5 says a refresh restores what the file
  held; the goldens that hold their words are regenerated and their
  diffs read line by line in the report.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- An MCP tool for either verb; a merge of a backup into a store that
  holds files; a schedule; a backup of the job queue or the login.
- A rename at GGG keeping its history (`NEXT.md`'s B6).
- The rest of C193's door (rebuild and prune under it): issue 212.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world (the mock-session
skill).

## Report
`TEMPLATE.md`.
