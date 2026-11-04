b55c8359 — both prior P2s are fixed by general rules and pinned, including the public-maker branches and exact three-clock case; one minor test regression remains.

- **P3 — [documents.rs:3384](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/tests/documents.rs:3384), shape 2:** Aggregating failures dropped the unconditional check that each inventory path exists. For a document derived from persisted tab facts, omitting `removed_at` from every serialized row now passes: `found` is empty, the null-only exemption suppresses failure, and the loop checks nothing. Previously this failed. Smallest fix: reject `found.is_empty()` independently of the null-only exemption.

- **Not verified:** gate/test results and claimed mutation runs; execution was forbidden. Static inspection confirms only `documents.rs` changed, with no production, reference, or transcript-golden changes and no retirements.

- **Nitpicks:** The P3 is minor: it weakens detection of a future field-omission regression; current output remains correct.

- **Brief:** No additional impediment for this fix round.
- **Plan:** No additional impediment.
- **Prompt:** No change suggested.
