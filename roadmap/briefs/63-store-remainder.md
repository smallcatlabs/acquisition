# 63 — the store's remainder: the two-writer pin, the index's lock as a type, five hygiene items

The remainder of issue #63 after the honesty slice (h1, `h1-build.md`
to `h1-last.md`) landed C144. First of the league group
(`roadmap/README.md`, "Pull-forward"). The rhythm is "A step".

## Goal

Issue #63 named four faults: the daemon absorbing a store failure as
success; `record` and `rebuild` opening deferred transactions that read
before they write, so a CLI `store rebuild` or `store import` beside an
ingesting daemon could fail the ingest's upgrade to the write lock; the
account index written load-modify-save to a fixed temporary name with
no sync, four of five writers outside the lock; and five low hygiene
items. C144 (the daemon reports no success it did not earn) closed the
first whole and the second and third in code: `record` and `rebuild`
take `BEGIN IMMEDIATE`, the index is written to a unique temporary,
synced, renamed and its directory synced, and every writer goes
through one function under the credentials lock. What is left is the
proof and the structure: no test forces the two-writer interleaving
the comment on `record` claims to survive; the index's "every writer
under the one lock" is a doc comment, held by discipline, and a test's
comment still names the wrong lock; and the five hygiene items stand.
When this step is done, the SQLite behaviour behind `BEGIN IMMEDIATE`
is pinned both ways (the deferred fault SQLite has, and the wait the
immediate transaction buys), the index writer takes the lock's guard
as a parameter so a writer outside it does not compile, each hygiene
item is fixed by one rule or declined with why, the two recovery
scenarios the issue's second comment asks for are tests, and the
401-restart case the first comment carries has a home. Nothing sends
to GGG; no daemon HTTP path changes. One hygiene item is a ruling,
brought to the owner before any code depends on it (Do 1). The
commits say "closes #63" only on the last, and only if every "closes
when" line of the issue is met or declined by a "Not doing" line.

## Read

- Issue #63 with its two comments (`gh issue view 63 --json
  body,comments`): the four faults, the hygiene list, the "closes when"
  line; the first comment's 401-restart case (from
  `roadmap/reviews/c135-fixes-380fccb1.md`, finding 1: a 401 clears the
  session's access token and the rejection landing rewrites the
  keyring entry through the one persist, which reads the index first;
  with the index unreadable at that moment and a restart before the
  next refresh, the rejected bearer is restored once more and GGG
  answers one more 401); the second comment's two recovery scenarios
  (a restart after a failed write; a login's finalisation).
- `AGENTS.md`: C35 (annotations are the only irreplaceable local state,
  written only through the store crate under revision compare-and-swap;
  no fact-side event deletes intent), C47 (malformed input is a
  structured error with a stable kind; the store crate denies `unwrap`),
  C48 (the SQLite schema is internal), C116 (no id in user-facing
  text), C122 (one pin at the lowest crate), "Structure over
  discipline" (a rule that can be broken silently becomes a dependency
  edge, a lint or a test).
- `decisions/store.md`: C35; C124 (a tool that presents itself as a
  read opens the store read-only and never migrates it; pinned by the
  `c124_` tests in `lib.rs` lines 4984 and 5068 and in
  `annotations.rs`); C30 (a refused body is evidence).
- `decisions/daemon.md`: C144 (its last clause: "the account index is
  written atomically, synced, under the one lock"); C148 (no
  credential-store or account-index I/O under `shared`; the keyring
  and index I/O under one credentials lock on a blocking thread); C6
  (the persisted queue, `daemon.db`, in `crates/acquisition-daemon/
  src/daemon/queue.rs`'s module doc: a daemon that cannot open or read
  it refuses to start).
- The code, line numbers from this tree (`35de06b8`):
  - `crates/acquisition-daemon/src/daemon/kinds.rs`: `index_apply`
    (line 136, "Called under `credentials`, never under `shared`"; the
    one function every index write goes through), `with_index` (159,
    takes the lock itself on a blocking thread), `record` (198, C144's
    doc and the failure that keeps the body).
  - The five `index_apply` call sites (`grep -rn '\.index_apply('
    crates/acquisition-daemon/src --include='*.rs' | grep -v /tests/`
    prints 5): `kinds.rs` 166 (inside `with_index`, lock taken at 165);
    `login.rs` 571 (inside `logout`, the guard `credentials` taken at
    508, dropped at 574); `refresh.rs` 160, 202, 217 (inside
    `persist_credentials`, line 129, whose doc says "Called under the
    caller's hold of `credentials`"; its callers `login.rs` 317
    (`complete_login`, guard at 286), `refresh.rs` 299
    (`persist_rejection`, guard at 281) and `refresh.rs` 592
    (`finish_refresh`, guard at 523 as an `Option<MutexGuard>`, taken
    only when the refresh succeeded)).
  - `crates/acquisition-store/src/index.rs`: `save` (442, the C144
    doc), `write_temp` (460: `.accounts.json.<pid>.<seq>.tmp`,
    `sync_all` at 476), `sync_dir` (491, unix only; "there the rename
    goes unsynced" on Windows, by design); the tests
    `c144_a_crash_between_write_and_rename_leaves_the_index_whole`
    (669) and `c144_two_accounts_saving_at_once_never_tear_the_index`
    (700), whose comment says "its every index write under `shared`":
    the lock has been `credentials` since C148.
  - `crates/acquisition-store/src/lib.rs`: `ensure_wal` (1031),
    `Store::init` (1118: `ensure_wal` at 1120, then the deferred
    version read at 1128, then the immediate transaction at 1143 that
    refuses `SchemaTooNew` or migrates), `record` (1324: the immediate
    transaction and its comment citing issue #63, landed at `eb1241f6`;
    `git log -S'taken before the ingest' -- crates/acquisition-store/src/lib.rs`
    prints that one commit), `rebuild` (2330, the same), `SchemaTooNew`
    and its `Display` (359, 364: "facts file uses schema v{}, newer than
    this build's v{}"); the tests `rebuild_reextracts_columns` (3682,
    the only rebuild test; nothing pins two writers) and
    `a_current_file_opens_while_a_writer_holds_the_lock` (4014: a raw
    connection holding `BEGIN IMMEDIATE` beside a `Store::open`, and a
    migrating opener made to wait by a scoped thread with a 300 ms
    sleep, the shape this step's pin must not copy).
  - `crates/acquisition-store/src/annotations.rs`: `prepare` (the
    version gate before any pragma that writes, the order `Store::init`
    lacks), `delete` (1031) and `delete_unreported`, `list` (1151),
    `export` (1211: the partial synced in `export_to`, published by
    `hard_link`, no directory sync after), `row_from_raw` (1333) and
    `row_of` (1349): both `serde_json::from_str(&value)?`, so one row
    whose value is not JSON fails `list` whole and fails `get`, `put`
    and `delete` for that row; `AnnotationError` (460: `Conflict`
    carries `current: Option<Box<AnnotationRow>>`, a row with a parsed
    value; `Json(serde_json::Error)` is the variant such a row raises
    today). The annotations' `BEGIN IMMEDIATE` pins
    (`c35_` tests around 1720, the shape of a two-writer test in this
    crate).
  - `crates/acquisition-store/src/jobs.rs`: `SCHEMA` (55), `JobDb::open`
    (139), `init` (151: `ensure_wal`, pragmas, `SCHEMA`; no
    `user_version` read or stamp); its tests from 640.
  - `crates/acquisition-store/examples/read-corpus.rs` (line 41,
    `Store::open`; its doc at 16 says so) and
    `crates/acquisition-search/examples/copy-as-store.rs` (line 18,
    `Store::open`; a tool that makes a store directory, not a read).
  - `crates/acquisition-cli/src/store_cmd.rs`: `rebuild` (841) and
    `import` (857, `store.record` per tab) through
    `crates/acquisition-verbs/src/frontend.rs` 334 (`Store::open`):
    the second writer beside the daemon.
- The daemon test harness this step's tests reuse:
  `crates/acquisition-daemon/src/daemon/tests/harness.rs` (`test_daemon`
  398, `test_db_path` 467, `logged_in` 943, `freeze` 980: the directory
  made unwritable, unix only, restored on drop); `tests/dispatch.rs`
  (`schema_too_new` 1826; the three `c144_` store tests from 1850, the
  staging to copy); `tests/queue.rs` (`persisting_daemon` 1168, a
  daemon on a file-backed `daemon.db` restored like `run` does;
  `the_queue_and_results_survive_a_daemon_restart` 1190, the two
  lifetimes pattern); `tests/login.rs` (`keyring_daemon` 953,
  `staged_login` 1162 and the `complete_login` calls at 1190 and 1234;
  `c144_forget_and_logout_say_what_the_index_write_did` 1302, the
  frozen-index pattern); `tests/refresh.rs` (`restored_daemon` 1034,
  `start::restore_sessions` driven directly).
- `REVIEW-CHECKLIST.md`, "Pricing and intent": "A first-open race on
  the one irreplaceable file" (WAL on a fresh file taking the write
  lock inside its own read; the gate's two reads); "Store and plans":
  "A version where an identity belongs"; "Protocol, client, daemon
  lifecycle, the world": "A test that owns what it does not clean";
  "Frontends and the surface an agent meets": "A claim the code did
  not make", "A fix staging the nearest failure, not the named one".
- `roadmap/briefs/h1-build.md`, `h1-fixes.md`, `h1-tail.md`,
  `h1-last.md`: what C144 built of #63 and why the index is keyed by
  uuid and holds a name before the keyring does.
- `roadmap/briefs/103-observe-bound.md` and `roadmap/LEDGER.md`'s last
  row: the form, and the pattern its three fix rounds named (a scripted
  peer asserting its own completeness).

## Input

What C144 already closed of #63, each with the command whose output
says so on this tree:

- **The absorb policy.** `grep -n 'C144' crates/acquisition-daemon/src/daemon/kinds.rs`
  prints lines 154 and 188: `record` fails the job in a sentence naming
  the store's fault and keeps the params and body as the failure's
  payload. Pinned: `c144_a_store_that_cannot_record_fails_the_job_and_keeps_the_body`
  (`tests/dispatch.rs` 1850: a schema-too-new file, a read-only store
  after the open, a malformed body) and
  `c144_a_job_whose_store_cannot_open_ends_failed_with_its_body` (1932,
  end to end). The owner's ruling the issue asked for is C144's
  sentence; nothing of it remains.
- **`BEGIN IMMEDIATE`.** `grep -n 'TransactionBehavior::Immediate' crates/acquisition-store/src/lib.rs`
  prints lines 1143 (the migration), 1339 (`record`) and 2334
  (`rebuild`); `TransactionBehavior::Deferred` appears once, at 1128
  (the version gate's read). Landed in `eb1241f6`. Not pinned:
  `grep -n 'fn .*rebuild\|fn .*beside\|fn .*two_writers\|fn .*second_writer' crates/acquisition-store/src/lib.rs`
  prints `rebuild_reextracts_columns` alone. The remainder is the pin.
- **The index write.** `sed -n 442,500p crates/acquisition-store/src/index.rs`:
  a per-process, per-save temporary name, `sync_all` on the file, a
  rename, `sync_dir` on unix. Pinned for the crash and the tear (lines
  669 and 700). The lock: `grep -rn 'Index::load(' crates/acquisition-daemon/src --include='*.rs' | grep -v /tests/`
  prints 5 sites, each under `credentials` by the doc comment on
  `index_apply` and on `persist_credentials`; nothing but a reader
  holds them to it. The remainder is structure, and the test comment
  at `index.rs` 700 naming `shared`.
- **The five hygiene items**, each at its site above, none fixed.
- **The 401-restart case**: `persist_rejection` (`refresh.rs` 281)
  calls `persist_credentials`, which reads the index first (141) and
  holds the name before the save (160); this step touches neither
  order.

The failing inputs the step's tests stage:

- Two connections on one facts file, in WAL: a deferred transaction
  that has read `items`, a second connection that commits a write to
  the same table, then the first's write: SQLite answers `database is
  locked` at once (`SQLITE_BUSY_SNAPSHOT`, which no busy handler
  retries). Then the same second connection holding `BEGIN IMMEDIATE`
  while `Store::record` and then `Store::rebuild` begin: each waits
  and then lands once the holder commits, the holder's commit released
  by the waiter's own busy handler being invoked (the signal that the
  wait is real), never by a sleep.
- A store whose facts file is stamped newer than this build, in
  rollback-journal mode: `Store::open` refuses it and `PRAGMA
  journal_mode` still answers `delete` after.
- A `daemon.db` stamped newer than this build: `JobDb::open` refuses
  it in words naming the file; a fresh one is stamped; one a build
  before the stamp wrote (tables, `user_version` 0, rows) opens and is
  stamped.
- A daemon on a file-backed `daemon.db` whose account's facts file is
  schema-too-new: its profile job ends failed with the body; a second
  lifetime on the same `daemon.db` lists no open job, answers the id
  from the table as that failure with its body, and the account server
  saw one POST and one GET in all.
- A login finalised with the index directory unwritable (unix): the
  flow fails in the words `complete_login` already has, no keyring
  entry exists under the name (*settled in the round:* the brief first
  said "the entry it saved is cleared back"; the index holds a name
  before the keyring does, so nothing is saved and nothing is cleared,
  which the test asserts), and `restore_sessions` over the directory
  afterwards restores no session and reports the keyring healthy.

## Do

1. **Ruling (the owner's, C35): the repair shape for an unparsable
   annotation row.** Today one row whose stored value is not JSON fails
   `list` whole (`row_from_raw`) and cannot be read, replaced or
   deleted (`row_of`), so one damaged row hides every price and the
   only API that may write the file cannot clear it. Recommended, the
   smallest honest shape: `list` returns every readable row and names
   each unreadable one beside them in the same answer (its scope, key,
   kind, revision and the parse fault), never dropping it and never
   failing the whole; `get` of such a row answers a structured
   `Unreadable` carrying the same; `delete` and `put` naming the row's
   revision do not parse its value, so the row can be cleared or
   replaced by the one compare-and-swap, and at a wrong revision answer
   `Unreadable` with the revision it stands at (a conflict cannot carry
   a row it cannot read). Nothing rewrites a value in place and no read
   repairs. The frontends' words for an unreadable row (the price list,
   the MCP's) follow as agent-facing design, the session's. The
   alternatives: keep `list` failing whole but name the row (the status
   quo with words; the row still cannot be deleted); or skip the row
   silently (refused: C35). **Commit this brief and stop here**: report
   the question to the orchestrating session; build nothing that
   depends on the answer until it comes. The rest of this list waits
   for its word, which may be to build items 2 to 8 first.
2. **The two-writer pin, in the store crate** (the SQLite behaviour the
   comment on `record` claims). One test in `lib.rs`'s tests, named for
   the fault, in two halves. The first stages the fault without
   `Store`: two raw connections on a WAL file created by `Store::open`,
   a deferred transaction that reads `items`, a committed write from
   the other, then the first's write, asserted to fail with
   `DatabaseBusy` at once. The second drives `Store::record` and
   `Store::rebuild` while a raw connection holds `BEGIN IMMEDIATE`: the
   store connection's busy handler (`Connection::busy_handler`, a plain
   `fn(i32) -> bool`, so its signal is a static atomic) is the proof
   the wait happened, and the holder's commit follows that signal;
   both calls return `Ok`. No sleep, no asserted duration. The pattern
   at `lib.rs` 4014 (a scoped thread and a 300 ms sleep) is not
   copied; the report says whether that older test should follow this
   one's shape (an issue, not this step).
3. **The index's lock becomes a parameter.** `index_apply` takes the
   `credentials` guard as its first argument (*settled in the round:*
   not the raw `&MutexGuard<'_, ()>` this brief first named but an
   opaque `held::CredentialsHeld`, which only `Daemon::hold_credentials`
   builds, so no sibling module can forge one), so a writer outside the
   lock does not compile; `persist_credentials`
   takes it the same way and its three callers pass theirs
   (`finish_refresh` holds an `Option`, taken only on a successful
   refresh: the persist happens inside that branch, so the guard is
   in hand, not unwrapped); `with_index` passes the guard it takes.
   The doc comments on both say the rule is now the type's. The test
   comment at `index.rs` 700 says `credentials`. No behaviour changes;
   the pin is the compiler, and the commit message says so.
4. **The version is read before anything is written, in every SQLite
   file the crate opens.** `Store::init` reads `user_version` in its
   deferred gate before `ensure_wal`, and refuses a newer stamp before
   any pragma that writes, as `Annotations::prepare` already does; the
   pin is a facts file in rollback-journal mode stamped newer, refused
   with its journal mode unchanged. `JobDb::init` gains the same gate
   and a stamp: `user_version` read first, a newer stamp refused in
   words naming `daemon.db` (the error's shape as `SchemaTooNew`'s,
   its words its own; C116, no id), then WAL, the schema, and the stamp
   written where it is 0 (a fresh file, or one a build before this
   wrote, whose table the schema's `IF NOT EXISTS` keeps). The daemon
   already refuses to start on an open that fails (C6, `start.rs` 488);
   the pin is the three inputs above in `jobs.rs`'s tests. The stamp
   starts at 1; C48 keeps the schema internal.
5. **The example that reads opens read-only** (C124). `read-corpus.rs`
   opens through `Store::open_read_only`; its doc says a copy at
   another schema version is refused with the remedy C124's door
   prints, and `search/MEASUREMENTS.md` line 59's command stands.
   `copy-as-store.rs` is not a read (it writes an index and moves the
   file) and stays; "Not in scope" says why. Pinned by the `c124_`
   tests already on the door; no new test, since the example is a
   binary outside the gate by its own doc.
6. **A publish syncs its directory.** `sync_dir` moves to `lib.rs` as
   `pub(crate)`, the one function both publishes call; `export`
   calls it after a successful `hard_link` or its rename fallback. No
   test can observe an fsync; the rule is one function at both sites,
   and the commit message says the pin is by inspection.
7. **The two recovery scenarios** of the issue's second comment, each
   a `c144_` test named for its fault: the restart after a failed
   record (`tests/dispatch.rs` or `tests/queue.rs`, on
   `persisting_daemon`, `schema_too_new` and the account server the
   test at 1932 uses; two lifetimes; the sent log read at the end)
   and the login finalised under an unwritable index (`tests/login.rs`,
   `staged_login` under `freeze`, unix only; then
   `start::restore_sessions` over the thawed directory as
   `restored_daemon` drives it). Each asserts what the second lifetime
   holds and what the server saw, never a task's report of itself (the
   #103 pattern).
8. **The 401-restart case** is not touched: the index-first order in
   `persist_credentials` stands, so no restart test under a frozen
   index is owed here. Since this step closes #63, the case needs a
   home: the close files it as its own issue (`area:daemon`,
   `trigger:live`, the first comment's text verbatim with its four
   conditions and its cost of one 401), named in the close message.
9. `cargo build --workspace`, then the gate in the background with its
   table to a file under the session's scratch; one commit per concern
   (2; 3; 4 as one commit for the one rule across two files, or two if
   the jobs stamp reads as its own story; 5; 6; 7 as two commits, one
   per scenario), each message naming this brief; the test rides in
   the commit of the fix it pins. Then the README's "Current step" as
   built, review pending, and the Codex review over the range.

## Done when

- A test in `crates/acquisition-store/src/lib.rs` fails a deferred
  write behind a committed one with `DatabaseBusy`, and shows
  `Store::record` and `Store::rebuild` each waiting for a held `BEGIN
  IMMEDIATE` and landing after it, the wait proven by the busy handler's
  invocation; `grep -n sleep` over the test's body finds nothing.
- `index_apply` and `persist_credentials` take a `&held::CredentialsHeld<'_>`
  (*settled in the round*, Do 3);
  `grep -rn 'fn index_apply' -A 3 crates/acquisition-daemon/src/daemon/kinds.rs`
  shows it; the test comment at `index.rs` 700 no longer names
  `shared` (`grep -n 'under .shared.' crates/acquisition-store/src/index.rs`
  finds nothing).
- `Store::init` reads `user_version` before `ensure_wal`; a test opens
  a rollback-journal file stamped newer and reads `journal_mode` as
  `delete` after the refusal. `JobDb::init` refuses a newer stamp,
  stamps a fresh file and an unstamped one; three cases in `jobs.rs`'s
  tests.
- `grep -n 'Store::open(' crates/acquisition-store/examples/read-corpus.rs`
  finds nothing; `open_read_only` is there.
- `sync_dir` is one `pub(crate)` function in `lib.rs`; `grep -rn
  'sync_dir(' crates/acquisition-store/src` prints its definition, the
  index's call and the export's.
- Two `c144_` tests in the daemon crate stage the restart after a
  failed record and the login finalised under an unwritable index, each
  reading the second lifetime's state, and the restart test the server's
  log (*settled in the round:* the login test has no server).
- Do 1's ruling is recorded in `decisions/store.md` by the owner's
  word before any code for it; if the word is to build items 2 to 8
  first, the close says what of Do 1 is built and what waits.
- The diff of the step's range touches no send path:
  `git diff --stat <first>^..<last> -- crates/acquisition-daemon/src/ratelimit crates/acquisition-daemon/src/daemon/kinds.rs`
  lists `kinds.rs` for `index_apply` and `record`'s doc alone, nothing
  under `ratelimit`.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The absorb policy: C144's sentence, closed at `eb1241f6`; this step
  restates nothing of it.
- `copy-as-store.rs`: it writes an index and moves the file, a maker
  and not a read, and its header read through the writing door is what
  lets `acq` migrate the placed copy; a read-only open there would
  refuse a legacy copy before it can be placed.
- The Windows directory sync: `sync_dir` is unix only by design (the
  previous index whole, never a torn one); unchanged.
- The pattern at `lib.rs` 4014 (a 300 ms sleep proving a migrating
  open waits): named in the report for an issue, not rewritten here.
- The 401-restart case: Do 8, an issue at the close.
- The frontends' words for an unreadable annotation row and the
  `acq price` surface: agent-facing design that follows Do 1's ruling,
  built in this step only on the owner's word, else listed at the
  close as left.
- `daemon.db` migration beyond the stamp: no column changes; the stamp
  is the floor a later change migrates from.
- A pin that a frontend's `store rebuild` or `import` beside a live
  daemon succeeds end to end as processes: the store crate's test
  holds the property at the lowest crate (C122); a process test would
  spawn a daemon for a property it does not add.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; scratch
under this session's scratchpad in a subdirectory named `63`; kill
only the processes you started. Every `acq` this step might run is in
a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under that
scratch); none is planned.

## Report

`TEMPLATE.md`.
