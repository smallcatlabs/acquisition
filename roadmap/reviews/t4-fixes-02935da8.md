e9b8c40e^..02935da8 — the fix-round done-when does not fully hold: directory tracking fixes the original defect, but the two regression-protection findings remain unresolved.

- **[P2] crates/acquisition-search/tests/build_name.rs:32 — shape 2:** The fixture tests call `digest_under` directly and never exercise Cargo’s invalidation boundary. For build-source input, restoring per-file watches would leave these fixture tests green while a warm build overlooks a newly added `src/leftover.rs.orig`. Smallest fix: add a gated regression that warms a scratch Cargo build, adds the file without modifying existing inputs, and requires the next build to refuse it.

- **[P2] crates/acquisition-search/tests/build_name.rs:47 — shape 10:** Removing the test-source exclusion case leaves test-edit isolation without a gated holder, contrary to C154; cases added during an unclosed step have no stated exception. For build-source input, moving a store test module back under `src/` would silently make its edits change the contract revision again. Smallest fix: add a structural check keeping these test modules outside digested directories and record the retirement with that holder.

- **Not verified:** execution results, gate status, test counts and timings; no builds or tests were run, as instructed. Review covered the committed diff and affected callers; client requests, persisted records and upstream responses were not exercised.

- **Nitpicks:** None; both findings concern regression protection explicitly required by the fix-round review.

- **Brief:** No changes suggested for this fix round.
- **Plan:** No changes suggested.
- **Prompt:** No changes suggested.
