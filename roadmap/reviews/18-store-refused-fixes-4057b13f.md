4057b13f — the fix round’s code-and-pin requirements hold: both prior findings are fixed by shared rules and pinned; completion remains conditional on the gate.

- Findings: None. Unknown client arguments are rejected before tool work; persisted upstream `body` content bypasses spelling rewrites, with whole-body equality pinned.
- Not checked: Builds, tests, gate exit status and runtime size checks are **not verified**, because execution was prohibited.
- Nitpicks: None.
- Brief: No change needed for this fix round.
- Plan: Nothing obstructed the review.
- Prompt: No change suggested.
