8129f6ba — all three prior findings are addressed; the fix round’s done-when holds by inspection apart from execution evidence.

- **Findings:** None. The read pin includes both omitted commands, both frontends consume the shared match count, and `EARLY`’s historical clause is removed. The changed goldens agree with their fixture variations.
- **Not checked:** Tests, gate status, and runtime daemon absence are **not verified**; execution was prohibited. No production source or transport path changed.
- **Nitpicks:** None raised.
- **Brief:** Nothing impeded this fix-round review.
- **Plan:** No separate plan was supplied; the brief and prior findings were sufficient.
- **Prompt:** No changes suggested.
