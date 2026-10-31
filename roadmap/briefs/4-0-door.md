# 4-0-door — the client's door as a machine (#147, step 1)

Cut four times: the first draft (`e4e6b305`), the re-cut
(`e8306c53`) and the third cut (`19f771d4`), each reviewed cold
(`roadmap/reviews/4-0-door-brief-<sha>.md`, the prompt beside); this
cut answers the third review's five findings, the first of them by
the owner's ruling recorded in `4-0-design.md` §4. What each finding
changed is listed at the foot, "Taken from the reviews".

## Goal

When this step is done, the client's door lifecycle (discovery,
connect, hello under its deadline, the verdicts, spawn or report, the
stop sent to replace) is a value: a `Door` machine with an enum of
states, events and effects and a pure `step`, pinned by a table of
event scripts in a synchronous test; the existing async loop is its
driver, performing effects and returning events, and the only place
an instant, a handle or a duration lives. The five-second `c86_` wait
is retired, held by the expiry row, the `c137_` real-wiring test at
zero (extended) and a wording test. No word a user meets changes.

The criterion that draws the machine's edge, the owner's ruling of
2026-10-31 (`4-0-design.md` §4, the dated line): **a wait enters the
machine when its ordering against another event is a decision of the
door, or when its only pin asserts elapsed time; a wait whose
interleavings are already forced without relying on elapsed time,
and whose
outcome the door takes as one value, stays a driver effect with its
bound inside.** The hello's deadline orders against the reply (the
loser is the verdict) and its one pin cost five seconds; the startup
deadline orders against the child's exit (look, or not bound); the
bind poll and the settle are pauses the machine orders. The stop's
wait for the daemon to go (gone or acknowledged, its tie forced by
injected futures) and the look for a winner (found, not found or
failed, its late connect forced on an injected instant) are effects:
their pins force expiry and interleaving without relying on elapsed
time (one waits through a real poll sleep, asserting nothing of it)
and the door takes each as one value.
This is C157's first machine (a lifecycle that waits is a value, time
an event) and proves the shape step 2, the daemon's `Life`, copies.
The criterion is the door's; step 2's brief answers for each of the
daemon's polls whether it holds there, since an idle poll today has
no competing future and the criterion must not hide the lifecycle
step 2 exists to expose (the third review). It is the smallest
machine, in the crate that sends nothing to GGG, so it may run during
a league sitting.

## Read

- `roadmap/briefs/4-0-design.md` §2 to §4 (the shape; the `Door` row
  of the table) and `roadmap/TIME-SEAM.md` §1 to §2 (why). Where this
  brief is narrower than the design's §3 and §4 (the machine names
  timers and the driver holds their durations; the stop's wait and
  the winner look stay effects; the startup deadline is consulted,
  not raced), the design's dated line under §4 records the owner's
  ruling of 2026-10-31, and this brief builds it.
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
  `looked_for_winner`, `found`, and whose policy inputs are `opts`,
  `no_spawn()` and `human_present()`: `no_spawn()` is read at 1328,
  1329, 1417 and 1514 today, `human_present()` once at 1327, the
  world at 1333); the three refusal reasons of a door that may not
  spawn (1512–1518) and of one that may not replace (1414–1420); the
  replace path (1436–1447: a failed stop kept as words, the child
  dropped, the 200 ms settle at 1439), the respawn even for a
  `spawn: false` caller (1444) and the stop sent before
  `StillAfterRespawn` (1388–1404, by pid, its outcome kept as
  `stopped`); the child polled on a turn that found nothing (1469),
  the lock holder read at that moment and the child's own pid
  filtered (1470), the one look for a winner (1472–1489) with the
  child dropped only when the look found one (1482) and the child's
  executable, exit status and log excerpt reported when it did not
  (1491–1497); the startup deadline consulted at one point of such a
  turn, after the poll or the spawn (1504); the words "did not bind …
  within Ns" from the duration the child was given (1533). The lock
  holder read by each door at its own point, kept: the use door's and
  the observe door's at the unresponsive verdict (1367, 1573), the
  stop door's before the handshake (1605). `observe_at` (1560) and
  `stop_any_at` (1597), the two other doors over the same reach and
  handshake; `stop_over` (1711). `stop`, `stop_within` and
  `stop_until` (1624–1700), **the stop's wait, kept as it stands**:
  the bound starts after `stopping` is read (1690), the read to EOF
  with its four kinds of end (1673–1682) and any other failure
  acknowledged without a look (1683), the endpoint looked at every
  `GONE_LOOK_EVERY`, the tie at 1691, `biased`. `handshake` (1736:
  the one exchange under `timeout_at`, and a reply that lands at or
  after the deadline is late all the same, 1764). `Client::request`
  (1844–1875: a per-request deadline on an open connection and
  `Stalled`, **not this step's**). `exchange` (1913) and `frame.rs`'s
  `read_frame` (38): neither restart-safe, so an abandoned exchange
  abandons its connection. `winner_listens` and `look_until` (1939,
  1968), **the look, kept as it stands**: its own bound, each reach
  under it, a found connection late if `now` has passed the deadline
  when it comes back (1979). `reach` and `Reached` (1263–1272:
  `Listening` carries the `Stream`, a handle). `Startup` (1242–1259:
  the duration and the instant, both the driver's after this step).
  `Verdict::of` and `DaemonId::judged` (778, 819): they open the
  sibling executable and read the wanted provider (`sibling()`,
  `want_provider()`; `artifact.rs:182` hashes), so they are the
  driver's, never the machine's. `Spawned`, `exited` and
  `spawn_daemon` (2002–2045); `lock_holder` (530); the knobs
  `hello_within`, `startup_within` and `parse_within` (335–352, 1232)
  and the constants `HELLO_WITHIN` and `STARTUP_WITHIN` (327, 1224);
  `STOPPED_WITHIN` and `GONE_LOOK_EVERY` (367, 373); `Unresponsive`
  and its Display (380–427), whose words stay. The in-crate tests
  under `mod tests`: the stop wait's
  (`a_stop_acknowledged_by_a_daemon_that_lingers_is_not_a_daemon_gone`,
  2235, with its `StillAfterRespawn` words;
  `a_stop_is_gone_only_once_the_endpoint_it_connected_to_is`, 2271,
  with the tie and the endpoint it connected to, 2276) and the look's
  (`c147_a_look_whose_connect_succeeds_after_its_deadline_finds_no_winner`,
  2337), **untouched**; `open_door`'s child-and-deadline seam
  (`c147_a_winner_found_on_the_last_poll_is_handshaken`, 2373;
  `c147_a_replacement_spawned_after_the_budget_ran_out_gets_its_own`,
  2449), kept as wiring tests through the driver's seam (Do 3), their
  decisions also rows. `src/endpoint.rs`: `connect` and `exists`
  (28–66), the Windows pipe-busy retry at 40–50 with its counter,
  which a restarted connect would reset: a driver concern, inside
  one `Connect`.
- `crates/acquisition-client/tests/contract.rs`: `c86_…` (326, the
  5 s wait: the holder files written at 331–335, `elapsed >= 5s` at
  344–347, the socket and holder asserted at 357–358 and 378, the
  words at 360–375, the peer still listening at 389–393); `c137_…`
  (407, the handshake deadline at zero, which this step extends; 484,
  the observe request at zero, `Client::request`'s, untouched; 1001,
  the startup deadline at zero, which expects "did not bind … within
  0s" at the first look after the spawn, with no further connect);
  `c147_…` (916, with its 2 s sleep at 936; 969; 1042; 1096).
  `tests/harness/mod.rs`: the isolation list (163–186: the mock
  provider, the scratch store and the three knobs at 180–182), `set_knob`
  (951), `wait_exit` (335) and the 120 s ceiling (442, 969, 1016).
- `crates/acquisition-cli/tests/watch_recovery.rs`: the one frontend
  test naming `Unresponsive`; it reads the same after.
- `crates/acquisition-client/Cargo.toml`: the crate links protocol,
  store, tokio, serde, serde_json, anyhow and sha2; no new
  dependency, and no `test-util` (the template's rule).

## Input

Measured on `main` at `6893fc74` by the design's read and the two
reviews', not by a run: the real-time sites in `client.rs` are the
`Startup` instant (1252, 1258), the settle and bind-poll sleeps (1439,
1507), the stop's bound and gone-poll (1631, 1686), the handshake's
deadline (1756–1764), the request's deadline (1852–1855, not this
step's), the look's deadline and pause (1945–1982), the duration
constants and their parser (327, 358, 1224), and the test module's
(2171, 2338, 2348). Of these, this step moves the first, the second
and the handshake's into the driver's table and races; the rest stay
where they are. `c86_` spends about five real seconds; the `c137_`
tests spend none. The last push's CI, read at the design: `Gate
(Linux)` 41209304807 and `Gate (Windows)` 41209304795 on `6893fc74`,
both success. Re-read at the step's start; a red run is this step's
first concern or is filed.

## Do

1. **The machine**, in a new module
   `crates/acquisition-client/src/door.rs`, its doc headed by C157 and
   carrying the Goal's criterion. The boundary: the machine holds data
   only; every handle (the `Stream`, the `Spawned`, the `DaemonId` and
   its `Verdict`), every instant and every duration is the driver's.
   What the machine sees of a peer is the driver's judgement,
   delivered once per handshake as event data.

   Construction inputs, read once at the door's start: the door,
   `enum Door { Use { spawn: bool, replace: bool }, Observe, Stop }`,
   the policy environment, `no_spawn: bool` and `human: bool`, and
   `child: Option<u32>`, a child this door already spawned (the
   driver's seam, `open_door`'s third argument at 1310: the driver
   holds that child and its deadline; the machine starts with the
   pid, no `Spawned` and no fresh arming, its first effect
   `Connect`).
   This consolidates the four reads of `no_spawn()` a use door makes
   today (1328, 1329, 1417, 1514) into one, deliberately: a change to
   `ACQ_NO_SPAWN` during a door is not seen, where before it could
   change only the refusal's reason mid-door. The machine derives the
   effective `spawn` and `replace` and the three refusal reasons of
   each (`NotSpawned::{Policy, NoSpawnEnv, NoTerminal}`,
   `NotReplaced::{NeverReplaces, NoSpawn, NoTerminal}`) exactly as
   1512–1518 and 1414–1420 do.

   State: `enum State { Reaching, AwaitingHello, Starting, Settling,
   Done }`, each with the data it needs, and these fields across them,
   one per local of the loop: `child: Option<u32>` (the pid of the
   child this door spawned; the `Spawned` stays in the driver),
   `respawned: bool`, `stop_failed: Option<String>` (the stop's error
   as words, kept until the next look as 1436, cleared at 1447),
   `looked_for_winner: bool`, `found: bool` (the connection in hand
   is a winner the look found, never judged against the startup
   deadline, 1346–1350). `startup` has no field: it is the driver's
   deadline, consulted (below).

   Event, one per observation the driver returns: `Reached`,
   `Nothing`, `ConnectFailed(String)`; `Identified { ours: bool,
   same_world: bool, pid: u32 }` (the driver's `DaemonId::judged`,
   once per handshake: `is_ours`, `world_matches`, `pid`);
   `Unresponsive` (the hello deadline, after the driver's
   normalisation in Do 3); `HelloFailed(String)`; `Spawned { pid:
   u32 }`, `SpawnFailed(String)`; `ChildRunning`, `ChildExited {
   holder: Option<u32> }` (the exit seen and the lock's recorded pid
   read at that moment, as 1469–1470; the machine filters the child's
   own pid as 1470 does; the driver keeps the exit status, the
   executable and the log offset for the words at 1491–1497);
   `Deadline(Timer)` and `Pending(Timer::Startup)` (the answers to
   `Consult`); `Found`, `NotFound`, `LookFailed(String)` (the look's three
   outcomes, `winner_listens`'s `Ok(Some)`, `Ok(None)` and `Err`,
   1942–1951; `Found` leaves the connection in the driver's hand; the
   driver keeps the typed error for the words); `Stopped(Stopped)`
   (`Gone` or `Acknowledged`, the stop wait's value, 1624) and
   `StopFailed(String)` (the stop refused, or its conversation
   failed, as `stop`'s error).

   Effect, one per operation the driver performs: `Connect`,
   `SendHello`, `Spawn`, `PollChild`, `Consult(Timer::Startup)`,
   `DropChild` (reap now, as `child = None` does at 1438 and 1482),
   `LookForWinner` (the whole look, `winner_listens` as it stands),
   `Stop` (the whole conversation, `stop` as it stands: send, read
   the acknowledgement, wait to go), `Arm(Timer)`, `Cancel(Timer)`,
   `Report(Outcome)`.

   `enum Timer { Hello, Startup, BindPoll, Settle }`: the machine
   names a timer; its duration is the driver's (Do 3). `Outcome` is
   the door's answer as data, one variant per arm of `ConnectError`,
   `Observed` and the stop door's result, carrying only what the
   machine knows (the refusal reason, the stop's words or value for
   `StillAfterRespawn { stopped }`, `started_here: bool`, whether the
   spawn failed by exit, by a lost look or by not binding); the driver
   builds the typed value from it and from what it holds (the
   `DaemonId`, the `Spawned`'s executable, status and log excerpt, the
   lock holder it reads at each door's own point, the durations it
   armed, the socket). The words of every report are built where they
   are built today.

   `step(&mut self, event) -> Vec<Effect>`, pure: no `async`, no
   `tokio`, no `Instant`, no `Duration`, no I/O type in the module.
   The transitions are the loop's, turn for turn. A turn that found
   nothing, with spawn allowed: no child, `Spawn`; `Spawned` arms
   `Startup` and consults it at once (1456–1460 then 1504);
   a child, `PollChild`; `ChildExited` with a holder not the child's,
   on the first such turn, `LookForWinner` (the child kept, `Startup`
   cancelled); `Found` drops the child and sends hello with `found`
   set; `NotFound` reports the spawn failed by a lost look; `LookFailed`
   reports the transport failure, as `?` does at 1477;
   `ChildExited` otherwise reports the spawn failed by exit;
   `ChildRunning` consults `Startup`; `Deadline(Startup)` reports not
   bound, with no further `Connect` or `PollChild`;
   `Pending(Startup)` arms `BindPoll`; `Deadline(BindPoll)` connects.
   A mismatch under `replace`: `Stop`, then whatever its event
   (`Stopped` or `StopFailed`, the words kept), `DropChild`, `Arm
   (Settle)`; `Deadline(Settle)` connects with `respawned` set. A
   second mismatch after a respawn: `Stop` when the daemon is this
   door's child by pid, its event carried into `StillAfterRespawn`;
   the stop door: `Identified`, then `Stop`, its event the report.

2. **The runner**: `drive(machine, script)` returning the effects,
   about ten lines in the client's test support, with a table of
   rows: each a name, the construction inputs, a script of events and
   the expected effects. Rows cover at least: hello answered and
   ours, on each door; the hello deadline on each door (the expiry
   row: `Report(Unresponsive)`, no `Spawn`, no `Stop`); another world;
   each mismatch verdict under `replace` and under `!replace`, each
   refusal reason; `Nothing` on a door that may not spawn, each
   refusal reason; `Nothing` with spawn allowed, `Spawned`,
   `Deadline(Startup)` (not bound at the first look, nothing more
   performed: the zero row); `Nothing`, `Spawned`, `Pending(Startup)`,
   `Deadline(BindPoll)`, `Nothing`, `ChildRunning`, `Deadline(Startup)`
   (not bound after a poll); the same with `Pending(Startup)` last
   (another bind poll); a machine built with a child, `Nothing`,
   `ChildRunning`, `Deadline(Startup)` (no `Spawn`, no fresh arming:
   the seam's row); `ChildExited` with a lock holder, `Found`,
   `Identified` as found and not started here; `ChildExited` with a
   lock holder, `NotFound` (lost); `ChildExited` with a lock holder,
   `LookFailed` (transport); the
   holder the child's own, or the second such turn (failed by exit);
   the stop door with `Stopped(Gone)` and with `Stopped(Acknowledged)`
   and with `StopFailed`; `StopFailed` on the replace path kept and
   reported when the next look finds the daemon (stop failed), and
   cleared when it finds nothing (the respawn goes ahead); a second
   mismatch after a respawn, the child stopped by pid with
   `Stopped(Acknowledged)` carried into `StillAfterRespawn`, and not
   stopped when another process started it; a respawn for a `spawn:
   false` caller. Each row's expected effects are written from the
   rulings and the loop's comments, never from running the driver (an
   oracle, `GLOSSARY.md`).

3. **The driver**: `open_door`, `observe_at` and `stop_any_at`
   rewritten as one loop over `step`, each effect performed by the
   existing functions (`reach`; the hello exchange and
   `DaemonId::judged`; `spawn_daemon`; `Spawned::exited` with
   `lock_holder`; `winner_listens`; `stop`; `endpoint::exists`). The
   driver owns:

   - **The durations**, in one table, `fn duration(timer) -> Duration`,
     read when the timer is armed, where the knobs are read today:
     `Hello` from `hello_within()`, `Startup` from `startup_within()`
     at each spawn (C137's "read at each spawn"), `BindPoll` 50 ms
     and `Settle` 200 ms. The duration armed is kept beside the
     deadline, so a report names it ("within Ns", 1533; the
     `Unresponsive`'s `within`). The stop wait's and the look's
     durations stay inside those effects (1631, 1686, 1945).
   - **The instants**: `Hello`, `BindPoll` and `Settle` are each a
     `tokio::time::sleep_until` held until its `Deadline` is delivered
     or the machine cancels it; `Startup` is an instant the driver
     holds from `Spawned` until `DropChild` or `Report`, answered on
     `Consult` with `Deadline(Startup)` when now is at or past it
     (at zero, at once) and `Pending(Startup)` otherwise; a
     replacement spawn holds a fresh one (1458–1460). No timer is
     delivered unasked except a racing one.
   - **The one race**: `Hello` abandons the hello exchange (1757), and
     the connection with it, exactly as `timeout_at` does today.
     `BindPoll` and `Settle` are the operation. Every other operation
     runs to completion whatever the clock says: the ordinary reach
     (unbounded today), the spawn, the child poll, and the two
     effects that carry their own bound inside (`Stop`,
     `LookForWinner`). The Windows pipe-busy retry stays inside one
     `Connect`, never restarted by a timer.
   - **Completion normalisation**: a hello reply that completes when
     its deadline has passed is delivered as `Unresponsive`, as 1764
     rules today: one pure function over a completion, a deadline and
     a now, with a value test of its own (an instant is a value
     there; nothing sleeps). The look's lateness (1979) and the stop's
     tie (1691) are their effects' own, pinned as they are (2337,
     2271).
   - **The holder**: read where each door reads it today (1367, 1573,
     1605) and placed in the typed value the driver builds; never an
     event.
   - **The seam**: `open_door`'s third argument (a child and its
     deadline, 1310) stays as the driver's: it holds the child and
     the deadline it was given and builds the machine with the
     child's pid (Do 1), so the two in-crate tests that drive it
     (2373, 2449) keep their wiring (the found stream handed to the
     hello; the replacement's fresh deadline and the daemon
     returned), their decisions also rows.

4. **The real-wiring tests**: one per effect kind the driver performs,
   under the harness's 120 s ceiling, asserting no duration. The
   `c137_` tests at 407 and 1001 are these tests and stay, 407
   extended (Do 5); 484 is `Client::request`'s and is untouched. The
   2 s sleep in `c147_a_spawn_race_is_won_however_late_the_winner_starts`
   (936) goes: the look outlasting a late winner is the look's own
   property (`look_until`, under its knob at its largest), and the
   test waits on its states alone (the log and the reap, as it already
   does), asserting the outcome only.

5. **Retire `c86_…`** (326). Its holders, each named in the commit's
   C154 line: the expiry row (the verdict on each door, no `Spawn`, no
   `Stop`);
   `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
   extended with the holder files `c86_` writes (331–335, both
   platforms' files, `424242`) and its assertions on each door: the
   observed `at.socket` is the session's, the holder is `Some(424242)`
   on observe, on use and in the stop's error (357–358, 378, 383); the
   default duration, `hello_within()` with the knob unset equal to
   five seconds, asserted in the same test before it sets zero (the
   deliberate replacement of `elapsed >= 5s`: C137, the contract is
   the arming and the expiry, not the wait); and a wording test over
   `Unresponsive`'s Display that takes `within` and `holder` as values
   and asserts `did not answer hello within 5s`, the platform's kill
   line with `424242`, and the Keychain clause on macOS only
   (360–375). The continued listening after the verdict (389–393) is
   held by the same `c137_` test's later successful observation of
   its peer. The commit carries one line, `retired c86_…; held by
   <row>, <test as extended> and <wording test>`.

6. `tools/gate.sh` once, on the final tree; the client's goldens and
   the CLI's `watch_recovery.rs` unchanged.

## Done when

- `crates/acquisition-client/src/door.rs` exists and
  `grep -n "tokio\|Instant\|Duration\|async\|std::io\|std::fs\|std::process" crates/acquisition-client/src/door.rs`
  finds nothing, quoted in the commit.
- The table has one row per transition Do 2 names, each row's name
  in the file; the commit message lists the row names.
- The real-time inventory of `client.rs`:
  `grep -n "timeout_at\|from_millis\|from_secs\|Instant::now\|sleep" crates/acquisition-client/src/client.rs`
  quoted in the commit with each line classified as one of: a
  duration constant or its parser (327, 358, 1224 today); the
  driver's `duration` table; the driver's arming, consulting and the
  one race; the normalisation function; the stop wait (1631, 1686
  today) or the look (1945–1982 today), unchanged; `Client::request`
  (1852–1855 today), unchanged; `mod tests`. A line that fits no
  class fails the step. The `Startup` struct (1242–1259) and the
  loop's two sleeps (1439, 1507) are gone as written, their lines in
  the table and the driver.
- `c86_a_peer_that_never_answers_hello_is_unresponsive_on_every_door_and_never_replaced`
  is gone, the commit message carrying its `retired X; held by Y`
  line naming the three holders of Do 5, and no test under
  `crates/acquisition-client/tests` asserts `elapsed` against a
  bound.
- `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
  writes the holder files and asserts the socket and `424242` on each
  door, and asserts the default duration.
- The five in-crate tests named in Read (2235, 2271, 2337, 2373,
  2449) are present by name on the step's tree.
- `crates/acquisition-cli/tests/watch_recovery.rs` and every golden
  under `crates/*/tests/goldens` are byte-identical to the step's
  parent commit.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- `Client::request`, its per-request deadline and `Stalled`
  (1844–1875): a lifecycle of an open connection after the door, with
  its own pin (`c137_…_an_observe_request_…`, 484). Untouched.
- The stop's wait and the winner look as machine timers: not by the
  Goal's criterion, not until a pin of theirs asserts elapsed time; their
  tie and lateness rules stay in `stop_until` and `look_until` with
  the tests that force them.
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
carries and nothing asserts (C137). Raised for review: the policy
snapshot (Do 1, the four reads of `no_spawn()` made one, so a
refusal's reason no longer changes mid-door) and what the criterion
says of each of the daemon's polls, for step 2's brief.

## Taken from the reviews

The first review's seven (`4-0-door-brief-e4e6b305.md`), as the
second review judged them, and what this cut changed where it found
them unanswered:

1. The verdict functions do I/O: the driver judges and delivers
   `Identified`; answered (narrower, sound).
2. `Client::request` excluded; answered.
3. A late success stays refused; the stop's tie: the hello's
   normalisation stays (Do 3); the stop's wait is now one effect with
   its tie inside, pinned as it stands (Goal's criterion).
4. The inputs and observations: the inventory stands; the stop's
   read failure is inside the `Stop` effect, as today; the child's
   status and log are the driver's until the look resolves (Do 1).
5. Timer lifetimes: one race (the hello), two pauses, and `Startup`
   consulted at the loop's one check point; nothing delivered
   unasked (Do 3).
6. Retirement holders: answered (narrower, sound); the expiry row no
   longer claims the holder (Do 5).
7. Report-only evidence and the grep: the grep is an inventory with
   its classes (done-when 3).

The second review's seven (`4-0-door-brief-e8306c53.md`):

1. Overdue timers delivered before the next effect read a stale flag
   after a poll and skipped the first look's check at zero: `Startup`
   is consulted, after the spawn and after the poll (Do 1, Do 3); the
   zero row is explicit (Do 2).
2. The split stop effects lost the biased tie: the stop's wait is one
   effect, `stop` as it stands (Goal, Do 1).
3. No event for a read failure after the acknowledgement: inside the
   `Stop` effect, acknowledged without a look as today (Read).
4. The child dropped before the look lost its diagnostics: dropped on
   `Found` only; the driver keeps status, executable and log (Do 1).
5. Done-when 3 excluded the driver's clock calls and the constants:
   an inventory with named classes (done-when 3).
6. The expiry row cannot name the holder: it does not; the holder is
   each door's read, in the typed value (Do 3, Do 5).
7. `no_spawn()` is read four times today: the single snapshot is a
   stated consolidation, raised for review (Do 1, Report).

The third review's five (`4-0-door-brief-19f771d4.md`):

1. The narrowing changed the ruled design before the owner recorded
   it, and prescribed the criterion to step 2: recorded as his ruling
   of 2026-10-31 in `4-0-design.md` §4, reworded by evidence (a pin
   that asserts elapsed time) rather than "one value"; step 2 answers it
   per poll (Goal, Read).
2. The look's transport error was a third outcome: `LookFailed`, with
   its row (Do 1, Do 2).
3. One script delivered a deadline unasked after `Pending`: split into
   two scripts (Do 2).
4. The seam's child had no way into the machine: `child` as a
   construction input, no `Spawned`, no fresh arming, with its row
   (Do 1, Do 2, Do 3).
5. The harness's knobs cited at 60: 180–182 (Read).
