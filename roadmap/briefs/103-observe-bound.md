# 103 — a bound on every observe request

## Goal

Today a client's request to the daemon waits without bound once the
handshake has answered: `Client::request`
(`crates/acquisition-client/src/client.rs`, line 1635) writes a frame
and reads the next one with no deadline, so `acq daemon status`, `acq
jobs`, `acq dash` and every other reader waits forever on a peer that
answers `hello` and never its status. The handshake itself is bounded
(C86, the handshake deadline, 5 s through `ACQ_HELLO_WITHIN`), and one
reader, `acq diagnose`, bounds its own status request by that deadline
in the verbs crate (`daemon.rs`, `daemon_within`, line 144; r4's review,
finding 3). When this step is done, every observe request a client
sends is bounded in one place, the client crate's `request`, by the
handshake's deadline and its knob; its expiry is a refusal that names
the peer (its socket and pid) and what to do, through the one shape
every frontend prints; and the verbs crate's own bound is gone, since
the layer below holds it (C122, one pin at the lowest crate; C46,
shared semantics in Rust). Nothing sends to GGG and nothing changes
what the daemon does. The issue is #103; the commits say "closes #103"
only on the last.

## Read

- Issue #103: the fault and the done-when ("every observe request a
  client sends has a bound, reached as a refusal that names the peer
  and what to do, pinned by forcing the expiry").
- `AGENTS.md`: C46 (shared semantics live in Rust: the bound is the
  client crate's, inherited by every frontend), C122 (pinned once at
  the lowest crate; a frontend pins what its adapter adds), C116 (no
  id in user-facing text), C47 (a structured error with a stable
  kind), "The gate decides".
- `decisions/daemon.md`: C86 (the handshake has a deadline: a peer
  that accepts and does not answer `hello` within 5 s is unresponsive,
  its own verdict on every door; this step leaves the verdict and the
  doors as they are and adds no new door state), C137 (a deadline's
  contract is its behaviour at expiry, pinned by forcing the expiry;
  the knob at zero forces it; the knob list names `ACQ_HELLO_WITHIN`),
  C85 (a connection carries one request in flight: a request whose
  answer never came leaves the connection misaligned, so it is not
  reused), C3 (the idle default; `daemon status` exists for
  debugging), C10 (an observation never spawns or replaces).
- `decisions/frontends.md`: C11 and C53 (`--json` everywhere, text one
  screen; the refusal is the one shape, `acquisition_verbs::refusal`).
- `crates/acquisition-protocol/src/protocol.rs`, `Request` (line 437):
  the fifteen variants, the source of what a client can send:
  ```
  sed -n '/^pub enum Request {/,/^}/p' crates/acquisition-protocol/src/protocol.rs \
    | grep -E '^    [A-Z][A-Za-z]*( \{|,)' | sed 's/[ {,]*$//' | tr -d ' '
  ```
  prints `Submit Status Result Cancel SetPriority List Subscribe
  AuthStart AuthStatus AuthCheck AuthLogout Quote DaemonStatus
  ResetTripwire Dashboard` (15). The daemon's dispatch
  (`crates/acquisition-daemon/src/daemon/server.rs`, `handle_request`,
  line 250) shows which answer from memory and which wait: `Status`,
  `Result`, `List`, `Subscribe`, `AuthStatus`, `DaemonStatus` and
  `Dashboard` answer under the shared lock (a job evicted from memory
  is read from the table, C6); `Quote` waits only for the start's
  restore (C148) and then answers from the limiter's state; `Submit`,
  `AuthStart`, `AuthLogout` wait for the restore and then act (a store
  write, a listener bind, the keyring); `AuthCheck` refreshes through
  GGG under the gate (invariant 1: a limiter wait up to 300 s is
  legitimate); `Cancel`, `SetPriority`, `ResetTripwire` act on memory
  or the rails file.
- `crates/acquisition-client/src/client.rs`: `HELLO_WITHIN` (line
  339), `hello_within` (line 347, read at each handshake),
  `parse_within` (line 364, the one rule of a deadline knob),
  `Unresponsive` and its `Display` (lines 386 and 415: the sentence
  that names the socket, the deadline and the kill line, the shape
  the new refusal follows), `Client` (line 959), `handshake` (line
  1535: `timeout_at` with the in-time check, "a reply is in time only
  if it arrives before the deadline", the mechanism the request bound
  reuses), `exchange` (lines 1613 and 1681), `request` (line 1635),
  `status` and `quote` (the wrappers; every site below reaches
  `request` through one of them, `expect_ack`, or directly).
- Every site a client sends a request from, found by
  ```
  grep -rn '\.request(&Request::\|expect_ack(&Request::\|client\.status(\|\.quote(jobs' \
    crates/acquisition-verbs/src crates/acquisition-cli/src crates/acquisition-mcp/src \
    crates/acquisition-client/src --include='*.rs'
  ```
  (27 lines on this tree), plus the two the pattern misses, the MCP's
  `ask` (`crates/acquisition-mcp/src/main.rs`, line 555, every MCP
  request) and the submit (`crates/acquisition-verbs/src/jobs.rs`,
  line 65). Every one of them calls `Client::request`, so one bound
  there is a bound on every site, present and future. The observe
  sites by reader: `daemon status` and `diagnose` (verbs `daemon.rs`,
  line 174); `jobs` and the watch's snapshot (`acquisition-cli/src/
  main.rs`, line 2407; verbs `refresh.rs`, line 621); `job`, `result`,
  the apply's progress and the failed children (verbs `jobs.rs`,
  lines 81, 470, 488, 506, 537; CLI lines 1841, 2464, 2690); `dash`
  (`dash.rs`, lines 69 and 131); the quote (verbs `refresh.rs`, line
  278); `auth status` and the login's poll (CLI lines 1371, 2030,
  2114); the subscription's turn (client line 1599). The MCP's
  `daemon_status`, `jobs` and `job` tools go through the verbs doors
  (`main.rs`, lines 2009, 2016, 2044); the web's page through
  `frontend.jobs` and `frontend.daemon` (`web/mod.rs`, lines 799 and
  815).
- `crates/acquisition-verbs/src/daemon.rs`: `daemon` (line 135) and
  `daemon_within` (line 144), the per-verb bound this step retires,
  with the sentence "answered the handshake and then not its status
  within … (the handshake's bound, ACQ_HELLO_WITHIN); the sections
  below are read from disk"; `diagnose.rs`, line 176, its one caller.
- `crates/acquisition-verbs/src/fault.rs`: `from_client` (line 164),
  a client error that is not a daemon refusal is `Fault::Transport`,
  printed `{e:#}` by every frontend through the one refusal shape.
- `crates/acquisition-verbs/src/refresh.rs`: `QUOTE_TIMEOUT` (line
  30) and `quote_within` (line 247): the planner's own bound around
  the whole quote attempt, door and request, whose expiry is a note on
  an offline plan, not a refusal. It stays: it bounds the door too, and
  its contract (never fail; `QuoteNote::Timeout`) is the plan's.
- `roadmap/briefs/r4-diagnose.md` and `roadmap/reviews/r4-*.md`,
  finding 3: how the diagnosis came to bound its own request.
- `README.md`, "Knobs" (line 247): the `ACQ_HELLO_WITHIN` row, whose
  words widen to the requests it now bounds; `tools/docs-check.sh`
  checks the row exists.
- Tests this step touches or relies on, found by grepping the test
  trees for `hello`, `handshake` and `deadline`:
  `crates/acquisition-client/tests/contract.rs`: `foreign_daemon`
  (line 253, a scripted peer that answers `hello` from a value and
  every other request with an error frame), `silent_peer` (line 291),
  `c86_a_peer_that_never_answers_hello_is_unresponsive_on_every_door_and_never_replaced`
  (line 315), `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers`
  (line 396: the knob set to zero after a peer is up, then to 60 for
  the same peer, the pattern the new pin follows);
  `crates/acquisition-verbs/tests/daemon.rs`:
  `a_peer_that_answers_hello_and_not_its_status_leaves_the_rest_to_the_disk`
  (line 962: a real daemon's `hello` captured and replayed by a
  listener that holds the status, under `ACQ_HELLO_WITHIN=1`; asserts
  the words "answered the handshake and then not its status", which
  the new refusal keeps);
  `crates/acquisition-client/tests/harness/mod.rs`, `isolation` (line
  176, the knob at 60 for every test process), `session` and
  `set_knob` (lines 882 and 926), `start_daemon_in_process` (line
  941), `acqd_built` (line 223). No test asserts on `daemon_within`
  or on "the sections below are read from disk" (grep of the test
  trees, 2026-10-28: the one occurrence of each is its source line).
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world": "A door that fails open on what it could not tell"
  ("every probe bounded"); "Frontends and the surface an agent meets":
  "A fix staging the nearest failure, not the named one" (a deadline
  that waited for a byte and then for a line: the bound here is the
  whole exchange, write and read), "A behaviour pinned at three
  layers", "A refusal that says what is wrong and not what to do", "A
  sibling sentence left standing", "A wait with no deadline".

## Input

No measurement. The failing inputs the tests stage:
- a scripted peer on the session's socket that answers `hello` as this
  build's daemon (pid 4242) and every other request with an error
  frame at once, observed under the knob at 60, then asked its status
  under the knob at zero: the request is refused as the bound's expiry
  whatever the peer answered, the refusal naming the socket, pid 4242,
  zero seconds and the kill line; a second request on the same client
  is refused the same way without a frame sent; the same peer asked to
  cancel a job under the knob at zero gets the peer's error frame, not
  the bound (an acting request is not bounded here);
- a real daemon's `hello` replayed by a listener that reads the status
  request and never answers, under `ACQ_HELLO_WITHIN=1`: the
  diagnosis's daemon section carries the refusal and the rest is read
  from disk (the existing verbs pin, unchanged), and `acq daemon
  status` as a process exits non-zero with the refusal on stderr
  naming the daemon's pid and the kill line.

## Do

1. **The class of a request, in the client crate.** A private function
   over `Request` with an exhaustive match, so a new variant must be
   classed before the crate builds: an observe request, answered from
   the daemon's memory and bounded here (`Status`, `Result`, `List`,
   `Subscribe`, `AuthStatus`, `Quote`, `DaemonStatus`, `Dashboard`),
   or an acting request, not bounded here (`Submit`, `Cancel`,
   `SetPriority`, `AuthStart`, `AuthCheck`, `AuthLogout`,
   `ResetTripwire`; where each one's wait is bounded is "Not in
   scope"). The match names, for each observe request, what it asked
   in words for the refusal ("its status", "the job list", "the job's
   status", "the job's result", "the subscription", "the login's
   status", "the quote", "the dashboard"). This is not a ruling: C86's
   verdict and doors are untouched, C137's knob list gains nothing (the
   handshake's knob is reused: the daemon answers each of these from
   memory as it answers `hello`, so one deadline fits both, and a
   second knob would be a second number for one property), and the
   protocol gains no field. It is agent-facing design, recorded in the
   commit and listed at the close.
2. **The bound, in `Client::request`.** An observe request runs its
   exchange under `tokio::time::timeout_at` with the handshake's
   in-time check (a reply is in time only if it arrives before the
   deadline, so a deadline of zero expires whatever the peer does),
   the deadline `hello_within()` read at the request as the handshake
   reads it. Its expiry is a typed error, `Stalled` (the socket, the
   pid the handshake reported, what was asked, the effective
   deadline), with a `Display` in the shape of `Unresponsive`'s: "the
   daemon at <socket> (pid <n>) answered the handshake and then not
   <asked> within <s>s; stop it by hand (`kill <n>`) and run the
   command again" (`taskkill /F /PID` on Windows, as `KILL` already
   says), and a `find` on an `anyhow::Error` as `DaemonError` has. No
   Keychain sentence: none of these requests touches the keyring. The
   expiry poisons the connection (C85: an answer may still arrive, and
   the next frame read would be it): the client keeps the `Stalled`
   and every later request on it is refused with the same words
   without a frame sent. The doc comments state the rule under C86 and
   C137 as the handshake's do. Pin, in `contract.rs` after the
   handshake's zero-deadline test:
   `c137_an_observe_request_under_the_deadline_at_zero_is_refused_naming_the_peer_and_an_acting_one_is_not`:
   `foreign_daemon` with pid 4242, observed under the knob at 60;
   the knob set to zero; `request(DaemonStatus)` is `Err` whose
   `Stalled::find` gives `within` zero, the session's socket and pid
   4242, and whose words carry "answered the handshake and then not
   its status within 0s" and the kill line; a second
   `request(DaemonStatus)` on the same client is the same refusal; a
   fresh observation under the knob at 60 and `request(Cancel)` under
   the knob at zero returns the peer's error frame as a `Response`.
3. **The verbs crate stops bounding for itself.** `daemon_within` and
   its `Option<Duration>` go; `daemon` calls `request` and maps its
   error as every other door does (`Fault::from_client`, so the
   refusal reaches every frontend as `Fault::Transport` in the client's
   words); the diagnosis calls `daemon` and keeps its own clause, "the
   sections below are read from disk", appended in `diagnose.rs` to
   whatever the daemon section's fault says. The verbs pin at line 962
   stands as written, its doc comment corrected to say the bound is the
   client's.
4. **The adapter's pin, in the CLI** (C122: the exit code and the
   stream are the adapter's), in `tests/daemon_observe.rs`:
   `a_daemon_that_answers_hello_and_not_its_status_is_refused_naming_its_pid`,
   a `#[tokio::test(flavor = "multi_thread")]` in the one binary: the
   verbs pin's staging (a real daemon started in-process for its
   `hello`, stopped, the reply replayed by a listener that holds the
   status), then `acq daemon status` run as a process with
   `ACQ_HELLO_WITHIN=1` on its command, from a blocking task; the exit
   code is non-zero, stderr carries "answered the handshake and then
   not its status within 1s" and the kill line with the replayed pid,
   and no id (the no-id sweep covers the words through
   `no_ids.rs`'s rule; the test asserts the sentence). The MCP adds
   nothing of its own: its `daemon_status` tool prints the same
   `Fault` through the one refusal function, pinned by the verbs
   crate's `every_refusal_is_one_shape_from_one_function`.
5. **The knob's row.** `README.md`, "Knobs": the `ACQ_HELLO_WITHIN`
   row says "how long a daemon has to answer the handshake, and each
   read request after it, before it is reported unresponsive".
6. `cargo build --workspace`, then the gate; one commit per concern
   (1 and 2 share one when the class and the bound are one function's
   change, named so in the message; 3 its own; 4 rides with 2 or 3 as
   the pin of the behaviour the CLI shows; 5 with 2), each message
   naming this brief.

## Done when

- `Client::request` bounds every observe request by `hello_within()`,
  read at the request, with the handshake's in-time check; the class
  is one exhaustive match over `Request` in the client crate, and
  `grep -n 'daemon_within' crates/` finds nothing.
- The expiry is a typed `Stalled` whose words name the socket, the
  pid, what was asked, the deadline and the kill line, with no id; a
  client whose request expired refuses every later request without a
  frame sent.
- A test in `contract.rs` forces the expiry at zero against a peer
  that answers, reads the words and the fields, and shows an acting
  request unbounded under the same knob.
- The verbs pin at `tests/daemon.rs` line 962 is green unchanged in
  its assertions; a CLI process test reads the refusal and a non-zero
  exit from `acq daemon status` against a peer that holds the status.
- The README's knob row names the requests the knob now bounds.
- The diff touches no daemon source and no send: `git diff --stat`
  of the step's range lists nothing under `crates/acquisition-daemon/`.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The acting requests' waits: `Submit`'s is the job's (`wait` in
  verbs `jobs.rs`, the apply loop's own deadline), `AuthStart`'s and
  `AuthLogout`'s are the keyring's (C148, the standing concern of the
  keyring's reach, raised at #68's close), `AuthCheck`'s is the
  limiter's (invariant 1: the wait is the point), `Cancel`,
  `SetPriority` and `ResetTripwire` are the acting observe-door verbs
  whose bound is a `trigger:user` issue if a user meets a stall there.
  Bounding them here would cut a legitimate wait with a wrong refusal.
- `Subscription::next`: an event stream waits by design (C85); its
  recovery is the subscriber's (`watch_recovery.rs`).
- The `daemon_stop` exchange on the bootstrap plane: `stop_any`'s
  request is unbounded after its handshake; a stop that hangs is a
  `trigger:user` issue, raised in the report.
- The planner's `QUOTE_TIMEOUT`: a bound on the whole attempt with a
  note as its expiry, the plan's contract; it stays beside the
  client's.
- A protocol field or a second knob: neither is needed (Do 1), and
  either would be a ruling.
- C86's and C137's registry text: the owner's; the report says what
  the close would have them record, if anything.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; scratch
under this session's scratchpad in a subdirectory named `103`; kill
only the processes you started.

## Report

`TEMPLATE.md`.
