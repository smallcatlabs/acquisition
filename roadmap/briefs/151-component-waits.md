# 151-component-waits — each named timeout fails where its test waits (#151)

Written by the orchestrating session on 2026-11-03, after the transport
seam's close (`LEDGER.md`, the row of that date), from the after-run's
saved logs (`runs/transport-seam-after/`, Weekly `41567365592` on the tag
`transport-seam-after` at `a6e34e8e`) and a read of the tree at
`f84960ff`. Every line number is `f84960ff`'s except the mutants' in the
Input table, which are the after-run's (`choke.rs` moved by 26 lines at
`d831658c`; the measure's patterns are `f84960ff`'s). Cut three times: the
first cut (`d5dffbac`) and the second (`3530b90c`) were each reviewed
cold (`roadmap/reviews/151-component-waits-brief-<sha>.md`, seven
findings then three, the prompts beside); this cut answers the second's
three, listed at the foot under "Taken from the review" with the
first's seven. Not reviewed a third time unless the owner asks. The worker runs on Opus 5.5. Its one ruling (Do 1) is
the owner's, given on 2026-11-03 ("Accept it") and in #151's body the
same day.

## Goal

When this step is done, each of the sixteen mutants named under Input is
caught, not timed out: every test that hung under it fails at the moment
its wait can no longer be answered, naming what it waited for, instead
of running to `HANG` (120 s) and being cut by cargo-mutants' 20 s
timeout. #151 then closes on the restated condition of Do 1.

Why it matters: these mutants break the core of invariants 1 to 3 (a
probe's 429, a Cloudflare block, the journal's record of a send, a
bearer's origin, the limiter's reading of the headers). They are
detected today, but a real regression of the same shape would show as
a test hanging to a ceiling, not a failure that names the property it
broke. The stuck verdict (C160, a door wait that fails at the park
nothing can wake) gave that to the door's waits; these are the waits it
does not reach yet.

## Read

- #151's body and its comment of 2026-11-03 (the seam's compare).
- `roadmap/briefs/transport-seam-design.md` §3, the row "#151's
  remainder": the thirteen open timeouts and where the design placed
  each, written before the logs below were read.
- `roadmap/briefs/151-stuck-verdict.md`, Goal and Do: how the verdict
  reads the park, and why a door opened beside another keeps the
  ceiling.
- `crates/acquisition-daemon/src/daemon/tests/door.rs` 1 to 60, the
  module doc: what C158's door is, what the hook counts (the clock's
  sleepers, a held request, a blocking task, a scenario's hold), and
  which tests stay off it, each class with its reason. It ends: a
  `#[tokio::test]` never uses the door, because its runtime would nest.
- `crates/acquisition-daemon/src/daemon/tests/harness.rs` 568
  (`wait_terminal`) and 591 (`advance_until`): the component tests'
  waits. Each is a yield loop under `tokio::time::timeout(HANG, …)`, so a
  wait that is never answered spins until the wall ceiling and the
  runtime never parks.
- `.cargo/mutants.toml`, its header: how a timeout counts as caught, and
  why an exclusion names its line.
- `TESTING-NOTES.md` and `REVIEW-CHECKLIST.md`'s daemon section.
- C122 (a behaviour is pinned once, at the lowest crate that holds it),
  C137 (a deadline is pinned by forcing its expiry), C158 (the door),
  C160 (the verdict), C154 (a retired test names what holds it now).

## Input

**The sweep.** Every timeout in the after-run, mapped to the tests that
were still running when cargo-mutants cut the suite. Each timeout's log
names them as nextest's `SIGTERM` lines. The command:

```sh
python3 -I - <<'EOF'
import json, re
for s in range(4):
    base = f"runs/transport-seam-after/mutants-shard-{s}"
    for o in json.load(open(f"{base}/outcomes.json"))["outcomes"]:
        if o["summary"] != "Timeout":
            continue
        log = open(f"{base}/{o['log_path']}", errors="replace").read()
        cut = sorted(set(re.findall(r"SIGTERM \[[^\]]*\] \([^)]*\) \S+ (\S+)", log)))
        print(o["scenario"]["Mutant"]["name"].split("src/", 1)[1], *cut, sep="\t")
EOF
```

It prints 77 timeouts, each naming at least one test. The runner ran
four tests at a time, so a mutant naming four may hang more; Do 3
finds the rest locally.

**The sixteen**, with the tests named at their cut (`daemon::tests::`
and `acquisition-daemon/src/` dropped):

| Mutant | Hung at the cut |
| --- | --- |
| `choke.rs:328:9` `admit_bearer_url -> Ok(())` | `ratelimit::spec::c128_a_bearer_goes_only_to_the_api_origin_and_nothing_reaches_either_server` (spec.rs 4192) |
| `choke.rs:330:46` origin guard `with true` | the same |
| `choke.rs:682:14` `delete !` in `head` | `dispatch::a_blocked_probe_is_no_probe_failure` (dispatch.rs 484) |
| `choke.rs:739:20` `\|\| with &&` in `head` | the same |
| `choke.rs:996:9` `record_completed with ()` | the same |
| `choke.rs:757:19` `== with !=` in `head` (the 401 guard) | `bearer::rejected_access_token_is_not_dispatched_after_its_401_under_the_gate` (bearer.rs 474), `bearer::rejected_access_token_is_not_dispatched_by_a_waiting_probe_after_its_401` (486), `stress::n6_integration_stress_…` (stress.rs 65) |
| `choke.rs:864:20` `&& with \|\|` in `finish_send` | `stress::n6_integration_stress_…` |
| `choke.rs:1130:5` `verdict_of -> None` | `dispatch::a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header` (dispatch.rs 629) |
| `choke.rs:1131:6` `delete !` in `verdict_of` | the same |
| `limiter.rs:112:5` `policy_state_key -> String::new()` | `door::c160_the_transports_hold_holds_the_clock_through_the_door` (door.rs 1707) |
| `limiter.rs:112:5` `policy_state_key -> "xyzzy"` | the same |
| `limiter.rs:444:9` `setup_after_probe_429 -> Ok(())` | `dispatch::a_head_429_that_fails_setup_holds_the_next_probe` (dispatch.rs 375) |
| `limiter.rs:485:9` `degrade_for with ()` | the same |
| `limiter.rs:545:26` `== with !=` in `observe_landed` | `bearer::rejected_…_by_a_waiting_probe_…`, `credentials::c148_a_token_rejected_while_its_save_is_held_is_not_handed_out_again` (credentials.rs 402, `multi_thread`, 4 workers), `exit::a_turned_away_job_is_not_picked_again_while_exiting` (exit.rs 386), `exit::exit_turns_away_a_job_parked_past_running` (353); four, so possibly more |
| `policy.rs:333:24` cap guard `with true` in `parse_retry_after` | `dispatch::a_head_429_that_fails_setup_holds_the_next_probe` |
| `policy.rs:333:32` `<= with >` in `parse_retry_after` | the same |

These are #151's thirteen open timeouts less `ChokePoint::sleep_until
-> ()`, plus the four that moved from caught to timeout at the seam's
close (`admit_bearer_url`'s two, `policy_state_key`'s two).

**The cut list is a floor, not the inventory.** The runner runs four
tests at once, and a run cut with four hung tests ran no further:
under `observe_landed`'s mutant the saved log
(`mutants-shard-*/log/*limiter.rs_line_545_col_26.log`) ends at 78 of
327 tests, with 249 never started. Do 3 finds the rest.

**The waits, as read from the tree** (to be confirmed by Do 3, which
may find others):

- **(a) An answer due at once.** The C128 test (spec.rs 4192) waits
  under `tokio::time::timeout(HANG, …)` for a refusal. The refusal
  comes before the first await in `get_bearer` (choke.rs 1052) and
  `head` (563); under the mutant, the bearer goes out to a listener
  that nobody reads.
- **(b) A component test's own polling wait,** on a plain
  `#[tokio::test]` runtime with no park hook:
  - `wait_terminal` (harness.rs 568), a yield loop under `HANG`;
  - `advance_until` (591), the same, moving the clock each pass;
  - `exit.rs`'s `wait_state` (114), a 5 ms sleep loop under `HANG`;
  - `dispatch.rs`'s own condition loops;
  - the stress test's wait on its refresh flight.

  The tests are `dispatch.rs` 375, 484 and 629, `exit.rs` 353 and 386,
  and `stress.rs` 65. Each test's doc says why it stays off the door.
- **(c) A door test's wait that has no verdict.** C160's verdict races
  only a wait raced through `or_stuck` (door.rs 1411), which installs
  the verdict's slot. The bearer scenarios' arrival barrier
  (bearer.rs 378) and the transport-hold test's row arrivals (door.rs
  1780) are plain `tokio::time::timeout(HANG, …)` reads, so no verdict
  can fire there. This is what the design's §3 row said of
  `bearer.rs:394`.
- **(c′) A door wait whose verdict the hook's accounting holds off.** A
  request the fixture holds keeps the hook from moving the clock and
  from calling the park stuck (door.rs 328). Under `policy_state_key`'s
  mutants, collapsed serialization may admit one GET and keep the
  second from arriving while the first is held. A count holds the
  clock; it does not keep the runtime from parking. (c) and (c′) can
  hold at once: the transport-hold test's row arrivals are a raw read
  (door.rs 1780), and its first GET can stay held behind its release
  (1666), keeping the count the hook checks first above zero. There,
  registering a verdict slot alone leaves the wait hanging.
- **(d) A test on a multi-thread runtime,** where the hook fires per
  worker and never means every worker is idle, so no park-based verdict
  exists: `credentials.rs` 402 (`multi_thread`, 4 workers). A
  wait-specific signal may still reach it, such as a channel or event
  that resolves or closes when what it waits for can no longer come.

**The measure** is a local cargo-mutants run over the sixteen. Each
must come back caught, and the run's log names each failing test. On
`f84960ff` these patterns list exactly the sixteen (`cargo mutants
--list` with them prints 16 lines):

```sh
cargo mutants -o "$SCRATCH/mutants-151" \
  --re 'choke\.rs:302:9: replace ChokePoint::admit_bearer_url -> Result<\(\), String> with Ok\(\(\)\)$' \
  --re 'choke\.rs:304:46: replace match guard \*allowed == origin with true' \
  --re 'choke\.rs:656:14: delete ! in ChokePoint::head$' \
  --re 'choke\.rs:713:20: replace \|\| with && in ChokePoint::head$' \
  --re 'choke\.rs:731:19: replace == with != in ChokePoint::head$' \
  --re 'choke\.rs:838:20: replace && with \|\| in ChokePoint::finish_send$' \
  --re 'choke\.rs:970:9: replace ChokePoint::record_completed with \(\)$' \
  --re 'choke\.rs:1104:5: replace verdict_of -> Option<Verdict> with None$' \
  --re 'choke\.rs:1105:6: delete ! in verdict_of$' \
  --re 'limiter\.rs:112:5: replace policy_state_key' \
  --re 'limiter\.rs:444:9: replace Limiter::setup_after_probe_429' \
  --re 'limiter\.rs:485:9: replace Limiter::degrade_for with \(\)$' \
  --re 'limiter\.rs:545:26: replace == with != in Limiter::observe_landed$' \
  --re 'policy\.rs:333:(24: replace match guard seconds <= RETRY_AFTER_CAP_SECS with true|32: replace <= with >) in parse_retry_after$'
```

The step changes no production source, so the lines hold; a pattern
that lists nothing has met an edit the step should not have made.

**The diagnostic run** goes past the four-at-once floor. For one
mutant, cargo-mutants passes what follows `--` to its test tool,
nextest here (`.cargo/mutants.toml`), so nextest's filter rides there
(or the worker runs nextest by hand over a copy of the
tree with the mutant's diff applied, from the measure's `diff/`). It
excludes, by name, each test already found hung under that mutant, and
repeats until the run completes, every test either passing or failing.
The union of the excluded tests is that mutant's inventory. The worker
records the command it used in the report.

**The last push's CI:** `Gate (Linux)` run `41581867229` and `Gate
(Windows)` run `41581867250`, both on `f84960ff`, both `success`.

## Do

1. **The ruling, the owner's, before any code.** #151's "Closes when"
   is restated as follows (drafted and accepted 2026-11-03, "Accept
   it"; the issue's body carries it, the old sentence struck through):

   > Closes when a Weekly after-run, compared by
   > `tools/mutants-compare.py` against `41567365592`
   > (`transport-seam-after`), shows each of the sixteen named in
   > `roadmap/briefs/151-component-waits.md` caught, not timed out.
   > `ChokePoint::sleep_until -> ()` is accepted as a timeout: it spins
   > the driver, and the timeout is the only signal a spin gives.
   > Shard minutes are recorded, not gated: about 85% of a shard is
   > building mutants, which grows with their count, not with what the
   > tests catch.

2. **The compare reads the cuts and the minutes.** `tools/mutants-compare.py`
   gains the sweep above:
   - each timeout of the after-run, with the tests its log names as cut;
   - each shard's minutes by outcome and phase, from `outcomes.json`.

   Both print by default after the existing sections. Its check,
   `tools/mutants-compare-breakers.sh`, builds fixtures with a
   nonexistent `log/x.log` and empty `phase_results` (line 80). Its
   fixtures gain a log with `SIGTERM` lines and phase durations, a case
   asserts each new section, and the check runs green. This is tooling,
   in its own commit.
3. **Diagnose before building, then stop.** Run the measure on the
   tree as it is, then the diagnostic run for each of the sixteen until
   its inventory is complete. For each hung test, find the wait that
   never returns:
   - the line, and the helper or loop it is in;
   - which kind it is, (a) to (d) above, or a new one;
   - for a door wait, two columns read apart: whether the wait has a
     verdict slot (c), and whether the hook's accounting held the
     verdict off (c′), with the count; both may be true;
   - the remedy proposed (Do 4), with the exact signal it reads, and
     whether it can be built in tests, test support and tools alone.

   Return the table to the orchestrating session before any test
   changes. The session approves it or answers within the round; this
   is the step's one agent-facing design decision. Where a kind has two
   defensible shapes and the smaller is not obvious, as (b kept off),
   (c′) or (d) may, the session asks a Fable oracle for the
   recommendation before answering, and records it as a dated bullet
   in its close. A remedy that needs a production edit, or a wait no
   exact signal reaches, stops the step here: the session brings it to
   the owner.
4. **Fail fast, by the first rule that fits.** Each rule names a signal
   that is exact: never a wall bound shorter than `HANG`, a count of
   yields, or an inference from the clock alone.
   - **(a)** An answer due at once is asserted ready at its first poll
     (`FutureExt::now_or_never`, or polling once by hand), so a send
     that would go out fails the assertion with the request still
     pending.
   - **(b)** A component test that can move onto the door moves (C158).
     Its waits then go through `or_stuck`. Its doc's reason for staying
     off is either gone, say why, or stands, and then the next rule
     applies.
   - **(b, kept off)** A component test that must stay off gets waits
     that end on an observation that nothing can answer them: the
     runtime parked with everything that could wake it accounted for,
     as the door's hook reads it, or a signal specific to that wait.
     An empty clock is not such a signal: a task a sleeper woke may
     still be waiting to run (testing.rs 69). Do 3 proposes the shape,
     and the session approves it.
   - **(c)** A door wait with no verdict slot goes through `or_stuck`, or
     the door's own wait that does.
   - **(c′)** A door wait whose verdict the accounting holds off: the
     hold is released or told apart where the mutant leaves it, by a
     shape approved at Do 3. Where (c) and (c′) both hold, the remedy
     answers both: a slot alone does not repair a held arrival barrier.
   - **(d)** A multi-thread test has no park-based verdict. It gets a
     wait-specific signal (Do 3 proposes it, the session approves it).
     A wait no exact signal reaches stops the step (Do 3); it is not
     built around.

   Production sources are not touched. A test whose assertions change
   says so in its doc where its meaning changes.
5. **Measure again.** Run the measure on the step's tree. Each of the
   sixteen is caught; a mutant that is not stops the step as
   incomplete, for the owner's ruling. Then run the gate. The
   orchestrating session dispatches the Weekly on a tag at the close
   and runs the compare.

## Done when

- `tools/mutants-compare.py` prints the cuts and the minutes. Its run
  over `runs/151-stuck-verdict-after` and `runs/transport-seam-after`
  prints, under the timeouts, the sixteen's rows as the Input table
  has them, and `tools/mutants-compare-breakers.sh` asserts both new
  sections and passes (CI runs it, `gate-linux.yml` 231; the worker runs
  it by hand before the commit).
- The step's last commit message carries the measure's outcome for each
  of the sixteen (caught, with the count from the run's `caught.txt`)
  and, for each, the tests that now fail under it.
- Each test the step retires or replaces has a `retired X; held by Y`
  line in the commit that does it (C154).
- `git diff --stat` over the step's range touches no file under
  `crates/*/src/` outside a `tests/` directory, `tests.rs`, `testing.rs`
  or `transport.rs` (the test support), plus `tools/`.
- No wait the step adds or changes is a wall bound below `HANG`, a count
  of yields, or an empty-clock inference (a reviewer's grep: `timeout(`,
  `yield_now` and `next_deadline` in the diff, each read).
- The gate is green on the step's tree.

## Not in scope

- **The other timeouts.** Sixty in the after-run, besides the sixteen
  and `sleep_until`'s, are promised nothing. Most are broad breaks (the gate
  never grants, the endpoint keys collapse, the policy parse returns a
  constant): the sweep, less the seventeen, gives 54 that name four cut
  tests and 6 that name one to three. Some share tests with the
  sixteen. `get_bearer`'s 401 guard (`choke.rs:1096:42` at the
  after-run) names the bearer and stress tests, so this step may change
  its outcome. The final measure lists any of them it happens to catch,
  as a note, not a claim.
- **`ChokePoint::sleep_until -> ()`.** It spins. Do 1 accepts it.
- **Shard minutes.** More shards, a cached build per mutant, or a lower
  cargo-mutants timeout. Do 1 records minutes, not gates them, and the
  workflow is unchanged.
- **Lowering `HANG`, or a wall bound in its place.** C137 rules it out.
- **The multi-thread verdict.** A park signal for several workers is a
  design of its own (`door.rs`'s doc, "several workers"). A (d) test
  gets a wait-specific signal or stops the step at Do 3.
- **Any production change,** including one that would make a mutant
  easier to see.

## Rules

`TEMPLATE.md`'s, and:

- This step touches tests, the test support and `tools/` only (binds
  the worker's work).
- Do 3 is a stop: no test changes before the orchestrating session
  answers the table (binds the worker's work).
- The measure runs with nothing else building on the machine, and
  never while the gate runs (binds the worker's work).
- A test moved onto the door keeps every assertion it had, unless the
  report names the property that now holds it, in C154's form (binds
  the thing built).

## Report

`TEMPLATE.md`'s, with the Do 3 table returned at the stop and again,
as built, in item 3.

## Taken from the review

The first cut's review (`roadmap/reviews/151-component-waits-brief-d5dffbac.md`):

1. The empty-clock shape for `advance_until` withdrawn; (b, kept off)
   requires an observed park with full accounting, or a wait-specific
   signal, approved at Do 3.
2. The door tests split into (c), a wait with no verdict slot, and (c′),
   a verdict the accounting holds off, each with its remedy.
3. A mutant that stays a timeout stops the step as incomplete, for the
   owner's ruling; done-when admits no exception.
4. Do 3's inventory is completed by repeated diagnostic runs that
   exclude the tests already found hung, and the waits read from the
   tree name `wait_state`, dispatch's loops and the stress test's flight
   wait.
5. Completion is in the last commit's message, and each retirement has
   a C154 line.
6. `tools/mutants-compare-breakers.sh` is named, and its fixtures and
   cases extended.
7. "The other 61 each hang four or more" is corrected, and the other
   timeouts are promised nothing.

The second cut's review (`roadmap/reviews/151-component-waits-brief-3530b90c.md`):

1. (d) was a known stop drawn as an unknown: it now rules out only a
   park-based verdict and takes a wait-specific signal approved at
   Do 3, stopping only where no exact signal reaches the wait.
2. (c) and (c′) are read as two columns that can both hold, and the
   remedy answers both where they do.
3. The other timeouts are sixty, 54 naming four cut tests and 6 naming
   one to three.
