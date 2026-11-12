9fe51220 — the fix-round done-when does not hold: the fraction finding is fixed and pinned, but trade-value identity remains incomplete.

The findings share one cause: the reverse lookup implements a narrower display identity than the translation it delegates to.

- **P1 — [stats.rs:337](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/stats.rs:337), shapes 2 and 7:** Lookup preserves `+` before `#`, so ordinary signed stats bypass translation and retain the wrong slot. Concrete persisted upstream input: `+5 Life per 4 Dexterity`; client filter `explicit.stat_2042405614` compares `.arg1` (5), while the card falls back to `.avg` (4.5), because the indexed `+# Life per # Dexterity` differs from the derived `# Life per # Dexterity`. Use the translation’s canonical template identity, including sign removal, and pin this equality; the life test currently passes through fallback.

- **P1 — [stats.rs:359](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/stats.rs:359), shapes 2 and 7:** Requiring literal digits to match contradicts the translation’s rule that those positions remain open. Concrete persisted upstream input already held in `tests/trade-translate.toml`: `15% chance to gain Phasing for 6 seconds on Kill`; client filter `explicit.stat_2918708827` compares `.arg1` (15), but lookup rejects the duration and the card generates `.avg` (10.5). Match using the translation’s selector semantics and extend the identity test to the existing six-second fixture.

- **P2 — [stats.rs:335](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/stats.rs:335), shape 7:** Indexing only listed texts omits plural forms that the translation sums. Concrete persisted upstream input: `Drops Burning Ground while moving, dealing 2500 Fire Damage per second for 4 seconds`, already in `tests/trade-translate.toml`; client filter `explicit.stat_3595254837` yields a sum selecting both `second` and `seconds`, while the card falls back to a plural-only sum, adding a second column when clicked. Build the reverse association from the translation’s summed displays and pin identity for this existing fixture.

Not verified: tests, gate, documentation build, browser behavior and timing claims; execution was prohibited.

Nitpicks: none; the findings affect shipped stat vocabulary or supported committed fixtures.

Brief: require signed placeholders, variable literal digits and plural forms in value-identity coverage.

Plan: the reverse association should share the translation’s matching semantics as well as its value writer.

Prompt: nothing impeded the review.
