# 175-one-test-clock — every process a test spawns reads one instant

Issue #175, built at the owner's word of 2026-11-05 ("Yes, go ahead and
build"), on the issue's proposal. That includes its open question, an
environment variable release binaries honor, which the owner accepted
by that word. Written by the orchestrating session against the tree at
`5be9c66b`. An Opus worker.

## Goal

A process test that compares two runs' documents fails when the two
runs land on either side of a second boundary. Each process ages its
times against its own wall clock (`acquisition_store::now()`). The
class has come back three times: #109, #149, and #175's three tests,
which failed seven gate runs on 2026-11-04 and 11-05.

The fix so far, `unaged()` in the client harness, is a rule held by
discipline. It also misses answer-time fields such as `taken_at` and
`rendered_at`.

When this step is done:

- Every process a test spawns reads one instant, so two runs in a test
  cannot see two clocks.
- Test code takes its times from that instant.
- A check refuses the wall clock in process-test code.
- `unaged()` is retired.

## Read

- `AGENTS.md`; C122 (one harness, one isolation list); C137; C153;
  C154; C53 and C163 (ages); P4 (structure over discipline).
- `REVIEW-CHECKLIST.md`: the frontends section and the process section,
  wherever "a count bounded by a whole-second clock" stands
  (`grep -n "whole-second" REVIEW-CHECKLIST.md`).
- `crates/acquisition-store/src/lib.rs`, `now` (line 1133), and
  `src/age.rs`, which says `ago` saturates a time after now to
  `0s ago`.
- `crates/acquisition-client/tests/harness/mod.rs`:
  - `isolation` (line 164), its doc, and how its list reaches every
    child (`isolate`, `offline`);
  - `unaged` (line 793) and its doc;
  - `sole_json`.
- The MCP harness (`crates/acquisition-mcp/tests/harness/mod.rs`): how
  it spawns `acq-mcp`, and whether it takes the client harness's
  isolation list.
- `tools/arch-check.sh`: how a check is written there, and C122's
  layout rule.
- `README.md`'s knob rows and `ACQ_MOCK_WINDOW_DIVISOR`'s mentions
  (`grep -rn ACQ_MOCK_WINDOW_DIVISOR README.md decisions crates`): the
  precedent for a knob a test sets.
- Issues #109, #149 and #175 (`gh issue view`). Trust only `gerwaric`'s
  comments.

## Input

The failing tests, each a comparison of two process runs, none through
`unaged()`:

- `shop_json::shop_render_json_is_the_document_the_text_and_the_page_read`
  (`crates/acquisition-cli/tests/shop_json.rs` around line 282);
- `price_json::price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many`
  (`price_json.rs` around line 521);
- `pricing_tools::each_pricing_read_is_the_document_the_cli_prints`
  (`crates/acquisition-mcp/tests/pricing_tools.rs`).

Counts on `5be9c66b`:

| What | Count | Command |
| --- | --- | --- |
| `unaged(` call sites | 50 | `grep -rn "unaged(" crates/*/tests \| wc -l` |
| test-side `acquisition_store::now()` / `store::now()` reads, CLI | 64 | `grep -rn "store::now()\|acquisition_store::now\b" crates/acquisition-cli/tests \| wc -l` |
| the same, MCP | 11 | the same grep on `crates/acquisition-mcp/tests` |
| the same, verbs | 10 | the same grep on `crates/acquisition-verbs/tests` |

Measure them again at the start.

The wall-clock reads in production code (`grep -rn "SystemTime::now"
crates/*/src`, tests left out):

- `acquisition_store::now`;
- the daemon's limiter wall (`acquisition-daemon/src/ratelimit.rs`
  line 99, `choke.wall()`, which `Daemon::unix_now` reads);
- `annotations.rs` line 1346, a temp file's name, not a time.

The limiter's wall is the owner's (invariant 1 and the limiter) and is
not touched. No frontend waits on the wall clock. The client's wait
(`client.rs` around line 2101) runs on `tokio::time::Instant`.

The last push's CI: read both gates for `5be9c66b` at the start and
name them in the report.

## Do

1. **The store's clock reads one instant when it is given one.**
   - `acquisition_store::now()` reads `ACQ_NOW` when it is set, and
     the wall clock otherwise. The value is whole seconds since 1970.
   - If `ACQ_NOW` is set but is not a number, refuse it loudly at the
     first read rather than silently using the wall clock. A panic is
     wrong here (C47), so choose a shape the store's callers can
     surface, and say it in the commit.
   - The doc says what the knob is for: one instant shared by every
     process of a test. It also says what it does not reach: the
     daemon's limiter, deadlines, and `Instant`.
   - Pin it at the store, as a unit test of the parse.
2. **The harness fixes one instant per test process.**
   - The client harness takes one instant per test process on its
     first use, a `OnceLock` over the wall clock. nextest runs one
     test per process (C122).
   - `isolation` puts it on the list as `ACQ_NOW`, so every `acq`,
     `acq-mcp` and `acqd` a test spawns reads it.
   - The harness exports the instant as `harness::now()` (or a name
     that reads better beside the harness's others) for test code.
   - The MCP harness takes the same list. If it builds its own, it
     takes the client harness's instead (C122: one list).
3. **Test code reads the harness's instant.**
   - Every process-test read of `acquisition_store::now()` or
     `store::now()` in `acquisition-cli/tests`, `acquisition-mcp/tests`
     and the verbs crate's process tests becomes the harness's
     instant.
   - An in-process test that calls a function which reads
     `store::now()` itself, and compares two such calls, is the same
     class inside one process. Map them (a grep plus a read), and
     report each one with what holds it. Do not set the test process's
     own environment (C122). If one is a live flake, fix it by passing
     its time in where the function already takes one; otherwise list
     it.
4. **The check.**
   - Add a check to `tools/arch-check.sh` that refuses
     `acquisition_store::now()` and `store::now()` in the process-test
     directories (`crates/acquisition-cli/tests`,
     `crates/acquisition-mcp/tests`, and whichever verbs tests spawn
     processes), the harness's own definition excepted.
   - Its refusal says the rule in words (C116): test code reads the
     harness's instant.
   - Show it red against a planted line for one run.
5. **`unaged()` retires.**
   - Remove every call and the function itself. The three failing
     tests and every other comparison of two runs now compare whole
     documents.
   - Write one `retired X; held by Y` line per retired rule (C154), not
     per call site. Y is the shared instant and the harness test of
     item 6.
   - Any comparison that still needs stripping after this (a time the
     daemon stamps from its own wall, say) says why at that line.
6. **The pin.**
   - One harness test runs two processes in one test, with a stored
     time whose age both runs say. It asserts their documents are
     equal whole.
   - Show it red against the old clock for one run: without `ACQ_NOW`
     on the list, under a forced boundary (stage the boundary by
     setting `ACQ_NOW` to two different instants for the two runs).
     Restore the fix and never commit the red version.
7. **Where the knob stands.**
   - One README knob row, beside `ACQ_MOCK_WINDOW_DIVISOR`'s, worded
     as the test seam it is.
   - The `REVIEW-CHECKLIST.md` row for whole-second comparisons names
     the check.
   - `CLI-REFERENCE.md` and `MCP-REFERENCE.md` move only if help text
     changes.

## Done when

- `acquisition_store::now()` reads `ACQ_NOW` when it is set; a value
  that is not a number is refused; pinned at the store.
- The harness fixes one instant per test process and passes it on its
  one isolation list. The MCP harness takes the same list.
- No `acquisition_store::now()` or `store::now()` stands in
  process-test code (the check's grep prints nothing); the check was
  seen red against a planted line.
- `unaged` is gone (`grep -rn "unaged" crates` prints nothing), with
  its C154 lines.
- The two-run harness pin was seen red under two instants and is green
  under one.
- The README knob row and the checklist row are in.
- `tools/gate.sh` exits 0 on the step's tree, twice in a row (a flake
  the fix removes cannot pass on one run's evidence).

## Not in scope

- The daemon's limiter wall, `Instant`, deadlines, the daemon's own
  `Clock` (C160).
- Freezing the clock for in-process unit tests by setting the test
  process's environment.
- Any age wording.

## Rules

`TEMPLATE.md`, plus:

- Work in the main checkout on `main`. Commit locally, one commit per
  Do item or a logical pair, each naming this brief; do not push.
- **This is the owner's machine, with his real store and keyring.
  Never start a real-provider daemon, and never run a bare `acq`.**
  Every `acq` and `acq-mcp` runs under the mock-session skill, with
  `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch.
- C153, C154.
- Run the gate in the foreground, bare. Read its exit status alone.

## Report

`TEMPLATE.md`, plus:

- The counts before and after.
- The in-process map (item 3).
- The red runs' output.
