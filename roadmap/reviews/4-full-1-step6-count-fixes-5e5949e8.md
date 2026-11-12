5e5949e8 — the fix round’s source-level done-when holds; both prior P2 findings are addressed.

- **Findings:** None. Form changes immediately abort the pending count and clear its display. Count folding skips recognized view fields before parsing, while retaining unknown-field refusal; the added cases pin malformed view handling.
- **Not checked:** Gate status and runtime behavior are **not verified**; this review used source inspection only, with no builds, tests, browser execution, or live traffic.
- **Nitpicks:** None.
- **Brief:** No change needed for this fix round.
- **Plan:** Nothing impeded the review.
- **Prompt:** Nothing impeded the review.
