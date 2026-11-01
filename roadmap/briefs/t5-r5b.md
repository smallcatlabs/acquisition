# t5-r5b — the queue and the rails as scenarios (#141, round 5b)

The second of round 5's three parts (`t5-design.md` §4 round 5 as
split on 2026-10-31 and amended at this brief's commit: 5b is
`queue.rs` and `rails.rs`, 37 door-able tests of 46; the nine
door-able tests of the other files join 5c; ruled 2026-10-31, C158).
Cut twice: the first draft (`5c301c06`) was reviewed cold
(`roadmap/reviews/t5-r5b-brief-5c301c06.md`, the prompt beside); this
cut answers its twelve findings, and a third cut the second review's
four, each named at the foot, "Taken from the reviews". Written from
two read-only inventories at `c03a11c5`
(`queue.rs` and the admission, queue-file, restore and cancel paths;
`rails.rs` and the rails), every line printed, neither file touched
since; the door's lines at `72ca3330` (round 5a's close); and a read
of the fixtures each recipe names. Rounds 1 to 5a closed
(`LEDGER.md`).

## Goal

When this round is done, admission is one table through the door: a
row is a provider mode, a session, a kind, params and a submitter, and
what the row asserts is the refusal's kind and words, nothing admitted
(`List` empty; on the mock rows, the next submit answered id 1) and
nothing on the wire.
A row that never passed the client's door (a kind or params the
admission would refuse) is seeded on disk, restored as a lifetime
restores its queue, and fails in the door's words with nothing sent.
The queue file failing is rows too: an operation after the break
answered `QueueFailed` with its words. The restore scenarios (a lost
login's fetch, the restore's own table) and the queue's cancel and
priority answers read `List`, `Status`, `Result` and the store's
persisted view. The rails' tripwire, ceiling and halts are scenarios
through the door: a halt's bare cause through `DaemonStatus.rails`,
its full words through a quote or the job's failure, a parked job
through `Status`, "nothing picked" through `settle`, the disk through
the store's observer read once `run` has returned (C45: an observer
reads the queue from the ledger with no daemon running). The door
gains a real-named mode on the fixture's base and the harness a live
session; `Token::Answering`'s former callers take
scripted token answers. The tests that need a function, a fabricated
state, a failed restore or three lifetimes in one test say so and stay
component tests. No production file changes; no word a user meets
changes.

## Read

- `roadmap/briefs/t5-design.md` §2, §4 round 5 (the dated lines);
  `t5-r5a.md` Do 1 (`restore()`, `Server::own`, `received()`, the
  token answers in the harness) and its foot; `t5-r3.md` Do 1 (the
  wire table's `Row` and `Step`, which this round does not widen: the
  admission table and the rails scenarios are their own shapes);
  `t5-r4.md` Do 1 (`quiet`, `submit`, `held`, `sent`).
- `decisions/daemon.md`: C158; C6 (the restore), C45 (an observer reads
  the queue from the ledger on disk through the store's `Persisted`
  view: the scenario's disk read), C50 (a lost login's fetch fails at
  restore), C133, C136 (the `daemon` submitter refused), C144;
  `decisions/network.md`: C25 (a halt leaves queued jobs waiting), C129
  (a Cloudflare answer is a stop), C24; `AGENTS.md`: C12, C153, C154.
- The admission path (`crates/acquisition-daemon/src/daemon/server.rs`
  260–288: `refuse_while_stopping` 519, `admit_client` (`daemon/queue.rs`
  1204–1232: the kind table, `probe` daemon-only 1214, mock-only kinds
  in real mode 1220, `admit_segments` 913, the row's own check),
  `admit_client_submitter` 1183 (`DAEMON_SUBMITTER` 1177), the restore
  wait at 273–275 only when an account is named or the kind needs one
  (`kind_needs_account`, `daemon.rs:1668`: false for `sleep`, and for
  `fetch` in mock mode), `resolve_account`, `submit_and_mark` 447–467
  (`validate_apply`, `kinds.rs:930`, the action bound 943;
  `admit_realm` 731; `admit_params_size` 1126; `admit_submitter` 1161),
  `submit_marked` 518–582 (the failed queue refused at 528–530,
  `next_id` 548, the persist and its rollback 569–573)); the kinds of
  refusal: `Refused`, `NotLoggedIn` (the selector), `QueueFailed`,
  `UnknownJob` (`no_job`), `WrongState` (595, 601, 686, 692;
  `server.rs:311`).
- The queue file: `persist` (`queue.rs` 76–96, under `shared`, a sticky
  failure); the dispatcher stops picking on a failed queue
  (`daemon/dispatch.rs` 81–83) and a transition whose write fails
  reverts (233–240); `stored_outcome`, `stored_job`, `job_list`
  (363, 377, 397: `QueueFailed`, never "no job"); `restore_jobs`
  (163–358: the prune by retention 167, a failed read fatal 171–176,
  probes dropped 211–214, a fault marker's job failed 229–243, C50
  244–256, the running rows' fates 257–303, `next_id` and `first_id`
  313–315, the terminal trees evicted 316–318); `Entry::from_row`
  (`daemon.rs:364`, no admission checks: a restored row reaches the
  runner unadmitted); `evict_tree` (`dispatch.rs` 490–523); the
  dispatcher's restore gate (`dispatch.rs` 73–77: nothing picked until
  `restore_landed`). The cancel (`queue.rs` 584–664, `server.rs:326`)
  and `set_priority` (680–706, `server.rs:330`). No queue-file I/O runs
  on the counted blocking thread (it runs under `shared` on the
  runtime); the `sleep` kind runs on real time (`kinds.rs` 563–577:
  200 ms slices, 3.0 s by default), invisible to the park hook.
- The rails (`crates/acquisition-daemon/src/rails.rs`): `RailsConfig`
  (95–110: `tripwire`, `max_sends`, `journal_path`, `state_path`); what
  persists (796–803: the trip, the block mark, `refresh_failed`), the
  lenient load (425–446); `halted()`'s full words (517–542: the block,
  "halted by live-test rails: {trip}", the unreachable with its remedy,
  the ceiling, "which a reset does not lift"); the ceiling on the count
  (558–571, 641–653: "ceiling: {n} of {max} sends used this daemon
  lifetime"; zero allows no send); `record` (579–660: counts every
  method, the marks, the trip on 429/403/503/401, the ceiling); the
  reset (698–712: writes first, a failed write the error, every halt
  standing); `status()` (747–783: `halted` the **bare cause**, not
  `halted()`'s words; `refresh_failed`, `sends`, `max_sends`); the
  failed write (788–794, 805–815, "RAILS STATE NOT WRITTEN"); the trip's
  announcement (`daemon.rs` 1594–1615: "SENDS STOPPED" for a block,
  "LIVE-TEST RAILS TRIPPED" else, the ceiling "for the rest of this
  daemon's lifetime"); the refused reset (`server.rs` 491–502:
  `Internal`, "the halt stands, for the reset did not reach disk"); C25
  at the pick (`dispatch.rs` 85, 103–105), after a refused send
  (286–315: `Exec::Parked`, back to Waiting, no retry spent; a cancel
  asked concludes Cancelled). Who carries the full halt words across
  the protocol: a job's failure (`kinds.rs:891`), a token refusal
  (`auth.rs:307`), `Quote.halted` (`quote.rs:181`; `protocol.rs` 344–355,
  the request at 480, an empty `jobs` with the account omitted on one
  session, `quote.rs` 25–33). The rails' state and reset writes run
  inline on the runtime under the rails mutex; the rails have no timer.
- The store's read API, `crates/acquisition-store/src/jobs.rs`:
  `JobDb::open` (150), `get` (226), `read_job` (450), `read` (513,
  `Persisted`): a scenario reads the queue on disk as `acq status`
  does with no daemon (C45), never through `shared`.
- `crates/acquisition-daemon/src/daemon/tests/door.rs` at `72ca3330`:
  `Server` (65–84: `Script`, `Account`, `Token(Delayed)`, `Mock`,
  `Own(OwnServer)` 82, `None`), `Server::own` (96); `DoorBuilder`
  (`rails` 356, `jobs_db` 362, `sessions` 368, `restore_landed` 374,
  `restore` 389, `restore_jobs` 400, `store_dir` 408, `world` 420,
  `life` 436, `run` 452); the provider fixed to `Provider::mock(&base)`
  (561); `Door` (`request`, `submit` (kind and account only, empty
  params, submitter "test"), `spawn_request` 715, `hold_clock` 726,
  `stop` 737, `await_job`, `await_terminal`, `await_auth`, `settle`,
  `quiet`, `journal` 876, `journal_text` 883, `advance` 889, `fixture`
  903, `held` 911, `sent` 930, `received` 943, `journal_methods` 952,
  `daemon` 991, `finish`); `beside` (concurrent, no hook of its own;
  forbids a second daemon that sends).
- `crates/acquisition-daemon/src/daemon/tests/harness.rs` at
  `72ca3330`: `token_answer` (546), `TOKEN_HEADERS` (563), `Received`
  (570), `scripted_server_received` (599), `HANG` (871), `tripwire_
  config` (1285), `token_server_answering` (1292: after this round its
  one caller is `dispatch.rs:428`), `logged_in_sessions` (1341, a
  refresh token only), `account_server` (1399), `MANUAL_WALL_ORIGIN`,
  `ParkingClock`.
- `crates/acquisition-daemon/src/daemon/tests/queue.rs`, whole (1770
  lines at `c03a11c5`; 30 `#[tokio::test]` and three `#[test]`; 16
  `ManualClock` lines; 14 `shared.lock()`; nine `handle_request` call
  sites and 17 `client_submit` calls through 77; 13 internal `submit`
  calls; five 50-yield loops at 51, 186, 537, 709, 1097 "proving
  absence"; one real sleep at 1713; `from_secs(60)` at 1677 a ceiling
  on the login's internal wait; `from_secs(3600)` at 117 a datum). Its
  helpers: `assert_nothing_admitted` (63: `jobs.is_empty()` and
  `next_id == 1`; 15 calls), `client_submit` (70), `assert_refused`
  (92, reads a `Response`, stays), `live_daemon` (109: a session with
  an access token an hour from the wall, written privately at
  111–118), `persisting_daemon` (1168: a file `JobDb` through
  `Arc::get_mut`, then `restore_jobs`), `break_queue` (1182: drops the
  jobs table of the db at a path), `disk_row` (1517). The tests, by
  line and class (the inventory's): door-able 19, 129, 200, 285, 313,
  359, 395, 467, 522, 552, 689, 724, 769, 844, 872, 986, 1077, 1127,
  1310, 1369, 1544, 1728, 1751; component 250, 590, 823 (`#[test]`
  tables), 1190 (three lifetimes, a fabricated Running), 1465 (the id
  rollback and "not picked" have no boundary read; its refusal is a
  row), 1497 (a conclusion fabricated after the break), 1591 (memory
  membership), 1668 (the login's internal wait), 1686 (a failed restore,
  which the door cannot answer), 1696 (see Do 4).
- `crates/acquisition-daemon/src/daemon/tests/rails.rs`, whole (692
  lines at `c03a11c5`; 13 `#[tokio::test]`; `ceiling_daemon` 186 (six
  users; `logged_in`, a file `JobDb` through `Arc::get_mut` at 196),
  `assert_parked` (221: `held()` and `JobDb::open` on disk),
  `wait_until` (11–23, a 3 s yield poll over `shared`),
  `wait_halted_and_settled` (203–217, a 3 s yield poll over "no job
  Running" and `halted()`); nine 100 ms sleeps (60, 126, 258, 312, 345,
  386, 535, 572, 614), every one "the daemon has done all it can and
  nothing more was sent, failed or re-picked"; the 3 s ceilings at 12,
  204, 378, 618; `token_server_answering` at 141, 244, 286, 332;
  `account_server` at 519, 588; private calls `reset_rails` (80, 409),
  `valid_access_token` (164, 437, 457), `rails.record` (147),
  `mark_refresh_failed` (677), `pick_runnable` (270, 317, 574, 632),
  `endpoint_state` (404), `halted()` (11 calls)). The tests, by line:
  26, 94, 140, 242, 284, 330, 361, 423, 474, 517, 552, 586, 647, all
  door-able (Do 5).

## Input

Fixed before the round: the two inventories' classifications and
counts (Read, Do 2 to Do 5); the door as round 5a left it. The gate's
test stage at `72ca3330`: 23 to 24 s on the orchestrating session's
machine (round 5a's close). The last push's CI on `72ca3330`: `Gate
(Linux)` and `Gate (Windows)`, in progress at this brief's commit, read
at the round's close. Round 0's baseline (run 41265798977 on the tag
`t5-round0-baseline`): three shards in progress and one failed at this
commit (a shard fails on a mutant the tests missed; its artifact is
uploaded whatever the outcome): round 6's input, read then.

## Do

1. **The door's and the harness's additions**, each with its rule in
   its doc:
   - `DoorBuilder::real_named()`: the provider built on the fixture's
     base as today, with the real provider's name (`Provider { name:
     GGG, ..Provider::mock(&base) }`, as `queue.rs` 692–695 builds it,
     `is_real()` true), for the rows that need the real-named mode
     (689, 769). Never `Provider::ggg()` (`provider.rs` 61–69), which
     carries the production URLs: a test daemon on it would send to
     GGG, and the fixture's empty record would prove nothing.
   - `live_sessions(clock: &ParkingClock) -> Sessions` in the harness:
     one session with an access token expiring an hour past the
     clock's wall, as `live_daemon` wrote it privately (`queue.rs`
     111–118); `logged_in_sessions()` stays for the refresh-due shape.
   - `Door::submit` takes what `Request::Submit` carries: a
     `submit_with(kind, params, priority, submitted_by, account)`
     beside the two-argument form, or the scenarios send
     `Request::Submit { .. }` through `request`; the worker chooses
     one shape and names it.
   - A quote through `request(Request::Quote { jobs: vec![], account:
     None })`: `Response::Quote { quote }` with `quote.halted`, the
     halt's full words (`halted()`'s), on a daemon with one session.
     No new door call.
   - A scenario's disk read is C45's observer read, after
     `DoorBuilder::run` has returned (`finish` asks the dispatcher to
     abort without awaiting it, `harness.rs` 1023–1024, and the hook
     holds the daemon until the scenario returns, `door.rs` 462–474:
     only once `run` is back is no daemon answering):
     `acquisition_store::jobs::Persisted::read_job(dir, id)` or
     `Persisted::read(dir)` (`jobs.rs` 450, 513) on a directory the
     test owns outside the closure, whose daemon db the scenario
     opened for `.jobs_db` (the path the observer reads,
     `daemon_db_path`, `jobs.rs:113`), never while the daemon answers
     and never through `shared`; a halted or parked daemon's rows are
     the same after the run as before it. A row the scenario writes
     before the door opens, or behind the daemon (1544's rows 40 and
     41, which model a row the daemon never admitted), is a persisted
     input the scenario stages, as `break_queue` stages a broken file:
     a write, not a read. `break_queue` stays in `queue.rs`.
   - "Nothing picked" is `settle` resolving: a re-pick spin never
     parks, so the park at which nothing can run is the boundary's word
     that the dispatcher picked nothing; `pick_runnable`'s four reads
     go that way (round 3 kept one such read component for a blocked
     probe, `dispatch.rs:468`; this round's rule covers the rails
     scenarios, whose "pick empty" follows a halt).
   - `Token::Answering`'s former callers take `Script` with
     `token_answer` (rails 244, 286, 332: `["200 OK"]`) or a fixture
     that records and answers nothing (rails 141, queue 467, 522, 552:
     `Server::Account` with a hold that never matches, whose `sent()`
     stays empty, as round 4's 458 did; the account server records a
     request before it parses a grant, `harness.rs` 1425–1447, so a
     malformed request it never expects is recorded rather than
     panicking the task); `token_server_answering`'s one remaining
     caller is `dispatch.rs:428`.
   - A seeded queue: the scenario writes rows into the `JobDb` it
     opened (as `disk_row` and the restore tests seed today), gives it
     through `.jobs_db` with `.restore_jobs(Retention::default())`, and
     the daemon restores as a start does; a restored row is a persisted
     input (`REVIEW-PROMPT.md`'s three kinds), trusted as the daemon
     trusts it.
   - A scenario whose admitted jobs must not run (a restore whose
     restored job would dispatch before the read: 1369; the queue
     broken after a submit: 1728) opens with `.restore_landed(false)`:
     the dispatcher picks nothing until the restore lands
     (`dispatch.rs` 73–77), and a `sleep` submit with `account: None`,
     a `Cancel`, a `SetPriority`, a `List`, a `Status` and a `Result`
     never wait on the restore (`server.rs` 273–275 waits when an
     account is named or the kind needs one; `kind_needs_account`
     is false for `sleep`, `daemon.rs` 1668–1672).

2. **The admission table** in `queue.rs`: one `#[test]` over rows,
   each a provider (`mock` or `ggg`), a session (`None`, live, or
   refresh-due), a submit (kind, params, priority, submitter, account)
   and what it is answered: the refusal's kind (`Refused`;
   `NotLoggedIn` for 872's selector row) and the words it must carry
   and must not; after every row's submits, `List` answers no job and
   the fixture's record is empty (`Server::Account` with a
   never-matching hold, which records any well-formed send and is
   never awaited; the old baits were aborted scripts); on the mock
   rows a sleep submit then answers `Submitted { id: 1 }` (what
   `assert_nothing_admitted` read privately: a refused submit spends
   no id), which the real-named rows cannot (a `sleep` is mock-only,
   `queue.rs` 1220–1223), so they assert the empty list and record
   alone.
   The rows, written from the old tests' submits and words: 19 (a
   client `probe`), 129 (a path param that is not one segment, 11
   rows, and the apply tuple's "action 0", 4 rows), 285 (a dot segment
   behind `;` or a space), 313 (a segment over the bound: the bound and
   a size description), 359 (a stash without an id or name), 522 (the
   retired `refresh` kind), 689 (mock-only kinds in real mode, on
   `Provider::ggg()`), 769 (an unknown kind names only client kinds:
   one row per provider), 724's five bad-seconds rows, 872's eight
   (the response within the frame, the message within 1024 bytes),
   986's two over-bound refusals, 1127's two, 1077's `daemon`
   submitter (C136). 285's and 313's `route_for` reads (304, 316, 350)
   are a function's: they join 250's table (a `#[test]`, component),
   named in the commit. 872's frame check (950–953: the request itself
   fits a frame) stays in its rows as the old test measures it. The
   positive twins the old tests carried become scenarios beside the
   table: 1077's `cli:1` submit admitted as id 1, Done, exactly one
   GET on the account server and the journal against it (1104–1116);
   724's zero-second sleep admitted and Done (739–745).

3. **The rows that never passed the door**, one `#[test]` over rows in
   `queue.rs`: a seeded waiting row (kind, params, the session seeded
   through `.sessions`), restored through `.restore_jobs`, the
   dispatcher running it; `await_terminal` answers `Failed` with the
   door's words (`Result`'s message), the fixture's record and the
   journal empty: 395's four rows, 467's three (on a refresh-due
   session, nothing reaching the token route), 552's one (its words
   equal to a client submit of the same params' refusal message), 844's
   four (each restored row's failure words equal to the client
   refusal's message for the same params, 849–859, as 552's), 724's
   tail (the `-1` row, a duplicate of 844's, retired held by it).

4. **The queue file and the restore**, scenarios in `queue.rs`:
   - 1751 (a result read failure is not "no job"): `.jobs_db` on the
     scenario's path, `break_queue`, `request(Result { 42 })` answering
     `QueueFailed` carrying "could not read the persisted queue".
   - 1728 (cancel and reprioritise report a failed write):
     `.restore_landed(false)`, a sleep submit, `break_queue`,
     `SetPriority` answering `QueueFailed` carrying "priority
     unchanged", `Cancel` answering `QueueFailed` carrying "may not
     survive"; `Status` reading the priority as it was before the
     failed change (3, the rollback at `queue.rs` 696–703) and the
     state Cancelled (held in memory, no eviction on a failed queue).
   - 1544 (a waiting row on disk, not in memory, is never acted on):
     the rows 40 and 41 written to the db after the door opened (the
     neighbour rows as today), `Cancel { 40 }` answering `UnknownJob`
     "no job 40", `SetPriority { 40 }` likewise, `Cancel { 41 }` and
     `SetPriority { 41 }` answering `WrongState`; after `run`, row
     40 still waiting through the store's observer read.
   - 1369 (the restore drops probes and finishes parents): the seeded
     rows as today (1398–1409: probe 3, the cancelling running job 4,
     the leagues job 5, parent 6 and its waiting child 7, parent 1),
     `.restore_jobs`, `.restore_landed(false)`; `List` for the restored
     states (job 4 Cancelled at the restore, `queue.rs` 265–267), the
     leagues row 5 Failed "not replayed" through `Result`, `Result` for
     parent 1 (success); `Cancel { 7 }` acknowledged, then `Result` for
     parent 6 (interrupted, its tally: it concludes only once its last
     child ends, 1432–1449); the next sleep submit `Submitted { id: 8
     }`; after `run`, the probe row 3 absent through the store's
     observer read.
   - 1310 (C50): the seeded rows, `.restore_jobs`, `.sessions(logged_
     in_sessions())`, `Server::Account`; `await_terminal` on 1 (Failed,
     the words naming no id) and 2 (Done); `sent()` `[POST, GET]`; the
     journal against the wire; after `run`, the rows' states through
     the store's observer read.
   - 1465's refusal (a failed write refuses new jobs, "refuses new
     jobs") becomes a row of Do 4's shape beside 1751; its id rollback
     and "not picked" have no boundary read and stay in 1465 as a
     component test, its doc saying so (1487's literal `true` named in
     the commit as pinning the halted branch `idle.rs:22` already
     pins).
   - 1696 (a transition that cannot persist does not run) stays a
     component test: its break must land after the admission and before
     the dispatcher's transition, which no boundary step orders; its
     real 100 ms sleep at 1713 becomes an awaited state under `HANG`
     (the queue failure set, read privately as the test already does).
   - Component, each with a doc line naming its boundary: 250, 590,
     823 (tables), 1190 (three lifetimes in one test, a Running
     fabricated), 1497, 1591, 1668, 1686. They keep `#[tokio::test]`
     and `persisting_daemon` where they use it (its `Arc::get_mut`
     stays theirs).
   - 986 and 1127 (admitted at the bound): their refusals are rows of
     Do 2; their admissions (a 3,408-action apply, an at-bound sleep, a
     50,000-action apply) stay one component test together, their doc
     saying the admitted job must not run (an apply needs an account,
     so its submit waits on the restore before `resolve_account`,
     `server.rs` 273–275, and `.restore_landed(false)` would hold it
     forever; landed, the `sleep` runs on real time and an apply's
     children reach the wire).
   - 200 (a league with a space, encoded on the wire): a scenario on
     `Script` (HEAD and GET to `/stash/Hard%20Mode`), `await_terminal`
     Done, the journal's two paths encoded.

5. **`rails.rs` through the door**, every test a scenario (a local
   table where the inventory names one: 242, 284 and 330 over
   (ceiling, accounts, token answers) → (the parked set, the wire)),
   on `.rails(..)` with the config the test had (`tripwire_config()`,
   `max_sends`, 647's `state_path` under a regular file), sessions
   through `.sessions` (`logged_in_sessions()`, `live_sessions`, or the
   two accounts `dispatch.rs`'s cancel scenarios seed, which move to
   the harness if 284 and 586 need them), the halt read through
   `DaemonStatus.rails.halted` (the bare cause, `rails.rs` 754–759) and
   its full words through a quote or the job's failure where the test
   asserted them (361's "this daemon lifetime", 474's "halted by
   live-test rails:" prefix, 423's containment), parked jobs through
   `Status` (Waiting, retries 0), the disk through the store's observer
   read after `run`, "nothing picked" through `settle` where no send
   is held and `quiet` where one is (517), the nine sleeps and the two
   polls through `settle` and `await_job`, the 3 s ceilings gone; a
   local table's expected halt words are a row field (a ceiling of
   none reads "ceiling: 0 sends allowed", `rails.rs` 560–561; of one,
   "ceiling: 1 of 1"):
   - 26 (a trip leaves queued jobs waiting until the reset): `Script`
     HEAD, 429, 200, 200 under `tripwire_config()`; two submits;
     `await_job(first, retries == 1 && Waiting)`; `halted` carrying
     "429 on GET /fetch"; `settle`; `Status` both Waiting; the journal
     HEAD, GET; `Dashboard.errors` carrying "LIVE-TEST RAILS TRIPPED";
     `ResetTripwire`; `await_terminal` both Done; the journal's four
     sends with the hold's `wait_ms`.
   - 94 (the ceiling after n sends holds the queue): `max_sends: Some(2)`,
     `Script` HEAD, GET; the first Done, `halted` carrying "ceiling: 2
     of 2", `settle`, the second Waiting, `DaemonStatus.rails.sends` 2,
     the journal two sends.
   - 140 (a halted daemon refuses a refresh before any send): `Script`
     HEAD 429 with GGG's headers under the tripwire (the staged trip
     the old test fabricated through `rails.record`), a job to trip
     it, then `AuthCheck` answering `Error { Upstream }` carrying
     "halted by live-test rails"; the journal one HEAD, no POST.
   - 242, 284, 330 (the ceiling parks a job after the pick; a
     refreshing job; a ceiling of none): `max_sends` 1, 1, 0; the
     token answer `Script [token_answer("200 OK", ..)]` or none; the
     submits; `settle`; `Status` Waiting with retries 0 for each parked
     job; `halted` carrying the row's words; the wire (`received()`)
     and the journal against it; after `run`, the rows through the
     store's observer read.
   - 361 (a HEAD 522 halts this lifetime): `Script` HEAD 522 (no body),
     HEAD 204, GET; `halted` carrying "522 on HEAD fetch at
     2000-01-01T"; the quote's `halted` carrying "this daemon
     lifetime"; `Status` Waiting; `Dashboard.degraded_endpoints` empty
     (the route Unknown); `ResetTripwire`; Done with retries 0; the
     journal HEAD, HEAD, GET.
   - 423 (a token 522 refuses in the halt's words): `Script` POST
     /token 522; `AuthCheck` twice, each `Error { Upstream }` whose
     message contains the quote's `halted` whole (the refusal prefixes
     it with "token refresh failed: ", `refresh.rs:512`; the old test's
     containment at 447 and 461), no page fragments, no "acq auth";
     `DaemonStatus.rails.refresh_failed` `None`; the journal one POST.
   - 474 (a Cloudflare 503 names the trip and quotes no page): the
     tripwire, `Script` HEAD, 503 with Cloudflare's page; `await_
     terminal` Failed, the error carrying "halted by live-test rails:
     503 on GET /fetch" and no page fragment, no "restart"; the
     journal HEAD, GET.
   - 517 (a cancel before the halt parks the job and concludes it
     cancelled): `max_sends` 1, `Server::Account` holding `("POST",
     "/token", 0)`; the submit, `held()`'s arrival, `Cancel`, `quiet`
     (a send is held, so `settle` would wait out `HANG`, door.rs
     217–224), release; `await_terminal` Cancelled; `sent()` one POST;
     the journal against it; after `run`, the row Cancelled through
     the store's observer read.
   - 552 (a blocked token POST leaves the job waiting): `Script` POST
     /token a 403 challenge; `settle`; `Status` Waiting; the journal
     one POST.
   - 586 (a parked whoami is not picked again while halted): two
     sessions, `max_sends` 1, `Server::Account` with no hold; two
     `whoami` submits; `await_terminal` on the one that lands (Done:
     `whoami` sleeps 300 ms of real time after its refresh,
     `kinds.rs:621`, which the hook cannot see, so `settle` alone could
     resolve with it Running), then `settle`; `Status` the other
     Waiting; the journal one POST; the wire.
   - 647 (a rails write that fails is logged and its reset refused):
     `.rails` with `state_path` under a regular file; a dead grant
     staged by `Script` POST /token 400 through `AuthCheck` (the mark
     at `refresh.rs:482`, whose own write fails too), then HEAD and a
     403 challenge for a job; `await_terminal` Failed;
     `Dashboard.errors` carrying "could not be written" (two lines, the
     grant's and the block's, told apart by count); `ResetTripwire`
     answering `Error { Internal }` carrying the same words; `halted`
     still `Some`.
   `ceiling_daemon`, `assert_parked`, `wait_until`,
   `wait_halted_and_settled` and the private calls retire with their
   tests.

6. **Retirements**, each with its C154 line: in `queue.rs`,
   `assert_nothing_admitted` (held by the table's `List` and id-1
   reads), `client_submit`, `live_daemon` (held by `live_sessions`),
   the five 50-yield loops (held by `settle` or the empty record),
   `persisting_daemon`'s door-able callers, the `logged_in` callers
   (held by `logged_in_sessions`), the 24 door-able tests by name (each
   held by its row or scenario); in `rails.rs`, the 13 tests by name,
   the helpers above, the nine sleeps and four ceilings;
   `token_server_answering`'s rails and queue callers. A private
   assertion with no boundary read is "held by X" or "kept", never
   dropped.

7. `tools/gate.sh` once per commit (three at least: the door's and
   harness's additions with the admission table; the rest of
   `queue.rs`; `rails.rs`); the test stage's seconds and the two files'
   wall time before and after in the report.

## Done when

- `grep -n "tokio::test" crates/acquisition-daemon/src/daemon/tests/queue.rs`
  finds only the component tests Do 4 names (1190, 1465, 1497, 1591,
  1668, 1686, 1696 at `c03a11c5`'s names, and 986/1127's admissions as
  one component test, named in the commit), quoted; the tables and scenarios are `#[test]`s, each row and
  scenario listed in the commit with the test it replaced.
- `grep -n "assert_nothing_admitted\|live_daemon\|client_submit\|yield_now\|from_millis(100)\|logged_in(" crates/acquisition-daemon/src/daemon/tests/queue.rs`
  finds nothing but 1696's awaited state if it polls, quoted;
  `shared.lock()` only inside the component tests, and `Arc::get_mut`
  only in `persisting_daemon`, which they keep.
- `grep -n "tokio::test\|from_millis(100)\|from_secs(3)\|wait_until\|wait_halted_and_settled\|ceiling_daemon\|assert_parked\|yield_now\|\.rails()\|\.halted()\|pick_runnable\|endpoint_state\|valid_access_token\|mark_refresh_failed\|rails.record\|Arc::get_mut\|shared.lock()" crates/acquisition-daemon/src/daemon/tests/rails.rs`
  finds nothing (the patterns are calls, `.rails()` and `.halted()`,
  not the identifiers a test name carries).
- `grep -n "token_server_answering(" crates/acquisition-daemon/src/daemon/tests/*.rs`
  finds its definition and `dispatch.rs`'s one caller only.
- `grep -n "fn real_named" door.rs` finds one and `Provider::ggg` is
  absent from `daemon/tests/`; `grep -n "fn live_sessions" harness.rs`
  finds one; the quote read is `request(Request::Quote ..)`, no door
  call added for it.
- `grep -n "JobDb::open\|read_job\|jobs::read\|Persisted" crates/acquisition-daemon/src/daemon/tests/{queue,rails}.rs`
  finds the scenarios' seeding writes (before the door opens, or the
  staged rows named in Do 4) and their observer reads after `run`
  returns, each quoted with the `run` above it, and the component tests'
  own; no scenario reads `shared`.
- `git diff --stat <this brief's last cut> <head> -- crates/` lists
  files under `daemon/tests/` only; every golden, `CLI-REFERENCE.md`
  and `MCP-REFERENCE.md` byte-identical.
- `tools/gate.sh` exits 0 on each commit's tree; the test stage's
  seconds and the two files' wall time before and after in the
  report, asserted by nothing.

## Not in scope

- 5c: the nine door-able tests of the other files (`exit.rs` 1456,
  1550, 1602 on `world`; `parents.rs` 14, 96; `dispatch.rs` 66, 303;
  `quote.rs` 16; `stress.rs` 32), `ManualClock`'s retirement as a
  harness-level swap, the sleeps, ceilings and polls elsewhere
  (#131), `test_daemon_scenario_at`'s last callers, `login.rs`'s
  duplicate `TOKEN_HEADERS`, the exempt-route check and its registry
  line.
- The `sleep` kind's real time (`kinds.rs` 563–577): no scenario
  dispatches a sleep it does not mean to run.
- A failed restore through the door (1686) and three lifetimes in one
  scenario (1190): component tests.
- Any production change; any word a user meets.

## Rules

`TEMPLATE.md`. What differs: this round runs in the main tree on
`main`, in a checkout other sessions may share; never push; `acq` and
`acqd` only in a mock world, under the harness; kill only this
session's pids; an experiment is undone from a scratch copy; nothing
tracked is edited while the gate runs. The owner has granted autonomy
and builds for 2026-10-31 (the overnight grant). The attribution line
names the model the worker runs on.

## Report

`TEMPLATE.md`, and: the row and scenario count by file and the tests
retired; the test stage's seconds beside 24 and the two files' wall
time before and after; the name of 986/1127's component test; every
agent-facing design
decision the brief left open (the admission table's row shape and the
restored-rows table's, the submit's wider form, the two-account
sessions' home, `real_named`'s shape) for the owner's review.

## Raised for review

- A scenario's disk read as C45's observer after `run` returns (Do
  1): the first cut read the live file while the daemon answered,
  which C158 does not permit and C45 says "never while a daemon
  answers"; the second read it after `finish`, which does not end the
  daemon; the reviewer may hold the rule against every disk read.
- "Nothing picked" as `settle` resolving generalises round 3's
  component precedent (`dispatch.rs:468`); the rule is stated in Do 1
  and the reviewer may hold it against the rails scenarios.
- The rails state's bare cause in `DaemonStatus` against the full
  words in a quote: two surfaces for one halt, by design (the status
  for a frontend's line, the words for a user's eye); a scenario reads
  the one the old test read.
- 647 stages its dead grant through a real rejected refresh, whose
  write also fails: the error ring then holds two "could not be
  written" lines where the old fabricated mark gave one.

## Taken from the reviews

The first review's twelve (`t5-r5b-brief-5c301c06.md`):

1. `Provider::ggg()` carries the production URLs: a real-named mode on
   the fixture's base, as the old test builds it (Do 1, done-when).
2. The live queue file read while the daemon answers widens C158: the
   disk is read as C45's observer once the run is over (the third cut:
   after `run` returns); a seeding write is a staged input (Do 1, Do
   4, Do 5, done-when, Raised).
3. A sleep submit is refused in real mode: the id check on the mock
   rows only (Do 2).
4. 517's `settle` under a held send: `quiet` (Do 5).
5. 586's `whoami` sleeps real time: its terminal awaited before
   `settle` (Do 5).
6. 1369's sequence and rows: the leagues row 5, probe 3 absent, the
   cancel before parent 6's result (Do 4).
7. 1728's priority is the one before the failed change (Do 4).
8. 423's refusal contains the quote's halt, prefixed (Do 5).
9. The ceiling table's halt words a row field (Do 5).
10. The positive twins and the exact equalities named: 1077's `cli:1`,
    724's zero-second sleep, 844's equality, 872's frame (Do 2, Do 3).
11. The done-when greps as calls, the kept helper's `Arc::get_mut`
    and the seeding writes allowed (Done when).
12. The account bait records before it parses a grant (Do 1).

The second review's four (`t5-r5b-brief-297a6566.md`):

1. `finish` aborts the dispatcher without awaiting it: the observer
   read after `run` returns, on a directory the test owns (Do 1, Do 4,
   Do 5, done-when, Raised).
2. The observer functions are `Persisted::read_job` and
   `Persisted::read` (Do 1).
3. A sleep submit waits on the restore when an account is named:
   `account: None`; 986/1127's admissions stay a component test (Do 1,
   Do 4).
4. The Goal's id check is the mock rows' (Goal).

The third review's two remnants (`t5-r5b-brief-49d6e3f6.md`): the
first review's recap said "after `finish`"; "if kept" and a
"decision" for 986/1127 where Do 4 keeps them (Taken from the
reviews, done-when, Report).
