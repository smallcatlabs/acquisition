# The test read — result

Note 36, step 1. One question per test: what change to the product
makes this fail, and is that change a boundary? Read at `d1583b2e` by
four runners, one group each, from the source alone; nothing was run.
The classes were proposals. Since 2026-10-13 the owner's verdicts are
in: boundary and harness "as classed", fifteen mechanism tests
accepted. The twenty that needed work have theirs: strengthened, rewritten at
the driving surface, or renamed to what they hold, each row saying
which; three are kept or reclassed on the session's call.

| Group | Tests | boundary | weak | mechanism | harness | cannot-fail | duplicate | Rows | Detail |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| daemon | 144 | 124 | 10 | 8 | 2 | 0 | 0 | `data/read-daemon.tsv` | `README-daemon.md` |
| store, plan, protocol, client | 200 | 193 | 4 | 3 | 0 | 0 | 0 | `data/read-core.tsv` | `README-core.md` |
| search | 322 | 310 | 1 | 5 | 4 | 1 | 1 | `data/read-search.tsv` | `README-search.md` |
| CLI, MCP | 114 | 108 | 4 | 1 | 1 | 0 | 0 | `data/read-frontends.tsv` | `README-frontends.md` |
| all | 780 | 735 | 19 | 17 | 7 | 1 | 1 | | |

The input is `data/inventory-<group>.tsv`, written by `inventory.py`;
every read file has one row per inventory row, in its order (checked by
`diff` over the first three columns). No row is at low confidence; 128
are at medium, each saying why in its `note`.

## What the classes hold

- **weak (19)**: the test can fail, but a wrong product its name
  promises to catch passes it. Most are a name with two halves and
  assertions for one. Three protect a boundary a decision names:
  `acquisition-mcp/tests/ggg_refusal.rs:16` (C13: the `apply_plan` half
  is refused before the code reaches `connect`, so only `submit_job`'s
  never-spawn is held; confirmed by the reviewing session at
  `acquisition-mcp/src/main.rs:877`), `acquisition-daemon/src/rails.rs:812`
  (the journal's "without secrets" scans reports that carry no secret)
  and `acquisition-daemon/src/daemon.rs:9487` (the "without a token
  request" half is vacuous in a harness with no refresh token).
- **mechanism (17)**: the test reaches a private function, field or
  counter. Two are the only pin of a decision's property and would be
  rewritten at the driving surface, never deleted:
  `daemon.rs:7191` (C21, auth waits do not block independent keys) and
  `daemon.rs:7586` (C5, priority within a scheduling key).
- **cannot-fail (1)**: `acquisition-search/tests/language.rs:245`
  compares the test file's own list of constructs with the test's own
  corpus; no product code is asked.
- **duplicate (1)**: `generated_completion.rs:441` is `#[ignore]`, the
  check of `:433` run by hand with a tally.
- **harness (7)**: tests of a checker, a generator or the mock
  provider. Not faults.
- One test classed boundary cannot fail in the local gate by design:
  `acquisition-daemon/src/auth.rs:225` returns before any assertion
  unless `ACQ_KEYRING_ROUNDTRIP` is set, which only CI sets.
- A comment the search read met: `acquisition-search/tests/sockets.rs:379`
  works a sum as 17 and the assertion two lines down expects 18. The
  test passes, so the comment's arithmetic is what is wrong.

## The speed lever

Measured by the reviewing session, one gate run on the tree of `d7918db0`,
the owner's machine, debug build, 2026-10-12: 175 s of test
binaries for 779 passing tests. The six generated-search binaries are
87 s of it, `generated_cross_checks` alone 46 s; after them
`acquisition-client/tests/contract.rs` 16 s, `acquisition-cli/tests/search_json.rs`
16 s and `acquisition-cli/tests/daemon_observe.rs` 10.5 s.

What the read found (`data/cases-<group>.tsv`):

- 25 property tests: 18 in search, 7 in the plan crate at proptest's
  default 256.
- The knob note 36 asks for half exists. `cases()` in
  `acquisition-search/tests/common/generated.rs` reads `PROPTEST_CASES`
  for five of the six generated binaries and `next.rs`;
  `generated_routes.rs` (512), `language.rs` (2000) and `bind.rs`
  (4000) set literals that bypass it. proptest's own macro applies the
  variable over an explicit config too, so `PROPTEST_CASES=N` sets all
  25 to one number: a replacement, never a scale. Read in the vendored
  proptest 1.11 source by two runners; not run.
- Nothing in the workflow, the manifests or a cargo config sets it, so
  the local gate and CI run the same counts today.
- The table-driven search tests (`category.rs`, `ranged.rs`,
  `percentile.rs`, `pseudo.rs`) are the other weight and have no knob:
  each holds a committed table whole.

## What the runners did not do

- The generators in `acquisition-search/tests/common/generated.rs` were
  not read line by line, so what the generated properties reach is
  taken from their fixed cases and their docs.
- Seed helpers in two CLI tests and five search fixtures were skimmed,
  their assertions read.
- No near-duplicate was classed; only exact ones, and one was found.
- The 23 store tests that use the private `conn` are classed by their
  dominant assertion, boundary with a note, never split.
