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
  the owner's ruling on #351), C35 (annotations are irreplaceable;
  backup is a store-managed snapshot), C167 (saved searches are the
  world's, in `user.db`), C193 (a frontend's write of facts goes
  through one verbs door, which refuses while a daemon is live), C46
  (semantics in Rust, the CLI an adapter), C53 and C149 (text is a
  function of one JSON object), C146 (a count says what it counts),
  C116 (no id in user-facing text), C124 (a read opens read-only and
  never migrates).
- `Annotations::export` (`crates/acquisition-store/src/annotations.rs`)
  and its doc: a checked snapshot, and where its publication can
  still replace a file. Its module doc: a raw copy of a file under
  WAL is no backup.
- `Seeded::write` (`crates/acquisition-client/tests/harness/seeded.rs`)
  and its variations: a world on disk with no daemon.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Backup and restore are one operation pointed two ways: a checked
   snapshot of each file, written where nothing stands.** The store
   gains for a facts file what `export` is for annotations and saved
   searches. No direction overwrites, and no merge exists.
2. **A backup is whole or it is no backup.** `acq store backup <dir>`
   makes `<dir>` itself, refusing one that exists, so nothing else
   writes inside it. Its manifest is written last, and a directory
   without one is not a backup: restore refuses it, and the refusal
   says it is an interrupted backup to remove. No resume, no repair.
3. **The manifest says what the backup holds, counted.** The
   account's name and uuid, when it was made, the build, and for each
   of the three files what it holds (prices, the policy, saved
   searches, items, events, removed items) or that the store had
   none. A file the store lacks is recorded as absent and nothing is
   made for it, in either direction.
4. **A restore checks everything, then puts back each file the store
   lacks, and names each it left alone.** Before the first file is
   written: the manifest is whole, its uuid is the account's, each
   file it names opens and passes its check. For its whole length no
   daemon runs and no second restore does, by a hold the builder
   finds in what a daemon's start already takes; where that cannot
   be held, it refuses and says to stop the daemon (C193). So no
   publisher races it. A file that stands is no failure: the answer
   says which were restored and which stand, and that a standing
   file is moved aside by the person before it can be restored. Run
   again, it puts back what is still lacking.
5. **The verbs are CLI only**, their semantics in the verbs crate.
   Each answers one document (C149) whose text leads with what was
   kept or put back, counted (C146), and ends with the next act. The
   help says what a backup holds, the whole facts file with its
   history, and what it does not: the login and the job queue.
6. **A person whose whole store is gone**: log in, stop the daemon,
   move aside the facts file the login made, restore. The help says
   so, and restore's answer names the file to move. Restore writes
   no index entry.
7. **Issue 330's two refusals say what is lost, and say move, not
   delete.** `FACTS_BELOW_FLOOR_REMEDY` and the refusal of a reserved
   file holding an account's facts tell a person to move the file
   and its `-wal` and `-shm` aside, that the next refresh fetches
   what the account holds now, and that the history the file held is
   not fetched again. Where a backup can help, the refusal names the
   verb. The store's module docs that call annotations the only
   irreplaceable state say what C35 and C201 say now, and C201's
   `Pinned` names the rehearsal.

## Find first
- `git grep -n -E '\.export\(' -- crates`: the exports' callers.
- `git grep -n 'VACUUM INTO\|quick_check\|hard_link' -- crates/acquisition-store/src`:
  the snapshot's parts, to share and not to copy.
- `git grep -n -i 'refetchable\|only irreplaceable' -- crates decisions AGENTS.md README.md`:
  every sentence decision 7 sorts; the report lists each.
- `git grep -n -E 'FACTS_BELOW_FLOOR_REMEDY|holds the facts of an account|restores the account' -- crates`:
  the tests and goldens that hold the two refusals' words.
- `git grep -n -E 'observe_at|pub async fn observe|Lock::acquire|pub fn acquire|lock_path' -- crates`:
  how a daemon's start is excluded and how a verb learns one is live.
- `git grep -n 'Store::open\|Annotations::open_for\|User::open_for' -- tools/arch-check.sh`:
  the door grep the new verbs must pass.
- `git grep -n 'pub fn' -- crates/acquisition-client/tests/harness/seeded.rs`.

## Done when
- A rehearsal over a seeded world, at a fixed clock, with no daemon:
  back up; remove the three files with their `-wal` and `-shm`;
  restore. Compared as documents before and after: the prices, the
  policy, the saved searches, every item's first-seen time, every
  event with no limit and no window, and the removed items.
- The same over a world with no intent and no saved search: the
  manifest says so, and restore makes neither file.
- A mixed restore, one file standing and two lacking, succeeds and
  says which is which.
- In a mock world: decision 6's sequence end to end; a restore under
  a live daemon refused; and a backup made while the daemon ingests,
  restored, reads whole by the first line's comparison.
- Tests pin each refusal: a directory that exists, one with no
  manifest, another account's backup.
- `CLI-REFERENCE.md` holds both verbs, regenerated, and the README's
  tour one line for them.
- The goldens that hold the two refusals carry `# holds` lines for
  the move, what a refresh fetches and what it does not; neither
  says a refresh restores the file.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- An MCP tool for either verb; a merge into a store that holds
  files; a schedule; resuming an interrupted backup or restore.
- A rename at GGG keeping its history (`NEXT.md`'s B6).
- The rest of C193's door (rebuild and prune under it): issue 212.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world (the mock-session
skill). Before any build or gate, `pgrep -f desktop-rig`: while it
prints a process the owner is sitting at the prototype and nothing
builds.

## Report
`TEMPLATE.md`, and each regenerated golden's diff read line by line.
