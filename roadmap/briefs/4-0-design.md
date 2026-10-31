# 4-0 — The time seam: the design

A Fable design, 2026-10-31, from `roadmap/TIME-SEAM.md` §4 and §6 and
`roadmap/TEST-SHAPE.md` §4, at the owner's word of that day ("after
that I want to start work on the time seam"). Nothing here is built or
ruled: the rulings in §7 are drafts for the owner, and the build
briefs follow his word. This file is deleted at the phase close with
the briefs; its reasoning stays in the commit that lands it.

What was read: `TIME-SEAM.md` whole, `TEST-SHAPE.md` §2 to §4, the
rulings C3, C10, C84, C86, C133, C134, C135 and C137, the brief
template's rule on paused tokio time, issues #70, #100, #131 and #141,
`TESTING-NOTES.md`'s "two clocks" entry and its five-call door. Four
Opus reads, one per candidate machine of `TIME-SEAM.md` §4, each
read-only and run-free; every count and line below is theirs, read
from the tree at `6893fc74`, not measured by running anything.

## 1. What the tree says, against `TIME-SEAM.md` §4

The four candidates are not four of one kind. Two are lifecycles, one
is already the shape wanted, and one is not the project's clock at
all.

**The client's door** (`crates/acquisition-client/src/client.rs`,
`open_door` at 1307–1545). There is no state type: the lifecycle is a
240-line async loop whose state lives in six mutable locals (`child`,
`startup`, `respawned`, `stop_failed`, `looked_for_winner`, `found`).
Its decisions are already pure and unit-tested (`Verdict::of`,
`DaemonId::judged`, `BootstrapReply::read`); what is entangled is the
sequencing. Real time enters at about ten sites: the hello deadline
(`timeout_at` plus a re-check `now() < deadline`, 1756–1765), the
same pattern for observe requests (1852–1855), the startup deadline
(`Startup::expired`, 1242–1259), the 20 s stop bound (`stop_until`,
1631), the winner look (`look_until`, 1968–1987, which takes an
injected `now` and then reads `tokio::time::Instant::now` itself for
its pause), and three poll pauses (50 ms bind poll, 200 ms settle
after a replace, 50 ms gone poll). Partial seams exist: `stop_until`
takes its bound as a future and `look_until` takes `now`. The `c86_`
test (`tests/contract.rs:326`) removes the knob and spends five real
seconds, asserting `elapsed >= 5s`; the `c137_` tests force expiry at
zero and spend none.

**The daemon's lifecycle** (`crates/acquisition-daemon/src/daemon/`).
The idle verdict is already a pure function over the injected clock:
`is_idle` (`server.rs:671`) reads `choke.now()`, and every `idle.rs`
test calls `ManualClock::advance` then `is_idle` and runs in no time.
What is not on the clock is the loop around it: the watchdog sleeps
`IDLE_POLL` (5 s) on tokio time (`server.rs:699`), the drain polls
`try_lock` every `DRAIN_POLL` (10 ms) under `sleep_until`
(`server.rs:581–603`), the exit bound is a tokio `Instant` plus an OS
thread on `std::time::Instant` (`daemon.rs:233–245`), and the accept
backoff sleeps on tokio (`start.rs:720`). The watchdog is never
spawned in the harness, so the idle exit as a whole is untested today;
`exit.rs` runs on real time (a 3 s `BOUND`, 5 ms polls, a 100 ms
`settle`, one multi-thread test of about 2.5 s). The daemon literal
has 23 fields plus a nested `Shared` of 8, built three times
(`start.rs:500`, `tests/harness.rs:543`, `tests/sessions.rs:219`),
and `start.rs:462` hard-wires `SystemClock`.

**The limiter** is the model, and it is purer than §4 says: time is a
value, not a clock. Every limiter method takes `now: Instant`; holds
are `Instant`s; ETAs are durations from `now`; `limiter.rs` and
`policy.rs` contain no clock call at all. The fuzz
(`ratelimit/tests/spec.rs:1023`) is exact because it has no clock: it
passes `now` as data over 1,000 drawn states and checks the optimised
ETAs against the simulation that keeps every send. The choke point
sleeps through the `Clock` trait (`ratelimit.rs:66–76`: `now`, `wall`,
`sleep`, `kind`); nothing in `choke.rs` touches `tokio::time`.

**The store's annotation wait is not a candidate.** The wait is
SQLite's `busy_timeout`, a loop inside SQLite's C code that no
injected clock can reach. The project owns three things there and
pins all three without waiting: the knob's parse and bounds, the
expiry verdict (`Busy`, never `Db`, with the wait SQLite enforced read
back) at a wait of zero, and that two writers never clobber, in
process with two connections (`unit/tests/annotations.rs:418`) and
across processes (`acquisition-cli/tests/price_story.rs:198`). §4's
"determinised in-process with two connections" is already done, and
its "cross-process twin shrinks to a smoke test" is wrong: that test
proves the real binary's compare-and-swap and receipts chain across
processes and asserts no time. It stays. What is left is small and
not a clock: `ensure_wal`'s hard-coded 5 s retry deadline
(`lib.rs:971–989`) has no expiry pin, and five `busy_timeout`s of 5 s
(`lib.rs:1035`, `:1062`, `jobs.rs:168`, `:460`, `:523`) have no knob.

**The test clocks.** Two, sharing no code. `ManualClock`
(`daemon/tests/harness.rs:22–67`): `sleep(d)` advances the clock by
`d` and yields, so two tasks each sleeping one second advance it two,
and which wakes first is the scheduler's; the code admits this at
`server.rs:687–691`, which is why the watchdog stays on tokio time.
`BlockingClock` (`daemon/tests/dispatch.rs:15–75`): sleeps park on a
semaphore and are released first come, first served, with no order
among deadlines. Neither keeps a heap of deadlines, so neither gives
"advance to T wakes exactly the sleepers due by T, in order". Paused
tokio time is not available: `test-util` splits the feature graph the
gate's `features` stage compares (the brief template's rule).

## 2. The reframing: time is an event, and a lifecycle is a value

The limiter found the right idea and the lifecycles did not copy it.
The limiter never asks what time it is; it is told. Extend that one
step: a lifecycle that waits never asks the clock either. It is a
value with a step:

```
step(state, event) -> (state, effects)
```

where the events include `Deadline(name)` and the effects include
`Arm(name, Duration)`. Time enters only as an event and leaves only as
a request. The socket, the child process, the file system and the
clock all live in a *driver*, a thin async loop that performs each
effect and feeds the resulting event back, racing the machine's next
deadline through the clock. This is the sans-I/O shape (the one
`quinn-proto` and `h2` use, where the protocol state machine is pure
and the runtime is outside it).

What it buys, in the order that matters here:

- **The machine needs no clock trait.** A machine test is a
  synchronous unit test: a script of events in, a list of effects out,
  one row per transition. It runs in microseconds and is exact on any
  runner, because nothing in it can be late. `TIME-SEAM.md` §6's first
  question (where the shared clock trait lives) dissolves: the client
  crate gains no trait and no new crate is cut; C1's edges do not
  move. The daemon's existing `Clock` stays what it is, the limiter's
  and the driver's.
- **A deadline's expiry is a row.** C137 says a deadline's contract is
  its behaviour at expiry; under this shape that behaviour is the row
  `(AwaitingHello, Deadline(hello)) -> (Unresponsive, [Report])`, and
  the five-second `c86_` test retires, held by that row plus the
  `c137_` real-wiring test that already forces the knob to zero.
- **The driver's waits become exact in production.** A watchdog that
  polls every 5 s becomes "sleep until the earliest moment the verdict
  could flip, or an activity event"; a drain that polls `try_lock`
  every 10 ms becomes "sleep until the bound, or a landing event".
  An idle daemon wakes at its deadline and not 720 times an hour, and
  `daemon status` can name the moment it will exit, read from the
  armed timer rather than recomputed.
- **The GUI's subscriber and reconnect are born as rows.** 4-full's
  reconnect is events (`Connected`, `Dropped`, `EventArrived`,
  `Deadline(backoff)`) and effects (`Connect`, `Subscribe`,
  `Snapshot`, `Arm(backoff, d)`), tested as a table before a socket
  exists. That is what `TIME-SEAM.md` §4 asked for and could not say
  how.

What it does not buy, said plainly: the real wiring still needs a
real socket, a real child and a real file system, so each driver keeps
one or two tests that assert "the wiring eventually works", bounded by
the hang ceiling alone and asserting no number. That is already
C137's rule for cross-process tests; this design keeps it and makes it
the whole of what real time pins.

## 3. The shape, in four parts

1. **A machine** per lifecycle: an enum of states, an enum of events,
   an enum of effects, and `step`. No I/O, no clock, no `async`, no
   `Arc`. Its deadlines are named (`Hello`, `Startup`, `Stop`, `Idle`,
   `Drain`), so a `Deadline` event says which one fired and an `Arm`
   effect says which one to set; a machine that leaves a state cancels
   the timers it armed there, as an effect, so a stale deadline can
   never arrive.
2. **A driver** per machine: the existing async code, reorganised so
   that each branch performs one effect and converts the result into
   one event. The client's driver uses `tokio::time` directly, as it
   does now, because the client has no clock trait and needs none; the
   daemon's driver sleeps through its `Clock`, gaining `sleep_until`.
   The exit bound's OS thread stays real by design (C133, one exit;
   the bound must hold when every tokio worker is blocked), and the
   driver treats it as the backstop it is.
3. **A runner** for machine tests: `drive(machine, script) -> effects`,
   a loop of `step`, about ten lines, in one test-support module both
   crates' tests can copy or import as C1 allows. A scenario is a
   table row: the script and the expected effects, which is the
   scenario-row shape `TEST-SHAPE.md` §4 names, with no clock in it.
4. **The parking clock**, the daemon's, replacing both test clocks:
   `sleep` and `sleep_until` park in a heap of deadlines;
   `advance_to(t)` wakes the sleepers due by `t` in deadline order;
   `advance_to_next()` moves to the earliest pending deadline;
   `sleepers()` reports how many are parked, subsuming
   `BlockingClock::wait_for_sleepers`. `advance(d)` keeps its name and
   meaning for the fifteen files that call it. This is tokio's paused
   clock behind the project's own trait, about 100 lines, and it is
   t5's (#141), not the machines': the machines need no clock, and the
   daemon's door is what drives a whole daemon through virtual time.

The door's `await terminal` is the one place virtual time must move
on its own: when every task is parked in the clock and nothing else
can run, the runner advances to the next deadline. Without tokio's
`test-util` there is no "nothing else can run" signal, so the door
approximates it (sleepers equal to the tasks it knows are waiting,
after a yield), which is what `BlockingClock` does today by hand. This
is the one piece of the design with a mechanism risk, and §6 puts a
spike before it.

## 4. What each candidate becomes

| Candidate | Machine | Driver keeps real | Retires |
| --- | --- | --- | --- |
| the client's door | `Door`: states Reaching, AwaitingHello, Judging, Spawning, LookingForWinner, Stopping, Done; events Reached, Nothing, HelloReply, HelloClosed, ChildExited, LockHolder, StopAck, EndpointGone, Deadline; effects Connect, SendHello, Spawn, Stop, LookLock, Arm, Cancel, Report | the socket, the child, the lock file, the endpoint look; one real-wiring test per door under the ceiling | the 5 s `c86_` wait (held by the expiry row and `c137_` at zero); `look_until`'s private `now` seam; the 2 s `sleep` in the spawn-race test (a `LockHolder` event in the script) |
| the daemon's lifecycle | `Life`: states Restoring, Serving, Draining, Leaving; events Opened, Closed, JobChanged, LoginPending, LoginDone, Landed, StopRequested, Signal, Deadline(Idle), Deadline(Drain); effects Arm, Cancel, RefuseSubmits, Checkpoint, RemoveSocket, Exit | the signal handlers, the bound's OS thread, `leave` (which exits the process), the socket file; the CLI's `daemon_signal_exit.rs` and `daemon_stop_checkpoint.rs` as they are | `exit.rs`'s 3 s `BOUND`, `settle`, the 5 ms polls and the two elapsed bounds #131 names; `IDLE_POLL` and `DRAIN_POLL` as constants |
| the limiter | already a value; untouched | the choke point's sleep through the clock | nothing |
| the store's annotation wait | none: the knob is the seam and SQLite is the clock | everything, as today | nothing; a "Not doing" line records why |

Narrowed 2026-10-31, the owner's ruling at the door brief's third cut
("cut as proposed with narrowings"; "Record, cut, review"), after two
cold reviews of the brief (`roadmap/reviews/4-0-door-brief-*`): the
`Door` row above is read with these changes. The machine names its
timers (`Hello`, `Startup`, `BindPoll`, `Settle`) and the driver holds
their durations, read where the knobs are read today. The stop's wait
(`stop_until`, its biased tie) and the winner look (`look_until`, its
late connect) stay driver effects with the seams and in-crate pins
they have, which force every interleaving without relying on elapsed
time: a wait
enters the machine when its ordering against another event is a
decision of the lifecycle, or when its only pin asserts elapsed time;
one whose interleavings are already forced without relying on elapsed
time, and whose
outcome the lifecycle takes as one value, stays an effect with its
bound inside. The startup deadline is consulted at the loop's one
check point, answered expired or pending, never raced or delivered
unasked; §3's "a machine that leaves a state cancels the timers it
armed there" is replaced by explicit `Cancel` effects where the loop
drops a deadline today. The criterion is ruled for the door; step 2's
brief answers it per poll of the daemon's, since an idle poll has no
competing future today and the criterion must not hide the lifecycle
step 2 exposes.

Narrowed 2026-10-31, step 2, by the orchestrating session under the
owner's grant of autonomy of that day ("act autonomously for the good
of the developing agents and the agent and human users"), from three
read-only reads of the tree at 5c9cd487 (`briefs/4-0-life.md`, Read);
the owner reads this at the step's close. The `Life` row above is read
with these changes. The verdict `is_idle` stays the driver's pure
function, consulted on the idle deadline's event (it is pinned by
`idle.rs` on the manual clock, asserting no time); the machine holds
the lifecycle's decisions: the idle deadline's expiry, the first cause
winning, the drain's cut-or-clean against its deadline, the second
signal. The quiet wait (`InFlight` empty) is a driver effect with its
poll inside; `DRAIN_POLL` stays as that pause and `IDLE_POLL` as the
re-judge interval while a clause other than time holds the daemon,
the idle deadline exact when only time holds. The driver sleeps on
tokio time, as the watchdog and the drain do today and for the reason
their comments give: the manual test clock advances on every sleep
(`harness.rs:62–66`), the blocking one orders nothing, so a driver on
the `Clock` would have no test clock to drive it; `Clock` gains no
`sleep_until` in step 2, and step 3's spike decides whether a parking
clock takes the driver. `Restoring` is not a state: the socket serves
during the restore and the restore is a landing the verdict and the
quiet wait both count (C148). The `settle` the row names is a test
helper, not a production wait; the elapsed upper bounds #131 names go
in this step, the sleeps that prove a wait stay for #131's own close.
The one constructor lands as the step's second commit.

The one constructor (t5's "construct with a base URL, a clock and a
credential store") is the daemon driver's constructor, and the
`Life` extraction is where it is built: a builder over `provider`,
`clock`, `credential_store` and `log`, with defaults for the 23
fields' mechanics (locks, counters, channels) and options for what
the three copies vary (rails, jobs database, initial sessions, restore
landed, bound and grace, login bound, the world's paths). `start.rs`
and the harness call the same builder, and `SystemClock` is one
argument at one site.

## 5. The open questions of `TIME-SEAM.md` §6, answered

1. **Where the clock trait lives.** Nowhere new. A machine has no
   clock; the client's driver uses tokio time as it does now; the
   daemon's `Clock` stays the daemon's. No crate is cut and no C1 edge
   moves.
2. **Logical time across the process boundary.** It does not cross.
   A cross-process test asserts order and outcome under the hang
   ceiling, never a duration; the daemon's log already names its
   lifecycle lines, and the `Life` driver logs each transition by its
   state name, so a cross-process test reads order from the log it
   already reads.
3. **Which "Open" rows 4-0 absorbs.** None beyond what the extraction
   touches. #100's lock merge had its keyring half taken by r2
   (C148); the typed `JobKind` and `frame.rs` sharing are not time
   and stay on #100's trigger. t5 (#141) rides inside, as the
   pull-forward item says, and is where the parking clock lands.
4. **The perf cliffs.** Closed before this design: clock-tests' close
   made the six performance budgets print their time and assert none
   of it (C137, the ledger row of 2026-10-29).

## 6. The build, in the order risk is lowest

4-0 runs beside the league, so each step must leave the live daemon
as it was until the step is green, and the daemon's own step lands
between sittings.

1. **The runner and the client's `Door`.** The smallest machine, in
   the crate that sends nothing to GGG. Proves the shape: the enum
   triple, `step`, the ten-line runner, the table of rows, the driver
   cut from `open_door` and `stop_until` with `handshake`, `exchange`,
   `reach`, `Verdict` and the reports untouched. The `c86_` retirement
   with its C154 line. Safe during a sitting; the owner's live runs do
   not pass through a test.
2. **The daemon's `Life` and the one constructor.** `is_idle`'s
   verdict becomes `Life`'s, the watchdog and the drain become its
   driver, `Clock` gains `sleep_until`, `start.rs` and the harness
   share the builder. The existing `exit.rs` and `idle.rs` pass before
   the extraction and after it, then the rows replace what the clock
   made instant and #131's daemon items close by the rule each names.
   Between sittings.
3. **A spike before t5**: the parking clock with `advance_to_next`
   driving one scripted-server scenario (a 429 hold, say) through the
   door, to see whether "every known task is parked" is a signal the
   door can read without `test-util`. One day; its answer shapes t5's
   brief or sends the question to the owner with two shapes.
4. **t5 (#141)**: the door, the parking clock replacing both test
   clocks, the 215 scenarios through it, `sessions.rs` as journaling
   scenarios, `responses.rs` retired. Its own brief, after the spike.
5. **The store's tail**, small and separable: `ensure_wal` takes its
   deadline as a parameter and gains an expiry pin at zero; the five
   fixed busy timeouts come under a knob or a ruling that they are
   SQLite's (§7).

Not in 4-0: the GUI's subscriber and reconnect (4-full's, born as
rows); the verbs' job-wait poll and the CLI's login poll (frontend
polls that the subscriber replaces in 4-full); the login callback
window (`login.rs:48`, a bounded wait on a browser, kept real behind
its knob).

## 7. Drafts for the owner

Each is his; nothing below is entered until he rules. (Ruled 2026-10-31,
"Accept as drafted": C157 entered, C137 amended, the store's line
added, the knob as shape (a) on #148, the machines on #147.)

- **C<next>, now C157 — A lifecycle that waits is a value: its state is
  data, its step a pure function of the state and one event returning
  the next state and its effects; time enters only as a named
  deadline's event and leaves only as a request to arm one; the
  socket, the process, the file system and the clock live in a driver
  that performs effects and returns events.** A machine is pinned by
  event scripts, one row per transition, in a synchronous test; a
  driver by one real-wiring test per effect kind under the hang
  ceiling, asserting no duration; a cross-process test asserts order
  and outcome. *Why:* a deadline tested in real time is a bet that the
  machine is healthy, and the Windows gate lost that bet most first
  runs; a deadline that is a row cannot be late. *Pinned:* the `Door`
  and `Life` tables, when built.
- **C137 amended, its last sentence:** "A new lifecycle that waits
  across processes is designed as a state machine over an injected
  clock" becomes "A new lifecycle that waits is designed as a machine
  whose time is an event (C157)".
- **`decisions/store.md`, "Not doing":** An injected clock in the
  store: refused. The annotation wait's seam is its knob and SQLite is
  its clock (`busy_timeout` runs inside SQLite); the project pins the
  knob's parse, the expiry verdict at zero and that two writers never
  clobber, in process and across processes. Until the store times a
  wait of its own.
- **The store's fixed waits, two shapes, his pick:** (a) one store
  knob covering every busy timeout and `ensure_wal`'s retry, named in
  C137's list beside `ACQ_ANNOTATION_WAIT`, set by the isolation list;
  or (b) a sentence in the "Not doing" line that the fixed 5 s waits
  are SQLite's and stay fixed. The recommendation is (a): it is one
  knob, the same mechanism C137 already names, and the isolation list
  already widens its sibling.
- **Timing beside the league:** step 1 during sittings, step 2
  between them, the spike and t5 at his word. His machine, his
  window.

## 8. Raised for review

- `TIME-SEAM.md` §4 item 3 (the store's cross-process twin "shrinks
  to a smoke test") is contradicted by the tree; this design corrects
  it in place, dated, as the README's rule for dated evidence allows.
- The watchdog's "earliest moment the verdict could flip" needs the
  limiter's live horizon as an instant; if `is_live` has no end
  instant, the driver keeps a coarse re-check at the limiter's horizon
  and the brief says so. A detail for step 2's brief, not a design
  question.
- Step 2 touches the exit path the owner will be running through
  during the league. The extraction is cut from the existing branches
  with the existing tests passing before and after; still, the owner
  names the window.
- Nothing in this design was run. The spike in §6 is where the one
  mechanism bet (auto-advance without `test-util`) meets the tree.
