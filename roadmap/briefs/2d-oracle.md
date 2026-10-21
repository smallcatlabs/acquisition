# 2d-oracle — the journal oracles run over every journaling daemon

Proposed in `2d-design.md` section 7, item 1, and run on the owner's
word of 2026-10-21 ("ok, go ahead"). A worker reads `roadmap/README.md`,
then this brief, then what it names, and nothing else unless the work
turns it up.

## Goal

Every daemon test whose daemon writes a journal has that journal read
by the two oracles that need only the journal (`assert_wire_contract`,
`assert_pacing_follows_responses`), and the daemon's probe-first rule
exempts the routes the product never probes, as the notes and the
product's own list say. Today twelve tests run all three oracles, ten
run one or two through their own helpers, and twenty whose daemons
journal check nothing; and the rule exempts only `oauth-token` where
`Daemon::NO_PROBE_ROUTES` names `profile` and `league` too, surviving
only because no checked journal carries a send on either. After this
step a journal no test reads does not exist in the daemon crate, and
2f's reviewers read oracles that run.

## Read

- `TESTING-NOTES.md`, "The journal is the contract surface": the
  invariants table and the paragraph before it, which this step
  rewrites to what becomes true.
- `crates/acquisition-daemon/src/daemon/tests/harness.rs`: the
  teardowns `finish_harness` (`:337`), `remove_harness_files` (`:342`),
  `finish_harness_wire` (`:379`); the oracles `assert_wire_contract`
  (`:424`), `assert_pacing_follows_responses` (`:455`),
  `assert_never_over_the_limit` (`:526`); `journal_sends` (`:392`);
  `test_daemon_with` sets the journal path (`:239`).
- `crates/acquisition-daemon/src/daemon/routes.rs:140-146`:
  `NO_PROBE_ROUTES` and `route_probes`, which read the route's name by
  `crate::ratelimit::split_endpoint_key` (the part before `@`).
- `crates/acquisition-client/tests/harness/mod.rs:655-668`: the
  process-level rule, which exempts `oauth-token`, `profile` and
  `league` by the name before `@`; the daemon's rule says the same
  after this step, in the same words.
- `decisions/network.md` C20 and C32 (probes, declared route knowledge)
  and `AGENTS.md`'s "Tests pin behaviour at boundaries".

## Input

- The tree at `af5badde`; the daemon crate has 147 tests, the
  workspace 996 under nextest.
- The twenty direct calls of `remove_harness_files` outside the
  harness, by file: `dispatch.rs` 1, `idle.rs` 2, `parents.rs` 5,
  `queue.rs` 7, `quote.rs` 2, `rails.rs` 1, `refresh.rs` 2. Each is a
  teardown that reads no journal.
- The failing input the alignment needs: before the exemption changes,
  make `declared_route_knowledge_holds_against_the_mock`
  (`tests/dispatch.rs`) end through `finish_harness` with the oracles
  on; it sends `GET profile` and `GET league` with no HEAD, and
  `assert_wire_contract` must fail on the first of them. After the
  exemption names the two routes, it passes. Say in the message that
  you saw both.
- The one journal that must stay empty: `whoami_is_refused_in_real_mode_without_a_token_request`
  asserts an empty journal; an oracle over an empty journal holds
  trivially.

## Do

1. **The rule.** `assert_wire_contract` exempts a send from the
   probe-first check when `split_endpoint_key(route).0` is
   `oauth-token`, `profile` or `league`, the three names the client
   harness's rule writes, in one `matches!`; its doc comment says the
   two routes are the ones `Daemon::NO_PROBE_ROUTES` declares and cites
   C32. The `counted` half is unchanged. No other oracle changes.
2. **One teardown.** `finish_harness` reads the journal's sends through
   `journal_sends` and runs `assert_wire_contract` and
   `assert_pacing_follows_responses` over them before it aborts the
   dispatcher and removes the files; a daemon with no journal (none in
   this crate today, but the helper takes any) skips the read.
   `finish_harness_wire` keeps the wire comparison and calls
   `finish_harness` for the rest, so the two oracles have one call
   site. Every test that today calls `remove_harness_files` directly
   calls `finish_harness` instead (the twenty of the Input); the
   scenario helpers in `bearer.rs` that run the two oracles themselves
   drop those calls and end through `finish_harness`. `remove_harness_files`
   becomes private to the harness or goes.
3. **What fails, and what to do.** Run the daemon crate's tests. A
   test that fails under an oracle for the exemption is the Input's
   staged case and passes after item 1. A test that fails for any
   other reason is a finding: do not loosen the oracle and do not edit
   the test's assertions; leave that test ending through
   `remove_harness_files` with a one-line comment naming the oracle and
   the journal line it refused, and put the journal line in your
   report. The expectation is that none does; if several do and share
   a cause, stop and report rather than annotating each.
4. **The notes.** The paragraph in `TESTING-NOTES.md` before the
   invariants table says what is now true: every journaling harness
   daemon ends through `finish_harness`, which runs the two
   journal-only oracles; the wire comparison runs where a scripted
   server records (`finish_harness_wire`, the twelve); never-over-
   the-limit runs in the N6 stress alone, the one test against a
   counting mock. The table's probe-first row already names the three
   exemptions. Nothing else in the notes changes.
5. **The ledger.** The `2d-oracle` row in `roadmap/README.md` and
   "Current step" (2e next) ride in the commit.

One commit, its message naming this brief, the staged failure seen and
the count of tests whose teardown changed; the full gate bare before
it (`cargo build --workspace` first, the nextest line, then the rest),
each stage's own exit status on its own line, the commit inside an
`if` on the ten statuses. Then the review as `roadmap/README.md`
"Roles and rhythm" says, over the one commit, saved verbatim at
`roadmap/reviews/2d-oracle-<sha>.md`, committed alone, no finding
fixed.

## Done when

- `grep -n "oauth-token" crates/acquisition-daemon/src/daemon/tests/harness.rs`
  finds the exemption as one `matches!` over the name before `@` with
  the three routes; `grep -rn "remove_harness_files(" crates/acquisition-daemon/src/daemon/tests/`
  finds calls inside `harness.rs` only, or none, but for a test item 3
  annotated.
- Two teardowns, as the launching session ruled when three tests
  failed under the oracles for reasons other than the exemption:
  `finish_harness` (a scripted server or none) runs the contract and
  pacing oracles, `finish_harness_mock` (the counting mock) the contract
  and never-over-the-limit oracles, each aborting the dispatcher before
  it reads; `finish_harness_wire` is the scripted one plus the wire
  comparison; no oracle is called outside the harness. The two tests
  that forged a counted `GET` 429 to trip the rails forge an uncounted
  `HEAD` 429 instead, the one assertion literal that follows named in
  the message.
- The message says the staged failure was seen on
  `declared_route_knowledge_holds_against_the_mock` before item 1 and
  not after.
- `TESTING-NOTES.md`'s paragraph says every journaling daemon ends
  through `finish_harness`; `tools/docs-check.sh` is green.
- The daemon crate still has 147 tests; the gate green on the step's
  own tree.

## Not in scope

- The login daemons and the sessions harness, which journal nothing
  today (`RailsConfig::default()`, `ChokePoint::new()`): making them
  journal is a harness change for another step.
- The client harness's rule, which already says what item 1 says.
- Any product change: a journal line an oracle refuses is a finding
  for 2f, reported, never fixed here.
- Merging the two test harnesses, the lock inversion, the copies'
  drift (`2d-design.md` section 7).

## Rules

- Every `acq` runs in a mock world; this step needs no `acq` at all,
  and never a daemon started by hand.
- The main tree and your own scratch subdirectory, named in the
  launch; kill only the processes you started.
- The gate before the commit, gated on each stage's own exit status,
  never on a filtered grep. Never run the drivers while the gate runs.
- One commit, the story in the message, which names this brief's file;
  a count stated in a message is measured after the last edit.
- A finding is a line in the report; never a new document, never a fix.
- After the commit, launch the review, save its findings verbatim at
  `roadmap/reviews/2d-oracle-<sha>.md` and quote them in your report;
  do not fix them before the owner has read them.

## Report

Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What the gate said, as exit statuses per stage.
3. Measurements: the teardown calls before and after; which tests the
   oracles newly cover; any journal line an oracle refused, verbatim.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is wrong,
   and what would have made the job easier. Unstructured, a sentence
   each; the session that reads it answers every item in the ledger.
