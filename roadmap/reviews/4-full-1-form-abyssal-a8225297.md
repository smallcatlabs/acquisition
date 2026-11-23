0dc94ad5^..a8225297 — the brief’s functional done-when is covered by the committed code and tests; three prose findings remain.

- **P3, shape 1 — [trade-ground-truth.md:233](/Users/tom/Development/GitHub/smallcatlabs/acquisition/docs/design/trade-ground-truth.md:233):** “Every total but zc01’s” incorrectly includes empty results. The committed upstream-response extracts give zb02, zb03 and zb09 totals of zero. Smallest fix: say “Every nonzero total except zc01’s.”
- **P3, shape 1 — [trade_oracle.rs:492](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/tests/trade_oracle.rs:492):** The comment says thirteen searches fetched ten items each, but this test selects sixteen searches, four empty, and asserts 120 fetched items. Smallest fix: change “thirteen” to “twelve.”
- **P3, shape 11 — [trade_oracle.rs:451](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/tests/trade_oracle.rs:451):** The new comment puts the sitting’s date into code, contrary to the repository’s history rule. Smallest fix: describe the property under test and cite T36–T38, leaving the date in the records.

Execution, regeneration and raw-recorder hash checks: **not verified**, as requested; review was limited to committed source, extracts, goldens and affected callers.

The two test-comment findings are nitpicks. The ground-truth claim merits correction because it misstates its evidence.

The brief should name `search_pages.txt` for the filled controls; the committed split from `search_form.txt` preserves the intended coverage.
