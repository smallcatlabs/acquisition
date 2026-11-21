c1ebbf1f — Batch 2’s done-when is not fully met: two history passages remain outside the brief’s exceptions.

- **P3 — `crates/acquisition-daemon/src/daemon/queue.rs:1764`, shapes 3/11:** The dated “realm’s 59 characters” fact remains classified as lacking a claim, although N42 explicitly records 59 entries from `GET /character`. Replace the dated ledger citation with N42 and correct the retained-match accounting.

- **P3 — `crates/acquisition-daemon/src/rails.rs:62`, shape 11:** Removing the dates leaves the former journal-schema ledger intact: “an earlier daemon’s journal” still describes the predecessor `runtime` and `build` fields. The brief explicitly cuts version ledgers whole. Remove that aside while keeping the current header’s fields and the harness’s `null` explanation.

- Gates, tests, documentation build and CI: **not verified**; execution was prohibited. Other batches and the final close are outside this commit.

- Both findings are wording-only nitpicks with no runtime effect, but directly concern this step’s removal rule. No executable changes or lost safety rationale were found.

- Brief: no blocker; its batch criteria were usable independently of the whole-step close. Plan and prompt: no changes suggested.
