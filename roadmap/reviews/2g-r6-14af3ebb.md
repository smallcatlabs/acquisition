a89e4704^..14af3ebb — Round 6’s inventory is implemented, but its done-when is incomplete: some document makers lack the required age pins.

- **P2 — `crates/acquisition-store/src/age.rs:45`**: Both `ago` and `before` subtract timestamps unchecked, newly exposing JSON reads to overflow. An accepted account-index entry with `last_login: -9223372036854775808` makes `accounts` panic with overflow checks enabled, or report `0s ago` after wrapping. Use `saturating_sub` in both helpers and pin the extreme timestamp cases once.

- **P3 — `crates/acquisition-verbs/tests/documents.rs:964` — shape 2**: The required per-maker tests do not pin the ages produced by `Events::read`, `RefusedBodies::read`, or `PolicyShown::of`. The event test checks counts, the refused-document test reads an empty store, and the policy’s frontend check accepts any string ending in `" ago"`; omitted event/refused ages or a policy age always saying `"0s ago"` escape those checks. Add populated, fixed-time assertions on these three makers’ serialized documents.

- **Not checked:** Gate status, test execution and counts are **not verified**; execution was prohibited.
- **Nitpicks:** Finding 1 concerns an unlikely corrupt-store input, but the panic makes it more than stylistic. Finding 2 is an explicit completion requirement.
- **Brief:** Clarify whether conflict `current` must remain literally identical to `price_show.manual` after presentation ages are added; the retained wording and weakened equality tests now disagree.
- **Plan:** No additional obstacle.
- **Prompt:** No change suggested.
