# t5-r5a — the refresh and the bearer as scenarios (#141, round 5a)

The first of round 5's three parts (`t5-design.md` §3 rows 4 and 5,
§4 round 5 as split at this brief's commit: 5a the refresh and bearer
families, 5b the door-able tests of the other files, 5c the sweep;
ruled 2026-10-31, C158). Cut twice: the first draft (`e818ed51`) was
reviewed cold (`roadmap/reviews/t5-r5a-brief-e818ed51.md`, the prompt
beside); this cut answers its six findings, each named at the foot,
"Taken from the review". Written from two read-only inventories at `9736bdcf` (`refresh.rs` and the production refresh
path; `bearer.rs`, the two clocks, `ManualClock`'s users by file, the
real-time waits, the exempt-route list), every line printed, and a
read of the fixtures each recipe names; rounds 1 to 4 closed
(`LEDGER.md`). Round 0's baseline, never run before, was dispatched
at this brief's writing (Input).

## Goal

When this round is done, every refresh and bearer scenario whose
property crosses the frontend boundary runs through the door: what
the token endpoint answers is one table of rows (a script of token
answers, the checks a frontend sends, what each is answered, the
rails' marks, the journal, the grants on the wire, the saves); the
keyring-and-index landing (C144) is read through the scenario's own
credential store, the index on disk and `AuthStatus`'s keyring word;
a saved entry's lifetime is read through a restored second lifetime
and a job that sends no refresh before expiry; the 401 family and the
laptop sleep move the clock by the scenario's hand where nothing
parks. The door gains a restore (the daemon's own start path), a
closure form of `Own`, and the wire's record (each request's method,
bearer and body). The tests that need a function, a harness property or the
landing's own hook say so and stay component tests. No production
file changes; no word a user meets changes.

## Read

- `roadmap/briefs/t5-design.md` §2 (the door), §3 rows 4 and 5
  (corrected at this brief's commit: refresh.rs moves a clock in four
  tests of 24; the laptop sleep has no production detection, the
  wall-face expiry is the whole mechanism), §4 rounds 0 and 5 (the
  dated lines); `t5-r4.md` Do 1 (`quiet`, the browser, `submit`,
  `held`, `sent`, the account server's code grant); `t5-r3.md` Do 1
  (the wire table's row shape, which the token table follows in
  spirit, not in code).
- `decisions/network.md`: C24 (a rejected grant is dead until login
  or logout), C87 (a 401'd bearer is dead; no send carries it after
  the rejection landed), C129, C130, C131 (a token 2xx with unusable
  pacing still installs); `decisions/daemon.md`: C135 (the access
  token kept beside the refresh token, a lifetime refreshes only at
  expiry), C144 (no success the daemon did not earn), C148, C158;
  `AGENTS.md`: C12 (the index on disk is the store's read API), C153,
  C154.
- The production refresh path, `crates/acquisition-daemon/src/daemon/
  refresh.rs`: `valid_access_token` (355) and `_once` (373: the wall
  read at 391, the 5 s margin at 394–401, `force_refresh`; the
  dead-grant check 414–419 before any flight; owner and waiter
  421–440); the mark at 470–489 (`mark_refresh_failed`, no auth
  notification); `finish_refresh` (534–660: a success lands on the
  counted blocking thread, 46–51; `keyring` "saving" at 590, the
  flight cleared 596, the rename 598–600, the publish 610, the persist
  618–625, the keyring word and its notification 636–643, the
  test-only `refresh_landed` hook 646); `persist_credentials`
  (130–231: index first, the save, the old names cleared, the words
  "keyring save skipped", "keyring save failed", "could not be
  cleared"); `reject_access_token` (243–260). `auth.rs`: `refresh`
  (227), the answer's classification (264–370: a rejected grant 157,
  a 3xx's redacted words, `MAX_ACCESS_TOKEN_LIFETIME_SECS` 194).
  `server.rs`: `auth_status` (62–88: `keyring_summary` "restoring"
  before the restore lands, 53–60; `access_expires_in_seconds` against
  `choke.wall()` 79–83, `None` without an expiry), `AuthCheck`
  (360–377, `force_refresh: true`), `AuthLogout` (384–391, `Ack {
  keyring }` or the refusal). `start.rs`: `begin_restore` (599, a
  counted landing; `land_restore` 613–640 reads the credential store
  and `store_dir` on `choke.wall()`); the builder's `restore_landed`
  (`daemon.rs:1223`; `start.rs:496` builds with `false`). `kinds.rs`:
  the 401 under the permit (`choke.rs` 692–698 and 1028–1035, Cloudflare
  excluded), `SessionBearer::rejected` (124); `whoami` sleeps 300 ms
  of real time (621): no scenario submits it. `routes.rs:154`:
  `NO_PROBE_ROUTES` = `profile`, `league`: a `profile` job sends no
  HEAD. `ratelimit/limiter.rs`: `PROBE_COOLDOWN` 60 s (68), the degraded
  route's `until` (403–405, read at 379), the C131 hold 60 s (36,
  548–566).
- `crates/acquisition-daemon/src/daemon/tests/door.rs` as round 4 left
  it: `Server::Own(String)` (77, its listener bound before `run`, the
  build arm 488), `Fixture::Own` carries nothing; `restore_landed`
  (351), `restore_jobs` (361), `store_dir` (369), `world` (381);
  `laptop_sleep` (836, `dead_code`), `advance` (830); `journal_text`
  (824, the header included); the daemon log private (265) and off
  the boundary (C158; `dispatch.rs:668–675` keeps the log's redaction
  a component case); `quiet`,
  `settle`, `held`, `sent`, `submit`, `await_auth`, `await_terminal`;
  `Token::Answering` (96, `dead_code`, no user).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`:
  `MANUAL_WALL_ORIGIN` (22); `ParkingClock` (82–249: `advance` moves
  both faces and wakes the due, `laptop_sleep` 163 moves the wall
  only and wakes nothing); `ScriptedResponse` (419: method, path,
  status, headers, body) and `scripted_server` (549: records methods
  only, 562, from `mockggg::read_request`, which carries the headers
  and the body); `token_server_answering` (1225: one connection per
  status, a fixed 200 body or `invalid_grant`; callers in dispatch,
  queue and rails stay); `test_daemon_scenario_at` (callers: bearer 4,
  log 3, credentials 1); `logged_in_sessions` (1274);
  `RecordingCredentialStore` (`failing`, `fail_clear`, `fail_save`,
  `saves`, `cleared`, `entries`); `freeze` (unix); `assert_wire_
  contract` (1033–1059, the exempt list); `wait_terminal` (786).
- `crates/acquisition-daemon/src/daemon/tests/refresh.rs`, whole
  (1342 lines; 22 `#[tokio::test]`, two `#[test]`; 14 `shared.lock()`
  occurrences with the four split ones, 15 `.auth` lines, 10
  `ManualClock` mentions, `valid_access_token` called 32 times, the
  private `logout` 8, `Arc::get_mut` 4, no sleep, no ceiling, five
  clock moves). Its helpers: `token_answer` (269, the scripted token
  answer; `TOKEN_HEADERS` 287), `token_daemon` (291),
  `session_refresh_token` (347), `token_daemon_with_store` (541),
  `indexed_test_user` (677: `store_dir` and the uuid `u-1` set
  privately, `record_login`), `journaled_token_posts`, `restored_
  daemon` (1127: `start::restore_sessions` on `clock.wall()`, the
  sessions written privately at 1168), `renamed_answer`. The tests,
  T1 to T24 by line: 14, 83, 127, 197, 316, 365, 424, 457, 494, 573,
  607, 643, 699, 763, 838, 892, 943, 1015, 1057, 1097, 1197, 1238,
  1285, 1319; classified in Do 2 to Do 4.
- `crates/acquisition-daemon/src/daemon/tests/bearer.rs`, whole (881
  lines; nine `#[tokio::test]`, all on `ManualClock` through
  `test_daemon_scenario_at`, entering by the internal `submit` and
  waiting through `wait_terminal` (8 calls), the session seeded by
  `install_tokens_locked` under `shared.lock()` at 103–115, 327–339,
  553–567, 690–702): `bearer_aware_server` (17–64: 401 to the stale
  bearer, on a HEAD only when `probe_checks_bearer`; the token POST
  rotates; records nothing), `bearer_recording_server` (203–287: one
  policy per path, records (method, bearer, after-a-401), holds every
  stale-bearer GET until `release`), the inline Cloudflare server
  (624–672: records (method, bearer); the first GET a 401 with
  `cf-mitigated: challenge`), `restored_scenario_daemon` (735–752),
  `frozen_index` (789–792, 817). The tests: 172 and 187 (family A, a
  two-row table over `probe_checks_bearer`; `advance(PROBE_COOLDOWN +
  1 s)` at 124 crosses the degraded route's cooldown in the probe row
  and nothing in the GET row), 486 and 499 (family B: the routes
  learned by a private `choke.head` ×3 at 350–362; a 3 s ceiling at
  387 over a yield poll; 200 bare `yield_now`s at 394–396 standing for
  "nothing else can run"), 534 (family C, the laptop sleep:
  `laptop_sleep(1800 s)` then `advance(2000 s)`, the journal's header
  read, every send's ts in [3800, 3860) s), 620 (family D,
  Cloudflare's 401: `halted()` and `reset_rails()` private at
  709–710), 849, 862 and 878 (family E, a three-row table over
  (`probe_checks_bearer`, `frozen_index`): two lifetimes on one
  manual clock and one server task, the index and the keyring entry
  read between them).
- `crates/acquisition-daemon/src/daemon/tests/exit.rs:681–720`: the
  one `Own` user (a std listener bound before `run`, `from_std`
  inside), which takes the closure form.
- `crates/acquisition-store/src/index.rs`: `Index` (`load`, `get`,
  `entries`, `persisted`, `record_login`): the store's read API.

## Input

Fixed before the round: the two inventories' classifications and
counts (Read, Do 2 to Do 5); the door as round 4 left it. The gate's
test stage at `9736bdcf`: 23 to 24 s on the orchestrating session's
machine (round 4's close). The last push's CI on `9736bdcf`: `Gate
(Linux)` 41264833704 and `Gate (Windows)` 41264833635, both green.
Round 0's baseline (`t5-design.md` §4 round 0): the weekly mutants
workflow dispatched on 2026-10-31 on the tag `t5-round0-baseline`
(`a8a719d6`, round 1's parent), run 41265798977, four shards, its
`mutants.out` artifacts the baseline round 6 compares against; in
progress at this brief's commit, read at its close.

## Do

1. **The door's additions**, each with its rule in its doc:
   - `DoorBuilder::restore()`: the daemon restores its sessions as a
     start does: built with `restore_landed(false)`, then
     `begin_restore(None)` (`start.rs:599`, a counted landing the hook
     holds the clock under) before the scenario runs, and `run` awaits
     the landing (`await_auth` on `keyring != "restoring"`) so the
     scenario meets a restored daemon, as a frontend does. The restore
     reads the scenario's credential store and `.store_dir`, on the
     door's clock's wall. The private `start::restore_sessions` calls
     and `shared.lock().auth = sessions` go from the scenarios.
   - `Server::Own` takes a builder closure run inside the door's
     runtime before the daemon is built, answering the server's base
     URL (`Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = String>>>>`
     or the worker's equivalent), so a scenario's server binds and
     spawns as `bearer_aware_server` does today, its record and
     barriers the scenario's own `Arc`s and oneshots captured before;
     `exit.rs:681`'s std listener takes the closure form; the doc's
     "bound before the runtime exists" goes.
   - The scripted server records each request's method, bearer and
     body (`read_request` carries all three), and the door answers
     the record in order (the worker names the call): the grant a
     token POST carried (`refresh_token=rt-old`, `rt-rotated`) and the
     bearer a GET carried (`at-kept`, `at-new`) are read on the wire,
     where the session's private tokens were read before (refresh.rs
     57–67, 241–253, 377–381, 438–442, 470–473, 517–521, 530–533,
     1215–1218, 1333). The rule: a private token value is a boundary
     property only through a send that carries it; a row that asserts
     a token "stands" after a failure makes the next send happen (a
     further scripted answer and a check) and reads its body; a value
     no send will ever carry again (T1's grant after the dead-grant
     mark, C24: no refresh until login or logout) has no boundary
     property, and its old assertion retires held by the row's "no
     second POST" and "disabled" words. The wire oracle's comparison
     is unchanged.
   - `Token::Answering` retires: its two would-be users (T1, T2) are
     rows of the token table on `Script`, since a scripted answer
     plays every body the table needs and the fixed body of
     `token_server_answering` cannot (headerless, malformed, renamed,
     out of range). `token_server_answering` keeps its callers in
     dispatch, queue and rails (5b). `token_answer` and
     `TOKEN_HEADERS` move to the harness.
   - `laptop_sleep` (the door's and `ParkingClock`'s) loses its
     `dead_code`: family C and T6/T8 call it. A clock move in a
     scenario is written against the deadline it crosses, named in a
     comment: the expiry on the wall (nothing parks on it; the hand
     moves it), the degraded route's cooldown (`PROBE_COOLDOWN`,
     nothing parks), the C131 hold (parked; the hook crosses it in 60
     parks of `HALT_POLL`, `wait_ms` 60000 exactly, so T6's move stays
     a `laptop_sleep`: an `advance` would expire the hold too).
   - A session seeded through `.sessions(..)` has its expiry on the
     clock's wall (`MANUAL_WALL_ORIGIN` + the lifetime), as
     `install_tokens_locked` wrote it; `bearer.rs`'s four seeds and
     `refresh.rs`'s `logged_in` go that way.

2. **The token table** in `refresh.rs`: one `#[test]` over rows, each
   a script of token answers (`token_answer` with its status, headers
   and body), the rails the row opens with (`tripwire_config` for T2,
   the default else), a sequence of steps (`Check`: `AuthCheck` and
   what it answers, `Auth { username }` or `Error { kind, carries,
   forbids }`; `Reset`: `ResetTripwire`; `Logout`: `AuthLogout`;
   `LaptopSleep(d)`), and what the row asserts after: `refresh_failed`
   words or `None` and `halted` words or `None` through
   `DaemonStatus.rails`; the journal's projection `[method, path,
   status, ok, wait_ms]` with the error's words where the row names
   them; the bodies on the wire (the grants, in order); the saves'
   count in the scenario's store; `forbids`, fragments each with its
   surfaces (`journal_text`, `Dashboard.errors`, the answer's
   words). The rows, written from the rulings and the old
   assertions, never from running the door: T1 (400: `NotLoggedIn`
   carrying "400 Bad Request"; the mark carries "HTTP 400"; a second
   check carries "disabled" and the journal still one POST, its body's
   grant `rt-old`; `Logout`, then the mark `None`; the old `rt-old`
   read at 57–67 retires by Do 1's rule); T2 (503 then 200 on the tripwire: `Upstream` carrying
   "503", no mark, `halted` carrying "503", `Reset`, `Auth {
   test-user }`, two POSTs); T3 (a 307 with a `Location` and a body
   echoing a sentinel: `Upstream` carrying "307" and the redacted
   target, `forbids` the sentinel over the answer, `journal_text` and
   `Dashboard.errors`, one POST; its log read, 178–179, stays a
   component case beside the table, as `dispatch.rs:668` keeps the
   log's redaction, its doc saying so); T4 (a Cloudflare 403
   challenge: `Upstream` carrying the block's words and
   "reset-tripwire", `forbids` "acq auth" and the challenge page over
   the answer, no mark, `halted` set, a second check `Upstream`; then
   `Reset` and a third check answered by a scripted 2xx, whose POST's
   body carries the grant `rt-old` (the session stood through the
   block, 241–253, read on the wire); T6 (a headerless 2xx then a
   2xx: `Auth`, saves `rt-rotated`, journal[0] `ok: false` with
   `x-rate-limit-policy` in its error, `LaptopSleep(3600)`, a second
   check, journal[1] `ok: true` with `wait_ms` 60000, the second body's
   grant `rt-rotated`); T7 (a 200 HTML body then a 2xx: `Upstream`, no
   save, no mark, journal[0] `ok: false`; a second check whose POST
   carries `rt-old` (438–442 on the wire) and lands, the save
   `rt-rotated`); T8 (malformed headers then a 2xx: as T6
   with `wait_ms` 0); T9 (`expires_in` `u64::MAX` then a 2xx:
   `Upstream` carrying "lifetime" and `forbids` the token text, no
   save, no mark; the second check `Auth`, the grant `rt-old` then the
   save `rt-rotated`; "not poisoned" held by the second check
   answering). The runner reads nothing through `door.daemon()`.

3. **The landing scenarios** (C144), each a `#[test]` through the
   door on `Script` with the row's token answers, `.store_dir` with
   the index seeded as `indexed_test_user` seeds it (`record_login(
   "test-user", "u-1")`, the session's uuid through `.sessions`), the
   scenario's `RecordingCredentialStore` with its knob, `AuthCheck`
   as the entry (force is what the old private call did at 902 and
   1024; the rest found a refresh due, no access token seeded):
   T10 (a rename clears the old entry: `Auth { NewName#2 }`, the
   store's entries and `cleared`), T11 (a failed save: `await_auth`
   on the session's `keyring` carrying "unavailable: keychain locked",
   `Dashboard.errors` carrying "keyring save failed", the store's
   entries empty and `cleared` equal to `["test-user"]`, 616–620),
   T12 (a rename
   whose old clear fails: `keyring` "ok" under the new name, the
   errors' words), T14 (one rename to `NewName#2` whose clear fails:
   the index's reach through `get` and `persisted`, the store's two
   entries; then the knob cleared and `AuthLogout { NewName#2 }`
   answering `Ack { keyring: Cleared }`, the store empty, the index
   released) and T18 (two renames, `B#2` then `C#3`, two checks: the
   store's three entries, every name in the index's reach; the knob
   cleared and `AuthLogout { C#3 }` answering `Cleared`, the store
   empty, the index released, 1015–1050), T16 (unix, `freeze` on the
   scenario's index: `keyring` carrying "unavailable: not saved", the
   errors carrying "keyring save skipped", `AuthLogout` answering
   `Cleared`, the store empty) and T17 (unix, the rename to `B#2`
   under the frozen index, then the index released and the clear
   failing: `AuthLogout { B#2 }` answers `Error { Internal }` whose
   printed `--account` word parses as today, the index holding `B#2`
   and the session gone; the knob cleared, the retry `AuthLogout`
   answers `Cleared`, the store empty, the index released, 943–1010),
   T15 (unix,
   an unindexed rename: the index still `test-user`, the logout
   emptying the store, `persisted()` empty, then a second door on the
   same `store_dir` and store with `.restore()`: `AuthStatus.accounts`
   empty). T13 stays a component test (the landing's own hook,
   `refresh_landed`, on the blocking thread: the staging point has no
   surface), its doc saying so; T19, T20 (`restore_sessions` as a
   function, `#[test]`) and T5 (a harness property, the bare
   constructor's default store, which `Door::open` takes as an
   argument) likewise.

4. **The save family** (C135): T21 and T24 through `.restore()` on a
   store seeded with the JSON entry (`at-kept`, expiry the wall origin
   + 3600, `rt-old`), `Script` answering `profile` GETs (no HEAD:
   `NO_PROBE_ROUTES`) and the one POST, each test's own script in its
   jobs' order (T21: `[GET, POST, GET]`; T24: `[GET, GET, POST, GET]`,
   the scripted server asserting each response's method in turn,
   harness.rs 557–561); the entry is `submit("profile", ..)` and
   `await_terminal`, the non-forcing path a frontend has (`AuthCheck`
   always forces): before the move the journal's methods are `[GET]`,
   the GET's bearer on the wire `at-kept` (the restored token,
   1215–1218) and `access_expires_in_seconds` 3600; T21
   `advance(3600)`, a second job, `[GET, POST, GET]`, the last GET's
   bearer `at-new`; T24 `advance(3594)` then a job, still `[GET, GET]`
   with `at-kept` (6 s left, 1333), then `advance(1)` and a job, the
   POST and a GET with `at-new` (5 s left: the margin, refresh.rs
   394). T22: `.restore()` on the bare `rt-old` entry: `access_expires_
   in_seconds` `None`, `AuthCheck`, one POST, the store's entry
   rewritten in the JSON form (`rt-rotated`, `at-new`, the expiry).
   T23: `AuthCheck`, the saved entry's expiry equal to the clock's
   wall plus `access_expires_in_seconds`, its tokens.

5. **`bearer.rs` through the door**, every server in `Own`'s closure
   form, the session seeded through `.sessions` with the stale token
   expiring at the wall + 3600: family A as two scenarios or a two-row
   local table (`submit` the kinds the tests submit, `await_terminal`
   ×2, `advance(PROBE_COOLDOWN + 1 s)` between them in both rows as
   today, the journal's methods `[HEAD, POST, HEAD, GET]` and `[HEAD,
   GET, POST, GET]`, exactly one 401, the first job's error carrying
   "401", "rejected" and "refreshes", the second Done); family C
   (`laptop_sleep(1800 s)`, `advance(2000 s)`, a job, no 401, a POST
   on the wire, the header through `journal_text` (`clock` "manual",
   the ts origin), every send's ts in [3800, 3860) s as today); family
   D (the inline server in the closure form recording bearers:
   `DaemonStatus.rails.halted` and `ResetTripwire` for the private
   calls, the journal without a POST, the record's GETs both `at-old`,
   the second job Done); family E as three rows of a local table: two
   doors in sequence on one `store_dir` and one credential store, the
   aware server built in each door's closure, the first door's journal
   with exactly one 401 and the store's entry keeping `rt-kept` with
   no access token, the second door `.restore()` with its journal's
   methods `[POST, HEAD, GET]` and no 401; the `frozen_index` row
   rewriting the index's bytes between the doors as today. Family B:
   through the door only where its warm-up (three routes learned by a
   private `choke.head`) can be the jobs' own sends against the
   closure-form recording server (a job per route answered before the
   held phase), then the arrival barrier, `quiet` for the 200 yields,
   the record and the journal; else the two stay component tests
   saying so (the learned routes have no protocol path, C127), their
   3 s ceiling `HANG`. The worker decides on the tree and names the
   decision in the report.

6. **Retirements**, each with its C154 line: in `refresh.rs`,
   `token_daemon`, `token_daemon_with_store`, `indexed_test_user`,
   `restored_daemon`, `journaled_token_posts`, `session_refresh_token`,
   `renamed_answer` if no caller remains, the private `logout` calls,
   the `Arc::get_mut` patches, the 22 tests by name (each held by its
   row or scenario of the same name, or kept as a component test with
   its doc line); in `bearer.rs`, `restored_scenario_daemon`, the
   four `install_tokens_locked` seeds, the 8 `wait_terminal` calls,
   the 200-yield loops and the 3 s ceiling at 387 (held by `quiet`
   and `HANG`, or kept with family B); `test_daemon_scenario_at`'s
   bearer callers (its log and credentials callers stay, named, 5c);
   in `door.rs`, `Token::Answering` and the `dead_code` allowances on
   `laptop_sleep`. A private assertion with no boundary read is
   "held by X" or "kept", never dropped (C154; round 4's fix round).

7. `tools/gate.sh` once per commit (three at least: the door's
   additions with the token table; the landing and save families;
   `bearer.rs`); the test stage's seconds and the two files' wall time
   before and after in the report.

## Done when

- `grep -n "tokio::test" crates/acquisition-daemon/src/daemon/tests/refresh.rs`
  finds only T5 (316), T13 (699) and T3's log case at `9736bdcf`'s
  names, quoted;
  the token table and the scenarios are `#[test]`s, each row and
  scenario listed in the commit with the test it replaced.
- `grep -n "valid_access_token(\|\.logout(\|Arc::get_mut\|shared.lock()\|^\s*\.shared$\|choke.rails()\|restore_sessions" crates/acquisition-daemon/src/daemon/tests/refresh.rs`
  finds nothing outside T5, T13, T19 and T20, quoted.
- `grep -n "tokio::test\|install_tokens_locked\|test_daemon_scenario_at\|yield_now\|from_secs(3)\|wait_terminal\|Arc::get_mut\|restore_sessions" crates/acquisition-daemon/src/daemon/tests/bearer.rs`
  finds nothing, or only family B's two tests if kept component, quoted.
- `grep -n "Answering\|dead_code" crates/acquisition-daemon/src/daemon/tests/door.rs`
  finds no `Token::Answering` and no `dead_code` on `laptop_sleep`;
  `grep -n "fn restore\b" door.rs` finds one, and the scripted
  record's accessor one (named in the commit); `Server::Own`'s variant
  holds a closure (quoted) and `exit.rs`'s user takes it; no door
  call reads the daemon log.
- `grep -n "token_server_answering(" crates/acquisition-daemon/src/daemon/tests/*.rs`
  finds its callers in dispatch, queue and rails and none in
  refresh.rs; `token_answer` is in the harness.
- Every clock move in a scenario names the deadline it crosses in a
  comment (grep `advance(\|laptop_sleep(` in refresh.rs and bearer.rs,
  each line quoted with its comment).
- `git diff --stat <this brief's last cut> <head> -- crates/` lists
  files under `daemon/tests/` only; every golden, `CLI-REFERENCE.md`
  and `MCP-REFERENCE.md` byte-identical.
- `tools/gate.sh` exits 0 on each commit's tree; the test stage's
  seconds and the two files' wall time before and after in the
  report, asserted by nothing.

## Not in scope

- 5b: the door-able tests of the other files (the inventory's 34:
  `queue.rs`'s 14 refusals through `handle_request` with their
  50-yield loops as `settle`, `rails.rs`'s 11 with its nine 100 ms
  sleeps, `exit.rs`'s three on `world`, `parents.rs`'s two, `dispatch.
  rs`'s two, `quote.rs`'s one, `stress.rs`'s one) and the class-(a)
  private reads of round 4's narrowing.
- 5c: `ManualClock`'s retirement as a harness-level swap to a
  hand-driven parking clock (the inventory: 84 component tests, four
  relying on the manual clock's self-advancing sleep: dispatch 371
  and 472, rails 26, stress 32), the 14 fixed sleeps, the 19 literal
  ceilings and `ANSWERED_WITHIN`, the polls (#131's remaining rows),
  `test_daemon_scenario_at`'s and `install_tokens_locked`'s last
  callers (log, credentials), the exempt-route check and its registry
  line (the design's §5 ruling of 2026-10-31 is not yet in
  `decisions/daemon.md`: entered with its pin, in 5c).
- The dead-grant mark's missing auth notification (refresh.rs
  482–488; a check answers after it, so no scenario waits): raised.
- `whoami`'s 300 ms real sleep (kinds.rs:621): no scenario submits it.
- Any production change; any word a user meets.

## Rules

`TEMPLATE.md`. What differs: this round runs in the main tree on
`main`, in a checkout other sessions may share; never push; `acq` and
`acqd` only in a mock world, under the harness; kill only this
session's pids; an experiment is undone from a scratch copy; nothing
tracked is edited while the gate runs. The owner has granted autonomy
and builds for 2026-10-31. The attribution line names the model the
worker runs on.

## Report

`TEMPLATE.md`, and: the row and scenario count by file and the tests
retired; the test stage's seconds beside 24 and the two files' wall
time before and after; family B's decision; every agent-facing design
decision the brief left open (the token table's `Row` and step shapes,
`restore()`'s wait, `Own`'s closure type, the wire record's accessor
and shape, the local tables' shapes in bearer.rs) for the owner's
review.

## Raised for review

- The design's §4 round 5 was one round; the inventory counts it as
  three (this brief's split, dated in the design under the grant of
  autonomy, for the owner's re-read).
- The design's §2.4 premise ("a test on a yield loop cannot run on a
  parking clock") is disproved by four tests already on a hand-driven
  parking clock beside yield polls (dispatch 132, exit 600,
  credentials 322 and 774); the blocker for a mechanical swap is a
  production sleep on the clock, four tests in all. 5c is cheaper
  than the design feared.
- The laptop sleep has no production detection: the wall-face expiry
  is the whole mechanism (refresh.rs 97 and 391, login.rs 167,
  start.rs 622, server.rs 22, 80, 460), and `access_expires_in_
  seconds` is its boundary read.
- `BlockingClock` and `SystemClock` are at zero in the daemon's tests
  already; the design's §1 counts are history.
- T1 is mostly a duplicate of `sessions.rs:801` (round 4): its own
  property is the logout clearing the mark, one step of one row.

## Taken from the review

The first review's six (`t5-r5a-brief-e818ed51.md`):

1. A log read widens the scenario boundary without a ruling: no
   `log_text`; T3's log check a component case beside the table (Do
   1, Do 2, done-when).
2. Holders held less than the retired tests: the wire record carries
   the bearer and the body, and a token "standing" is read through
   the next send (T4's reset and third check, T7's second check,
   T21/T24's bearers); T11's entries and clear; T1's grant after the
   mark retired by the rule (Do 1 to Do 4).
3. T18 ends under `C#3`; T17's first logout is `Internal` with the
   printed retry, then the retry's `Cleared` (Do 3).
4. T24's own four-response script (Do 4).
5. The code diff checked under `crates/` from the brief's last cut
   (done-when).
6. `world` at door.rs:381, `TOKEN_HEADERS` at refresh.rs:287 (Read).

The second review's one remnant (`t5-r5a-brief-17265e8f.md`): the
table's surface list named `log_text` (Do 2).
