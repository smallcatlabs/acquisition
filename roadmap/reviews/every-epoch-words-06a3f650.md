06a3f650 — the brief’s done-when does not fully hold: the inventory and clock pins have gaps.

- **P2 — [documents.rs:3155](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/tests/documents.rs:3155), shapes 1 and 2:** The inventory omits time paths explicitly required by Input, and the fixture leaves those branches empty. For persisted facts containing a folder or substash, `items_covered_below[].basis.at` and `manual.updated_at` have no inventory rows; neither do league notes’ `list_fetched_at` paths. Consequently, removing aging from `items_covered_below` would escape both new pins on this fixture. Smallest fix: populate these branches through their public makers and add their time/words mappings to the existing inventory.

- **P2 — [documents.rs:3240](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/tests/documents.rs:3240), shape 2:** The new pins do not distinguish the plan snapshot’s required reference clock from the answer’s clock. For a plan with different snapshot, generation and answer times, replacing `before("generated_at", …)` with `ago(now, …)` for `snapshot_taken_at_age` still satisfies `said`; no other assertion checks this new field’s clock. The production expression is correct, but the brief’s named failing case is unpinned. Smallest fix: extend the existing plan-age boundary test with distinct fixed times and an exact snapshot-age assertion.

- **Not verified:** gate/test results, including `tests/next.rs` passing, red-first staging, and fixture-regeneration run counts; execution was forbidden, and the final commit cannot establish staging or run history.

- **Nitpicks:** Neither finding; both concern the structural protection this step was commissioned to provide.

- **Brief:** Distinguish frozen rendering text from required gloss edits explicitly; “no text line” conflicts with Do 5 and needs the approved exceptions.
- **Plan:** No additional impediment.
- **Prompt:** No additional impediment.
