2b002423 — #234’s done-when is partly met: both writes have CLI and MCP busy-path coverage, but the general refusal rule is incomplete.

- **P2 — `crates/acquisition-store/src/annotations.rs:675`, shape 4.** The `Db`, `Io`, and `Json` fallbacks still provide no next action. A valid price or policy write against a persisted file containing non-SQLite bytes ends with “could not be read or written: file is not a database”; no frontend adds recovery advice. Give these fallbacks a shared, actionable recovery sentence and pin a non-busy refusal.
- **P3 — `crates/acquisition-store/src/annotations.rs:633`, shape 3.** `NotCurrent` still passes through “the annotation file,” despite having no wording arm above the store. Concrete persisted input: `user.db` stamped at schema 4, two known accounts, and a valid price write naming no account; `Frontend::main_uuid` reaches this refusal through the read-only opener. Apply the same user-facing naming rule to this path and pin its frontend output.
- **P3 — `crates/acquisition-store/unit/tests/annotations.rs:711`, shape 10.** The unit test retains a wording assertion now held by the new CLI golden. Changing the recovery sentence requires editing both pins, although this test’s boundary is already covered by its `Busy` assertion and successful write after unlocking. Remove the sentence assertion.

The first two findings share an incomplete application of the proposed fallback rule.

- **Not checked:** runtime behavior, tests, gate statuses, and test counts are **not verified**; execution was prohibited. The named retirement’s replacement golden exists and is included in the gate’s test suite.
- **Nitpicks:** the duplicate wording assertion is a maintenance nitpick; the remaining “annotation file” wording is a prose defect.
- **Brief, plan, prompt:** no changes suggested.
