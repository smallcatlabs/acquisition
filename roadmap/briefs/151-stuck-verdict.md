# 151-stuck-verdict — the daemon's one clock, and a door wait that fails at the exact park (#151)

Cut three times: the first draft (`e1a0dcb8`) and the re-cut
(`132a43ef`), each reviewed cold
(`roadmap/reviews/151-stuck-verdict-brief-<sha>.md`, the prompt beside:
eight findings, then seven, two nitpicks each); this cut answers the
second review's seven, listed at the foot under "Taken from the
reviews" with the first's eight. Written by the orchestrating
session on 2026-11-01 from `roadmap/reviews/time-seam-retrospective.md`
(Root A and rows A1, A2 and E4 of its §10), a re-read of the tree at
`b67776d7` (every line number below from it) and a read of the saved
mutant logs under `runs/`. Its one ruling (Do 1) was the owner's and
is in the registry ahead of the code ("yes to all", 2026-11-01); its
words were narrowed at the first review and qualified at the second
under his grant of that day, for his re-read (Do 1). To be reviewed cold again before a worker reads
it.

## Goal

When this step is done, every tokio timer in the daemon's production
sources sleeps through the daemon's one `Clock`, held by a check in
`tools/arch-check.sh` in the shape check 10 has for `spawn_blocking`;
and a wait on the hook's own door, whose answer nothing the hook
accounts for can still bring, fails at that park, naming what it waited
for, instead of running to the 120 s ceiling (a door opened beside
another has no hook and keeps the ceiling). Of the 48 mutants that moved from caught to
timeout at t5's close (#151), the ones whose hung scenario waited at
such a park are caught again; the four that replace the count the
hook reads are listed as the hook's own instrument, told apart by a
pin; the rest are named with their class. A weekly run after the close
shows which. No word a user meets changes, no log line a test reads
changes, and the daemon in production waits exactly as long as it does
today: `SystemClock` is tokio's timer behind the trait.

Why it matters: the spike found that the runtime's park is the one
exact signal that nothing can run, and t5 used it to move time forward.
The same park, read against what the hook accounts for (the clock's
sleepers, a send in flight, a blocking task, a hold of the scenario's),
is exact the other way: nothing the hook knows of can wake the daemon,
so a door wait at that park will not be answered. Today the hook
increments a counter nobody reads (`door.rs:231`) and the scenario
waits `HANG`; under cargo-mutants' timeout (17.6 to 24.4 s in the saved
logs) that is a timeout, not a catch. What keeps the verdict from being
exact is six timers in the daemon on tokio's own wheel, which park
beside the clock unseen, and any wait of the scenario's own on the
runtime's I/O driver, which the door's rule already says to hold the
clock across. The first is closed by a rule with a check; the second
stays a rule, and a scenario that breaks it meets a verdict that names
it, loudly, which is the right direction of failure: a false verdict is
a test failure that says why; a missed one was a two-minute silence.

## Read

- `roadmap/reviews/time-seam-retrospective.md` §4 Root A, §5 (what the
  developer seat pays), §6 (a lower ceiling and `test-util` rejected),
  §10 rows A1, A2, E4. `roadmap/reviews/151-stuck-verdict-brief-e1a0dcb8.md`,
  the first review: finding 1 (quiescence is not proof: the I/O driver),
  finding 2 (the occupancy mutants), finding 3 (`timeout` against
  `select`), finding 5 (the `cfg(test)` reader).
- `roadmap/briefs/4-0-spike.md`, "Answer": the park hook's discovery
  and "What no count covers": the real-time tokio waits it names, of
  which this step takes every one but reqwest's (the drain's look went
  onto the clock in t5 round 2).
- `roadmap/briefs/t5-design.md` §2.2 (the runtime is the door's), §2.3
  (every blocking task counted, check 10), §2's last paragraph (a
  silent server waits its real timeout) and §3's `BlockingClock` row
  (the sleepers test kept as a component test on the parking clock).
- `decisions/daemon.md`: C160 (as entered and narrowed: every tokio
  timer in the daemon sleeps through its one `Clock`; a door scenario
  fails at the park where nothing the hook accounts for is pending; a
  scenario's own wait outside the accounting stands under a hold),
  C158 (a scenario's time moves by its own hand or when nothing it
  started can run), C157, C137 (no wall-clock bound a slow machine can
  miss: the verdict is not a bound), C133 (the exit bound's OS thread,
  real), C148, C3, C84 (one tree, one artifact: no `test-util`, no
  `tokio_unstable` cfg). The "Not doing" line on a request timeout's
  expiry (line 62), as amended. `AGENTS.md`: C122, C153, C154.
- `crates/acquisition-daemon/src/ratelimit.rs`: the `Clock` trait
  (68–88), `SystemClock` (90–115) whose two `tokio::time::` lines (106,
  113) are the only ones the check allows; the two `#[cfg(test)]`
  attributes on single items at 44–45 and 48–50 (a `use` and a `mod`
  line, semicolon-terminated: the reader of Do 6 must not brace-match
  them). `ratelimit/choke.rs`: `ChokePoint`'s private `clock` field
  (205), its `sleep` and `sleep_until`; `CONNECT_TIMEOUT` and
  `REQUEST_TIMEOUT` (216–217, 10 s and 60 s real, the client's own,
  240–241); `actual_send_occupancy` (473–475) over `SendGate::occupancy`
  (`gate.rs` 78–81): the count the hook reads.
- **The six timers**, every `tokio::time::` call in the daemon's
  production sources outside `tests/`, found by `grep -rn "tokio::time"
  crates/acquisition-daemon/src | grep -v "/tests/\|spec.rs"` at
  `b67776d7` (eleven lines: these six, `SystemClock`'s two, and three
  in `auth.rs` at 898, 916 and 924 inside its tail test module, which
  opens at 647–648 as `#[cfg(test)]` then `mod tests {` and closes on
  the file's last line):
  - `daemon/kinds.rs:572–577`, the `sleep` kind: a deadline from
    `Instant::now()` and 200 ms slices on tokio time, the slices so a
    cancel is seen within 200 ms (`self.cancelled(id)`, 574); its
    comment ("Real seconds by design, not the daemon's clock") was
    written when the manual clock advanced on every sleep, a reason
    the parking clock retired. Submitted in the daemon's tests by
    `exit.rs` 386, 425 (60 s through the door, cut by the bound), 458,
    1256, 1296, `idle.rs:182`, `parents.rs:192`, `evict.rs:190`
    (`grep -rn '"sleep"'`); by the CLI's provider tests at 0 s.
  - `daemon/kinds.rs:621`, the `whoami` kind's 300 ms pause
    (`dispatch.rs`, `queue.rs`, `rails.rs` submit it).
  - `daemon/start.rs:696`, the accept backoff inside `serve` (675–700,
    `self: Arc<Self>`, so `self.choke` is in reach; C134). Pinned by
    `exit.rs:1897`, `a_run_of_listener_errors_logs_a_bounded_number_of_lines`
    (1.18 s real today), and the two listener tests the door's doc
    names as off the door (`door.rs:42–43`).
  - `daemon/login.rs:48–51`, the callback bound:
    `tokio::time::timeout(daemon.login_within, wait_callback(...))` in
    a spawned task holding `daemon` (46); its arms at 54 (`Ok(Ok(code))`)
    and 96 (`Err(_)`, the expiry's words). Pinned at zero by
    `tests/login.rs:743–753` (`login_within(Duration::ZERO)`).
  - `daemon/login.rs:445`, `wait_job_terminal`'s bound (436–452, the
    profile wait), consumed by `.ok()` at 480; its expiry has no pin
    (the ledger's open item of 4-0 step 2; `queue.rs:1668`'s pin is a
    broken-queue answer).
  - `daemon/login.rs:1007`, `answer_callback`'s read bound
    (`CALLBACK_READ_WITHIN`, 959, 10 s): `wait_callback` (975) takes
    the listener, the state and the bound and spawns `answer_callback`
    (1001) per connection; neither holds a daemon or a clock.
  - Beside them, two real-time reads: `Instant::now()` at
    `kinds.rs:572–573` (goes onto `self.choke.now()`), and
    `daemon.rs:273` and `:282`, the exit bound's OS thread
    (`std::thread::sleep` on a `std` instant: real by design, C133; no
    tokio timer, outside the check by its spelling).
- `crates/acquisition-daemon/src/daemon/tests/door.rs`: the module doc
  (1–56); `Counters` (156–175, "asserted by nothing"); `IdleHook` and
  `on_idle` (177–237: quiet, hold, send, blocking, settle, advance,
  nothing parked, in that order; the send branch at 219 reads
  `actual_send_occupancy`); `SettleSlot` (207); `idle_runtime`
  (240–254); `DoorBuilder::run` (483–508: the hook installed after the
  build, the scenario awaited inside `block_on` at 504, the counters
  returned); `beside` (521–: no hook of its own); `settle` (715–735)
  and `quiet` (737–763, whose doc names the socket-read park); the
  `hold_clock` rule (796–811); the waits this step gives the verdict:
  `await_leave` (822–832), `await_job` (839–867), `await_terminal`
  (869–897), `await_auth` (899–946), and `finish`'s awaits on a script
  answered whole (1086, 1093), each under `tokio::time::timeout(HANG,
  …)`; `counters()` (1060). `harness.rs:849`, `HANG`; `advance_until`
  (827–844, a yield loop for a component test on a plain runtime: not
  this step's, Not in scope says why); `scripted_server_received`
  (560–606: the task ends after its last answer and the listener drops
  with it, so a send past the script is refused at once, never a
  silent wait); the one reader of `Counters`, `dispatch.rs:2486–2507`.
- `crates/acquisition-daemon/src/daemon/tests/rails.rs:118–165`,
  `tripped_daemon_leaves_queued_jobs_waiting_until_reset`: the scenario
  24 of the 48 mutants hung (Input), waiting in `await_job(first, …)`
  (129) on a predicate a mutant keeps from ever holding, then
  `settle` (146). `tests/dispatch.rs:133–200`,
  `dispatcher_rate_waits_do_not_cap_independent_job_progress`: the
  `#[tokio::test]` component test ten of the 48 hung, in its yield
  loop `while clock.sleepers() < 2` under `HANG` (174–178), kept off
  the door by t5's design as `BlockingClock`'s holder; `stress.rs:44`
  (one mutant, two fixtures, component by the door's doc).
  `dispatch.rs:98–104`, the halted daemon as a scenario stages it.
- `tools/arch-check.sh`: the header's list (28–41) and check 10
  (462–512: the Python reader, comment lines blanked, the helper's span
  by brace depth, the `ok` line and the `BLOCKING` refusals); the stale
  words at 465–466 ("until its closure returns"; `daemon.rs:1475–1494`
  says what is true). `tools/arch-check-breakers.sh`: `run_case` (110),
  `adddep` (70), `tool` (90), the `owned` list (36–45) and the
  `files=(…)` restore list (44–46, which names neither `ratelimit.rs`
  nor `auth.rs`), the trap (47); cases for §5, §7, §8, §9 and §11,
  none for check 10 (`grep -n spawn_blocking` finds nothing).
- `.cargo/mutants.toml` (the scope; the exclusions' form and their
  rule: a line "names its line", a pattern per function; "a mutant
  that hangs the suite is reported as a timeout, and counts as
  caught"); `.github/workflows/weekly.yml` (the four shards,
  `workflow_dispatch`, no `--timeout`, so cargo-mutants' auto timeout);
  `tools/mutants-compare.py`; #151.
- `roadmap/LEDGER.md`, the rows of 4-0 step 2, t5 round 1 (the
  blocking count's handoff), round 2 (`hold_clock`; the 1 ms polls),
  round 4 (the login's timeouts on tokio's clock), round 5c (the halted
  fault scenario hangs to `HANG`) and round 6 (the baseline and
  after-run ids and tags).

## Input

Fixed before the work starts:

- **The timer sweep**, run at `b67776d7` by the command in Read: six
  production sites. The measure of the class is that command's output
  outside `/tests/`, `spec.rs` and the tail test module of `auth.rs`,
  which Do 6's check makes a gate stage.
- **The ruling**, entered: C160 in `decisions/daemon.md` after C158,
  791 bytes (`python3`, `len(line.encode())`), under the registry's
  800; `python3 -I tools/bothy.py check` green on it.
- **The 48 moved mutants by the scenario that hung**, read by the
  orchestrating session on 2026-11-01 from each mutant's saved log
  under `runs/t5-round6-after/mutants-shard-*/log/` (the log whose
  second line carries the mutant's description; nextest's `SIGTERM`
  lines name the tests still running when cargo-mutants' timeout
  killed the run, 17.6 to 24.4 s in; the list from
  `tools/mutants-compare.py runs/t5-round0-baseline runs/t5-round6-after
  --before-rev a8a719d6 --config-rev e139e522`, "paired mutants whose
  outcome changed", 48 `caught -> timeout`):

  | Hung scenario | Mutants | Where it waits |
  | --- | --- | --- |
  | `rails::tripped_daemon_leaves_queued_jobs_waiting_until_reset` | 24 (22 of `policy.rs`'s parse, `choke.rs` 600 and 644 in `head`) | `await_job(first, retries == 1 && Waiting)`, `rails.rs:129`: a predicate the mutant keeps from holding, nothing pending |
  | `dispatch::dispatcher_rate_waits_do_not_cap_independent_job_progress` | 10 (`limiter.rs` 112 ×2, 218, 737, 738, 900, 912, 930, 990; `choke.rs:342`) | the yield loop `while clock.sleepers() < 2`, `dispatch.rs:174–178`, a component test |
  | `dispatch::c158_the_wire_table_through_the_door`, `c158_a_blocking_task_holds_the_clock_…`, `exit::a_faulted_probe_fails_its_route_jobs_even_when_halted`, `exit::exit_cuts_a_stuck_job_at_the_bound` | 4 (`gate.rs:79` ×2, `choke.rs:474` ×2: occupancy replaced with `(1, 0)` and `(1, 1)`) | every advance withheld: the hook's send branch (`door.rs:219`) reads the mutated count |
  | `dispatch::a_head_429_that_fails_setup_holds_the_next_probe` | 4 (`limiter.rs` 444, 485; `policy.rs` 333:32, 333:24 true), of which 333:32 also hung the first row's scenario and is counted there | a door wait |
  | `dispatch::a_blocked_probe_is_no_probe_failure` | 3 (`choke.rs` 619, 676, 934) | a door wait |
  | `dispatch::a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header` | 2 (`choke.rs` 1063, 1064) | a door wait |
  | `stress::n6_integration_stress_…` | 1 (`choke.rs:800`) | a component test, two fixtures |
  | four tests at once (`exit.rs` ×2, `credentials.rs`, `bearer.rs`) | 1 (`limiter.rs:545`) | mixed |

  The table's memberships sum to 49 for 48 mutants, `policy.rs:333:32`
  in two rows and counted once. The classes the step acts on: **(a)**
  a door wait at a park with nothing pending (the verdict's: the first,
  fourth, fifth and sixth rows and the mixed one's door scenarios, 33
  mutants, 48 less the 4 and the 11 below); **(b)** the hook's own
  instrument (the third row, 4); **(c)** a component test's yield loop
  on a plain runtime with no hook (the second and seventh rows, 11). Do
  2's measurement confirms each class by running one mutant of it
  locally before Do 5 is built.
- **The measurement this step owes before Do 5**: `nothing_parked` per
  door scenario across the daemon's suite (Do 2). Nothing prints the
  counters today. The expected answer after Do 3 and Do 4 is zero in
  every passing scenario; a nonzero count is a scenario whose own wait
  stands outside the hook's accounting and gets a hold, or a timer Do 3
  missed, or a gap the step names and stops on.
- **The failing input the stuck pin stages** (Do 5): a daemon whose
  rails are halted (the tripwire tripped as `rails.rs` stages it) and a
  job submitted after the halt, parked on disk, the dispatcher waiting
  on its `Notify` with nothing parked on the clock; the scenario awaits
  that job's end. Round 5c raised this exact hang.
- **The mutants**: the baseline run 41265798977 (`t5-round0-baseline`,
  `a8a719d6`) and the after-run 41278094352 (`t5-round6-after`,
  `e139e522`) under `runs/` on the orchestrating session's machine. The
  run that closes #151 is dispatched by the orchestrating session after
  this step's close, on a tag, as round 0's was: not this step's to wait
  for.
- **The last push's CI**: `Gate (Linux)` 41341397975 and `Gate
  (Windows)` 41341397978 on `4680cf39`, both success (`gh run list`,
  2026-11-01). The commits since are documents and the registry. Re-read
  at the step's start; a red run is this step's first concern or is
  filed.

## Do

1. **The ruling**, entered: C160 in `decisions/daemon.md` after C158,
   the owner's word of 2026-11-01 ("yes to all"), the "Not doing" line
   on a request timeout's expiry amended with it. Narrowed at the first
   cold review (finding 1 and finding 4) by the orchestrating session
   under his grant of that day, for his re-read: the entry no longer
   says a park is "the runtime's word that the daemon will do nothing
   more"; it says the hook accounts for four things (the clock's
   sleepers, a send in flight, a blocking task, a hold of the
   scenario's), a wait of the scenario's own outside those stands under
   a hold, and so a false verdict names the scenario and a missed one
   cannot happen; the six-site inventory and the mutant count left the
   entry for this brief; 1,309 bytes became 791. Nothing in this step
   waits on a further word; if the owner rewords it, Do 5's words
   follow his.

2. **The measurement**, before any production edit, its own commit:
   the harness reads `ACQ_DOOR_COUNTERS` (test support only, no README
   row), and when it is set the hook prints, flushed to stderr at the
   park itself, one line for every park that did not advance the clock
   (held by a send, a blocking task or a hold; a settle; a quiet;
   nothing parked), carrying the scenario's name
   (`std::thread::current().name()`) and the counters so far, and
   `DoorBuilder::run` prints the counters once more at its end; so a
   scenario cargo-mutants kills leaves its last park's branch in the
   log above the `SIGTERM` line. Then two runs, both in the report and
   summarised in the commit message by the command and its totals:
   - `ACQ_DOOR_COUNTERS=1 cargo nextest run -p acquisition-daemon
     --no-capture`, parsed to a table (scenario, advances, held_by_send,
     held_by_blocking, held_by_scenario, settles, quiets,
     nothing_parked): the "before". A scenario whose `nothing_parked`
     is not zero is listed with the wait it was in at that park.
   - One mutant of each class in Input, run locally and alone:
     `cargo mutants -p acquisition-daemon --re '<description>'` for
     `parse_limits -> Result<Vec<Window>, PolicyParseError> with
     Ok(vec![])` (class a), `SendGate::occupancy -> (usize, usize)
     with (1, 0)` (class b) and `Limiter::wait_for -> Duration with
     Default::default()` (class c), each with `ACQ_DOOR_COUNTERS=1`
     passed through (cargo-mutants' `--env` or the environment), the
     hung test and the hook's branch at its last park read from the
     log; for class (c), whose test runs on a plain runtime with no
     hook, the record is "no hook; a runnable yield loop". This
     confirms the class each row of Input assigns; a mutant that lands
     in another class moves its row, and the report says so.

3. **The six timers onto the clock**, one commit, each site named in
   the message with the clock call that replaced it:
   - the `sleep` kind: the deadline from `self.choke.now()`, the slices
     `self.choke.sleep(200 ms)`; the comment rewritten to what is true
     (real seconds under `SystemClock`; parked and advanced under the
     door); the payload and the words unchanged;
   - the `whoami` kind: `self.choke.sleep(300 ms)`;
   - the accept backoff: `self.choke.sleep(pause)`;
   - the login's callback bound and the profile wait: one helper on
     `Daemon`, `within(&self, bound: Duration, future) -> Result<T,
     Expired>` with `Expired` the daemon's own unit struct (tokio's
     `Elapsed` cannot be built outside tokio), a `biased` `select!`
     that polls the future first and the clock's `sleep(bound)` second,
     so a future ready at the same instant as its bound wins, as
     tokio's `timeout` has it (finding 3); the three arms at
     `login.rs` 54 and 96 and the `.ok()` at 480 compile unchanged,
     reading `Ok` and `Err` as before;
   - the callback's read bound: `ChokePoint` gains `pub(crate) fn
     clock(&self) -> Arc<dyn Clock>` (a clone of its field, 205);
     `wait_callback` and `answer_callback` take an `Arc<dyn Clock>` and
     race the read through `within`'s shape over it (a free function
     twin of the helper, or the helper taking the clock: the worker's
     choice, named).
   After this commit the grep in Read prints `SystemClock`'s two lines
   and `auth.rs`'s three test lines and nothing else; the message quotes
   it. Every existing test passes unchanged in what it asserts; the
   `sleep` kind's rows and scenarios now park on the clock; a component
   test that submitted a `sleep` job on a hand-driven clock and never
   ran the dispatcher (`idle.rs:182`) sees no difference. The worker
   reads each site's tests before the edit and names in the report any
   whose duration changed.

4. **The holds the measurement named**, one commit: every
   `nothing_parked` park Do 2's first run found in a passing scenario
   is a wait of the scenario's own on the I/O driver (a socket of the
   scenario's, a request on its way to a fixture before the gate) and
   gets a `hold_clock` across it by the door's rule (`door.rs:796–811`),
   or a daemon timer Do 3 missed, which goes onto the clock here with
   its site named. Then Do 2's first command again: the "after" table,
   its totals in the commit message. **The stop rule:** if a passing
   scenario still shows a `nothing_parked` park that is neither, the
   step does not build Do 5; it names the site and what the runtime
   was waiting on in the report and in the last commit's message, and
   the orchestrating session brings C160's second clause back to the
   owner narrowed to what the evidence holds. The first clause and its
   check stand either way; done-when says what the stopped outcome is.

5. **The verdict and the classes**, on the evidence of Do 4 (zero
   `nothing_parked` across every passing scenario), one commit:
   - the hook's last branch (`door.rs:233–235` today) becomes the
     verdict: a park that finds nothing parked, no send, no blocking
     task, no hold, no settle and no quiet pending sends on a slot the
     door holds (`SettleSlot`'s shape, 207), the counters gaining
     `stuck` and still counting it;
   - every door wait races its wait against that slot in a `biased`
     `select!`, the wait first: `await_job`, `await_terminal`,
     `await_auth`, `await_leave`, and `finish`'s awaits on a script
     answered whole; the selected verdict arm panics with the wait's
     name, the job id or predicate it waited for, and the counters, in
     words a reader can act on ("the daemon can be woken by nothing the
     hook accounts for: nothing parked on its clock, no send, no
     blocking task, no hold; awaiting job 3's end"); `HANG` stays
     around each as the backstop a wait outside the accounting can
     still reach (a `beside` door's daemon, whose counts the hook does
     not read);
   - `settle` and `quiet` are untouched: each is woken by the hook at
     the parks its doc names, before the verdict's branch;
   - the pin, `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park`
     in `door.rs`'s own tests: the halted daemon and the parked job of
     Input, the scenario's `await_terminal` run under
     `std::panic::catch_unwind` around `run` (the scenario is awaited
     inside `block_on`, `door.rs:504`, so the arm's panic unwinds to
     the test), the message asserted to name the verdict and the job,
     the counters read before the unwind (`counters()`, 1060) showing
     one stuck park and no advance since. The test asserts no duration
     (C137); that it ended by the verdict and not by `HANG` is the
     message's own words;
   - **class (b), the hook's instrument**: a component pin over the
     gate, `c160_the_hooks_count_is_zero_at_rest_and_one_under_a_held_send`
     (`SendGate::occupancy` and `actual_send_occupancy` read `(0, MAX)`
     on a fresh gate and `(1, MAX)` while one permit is held), which
     tells the four mutants apart at once; and the four listed in
     `.cargo/mutants.toml`'s `exclude_re` in its form (the function's
     name, never the line), with the reason: the door's park hook reads
     this count, so every scenario that needs an advance stalls under
     the mutant, and the run is killed before a completed result
     though the pin's failure prints; the pin is named in the comment;
     an excluded mutant is not run, so the compare tool lists the four
     as removals;
   - **class (c), the yield loops**: `dispatcher_rate_waits_do_not_cap_independent_job_progress`
     onto the door's runtime as a component test, the shape the
     blocking-count probe already has (`dispatch.rs:2458–2510`: built
     through `Door::open … .run`, reading the clock, never a scenario,
     since it asserts `sleepers() == 2`, a clock-internal fact outside
     C158's boundary, and its doc names the parking clock as the
     boundary it needs); its property kept: the independent job
     completes while two rate waits stay parked, `sleepers() == 2` read
     after `settle`, which resolves at the first park that would
     advance; under a limiter mutant that waits for nothing the
     independent job completes, the runtime parks with nothing parked,
     `settle` wakes and the assertion fails at once. The retirement
     line: `retired the yield loop while clock.sleepers() < 2; held by
     settle`. `stress.rs:44` stays a component test on a plain runtime
     (two fixtures) and is named in the report as class (c) left under
     `HANG`, one mutant.

6. **Check 12 and the breakers**, one commit: in `tools/arch-check.sh`,
   after check 11, "every tokio timer in the daemon sleeps through its
   clock (C160)": under `crates/acquisition-daemon/src`, outside
   `/tests/` and comment lines, `tokio::time::` appears only inside
   `impl Clock for SystemClock` in `ratelimit.rs` (the span found as
   check 10 finds `fn blocking`'s, 491–495) and inside a file's tail
   test module, defined as: a line `#[cfg(test)]` at column zero whose
   next line is `mod <ident> {` at column zero, the module's closing
   brace being the first line after the opener that is exactly `}` at
   column zero, which must be the file's last non-blank line; a file
   where it is not (an item after the module), or where two such
   modules open, is refused by name as unrecognised, in check 12's own
   words. A
   `#[cfg(test)]` on anything else exempts nothing (the item forms at
   `ratelimit.rs` 44–50 are never brace-matched); braces inside string
   literals (`auth.rs:678`) are never counted, since no brace is
   counted at all for the module. Every other site is refused by path
   and line in words that say what to do ("sleep through the daemon's
   Clock, `choke.sleep` or `choke.sleep_until`, or move the test into
   the file's tail test module (C160)"). The header's list gains item
   12, and 465–466's words say the blocking count holds until a runtime
   task observes the completion. In `tools/arch-check-breakers.sh`,
   `ratelimit.rs`, `auth.rs` and `kinds.rs` join the `files=(…)` restore
   list, and five cases:
   - a `tokio::time::sleep` written into a production line of
     `kinds.rs` (outside any test module): refused, naming the line;
   - the same line written into `auth.rs` *before* its tail test
     module, the module kept: refused, naming the line (a reader that
     exempted the whole file would pass this);
   - a `tokio::time::sleep` under a `#[cfg(test)] fn` outside the tail
     module in `kinds.rs`: refused (the reader exempts the tail module
     alone);
   - a multiline production function containing `tokio::time::sleep`,
     ending with its own column-zero `}`, appended after `auth.rs`'s
     tail module: the file refused as unrecognised by check 12's own
     words (a reader that took the last column-zero brace would exempt
     it);
   - a second `spawn_blocking` written into `kinds.rs`: check 10's
     `BLOCKING` refusal (its first breaker; the group's rule of
     2026-10-30, each fixed check gains one).

7. `tools/gate.sh` once per commit on each commit's tree, its exit
   status read on its own line (never through a pipe), and
   `tools/arch-check-breakers.sh` on the tree of Do 6; every golden
   under `crates/*/tests/goldens` byte-identical to the step's parent;
   `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the README unchanged.

## Done when

Each line from the commit alone, its tree or its message; execution
evidence beyond what a message summarises is the report's.

- `decisions/daemon.md` holds C160 as the registry shows it at the
  commit carrying this cut, or as the owner reworded it after.
- `grep -rn "tokio::time" crates/acquisition-daemon/src | grep -v "/tests/"`
  prints `ratelimit.rs`'s two `SystemClock` lines and `auth.rs`'s three
  tail-module lines and nothing else, quoted in Do 3's commit message
  with each of the six retired sites beside the clock call that
  replaced it; `grep -rn "Instant::now()" crates/acquisition-daemon/src
  | grep -v "/tests/"` prints `ratelimit.rs:94` and the exit bound
  thread's two `std` reads and nothing else, quoted in the same message.
- Do 2's commit message carries the two commands and their totals (the
  sum of `nothing_parked` over the door scenarios; for each of the
  three mutants, the hung test and the hook's branch at its last park);
  Do 4's carries the "after" total.
- **If Do 5 was built:** `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park`
  and `c160_the_hooks_count_is_zero_at_rest_and_one_under_a_held_send`
  exist by name; `.cargo/mutants.toml` lists the four occupancy mutants
  by function with the reason and the pin's name;
  `dispatcher_rate_waits_do_not_cap_independent_job_progress` runs on
  the door's runtime through `settle`, its doc naming the parking clock
  as its boundary, the yield loop gone, its retirement line in the
  commit; every door wait's doc says `HANG` is the backstop behind the
  verdict; the commit message names the
  mutants of class (a) the verdict catches by their rows in Input.
- **If Do 5 was not built** (the stop rule): the last commit's message
  names the scenario, the park and what the runtime waited on; Do 3,
  Do 4 and Do 6 stand; the report's "What you left out" says so; #151
  stays open with a comment naming the site.
- Check 12 exists in `tools/arch-check.sh` with its header line, and
  `tools/arch-check-breakers.sh` has the five cases of Do 6, each
  refusing by the words the check prints, the breakers' conclusion (its
  case count and that every case refused by name) in Do 6's commit
  message; `tools/arch-check.sh:465–466`'s words corrected.
- No test under `crates/acquisition-daemon/src/daemon/tests` holds a
  wall-clock bound other than `HANG`.
- Every golden byte-identical; the three references unchanged; every
  log line a test reads unchanged (the worker greps the daemon's `log(`
  lines it touched against `tests/log.rs` and the CLI's
  `daemon_signal_exit.rs`).
- `tools/gate.sh` exits 0 on each commit's tree.

Not this step's done-when, the close's: the weekly workflow dispatched
on a tag at the step's last commit by the orchestrating session, its
four shards under `runs/`, and `python3 -I tools/mutants-compare.py
runs/t5-round6-after runs/<this tag>` showing the class (a) mutants
caught, the four of class (b) as removals (listed, so not run), and no
new survivor; the longest shard against the baseline's 106 minutes,
which #151's "Closes when" names and the close reads honestly, since
class (c)'s one mutant still runs to the timeout.

## Not in scope

- reqwest's connect and request timeouts (`choke.rs:216–217`, 240–241):
  the HTTP client's own timers; the "Not doing" line as amended.
- The exit bound's OS thread (`daemon.rs:273–282`): real by design,
  C133; outside the check by its spelling.
- `advance_until` (`harness.rs:835`), `drive_drain` and `await_a_look`
  (the 1 ms real polls): component helpers on a plain runtime with no
  hook. A logical "nothing parked and the state not reached" verdict
  there is not exact: on a plain runtime a yield loop keeps a task
  ready, the I/O driver is polled between yields, and a fixture's
  answer can still arrive; a count of yields would be the spike's
  refuted door rule again. They stay under `HANG`; `stress.rs:44`'s one
  mutant with them.
- The `beside` door's missing hook (`door.rs:521–`): its rule stands;
  the verdict reads the first daemon's counts, as the hook always did.
- `settle`'s and `quiet`'s meanings and `quiet`'s sentence rule:
  unchanged.
- `HANG`'s value, the nextest profiles, cargo-mutants' timeout:
  unchanged (the retrospective's §6).
- The limiter spec's eight real-clock tests (`ratelimit/tests/spec.rs`
  1925 to 2332): #131's, a later small step.
- The client's and the frontends' tests: cross-process, #131's client
  rows and t6's.
- `IDLE_REJUDGE`, `DRAIN_POLL`, `HALT_POLL`: kept with their reasons.
- A README row for `ACQ_DOOR_COUNTERS`: test support, not a knob.
- Any word a user meets, any `daemon status` field, any protocol
  change.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on
`main`, in a checkout other sessions may share, in a window the owner
names (his machine is his during a league sitting); never push; `acq`
and `acqd` only in a mock world, under the harness; kill only this
session's pids; an experiment is undone from a scratch copy, never
`git checkout -- <file>`. The local mutant runs of Do 2 are measurements:
alone on the machine, no gate beside them. Six commits at most (the
measurement, the timers, the holds, the verdict and the classes, the
check, the gate's last), each gated on its own tree. The group's
review rule binds: every commit here changes non-test code or a tool,
so the step's commits are reviewed cold as one range, and a fix round
that changes behaviour is reviewed again. The attribution line names
the model the worker runs on. The rules on paused tokio time and on
wall-clock bounds bind the thing this step builds: the verdict is not
a bound, and the pin asserts none.

## Report

`TEMPLATE.md`, and: the two `Counters` tables (Do 2, Do 4), each with
its command; the three local mutant runs, each with the hung test and
the hook's last branch; the `sleep` and `whoami` tests whose durations
changed under Do 3, before and after, asserted by nothing; the
breakers' run. Raised for review, at least: whether the verdict's words
should name the counters or only the wait; whether `ACQ_DOOR_COUNTERS`
should stay after the measurement or go with the step; any scenario
Do 4 gave a hold to whose wait was not a socket's; and any mutant of
Input whose class the local run overturned.

## Drafts for the owner

- **C160's narrowed words** (Do 1): entered under the grant, for his
  re-read; a reword from him replaces them and Do 5's words follow.
- **The four listed mutants** (Do 5, class b): an exclusion is a line
  in a file the registry's C122 governs by its comment's rule; the
  worker lists them with the pin's name, and the owner strikes the
  listing if he would rather they stay timeouts in the weekly run.
- **The stop rule** (Do 4), as the first cut had it.

## Taken from the reviews

The first review's eight (`151-stuck-verdict-brief-e1a0dcb8.md`), as
the second review judged them (five answered, finding 1 and finding 2
narrower, finding 5 not answered until this cut):

1. Quiescence is not proof of no future progress (the I/O driver's
   readiness is outside the hook's accounting): C160 narrowed to what
   the hook accounts for, with the scenario's own waits under a hold by
   rule; the measurement confirms the existing scenarios obey the rule
   and is no longer called proof; a false verdict is loud and names the
   scenario (Goal, Do 1, Do 4).
2. Four of the 48 mutate the count the hook reads, so no verdict can
   fire: the 48 classified from the saved logs by the scenario that
   hung (Input); class (b) gets a direct pin and a listed exclusion
   with its reason; class (c) gets the sleepers test onto the door and
   one named leftover; the promise narrowed to class (a) (Goal, Do 5,
   done-when, the close).
3. A plain `select!` is not `timeout` and `Elapsed` is crate-private:
   `within` with the daemon's own `Expired`, `biased`, the future
   first; the callers' arms unchanged; the callback helpers take a
   cloned clock through a new `ChokePoint::clock` (Do 3).
4. The entry over budget and carrying the inventory: 791 bytes,
   "tokio timer", the inventory and the count in this brief (Do 1).
5. The `cfg(test)` reader's boundaries and the breakers' power: the
   tail-module rule with no brace counting, a file refused when
   unrecognised, five cases including a production timer placed before
   `auth.rs`'s test module and an appended item after it; the restore
   list gains the files; §8 added to the inventory (Read, Do 6).
6. The stop rule against unconditional done-when lines: the lines
   conditioned on Do 5 built or not, the stopped outcome defined
   (Done when).
7. Report-only evidence: the commands and their totals in the commit
   messages, the tables in the report (Do 2, Do 4, Do 6, Done when).
8. "The losing arm panics": the selected verdict arm panics (Do 5).

The second review's seven (`151-stuck-verdict-brief-132a43ef.md`):

1. The last column-zero brace is not the module's: the first standalone
   `}` after the opener ends the module and must end the file; the
   appended-item breaker is a multiline production function holding a
   timer, refused by check 12's words (Do 6).
2. "A missed one cannot happen" exceeded the `beside` exception: C160
   qualified to a wait on the hook's own door, a `beside` door keeping
   the ceiling; the Goal likewise (Do 1, Goal).
3. An end-of-scenario print cannot show a killed scenario's last park:
   the hook prints, flushed, at every park that did not advance, with
   the scenario's name; class (c) recorded as "no hook; a runnable
   yield loop" (Do 2).
4. The sleepers test asserting `sleepers()` is a component test, never
   a scenario: on the door's runtime as the blocking-count probe is,
   its boundary named, the retirement the yield loop's (Do 5, Done
   when).
5. An excluded mutant does not run: the four are removals in the close's
   compare; the reason worded as the run killed before a completed
   result (Do 5, the close).
6. 33 of class (a), the overlap marked in its row (Input).
7. Done-when 1 cites the commit carrying this cut (Done when).
