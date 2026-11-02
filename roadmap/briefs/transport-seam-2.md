# transport-seam-2 — the in-process transports and the fixtures (C161, C160 as amended; step 2 of the design's §6)

The first cut, written by the orchestrating session (Opus) on 2026-11-01
from the ruled design (`roadmap/briefs/transport-seam-design.md`, §2, §3,
§5's last item, §6 step 2) and a re-read of the tree at `e1244232`,
every line number below from it. The design's line numbers were
`e812a0d7`'s; step 1 moved the spec (its binds now at other lines), the
harness's component constructors and `auth.rs`'s tests, and this brief
cites them as they stand. C160's first amendment is entered at
`e1244232` (the hook accounts for "a request a test transport holds";
"a `beside` door has no hook and keeps the ceiling" gone), as the owner
ruled ("Accept as drafted", the design's §7). To be reviewed cold before
a worker reads it. Opus builds it (the owner's word of 2026-11-01, in
series). No production send changes: the one production edit is the
mock's split, in mock mode, so the step may run in either window beside
the league (the design's §5). The last push's CI: `Gate (Linux)`
41391299544 and `Gate (Windows)` 41391299506 on `ac4642f5`, both
success; re-read at the step's start.

## Goal

When this step is done, no provider exchange in a door scenario is real
I/O (C161): every fixture the door and the component constructors serve
is a transport in the daemon's test support, in-process, which records
a request in its first poll and holds the scenario's clock from that
poll until the request's body completes or its future is dropped, so a
request a fixture holds is a hold the park hook sees (C160 as amended)
without reading the gate's occupancy. The stateful mock answers through
one function its TCP face and its in-process face share. `beside` takes
the first door's clock and hook, so its waits gain the verdict. The two
`occupancy` exclusions in `.cargo/mutants.toml` retire onto the `c160_`
count pin, which fails at once once the hook no longer reads the count.
The only sockets left in the daemon's tests are the real-wiring tests
of the production transport (the spec's six, the crate's one), the
login's callback listener and its two component tests, and the
browser's one GET to the daemon's callback.

Why it matters: a real socket in a scenario was the one wait the hook
could not see (C161's *Why*). This step removes it, so step 3 can put
the request's deadline on the clock and pin its expiry as a row, and so
the hook's verdict reaches the scenarios where a send never completes.

## Read

- `roadmap/briefs/transport-seam-design.md` §1 ("The test fixtures and
  how each reaches the wire", "What the park hook cannot see"), §2 (the
  test transports, the hold, the stateful mock), §3 (every row from "the
  scripted server" to "`beside`", and "the mutants scope"), §4, §5 (the
  mock's split, the mechanism bet), §6 step 2.
  `roadmap/briefs/transport-seam-1.md` and `transport-seam-1b.md`, Not in
  scope (the builder's transport option is this step's) and the
  `acquisition-transport` crate's `src/lib.rs` (the neutral types:
  `Prepared`, `Answer`, `Transport`, `Pending`; its doc's list of public
  items).
- `decisions/daemon.md` C160 as amended (26), C158 (a scenario crosses
  the frontend boundary only; a component test says its boundary), C157;
  `decisions/network.md` C161 (26), C21, C128. `AGENTS.md` C122, C153,
  C154. `TESTING-NOTES.md` 207–213 (the user-agent held by the mock's
  400 and, since step 1, the spec's table).
- `crates/acquisition-daemon/src/daemon/tests/door.rs`: the module doc
  (its "What stays off the door", and 54–57 on the request timeout,
  which is step 3's); `Server` (72–102: `Script`, `Account`, `Token`,
  `Mock`, `None`) and `Server::own` (103); `Counters` (162–176,
  `held_by_send` at 166, `nothing_parked` at 173) and their print under
  `ACQ_DOOR_COUNTERS` (227, 238–255); the hook, `IdleHook::on_idle`
  (264–313: `quiet` first, the scenario's holds, the gate's occupancy at
  276, the blocking count, `settle`, the clock's advance, the verdict);
  `Door::open` (414), `run` (560), `beside` (604), the kinds' match
  (`Server::None`'s base `http://127.0.0.1:1` at 652), `settle` (810),
  `quiet` (835) and its precondition doc, `browser` (850–885, a reqwest
  client with one redirect, under `hold_clock` at 851), `hold_clock`
  (886), `finish` (1211). `grep -rho "Server::<kind>"` over
  `daemon/tests/`: `Script` 19, `Account` 19, `Token` 4, `Mock` 5, `None`
  14; `Server::own(` called seven times (`dispatch.rs:146`, `bearer.rs`
  157, 350, 546, 653 and 754, `exit.rs:701`) over five binds
  (`dispatch.rs:25`, `bearer.rs` 66, 259 and 604, `exit.rs:702`).
  `hold_clock()` called at `door.rs:851` (`browser`), `exit.rs:736` (the
  callback's hand-rolled GET) and `dispatch.rs:195` (a freeze on
  purpose); `.quiet()` at eight sites; `.beside()` at `dispatch.rs:1129`
  (on the first door's clock) and `dispatch.rs:2303` (on its own
  `ParkingClock::new()`, 2296).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`:
  `ScriptedResponse` (187, its full and raw forms), `Received` (344),
  `scripted_server` (362) and `scripted_server_received` (373, the bind at
  381), `delayed_token_server` (436, bind 445), the component
  constructors `test_daemon` (492, `Boundary` first, then `base`),
  `test_daemon_with` (586), `test_daemon_at` (605, building through
  `Daemon::builder` at 627), `assert_journal_matches_wire` (881), the
  pacing oracle `assert_pacing_follows_responses` (999), the answering
  token server `token_server_answering` (1135, bind 1138), the account
  server `account_server` (1282, bind 1290).
- The component fixtures behind the constructors: `log.rs` 20
  (`token_aware_server`, bind 21), 211 (`token_endpoint_answering`, bind
  216), 365 (`api_answered_by`, bind 371); `sessions.rs` 102
  (`abandon_then_success_token_responses`, bind 105) and 487
  (`authentication_completes_before_the_api_send_takes_a_gate_slot`,
  bind 488). `login.rs` 646 and 684 are the callback listener's own
  tests and stay.
- `crates/acquisition-daemon/src/auth.rs`'s two token tests on
  `ChokePoint::new()` and so on `SystemClock`:
  `oauth_clean_200_waits_for_body_completion_before_recording_success`
  (807, bind 808, the choke point at 827) and
  `code_exchange_and_refresh_serialize_across_token_policy_discovery`
  (845, bind 846, at 891).
- `crates/acquisition-daemon/src/ratelimit/tests/spec.rs`, its thirteen
  binds (`grep -n "TcpListener::bind"`): `serve_one_raw`'s (97, called by
  nine tests), `counting_server`'s (2824, called by the 3xx test), and in
  tests at 2026 (`a_send_parked_in_the_gate_is_refused_after_a_trip`),
  2639 (the C161 table's wire run), 2943 (the 3xx test's foreign
  listener), 3080 (`common_gate_caps_gets_until_response_bodies_complete`),
  3147 and 3162 (`head_waits_exclusively_for_an_incomplete_get_body`),
  3226, 3272 and 3301 (inside the test around `held_get`, 3218), and
  3840–3841 (C128's). The tests that stay TCP for good carry the
  sentence "A real-wiring test of the production transport" (1928, 2589,
  2744, 2865, 3046, 3836).
- `crates/acquisition-daemon/src/mockggg.rs`: `start` (646) and
  `start_with_clock` (657, the listener, the clock, the window divisor),
  `handle` (681–972: the user-agent check 691–706, the routes, the
  browser's two pages), `HttpRequest` and `read_request` (1031, 1041),
  `respond_with` (1101).
- `crates/acquisition-daemon/src/daemon/tests/dispatch.rs`:
  `c160_the_hooks_count_is_zero_at_rest_and_one_under_a_held_send`
  (2556, its assertions on the gate's and the choke point's counts at
  rest, held and landed); the wire-table rows that carry reqwest's body
  failure words, "error decoding response body", through a raw script
  (2129–2147, 2151–2170).
- `.cargo/mutants.toml` 70–77, the two `occupancy -> (1, _)` exclusions
  and their comment; `tools/daemon-tests-census.py` (at `e1244232`: 197
  tests in 20 files, door 91, boundary 73, sentence 33, neither 0).
- The daemon's builder (`daemon.rs` 1200, its private test-only options
  under `#[cfg_attr(not(test), allow(dead_code))]`, `build` at 1313
  constructing the choke point) and `ChokePoint::with_transport`
  (`ratelimit/choke.rs`, under `#[cfg(test)]`).

## Input

- **The hook's counters before** (the design's §5, the mechanism bet,
  measured before it is asserted): every door scenario run with
  `ACQ_DOOR_COUNTERS=1` at `e1244232`, each scenario's line kept in the
  worker's scratch; the step's first commit's message quotes the totals
  per counter and every scenario with a nonzero `nothing_parked` (#151's
  measure: zero on every passing scenario).
- **The census at `e1244232`**, by `python3 -I tools/daemon-tests-census.py`:
  197 tests, door 91, boundary 73, sentence 33, neither 0.
- **The spec's binds**: thirteen at `e1244232` by the grep in Read; the
  step's spec commit lists every test that reaches one (by the bind's
  enclosing test or a helper's callers), classifies each kept (it carries
  the real-wiring sentence) or moved, and moves none before the list is
  in its message.
- **No failing input to stage up front**: every existing test keeps its
  property; the boundary rows of Do 6 are new and each is seen failing
  first against a deliberately broken hold.

## Do

In this order, one commit each, each gated on its own tree.

1. **The measurement and the test transports.** First the counters
   above, the before. Then `crates/acquisition-daemon/src/tests/transport.rs`
   (test support beside `testing.rs`, `#[cfg(test)]`, declared in
   `lib.rs` as `testing` is), implementing `acquisition_transport::Transport`:
   - **the hold**, one count shared by every transport a door or a
     component daemon serves, taken in a request's first poll (the
     request recorded in that same poll, before any `Pending`) and
     released when the answer's body completes or the future or the body
     is dropped (a guard, so a cancel releases it); a fixture that
     answers its headers and holds its body behind a release keeps the
     hold through the body, as the gate's permit does (C21);
   - **the scripted transport**: the `ScriptedResponse` list in order,
     asserting each request's method and path as the scripted server
     does, recording `Received` (method, path, the bearer read from the
     prepared request's `Authorization` header and stripped of `Bearer `,
     the form's body as the wire carried it, so `Received::grant` reads
     as today), the full form, the raw form (status line, headers and
     `Content-Length` parsed from the bytes; a body shorter than its
     length fails the body future in the transport's own words, a short
     fixed sentence of the test support's, never reqwest's), and a
     silent form (before the headers or after them) that holds nothing
     and never completes, for step 3's rows;
   - **the account transport** and **the two token transports**
     (delayed and answering): the logic of `account_server`,
     `delayed_token_server` and `token_server_answering` behind the
     trait, their `arrived` and `release` barriers kept, a held request
     held by the hold;
   - **the refusing transport**: every send fails as a transport failure
     in the test support's words, nothing recorded;
   - **the own transport**: a closure the scenario builds, its barriers
     its own, taking the hold like the rest.
   Each transport is pinned in this commit by its own component tests
   beside it (the record made in the first poll, the hold taken there
   and released at the body's end and on a drop, the raw form's parse,
   the refusal), so nothing lands without a caller (`clippy -D warnings`
   refuses dead code under `--all-targets`). The TCP fixtures stay in
   this commit, used as today; no test moves. The way in lands with its
   first caller: `Daemon::builder`'s test-only transport option (step
   1's Not in scope) in Do 3, the door's; the component constructors'
   transport in place of a base in Do 5; both through
   `ChokePoint::with_transport`.

2. **The mock's answer.** `mockggg::answer`, the match of `handle`
   (681–972) over a parsed `HttpRequest` giving a status, headers and a
   body; `handle` reduced to read, answer, respond. The user-agent check
   stays in `answer`, so both faces run it. The listener, `start`,
   `start_with_clock` and the window divisor are unchanged: the shipped
   daemon's mock mode, `acq demo` and every client-harness process test
   run the TCP face. The in-process mock transport in the test support:
   the prepared request turned into an `HttpRequest` (method, path and
   query, lowercased headers, the form as the body), `answer` called,
   the answer returned with its body ready. The mock's own unit tests
   and the client harness's process tests are the evidence the TCP face
   answers as before; no golden moves.

3. **The door onto the transports.** The six `Server` kinds build their
   transports (`Script`, `Account`, `Token`, `Mock`, `own`, `None` as the
   refusing one); the door's daemon gets its transport through the
   builder's new test-only option (`Daemon::builder`, `daemon.rs` 1200,
   in the form of its other test-only options); the fixture's record the oracles read
   (`assert_journal_matches_wire` and the rest, selected at `finish`) is
   the transport's, by the same functions. The hook reads the
   transports' hold in place of the gate's occupancy: `held_by_send`
   renamed `held_by_transport` in `Counters` and its printed line, the
   occupancy read at 276 gone. `browser` reads the mock's two pages from
   the in-process mock and makes its one real GET to the daemon's
   callback listener under `hold_clock`, as today. `hold_clock`'s doc
   keeps two reasons (the browser's GET to the callback, `exit.rs:736`'s
   hand-rolled one; a scenario's deliberate freeze, `dispatch.rs:195`)
   and drops "a request on its way to the scenario's own server".
   `quiet`'s precondition keeps its arrival half (a scenario whose
   assertion depends on a request having reached its fixture calls
   `quiet` after that request's barrier, since a park on the way to the
   transport, a pacing sleep or a credential read, still resolves it
   first) and loses its socket half. The two wire-table rows that carry
   reqwest's "error decoding response body" (2129–2170) carry the test
   transport's words instead; reqwest's words for a cut body stay held
   by the spec's kept real-wiring tests
   (`truncated_429_and_500_keep_status_and_record_transfer_evidence`,
   `full_200_with_truncated_body_is_network_after_observing_policy`),
   which the message names. The TCP scripted, account and token servers
   and `Server::own`'s listeners retire here, with their binds: the C154
   lines name each, held by its transport and the oracles over it.

4. **`beside` with the hook.** `beside` takes the first door's clock
   (a `beside` on another clock refused, in words that say so) and
   registers its whole accounting with the running hook before any
   restore wait: its daemon's blocking count, its transports' hold, its
   holds, its `quiet`, `settle` and stuck slots; unregistered at its
   `finish`. `dispatch.rs:2303`'s caller moves onto the first door's
   clock. A `c160_` case: a wait on the `beside` door that can never
   proceed fails at the park by the verdict, not at `HANG`. The module
   doc's and C160's "no hook" sentences are already gone from the
   registry; the doc follows.

5. **The component fixtures and the spec.** `log.rs`'s three and
   `sessions.rs`'s two listeners become transports through the
   component constructors' option; `auth.rs`'s two token tests move to a
   parking clock through `with_transport` (the crate's last
   `ChokePoint::new()` in a test gone; `grep -rn "ChokePoint::new()"
   crates/acquisition-daemon/src --include='*.rs'` then prints no test
   line). The spec's binds, inventoried by test in the message first
   (Input), then moved to in-process transports except the six kept
   real-wiring tests; a moved test keeps its name and its assertions,
   its server's raw bytes kept as the raw form.

6. **The exclusions and the boundary rows.** The two `occupancy -> (1,
   _)` exclusions leave `.cargo/mutants.toml`, each retired onto the
   `c160_` count pin (`retired the exclusion of <pattern>; held by
   c160_the_hooks_count_is_zero_at_rest_and_one_under_a_held_send`,
   whose assertions on the counts at rest fail at once under the mutant
   now that the hook no longer reads it: the worker applies each mutant
   by hand, sees the pin fail rather than the suite stall, undoes it from
   a scratch copy, and says so in the report). The pin's doc says the
   count is `daemon status`'s (`server.rs` 405, 446), not the hook's. The
   boundary rows of the hold, as a `c160_` table through the door, each
   seen failing first against a hold broken by hand (taken late, released
   at the headers, never released on drop): two requests held at once
   (the clock held until both bodies end); a release fired before the
   request's first poll (no hold leaks, nothing waits); a body held
   behind a release after its headers (the clock held and `settle`
   pending until the release); a request whose future a cancel drops
   (its hold gone with it, the scenario proceeds). The silent body's row
   is step 3's.

7. **The counters after, and the words.** The same counters run as
   Input's, after: totals per counter, `held_by_send` gone from the
   output, every passing scenario at zero `nothing_parked`, quoted with
   the before in the message. `TESTING-NOTES.md` 207–213: the mock's 400
   now sees the prepared request in-process on the door's scenarios and
   the wire on the client harness's, and the production transport's
   real-wiring tests hold the wire half. `door.rs`'s module doc: the
   fixtures are transports; "What stays off the door" unchanged unless
   a sentence names a socket the step removed. The census re-run:
   door 91 unless the build finds otherwise, said either way. The last
   commit sets "Current step" to say the build has landed and its
   review is pending.

8. `tools/gate.sh` once per commit on each commit's tree, its exit
   status read on its own line; `tools/arch-check-breakers.sh` once on
   the last tree, its last line in the report; every golden,
   `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and `README.md` byte-identical
   to `e1244232`.

## Done when

Each line from the commits alone, their tree or their messages, except
the last.

- `crates/acquisition-daemon/src/tests/transport.rs` holds the hold and
  the scripted (full, raw, silent), account, two token, refusing, own and
  mock transports; the hold is taken in a request's first poll and
  released by a guard on body completion or drop.
- `grep -rn "TcpListener::bind" crates/acquisition-daemon/src --include='*.rs'`
  prints only: the spec's kept real-wiring tests (each carrying the
  sentence), `login.rs` 646 and 684, the mock's listener in `mockggg.rs`,
  and any listener a kept test needs; the messages carry a C154 line for
  every retired listener, each naming its holder.
- `door.rs`'s hook reads no `actual_send_occupancy`; `Counters` has
  `held_by_transport` and no `held_by_send`; `hold_clock` names two
  reasons and `quiet` one precondition half.
- `mockggg.rs` has `answer`, called by `handle` and by the in-process
  mock; the user-agent check is inside `answer`.
- `beside` takes the first door's clock, refuses another, registers with
  the hook and unregisters at `finish`; no `beside` caller builds its own
  clock; a `c160_` case on a `beside` wait.
- `.cargo/mutants.toml` has no `occupancy` exclusion; two retirement
  lines name the `c160_` pin.
- The boundary rows of Do 6 exist as a `c160_` table; the two
  wire-table rows carry the test transport's words, and the message
  names the spec tests holding reqwest's.
- `auth.rs`'s two token tests build no `ChokePoint::new()`.
- The counters before and after, the spec's inventory, and the census
  after are in the messages; no golden, reference or README line changed.
- The gate's exit status per stage on each commit's tree, the breakers'
  last line, each boundary row and each mutant seen failing, are
  execution evidence in the worker's report; the orchestrating session
  re-runs the gate on the step's final tree.

## Not in scope

- The deadline on the clock, `.timeout()` and `CONNECT_TIMEOUT`
  removed, the expiry rows, the silent body's row, the pacing oracle's
  lost-send case, the door module doc's 54–57, `TESTING-NOTES.md`'s
  "Absence": step 3, after C160's second amendment.
- Any production send: the choke point, the transport crate and the
  production transport are untouched; the mock's split is the one
  production edit, in mock mode, its TCP face answering as before.
- The client harness's process tests and their `acqd` children: TCP,
  the process boundary the retrospective ruled the seam ends at.
- The login's callback listener and its two component tests; the
  browser's GET to it.
- #151's open timeouts: the design claims none (§3); the seam's close
  compares the mutants.
- Moving a component test onto the door: C158 asks for its boundary,
  never a move.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on `main`,
in a checkout other sessions may share; never push; `acq` and `acqd`
only in a mock world (the `mock-session` skill); never a daemon against
the real provider; kill only this session's pids; `cargo build
--workspace` before anything that starts a daemon; the gate's exit
status read on its own line, never through a pipe; long commands in the
foreground where they fit, since an idle session does not wake when a
background process ends; an experiment (a broken hold, a mutant by
hand) undone from a scratch copy and compared with `cmp`. Seven commits,
each gated on its own tree. The step changes non-test code (the mock)
and `.cargo/mutants.toml`, so its commits are reviewed cold as one
range. The renamed counter, the test transports' failure words and the
`beside` refusal's words are the session's agent-facing design,
recorded in their commits and listed at the close. The rules bind the
worker's own work, except Do 1's hold rule, which binds what it builds.

What the brief's writer asks: each boundary row of Do 6 and each mutant
of Do 6 seen failing first, named in the report, not the history.

## Report

`TEMPLATE.md`, and: the gate's statuses per stage for each commit; the
breakers' last line; the counters before and after, with every scenario
whose `nothing_parked` was nonzero before or after; the spec's inventory
by test; the census after; each boundary row and mutant seen failing and
how. Raised for review, at least: any scenario that needed a
`hold_clock` or a `quiet` it did not need before; any fixture whose
logic could not move behind the trait unchanged; any word a moved test
asserted that changed.
