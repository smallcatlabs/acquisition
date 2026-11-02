# transport-seam-3 — the deadline onto the clock (C161, C160 as amended twice; step 3 of the design's §6)

The second cut: the first (`db356fd4`) was reviewed cold
(`roadmap/reviews/transport-seam-3-brief-db356fd4.md`, the prompt
beside: four findings), answered as listed at the foot under "Taken
from the reviews". The first cut was written by the orchestrating
session (Opus) on 2026-11-02 from the ruled design (`roadmap/briefs/transport-seam-design.md`,
§2, §3, §5, §6 step 3, §7) and a re-read of the tree at `4f372d1c`,
every line number below from it. Step 2 closed at `a76bc94f`
(`roadmap/LEDGER.md`). C160's second amendment is entered at
`4f372d1c`: "a send's deadline is the choke point's, on its clock
(C161)" in place of "reqwest's timeouts, the one timer off the clock,
sit inside occupancy", as the owner ruled ("Accept as drafted", the
design's §7). The owner picked the connect timeout's shape (a): one
deadline on the clock over the whole exchange, `CONNECT_TIMEOUT`
retired. To be reviewed cold before a worker reads it. Opus builds it
(the owner's word of 2026-11-01, in series). This step changes every
production send, so it runs in the window the owner names (the design's
§7: before Phase 3 starts with the rung after it, or after the league
run), and the rung, his live run, follows it. The last push's CI:
`Gate (Linux)` 41448835564 and `Gate (Windows)` 41404041722 on
`a76bc94f`, both success; re-read at the step's start.

## Goal

When this step is done, a send's deadline is the choke point's, on the
daemon's clock (C161; C160 as amended): one absolute instant per send,
`REQUEST_TIMEOUT` (60 s) from the moment the choke point hands the
prepared request to its transport, races the answer and then the body
through the clock's `sleep_until`. The one reqwest client keeps no
timeout of its own. `CONNECT_TIMEOUT` is retired, held by the one
deadline. A send cut before its headers fails as a transport failure in
the deadline's words, and is counted and paced as a lost send (C19). A
send cut during its body keeps its status, and its outcome is a body
failure in the deadline's words. Either way the permit is released at the
cut and the transport's future is dropped. The wire table pins both cuts
through the door, on virtual time: the expiry is a row, no longer a
real timeout the hook cannot see. The pacing oracle reads a lost send.
One real-wiring test shows the expiry closes the production transport's
connection.

Why it matters: the retrospective's §3 called the request timeout
"structural, not a choice", because reqwest's timers run on tokio's wheel
inside a client no clock reaches. The seam makes it a choice, and this
step makes it: the deadline is the daemon's, visible to the park hook,
pinned by forcing its expiry (C137), and a silent fixture in a scenario
now ends at the deadline instead of hanging to the ceiling.

## Read

- `roadmap/briefs/transport-seam-design.md`: §2 ("the deadline" and its
  shape), §3 (the timeouts row), §5 (a hung connect fails later under
  one deadline), §6 step 3, §7 (the connect timeout's two shapes and
  the owner's (a); "The deadline's words").
- `decisions/network.md` C161 (the seam; the choke point owns the
  deadline), C19 (a send lost in transport is paced as if counted), C21
  (the permit spans a request until its body completes), C129 and C130
  (the verdict and the 3xx, unchanged). `decisions/daemon.md` C160 as
  amended at `4f372d1c` (26), C158 (the wire table through the door),
  C137 (a deadline is pinned by forcing its expiry).
- `crates/acquisition-transport/src/lib.rs`: the doc's 40–53 (the client
  "keeps both timeouts the daemon gives it"; "What comes next"),
  `Production::new` (115–122, its two `Duration`s), `build_client`
  (150–166, `.connect_timeout(connect)` 162 and `.timeout(request)` 163),
  the crate's one test calling `Production::new` with both (217–221).
- `crates/acquisition-daemon/src/ratelimit/choke.rs`: the timeouts'
  doc and constants (220–225; L0 rail 3), `production()` (238–246),
  the four sends: `head` (543; the transport called at 586, its body
  drained at 615), `post_form` (1001; 1018), `get_bearer` (1027; 1043),
  `get` (1062; 1071–1078); `finish_send` (809; the lost-send note
  823–831, the body awaited at 853); `record_completed` (941; the
  outcome's words 954–960, `error` 973), and `sleep_until` (351–354).
- `crates/acquisition-daemon/src/daemon.rs`: `Expired` (1009–1012),
  `within_on` (1014–1033), `Daemon::within` (1581–1588).
  `crates/acquisition-daemon/src/ratelimit.rs`: `Clock` (69–89, its
  `sleep_until` default), `SystemClock::sleep_until` (110–115).
- `crates/acquisition-daemon/src/tests/transport.rs`: the module doc
  (1–30), `Reply` (92–107, `Silent` and `SilentBody`), `InProcess`
  (124–156: the hold taken in the first poll), `answered` (204–235: a
  silent reply lets the hold go), `ScriptedResponse` (285–317),
  `Scripted` (320–332), `scripted` (424–476), its component test of the
  forms (`the_scripted_transport_answers_its_forms_and_records_each_request`,
  835–936).
- `crates/acquisition-daemon/src/daemon/tests/door.rs`: the module doc's
  59–65 (a silent fixture: "nothing cuts the send … the seam's next
  step"), `Server::Script` (84), `build` (673–684, the scripted transport
  on the door's hold).
- `crates/acquisition-daemon/src/daemon/tests/dispatch.rs`: `Row`
  (1163–1176, `script: Vec<ScriptedResponse>` and `virtual_at_most`),
  `journal_projection` (1342–1357, its seven columns), `probe_sent`
  (1423–1425), the headerless 503 row (1946–1988, the model for the
  first expiry row: a policy on the probe, then two fetches), the
  wire twins and the cut-body rows (2114–2172), `c158_the_wire_table_through_the_door`
  (2205; `virtual_at_most`'s check at 2417).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`: the two
  teardowns (718–776; the door's `finish` runs `finish_harness`, so
  every wire-table row meets `assert_pacing_follows_responses`), and
  that oracle (902–974: a send after a counted headerless response may
  wait up to the route's longest period plus the pad; a send after a
  send with no status falls to "nothing demanded a hold", `wait == 0`).
- `crates/acquisition-daemon/src/ratelimit/limiter.rs` `note_lost_send`
  (725–734: a lost send counts only on an established policy).
- `crates/acquisition-daemon/src/ratelimit/tests/spec.rs`: `parked_over`
  (37–44), `answering` and its answer helpers (46–113), the seam table
  `SEAM_TABLE` (2567) and `send_the_seam_table` (2606), the four
  sends, `c161_each_send_is_prepared_by_the_choke_point_and_reaches_the_wire_as_prepared`
  (2667, in-process and on the wire), the real-wiring tests' TCP shape
  (the listener at 2714–2731).
- `crates/acquisition-daemon/src/daemon/tests/responses.rs`
  `http_status_keeps_precedence_over_error_body_transport_failure` (116)
  and `body_read_transport_failure_is_not_recast_as_bad_json` (97).
- `TESTING-NOTES.md` 116–117 ("Absence": "the answer is the client's
  timeouts"). `roadmap/reviews/time-seam-retrospective.md` 148 (the row
  "Keep, and note it is structural, not a choice").
- `REVIEW-CHECKLIST.md`, the network area's rows, the transport row
  step 1 added.

## Input

- The ruling: C161 and C160 as amended at `4f372d1c`; the owner's (a).
  Nothing else to ask him before the code.
- The words, agent-facing and the session's under the owner's delegation
  (the design's §7, listed for his read, not ruled): a send cut before
  any answer fails in the words "the send did not complete within 60 s".
  These go in the journal's `error` field, the send record's outcome
  ("error: …") and a job's failure, which the job wraps as it wraps any
  transport failure today. A send cut during its body keeps its status,
  and its body failure says "the body did not complete within 60 s". The
  outcome then reads "<status>; body transfer failure: the body did not
  complete within 60 s", with `error` null, as `record_completed` writes
  any body failure today. "60 s" is formatted from `REQUEST_TIMEOUT`,
  never a second literal. The build may word either otherwise; its
  commit names the words and the close lists them.
- The two door rows, fixed here from the design:
  1. **Silent before its headers.** A HEAD answered 204 with the
     headerless row's policy (`X-Rate-Limit-Account: 1:10:60`), two
     fetches submitted together, the first GET silent, the second
     answered 200. The first fails at dispatch plus 60 virtual seconds,
     journaled `["GET", "/fetch", null, false, true, 0, null]`, its job
     carrying the deadline's words. The second is held as a lost send
     (C19: `note_lost_send` on the established policy) and lands. Its
     journaled `wait_ms` is the exact value the policy's hold gives,
     measured from the cut on a parking clock, which the build derives
     from the limiter and from where `since` is taken. The build states
     the derivation in its commit message and does not read the value
     off a run alone.
  2. **Headers at 20 virtual seconds, then a silent body.** A HEAD
     answered 204, one fetch, its GET answered by the scripted
     transport's delayed form. The transport sleeps 20 s on the door's
     clock, holding nothing, so the park hook advances to it. It then
     answers a 200's head with GGG's headers and a body that never
     completes. The send is cut at dispatch plus 60, not 80: one
     instant, no fresh budget for the body. It is journaled with its
     status, `["GET", "/fetch", 200, false, true, 0, null]`, its job
     failed in the body's words. The row bounds virtual time so a cut at
     dispatch plus 80 fails it (`virtual_at_most`). If the runner's
     measure of moved time counts a timer after the cut, the row's bound
     says what it reads.
- The component cases, fixed here: each of the four sends cut at
  exactly dispatch plus 60 against a silent test transport, its permit
  released at the cut (the gate's occupancy zero after); a silent body
  after a 500's head keeping the 500 first and naming the body's
  failure (status precedence, the deadline's words in the existing
  precedence case or a row beside it); the send's future dropped
  mid-exchange (as a cancel drops it) releasing the transport's hold
  and the permit.
- The real-wiring case: a TCP listener that reads one request and never
  answers. The production transport sends to it from a choke point on a
  parking clock, advanced to the deadline by the test's hand. The send
  fails in the deadline's words, and the listener's next read returns
  end of file (the connection closed when the future dropped). It is
  bounded by `HANG` alone.
- Counts, measured at `4f372d1c`: `git grep -n
  "REQUEST_TIMEOUT\|CONNECT_TIMEOUT" -- crates` prints four lines, all
  in `choke.rs` (224, 225, 243, 244); `git grep -n "Production::new" --
  crates` prints two calls (`choke.rs:241`, the crate's test at 217)
  and two doc mentions; the scripted transport has six callers outside
  its definition (`git grep -n "scripted(" crates/acquisition-daemon/src`:
  `auth.rs:840`, `door.rs:678`, `harness.rs:382`, three in
  `transport.rs`'s tests).

## Do

1. **The deadline helper and the four sends.**
   - A helper beside `within_on` that takes an absolute instant. It races
     a future against `clock.sleep_until(instant)`, `biased` with the
     future first, the sleep polled `unconstrained`, as `within_on` does
     (C160), and yields `Expired` or the output.
   - It sits where the choke point can call it without `ratelimit/`
     naming the daemon's module. If that means moving it, `within_on` and
     `Expired` to `ratelimit.rs` beside `Clock`, the build moves them and
     expresses `within_on` through it, so there is one race, not two.
   - In each of the four sends, the instant is taken once, `self.now() +
     REQUEST_TIMEOUT`, immediately before the transport is called. It
     bounds the transport's answer, and then, in `head` and in
     `finish_send`, the body's await, under the same instant.
   - In `post_form` and the two GETs, an answer cut becomes `Err` in the
     send's words, which flows through `finish_send` as any transport
     failure does: observed, the lost send noted (counted sends alone,
     823–831), journaled, classified `Network`. In `head` it takes the
     probe's own path, unchanged: an uncounted probe failure ("HEAD
     failed: …"), observed by `observe_probe` and recorded by its own
     `record_completed` call (680–700), no lost send. A body cut becomes
     the body's `Err` in the body's words, the status kept. `finish_send` gains the
     instant, or the body arrives already bounded; the build picks one
     and says which.
   - Nothing else in the send's order moves: admission, the token read
     after admission (C87), the verdict and the record while the permit
     is held (C129), the 401's publication before release.
   - The component cases of Input land here, in the spec over
     `parked_over` and in `responses.rs`. The four sends' case may run the
     seam table over a silent transport.
   - `c160_the_transports_hold_holds_the_clock_through_the_door`
     (`door.rs` 1690) asserts `clock.sleepers() == 1` while its requests
     are held (1789). Every held send now parks its deadline on the same
     clock, so the count becomes the test's own sleep plus one per send
     not yet ended (three, then fewer, in the two-request row; two in the
     held-body row). The test is adapted in this commit to count the
     sends' deadlines, and it keeps its assertions that the hold is seen,
     `settle` stays pending under a hold, and the clock does not move.
   - Every other test that reads the parking clock's queue is checked the
     same way, for a send in flight at the read, whose deadline now joins
     it. That means a sleeper count, a next deadline, or a hand advance to
     the next deadline: `git grep -n "sleepers()\|next_deadline()\|advance_to_next()"
     -- crates/acquisition-daemon/src`, 20 lines at `4f372d1c`. A hand
     advance that could reach a held send's deadline would cut it.
     - Read at this cut and found clear: `dispatch.rs` 191 and 205 (the jobs
       at their rate waits) and `exit.rs` 641 (a job held after its 429).
       `await_a_look`'s callers, `exit.rs` 560, 820 and 1389, wait on the
       drain's look behind a held lock or a staged login, with no send in
       flight. `drive_drain` advances only with nothing in flight.
     - Still to confirm in the build: `advance_until`'s callers
       (`harness.rs` 591–600; called at `dispatch.rs` 410 and 599, `stress.rs` 184) and `dispatch.rs` 2524.
     - The report lists each reader and what was adapted.
2. **The client without timeouts.**
   - `build_client` loses `.connect_timeout` and `.timeout`, and
     `Production::new` loses its two `Duration`s. Its one caller,
     `production()`, and the crate's test follow.
   - `CONNECT_TIMEOUT` is deleted. `REQUEST_TIMEOUT` stays in `choke.rs`
     as the deadline's one constant, its doc saying it is the choke
     point's, on the clock (C161; L0 rail 3's bound).
   - The crate's doc (40–53) says the client keeps no timeout, since the
     deadline is the choke point's on the daemon's clock. Its "What comes
     next" paragraph goes, its subject done.
   - The real-wiring test of Input lands in the spec, beside the kept
     real-wiring tests. A C154 line: retired `CONNECT_TIMEOUT` and the
     client's two timeouts; held by the deadline and that test (the
     production transport's cut), and by the component case of the four
     sends.
   - Every kept TCP test whose exchange reqwest's timeout bounded gains an
     outer ceiling, `HANG` in the daemon's tests and a named ceiling of the
     same value in the transport crate's one test, around the exchange and
     its completion waits. These are the crate's
     `the_production_transport_sends_the_prepared_user_agent_not_its_default`
     (217, awaiting the transport directly) and the spec's six kept
     real-wiring tests (named in `b79e36f8`'s message; the preparation
     table at 2667 sends on a parking clock nobody advances, so the new
     deadline cannot fire there). No elapsed-time assertion is added: the
     ceiling is a hang's bound (C137), and each test's own assertions are
     unchanged.
   - This and Do 1 may be one commit if the tree needs it. Do 1 alone
     leaves reqwest's 60 s racing the clock's.
3. **The scripted transport's delayed form and the two door rows.**
   - The scripted transport gains a delayed form: a step that answers its
     head after a virtual delay slept on the clock the transport is given,
     holding nothing while it sleeps, then a body that never completes.
     The build gives the transport the clock in whatever shape changes
     its six callers least. The form's component test lands beside the
     forms' (835).
   - The wire table lets a row name a silent step and a delayed one,
     changing no existing row's meaning, by the smallest shape that does
     it (`Row::script` and `Server::Script` over `Scripted`, or a field
     beside them).
   - The two rows of Input are added under a comment opening `// C161:`
     and naming C19 for the first.
   - The pacing oracle's new case: a counted send with no status (lost in
     transport) is treated as the headerless case treats a counted
     response without policy headers, so the next send on its route may
     wait up to the route's longest period plus the pad. The case is
     written from C19, not from the row's number.
4. **The words and the records.**
   - `TESTING-NOTES.md` 116–117: "the answer is the client's timeouts"
     becomes the send's deadline on the daemon's clock (C161), whose expiry
     the wire table forces.
   - The door's module doc 59–65: a silent fixture now ends at the
     send's deadline on the door's clock; the hook advances to it, and
     the send fails as the rows pin.
   - `transport.rs`'s module doc: its last sentence on a silent reply,
     which "never completes", becomes one that the choke point's deadline
     cuts.
   - The retrospective's row at 148 is corrected in place, dated
     2026-11-02: the timeout's expiry is the choke point's on the clock
     since this step, a choice the seam made possible, with this brief's
     path.
   - `REVIEW-CHECKLIST.md`'s transport row gains the sentence that no
     timer bounds a send but the choke point's deadline, so a timeout
     added to the client is a finding.
   - "Current step" in `roadmap/README.md` says the build landed and its
     review is pending.

## Done when

Each line from the commits alone, their tree or their messages, except
the last.

- `git grep -n "CONNECT_TIMEOUT\|connect_timeout\|\.timeout(" -- crates`
  prints nothing; `REQUEST_TIMEOUT` is defined once, in `choke.rs`, and
  read by the four sends' deadline; `Production::new` takes the
  user-agent alone.
- Each of the four sends takes one instant immediately before its
  transport call, and bounds the answer and the body by it through the
  one helper on the choke point's clock; the helper and `within_on` are
  one race.
- The component cases exist: the four sends cut at dispatch plus 60
  exactly with the permit released, a silent body after a 500 keeping
  its status, a dropped send releasing the hold and the permit.
- The wire table has the two rows under `// C161:`; the second's bound
  refuses a cut at dispatch plus 80; the pacing oracle reads a lost send
  as C19 says; the scripted transport's delayed form has its component
  test.
- The real-wiring test sees the connection closed at the cut, bounded by
  `HANG` alone.
- `TESTING-NOTES.md`'s "Absence", the door's and `transport.rs`'s module
  docs, the transport crate's doc, the retrospective's row (dated) and
  the checklist row say what this step built; a C154 line names
  `CONNECT_TIMEOUT`'s and the client timeouts' holders.
- The deadline's words are in a commit message, for the close's list; no
  golden or reference changed (`git diff --stat` over the range, empty
  for `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and every golden). The
  deadline's words do reach a user: a job's failure wraps a transport
  error ("GET <url> failed: …", `kinds.rs` 855–856) and the CLI prints a
  failed job's error. The goldens stay unchanged because none of their
  cases runs a send to its expiry; every other failure's words are
  unchanged.
- The gate's exit status per stage on each commit's tree, the breakers'
  last line, and each case of Do 1 to Do 3 seen failing first, are
  execution evidence in the worker's report; the orchestrating session
  re-runs the gate on the step's final tree.

## Not in scope

- The rung: the owner's live run after the step, under the live-run
  skill; never an agent's.
- The seam's close (the design's §6 item 4): the mutants after-run
  against the baseline, #151's open timeouts named caught or not, the
  ledger row. The orchestrating session's, after the rung.
- Any change to admission, pacing arithmetic, the verdict, the 3xx, the
  401's publication, or the journal's fields: the deadline adds an
  error's words and moves no field.
- The login's callback bounds (`within_on`'s callers) and the frontends'
  waits: the helper is shared, and their behaviour is unchanged.
- The client harness's process tests: their `acqd` runs the production
  transport against TCP fixtures, whose sends complete. No silent
  fixture is added there.
- A per-send deadline other than `REQUEST_TIMEOUT`, or a knob for it.

## Rules

`TEMPLATE.md`. What differs:
- This step runs in the main tree on `main`, in a checkout other sessions
  may share. Never push.
- `acq` and `acqd` run only in a mock world (the `mock-session` skill),
  never a daemon against the real provider. Kill only this session's
  pids. Run `cargo build --workspace` before anything that starts a
  daemon.
- Read the gate's exit status on its own line, never through a pipe.
  Run long commands in the foreground where they fit, since an idle
  session does not wake when a background process ends.
- Undo an experiment (a removed deadline, a fresh body budget, a hold
  kept through the delay) from a scratch copy and compare with `cmp`.
- Each commit is gated on its own tree. The step changes every
  production send, so its commits are reviewed cold as one range.
- The deadline's words are the session's agent-facing design, recorded
  in their commit and listed at the close.
- The rules bind the worker's own work, except Do 3's "holding nothing
  while it sleeps", which binds what it builds.

What the brief's writer asks: report the following seen failing first,
in the report, not the history:
- each door row;
- each component case;
- the real-wiring case: with the deadline removed it reaches `HANG`, or
  it is shown failing another way the report names;
- the pacing oracle's new case, against a lost send held longer than its
  ceiling.

## Report

`TEMPLATE.md`, and:
- the gate's statuses per stage for each commit;
- the breakers' last line;
- the first row's `wait_ms` and its derivation;
- each case seen failing and how.

Raised for review, at least:
- any send whose order around the permit had to move;
- any caller of `within_on` whose behaviour the shared race changed;
- any test that waited on reqwest's timeout and now needed a change;
- any word a test asserted that changed.

## Taken from the reviews

The first review's four (`transport-seam-3-brief-db356fd4.md`):

1. A door test counted one sleeper while sends were held, which the
   deadlines break: adapted in Do 1, and every test reading the parking
   clock's queue checked for a send in flight, the ones read at this cut
   listed (Do 1).
2. The kept TCP tests lose their hang bound with reqwest's timeouts:
   an outer ceiling around each, no elapsed-time assertion (Do 2).
3. A cut HEAD does not pass through `finish_send`: the probe's own path
   kept, uncounted, no lost send (Do 1).
4. The deadline's words do reach a user through a job's failure: the
   goldens stay unchanged because none runs a send to its expiry (Done
   when).
