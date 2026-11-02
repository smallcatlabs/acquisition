# transport-seam-1 — the seam, no behaviour change (C161, step 1 of the design's §6)

The second cut: the first (`dad73240`) was reviewed cold
(`roadmap/reviews/transport-seam-1-brief-dad73240.md`, the prompt
beside: five findings), answered as listed at the foot under "Taken from
the reviews". Written by the orchestrating session (Opus) on 2026-11-01
from the ruled design (`roadmap/briefs/transport-seam-design.md`, §6
step 1; §7 ruled that day, "Accept as drafted", C161 entered at
`10d1dfc3`) and a re-read of the tree at `10d1dfc3`, every line number
below from it. The design's line numbers were `e812a0d7`'s; no Rust
source, manifest or tool has changed since (`git diff --stat e812a0d7
10d1dfc3 -- crates tools Cargo.toml Cargo.lock` prints nothing; only
`.cargo/mutants.toml`, one exclusion's line, and the two decision
files moved). To be reviewed cold before a worker reads it. No ruling
is owed before this step: C161 is entered and the "Not doing" line on a
request timeout's expiry is gone (`decisions/daemon.md`, its "Not
doing" list, at `10d1dfc3`); C160's two amendments wait for steps 2 and
3. Opus builds it (the owner's word of 2026-11-01: Opus 5.5, in
series). It touches the send path the owner's league run goes through,
so it runs in the window he names (the design's §5, first risk).

Dated 2026-11-01, after the build: Do 2's module-tree privacy of the
production transport and Do 4's check 13 were settled by the round
`briefs/transport-seam-1b.md` (#160, the owner's "(b) Own crate"): the
transport is its own crate, `crates/acquisition-transport`, check 13 is
retired onto check 5's edges, and the production transport's one caller
and its crate's export of no reqwest item are held by review.

## Goal

When this step is done, every request the daemon sends to the provider
is a value the choke point prepares (its method, its URL, the
registration's user-agent, the bearer as a sensitive header, the form
of a POST) and hands to a transport, and the one transport in
production wraps the one reqwest client, privately, in
`crates/acquisition-daemon/src/ratelimit/transport.rs`; the word
`reqwest` stands in that one production file, and a new check 13 of
`tools/arch-check.sh` refuses it anywhere else in the daemon's
production sources, so a second sender is a check failure rather than
a review finding. Nothing a user, a journal line, a frontend or a test
meets changes: the wire carries the same request (its method, path,
headers and body; reqwest may order the headers otherwise, since the
user-agent is now a request header beside the client's default, which
a request header replaces), the journal the same
fields and words, the limiter the same observations in the same order,
the gate the same permit over the same span. A real-wiring test holds
that what the choke point prepares is what reaches a listener, and the
seam is shown to take another transport by one in-process run of the
same table.

Why it matters: this draws C161's boundary (the design's §2), the line
steps 2 and 3 build on. Step 2 puts the door's fixtures behind
in-process transports, so the park hook sees a held request instead of
a socket; step 3 moves the request's deadline onto the daemon's clock.
Neither can start until the choke point speaks to a transport and not
to reqwest.

## Read

- `roadmap/briefs/transport-seam-design.md` whole; §2 (the prepared
  request, the transport, the production transport, the user-agent's
  two readers), §3's rows for the four send sites, the one client, the
  user-agent, the bearer and the re-exported types, §4 ("what must not
  change"), §5 (the first two risks), §6 step 1, §7 (ruled).
- `decisions/network.md`: C161 (line 26, entered; its *Pinned* names
  check 13 "when built", which this step builds), C21 (11: the permit
  spans each request from just before dispatch until its body
  completes), C128 (20: a bearer only to the API origin, judged before
  any wait or permit), C130 (22: no redirect followed), C150 (24: the
  proxy variables route the one client). `AGENTS.md`: invariant 1 (one
  choke point, one gate), 4 (the user-agent), 5 (refresh tokens never
  in plaintext on disk); C1, C154 (retirement lines), C153 (a finding
  joins an existing surface first). `decisions/daemon.md`: C84 (the
  artifact mismatch; the `features` stage).
- `crates/acquisition-daemon/src/ratelimit/choke.rs`:
  `CompletedResponse` (108–120, `status: reqwest::StatusCode` at 109);
  `classify_response` (155–180, D8's precedence); `ChokePoint` (197–211,
  the private `http: reqwest::Client` at 201, its comment 198–200);
  `CONNECT_TIMEOUT` and `REQUEST_TIMEOUT` (216, 217, kept by this step);
  `build_client` (230–244: the user-agent on the client at 238, no
  redirect at 239, the two timeouts at 240–241); `ChokePoint::new` (257),
  `with_rails` (261), `with_clock_and_rails` (275–289, `pub(crate)`, the
  daemon's and the spec's); the four send sites, `head` (527–731: C128's
  check 535–542 before `acquire_head`, the token read after admission
  556–562, `wait` measured at 563, the reqwest request built and sent
  564–568, the headers read 569–590 before the body is drained 594–601,
  the classification and the journal after), `post_form` (988–1007, the
  send 998–1004), `get_bearer` (1013–1044, C128 at 1021 before
  `acquire_send`, the token read 1023, `authorize` and the send
  1027–1031, the 401 published under the permit 1037–1042) and `get`
  (1048–1060, the send 1057); `observe` (752–783, the headers observed
  and the gate rekeyed before the body, taking `&Result<reqwest::Response,
  String>`); `finish_send` (793–923: the lost send noted at 807–815 for a
  counted send with no answer, the body read at 840 under the permit,
  the redirect evidence 837–839 replacing a 3xx's body, the
  classification at 848, the journal at 862); `record_completed`
  (928–982); `verdict_of` (1066–1072). `grep -c reqwest
  crates/acquisition-daemon/src/ratelimit/choke.rs`: 12 lines.
- `crates/acquisition-daemon/src/auth.rs`: `Secret` (52), `expose`
  (57, the text read in this file alone), `authorize` (61–65, reqwest's
  `bearer_auth`, which marks the header sensitive); the token request,
  `exchange_code` (198) and `refresh` (227) through `token_request`
  (246, its `post_form` at 255), whose form carries the code, the
  verifier or the refresh token as `&str` already exposed. The
  credential discipline (#113, a log safe by construction): `Secret`'s
  doc (38–49: built and read in this module alone, a placeholder under
  `Debug` and `Display`), its `Debug` (87–91), and the token response's
  `Debug` naming neither token (165–179).
- `crates/acquisition-daemon/src/ratelimit/policy.rs`: the five
  functions taking `&reqwest::header::HeaderMap` (`Policy::from_headers`
  111, `retry_after_from_headers` 338, `response_headers_snapshot` 427,
  `redirect_evidence` 523 with `reqwest::header::LOCATION` at 524,
  `rate_limit_snapshot` 590). `crates/acquisition-daemon/src/daemon/kinds.rs`:
  `reqwest::StatusCode` at 1014 (`cloudflare_failure`) and 1044
  (`classify_api_body`); the one caller of `get`, 817.
  `crates/acquisition-daemon/src/ratelimit.rs`: the module doc's
  sentence on "the workspace's only `reqwest::Client`" (12–14), the
  module list (41–48: `mod choke;`, `mod limiter;`, `mod policy;`, the
  spec under `#[cfg(test)]`). `crates/acquisition-daemon/src/provider.rs:14`,
  `USER_AGENT`, the one constant.
- Those types are `http`'s re-exported: `http` 1.5.0 is in `Cargo.lock`
  (1472) as reqwest's dependency, no direct dependency of any crate, and
  already built with its `default` feature (`cargo tree -p
  acquisition-daemon -e features -i http`); reqwest is the daemon's
  alone (`grep -n reqwest crates/*/Cargo.toml`: the daemon's
  `Cargo.toml:35`, `default-features = false`, `json` and `rustls-tls`).
- `crates/acquisition-daemon/src/mockggg.rs`: the registration's
  user-agent written out at 30 (`REGISTERED_USER_AGENT`) and the 400 for
  a request without it (691–706); `HttpRequest` (1031: method, path,
  lowercased headers, body) and `read_request` (1041), the parser the
  mock and the login's callback share. Its tail test module (1119) names
  `reqwest` at 1165, test code.
- `crates/acquisition-daemon/src/ratelimit/tests/spec.rs`: on the
  crate's one test clock since the residue step (`parked_choke`, 28; no
  `ChokePoint::new()` left); `NO_BEARER_API` (22); `serve_one_raw` (96,
  reads the request into a buffer it discards), `raw_response` (107),
  `full_rate_headers`, `counting_server` (2420). The production client's
  wire tests that stay TCP for good (the design's §2 and §5: the
  real-wiring tests of the production transport):
  `full_200_with_truncated_body_is_network_after_observing_policy`
  (1929), `malformed_200_with_truncated_body_is_network_and_preserves_policy`
  (2351), `a_3xx_is_one_send_one_journal_line_and_no_second_request`
  (2469, C130), `truncated_429_and_500_keep_status_and_record_transfer_evidence`
  (2648) and `c128_a_bearer_goes_only_to_the_api_origin_and_nothing_reaches_either_server`
  (3435). No spec test reads the user-agent a request carries (`grep -n
  -i "user-agent\|USER_AGENT" crates/acquisition-daemon/src/ratelimit/tests/spec.rs`
  prints nothing); the mock's 400 holds it today for every mock-facing
  test (`TESTING-NOTES.md` 207–213).
- The words a failed send carries, which this step keeps: a transport
  failure's and a body failure's are reqwest's own `to_string()`
  (`choke.rs` 568, 597, 840, 1004, 1031, 1057), pinned by
  `crates/acquisition-daemon/src/daemon/tests/dispatch.rs` 2139 and 2164
  ("error decoding response body") and by the "body transfer failure"
  outcome (`spec.rs` 1949, 2672; `auth.rs:828`).
- `crates/acquisition-daemon/src/daemon.rs`: `Daemon::builder` (1200),
  its private options, each test-only one under
  `#[cfg_attr(not(test), allow(dead_code))]`, and `build` constructing
  the choke point through `with_clock_and_rails` (1313).
- `tools/arch-check.sh`: the header's list (2–49, check 12 at 43–47) and
  check 12 (634 to the end): the tree it reads (`crates/acquisition-daemon/src`,
  `/tests/` paths dropped, comment lines blanked), the tail test module
  it recognises (one `#[cfg(test)]` / `mod <ident> {` pair at column
  zero, closed by the file's last line; a second, or a module that does
  not close the file, refused as unrecognised), its rename refusals, its
  `ok` line. `tools/arch-check-breakers.sh`: its header (the timer cases
  named at 19–25), the timer cases (284–293: the tree as it stands
  passing, then each refusal staged through `in_tests` or `drift`, the
  line each refusal must name read from the file), its restore of every
  touched file byte for byte; run by CI's breakers job, not the gate.
- `GLOSSARY.md` (a working noun gets a line when a second document uses
  it), `TESTING-NOTES.md` 207–213 (the user-agent held by the mock).
- `.cargo/mutants.toml` 14–16: `ratelimit.rs` and `ratelimit/*.rs` in
  the scope, so `transport.rs` is examined from its first commit.

## Input

Fixed before the work starts:

- **The measure of the class**: the lines check 13 would refuse at
  `10d1dfc3`, read in check 12's way (under
  `crates/acquisition-daemon/src`, `/tests/` paths dropped, comment
  lines blanked, a file's tail test module skipped) for the word
  `reqwest`: 20 lines, `choke.rs` 11 (109, 201, 230, 237, 239, 243, 534,
  755, 798, 933, 1067), `policy.rs` 6 (111, 338, 427, 523, 524, 590),
  `kinds.rs` 2 (1014, 1044), `auth.rs` 1 (63); the comment lines
  (`ratelimit.rs:13`, `choke.rs:198`, `auth.rs:61`) and the two in tail
  test modules (`mockggg.rs:1165`, `rails.rs:1423`) not among them (a
  Python reader of check 12's shape, run by the orchestrating session
  on 2026-11-01). After this step: every such line in `transport.rs`.
- **The send sites**: four, `head` (564–568), `post_form` (998–1004),
  `get_bearer` (1027–1031), `get` (1057); `grep -n 'self\.http'
  crates/acquisition-daemon/src/ratelimit/choke.rs` prints three (564,
  1028, 1057), `post_form`'s `.http` standing alone at 999.
- **The failing input the new tests stage**: none fails today; the
  table of Do 3 pins what holds, and its two runs are the evidence the
  seam changed nothing on the wire.
- **The last push's CI**: `Gate (Windows)` 41379526988 on `10d1dfc3`,
  success; `Gate (Linux)` 41379526995 on `10d1dfc3` in progress when
  this brief was written (`gh run list --branch main --workflow "Gate
  (Linux)" --limit 1`, the same for `Gate (Windows)`). The commit is a
  registry and roadmap edit. Re-read at the step's start; a red run is
  this step's first concern or is filed.

## Do

In this order, one commit each, each gated on its own tree.

1. **`http` named where its types are.** `http = "1"` a direct
   dependency of the daemon (no feature named, so the graph is the one
   reqwest already builds; the `features` stage is the proof), and every
   production `reqwest::StatusCode`, `reqwest::header::HeaderMap` and
   `reqwest::header::LOCATION` named as `http`'s: `choke.rs` 109, 534,
   933 and 1067, `policy.rs` 111, 338, 427, 523, 524 and 590, `kinds.rs`
   1014 and 1044. The same types, so nothing else moves; tests are left as
   they are (`responses.rs` names `reqwest::StatusCode` and compiles
   unchanged). After this commit the measure above prints 8 lines,
   `choke.rs` 7 and `auth.rs` 1.

2. **The seam.** `crates/acquisition-daemon/src/ratelimit/transport.rs`,
   a child module of `choke` (below), holding:
   - **`Prepared`**, the request as a value: the method (`http::Method`:
     GET, HEAD or POST), the URL as the string the caller gave (so a URL
     that does not parse fails inside the production transport with
     reqwest's words, as today), the headers (`http::HeaderMap`) and the
     form of a POST. The user-agent is written on every prepared
     request from `provider::USER_AGENT`; the bearer, where there is one,
     by `Secret::authorize` into the headers. If `Prepared` has a
     `Debug`, it prints no form value and no header value marked
     sensitive (the credential discipline of `auth.rs` 38–49, #113: a
     test failure or a log line names no token).
   - **`Answer`**: the status (`http::StatusCode`), the response headers
     (`http::HeaderMap`), and the body as a future that completes later
     with the text or the failure's words (`Result<String, String>`).
     The body is a future on purpose: the limiter observes the headers
     before the body is read and the gate's permit spans the body (C21),
     both as today.
   - **`Transport`**, one method: send a `Prepared`, answer an `Answer`
     or a transport failure's words (`Result<Answer, String>`), as a
     future the choke point awaits under its permit. The trait is
     object-safe, so the choke point holds `Box<dyn Transport>` (or an
     `Arc`). `Prepared`, `Answer` and `Transport` are the neutral types,
     visible to the crate, since step 2's transports live in `src/tests/`;
     nothing neutral sends anything.
   - **The production transport**: the one `reqwest::Client`, built by
     `build_client` moved here unchanged (its user-agent from the same
     constant, kept for the HTTPS proxy's `CONNECT`, which a request
     header does not reach: the design's §1 and §2; no redirect; both
     timeouts; the proxy variables as reqwest's default, C150). Its
     `send` builds the reqwest request from the `Prepared` (the method,
     the URL string, the headers, `.form(&pairs)` for a POST's form, so
     the body bytes and the content type are reqwest's as today) and
     maps reqwest's errors with `to_string()`, the transport failure's
     and the body's, so every word a failed send carries is the word it
     carries today. **Only the choke point can construct it or reach
     its client**, by the module tree, so the compiler holds it: the
     file `ratelimit/transport.rs` is a child module of `choke` (declared
     in `choke.rs`, a `#[path]` keeping the file where the mutants glob
     `ratelimit/*.rs` examines it), the production type and its
     constructor are private to `choke` and its child, the client is a
     private field, and the module exports no client, no constructor or
     factory of one and no alias of a reqwest type (the neutral types
     are re-exported by `choke` to the crate). A sibling of `choke`
     (`limiter`, `policy`, the spec) or any file outside `ratelimit`
     that names the production type fails to compile. The choke point
     owns the transport privately and the transport owns the client
     privately (invariant 1's structural form, C161); check 13 keeps the
     word in the file, the module tree keeps the file's sender behind
     the choke point.
   - The module doc carries the mechanism, headed by C161 (the ruling's
     words stay the registry's): what a transport is, why the body is a
     future, the user-agent's two readers, what step 2 and step 3 will
     add.

   Then in `choke.rs`: the field `http: reqwest::Client` becomes the
   private transport; `new`, `with_rails` and `with_clock_and_rails`
   construct the production one; `ChokePoint::with_transport(clock,
   rails, api_base, transport)` beside `with_clock_and_rails`, under
   `#[cfg(test)]`, so a shipped daemon can hold no other transport. Each
   of the four send sites builds its `Prepared` where it builds the
   reqwest request today, after admission and after the token is read
   (C87), C128's check still before any wait or permit, and awaits the
   transport where it awaits `send()` today; `observe` and `finish_send`
   take `Result<Answer, String>` and read the status and headers before
   awaiting the body as they do today; `head` drains the body as it does
   today. `wait` is measured before the transport is called, as before
   `self.http`. `classify_response`, `record_completed`, the verdict,
   the lost-send note and every caller above the choke point are
   unchanged.

   `Secret::authorize` takes the prepared request's `&mut
   http::HeaderMap` instead of a `RequestBuilder` and writes
   `Authorization: Bearer <token>` there, marked sensitive, as reqwest's
   `bearer_auth` does; the token's text is still read in `auth.rs`
   alone. **The one change of words** this step may make: a token that
   is no valid header value. Today reqwest records it as a builder
   error and the send fails at `send()` under the permit, the failure in
   reqwest's words; each of the two bearer paths then goes on as it
   does for any send that landed no answer, and keeps doing so:
   - `get_bearer` (1027–1031): through `finish_send`, journaled
     `counted` as a transport failure, noted as a lost send (807–815),
     `SendError::Transport` to the caller;
   - `head` (564–568): `HEAD failed: <words>` as the probe's outcome
     (618), the route observed as a failed probe and degraded (660–665),
     journaled with `counted` false (679), so no lost send is noted, and
     `ProbeError::Failed` with no status to the caller (720–724).
   The new path fails at the same point on each, with everything above
   unchanged but the words after the failure's own prefix, which are
   the daemon's and name no token text. Pinned by one component case
   through `with_transport`, a row per path: the transport never called;
   the GET's journal line counted, its `error` the daemon's words, the
   next send on its route paced as after a lost send; the HEAD's line
   uncounted, the route degraded, no lost send noted. The words listed
   at the close. If the worker finds reqwest's words reachable without
   copying them by hand, the words stay reqwest's and the case says so.

   Comments that name the client follow it: `ratelimit.rs` 12–14 (the
   choke point privately owns the transport, which privately owns the
   one client), `choke.rs` 198–200, `auth.rs` 61–62.

3. **The table, two runs.** In `spec.rs`, one table of the four sends
   (a bearer GET, a HEAD with a bearer, a token POST with its form, an
   unauthenticated GET), each row's expectation the method, the path,
   the user-agent (`provider::USER_AGENT`), the bearer's header where
   the row has one and none where it has not, and for the POST the
   form's pairs; asserted twice, so the same rows hold both sides of the
   seam:
   - **in-process**: a recording transport through `with_transport` on
     the spec's parking clock, answering each send a 200 with the
     spec's full rate headers, the row asserted on the `Prepared` it
     received (the bearer read as a header value, not a string a field
     carried), and the bearer's header marked sensitive;
   - **on the wire**: the production transport (`with_clock_and_rails`
     on the parking clock, the API base the listener's) against a TCP
     listener that reads each request whole with `mockggg::read_request`
     and answers the same 200, the row asserted on the bytes it read,
     and for the POST its content type
     (`application/x-www-form-urlencoded`) and its body decoded to the
     same pairs.
   The choke point's record of each send (`recent_sends`' outcome and
   `ok`) is the same in both runs. This is the real-wiring test of the
   prepared request (the design's §2 list, its first item); the 3xx and
   the truncated 200 are already real-wiring tests of the production
   path (1929, 2351, 2469, 2648, and C128's 3435), kept as they are,
   each gaining one doc sentence naming it a real-wiring test of the
   production transport that stays TCP for good (C161), so step 2's
   inventory of the spec's binds reads it from the test. One new test
   function, or two if the worker finds the runs read better apart; no
   new file (C153).

   The table's wire run cannot tell a forwarded user-agent from the
   client's default, since both are `provider::USER_AGENT` and reqwest
   fills an absent request header from the client's (the locked
   reqwest's `async_impl/client.rs`, the default headers inserted only
   where the request has none). So one more real-wiring case, in
   `transport.rs`'s own tail test module (the one place the production
   type is in reach of a test): a `Prepared` whose user-agent is a value
   distinct from the client's default, sent through the production
   transport to a TCP listener, the listener reading that value and not
   the default. The in-process run holds what the TCP run cannot: that
   the choke point writes the user-agent and the bearer onto the
   prepared request itself, the bearer marked sensitive.

4. **Check 13.** In `tools/arch-check.sh`, in check 12's shape and
   reusing its reader of a file's tail test module: under
   `crates/acquisition-daemon/src`, `/tests/` paths dropped and comment
   lines blanked, the word `reqwest` stands only in
   `ratelimit/transport.rs` and inside a file's tail test module; the
   crate is never renamed (`use reqwest as`, `self as` in a `use
   reqwest::{…}` group, `extern crate reqwest as`, read as check 12
   reads tokio's), and the daemon's manifest declares it under its own
   name (a dependency entry with `package = "reqwest"` is a rename the
   word check could not see); a file the tail-module reader cannot
   recognise is refused as check 12 refuses it. The refusal names the
   line and what to do (send through the choke point, which owns the
   one transport; C161), and the `ok` line names the one file and the
   count of tail test modules. The header's list gains item 13. Its
   breaker cases in `tools/arch-check-breakers.sh`, in the timer cases'
   form and in this commit (TEMPLATE, "a breaker case in the commit of
   the check it stages"): the tree as it stands passing; `reqwest` in a
   production line of another file; before a tail test module, the
   module kept; under a test attribute that is no module; through a
   crate renamed in a source; through a dependency renamed in the
   manifest; a file whose test module does not close it. C161's
   *Pinned* names check 13 and its breakers, keeping "the wire table's
   expiry rows, when built" for step 3.

5. **The words.** `GLOSSARY.md` gains "transport" (what carries a
   prepared request to the provider, or to a fixture under a test, and
   answers; the choke point prepares the request and classifies the
   answer) and "fixture" (a test's stand-in for GGG, answering what a
   transport carries to it), each one plain line in the file's form.
   `TESTING-NOTES.md` 207–213 gains one sentence: the spec's table holds
   that the prepared user-agent reaches the wire through the production
   transport, beside the mock's 400. The build's last commit sets
   "Current step" in `roadmap/README.md` to say the build has landed and
   its review is pending.

6. `tools/gate.sh` once per commit on each commit's tree, its exit
   status read on its own line; every golden under
   `crates/*/tests/goldens`, `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and
   `README.md` byte-identical to the step's parent;
   `tools/arch-check-breakers.sh` run once on Do 4's tree, its last line
   in the report.

## Done when

Each line from the commit alone, its tree or its message, except the
last, which names the execution evidence and where it lives.

- `crates/acquisition-daemon/Cargo.toml` names `http` as a dependency
  with no feature; `grep -rn reqwest crates/acquisition-daemon/src
  --include='*.rs'`, comment lines, `/tests/` paths and tail test
  modules left out, prints only `ratelimit/transport.rs` lines (the
  message quotes the 20 before and the count after); `tools/arch-check.sh`
  holds check 13 and the header names it.
- `choke.rs` names no `reqwest` and no `self.http`; its four send sites
  each build a `Prepared` and call the transport; `with_transport`
  exists under `#[cfg(test)]` and nowhere else does a constructor take a
  transport; `build_client` lives in `transport.rs` with its user-agent,
  its redirect policy and both timeouts as at `10d1dfc3`.
- `Secret::authorize` takes a header map, marks the value sensitive,
  and `auth.rs` is still the one file that reads a `Secret`'s text
  (`grep -n "fn expose\|\.expose()" crates/acquisition-daemon/src --include='*.rs' -r`
  prints `auth.rs` lines only).
- `spec.rs` holds the two-run table of Do 3, with the five kept wire
  tests each carrying their sentence, and the component case of the
  invalid header value with its GET and HEAD rows; the message names
  the words that case pins, or says reqwest's words were kept;
  `transport.rs`'s tail test module holds the distinct user-agent case.
- `transport.rs` is declared as a child of `choke`; the production
  type, its constructor and its client are visible to `choke` alone; no
  item of the module names a reqwest type in a signature visible outside
  it (read from the file).
- `tools/arch-check-breakers.sh` holds the seven cases of Do 4 in Do 4's
  commit; C161's *Pinned* names check 13.
- `GLOSSARY.md` has the two lines; `TESTING-NOTES.md` its sentence;
  "Current step" says the review is pending.
- No golden, no reference and no README line changed (`git diff --stat
  <first>^..<last> -- crates/*/tests/goldens CLI-REFERENCE.md
  MCP-REFERENCE.md README.md` prints nothing); no test removed and no
  existing assertion changed except the five doc sentences of Do 3.
- The gate's exit status on each commit's tree is execution evidence:
  the worker's report carries it per stage for each commit with the
  breakers' last line, the orchestrating session re-runs the gate on
  the step's final tree, and the close's message names the trees and
  the results.

## Not in scope

- The in-process transports, the six `Server` kinds onto them, the
  mock's `answer` split, the hook's occupancy read, `quiet`, `beside`,
  the spec's servers moved in-process, `auth.rs`'s two token tests on a
  parking clock: step 2, after C160's first amendment.
- **The builder's transport option** (`Daemon::builder`, the design's
  §6 step 1 last sentence): moved to step 2, where the door's six
  `Server` kinds are its first callers. A private builder option with no
  caller fails `clippy -D warnings` under `--all-targets`, and an
  allowance kept for one step is the shortcut this project refuses; the
  narrowing is this brief's, dated 2026-11-01, and step 2's brief takes
  it. `ChokePoint::with_transport` is not moved: Do 3 is its caller.
- The deadline on the clock, `.timeout()` and `CONNECT_TIMEOUT` removed
  (the owner's (a)), the expiry rows, the pacing oracle's new case,
  `TESTING-NOTES.md`'s "Absence", the retrospective's §3 row: step 3,
  after C160's second amendment. Both timeouts stay on the client here.
- The journal's fields and words, the classification, the limiter, the
  gate, the rails, C128's refusal and its words, the token exchange and
  the refresh above `post_form`, the login's callback listener, the
  daemon's endpoint, the client harness: unchanged (the design's §2,
  "What the seam does not touch").
- Tests under `/tests/` paths naming `reqwest` (the browser's GET, the
  callback GETs, the account bait's client, `responses.rs`'s status
  type): left as they are; check 13 reads production sources as check
  12 does.
- Another HTTP client crate added to the daemon (hyper, ureq): no such
  dependency exists, and check 5 holds the dependency edges; refusing a
  named list of HTTP crates is a mechanism no ruling asks for. Raised at
  the close if the worker finds it wanted.
- The mutants run: the step's close (the design's §6 step 4) compares
  after the seam's three steps, not after this one.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on `main`,
in a checkout other sessions may share, in a window the owner names;
never push; `acq` and `acqd` only in a mock world under the harness
(the `mock-session` skill), and nothing in this step needs either;
never a daemon against the real provider; kill only this session's
pids; `cargo build --workspace` before anything that starts a daemon;
the gate's exit status read on its own line, never through a pipe. Five
commits, each gated on its own tree. The step changes non-test code and
`tools/`, so its commits are reviewed cold as one range, and a fix
round that changes behaviour is reviewed again. The attribution line
names the model the worker runs on. The invalid-header words of Do 2
and the glossary lines are agent-facing design: the session's, recorded
in the commit that builds them and listed at the close for the owner's
read. The rules bind the worker's own work; the one that binds what it
builds is Do 2's visibility (the production transport constructed only
by the choke point).

What the brief's writer asks: Do 3's in-process run seen failing first
(a `Prepared` missing its user-agent, staged by hand and undone from a
scratch copy) and Do 4's refusals seen in the breakers, both named in
the report, not the history.

## Report

`TEMPLATE.md`, and: the gate's statuses per stage for each of the five
commits; the breakers' last line; the measure of Do 1 and Do 4 before
and after (the 20 lines, then 8, then the `transport.rs` lines); the
in-process run seen failing and how; the invalid-header case's words.
Raised for review, at least: any send whose `Prepared` could not be
built where its reqwest request was built; any word a failed send
carries that changed; any place the module tree did not let the
production transport stay private to `choke`.

## Taken from the reviews

The first review's five (`transport-seam-1-brief-dad73240.md`):

1. A sibling of `choke` could construct the production transport and
   send through it without the word `reqwest`: the file a child module
   of `choke`, the type, its constructor and its client private to it,
   nothing of reqwest exported, settled now (Do 2, Done when).
2. The invalid-token path covered the GET alone: the HEAD's path named
   (uncounted, degraded, no lost send), a row each in the component case
   (Do 2, Done when).
3. The wire run's user-agent is masked by the client's identical
   default: a case in `transport.rs`'s tail test module with a distinct
   prepared value; what the in-process run adds named (Do 3, Done when).
4. `policy.rs:590` missing from Do 1's list: added; 20 before, 8 after
   Do 1 (Do 1).
5. The secret-formatting discipline cited as C144: cited as `auth.rs`'s
   #113 docs (Read, Do 2).
