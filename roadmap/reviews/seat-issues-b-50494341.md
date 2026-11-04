9a982ab2..50494341 — the brief’s done-when does not fully hold; four defects remain.

- **[P1] `crates/acquisition-plan/src/pricing.rs:997` and `:1047` — a coverage failure after the mutation discards the successful write’s receipt.** Concrete persisted input: `tab/pc/tw` held live in both Standard and Big, with a price at revision 1; clearing it at revision 1 deletes the row, then `league_of` fails, so both surfaces return an error without the prior value or undo. The suggested `--league` remedy cannot be supplied to these write verbs. Smallest fix: retain the successful receipt and report unavailable coverage beside it; pin a coverage failure after a landed mutation.

- **[P2] `crates/acquisition-verbs/src/frontend.rs:360` — shapes 1 and 4: an unreadable facts file silently removes the promised counts from a container clear’s receipt.** Concrete persisted input: the existing test’s `"not a database"` facts file; the clear succeeds, but `.ok()` erases the reason, JSON omits both counts, and text says nothing about them, contrary to the count’s explanation requirement. This and the preceding finding need one coverage-unavailable receipt state carrying the reason and next action. Keep the read-only open, which correctly avoids creating or migrating the facts database.

- **[P2] `crates/acquisition-plan/src/pricing.rs:864` — shape 1: character receipts count only the default realm.** Concrete client input: set or clear `character/ch2` when persisted facts hold that character and its equipment in `poe2/Standard`; `realm_of` defaults the realm-less character address to pc, and `show_view` returns none, silently omitting the counts despite readable facts. Smallest fix: derive the character’s realm from the facts before building its report, with a non-pc character pin.

- **[P2] `crates/acquisition-plan/src/pricing.rs:1149` — shapes 5 and 7: a page answer carries the complete page payload twice.** Concrete reading: `shop_render {"page":1}` for a page near 50,000 characters returns that text under both `text` and `pages[0].text`, on MCP and CLI JSON, roughly doubling the page payload before metadata. Smallest fix: preserve the existing top-level page keys and attach render context without another complete page payload; pin a near-limit page’s serialized size.

- **What I did not check:** builds, tests, failing-before evidence, size measurements, and both gates: **not verified**, because execution was prohibited. The seat outputs were unavailable; their wording was judged against the brief’s quotations.

- **Nitpicks:** none. The multi-league failure may be uncommon, but losing a landed write’s receipt is consequential.

- **Brief:** permit render metadata beside the existing page fields without requiring a second complete page under `pages`.
- **Plan:** no other impediment.
- **Prompt:** no change needed.
