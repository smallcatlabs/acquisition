# Investigator's report: tests

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/tests.md`.

**Track: which tests are load-bearing (read-only; no cargo, no acq).** Scripts: `<scratch>/tests/fn.py` (extracts one test), `<scratch>/tests/shape.py` (sorts tests by shape).

## 1. The picture

| Noun | Size (V) | What pays for it |
|---|---|---|
| Daemon scenarios + rate-limit spec | 28.5k test lines, 366 tests | Invariants 1–3. The oracles come from the ground-truth register, not the code: `wire_rows()` (dispatch.rs:1492, 1,953 lines of script → journal rows), `pacing_rows()` (spec.rs:611), `Life` `table()` (exit.rs:927), plus the teardown oracles |
| Search corpora | language/trade TOMLs, 395 rows, 3,017 lines; 17 proptests | Query → answer against captured site searches |
| CLI tests | 31.7k lines, 345 tests; web.rs is 8,339 lines for 52 tests, 160 lines per test | The adapter: form fold, listener security, navigation, printed commands that must run |
| Goldens | 74 files, 1.05 MB, 829 `# holds` facts (one fact per ~1,270 bytes); web/ holds ~800 KB | Wording. Only the facts survive a blind regeneration |
| Test-fn shapes (my heuristic) | 95.3k lines in test fns: 45% loop over cases, 41% single scenario, 10% golden, 3% cross-door equality, 1% proptest | — |
| Support code | ~38k test lines are not test fns (harnesses, seeds) | Hand-built worlds: 34 seed fns, 148 `store.record(` calls and 80 item literals in the frontend tests alone |

Census (V, `git ls-files` + grep `#[(tokio::)test]`; source counts include inline `#[cfg(test)]`):

| Crate | Source lines | Test lines | Tests |
|---|---|---|---|
| cli | 27,953 | 31,686 | 345 |
| search | 35,279 | 37,433 | 457 |
| daemon | 19,632 | 28,543 | 366 |
| plan | 8,991 | 9,089 | 135 |
| store | 10,037 | 8,791 | 162 |
| verbs | 7,332 | 6,676 | 98 |
| mcp | 2,905 | 7,002 | 93 |
| client | 5,196 | 3,002 | 40 |
| protocol | 1,967 | 1,064 | 21 |

Assertions (V): 7,738 `assert_eq!/assert_ne!` against 614 single-line `assert!(…contains(`. There are 1,075 `.contains("literal")` lines in all.

Per-test times: none recorded locally (V). `runs/gate/verdicts.tsv` keeps stage totals only: the test stage's median is 39 s (n=218), 41 s in the last week.

**Samples**, 60 tests read whole:
- web.rs: test #round(k·52/20), k = 1…20.
- The other CLI and MCP tests: every 19th of 386, starting at the 10th.
- The daemon: every 18th, starting at the 9th.

| Sample | Load-bearing | Duplicate of a lower pin | Pins spelling | Pins a mechanism |
|---|---|---|---|---|
| web.rs | 14 | 0 | 6 | 0 |
| CLI/MCP | 15 | 2 | 3 | 0 |
| daemon | 16 | 1 | 0 | 3 |

- **Would fail on a harmless rewording or restyle:** 16 of 20 web, ~11 of 20 CLI/MCP, 6 of 20 daemon. Mostly incidental literals inside otherwise good tests.
- **Could fail on the bug it is named for:** all 60. I found no vacuous oracle.
- **Duplicates:**
  - CLI `jobs_cmd.rs:874` re-pins `verbs/tests/documents.rs:1257-1265`.
  - `price_json.rs:1316` re-pins the plan's bound (`listing.rs:919`), and its text is also pinned in `price_cmd.rs:1759`.
  - `the_choke_point_reports_policyless_and_degraded_routes` re-reads the limiter.
- **Spelling-only, for example:**
  - `an_empty_search_form_asks_the_rows_every_other_door_asks` compares a whole `<select>` byte for byte.
  - `c53_price_list_groups…` compares ~15 exact text lines.
  - mcp `the_never_fetched_count_says…` checks a sentence that is also held in `describe.rs`, two goldens and `surface_size.rs`.

**The strongest tests share one shape:** a table of inputs (a scripted wire, headers plus history, event scripts, captured searches) against an oracle written from a ruling or a ground-truth claim, never from the code's output. By lines that is ~6–9k of 133k: the row tables (~3.4k), spec.rs's arithmetic oracles, and the TOML corpora (3k). About 5–7%.

**Goldens** (three read):
- `journey/a_first_session.txt` is good: its facts come from an independent table, `client/tests/harness/journey.rs`.
- `seat_faults/a_tab_with_no_pages_on_record.txt` holds one fact, the finding's sentence. Its ~25 other lines are pinned only by a file anyone can regenerate.
- `web/search_pages.txt` is 278 KB with 139 facts, and the facts are raw markup (`<span class="fate fate-ambiguous"`). They break on a class rename and carry no meaning.
- `panes_golden` skips the facts > 0 check (harness/mod.rs:1758-1782), so `web/search_form.txt`, 64 KB with zero facts, is pinned only by bytes.

**Invariant pins:** all 15 named tests exist (V, `git grep`). `a_cloudflare_block_admits_nothing_and_survives_a_restart` and `dispatcher_never_retries_403_or_503` are rows of `wire_rows()` (dispatch.rs:1727, :2203), not fns.

## 2. Theses

**T1. The rule.** *A test earns its keep when its oracle is not the code's own output (a ground-truth claim, the wire, another door's document, or a row written from a ruling), and when it is the lowest test whose oracle that fault can fail. The code's own words are held once per surface, by one golden of named facts; any other literal of them is spelling, and goes.*
- **What goes** (I, scaled from the samples):
  - ~16 web tests, ~96 other CLI/MCP tests and ~37 daemon tests, about 150 of 804, plus the incidental literals inside kept tests (the pool is the 1,075 literal-`contains` lines).
  - By lines: ~40% of web.rs's sampled lines, 18% of CLI/MCP's, 7% of the daemon's. That is ≈11k test lines, and the ~0.9 MB of page goldens cut down to their facts.
- **Why its edge cases go:** a fact re-asserted at a door is the lower crate's edge case. The door never owned it.
- **Smallest input whose behaviour changes:** a rewording of a refusal now edits one golden, not three files.
- **Touches:** C122 and C153, which it enforces. The invariant pins are untouched; every one is an oracle-not-output test.
- **Security, performance, function, agent ease:**
  - Security: unchanged.
  - Performance: a little less CI.
  - Function: unchanged.
  - Agent ease: better. An agent can tell which assertion is the property.
- **Strongest objection:** a literal inside a load-bearing test sometimes is the property (C143's "no `acq ` on a page"). The rule has to read "words about words" as an oracle, and that is judgment.

**T2. Half the size by shape, not by deletion.**
- **The finding:** the sample says 75% of tests are load-bearing. Retiring tests (C154: 407 `retired …; held by` lines in 122 commits, only one test file ever deleted) conserves properties by design, and can never halve this estate.
- **Where the size is:** each frontend finding is a scenario over its own hand-built world.
- **The design:**
  - (i) One door corpus: rows of (request; `acq` args; MCP tool and args; web fields) compared as JSON documents. The 13 web "answers as `acq search`" tests and the 27 uses of `assert_spoken_as` already do this, one by one.
  - (ii) One world: the journey's facts table, plus variant rows (no league list, two accounts, xbox) in place of the 34 seed fns.
  - (iii) Page goldens hold a structural projection (forms with their submitted fields, links, statuses, facts). `harness/submitted.rs` exists, and web.rs carries its own `heads`, `box_fields`, `attribute` and `main_of` readers.
  - (iv) The implicit machines (job, session, admission) as exhaustive tables, as verification.md H1/H3 argue.
- **What goes** (I): web.rs from 8.3k to ~3k lines, the 34 seeds to one builder, seat_faults.rs (4,393 lines, the incident ledger C153 names) to corpus rows, and web goldens from ~800 KB to tens of KB.
- **Smallest bug the cut suite misses:** a page-only fault in a world shape the corpus lacks. Test 2 is an example, "a store with no league list"; it survives only if its world is a variant row. Also a CSS-breaking class typo that changes no structure.
- **Touches:** C122, C123, C153. The daemon's estate stays as it is.
- **Security, performance, function, agent ease:**
  - Security: c138 and C16 stay as adapter tests.
  - Performance: fewer process spawns.
  - Function: same.
  - Agent ease: a finding becomes one row, not 160 lines.
- **Strongest objection:** the corpus becomes a second language to maintain. Agents can fit any table, and a too-generic row hides what was asked.

**T3. One measurement that lets two things go: the kill matrix.**
- **The weekly run as it stands** (V: `.cargo/mutants.toml`, run 45221916564):
  - It mutates 8.4k of 118.5k source lines (the limiter, gate, rails, kinds, provider, auth and transport).
  - ~800 mutants in six 2–3 h shards. Last run: 7 missed (#320, fixed), and ~13% timeouts counted as caught.
  - It covers none of search, plan, store, verbs, CLI or MCP.
- **Why it is cheap:** nextest runs with `fail-fast = false` (`.config/nextest.toml`), and the shards upload `mutants.out` (~13 MB). So each caught mutant's log should already name every test that killed it (I; not downloaded).
- **The measure:** parse those logs into test × mutant. A daemon test with no unique kill over those files is a C154 candidate whose Y is named by evidence.
- **What it lets go:** (a) the prose argument behind a retirement in those files, and (b) the hand "broken deliberately and seen to fail" practice for the oracles' targets.
- **Strongest objection:** it covers 7% of the source, and a unique kill over-values a test for a trivial mutant.

## 3. One thing I would defend

The harness self-tests (`door.rs` c160, `the_account_bait_records_a_request_it_cannot_read`) look like mechanism, and a tidier reader would cut them. They are the only thing standing between the scenario oracles and TESTING-NOTES' "looks like coverage, isn't". Keep them.

## 4. Where the old way is right

The daemon's regression-per-finding scenarios, run over a scripted wire with oracles taken from the register, are the old practice done well. 16 of 20 sampled are load-bearing.

## 5. Where I part from the scouts

- **T8** ("the frontends re-pin spelling"): the half about re-pinning lower facts is wrong. 0 of 20 web tests duplicate a lower pin; they test the adapter. The spelling half is right.
- **tests.md #4** ("C153 and C154 are ceremony"): partly. The retirements are real, but they move tests into tables rather than deleting them, so deletion alone cannot shrink this estate. The ratio of 508 `contains(` to everything else overstates spelling; at assertion level `assert_eq!` dominates 12:1.
- **verification.md H1:** I agree.
- **The objection that deleting tests risks the GGG relationship:**
  - CI time is not the cost. The gate's test stage median is 39 s, and CI gate-linux's median is 11.9 min with the breakers as the critical path (tests.md).
  - The real costs (V):
    - Rewrite friction: in the last 300 commits, 41k test lines plus 15.6k golden lines churned against 63k source lines, 0.9 per source line.
    - 204 of 2,156 commits touched goldens; search_pages.txt alone had 73.
    - 20 of 565 reviews flagged a test that could not catch its fault.
    - Reading: 8.3k lines of web.rs for any web change.
  - Nothing above touches an invariant pin.

## 6. What I did not check

- I did not sample the search, store, plan or verbs tests (~64k lines).
- I did not download the mutants artifacts, so the per-test kill data in their logs is unconfirmed.
- I ran no tests.
- I did not read browser.rs beyond one test.

## 7. Small finds

- `panes_golden` takes no facts, so a blind regeneration of `web/search_form.txt` (64 KB) passes.
- The `never_fetched` sentence is held in 5 places, against C153.
- The "N of M items listed" line is pinned in `price_cmd.rs:1759`, `price_json.rs:1316` and two seat_faults goldens.
- `keyring_round_trip_survives_a_fresh_entry` returns green without `ACQ_KEYRING_ROUNDTRIP`, so a skip reads as a pass.
- web.rs's HTML readers duplicate `harness/submitted.rs`.
- `assert_never_over_the_limit` takes an `_exact` argument it never uses (harness.rs:1052).
- About 13% of mutants time out and are counted as caught; whether each timeout is a real catch is unexamined.
