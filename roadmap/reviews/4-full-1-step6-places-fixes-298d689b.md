298d689b — fix-round done-when holds by inspection: two findings are fixed and pinned; the third is explicitly declined with a pinned counterexample. One documentation inconsistency remains.

- **P3 — crates/acquisition-search/src/places.rs:22 — shape 3.** The module documentation still promises “the two are one text,” contradicting the exception this commit documents and tests. With persisted `Standard/d1` holding an item and listed-but-empty `standard`, the bucket uses `league=Standard` while the place uses `league~…`. Smallest fix: qualify this sentence with the exception documented in `counts::place_terms`. No P1 or P2 defect found.
- **Not verified:** gate, doc build, test outcomes/counts, and browser behavior; execution was prohibited.
- **Nitpicks:** the finding above concerns wording alone and a case-variant league fixture unlikely in practice.
- **Brief:** no additional obstacle. **Plan:** none supplied. **Prompt:** no change needed.
