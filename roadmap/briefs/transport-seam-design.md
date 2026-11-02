# transport-seam-design — the transport seam: the design

Provenance: Fable (`claude-fable-5-1`), one design session on 2026-11-01
at `e812a0d7`, read-only: a read and a proposal, nothing built, nothing
ruled, no issue changed. It follows the time-seam retrospective
(`roadmap/reviews/time-seam-retrospective.md`, §4 Root A, §6
"Virtualising reqwest's timeouts", §8 "What I would want") and the
owner's word of 2026-11-01 (`roadmap/README.md`, the rulings list: the
transport seam's design brief is Fable's, its build Opus's, in series).
What was read: `AGENTS.md`, `GLOSSARY.md`, `roadmap/README.md`, the
retrospective, `decisions/daemon.md` (C157, C158, C160, C133, C148 and
the "Not doing" line on a request timeout's expiry), `decisions/network.md`
whole, `docs/design/network-ground-truth.md` at N1 to N4, N28, N33 and
N35, `TESTING-NOTES.md`, `REVIEW-CHECKLIST.md`'s network and daemon rows,
and the code: `choke.rs` whole, `ratelimit.rs`, `gate.rs`, `rails.rs`'s
`SendReport` and `Verdict`, `auth.rs`'s `Secret` and token request,
`daemon/login.rs`, `mockggg.rs`, `daemon/kinds.rs`'s send site, the
daemon's `within_on`, the test door, the harness, the parking clock, the
limiter's spec, the client harness, `.cargo/mutants.toml` and checks 10
to 12 of `tools/arch-check.sh`. Every count below names its command, run
at `e812a0d7`; line numbers are that commit's. The last push's CI on
`main` is green on both gates (`gh run list --branch main --workflow
"Gate (Linux)" --limit 1`: 41367513039; `"Gate (Windows)"`:
41367512967). A design brief: §7 is the owner's to rule, and the build
briefs of §6 follow his word, one at a time, each reviewed cold. Cut
three times: the first cut (`855ddcf6`) and the second (`74423db0`)
each reviewed cold (`roadmap/reviews/transport-seam-design-<sha>.md`,
the prompt beside); this cut answers the second review's five findings
and takes the orchestrating session's input on #151's compare, read on
2026-11-02 (`73f182bd`), each at "Taken from the reviews" at the foot.

## 1. What the tree says

### The HTTP client and the sockets

The daemon's production sources touch one HTTP client and three
listeners.

- **The one client** is `ChokePoint::http` (`choke.rs:201`), built by
  `build_client` (230 to 244) with the user-agent on the client (238;
  reqwest 0.12.28 hands a client's default user-agent to its connector,
  which sends it on an HTTPS proxy's `CONNECT` too: `async_impl/client.rs`
  663 and `connect.rs` 863 to 866 in the locked crate), no redirect
  (C130), a connect timeout of 10 s and a whole-request timeout of 60 s (`CONNECT_TIMEOUT`, `REQUEST_TIMEOUT`, 216 and 217; no test
  names either: `grep -rn "CONNECT_TIMEOUT\|REQUEST_TIMEOUT"
  crates/acquisition-daemon/src` finds the definitions and the builder
  alone). Four sites send through it (`grep -n "self\.http\|\.post(url)"`):
  the probe's HEAD (564), the token POST (`post_form`, 1000), the bearer
  GET (`get_bearer`, 1028) and the unauthenticated GET of the mock-only
  kinds (`get`, 1057, the one caller `kinds.rs:817`). The word `reqwest`
  is on 12 lines of `choke.rs` (`grep -c`) and leaks as types elsewhere
  (`grep -rn reqwest crates/ --include='*.rs'`): `CompletedResponse::status`
  (109) and `kinds.rs` 1014 and 1044 name `reqwest::StatusCode`;
  `policy.rs` parses `reqwest::header::HeaderMap` at five sites (111,
  338, 427, 523, 590) and names `reqwest::header::LOCATION` at 524; `auth.rs:63` attaches the bearer to a
  `reqwest::RequestBuilder` (`Secret::authorize`, called at `choke.rs`
  566 and 1028). Those are `http`'s types re-exported: `http` 1.5.0 is
  in `Cargo.lock` as reqwest's dependency and is no direct dependency of
  any crate. The dependency is `reqwest 0.12`, default features off,
  `json` and `rustls-tls` (`crates/acquisition-daemon/Cargo.toml:35`);
  check 5 of `tools/arch-check.sh` forbids it to the store and the search
  (`arch-check.sh` 311, 313) and names it nowhere else.
- **The login's callback listener** (`login.rs:23`, bound at `auth_start`;
  `wait_callback` 976, `answer_callback` 1004 on a `TcpStream`) is the
  one the browser connects to. It reads under `CALLBACK_READ_WITHIN` on
  the daemon's clock (C160) and answers with `mockggg::read_request` and
  `respond`. It is inbound, and C130 places it, with the browser's
  authorize navigation, outside invariant 1.
- **The mock's listener** (`mockggg.rs:658`, `start_with_clock`; `handle`
  681 to 972, one connection per task; `read_request` 1041, `respond_with`
  1101) is the stand-in GGG the shipped daemon starts in mock mode
  (`start.rs:455`, `mockggg::start`, which alone reads
  `ACQ_MOCK_WINDOW_DIVISOR`). It holds invariant 4 where GGG would see
  it: a request without the registration's user-agent, written out at
  line 30, is answered 400 (691 to 706), the browser's two pages
  excepted.
- **The daemon's own endpoint** (`daemon/endpoint.rs`: a Unix socket or a
  named pipe) is the frontend plane, C9's, and not this seam's.

Of these, the choke point owns the client and nothing else; the login
owns the callback; the mock owns its listener. Nothing else in the
daemon's sources binds or connects (`grep -rn
"TcpListener\|TcpStream\|UnixListener\|named_pipe"
crates/acquisition-daemon/src --include='*.rs'`, the `tests/` paths and
`spec.rs` dropped, leaves `daemon.rs:114`'s import, `login.rs` 12, 23,
977 and 1005, `mockggg.rs` 16, 658, 682, 1041, 1096 and 1102,
`endpoint.rs`, and two listeners in `auth.rs`'s tail test module, 796
and 834).

### The test fixtures and how each reaches the wire

Every fixture is a real TCP server inside the test's runtime, reached
by the one reqwest client over loopback. `grep -c "TcpListener::bind"`
over the daemon's test files: `harness.rs` 4 (the scripted server 381,
the delayed token server 445, the answering token server 1138, the
account server 1290), `dispatch.rs` 1 (a saturated probe server, 25),
`bearer.rs` 3 (66, 259, 604), `log.rs` 3 (21, 216, 371), `sessions.rs`
2 (105, 488), `exit.rs` 1 (702, a scenario's own server), `login.rs` 2
(646, 684: the callback listener's own component tests, the other
side), `auth.rs` 2 (the token request's own tests, on
`ChokePoint::new()` and so on `SystemClock`, 815 and 879) and
`ratelimit/tests/spec.rs` 12 (the raw-response helper `serve_one_raw`
at 97, and the wire tests of the trip, the 3xx, the gate's span, the
exclusive HEAD and C128). The door's `Server` kinds (`door.rs` 72 to
92) wrap the harness's: `Script` 19 occurrences, `Account` 19, `Token`
4, `Mock` 5, `own` 11, `None` 14 (`grep -rho "Server::<kind>" | wc -l`
over `daemon/tests/`); `None` is a base nothing listens on
(`http://127.0.0.1:1`, `door.rs` 652), so every send is refused at
connect. `Server::own` is called seven times (`grep -rn "Server::own("`,
the definition and its doc dropped: `dispatch.rs:146`, `bearer.rs` 157,
350, 546, 653 and 754, `exit.rs:701`) over five bind sites
(`dispatch.rs:25`, `bearer.rs` 66, 259 and 604, `exit.rs:702`); the
five binds of `log.rs` and `sessions.rs` are component fixtures behind
the harness's constructors, not the door's.

What a fixture does: the scripted server (`scripted_server_received`,
373) answers one connection per `ScriptedResponse` in order, asserts
each request's method and path, records the methods (what the journal
is held equal to) and each request whole as `Received` (344: method,
path, the bearer stripped of `Bearer `, the body; `Received::grant`
reads a refresh grant from the form), and writes raw bytes when the
status is empty (the truncated and garbage shapes). The account server
(1282) answers by the account a bearer or a grant names, records
`(method, account)`, and holds the one request the scenario names
behind `arrived` and `release`. The delayed token server (436) holds one
refresh the same way. The stateful mock keeps rate-limit counters on the
daemon's clock (`start_with_clock`) and the N6 and never-over-the-limit
oracles are real only against it (`TESTING-NOTES.md`, "Three kinds of
cannot"). A scenario's own server (`Server::own`) binds and spawns
inside the door's runtime with barriers the scenario holds. The spec's
servers answer raw bytes: a truncated 200, a malformed one, a 3xx, a
held HEAD, a foreign listener that must receive nothing (C128).

The oracles read two things: the journal (`rails.rs` 819, one line per
completed exchange: method, route, path, status, `counted`, the rate
and header snapshots, `wait_ms` at 833, the error's words; `SendReport`
371 to 398 is the whole of what a send reports) and the fixture's
record, held equal to it (`assert_journal_matches_wire`, `harness.rs:881`);
then the contract (`assert_wire_contract` 947, `probe_exempt` 979), the
pacing (`assert_pacing_follows_responses` 999) or the limit
(`assert_never_over_the_limit` 1095), selected by the fixture kind at
`Door::finish` (`door.rs:1211`). The census (`python3 -I
tools/daemon-tests-census.py`): 197 tests in 20 files, door 91,
boundary 73 (Child 1, Clock 5, Fixtures 1, Function 52, Socket 2,
Workers 12), sentence 33, neither 0. The limiter's spec: 16 tokio tests
and 49 synchronous (`grep -c "#\[tokio::test\]"`, `grep -c "#\[test\]"`),
every choke point on `ParkingClock` (six `with_clock_and_rails` sites,
no `ChokePoint::new()`).

### What the park hook cannot see, and why

The hook (`door.rs` 264 to 313) runs at every park of the door's
current-thread runtime and asks, in order: a `quiet` waiting; the
scenario's holds; a send in flight (the gate's occupancy, `choke.rs:480`, read at `door.rs:276`);
a blocking task in flight; a `settle` waiting; then it advances the
clock to the next deadline, and with nothing parked it is the stuck
verdict (C160). A send in flight holds the clock because it is real
I/O: the fixture answers in real time on another task, through the
kernel, and virtual time must not run ahead of it. That one fact is
what the retrospective's three rules stand in for:

1. **`hold_clock`** (`door.rs:886`; three sites): a scenario's wait that
   neither count sees. `exit.rs:736` holds across "the callback and the
   exchange's way to the wire"; `browser` (851) holds across the
   browser's GET; `dispatch.rs:195` freezes time on purpose while an
   independent job runs beside two parked rate waits. The first is the
   fixture's socket, the second the daemon's callback socket, the third
   the scenario's own intent.
2. **The "Not doing" line** (`decisions/daemon.md`): a request timeout's
   expiry is not virtualised, because reqwest's timers run on tokio's
   wheel inside the client the choke point owns; C160 names them "the
   one timer off the clock" and says they sit inside occupancy. A silent
   fixture holds its permit through the real 60 s, and the door's module
   doc says so (`door.rs` 54 to 57).
3. **`beside`** (604; two sites, `dispatch.rs` 1129 and 2303, a second
   lifetime on `Server::None`): no hook of its own, so its waits have no
   verdict and keep `HANG`. The hook reads one daemon's occupancy and one
   daemon's blocking count.

A fourth sentence belongs with them: **`quiet`'s precondition** (835;
eight sites): a scenario may call it only after the arrival barrier of
a request it holds has fired, because before an arrival a park may be a
socket read's on the way to the fixture, and the quiet would resolve
early. The quiet is asked first at every park (`door.rs:267`), before
any accounting, so it also resolves at a park on the way to the
transport: a pacing sleep, a blocking credential read. The first three
are rules about a socket, and the fourth is half one. Remove the socket
and three become structure; the fourth keeps its half.

## 2. The reframing: the wire is a value

One sentence: **the choke point prepares a request, owns its deadline on
the daemon's clock and classifies the answer; a transport only carries
it; in production the transport is the one reqwest client, and a door
scenario's is in-process, so no provider exchange in a scenario is real
I/O.** A real-wiring test of the production transport keeps its
listener, as C157 provides; the browser's one GET to the daemon's
callback stays a real socket under a hold.

The seam sits inside `choke.rs`, between the admission loops and the
four send sites, at exactly the line where `self.http` is named today.

- **A prepared request** is a value: the method (GET, HEAD or POST), the
  URL, the user-agent, the bearer as a `Secret` (never a string) and the
  form for a POST. The choke point builds it after admission and after
  C128's origin check, as it builds the reqwest request today; the
  user-agent is written on the prepared request too, read from the one
  constant (`provider::USER_AGENT`), so the one place every send is
  shaped carries invariant 4 and an in-process fixture sees it; the
  production client keeps its default user-agent from the same constant,
  because reqwest sends a client's default on an HTTPS proxy's `CONNECT`
  and a request header does not reach that path (§1). One constant, two
  readers, neither hand-kept. `Secret::authorize`
  takes the request's header map instead of a `RequestBuilder` and
  writes the sensitive header value there, so the token's text is still
  read in `auth.rs` alone (invariant 5 and the `Secret` discipline
  unchanged).
- **A transport** is one method: send a prepared request, answer a
  status, the response headers and a body that completes later, or a
  transport failure in words. The body is a future on purpose: today the
  limiter observes the headers before the body is read (`observe`, 752;
  `finish_send` 793) and the gate permit spans body completion (C21);
  both stay as they are, the body awaited under the same permit and the
  classification (`classify_response`, 155) fed the same inputs. The
  types are `http`'s (`StatusCode`, `HeaderMap`), named as such: `http`
  becomes a direct dependency of the daemon (already in the lock at the
  version reqwest pins, so no feature moves, C84), and `reqwest` is named
  in one production file.
- **The production transport** wraps the one `reqwest::Client`, built as
  today with its user-agent, less its timeouts once step 3 lands, still
  private, still following no redirect, still honouring the proxy
  variables (C150), the proxy path unchanged by construction. It
  lives in `crates/acquisition-daemon/src/ratelimit/transport.rs`,
  inside the mutants glob (`.cargo/mutants.toml`, `ratelimit/*.rs`).
  Invariant 1 keeps its structural form: the choke point owns the
  transport privately, the transport owns the client privately, and a
  new check 13 of `tools/arch-check.sh`, in check 12's shape, refuses the
  word `reqwest` in any daemon source outside that file, a comment line
  or a tail test module. A second sender is then a check failure, not a
  review finding.
- **The deadline is the choke point's**, on its `Clock`: one absolute
  instant per send, `REQUEST_TIMEOUT` from dispatch, raced through
  `sleep_until` against the transport's answer and again against its
  body, the same instant both times, so the body spends what the headers
  left and no second budget opens (`within_on`, `daemon.rs:1023`, is
  the select's shape, the future first and the sleep polled
  unconstrained; it takes a duration, so the helper that takes the
  absolute instant is new beside it). An expiry on the answer is a
  transport failure; an expiry on the body is a body failure under the
  status that landed, which is what reqwest's timeout gives today (D8: a
  non-2xx keeps its status, a 2xx with a failed body is `Network`);
  either way the permit is released as the exchange ends. Dropping the
  transport's future on expiry closes its connection, which is what
  reqwest does for its own timer. `CONNECT_TIMEOUT`
  is the owner's call (§7): one deadline, or the production transport
  keeping a connect bound of its own.
- **The test transports** are in-process implementations of the same
  trait, in the crate's test support beside the parking clock
  (`src/tests/transport.rs`, visible to the daemon's tests and the spec
  alike): a scripted one (the `ScriptedResponse` list, the raw form
  kept, recording `Received` as today), the account server's logic, the
  two token servers', the stateful mock's face, a scenario's own closure,
  and one that refuses every send as connection refused (`Server::None`).
  A request's arrival at a fixture is the transport's first poll: it
  records the request before it first returns `Pending`, in the daemon
  task's own poll, so no park falls between the send and the record. **A
  request in flight at an in-process fixture holds the clock from its
  first poll until its body is done**, by the transport, not the
  fixture's author: the hold is taken in that first poll and dropped
  when the body future completes or is dropped, the same span the gate's
  occupancy covers today (from before dispatch until the body completes,
  C21), so a fixture that answers its headers and holds its body behind
  a release (the spec's own shape, `spec.rs` 2696 to 2702) holds the
  clock through the body as it holds the permit; a release fires from
  the scenario's own task, as today, and the transport observes it
  before it lets the hold go. So a held request (the account server's,
  the delayed token server's, a held body's) is a hold the hook sees
  without a count of the gate's, and a script that declares a response
  silent, before its headers or after them, takes no hold for the silent
  part, so the clock runs to the choke point's deadline and the
  timeout's expiry is a row in virtual time. The hold
  proves nothing about arrival to a scenario that parks before the
  transport is reached; `quiet`'s arrival rule stays for that (§3).
- **The stateful mock** splits into an answer and a wrapper: `answer`,
  the match of `handle` (681 to 972) over a parsed `HttpRequest` giving a
  status, headers and a body, and the TCP `handle` reduced to read,
  answer, respond. The listener stays, since the shipped daemon's mock
  mode serves a user's browser and the client harness's `acqd` children
  (`acq demo`, the mock-session skill, every process test), and the
  user-agent check stays in `answer`, where both faces run it. Under the
  door, `Server::Mock` is the answer behind the in-process transport, and
  `browser` reads the mock's two pages in-process and then makes its one
  real GET to the daemon's callback listener under a hold, as today.

What the journal and the oracles see: the same lines, from the same
`SendReport`, written at the same point (`record_completed`, 928, under
the permit). `wait_ms` is measured before the transport is called, as it
is measured before `self.http` today. The fixture's record is what the
transport received, compared by method as today; `Received`'s bearer
and grant are read from the prepared request, so the oracle holds the
journal against the transport's input where it held it against the
wire's bytes, and the bytes are the kept TCP tests' (below). What
changes in the journal, by phase (`record_completed`, 928 to 956:
`error` is set only from a failure with no answer or a protocol
failure; a landed answer whose body failed keeps `error` null and says
so in its outcome): a send the deadline cuts before any answer journals
`error` in the daemon's words ("the send did not complete within 60 s")
where today it carries reqwest's ("error sending request for url (…)"),
and the job that fails on it carries those words; a send cut during its
body keeps its status, `error` null and the outcome "<status>; body
transfer failure: …" as today, the words after the colon the daemon's.
Those are words a user meets and are listed for the owner. What is no
longer asserted a hundred times: that reqwest sends the prepared
request as prepared, which the mock's user-agent check held on every
mock-facing test. It is held instead by the production transport's own
real-wiring tests (C157's word): a few TCP tests in the spec on a
hand-driven clock, that a prepared GET, HEAD and POST reach a listener
with their method, path, user-agent, bearer header and form body as
prepared; that a 3xx is not followed; that a truncated 200 is a body
failure (step 1); and that the deadline's expiry closes the connection
and releases the permit (step 3). The live rung is the rest, and it
does not reach two paths: the code exchange, which `tools/tracer-rung.sh`
skips when the index already holds the account's uuid (393 to 397),
and a proxy. The exchange is the refresh's path (`token_request`,
`auth.rs` 245 to 262, one `post_form` for both grants), which the rung
does exercise; the proxy path is unchanged by construction (§1).

What the seam does not touch: the admission loops (`acquire_send`,
`acquire_head`), the limiter, the gate, the rails, the classification,
C128's check (still before any wait or permit), the token exchange and
the refresh (`auth.rs` 198 and 227 call `post_form` and know nothing of
the transport), the callback listener (inbound; nothing of it goes
through the transport), the daemon's endpoint, the client harness, and
the parked machine of the admission loop (`decisions/daemon.md`, "Not
doing").

## 3. What each piece becomes

| Piece | Today | Under the seam | Retires, held by |
| --- | --- | --- | --- |
| the four send sites (`choke.rs` 564, 1000, 1028, 1057) | each builds a reqwest request and awaits it | each builds a prepared request and calls the transport; the deadline around both phases | `self.http`; held by the transport field and check 13 |
| the one client (201, `build_client` 230) | user-agent, no redirect, two timeouts | the production transport's: user-agent kept (the proxy path), no timeouts after step 3, no redirect, proxies as reqwest's default | reqwest's timeouts; held by the choke point's deadline |
| the user-agent (invariant 4) | on the client; the mock answers 400 without it | on the prepared request as well, both from `provider::USER_AGENT`; the mock's `answer` keeps the check on both faces; one real-wiring test that the prepared header reaches the wire | nothing |
| the bearer (`Secret::authorize`, `auth.rs:63`) | into a `RequestBuilder` | into the prepared request's header map, sensitive, in `auth.rs` | nothing |
| `reqwest::StatusCode` and `HeaderMap` (`choke.rs:109`, `kinds.rs` 1014 and 1044, `policy.rs` six sites) | reqwest's re-exports | `http`'s, named as such | nothing |
| the timeouts (216, 217) | reqwest's, on tokio's wheel, off the clock | the choke point's absolute deadline on `Clock::sleep_until`, one instant over answer and body; `CONNECT_TIMEOUT` per §7 | reqwest's `.timeout()`; held by the deadline and the wire table's two expiry rows (silent before any answer; headers then a silent body, cut at the same instant); C160's exception sentence; held by the amended entry |
| the scripted server (`harness.rs:373`) | one TCP connection per response | the scripted transport, the same list, the raw form kept, a silent form added, `Received` as today | 1 listener; held by the transport |
| the account server (1290) and the two token servers (445, 1138) | TCP, barriers | the same logic behind the transport; a held request holds the clock | 3 listeners; held by the transport's hold |
| `Server::own` (seven calls over five binds: `dispatch.rs` 25, `bearer.rs` 66, 259, 604, `exit.rs` 702) | a listener the scenario binds inside the runtime | a transport the scenario builds, its barriers its own | 5 listeners; held by the closure form |
| the component fixtures (`log.rs` 21, 216, 371; `sessions.rs` 105, 488) | listeners behind `test_daemon*` | transports behind the same constructors, through the builder's transport option | 5 listeners; held by the builder's option (`login.rs`'s two binds are the callback's and stay) |
| `Server::None` (14 sites) | a port nothing listens on | the refusing transport | the base `http://127.0.0.1:1`; held by the refusing transport |
| the stateful mock (`mockggg.rs`) | one TCP face | `answer` plus the TCP wrapper; the listener kept for the shipped daemon; the in-process face for `Server::Mock` | nothing |
| the spec's servers (12 binds) | raw TCP responses | in-process raw answers, except the real-wiring tests named in §2, which stay TCP on a hand-driven clock | the binds the build's inventory by test moves (step 2 counts them before it moves any); held by the in-process raw form and the kept tests |
| `auth.rs`'s two token tests (796, 834, on `SystemClock`) | TCP, `ChokePoint::new()` | in-process on a parking clock through `with_transport` | 2 listeners and the spec's last system clocks; held by the parking clock |
| the hook's occupancy read (`door.rs` 276) | a send in flight holds the clock, from before dispatch until the body completes | the transport's hold on a request in flight, the same span, first poll to body's end; occupancy read by nothing under the door | `held_by_send` and the two `occupancy -> (1, _)` exclusions in `.cargo/mutants.toml`; held by a `held_by_transport` count and by `c160_the_hooks_count_is_zero_at_rest_and_one_under_a_held_send` (`dispatch.rs` 2554 to 2591), whose assertions on the gate's and the choke point's counts at rest, held and landed stay and now fail the mutants at once instead of stalling the suite; its doc says the count is `daemon status`'s (`server.rs` 405, 446) |
| `hold_clock` (886) | three sites | two reasons left: the browser's GET to the daemon's callback (`browser` and `exit.rs:736`'s hand-rolled one) and a scenario's explicit freeze (`dispatch.rs:195`) | "a request on its way to the scenario's own server" as a reason; held by the transport's arrival at the call |
| `quiet` (835) | a precondition sentence with two halves | the same wait; the socket half goes (a park on the way to the fixture is no longer a socket read's), the arrival half stays: a scenario whose assertion depends on a request having reached its fixture calls `quiet` after that request's barrier, since a park on the way to the transport (a pacing sleep, a credential read) still resolves it first | the socket half; held by the record in the transport's first poll |
| `settle` (810) | unchanged | unchanged | nothing |
| `beside` (604) | no hook; `HANG` only; one caller shares the first door's clock (`dispatch.rs:1122`), one builds its own (`2296`) | takes the first door's clock and hook (a `beside` on another clock is refused), registers its whole accounting with the running hook before any restore wait (its daemon's blocking count, its holds, its `quiet`, `settle` and stuck slots) and unregisters at `finish`; the second caller moved onto the shared clock; its waits gain the verdict, `HANG` stays the backstop | "a `beside` door has no hook" in C160 and the module doc; held by the registration and a `c160_` case on a beside wait |
| the "Not doing" line on a request timeout's expiry | structural | removed when C<next> is entered, before step 1 | the line; held by the expiry rows and C<next> |
| #151's remainder: the ten mutants under which three door scenarios still hang after C160 (the compare of `73f182bd`, the issue's comment of 2026-11-02: `setup_after_probe_429`, `degrade_for` and the two `parse_retry_after` caps under `a_head_429_that_fails_setup_holds_the_next_probe`; `head`'s `delete !` and `\|\| with &&` and `record_completed -> ()` under `a_blocked_probe_is_no_probe_failure`; the two `verdict_of` mutants under `a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header`) | each breaks a send's completion or a probe's setup, so the gate's occupancy stays held, the hook rightly moves no time and calls nothing stuck, and reqwest's real timeout ends the wait | the fixture has answered and released its hold, nothing is parked and the daemon's own side never completes: the park finds nothing the hook accounts for and the verdict fires at once; the fixture-side hold is what tells a send in flight from one the daemon will never complete | those timeouts, each named at the close in C154's form, `retired <mutant> timeout; held by the verdict`, as the compare proves them caught; `Limiter::observe_landed`'s `== with !=` and `ChokePoint::sleep_until -> ()` hang component tests off the door, the second by spinning the lifecycle's driver, which no count can see, and stay |
| the login's exchange and refresh | through `post_form` | unchanged | nothing |
| the callback listener (`login.rs:23`) | TCP, on the clock | unchanged; outside invariant 1 (C130); its two component tests stay | nothing |
| the client harness's servers | none; `acqd` spawned in mock mode over its TCP mock | unchanged | nothing |
| `daemon status`'s occupancy | `actual_send_occupancy` | unchanged | nothing |
| the journal, the oracles, `SendReport` | as `TESTING-NOTES.md` holds them | the fields unchanged; the pacing oracle (`harness.rs` 1003 to 1040) gains the case of a counted send that landed no status, which the choke point paces as lost (`note_lost_send`, `choke.rs:814`) and the oracle today reads as "nothing demanded a hold" | nothing |
| the mutants scope | `ratelimit.rs`, `ratelimit/*.rs`, `gate.rs` | `transport.rs` inside the glob; the test transports outside it, as test support | the two occupancy exclusions (above) |
| `TESTING-NOTES.md`, "Absence" | "the answer is the client's timeouts" | "the answer is the choke point's deadline" | nothing |

## 4. What it retires, and what it must not change

In C154's form, each at the step that holds it:

- retired `build_client`'s timeouts; held by the choke point's deadline
  with the wire table's two expiry rows and the real-wiring test of the
  expiry's cancellation.
- retired the fixtures' TCP listeners in the daemon's tests (the four of
  the harness, the five behind `Server::own`, the five component
  fixtures of `log.rs` and `sessions.rs`, the spec's raw-response helper
  and the callers the inventory moves, `auth.rs`'s two); held by the
  in-process transports and the oracles over them.
- retired the hook's occupancy read and its `held_by_send` count; held
  by the transport's hold on a request in flight, first poll to body's
  end, counted as the scenario's.
- retired the ten door-scenario timeouts #151 leaves open; held by the
  verdict reading the transport's hold, each named at the close as the
  compare proves it caught, and the two component-test timeouts named
  as what the seam does not reach.
- retired the two `occupancy -> (1, _)` exclusions; held by the
  `c160_` count pin's assertions, which fail at once once the hook no
  longer reads the count.
- retired `quiet`'s socket half and the socket reason of `hold_clock`;
  held by the record in the transport's first poll.
- retired the "Not doing" line on a request timeout's expiry and C160's
  "the one timer off the clock"; held by the deadline on the clock and
  the expiry rows.
- retired "a `beside` door has no hook"; held by the registration.
- retired `CONNECT_TIMEOUT`, if §7 rules so; held by the one deadline.

What must not change, and what pins it:

- **The journal's fields**: every field of `SendReport`, `wait_ms`
  measured from ready to dispatch, the header's clock, one line per
  completed exchange under the permit. Pinned by the oracles and the
  live-run verifiers (`tools/tracer-verify.py`, `tools/persist-verify.py`),
  which read the journal and nothing of the transport. The words in
  `error` and the outcome for a cut send change as §2 says, and the
  pacing oracle gains one case; nothing else.
- **The wire oracle**: journal equals what the fixture received, in
  order, by method. The fixture is the transport now, so the evidence is
  the transport's input rather than the wire's bytes; the comparison is
  the same function, and the bytes are the kept TCP tests'.
- **The rails' reports**: the same `SendReport` at the same point, so
  the tripwire, the ceiling, the block mark and the journal see what
  they see today.
- **C79, C88, C128**: no new surface is touched; the real provider stays
  the default and the mock opted into; the origin check stays before any
  wait or permit, and the refusal's words stay.
- **C21, C130, C150**: the permit spans body completion; no redirect is
  followed (the production transport's policy, pinned by its real-wiring
  test; an in-process transport follows nothing by construction); the
  proxy variables route the production transport as they route the
  client today.
- **Invariants 4 and 5**: the user-agent on every prepared request and
  on the production client, from one constant; the token's text read in
  `auth.rs` alone.
- **The live rung's evidence**: `RUN-LEDGER.md`'s rows read the journal
  through the verifiers; nothing they read moves.

## 5. The risks

- **The production path changes under a league run.** Steps 1 and 3 of
  §6 touch the send path the owner's run goes through: the request's
  shape (step 1) and the timeouts (step 3). Phase 3, the owner's run,
  has not started (`roadmap/README.md`, "Current step": at his word). So
  both land before it starts, with a live rung after step 3
  (`tools/tracer-rung.sh`, on the owner's list, "edited at 2g, not run
  since"; its `--mock` rehearsal first), or both wait until it ends.
  Step 2 touches no production send and may land in either window.
- **The user-agent's assurance thins.** Today every mock-facing test
  proves the client sends it; after step 2 the mock's check sees the
  prepared request, and one real-wiring test proves the production
  transport sends what it is given. A reqwest upgrade that dropped a
  header would be seen once, not a hundred times; the rung sees it live.
  The real-wiring tests are kept for good and named in step 1's brief.
  The proxy `CONNECT` keeps the client's default, which no test and no
  rung exercises today or after; the design leaves that line as it is.
- **The deadline's words** reach a user as a job's failure and the
  journal's `error` field. The session's agent-facing choice; listed
  for the owner in §7.
- **A hung connect fails later under one deadline.** If `CONNECT_TIMEOUT`
  retires, a dead network holds a permit 60 s per send instead of 10 s
  before the limiter paces the next as a lost send (C19). §7 gives the
  owner both shapes; a third, a connect bound of the transport's own
  driven by the clock, is a larger design and is not offered.
- **The mock's split is a production refactor in mock mode.** The TCP
  face must answer exactly as before: the client harness's process tests
  and `acq demo` rehearse it, and the journey golden holds the mock's
  words.
- **The mutants scope.** `transport.rs` sits inside the glob and gains
  mutants of its own; the compare runs as t5 round 6 and #151 ran it,
  on the weekly workflow's artifacts (`gh run list --workflow Weekly`:
  41265798977 the t5 baseline, 41278094352 t5's after, 41360883538 on
  `151-stuck-verdict-after`, read on 2026-11-02 at `73f182bd`: paired
  402, timeout to caught 36, timeout to timeout 67, the shards 87 to
  119 minutes). This step's baseline is that run; its after-run is
  dispatched at the close on a tag, and the compare names each of
  #151's ten door-scenario timeouts caught or still hanging.
- **The client harness's servers** stay TCP: they spawn a real `acqd`
  whose mock keeps its listener. Nothing there moves, and nothing there
  gains virtual time; that is the process boundary the retrospective
  ruled the seam ends at.
- **Windows.** The trait is platform-neutral; the production transport's
  real-wiring tests run on the Windows gate as the spec's TCP tests do
  today; the named pipe is untouched.
- **The one mechanism bet**: a transport's hold on a request in flight
  as the hook's accounting in place of occupancy. The spike precedent
  (`4-0-spike.md`) says a count the door keeps is wrong once two park;
  this count is kept by the transport in its first poll and released
  at the answer or at the observed release, so a park between them sees
  the hold. Step 2 measures it before it asserts it, as #151 did: the
  hook's counters printed per scenario under `ACQ_DOOR_COUNTERS`, every
  passing scenario at zero `nothing_parked`; and it pins the boundary
  cases by row: two requests held at once; a release before the first
  poll; a body the fixture holds behind a release after its headers,
  which keeps the clock held and `settle` pending until the release,
  against a body declared silent, which holds nothing and runs to the
  deadline once step 3 lands; a transport future dropped by a cancel,
  its hold gone with it.

## 6. The build, in the order risk is lowest

Each a build brief from `briefs/TEMPLATE.md`, Opus-built, gated,
committed on `main`, reviewed cold, in series; no step needs a later
one, and a retirement waits for its holder.

1. **The seam, no behaviour change**, after C<next> is entered and the
   "Not doing" line removed (§7): the boundary it draws is this step's.
   `Prepared`, `Answer`, the
   `Transport` trait and the reqwest transport in `ratelimit/transport.rs`;
   the four send sites through it; the user-agent onto the prepared
   request; `Secret::authorize` onto a header map; `http` as a direct
   dependency and its types where reqwest's re-exports are named; check
   13 with its breaker cases; the real-wiring tests of §2 that need no
   deadline (the prepared request on the wire, the 3xx, the truncated
   200) in the spec, TCP, on a hand-driven clock; a `GLOSSARY.md` line
   for "transport" and "fixture". The client keeps its user-agent and
   its timeouts in this step; the expiry's test is step 3's. Every existing
   test as it is; the `features` stage proves no feature moved. The one
   constructor the door and the daemon share (`Daemon::builder`,
   `daemon.rs:1200`) gains a transport option, defaulting to the
   production one; `ChokePoint::with_transport` beside
   `with_clock_and_rails`.
2. **The in-process transports and the fixtures**, after C160's first
   amendment is entered (§7). `src/tests/transport.rs`:
   the scripted, account, token, mock, own and refusing transports; the
   six `Server` kinds onto them; the mock's `answer` split out of
   `handle`, the TCP face kept; `browser` reshaped; the hook's occupancy
   read replaced by the transport's hold, the counters renamed; `quiet`'s
   socket half struck and its arrival half reworded; `beside` on the
   first door's clock and hook, registered whole and unregistered at
   `finish`, its second caller moved, and a `c160_` case on its wait;
   the spec's servers inventoried by test, then in-process except the
   kept ones; the component fixtures of `log.rs` and `sessions.rs`
   through the builder's option; `auth.rs`'s two tests on a parking
   clock; the two occupancy exclusions retired on the `c160_` count
   pin; the measurement of §5's last item before the assertion, and its
   boundary rows. The census stays door 91 unless the build finds
   otherwise. No production send changes; the mock split is the one
   production edit, in mock mode.
3. **The deadline onto the clock**, after C160's second amendment is
   entered (§7). One
   absolute instant per send through `sleep_until`, around the answer
   and around the body, through a helper beside `within_on` that takes
   the instant; reqwest's `.timeout()` removed and `CONNECT_TIMEOUT` as
   §7 rules; two rows in the wire table
   (`c158_the_wire_table_through_the_door`): a silent script, the send
   failed at 60 virtual seconds, journaled as a transport failure, and
   the next send on the route held as a lost send (C19); and headers
   answered 20 virtual seconds after dispatch then a silent body, cut
   at dispatch plus 60, the status kept and the outcome a body failure,
   the row refusing a cut at dispatch plus 80 (a fresh budget opened
   for the body); the pacing oracle's new case
   for the first; a component row each for status precedence, the
   permit's release at the cut and the transport future dropped on a
   cancel; the real-wiring test that the expiry closes the connection;
   `TESTING-NOTES.md`'s "Absence" sentence; the retrospective's §3 row
   ("structural, not a choice") corrected in place, dated. The failure's
   words listed at the close. Then the rung.
4. **The close.** The mutants after-run against the baseline named in
   §5, no new survivor in the scope, #151's ten named caught or not;
   the ledger row; the hook's counters per scenario before and after;
   `held_by_send` gone from the output.

Not in this design: the admission loop as a machine (parked, its trigger
named); the login flow as a machine (the step after this one, by the
owner's word of 2026-11-01); the frontends' waits (Root B, 4-full's);
the client harness's process boundary.

## 7. Drafts for the owner

Each is his; nothing below is entered until he rules.

- **C<next> — Every request the daemon sends to the provider crosses
  one transport seam behind the choke point, which prepares it, owns
  its deadline on the daemon's clock and classifies the answer, while a
  transport only carries it; a door scenario's transport is in-process,
  so no provider exchange in a scenario is real I/O.** In production
  the transport is the one HTTP client, owned privately through the
  choke point; what invariant 1 leaves outside stays outside; a
  real-wiring test of it keeps its listener; a test transport holds the
  clock from a request's first poll to its body's end. *Why:* a real
  socket inside a scenario was the one wait the park hook could not
  see. *Pinned:* nothing yet (check 13 of `tools/arch-check.sh` and the
  wire table's expiry rows, when built).

  For `decisions/network.md`, the choke point's file. 781 bytes
  (`wc -c` on the line, 782 with its newline); staged under the next id
  in a copy-backed `decisions/network.md` at the foot of its rulings,
  `python3 -I tools/bothy.py check` passed ("161 decisions in 10 files")
  and `tools/docs-check.sh` passed with its note that the id is uncited,
  as any entry is before its build; the file restored byte for byte
  (`cmp`), the tree clean. The mechanism (the trait, the prepared
  request's fields, the two implementations, the hold in the first poll)
  is `transport.rs`'s doc comment, not the entry's. Entered before step
  1, the step that draws its boundary (`AGENTS.md`, rulings first);
  between that entry and step 3 the deadline it names is reqwest's
  still, which the entry's commit says.
- **C160 amended twice, each before the step that depends on it**, as
  C160 itself was entered ahead of its code. Before step 2: "The hook
  accounts for the clock's sleepers, a send in flight, a blocking task
  and a hold of the scenario's" becomes "The hook accounts for the
  clock's sleepers, a request a test transport holds, a blocking task
  and a hold of the scenario's", and "a `beside` door has no hook and
  keeps the ceiling" goes. Before step 3: "reqwest's timeouts, the one
  timer off the clock, sit inside occupancy" becomes "a send's deadline
  is the choke point's, on its clock (C<next>)". The pins named at each
  close.
- **The "Not doing" line** on virtualising a request timeout's expiry
  (`decisions/daemon.md`): removed in the commit that enters C<next>,
  before step 1, since the two cannot stand together, that commit
  naming C<next> as what holds it (bothy refuses a struck-through line;
  the history is the commit's).
- **`CONNECT_TIMEOUT`, two shapes, his pick:** (a) one deadline on the
  clock, `REQUEST_TIMEOUT` from dispatch over the whole exchange, and
  `CONNECT_TIMEOUT` retired, held by it; or (b) the production transport
  keeps reqwest's connect bound as its own, and C160's exception narrows
  to "the production transport's connect bound". The recommendation is
  (a): one timer, no exception to C160, one constant; its cost is a hung
  connect failing at 60 s rather than 10 s, bounded either way, the next
  send paced as lost under C19.
- **The deadline's words**, agent-facing and the session's under the
  owner's delegation of such design, listed here for his read rather
  than ruled: a send cut before any answer fails as "the send did not
  complete within 60 s", in the journal's `error` field and a job's
  failure; a send cut during its body keeps its status and says "body
  transfer failure: the body did not complete within 60 s" in its
  outcome, `error` null as today; a build may word either otherwise and
  the close lists it.
- **Timing beside the league:** steps 1 and 3 before Phase 3 starts,
  the rung after 3; or both after the run ends; step 2 in either window.
  His machine, his window.

## 8. Raised for review

- Nothing in this design was run beyond the reads, the counts and the
  registry check of §7. The one mechanism bet, the transport's hold in
  place of occupancy, is measured in step 2 before it is asserted (§5).
- Where the two seats of the retrospective want different things: the
  agentic developer wants no real I/O inside a scenario and the timeout
  as a row, which this design gives whole; the agentic user wants the
  production send path left as it was and its failure words stable,
  which steps 1 and 3 touch twice. The design pays the user's side with
  the order of §6, the real-wiring tests and the rung, and with the
  words listed in §7 rather than changed in passing. They agree on one
  sender.
- The `beside` registration (§3) is a small change to the hook made in
  the step that already rewrites the hook's accounting; a reviewer who
  finds it wider than a row names it a step of its own.
- The spec's wire tests are counted as bind sites (12) and not as tests;
  step 2's brief inventories them by test before it moves any, as t5's
  rounds did after a first count overturned; the first cut's `Server::own`
  inventory was that fault in this brief, corrected above.
- `http` as a direct dependency is a mechanism, not a ruling, and adds
  no feature; it is named here because it is the line that lets check
  13 hold `reqwest` to one file.
- #151 stays open on the ten door-scenario timeouts its compare left
  (`73f182bd`); this design claims them for the verdict by mechanism
  and proves none here: the close's compare does, one line each.
- The retrospective's §3 row calls the request-timeout line "structural,
  not a choice"; this design makes it a choice, and step 3 corrects the
  row in place, dated, as the README's rule for dated evidence allows.

## Taken from the reviews

The first review's ten (`transport-seam-design-855ddcf6.md`):

1. The user-agent moved off the client would leave the HTTPS proxy's
   `CONNECT` without it: the client keeps its default from the one
   constant and the prepared request carries it too (§1, §2, §3, §5).
2. The pacing oracle demands a zero wait after a send that landed no
   status, which the choke point paces as lost: the oracle gains the
   case and the row exercises the next send (§3, §4, step 3).
3. The deadline's holder covered the answer and not the body, and step
   1 carried step 3's expiry test: one instant over both phases, two
   expiry rows and the component rows, the expiry's wiring in step 3
   (§2, §3, §6).
4. `beside`'s second caller builds its own clock, and a registration
   of the blocking count alone misses the holds and the slots: the
   first door's clock and hook, the whole accounting registered before
   any restore wait, unregistered at `finish`, the caller moved (§3,
   step 2).
5. Arrival at the call did not retire `quiet`'s precondition, since a
   park on the way to the transport resolves it first: the record in
   the transport's first poll, the socket half struck, the arrival half
   kept (§1, §2, §3, §4).
6. The ruling's bold sentence reached past invariant 1's boundary and
   carried mechanism: scoped to the provider's exchanges, the
   exclusions named, the mechanism after the bold, 797 bytes (§7).
7. The C160 amendments were deferred to the closes against "rulings
   first": each entered before the step that depends on it, the "Not
   doing" line removed with C<next> (§3, §6, §7).
8. "Unchanged" conflated the journal's fields, its words and the
   fixture's evidence: the two phases' field and word behaviour stated,
   the oracle described as journal against the transport's input with
   the bytes held by the kept TCP tests (§2, §4, §7).
9. The occupancy exclusions had a concrete holder already: the `c160_`
   count pin named, its assertions kept (§3, §4, step 2).
10. The `Server::own` inventory counted occurrences: seven calls over
    five binds, the five component fixtures of `log.rs` and
    `sessions.rs` named apart and given their migration (§1, §3, §4,
    step 2).

The second review's five (`transport-seam-design-74423db0.md`), and
the orchestrating session's input of 2026-11-02:

1. The transport's hold ended at the headers where occupancy spans the
   body: the hold runs from the first poll to the body's end, a held
   body holds the clock as it holds the permit, a silent part holds
   nothing (§2, §3, §4, §5).
2. The seam ruling was scheduled before step 3 while step 1 draws its
   boundary: entered before step 1, the "Not doing" line removed with
   it (§3, §6, §7).
3. The second expiry row could pass on a fresh body budget: headers at
   dispatch plus 20, the cut at plus 60, plus 80 refused (step 3).
4. "Under a test it is in-process" took in the kept real-wiring tests,
   and "nothing a scenario starts is real I/O" stood in two places: a
   door scenario's transport is in-process, the real-wiring test keeps
   its listener, the browser's GET named (§2, §7); 781 bytes.
5. One duplicated sentence deleted (§2).
6. #151's compare (`73f182bd`): the ten door-scenario timeouts that the
   fixture-side hold retires, the two component ones that stay, the
   baseline now read (§3, §4, §5, §6, §8).
