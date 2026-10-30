`3a251693^..8b0371be` — the fix-round’s done-when holds by static inspection against the supplied fates.

- Findings: none. P2’s compiler selection is fixed and pinned; P3 remains declined as instructed. The extract tables match the committed tree, historical citations resolve to the named parent, and the two deleted extracts match their historical blobs. All four affected breaker suites truncate by characters.
- Execution evidence—gate statuses, breaker results and counts—**not verified**; no builds or tests were run.
- I did not regenerate extracts or inspect private stores because this round changes their retention and citations. No runtime client-input, persisted-input or upstream-input handling changed.
- Nitpicks: none.
- Brief: record the accepted P3 exception and revised #135 fate so its original instructions no longer conflict with the approved outcome.
- Plan: no impediment.
- Prompt: no change suggested.
