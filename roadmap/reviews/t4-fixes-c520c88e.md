e9b8c40e^..c520c88e — the fix-round done-when does not fully hold: directory tracking fixes finding 1, but its requested regression pin is absent.

- **[P2] crates/acquisition-search/tests/build_name.rs:32 — shape 2:** The tests still call `digest_under` directly, bypassing the Cargo invalidation boundary the prior finding named. For build-source input, restoring the faulty per-file watches would leave these fixture tests green while a warm build overlooks a newly added `src/leftover.rs.orig`. Smallest fix: add a gated regression that warms a scratch Cargo build, adds the file without modifying existing inputs, and requires the next build to refuse it.

- **[P2] crates/acquisition-search/tests/build_name.rs:47 — shape 10:** The test-source exclusion case is removed without a gated replacement holding test-edit isolation; C154 provides no exception for cases added during an unclosed step. Moving a store test module back under `src/` would silently make its edits change the contract revision again, as the commit acknowledges. Smallest fix: add a structural check keeping these test modules outside digested directories, and record the retirement with that holder.

- **Not verified:** execution results, gate status, test counts and timings; no builds or tests were run, as instructed. Review covered the committed diff and affected callers, without live client, persisted-store or upstream-response exercises.

- **Nitpicks:** None; both findings concern explicitly required regression protection.

- **Brief:** No changes suggested for this fix round.
- **Plan:** No changes suggested.
- **Prompt:** No changes suggested.
