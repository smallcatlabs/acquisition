# r2 — The credential I/O leaves the shared lock: the design

A Fable design for readiness step r2 (`roadmap/README.md`, row r;
the brief is `r2-keyring-design.md`; #81, and #55 decided in section
5). Every `file:line` is at `668b2e8f`. Nothing here is built: the
output is this document and the drafted ruling in section 7, which the
owner rules on before `r2-build.md` is written.

## 1. The problem, in the tree

The daemon's one state lock, `shared` (`crates/acquisition-daemon/src/daemon.rs:842`),
is held across blocking I/O on four paths, and every connection takes
it before it reads `hello` (`daemon/server.rs:85-91`). The I/O is the
OS keyring (a save, and one clear per name the account may be under)
and the account index (`accounts.json`: a load, then one to three
atomic, synced writes, `acquisition-store/src/index.rs:442-460`). On
macOS an ad-hoc-signed `acqd` prompts on a keychain modify, so the
save can hold the lock for as long as the dialog stands. While it
does: a client's `hello` waits behind `shared`, meets the 5 s handshake
deadline and reports the daemon unresponsive (C86's "a healthy daemon
answers `hello` in milliseconds" is false in that state); a job past
admission reads its bearer under `shared` (`daemon/kinds.rs:98`) and
holds its gate permit while it waits; `daemon status`, `auth status`
and the dashboard wait with them; and the exit's drain, which never
blocks on `shared` (`daemon/server.rs:493`), cannot see what is in
flight and cuts at its bound. With C135 unbuilt every lifetime
refreshes at its first token use, so the first job after each rebuild
is the one that meets the dialog.

## 2. The trace: what each path holds the lock for

For each path, what runs under one hold of `shared` today, and which of
it needs the lock. "Needs" means: a write to the session table or a
decision another session-table writer could invalidate. The I/O needs
*ordering* with the other three paths (C144's property: a logout
lands wholly before or wholly after a refresh's landing; no login
lands between a forget's clear and its mark), and that ordering is
what today's single hold buys. It does not need `shared`.

| Path | Under `shared` today | Needs `shared` | Needs ordering, not `shared` |
| --- | --- | --- | --- |
| **Refresh** (`finish_refresh`, `daemon/refresh.rs:435`, through `install_tokens_locked` `:71` and `persist_credentials_locked` `:126`) | flight-current check; the token install; the index load (`:138`); the index hold-name write; the keyring save; a clear on failure; the index record write; the old-name clears; the settle write; the session rename; the keyring mark | the flight-current check, the install (access token, expiry, refresh token, username, generations, flight cleared), the rename, the keyring mark | every index load and write, every keyring call |
| **Login** (`complete_login`, `daemon/login.rs:233`) | pending-and-staging check; the staging slot taken; the same persist; on an index failure a rollback clear and the flow failed; on success the session replaced and the flow closed | the staging taken, the flow's result, the session replaced | the persist, the rollback clear |
| **Logout and forget** (`logout`, `daemon/login.rs:418`) | the live-or-known decision (a forget loads the index, `:518`); the session dropped; the index load for the entry's names (`:432`); every clear (`:641`); the settle write | the decision, the session dropped | the index loads, the clears, the write |
| **The profile job's uuid** (`with_index`, `daemon/kinds.rs:157`, called at `:381`) | an index load and write, under `shared` only to serialise with the other three | nothing | the index write |

The restore at start (`daemon/start.rs:464`) reads the keyring under no
lock, before `serve`; it is outside this design (section 9).

## 3. The shape: the landing, under a credentials lock

**The rule.** The session table changes under `shared`, in holds that
do no I/O. The keyring and index I/O of the four paths runs under one
new lock, `credentials`, on a blocking thread
(`tokio::task::spawn_blocking`), and never under `shared`. No request
path, no bearer read, no status document and no `hello` takes
`credentials`, so none can wait behind a prompt. The lock order
becomes `credentials` → `store` → `shared` → `jobs_db` →
`queue_failure` → `log`: a holder of `credentials` may take `shared`
to publish; nothing holding `shared` takes `credentials`. The
lock-order comment (`daemon.rs:833`) gains the one sentence.

**The landing.** Each path becomes one function run on a blocking
thread, holding `credentials` for its length and taking `shared` twice
inside, briefly:

1. *Decide and install, under `shared`.* The path checks what today's
   first lines check and makes its session-table change: the refresh
   verifies its flight is current and installs the tokens, renames the
   session if the provider renamed the account, and sets the session's
   keyring mark to a transitional word, `saving` (`auth status` must
   not say `ok` for a save that has not landed, C144); the login takes
   the staging slot and marks the flow as landing; the logout decides
   live-or-known and drops the session. Then `shared` is released.
2. *The I/O, under `credentials` alone.* `persist_credentials_locked`
   becomes `persist_credentials`, body unchanged: the index holds the
   name, the keyring saves, a failed save clears the entry it would have
   replaced, the index records, the old names are cleared, the
   released names are settled. The logout's clears and write, the
   login's rollback clear and the profile job's uuid write run here
   too.
3. *Publish, under `shared`.* The refresh finds its session by the
   generations it installed and writes the keyring mark (`ok` or
   `unavailable: …`, today's words) and clears the flight; the login
   replaces the session with the staged tokens, or on an index failure
   fails the flow whole as today (C50), and clears the landing mark;
   the logout has nothing left to write. Then `credentials` is
   released, and the warnings are logged outside every lock
   (`note_error` takes `shared`, `daemon.rs:968`).

**Why `credentials` is a lock and not "compute outside, publish
inside" alone.** Without it the refresh's save would run while a
logout could take `shared`, drop the session, clear the keyring and
mark the index not persisted; the save would then land a rotated token
in a keyring entry the index says is gone, the fault C144 names (no
credential out of the index's reach). The ordering today's one hold
gives the four paths is kept by giving the I/O a lock of its own. The
cost: a logout's clears hold `credentials` while a refresh waits to
install; that wait is a user's act and bounded by the clears, and the
refresh's waiters are not jobs under permits (the C87 retry refreshes
outside the permit, `daemon/kinds.rs:662`). A per-account lock would
remove it, and renames and the one index file would cross accounts
under it; one lock, until a run shows the wait.

**Why a blocking thread.** The daemon's runtime is tokio's
multi-thread default (`main.rs:21`), one worker per core. A landing
run inline on the caller's worker, as today's I/O is, blocks that
worker for the prompt's length; the others serve `hello`, until a
second landing waits on `credentials` from a second worker, and on a
two-core guest that is every worker. `spawn_blocking` makes "no worker
blocks on credential I/O" a property of where the code runs, not of
the core count, and the waiting on `credentials` happens on blocking
threads too. The closure needs the daemon as `Arc<Self>`: the
dispatcher's `run_active` (`daemon/dispatch.rs:119`), `auth_start`
(`daemon/login.rs:16`) and the request handler already hold one, so
`valid_access_token`, `complete_login`, `logout` and `with_index`
take `self: &Arc<Self>` through their callers: about a dozen
signatures (the kind runners, `api_get`, the refresh pair and its
guard, the login pair, `logout`, `with_index`). Or none: the landing's
inputs (`shared`, `credentials`, the credential store, the store
directory, the log) held in `Arc`s on `Daemon` and cloned into the
closure. The build picks and says which; the property is the same.

**Who waits for what.** The refresh's owner awaits its landing whole
and gets, as today, the token with the save's outcome behind it. Every
other reader is served at step 1: a waiter on the flight, because the
flight's watch channel is published at the install, not at the end
(`wait_for_refresh`, `daemon/refresh.rs:531`); a bearer read under a
permit, because the token is in the session from step 1
(`SessionBearer::token`, `daemon/kinds.rs:98`); a `force_refresh`
caller arriving between steps 1 and 3 finds the flight still set and
joins it, as a joiner of an in-flight refresh does today. The flight
stays set until step 3, so the drain (`InFlight::of`,
`daemon/server.rs:653`) counts the save.

**The single-flight guard.** `RefreshOwnerGuard` (`daemon/refresh.rs:17`)
still serialises refreshes per account and still publishes on drop
when the owner leaves before its POST lands. Once `finish` has handed
the flight's sender to the landing closure the guard has nothing to
publish: a `spawn_blocking` closure runs to completion whether its
awaiter lives, so an owner cancelled during its landing no longer
strands its waiters (the checklist's "an owner that leaves without
publishing" shape is held by the closure, not the guard).

**A save that fails.** Unchanged from today, and the brief's sentence
is wider than the tree: the refresh's job does *not* fail on a failed
save. `c144_a_failed_save_clears_the_entry_it_would_have_replaced`
(`daemon/tests/refresh.rs:595`) asserts the token is returned and "the
session runs on, in memory", the session's keyring mark says
`unavailable: keychain locked`, the entry is cleared, and the failure
is in the error ring. The design keeps exactly that: the mark lands at
step 3, the warning after it. What changes is that `auth status` can
show `saving` while the prompt stands, which is the true state.

**The exit's drain at the bound.** The drain waits for a save in
flight as it waits for a refresh today (the flight is set until step
3), up to C133's bound; `exit_waits_for_a_refresh_and_installs_the_token`
(`daemon/tests/exit.rs:120`) stays green. At the bound the process
leaves with the save cut: the keyring holds the previous token (the
save never landed) or the new one (landed, the mark unrecorded), the
session leaves with the process, and the next lifetime restores what
the keyring holds; a rotated-away token is a dead grant C24 marks at
its first use, and the user logs in again. That is today's outcome at
the bound, reached now with `hello` and `daemon_stop` answered
throughout, and with the drain able to see the save (it no longer
finds `shared` held).

**The second login.** Today `begin_auth_flow` (`daemon/login.rs:98`)
supersedes any pending flow and clears the staging slot under
`shared`; a login's landing was atomic, so no flow was ever superseded
mid-record. With the landing in three holds a second `auth_start`
could clear the flow between steps 1 and 3 after the credential was
saved and the index marked persisted, registering no session for a
login the keyring holds. So a landing marks itself (`Sessions` gains
the landing flow's generation beside `pending`), `begin_auth_flow`
refuses while one is set, and the drain counts it as it counts a
staged login with a terminal profile job (`daemon/server.rs:653`,
the `login` clause). The refusal's kind and words are C133's shape,
`refused`, "try again in a moment": *a login is being recorded; try
again in a moment*. A browser-waiting login is still superseded as
today. The words are the session's, agent-facing, listed at the close.

**Two test seams move.** `refresh_landed_hook` (`daemon.rs:901`), "the
pause where a refresh's landing releases `shared`", runs after step 3,
once `credentials` is released: `c144_a_logout_where_a_refresh_releases_the_lock_keeps_its_mark`
(`daemon/tests/refresh.rs:687`) then holds as written (the save landed
before the logout, the mark stands). The hook placed at step 1's
release would deadlock: its logout would take `credentials` from the
thread that holds it.
`c144_a_login_cannot_land_between_a_forgets_clear_and_its_mark`
(`daemon/tests/login.rs:1151`) tries `shared` at the forget's clear
(`on_clear`, `harness.rs:107`) and completes the login inline when it
is free; after, `shared` is free there and the inline completion would
take `credentials` on the holding thread. The test keeps its property
and changes its probe: it tries `credentials`, finds it held, and
completes the login after the forget returns, asserting as now that
the login landed and the entry is persisted.

## 4. `hello` ahead of the lock

The connection count and `last_activity` leave `Shared`
(`daemon.rs:730-731`) for two atomics on `Daemon`: an `AtomicUsize`
and an `AtomicU64` of nanoseconds since `started` on the daemon's
clock (`choke.now()`, the clock `is_idle` already reads). `handle_conn`
increments and stamps before reading `hello` as it does now, taking
no lock; the decrement at the loop's end (`daemon/server.rs:231`),
the submit's stamp (`daemon/queue.rs:438`), `daemon status` and the
dashboard read or write them the same way. `is_idle`
(`daemon/server.rs:589`) reads the atomics beside its hold of
`shared` for the jobs and the pending login.

Why atomics and not the bookkeeping after the `hello` exchange: the
count's job is the idle verdict, and today the increment precedes
everything on the connection, so a verdict can never read zero with a
connection accepted and un-greeted. Moving the increment after
`hello` opens that window for the length of the exchange. Why not a
second small lock: two words need no ordering between them that the
verdict depends on, and the verdict's one race, a connection accepted
after it read the count, is today's and unchanged (the connection
meets `refuse_while_stopping` on submit and `hello` is still answered,
C133). With the I/O off `shared`, nothing `hello` takes is held
across I/O: `log` is innermost and never held across more than a
line's write.

## 5. C135, the access token in the keyring (#55)

After, as its own round. This step changes *when* the credential I/O
runs; C135 changes *what* the keyring entry holds and what the restore
reads, in the same two functions (`persist_credentials`, `restore_sessions`),
and one review should read one change. C135 reduces how often the
refresh runs (one token POST per ten hours instead of one per
lifetime) and does not remove this schedule: the first refresh after
a rebuild still meets the dialog, at expiry instead of at first use.

The start's keyring read is a second site of the same symptom, and
the tree already records that it prompts: `tools/sign-acqd.sh`'s
header (2026-09-12) says an ad-hoc-signed `acqd` is asked twice per
login after every rebuild, "the read at start, the modify on save".
The socket is bound (`daemon/start.rs:427`) before the restore reads
the keyring (`:464`) and served after (`:598`), so a client that
connects during that prompt waits for `hello` past its deadline and
reports the daemon unresponsive, before any save. The read holds no
lock, so this design's rule does not reach it; after C135 it is the
one prompt a rebuild meets. Its fix is a lifecycle state, not a lock:
serve before the restore, the restore on a blocking thread, the
sessions arriving under `shared` with `auth status` saying
*restoring* until they do, the dispatcher, the drain and the idle
verdict holding for it. The restore is the function C135 rewrites
(two tokens read instead of one), so the recommendation is an issue
now and the design in the C135 round, one change to the restore
reviewed once; the owner may instead fold it into r2's build as a
fourth site, at the cost of a lifecycle change in a step cut as a
lock change.

## 6. Where this meets the time seam

`TIME-SEAM.md` section 4 names the daemon's lifecycle machine (start,
listening, the idle verdict, the drain, one exit) as 4-0's second
candidate and places "the keyring save stalling under `shared`"
inside it. This step does not wait for the seam and gives the machine
two of its inputs: a landing is a state of the session (installed,
saving, marked) rather than a lock hold, which a machine can name; and
the connection count and activity stamp are atomics the verdict reads,
which a machine over an injected clock reads the same way. The three
tests below are forced with a blocking store and never a clock, so
they are the seam's shape already. The one pin this step leaves to the
seam is the save cut at the bound (section 3): a test that advances
the drain's deadline past a blocked save is a lifecycle-machine test,
written when the drain's polls become transitions; today's `exit.rs`
pins the bound with tokio's paused time, and the build may pin it the
same way if the owner wants it before 4-0.

## 7. The drafted ruling

One new entry, not an amendment: C86 states the handshake's deadline
and its verdict, C144 states what a credential result must reach;
neither says where the I/O may run, and both are cited from the client
and the store, which this rule does not touch. The id is the next free
one (147 is the highest in the registry), written when the owner
rules; `C<n>` below stands for it. Measured with a three-digit id in
place: 780 bytes.

- **C<n> — No credential-store or account-index I/O runs under the daemon's shared lock, and `hello` is answered under no lock such I/O can hold.** A refresh, a login, a logout and a forget change the session table under `shared` and do their keyring and index I/O under one credentials lock, on a blocking thread, publishing the result under `shared` after; a waiter on a refresh and a bearer read under a permit are served at the install, never the save; a login being recorded refuses a second; the exit's drain waits for a save in flight to its bound. *Why:* a keychain prompt under `shared` made every client report the daemon unresponsive and held jobs behind their token read (#81). *Pinned:* `acquisition-daemon/src/daemon/tests/credentials.rs`, the three `c<n>_` tests.

## 8. The tests

Three new tests in one module, `daemon/tests/credentials.rs`, each
staging one of the brief's schedules with a credential store whose
`save` and `clear` block until the test releases them (a
`RecordingCredentialStore` with a gate, or a sibling), and never a
clock (C137). Their names carry the ruled id.

| Test | Stages | Expected after |
| --- | --- | --- |
| `c<n>_hello_is_answered_while_a_refresh_saves` | a refresh's landing blocked in `save`; a connection over a Unix socket (`exit.rs:1097` serves one in-process) sends `hello` | the `hello` reply arrives while the save is blocked; the save is then released and the session's keyring mark is `ok` |
| `c<n>_a_permit_holder_reads_the_installed_token_while_the_save_blocks` | a refresh's landing blocked in `save`; a job past admission with a gate permit reads its bearer | the bearer read returns the token the refresh installed, before the save is released; the session's mark reads `saving` under the read |
| `c<n>_status_answers_and_a_second_login_is_refused_while_a_login_lands` | a login's landing blocked in `save`; `daemon status` is asked and `auth_start` is called | the status answers with the login pending; the second start is refused in the words of section 3; the save is released and the first login registers its session |

Before the build, each of the three is run against the tree as it is
and must fail there in the way section 1 describes (the `hello` and
the status wait; the bearer read waits), so the build's commit can say
what moved.

What must stay green, named: the `c144_` family in the daemon
(`refresh.rs`, `login.rs`), with the two seam changes of section 3;
`rejected_access_token_*` and `a_cloudflare_401_on_the_get_keeps_the_access_token`
(`bearer.rs`, C87: the 401 clears under `shared` alone and is
untouched); `exit_waits_for_a_refresh_and_installs_the_token`,
`c133_a_stop_between_the_profile_and_the_login_waits_for_the_login`
and `a_login_whose_profile_job_waits_is_not_held_by_the_drain`
(`exit.rs`); `a_superseded_login_cannot_complete_and_leaves_no_zombie`
(`login.rs:286`, a browser-waiting login is still superseded);
`a_bare_harness_daemon_saves_a_rotated_token_into_the_test_store`
(`refresh.rs:308`, invariant 5's test side); the stress's
`wait_for_refresh_waiters` (`stress.rs:9`) counts receivers on a
flight that is now set until step 3, which only widens its window.

The measurement the build brief asks for: the `hello` round trip over
a socket while a save is blocked, in-process on the mock provider
with the blocking store, before and after, stated as a number in the
build's message. Before, it is the client's handshake deadline (5 s,
`HELLO_WITHIN`) or unbounded from a raw socket; after, milliseconds.

## 9. Corrections to the brief, and what is left for the owner

Written from the tree, as the template asks:

- "The refresh's job fails naming it" (the brief's Input): it does not,
  today or after; section 3 says what reaches whom, and the `c144_`
  test that pins it.
- `server.rs` and `harness.rs` are `daemon/server.rs` and
  `daemon/tests/harness.rs`; `handle_conn`'s hold is at `:85-91`, not
  85 to 92; `is_idle` is at `:589`.
- The index write is one to three synced writes per landing, not one
  (`persist_credentials_locked`: the hold-name write, the record
  write, the settle write).

Left for the owner, beside the ruling:

- The start's keyring read (section 5): the same symptom at start,
  recorded in the tree, held by no lock. Recommended: an issue now,
  designed in the C135 round; the alternative is a fourth site in
  r2's build.
- The second login's refusal words (section 3) are agent-facing and
  the session's; listed here so the close can list them.
- Whether the save cut at the bound is pinned now with paused time or
  left to 4-0 (section 6).

## Not in scope

As the brief's: the lock merge whole, the typed `JobKind`, `frame.rs`
shared; the time seam; the quote's simulation under the limiter lock
and the outranked check; keyring 4 or any change to which store holds
the token; and the start's keyring read, unless the owner folds it in
(section 5).
