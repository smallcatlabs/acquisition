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

## Answer (2026-10-31)

**The question.** Can a parking clock behind `Clock` drive a
scripted-server scenario through the daemon's door in virtual time
without `test-util`, by the design's count ("sleepers equal the tasks
the door knows wait, after a yield"), and should `Life` move onto it?

**The answer.** The clock works and the count does not. The count is
right while one task parks and wrong once two do. The exact signal is
the runtime's own word that nothing can run: a current-thread runtime's
`on_thread_park` hook (stable tokio, no feature split, so C84 holds),
which advances the clock only when the daemon's two counts of work off
the runtime are zero: the gate's occupancy (sends in flight) and C148's
`landings` (blocking writes in flight). `Life` should move onto the
clock in t5, together with the exit's deadline and the quiet wait's
poll. Moving it alone breaks 8 tests.

**The runs.** Each number below comes from one of these.
(a) `cargo nextest run -p acquisition-daemon -E 'test(/spike_/)'
--no-capture` at `c6272386`, the branch head. It runs the scenario and
clock tests in `tests/dispatch.rs`.
(b) The same command at `b62b2f6c`, the scratch commit that puts the
`Life` driver's timers on the clock. It adds the three `spike_life_*`
tests in `tests/exit.rs`.
(c) `cargo nextest run -p acquisition-daemon
spike_the_429_hold_on_the_system_clock --run-ignored only
--no-capture`, with `tests/dispatch.rs` as committed at `64a117a4`.
(d) `cargo nextest run -p acquisition-daemon --no-fail-fast -E
'test(/daemon::tests::(exit|idle)::/)'` at `b62b2f6c`.
(e) `cargo nextest run -p acquisition-daemon` at `c6272386`: 338
passed, 1 skipped (the system-clock run).

**Measured.**

- **The 429 hold, one job.** The script is a probe, a GET answered 429
  with `Retry-After: 0` (a 61 s hold), then the GET landing.
  - Manual clock: 3.5 ms (a).
  - Door rule on the parking clock: 61 cycles, 61 s virtual, 4.0 ms
    real; at most 1 yield to match per cycle; nothing misjudged (a).
  - Door rule with a 100-yield confirmation window: 74.9 ms (a).
  - Park hook: 61 advances, 3.5 ms; 16 parks held by a send (a).
  - System clock: 61.008 s real (c).
  - Every clock journals the same three sends, as (method, path,
    status, counted, `wait_ms`): `HEAD /fetch 204 false 0`,
    `GET /fetch 429 true 0`, `GET /fetch 200 true 61000`. That is the
    same sends in the same order, with no difference.
  - The admission slicing costs one cycle per second of hold: each
    cycle is one park in the dispatcher's 1 s pacing slice
    (`dispatch.rs:220`). The choke point's admission loop never parked,
    since its check passed after the pacing.
- **Two keys held at once.** The script runs on the counting mock (the
  in-process mock that counts its rate-limit windows): 8 fetches and 8
  character lists, two policies at 5 sends per 10 s.
  - The door rule as drawn: **3 distinct journals in 5 runs** (a); 377
    cycles, about 92 ms per run. It never knew more than 1 task
    waiting.
  - With the 100-yield confirmation window: 1 distinct journal in 5
    runs, about 535 ms per run. Every run recorded a misjudgment at
    cycle 0: the door matched at (1 waiting, 1 parked), and 100 yields
    later it was (2, 2) (a).
  - The park hook: 1 distinct journal in 5 runs, asserted in every
    invocation; 377 advances, about 13 ms per run (a). Its journal
    equals the confirmation window's, read side by side in one earlier
    invocation, `cargo nextest run -p acquisition-daemon spike_
    --no-capture`, before `64a117a4`.
  - Control, the hook advancing even while a send is in flight: 3, 3
    and 1 distinct journals in 5 runs, over three invocations: one
    before `64a117a4`, then (b), then (a).

**What the approximation got wrong.**

- **It counts only the tasks already waiting.** The door can count
  what `InFlight::of` calls `held`: jobs picked and waiting at the
  limiter, each parked in the dispatcher's pacing or the choke point's
  admission. It cannot count a task that is runnable but has not yet
  parked. At cycle 0 of the two-key run, the second key's job had not
  been picked yet. The count read 1 = 1, so the door advanced while
  the dispatcher was still runnable. That changed which job was held
  for 16 s and made the journals differ.
  - A bounded spin (the confirmation window) hid the miss in this
    scenario, but its size is a guess and it cost about 5.8 times the
    real time.
  - A task parked somewhere the count does not look is a second blind
    spot: a Running job that re-parks in `acquire_send` after the
    pacing let it through. No scenario here produced one.
  - Elsewhere, the mock parks nowhere (its windows read `now`), and the
    dispatcher's main loop waits on a `Notify`, not on the clock.
- **The yield loop itself is wrong for the exact signal.** A door that
  yields keeps the runtime from ever parking. The door must await the
  job's events (`await_terminal`) and let the runtime idle.

**The smallest exact mechanism.** `idle_runtime` and `IdleHook` in
`tests/harness.rs`: a current-thread runtime whose park hook calls
`advance_to_next()` unless a send or a landing is in flight. Both
conditions carry load:

- **Sends.** The control that ignores sends gave up to 3 distinct
  journals.
- **Landings.** With the drain's deadline on the clock and the bound at
  3 s, a control that ignores landings cut the job on the wire in 3 of
  5 runs at first and 5 of 5 in (b). The cause is time advancing beside
  the profile's `spawn_blocking` landing. Counting landings makes it
  clean (b).
- **What no count covers.**
  - The `Life` driver's verdict also runs on `spawn_blocking`
    (`server.rs`, `Effect::Judge`), and nothing counts it. In (b)'s
    busy hold, 14 parks found nothing on the clock, which fits the
    verdict's windows, when the idle timer is disarmed and the judge
    runs on its thread. Harmless there, but a parked job's clock would
    move under the judge.
  - Real-time tokio waits never park on the clock: the sleep kind's
    200 ms slices, whoami's 300 ms, the accept backoff, the drain's
    10 ms look and reqwest's timeouts. The runtime idles beside them
    and virtual time moves. That is harmless unless one races a clock
    deadline. One did: in (b)'s clean drain, the 10 ms tokio look lost
    to the virtual drain deadline. The drain still ended clean, by the
    deadline's last look (2.99994 s virtual).
- **Multi-thread runtimes are out.** `on_thread_park` fires per worker,
  so it never means that all workers are idle. The one multi-thread
  `Life` test (`c133_a_signal_stops_admission_while_shared_is_held`)
  stays off the door.

**Whether `Life` should move onto the clock.** Yes, in t5, and not on
`main` before the parking clock replaces `ManualClock` in `exit.rs`.
`Clock::sleep_until` is kept on the branch, with a default over `sleep`
and the parking clock's own. The scratch driver move (`b62b2f6c`,
reverted by `c6272386`) was discarded.

- **On the parking clock under the hook (b):**
  - `c137_the_idle_deadline_at_zero_leaves_a_quiet_daemon`'s twin left
    in 1.5 ms real, at 0 s virtual.
  - The busy hold (a connection held at knob zero) re-judged at
    `IDLE_REJUDGE` for 60 s virtual: 13 busy notices, 12 advances,
    1.8 ms real. The connection then closed and the driver left at 65 s
    virtual, with nothing cut.
- **On the manual clock (d):** 8 of `exit.rs`'s 36 tests fail, because
  every drain deadline fires at once.
- **The std/tokio instant.** `Exit::arm` sets the drain's deadline on
  tokio time and has no clock. The scratch carried the remainder
  across, which reads two clocks. In t5 the exit's deadline must be set
  on the daemon's clock.
- **The bound's OS thread** stays real by design (C133). No harness
  daemon installs an end.

**Recommendation for t5's brief** (no second shape is needed; the
door's count is refuted by (a)):

1. Land `ParkingClock` as it is on the branch, replacing `ManualClock`
   and `BlockingClock`. `advance` keeps its meaning, and `sleepers()`
   subsumes `wait_for_sleepers`.
2. The door owns its runtime. It builds `idle_runtime`, installs the
   hook over the daemon it constructs, and awaits terminal states on
   job events; it never polls or yields. Scenarios become `#[test]`
   functions entering through the door's `block_on`.
3. The hook's conditions are the daemon's own counts: gate occupancy
   and `landings`.
   - Count the verdict's `spawn_blocking` (route it through a counted
     helper).
   - Pin "every `spawn_blocking` is counted" with an arch-check grep,
     so a new blocking task cannot silently break determinism.
4. Move `Life`'s three waits together: idle, drain, and the quiet
   wait's look (onto the clock or onto a `Notify`). Move `Exit`'s
   deadline onto the daemon's clock. The bound's thread stays real.
5. Keep multi-thread tests off the door.

**Open, not pursued:**

- A Running job parked in `acquire_send`, unseen by any scenario here.
  The hook does not need to see it.
- Whether reqwest's tokio-time timeouts need anything. Today they are
  real time, unchanged, and the hook never advances them.
- The red `Gate (Windows)` run 41237037443 on `ad1828d4` (`Gate
  (Linux)` 41237037407 passed). The one failure is
  `acquisition-cli::main
  price_json::every_offer_a_refusal_prints_runs_as_printed`
  (`price_json.rs:1826`). It compares two runs' whole JSON, and the
  sides differ only in `listing.basis.at_age` ("1s ago" against
  "0s ago") and `taken_at` (…019 against …018): a second boundary, not
  this spike's area. The worker's attempt to file it was refused by its
  permission classifier, so it goes to the orchestrating session to
  file.
