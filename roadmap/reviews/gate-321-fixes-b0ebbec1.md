7b45174c^..b0ebbec1 — both earlier findings are fixed and pinned; the issue’s done-when still has one gap.

- **P3 — tools/arch-check.sh:1165, shape 1:** The check accepts comment text as an operator or guard, so an exclusion can pass after its code disappears. For repository source input, append `/*` to the line immediately before the excluded `ChokePoint::head` arm and `*/` after that arm’s closing brace, preserving every line and column: the guard comparison still succeeds, although that guard no longer exists as code. Ensure the named position is outside a comment, refusing contexts the check cannot establish, and add a breaker that preserves the exclusion’s coordinates while commenting out its code. This affects neither client, persisted, nor upstream input.

- **Not checked:** Builds, gate, and breaker execution — not verified, as requested; review was read-only.

- **Nitpicks:** The P3 finding requires an uncommon edit that preserves coordinates while commenting out code; I would call it a nitpick in practice, though it contradicts the stated property.

- **Brief:** The issue supplied sufficient scope and acceptance criteria.
- **Plan:** No separate plan was supplied.
- **Prompt:** Nothing impeded the review; no change suggested.
