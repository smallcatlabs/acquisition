# 4-0-door — the client's door as a machine (#147, step 1)

## Goal

When this step is done, the client's connect-and-handshake lifecycle
(discovery, connect, hello under its deadline, the verdicts, spawn or
report, the stop-and-wait) is a value: a `Door` machine with an enum
of states, events and effects and a pure `step`, pinned by a table of
event scripts in a synchronous test; the existing async loop is its
driver, performing effects and returning events; the five-second
`c86_` wait is retired, held by the expiry row and the `c137_`
real-wiring test at zero. No word a user meets changes. This is
C157's first machine (a lifecycle that waits is a value, time an
event) and proves the shape step 2, the daemon's `Life`, copies. It
is the smallest machine, in the crate that sends nothing to GGG, so
it may run during a league sitting.

## Read

- `roadmap/briefs/4-0-design.md` §2 to §4 (the shape; the `Door` row
  of the table) and `roadmap/TIME-SEAM.md` §1 to §2 (why).
- `decisions/daemon.md`: C157 (a lifecycle that waits is a value),
  C137 (a deadline's contract is its expiry behaviour; the knobs),
  C86 (the hello deadline: unresponsive on every door, never
  absence), C10 (the single-version handshake: replaced, never
  negotiated with), C84 (the two identity values), C147 (a door takes
  the world it acts on; the environment read once). `decisions/network.md`:
  C88 (the real provider is the default; a human spawns).
- `AGENTS.md`: C122 (pinned once, at the lowest crate), C153 (a
  finding joins the surface that holds it), C154 (a retirement names
  its holder).
- `crates/acquisition-client/src/client.rs`: the module doc;
  `open_door` (1307–1545, the loop whose state is the locals `child`,
  `startup`, `respawned`, `stop_failed`, `looked_for_winner`,
  `found`); `stop_until` (1647); `handshake` (1736);
  `winner_listens` and `look_until` (1939, 1968); `reach` and
  `Reached` (1263–1272); `Startup` and `expired` (1242–1259);
  `Verdict::of` and `DaemonId::judged` (778, 819), pure and to stay
  so; `Spawned`, `exited` and `spawn_daemon` (2002–2045);
  `lock_holder` (530); the knobs `hello_within`, `startup_within`,
  `parse_within` and `MAX_WITHIN_SECS` (335–352, 1232);
  `STOPPED_WITHIN` and `GONE_LOOK_EVERY` (367, 373); `Unresponsive`
  and its Display (380–427), whose words stay. `src/endpoint.rs`:
  `connect` and `exists` (28–66), the Windows pipe-busy retry at 46,
  a driver concern.
- `crates/acquisition-client/tests/contract.rs`: `c86_…` (326, the
  5 s wait, `elapsed >= 5s` at 344–347); `c137_…` (407, 484, 1001);
  `c147_…` (916, with its 2 s sleep at 936; 969; 1042; 1096).
  `tests/harness/mod.rs`: the isolation list's knobs at 60
  (180–181), `set_knob` (951), `wait_exit` (335) and the 120 s
  ceiling (442, 969, 1016).
- `crates/acquisition-cli/tests/watch_recovery.rs`: the one frontend
  test naming `Unresponsive`; it reads the same after.
- `crates/acquisition-client/Cargo.toml`: the crate links protocol,
  store, tokio, serde, serde_json, anyhow and sha2; no new
  dependency, and no `test-util` (the template's rule).

## Input

Measured on `main` at `6893fc74` by the design's read, not by a run:
ten sites where real time enters `client.rs` (the table in
`4-0-design.md` §1); `c86_` spends about five real seconds; the
`c137_` tests spend none. The last push's CI, read at the design:
`Gate (Linux)` 41209304807 and `Gate (Windows)` 41209304795 on
`6893fc74`, both success. Re-read at the step's start; a red run is
this step's first concern or is filed.

## Do

1. **The machine**, in a new module
   `crates/acquisition-client/src/door.rs`, its doc headed by C157:
   `enum State` (Reaching, AwaitingHello, Judging, Spawning,
   LookingForWinner, Stopping, Done), `enum Event` (Reached, Nothing,
   ConnectFailed, HelloReply, HelloClosed, ChildExited, LockHolder,
   StopAck, StopRefused, EndpointGone, Deadline), `enum Effect`
   (Connect, SendHello, Spawn, Stop, LookLock, Arm, Cancel, Report),
   `enum Timer` (Hello, Startup, Stop, Look, Pause), and a `step`
   taking one event and returning the effects. The policy flags
   (`spawn`, `replace`, from C88 and the door) and the knob values
   are the machine's construction inputs, read by the driver once
   (C147); the machine arms durations, never instants. A machine
   leaving a state cancels the timers it armed there, as an effect.
   `Verdict::of` and `DaemonId::judged` are called by `step`,
   unchanged.
2. **The runner**: `drive(machine, script)` returning the effects,
   about ten lines in the client's test support, with a table of
   rows: each a name, a script of events and the expected effects.
   Rows cover at least: hello answered and ours; the hello deadline
   on each door (the expiry row: `Unresponsive` with its `within`, no
   `Spawn`, no `Stop`); another world; each mismatch verdict under
   `replace` and under `!replace`; `Nothing` with spawn allowed, then
   the startup deadline; the child exited with a lock holder, the
   winner look found and not found; stop acknowledged and gone, and
   the stop bound expired; a second mismatch after a respawn. Each
   row's expected effects are written from the rulings, never from
   running the driver (an oracle, `GLOSSARY.md`).
3. **The driver**: `open_door`, `stop_until`, `observe_at` and
   `stop_any_at` rewritten as one loop over `step`, each effect
   performed by the existing functions (`reach`, `handshake`,
   `exchange`, `spawn_daemon`, `Spawned::exited`, `lock_holder`,
   `endpoint::exists`) and each `Arm` a `tokio::time::sleep_until`
   raced against the pending I/O; `Deadline` is the event when the
   sleep wins. `look_until`'s injected `now` and the re-check
   `now() < deadline` go, held by the rows. No poll constant remains
   in `client.rs` except as a `Pause` the machine arms (the bind
   poll, the gone poll, the settle after a replace).
4. **The real-wiring tests**: one per effect kind the driver performs,
   under the harness's 120 s ceiling, asserting no duration. The
   `c137_` tests are these tests and stay; the 2 s sleep in
   `c147_a_spawn_race_is_won_however_late_the_winner_starts` (936)
   becomes a `LockHolder` row in the table and a real-wiring test
   that asserts the outcome only.
5. **Retire `c86_…`** (326): its property is held by the expiry row
   (the verdict, no spawn, no replace, the holder named) and by
   `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
   (the real wiring at zero); its words (`within 5s`, the kill line,
   the keychain clause) by a wording test over `Unresponsive`'s
   Display that takes `within` as a value. The commit carries the
   C154 line, `retired <test>; held by <row> and <test>`.
6. `tools/gate.sh` once, on the final tree; the client's goldens and
   the CLI's `watch_recovery.rs` unchanged.

## Done when

- `crates/acquisition-client/src/door.rs` exists and its machine has
  no `async`, no `tokio`, no `Instant` and no I/O type in its
  signatures: `grep -n "tokio::\|Instant\|async fn" crates/acquisition-client/src/door.rs`
  finds nothing, quoted in the commit.
- The table has one row per transition Do 2 names, and the report
  prints the table's run time, asserted by nothing (C137).
- `client.rs` holds no `timeout_at` and no `now() < deadline`
  re-check, and its only `from_millis` sleeps are the `Pause` timers
  the machine arms: `grep -n "timeout_at\|from_millis" crates/acquisition-client/src/client.rs`
  quoted in the commit.
- `c86_a_peer_that_never_answers_hello_is_unresponsive_on_every_door_and_never_replaced`
  is gone, the commit message carrying its `retired X; held by Y`
  line, and no test under `crates/acquisition-client/tests` asserts
  `elapsed` against a bound.
- `crates/acquisition-cli/tests/watch_recovery.rs` and every golden
  under `crates/*/tests/goldens` are byte-identical to the step's
  parent commit.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- The daemon's `Life`, its constructor and `Clock::sleep_until`:
  step 2 of #147, its own brief after this closes.
- The parking clock and t5 (#141): step 3's spike, then #141.
- A shared clock trait or crate: none (`4-0-design.md` §5); the
  driver uses tokio time directly.
- The login callback window, the verbs' job-wait poll and the CLI's
  login poll: `login.rs`'s and 4-full's, not this step's.
- Any word a user meets: the reports' Display impls stay as they are.
- The store's fixed waits: #148.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on
`main`, in a checkout other sessions may share; never push; `acq`
only in a mock world; kill only this session's pids; an experiment
is undone from a scratch copy, never `git checkout -- <file>`. The
owner's machine is his during a league sitting: this step's tests
spawn a real `acqd` in a mock world only, and the worker runs no
build while he sits. The attribution line names the model the worker
runs on.

## Report

`TEMPLATE.md`.
