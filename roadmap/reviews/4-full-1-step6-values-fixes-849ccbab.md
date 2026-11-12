849ccbab — the fix round’s done-when does not fully hold: the twelve-value bound remains incomplete.

- **P2 — `crates/acquisition-search/src/answer.rs:1149` (shapes 2, 5):** The bound counts only an explicitly supplied sort, so a sort inferred from the query can still produce thirteen values. **Failing client input:** query `ilvl>=1`, twelve named `rows.values` containing `sum("# to maximum a")` through `sum("# to maximum l")`, and no `sort`. Binding accepts twelve; `filled` then infers `ilvl` and appends it as the thirteenth. **Smallest fix:** enforce the combined bound after resolving the effective sort, refusing overflow for named values; extend the existing boundary test with this case.

Execution, gate status, test count and documentation checks: **not verified**, as requested; no build, tests or application was run. Unchanged daemon transport, concurrency and persistence internals were outside the affected paths.

Nitpicks: none; the finding concerns the explicit size contract.

Brief: add the combination of twelve named values and an inferred sort to the boundary cases. Plan: no change suggested. Prompt: no obstruction.
