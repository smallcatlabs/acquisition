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
briefs of §6 follow his word, one at a time, each reviewed cold.

## 1. What the tree says

### The HTTP client and the sockets

The daemon's production sources touch one HTTP client and three
listeners.

- **The one client** is `ChokePoint::http` (`choke.rs:201`), built by
  `build_client` (230 to 244) with the user-agent on the client, no
  redirect (C130), a connect timeout of 10 s and a whole-request timeout
  of 60 s (`CONNECT_TIMEOUT`, `REQUEST_TIMEOUT`, 216 and 217; no test
  names either: `grep -rn "CONNECT_TIMEOUT\|REQUEST_TIMEOUT"
  crates/acquisition-daemon/src` finds the definitions and the builder
  alone). Four sites send through it (`grep -n "self\.http\|\.post(url)"`):
  the probe's HEAD (564), the token POST (`post_form`, 1000), the bearer
  GET (`get_bearer`, 1028) and the unauthenticated GET of the mock-only
  kinds (`get`, 1057, the one caller `kinds.rs:817`). The word `reqwest`
  is on 12 lines of `choke.rs` (`grep -c`) and leaks as types elsewhere
  (`grep -rn reqwest crates/ --include='*.rs'`): `CompletedResponse::status`
  (109) and `kinds.rs` 1014 and 1044 name `reqwest::StatusCode`;
  `policy.rs` parses `reqwest::header::HeaderMap` at six sites (111,
  338, 427, 523, 524, 590); `auth.rs:63` attaches the bearer to a
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
connect.

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
early. Each of the four is a rule about a socket. Remove the socket and
each becomes structure or goes.

## 2. The reframing: the wire is a value

One sentence: **the choke point prepares a request, owns its deadline on
the daemon's clock and classifies the answer; a transport only carries
it; in production the transport is the one reqwest client, and under a
test it is in-process, so nothing a scenario starts is real I/O.**

The seam sits inside `choke.rs`, between the admission loops and the
four send sites, at exactly the line where `self.http` is named today.

- **A prepared request** is a value: the method (GET, HEAD or POST), the
  URL, the user-agent, the bearer as a `Secret` (never a string) and the
  form for a POST. The choke point builds it after admission and after
  C128's origin check, as it builds the reqwest request today; the
  user-agent moves from the client to the prepared request, so the one
  place every send is shaped carries invariant 4 too. `Secret::authorize`
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
  today less the user-agent and the timeouts, still private, still
  following no redirect, still honouring the proxy variables (C150). It
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
  body (`within_on`, `daemon.rs:1023`, is the shape). An expiry on the
  answer is a transport failure; an expiry on the body is a body failure
  under the status that landed, which is what reqwest's timeout gives
  today (D8: a non-2xx keeps its status, a 2xx with a failed body is
  `Network`). Dropping the transport's future on expiry closes its
  connection, which is what reqwest does for its own timer. `CONNECT_TIMEOUT`
  is the owner's call (§7): one deadline, or the production transport
  keeping a connect bound of its own.
- **The test transports** are in-process implementations of the same
  trait, in the crate's test support beside the parking clock
  (`src/tests/transport.rs`, visible to the daemon's tests and the spec
  alike): a scripted one (the `ScriptedResponse` list, the raw form
  kept, recording `Received` as today), the account server's logic, the
  two token servers', the stateful mock's face, a scenario's own closure,
  and one that refuses every send as connection refused (`Server::None`).
  A request's arrival at a fixture is the call itself: the transport
  records it in the daemon task's own poll, before any park. **A request
  in flight at an in-process fixture holds the clock until it is
  answered or released**, by the transport, not the fixture's author: so
  a held request (the account server's, the delayed token server's) is a
  hold the hook sees without a count of the gate's, and a script that
  declares a response silent takes no hold, so the clock runs to the
  choke point's deadline and the timeout's expiry is a row in virtual
  time.
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
and grant are read from the prepared request. What changes: the
`error` words of a send that never landed are the daemon's own
("the send did not complete within 60 s") where today they are reqwest's
("operation timed out"), and a job that fails on one carries those
words; that is a word a user meets and is listed for the owner. What is
no longer asserted a hundred times: that reqwest sends the prepared
request as prepared, which the mock's user-agent check held on every
mock-facing test. It is held instead by the production transport's own
real-wiring tests (C157's word): a few TCP tests in the spec on a
hand-driven clock, that a prepared GET, HEAD and POST reach a listener
with their method, path, user-agent, bearer header and form body as
prepared; that a 3xx is not followed; that a truncated 200 is a body
failure; and that the deadline's expiry closes the connection. The live
rung is the rest.

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
| the one client (201, `build_client` 230) | user-agent, no redirect, two timeouts | the production transport's, no user-agent, no timeouts, no redirect, proxies as reqwest's default | `build_client`'s user-agent line; held by the prepared request's field and a real-wiring test |
| the user-agent (invariant 4) | on the client; the mock answers 400 without it | on the prepared request; the mock's `answer` keeps the check on both faces; one real-wiring test that it reaches the wire | nothing |
| the bearer (`Secret::authorize`, `auth.rs:63`) | into a `RequestBuilder` | into the prepared request's header map, sensitive, in `auth.rs` | nothing |
| `reqwest::StatusCode` and `HeaderMap` (`choke.rs:109`, `kinds.rs` 1014 and 1044, `policy.rs` six sites) | reqwest's re-exports | `http`'s, named as such | nothing |
| the timeouts (216, 217) | reqwest's, on tokio's wheel, off the clock | the choke point's absolute deadline on `Clock::sleep_until`; `CONNECT_TIMEOUT` per §7 | reqwest's `.timeout()`; held by the deadline and the wire table's silent row; C160's exception sentence; held by the amended entry |
| the scripted server (`harness.rs:373`) | one TCP connection per response | the scripted transport, the same list, the raw form kept, a silent form added, `Received` as today | 1 listener; held by the transport |
| the account server (1290) and the two token servers (445, 1138) | TCP, barriers | the same logic behind the transport; a held request holds the clock | 3 listeners; held by the transport's hold |
| `Server::own` (11 sites: `bearer.rs` 66, 259, 604; `log.rs` 21, 216, 371; `sessions.rs` 105, 488; `exit.rs` 702; `dispatch.rs` 25) | a listener the scenario binds inside the runtime | a transport the scenario builds, its barriers its own | 10 listeners (`login.rs`'s two are the callback's and stay); held by the closure form |
| `Server::None` (14 sites) | a port nothing listens on | the refusing transport | the base `http://127.0.0.1:1`; held by the refusing transport |
| the stateful mock (`mockggg.rs`) | one TCP face | `answer` plus the TCP wrapper; the listener kept for the shipped daemon; the in-process face for `Server::Mock` | nothing |
| the spec's servers (12 binds) | raw TCP responses | in-process raw answers, except the real-wiring tests named in §2, which stay TCP on a hand-driven clock | 8 or so listeners (the build counts); held by the in-process raw form and the kept tests |
| `auth.rs`'s two token tests (796, 834, on `SystemClock`) | TCP, `ChokePoint::new()` | in-process on a parking clock through `with_transport` | 2 listeners and the spec's last system clocks; held by the parking clock |
| the hook's occupancy read (`door.rs` 276) | a send in flight holds the clock | the transport's hold on a request in flight; occupancy read by nothing under the door | `held_by_send` and the two `occupancy -> (1, _)` exclusions in `.cargo/mutants.toml`; held by `held_by_scenario` counting the transport's holds and by `daemon status`'s own read (`server.rs` 405, 446), whose pin the close's mutants compare names or finds wanting |
| `hold_clock` (886) | three sites | two reasons left: the browser's GET to the daemon's callback (`browser` and `exit.rs:736`'s hand-rolled one) and a scenario's explicit freeze (`dispatch.rs:195`) | "a request on its way to the scenario's own server" as a reason; held by the transport's arrival at the call |
| `quiet` (835) | a precondition sentence | the same wait, no precondition: an arrival is the call | the precondition; held by the transport's arrival at the call |
| `settle` (810) | unchanged | unchanged | nothing |
| `beside` (604) | no hook; `HANG` only | registers its daemon's blocking count and its wait slot with the running hook (one clock, one runtime); its waits gain the verdict, `HANG` stays the backstop | "a `beside` door has no hook" in C160 and the module doc; held by the registration and a `c160_` case on a beside wait |
| the "Not doing" line on a request timeout's expiry | structural | retired at §6 step 3's close | the line; held by the silent row and C<next> |
| the login's exchange and refresh | through `post_form` | unchanged | nothing |
| the callback listener (`login.rs:23`) | TCP, on the clock | unchanged; outside invariant 1 (C130); its two component tests stay | nothing |
| the client harness's servers | none; `acqd` spawned in mock mode over its TCP mock | unchanged | nothing |
| `daemon status`'s occupancy | `actual_send_occupancy` | unchanged | nothing |
| the journal, the oracles, `SendReport` | as `TESTING-NOTES.md` holds them | unchanged | nothing |
| the mutants scope | `ratelimit.rs`, `ratelimit/*.rs`, `gate.rs` | `transport.rs` inside the glob; the test transports outside it, as test support | the two occupancy exclusions (above) |
| `TESTING-NOTES.md`, "Absence" | "the answer is the client's timeouts" | "the answer is the choke point's deadline" | nothing |

## 4. What it retires, and what it must not change

In C154's form, each at the step that holds it:

- retired `build_client`'s user-agent and timeouts; held by the prepared
  request's user-agent field with one real-wiring test, and by the
  choke point's deadline with the wire table's silent row.
- retired the fixtures' TCP listeners in the daemon's tests (the four of
  the harness, the ten `Server::own` sites, the spec's raw-response
  helper and its callers, `auth.rs`'s two); held by the in-process
  transports and the oracles unchanged over them.
- retired the hook's occupancy read and its `held_by_send` count; held
  by the transport's hold on a request in flight, counted as the
  scenario's.
- retired the two `occupancy -> (1, _)` exclusions; held by nothing the
  hook needs, and the close's mutants compare says whether `daemon
  status`'s pin catches them or the lines return as missed.
- retired `quiet`'s precondition and the socket reason of `hold_clock`;
  held by an arrival that is the call.
- retired the "Not doing" line on a request timeout's expiry and C160's
  "the one timer off the clock"; held by the deadline on the clock and
  the silent row.
- retired "a `beside` door has no hook"; held by the registration.
- retired `CONNECT_TIMEOUT`, if §7 rules so; held by the one deadline.

What must not change, and what pins it:

- **The journal's lines**: every field of `SendReport`, `wait_ms`
  measured from ready to dispatch, the header's clock, one line per
  completed exchange under the permit. Pinned by the oracles and the
  live-run verifiers (`tools/tracer-verify.py`, `tools/persist-verify.py`),
  which read the journal and nothing of the transport.
- **The wire oracle**: journal equals what the fixture received, in
  order, by method. The fixture is the transport now; the comparison is
  the same function.
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
- **Invariants 4 and 5**: the user-agent on every prepared request; the
  token's text read in `auth.rs` alone.
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
- **The deadline's words** reach a user as a job's failure and the
  journal's `error` field. The session's agent-facing choice; listed
  for the owner in §7.
- **A hung connect fails later under one deadline.** If `CONNECT_TIMEOUT`
  retires, a dead network holds a permit 60 s per send instead of 10 s
  before the limiter paces the next as a lost send (C19). §7 gives the
  owner both shapes.
- **The mock's split is a production refactor in mock mode.** The TCP
  face must answer exactly as before: the client harness's process tests
  and `acq demo` rehearse it, and the journey golden holds the mock's
  words.
- **The mutants scope.** `transport.rs` sits inside the glob and gains
  mutants of its own; the compare runs as t5 round 6 and #151 ran it,
  on the weekly workflow's artifacts (`gh run list --workflow Weekly`:
  41265798977 the t5 baseline, 41278094352 t5's after, 41360883538 on
  `151-stuck-verdict-after`, in progress at this read, 1 h 27 min).
  This step's baseline is the `151-stuck-verdict-after` run once #151's
  compare has read it; its after-run is dispatched at the close on a tag.
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
  this count is kept by the transport at the call and released at the
  answer, in the daemon task's poll, so a park between them sees the
  hold. Step 2 measures it before it asserts it, as #151 did: the
  hook's counters printed per scenario under `ACQ_DOOR_COUNTERS`, every
  passing scenario at zero `nothing_parked`.

## 6. The build, in the order risk is lowest

Each a build brief from `briefs/TEMPLATE.md`, Opus-built, gated,
committed on `main`, reviewed cold, in series; no step needs a later
one, and a retirement waits for its holder.

1. **The seam, no behaviour change.** `Prepared`, `Answer`, the
   `Transport` trait and the reqwest transport in `ratelimit/transport.rs`;
   the four send sites through it; the user-agent onto the prepared
   request; `Secret::authorize` onto a header map; `http` as a direct
   dependency and its types where reqwest's re-exports are named; check
   13 with its breaker cases; the real-wiring tests of §2 in the spec,
   TCP, on a hand-driven clock; a `GLOSSARY.md` line for "transport" and
   "fixture". The client keeps its timeouts in this step. Every existing
   test as it is; the `features` stage proves no feature moved. The one
   constructor the door and the daemon share (`Daemon::builder`,
   `daemon.rs:1200`) gains a transport option, defaulting to the
   production one; `ChokePoint::with_transport` beside
   `with_clock_and_rails`.
2. **The in-process transports and the fixtures.** `src/tests/transport.rs`:
   the scripted, account, token, mock, own and refusing transports; the
   six `Server` kinds onto them; the mock's `answer` split out of
   `handle`, the TCP face kept; `browser` reshaped; the hook's occupancy
   read replaced by the transport's hold, the counters renamed; `quiet`'s
   precondition struck; `beside` registered with the hook and a `c160_`
   case on its wait; the spec's servers in-process except the kept ones;
   `auth.rs`'s two tests on a parking clock; the two occupancy exclusions
   retired; the measurement of §5's last item before the assertion. The
   census stays door 91 unless the build finds otherwise. No production
   send changes; the mock split is the one production edit, in mock mode.
3. **The deadline onto the clock.** One absolute deadline per send
   through `sleep_until`, around the answer and around the body; reqwest's
   `.timeout()` removed and `CONNECT_TIMEOUT` as §7 rules; the silent
   form's row in the wire table (`c158_the_wire_table_through_the_door`):
   a silent script, the send failed at 60 virtual seconds, journaled as
   a transport failure, paced as a lost send (C19); the real-wiring test
   that the expiry closes the connection; C160 amended and the "Not
   doing" line retired at the close; `TESTING-NOTES.md`'s "Absence"
   sentence; the retrospective's §3 row ("structural, not a choice")
   corrected in place, dated. The failure's words listed at the close.
   Then the rung.
4. **The close.** The mutants after-run against the baseline named in
   §5, no new survivor in the scope; the ledger row; the hook's counters
   per scenario before and after; `held_by_send` gone from the output.

Not in this design: the admission loop as a machine (parked, its trigger
named); the login flow as a machine (the step after this one, by the
owner's word of 2026-11-01); the frontends' waits (Root B, 4-full's);
the client harness's process boundary.

## 7. Drafts for the owner

Each is his; nothing below is entered until he rules.

- **C<next> — Every send the daemon makes crosses one transport
  seam behind the choke point, which prepares the request, owns its
  deadline on the daemon's clock and classifies the answer; a transport
  only carries it; the production transport is the workspace's one HTTP
  client, privately owned, and a test's is in-process, so nothing a
  scenario starts is real I/O.** A prepared request carries method, URL,
  user-agent, bearer and form; an answer is a status, headers and a body
  completed under the same permit; a fixture that holds a request holds
  the clock. *Why:* a real socket inside a scenario was the one wait the
  park hook could not see, and three rules stood where structure
  belongs. *Pinned:* nothing yet (check 13 of `tools/arch-check.sh` and
  the wire table's silent row, when built).

  For `decisions/network.md`, the choke point's file. 792 bytes
  (`wc -c` on the line, 793 with its newline); staged in a copy of the
  file at the foot of its rulings, `python3 -I tools/bothy.py check`
  passed ("161 decisions in 10 files") and `tools/docs-check.sh` passed
  with its note that C<next> is uncited, as any entry is before its build;
  the file restored byte for byte (`cmp`), the tree clean. The mechanism
  (the trait, the two implementations, the hold at the call) is
  `transport.rs`'s doc comment, not the entry's.
- **C160 amended**, two sentences: "The hook accounts for the clock's
  sleepers, a send in flight, a blocking task and a hold of the
  scenario's" becomes "The hook accounts for the clock's sleepers, a
  request a transport holds, a blocking task and a hold of the
  scenario's"; "reqwest's timeouts, the one timer off the clock, sit
  inside occupancy" becomes "no timer is off the clock: a send's
  deadline is the choke point's (C<next>)"; and "a `beside` door has no hook
  and keeps the ceiling" goes, at step 2's close.
- **The "Not doing" line** on virtualising a request timeout's expiry
  (`decisions/daemon.md`): removed at step 3's close, the commit naming
  C<next> as what holds it (bothy refuses a struck-through line; the
  history is the commit's).
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
  than ruled: a send cut by the deadline fails as "the send did not
  complete within 60 s", in the journal's `error` field and a job's
  failure; a build may word it otherwise and the close lists it.
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
  rounds did after a first count overturned.
- `http` as a direct dependency is a mechanism, not a ruling, and adds
  no feature; it is named here because it is the line that lets check
  13 hold `reqwest` to one file.
- The weekly mutants run on `151-stuck-verdict-after` was in progress at
  this read; this step's baseline waits on #151's compare of it.
- The retrospective's §3 row calls the request-timeout line "structural,
  not a choice"; this design makes it a choice, and step 3 corrects the
  row in place, dated, as the README's rule for dated evidence allows.
