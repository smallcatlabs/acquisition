# 115 — the harness's login returns the guard that stops its daemon

Issue #115. The last of the hygiene group in "Pull-forward"
(`roadmap/README.md`), after #125's close; #91 follows it. The rhythm is
"A step". Test-only: the CLI's test harness and the CLI's tests, nothing
a user meets.

## Goal

A CLI test that logs in through the harness (`login`, `login_with`,
`login_as_mock`) gets back the guard that stops the daemon its login
started, marked `#[must_use]`, so the test holds it or says in code why
not; the guard is structure, not discipline (P4, a rule that can be
broken silently becomes a check). Today #68's guard (`stop_on_drop`,
which runs `acq daemon stop` when a test ends, failure included) and
#96's accounting in it (no endpoint left under the test's scratch
runtime directory after the stop) are held by six tests that chose to
declare it; every other test whose login spawned its daemon leaves it to
the isolation list's 30 s idle exit when it fails, holding its world's
lock and files past the scratch's removal. A login against a daemon the
test already owns (one it started with `start_daemon`, or an earlier
login's) gets a guard that finds nothing to stop, so the test's own
`Proc` keeps ending it as now. When the step is done no CLI test that
logs in can end with a daemon of its own world still bound, and the
guard's accounting says so on every one, on Windows included, where
#125's run 40886043255 showed that accounting reading a pipe the stop
had already ended.

## Read

- `AGENTS.md`: C122 (a behaviour is pinned once, at the lowest crate
  that holds it; the process tests share one harness and one isolation
  list), P4 under "Principles".
- `roadmap/README.md` ("A step", "Pull-forward"); issue #115
  (`gh issue view 115`; no comments).
- `roadmap/briefs/96-tour-leak.md`: the guard's accounting, why it
  counts endpoints and not processes, and its pin.
- `decisions/daemon.md`: C83 (a daemon's socket derives from its world
  into the scratch runtime directory and nothing else binds there),
  C133 (the exit: drain, checkpoint, remove the endpoint, then
  `process::exit`), C137 (no test asserts a wall-clock bound a slow
  machine can miss; a liveness wait is widened so a starved runner
  cannot fail a healthy path). `decisions/frontends.md` holds no ruling
  on a test's teardown.
- `REVIEW-CHECKLIST.md`: "A test that owns what it does not clean" and
  "A stop that returned early" (Protocol, client, daemon lifecycle);
  "A fix staging the nearest failure, not the named one" (Frontends).
- The code:
  - `crates/acquisition-cli/tests/harness/mod.rs:121` to 181:
    `StopOnDrop`, `stop_on_drop`, `login`, `login_with`, `login_as_mock`.
  - `crates/acquisition-client/tests/harness/mod.rs`: `scratch_tmp`
    (138), `Proc` and its drop (323, 348), `sockets_under` (464 Unix,
    489 Windows, the live pipes of the worlds beside the runtime
    directory), the shared `login` (834).
  - `crates/acquisition-client/src/client.rs:393`, `STOPPED_WITHIN`
    (20 s), and `stop` (1578): after `stopping` the stop reads its
    connection to its end, which the comment calls the daemon's process
    ending after its endpoint is removed.
  - `crates/acquisition-client/src/endpoint.rs:66`, `exists`: on
    Windows a pipe exists while `\\.\pipe\` lists its name.
  - `crates/acquisition-daemon/src/daemon/server.rs:616`, `leave`, and
    `daemon/endpoint.rs:70` to 130: on Windows the listener holds the
    next pipe instance until the process ends; nothing removes the name
    before `process::exit`.
  - The guard's users: `error_kind.rs` (four tests, three of them
    logging in), `readme_tour.rs:304` to 390 (`recipe_daemon`, the
    recipe test, #96's pin
    `a_daemon_the_recipe_starts_is_stopped_when_its_test_fails`).
- `python3 -I tools/bothy.py pack --path` for the harness routes to
  `daemon` and `frontends`; nothing there beyond the above bears on a
  test's teardown.

## Input

Measured on this tree (`235b28a6`).

- **The sites.** Harness logins in the CLI's tests:
  `grep -rnE '(^|[^_])login(_with|_as_mock)?\(' --include='*.rs'
  crates/acquisition-cli/tests | grep -v 'harness/\|record_login\|fn login'`
  gives 23 calls in 12 files: `apply_loop.rs` 2, `characters_wire.rs` 1
  (in `login_journaled`), `daemon_signal_exit.rs` 1,
  `daemon_stop_checkpoint.rs` 1, `error_kind.rs` 4, `journey.rs` 1,
  `new_league.rs` 1, `readme_tour.rs` 1 (in `recipe_daemon`),
  `realm_wire.rs` 1 (in `login_journaled`), `seat_faults.rs` 4 (one in
  `a_saved_plan`), `store_answers.rs` 1, `web.rs` 5. Ten of the twelve
  hold no guard. The issue's 16 files is wider than true: its grep also
  matched the store's `record_login` (`diagnose.rs`, `plan_json.rs`,
  `price_json.rs`, `price_story.rs`, `search_json.rs`, `shop_json.rs`
  call only that). Eleven of the 23 log in against a daemon already
  running in their world: eight after the test's `start_daemon`
  (`daemon_signal_exit.rs`, `journey.rs`, `new_league.rs`,
  `store_answers.rs`, and in `seat_faults.rs` three tests and
  `a_saved_plan`, whose three callers start one first), and three later
  logins, one in `error_kind.rs`'s logout test and two in `web.rs`'s
  account test, whose first login's daemon still runs. The other twelve
  spawn their daemon; four of them hold a guard today.
- **One guard is not a login's.** `error_kind.rs`,
  `the_login_ends_with_the_daemons_words_not_a_bound_of_its_own`, runs
  `acq auth` itself under `ACQ_LOGIN_WITHIN=0` and expects it to fail,
  so the harness's login cannot start its daemon; it keeps
  `stop_on_drop`.
- **One test reorders its daemons.** `web.rs`,
  `the_jobs_page_opens_with_the_halt`: its login spawns a daemon, the
  test stops it, then starts its own with `start_daemon_with`. That
  `Proc` drops before an earlier-declared guard, killing its daemon, and
  a killed daemon leaves its socket file on Unix, which the guard's
  accounting would read as a daemon left.
- **The Windows evidence.** `gh run view 40886043255 --log-failed`
  (`main` at `a8c5c5be`, Gate (Windows)): `error_kind::c144_logout_json_
  reads_the_keyring_field_not_the_sentence` (2.36 s) and
  `c144_logout_json_says_what_the_keyring_did` (4.11 s) each failed in
  the guard, "a daemon of the test under …\acq-forged-8724-0 outlived its
  stop: [\\.\pipe\acq-…-836d3641227e]" (and `acq-logout-1816-0`).
  Their durations say the stop returned on its connection's end, not
  past its 20 s bound. Every other Windows gate run since #96's guard
  was pushed (`c308cf0d`, run 40807489901) ran both tests and passed
  them: 17 of 18 runs, #119's two, on its rebase onto `a8c5c5be` (runs 40886442836 and
  40886465932), among them. Two failed drops in 36 of those two tests.
- **The reading of it.** On Windows a pipe's name lives while any handle
  to an instance of it is open. The daemon removes nothing before
  `process::exit`; the system closes its handles as the process is torn
  down, and the stop's client sees its connection end when that
  instance's handle closes, which need not be after the listener's
  next instance. So the stop can return, correctly, with the name
  still listed for a moment. That is a reading of the code and the two
  durations, not a measurement: no Windows machine runs this step. What
  it rules out is the daemon staying up: a daemon that stayed would hold
  its stop connection open, and the stop would have waited its 20 s.
- **What the returned guard does to it.** Neither explains nor fixes
  it: the guard runs the same check after the same stop, and returned
  from every login it would run that check on more tests'
  drops each Windows run, eight of them new. So the step settles it in the guard (Do 2)
  rather than filing it.

## Do

1. **The guard returned** (`crates/acquisition-cli/tests/harness/mod.rs`).
   `login`, `login_with` and `login_as_mock` return `StopOnDrop`, the
   type marked `#[must_use]` with words saying the guard stops the
   daemon the login started when the test ends. Before its `acq auth`
   the login asks `acq daemon status --json` (which never spawns); when
   a daemon is already running in the world the guard is inert, finding
   nothing to stop, since the daemon is the test's own or an earlier
   login's. `stop_on_drop` stays, for a daemon a command other than the
   harness's login spawns; its one user is then the `error_kind.rs`
   test Input names. The doc comment says the rule (the guard owns the
   daemon its login started) and why the decision is the tool's own
   word for a running daemon and not an endpoint on disk (a killed
   daemon's socket file outlives it on Unix).
2. **The accounting waits for the endpoint to go**, then counts. After
   its stop an active guard polls `sockets_under(&scratch_tmp(base))`
   until it is empty, for at most the client's `STOPPED_WITHIN`; one
   still bound then fails the test, or is printed beside a failure
   already under way, as now. On Unix the endpoint is removed before
   the process exits, so the first look finds it gone; the wait is
   Windows' handle teardown, and its bound is a liveness bound on a
   healthy path that takes milliseconds (C137), never the assertion: a
   daemon that stayed is still named, 20 s later. The doc comment says
   so, with run 40886043255 as its evidence.
3. **Every site holds it.** Each of the 23 calls binds the guard to a
   named local (`_login`, or `_daemon` where no `Proc` takes the name);
   `login_journaled`, `recipe_daemon` and `a_saved_plan` return it to
   their tests. `error_kind.rs`'s three `stop_on_drop` lines before a
   login and
   `readme_tour.rs`'s two become the login's returned guard; #96's pin
   keeps its shape, the guard now `recipe_daemon`'s. `web.rs`'s halt
   test drops its guard explicitly after its own `daemon stop`, with a
   comment saying why (the daemon it starts next is its own `Proc`).
4. **The pin** (`crates/acquisition-cli/tests/login_guard.rs`, a module
   of the one test binary), named for the property:
   `a_login_against_a_daemon_the_test_owns_gets_a_guard_that_stops_nothing`:
   `start_daemon`, a login, the guard dropped, and `acq daemon status
   --json` still answering `running: true` with the started pid. Run
   once with the guard made always active (a scratch copy of the
   harness, restored from it), the failure in the report. The failure
   path of a login that spawned its daemon stays pinned once, by #96's
   pin, whose daemon is now started and guarded by `login_with`
   (C122: pinned once).
5. The full gate (`tools/gate.sh`) after `cargo build --workspace`, in
   the background, its output under this session's scratch. Commits by
   concern: 1, 2 and 4 with the sites they force (3), since the harness
   change does not compile without them under `-D warnings`; then
   "Current step" as built, review pending; then the Codex review over
   the range.

## Done when

- `login`, `login_with` and `login_as_mock` in the CLI harness return
  `StopOnDrop`, and `StopOnDrop` carries `#[must_use]`.
- Under `crates/acquisition-cli/tests`, outside the harness,
  `stop_on_drop` is named once, by
  `the_login_ends_with_the_daemons_words_not_a_bound_of_its_own`.
- Every harness login under `crates/acquisition-cli/tests` binds its
  guard to a named local or returns it, except where the test drops it
  on purpose: `web.rs`'s halt test, after its own stop, with a comment
  saying why, and `login_guard.rs`'s pin, whose drop is what it tests.
  Settled in the round (#115's review, its note on the brief; the line
  first named `web.rs` alone).
- The guard's drop polls `sockets_under(&scratch_tmp(base))` for at
  most `STOPPED_WITHIN` before failing, and its doc comment names run
  40886043255.
- `a_login_against_a_daemon_the_test_owns_gets_a_guard_that_stops_nothing`
  and `a_daemon_the_recipe_starts_is_stopped_when_its_test_fails` are
  green.
- The commit message states the measured count of harness logins (23 in
  12 files) and that the issue's 16 counted `record_login`.
- The gate is green on the step's tree, each stage by its own exit
  status. The Windows behaviour of Do 2 is not verified by this step;
  the owner's push runs the Windows gate.

## Not in scope

- The MCP's and the client's harnesses: their logins (`acquisition-mcp`,
  `acquisition-client`) go through the shared `login`, which stays as
  it is; this step is the CLI adapter's. Raised in the report with a
  count if the same leak stands there.
- Tests that spawn a daemon lazily without logging in (a use verb
  first): no login, no guard; raised with a count, not fixed.
- The daemon removing its Windows pipe before `process::exit`, or the
  client's stop waiting for the name to go: a product change to the
  stop's contract (C133), not a test's teardown. Raised, not changed.
- #116 (a daemon judged another contract left running after a respawn).

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `115`;
the tests' own scratch is the harness's. Kill only the processes you
started. Any `acq` is in a mock world. Nothing sends to GGG. Never push.

## Report

`TEMPLATE.md`.
