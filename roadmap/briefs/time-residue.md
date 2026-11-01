# time-residue — the seam's leftovers: the spec's clock, the client's ceilings, the boundary value, the mock's window (#131, #152)

The first cut, written by the orchestrating session on 2026-11-01 from
`roadmap/reviews/time-seam-retrospective.md` (rows E1, E2, E3 and C1 of
its §10; the owner's "yes to all" of that day, question 3: a `Boundary`
value at the component constructors meets #152's "names its boundary")
and a re-read of the tree at `8ad6030c`, every line number below from
it. To be reviewed cold before a worker reads it; what each review
changes is listed at the foot. No ruling is needed: every change is
test support, a ceiling, or a knob of the mock's, which is the
session's agent-facing design, listed at the close for the owner's
read. Opus builds it (the owner's word of 2026-11-01: Opus for these,
in series).

## Goal

When this step is done, the daemon's one test clock lives once in the
crate and the limiter's spec runs on it, so no test of the choke point
asserts a wait against real time (#131's last daemon rows); the
client's contract tests wait for a real daemon under the harness's one
ceiling and no shorter number (#131's client rows); every test under
`daemon/tests` either enters by the door or says, as a value at the
constructor that builds its daemon or as a sentence where it builds
none, which boundary keeps it off the door, and a census prints zero
that say neither (#152); and the mock's rate-limit windows can be
shortened by a knob the test harness sets, so the suite's longest test,
an 18.5 s real wait on the mock's stash window, no longer sets the
gate's critical path. No word a user meets changes but one README row
for the knob; no golden changes but by the knob's measured effect, each
read as a diff of what a user reads.

Why it matters: these are the four leftovers the retrospective found
once the seam was built, each small, none a design. Together they take
the last real-time bets out of the daemon's and the client's tests and
make C158's "says which boundary" a value a census reads rather than a
sentence a sweep greps for.

## Read

- `roadmap/reviews/time-seam-retrospective.md` §5 (the measurements:
  the suite's longest test, the daemon's share of the wall), §10 rows
  E1, E2, E3, C1, §11 question 3. `roadmap/briefs/151-stuck-verdict.md`,
  Not in scope (what it left to this step) and its Rules.
- `decisions/daemon.md`: C158 (a daemon scenario crosses the frontend
  boundary only; a test that needs another boundary says which and
  stays a component test: this step makes "says which" a value), C137
  (no wall-clock bound a slow machine can miss; the knobs and the
  isolation list), C160 (every tokio timer through the one clock; the
  verdict), C122 (one harness, every test process takes the isolation
  list). `AGENTS.md`: C153, C154; "Where things live": a knob is a
  README row.
- `crates/acquisition-daemon/src/ratelimit/tests/spec.rs` (the
  limiter's spec, `use super::*` at 14, so it sees `Clock` and
  `SystemClock` and nothing under `daemon/tests`): the twelve choke
  points built on the system clock, `ChokePoint::new()` at 2174, 2217,
  2252, 2303, 2332, 2363, 2384, 2639, 2687, 2756, 2824 and 3210, and
  `with_clock_and_rails(Arc::new(SystemClock), …)` at 1956, 2042, 2467,
  2534 and 3417; the two that assert a wait against real time, `wait >
  Duration::from_secs(5)` at 2209 (`headerless_503_on_an_established_policy_paces_as_if_counted`)
  and 2247 (`transport_failure_on_an_established_policy_paces_as_if_counted`),
  each on a 10 s window the headers name, so a stall over 5 s between
  the send and the check fails them (#131); the 50 ms sleep "let it
  park" at 2063; the 5 s `bound` at 3427. `ratelimit/choke.rs`:
  `ChokePoint::new` (257, the system clock and default rails) and
  `with_clock_and_rails` (275). `daemon/tests/harness.rs`:
  `ParkingClock` (37–212, `pub(super)` to `daemon::tests`), its
  `advance`, `advance_to`, `advance_to_next`, `sleepers`,
  `next_deadline`, `laptop_sleep`; `advance_until` (835); `HANG` (849).
- `crates/acquisition-client/tests/contract.rs`: the nine waits of ten
  seconds on a healthy path, `wait_exit(Duration::from_secs(10))` at
  242 (`c85_the_daemon_identifies_itself_and_stops_across_a_contract_mismatch`),
  756 (`c83_a_daemon_on_another_world_is_refused_and_never_replaced`),
  1364 and 1600 (`c83_a_daemon_on_its_way_out_never_unlinks_a_successors_socket`),
  and `tokio::time::timeout(Duration::from_secs(10), subscription.next())`
  at 1291, 1327 (`c85_a_subscription_carries_events_only_and_lag_is_resync_required`),
  1352, 1365, 1392 (`c85_a_subscriber_sees_the_disconnect_and_subscribes_and_snapshots_again_after_a_restart`).
  `tests/harness/mod.rs`: `wait_exit(within)` (337, a 50 ms poll under
  the bound it is given), the 120 s ceilings at 444, 971, 1018, the
  isolation list (164–186) and its knobs (179–184), `set_knob` (953).
  The other short bounds under `crates/acquisition-client/tests/*.rs`
  outside `harness/` (twelve lines by `grep -rn "from_secs([1-9])\|from_secs(1[0-9])\|from_millis"`,
  `ZERO` and `HELLO_WITHIN` excepted), which Do 2 inventories: each is
  a healthy-path ceiling (this step's), a sleep that proves a wait
  (#131's second list, left), or a forced expiry (kept).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`: the three
  component constructors, `test_daemon(base, clock)` (651),
  `test_daemon_with(provider, clock, config)` (736) and
  `test_daemon_at(provider, clock, config, log_path, sessions)` (753),
  called 43 times from 13 files outside the harness (`grep -rn
  "test_daemon(\|test_daemon_with(\|test_daemon_at("`: `credentials.rs`
  5, `dispatch.rs` 7, `evict.rs` 3, `exit.rs` 6, `idle.rs` 4, `list.rs`
  1, `log.rs` 3, `login.rs` 1, `parents.rs` 4, `queue.rs` 4, `quote.rs`
  1, `refresh.rs` 3, `stress.rs` 1). `daemon/tests/door.rs`'s module doc, its paragraph "What stays off
  the door", the boundaries C158 names (several workers; a child process; the daemon's
  own frontend or callback socket; a function or a table; the limiter's
  spec) and the convention the census reads today, a doc sentence
  saying "a component test" or "a component case" (`idle.rs` 0 of 4
  tests, `list.rs` 0 of 6, `log.rs` 1 of 14, `evict.rs` 0 of 7,
  `credentials.rs` 3 of 24, `exit.rs` 19 of 55, `responses.rs` 0 of 7,
  by `grep -c`). #152's text: the 54 at t5's close, the census a
  report-only script, "the sweep may commit a small reader under
  `tools/` if a second census wants one".
- `crates/acquisition-daemon/src/mockggg.rs`: `MockPolicy` (58–150,
  `windows: &'static [(u32, u64, u64)]`, max hits, period in seconds,
  restriction in seconds, N23), `policy_template` (163–185: `/stash`
  `(10, 15, 60), (30, 60, 300)`; `/stash/tab` `(15, 10, 60), (30, 300,
  300)`; `/fetch` `(5, 10, 60), (30, 300, 300)`; the character and
  league policies; `/token` `(60, 30, 30)`), called once per path at
  214 on first contact; `start` and `start_with_clock` (602–625: the
  clock the windows count on); `ACQ_MOCK_DEGRADED_HEAD` (794–796), the
  mock's one knob today, its README row at `README.md:275`. The CLI's
  `demo` help ("the mock's 5-per-10 s policy", `main.rs:462`), unchanged
  with the knob unset. `crates/acquisition-cli/tests/new_league.rs`
  (55–110: the applies before the plan that fill the stash window, the
  golden `goldens/new_league/launch_morning.txt:72` holding
  `stash-request-limit  7 requests — ~` with the seconds masked); the
  client harness's masking of an ETA's seconds (the `~<eta>s` the
  golden holds), which normalises a number, not a word. The retrospective's measurement:
  that test 18.5 s, the suite 23.4 s wall.
- `README.md:258–276`, the knob table's form; `CLI-REFERENCE.md` and
  `MCP-REFERENCE.md`, unchanged by this step.

## Input

Fixed before the work starts:

- **The spec's real-clock sites**: the seventeen constructions above,
  two of them asserting elapsed time; the measure of the class is
  `grep -n "ChokePoint::new()\|Arc::new(SystemClock)" crates/acquisition-daemon/src/ratelimit/tests/spec.rs`,
  seventeen lines at `8ad6030c`.
- **The client's ten-second waits**: the nine lines above, by `grep -n
  "from_secs(10)" crates/acquisition-client/tests/contract.rs`.
- **The census at `8ad6030c`**, by the counts in Read: of the files
  with no door test, the tests saying "component" in their own doc are
  4 of 62 (`idle.rs`, `list.rs`, `log.rs`, `evict.rs`, `credentials.rs`,
  `responses.rs`); #152 counted 54 tests saying neither at `e139e522`
  across the suite. Do 3 recounts with its own reader before and after.
- **The suite's wall**: 1,407 tests in 23.4 s, the longest
  `new_league::a_new_league_starts_empty_beside_standard_and_a_typo_is_named`
  at 18.5 s, the next two 6.1 s and 5.6 s (the retrospective's run of
  `cargo nextest run --workspace --all-targets --no-fail-fast`, this
  machine, 2026-11-01). Do 4 measures the same way before and after.
- **The last push's CI**: `Gate (Linux)` 41360883340 and `Gate
  (Windows)` 41360883331 on `8ad6030c`, both success. Re-read at the
  step's start; a red run is this step's first concern or is filed.

## Do

In this order, one commit each, each gated on its own tree.

1. **The crate's one test clock, and the spec on it.** `ParkingClock`
   moves from `daemon/tests/harness.rs` to a crate-level test-support
   module (`crates/acquisition-daemon/src/testing.rs` under
   `#[cfg(test)]`, or the module the worker names), `pub(crate)`, the
   harness re-exporting it so the constructions under `daemon/tests`
   read as before (111 by `grep -c "ParkingClock::new()"` summed over `daemon/tests/*.rs`); then
   every choke point the spec builds on the system clock takes a
   parking clock through `with_clock_and_rails` (the seventeen sites;
   `ChokePoint::new` stays for production). The two tests that asserted
   `wait > 5 s` assert the wait the rules give exactly, read from the
   headers each test sends (the spec's own form: these headers and this
   history give this wait), since on a parking clock no time passes
   between the send and the check; the 50 ms "let it park" sleep at
   2063 becomes a wait on `sleepers()` under `HANG`; the 5 s `bound` at
   3427 is read and classified in the message (a forced expiry stays,
   a ceiling becomes `HANG`). The retirement lines: `retired wait > 5s
   at <test>; held by the exact wait on the parking clock`, one per
   test. `cargo nextest run -p acquisition-daemon` before and after,
   the spec's slowest tests named with their durations, asserted by
   nothing.

2. **The client's ceilings.** The nine ten-second waits become the
   harness's ceiling: `wait_exit` called with the 120 s the harness
   uses elsewhere (a named constant in `harness/mod.rs`, replacing the
   three literal `from_secs(120)`s at 444, 971 and 1018 too), and the
   seven subscription reads under the same constant. Then the inventory
   of the twelve other short bounds, each classified in the commit
   message as a healthy-path ceiling (made the constant), a sleep that
   proves a wait (left, #131's), or a forced expiry (kept, with the
   knob it forces); a line that fits no class fails the step. No test
   under `crates/acquisition-client/tests` then waits for a healthy
   daemon under a number smaller than the ceiling.

3. **The boundary as a value.** `enum Boundary` in the daemon's test
   support, one variant per boundary C158 and the door's doc name:
   `Workers` (several workers), `Child` (a child process), `Socket`
   (the daemon's own frontend or callback socket), `Function` (a
   function or a table, where the test builds a daemon only to call
   one), `Clock` (a hand-driven clock the test reads or moves itself,
   as `idle.rs` does), `Fixtures` (two fixtures, `stress.rs`). The
   three constructors take it as their first argument, and the 43 call
   sites name theirs; a test that builds no daemon and enters no door
   (the `Life` table, `responses.rs`, `log.rs`'s pure tests) keeps the
   doc sentence. A reader, `tools/daemon-tests-census.py`, report-only
   and not a gate stage (evidence before ceremony: a second census
   wanted one), prints per test function under `daemon/tests/*.rs` one
   of: door (the function body reaches `Door::open` or a `DoorBuilder`
   `.run(`), a boundary (the constructor's first argument), a sentence
   ("a component test" or "a component case" in its doc), or neither;
   its totals before (at `8ad6030c`) and after in the commit message.
   After: zero neither. The doc sentences already written stay; a value
   and a sentence may both stand. The census's case-loop rule (a test
   over a row vector) is the round 6 script's, restated in the reader's
   docstring.

4. **The mock's window, behind a measurement.** First the measurement:
   with a divisor applied by hand in a scratch copy of `mockggg.rs`
   (every window's period and restriction divided by ten, the hit
   counts kept), the three frontend suites and the client's run
   (`cargo nextest run --workspace --all-targets --no-fail-fast`),
   every failing test and every changed golden listed in the report
   with what changed in its words. Then the knob, if the measurement
   allows it: `ACQ_MOCK_WINDOW_DIVISOR=<n>` read once at the mock's
   start (`start_with_clock`), default 1, dividing each policy's period
   and restriction (seconds, never below one) and nothing else; unset
   or 1, the mock's windows are what they are today, so `demo` and the
   README's words stand; an unparseable or zero value refused at start
   in words that say the form. The client harness's isolation list sets
   it to 10 for every test daemon. A README row in the knob table, in
   the row's form, beside `ACQ_MOCK_DEGRADED_HEAD`. Then each golden the
   measurement moved is read as a diff of what a user reads (shape 9):
   a line whose words changed because a window now drains inside the
   test is a test whose property depends on the window outlasting it,
   and that test sets the knob to 1 for its own daemon through
   `set_knob` and says why; a line that differs only in a masked number
   is the `eta` rule's and changes nothing. **The stop rule:** if the
   measurement moves a golden the worker cannot explain by one of those
   two readings, or more than five tests, the knob lands with the
   isolation list leaving it unset (the knob a user may set, the suite
   unchanged), the report names each moved test, and the retrospective's
   E3 stays open as a comment on #131. The suite's wall and its longest
   test before and after, by the retrospective's command, asserted by
   nothing.

5. `tools/gate.sh` once per commit on each commit's tree, its exit
   status read on its own line; every golden under `crates/*/tests/goldens`
   byte-identical to the step's parent except those Do 4 names, each
   with its reading in the message; `CLI-REFERENCE.md` and
   `MCP-REFERENCE.md` unchanged; the README changed by one row.

## Done when

Each line from the commit alone, its tree or its message.

- `grep -n "ChokePoint::new()\|Arc::new(SystemClock)" crates/acquisition-daemon/src/ratelimit/tests/spec.rs`
  prints nothing, quoted in Do 1's message; `ParkingClock` is defined
  once in the crate (`grep -rn "struct ParkingClock"` prints one line);
  the two retirement lines are in the message; no test under
  `ratelimit/tests` asserts a wait against an inequality with a literal
  duration (`grep -n "wait > Duration\|wait >= Duration"` prints
  nothing).
- `grep -n "from_secs(10)" crates/acquisition-client/tests/contract.rs`
  prints nothing; the harness has one named ceiling constant and no
  literal `from_secs(120)` outside its definition; Do 2's message
  classifies the twelve other lines.
- `tools/daemon-tests-census.py` exists, report-only, and its output at
  Do 3's tree (quoted in the message with the before) shows zero
  "neither"; the three constructors take `Boundary` first and
  `grep -rn "Boundary::" crates/acquisition-daemon/src/daemon/tests`
  counts at least 43 sites.
- Do 4's message carries the measurement (the command, the failing
  tests, the changed goldens with their words) and either the knob with
  the isolation list set to 10 and each moved golden's reading, or the
  knob with the list leaving it unset and the stop rule's reason; the
  README has the row; `mockggg.rs` reads the knob once at start and
  refuses a bad value in words that say the form (a test pins the
  refusal and the default).
- The suite's wall and longest test, before and after, in Do 4's
  message, asserted by nothing.
- `tools/gate.sh` exits 0 on each commit's tree.

## Not in scope

- The daemon's production timers, the verdict, check 12: #151's step,
  closed. `advance_until`'s yield loop: kept, the stuck-verdict brief's
  Not in scope says why.
- The frontends' own process tests' sleeps (the CLI's, the MCP's, the
  verbs'): cross-process, t6's (#142) and #131's second list.
- Moving any component test onto the door: #152 asks for the value or
  the sentence, never a move; a test the census finds door-able is its
  own issue.
- A gate stage for the census: not until a second census is wanted
  (evidence before ceremony); the reader is report-only.
- The mock's hit counts and its `demo` words: unchanged; the divisor
  touches periods and restrictions alone.
- Any protocol or user-facing change beyond the one README row.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on `main`,
in a checkout other sessions may share, in a window the owner names;
never push; `acq` and `acqd` only in a mock world under the harness
(the `mock-session` skill); kill only this session's pids; an
experiment (Do 4's scratch copy of `mockggg.rs`) is undone from a
scratch copy, never `git checkout -- <file>`; `cargo build --workspace`
before anything that starts a daemon; the gate's exit status read on
its own line, never through a pipe. Four commits, each gated on its own
tree. The group's review rule binds: Do 4 and the census tool change
non-test code and `tools/`, so the step's commits are reviewed cold as
one range, and a fix round that changes behaviour is reviewed again.
The attribution line names the model the worker runs on. The knob is
agent-facing design: the session's, recorded in Do 4's commit and
listed at the close for the owner's read.

## Report

`TEMPLATE.md`, and: the spec's slowest tests before and after Do 1;
the census totals before and after Do 3; Do 4's measurement whole (the
failing tests, the moved goldens, each with its words); the suite's
wall and longest test before and after. Raised for review, at least:
any constructor site whose boundary the worker could not name from the
test's own doc; any golden whose change the two readings of Do 4 did
not cover; and whether the census reader should become a gate stage.

## Taken from the reviews

None yet: this is the first cut.
