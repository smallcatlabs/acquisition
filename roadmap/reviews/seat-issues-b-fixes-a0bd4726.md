ffeba345..a0bd4726 — the fix-round done-when does not fully hold; page-answer keys at a0bd4726 are a superset of those at 9a982ab2.

- **[P2] `crates/acquisition-plan/src/pricing.rs:941` — shape 1: a league-less character is mistaken for an absent target, silently losing its receipt counts.** Concrete persisted input: a live pc character with `league = NULL` and one fetched equipment item. `places_of` drops the null league, so set and clear return neither counts nor `counts_unavailable`, although `price show` includes that character in Standard. Smallest fix: retain the character’s realm independently of league membership, apply show’s league fallback, and pin this case.

- **[P2] `crates/acquisition-verbs/src/receipt.rs:66` — shapes 1 and 6: the unreadable-facts fallback promises counts through a request that cannot reliably find them after recovery.** Concrete persisted input: clear `character/ch2` while facts are unreadable, then restore facts holding it only in poe2/Standard; the offered request omits realm, defaults to pc, and refuses. Restoring a tab held in two leagues likewise leaves the offered request refusing for missing league. Smallest fix: make the follow-up resolve the held scopes at execution time before promising counts, and pin both recovery cases.

- **[P2] `crates/acquisition-cli/src/price_cmd.rs:1245` — shapes 3, 6 and 7: the final “next” sentence bypasses the scoped reads supplied immediately above it.** Concrete reading: the new `a_receipt_uncounted.txt:8` prints `acq price show --account 'Alice#1234' tab/pc/tw`, which refuses because the fixture holds that tab in Big and Standard. The target/account order parses; the missing league causes the failure. Smallest fix: reuse the receipt’s scoped reads for this sentence and exercise every printed read, including this final one.

- **[P3] `crates/acquisition-plan/src/pricing.rs:39` — shapes 1 and 3: comments still describe contracts the implementation deliberately replaced.** Concrete reading: this module documentation and line 1050 promise no reads after the write, while coverage now runs afterward; `crates/acquisition-cli/tests/shop_json.rs:258` still describes the removed page-level `pages`. Smallest fix: update these sibling comments to the landed rules.

- **What I did not check:** execution, failing-before evidence, test counts, measured sizes, and gates: **not verified**, because execution was prohibited. Static inspection found no remaining coverage error propagated after a successful mutation; the read-only clear opening remains. The page carries render context without `pages`, its text once, and CLI stdout prints the page alone. Existing populated `PriceShow` callers retain their output; reference changes match source changes, and changes stay within the ordered findings.

- **Nitpicks:** finding 4 is documentation-only. Findings 1–3 affect promised behavior, even where the triggering facts are uncommon.

- **Brief:** no further impediment after the page rule’s settlement.
- **Plan:** use one scope-aware read-offer maker for every receipt sentence.
- **Prompt:** no change needed.
