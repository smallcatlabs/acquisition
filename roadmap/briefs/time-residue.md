# time-residue — the seam's leftovers: the spec's clock, the client's ceilings, the boundary value, the mock's window (#131, #152)

Cut three times: the first draft (`64c91def`) and the re-cut
(`a7904b32`), each reviewed cold
(`roadmap/reviews/time-residue-brief-<sha>.md`, the prompt beside: nine
findings, then three, two nitpicks each); this cut answers the second
review's three, listed at the foot under "Taken from the reviews" with
the first's nine. Written by the orchestrating session on
2026-11-01 from `roadmap/reviews/time-seam-retrospective.md` (rows E1,
E2, E3 and C1 of its §10; the owner's "yes to all" of that day,
question 3: a `Boundary` value at the component constructors meets
#152's "names its boundary") and a re-read of the tree at `d326cfa5`,
every line number below from it. To be reviewed cold again before a
worker reads it. No ruling is needed: every change is test support, a
ceiling, or a knob of the mock's, which is the session's agent-facing
design, listed at the close for the owner's read. Opus builds it (the
owner's word of 2026-11-01: Opus for these, in series).

## Goal

When this step is done, the daemon's one test clock lives once in the
crate and the limiter's spec runs on it, so no test of the choke point
asserts a wait against real time (#131's last daemon rows); the
client's contract tests wait for a real daemon under the harness's one
ceiling and no shorter number (#131's client rows); every attributed
test under `daemon/tests` either enters by the door or says, as a value
at the constructor that builds its daemon or as a sentence in its own
or its module's doc where it builds none or builds one directly, which
boundary keeps it off the door, and a census prints zero that say
neither (#152); and the mock's rate-limit windows can be shortened by a
knob. Whether the test harness sets that knob is what Do 4's
measurement decides: where it can, the suite's longest test, an 18.5 s
real wait on the mock's stash window, stops setting the gate's critical
path; where the rehearsal that waits must keep its window, the knob
lands for a user to set, the suite stands, and the retrospective's E3
stays open on #131 with the reason. No word a user meets changes but one
README row for the knob; no golden changes but by the knob's measured
effect, each read as a diff of what a user reads.

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
  `roadmap/reviews/time-residue-brief-64c91def.md`, the first review:
  finding 1 (a GET parked on the gate's oneshot is no clock sleeper),
  finding 3 (`set_knob` against the isolation list a spawned daemon
  reapplies), finding 5 (what the census reader must traverse and
  accept), finding 7 (the rehearsal that requires a wait).
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
  `SystemClock` and nothing under `daemon/tests`): eighteen choke
  points built on the system clock, thirteen `ChokePoint::new()` at
  1925, 2174, 2217, 2252, 2303, 2332, 2363, 2384, 2639, 2687, 2756,
  2824 and 3210, and five `with_clock_and_rails(Arc::new(SystemClock), …)`
  at 1956, 2042, 2467, 2534 and 3417 (`grep -n "ChokePoint::new()\|Arc::new(SystemClock)"`,
  eighteen lines); the two that assert a wait against real time,
  `wait > Duration::from_secs(5)` at 2209
  (`headerless_503_on_an_established_policy_paces_as_if_counted`) and
  2247 (`transport_failure_on_an_established_policy_paces_as_if_counted`),
  each on a window the headers name, so a stall over 5 s between the
  send and the check fails them (#131); the first review read the
  exact wait those headers give as 16 s, which the worker confirms from
  the headers before asserting it. The 50 ms sleep "let it park" at
  2063, before a trip, waits for a GET that is parked on the send
  gate's oneshot (`choke.rs:409` `self.gate.acquire(…).await`, `gate.rs:139`),
  which is no clock sleep, so `sleepers()` never counts it. The five
  tokio completion bounds, `tokio::time::timeout(Duration::from_secs(3), get)`
  at 2085 and the four one-second bounds on arrival receivers at 2709,
  2782, 2951 and 2964, which a parking clock does not touch; the 5 s
  `bound` at 3427. `ratelimit/choke.rs`: `ChokePoint::new` (257, the
  system clock and default rails) and `with_clock_and_rails` (275).
  `daemon/tests/harness.rs`: `ParkingClock` (37–212, `pub(super)` to
  `daemon::tests`), its `advance`, `advance_to`, `advance_to_next`,
  `sleepers`, `next_deadline`, `laptop_sleep`; `advance_until` (835);
  `HANG` (849, `pub(super)` likewise, so the spec cannot see it today).
- `crates/acquisition-client/tests/contract.rs`: the nine ten-second
  waits on a healthy path, four exit waits
  `wait_exit(Duration::from_secs(10))` at 242
  (`c85_the_daemon_identifies_itself_and_stops_across_a_contract_mismatch`),
  756 (`c83_a_daemon_on_another_world_is_refused_and_never_replaced`),
  1364 (`c85_a_subscriber_sees_the_disconnect_and_subscribes_and_snapshots_again_after_a_restart`)
  and 1600 (`c83_a_daemon_on_its_way_out_never_unlinks_a_successors_socket`),
  and five subscription reads
  `tokio::time::timeout(Duration::from_secs(10), subscription.next())`
  at 1291, 1327 (`c85_a_subscription_carries_events_only_and_lag_is_resync_required`),
  1352, 1365 and 1392 (the restart test); the three polling intervals,
  `sleep(20 ms)` at 894 and 982 and `sleep(50 ms)` at 1083, the cadence
  of a loop under a ceiling, which prove no wait and force no expiry;
  the default's assertion `HELLO_WITHIN == 5 s` at 367, a value. That is
  the whole of `grep -rn "from_secs([1-9])\|from_secs(1[0-9])\|from_millis"`
  under `crates/acquisition-client/tests/*.rs` outside `harness/`:
  thirteen matches, the twelve timing sites (the nine waits and the
  three intervals) and the default-value assertion. `tests/harness/mod.rs`:
  `wait_exit(within)` (337, a 50 ms poll under the bound it is given),
  the 120 s ceilings at 444, 971 and 1018, `wait_all_terminal`'s 60 s
  at 1039–1043 (called from `contract.rs` 1287 and 1349, the
  subscription tests), the isolation list (164–186) and its knobs
  (179–184), `daemon_command_at` (376–380, `isolate_at` reapplied to
  every daemon command), `start_daemon_with`'s per-daemon `env` applied
  after it (440–444), `set_knob` (953, the parent's environment, which
  a spawned daemon's isolation overwrites).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`: the three
  component constructors, `test_daemon(base, clock)` (651),
  `test_daemon_with(provider, clock, config)` (736) and
  `test_daemon_at(provider, clock, config, log_path, sessions)` (753),
  called 43 times from 13 files outside the harness (`grep -rn
  "test_daemon(\|test_daemon_with(\|test_daemon_at("`: `credentials.rs`
  5, `dispatch.rs` 7, `evict.rs` 3, `exit.rs` 6, `idle.rs` 4, `list.rs`
  1, `log.rs` 3, `login.rs` 1, `parents.rs` 4, `queue.rs` 4, `quote.rs`
  1, `refresh.rs` 3, `stress.rs` 1), some through a helper of the
  file's own that the test reaches by a call, one level deep
  (`list.rs:50` `learned_daemon`) or deeper (`credentials.rs:649` calls
  `restoring_daemon`, which calls `restoring_daemon_on` at 568, which
  builds at 575; `exit.rs:1227` and `:1232` call a helper whose own doc
  at 1194 says "component test", which calls `exit_daemon`, then
  `exit_daemon_on` at 77, which builds at 83); two tests build
  a daemon directly through `Daemon::builder` (`login.rs:73`,
  `sessions.rs:193`); pure tests that build none (the `Life` table,
  `exit.rs:1095`, with no sentence today); a module doc that names the
  boundary for every test of its file (`responses.rs:2`, "the
  classification's component table"). `daemon/tests/door.rs`'s module
  doc, its paragraph "What stays off the door", the boundaries C158
  names (several workers; a child process; the daemon's own frontend or
  callback socket; a function or a table; the limiter's spec) and the
  convention the census reads today, a doc sentence saying "a component
  test" or "a component case". #152's text: 54 at `e139e522` by its
  script, since lost; "the sweep may commit a small reader under
  `tools/` if a second census wants one".
- `crates/acquisition-daemon/src/mockggg.rs`: `MockPolicy` (58–150,
  `windows: &'static [(u32, u64, u64)]`, max hits, period in seconds,
  restriction in seconds, N23; the restriction a 429 starts, 114–137),
  `policy_template` (163–185: `/stash` `(10, 15, 60), (30, 60, 300)`;
  `/stash/tab` `(15, 10, 60), (30, 300, 300)`; `/fetch` `(5, 10, 60),
  (30, 300, 300)`; the character and league policies; `/token` `(60,
  30, 30)`), called once per path at 214 on first contact; `start` and
  `start_with_clock` (602–625: the clock the windows count on);
  `ACQ_MOCK_DEGRADED_HEAD` (794–796), the mock's one knob today, its
  README row at `README.md:275`. The CLI's `demo` help ("the mock's
  5-per-10 s policy", `main.rs:462`), unchanged with the knob unset.
  `crates/acquisition-cli/tests/new_league.rs`: the daemon started
  explicitly at 48, the applies before the plan that fill the stash
  window (55–60), the step at 103 that requires a wait ("a wait, not
  `ready`: the golden masks its seconds"), the golden
  `goldens/new_league/launch_morning.txt:72` holding
  `stash-request-limit  7 requests — ~`; the CLI harness's `eta_rule`
  (`crates/acquisition-cli/tests/harness/mod.rs:1113`, applied at 706),
  which masks an ETA's seconds (the `~<eta>s` the golden holds) and
  normalises a number, not a word, so `ready` and a masked ETA are
  told apart. The retrospective's measurement: that test 18.5 s, the
  suite 23.4 s wall.
- `README.md:258–276`, the knob table's form; `CLI-REFERENCE.md` and
  `MCP-REFERENCE.md`, unchanged by this step.

## Input

Fixed before the work starts:

- **The spec's real-clock sites**: the eighteen constructions above,
  two asserting elapsed time, five tokio completion bounds beside them;
  the measure of the class is the grep in Read, eighteen lines at
  `d326cfa5`.
- **The client's short bounds**: the twelve lines above, nine waits and
  three intervals, by the grep in Read; `wait_all_terminal`'s 60 s.
- **The census at `d326cfa5`**, attributed tests (`#[test]` and
  `#[tokio::test]`, by `grep -c` per file): 197 across the nineteen
  files under `daemon/tests`, `door.rs`'s one (1367) among them; in the
  eight files with no door test (`idle.rs` 4, `list.rs`
  2, `log.rs` 8, `evict.rs` 3, `credentials.rs` 11, `responses.rs` 5,
  `diagnostics.rs` 1, `stress.rs` 1: 35), four tests say "component"
  in their own doc (`log.rs` 1, `credentials.rs` 3) and `responses.rs`'s
  five are covered by their module doc. #152 counted 54 "neither" at
  `e139e522` over every file by its own script. Do 3's reader recounts
  before and after by the rules it states; its before is the number
  the close reports, not these.
- **The suite's wall**: 1,407 tests in 23.4 s, the longest
  `new_league::a_new_league_starts_empty_beside_standard_and_a_typo_is_named`
  at 18.5 s, the next two 6.1 s and 5.6 s (the retrospective's run of
  `cargo nextest run --workspace --all-targets --no-fail-fast`, this
  machine, 2026-11-01). Do 4 measures the same way before and after.
- **The last push's CI**: `Gate (Linux)` 41360883340 and `Gate
  (Windows)` 41360883331 on `8ad6030c`, both success. The commits since
  are documents. Re-read at the step's start; a red run is this step's
  first concern or is filed.

## Do

In this order, one commit each, each gated on its own tree.

1. **The crate's one test clock, and the spec on it.** `ParkingClock`
   and `HANG` move from `daemon/tests/harness.rs` to a crate-level
   test-support module (`crates/acquisition-daemon/src/testing.rs`
   under `#[cfg(test)]`, or the module the worker names), `pub(crate)`,
   the harness re-exporting both so the constructions under
   `daemon/tests` read as before (111 by `grep -c "ParkingClock::new()"`
   summed over `daemon/tests/*.rs`); then every choke point the spec
   builds on the system clock takes a parking clock through
   `with_clock_and_rails` (the eighteen sites; `ChokePoint::new` stays
   for production). The two tests that asserted `wait > 5 s` assert the
   wait the rules give exactly, read from the headers each test sends
   (the spec's own form: these headers and this history give this
   wait), since on a parking clock no time passes between the send and
   the check. The 50 ms "let it park" sleep at 2063 goes: the GET is
   pinned and polled once through `std::future::poll_fn` before the
   trip, so it registers at the gate's oneshot deterministically, no
   sleep and no count (a parked sender is no clock sleeper). The five
   tokio completion bounds (2085, 2709, 2782, 2951, 2964) and the 5 s
   `bound` at 3427 are each classified in the message: a healthy-path
   ceiling becomes `HANG`, now visible to the spec; a bound that proves
   a send is held (a timeout whose expiry is the assertion) is a sleep
   that proves a wait, #131's second list, kept and named; a forced
   expiry is kept with what forces it. A line that fits no class fails
   the step. The retirement lines: `retired wait > 5s at <test>; held
   by the exact wait on the parking clock`, one per test. `cargo
   nextest run -p acquisition-daemon` before and after, the spec's
   slowest tests named with their durations, asserted by nothing.

2. **The client's ceilings.** The harness gains one named ceiling
   constant (120 s) replacing its three literals at 444, 971 and 1018;
   the four exit waits and the five subscription reads take it, and
   `wait_all_terminal`'s 60 s becomes it too. The three polling
   intervals (894, 982, 1083) are a loop's cadence under a ceiling and
   stay, named as such in the message with the ceiling each runs under.
   After this commit no test under `crates/acquisition-client/tests`
   waits for a healthy daemon under a number smaller than the ceiling,
   and the message classifies every one of the twelve lines and
   `wait_all_terminal`.

3. **The boundary as a value.** `enum Boundary` in the daemon's test
   support, one variant per boundary C158 and the door's doc name:
   `Workers` (several workers), `Child` (a child process), `Socket`
   (the daemon's own frontend or callback socket), `Function` (a
   function or a table, where the test builds a daemon only to call
   one), `Clock` (a hand-driven clock the test reads or moves itself,
   as `idle.rs` does), `Fixtures` (two fixtures, `stress.rs`). The
   three constructors take it as their first argument, and every call
   site names its own, a helper's call sites included (a helper that
   builds a daemon for several tests takes the boundary from its
   caller, or names the one every caller shares). A test that builds a
   daemon directly (`Daemon::builder` at `login.rs:73` and
   `sessions.rs:193`) or builds none (the `Life` table at `exit.rs:1095`,
   `responses.rs`, `log.rs`'s pure tests) says its boundary in its own
   doc or its module's; `exit.rs:1095` gains its sentence. A reader,
   `tools/daemon-tests-census.py`, report-only and not a gate stage
   (evidence before ceremony: a second census wanted one), counts each
   attributed test function once (`#[test]` or `#[tokio::test]`
   immediately above a `fn`) and prints for it one of: door (its body,
   or a helper in the same file it calls, reaches `Door::open` or a
   `DoorBuilder`'s `.run(`), a boundary (its body, or such a helper,
   reaches one of the three constructors, the first argument read), a
   sentence ("a component test", "a component case" or "component
   table" in its own doc or its module's), or neither; a test with both
   a value and a sentence counts under the value. Helper chains within
   the file are followed to any depth, cycle-safe (each helper visited
   once per test), and a `Boundary` a helper forwards from its own
   arguments resolves to the value the test passes; a helper's own doc
   sentence is no sentence for the tests that call it (`exit.rs:1194`'s
   covers that function alone), so `exit.rs:1227` and `:1232` take
   their value through the chain or gain their own sentence; a test
   that reaches a constructor only through a helper in another file is
   "neither" until that helper takes the boundary from its caller. The
   reader's scope is every file under `daemon/tests`, `door.rs`
   included. The reader counts tests, never rows: no case loop is
   expanded. Its totals before (at `d326cfa5`)
   and after in the commit message. After: zero neither.

4. **The mock's window, behind a measurement.** First the measurement:
   with a divisor applied by hand in a scratch copy of `mockggg.rs`
   (every window's period and restriction divided by ten, each floored
   at one second, the hit counts kept; the token policy's `(60, 30, 30)`
   among them), the whole suite (`cargo nextest run --workspace
   --all-targets --no-fail-fast`), every failing test listed in the
   report with what failed (a fact assertion before any golden
   comparison counts as a failing test) and every changed golden with
   what changed in its words. Then the knob, if the measurement allows
   it: `ACQ_MOCK_WINDOW_DIVISOR=<n>` read once at the mock's start
   (`start_with_clock`), default 1, dividing each policy's period and
   restriction (seconds, floored at one) and nothing else; unset or 1,
   the mock's windows are what they are today, so `demo` and the
   README's words stand; an unparseable or zero value refused at start
   in words that say the form, pinned with the default by a unit test
   in `mockggg.rs`. Where the harness may set it, the client harness's
   isolation list sets it to 10 for every test daemon it spawns, and a
   test whose property depends on a window outlasting it keeps its own
   daemon at 1 through `start_daemon_with`'s per-daemon environment
   (applied after the isolation list, `harness/mod.rs:440–444`; never
   `set_knob`, which the isolation a spawned daemon reapplies
   overwrites), saying why in a comment. Each golden the measurement
   moved is read as a diff of what a user reads (shape 9): a line
   whose words changed because a window now drains inside the test is
   that test's own daemon kept at 1; a line that differs only in a
   masked number is the ETA mask's and changes nothing; a line that
   fits neither reading is a finding for the report. **The stop rule,
   over all affected tests, fact failures and goldens alike:** if the
   measurement moves a test the two readings do not cover, or more than
   five tests in all, the knob lands with the isolation list leaving it
   unset (the knob a user may set, the suite unchanged), the report
   names each moved test, and the retrospective's E3 stays open as a
   comment on #131 with the reason. **The rehearsal:** `new_league.rs`
   starts its daemon explicitly (48) and requires a wait at 103; if the
   measurement shows that wait vanishing under the divisor, the
   rehearsal keeps its daemon at 1 and the suite's critical path stays
   18.5 s, which the message says plainly, E3 staying open for that
   test alone. A README row in the knob table, in the row's form,
   beside `ACQ_MOCK_DEGRADED_HEAD`, whichever way the measurement went.
   The suite's wall and its longest test before and after, by the
   retrospective's command, asserted by nothing.

5. `tools/gate.sh` once per commit on each commit's tree, its exit
   status read on its own line; every golden under `crates/*/tests/goldens`
   byte-identical to the step's parent except those Do 4 names, each
   with its reading in the message; `CLI-REFERENCE.md` and
   `MCP-REFERENCE.md` unchanged; the README changed by one row.

## Done when

Each line from the commit alone, its tree or its message, except the
last, which names the execution evidence and where it lives.

- `grep -n "ChokePoint::new()\|Arc::new(SystemClock)" crates/acquisition-daemon/src/ratelimit/tests/spec.rs`
  prints nothing, quoted in Do 1's message; `ParkingClock` and `HANG`
  are each defined once in the crate (`grep -rn "struct ParkingClock\|const HANG"`
  prints one line each); the two retirement lines are in the message;
  no test under `ratelimit/tests` asserts a wait against an inequality
  with a literal duration (`grep -n "wait > Duration\|wait >= Duration"`
  prints nothing); the message classifies the five completion bounds
  and the 5 s `bound`, and `spec.rs:2063`'s sleep is gone.
- `grep -n "from_secs(10)" crates/acquisition-client/tests/contract.rs`
  prints nothing; the harness has one named ceiling constant, no literal
  `from_secs(120)` outside its definition and no `from_secs(60)` in
  `wait_all_terminal`; Do 2's message classifies the twelve lines and
  `wait_all_terminal`.
- `tools/daemon-tests-census.py` exists, report-only, states its rules
  in its docstring as Do 3 gives them, and its output at Do 3's tree
  (quoted in the message with the before) shows zero "neither"; the
  three constructors take `Boundary` first and `grep -rn "Boundary::"
  crates/acquisition-daemon/src/daemon/tests` counts at least 43 sites;
  `exit.rs`'s `Life` table has its sentence.
- Do 4's message carries the measurement (the command, the failing
  tests with what failed, the changed goldens with their words) and
  either the knob with the isolation list set to 10 and each moved
  test's reading (a daemon kept at 1 through `start_daemon_with`, or
  the ETA mask), or the knob with the list leaving it unset and the
  stop rule's reason; the README has the row; `mockggg.rs` reads the
  knob once at start, floors each divided second at one, and refuses a
  bad value in words that say the form, a unit test pinning the refusal
  and the default.
- The suite's wall and longest test, before and after, in Do 4's
  message, asserted by nothing; where the rehearsal kept its window,
  the message says the critical path stands.
- The gate's exit status on each commit's tree is execution evidence:
  the worker's report carries it per stage for each commit, the
  orchestrating session re-runs the gate on the step's final tree, and
  the close's message names the trees and the results.

## Not in scope

- The daemon's production timers, the verdict, check 12: #151's step,
  closed. `advance_until`'s yield loop: kept, the stuck-verdict brief's
  Not in scope says why.
- The frontends' own process tests' sleeps (the CLI's, the MCP's, the
  verbs'): cross-process, t6's (#142) and #131's second list; the
  client's three polling intervals, a loop's cadence.
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

`TEMPLATE.md`, and: the gate's statuses per stage for each of the four
commits; the spec's slowest tests before and after Do 1; the census
totals before and after Do 3; Do 4's measurement whole (the failing
tests with what failed, the moved goldens, each with its words); the
suite's wall and longest test before and after. Raised for review, at
least: any constructor site whose boundary the worker could not name
from the test's own doc; any golden or failure the two readings of
Do 4 did not cover; and whether the census reader should become a gate
stage.

## Taken from the reviews

The first review's nine (`time-residue-brief-64c91def.md`), as the
second review judged them (eight answered, finding 5 narrower until
this cut):

1. A `sleepers()` wait cannot see a GET parked on the gate's oneshot:
   the GET is pinned and polled once before the trip (Read, Do 1).
2. `wait_all_terminal`'s 60 s left under the ceiling: converted, and in
   done-when (Do 2).
3. `set_knob` is overwritten by the isolation a spawned daemon
   reapplies: the per-daemon environment of `start_daemon_with`, after
   isolation (Do 4).
4. Four exit waits and five subscription reads, three polling intervals
   unclassed: the inventory corrected, the intervals a class kept as
   cadence (Read, Do 2).
5. The reader's coverage: helpers traversed one level within the file,
   a module's doc accepted, direct builders documented, attributed
   tests counted once, no case expansion; the owner's acceptance
   needs no further ruling (Do 3).
6. An eighteenth construction and five completion bounds: the inventory
   corrected, the bounds classified, `HANG` made spec-visible (Read,
   Do 1).
7. The performance goal unconditional against the rehearsal's required
   wait: the Goal conditional, the rehearsal named, the threshold over
   all affected tests including fact failures (Goal, Do 4).
8. A baseline counting helpers as tests: attribute-based counts, the
   reader's before the number the close reports (Input).
9. Gate success as execution evidence: named as such, with the report,
   the re-run and the close's message as its homes (Done when).

The second review's three (`time-residue-brief-a7904b32.md`):

1. A one-level traversal misses chains two and three deep: helper
   chains followed to any depth, cycle-safe, forwarded values resolved,
   a helper's own sentence no sentence for its callers (Read, Do 3).
2. The baseline omitted `door.rs`'s one attributed test: 197 across
   nineteen files, the reader's scope every file (Input, Do 3).
3. The ETA mask named as the client harness's: the CLI harness's
   `eta_rule` (Read).

The third review's one nitpick (`time-residue-brief-bcd7c96b.md`),
taken in a documents-only round with no further review (the rhythm's
rule): the grep gives thirteen matches, twelve timing sites and the
default-value assertion (Read).
