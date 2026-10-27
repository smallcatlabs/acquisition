e26871c3^..fd564e33 — the fix-round done-when does not fully hold: the group wording still overstates one result shape.

- **P2 — crates/acquisition-mcp/src/main.rs:523, shape 1:** The revised observer sentence promises `running: false` for `job_result`, but `JobResult` never serializes that field. With no daemon and a retained completed job, `job_result {"id":1}` returns the outcome without `running`. Keep all four tools in “observe only,” restrict the field promise to the three tools that emit it, and strengthen the wording pin to cover that distinction.

- **P3 — crates/acquisition-plan/src/pricing.rs:807, shape 3:** The newly selectable `status counts` entry still explains two counts through “the two words above,” whose definitions its isolated response omits. `describe {"names":["status counts"]}` returns neither `effective` nor `relation`. Replace the positional reference with explicit lookup names or self-contained explanations, and pin the isolated reading.

- **Not checked:** Execution, gate status, test count and measured wire sizes were not verified, as required by the read-only review.

- **Nitpicks:** P3 is minor because the advertised `pricing` section request includes the missing context; P2 concerns an ordinary offline result.

- **Brief:** Update Do 5’s original pricing-writer generalization to match the corrected tool groups.
- **Plan:** No change suggested.
- **Prompt:** No change suggested.
