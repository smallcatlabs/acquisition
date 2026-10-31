# 4-0-door — the client's door as a machine (#147, step 1)

Re-cut from the cold review of the first draft (`e4e6b305`, saved
verbatim at `roadmap/reviews/4-0-door-brief-e4e6b305.md` with its
prompt beside): the seven findings and what each changed are listed
at the foot, "Taken from the review".

## Goal

When this step is done, the client's door lifecycle (discovery,
connect, hello under its deadline, the verdicts, spawn or report, the
stop and the wait for the daemon to go) is a value: a `Door` machine
with an enum of states, events and effects and a pure `step`, pinned
by a table of event scripts in a synchronous test; the existing async
loop is its driver, performing effects and returning events, and the
only place an instant, a handle or a duration lives. The five-second
`c86_` wait is retired, held by the expiry row, the `c137_`
real-wiring test at zero (extended) and a wording test. No word a
user meets changes. This is C157's first machine (a lifecycle that
waits is a value, time an event) and proves the shape step 2, the
daemon's `Life`, copies. It is the smallest machine, in the crate
that sends nothing to GGG, so it may run during a league sitting.

## Read

- `roadmap/briefs/4-0-design.md` §2 to §4 (the shape; the `Door` row
  of the table) and `roadmap/TIME-SEAM.md` §1 to §2 (why). Where this
  brief is narrower than the design's §3 (the machine names timers,
  the driver holds their durations), this brief rules.
- `decisions/daemon.md`: C157 (a lifecycle that waits is a value),
  C137 (a deadline's contract is its expiry behaviour; the knobs),
  C86 (the hello deadline: unresponsive on every door, never
  absence), C10 (the single-version handshake: replaced, never
  negotiated with), C84 (the two identity values), C147 (a door takes
  the world it acts on; the environment read once, at the frontend's
  construction, which this step does not widen: the knobs stay read
  where they are read today). `decisions/network.md`: C88 (the real
  provider is the default; a human spawns).
- `AGENTS.md`: C122 (pinned once, at the lowest crate), C153 (a
  finding joins the surface that holds it), C154 (a retirement names
  its holder).
- `crates/acquisition-client/src/client.rs`: the module doc;
  `ConnectOptions` (256–283: `spawn`, `replace`; `interactive`,
  `autonomous`); `open_door` (1307–1545, the loop whose state is the
  locals `child`, `startup`, `respawned`, `stop_failed`,
  `looked_for_winner`, `found`, and whose inputs are `opts`,
  `no_spawn()` and `human_present()` read once at 1327–1329, the
  world at 1333); the three refusal reasons of a door that may not
  spawn (1512–1518) and of one that may not replace (1414–1420); the
  replace path (1436–1447: a failed stop kept as words, the child
  dropped, the 200 ms settle at 1439), the respawn even for a
  `spawn: false` caller (1444) and the stop sent before
  `StillAfterRespawn` (1388–1404, by pid); the startup deadline
  consulted only on a turn that found nothing (1504); the words
  "did not bind … within Ns" from the duration the child was given
  (1533). `observe_at` (1560) and `stop_any_at` (1597), the two other
  doors over the same reach and handshake; `stop_over` (1711).
  `stop`, `stop_within` and `stop_until` (1624–1700: the bound starts
  after `stopping` is read, 1690; the wait is the read to EOF then
  the endpoint looked at every `GONE_LOOK_EVERY`; the tie rule at
  1691, `biased`: the end and the absence seen at the poll the bound
  expires are `Gone`). `handshake` (1736: the one exchange under
  `timeout_at`, and a reply that lands at or after the deadline is
  late all the same, 1764). `Client::request` (1844–1875: a
  per-request deadline on an open connection and `Stalled`, **not
  this step's**). `exchange` (1913) and `frame.rs`'s `read_frame`
  (38): neither restart-safe, so an abandoned exchange abandons its
  connection. `winner_listens` and `look_until` (1939, 1968: the
  look's own bound, each reach under it, a found connection late if
  `now` has passed the deadline when it comes back, 1979).
  `reach` and `Reached` (1263–1272: `Listening` carries the
  `Stream`, a handle). `Startup` (1242–1259: the duration and the
  instant, both the driver's after this step). `Verdict::of` and
  `DaemonId::judged` (778, 819): they open the sibling executable and
  read the wanted provider (`sibling()`, `want_provider()`;
  `artifact.rs:182` hashes), so they are the driver's, never the
  machine's. `Spawned`, `exited` and `spawn_daemon` (2002–2045);
  `lock_holder` (530); the knobs `hello_within`, `startup_within`
  and `parse_within` (335–352, 1232); `STOPPED_WITHIN` and
  `GONE_LOOK_EVERY` (367, 373); `Unresponsive` and its Display
  (380–427), whose words stay. The in-crate tests under `mod tests`
  that drive the seams this step removes: `stop_within` and
  `stop_until` with an injected bound and look
  (`a_stop_acknowledged_by_a_daemon_that_lingers_is_not_a_daemon_gone`,
  2235; `a_stop_is_gone_only_once_the_endpoint_it_connected_to_is`,
  2271, with the tie); `look_until` with an injected `now`
  (`c147_a_look_whose_connect_succeeds_after_its_deadline_finds_no_winner`,
  2337); `open_door` with a child and a deadline
  (`c147_a_winner_found_on_the_last_poll_is_handshaken`, 2373;
  `c147_a_replacement_spawned_after_the_budget_ran_out_gets_its_own`,
  2449). `src/endpoint.rs`: `connect` and `exists` (28–66), the
  Windows pipe-busy retry at 40–50 with its counter, which a restarted
  connect would reset: a driver concern, inside one `Connect`.
- `crates/acquisition-client/tests/contract.rs`: `c86_…` (326, the
  5 s wait: the holder files written at 331–335, `elapsed >= 5s` at
  344–347, the socket and holder asserted at 357–358 and 378, the
  words at 360–375, the peer still listening at 389–393); `c137_…`
  (407, the handshake deadline at zero, which this step extends; 484,
  the observe request at zero, `Client::request`'s, untouched; 1001,
  the startup deadline at zero); `c147_…` (916, with its 2 s sleep at
  936; 969; 1042; 1096). `tests/harness/mod.rs`: the isolation list
  (163–186: the mock provider, the scratch store and the three knobs
  at 60), `set_knob` (951), `wait_exit` (335) and the 120 s ceiling
  (442, 969, 1016).
- `crates/acquisition-cli/tests/watch_recovery.rs`: the one frontend
  test naming `Unresponsive`; it reads the same after.
- `crates/acquisition-client/Cargo.toml`: the crate links protocol,
  store, tokio, serde, serde_json, anyhow and sha2; no new
  dependency, and no `test-util` (the template's rule).

## Input

Measured on `main` at `6893fc74` by the design's read and the cold
review's, not by a run: the real-time sites in `client.rs` are the
`Startup` instant (1252, 1258), the settle and bind-poll sleeps (1439,
1507), the stop's bound and gone-poll (1631, 1686), the handshake's
deadline (1756–1764), the request's deadline (1852–1855, not this
step's), the look's deadline and pause (1945–1982), and the test
module's (2171, 2338, 2348). `c86_` spends about five real seconds;
the `c137_` tests spend none. The last push's CI, read at the design:
`Gate (Linux)` 41209304807 and `Gate (Windows)` 41209304795 on
`6893fc74`, both success. Re-read at the step's start; a red run is
this step's first concern or is filed.

## Do

1. **The machine**, in a new module
   `crates/acquisition-client/src/door.rs`, its doc headed by C157.
   The boundary: the machine holds data only; every handle (the
   `Stream`, the `Spawned`, the `DaemonId` and its `Verdict`), every
   instant and every duration is the driver's. What the machine sees
   of a peer is the driver's judgement, delivered once per handshake
   as event data.

   Construction inputs, read once at the door's start as the loop
   reads them today (1327–1333; C147 is not widened): the door,
   `enum Door { Use { spawn: bool, replace: bool }, Observe, Stop }`,
   and the policy environment, `no_spawn: bool` and `human: bool`.
   The machine derives the effective `spawn` and `replace` and the
   three refusal reasons of each (`NotSpawned::{Policy, NoSpawnEnv,
   NoTerminal}`, `NotReplaced::{NeverReplaces, NoSpawn,
   NoTerminal}`) exactly as 1512–1518 and 1414–1420 do.

   State: `enum State { Reaching, AwaitingHello, Starting,
   LookingForWinner, Settling, Stopping, Done }`, each with the data
   it needs, and these fields across them, one per local of the loop:
   `child: Option<u32>` (the pid of the child this door spawned; the
   `Spawned` stays in the driver), `startup_expired: bool` (set by
   the startup deadline's event, read only on a turn that found
   nothing, as 1504), `respawned: bool`, `stop_failed:
   Option<String>` (the stop's error as words, kept until the next
   look as 1436, cleared at 1447), `looked_for_winner: bool`, `found: bool`
   (the connection in hand is a winner the look found, never judged
   against the startup deadline, 1346–1350).

   Event, one per observation the driver returns: `Reached`,
   `Nothing`, `ConnectFailed(String)`; `Identified { ours: bool,
   same_world: bool, pid: u32 }` (the driver's `DaemonId::judged`,
   once per handshake); `Unresponsive` (the hello deadline, after the
   driver's normalisation in Do 3); `HelloFailed(String)`;
   `Spawned { pid: u32 }`, `SpawnFailed(String)`; `ChildRunning`,
   `ChildExited { holder: Option<u32> }` (the exit seen and the
   lock's recorded pid read at that moment, as 1469–1470; the machine
   filters the child's own pid as 1470 does); `StopAck`,
   `StopRefused(String)`; `Closed` (the connection read to its end);
   `EndpointGone`, `EndpointPresent`; `Deadline(Timer)`.

   Effect, one per operation the driver performs: `Connect`,
   `SendHello`, `Spawn`, `PollChild`, `DropChild` (reap now, as
   `child = None` does at 1438 and 1482), `SendStop`, `ReadToEnd`,
   `LookEndpoint`, `Arm(Timer)`, `Cancel(Timer)`, `Report(Outcome)`.

   `enum Timer { Hello, Startup, Look, Stop, BindPoll, GonePoll,
   Settle }`: the machine names a timer; its duration is the driver's
   (Do 3). `Outcome` is the door's answer as data, one variant per
   arm of `ConnectError`, `Observed` and the stop's result, carrying
   only what the machine knows (the refusal reason, the stop's words,
   `started_here: bool`, `Stopped::{Gone, Acknowledged}`); the driver
   builds the typed value from it and from what it holds (the
   `DaemonId`, the `Spawned`'s executable and log excerpt, the lock
   holder it reads, the durations it armed, the socket). The words of
   every report are built where they are built today.

   `step(&mut self, event) -> Vec<Effect>`, pure: no `async`, no
   `tokio`, no `Instant`, no `Duration`, no I/O type in the module.
   The transitions are the loop's, turn for turn: a `Nothing` turn
   with a child polls it first (`PollChild`); `ChildExited` with a
   holder not the child's, on the first such turn, drops the child,
   cancels `Startup`, arms `Look` and connects; `ChildRunning` with
   `startup_expired` reports not bound, else arms `BindPoll`; a
   replace sends the stop, drops the child, arms `Settle`, and the
   next turn reaches again with `respawned` set; `Identified` with
   `found` set is reported as found, not started here; the stop door
   and the replace path share `Stopping`.

2. **The runner**: `drive(machine, script)` returning the effects,
   about ten lines in the client's test support, with a table of
   rows: each a name, the construction inputs, a script of events and
   the expected effects. Rows cover at least: hello answered and
   ours, on each door; the hello deadline on each door (the expiry
   row: `Report(Unresponsive)`, no `Spawn`, no `SendStop`); another
   world; each mismatch verdict under `replace` and under `!replace`,
   each refusal reason; `Nothing` on a door that may not spawn, each
   refusal reason; `Nothing` with spawn allowed, `Spawned`, then
   `ChildRunning` with the startup deadline seen (not bound) and not
   seen (the bind poll); `ChildExited` with a lock holder, then the
   winner found and `Identified` as found, and not found by the
   look's deadline; `ChildExited` with no holder, or on the second
   such turn (the spawn failed); the stop door: `StopAck`, `Closed`,
   `EndpointPresent`, the gone poll, `EndpointGone` (gone); the same
   with `Deadline(Stop)` after `EndpointPresent` (acknowledged); the
   tie row: `StopAck`, `Closed`, `EndpointGone` with `Deadline(Stop)`
   delivered after them (gone, because the driver delivers the
   observation first, Do 3); `StopRefused` on the replace path kept
   and reported when the next look finds the daemon (stop failed),
   and cleared when it finds nothing (the respawn goes ahead); a
   second mismatch after a respawn, the child stopped by pid and not
   when another process started it; a respawn for a `spawn: false`
   caller. Each row's expected effects are written from the rulings
   and the loop's comments, never from running the driver (an oracle,
   `GLOSSARY.md`).

3. **The driver**: `open_door`, `observe_at`, `stop_any_at` and the
   stop conversation (`stop`, `stop_within`, `stop_until`,
   `stop_over`) rewritten as one loop over `step`, each effect
   performed by the existing functions (`reach`, `handshake`'s
   exchange and `DaemonId::judged`, `spawn_daemon`,
   `Spawned::exited` with `lock_holder`, the stop exchange, the read
   to EOF, `endpoint::exists`). The driver owns:

   - **The durations**, in one table, `fn duration(timer) -> Duration`,
     read when the timer is armed, where the knobs are read today:
     `Hello` and `Look` from `hello_within()` (the look's bound is
     the handshake's knob, 1476), `Startup` from `startup_within()`
     at each spawn (C137's "read at each spawn"), `Stop` from
     `STOPPED_WITHIN`, `GonePoll` from `GONE_LOOK_EVERY`, `BindPoll`
     50 ms and `Settle` 200 ms. The duration armed is kept beside the
     sleep, so a report names it ("within Ns", 1533 and 1485, the
     `Unresponsive`'s `within`).
   - **The instants**: each `Arm` is a `tokio::time::sleep_until`
     held by the driver until its `Deadline` is delivered or it is
     cancelled; `Startup`'s instant replaces the `Startup` struct.
   - **Which deadline may abandon which operation**, the only races
     the driver runs, each as the code bounds it today: `Hello`
     abandons the hello exchange (1757), and the connection with it;
     `Look` abandons a look's connect and its pause (1978–1982);
     `Stop` abandons the read to EOF and the gone pause (1691–1695);
     `BindPoll`, `GonePoll` and `Settle` are the operation. `Startup`
     abandons nothing: it is delivered as a flag. Every other pending
     operation runs to completion whatever timer fires: the ordinary
     reach (unbounded today), the stop exchange (unbounded today),
     the spawn, the child poll. A timer that is not racing an
     operation and has passed is delivered, oldest first, before the
     next effect is performed, so the machine's flag is current on
     the turn that reads it. The Windows pipe-busy retry stays inside
     one `Connect`, never restarted by a timer.
   - **Completion normalisation**, the rule that keeps a late success
     refused: a hello reply or a look's connect that completes when
     the deadline it ran under has passed is delivered as that
     deadline's event (`Unresponsive`; `Deadline(Look)`), as 1764 and
     1979 rule today; the stop's end and absence seen at the poll the
     bound expires are delivered as `Closed` and `EndpointGone`
     before `Deadline(Stop)`, as 1691's `biased` rules. The rule is
     one pure function over a completion, a deadline and a now, in
     the driver, with a value test of its own (an instant is a value
     there; nothing sleeps).
   - **Timer lifetimes across states**: `Hello` is armed at
     `SendHello` and ends with the exchange; `Startup` is armed at
     `Spawned` and cancelled only by the machine's `Cancel` (the
     child dropped: a replace, a winner look), a replacement spawn
     arming it afresh (1458–1460); `Look` is armed at the look's
     start, whole, and ends with it; `Stop` is armed at `StopAck`,
     after the acknowledgement (1652).

   `look_until`'s injected `now`, `stop_until`'s injected bound and
   look, and `open_door`'s child-and-deadline seam go: each was a
   seam for forcing an interleaving, and the rows force it now. The
   in-crate tests that drove them (Read) become rows and the
   normalisation's value test, each retired with its C154 line, or
   stay where a row cannot hold what they pin (the worker names
   which, and why).

4. **The real-wiring tests**: one per effect kind the driver performs,
   under the harness's 120 s ceiling, asserting no duration. The
   `c137_` tests at 407 and 1001 are these tests and stay, 407
   extended (Do 5); 484 is `Client::request`'s and is untouched. The
   2 s sleep in `c147_a_spawn_race_is_won_however_late_the_winner_starts`
   (936) goes: the row "a winner found after the look's pause" holds
   that lateness, and the test waits on its states alone (the log and
   the reap, as it already does), asserting the outcome only.

5. **Retire `c86_…`** (326). Its holders, each named in the commit's
   C154 line: the expiry row (the verdict on each door, no `Spawn`,
   no `SendStop`, the holder named in the outcome);
   `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
   extended with the holder files `c86_` writes (331–335, both
   platforms' files, `424242`) and its assertions on each door: the
   observed `at.socket` is the session's, the holder is
   `Some(424242)` on observe, on use and in the stop's error (357–358,
   378, 383); the default duration, `hello_within()` with the knob
   unset equal to five seconds, asserted in the same test before it
   sets zero (the deliberate replacement of `elapsed >= 5s`: C137,
   the contract is the arming and the expiry, not the wait); and a
   wording test over `Unresponsive`'s Display that takes `within` and
   `holder` as values and asserts `did not answer hello within 5s`,
   the platform's kill line with `424242`, and the Keychain clause on
   macOS only (360–375). The continued listening after the verdict
   (389–393) is held by the same `c137_` test's later successful
   observation of its peer. The commit carries one line, `retired
   c86_…; held by <row>, <test as extended> and <wording test>`.

6. `tools/gate.sh` once, on the final tree; the client's goldens and
   the CLI's `watch_recovery.rs` unchanged.

## Done when

- `crates/acquisition-client/src/door.rs` exists and
  `grep -n "tokio\|Instant\|Duration\|async\|std::io\|std::fs\|std::process" crates/acquisition-client/src/door.rs`
  finds nothing, quoted in the commit.
- The table has one row per transition Do 2 names, each row's name
  in the file; the commit message lists the row names.
- Every real-time site in `client.rs` outside `#[cfg(test)]` is in
  the driver's duration table or is one of the two public constants:
  `grep -n "timeout_at\|from_millis\|from_secs\|Instant::now\|sleep" crates/acquisition-client/src/client.rs`
  quoted in the commit, each line either in `duration`, in
  `Client::request` (1844–1875, which stays as it is), the
  declaration of `STOPPED_WITHIN` or `GONE_LOOK_EVERY`, the
  normalisation function, or under `mod tests`.
- `c86_a_peer_that_never_answers_hello_is_unresponsive_on_every_door_and_never_replaced`
  is gone, the commit message carrying its `retired X; held by Y`
  line naming the three holders of Do 5, and no test under
  `crates/acquisition-client/tests` asserts `elapsed` against a
  bound.
- `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
  writes the holder files and asserts the socket and `424242` on each
  door, and asserts the default duration.
- `crates/acquisition-cli/tests/watch_recovery.rs` and every golden
  under `crates/*/tests/goldens` are byte-identical to the step's
  parent commit.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- `Client::request`, its per-request deadline and `Stalled`
  (1844–1875): a lifecycle of an open connection after the door, with
  its own pin (`c137_…_an_observe_request_…`, 484). Untouched.
- The daemon's `Life`, its constructor and `Clock::sleep_until`:
  step 2 of #147, its own brief after this closes.
- The parking clock and t5 (#141): step 3's spike, then #141.
- A shared clock trait or crate: none (`4-0-design.md` §5); the
  driver uses tokio time directly.
- The login callback window, the verbs' job-wait poll and the CLI's
  login poll: `login.rs`'s and 4-full's, not this step's.
- Any word a user meets: the reports' Display impls stay as they are.
- Where the knobs are read: as today, at each arming (C147 is not
  widened to a lifetime-wide snapshot).
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

`TEMPLATE.md`, and the table's run time, as a number the report
carries and nothing asserts (C137).

## Taken from the review

The seven findings of `roadmap/reviews/4-0-door-brief-e4e6b305.md`:

1. The verdict functions do I/O: the driver judges, once per
   handshake, and delivers `Identified`; `step` applies the door's
   policy to data (Do 1).
2. `Client::request` is a separate lifecycle: excluded (Not in
   scope), its test untouched, done-when 3 scoped to the door.
3. A late success must stay refused and the stop's tie goes the other
   way: the driver's completion normalisation, one pure function with
   a value test, and the tie row (Do 3, Do 2).
4. The enums missed inputs: the door, the policy environment, the
   refusal reasons, the six locals as fields, `Spawned`/`SpawnFailed`,
   `PollChild`, `Closed`, `EndpointPresent`, handles kept in the
   driver, the durations the driver's with their words (Do 1, Do 3).
5. Timer lifetimes: ownership per timer, which deadline abandons
   which operation, `Startup` as a flag that abandons nothing, the
   settle and the pipe-busy retry kept, the blanket cancel-on-leaving
   rule dropped (Do 3).
6. Retirement holders: the zero test extended with the holder files
   and the socket and holder assertions on each door, the default
   duration asserted as the elapsed bound's replacement, the wording
   test in the C154 line (Do 5, done-when 4 and 5).
7. Report-only evidence and the overbroad grep: the run time moved
   to Report, the row names in the commit, the grep scoped with its
   exceptions named (done-when 2 and 3).
