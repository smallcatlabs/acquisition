# 4-0-life — the daemon's lifecycle as a machine (#147, step 2)

Written from three read-only reads of the tree at `5c9cd487` (the
lifecycle code, its tests, the clock), after step 1 closed
(`4-0-door.md`, `LEDGER.md`'s row of 2026-10-31). Where this brief
is narrower than the design, the dated paragraph under
`4-0-design.md` §4 records it, with the owner's grant of autonomy of
2026-10-31 and the tree's reason; the owner reads it at the close.

## Goal

When this step is done, the daemon's lifecycle (serving, the idle
verdict's deadline, the one exit's beginning by its first cause, the
drain against its bound, the second signal, the leave) is a value: a
`Life` machine with an enum of states, events and effects and a pure
`step`, pinned by a table of event scripts in a synchronous test; the
watchdog, the drain and the exit paths are one driver loop over it,
the only place an instant, a handle or a duration lives. The idle
daemon wakes at its deadline, not every five seconds, and the drain's
cut-or-clean is a row. The daemon's three elapsed-time upper bounds
(#131) go by the rule C137 names, and the idle exit gains the one
pin it never had: the knob at zero ends a real daemon. No word a user
meets changes, and no log line a test reads changes.

The criterion ruled for the door (`4-0-design.md` §4) is answered here
per wait, as the third brief review asked: the idle deadline enters
the machine because its expiry is the lifecycle's decision (exit) and
it had no pin at all; the drain's deadline enters because it orders
against the quiet (cut or clean), and its only pin asserted elapsed
time (`exit.rs:302`); the second signal orders against the drain and
enters. The quiet wait (`InFlight` empty), the verdict itself
(`is_idle`, pure and pinned by `idle.rs` on a manual clock), the exit
bound's OS thread (C133, real by design), the accept backoff and the
login's three bounds stay driver effects with their bounds inside:
each is one value to the lifecycle, and each has its pin.

## Read

- `roadmap/briefs/4-0-design.md` §2 to §4 (the shape; the
  "daemon's lifecycle" row and the dated paragraphs under §4) and §6
  item 2; `roadmap/TIME-SEAM.md` §4 item 2 (which counts `HALT_POLL`
  among the lifecycle's polls: it is the choke point's, `choke.rs:222`
  and `:386`, not this step's).
- `crates/acquisition-client/src/door.rs`: the module doc and the
  table test, the shape this step copies (the enum triple, `step`,
  `drive`, a row per transition, the driver's duration table).
- `decisions/daemon.md`: C157, C137 (the knob at zero forces expiry;
  "the idle exit's" is in its knob list), C133 (one exit, every cause
  reaches it; 15 s for the in-flight to land; the bound's thread),
  C134 (a listener failure never ends the process), C3 (exits when
  idle), C148 (the restore holds the verdict and the drain).
- `AGENTS.md`: C122, C153, C154.
- `crates/acquisition-daemon/src/daemon/server.rs`: `is_idle`
  (671–689: six clauses, `shared` locked at 674, the limiter at 688;
  pure over the state and `choke.now()`); `idle_watchdog` (696–704:
  the knob read once at 697, `sleep(IDLE_POLL)` at 699, on tokio time
  on purpose, 691–695, because the manual clock advances on every
  sleep); `exit` (528–534: begin, drain, leave; a second cause pends
  forever at 529–531); `exit_armed` (540–544, the signal's path);
  `begin_exit` (550–556) and `stop_admitting` (563–567: the rails'
  `begin_stopping`, the log line, the dispatcher woken); `drain`
  (575–605: the deadline read at 576–581, `try_lock` then
  `InFlight::of` at 584, "exit: nothing in flight" at 588, the cut
  from the last snapshot at 593–601, `sleep_until(deadline.min(now +
  DRAIN_POLL))` at 603); `leave` (616–662: the store's checkpoint
  under its lock at 621–643, then the jobs database's at 644–657, the
  socket at 658–660, `process::exit(0)` at 661); `InFlight::of`
  (743–787, six parts) and `is_empty` (789–796); the stop
  conversation at 189–191 (the connection's task calls `exit` and
  never returns); `refuse_while_stopping` (515–523) and `restored`
  (506); `has_live_jobs` (711–717); the connection counter and
  activity stamps (105–106, 246–247).
- `crates/acquisition-daemon/src/daemon.rs`: the constants
  `IDLE_SHUTDOWN` (169), `IDLE_POLL` (171), `EXIT_BOUND` (177),
  `DRAIN_POLL` (180), `LEAVE_GRACE` (186); `Exit` (194–208: `begun`
  outside every lock, `deadline` a `tokio::time::Instant`), `arm`
  (229–250: the swap at 230 is the first cause winning; the deadline
  at 233; the OS thread "acqd-exit-bound" at 236–246, `std::thread
  ::sleep` to bound plus grace then `end(&line, 1)`), `end_at_once`
  (255–260); `Daemon` (900–986, 23 fields; `exit` at 958, `restore`
  at 966); `mark_activity` (1146–1150, `fetch_max`); `landings`
  (1114, 1117).
- `crates/acquisition-daemon/src/daemon/start.rs`:
  `idle_shutdown_from_env` (14–21: an unparseable value is the
  default; zero parses to zero; nothing says zero forces the exit);
  the signal watcher (120–123, 142–163, on its own thread and
  runtime; the first signal at 209–229 arms and spawns `exit_armed`,
  or only logs if the exit had begun; the second at 230–236 ends at
  once with 128 + n; a signal before the daemon is built exits at
  217); `end_at_once` (243–255, writes through the log's clone,
  removes the socket, exits); `run`'s construction (462 the clock,
  474–476 rails and choke, 500–534 the `Daemon` literal, 537 the
  `end` installed, 538 the signals attached, 545 the panic hook's
  abort, 590 the knob logged, 605–614 the restore begun, the
  watchdog spawned at 612, `serve` at 614); `begin_restore`
  (627–629) and the restore's landing (660–661); `serve` and the
  accept backoff (699–724, `ACCEPT_PAUSE_FIRST` 686, cap 683).
- `crates/acquisition-daemon/src/rails.rs`: `halted()` is `Some`
  once stopping (541), `begin_stopping` (550–552).
- `crates/acquisition-daemon/src/ratelimit.rs`: the `Clock` trait
  (66–76: `now` is `std::time::Instant`, `sleep`, no `sleep_until`),
  `SystemClock` (78–96). `src/daemon/tests/harness.rs`: `ManualClock`
  (22–68: `sleep` advances the clock and yields, 62–66, so a driver
  on it would fire at once); `test_daemon` (413), `test_daemon_with`
  (498), the harness's `Daemon` literal (543–576), `wait_terminal`'s
  3 s (613). `src/daemon/tests/dispatch.rs`: `BlockingClock` (15–75,
  FCFS, its sleeper count never decremented at 65). Neither test
  clock can drive a deadline in order; that is step 3's spike.
- `crates/acquisition-daemon/src/daemon/tests/exit.rs`, whole:
  `BOUND` 3 s (18) and its uses (87, 118, 168, 319, 363, 456, 480,
  533); `exit_daemon_on` patching `jobs_db` and `exit` (36–37);
  `wait_state`'s 3 s over a 5 ms poll (60–62); `settle`, a 100 ms
  sleep (76–78); the three tests that assert an elapsed upper bound,
  #131's: `exit_cuts_a_stuck_job_at_the_bound` (281, `waited < 3s`
  at 302), `exit_turns_away_a_job_held_at_the_limiter` (449, on the
  system clock at 456, `waited < 2s` at 486),
  `the_bound_ends_the_exit_though_every_worker_is_blocked` (383, the
  holder's 2500 ms sleep at 404, `recv_timeout(2s)` at 420, `waited
  < 1500ms` at 424, code 1 at 431); the four that `settle` then
  assert the drain is not finished (85, 116, 503, 566; the asserts at
  94–98, 133–134, 546–549, 591–592); `a_submit_after_the_exit_begins
  _is_refused_as_stopping` (318, the words at 334–335);
  `the_idle_verdict_holds_while_a_login_is_pending` (346, sync, the
  manual clock at 350); `a_second_begin_exit_returns_at_once` (362,
  its 20 ms sleep at 366); `exit_turns_away_a_job_parked_past_running`
  (223, the 50 ms sleep at 193) and
  `a_turned_away_job_is_not_picked_again_while_exiting` (253); the
  C134 tests from 701, not this step's.
- `tests/idle.rs` (204 lines, no real time): the four verdict tests
  (10, 47, 121, 173), driven by `ManualClock::advance` (56, 59, 154,
  193); they stay as they are. `tests/credentials.rs`: `c148_…`
  (757: the verdict false and the drain pending while the restore is
  held, 782–790). `tests/sessions.rs`: the third `Daemon` literal
  (219–265).
- `crates/acquisition-cli/tests/daemon_signal_exit.rs` (Unix): the
  two SIGTERM tests (85, 147), which read the log's "SIGTERM", "job
  … -> done", "exit: nothing in flight", "a second signal" and
  "checkpointed on exit" lines and assert exit codes, no duration;
  `daemon_stop_checkpoint.rs` (29). `crates/acquisition-cli/tests
  /provider_default.rs`: the lazily started daemon's tests (142, with
  `took < 60s` at 168, a hang's bound; 185, with a 500 ms sleep at
  214; 232). `crates/acquisition-cli/tests/readme_tour.rs:324` (the
  knob's parse and report). `crates/acquisition-client/tests/harness
  /mod.rs:178` (`ACQ_IDLE_SHUTDOWN=30` on the isolation list) and
  `wait_exit` (335), the 120 s ceiling (442).
- `crates/acquisition-daemon/Cargo.toml`: no new dependency; no
  `test-util` (the gate's `features` stage, C84).

## Input

Measured on `main` at `5c9cd487` by the reads, not by a run. The
lifecycle's real-time sites outside `#[cfg(test)]` and outside the
limiter: `daemon.rs` 171 (`IDLE_POLL`), 177, 180, 186, 233, 236, 245;
`server.rs` 581, 593, 603, 699; `start.rs` 720 (the accept backoff);
the login's bounds `login.rs` 48, 434, 992; the mock kinds' `kinds.rs`
572–621. Of these this step moves 171, 233, 581–603 and 699 into the
driver's table and races; 236–245 stay the bound thread's; the rest
stay. No test observes a real idle exit. Three daemon tests assert an
elapsed upper bound (`exit.rs` 302, 486, 424). The last push's CI:
`Gate (Linux)` 41226030081 and `Gate (Windows)` 41226029943 on
`5c9cd487`, both success. Re-read at the step's start; a red run is
this step's first concern or is filed.

## Do

1. **The machine**, in a new module
   `crates/acquisition-daemon/src/daemon/life.rs`, its doc headed by
   C157 and carrying the criterion as this step applies it. The
   boundary: the machine holds data only; every handle (the daemon,
   `Exit`, the stop connection, the signal streams, the `InFlight`
   snapshot, the log), every instant and every duration is the
   driver's.

   Construction inputs: none but the machine's own initial state (the
   knob and the bound are the driver's durations).

   State: `enum State { Serving, Draining, Done }` and the fields
   `cause: Option<Cause>` (`enum Cause { Idle, Stop, Signal }`, the
   first cause, which wins: `arm`'s swap at `daemon.rs:230`) and
   `signals: u8` (how many signals the watcher has delivered: the
   first begins or is only logged, the second ends at once,
   `start.rs` 209–236).

   Event, one per observation the driver returns: `Deadline(Timer)`;
   `Idle`, `Busy` (the verdict's answers to `Judge`); `StopRequested`
   (`daemon_stop` read, `server.rs:189`); `Signal` (one signal, the
   driver keeping its name and number); `Quiet` (nothing in flight:
   `InFlight::is_empty`, `server.rs:789`).

   Effect, one per operation the driver performs: `Arm(Timer)`,
   `Cancel(Timer)`, `Judge` (the driver's `is_idle` with the knob,
   `server.rs:671`, answered `Idle` or `Busy`), `Begin(Cause)` (the
   driver's `Exit::arm` with the cause's words, lock-free, then
   `stop_admitting`: the rails' `begin_stopping`, the log line, the
   dispatcher woken, `server.rs:563–567`; the bound's OS thread is
   armed inside it as today, `daemon.rs:236–246`), `AwaitQuiet` (the
   driver's wait for `InFlight` empty: `try_lock` and look, a pause
   of `DRAIN_POLL` between looks, `server.rs:584–603`), `Cut` (the
   log line from the last snapshot the driver saw, `server.rs:593–601`),
   `Leave { checkpoint: bool, code: i32 }` (the driver's `leave` at
   `server.rs:616` when `checkpoint`, else `end_at_once` at
   `daemon.rs:255`, 128 + n for a second signal, `start.rs:230–236`).

   `enum Timer { Idle, Drain }`: the machine names a timer; its
   duration is the driver's (Do 3). The bound thread's deadline
   (bound plus grace) is not a machine timer: it is the driver's
   backstop, real by design (C133, `4-0-design.md` §3).

   `step(&mut self, event) -> Vec<Effect>`, pure: no `async`, no
   `tokio`, no `Instant`, no `Duration`, no I/O type, no `Arc`, no
   lock in the module. The transitions, turn for turn as the code
   decides today: in `Serving`, `Deadline(Idle)` judges; `Busy` arms
   `Idle` again; `Idle` begins with cause `Idle`; `StopRequested`
   begins with cause `Stop`; the first `Signal` begins with cause
   `Signal`; a begin is `Begin(cause)`, `Cancel(Idle)`, `Arm(Drain)`,
   `AwaitQuiet`, and the state is `Draining`. In `Draining`:
   `StopRequested` and a first `Signal` do nothing (the stop's
   connection pends, `server.rs:529–531`; the signal is only logged,
   `start.rs:225–229`, the driver's line); a second `Signal` is
   `Leave { checkpoint: false, code: 128 + n }` (the driver holds n);
   `Quiet` is `Cancel(Drain)`, `Leave { checkpoint: true, code: 0 }`;
   `Deadline(Drain)` is `Cut`, `Leave { checkpoint: true, code: 0 }`
   (the drain's return is discarded today, `server.rs:532`: cut or
   clean differ only in the log); `Deadline(Idle)`, `Idle` and `Busy`
   do nothing. In `Done`, every event does nothing. An event a state
   cannot receive is not a panic here: the driver's sources (the
   timers, the stop channel, the signals, the quiet wait) race, and a
   late one is dropped by the row that does nothing.

2. **The runner and the table**: `drive(machine, script)` as
   `door.rs`'s, in the daemon's test support, and a table of rows,
   each a name, a script and the expected effects, written from the
   rulings and the code's comments, never from running the driver.
   Rows cover at least: the idle deadline judged busy then armed
   again; judged idle, the exit begun by idle; a stop begins the exit
   (C133's refusal: `Begin` before anything else); a signal begins
   it; the second cause does nothing (stop after idle, stop after
   stop, signal after stop); the second signal leaves at once without
   a checkpoint; quiet before the drain's deadline leaves clean;
   the drain's deadline cuts then leaves; a late idle deadline during
   the drain does nothing; every event in `Done` does nothing. The
   table's run time goes in the report, asserted by nothing (C137).

3. **The driver**: `idle_watchdog`, `exit`, `exit_armed`, the stop
   path's call and the signal watcher's two arms rewritten as one
   loop over `step` in `server.rs` (or `life.rs`'s driver half; the
   worker chooses and says), the loop awaiting its sources (the armed
   timers, the stop requests, the signals, the quiet wait) and
   performing each effect with the functions that exist (`is_idle`,
   `Exit::arm`, `stop_admitting`, `InFlight::of`, `leave`,
   `end_at_once`). The signal watcher keeps its own thread and
   runtime (`start.rs:120–123`) and hands each signal to the loop as
   an event; `Exit::arm` keeps its lock-free swap, and `begun` stays
   the atomic `refuse_while_stopping` reads (`server.rs:515`), so a
   request is refused the moment the exit begins, before the loop's
   next turn. The driver owns:

   - **The durations**, one table `fn duration(timer) -> Duration`:
     `Drain` from `EXIT_BOUND` or the `Exit`'s bound (the tests'
     `with_bound`); `Idle` the knob's remainder, `knob − (now −
     last_activity)`, floored at `IDLE_POLL` when the verdict was
     `Busy` by a clause other than time (a connection, a live job, a
     pending login, a landing, the limiter live: `server.rs:680–688`),
     so an idle daemon wakes at its deadline, once, and a busy one
     re-judges as often as it polls today. `IDLE_POLL` is renamed to
     say what it now is, the re-judge interval, its value kept.
     The knob is read once at the loop's start, as the watchdog reads
     it (`server.rs:697`); `daemon status` and the start line keep
     their own reads (`server.rs:416`, `start.rs:590`).
   - **The instants**: each `Arm` a `tokio::time::sleep_until` held
     until its `Deadline` is delivered or cancelled; on tokio time,
     as the watchdog and the drain are today and for the reason
     their comments give (`server.rs:691–695`, `daemon.rs:196–201`):
     the manual clock advances on every sleep. `Clock` gains no
     `sleep_until` in this step (the dated paragraph; step 3's
     spike decides whether a parking clock takes the driver).
   - **The one race**: `Drain` abandons `AwaitQuiet` (the drain's
     `deadline.min(now + DRAIN_POLL)` today, `server.rs:603`); the
     quiet wait's looks are the driver's, `DRAIN_POLL` kept as the
     pause between them. `Idle` abandons nothing: its deadline is a
     source the loop awaits beside the others.
   - **The sources' order on a tie**: a `Quiet` ready at the poll the
     drain's deadline passes is delivered first (today's loop looks
     before it reads the clock, `server.rs:584–593`); a stop or a
     signal ready with a deadline is delivered before it (the cause
     that reached `arm` first wins today, and a deadline is never a
     cause). One `biased` select, its order stated in a comment.
   - **Timer lifetimes**: `Idle` armed at the loop's start and after
     each `Busy`, cancelled at `Begin`; `Drain` armed at `Begin`,
     ended by `Quiet` or its own expiry; the bound thread armed inside
     `Begin`, never cancelled (a leave ends the process first).
   - **The words**: every log line as today (`"idle timeout;
     exiting"`, `"{first} received; exiting"`, `"stop requested;
     exiting"`, `"exit: nothing in flight"`, the cut line, the second
     signal's line, the checkpoint lines, the line a signal after the
     exit began writes, `start.rs:225–229`):
     the driver's, read by the cross-process tests, unchanged.
   - **The seam for the in-process tests**: `Leave` is performed
     through a hook the harness installs (the `end` hook exists,
     `daemon.rs:246`, `start.rs:537`; the leave's `process::exit`
     needs the same), so a test runs the driver to its `Leave` and
     reads the effect, never the process's end. The harness gains
     `spawn_life(daemon, knob, bound, hook)`; the production `run`
     installs the real hooks and spawns the same loop at `start.rs:612`.

4. **The real-wiring tests**, under the harness's ceilings, asserting
   no duration:
   - In-process, the driver at the knob and the bound forced to zero
     (C137): `c137_the_idle_deadline_at_zero_leaves_a_quiet_daemon`
     (the loop with knob zero on a daemon with no connection, no job,
     no login, the restore landed: the hook sees `Leave { checkpoint:
     true, code: 0 }`, the log has "idle timeout; exiting"; the same
     daemon with a connection open is judged busy and does not
     leave); `c137_the_drain_bound_at_zero_cuts_what_is_in_flight`
     (a job running, a stop requested, the bound zero: the hook sees
     the cut line then `Leave`; the row holds the order, this test
     the wiring). These are the holders of `exit.rs:302`'s upper
     bound.
   - Cross-process, one new test beside `daemon_signal_exit.rs`'s
     two: `c137_a_daemon_under_the_idle_knob_at_zero_exits_on_its_own`
     (the client harness starts a daemon with `ACQ_IDLE_SHUTDOWN=0`
     and no connection held, `wait_exit` under the 120 s ceiling, the
     log's "idle timeout; exiting", exit code 0, no socket left): the
     first observation of a real idle exit. `start.rs:14–15` says
     zero forces the exit, as `ACQ_LOGIN_WITHIN`'s doc does (23–25).
   - `daemon_signal_exit.rs`, `daemon_stop_checkpoint.rs`,
     `provider_default.rs` and `readme_tour.rs:324` unchanged.

5. **#131's daemon items that this step's rows and wiring hold**,
   each by its rule, one commit:
   - `exit.rs:302`: the upper bound goes; the row "the drain's
     deadline cuts" and the zero-bound wiring hold it; the test keeps
     its outcome assertions (the cut reported, the row Running on
     disk, the log line).
   - `exit.rs:486`: the upper bound goes; the test asserts the
     outcome instead (the drain returned clean with nothing cut
     under `BOUND`, which it could not have if it waited the 30 s
     hold).
   - `exit.rs:424`: the upper bound goes; the lower bound (`waited >=
     bound + grace`, which slowness cannot miss) and the code stay;
     the bound thread is real by design and this test is its one
     pin.
   - `exit.rs:60` and `harness.rs:613`: the 3 s ceilings over the
     5 ms polls become the 120 s hang ceiling (#131's "small each").
   - `exit.rs:366`: the 20 ms sleep goes; the row "the second cause
     does nothing" holds the property, the test keeps its assertion
     that the deadline is unchanged.
   - The four `settle`-then-not-finished tests (85, 116, 503, 566)
     keep their wiring (what `InFlight` counts) with `BOUND` raised to
     the knob's largest so it cannot expire before the release; their
     ordering (Draining stays until Quiet) is a row. The `settle`
     sleeps stay: a sleep that proves a wait false-passes and never
     flakes (#131's second list), and their replacement by a state
     wait is #131's own close, not this step's.
   - `exit.rs:193` and `:262`: as the four above; #131's.

6. **The one constructor**, its own commit after the machine's:
   `Daemon::builder(provider, clock, credential_store, log)` with
   options for what the three literals vary (rails, the jobs
   database, the initial sessions, restore landed, the exit's bound,
   grace and hooks, `login_within`, the world's paths and artifact,
   the events capacity; `declare_route_knowledge` called by the
   builder), replacing `start.rs:500–534`, `harness.rs:543–576` and
   `sessions.rs:219–265`; the `Daemon` literal exists once, in the
   builder. The `Arc::get_mut` patches the tests make after
   construction stay as they are (not this step's). t5's "construct
   with a base URL, a clock and a credential store" is this builder.

7. `tools/gate.sh` once per commit, on each commit's tree; every
   golden under `crates/*/tests/goldens` and every log line a test
   reads unchanged.

## Done when

- `crates/acquisition-daemon/src/daemon/life.rs` exists and
  `grep -n "tokio\|Instant\|Duration\|async\|std::io\|std::fs\|std::process\|Arc<\|Mutex" crates/acquisition-daemon/src/daemon/life.rs`
  finds nothing, quoted in the commit.
- The table has one row per transition Do 2 names, each row's name
  in the file; the commit message lists the row names.
- The real-time inventory of the lifecycle:
  `grep -n "timeout\|from_millis\|from_secs\|Instant::now\|sleep\|elapsed" crates/acquisition-daemon/src/daemon/server.rs crates/acquisition-daemon/src/daemon/start.rs crates/acquisition-daemon/src/daemon.rs`
  quoted in the commit with each line classified as one of: a
  duration constant or its knob's parser; the driver's `duration`
  table; the driver's arming and the one race; the bound thread
  (`daemon.rs` 236–245, unchanged); the accept backoff (`start.rs:720`,
  unchanged); the panic hook's uptime (`start.rs:297`); `mod tests`.
  A line that fits no class fails the step. `IDLE_POLL`'s
  `sleep` (`server.rs:699`) and the drain's own `sleep_until`
  (`server.rs:603`) are gone as written, their lines in the table
  and the driver.
- `grep -n "waited <" crates/acquisition-daemon/src/daemon/tests/exit.rs`
  finds nothing, and no test under `crates/acquisition-daemon/src
  /daemon/tests` asserts an elapsed upper bound (the commit says
  which lines went and which rule holds each).
- The two in-process `c137_` tests and the cross-process one of Do 4
  exist by name; `idle.rs`'s four tests, `c148_…` and the two
  `daemon_signal_exit.rs` tests are present by name and unchanged in
  what they assert.
- `grep -rn "Daemon {" crates/acquisition-daemon/src --include=*.rs`
  shows one construction (the builder's), the rest patterns or
  doc, each classified in the builder commit's message.
- Every golden under `crates/*/tests/goldens` byte-identical to the
  step's parent commit; `CLI-REFERENCE.md` and `MCP-REFERENCE.md`
  unchanged.
- `tools/gate.sh` exits 0 on each commit's tree.

## Not in scope

- `Clock::sleep_until` and a driver on the daemon's `Clock`: step 3's
  spike (#147) decides whether a parking clock takes the driver; the
  tree's two test clocks cannot (`harness.rs:62–66`,
  `dispatch.rs:65`).
- The parking clock and t5 (#141).
- The verdict's clauses (`is_idle`): the driver's pure function,
  pinned by `idle.rs`; a change to what holds the daemon is a ruling
  (C3).
- `InFlight`'s six parts: what the drain waits for is C133's; the
  wait is an effect.
- The accept backoff, the login's three bounds, the mock kinds'
  sleeps: effects, one value each, untouched.
- The `settle` sleeps and the 50 ms sleep of `exit.rs:193`: #131's,
  by its own rule, after this step.
- The `Arc::get_mut` patches in the daemon's tests.
- Any word a user meets, any log line a test reads, `daemon status`'s
  fields: unchanged. (Naming the moment the daemon will exit in
  `daemon status`, which the armed timer now knows, is a feature
  waiting to be asked for: an issue if a user asks.)
- The store's fixed waits: #148.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on
`main`, in a checkout other sessions may share; never push; `acq`
and `acqd` only in a mock world; kill only this session's pids; an
experiment is undone from a scratch copy, never `git checkout --
<file>`. The owner is away from the machine on 2026-10-31 and has
granted the orchestrating session autonomy; builds run. Two commits
at least, the machine's and the builder's, each gated. The
attribution line names the model the worker runs on.

## Report

`TEMPLATE.md`, and the table's run time as a number nothing asserts
(C137). Raised for review: the `Idle` duration's floor (the re-judge
interval while a non-timed clause holds) against an activity-driven
wake, which would need notifications at five sites and the limiter's
quiet instant; and whether `daemon status` should name the exit
moment.
