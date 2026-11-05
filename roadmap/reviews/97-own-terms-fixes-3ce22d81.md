3ce22d81 — the fix round’s done-when does not fully hold: stale-plan recovery is fixed and pinned, but counts recovery remains incomplete.

- **[P3] [web/mod.rs:453](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:453), shapes 1 and 4:** The counts encoding still drops realm offers for inputs the form supports. Concrete **client input**, against a **persisted store holding multiple realms**: `/search?text=class%3Arings&view.count=line%3Aresist%2Clife`. `Counts::lists()` produces two lists, emitted as repeated `view.count` fields; the fold retains only the last, so equality fails and every link disappears. The same round-trip mismatch affects `view.count=Rarity`: serialization canonicalizes it to `rarity`, which fails literal equality. Smallest fix: emit one form-compatible count list, handle equivalent key spellings consistently, and extend the existing followed-link test with both inputs.

- **Not checked:** Builds, tests, fixture regeneration and gate results are **not verified**, because execution was prohibited.
- **Nitpicks:** The mixed-case variant is unlikely in practice; the multi-narrowing spelling is explicitly advertised by the form.
- **Brief:** No impediment to assessing this fix round.
- **Plan:** No separate plan was supplied.
- **Prompt:** No suggested change.
