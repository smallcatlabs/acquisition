8ca96748 — the fix round’s done-when holds by inspection for both prior findings: each is fixed by one rule and pinned by a breaker case.

- Findings: none. The parser validates the entire holder list before extraction, and `show` resolves retired reference labels. The new breaker cases exercise the reported failures.
- Execution not verified: builds, the gate and breakers were not run, as requested. Reviewed input is repository-authored registry text; client requests, persisted store data and upstream responses are unaffected.
- Nitpicks: none.
- Brief: no concern with the fix round’s scope.
- Plan: no concern with the implementation.
- Prompt: nothing obstructed the review.
