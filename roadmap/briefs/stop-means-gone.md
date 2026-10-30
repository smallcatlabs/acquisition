# stop-means-gone — a stop returns once the daemon's endpoint is gone (#130, #144)

## Goal
Group B of the owner's 2026-10-30 open-issues review ("Closures, then
A and B"). The owner chose #130's option 2. Today `acq daemon stop` can
return on Windows while the daemon's pipe is still listed. A command
run straight after can then reach a dying daemon. On CI, this window
cost re-runs on 2026-10-29 (run 40886043255) and 2026-10-30 (#136,
folded into #130).

#115 hid the window from the tests: the CLI harness's guard looks
again for the endpoint after the stop. The product still has it. When
this step is done, the client's stop waits, within the bound it
already has, until the endpoint is gone after the connection ends. The
harness's second look then retires, held by the product.

Beside it, #144: the client contract test's fake peer treats a
connection reset as a failure. A reset is how a Unix socket reports
the client hanging up while unread bytes remain, so the fake peer must
read it as the hang-up.

## Read
- `crates/acquisition-client/src/client.rs`:
  - `Stopped` (`:573`): its two variants' docs.
  - `Client::stop` and `stop_within` (`:1630` to `:1690`): the seam C137
    asks for is there already (`within`).
  - The unit test that drives `stop_within` (`:2190`, `:2208`).
- `crates/acquisition-client/src/endpoint.rs:66`, `exists`: on Unix the
  socket file; on Windows a name read from the pipe listing, without
  connecting.
- `crates/acquisition-cli/tests/harness/mod.rs:129` to `:175`,
  `StopOnDrop`: its doc and the re-look loop that this step retires.
- `crates/acquisition-cli/tests/provider_default.rs`,
  `a_stopped_daemon_is_gone_when_the_stop_returns`: the end-to-end pin
  (`REVIEW-CHECKLIST.md`, "A stop that returned early").
- `decisions/daemon.md`:
  - C133, the exit sequence: drain, checkpoint, remove the endpoint,
    then exit. This step changes nothing in the daemon.
  - C137: the deadline's contract is its expiry behaviour, pinned by
    forcing it.
- Issue #130 with its comment (the two options, the folded #136), and
  issue #144.
- `crates/acquisition-client/tests/contract.rs:523`, `read_to_eof`.

## Input
- **CI at `3c3cabad`**: `Gate (Linux)` run 41140424622 success,
  `Gate (Windows)` run 41140424614 success.
- **The evidence**:
  - Windows run 40886043255: two `error_kind` logout stops returned in
    2.36 s and 4.11 s with the pipe still listed.
  - Run 40986850126 at `87561f01`: `a_stopped_daemon_is_gone_when_the_stop_returns`
    failed in 3.7 s.
  - Run 41129974645 at `61ae2496` (Linux): `contract.rs:532` panicked on
    `ConnectionReset`.
- **The owner's choice**: option 2. The daemon's exit sequence (C133)
  is unchanged.
- **No fixed failing input on this Mac.** Unix removes the socket file
  before the exit, so the window appears only on Windows. The new pin
  must create the window itself, not wait for it.

## Do
1. **The product, one commit.** In `stop_within`, once the connection
   has ended, wait for `endpoint::exists(<the endpoint this client
   connected to>)` to be false, inside the same `within` bound and
   counting from the same start. Only then return `Stopped::Gone`. An
   endpoint still present when the bound expires gives
   `Stopped::Acknowledged`, as a read that times out does today. Update
   `Stopped::Gone`'s doc and the comment in `stop_within` to say what is
   now checked.
   The pin is a client unit test against a fake peer that:
   - answers `stopping`;
   - ends the connection;
   - leaves its endpoint in place until the test removes it.

   The test asserts that the stop has not returned while the endpoint
   is there, then removes it and gets `Gone`. A second case keeps the
   endpoint, with the bound forced to expire, and gets `Acknowledged`.
   Force the interleaving (paused tokio time, or a stop future polled
   to pending before the removal); never a sleep that proves a wait
   (C137). The poll interval is the product's own choice; write it into
   the doc.
2. **The harness, one commit.** `StopOnDrop` looks once after `acq
   daemon stop`. Its loop and the paragraph explaining Windows' late
   pipe go. The commit message carries `retired StopOnDrop's re-look;
   held by <the step-1 pin> and a_stopped_daemon_is_gone_when_the_stop_returns`
   (C154, the retirement rule). If the guard's doc keeps a reason the
   product now holds, it cites the product.
3. **#144, one commit.** `read_to_eof` takes `ConnectionReset` (and
   `ConnectionAborted`, which is its Windows twin) as the client's
   hang-up, beside `Frame::Closed`. Its comment says why: a reset is
   how a Unix socket reports the client closing with unread bytes
   behind it. Whether the client should drain before closing is the
   daemon area's question, and stays out of this step.
4. `tools/gate.sh` once on the final tree; the "built" commit waits on
   its exit status.

## Done when
- `stop_within` returns `Stopped::Gone` only when the connection has
  ended and `endpoint::exists` is false for its endpoint. The stop
  waits at most `within`, and an end it sees at the poll the bound
  expires is reported as Gone. The step-1 pin forces both outcomes and
  the tie, and holds no wall-clock bound. Settled in the round (the
  review of this step, the first finding): the brief first said "both
  inside `within`", which reads a tie as Acknowledged, a report that
  the daemon may still be running when it was seen gone.
- `StopOnDrop` has no loop, and the retirement line is in the
  harness commit's message.
- `read_to_eof` accepts a reset as the hang-up.
- Nothing under `crates/acquisition-daemon/` changes.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- Option 1, the daemon dropping its pending pipe instance: a change to
  C133's exit sequence.
- Draining before close, in the client or the daemon (#144's second
  half).
- The time seam's lifecycle machines (`TIME-SEAM.md` §4). This wait
  lives under the existing seam and is the kind of transition the seam
  will later model.
- #131's daemon-test waits: they are t5's (#141).

## Rules
`TEMPLATE.md`. What differs:
- You work in the main checkout (`/Users/tom/Development/GitHub/smallcatlabs/acquisition`),
  with your scratch under
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/70b56735-1662-47f4-9f40-9a9bbfd8aaf9/scratchpad/b/`.
  The step runs after map-before-seam, with no other build running.
  Commit; never push.
- The Windows-only path (`exists` reading the pipe listing) is linted by
  the gate's Windows-target clippy stage and run only in CI. Say so in
  the report; don't claim a Windows result.
- Step 1 changes product code, so the orchestrating session reviews the
  step.

## Report
`TEMPLATE.md`.
