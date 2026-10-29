9e95b576^..9e95b576 — the fix round’s source-level done-when holds: the prior finding is corrected, and the owner-approved matching rule is implemented and pinned.

- **Findings:** None. Current usernames and uuids take precedence; held names match whole, case-insensitively, with duplicate holders refused as ambiguous. The daemon and frontend inherit this through `Index::resolve`. No regression or invariant bypass found in client-selector handling or persisted-index matching; upstream parsing is unchanged.
- **Not verified:** Test results and gate status; execution was prohibited.
- **Nitpicks:** None.
- **Brief:** Its original #74 instructions describe the superseded matching rule; the fix commit clearly states the owner-approved replacement.
- **Plan:** No changes suggested.
- **Prompt:** No changes suggested.
