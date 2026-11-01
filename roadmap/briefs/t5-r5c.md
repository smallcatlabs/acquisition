# t5-r5c — the sweep (#141, round 5c)

The third of round 5's three parts (`t5-design.md` §4 round 5 as
split on 2026-10-31; ruled 2026-10-31, C158). First draft. Written
from one read-only inventory at `5c301c06` (the nine door-able tests
of the other files, `ManualClock`'s 69 constructions, the waits left,
the harness's duplicates, the exempt-route set's five copies), every
line printed, re-anchored to the tree at 1092c57b (round 5b's
close) where 5b moved a line (`door.rs`, `exit.rs`, `dispatch.rs`,
`harness.rs`); rounds 1 to 5b closed (`LEDGER.md`).

## Goal

When this round is done, the last door-able tests of the other files
run through the door (the three fault-marker tests on a world the
door binds inside its runtime, the apply fan-out and its admission
rows, the auth-wait and route-knowledge scenarios, the quote); every
daemon test runs on a hand-driven parking clock, `ManualClock`
retired with its last construction; every literal ceiling is `HANG`
and every sleep that stood for a state wait is an awaited state or
named with the reason it stays; the harness's three duplicates are
one; the routes whose first send carries no probe are declared in one
place, every reader of the wire holds the daemon to it, and a check
holds the four copies outside the daemon crate equal to it, the
owner's ruling of 2026-10-31 entered in the registry. The stress test
is named as the component it is (two fixtures the door's one server
cannot host). The one production edit is a doc comment that named
`ManualClock`; no word a user meets changes.

## Read

- `roadmap/briefs/t5-design.md` §2, §3 (the exempt-route row), §4
  round 5 (the dated lines), §5 (the exempt-route list "kept in the
  client's harness under a check", ruled 2026-10-31); `t5-r5b.md` Do
  1 and Do 2 (the admission table, `live_sessions`, `real_named`, the
  observer read after `run`); `t5-r5a.md` Do 1 (`Server::own`,
  `restore()`); `t5-r4.md` Do 1 (`quiet`, `submit`, `held`).
- `decisions/daemon.md`: C158, C45, C134 (a fault marker fails its
  job once; a faulted probe closes its route), C6, C83 (the world);
  `decisions/network.md`: C32, C40, C42, C43 (the Pinned lines naming
  tests this round moves by name: `dispatch.rs:303`, `quote.rs:16`,
  `parents.rs:96` and `:14`, which keep their names), N16 (one HEAD at
  startup), N24 (a HEAD is uncounted); `AGENTS.md`: C1 (the client
  links the protocol and the store, never the daemon), C12, C153,
  C154; `tools/arch-check.sh` (check 5 walks the dev edges: a file
  read is not an edge).
- The fault marker: `FAULT_MARKER` and `fault_marker_path`
  (`daemon/queue.rs` 52–53, 101–104: `None` without a `world`);
  `install_panic_hook` writes it (`start.rs` 255–280); `restore_jobs`
  reads it (`queue.rs` 180–184, `read_fault_marker` 109–131), fails
  the named job or the probe's route jobs (217–243), degrades the
  route (194–199); the child-process test pins the real writer
  (`exit.rs:1777`). The world: `DoorBuilder::world(world, socket:
  BoundSocket, store_dir)` (door.rs:437, never called) needs a
  `BoundSocket` that only `Listener::bind` produces (async, a tokio
  listener): it must bind inside `run`, before the daemon is built, as
  `Server::own`'s closure runs.
- The exempt-route set: `NO_PROBE_ROUTES` (`routes.rs:154`,
  `["profile", "league"]`, `pub(super)`, read by `route_probes`
  165–167); `TOKEN_ROUTE` (`ratelimit/limiter.rs:38`, "oauth-token";
  `auth.rs:256`); its four copies: the daemon harness's oracle
  (`harness.rs` 1123–1127, the `probed` match), the client harness
  (`crates/acquisition-client/tests/harness/mod.rs` 718–721, with its
  second-probe check 730–733), `tools/persist-verify.py:27` and
  `tools/tracer-verify.py:44` (`NO_PROBE = {...}`). No oracle asserts
  a HEAD absent on an exempt route's first send today; only
  `dispatch.rs:303` pins it, against the mock. Precedents for a test
  reading another crate's file as text: `acquisition-cli/tests/
  cli_reference.rs:13`, `readme_tour.rs:44`, `no_ids.rs:7`.
- `ManualClock` (`harness.rs` 24–64: `new`, `advance`, a `sleep` that
  advances itself then yields; nothing parks) and `ParkingClock`
  (76–244: `advance` wakes the due, `advance_to_next`, `sleepers`,
  `laptop_sleep`; the same wall origin and `kind`). Its 69
  constructions in 15 files and one typed parameter
  (`world_daemon_on(clock: Arc<ManualClock>)`, `exit.rs:1415`); the
  ten file-local helpers that construct one (`login_daemon`,
  `session_daemon`, `live_daemon`, `persisting_daemon`,
  `ceiling_daemon`, `learned_daemon`, `world_daemon_at`,
  `saving_daemon`, `refresh_answered_by_headers`, `api_answered_by`);
  the doc mentions (`dispatch.rs:2529`; `server.rs:677`, a production
  doc comment). The four tests that rely on the self-advancing sleep
  (a landed 429's hold, a window: `limiter.rs` 989–1020, 676–711):
  `dispatch.rs` 371 (the 429's hold before the next probe, the journal
  `[0, hold, 0]`) and 472 (the 1015's 30 s hold after the reset),
  `rails.rs:26` (5b's), `stress.rs:32` (the sixth fetch's window). The
  production sleeps clamp to `[PACING_FLOOR, HALT_POLL]` (`choke.rs`
  397, 417, 436, 453) or slice by a second (`dispatch.rs:220`); the
  hand-driven shape is `dispatch.rs` 196–203 (an `advance_to_next`
  loop beside the yield poll).
- The nine door-able tests, at `5c301c06`:
  - `exit.rs:1457` (a fault marker fails its job once, C134:
    `world_daemon`, three rows upserted into `daemon_db_path(dir)`
    1462–1467, `fault.json` 1468–1469, two lifetimes 1470 and 1504, no
    dispatcher; `job_list()`, `JobDb::get`, `marker.exists()`),
    `exit.rs:1551` (a faulted probe closes its route: `seed_faulted_
    probe` 1524–1548, a `Script` bait aborted at 1592, `wait_terminal`
    ×2, an internal submit of a fetch failing "degraded", the journal
    and wire empty), `exit.rs:1603` (even when halted: `rails.json`
    `{"tripped"}` under `tripwire_config()`, `halted()`, `reset_rails`,
    `advance(PROBE_COOLDOWN + 1 s)`, the 100 ms `settle()` ×2).
  - `parents.rs:14` (apply fans out exactly its tuples and records
    through the store, C43: Mock, an internal apply submit as Alice,
    `job_list`, `Store::open(account_path(..)).status().responses`),
    `parents.rs:96` (apply admission refuses bad vocabulary and blown
    budgets whole, C42: eleven internal submits, ten refusal words,
    `jobs.is_empty()`, the next id 1 and Waiting).
  - `dispatch.rs:66` (auth waits do not cap independent progress: the
    delayed token server, a `Noop` store and an "old-user" session set
    privately, two `whoami` submits Running behind the held refresh,
    a 0.0 s sleep Done, a 1 s yield loop over `shared.list`),
    `dispatch.rs:303` (declared route knowledge holds against the mock,
    C32: `recent_sends`, `endpoint_state` ×2; its routes learned by the
    jobs, no private `choke.head`), `dispatch.rs:224` (malformed
    listings fail the job, not just the store: its only private read
    the store on disk, which C12 makes a frontend's; round 3 kept it
    for that read).
  - `quote.rs:16` (C40: a private `choke.head("fetch")` 22–31 learns
    the route, then `handle_request(Quote)`; `recent_sends().len()`,
    `jobs.is_empty()`).
  - `stress.rs:32` (N6: the mock and a delayed token server joined by
    `provider.token_url` set by hand 39–41, a session and an expiry
    written privately, `wait_for_refresh_waiters` 131, the private
    reads 183–222: the refresh token (held by the store's save), the
    access token (no twin on the mock), `refresh_flight` (none),
    probes (`List`), send counts (the journal), occupancy
    (`DaemonStatus.in_flight`), policies (`Dashboard.policies`)).
- The waits at `5c301c06`: the scenario sleeps `exit.rs:126`
  (`settle()`, 100 ms, called at 386, 808, 1629, 1649), `exit.rs:311`
  ("the parked refresh reached the gate", two component users),
  `exit.rs:558` (`std::thread::sleep`, a multi-thread component),
  `spec.rs:2016` (the limiter's spec); the literal ceilings
  `dispatch.rs:89` (1 s), `stress.rs:10` (1 s), `log.rs:114` (10 s),
  `exit.rs` 1856 (5 s) and 1900 (10 s), `login.rs` 628, 633, 659, 669
  (5 s), `credentials.rs:20` `ANSWERED_WITHIN` (30 s, equal to the
  local nextest kill of `.config/nextest.toml`, which `HANG` also
  equals); the polls `wait_terminal` (44 calls), `wait_state`
  (`exit.rs:109`, 4), `drive_drain` (8), `await_a_look` (2),
  `wait_for_refresh_waiters` (stress 9–29 at 1 s; sessions 463–483 at
  `HANG`), the inline loops (`dispatch.rs` 90–103, 174, 198, 501, 529;
  `exit.rs` 611–622, 1363–1366, 1900–1905; `log.rs` 114–122;
  `credentials.rs:433`).
- The harness's duplicates: `test_daemon_scenario_at` (`harness.rs`
  802–821; callers `credentials.rs:490`, the one with a need, the log
  path inside the child's dir; `log.rs` 94, 258, 412, each served by
  `test_daemon_with`); `install_tokens_locked` (`refresh.rs` 88–102,
  production; test callers `log.rs` 268 and 422, seeding
  "scenario-user", replaceable by a `Sessions` value: every generation
  use is relative); `TOKEN_HEADERS` (`harness.rs:563`; copies
  `login.rs:86`, byte-identical, shadowing the glob import, used at
  95; `log.rs:202` `TOKEN_RATE` and `log.rs:38`, identical).
- #131's daemon rows (read at HEAD by the inventory): closed by 4-0
  and t5 so far: the exit upper bounds, `BOUND`, the sessions and
  dispatch ceilings, `wait_for_sleepers`, credentials 360, dispatch
  1647 and 1771; 5b closes queue 1713 and rails 12, 204, 378, 618;
  this round closes dispatch 89, stress 10, login's four, exit 311's
  reason named; open after it: `spec.rs` 2016, 2038, 2162, 2200, 2662,
  2735, 2904, 2917, 3380 (the limiter's spec, its own clock) and the
  client's rows.

## Input

Fixed before the round: the inventory above; the door as round 5b
left it. The gate's test stage at 1092c57b: 23 s on the
orchestrating session's machine. The last push's CI on 1092c57b:
read at the round's start. Round 0's baseline (run 41265798977): its
shards' artifacts, round 6's input. The registry line, drafted for
`decisions/network.md` in the entry rule's form (the owner's ruling of
2026-10-31, the design's §5), entered as C159 at this brief's commit
(`python3 -I tools/bothy.py check` green on it), its Pinned "nothing
yet" until the round builds the oracle and the check:

> - **C159 — The routes whose first send carries no probe are the
> daemon's declaration and the token endpoint, and every reader of the
> wire holds the daemon to it.** A declared route's or the token
> endpoint's first send in a lifetime is no HEAD and every other
> route's is one; a reader of the wire outside the daemon crate keeps
> its own copy of that set only under a check that fails when the copy
> and the daemon's set differ. *Why:* the set was written in five
> places and the oracles only skipped the exempt routes, so a probe
> sent on one passed unseen. *Pinned:* nothing yet (the daemon
> harness's wire-contract oracle and a gate check over the copies,
> when built).

## Do

1. **The door's world.** `DoorBuilder::world(world: String, store_dir:
   PathBuf)`: the socket bound inside `run` before the daemon is
   built (`Listener::bind` on a path under the test's own directory),
   as `Server::own`'s closure runs; the first caller the three fault
   tests. A fault marker and the rows of a previous lifetime are
   persisted inputs the scenario stages before the door opens
   (`fault.json` under the store dir; rows through the store's
   `JobDb`), as 5b stages rows; after `run` returns, the marker's
   absence and the rows' states are the observer's reads (the file
   under the directory the test owns; `Persisted::read_job`).
   - 1457: `.world`, `.jobs_db(JobDb::open(daemon_db_path(dir)))`,
     `.restore_jobs(Retention::default())`, `.restore_landed(false)`
     (the sleep rows must not run), `Server::Account(("GET", "/never",
     0))`; `List` for `[(1, Failed), (2, Waiting), (3, Waiting)]`,
     `Result { 1 }` carrying "interrupted by a fault" and "not
     replayed"; after `run`, the marker absent; a second door on the
     same directory (its own `run`, after the first's), `.restore_jobs`
     again: `List` `[(2, Waiting), (3, Waiting)]`, `Result { 1 }`
     unchanged.
   - 1551: the seeded probe row 1 Running and fetch row 2 Waiting with
     the marker naming the probe; `.world`, `.jobs_db`, `.restore_jobs`
     (the dispatcher running); `await_terminal(2)` Failed carrying
     "route fetch" and "a probe on this route broke the daemon";
     `submit("fetch", None)` then `await_terminal` Failed carrying
     "route fetch is degraded"; `journal()` and `sent()` empty; after
     `run`, the marker absent.
   - 1603: as 1551 with `.rails(RailsConfig { state_path, ..tripwire_
     config() })` and `rails.json` seeded tripped; `DaemonStatus.rails.
     halted` `Some`; `await_terminal(2)` Failed with the route words;
     `settle`; `advance(PROBE_COOLDOWN + 1 s)` (the degraded route's
     cooldown, which nothing parks on); `ResetTripwire` acknowledged;
     `settle`; the journal empty; after `run`, row 2 Failed through
     the observer's read and the marker absent.

2. **The other door-able tests.**
   - `parents.rs:14`: `Server::Mock`, `.store_dir`, `.sessions(live_
     sessions(&clock))`; `request(Submit { apply, params, account:
     Alice })`; `await_terminal`: the payload's requests 2, child_jobs
     2, done 2; `List` filtered on the parent: the children equal the
     tuples in order, as Alice; `Store::open(account_path(..))`'s
     `status().responses` 2 (the store's read API, a frontend's read,
     during the run as `acq` reads it).
   - `parents.rs:96`: rows of 5b's admission table (an apply with bad
     vocabulary, a blown budget: the ten refusal words, nothing
     admitted, the next sleep id 1); the admitted apply's "Waiting"
     (166–169) is held by `parents.rs:14` (an admitted apply runs to
     Done) and by the id check; the commit's C154 line says so. The
     protocol path runs `admit_client` before `validate_apply`
     (`server.rs:269`): a row whose words differ from the old test's
     is a finding for the report, not a row rewritten to fit.
   - `dispatch.rs:66`: `Server::Token(Delayed("200 OK", token_body(..)))`,
     `Door::open(.., Arc::new(NoopCredentialStore))`, `.sessions` with
     `rt-old` under "old-user" (not `logged_in_sessions()`: another
     name, no keyring word); `held()`; two `whoami` submits; `arrived`;
     `quiet`; `List` both Running (or `await_job(.., Running)` ×2); a
     sleep with `seconds: 0.0` through the wider submit (an empty
     params sleep runs 3 s real, `kinds.rs:571`); `await_terminal`
     Done; release; both Done; the journal one POST.
   - `dispatch.rs:303`: `Server::Mock`, `.sessions(live_sessions)`;
     `submit("profile", Alice)` and `submit("leagues", Alice)`,
     `await_terminal` ×2; `Dashboard.sends` (what `recent_sends` was,
     `server.rs:471`): no HEAD, GET /profile, GET /account/leagues;
     `Dashboard.policyless_endpoints` naming profile; `Dashboard.
     policies` naming league's policy ("league-request-limit" as a
     prefix); the journal's methods and paths.
   - `dispatch.rs:224`: through the door if its only private read is
     the store on disk (the inventory's reading); else kept, its doc
     saying which other read.
   - `quote.rs:16`: `Server::Mock`; `submit("fetch", None)` and
     `await_terminal` (a fetch needs no account in mock mode,
     `daemon.rs:1671`) learn the route through its probe and GET; then
     `request(Quote { .. })`: the provider, `halted` `None`, the work
     echoed, the scopes `[character-list, mock-fetch-request-limit]`,
     the window one used of five (the GET counts, the HEAD does not,
     N24: the old `(0, 5)` was before any send), the ETA at least 10,
     the notes and `not_covered` words as today; `List` unchanged
     across the quote and the journal unchanged across it (what "no
     job" and "nothing sent" asserted).
   - `stress.rs:32` stays a component test, its doc saying why: it
     needs the mock and a held token server at once, which the door's
     one `Server` cannot host (an `Own` relay is a twin not drawn); it
     takes a hand-driven parking clock (Do 3), its ceiling `HANG`, its
     `wait_for_refresh_waiters` kept under `HANG`, and its private
     reads named with their twins (occupancy, sends and policies have
     exact ones; the access token and the flight none).

3. **`ManualClock` retired.** Every construction becomes
   `ParkingClock::new()` (the ten helpers one line each, the rest per
   site), `world_daemon_on`'s parameter `Arc<ParkingClock>`, the doc
   mentions reworded (`dispatch.rs:2529`; `server.rs:677`, the one
   production edit, a comment); the type and its `Clock` impl deleted
   from the harness. The three tests that relied on the self-advancing
   sleep and stay off the door (`dispatch.rs` 371 and 472, `stress.rs:
   32`) gain the hand-driven loop beside their yield poll
   (`advance_to_next` while the state is not yet reached, the shape of
   `dispatch.rs` 196–203), each naming in a comment the hold it
   crosses; a test that already advances keeps its meaning (`advance`
   wakes the due). A test that neither sleeps on the clock nor
   advances is unchanged in meaning: the commit says so once, not per
   test.

4. **The waits.** Every literal ceiling becomes `HANG` (`dispatch.rs:
   89`, `stress.rs:10`, `log.rs:114`, `exit.rs` 1856 and 1900, `login.
   rs` 628, 633, 659, 669); `ANSWERED_WITHIN` stays at 30 s with its
   doc corrected (the local kill is 120 s, `HANG`'s equal; the guard
   fires first by its own margin). The scenario sleeps: `exit.rs:126`'s
   `settle()` retires (386 an awaited state through `wait_state`, 808
   through `await_a_look`, 1629 and 1649 the door's `settle` once 1603
   moves); `exit.rs:311` stays with its reason in a comment (its exact
   signal is `quiet` on a door; its two users are component tests of
   the exit's park); `exit.rs:558` stays (a multi-thread component with
   workers blocked); `log.rs` 114–122's poll under `HANG`. The polls
   that stay (`wait_terminal`'s component callers, `wait_state`,
   `drive_drain`, `await_a_look`, sessions' `wait_for_refresh_waiters`)
   are named in the commit with their reason (a component test on a
   plain runtime has no hook).

5. **The harness's duplicates.** `test_daemon_scenario_at` retires:
   `test_daemon_with` gains the log path as a parameter (or a sibling
   that takes one) for `credentials.rs:490`'s child, whose log must
   live inside the child's directory; `log.rs`'s three callers take
   `test_daemon_with`. `install_tokens_locked`'s two test seeds
   (`log.rs` 268, 422) become `Sessions` values on the builder (the
   expiry wall-relative, the generations at zero: every use is
   relative). `TOKEN_HEADERS`: `login.rs:86`, `log.rs:202` and the
   inline copy at `log.rs:38` take the harness's.

6. **The exempt-route set.** One source: `NO_PROBE_ROUTES` composed
   with `TOKEN_ROUTE`. The daemon harness's oracle (`assert_wire_
   contract`) reads the constants (both visible from `daemon::tests`)
   in place of its literal, and gains the assertion no oracle made: a
   declared route's or the token endpoint's first send in a lifetime
   is no HEAD, every other route's is one (the inventory's shape: every
   route into `seen`; on a first send, `assert_ne!` for an exempt
   route, `assert_eq!` else). A gate check, `tools/exempt-check.sh`
   (or a case of `tools/arch-check.sh`, the worker's call), run by
   `tools/gate.sh` and `--docs`, extracts the literal set from the
   client harness (`mod.rs` 718–721) and the two Python tools and holds
   each equal to the composed set read from `routes.rs` and
   `limiter.rs`; a breaker case (`tools/arch-check-breakers.sh`'s
   shape) stages a drifted copy and sees the check fail. The registry
   gains the owner's ruling of 2026-10-31 (the design's §5, "the
   client keeps its copy under a check"), worded as a property in
   `decisions/network.md`, under 800 bytes, its Pinned the oracle and
   the check; the brief's draft (Input, below) passes `python3 -I
   tools/bothy.py check` before this brief's commit. `dispatch.rs:303`
   keeps pinning the mock's two declared routes; the oracle pins the
   rule.

7. **Retirements**, each with its C154 line: the nine tests by name
   (eight moved, stress kept), `ManualClock` (held by `ParkingClock`
   and the three loops), `test_daemon_scenario_at`, the sleeps and
   ceilings named in Do 4, the `TOKEN_HEADERS` copies, the oracle's
   literal (held by the constants and the check).

8. `tools/gate.sh` once per commit (four at least: the door's world
   with the fault tests; the other door-able tests; `ManualClock` and
   the waits; the harness's duplicates and the exempt-route set with
   its registry line); the test stage's seconds before and after.

## Done when

- `grep -rn "ManualClock" crates/` finds nothing.
- `grep -n "tokio::test" crates/acquisition-daemon/src/daemon/tests/{exit,parents,dispatch,quote,stress}.rs`
  finds none of the eight moved tests' names and finds `stress.rs:32`'s,
  quoted; each moved test keeps its name as a `#[test]` (the Pinned
  lines of C32, C40, C42, C43 and C134 still name their holders).
- `grep -rn "timeout(Duration::from_secs(" crates/acquisition-daemon/src/daemon/tests/`
  finds nothing; `grep -n "sleep(Duration::from_millis(" crates/acquisition-daemon/src/daemon/tests/*.rs`
  finds only `exit.rs:311` and the poll intervals, each with its
  comment, quoted.
- `grep -rn "test_daemon_scenario_at\|install_tokens_locked" crates/acquisition-daemon/src/daemon/tests/`
  finds nothing; `grep -rn "TOKEN_HEADERS\|TOKEN_RATE" crates/acquisition-daemon/src/daemon/tests/`
  finds one definition.
- `grep -rn '"oauth-token" | "profile" | "league"' crates/acquisition-daemon/` finds nothing; the oracle's first-send assertion exists (quoted); `tools/exempt-check.sh` (or its case) runs in `tools/gate.sh` and its breaker fails on a drifted copy (the breaker's output quoted in the report); the registry line is in `decisions/network.md` and `python3 -I tools/bothy.py check` passes.
- `DoorBuilder::world` takes a world and a store directory and binds
  inside `run`; the three fault tests call it (quoted).
- `git diff --stat <this brief's last cut> <head> -- crates/` lists
  files under `daemon/tests/` and `daemon/server.rs` (a doc comment)
  only; every golden, `CLI-REFERENCE.md` and `MCP-REFERENCE.md`
  byte-identical.
- `tools/gate.sh` exits 0 on each commit's tree; the test stage's
  seconds before and after in the report, asserted by nothing.

## Not in scope

- A door twin for `stress.rs:32` (an `Own` relay holding the token
  POST in front of the mock): not until a scenario asks.
- `spec.rs`'s waits (the limiter's spec, on its own clock) and the
  client's #131 rows: #131 stays open for them, its daemon rows
  closed by the close's comment.
- `exit.rs:311` and `exit.rs:558`: named with their reasons; a park
  signal for a component test is a hook the plain runtime lacks.
- Round 6 (the mutants against the baseline).

## Rules

`TEMPLATE.md`. What differs: this round runs in the main tree on
`main`, in a checkout other sessions may share; never push; `acq` and
`acqd` only in a mock world, under the harness; kill only this
session's pids; an experiment is undone from a scratch copy; nothing
tracked is edited while the gate runs. The owner has granted autonomy
and builds for the night of 2026-10-31. The attribution line names the
model the worker runs on.

## Report

`TEMPLATE.md`, and: the test count by file before and after and the
`ManualClock` constructions swapped; the test stage's seconds before
and after; `dispatch.rs:224`'s decision; every agent-facing design
decision the brief left open (`world`'s shape, the check's home and
name, the oracle's words, the loop helper's shape if one is shared)
for the owner's review.

## Raised for review

- The exempt-route ruling is the owner's of 2026-10-31 (the design's
  §5), entered here with its pin; the check's shape (a gate script
  over five copies) is this brief's, since two copies are Python
  tools no Rust check reaches.
- The fault marker as a staged persisted input: a hook wrote it, not
  the store; the scenario stages it as it stages rows.
- `ANSWERED_WITHIN` kept at 30 s against a local kill of 120 s.
