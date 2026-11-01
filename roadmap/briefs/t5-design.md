# t5-design — the daemon's tests onto the five-call door, inside the time seam (#141)

Provenance: Fable, the orchestrating session of 2026-10-31, after
4-0's three steps closed (`LEDGER.md`), from the spike's Answer
(`4-0-spike.md`) and one read-only measurement of the daemon's tests
at `a4834369`, every count from a printed command. Cut three times: the first
draft (`6284d681`) and the re-cut (`189792f5`), each reviewed cold
(`roadmap/reviews/t5-design-<sha>.md`, the prompt beside); this cut
answers the second review's seven findings. What each finding
changed is at the foot, "Taken from the reviews". A design brief: §5 is the owner's to rule; the rounds of
§4 become build briefs after it, one at a time, each reviewed cold.
Nothing here is a ruling until he says so.

## 1. What the tree says

Every number is a count of textual occurrences unless it says
"calls" (occurrences less definitions) or "rows"; the review re-ran
each.

The daemon's tests at `a4834369`: 197 tokio tests and 28 synchronous
under `crates/acquisition-daemon/src/daemon/tests/`, 16 and 46 in the
limiter's spec (`ratelimit/tests/spec.rs`), one synchronous in
`tests/main.rs` (the binary's arguments); 12 of the tokio tests run
on several workers (`credentials.rs`, ten at four workers; `exit.rs`
387 and 1098 at two). They enter the daemon by its internals: the
queue's `submit` 111 times (`dispatch.rs` 39), `handle_request` 27,
`handle_conn` never; the harness's `test_daemon*` constructors 121
calls and `Daemon::builder` twice (the harness, 544; `sessions.rs`
216, its own `test_daemon` on `SystemClock` and `Rails::disabled()`).
They read private state through `shared.lock()` 139 times on one
line, 162 counting the expression across line breaks (`login.rs` 33,
`sessions.rs` 21, `queue.rs` 14), the sessions' `auth` 154 (`login.rs`
46), the choke point 79. The oracles are one teardown family in
`harness.rs`: `finish_harness` (664, the wire contract and the pacing)
and `finish_harness_mock` (674, never over the limit), both through
`finish_with` (686), which skips every oracle when no journal file
exists (694); `finish_harness_wire` (734) adds the journal's equality
with the server's recorded methods (`assert_journal_matches_wire`,
713). 130 call sites end through `finish_harness`, 33 through the wire
form, 5 through the mock's; `diagnostics.rs`, `list.rs`,
`responses.rs` and `sessions.rs` end through none. The scripted server
(`harness.rs:351`, a `Vec<ScriptedResponse>` answering one connection
each in order, a complete body of the stated size every time,
recording the methods) is called 44 times; the other fixtures are the
account server (`harness.rs:1046`, answers by account), the token
servers (384, 949) and `sessions.rs`'s own servers with arrival and
release barriers and request-body checks (94); the stateful mock
(`mockggg.rs:602`) 11 calls, seven of them on `SystemClock` beside a
daemon on `ManualClock` (`dispatch.rs:456`, `parents.rs:15`,
`login.rs` 78, 190, 232, 252, 635): the clock split.

The clocks: `ManualClock` 113 mentions in 16 files (its `sleep`
advances itself and yields, `harness.rs:64`; `clock.advance` called
in five files); `BlockingClock` 5, one file, one test; `SystemClock`
two in these tests, five in the spec. Real time left after 4-0 step
2: 20 fixed sleeps that stand for a state wait (`rails.rs` nine,
`dispatch.rs` five, `exit.rs` three, `queue.rs:1713`,
`credentials.rs:358`, `spec.rs:2016`); 27 literal ceilings of one to
ten seconds where the hang ceiling belongs; two polls with no ceiling
(`login.rs` 346 and 404, waits on a login whose code exchange can
end before any profile job exists, 342 and 403); the elapsed reads of
`list.rs:172` and `exit.rs` 301 and 428, the lower bounds step 2
kept: 301's measures the tokio drain, 428's the bound's real thread.

The scenario families, by their script constructors and one test each
(rows where the test is a table): a 401 (`bearer.rs:486`); GGG's 429
(`dispatch.rs:518`), a 429 whose `Retry-After` is missing, garbled,
negative or over the cap (`dispatch.rs:597`, four rows), a HEAD's 429
(`dispatch.rs:1252`); Cloudflare's 1015 and 403 challenge
(`dispatch.rs:744`) and 522 (`dispatch.rs:984`); an origin 503
(`dispatch.rs:1109`) and GGG's own 4xx (`dispatch.rs:910`, four rows);
the laptop sleep (`bearer.rs:534`, three calls); the tripwire
(`exit.rs:1367`, six calls); a keyring that fails (`login.rs:1012`,
eight) and a store that cannot be written (`login.rs:1201`, six); the
exit and drain (`credentials.rs:318`); the idle lifecycle
(`exit.rs:909`); a cancel (`dispatch.rs:1630`). `responses.rs`'s five
synchronous tests judge the classification of a body as a function;
`dispatch.rs`'s wire tests hold two of the five at the boundary (the
retry bound, `responses.rs:27`, by 518 and 553 jointly; the body
outcome that requeues, 40, by 595 and the headerless 429 at 744) and
not the other three: a truncated 200 staying a transport failure with
its cause (92), a non-2xx status keeping precedence over a body that
failed to transfer (111), and a 200 whose body is not JSON (129; the
malformed-listing wire test at `dispatch.rs:376` sends valid JSON).
`BlockingClock`'s one test (`dispatch.rs:192`) proves a zero-duration
job completes while two different rate waits stay parked (233, 244).
The exempt-route list is written three times in two domains:
`routes.rs:154` governs the API's probes (`profile`, `league`); the two
oracles (`daemon/tests/harness.rs:787`,
`acquisition-client/tests/harness/mod.rs:720`) add `oauth-token`,
which goes by another path and is exempt by that rule, not this
list; no oracle asserts a HEAD absent on an exempt route's first
send, the wire contract only skips them. `.cargo/mutants.toml` covers
the limiter, the policy parse and the gate, its 12 exclusions naming
exact lines; the baseline is text (`LEDGER.md`'s #86 row: 388 mutants,
46 missed, 34 pinned since, 12 listed), the weekly workflow's artifact
(four shards on its own checkout, `.github/workflows/weekly.yml` 35,
55) the only run.

The spike's Answer, which this design rests on: a parking clock
behind `Clock` drives the 429 hold through the daemon in virtual time
and journals the system clock's three sends in the same order; the
design's door count is refuted (right with one task parked, wrong
with two); the exact signal that nothing can run is a current-thread
runtime's `on_thread_park` hook, stable tokio, advancing the clock only
while the gate's occupancy and the daemon's `landings` are zero; the
`Life` driver moves onto the clock together with the exit's deadline
and the quiet wait's look, never alone; the judge's `spawn_blocking`
(`server.rs:882`; the landing's at `daemon.rs:1416`) is the one
blocking thread nothing counts; multi-thread runtimes are out, since
the hook fires per worker. The daemon's other threads are persistent
and never scenario work: the signal watcher (`start.rs:162`), the
exit bound's thread (`daemon.rs:276`); the reaper is the client's.

## 2. The shape

One sentence: **a scenario crosses the daemon's frontend boundary
only: a script, requests and clock moves in, a journal and the
protocol's answers out, through five calls, on a runtime that moves
time automatically only when nothing the scenario started can run.**

1. **The door**, in the daemon's test support (`daemon/tests/door.rs`),
   five calls and what a scenario needs beside them:
   - `Door::open(server, clock, credentials) -> Door`, with a builder
     for what the harness's constructors vary today (rails, the
     initial sessions, a persistent world and database, the exit's
     bound, the events capacity): the daemon from `Daemon::builder`
     on the given clock and credential store; journaling on always;
     the park hook installed over it. `server` is one of the
     harness's fixture kinds, each kept as it is: `Script`
     (`ScriptedResponse`s, gaining a raw form for a truncated or
     non-JSON body), `Account` (answers by account), `Token` (the two
     token servers), `Mock` (the stateful mock, on the daemon's
     clock), `Own` (a server the scenario builds, as `sessions.rs`
     does with barriers and body checks) and `None`.
   - `door.request(request) -> Response`: a protocol `Request` through
     `handle_request` (`server.rs:254`), the path every frontend takes,
     so a submit's `JobId`, a refusal, an auth reply, a status, a
     cancel's acknowledgement and a logout's result all come back as
     the frontend sees them, with priority and account as
     `Request::Submit` carries them (`protocol.rs:424`).
   - `door.await_terminal(id) -> (JobInfo, Outcome)`: on the daemon's
     job events (finish, cancel and restore all emit: `dispatch.rs:471`,
     `queue.rs` 648 and 349), the events a hint and the state re-read
     (subscribe before the snapshot, as the frontends do); and
     `door.await_auth()` for the login's waits (a login landed or
     failed before any profile job exists, `login.rs` 342 and 403):
     these the job events cannot wake (`finish_auth_flow_error` at
     `login.rs:472` records the result and publishes no event; the
     success is published after the profile job's, 341), so the daemon
     gains an auth-change notification at its auth transitions (the
     flow's end, success and failure, the logout), the door subscribes
     before it reads, and the answer is read through `AuthStatus`
     (`protocol.rs:455`, which documents the poll it replaces).
   - `door.journal() -> Journal`: the sends as the oracles read them.
   - `door.finish()`: the teardown and the oracles the server kind
     selects, run always: the door supplies a scratch `journal_path` to every
     daemon it builds, whatever rails it was given (the builder's
     default rails have none, `daemon.rs:1232`, and `rails.rs:447–480`
     writes no header without one; the harness sets one at
     `harness.rs:506`), fails construction if it cannot open it, and
     so has a journal file even under `None`; a missing file at finish
     fails, and the empty journal's oracles run. The skip at `harness.rs:694` is not the
     door's.
   A sixth, `door.advance(d)` (and `laptop_sleep`), moves the clock by
   the scenario's own hand, for a verdict read at a moment
   (`idle.rs`'s shape) and the refresh families' expiry
   (`refresh.rs:405`); the hook moves it otherwise.
2. **The runtime is the door's.** A scenario is a `#[test]` function
   calling `Door::run(|door| async { … })`, which builds a
   current-thread runtime, installs the park hook and blocks on the
   scenario. The hook, at every park: if a send is in flight (the
   gate's occupancy, `choke.rs:466`) or a scenario's blocking task is
   (below), leave the clock; else `advance_to_next()`. Time therefore
   moves on its own exactly when the runtime has nothing runnable and
   nothing the scenario started runs off it: tokio's own paused-clock
   rule behind the project's trait, with no feature split (C84).
3. **Every blocking task the daemon spawns is counted.** The daemon
   gains one helper, `Daemon::blocking(f)`, the only `spawn_blocking`
   in `crates/acquisition-daemon/src` outside `tests/`, counting from
   before the spawn until the closure returns; the landing's and the
   judge's threads go through it. `landings` stays its own count,
   what the idle verdict and the drain read (C148): the hook reads
   the blocking count, the verdict does not. The persistent threads
   (the signal watcher, the bound's thread) are not scenario work and
   are not counted. `tools/arch-check.sh` pins it: `spawn_blocking`
   in the daemon's non-test sources at that one site (the test at
   `credentials.rs:255` is outside the guard's scope). A new blocking
   task cannot silently break determinism.
4. **The clock**: `ParkingClock` as the spike's branch holds it, in
   the harness (`advance` keeps its name and meaning; `laptop_sleep`
   kept; `sleepers()` subsumes `wait_for_sleepers`); `Clock::
   sleep_until` on `main` with the spike's default; the stateful mock
   on the daemon's clock (`start_with_clock`) at the seven split
   sites. `ManualClock` and `BlockingClock` retire only when their
   last user has moved (§4, round 5): a test on an ordinary tokio
   runtime with the harness's yielding `wait_terminal` (582) cannot
   run on a parking clock, since the hook never fires beside a yield
   loop.
5. **`Life` on the clock**, the three waits together: the idle
   deadline and the drain through `sleep_until`, the quiet wait's look
   through the clock's `sleep`; `Exit::arm` sets the drain's deadline
   on the daemon's clock (`daemon.rs:267` today, tokio's), so the
   driver reads one clock; the bound's OS thread stays real (C133).
   Every caller of `Life` moves in the same round, the retained
   component tests included: a drain driven on `ManualClock`
   (`credentials.rs` 564 and 781, through the harness's `drain` at
   633) would expire at once, since that clock's `sleep` advances
   itself and the default `sleep_until` is its `sleep`; such a test
   takes the system clock or a parking clock it advances by hand.
   Then `exit.rs`'s drain tests run in virtual time through the door:
   `HELD` and `BOUND` virtual, the `settle` sleeps awaited events;
   `exit.rs:301`'s lower bound (the tokio drain) becomes a logical
   assertion on the clock's deadline, and 428's (the bound's real
   thread) stays real with its test, off the door.
6. **What stays off the door, by the boundary it needs**, each file
   saying so in its module doc:
   - several workers: `credentials.rs`'s ten keyring tests,
     `exit.rs:387` (the bound thread against blocked workers) and
     `exit.rs:1098` (a signal's admission while `shared` is held);
   - a child process: `credentials.rs:466`, `exit.rs:1494`;
   - the daemon's own frontend socket or callback socket, which no
     scripted GGG server stands in for: `credentials.rs` 132 and 646
     (a real protocol connection), the login's callback tests on
     `wait_callback` (`login.rs` 531, 561); `exit.rs:1618` serves a
     real listener, 1662 a fabricated one (a component test);
   - component tests of a function or a table: `responses.rs`, `idle.rs`'s verdict reads, the `Life` and
     `Door` tables, `queue.rs`'s internal-input tests that submit as
     the `daemon` submitter what the protocol refuses (427, 491; the
     admission rules at `queue.rs` 1183 and 1204), fabricated queue
     and verdict states;
   - the limiter's spec and `tests/main.rs`.
   `credentials.rs:318` (a stop waiting for a held clear) needs none
   of these once its sleep is an awaited event and migrates.

What the shape does not promise: a server that never answers. A
silent request holds its send permit through its real timeout
(`choke.rs` 1008, 1018), so the hook freezes virtual time beside it
and the scenario waits the real timeout; a script answers every
connection it is asked (the delayed token server already stages a
late answer), and a scenario that needs a timeout's expiry is out of
the door until one is asked for (§5's line).

## 3. What each family becomes

| Today | Under the door | Retires, held by |
| --- | --- | --- |
| the 429 hold, the terminal `Retry-After`s, the HEAD 429 (`dispatch.rs`) | one table of rows, each a script, the requests and the expected journal (method, path, status, counted, `wait_ms`) | the 100 ms and 50 ms sleeps at `dispatch.rs` 812, 1050, 1441, 1652, 1776, held by awaited events |
| `BlockingClock`'s one test (`dispatch.rs:192`) | the same test on the parking clock, asserting what it asserts today: the independent job completes while two rate waits stay parked (`sleepers() == 2`) | `BlockingClock`, held by that test |
| Cloudflare's three shapes, the origin 5xx, GGG's 4xx | the same table, the failure read as the daemon classifies it (below) | the sleeps that prove absence; the literal ceilings |
| the 401, the laptop sleep, the refresh families (`bearer.rs`, `refresh.rs`) | rows with a clock-move column (`advance`, `laptop_sleep`) | nothing until round 5 |
| `responses.rs` | kept as the classification's component table (`classify_api_body`, `may_requeue_429`); the boundary gains three wire twins through the script's raw form (a truncated 200 a transport failure with its cause; a truncated body under a non-2xx status; a 200 whose body is not JSON), so every classification the table names is seen once at the boundary | nothing |
| `sessions.rs` | journaling scenarios through `Own` servers: "one owner" is "one token POST on the wire", read from the journal, not from `auth` | its own `test_daemon`, `SystemClock`, `Rails::disabled()`, its `shared.lock()`s, the generation counters |
| `login.rs` | the login's scenarios through the door where they cross the frontend boundary, their `shared.lock()`s and `auth` reads replaced by the journal, the protocol's answers and `await_auth`; the callback tests off the door | the two unbounded polls (346, 404) |
| the exit and the drain (`exit.rs`, `credentials.rs:318`) | virtual time through the door; the `Life` table and its wiring tests as they are | `settle`; `HELD` and `BOUND` as real durations |
| the exempt-route list | the two oracles' lists are `NO_PROBE_ROUTES` composed with the token rule, stated as such; the daemon's harness reads the constant, the client's keeps its copy under a check (§5) | one copy; plus one new oracle, a HEAD absent on an exempt API route's first send |
| the idle verdict (`idle.rs`) | as it is, on the parking clock's `advance` | nothing |
| the limiter's spec | as it is | nothing |

How a row reads a failure: `Outcome::Failure` carries an error, its
children and a payload and no classification field
(`protocol.rs`, `job.rs:150`), and several of the kinds a row would
name are flattened to `ApiError::Other` on the way (`kinds.rs` 1070,
1082, 1091), while halted work is parked, not terminal
(`dispatch.rs:565`). So a door row asserts what crosses the boundary: the journal (the
sends, their waits, a retry made or not) and the protocol's answer
(the outcome, the error's words where a golden holds them); the
classification itself stays a component property, `responses.rs`'s
table over `classify_api_body`, and the ruling's boundary is kept
without an exception. A classification field in `Outcome` would be
a protocol change, serialized and persisted, a user's JSON: §5 puts
it to the owner as a question, not a plan.

## 4. The rounds

Each a build brief, gated, committed, reviewed cold, one at a time,
in the main tree between sittings; no round needs a later one, and a
retirement waits for its last user.

0. **The baseline**, before round 1, on the round's input commit, in
   the background on a machine the owner names (hours): `cargo
   mutants` over `.cargo/mutants.toml`'s scope, all shards, with the
   commit, the config, the toolchain and the tool's version recorded,
   its `mutants.out` kept under `runs/`. Round 6 compares
   corresponding mutants and classifies additions and removals apart.
   Dispatched on 2026-10-31 by the orchestrating session, after
   round 4: the weekly workflow (`workflow_dispatch`) on the tag
   `t5-round0-baseline` (`a8a719d6`, round 1's parent), run
   41265798977, four shards, its artifacts the baseline; no machine
   of the owner's.
1. **The clock and the door.** `ParkingClock`, `Clock::sleep_until`,
   the counted blocking helper and its arch check, the auth-change
   notification, the door's five calls with its builder, fixture
   kinds and scratch journal, the 429 hold as the first
   table (the spike's scenario), the mock on the daemon's clock at
   the seven sites, `BlockingClock`'s test moved and its clock retired.
   The gate's test time must not grow: the table runs in milliseconds.
2. **`Life` on the clock.** The three waits and `Exit`'s deadline onto
   the clock; every `Life` caller inventoried and moved with it, the
   retained component tests on a system or hand-driven parking clock;
   `exit.rs`'s drain tests through the door in virtual time; 301's
   bound logical.
3. **The families as tables.** The scripted-server families of §3's
   first and third rows, asserted at the boundary; the three wire
   twins; `responses.rs` kept as the component table.
4. **The sessions and the login as scenarios.** `sessions.rs` and
   `login.rs` through the door where they cross the boundary; the
   private-state reads out of the top five files where a journal, an
   answer or `await_auth` holds the property; the rest named as
   component tests. Narrowed on 2026-10-31 by the orchestrating
   session under the owner's grant of autonomy, for his re-read: the
   round takes `login.rs`, `sessions.rs` and the cancel family
   (`t5-r4.md`); the private reads of the other files join round 5.
5. **The sweep.** The refresh and bearer families onto `advance`;
   `ManualClock` retired, its last user moved; the exempt-route
   source and its oracle; the 20 fixed sleeps to awaited states; the
   27 literal ceilings and the two unbounded polls to the hang
   ceiling; #131's remaining daemon rows closed by their rules.
   Split on 2026-10-31 by the orchestrating session under the owner's
   grant of autonomy, for his re-read, from two inventories at
   `9736bdcf`: 5a the refresh and bearer families (`t5-r5a.md`); 5b
   `queue.rs` and `rails.rs` (`t5-r5b.md`; 37 door-able tests of 46 by
   the inventories of 2026-10-31, which overturned the first count's
   classes) and round 4's narrowed private reads of those two files;
   5c the sweep, with the nine door-able tests of the other files (`ManualClock`'s retirement as
   a harness-level swap, four tests relying on its self-advancing
   sleep; the 14 sleeps, 19 ceilings and the polls left; the
   exempt-route check and its registry line). Two premises corrected
   by those inventories: the laptop sleep has no production detection
   (the wall-face expiry is the mechanism), and a yield-loop test can
   run on a hand-driven parking clock (four already do); the blocker
   is a production sleep on the clock.
6. **The close.** `cargo mutants` after round 5 against round 0's
   baseline: no new survivor in the configured scope; the scenario
   count before and after (rows, not occurrences); the gate's test
   time before and after.

## 5. Drafts for the owner

Each is his; nothing below is entered until he rules. (Ruled
2026-10-31, "Accept as drafted with your recommendations. The
reasoning makes sense.": C158 entered in `decisions/daemon.md` with
the two "Not doing" lines, the `Outcome` field declined until a
frontend asks, the exempt-route list kept in the client's harness
under a check.)

- **C<next>, now C158 — A daemon scenario crosses the daemon's frontend
  boundary only, and its time moves by its own hand or when nothing
  it started can run.** It enters through the protocol and reads the
  journal and the protocol's answers; it moves its clock explicitly
  or lets the runtime move it, never a poll; a test that needs
  another boundary (several workers, a child process, the daemon's
  own socket, a function or a table) says which and stays a component
  test. *Why:* a test that drives internals pins this build's
  mechanics and dies with a rewrite; a count of waiting tasks is
  wrong once two park, and the runtime's own park is the one exact
  signal. *Pinned:* nothing yet (the door's table and the arch check
  on `spawn_blocking`, when built).

  The mechanism, the park hook and its two counts, is the door's doc
  comment, not the entry's.
- **A "Not doing" line for `decisions/daemon.md`:** virtualising a
  request timeout's expiry: not until a scenario requires it. A
  script answers every connection, a late answer is staged by the
  delayed server, and a silent one waits its real timeout.
- **A question, not a draft: a classification field in `Outcome`.**
  Classification stays in component tests; a
  field in `Outcome` would be the frontends' to read and a user's
  JSON to change (serialized, persisted, read back by older
  answers). The owner says whether it is wanted now, as an issue
  with a trigger, or never.
- **Where the client's harness reads the exempt-route list** (C1: the
  client links the protocol and the store, never the daemon): three
  shapes: the list as data in the protocol crate; a
  standard-library-only data file both harnesses include; the client
  keeps its copy under a check that compares it with
  `NO_PROBE_ROUTES` composed with the token rule. The smallest is the
  check; the build brief for round 5 brings the three if the owner
  does not rule here.

## 6. Raised for review

- The judge counted as blocking work changes nothing the idle verdict
  reads; a scenario that holds `shared` on another thread while the
  judge waits never advances the clock, which is why the blocked-judge
  test stays multi-thread and off the door.
- `handle_request` as the door's path takes the refusals' words
  through the protocol, which the frontends' goldens hold; a row
  asserting a refusal asserts its kind, not its sentence.
- The scenario count's fall (215 toward about 120, `TEST-SHAPE.md`
  §3) is an estimate the rounds measure in rows; no round is held to
  it.
- The baseline's hours: round 0 is a machine's night, not a worker's
  day; the owner names the machine.

## Taken from the reviews

The first review's thirteen (`t5-design-6284d681.md`), as the second
review judged them:

1. The door's submit could not carry a refusal, an auth reply or a
   non-terminal wait, and the internal-input tests cannot enter
   through the protocol: `request -> Response`, `await_auth`, and
   the component partition (§2.1, §2.6).
2. `ManualClock` retired before its users moved: the retirement
   waits for the last user, in round 5 (§2.4, §4).
3. Two holders held less: `BlockingClock`'s test kept on the parking
   clock with its two parked waits; `responses.rs`'s two unheld
   properties get wire twins first, each of the five named by line
   (§1, §3).
4. The server fixtures and construction inputs: the door's kinds
   are the harness's, with a raw script form, and a builder for what
   the constructors vary (§2.1).
5. A failure kind in `Outcome` is a protocol change: read from a
   daemon-internal enum; the field a question for the owner (§3, §5).
6. `exit.rs:301`'s lower bound measures the tokio drain and goes
   logical; 428's stays real; the already-retired ceilings struck
   (§1, §2.5, §3).
7. The off-door inventory by boundary, with 1098 and 1662 named
   rightly, the callback tests added, `credentials.rs:318` migrating
   (§2.6).
8. The missing-journal skip: every door scenario has a journal, a
   missing file fails (§2.1).
9. The blocking count's scope: scenario work only, `landings` kept
   for the verdict, the persistent threads named, the guard scoped;
   the silent-server claim corrected to the real timeout (§2.3, §2's
   last paragraph).
10. The mutants baseline: round 0 before round 1, recorded and
    shard-complete (§4).
11. The ruling draft as a property under 800 bytes, explicit moves
    preserved, the mechanism to the code (§5).
12. The exempt lists' two domains and a third shape (§1, §3, §5).
13. The census's occurrences labelled, the calls and rows counted as
    such (§1).

The second review's seven (`t5-design-189792f5.md`):

1. The job events cannot wake the auth waits: an auth-change
   notification, the door subscribing before it reads `AuthStatus`
   (§2.1, round 1).
2. `responses.rs:92` had no holder and the five-test map was wrong:
   the map corrected (two held, three unheld); the file is kept as the
   classification's component table and the boundary gains three
   twins (§1, §3, round 3).
3. Round 2 left `Life` callers on the manual clock: every caller
   moves in the round (§2.5, round 2).
4. The internal classification was an exception the ruling did not
   name: door rows assert the boundary, the classification stays a
   component property (§3).
5. A header only under a configured journal path: the door supplies
   a scratch one and fails construction without it (§2.1).
6. The ruling's form: the parenthesis out, the mechanism to the
   code, the pins named as "nothing yet" with what will pin it; the
   "Not doing" line in the registry's wording (§5).
7. The drain deadline's citation: `daemon.rs:267` (§2.5).
