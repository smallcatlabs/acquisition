# t5-design — the daemon's tests onto the five-call door, inside the time seam (#141)

Provenance: Fable, the orchestrating session of 2026-10-31, after
4-0's three steps closed (`LEDGER.md`), from the spike's Answer
(`4-0-spike.md`) and one read-only measurement of the daemon's tests
at `a4834369`, every count from a printed command. A design brief:
§5 is the owner's to rule; the rounds of §4 become build briefs after
it, one at a time, each reviewed cold. Nothing here is a ruling
until he says so.

## 1. What the tree says

The daemon's tests at `a4834369`: 213 tokio tests, 75 synchronous,
under `crates/acquisition-daemon/src/daemon/tests/` (one more in
`tests/main.rs`); 12 of the tokio tests run on several workers
(`credentials.rs`, ten at four workers; `exit.rs` 387 and 1098 at
two). They enter the daemon by its internals: the queue's `submit`
111 times (`dispatch.rs` 39), `handle_request` 27 times, `handle_conn`
never; the harness's `test_daemon*` constructors 126 times and
`Daemon::builder` twice (the harness, 544; `sessions.rs` 216, its own
`test_daemon` on `SystemClock` and `Rails::disabled()`). They read
private state 139 times through `shared.lock()` (`login.rs` 29,
`sessions.rs` 18, `queue.rs` 14), the sessions' `auth` 124 times
(`login.rs` 42), the choke point 65 times. The oracles are one
teardown family in `harness.rs`: `finish_harness` (664, the wire
contract and the pacing) and `finish_harness_mock` (674, never over
the limit), both through `finish_with` (686), which skips every oracle
when no journal exists (693); `assert_journal_matches_wire` (713).
132 tests end through `finish_harness`, 34 through the wire form, 6
through the mock's; `diagnostics.rs`, `list.rs`, `responses.rs` and
`sessions.rs` end through none. The scripted server (`harness.rs:351`,
a `Vec<ScriptedResponse>` answering one connection each in order,
recording the methods) is called 45 times; the stateful mock
(`mockggg.rs:602`) 11, seven of them on `SystemClock` beside a daemon
on `ManualClock`: the clock split.

The clocks: `ManualClock` 113 mentions in 16 files (its `sleep`
advances itself and yields, `harness.rs:64`); `BlockingClock` 5, one
file, one test; `SystemClock` 7. Real time left in the tests after
4-0 step 2: 21 fixed sleeps that stand for a state wait (`rails.rs`
nine, `dispatch.rs` five, `exit.rs`'s `settle` and two more,
`queue.rs:1713`, `credentials.rs:358`, `spec.rs:2016`); 27 literal
ceilings of one to ten seconds where the hang ceiling belongs; two
polls with no ceiling at all (`login.rs` 346 and 404); the elapsed
reads of `list.rs:172` and `exit.rs` 301 and 428, the last two the
lower bounds step 2 kept.

The scenario families, by the script constructors and the test that
stages each: a 401 (`bearer.rs:486`); GGG's 429 (`dispatch.rs:518`,
sixteen stagings), a 429 whose `Retry-After` is missing, garbled,
negative or over the cap (`dispatch.rs:595`, five), a HEAD's 429
(`dispatch.rs:1252`); Cloudflare's 1015 and 403 challenge
(`dispatch.rs:744`) and 522 (`dispatch.rs:984`); an origin 503
(`dispatch.rs:1109`) and GGG's own 4xx (`dispatch.rs:897`, eleven); the
laptop sleep (`bearer.rs:534`, four); the tripwire (`exit.rs:1367`,
seven); a keyring that fails (`login.rs:1012`, eight) and a store that
cannot be written (`login.rs:1201`, seven); the exit and drain
(`credentials.rs:318`); the idle lifecycle (`exit.rs:909`); a cancel
(`dispatch.rs:1630`). `responses.rs`'s five synchronous tests judge
the classification of a body as a function; `dispatch.rs`'s wire tests
stage every case but one (a 200 whose body fails to transfer, which
no wire test stages). The exempt-route list is written three times
and the copies disagree: `routes.rs:154` has `profile` and `league`;
the two harnesses (`daemon/tests/harness.rs:787`,
`acquisition-client/tests/harness/mod.rs:720`) add `oauth-token`,
unprobed by another path; no oracle asserts a HEAD absent on an
exempt route, the wire contract only skips them. `.cargo/mutants.toml`
covers the limiter, the policy parse and the gate; the baseline is
text (`LEDGER.md`'s #86 row: 388 mutants, 46 missed, 34 pinned since,
12 listed), the weekly workflow's artifact the only run.

The spike's Answer, which this design rests on: a parking clock
behind `Clock` drives the 429 hold through the daemon in virtual time
and journals the system clock's three sends in the same order; the
design's door count is refuted (right with one task parked, wrong
with two); the exact signal that nothing can run is a current-thread
runtime's `on_thread_park` hook, stable tokio, advancing the clock only
while the gate's occupancy and the daemon's `landings` are zero; the
`Life` driver moves onto the clock together with the exit's deadline
and the quiet wait's look, never alone; the judge's `spawn_blocking`
is the one blocking thread nothing counts; multi-thread runtimes are
out, since the hook fires per worker.

## 2. The shape

One sentence: **a scenario is a script, requests and clock moves in,
a journal out, through five calls, on a runtime that moves time only
when nothing can run.**

1. **The door**, in the daemon's test support (`daemon/tests/door.rs`),
   five calls and nothing else a scenario needs:
   - `Door::open(server, clock, credentials) -> Door`: the daemon from
     `Daemon::builder` on the given clock and credential store, facing
     a scripted server, the stateful mock on the same clock, or no
     server; the journal beside its log; the park hook installed over
     it. `server` is `Script(Vec<ScriptedResponse>)`, `Mock` or `None`.
   - `door.submit(request) -> JobId`: a protocol `Request`, through
     `handle_request` (the request path every frontend takes, with
     its refusals), never the queue's `submit`.
   - `door.await_terminal(id) -> (JobInfo, Outcome)`: on the daemon's
     job events, never a poll or a yield (the spike's `await_terminal`).
   - `door.journal() -> Journal`: the sends as the oracles read them.
   - `door.finish()`: the teardown and the oracles the server kind
     selects (`finish_harness`, its wire or mock form), run always,
     never skipped for a missing journal: a scenario with no journal
     says so by its server kind.
   A sixth, `door.advance(d)`, moves the clock by hand for the tests
   that read a verdict at a moment (`idle.rs`'s shape); the door never
   advances on its own except through the hook.
2. **The runtime is the door's.** A scenario is a `#[test]` function
   calling `Door::run(|door| async { … })`, which builds a
   current-thread runtime, installs the park hook and blocks on the
   scenario. The hook, at every park: if a send is in flight (the
   gate's occupancy) or a blocking thread is (the daemon's count),
   leave the clock; else `advance_to_next()`. Time therefore moves
   exactly when the runtime has nothing runnable and the daemon has
   nothing off the runtime, which is tokio's own paused-clock rule
   behind the project's trait, with no feature split (C84).
3. **Every blocking thread is counted.** The daemon gains one helper,
   `Daemon::blocking(f)`, the only `spawn_blocking` in the crate,
   counting from before the spawn until the closure returns (as
   `landings` already does for the keyring writes, which it
   subsumes for the hook's purpose); the judge's thread goes through
   it. `tools/arch-check.sh` pins it: `spawn_blocking` appears in
   `crates/acquisition-daemon/src` at that one site. A new blocking
   task cannot silently break determinism.
4. **The clock**: `ParkingClock` as the spike's branch holds it, in
   the harness, replacing `ManualClock` (`advance` keeps its name and
   meaning; `laptop_sleep` kept) and `BlockingClock` (`sleepers()`
   subsumes `wait_for_sleepers`); `Clock::sleep_until` on `main` with
   the spike's default; the stateful mock always on the daemon's
   clock (`start_with_clock`), closing the clock split.
5. **`Life` on the clock**, the three waits together: the idle
   deadline and the drain through `sleep_until`, the quiet wait's look
   through the clock's `sleep`; `Exit::arm` sets the drain's deadline
   on the daemon's clock, so the driver reads one clock; the bound's
   OS thread stays real (C133). Then `exit.rs`'s drain tests run in
   virtual time through the door: `HELD` and `BOUND` become virtual,
   the `settle` sleeps become awaited events, the lower bounds of
   `exit.rs` 301 and 428 stay for the real thread.
6. **What stays off the door, named**: the twelve multi-thread tests
   (the keyring's `credentials.rs` ten, `exit.rs`'s blocked-worker
   bound and listener-error tests), the two child-process helpers
   (`credentials.rs:466`, `exit.rs:1494`), the three real-socket
   protocol tests (`credentials.rs` 132 and 646, `exit.rs` 1618 and
   1662), `tests/main.rs` (the binary's arguments), and the limiter's
   spec (`ratelimit/tests/spec.rs`, the component's own tests, on its
   own clocks). Each file that stays says so in its module doc.

What the shape does not promise: a scenario whose server never
answers. reqwest's connect and request timeouts are real time under
the hook (the runtime idles beside them and virtual time moves), so a
script answers every connection it is asked, as the scripted server
does today; a hang is the hang ceiling's.

## 3. What each family becomes

| Today | Under the door | Retires |
| --- | --- | --- |
| the 429 hold, the terminal `Retry-After`s, the HEAD 429 (`dispatch.rs`) | one table of rows, each a script, the requests and the expected journal (method, path, status, counted, `wait_ms`) | the `BlockingClock` and its one test; the 100 ms and 50 ms sleeps at `dispatch.rs` 812, 1050, 1441, 1652, 1776 |
| Cloudflare's three shapes, the origin 5xx, GGG's 4xx | the same table, a failure kind column | the sleeps that prove absence; the literal ceilings |
| the 401, the laptop sleep, the refresh families (`bearer.rs`, `refresh.rs`) | rows with a clock move column (`advance`, `laptop_sleep`) | `ManualClock` |
| `responses.rs` | the body-transport case gains its wire twin first; then the file retires, held by the table's failure-kind column (C154's line) | `responses.rs` |
| `sessions.rs` | journaling scenarios: "one owner" is "one token POST on the wire", read from the journal, not from `auth` | its own `test_daemon`, `SystemClock`, `Rails::disabled()`, 18 `shared.lock()`s, the generation counters |
| `login.rs` | the login's scenarios through the door, its 29 `shared.lock()`s and 42 `auth` reads replaced by the journal and the protocol's answers where they pin a contract, dropped where they pin a mechanism | the two unbounded polls (346, 404) |
| the exit and the drain (`exit.rs`, `credentials.rs:318`) | virtual time through the door; the `Life` table and its wiring tests as they are | `settle`; `HELD` and `BOUND` as real durations; the 3 s and 5 s ceilings |
| the exempt-route list | one source: the harnesses read `NO_PROBE_ROUTES` (the daemon's through the crate, the client's through the protocol or a shared constant, ruled in the build brief); `oauth-token`'s exemption named where it is decided | two copies; plus one new oracle, a HEAD absent on an exempt route's first send |
| the idle verdict (`idle.rs`) | as it is, on the parking clock's `advance` | nothing |
| the limiter's spec | as it is | nothing |

A failure carries a structured kind (the classification `responses.rs`
reads: rate-limited, terminal rate limit, Cloudflare, transport, bad
JSON, halted, cancelled) in `Outcome`, so a row asserts the kind once
and the words stay the goldens'. This is agent-facing only where a
frontend reads it; the build brief says which.

## 4. The rounds

Each a build brief, gated, committed, reviewed cold, one at a time,
in the main tree between sittings; no round needs a later one.

1. **The clock and the door.** `ParkingClock`, `Clock::sleep_until`,
   the counted blocking helper and its arch check, the door's five
   calls, the 429 hold as the first table (the spike's scenario), the
   mock on the daemon's clock. Retires `BlockingClock`. The gate's
   runtime must not grow: the table runs in milliseconds.
2. **`Life` on the clock.** The three waits and `Exit`'s deadline onto
   the clock; `exit.rs`'s drain tests through the door in virtual
   time; `ManualClock` retired, its fifteen files on `advance`.
3. **The families as tables.** The scripted-server families of §3's
   first three rows, with the failure kind; `responses.rs` retired
   after its twin lands.
4. **The sessions and the login as scenarios.** `sessions.rs` and
   `login.rs` through the door; the private-state reads out of the
   top five files where a journal or an answer holds the property.
5. **The sweep.** The exempt-route source and its oracle; the 21 fixed
   sleeps to awaited states; the 27 literal ceilings and the two
   unbounded polls to the hang ceiling; `#131`'s remaining daemon rows
   closed by their rules.
6. **The close.** `cargo mutants` over the limiter and the gate before
   round 1 (hours, in the background, its `mutants.out` kept under
   `runs/`) and after round 5: no new survivor; the scenario count
   before and after; the gate's test time before and after.

## 5. Drafts for the owner

Each is his; nothing below is entered until he rules.

- **C<next> — The daemon's tests enter through one door, and time
  moves only when nothing can run.** A scenario constructs a daemon on
  a clock and a credential store facing a script, the mock or nothing;
  submits through the protocol; awaits terminal on the daemon's
  events; reads the journal; finishes through the oracles. The
  scenario owns a current-thread runtime whose park hook moves the
  clock to its next deadline only while no send and no blocking
  thread is in flight, and every blocking thread the daemon spawns is
  counted through one helper. A test that needs several workers, a
  real socket or a child process stays off the door and says so. *Why:*
  the spike showed the door's own count of waiting tasks is wrong once
  two park, and the runtime's park is the one exact signal; a test
  that drives internals pins this build's mechanics and dies with a
  rewrite. *Pinned:* the arch check on `spawn_blocking`; the door's
  own table.
- **A "Not doing" line for `decisions/daemon.md`:** an injected clock
  for reqwest's timeouts: refused; a script answers every connection,
  and a server that never answers is a hang the ceiling catches. Not
  until a scenario needs a silent server.
- **Where `NO_PROBE_ROUTES` is read from the client's harness** (C1:
  the client links the protocol and the store, never the daemon): the
  protocol crate carries the list as data, or the client harness keeps
  its copy and a check compares the two. The smaller is the check; the
  build brief for round 5 brings the two shapes if the owner does not
  rule here.

## 6. Raised for review

- The judge counted as blocking work changes nothing the idle verdict
  reads (it is not a landing), but a scenario that holds `shared` on
  another thread while the judge waits never advances the clock: the
  blocked-judge test stays multi-thread and off the door, as it is.
- `handle_request` as the door's submit path takes the refusals'
  words through the protocol, which the frontends' goldens hold; a
  row asserting a refusal asserts its kind, not its sentence.
- The scenario count's fall (215 toward about 120, `TEST-SHAPE.md`
  §3) is an estimate the rounds measure; no round is held to it.
- The weekly `cargo mutants` artifact is the baseline the close
  compares with; if the workflow's artifact has expired, round 6's
  before-run is the baseline and takes hours of a machine the owner
  names.
