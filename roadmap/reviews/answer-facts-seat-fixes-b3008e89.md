0a77a06c^..b3008e89 — done-when does not fully hold: fault 5’s counting rule remains incomplete; the other eight fixes and their pins are present.

- **[P2] [search_cmd.rs:1737](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:1737), shape 2:** The heading now counts `(none)` but still excludes the printed `(undecided)` bucket. Concrete persisted input: the existing `counts.rs` fixture contains three rarity values, five items without rarity, and an item with upstream `"rarity": 7`; its table renders five buckets beneath `4 values`. The limited view’s `shown` count is likewise short. Count both special buckets in both clauses, and extend the CLI pin with an undecided item.

- **[P3] [age.rs:93](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/age.rs:93), shape 1:** The parser promises malformed timestamps become absent but accepts impossible dates and arbitrary fractional suffixes. Upstream `startAt: "2026-02-31T00:00:00Z"`, subsequently persisted unchanged, becomes March 3; `"2026-11-02T20:00:00.garbageZ"` also becomes a start time. Validate the calendar date and require a nonempty decimal fraction before conversion; add these rejection cases to the existing parser table.

- **[P3] [listing.rs:741](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/listing.rs:741), shape 3:** `ListFilter.game_text` still documents only residue and invalid notes as text that decides nothing, although client input `game_text: "skip"` now selects operative game statements. Describe the field as game-side text and refer to `GameText`, keeping the variants’ descriptions there.

- **Not checked:** Gate results, runtime behavior, measured sizes, and reported red tests are **not verified**; execution was prohibited. Static inspection found no JSON key renames, unexplained reference changes, or changes to sending, cancellation, or persistence paths.

- **Nitpicks:** The malformed-timestamp cases and stale internal rustdoc are low-priority nitpicks. The count finding leaves the brief’s stated rule incomplete.

- **Brief:** Its golden-change whitelist omits necessary tie-winner and gloss changes, plus the resulting web league-order change; those changes have source explanations and should be explicitly allowed.

- **Scope:** Beyond the nine faults and their affected callers/fixtures, the range changes only the roadmap’s status paragraph.

- **Plan/prompt:** Nothing else obstructed the review.
