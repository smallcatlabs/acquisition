# Which tests are load-bearing

The plan's E1. Investigator's report: `reports/investigators/tests.md`. Portfolio: proposals 1 and 4.

## What was found

The investigator read sixty tests whole, by a stated rule, from the web tests, the other CLI and MCP tests, and the daemon's.

| Sample of 20 | Load-bearing | Duplicate of a lower pin | Pins spelling | Pins a mechanism | Would fail on a harmless rewording |
| --- | --- | --- | --- | --- | --- |
| `cli/tests/web.rs` | 14 | 0 | 6 | 0 | 16 |
| other CLI and MCP | 15 | 2 | 3 | 0 | about 11 |
| daemon | 16 | 1 | 0 | 3 | 6 |

Every one of the sixty could fail on the bug it is named for. Three quarters are load-bearing. So the estate cannot be halved by retiring tests: C154 conserves properties by design, and has produced 407 retirement lines and one deleted file.

The size is in the shape. Each frontend finding is a scenario over its own hand-built world: 34 seed functions, 148 `store.record(` calls and 80 item literals in the frontend tests; `web.rs` runs 160 lines per test.

The strongest tests share one form: a table of inputs against an oracle written from a ruling or a ground-truth claim, never from the code's output. That is 5 to 7% of the estate by lines, nearly all of it in the daemon and the search.

## Checked by the orchestrator

- Goldens: 74 files, 1,049,903 bytes, 829 `# holds` lines; the web's are 795,121 bytes.
- `web/search_form.txt` is 63,988 bytes with no fact; `panes_golden` takes none.
- `web/search_pages.txt` has 73 commits since 2026-10-12.
- `keyring_round_trip_survives_a_fresh_entry` returns early without `ACQ_KEYRING_ROUNDTRIP`.
- Not checked: the sixty classifications, which are the investigator's judgment; the search, store, plan and verbs tests (about 64k lines) were not sampled.

## Where this parts from the plan

- The advisor's T8 says the frontends re-pin facts a lower crate holds. In the sample none of twenty web tests did; they test the adapter. The spelling half of T8 is right.
- The tests scout called C153 and C154 ceremony. The retirements are real; they move tests into tables. Deletion alone will never shrink this.
- The cost of the estate is not CI time (the test stage's median is 39 s). It is rewrite friction: over the last 300 commits, 0.9 lines of test and golden changed per line of source, and one golden file took 73 commits.

## Position

A test earns its keep when its oracle is not the code's own output and it is the lowest test that fault can fail. The code's own words are held once per surface by one golden of named facts; any other literal of them is spelling.

The estate shrinks by shape, and proposal 1 is what makes the shape possible: one request corpus run through three doors and compared as documents; one world with variant rows in place of 34 seeds; page goldens that hold structure (forms with their fields, links, statuses); the implicit machines as exhaustive tables. The investigator's estimate is `web.rs` from 8.3k to about 3k lines, `seat_faults.rs` (4,393 lines) to corpus rows, and web goldens from about 800 KB to tens.

The smallest bug the cut suite would miss: a fault that shows only on a page, in a world shape the corpus lacks. Each such world must become a variant row.

The invariant pins are untouched. All fifteen tests `AGENTS.md` names exist. The harness's own self-tests stay.

One measurement is worth its cost: the weekly mutation run already uploads logs that should name every test that killed each mutant. Read as a matrix, they would say which daemon tests no mutant needs, by evidence. It covers 7% of the source.

## Route

Proposals 1 and 4. Issues drafted: `a-golden-with-no-facts`, `a-skipped-keyring-test-reads-as-a-pass`, `mutant-timeouts-count-as-caught`.

## After the skeptic

The skeptic did not attack this track directly. Its point on thesis 1 applies: a corpus compared as documents cannot hold what a door adds (exit codes, escaping, an HTTP status), so those tests stay beside it.
