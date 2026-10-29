09277352^..2c0ac2b9 — done-when holds partly: prior findings 1–3 and the three brief corrections are addressed, but Do 1’s price surfaces remain defective.

- **[P2] [listing.rs:1762](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/listing.rs:1762):** Unreadable JSON rows reach report metadata but never enter the manual-resolution index, violating the checklist’s “fall-through past an unreadable row.” **Failing persisted input:** an item’s own price value is `{nope`, while its public tab names a price; the tab price wins instead of becoming unresolved. A readable inherited manual price can likewise win incorrectly. The same omission excludes these rows from accounting and status text. Include their addresses in `Rows.unreadable` and `RowAccounting`, and pin a damaged row on an actual subject through resolution.

- **[P2, shape 6] [price_cmd.rs:596](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/price_cmd.rs:596):** Both printed repair commands contain unsupported flags. **Failing reading:** the suggested `acq price clear --account a --realm pc --league Standard item/i-bad --if-revision 7` is rejected because `clear` accepts neither realm nor league; `set` has the same problem. Use `write_flags(account)` and pin parsing of the printed command; the new test currently asserts the invalid spelling.

- **[P2, shape 5] [listing.rs:927](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/listing.rs:927):** The new `unreadable` list bypasses output bounds and carries no total or omission count. **Failing input:** eleven damaged persisted rows are all returned by default, even with client input `limit: 1`; status also clones the entire list despite its default bound. Bound this list at the view layer, retaining whole counts and honoring `limit`/`expand`, and pin both bounded and expanded answers.

- **[P2] [price.rs:779](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/price.rs:779):** A stale write over an unreadable row loses the structured revision-conflict contract on both frontends. **Failing input:** client `price clear item/i-bad --if-revision 6` meets an unreadable persisted row at revision 7; `Unreadable` passes through `on_conflict` unchanged, producing generic `kind: "refused"` without a `revision` field. Map this case centrally to the pricing refusal with `kind: "conflict"`, the current revision, and words naming the price target; pin the resulting refusal document.

- **[P3] [lib.rs:2398](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/lib.rs:2398):** `orphaned_item_annotations` silently discards unreadable orphan rows. **Failing persisted input:** the only annotation belongs to a removed or unseen item and contains invalid JSON; this API now successfully returns an empty vector. Return unreadable orphan metadata beside readable orphans, matching the new list contract, and pin that boundary.

- **What I did not check:** Execution, test results, and gate exit statuses: **not verified**, as requested. Unchanged portions of the earlier build were not re-reviewed beyond callers needed for this range; no send path changes here. The generated MCP reference diff matches its source change.

- **Nitpicks:** Finding 5 has no current production caller and is the closest to a nitpick; the other findings affect exposed behavior.

- **Brief:** The three settled sentences now match the implementation.
- **Plan:** No change proposed.
- **Prompt:** No change proposed.
