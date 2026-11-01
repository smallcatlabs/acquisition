`roadmap/briefs/t5-r6.md` at `4d00ca42`, unchanged in the working tree, is buildable as drawn.

- Census finding: **Answered.** Do 5 requires one reproducible, source-backed inventory at both commits, expands independent cases, excludes table wrappers, and reports documented component tests separately; Input fixes no before scenario number.
- Timing finding: **Answered.** Input accurately preserves the ledger’s distinctions: 40 s input, 42 s worker, 22–23 s orchestrator for round 1; 24–25 s for round 2; 22–23 s for round 5c (`roadmap/LEDGER.md:63`, `:64`, `:69`).

Findings: None. The historical citations match `a4834369`: `parents.rs:107` contains ten refusal cases, `dispatch.rs:597` four retry-header cases, and `exit.rs:657` the lifecycle table. The revised census is executable through source inspection and a recorded inventory; it no longer claims attribute counting supplies the answer. No contradiction introduced by this cut was found.

What I did not check and why: I did not calculate the complete census, inspect run artifacts, verify CI or independently fetch #141; this was a confirming brief review. No builds, tests, mutants, gate or daemons were run, and no files were changed.

Nitpicks: None remaining from these two findings.

Round 6 may start as drawn.
