# 68 — the login callback trusts one connection and misreads a denial

## Goal

Today the daemon's login waits for the browser's redirect in
`wait_callback` (`crates/acquisition-daemon/src/daemon/login.rs`, lines
766 to 799): it accepts one connection at a time, reads each request
with no bound, ends the flow on the first request to the callback path
whatever it carries, and answers every request without a matching
`code` and `state` as "state mismatch", a denial among them. The CLI,
meanwhile, gives up 300 s after printing the URL
(`crates/acquisition-cli/src/main.rs`, lines 2105 and 2120 to 2121)
while the daemon allows 300 s for the callback, then the exchange, then
120 s for the profile (`daemon.rs`, `LOGIN_PROFILE_TIMEOUT`, line 293):
a late callback prints "login did not complete within 5 minutes" as the
daemon registers the session. When this step is done, each callback
connection is read under a bound and in its own task, so a connection
that sends nothing holds nothing up; a redirect carrying `error=` with
the flow's state ends the flow in the browser's words; a request with
another state is answered and ignored; the daemon's wait for the browser
is one bound with one knob, and the CLI waits on the daemon's flow
alone, so it never gives up while the daemon still works. Nothing sends
to GGG and nothing changes what the daemon sends: the authorize
navigation is the browser's, the exchange and the profile fetch are
untouched. The issue is #68; the commits say "closes #68" only on the
last.

## Read

- Issue #68: the four faults and the done-when. Its line numbers are
  of 2026-10-26; the ones here are of this tree.
- `AGENTS.md`: invariant 1 (the loopback callback is outside the
  choke point: it is a listener, never a send), C122 (one pin, at the
  lowest crate), C116 (no id in user-facing text), C47 (malformed
  external input is a structured error), "The gate decides".
- `decisions/daemon.md`: C137, a deadline's contract is its behaviour
  at expiry, pinned by forcing the expiry; a deadline widens through a
  named knob, the knob at zero forces the expiry. C133, a pending login
  holds the idle verdict and the exit's drain. C148, the login's
  landing. C50, a login whose profile fetch fails fails whole (the
  profile step this brief leaves alone).
- `decisions/network.md`: C130, the loopback callback is outside the
  one-client boundary; N33 in `docs/design/network-ground-truth.md`,
  the authorize page is browser-only, so the daemon only ever listens
  for its redirect; nothing there records the shape of a denial
  redirect (whether GGG echoes `state` on `error=access_denied`), see
  "Input".
- `crates/acquisition-daemon/src/daemon/login.rs`: `auth_start`
  (lines 19 to 103): the listener bound, the state minted, the spawned
  task with the 300 s `tokio::time::timeout` around `wait_callback`
  (line 47) and the three arms after it (the code exchanged, the
  callback's error, the timeout's words "auth flow timed out after
  300s", lines 83 to 93); `finish_auth_flow_error` (line 462), the one
  way a flow before its exchange ends failed; `wait_callback` (lines
  766 to 799), the fault. `start_auth_flow` (line 106): a newer start
  supersedes a flow waiting on the browser, so a flow's listener task
  may outlive its generation; its result is then ignored by the
  generation check the arms already make.
- `crates/acquisition-daemon/src/mockggg.rs`: `read_request` (lines 964
  to 1017), the HTTP read the callback shares with the mock, unbounded
  by design of a test stand-in; `respond` (line 1019). The bound goes
  around the call in the callback's task, not into the mock. The mock's
  `/authorize` and `/approve` (lines 633 to 700) redirect with `code`
  and `state`; the mock has no denial path, and this step adds none:
  the test drives the callback directly, as the daemon's login tests
  do.
- `crates/acquisition-daemon/src/daemon.rs`: `LOGIN_PROFILE_TIMEOUT`
  (line 293), the profile step's bound, untouched; the `Daemon` struct
  (line 907), where the browser bound becomes a field;
  `daemon/start.rs`, `idle_shutdown_from_env` (line 16), the one shape
  a daemon knob is read in, and the `Daemon` literal at line 488.
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`:
  `test_daemon_scenario_at` (line 514), the test's `Daemon` literal;
  `test_daemon` (line 398), `TestPath`, `finish_harness`.
- `crates/acquisition-daemon/src/daemon/tests/login.rs`: the six tests
  that drive the callback with `reqwest::get` on the authorize URL's
  `redirect_uri` and `state` (five there, lines 335 to 612; one in
  `exit.rs`, line 537), the pattern the new pins follow;
  `login_result` (line 59). The pins land in this file's callback
  section, after `a_superseded_login_flow_sends_no_code_exchange`.
- `crates/acquisition-cli/src/main.rs`: `login` (lines 2084 to 2140),
  the poll loop with its own deadline; `print_auth`. No test asserts on
  "did not complete within 5 minutes" (grep of the test trees and the
  references, 2026-10-28: the one occurrence is the source line).
- `crates/acquisition-client/tests/harness/mod.rs`: `isolation` (line
  160), the one list every test process takes; `login` (line 834), the
  scripted login that sets `env` on the `acq auth` that starts the
  daemon; `approve` (line 819). The CLI's `harness::login_with` wraps
  it.
- `README.md`, "Knobs" (line 235): a row per knob the crates read,
  checked by `tools/docs-check.sh` (its check 4).
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world": "A wait with no deadline"; "A fix staging the nearest
  failure, not the named one" (a deadline that waited for a byte and
  then for a line); "Frontends and the surface an agent meets": "A
  refusal that says what is wrong and not what to do", "A sibling
  sentence left standing"; "A claim the code did not make".

## Input

No measurement. The failing inputs the tests stage, each against the
daemon's own listener in a test:
- a TCP connection to the callback port that sends nothing, then the
  browser's redirect on a second connection: the login completes;
- a connection whose read bound the test sets to zero: the connection
  is closed without a response and the flow still waits;
- a `GET` on the callback path with the flow's `state` and
  `error=access_denied&error_description=...`: the flow ends failed,
  its words carrying the browser's;
- a `GET` on the callback path with a `code` and another `state`: the
  request is answered, the flow still waits, and the browser's redirect
  that follows completes the login;
- the daemon's browser bound set to zero: the flow ends failed with the
  bound's words, through the one path every flow failure before its
  exchange takes;
- `acq auth --no-browser --json` under `ACQ_LOGIN_WITHIN=0` on the
  daemon it starts, never approved: the CLI exits non-zero saying the
  daemon's words, not a bound of its own.

A fact this step cannot settle: whether GGG's authorize page echoes
`state` on a denial redirect. RFC 6749 §4.1.2.1 requires it when the
request carried one, and the daemon reads a denial only with the flow's
state (a request another process can forge must not end the flow); a
denial without the state is answered and ignored like any other, and the
flow waits to its bound. Raised in the report for the owner's next live
login, never guessed.

## Do

1. **The bounded read, each connection its own task.** `wait_callback`
   takes the read bound as a parameter (`auth_start` passes a constant,
   `CALLBACK_READ_WITHIN`, 10 s) and accepts in a loop that spawns a
   task per connection; the task reads the request under
   `tokio::time::timeout` and drops the connection unanswered when the
   bound expires or the read fails; the accept loop selects between the
   next accept and a channel the tasks report the flow's end on. The
   doc comment states the rule. Pins, in `tests/login.rs`:
   `a_callback_connection_that_sends_nothing_does_not_block_the_login`
   (a silent connection, then the redirect, through `auth_start` and
   the mock's exchange) and
   `a_callback_read_past_its_bound_is_dropped_and_the_flow_waits`
   (`wait_callback` called directly with a zero bound: the silent
   connection reads EOF, the flow has not ended, and a second
   connection's request is accepted and dropped at the same bound).
   *Settled in the round:* the brief said a good redirect on the same
   listener then ends the flow; a zero bound drops every read, so the
   silent-connection test, with an hour's bound, shows the redirect
   answered instead.
2. **A denial in the browser's words.** A callback request with the
   flow's `state` and an `error` query ends the flow failed with the
   words "the login was denied in the browser: <error>: <description>"
   (the description when present), through `finish_auth_flow_error` as
   the callback's other errors are; the browser is answered 200 with a
   page that says the login was denied and to run `acq auth` again to
   retry. Pin: `a_denied_login_is_reported_in_the_browsers_words`:
   `login_error` carries `access_denied` and the description, not
   "state mismatch".
3. **A wrong-state request is answered and ignored.** A callback
   request whose `state` is not the flow's, or that carries neither
   `code` nor `error`, is answered 400 with a page saying this is not
   the login Acquisition is waiting for and to return to the terminal,
   and the flow waits. Pin:
   `a_callback_with_another_state_is_answered_and_the_login_waits`: the
   stray request gets its 400 and `pending` stays true, then the real
   redirect completes the login through the mock.
4. **One browser bound, with its knob.** The daemon's wait for the
   browser is `LOGIN_WITHIN` (300 s) on the `Daemon` as `login_within`,
   read at the start from `ACQ_LOGIN_WITHIN` in the shape of
   `idle_shutdown_from_env`; the test harness sets the default. Its
   expiry's words: "the browser did not complete the login within
   <n> s; run `acq auth` again". The README's knob table gains the row.
   Pin: `an_expired_browser_wait_fails_the_flow_with_its_words`: the
   field at zero, `auth_start`, then `login_error` carries the words
   and `pending` is false. The test harness's isolation list does not
   widen it: 300 s is no starvation margin, and the knob is the
   user's, not a liveness deadline's.
5. **The CLI waits on the daemon's flow.** The CLI's `login` loop drops
   its own 300 s deadline and the "did not complete within 5 minutes"
   line: it polls `auth_status` while `pending` and ends with the
   flow's own result, a transport error, or the user's interrupt. The
   daemon's flow is bounded at each step (the browser wait, the
   exchange's send under the choke, the profile's 120 s), so the CLI's
   wait is. Pin, in the CLI's `tests/error_kind.rs` or a sibling file
   of the one binary: `the_login_ends_with_the_daemons_words_not_a_bound_of_its_own`:
   `acq auth --no-browser` with `ACQ_LOGIN_WITHIN=0` on the command (the
   daemon it starts reads it), never approved; the exit code is
   non-zero, stderr carries "login failed:" and the daemon's words, and
   nothing says "5 minutes". *Settled in the round:* the brief said
   `--json` and stderr together; the CLI's JSON error goes to stdout,
   so the pin runs in text mode, where the failure is on stderr.
6. `cargo build --workspace`, then the gate; one commit per concern
   (1, 2 and 3 may share one when the fix is one function's rewrite,
   named so in the message), each message naming this brief.

## Done when

- A callback connection that sends nothing does not hold the next
  accept: a test stages one and the login completes on the next
  connection; the read bound is a parameter of `wait_callback`, and a
  test with it at zero shows the connection dropped and the flow still
  waiting.
- A callback request with the flow's state and an `error` query ends
  the flow with `login_error` carrying the error and its description,
  and the words "state mismatch" are gone from `login.rs`.
- A callback request with another state is answered 400 and the flow
  waits: a test stages one and completes the login after it.
- The browser wait is one field on the `Daemon` with one knob,
  `ACQ_LOGIN_WITHIN`, a README row, and a test that forces its expiry
  reads the bound's words in `login_error`.
- The CLI's `login` has no deadline of its own: `grep -n '5 minutes'
  crates/acquisition-cli/src/main.rs` finds nothing, and a process test
  under `ACQ_LOGIN_WITHIN=0` reads the daemon's words from `acq auth`.
- Every refusal page and failure sentence the step adds says what to do
  and carries no id.
- Nothing under `auth.rs`, the choke point or the mock's `/token` route
  changes; the diff touches no send.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The profile step's 120 s bound and its words (`LOGIN_PROFILE_TIMEOUT`):
  the issue names it only to size the CLI's wait.
- A bound on the mock's own `read_request` for the mock's server: the
  mock is a test stand-in; the callback's bound wraps the call.
- A denial path in the mock's `/authorize`: the pins drive the callback
  directly, as the daemon's login tests do; a scripted denial for the
  seat is a `trigger:seat` issue if a seat asks.
- A protocol field carrying the daemon's deadline to the CLI: the CLI
  waits on `pending`, which the protocol already carries; a field would
  be a ruling.
- The C137 registry entry's knob list: the owner's; raised in the
  report.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; kill only
the processes you started.

## Report

`TEMPLATE.md`.
