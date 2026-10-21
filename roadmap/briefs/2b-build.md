# 2b — The tests (an Opus build from the Fable design)

## Goal
The suite keeps everything it pins and loses the ways it was built
twice: one process harness with one isolation list; each behaviour
pinned once, at the lowest crate that holds it, the frontends pinning
only what their adapters add; the CLI's wording held by goldens that
name their facts; one integration binary per crate except the files
that set the process environment; nextest as the gate's runner, a
process per test, never retried. `2b-design.md` is the drawing; this
brief is the order of work. When it is done a wording change is one
regenerated file read in review, a new daemon knob is scrubbed in
every test without a list to keep, a worker's edit loop on the search
tests halves, and the gate's test stage runs in about thirty seconds.

## Read
- `roadmap/README.md`, "Roles and rhythm" and "Starting a session";
  `roadmap/briefs/TEMPLATE.md` (the rules and the report form).
- `roadmap/briefs/2b-design.md` whole. Sections 2.1 to 2.5 are the
  shape; section 4 the rulings; appendix A names every test this step
  deletes, moves down or thins, by file and line as of `3bc71a86`;
  appendix B the golden sites; appendix C the harness API; appendix D
  the measurements to repeat.
- `AGENTS.md`: the hard rules (the mock world, build order, the gate
  deciding on each stage's own status), the quality gate, "Where things
  live"; C1, C46, C47, C84, C85.
- `TESTING-NOTES.md` ("Looks like coverage, isn't": the shapes a test
  must not take; the journal invariants, which this step never
  touches).
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets":
  every row naming a frontend test; none of those tests is deleted.
- The code: `crates/acquisition-cli/tests/harness/mod.rs`,
  `crates/acquisition-mcp/tests/harness/mod.rs`, the harness half of
  `crates/acquisition-client/tests/contract.rs` (lines 67–335 and
  1162–1273) and of `crates/acquisition-verbs/tests/daemon.rs` (lines
  36–240); `crates/acquisition-cli/tests/cli_reference.rs`,
  `crates/acquisition-mcp/tests/mcp_reference.rs`,
  `crates/acquisition-protocol/tests/wire.rs` (the fixture convention:
  write and pass under `ACQ_UPDATE_FIXTURES=1`); `tools/arch-check.sh`
  from its `doors` rule to the end and `tools/arch-check-breakers.sh`
  (how a rule and its breaker cases are written);
  `tools/docs-check.sh` (the `budget` line; the fixture and knob
  scans);
  `crates/acquisition-search/src/tabs.rs` (`LISTED_WHOLE`, `MOST_ITEMS`:
  the thresholds the tabs test stages); `.github/workflows/gate-linux.yml` and `gate-windows.yml`
  (the test steps); `.github/workflows/release.yml` (the
  `taiki-e/install-action` use to copy).

## Input
- The tree at the commit this brief lands in; the design's appendix D
  measurements, taken on this machine on 2026-10-21 (87 binaries;
  `cargo test` 94.0–94.5 s; nextest 29–30 s; the search relink 13.5 s).
- `cargo-nextest` 0.9.149 is installed on this machine
  (`~/.cargo/bin/cargo-nextest`).
- The owner's permission (2026-10-21) to raise the `AGENTS.md` byte
  budget during phase 2, which he reassesses himself; the budget is
  raised to what the file measures plus a hundred, the permission
  quoted in the commit message.
- The owner's word on the rulings: `2b-design.md` section 7, to be
  filled before this brief runs; quote it in commit 1's message.
- The tests this step changes are the ones appendix A names, by file
  and line as of `3bc71a86`; a line that has moved is found by its
  name. Every `*-REFERENCE.md` and fixture regenerated here is named in
  its commit's message with what changed in it.
- Failing inputs the new tests stage: a `Tabs::of` over a store with
  more than thirty tabs across two types (`LISTED_WHOLE` in
  `crates/acquisition-search/src/tabs.rs` is 30, below which every tab
  is listed; the existing forty-one-tab fixture serves): counts first,
  the ten with the most items listed (`MOST_ITEMS`), the cut said,
  `expand` lifting it, a `Selection` by type and by never-fetched; `list_seen` under a query that fixes one league; an
  exact trade clause's report (`{at, status, term, path}` alone); a
  `keys` entry holding a comma; a cross table past its limit
  (`view.cross.limit`); an unknown count key (the refusal names where
  the keys are listed); an unknown bare word (`name:` first, then its
  other reading); a touched term's row keys; `next` keeping a site
  search, edited text refused, `scope.realm` beside a URL refused,
  `trade: 7` refused; `league_list` before and after an empty fetch;
  `Frontend::jobs()` and `daemon()` over a ledger holding waiting and
  recorded-running rows; `DaemonReport::Running` and `Incompatible`
  serialised flat; an `unspent` plan against a job spelt `"sub":
  null`; the pricing verbs' limit, conflict and unreadable-row
  refusals; the MCP `show` with a malformed `against` (kind `tree`);
  the MCP `item_events` with `hours` and with none; for the layout
  check, a second file without `session(` under one crate's `tests/`,
  and a `session(` file listed as a `mod` of the plain one; for the orphan check, a golden file with no
  `golden(` call naming it.

## Do
One commit per item, in this order, in the main tree. Before each
commit `cargo build --workspace`, then the gate bare, each stage's
exit status read on its own line, the commit inside an `if` on every
status; from item 6 on the gate's test line is the nextest one, and
`cargo test --workspace --all-targets` is run once more before that
commit to prove the fallback still passes. Where a sentence of the
design cannot be built as drawn, build what the tree allows and name
the change under 5 of your report.

1. **The rulings** (documents only; `tools/docs-check.sh`, `python3 -I
   tools/bothy.py check` and `git diff --check` green). C<next> added to
   `AGENTS.md`'s cross-cutting rulings verbatim from the design's
   section 4 (its *Pinned* in words, as written); the budget line in
   `tools/docs-check.sh` raised to the file's size plus a hundred, the
   owner's permission quoted in the message. The gate line itself
   changes in item 6.
2. **The harness.** The shared file at
   `crates/acquisition-client/tests/harness/mod.rs` with appendix C's
   API (confirm the `#[path]` string compiles from a `mod.rs` first,
   in one build); `contract.rs` takes it with `mod harness;`, the
   verbs crate's `tests/daemon.rs` by `#[path]`, and the CLI and MCP
   harnesses become thin adapters that keep today's names, so the
   nine CLI files and the MCP files change no call sites. Every private
   copy deleted: the ten `sole_json`,
   six `http_get`, four `place_link`, eight scratch types, the four
   daemon-backed CLI files' `command`/`login`/scrub lists, the six
   offline ones', `plan_loop.rs`'s. The isolation list is the one in
   section 2.1 (remove every `ACQ_*` by prefix, then set the eight
   values); `scratch()` from the original temp dir; the daemon's
   stderr piped and printed on a start timeout; `approve` encodes `#`.
   `reqwest` leaves the client's and verbs' dev-dependencies (the
   lock changes; the contract revision moves once, say so). The
   `same_text` diff in its own std-only file `diff.rs` beside the
   harness, re-exported by it: `cli_reference.rs` and
   `mcp_reference.rs` use it through their adapters, and the protocol's
   `wire.rs` includes `diff.rs` alone by `#[path]` (its tests link
   serde alone; `tools/arch-check.sh` holds that), in place of their
   one-sentence failures (the write-and-pass under
   `ACQ_UPDATE_FIXTURES=1` unchanged); the plan's two inline fixtures
   are not touched. In `contract.rs`,
   the scripted foreign peer exits after it answers `stopping`, as a
   daemon does, in the two tests that today wait out the client's
   10 s stop bound; if either test asserts the bound itself, it keeps
   the wait and the message says which. Accept knowingly, and name in
   the message, the behaviour changes section 2.1 lists; if a test
   breaks on one (an idle at 30 s, a root listing that sees `tmp/`),
   say which and fix the test, not the list.
3. **One layer per behaviour.** The lower tests first, in the same
   commit: appendix A's move-downs, each a test named for its property
   in the file named (`crates/acquisition-search/tests/tabs.rs` is new;
   the others are rows or tests beside the ones cited); then the 13
   deletions with their two dead helpers; then the 28 thins, each
   keeping exactly the adapter's part appendix A names (aq3 keeps one
   successful `show` with a text `against`, the site-search test one
   with a trade `against`). `Seat::search`
   in `crates/acquisition-mcp/tests/search_tools.rs` becomes one test,
   `the_search_tool_is_acq_search_request`, over four requests (an
   answer, a refusal, a continuation, an offer), and the other tests
   stop spawning `acq`. The two new MCP tests for the gaps (`show`'s
   malformed `against`, `item_events`' `hours`). If a new test fails
   against the product, mark it `#[ignore]` with the reason in its doc
   and report it under 5: this step changes no production code.
4. **Goldens.** `Transcript` with `run(args, facts)` and
   `golden(file!(), name, &Transcript)` in the CLI adapter, the
   normaliser's named rules in the order section 2.3 gives, the file at
   `crates/acquisition-cli/tests/goldens/<stem>/<name>.txt` with the
   header line the design specifies; each command's `facts` checked
   against that command's raw output before the update switch, a
   transcript with no fact on any command refused, text commands only
   recorded (a `--json` command stays asserted by field). Convert appendix B's
   sites (about 20 to 23 goldens), each call carrying the one or two
   facts its seat finding named; the MCP's sentence literals shrink to
   fact words with no new golden; the daemon-backed texts stay
   literals. `seat_faults.rs`'s pointer test (today `:1330`) parses the
   printed command and runs it. `tools/docs-check.sh` gains the orphan
   line (every golden file is named by a `golden(` call in its stem's
   test file) with its breaker case in `docs-check-breakers.sh`, in
   the same commit as the check. The message names every golden
   written.
5. **One binary per crate.** `tests/main.rs` of `mod` lines in the
   search, CLI, MCP and protocol crates; `common`, the harness adapter
   and `generated.rs` compiled once each; the exceptions stay their own
   files: CLI `watch_recovery.rs` (keep its `#![cfg(unix)]`), MCP
   `plan_loop.rs`, the verbs crate's `daemon.rs` beside
   `documents.rs`, the client's `contract.rs`. Every place that prints
   or documents a regeneration command with `--test <file>` is
   rewritten to the merged form (the seven files the design names,
   `CLI-REFERENCE.md` and `MCP-REFERENCE.md` among them, regenerated).
   `tools/arch-check.sh` gains the layout rule as section 2.4 states
   it (under a crate's `tests/`, at most one file that does not call
   `session(`; every other file calls it; none is a `mod` of the plain
   one) with its two breaker cases in `arch-check-breakers.sh`, run
   once.
   Measure: `cargo test --workspace --all-targets --no-run` lists the
   binaries (expect 24); time `cargo test --workspace --all-targets`
   and the search relink after `touch crates/acquisition-search/tests/
   common/mod.rs`, for the report.
6. **nextest.** `.config/nextest.toml` as section 2.5 draws it
   (`default` and `ci` profiles, `retries = 0` in both with the reason
   as a comment, `fail-fast = false`, the slow timeouts); both gate
   workflows install `cargo-nextest` with `taiki-e/install-action@v2`
   pinned to `0.9.149` after the toolchain step and run `cargo nextest
   run --workspace --all-targets --locked --profile ci` in place of
   `cargo test` (Linux inside its `dbus-run-session` wrapper; Windows
   dropping `--no-fail-fast`, now the profile's). `AGENTS.md`: the gate
   block's test line, the build-order sentence (`cargo test
   --all-targets`, and `cargo nextest run`, rewrite `acq-mcp`), the
   once-per-clone line (`cargo install cargo-nextest --locked`); the
   budget line follows the file. Time `cargo nextest run --workspace
   --all-targets` twice and once with `ACQ_CASES=full`, for the report.
7. **The records** (documents only). C<next>'s *Pinned* names the files
   now that they exist (`.config/nextest.toml`,
   `crates/acquisition-client/tests/harness/mod.rs`); one row in
   `REVIEW-CHECKLIST.md` under the frontends, "A behaviour pinned at
   three layers", naming the rule and the layout check that hold it;
   one paragraph in `TESTING-NOTES.md`, "The suite's shape", saying
   where the harness, the goldens and the layout rule are and that a
   test which sets the process environment is its own binary.

After the last commit, launch the review as `roadmap/README.md` "Roles
and rhythm" says, over the range `<commit 2>^..<commit 7>`, save it
verbatim at `roadmap/reviews/2b-<last sha>.md`, commit that file alone
(`git diff --check` suffices), and fix no finding. Do not edit
`roadmap/README.md`. Do not push.

## Done when
- `grep -rn "fn sole_json\|fn http_get\|fn place_link\|fn approve\|fn
  scratch_tmp" crates/*/tests` finds each once, in
  `crates/acquisition-client/tests/harness/mod.rs`; `grep -rln
  "env::set_var" crates/*/tests` names the harness file alone, and
  `grep -rln "session(" crates/*/tests` names it and exactly
  `contract.rs`, the verbs crate's `daemon.rs`, the CLI's
  `watch_recovery.rs` and the MCP's `plan_loop.rs`; `reqwest` is in neither the client's nor the verbs
  crate's manifest.
- Every test appendix A lists as deleted is absent by name; every
  move-down has its lower test by the name the commit gives it; every
  thin keeps the part named; the two gap tests exist; `Seat::search`
  spawns `acq` in one test.
- `crates/acquisition-cli/tests/goldens/` holds the goldens, each
  named by a `golden(` call, its transcript carrying a fact on at
  least one command; `tools/
  docs-check.sh` refuses an orphan golden, with its breaker case.
- `cargo test --workspace --all-targets --no-run` builds 24 binaries;
  under each crate's `tests/` at most one file does not call
  `session(` and no `session(` file is a `mod` of it; `tools/arch-check.sh` refuses the two layout breakers.
- `.config/nextest.toml` exists with `retries = 0` in both profiles;
  both gate workflows install nextest and run it; `AGENTS.md`'s gate
  names `cargo nextest run --workspace --all-targets`; C<next> is recorded
  with its *Pinned* paths; the checklist row and the testing-notes
  paragraph exist.
- The gate, as `AGENTS.md` then lists it, is green on the step's own
  tree; `cargo test --workspace --all-targets` is green too; both
  references and every fixture regenerated are named in their commit
  messages.

## Not in scope
- The journey test, real-shaped mock items and the normalisation of
  job ids in goldens: step 2c.
- The daemon and limiter file splits: step 2d.
- A test-kit crate, a diff dependency, a `testkit` feature on the
  client: declined in the design's section 5.
- Any change to production code (`src/` of any crate), including a fix
  for a fault a new test finds: report it, ignored with its reason.
- Deleting or thinning a test appendix A does not name; loosening an
  assertion to make a golden pass; a golden with no facts; a retry
  count above zero anywhere.
- Running both runners in the gate; removing `cargo test` support.

## Rules
`TEMPLATE.md`'s rules, and: the main tree, one worker, items in order;
every `acq` the tests spawn runs in its own mock world through the
harness and you run none by hand; `cargo build --workspace` before any
test run and after any edit to a contract input (the manifests and the
lock change in item 2); never run the live drivers while the gate
runs; a size or a time stated in a message is measured after the last
edit; the design is taken verbatim unless the tree disproves a
sentence, each correction named in the report.

## Report
`TEMPLATE.md`'s six sections, with: under 1 the seven commits and the
review's; under 2 the gate per commit as statuses, and for item 6 both
runners; under 3 appendix D's table with the "after" column measured
(binaries, `cargo test` wall, nextest wall local and full, the search
relink, the test count, lines of test Rust removed and goldens
committed); under 5 every shape built otherwise than drawn and every
test outside appendix A that changed, with why; under 6 what got in
the way, and your own judgement on two questions: whether the ten-row
golden set and the fact words hold a seat finding as well as the
literals did, and whether one binary per crate changed your edit loop
as the design claims. The review's findings whole as an appendix.
