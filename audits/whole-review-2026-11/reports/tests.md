## Test estate as a system: scouting report

### 1. Model

The estate is built around four contract surfaces (TESTING-NOTES.md "The suite's shape"):
- **The send journal on the GGG side.** Oracles live in `daemon/tests/harness.rs` and run at teardown.
- **Store and plan public APIs.**
- **The search answer.** Corpora plus proptest "generated" properties.
- **The doors.** CLI goldens with `# holds` facts, MCP field-for-field comparison, and the protocol wire fixtures.

The daemon's tests live *inside* `src/` (28k lines under `src/**/tests/`). Since t5, 115 of 228 scenarios enter through the `Door`, according to `tools/daemon-tests-census.py`. The store and plan unit tests moved to `unit/tests/`. Every other crate's integration tests are one `tests/main.rs` binary per crate. The exceptions are the files that set the process environment, which leaves about 12 integration binaries.

The gate runs nextest, one process per test (1,740 in CI), and never retries. CI runs `ACQ_CASES=full`, which is why proptest failures show up in CI that a local run misses.

Three meta-layers sit around the tests:
- The breakers, about 1,880 lines of shell that break each check and watch it fail. They run in CI only.
- Weekly cargo-mutants over 8 daemon and transport files.
- The C153/C154 rules: a finding becomes a row, and a retirement names what holds it.

The organising idea is good and written down. In practice nearly everything sits at the doors and in the GUI, and that layer keeps growing.

### 2. Measurements

| What | Value | Command |
|---|---|---|
| Test lines vs source lines (crates) | 143,568 vs 109,246; 1,723 test fns (1,676 at `3618728f`, when it was 138,238 vs 106,043) | python over `git show REV:file`; inline `#[cfg(test)]` tails counted as test lines |
| Lines added since `3618728f` | +5.3k test, +3.2k source | same |
| Test lines by crate | cli 33.7k, search 39.2k, daemon 31.4k, plan 9.1k, store 8.8k, verbs 7.0k, mcp 7.0k | same |
| Tests named by a ruling/claim id | 576 of 1,717 (34%; was 429/1,354) | regex `^(c\|n\|r\|d\|p)\d+_` over test fn names |
| CLI goldens | 74 files, 1.05 MB. 33 are `seat_faults/`; 14 `web/` goldens hold ~700 KB, including `search_pages.txt` at 277 KB (73 commits) | `git ls-files … \| xargs wc -c` |
| `.contains("` in CLI tests | 508 (456 at `3618728f`) | `git grep -h '\.contains("'` |
| Seed/fixture functions in door tests | 24 (TEST-SHAPE counted 18) | `git grep -nE 'fn (seed\|seed_…)\('` |
| Breaker shell | 1,015 lines (Oct 29), 1,807 (`3618728f`), 1,882 (HEAD) | `git show REV:tools/*breakers*` |
| Retirement lines, all time / since 11-20 | 407 / 3. One test-ish file ever deleted under `crates/` (a golden, `f87e645d`) | `git log --format=%B \| grep -c '^retired .*; held by'`; `--diff-filter=D` |
| Local full gate (228 runs, `runs/gate/verdicts.tsv`) | median 65 s, p90 111 s, max 219 s. Test stage median 39 s, doc stage p90 51 s. 31 of the 228 runs were red | python over the TSV |
| Linux CI (8 recent successful runs) | gate job 218–480 s (test step 148 s, of which the run itself is 49 s); **breakers job 590–665 s**, of which `arch-check-breakers.sh` alone is 439 s | `gh run view --json jobs` |
| Windows CI test step | 542 s | same |
| CI per-test time | 285 test-s total; 1,417 tests under 0.1 s; cli 140 s, search 104 s, daemon 14 s; slowest `new_league` at 17.8 s | `gh run view --log` on 45551188748 |
| Weekly mutants | 6 shards of about 3 h each. Each mutant costs ~77 s to build and ~4 s to test | weekly run 45221916564 log |

### 3. Where a deep review would pay (ranked)

**1. The breakers now set the CI critical path. (Verified; high confidence.)**
- TEST-SHAPE §3 proposed "the breakers replaced by a positive control inside each check". Instead they grew 85% in size.
- `arch-check-breakers.sh` "runs the check once per case" (its header) over about 130 cases. At about 7.5 min, that makes the breakers job 2.5× the gate job on every push.
- The Oct-30 correction ("ends before [the gate]") is no longer true.
- Question for the reviewer: which check properties need a mutation of the real tree, and which could be one positive-control assertion inside the check?
- Payoff: about 7–8 min per push and about 1.5k lines of shell, plus the self-hosted runner variable (`gate-linux.yml:190`).
- Read: `tools/*-breakers.sh`, `.github/workflows/gate-linux.yml:178-235`.

**2. #312 was closed, but the test still fails on Windows. (Verified.)**
- `browser::two_stat_groups_one_weighted_are_made_edited_and_run_from_the_page` hit TIMEOUT at 240 s on main at `0701b7d6` and `178d51bb`. Both commits contain the fix `51d99a3d`.
- Before the fix it FAILed on Windows 5 times (runs 45309022981, 45267847160, …).
- No open issue covers it. The 240 s is the CI profile's 60 s × 4 ceiling.
- The class is real: `Cdp::click` reads an element's place and sends the press in separate calls (`browser.rs:263`; #308 and #312 hit the same cause).
- Question: should browser tests wait on page events instead of positions, and do they belong in the Windows gate at all?
- Payoff: a green Windows gate. A hang that every main push meets is probably worth more than any line count.

**3. The doors/GUI layer is where size accumulates, and t6 (#142) is parked behind the GUI.**
- `web.rs`: 8,339 lines for 52 tests (160 lines per test), 73 commits since 11-10.
- `browser.rs`: 147 lines per test.
- The `web/` goldens are HTML of 755–1,527 lines each, mostly long lines (277 KB in one file), with `# holds` facts at a ratio of about 1 per 10 lines. A blind regeneration is caught only for those facts. A 1,959-line diff in two weeks is not reviewed in any meaningful sense (judgment).
- Questions: can page goldens hold a structural projection (forms, controls, links) instead of whole HTML? Can the 24 seeds become t6's one facts table?
- Payoff: thousands of lines and most golden bytes. Medium confidence.

**4. C153/C154 run as ceremony, not shrinkage.**
- 407 retirement lines exist, yet only one test-related file was ever deleted, and the tests-to-source ratio is still about 1.3:1.
- `seat_faults.rs` (CLI: 4,393 lines, 65 tests, 33 goldens; search: 1,789) is still the incident ledger TEST-SHAPE named.
- Example: the `ambiguous_account` kind is asserted in verbs (11 hits), cli `seat_faults.rs:260,285`, mcp `saved_tools.rs:777`, client contract, and daemon sessions/responses. The CLI test says so itself ("which failure each selector is, is the verbs crate's"), then re-asserts the list.
- The account-pick sentence lives in two goldens (`account_to_pick*.txt`) plus three test literals (`seat_faults.rs:304,346,408`).
- The "unreadable" state is in assessment candidate 6, and nothing has acted on it.
- What to measure first (judgment):
  - (a) for each frontend test, whether any lower crate's test already holds the same kind/fields: a grep join over refusal kinds;
  - (b) mutation of `acquisition-verbs` with `test_package` set to verbs, cli and mcp, recording which package kills each mutant. A frontend test that kills only mutants verbs already kills is retirement evidence with a named Y, which is exactly what C154 asks for.

**5. The real-time waits that TEST-SHAPE §1 and the t2 brief named are still there.**
- `new_league` takes 17.8 s on the mock's real stash limiter, the same 18 s as in October. TIME-SEAM was meant to be the cure.
- Small payoff (about 15 s of a 49 s run) but cheap. High confidence.

**6. Test-only knobs in production.**
- `ACQ_NOW` (`acquisition-store/src/lib.rs:1166`), `ACQ_MOCK_WINDOW_DIVISOR` and `ACQ_MOCK_DEGRADED_HEAD` in the shipped mock, the `*_WITHIN` deadlines, and `ACQ_DOOR_COUNTERS` in tests. About 29 `ACQ_*` names are read in source.
- Question: which are user knobs and which are test seams that ship in the binary?
- Low to medium payoff (concepts, not lines).

### 4. Low yield / already covered

- **The rate-limit spec** (`spec.rs`, 72 tests, 19 row tables), the journal oracles and the teardown contract. These are the estate's best idea; leave them alone and copy them.
- **Mutation testing.** It finds real gaps and is acted on the same day: the 7 missed mutants from 11-23 became #320, fixed by `9802fc2b`. C200's exclusion check landed in `15281cc4`, closing #321. Every exclusion is reasoned line by line.
- **The search's generated/metamorphic properties and the trade oracle.** They are the right shape already. t7 rightly comes last.
- **The local gate's speed.** A median of 65 s is not the bottleneck. CI and Windows are.
- **The daemon door collapse (t5).** Done. The census leaves only 3 tests with neither door nor boundary. About 30 internal asserts remain in `sessions.rs` and 22 in `login.rs`, which is minor.

### 5. Blind spots

- **Mutation scope.** Only invariants 1–5 code is mutated. Nothing measures whether the search, plan, store or verbs tests are load-bearing.
- **Timeouts counted as caught.** 15–23 per shard count as caught, about 100 of roughly 800 mutants. Whether each one is a real catch or nextest's ceiling is unexamined beyond #151.
- **Most CI runs are cancelled.** About 60% of recent gate runs end cancelled (`cancel-in-progress`), so an intermittent failure is seen far less often than it happens.
- **Not measured:** per-binary compile cost now, how big the golden normaliser is, and CPU-seconds for the breakers locally.
- **Failure classes on main, observed:** browser click races (Windows); cargo-deny advisories (7 of the last 12 Linux failures); daemon bearer concurrency races (#262; `bearer.rs` failed on Windows at `38e54eea`); real proptest finds under `ACQ_CASES=full` (`generated_equivalence` inv. 7, `generated_cross_checks_3`); Windows path/PowerShell cases (#260).

### 6. Surprises

- The breakers job grew from "beside the gate, ends first" to the critical path.
- Closing #312 did not stop the test from failing (now a TIMEOUT) on Windows.
- `decisions/shape-baseline.tsv` has nothing to do with tests. It is the registry's headline/clause-length baseline for ruling entries.
- `.contains("` literals in CLI tests went 231 (a reader's regex, Oct 29) → 456 → 508. That is the opposite of C153's direction, though the regexes differ, so the trend is better shown by 456 → 508.
- Seed functions went 18 → 24, against t6's goal of one world.
