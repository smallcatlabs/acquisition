`b098e1b6^..f8ffb1ec` — All three prior findings are fixed by shared rules and pinned in source; full done-when remains conditional on the gate.

1. **[P3] [annotations.rs:574](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/annotations.rs:574):** The new exhaustion refusal reaches users in store terminology, contrary to the rule that refusals speak the verb’s words. With an item’s persisted revision at `i64::MAX`, a matching price write prints `annotation item/i1/buyout …`; policy writes similarly expose `account//sync-policy`. Translate this variant into price or sync-policy wording in the shared error adapters and pin that wording.

2. **[P3] [price.rs:636](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/price.rs:636), shape 3:** `PriceWrite`’s public documentation still directs frontends to read persisted priors through `Buyout::parse`, the bypass this round fixes. Following that instruction accepts `"12.50"` where the canonical check refuses it. Replace the instruction with `check_value::<Buyout>` and direct undo consumers to `PriceWrite::undo`.

- **Not checked:** Gate statuses, test count, and runtime behavior—not verified because execution was prohibited; network sending code was outside the affected paths.
- **Nitpicks:** Finding 1 requires a damaged revision or an impractical write count; finding 2 affects future implementation guidance, with no remaining failure in the current callers inspected.
- **Brief:** The narrowed revision guarantee now matches what per-target revisions prove.
- **Plan:** No separate plan issue.
- **Prompt:** No obstacle.
