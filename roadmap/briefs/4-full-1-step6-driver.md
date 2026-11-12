# 4-full-1, step 6.3 — The spike: the check's driver

## Goal
One question of step 6's spike is answered by measurement: what
drives the browser-driven check, and what it adds to the gate. It is
asked of a page `main` already serves, so it waits on nothing. `main`
gains a record and, where a driver worked, two decisions drafted for
the owner: the driver and the check's home.

## Inputs
- `4-full-1-design.md` §2 (the check is for a presentation rule
  inside a script: the picker's today, the window's at step 6) and §7
  item 6 (the driver measured by what it adds to the gate; two
  shapes to the owner).
- `crates/acquisition-cli/src/web/assets/count.js`, the rule under
  check: after a form changes, the number beside its button is the
  one the `count` route answers for the form as it stands.
  `REVIEW-CHECKLIST.md`, "A script rule no test reaches": that rule,
  held by reading today.
- `crates/acquisition-cli/tests/web.rs` and `tests/harness/mod.rs`
  (`Pages`): how a test starts `acq web` in a mock world and asks a
  route; `the_count_answers_as_acq_search_at_limit_zero_does`.
- `decisions/frontends.md`, "Not doing": no Node or bundler in the
  gate.
- `tools/arch-check.sh`, check 5: no member reaches an HTTP client but
  through the transport crate, a dev-dependency's reach included.
  `crates/acquisition-cli/Cargo.toml`, the feature pins: a feature a
  dev-dependency asks unifies into every test build (C84, the
  artifact mismatch), which `tools/gate.sh --only features` checks.
- `deny.toml`: the licences allowed. C122 (a crate's integration
  tests are one binary): the check is a module of `tests/main.rs`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Where it lives.** A detached copy of `main` outside the
   repository, under scratch (`git worktree add --detach`); nothing is
   committed there. `main` gains one commit: the record,
   `audits/web-2026-11/driver/README.md`, and beside it, for each
   candidate that was built, its whole diff from the copy's base, the
   lockfile included, as a patch that passes the gate's `diff` stage
   (a blank context line may lose its space; `git apply` reads it)
   and is proven to apply to that base. A candidate set aside on
   paper has its assessment in the record and no patch.
2. **The check, written once per candidate.** A test starts the web
   pages as `web.rs` does, loads the form page in the browser, and
   types a query into the query box as a user's keys would, then a
   second in its place. The two are valid queries whose `count`
   answers differ in `matched`, which the test asserts. It passes
   when, after each, the "Matches" output comes to show the total
   the route answers the test for that form. It waits under the
   runner's hang ceiling alone, no bound of its own (C137, no
   wall-clock bound a slow machine can miss); the time each wait took
   is recorded.
3. **The candidates, the lightest first.**
   - (c) No protocol: the browser launched headless at a URL and
     nothing more. Assessed on paper first: say what the page would
     have to carry to drive itself and report, and whether `main`
     would have to serve it. Built only if it would not.
   - (a) A CDP client crate as a dev-dependency of the cli crate.
     Read the maintained ones' current docs and take one that
     downloads no browser.
   - (b) Hand-written: the browser launched headless with a debugging
     port, the protocol spoken over a hand-written loopback WebSocket
     client, no new crate. Abandoned if its client passes 400 lines
     before the check passes; record how far it got.
   Ruled out: WebDriver (a driver binary matched to each browser's
   version) and anything that fetches a browser.
4. **A candidate is admitted by the tree's own checks, unchanged.**
   With the candidate in the copy: `tools/arch-check.sh`,
   `tools/gate.sh --only features` and `cargo deny check` each exit 0.
   One that fails any is recorded with the failing line and measured
   no further; no check is changed to admit it.
5. **What is timed.** Every stage of `tools/gate.sh` that compiles
   the cli crate's tests, each by the stage's own command, timed
   apart: `build`, `test` (its build, `--no-run`), `clippy`,
   `winclippy`; read `run_stage` for any other. Clean: each after
   `cargo clean`, twice. Incremental: each after touching
   `crates/acquisition-cli/tests/web.rs`, three times, the median.
   The control is the copy untouched; each admitted candidate is
   timed the same way, the machine doing nothing else. The record
   says which stages were timed and keeps them apart from any total
   it projects.
6. **The tolerance, named before the first run.** A candidate's
   difference from the control is its cost only where it is over a
   tenth of the control's figure and over the gap between the
   control's own samples. A smaller one is said as unresolved.
7. **What else is recorded,** each number beside its command, said as
   this Mac's: crates in `Cargo.lock` (`grep -c '^name = '
   Cargo.lock`); the check alone, the median of 20 runs and the
   failures in 20; the browsers here it drives (Brave, Chrome); what
   a machine must have installed.
8. **What may be proposed.** A driver is proposed only if its check
   passed 20 of 20 here. If none did, the record says inconclusive,
   names each blocker with its evidence, and drafts the next question
   in place of both decisions.
9. **The check's home is the owner's.** With a working driver, the
   record gives both shapes with its numbers: a gate stage (a browser
   on every gate machine; a missing one fails, never skips) or a
   CI-only job on hosted runners. A hosted image's software is read
   from its published list and said as read; a cost in CI is said as
   projected, never as measured. No workflow or gate change, in the
   copy or on `main`.
10. **Filed before the commit.** The builder sends its drafts to the
    launching session, which files one `trigger:ruling` issue each;
    the commit's message names them.

## Find first
- `grep -n "http_clients=\|http_wire=" tools/arch-check.sh`: the
  crates check 5 names, read before choosing a candidate.
- `git grep -n "feature pins" -- crates/acquisition-cli/Cargo.toml`:
  where a pin would stand if a candidate needs one, for the record.
- `git grep -n "pub fn start_web\|pub const SCRIPTS" -- crates/acquisition-cli/tests/harness/mod.rs`
  and `git grep -n "fn offline_web\|fn seed" -- crates/acquisition-cli/tests/web.rs`:
  how a test starts the pages, the scripts served, the seeded world.
- `git grep -n "data-count" -- crates/acquisition-cli/src/web`: the
  form and the outputs the check reads.
- `git grep -n "browser-driven" -- REVIEW-CHECKLIST.md`: two rows,
  page state the scan cannot see and a script rule no test reaches;
  the record says which properties of each a check could take over.

## Done when
- The record is on `main`, its verdict first: each candidate's
  admission (Decision 4), the timings and what Decision 6 makes of
  them, Decision 7's numbers, the machine, the date, the commands;
  and either the two decisions with both homes' numbers or Decision
  8's inconclusive outcome.
- The commit's message names each issue filed (Decision 10).
- Each built candidate's patch is beside the record (Decision 1).
- `tools/gate.sh --docs` exits 0 on `main`'s commit.

## Not in scope
- The check as the gate will run it, the picker's, the window's: the
  step that follows the owner's pick.
- A change to a workflow, the gate or a check; a dependency on `main`.
- The window script and its prototype: the other brief's.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world, as the cli crate's
tests start it. A browser the builder launches is headless, on a
profile directory under scratch, never the owner's profile. After the
commit the copy is removed and its worktree pruned, said in the
report.

## Report
`TEMPLATE.md`.
