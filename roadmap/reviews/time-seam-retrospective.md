# The time seam, read whole: a retrospective of 4-0 and t5

Provenance: Fable (`claude-fable-5-1`), one session on 2026-11-01 at
`4680cf39`, read-only; a read and a proposal, nothing built, no issue
changed. What was read: `AGENTS.md`, `GLOSSARY.md`, `roadmap/README.md`,
`TIME-SEAM.md`, `TEST-SHAPE.md`, the 4-0 briefs with their dated
narrowings, the spike's Answer, t5's design brief, the 4-0 build
reviews, the rows of `LEDGER.md` from 4-0 step 1 to t5 round 6, the
rulings C3, C84, C85, C86, C122, C133, C134, C137, C147, C148, C153,
C154, C157, C158, C159 and the daemon's and store's "Not doing" lines,
every open issue's title and the bodies and comments of the ones that
touch time, the spike's branch (the tag `4-0-step3-spike`), and the
code: `door.rs` in the client, `life.rs`, the `Life` driver in
`server.rs`, `Exit` in `daemon.rs`, the `Clock` trait, the choke
point's admission loop, the dispatcher's pacing, `ParkingClock` and the
park hook in the daemon's test door, the client harness's isolation
list and ceilings, `.config/nextest.toml`, `.cargo/mutants.toml` and
`weekly.yml`. One Opus reader digested t5's eight round briefs and
fifteen reviews with the close commits; every claim of its I use below
I checked against the tree. Three commands were run, alone on the
machine, after `cargo build --workspace`: `cargo nextest run --workspace
--all-targets --no-fail-fast`, `cargo nextest run -p acquisition-daemon
--all-targets --no-fail-fast`, and `gh run list` over both gates and
the weekly workflow. Every number below names its source.

The owner's questions are answered in §2 to §6; §7 to §9 are what the
read turned up that the questions did not ask for; §10 is the next
steps with their cost and retirements; §11 is his alone.

## 1. The verdict in one paragraph

The seam is right as built, and the leftovers are leftovers. The two
machines are the correct shape, the criterion that drew their edges
was the right narrowing, the spike answered the one open mechanism with
evidence, and t5 landed the daemon's tests on a boundary that a rewrite
of the daemon's internals would survive. What the arc did not do, and
could not have been expected to do, is make the gate faster: the test
stage was 23 s before t5 and 23 s after, because the daemon's tests
were never the wall. The seam's payoff is determinism on a starved
runner and a test boundary, not seconds. Reading the system now that
it is done, two ideas were discovered inside the build and not yet
named as what they are. The first: the runtime's park is the one exact
signal that nothing can run, found by the spike and used to move time
forward, and not yet used for the other thing it is exact about, that
a scenario waiting at such a park is stuck. That one idea settles #151
and the last wall-clock bet inside the door. The second: the door built
for tests the three waits every frontend lacks (`await_terminal`,
`await_auth`, `await_leave`, with the daemon's new `auth_changed`
notification behind them), while every frontend still polls. That is
4-full's first design question, seen early. Everything else open is a
row, a sentence, or a knob.

## 2. The arc as built, against the design

`4-0-design.md` set out four things: a machine per lifecycle with a
pure `step`, a driver per machine holding every instant, a ten-line
runner with a table of rows, and a parking clock replacing both test
clocks; and it named one mechanism risk, auto-advance without
`test-util`, and put a spike before it.

What landed:

- **The client's `Door`** (`crates/acquisition-client/src/door.rs`,
  1,167 lines, 43 rows, `wc -l` and the ledger's row of step 1). States
  Reaching, AwaitingHello, Starting, Settling, Done; the driver is the
  three door functions in `client.rs` over one loop. The five-second
  `c86_` wait retired, held by three rows, the extended `c137_` test
  and a wording test. No word a user meets changed.
- **The daemon's `Life`** (`life.rs`, 198 lines, 19 rows; the driver
  `Daemon::life` about 350 lines of `server.rs`). States Serving,
  Draining, Done. What the step added beyond the design: the idle
  commit claims the connections' word the verdict judged (an atomic
  word holding the open count and an admissions generation), so a
  connection admitted between the verdict and the swap fails the claim.
  That closed a window the old watchdog had and that the build's cold
  review found (`4-0-life-8b7b10f5.md`, P2). The idle daemon wakes at
  its deadline when only time holds it.
- **The spike** refuted the design's advance rule (sleepers equal the
  tasks the door knows wait): right with one task parked, wrong with
  two, three distinct journals in five runs. It found the exact signal,
  a current-thread runtime's `on_thread_park` hook gated by the gate's
  occupancy and the blocking count, on stable tokio with no feature
  split. This is the arc's best single finding and the design's own
  §3 item 4 was wrong about it; the design was right to spike it.
- **t5** built the door (`daemon/tests/door.rs`, 1,178 lines), the
  `ParkingClock`, the counted blocking helper with check 10, the
  auth-change notification (the one production addition t5 made for
  the door's sake), seven tables of 130 rows, and moved 86 tests
  through the door. `ManualClock` and `BlockingClock` are gone, every
  ceiling in the daemon's tests is `HANG`, the exempt-route set has one
  source under check 11, and C159 was entered.

Where they diverged from the design, and whether rightly:

- **The design's §4 table said `IDLE_POLL` and `DRAIN_POLL` retire as
  constants.** `IDLE_REJUDGE` (5 s) and `DRAIN_POLL` (10 ms) both stand
  (`daemon.rs:179`, `:194`). The idle wake is exact only when no clause
  but time holds the daemon; a connection open, a live job, a pending
  login, a landing or a live limiter makes the driver re-judge every
  5 s. Right for now (§3 says why), with a trigger that should be named
  now and is not.
- **The design said `Clock` gains `sleep_until` in step 2.** Step 2
  declined it because neither test clock could drive a deadline in
  order; t5 round 2 added it once the parking clock existed. Right: the
  narrowing was the correct order of operations, reversed one step
  later by the evidence that justified it.
- **The design's estimate, 215 scenarios toward about 120.** The census
  in rows went 378 to 416. The estimate counted occurrences of test
  functions; the census counted rows; the tables outweighed the
  retirements. The suite is larger and better shaped, not smaller. The
  divergence is in the unit, and the design's number was never a
  target (`t5-design.md` §6 says so), so nothing was lost but a
  sentence that should have named its unit.
- **The design said 4-0 runs beside the league with step 2 between
  sittings.** All three steps and all of t5 ran on 2026-10-31 and
  2026-11-01 under the owner's grant of autonomy, 147 commits between
  `6893fc74` and t5's close (`git log --oneline 6893fc74..e24dabc9 |
  wc -l`), touching 43 files under `crates`, `tools`, `.config` and
  `.github` for +14,605 and -8,030 lines (`git diff --stat`). The
  owner's re-read of the narrowings is the price of that pace, and the
  narrowings are all recorded with their dates. Right, given the grant;
  §8 says what the pace cost the reviews.

What `TIME-SEAM.md` §4 asked for and the arc did not deliver, each by
design: the store's annotation wait (corrected in the design: SQLite is
its clock, #148 gave its fixed waits one knob); logical time across the
process boundary (it does not cross; a cross-process test asserts order
and outcome); a shared clock trait (none; no C1 edge moved).

## 3. The narrowings, with hindsight

Each dated narrowing in the briefs, and whether hindsight keeps,
widens or reverses it.

| Narrowing | Ruled by | Hindsight |
| --- | --- | --- |
| The criterion: a wait enters the machine when its ordering against another event is a decision, or when its only pin asserts elapsed time; otherwise it stays a driver effect with its bound inside (`4-0-design.md` §4, the owner, at the door brief's third cut) | owner | **Keep, and widen its home.** It is the sentence that kept both machines small and honest, and the two reviews of the door brief converged on it. It lives in the design brief's dated paragraph and in two module docs. `4-0-design.md` says it is deleted at the phase close. After that, the criterion's only home is `door.rs` and `life.rs`. The GUI's subscriber machine will need it. It belongs as C157's second sentence (§11, question 2). |
| The criterion ruled for the door only; step 2 answers it per poll | owner | **Keep as the process; it has now been applied twice and held both times.** Folding it into C157 makes the per-machine answer the norm rather than a re-ruling. |
| The stop's wait and the winner look stay driver effects with their in-crate pins | owner | **Keep.** Their pins force every interleaving on injected futures and instants and assert no elapsed time; a machine row would pin the same thing twice. |
| `ACQ_NO_SPAWN` read once per door (the four reads made one) | the session, raised for review, taken at the close | **Keep.** A policy that changes mid-door was never a feature. |
| `Clock` gains no `sleep_until` in step 2; the `Life` driver sleeps on tokio time | the session under autonomy | **Reversed by t5 round 2, rightly.** The order was correct: a `sleep_until` with no clock to drive it would have been dead code for a day. |
| `DRAIN_POLL` stays as the quiet wait's look | the session | **Keep, and stop counting it as debt.** The look is a `try_lock` every 10 ms (`server.rs:931`): a 15 s drain is at most 1,500 non-blocking looks in production and 1,500 virtual advances under the door. A notification would need a sixth wake site beside the five `InFlight` counts. The 1 ms real polls that t5 round 2 left in two test helpers (`drive_drain`, `await_a_look`, its ledger row) are the only trace, and they are the test side's, held under `HANG`. |
| `IDLE_REJUDGE` as the re-judge interval while a clause other than time holds | the session | **Keep for now; name the trigger now.** 4-full's subscriber holds a connection open for hours, and under this floor the daemon judges every 5 s for every hour the GUI is open. The design's sentence "not 720 times an hour" is true only of an idle daemon with no client attached. The cost is a lock and six reads per judge; it is not a bug. The trigger is the first long-lived subscriber, and the fix is the connections' word the step already built plus a `Notify` on close-to-zero and the limiter's quiet instant. The close declined it as "an issue if it ever matters"; it will matter at 4-full and should be written as 4-full's first lifecycle row rather than rediscovered. |
| The second signal is the watcher's backstop, no transition; the bound's OS thread stays real | the session, from the brief reviews | **Keep.** C133's forced end must work while every tokio worker is blocked; a transition would put it behind the loop. |
| The verdict `is_idle` stays the driver's pure function, judged on a blocking thread | the session; the fourth brief review | **Keep.** The clauses read `shared`; a machine that read them would hold a lock, which C157 forbids by shape. |
| `Restoring` is not a state | the session | **Keep.** The restore is a landing the verdict counts (C148). |
| The store declined as a candidate; the knob is the seam | owner | **Keep.** #148 closed on it, each expiry pinned at zero. |
| Virtualising a request timeout's expiry: not until a scenario requires it | owner, t5's design | **Corrected 2026-11-02: a choice the transport seam made possible (C161), and made.** This row's verdict was "keep, and note it is structural, not a choice". Since step 3 (`roadmap/briefs/transport-seam-3.md`) the client keeps no timeout; a send's deadline is the choke point's, on the daemon's clock, one instant over the answer and the body, and the wire table forces its expiry on virtual time. What this row said was true while reqwest's timers ran on tokio's wheel inside the client. |
| A classification field in `Outcome`: not until a frontend asks | owner | **Keep.** The rows assert the journal and the protocol's answer; a field for the tests' sake would be a user's JSON changed for a test. |
| Round 4 narrowed to `login.rs`, `sessions.rs` and the cancel family; round 5 split into 5a, 5b, 5c; 5b amended | the session under autonomy, from inventories | **Keep.** Each split was an inventory overturning a first count, recorded the same day. The shape of the work (inventory, then cut) is the right one; the counts in the design were the wrong input, and the briefs say so. |
| C159 narrowed to the daemon oracle's assertion plus checked copies | the session, to the owner's words, after a review found the draft wider than the ruling | **Keep.** |
| Multi-thread tests stay off the door | the spike | **Keep.** `on_thread_park` fires per worker. There is no multi-thread paused clock in tokio either. |
| #141 closed under C158's wording: "every scenario enters through the door" read as "a scenario crosses the frontend boundary; another boundary stays component, saying which" | the session | **Keep, with #152 as the debt it names.** 54 tests say neither. §7 proposes the structural form. |

No narrowing should be reversed. One should be widened (the criterion's
home), and one should have its trigger named (the re-judge floor).

## 4. The shared roots

Which open issues are symptoms of one missing or misplaced idea. Every
open issue was scanned by title (67 of them, `gh issue list`); the
ones below are the ones that touch time or the seam. The owner's list
was right about #131, #151, #152, #70, #108, #15, #14, #1, #56, #66,
#102, #142; I drop #66 and #102 from the seam's roots (they are the
limiter's and the queue lock's, not time's) and add #150 only to say
it does not fit.

### Root A. The park is exact about stuck as well as about idle

**The idea in a sentence:** when the door's runtime parks with nothing
on the clock, nothing in flight and no hold, the scenario can never
proceed, and its wait should fail at that park rather than run to
`HANG`.

Today the hook, at such a park, increments `nothing_parked`
(`door.rs:231`) and parks. The scenario's `await_terminal` then waits
its 120 s ceiling, and nextest kills it at 120 s too. That is the last
wall-clock bet inside the door, and it is the one #151 found: a mutant
that breaks the limiter makes a row wait for an answer that never
comes, the retired component test failed its assertion at once, and 48
mutants moved from caught to timeout, the longest shard from 106 to
125 minutes (`gh run list` for the weekly workflow agrees: 106 and 125
minutes for the two dispatched runs).

What makes the verdict exact is the same thing that makes the advance
exact: nothing the daemon does can wake the runtime except a clock
sleep, a send, a blocking task or the scenario's own hold. Four
production waits in the daemon are not on the clock and would make the
verdict wrong at their sites: the mock sleep kind's 200 ms slices and
the whoami kind's 300 ms (`kinds.rs:577`, `:621`), the accept backoff
(`start.rs:696`), and the login's two bounds (`login.rs:48`, `:445`;
the third site, `:1007`, is under the file's `#[cfg(test)]`). Each
sleeps on tokio time while the runtime parks beside it with nothing on
the parking clock. Move those four onto `Clock` (the kinds and the
login already hold a daemon; the accept loop reads `self.choke`), and
the door's park with nothing parked means exactly what #151 needs it
to mean. reqwest's timeouts stay real and are covered: a request in
flight holds the gate's occupancy from before dispatch until the body
completes.

The one rule this becomes: **every wait in the daemon's sources sleeps
through `Clock`**, pinned by a grep in `tools/arch-check.sh` in the
shape check 10 already has for `spawn_blocking` (`tokio::time::` in
the daemon's non-test sources, at `ratelimit.rs`'s two sites alone).
That is invariant 1's shape applied to time: one choke point, now for
waits.

- **Settles by its own "Closes when":** #151 (the moved mutants
  caught, the longest shard under 106 minutes), once a weekly run
  shows it.
- **Touches, does not settle:** #131's two remaining daemon rows (the
  limiter spec's real-clock tests at `spec.rs` 1925 to 2332, which take
  `ChokePoint::new()` and its system clock; a parking clock there is
  half a day and is #131's, not this root's); the "Not doing" line on
  request timeouts (unchanged; it is structural); the ledger's open
  "login timeouts on tokio's clock" (closed by the move).
- **Evidence before ceremony:** the hook's `Counters` are read by one
  test (`dispatch.rs:2486`, the blocking-count probe) and asserted
  nowhere else. The first act is a run of the daemon's suite that
  prints `nothing_parked` per scenario. If every passing scenario shows
  zero after the four moves, the verdict is exact and the step builds
  it; a scenario that shows one names what it waited on, and that is
  the fifth site.

### Root B. The door built the wait the frontends lack

**The idea in a sentence:** a frontend asks the daemon once and is
told when the answer changes; today every frontend asks again on a
timer.

The daemon's protocol carries job events on a subscription (C85) and
nothing else as events. So: `acquisition-verbs/src/jobs.rs:24` polls a
job's status every 500 ms; `acq auth` polls `AuthStatus` every 500 ms
(`main.rs:2306`); `jobs --watch` prints on events but ends when the
daemon goes (#15); the dashboard polls four times a second; the MCP's
tool text says "poll job_status, then job_result" and, in the agent's
own words from the league seat, a hold "can reach about 5 minutes".
Meanwhile t5 round 1 added `Daemon::auth_changed`, a `Notify` at eleven
publication sites, so that the door could `await_auth`; and the door's
`await_terminal` and `await_leave` are the two other waits a frontend
wants. The daemon now has the events; the protocol does not carry
them; the frontends poll what the tests await.

This is not a step for now: it is C85's, the owner's, and 4-full's
subscriber is the consumer that validates it (P2, real use validates).
Two shapes, for 4-full's design brief to weigh: the subscription
widened to carry auth and lifecycle events beside job events (C85
amended, one connection, the GUI's natural form), or a bounded wait on
the request plane (`job_status` with a `wait_seconds` that answers at
the first change or at the bound), which is the agent's natural form:
one call, one turn, no sleeping in the agent's loop. They are not
exclusive. The MCP would want the second whatever the GUI gets.

- **Settles by its own "Closes when":** none now. #15 ("a watch that
  waits for the daemon to return") is about reconnect, which is the
  subscriber machine's row; #14 (results over a subtree as events) is
  the same channel carrying more; #1 (the keeper) and #2 (correlation
  ids) are the connection's shape; #56 (the frame bound) is what a
  long-lived subscriber meets first. This root does not close any of
  them; it is the sentence their design brief should open with.
- **The two seats agree here** (§5): the test wants to be told and the
  agent wants to be told. The door is the proof that the daemon can
  tell.

### Root C. "Says which boundary" is a sentence where it could be a type

**The idea in a sentence:** a test that stays off the door names its
boundary where the harness builds its daemon, as a value the census
reads, not as a doc sentence the census greps for.

#152 counts 54 tests that neither enter by the door nor say their
boundary. The three harness constructors that remain for component
tests (`test_daemon`, `test_daemon_with`, `test_daemon_at`,
`harness.rs` 651 to 753, called from 13 files) take a base, a clock and
options, and nothing that says why this test is not a scenario. C158
says such a test "says which"; nothing checks it. A `Boundary`
argument on those constructors (workers, a child process, the daemon's
own socket, a function or a table, a second fixture) makes the census
a grep of one argument, and a `#[tokio::test]` that builds a daemon
with no boundary does not compile. The doc sentence stays where it
helps a reader; the type is what the census reads.

- **Settles by its own "Closes when":** #152, if the owner reads "a
  doc, its own or its module's, that names its boundary" as met by a
  value at the constructor (§11, question 3). Otherwise the sweep as
  filed settles it in the same half day and leaves the property to
  discipline.
- **Touches:** nothing else.

### What fits no reframe

- **#70, the Windows watch.** Its two named tests are structurally
  fixed since the #70 step (the knobs widened to 60 s by the isolation
  list, C137) and 4-0 retired the one five-second wait. Its "Closes
  when" wants a week of hosted Windows runs without a re-run, and
  since C155 main's pushes run on the owner's VM (the last eight
  Windows runs on main took 1 to 7 minutes, all green, `gh run list`),
  so the evidence it asks for can no longer be collected on main. It
  is not a symptom of a missing idea; it is a watch whose instrument
  moved. The owner closes it on the class evidence or re-scopes the
  watch to pull requests (§11, question 4).
- **#131's client rows.** The ten remaining 3 to 10 s waits in
  `acquisition-client/tests/contract.rs` (`grep -c`) are cross-process
  waits on a real `acqd` under a real poll; their rule is the 120 s
  ceiling, an hour's edit, no idea missing.
- **#108.** With `Spawn` an effect of the client's machine, an
  injectable spawner is now a driver seam rather than a redesign; the
  issue got smaller without being touched. Still its own small step.
- **#142 (t6) and #143 (t7).** Not time; the scenario corpus and the
  search's case table. t6 is where the frontends' 54 real-time waits
  in their process tests (`grep -c` over `cli`, `mcp` and `verbs`
  tests for sleeps and 120 s bounds) get reshaped, if they need to be;
  they are cross-process by nature and most are the ceiling.
- **#150.** A proptest draw in the search; nothing to do with time.
- **#66, #102, #100, #56.** The limiter's edges, the queue lock, the
  architecture rows, the frame: real, and not the seam's.

A reframe that covered all of these would be a warning sign. Three
cover three things each; the rest are rows.

## 5. The two seats

### The agentic developer

What time costs it now, measured:

| Measure | Value | Source |
| --- | --- | --- |
| Full suite, warm, this machine | 1,407 tests, 23.4 s wall | `cargo nextest run --workspace --all-targets --no-fail-fast`, the Summary line |
| The daemon crate alone | 305 tests, 2.2 s wall, 17.9 test-seconds, 235 under 50 ms | `cargo nextest run -p acquisition-daemon --all-targets --no-fail-fast` |
| The suite's longest test | 18.5 s, `new_league::a_new_league_starts_empty_beside_standard_and_a_typo_is_named` | the same run's `PASS` lines, sorted |
| The next two | 6.1 s and 5.6 s, the two trade captures | the same |
| The daemon's longest | 1.35 s, the admission table of 68 rows | the daemon run |
| Linux gate on main, last eight runs | 5 to 7 minutes | `gh run list --workflow "Gate (Linux)"` |
| Weekly mutants, the two dispatched runs | 106 then 125 minutes | `gh run list --workflow "Weekly"` |

So: the daemon's tests, the seam's whole territory, are a tenth of the
suite's test-seconds and none of its wall. The wall is one CLI process
test waiting out the mock daemon's real stash-limiter window, which
the seam cannot reach because the mock daemon is another process on
`SystemClock`. `TEST-SHAPE.md` §4 called a fast-window knob on the
mock "a stop-gap, the seam being the cure"; the seam, as ruled, ends at
the process boundary, so for that test the stop-gap is the cure. A
mock window set by the isolation list (the mock already reads
`ACQ_MOCK_DEGRADED_HEAD`) would take the suite's critical path from
18.5 s to whatever the trade captures cost; the golden already masks
the wait's seconds and asserts only that a wait appeared.

(Corrected 2026-11-01 by the residue step's measurement, `2afd10b8`:
with the mock's windows divided by ten the rehearsal's wait went and
the wall did not move, 23.13 s to 23.24 s, because the suite is
throughput-bound on this machine, about 205 test-seconds over ten
threads; the rehearsal does not set the wall. The knob landed for a
user, the harness leaves it unset, `2d855904`, and E3 stays open for
the rehearsal alone on #131. The premise of this paragraph and of §10's
row E3 was wrong.)

The other costs, in the seat's own words:

- **Flakes on a starved runner.** None of #70's class since
  2026-10-27 (#70's watch comments). The seam and the isolation list
  did what C137 promised.
- **Mutants that time out instead of failing.** 48 of them, #151, Root
  A. The one regression the arc introduced, and it introduced it by
  removing component tests that failed at once in favour of scenarios
  that wait. The cure is in the same mechanism that caused it.
- **Tests whose boundary it cannot tell from their doc.** 54, #152,
  Root C.
- **Waits it cannot reason about.** The door has three: `settle`
  (resolve at the first park that would move the clock), `quiet`
  (resolve at the first park, whatever holds the clock, and only after
  a held request's arrival barrier) and `hold_clock` (the hook moves
  nothing while the guard stands). Their docs are good (`door.rs` 715
  to 801) and their rules are real, but a scenario author must learn
  when each is right, and `quiet`'s precondition is a sentence, not a
  type. Usage across the tests: `settle` 14 sites, `quiet` 8,
  `hold_clock` 1, `advance` 7 (`grep -o "door\.[a-z_]*("`, counted).
  That is a small vocabulary used rarely; the cost is in the first
  reading, not the hundredth use. I would not simplify it: each of the
  three is a different fact about the runtime, and merging them would
  hide the fact.
- **A harness to learn before writing a test.** `door.rs` is 1,178
  lines with about forty `pub(super)` calls. Five carry the load:
  `finish` 89 uses, `request` 70, `await_terminal` 35, `submit` 34,
  `journal_methods` 30. A scenario is readable from those five plus a
  fixture kind. The module doc is a page and says what stays off the
  door and why. This is learnable in an hour by an agent that reads
  the doc first, and the brief template already says to.
- **Wall-clock the seat pays that is not tests:** the compile. t4
  measured it and it is not this retrospective's.

What the seat still lacks: the stuck verdict (Root A); the boundary
as a value (Root C); the mock window on the isolation list; the
limiter spec's eight real-clock rows on a parking clock (#131); and
one stale comment, `tools/arch-check.sh:466`, which still says the
blocking count holds "until its closure returns" where round 1's fix
made it "until a runtime task observes the completion".

### The agentic user

What time looks like from `acq` and `acq-mcp`:

- **A job's wait is a state and a prediction**: `eta_seconds` on
  `JobInfo` (`job.rs:88`), the limiter's prediction corrected by
  headers; `next_safe_in_seconds`, `retry_after_secs` and
  `seconds_left` per policy and window on `daemon status`
  (`status.rs` 40 to 82); a quote's `eta_seconds` and its observation
  time (C40). Ages are words beside epochs on every surface (C53).
- **A route the daemon has not probed has no time**: the league seat's
  one finding about time, "a plan on an unlearned route gives no time
  until its probe" (`audits/league-2026-11/seat/README.md`, the plan's
  `eta_seconds: null`, judged not a fault because the JSON says so).
- **Waiting is the user's own loop**: `job_status` until terminal,
  `acq result --wait`, `acq auth` watching `AuthStatus`, `jobs
  --watch` until the daemon goes. The MCP's text says holds can reach
  about five minutes, which is honest, and leaves the agent to decide
  how often to ask.
- **A slow daemon is "unresponsive" after 5 s** on the handshake and
  on every read request after it (C86, #103's `Stalled`); a quote is
  bounded at 5 s by the plan and noted. Both are right: a healthy
  daemon answers reads in milliseconds whatever the limiter holds.
- **The daemon's own end is a knob, not a moment**: `daemon status`
  carries `idle_shutdown_seconds` and `uptime_seconds`
  (`protocol.rs:593`, `:599`). The `Life` driver now holds the armed
  instant; the close declined to put it in the protocol until a user
  asks. An agent that spawned a daemon for a verb and reads "exits an
  hour after its last connection" (the README's line the seat checked)
  has the knob and the uptime and can compute the rest, so this is
  cosmetic until a GUI shows a countdown.

What the user seat lacks: a wait it does not have to run itself (Root
B); and nothing else that the seam could give it. Time on the user's
side is already honest: predicted where the limiter can predict, null
where it cannot, words beside numbers.

### Where the seats differ, and where they are one need

They differ, and should not be averaged:

- The developer wants time **virtual and silent**; the user wants it
  **real and stated**. A row asserts no number; an answer carries one.
- The developer wants a wait to **fail at once** when nothing can
  happen (Root A); the user wants a wait to **hold** through a 300 s
  limiter hold without anyone calling it a hang. The same park that is
  "stuck" under the door is "waiting on GGG" in production, and the
  difference is the occupancy count: under the door a send in flight
  holds the clock, in production it holds the user. The two verdicts
  are built from the same two counts.
- The developer wants the daemon's **deadlines named** so a row can
  fire them (`Timer::Idle`, `Timer::Drain`, `Timer::Hello`); the user
  wants the **moment** named so a status can print it. The armed
  instant the driver holds is one fact wanted from two sides; the row
  reads it as an event and the status would read it as a field.

They are one need in two places: **being told**. The door's
`await_terminal` and `await_auth` are what `jobs.rs:24` and
`main.rs:2306` poll for. 4-full's subscriber is where the two seats
meet, and its design brief should say so in its first paragraph.

## 6. What not to do

Reframes considered and rejected, each with its reason.

- **tokio's `test-util` paused clock.** It is exactly what the parking
  clock and the park hook are, behind the project's trait. It splits
  the feature graph the gate's `features` stage holds equal (C84), and
  it would own the daemon's timers while reqwest kept its own. The
  project built the same mechanism without the split; keep it.
- **A shared clock crate or trait for the client.** The client's door
  has no clock and needs none: its one race is pinned at zero, its
  real-wiring tests are cross-process. A crate would move a C1 edge
  for a trait with one implementor.
- **A clock in the store.** SQLite's `busy_timeout` is its clock; the
  knob is the seam; #148 closed it. Ruled, and the ruling is right.
- **A structured adjudication or retry step for CI reds.**
  `TIME-SEAM.md` §3 declined it pending a week of pushes. The week
  passed with no red of the class. Declined for good, unless the
  class returns.
- **The choke point's admission slices (`HALT_POLL`, 1 s) as a
  notification.** The slices exist so a halt is seen within a second
  by a task parked in a 300 s hold; under the door each slice is one
  advance, 61 for a 61 s hold, in 4 ms. Cheap in both worlds. The
  third brief review was right to keep it out of `Life`.
- **Pulling the verdict's clauses into the `Life` machine.** They read
  `shared`; C157 keeps locks out of the machine by shape. The machine
  is thin (19 rows over three states) and that thinness is the design
  holding, not a gap. The driver's substance (the absolute deadlines,
  the judge on its thread, the atomic word) is where the lifecycle's
  difficulty actually lives, and the driver is pinned by its wiring
  tests.
- **A classification field in `Outcome` for the rows' sake.** Ruled
  "Not doing"; the rows read the journal and the answer; right.
- **Virtualising reqwest's timeouts.** Not reachable without owning
  the HTTP client's timer. The "Not doing" line is a fact, not a
  deferral; say so when it is next read.
- **Moving every component test onto the door.** C158 says a test
  that needs another boundary stays and says which. 54 component tests
  are the right number if each says why (Root C), and the ones that
  need workers, a child or a socket cannot be scenarios.
- **Lowering `HANG` or the nextest ceiling to catch mutants faster.**
  A lower ceiling is a wall-clock bet again. Root A fails at the exact
  instant instead.
- **A fourth door wait that merges `settle` and `quiet`.** They are
  different facts about the runtime; a merge would hide which one a
  scenario relied on, which is the fault class the cold reviews kept
  finding (§8).

## 7. Observations, patterns, insights

Beyond the questions asked.

- **The arc's one production addition for the tests' sake is also the
  frontends' missing primitive.** `auth_changed` was added so the door
  could await; the frontends do not read it. When one mechanism is
  built for a test and turns out to be a feature, that is the system
  saying where the next consumer is (Root B).
- **The park hook is invariant 1 for time.** One place every wait goes
  through (`Clock`), one place that decides whether time moves (the
  hook), and the counts that hold it (occupancy, blocking). The four
  off-clock waits in the daemon are the "daemon HTTP path that bypasses
  the choke point" of this invariant: each is reasonable locally and
  each makes the signal inexact. Root A names the rule.
- **The gate's wall was never the seam's to win, and the briefs knew
  it.** t5 round 6's brief calls the test-stage figures "observations
  under differing loads, never a measured improvement". The retrospective
  confirms it with a different instrument: the critical path is one
  CLI test. Any future claim that a test change saved wall time should
  name the critical path, not the stage total.
- **The machines are smaller than their drivers, and that is the
  shape.** `life.rs` is 198 lines; its driver about 350. `door.rs`'s
  machine is longer because its table lives in the file. C157's value
  is not that the machines hold the logic; it is that the deadlines are
  named and the driver is one loop with one biased select whose order
  is stated in a comment (`server.rs:935`). A reader can now answer
  "what happens if a stop arrives during the judge" from one function.
  Before, it was four functions and a watchdog.
- **The atomic word is the most elegant thing in the arc.** The build
  review found that a completed idle verdict could end a daemon after a
  new client connected; the fix was not a lock but a word the verdict
  reads and the commit claims (`server.rs:831`, `begin`). Admission and
  the idle exit contend on one `u64`. That is structure over discipline
  at its best, and it came from a cold review, not the brief.
- **The spike was worth a day and the design's count was worth
  refuting.** Had t5 been briefed on the design's rule, every two-key
  scenario would have been nondeterministic and the fault would have
  surfaced as flakes, the exact disease the seam was built to end.
  "A mechanism bet gets a spike" should be in the template if it is
  not.
- **Estimates in the wrong unit mislead even when nothing is held to
  them.** 215 toward 120 (functions) against 378 to 416 (rows). The
  arc's own ledger caught it. The project's memory already says
  estimates name their unit; this is the case that proves it.
- **The retirement line is the arc's best process invention and its
  commonest fault.** Four of the cold reviews' findings were a holder
  that held less than the test it retired (round 2's retry count,
  round 3's journal surface, round 4's generation reads, round 5b's
  dropped assertion). The line `retired X; held by Y` names a test; the
  review had to read the test's assertions to see what Y did not hold.
  A cheaper form for the next group: name the assertion, not the test,
  when a test holds more than one.
- **"Nothing asserts them" is a smell the counters carry.** The hook's
  `Counters` were built to answer the spike's questions and one test
  reads one of them. Root A gives them a use; if it does not land, they
  should go.
- **The ParkingClock journals as `manual`.** Round 2 raised it; it is
  right: the journal header's purpose is that a fake-clock journal can
  never pass for live evidence (`ratelimit.rs:84`), and "manual" says
  not-live. Leave it.
- **The briefs were cut three to five times each under cold review
  before a build, and the builds' reviews then found one to three real
  faults each.** The pattern from `review-convergence-patterns` holds
  here in full: handoff windows (round 1's blocking count, round 2's
  stop before re-pick) were the findings that mattered, and the
  worker's raised list predicted the review in every round. The
  rhythm is expensive in documents and cheap in faults; for a phase
  that touched the exit path the owner was running through, that was
  the right trade.

## 8. What I find elegant, what I find clunky, what I would want

**Elegant.** The criterion. The atomic word. The park hook as tokio's
paused clock without the feature. `Row` as a sequence of steps with a
`forbids` per surface. The exempt-route set read from the production
constant by the oracle, with the copies held equal by a check, so the
set is written once and read four times. The `Leave` hook, so a test
runs the driver to its last effect and the process never ends. A
scenario that reads like the frontend's day: open, submit, await, read
the journal, finish.

**Clunky.** Three waits (`settle`, `quiet`, `hold_clock`) with rules a
reader learns from prose, and `quiet`'s precondition ("only after the
arrival barrier of a request it holds at its own fixture") unenforced.
`beside` doors with no hook of their own, a sentence a scenario can
miss. The component constructors carrying no reason for being
component. A `nothing_parked` counter with no consequence. The 1 ms
real polls left in two drain helpers. `IDLE_REJUDGE` surviving as a
production poll under a design sentence that says the daemon wakes once.
One stale comment in a check. And the briefs: each is a page of line
numbers that go stale the day they are written, which the reviews then
re-verify; the template's "Read" section does a job a tool could do
(`bothy.py pack --path` prints what to read; it could print the lines
too).

**What I would want.** The one-clock rule with its check, so the next
`tokio::time::sleep` in the daemon fails the gate the way a second
`spawn_blocking` does. The stuck verdict, so a broken daemon fails a
row in milliseconds rather than two minutes. The boundary as a value.
And 4-full designed from Root B's sentence, with the subscriber born as
rows under the criterion, which is the whole reason 4-0 ran before
4-full.

## 9. The road the arc leaves

The time seam is closed as a phase in all but name: #147's one line
"the daemon's idle and drain paths hold no poll constant" is met
narrower than written (`IDLE_REJUDGE`, `DRAIN_POLL`), each with its
reason in §3, and the owner closes it or carries the line to the
trigger §3 names. Nothing below is a reason to reopen the design.

## 10. Next steps, each with its cost and what it retires

In the order of value. A "ruling" column says whether the owner must
rule before the work; the mechanism is the session's either way.

| Step | What | Cost | Retires (C154's form) | Ruling |
| --- | --- | --- | --- | --- |
| A1. The daemon's waits on one clock | the four off-clock waits (`kinds.rs` 577 and 621, `start.rs:696`, `login.rs` 48 and 445) through `Clock`; a check in `tools/arch-check.sh` in check 10's shape: `tokio::time::` in the daemon's non-test sources at `ratelimit.rs`'s two sites alone, with its breaker | half a day | retired the ledger's open "login timeouts on tokio's clock"; held by the check | yes: the rule is a boundary (§11, question 1) |
| A2. The stuck verdict | first a run printing `nothing_parked` per scenario; then the door's waits (`await_terminal`, `await_job`, `await_auth`, `await_leave`) fail at a park that finds nothing parked, nothing in flight and no hold, naming what the scenario waited for; `HANG` stays as the backstop a real-time wait can still reach | a day, after A1 | retired `HANG` as the failure path of a door wait; held by the stuck verdict (`HANG` stays a backstop). #151 closes on the next weekly run's compare | no: mechanism under C158, if A1 is ruled |
| C1. The boundary as a value | `Boundary` on the three component constructors; the census reads it; #152's 54 tests each name one | a day | retired the census's doc grep; held by the constructor's argument | the owner's reading of #152's close (§11, question 3) |
| D1. The criterion into C157 | one sentence, before the phase close deletes `4-0-design.md` | an hour | nothing | yes (§11, question 2) |
| E1. The limiter spec's real clock | the eight `ChokePoint::new()` tests at `spec.rs` 1925 to 2332 on a parking clock (#131's last daemon rows) | half a day | retired the spec's `wait > 5s` real-time rows; held by the same rows on the parking clock | no |
| E2. The client's ceilings | `contract.rs`'s ten 3 to 10 s waits to the 120 s ceiling (#131's client rows) | an hour | retired nothing (a ceiling resized) | no |
| E3. The mock's window on the isolation list | a mock knob for its policy windows, set by the client harness's list; the README's knob table gains the row | half a day | retired nothing; the suite's longest test falls from 18.5 s to the window | no, with the owner's later read as agent-facing design |
| E4. The stale comment | `tools/arch-check.sh:466` | minutes | nothing | no |
| F1. #70 | close on the class evidence, or re-scope the watch to pull requests | minutes | nothing | yes (§11, question 4) |
| G1. 4-full's design brief's first input | Root B's sentence; the subscriber as rows under the criterion; `IDLE_REJUDGE`'s trigger as the subscriber's first lifecycle row; the exit moment as a status field when the GUI shows a countdown | the design session's | nothing now | C85's amendment is the owner's, at that design (§11, question 5) |

Not proposed: anything in `Life`'s or `Door`'s machines; a change to
`settle`, `quiet` or `hold_clock`; a change to the briefs' rhythm.

## 11. For the owner

The decisions only he can make, each as a question with its options.

1. **The daemon's one clock.** Should "every wait in the daemon's
   sources sleeps through `Clock`; reqwest's timeouts are the one
   exception, held by the gate's occupancy" be entered as a ruling in
   `decisions/daemon.md`, pinned by an arch check in check 10's shape?
   (a) Yes, as a ruling with the check, recommended: it is invariant 1's
   shape for time and it is what makes the stuck verdict exact. (b) The
   mechanism only, as C158's doc comment, no registry line. (c) Decline;
   #151 waits for a lower ceiling or stays.
2. **The criterion's home.** `4-0-design.md` holds the ruled criterion
   and says it is deleted at the phase close. (a) Fold the criterion
   into C157 as its second sentence, recommended. (b) Leave it in the
   two module docs and let the GUI's machine cite one of them. (c) Keep
   the design brief undeleted.
3. **#152's close.** Does a `Boundary` value at the component
   constructors meet "a doc, its own or its module's, that names its
   boundary"? (a) Yes; build C1, recommended. (b) No; the doc sweep as
   filed.
4. **#70.** (a) Close now on the class evidence (no red of its class
   since 2026-10-27; its two tests structurally fixed), noting that
   main's Windows runs are on the VM and the hosted-runner evidence is
   thin, recommended. (b) Re-scope the watch to pull-request runs and
   keep it open. (c) Leave as is.
5. **4-full's first question.** Should 4-full's design brief open with
   Root B (the daemon tells; a frontend never asks twice) and weigh
   C85's amendment (the subscription carrying auth and lifecycle events)
   beside a bounded wait on the request plane for the MCP? (a) Yes,
   recommended; nothing is built until that design is ruled. (b) No;
   4-full's design starts from `GUI-NOTES.md` alone.
6. **#147's last line.** "The daemon's idle and drain paths hold no poll
   constant" is met narrower than written. (a) Close #147, the
   narrowings recorded in §3, with `IDLE_REJUDGE`'s trigger named as
   4-full's first subscriber, recommended. (b) Carry the line to a new
   issue on that trigger.
