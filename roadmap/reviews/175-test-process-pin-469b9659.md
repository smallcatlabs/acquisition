# Review of the test process pin at 469b9659

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

469b9659^..469b9659 — #175’s clock goal holds in source for harness processes, but the commit introduces a supported-runner regression.

- **P2 — [lib.rs:2992](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/unit/tests/lib.rs:2992):** The two new tests require incompatible states of the same process-global `OnceLock`, so they cannot both pass under the supported `cargo test` fallback. With `ACQ_NOW` unset, `cargo test -p acquisition-store --lib a_pin` selects both: an unpinned read first makes the successful-pin assertion fail; a successful pin first makes the refusal assertion fail. This concerns test execution, not client, persisted, or upstream input. Run each case in a fresh child process using the existing self-spawn pattern; no additional production clock machinery is needed.

- **Not checked:** Runtime behaviour, gate results, and test counts are **not verified**; execution was prohibited.

- **Nitpicks:** None; the finding affects an explicitly supported runner.

- **Brief:** The commit’s stated fault and rule provided sufficient scope.
- **Plan:** No separate plan was supplied or needed.
- **Prompt:** Nothing impeded the review.
