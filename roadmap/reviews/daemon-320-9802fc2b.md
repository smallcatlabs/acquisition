9802fc2b — The corrected issue’s implementation requirements hold by inspection: all seven missed mutants are addressed.

- Findings: none. The strengthened tests reach the block guard, fractional-second rounding, and persisted reset. The ETA helper has only a test caller and is now test-only; the HEAD exclusions match the intended guard at its new location.
- Not verified: builds, tests, mutation results, gate status, and workflow execution; execution was prohibited.
- Nitpicks for unlikely cases: none.
- Brief: the correction makes the intended seven mutants clear.
- Plan: no separate plan was supplied.
- Prompt: no change needed.
