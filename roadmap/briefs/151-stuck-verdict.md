# 151-stuck-verdict — the daemon's one clock, and a door wait that fails at the exact park (#151)

The first cut, written by the orchestrating session on 2026-11-01 from
`roadmap/reviews/time-seam-retrospective.md` (Root A and step A1, A2
and E4 of its §10) and a re-read of the tree at `4eb55bc7`, every line
number below from that tree. To be reviewed cold before a worker reads
it, as the 4-0 and t5 briefs were; what each review changes is listed
at the foot. Its one ruling (Do 1) was the owner's and is in the
registry ahead of the code ("yes to all", 2026-11-01).

## Goal

When this step is done, every wait in the daemon's production sources
sleeps through the daemon's one `Clock`, held by a check in
`tools/arch-check.sh` in the shape check 10 has for `spawn_blocking`;
and a door scenario whose wait can never be answered fails at the park
where the runtime has nothing it started left to run, naming what it
waited for, instead of running to the 120 s ceiling. The 48 mutants
that moved from caught to timeout at t5's close (#151) are caught
again, which a weekly run after the close shows. No word a user meets
changes, no log line a test reads changes, and the daemon in production
waits exactly as long as it does today, since `SystemClock` is tokio's
timer behind the trait.

Why it matters: the spike found that the runtime's park is the one
exact signal that nothing can run, and t5 used it to move time forward.
The same park, with nothing parked on the clock, nothing in flight and
no hold, is exact about the other thing: the scenario is stuck. Today
that park increments a counter nobody reads (`door.rs:231`) and the
scenario waits `HANG`. What makes the verdict inexact is six waits in
the daemon that sleep on tokio's own timer, so the runtime parks beside
the clock with a timer the hook cannot see; and any wait of the
scenario's own on the runtime's I/O driver, which the door's rules
already say to hold the clock across. This step closes the first by a
rule with a check, measures the second before it builds on it, and
builds the verdict only on the evidence.

## Read

- `roadmap/reviews/time-seam-retrospective.md` §4 Root A (the idea and
  its evidence), §5 (what the developer seat pays), §6 (the lower
  ceiling and `test-util` rejected, and why), §10 rows A1, A2, E4.
- `roadmap/briefs/4-0-spike.md`, "Answer": the park hook's discovery,
  its two conditions (sends in flight, landings), and "What no count
  covers": the real-time tokio waits named there (the sleep kind's
  200 ms slices, whoami's 300 ms, the accept backoff, the drain's look,
  reqwest's timeouts), of which this step takes every one but reqwest's
  (the drain's look went onto the clock in t5 round 2).
- `roadmap/briefs/t5-design.md` §2.2 (the runtime is the door's; the
  hook's rule), §2.3 (every blocking task counted, check 10), the "Not
  doing" draft on a request timeout's expiry (§5), and §2's last
  paragraph: a silent server waits its real timeout. This step keeps
  that line true and says why it is structural (Do 1).
- `decisions/daemon.md`: C158 (a daemon scenario crosses the frontend
  boundary only; its time moves by its own hand or when nothing it
  started can run: this step's second clause is that signal read the
  other way), C157 (a lifecycle that waits is a value; the `Life`
  driver's waits are on the clock already), C137 (a deadline's
  contract is its expiry behaviour; the knobs; no wall-clock bound a
  slow machine can miss: the stuck verdict is not a bound), C133 (one
  exit; the bound's OS thread stays real), C148 (landings), C3 (the
  idle exit, whose knob the `sleep` kind's rows meet), C84 (one tree,
  one artifact: no `test-util`, no `tokio_unstable` cfg, which the
  `features` and `artifacts` stages would see). The "Not doing" line
  of 2026-10-31 on virtualising a request timeout's expiry (line 61).
  `AGENTS.md`: C122, C153, C154.
- `crates/acquisition-daemon/src/ratelimit.rs`: the `Clock` trait
  (68–90: `now`, `wall`, `sleep`, `sleep_until` with its default,
  `kind`), `SystemClock` (90–115), whose two `tokio::time::` lines
  (106, 113) are the only ones the check allows. `ChokePoint::sleep`
  and `sleep_until` (`ratelimit/choke.rs`, by grep): the daemon's
  callers reach the clock as `self.choke.sleep(d)`.
- **The six waits**, each a `tokio::time::` call in the daemon's
  production sources outside `tests/`, found by `grep -rn
  "tokio::time" crates/acquisition-daemon/src | grep -v "/tests/\|spec.rs"`
  at `4eb55bc7` (eleven lines: these six, `SystemClock`'s two, and
  three in `auth.rs` at 898, 916 and 924 under its `#[cfg(test)]` at
  647):
  - `daemon/kinds.rs:572–577`, the `sleep` kind: a deadline from
    `Instant::now()` and 200 ms slices on tokio time, its comment
    ("Real seconds by design, not the daemon's clock") written when the
    manual clock advanced on every sleep, a reason the parking clock
    retired; the slices exist so a cancel is seen within 200 ms
    (`self.cancelled(id)`, 574). Submitted in the daemon's tests by
    `exit.rs` 386, 425 (60 s through the door, cut by the bound), 458,
    1256, 1296, `idle.rs:182`, `parents.rs:192`, `evict.rs:190`
    (`grep -rn '"sleep"'`); by the CLI's provider tests at 0 s (t2).
  - `daemon/kinds.rs:621`, the `whoami` kind's 300 ms pause. Submitted
    by `dispatch.rs`, `queue.rs`, `rails.rs` (`grep -rln '"whoami"'`).
  - `daemon/start.rs:696`, the accept backoff inside `serve` (675–700,
    `self: Arc<Self>`, so `self.choke` is in reach; C134, served
    through). Pinned by `exit.rs`'s
    `a_run_of_listener_errors_logs_a_bounded_number_of_lines` (1.18 s
    today, the daemon run of the retrospective) and the two listener
    tests the door's doc names as off the door (`door.rs:42–43`).
  - `daemon/login.rs:48–51`, the login's callback bound:
    `tokio::time::timeout(daemon.login_within, wait_callback(...))`
    inside a spawned task; `daemon.choke` is in reach (`login.rs:63`).
    Pinned at zero by the browser-wait test the 4-0 life brief cites
    (`tests/login.rs`, by name: grep `login_within` there).
  - `daemon/login.rs:445`, `wait_job_terminal`'s bound (436–452, "the
    profile wait"), whose expiry has no pin (the ledger's open item of
    4-0 step 2 and #131's comment of 2026-10-31; `queue.rs:1668`'s
    pin is a broken-queue answer).
  - `daemon/login.rs:1007`, `answer_callback`'s read bound
    (`CALLBACK_READ_WITHIN`, 959, 10 s; `wait_callback` at 975 takes
    the listener, the state and the bound and spawns `answer_callback`
    per connection, which holds no daemon): production code, outside
    any `#[cfg(test)]` (the file's one such attribute, 139, is on a
    helper function).
  - And the two real-time reads beside them, `Instant::now()` at
    `kinds.rs:572–573` (the `sleep` kind's deadline, which goes onto
    `self.choke.now()`) and `daemon.rs:273` and `:282` (the exit
    bound's OS thread, `std::thread::sleep` on a `std` instant: real by
    design, C133, no tokio wait, outside the check by its spelling).
- `crates/acquisition-daemon/src/daemon/tests/door.rs`: the module doc
  (1–56: what the door promises, what stays off it, "a silent request
  holds its send permit through its real timeout"); `Counters`
  (156–175, "readable after, asserted by nothing"); `IdleHook` and
  `on_idle` (177–237: the branch order, quiet, hold, send, blocking,
  settle, advance, nothing parked); `idle_runtime` (240–254);
  `DoorBuilder::run` (483–508: the hook installed after the build,
  removed after the scenario, the counters returned); `beside` (521–:
  no hook of its own, the first daemon's counts alone); `settle` (715–
  735) and `quiet` (737–763), whose docs say when a park may be a
  socket read's; `hold_clock` (796–811); the waits this step gives the
  verdict: `await_leave` (822–832), `await_job` (839–867),
  `await_terminal` (869–897), `await_auth` (899–946), each under
  `tokio::time::timeout(HANG, …)`. `harness.rs:849`, `HANG`, "never a
  measure of the wait". The one reader of `Counters` today:
  `dispatch.rs:2486–2507`, the blocking-count probe, which asserts no
  advance while the count is held.
- `tools/arch-check.sh`: the header's check list (28–41) and check 10
  (462–512): the Python reader over `crates/acquisition-daemon/src/**/*.rs`
  outside `/tests/`, comment lines blanked, the helper's span found by
  brace depth, the `ok` line and the `BLOCKING` refusals naming the
  site; the stale words at 465–466 ("until its closure returns", which
  t5 round 1's fix made "until a runtime task observes the
  completion", `daemon.rs:1475–1494`). `tools/arch-check-breakers.sh`:
  `run_case <name> <refuse|pass> <pattern> <setup…>` (110), the edits
  (`adddep` 70, `tool` 90, the `zz_breaker.rs` files the `owned` list
  names, 36–45), the restore from copies and the trap (47); it has
  cases for §5, §7, §9 and §11 and none for check 10 (`grep -n
  spawn_blocking` finds nothing).
- `.cargo/mutants.toml` (the scope, the exclusions, "a mutant that
  hangs the suite is reported as a timeout, and counts as caught");
  `.github/workflows/weekly.yml` (the four shards, `workflow_dispatch`,
  exit 3 a timeout); `tools/mutants-compare.py` (its docstring: two
  runs' shards under `runs/`, keyed by file, function and description);
  the ledger's t5 round 6 row for the baseline and after-run ids and
  the tag names. #151's own text: the suspected cause and its
  "Closes when".
- `roadmap/LEDGER.md`, the rows of 4-0 step 2 (the login's profile
  wait named as a gap), t5 round 1 (the blocking count's handoff, the
  review's finding), round 2 (`hold_clock`, from a review's note on an
  uncounted callback wait; the 1 ms real polls in `drive_drain` and
  `await_a_look`, not this step's), round 4 (the login's three
  timeouts on tokio's clock, raised), round 5c (the halted fault
  scenario hangs to `HANG` rather than failing fast, raised).

## Input

Fixed before the work starts:

- **The sweep**, run at `4eb55bc7` by the command in Read: six
  production sites, listed there with their lines. The measure of the
  class is that command's output restricted to files outside
  `/tests/` and `spec.rs`, which Do 3's check makes a gate stage.
- **The ruling** (Do 1), drafted below and checked: with the line
  inserted after C158 in `decisions/daemon.md`, `python3 -I
  tools/bothy.py check` printed `ok registry 160 decisions in 10 files
  … every path named exists` on 2026-11-01 (the file restored from its
  copy, byte-identical).
- **The measurement this step owes before Do 5**, not yet taken:
  `nothing_parked` per door scenario across the daemon's suite. The
  door returns its `Counters` from `run` and nothing prints them. Do 2
  adds the printing under an environment variable the harness reads,
  and the report carries the table. The expected answer after Do 3 and
  Do 4 is zero in every passing scenario; the brief does not assume
  it (the `quiet` doc says a park may be a socket read's).
- **The failing input the stuck pin stages** (Do 5): a daemon whose
  rails are halted (the tripwire tripped, as `rails.rs`'s scenarios
  stage it) and a job submitted after the halt, so it is parked on
  disk and the dispatcher waits on its `Notify` with nothing on the
  clock; a scenario that awaits that job's end. Today that scenario
  would wait `HANG` (t5 round 5c raised exactly this case).
- **The mutants**: the baseline run 41265798977 on the tag
  `t5-round0-baseline` (`a8a719d6`) and the after-run 41278094352 on
  `t5-round6-after` (`e139e522`), both under `runs/` on the
  orchestrating session's machine as round 6 left them; 107 timeouts of
  416 against 62 of 407, the shards 73, 93, 115 and 125 minutes against
  83, 106, 66 and 83. The run that closes #151 is dispatched by the
  orchestrating session after this step's close, on a tag, as round 0's
  was: not this step's to wait for.
- **The last push's CI**: `Gate (Linux)` 41341397975 and `Gate
  (Windows)` 41341397978 on `4680cf39`, both success (`gh run list
  --branch main --workflow … --limit 1`, 2026-11-01). The retrospective's
  commit `4eb55bc7` is ahead of that push, documents only. Re-read at
  the step's start; a red run is this step's first concern or is filed.

## Do

1. **The ruling**, the owner's, entered before any code (ruled 2026-11-01, "yes to all": C160 entered in `decisions/daemon.md`, the "Not doing" line amended, the commit carrying this line): C160, now C160, in
   `decisions/daemon.md` after C158, in the form the registry holds,
   drafted here verbatim for his word:

   > - **C160 — Every wait in the daemon's sources sleeps through its one `Clock`, and a door scenario that can never proceed fails at the park where nothing it started can run.** The sleep kind's slices and the whoami pause, the accept backoff, the login's callback bound, its read bound and the profile wait sleep through the daemon's clock; reqwest's connect and request timeouts are the one wait outside it, and a request in flight holds the gate's occupancy for their whole span; the exit bound's OS thread is no wait of the runtime's (C133). Under the door, a park that finds nothing parked on the clock, no send and no blocking task in flight and no hold of the scenario's is the runtime's word that the daemon will do nothing more, and every door wait fails there naming what it waited for; the hang ceiling is the backstop a wait outside the daemon can still reach. *Why:* a wait on tokio's own timer parks beside the clock unseen, so the hook could not tell a daemon that was finished from one that was waiting, and 48 mutants the component tests failed at once ran a scenario to its ceiling (#151). *Pinned:* nothing yet (check 12 of `tools/arch-check.sh` with its breaker, and `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park` in `acquisition-daemon/src/daemon/tests/door.rs`, when built).

   And one amendment to the "Not doing" line of 2026-10-31
   (`decisions/daemon.md:61`), for his word with it: after "a silent
   one waits its real timeout", the sentence "reqwest's timers are the
   HTTP client's own, which no clock of the daemon's reaches; the line
   is structural, not deferred (C160)". If he rules otherwise on the
   second clause of C160 (the verdict), Do 2 to Do 4 and Do 6 stand on
   the first clause alone and Do 5 is struck; the brief is re-cut to
   say so.

2. **The measurement**, before any production edit, its own commit:
   the harness reads one environment variable, `ACQ_DOOR_COUNTERS`
   (test support only, no README row: it is not a knob a user meets),
   and when it is set `DoorBuilder::run` prints one line per scenario
   at its end, the test's name as `std::thread::current().name()`
   gives it and every field of `Counters`. The command, run with
   `--no-capture` so the lines reach the log, and its parse into a
   table (scenario, advances, held_by_send, held_by_blocking,
   held_by_scenario, settles, quiets, nothing_parked) go in the report
   as the "before". A scenario whose `nothing_parked` is not zero is
   listed with the wait it was in at that park, read by the worker
   from the scenario (the door call it was inside, or the scenario's
   own await). This is the evidence Do 4 and Do 5 act on.

3. **The six waits onto the clock**, one commit, each site named in
   the message with the clock call that replaced it:
   - the `sleep` kind: the deadline from `self.choke.now()`, the slices
     `self.choke.sleep(200 ms)`; the comment rewritten to say what is
     true (real seconds under `SystemClock`; parked and advanced under
     the door, so a 60 s sleep job costs a scenario nothing); the
     payload and the words unchanged;
   - the `whoami` kind: `self.choke.sleep(300 ms)`;
   - the accept backoff: `self.choke.sleep(pause)`;
   - the login's callback bound and the profile wait: one helper on
     `Daemon`, `within(&self, bound, future) -> Result<T, Elapsed>` or
     the project's own name for it, a `select` of the future against
     `self.choke.sleep(bound)`, answering what `tokio::time::timeout`
     answered so the callers' arms do not change; both callers through
     it;
   - the callback's read bound: `wait_callback` and `answer_callback`
     take the clock they sleep on (an `Arc<dyn Clock>` or the choke
     point's handle, whichever the two callers already hold), the read
     bound raced through it.
   After this commit the grep in Read prints `SystemClock`'s two lines
   and `auth.rs`'s three test lines and nothing else; the message
   quotes it. Every existing test passes unchanged in what it asserts:
   the `sleep` kind's rows and scenarios (Read lists them) now park on
   the clock instead of sleeping real time, and a component test that
   submitted a `sleep` job on a hand-driven clock and never ran the
   dispatcher (`idle.rs:182`) sees no difference. The worker reads
   each site's tests before the edit and names in the report any whose
   duration changed.

4. **The holds the measurement named**, one commit: every
   `nothing_parked` park Do 2 found in a passing scenario is a wait of
   the scenario's own on the I/O driver that the hook cannot see (a
   socket of the scenario's, a request on its way to a fixture before
   the gate), and gets a `hold_clock` across it, by the door's own rule
   (`door.rs:796–801`); or it is a daemon wait Do 3 missed, which goes
   onto the clock in this commit with its site named. Then Do 2's
   command again: the "after" table. **The stop rule:** if any passing
   scenario still shows a `nothing_parked` park that is neither of
   those two, the step does not build Do 5. It names the site and what
   the runtime was waiting on in the report, under "What you left out",
   and the orchestrating session brings C160's second clause back to
   the owner narrowed to what the evidence holds. The first clause and
   its check stand either way.

5. **The stuck verdict**, one commit, on the evidence of Do 4 (zero
   `nothing_parked` across every passing scenario):
   - the hook's last branch (`door.rs:233–235` today) becomes the
     verdict: a park that finds nothing parked, no send, no blocking
     task, no hold, no settle and no quiet pending sends on a slot the
     door holds (the shape `SettleSlot` has, 207), the counters
     still counting it;
   - every door wait races its wait against that slot: `await_job`,
     `await_terminal`, `await_auth`, `await_leave`, and the awaits of
     `finish` on a script answered whole (1086, 1093); the losing arm
     panics with the wait's name, the job id or predicate it waited
     for, and the counters, in words a reader can act on ("the daemon
     will do nothing more: nothing parked on its clock, no send, no
     blocking task, no hold; awaiting job 3's end"); `HANG` stays
     around each as the backstop a wait outside the daemon can reach
     (a `beside` door's daemon, whose counts the hook does not read);
   - `settle` and `quiet` are untouched: each is woken by the hook at
     the parks its doc names, before the verdict's branch;
   - the pin, `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park`
     in `door.rs`'s own tests: the halted daemon and the parked job of
     Input, the scenario's `await_terminal` run under
     `std::panic::catch_unwind` (the door's `run` blocks on a runtime
     the test owns, so the panic unwinds to the test), the message
     asserted to name the verdict and the job, and the counters read
     after to show one stuck park and no advance. The test asserts no
     duration (C137); that it ends by the verdict and not by `HANG` is
     the message's own words.
   - `Counters` gains `stuck`; its doc keeps "asserted by nothing"
     for the rest and names the one test that reads this field.

6. **Check 12 and the breakers**, one commit: in `tools/arch-check.sh`,
   after check 11, "every wait in the daemon sleeps through its clock
   (C160)": under `crates/acquisition-daemon/src`, outside `/tests/`
   and comment lines, `tokio::time::` appears only inside `impl Clock
   for SystemClock` in `ratelimit.rs` (the span found as check 10 finds
   `fn blocking`'s), each other site refused by path and line in words
   that say what to do ("sleep through the daemon's Clock, `choke.sleep`
   or `choke.sleep_until` (C160)"); `auth.rs`'s three lines are under
   `#[cfg(test)]` and the reader must blank a `#[cfg(test)]` item's or
   module's span as it blanks comments, or the check lists them: the
   worker picks the reader's rule and the message says which. The
   header's list (28–41) gains item 12. In
   `tools/arch-check-breakers.sh`: a case writing a `tokio::time::sleep`
   into a scratch file under `crates/acquisition-daemon/src/` (the
   `owned` list gains the path, the restore removes it) and expecting
   the refusal naming it; a case moving the span's one allowed site
   outside `SystemClock` in a copy of `ratelimit.rs` and expecting the
   refusal; and, since check 10 has none, a case writing a second
   `spawn_blocking` into the same scratch file and expecting check 10's
   `BLOCKING` refusal (the group's rule of 2026-10-30: each fixed check
   gains a breaker case). The stale words at 465–466 corrected to the
   count's actual rule (E4 of the retrospective).

7. `tools/gate.sh` once per commit on each commit's tree, and
   `tools/arch-check-breakers.sh` on the tree of Do 6; every golden
   under `crates/*/tests/goldens` byte-identical to the step's parent;
   `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the README unchanged.

## Done when

- `decisions/daemon.md` holds C160 as Do 1 drafts it, or as the owner
  reworded it, in the commit before the first production edit; the
  "Not doing" line reads as he ruled.
- `grep -rn "tokio::time" crates/acquisition-daemon/src | grep -v "/tests/"`
  prints `ratelimit.rs`'s two `SystemClock` lines and `auth.rs`'s three
  `#[cfg(test)]` lines and nothing else, quoted in Do 3's commit
  message with each of the six retired sites beside the clock call
  that replaced it.
- `grep -rn "Instant::now()" crates/acquisition-daemon/src | grep -v "/tests/"`
  prints `ratelimit.rs:94` (`SystemClock::now`) and the exit bound
  thread's two `std` reads in `daemon.rs` and nothing else, quoted in
  the same message.
- The report carries two `Counters` tables, Do 2's and Do 4's, from
  the command each names; Do 4's shows `nothing_parked` zero in every
  passing scenario, or the step stopped before Do 5 and the report
  says at which scenario and why.
- `c160_a_wait_the_daemon_will_never_answer_fails_at_the_park` exists
  by name in `door.rs`, stages the halted daemon and the parked job,
  and asserts the verdict's words and `stuck == 1`, no duration.
- Check 12 exists in `tools/arch-check.sh` with its header line, and
  `tools/arch-check-breakers.sh` has the three cases Do 6 names, each
  refusing by the words the check prints; the breakers' run is in the
  report (they are not the gate's).
- `tools/arch-check.sh:465–466`'s words say the count holds until a
  runtime task observes the completion.
- No test under `crates/acquisition-daemon/src/daemon/tests` and no
  door wait holds a wall-clock bound other than `HANG`; every `HANG`
  around a door wait is now the backstop behind the verdict, which the
  wait's doc says.
- Every golden byte-identical; the three references unchanged; every
  log line a test reads unchanged (the worker greps the daemon's
  `log(` lines it touched against `tests/log.rs` and the CLI's
  `daemon_signal_exit.rs`).
- `tools/gate.sh` exits 0 on each commit's tree.

Not this step's done-when, the close's: the weekly workflow dispatched
on a tag at the step's last commit by the orchestrating session, its
four shards under `runs/`, and `python3 -I tools/mutants-compare.py
runs/t5-round6-after runs/<this tag>` showing the 48 moved mutants
caught and no new survivor, with the longest shard under 106 minutes,
which is #151's "Closes when".

## Not in scope

- reqwest's connect and request timeouts (`choke.rs:240–241`): the
  HTTP client's own timers; the "Not doing" line as Do 1 amends it.
- The exit bound's OS thread (`daemon.rs:273–282`): real by design,
  C133; outside the check by its spelling (`std::thread::sleep`).
- The 1 ms real polls in `harness.rs:923` and `:950` (`advance_until`
  and the drain helper): component helpers on a plain runtime with no
  hook, the ledger's open item of t5 round 2, not a door wait.
- The `beside` door's missing hook (`door.rs:521–`): its rule stands
  (a daemon that is asked, sends nothing, is finished in the
  scenario); the verdict reads the first daemon's counts, as the hook
  always did.
- `settle`'s and `quiet`'s meanings and the sentence rule under
  `quiet`: unchanged; a type for `quiet`'s precondition is not asked
  for.
- `HANG`'s value, the nextest profiles' ceilings, cargo-mutants'
  timeout: unchanged (the retrospective's §6, a lower ceiling is a
  wall-clock bet).
- The limiter spec's eight real-clock tests (`ratelimit/tests/spec.rs`
  1925 to 2332, `ChokePoint::new()`): #131's, the retrospective's E1,
  a later small step.
- The client's and the frontends' tests, their ceilings and sleeps:
  cross-process, #131's client rows and t6's.
- `IDLE_REJUDGE`, `DRAIN_POLL`, `HALT_POLL`: kept with their reasons
  (the retrospective's §3 and §6).
- The `.cargo/mutants.toml` exclusions: unchanged; a mutant this step's
  verdict newly catches needs no line, and one it newly misses is a
  finding for the close, not an exclusion.
- A README row for `ACQ_DOOR_COUNTERS`: test support, not a knob.
- Any word a user meets, any `daemon status` field, any protocol
  change.

## Rules

`TEMPLATE.md`. What differs: this step runs in the main tree on
`main`, in a checkout other sessions may share, in a window the owner
names (his machine is his during a league sitting); never push; `acq`
and `acqd` only in a mock world, under the harness; kill only this
session's pids; an experiment is undone from a scratch copy, never
`git checkout -- <file>`. Six commits at most (the measurement, the
waits, the holds, the verdict, the check, the gate's last), each gated
on its own tree. The group's review rule binds: every commit here
changes non-test code or a tool, so the step's commits are reviewed
cold as one range, and a fix round that changes behaviour is reviewed
again. The attribution line names the model the worker runs on. The
rules on paused tokio time and on wall-clock bounds bind the thing
this step builds, not only the session's own work: the verdict is not
a bound, and the pin asserts none.

## Report

`TEMPLATE.md`, and: the two `Counters` tables (Do 2, Do 4), each with
its command; the `sleep` and `whoami` tests whose durations changed
under Do 3, before and after, asserted by nothing; the breakers' run.
Raised for review, at least: whether the verdict's words should name
the counters or only the wait; whether `ACQ_DOOR_COUNTERS` should stay
after the measurement or go with the step; and any scenario Do 4 gave
a hold to whose wait was not a socket's, which is the case the brief
did not foresee.

## Drafts for the owner

Each is his; nothing is entered until he rules.

- **C160**, as Do 1 quotes it, with the "Not doing" amendment beside
  it (ruled 2026-11-01, entered). Recommended as drafted: the first clause is invariant 1's shape
  for time and costs six edits and a check; the second is what the
  spike's signal was always exact about, and the measurement in Do 2
  to Do 4 is where the brief stops being sure and starts checking.
- **The stop rule** (Do 4): that a `nothing_parked` park the brief
  cannot explain stops the step short of the verdict rather than
  widening the hook's conditions by a guess. Recommended: a count
  guessed at is the spike's refuted door rule again.
- **Check 10's breaker case riding in Do 6**: the group's rule says
  each fixed check gains one; it was not written at t5 round 1.
  Recommended: it is ten lines in a file the step opens anyway.

## Taken from the reviews

None yet: this is the first cut.
