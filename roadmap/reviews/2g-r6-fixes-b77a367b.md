42fbb560^..b77a367b — P2 and P3 are fixed and pinned by inspection; option (a)’s clarification remains incomplete.

- **P3 — `crates/acquisition-plan/src/pricing.rs:953` — shape 3:** The public `describe` revision entry still promises `current` is “the live row as `manual` prints it,” retaining the ambiguity the brief now resolves. Read `describe` with `{"names":["revision"]}`, then compare a stale write’s `current` with `price_show.listing.manual`: only the latter carries `updated_at_age`. Add the brief’s age exclusion here and to the sibling API comments at lines 85 and 152; the existing equality tests already pin that distinction.

- **Not checked:** Gate status, test execution and counts are **not verified**; execution was prohibited.
- **Nitpicks:** The finding is a documentation nitpick for consumers reading only the value and revision; literal whole-object comparisons expose it.
- **Brief:** Option (a) is clear, but its correction should also reach the public description.
- **Plan:** No obstacle.
- **Prompt:** No change suggested.
