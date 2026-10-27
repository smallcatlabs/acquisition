# c135 — The access token beside the refresh token, and the start's read off the socket's path

The C135 round (`roadmap/README.md`, "Current step"), the next step of
the League 3.30 milestone after the readiness group. One round, two
issues that are two sites of one question — when the daemon touches
the keyring, and what a client sees while it does: #55 (C135, the
access token kept in the keyring beside the refresh token and restored
with it, so a lifetime refreshes only at expiry) and #92 (the start's
keyring read off the path `hello` waits on). The shape is
`r2-design.md` section 5, accepted by the owner as the issue on
2026-10-26; this brief draws it from the tree at `c5a736db` (every
`file:line` is there) so an Opus worker builds it from this document
alone. One change to the restore, reviewed once. About two days.

## Goal

A keyring entry that holds the access token and its expiry beside the
refresh token, written by the refresh's and the login's landings and
read by the start's restore, so a restored session sends no token POST
until its access token expires and an entry an earlier daemon saved
restores as it does today. And a start whose socket is served before
the restore reads the keyring: the restore is the daemon's fifth
landing, under the credentials lock on a blocking thread (C148, no
credential I/O under the shared lock), its sessions arriving under
`shared`; `hello`, `daemon_stop` and the status documents answer while
it reads, `auth status` saying the restore is under way; a request
that needs a session waits for it; the dispatcher, the idle verdict
and the exit's drain hold for it. Why now: with C148 built, the read at
start is the one keychain prompt a rebuilt, unsigned `acqd` meets
(`tools/sign-acqd.sh`'s header: "the read at start, the modify on
save"), and today that prompt stands between the socket's bind and its
serve, so every client reports the daemon unresponsive (C86) before any
save; and every lifetime's first token use costs GGG a token POST that
a ten-hour token makes needless (C135's *Why*).

## Read

- `roadmap/README.md`, "A step" and "Current step"; `r2-design.md`
  section 3 (the landing: three holds, the lock order, `spawn_landing`)
  and section 5 (this round's shape, verbatim); `r2-build.md` Do 2 (how
  r2 built a landing; the daemon reaches the closure as `&Daemon`).
- `decisions/daemon.md`: C135 (the ruling, Unbuilt, naming #55 and the
  pin `a_restored_session_sends_no_refresh_before_expiry`, which does
  not exist yet); C148 (the landings: what this round extends, the Do 1
  question); C133 (one exit, a bounded drain; the refusal shape "try
  again in a moment" is a conflicting or departing daemon's, not an
  early request's); C134 (a panic in a landing aborts, under the job's
  mark or none); C86 (the handshake deadline: a healthy daemon answers
  `hello` in milliseconds); C3 (the idle exit, an hour; the agent's
  recipe); C6 (the queue read back at start, as amended today: a
  concluded tree may leave memory); C144 (every credential result
  reaches the words: a restore's load failure already does, start.rs:685
  and :726); C50 (a lost login's profile fetch fails at restore,
  queue.rs:206, untouched); C137 (no wall-clock bound: every schedule
  here is forced by a held store call). `decisions/network.md`: C24 (a
  rejected grant is dead: what an older daemon reading the new entry
  meets); C87 (the 401 clears under `shared`; untouched); N33 (the token
  endpoint's own policy; a POST is a journal fact).
- #55 and #92 (`gh issue view`), the owner's rulings in their bodies.
- `crates/acquisition-daemon/src/daemon/start.rs`: `run_with_log`
  (`:400`): `Listener::bind` (`:428`), the restore (`:465`, reads
  the keyring before the daemon exists), `Daemon` built (`:488`, the
  sessions placed in `shared`), the start's log lines (the keyring
  summary and the session names, `:560-:576`), `restore_jobs` (`:594`), the
  dispatcher and the watchdog spawned (`:600-:601`), `serve` (`:603`;
  its definition `:624`); `load_reached` (`:685`) and `restore_sessions`
  (`:726`, a pure function of the store, the index and the log: five
  tests call it, named below).
- `daemon.rs`: `Staging` (`:512`, the login's access token and expiry
  are already there), `AuthSession` (`:529`, `access_expires_at` is
  wall-clock `SystemTime`, and why), `Sessions` (`:553`; `keyring`
  `:576`, "shown when there is no session to ask"), `Daemon` (`:898`;
  `credentials` `:904`, `landings` under `Shared`), `CredentialStore`
  (`:977`, the trait: `save` takes the refresh token today),
  `OsCredentialStore` (`:985`), `spawn_landing` (`:1028`),
  `mark_activity` (`:1052`), `resolve_account` (`:1121`),
  `kind_needs_account` (`:1169`: in the mock `fetch` and `sleep` need
  none).
- `auth.rs`: `keyring_disabled` (`:344`), `keyring_save` (`:355`),
  `keyring_load` (`:363`), `keyring_clear` (`:395`): the entry today is
  the bare refresh token, one string under the username, no version.
- `daemon/refresh.rs`: `install_tokens_locked` (`:84`),
  `persist_credentials` (`:122`, the one persist, login's and
  refresh's), `valid_access_token_once` (`:279`; the one expiry check,
  `:300-:306`, a 5 s margin), `finish_refresh` (`:440`), `Landing`
  (`:564`), `Persisted` (`:573`).
- `daemon/login.rs`: `auth_start` (`:19`), `begin_auth_flow` (`:126`),
  the staging's expiry (`:153`), `complete_login` (`:271`; persists at
  `:302` from the staging), `logout` (`:492`) and `logout_blocking`
  (`:574`).
- `daemon/server.rs`: `keyring_summary` (`:41`, the one word the three
  status documents print), `auth_status` (`:47`), `handle_request`
  (`:236`; the session-reading requests are `Submit` through
  `resolve_account`, `AuthStart`, `AuthCheck` through
  `canonical_account`, `AuthLogout`), `refuse_while_stopping` (`:455`),
  `drain` (`:515`), `is_idle` (`:611`), `InFlight::of` (`:680`) and
  `describe` (`:735`). `daemon/dispatch.rs`: `dispatcher` (`:49`),
  `pick_runnable` (`:60`; the exit's check at its head, `:67`).
- `crates/acquisition-protocol/src/protocol.rs`: `SessionStatus`
  (`:428`), `Response::Auth` (`:569`; `keyring` `:592`), `DaemonStatus`
  (`:598`; `keyring` `:625`), `Dashboard` (`:636`; `keyring` `:648`):
  three documents, one field, one summary function. The protocol does
  not change in this round.
- `crates/acquisition-client/src/client.rs`: `HELLO_WITHIN` (`:339`),
  `exchange` (`:1681`: a request's answer is read under no deadline,
  which is why an early request may wait).
- `crates/acquisition-cli/src/main.rs:1836-:1877`, `auth status`'s
  text: "not logged in — run acq auth" when `logged_in` is false, and
  `keyring: {word}` last.
- The tests. `daemon/tests/harness.rs`: `ManualClock` (`:22`; `advance`
  moves both faces), `RecordingCredentialStore` (`:91`; `entries` holds
  the secret string by username; `on_clear` `:107`, `on_save` `:110`),
  `logged_in` (`:907`). `daemon/tests/credentials.rs`: `answered`
  (`:26`), `hold_the_next` (`:45`), `saving_daemon` (`:63`), the six
  `c148_` tests (`:122`, `:175`, `:228`, `:317`, `:391`, `:509`); the
  first serves a harness daemon over a Unix socket and is the shape of
  this round's `hello` test. `daemon/tests/refresh.rs`:
  `token_daemon_with_store` (`:529`),
  `c144_restore_reaches_a_token_under_a_held_name` (`:971`),
  `c144_restore_reports_a_held_names_failed_load` (`:1005`).
  `daemon/tests/login.rs`:
  `c144_restore_reads_through_the_credential_store_and_reports_a_failed_load`
  (`:909`). `daemon/tests/sessions.rs`: `MemoryCredentialStore`
  (`:62`, a third store the trait change touches). `daemon/tests/exit.rs`:
  `exit_waits_for_a_refresh_and_installs_the_token` (`:116`),
  `the_idle_verdict_holds_while_a_login_is_pending` (`:346`).
  `daemon/tests/idle.rs`:
  `idle_verdict_follows_the_clock_the_connections_and_the_queue`
  (`:47`). `crates/acquisition-client/tests/harness/mod.rs`:
  `start_daemon_with` (`:416`) and `start_daemon_in_process` (`:925`),
  the two waits every process test's daemon comes up through, both on
  `daemon status`; the isolation list (`:171`) turns the keyring off in
  every mock world, as the mock-session skill does.
- The tests that assert on the words the status documents print, found
  by grepping the test trees for `keyring`:
  `crates/acquisition-cli/tests/seat_faults.rs:66`
  (`a_fresh_daemon_told_not_to_use_the_keyring_never_reports_it_ok`,
  asks `auth status` on a daemon just started and holds
  `keyring: unavailable: disabled by ACQ_NO_KEYRING`, text and `--json`;
  its golden `goldens/seat_faults/auth_status_without_a_keyring.txt`);
  `crates/acquisition-verbs/tests/documents.rs:692` and `:721`
  (`keyring: "ok"` in a document fixture); `crates/acquisition-protocol/tests/wire.rs:337-:373`
  (the wire fixtures `fixtures/wire/response/{auth,daemon_status,dashboard}.json`,
  unchanged: no field moves). No test anywhere prints or expects the
  word `restoring`; the CLI's goldens that mention the keyring
  (`journey/a_first_session.txt:262`, `new_league/launch_morning.txt:220`,
  `seat_faults/a_stop_names_the_logins_it_ended.txt:7`) print a
  logout's sentence, not the status.
- `TESTING-NOTES.md`, "The journal is the contract surface": the pins
  table; a token POST count is a journal fact and this round adds a row.
  `REVIEW-CHECKLIST.md`, "Network" ("An owner that leaves without
  publishing"; "A parsed value that overflows": the expiry restored from
  the entry enters arithmetic and is bounded where it is read) and
  "Protocol, client, daemon lifecycle, the world".

## Input

Fixed before the work. No measurement: the before of #92 is the tree's
order (the socket bound at start.rs:428, the restore returned at :465,
served at :603: a client's `hello` waits the whole read, by
construction) and the before of #55 is one `POST oauth-token` per
lifetime at the first token use, the fact the first pin's journal
states. The keyring is off in every mock world, so neither is measured
across lifetimes in one; the journal of the pin is the number.

The schedules the tests stage, each with the recorder, never a clock
for a deadline (C137); the `ManualClock` moves the wall only to carry a
token past its expiry:

1. **A restored session with an unexpired access token sends no token
   POST before expiry, and one at expiry.** The recorder holds, under
   `test-user`, an entry in the new form: refresh token `rt-kept`,
   access token `at-kept`, expiry 3600 s after the manual clock's wall;
   the index lists `test-user` persisted; a scripted token server would
   answer a POST. The sessions `restore_sessions` returns are the
   daemon's. A `valid_access_token(None, false, None)` returns
   `at-kept` and the wire and the journal hold no POST. The clock
   advances 3600 s; the next call POSTs once and installs the server's
   token; the journal holds exactly one `POST oauth-token`.
2. **An older entry restores as today.** The recorder holds the bare
   string `rt-old` under `test-user`. The restored session has the
   refresh token and no access token; the first use POSTs once; the
   landing's save rewrites the entry in the new form, read back as
   such.
3. **A refresh's landing and a login's landing save the access token
   beside the refresh token.** After each, the recorder's entry parses
   to the installed access token, the session's expiry (as unix
   seconds) and the rotated refresh token.
4. **A client connecting while the restore reads gets `hello`, and the
   status documents say the restore is under way.** A harness daemon
   whose recorder holds an entry under `test-user`, the index listing
   it persisted, the recorder's next `load` held (an `on_load` slot
   beside `on_save`); the restore spawned as the start spawns it;
   served over a Unix socket as `c148_hello_is_answered_while_a_refresh_saves`
   serves one. While the load is held: `hello` is answered,
   `auth_status` answers with `logged_in: false` and `keyring:
   "restoring"`, `daemon_status` answers. Released: `test-user` is
   live with both tokens and its mark is `ok`, and the three documents
   say so.
5. **A request that needs a session waits for the restore.** The same
   staging; a `Submit` of a kind that needs an account (`characters`;
   in the mock `fetch` needs none) is sent while the load is held and
   has not answered when the test looks; released, it answers
   `Submitted` and the job's account is `test-user`. Waits, never
   refused, for the reasons under Do 5.
6. **The dispatcher, the idle verdict and the exit's drain hold for
   the restore.** The same staging, a waiting job inserted:
   `pick_runnable` returns nothing while the load is held and picks it
   after; `is_idle(Duration::ZERO)` is false while held and true after
   with no connection; `begin_exit` then `drain` has not returned
   while held, and returns `None` (nothing cut) once released.

And one property that survives every schedule: the `c144_` restore
tests above, the `c148_` six, the C87 family (`bearer.rs`,
`rejected_access_token_*`), `exit_waits_for_a_refresh_and_installs_the_token`
and the idle tests pass unchanged, except where Do 3 renames a
parameter.

## Do

1. **The ruling, the owner's.** C148 names four landings (a refresh, a
   login, a logout and a forget) and C135 is Unbuilt. The restore
   joining the landings, the entry's form and what an older daemon
   meets are properties another area must know (the client's door, a
   release's downgrade), so they are registry sentences, entered on the
   owner's word before the commits of Do 5 to 7 and not by the
   worker. Recommended text, 762 bytes together, the amendment to C148
   one sentence and the C135 entry rewritten:

   > C148, after "publishing the result under `shared` after;": *the
   > start's restore is the fifth landing: the socket serves before it,
   > a request that needs a session waits for it, and the dispatcher,
   > the idle verdict and the exit's drain hold for it;*
   >
   > **C135 — The access token is kept in the keyring beside the
   > refresh token and restored with it; a lifetime refreshes only at
   > expiry.** The entry is one JSON object the daemon alone writes and
   > reads; a bare token, an entry an earlier daemon saved, restores as
   > before and is rewritten at its first refresh; an earlier daemon
   > reading the new entry meets a dead grant (C24) and logs in again.
   > *Why:* one token POST per lifetime was one per minute of idle
   > cycling. *Pinned:* `acquisition-daemon/src/daemon/tests/refresh.rs`,
   > `a_restored_session_sends_no_refresh_before_expiry`.

   Do 2 to 4 depend on no sentence the owner has not already given
   (C135's head, #55's body) and are built first; Do 5 to 7 are built
   once the amendment is entered, under whichever words he enters.

2. **The entry.** In `auth.rs` beside the keyring functions, one type
   for what the keyring holds: the refresh token, the access token and
   its expiry as unix seconds, the last two optional; one function
   writes it as a JSON object and one reads it, and a secret that is
   not such an object is a bare refresh token (schedule 2). The expiry
   read back is bounded where it is read, as `expires_in` is
   (`auth.rs:283`): a value past the clock plus
   `MAX_ACCESS_TOKEN_LIFETIME_SECS` restores no access token. No
   version number: the object's shape is the version, a later field is
   optional, and the bare string is the form before it. Two unit tests
   in `auth.rs`: the round trip, and the bare token read as one.
   `CredentialStore::save`'s second parameter becomes the secret (a
   string the trait never parses); the three test stores change their
   parameter name and nothing else; `OsCredentialStore` and
   `keyring_save` follow. The format is written and read in
   `persist_credentials` and `restore_sessions` alone, the two
   functions section 5 named.

3. **The landings write it; the restore reads it.** `Landing`
   (`refresh.rs:564`) gains the access token and expiry taken from the
   session at the install, so what is saved is what the session holds;
   the login's landing takes them from `Staging`; `persist_credentials`
   takes the entry and saves its secret. `restore_sessions` builds the
   session with both tokens and the expiry, and does not judge expiry:
   the one check at `refresh.rs:300` decides at the first use. The
   tests of schedules 1 to 3 in `refresh.rs` and `login.rs`, the first
   named as C135 names it; each run against the tree first, the
   failure quoted in the commit (schedule 1 fails on the POST the
   journal shows). The tests that assert on the recorder's saved
   strings (`store.entries`, `store.saves`: 52 lines over seven files,
   `grep -rn 'saves.lock\|entries.lock' crates/acquisition-daemon/src/daemon/tests`)
   compare the parsed entry's refresh token where they compared the
   string; the message counts the sites changed. `TESTING-NOTES.md`'s
   pins table gains the row: a restored session with an unexpired
   access token sends no `POST oauth-token` before expiry and one at
   it (C135). `auth check` stays a forced refresh: the POST the user
   asks for is not the one C135's *Why* names.

4. **The registry, C135.** Its Unbuilt line goes and its *Pinned:*
   names the test, in the words the owner enters at Do 1; the module
   docs of `refresh.rs` and `start.rs` say what the entry holds.

5. **The restore as a landing.** `restore_sessions` stays the pure
   function its five tests call. `run_with_log` builds the daemon with
   `Sessions` whose `keyring` is `restoring` and no session, logs its
   start lines, restores the queue, then spawns the restore through
   `spawn_landing` (counted in `landings`, under `credentials`, on a
   blocking thread, under no job's mark) before the dispatcher, the
   watchdog and `serve`; the landing calls `restore_sessions` with
   the daemon's store, service, directory and `keyring_disabled()`,
   collecting its log lines, and publishes under `shared`: `by_account`,
   `last_login` and `keyring`, never the flow state (`pending`,
   `staging`, `landing`, `next_flow`, `flow_result`, `exchanges`),
   which belong to requests that waited. Then it marks activity
   (`mark_activity`, so the idle hour counts from the restore and a
   prompt that stood an hour is not followed by an exit), wakes the
   dispatcher (`work.notify_one()`), logs the lines it collected and
   the summary the start logs today ("keyring: … | session: …", moved
   here, since the start no longer knows it). One primitive says
   whether the restore has landed (a `tokio::sync::watch` on `Daemon`
   is the recommendation; the harness daemons start landed, having
   nothing to restore). It is awaited, after `refuse_while_stopping`
   and before the session is read, by the four session-reading
   requests: a `Submit` whose kind needs an account, `AuthStart`,
   `AuthCheck`, `AuthLogout`. **A request waits, never refused**:
   the client's spawn (C3, C88) submits the moment `hello` answers, so
   a refusal is a race every mock-world `acq` would meet and a user's
   first `acq` after a rebuild would be told to try again while the
   prompt it must answer stands; the client reads a request's answer
   under no deadline (`client.rs:1681`), so the wait is the prompt's
   own length and ends when the user answers it; and C133's refusal
   ("try again in a moment") is a departing daemon's or a conflicting
   login's, not an early request's. `hello`, `daemon_stop`, the status
   documents, `list`, `status`, `result`, `quote`, `cancel`,
   `set_priority` and `subscribe` never wait. `pick_runnable` returns
   nothing until the restore has landed, at its head beside the exit's
   check (`dispatch.rs:67`): a job the queue restored (C6) would
   otherwise read a session that is not there and fail as not logged
   in. `is_idle` adds `landings == 0` to its conjunction, one general
   rule (a daemon with a keyring or index write in flight is not idle)
   that covers the restore; the drain counts it through `landings`
   already. The tests of schedules 4 to 6 in `credentials.rs`, named
   with the id the owner enters (`c148_` if the amendment is entered as
   drafted), the recorder gaining `on_load`; the daemon reaches the
   restore's closure as r2's landings do.

6. **The words.** `keyring: restoring` is the one word the three
   documents print through `keyring_summary` while the restore reads,
   as `saving` is a refresh's since r2: a field's word, not a sentence,
   and no protocol change. `auth status`'s text, when the word is
   `restoring`, prints that the sessions are being restored from the
   keyring and that a keychain prompt may be standing, instead of
   "not logged in — run acq auth", which would be false; `--json` is
   the document as it is. The words are the session's, listed at the
   close. The two waits in the client harness (`start_daemon_with`,
   `start_daemon_in_process`) take the daemon as up only once its
   `daemon status` says a keyring word other than `restoring`, so
   `a_fresh_daemon_told_not_to_use_the_keyring_never_reports_it_ok`
   and every golden hold without a race (the restore with the keyring
   off is microseconds on a blocking thread, which a starved runner
   may still lose to the first `auth status`; C137).

7. **The module docs and the checklist.** `start.rs`'s `run_with_log`
   comment and the module docs of `server.rs` and `dispatch.rs` say the
   restore is a landing and what holds for it, headed by the id. No
   checklist row unless a fault came back.

8. The gate, the commit, the review, the close: `roadmap/README.md`,
   "A step". The close lists the `restoring` word and the `auth status`
   sentence as the step's agent-facing decisions, and closes #55 and
   #92.

## Done when

- `a_restored_session_sends_no_refresh_before_expiry` exists in
  `daemon/tests/refresh.rs`, failed on `c5a736db` as the commit's
  message quotes (a POST the journal shows), and passes; the bare-entry
  test and the two save tests of schedules 2 and 3 pass; the two
  `auth.rs` unit tests pass.
- `grep -rn 'from_secret\|to_secret' crates/acquisition-daemon/src`
  (or the names the build chose, stated in the message) names
  `auth.rs`, `persist_credentials` and `restore_sessions` and no other
  production site.
- `TESTING-NOTES.md`'s pins table has the C135 row.
- C135's entry has no Unbuilt line and names the test.
- The three tests of schedules 4 to 6 exist in `credentials.rs` and
  pass; `restore_sessions` is called from a landing spawned through
  `spawn_landing` before `serve`, visible in the diff of `start.rs`.
- The four session-reading requests await the restore after
  `refuse_while_stopping`; `pick_runnable` returns nothing before it
  lands; `is_idle` reads `landings`: each visible in the diff.
- `keyring_summary` is the one site that prints `restoring`, and the
  CLI's `auth status` text prints the restore sentence for it: the
  message quotes the sentence.
- The client harness's two waits read the keyring word; the message
  says so.
- The `c144_` restore tests, the six `c148_` tests, `rejected_access_token_*`,
  `exit_waits_for_a_refresh_and_installs_the_token`,
  `the_idle_verdict_holds_while_a_login_is_pending` and the `idle.rs`
  tests pass, unchanged but for the parameter rename of Do 2, named in
  the message.
- The wire fixtures under `acquisition-protocol/tests/fixtures/wire`
  are unchanged.
- The commits of Do 5 to 7 follow the registry commit that enters the
  owner's words of Do 1; the message of the first of them cites it.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- A `restoring` field on the protocol's documents (the word in the
  field is r2's `saving` shape; a field is a protocol change, ruled
  first, if a frontend ever needs the state as a bool).
- A refusal while restoring (Do 5 says why a wait); a client-side retry
  on it (none is needed).
- A downgrade path for an older daemon reading the new entry (C24 and
  `acq auth` are the path; the ruling says so).
- `auth check` without a POST when the access token is valid (the user
  asks for the POST).
- The save cut at the exit's bound (the time seam, 4-0, `TIME-SEAM.md`
  section 4; the restore cut at the bound writes nothing and is safe
  by construction).
- A restore that merges with sessions logins landed meanwhile (no
  request that could land one runs before the restore lands).
- Keyring 4 or which store holds the token; the lock merge whole; the
  typed `JobKind`.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `c135`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands in the background; `tools/gate.sh`, and `cargo build
--workspace` before any daemon and after any test run (C84, the
artifact mismatch). The owner's words of Do 1 are entered by him, not
the worker; Do 2 to 4 are built and committed before them, Do 5 to 7
after.

## Report

`TEMPLATE.md`.
