# r2 — The credential I/O leaves the shared lock: the build

Readiness step 2 (`roadmap/README.md`, row r), the build half, after
the design `r2-design.md` and the owner's ruling C148 (no credential
or index I/O under the shared lock; `hello` answered under no lock
such I/O can hold), entered 2026-10-26 with an Unbuilt line. Closes
#81. #55 (C135) and #92 (the start's read) follow in their own round.
About two days.

## Goal

A daemon whose keyring and account-index I/O runs under a credentials
lock on a blocking thread, never under `shared`; whose `hello`,
status documents and bearer reads wait on no credential I/O; whose
every credential result still reaches the caller and the words the
user reads (C144, no success the daemon did not earn); and whose exit
drain still waits for a save in flight. The `c144_` tests, the C87
family and the exit's refresh pins stay green; three new tests pin
the three schedules; C148's Unbuilt line and *Pinned:* are replaced
by the tests' names.

## Read

- `r2-design.md` whole: section 2 (the trace table), 3 (the landing,
  the lock order, who waits for what, the second login, the two test
  seams), 4 (`hello`), 8 (the tests), 9 (the brief's corrections).
- C148 (`decisions/daemon.md`); C144 (every credential result reaches
  the caller; the index written under one lock, which is now the
  credentials lock); C133 (one exit, a bounded drain); C134 (a panic
  aborts; the hook takes no lock); C86 (the handshake deadline); C87
  (`decisions/network.md`, a dead bearer: the 401 clears under
  `shared` alone and is untouched); C50 (a login whose index write
  fails fails whole); C137 (no wall-clock bound; a deadline pinned by
  forcing its expiry).
- `crates/acquisition-daemon/src/daemon.rs`: the lock order
  (`:833`), `Shared` (`:726`), `Sessions` (`:526`), `Staging`
  (`:485`), `CredentialStore` (`:910`), `refresh_landed_hook`
  (`:901`), `note_error` (`:968`, takes `shared`).
- `daemon/refresh.rs`: `RefreshOwnerGuard` (`:17`) and its `Drop`
  (`:36`), `install_tokens_locked` (`:71`),
  `persist_credentials_locked` (`:126`), `valid_access_token_once`
  (`:283`), `finish_refresh` (`:435`), `wait_for_refresh` (`:531`).
- `daemon/login.rs`: `begin_auth_flow` (`:98`), `complete_login`
  (`:233`), `abort_login` (`:337`), `logout` (`:418`), `known_account`
  (`:509`), `settle` (`:559`), `clear_account_entries` (`:641`).
- `daemon/kinds.rs`: `SessionBearer::token` (`:98`), `index_apply`
  (`:135`), `with_index` (`:157`) and its caller (`:381`), the C87
  retry outside the permit (`:662`, `:776`).
- `daemon/server.rs`: `handle_conn` (`:85`, the hold before `hello`;
  `:231`, the decrement), `handle_request` (`:235`; `daemon_status`
  and the dashboard read `connections`), `begin_exit` (`:468`),
  `drain` (`:493`), `is_idle` (`:589`), `InFlight::of` (`:653`).
- `daemon/queue.rs:438` (`last_activity` on submit);
  `daemon/dispatch.rs:119` (`run_active`, holds `Arc<Self>`);
  `crates/acquisition-store/src/index.rs:175` (`load`), `:442`
  (`save`, the atomic synced write).
- The tests: `daemon/tests/harness.rs` (`RecordingCredentialStore`
  `:91`, its `on_clear` `:107`, `test_daemon_with` `:431`,
  `scripted_server` `:293`); `tests/refresh.rs` (`token_daemon_with_store`
  `:529`, the `c144_` tests from `:561`); `tests/login.rs`
  (`login_daemon` `:23`, `keyring_daemon` `:751`, `logout_answer`
  `:778`, `staged_login` `:952`, the `c144_` tests from `:659`);
  `tests/bearer.rs` (`rejected_access_token_*` `:486`, `:499`;
  `a_cloudflare_401_on_the_get_keeps_the_access_token` `:620`);
  `tests/exit.rs` (`exit_waits_for_a_refresh_and_installs_the_token`
  `:120`, `c133_a_stop_between_the_profile_and_the_login_waits_for_the_login`
  `:570`, `a_login_whose_profile_job_waits_is_not_held_by_the_drain`
  `:624`, the in-process `serve` over a Unix socket `:1097-1109`);
  `tests/stress.rs` (`wait_for_refresh_waiters` `:9`); the module
  list in `daemon/tests.rs`.
- `REVIEW-CHECKLIST.md`, "Network" ("An owner that leaves without
  publishing") and "Protocol, client, daemon lifecycle, the world".

## Input

The three schedules, each staged with a credential store whose `save`
and `clear` block until the test releases them, and never a clock:

1. A refresh is saving; a connection sends `hello`. Today it waits on
   `shared`; after, it is answered.
2. A refresh is saving; a job past admission holds a gate permit and
   reads its bearer. Today it waits; after, it reads the token the
   refresh installed.
3. A login is landing; `daemon status` is asked and `auth_start` is
   called. Today both wait; after, the status answers with the login
   pending and the second start is refused.

Fixed before the work: each of the three, run against `610286d1`,
fails as section 1 of the design describes (the `hello` and the
status wait; the bearer read waits), recorded in the first commit's
message. And the measurement's "before": the `hello` round trip over
the in-process socket while a save blocks, which is the test's own
bound or the client's `HELLO_WITHIN` (5 s) from a client.

## Do

1. **The tests first**, `daemon/tests/credentials.rs`, named in the
   design's section 8 with the ruled id (`c148_hello_is_answered_while_a_refresh_saves`,
   `c148_a_permit_holder_reads_the_installed_token_while_the_save_blocks`,
   `c148_status_answers_and_a_second_login_is_refused_while_a_login_lands`).
   The blocking store is the harness's recorder with a gate (an
   `on_save` hook beside `on_clear`, or a sibling store), released by
   the test from its own task. The first test connects over a Unix
   socket as `exit.rs:1097` does. Commit them failing, each failure
   quoted.
2. **The credentials lock and the landing.** `Daemon` gains
   `credentials`, a std mutex taken only on a blocking thread; the
   lock-order comment at `daemon.rs:833` gains it at the head
   (`credentials` → `store` → `shared` → …). Each of the four paths
   becomes one landing run under `tokio::task::spawn_blocking`, in
   the three holds the design's section 3 draws: decide and install
   under `shared`; the I/O under `credentials` alone
   (`persist_credentials_locked` renamed `persist_credentials`, body
   unchanged); publish under `shared`; warnings logged after. The
   refresh publishes its watch at the install and clears its flight
   at the publish; the owner awaits its landing whole;
   `RefreshOwnerGuard::finish` hands the sender to the landing. The
   daemon reaches the closure as `Arc<Self>` (about a dozen
   signatures) or by its inputs cloned in; say which in the message
   and why.
3. **The login's landing mark.** `Sessions` gains the landing flow's
   generation; `begin_auth_flow` refuses while it is set, kind
   `refused`, "a login is being recorded; try again in a moment";
   `InFlight::of` counts it as it counts a staged login; the mark is
   cleared at the publish. A browser-waiting login is still
   superseded.
4. **`hello` ahead of the lock.** `connections` and `last_activity`
   leave `Shared` for two atomics on `Daemon` (a count; nanoseconds
   since `started` on the daemon's clock); every reader and writer
   named in the design's section 4 uses them; `is_idle` reads them
   beside its hold.
5. **The two seams.** `refresh_landed_hook` runs after the landing's
   `credentials` release; `c144_a_login_cannot_land_between_a_forgets_clear_and_its_mark`
   probes `credentials` instead of `shared` and completes the login
   after the forget returns, asserting as now.
6. **The measurement**, alone on the machine: the `hello` round trip
   over the in-process socket while a save blocks, before (step 1's
   tree) and after, as numbers in the message.
7. **The registry.** C148's Unbuilt line goes; its *Pinned:* names
   `acquisition-daemon/src/daemon/tests/credentials.rs` and the three
   `c148_` tests; the module docs of `refresh.rs`, `login.rs` and
   `server.rs` name C148 where they describe the landing.
8. The gate, the commit, the review, the close: `roadmap/README.md`,
   "A step". The close lists the second login's refusal words as the
   step's agent-facing decision.

## Done when

- The three `c148_` tests exist, failed on `610286d1` as the first
  commit's message quotes, and pass on the step's tree.
- No call to the credential store or to `Index::load`, `index_apply`
  or `with_index` runs under `shared`: `grep -n 'credential_store\.\|Index::load\|index_apply' crates/acquisition-daemon/src/daemon/*.rs`
  names only sites inside a landing or `restore_sessions`, and the
  message says so.
- `handle_conn` takes no lock before `hello` is read: visible in the
  diff.
- The `c144_` families in `refresh.rs` and `login.rs`,
  `rejected_access_token_*`, `a_cloudflare_401_on_the_get_keeps_the_access_token`,
  `exit_waits_for_a_refresh_and_installs_the_token`,
  `c133_a_stop_between_the_profile_and_the_login_waits_for_the_login`,
  `a_login_whose_profile_job_waits_is_not_held_by_the_drain` and
  `a_superseded_login_cannot_complete_and_leaves_no_zombie` pass
  unchanged, except the two seam changes of Do 5, named in the
  message.
- The lock-order comment names `credentials` and its place.
- The measurement's two numbers are in the message.
- C148's entry has no Unbuilt line and names the three tests.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- The start's keyring read (#92, the C135 round); C135 itself (#55).
- The save cut at the exit's bound, pinned under the time seam (4-0)
  unless the owner asks for it with paused time first.
- The lock merge whole, the typed `JobKind`, `frame.rs` shared, the
  quote's simulation under the limiter lock, the outranked check
  (under "Open" and `STATE-2026-10-26.md`).
- A per-account credentials lock (the design's section 3: one lock
  until a run shows the wait).

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main` (the step touches the daemon's one lock, so no parallel
worker); scratch under your session's scratchpad in a subdirectory
named `r2`; the measurement in Do 6 runs with no other build, gate or
measurement on the machine; commit on `main`, never push; the
`Co-Authored-By` line for your model last.

## Report

`TEMPLATE.md`.
