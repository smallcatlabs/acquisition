# 4-0-spike — the parking clock, one day (#147, step 3)

A spike, not a step: it answers one question with evidence and lands
no production code on `main`. Its answer shapes t5's brief (#141) or
goes to the owner with two shapes. Written from the step 2 reads of
the tree (`4-0-life.md`, Read; the clock reader's report, folded into
Input) after step 2 closed on 2026-10-31.

## Goal

When this spike is done, the project knows whether a parking clock
behind the daemon's `Clock` (a deadline heap: `sleep` and
`sleep_until` park, `advance_to(t)` wakes the sleepers due by `t` in
deadline order, `advance_to_next()` moves to the earliest pending
deadline, `sleepers()` says how many are parked) can drive one
scripted-server scenario through the daemon's test door in virtual
time without tokio's `test-util`: whether "every task this door knows
of is parked" is a signal the door can read, how the admission loops'
sliced sleeps (at most `HALT_POLL`, 1 s, per park) behave under it,
and whether the `Life` driver should move onto it. The answer is a
dated section of this file, with the measurements, and a comment on
#141; the code stays on the spike's branch for t5's brief to cite.

## Read

- `roadmap/briefs/4-0-design.md` §3 item 4 (the parking clock as the
  design drew it, with its one named mechanism risk) and §6 item 3;
  the two dated paragraphs under §4 (what the door and the lifecycle
  became); `roadmap/TEST-SHAPE.md` §2 item 4 and §4 (the daemon's
  test door t5 builds); issue #141 (t5: construct with a base URL, a
  clock and a credential store; submit and await through the
  protocol; read the journal).
- `decisions/daemon.md`: C157, C137; `decisions/network.md`: the
  limiter's rulings the choke point cites; `AGENTS.md`: C84 (one
  tree, one artifact: the gate's `features` stage fails a dev-only
  feature split, which is why `test-util` is out, `tools/gate.sh`
  164–228).
- `crates/acquisition-daemon/src/ratelimit.rs`: the `Clock` trait
  (66–76: `now` a `std::time::Instant`, `wall`, `sleep`, `kind`),
  `SystemClock` (78–96). `ratelimit/choke.rs`: `HALT_POLL` (222) and
  the admission loops `acquire_send` (372–416) and `acquire_head`
  (418–450), each parking `wait.clamp(50 ms, HALT_POLL)` through the
  clock and re-checking rails and `wanted` after every park; `check`
  (330–333, `Limiter::wait_for`). `daemon/dispatch.rs:184–222`: the
  dispatcher's pacing, `choke.sleep(d.min(1 s))` at 220.
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`:
  `ManualClock` (22–68: `sleep` advances the clock by its own
  duration and yields, never parks); `scripted_server` (351);
  `test_daemon_scenario` (522, 531); `wait_terminal` (613). `tests/
  dispatch.rs`: `BlockingClock` (15–75: `sleep` counts a sleeper,
  awaits a semaphore permit, then adds its own duration; FCFS, no
  deadline order; the count never decremented, so
  `wait_for_sleepers(n)` at 33–42 waits for n sleeps begun, not n
  parked) and its one test,
  `dispatcher_rate_waits_do_not_cap_independent_job_progress`
  (191–250). `mockggg.rs`: `start` (602), the 429 answers (818, 891)
  and the windows the mock counts against the `now` it is passed
  (92–121).
- `crates/acquisition-daemon/src/daemon/server.rs`: `Daemon::life`,
  the lifecycle's driver on tokio time, and its doc saying why
  (step 2's close); `daemon.rs`: `Daemon::builder`.
- `crates/acquisition-daemon/src/daemon/tests/exit.rs`: the `Life`
  table and the in-process wiring tests, as the shape a driver test
  on a parking clock would copy.

## Input

Fixed before the spike: step 2's reads of the two test clocks (above,
every line printed from the tree at 5c9cd487 and unchanged since);
the design's approximation of "nothing else can run" (§3 item 4:
sleepers equal to the tasks the door knows are waiting, after a
yield); the admission loops' slicing, so a 30 s hold is at least
thirty park-and-advance cycles, each re-reading `now`, the rails and
`wanted`; the fact that `Clock::now` is a std `Instant` while tokio's
timers take tokio's. The last push's CI: `Gate (Linux)` 41236802385
and `Gate (Windows)` 41236802362 on `f280fbc3`, in progress at the
brief; a red run is this spike's first concern or is filed. The
scenario: one job against the scripted server answering a 429 with a
hold (the harness's 429 scenario, found by grep in `tests/`), then
the job landing after the hold, journal intact.

## Do

1. **The clock**, in the daemon's test support only (`tests/
   harness.rs` or a module beside it), about 100 lines: `ParkingClock`
   implementing `Clock`, with `advance(d)` (the name and meaning
   `ManualClock`'s callers rely on), `advance_to(t)`,
   `advance_to_next()`, `sleepers()`, and `sleep` parking in a heap
   keyed by deadline, woken in order, each sleeper's own deadline
   its wake (no summed overshoot). `now` and `wall` as `ManualClock`'s
   (both faces; `laptop_sleep` kept). Nothing in production code.
2. **The door's advance rule**, as a test helper: `run_until_terminal
   (daemon, job)` that drives the scenario by yielding until the
   sleeper count equals the tasks the door knows are waiting, then
   `advance_to_next()`, repeating until the job is terminal or a
   bound on cycles trips. Measure: how the door learns "tasks waiting"
   (what it can count: the dispatcher's park, the choke point's
   admission parks, the mock's own), and whether a yield suffices or
   a bounded spin is needed.
3. **The scenario**: the 429 hold through the door on the parking
   clock, the journal compared to the same scenario on `SystemClock`
   (the existing test's journal, or one run of it): same sends, same
   order, no real waiting. Record the cycle count, the wall time, and
   every place the approximation misjudged (a task not parked that
   the count thought was, or a park the count missed).
4. **The second question**: `Clock::sleep_until(Instant)` added on
   the spike's branch with a default over `sleep` and the parking
   clock's own, and the `Life` driver's `Arm` moved onto it in a
   scratch copy, driven through its in-process tests on the parking
   clock: does `c137_the_idle_deadline_at_zero_leaves_a_quiet_daemon`
   and the busy-hold test run with no real time, and does anything
   break (the judge's `spawn_blocking` thread against a parked
   clock; the std versus tokio instant). Keep or discard; say which.
5. **The answer**, a dated `## Answer` section at the foot of this
   file, in the form: the question in one line; what was measured
   (the cycle count, the wall time, the journal diff); what the
   approximation got wrong, if anything, and the smallest mechanism
   that would make it exact; whether the driver should move onto the
   clock; and the recommendation for t5's brief, or two shapes for
   the owner with the smallest named. Then a comment on #141 pointing
   at the section and the branch.

## Done when

- The `## Answer` section exists at the foot of this file on the
  spike's branch, dated, in the form Do 5 names, with every number
  from a run the section names by command.
- `ParkingClock` and the scenario test exist on the spike's branch
  under `crates/acquisition-daemon/src/daemon/tests/`, and `cargo
  nextest run -p acquisition-daemon` is green there (the full gate is
  not owed: a spike lands nothing on `main`).
- `git diff main --stat` on the branch shows no file outside
  `crates/acquisition-daemon/src/daemon/tests/`, `ratelimit.rs` (the
  `sleep_until` default, if kept) and this brief.
- A comment on #141 names the section and the branch.
- The spike took one day or less of the worker's time; a question it
  could not answer in that time is named as open in the section, not
  pursued.

## Not in scope

- Landing the parking clock on `main`: t5's (#141), after its brief.
- Moving the `Life` driver or the door's driver onto a clock on
  `main`: the answer says whether; the move is t5's.
- tokio's `test-util` or `start_paused`: forbidden (C84).
- Replacing `ManualClock` or `BlockingClock` in the existing tests:
  t5's.
- Any production change: none; a spike reads and measures.
- The store's waits (#148).

## Rules

`TEMPLATE.md`. What differs: this spike runs in its own worktree on
its own branch, never on `main`; its commits are its own and are not
reviewed cold (a spike's evidence is read by the orchestrating
session); never push; `acq` and `acqd` only in a mock world, under
the harness; kill only this session's pids. The owner is away on
2026-10-31 and has granted the orchestrating session autonomy; builds
run. The attribution line names the model the worker runs on.

## Report

`TEMPLATE.md`, and the `## Answer` section verbatim in the closing
message.
