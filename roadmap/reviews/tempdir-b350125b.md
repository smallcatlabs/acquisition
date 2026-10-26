# Codex review of b350125b (test temp folders)

`codex exec --sandbox read-only`, codex-cli 0.164.0, roadmap/briefs/REVIEW-PROMPT.md plus four done-when lines (no brief), 2026-10-26. Verbatim:

b350125b — the done-when does not hold: cleanup still fails on Windows and on some panic paths; execution-dependent checks are not verified.

- **[P2] [exit.rs:323](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:323), shape 1:** `_db` drops before `daemon`, so even this passing test leaves its directory on Windows. The daemon still owns the SQLite connection when `TempDir` attempts deletion; SQLite disallows deleting its open file, and `TempDir::drop` silently ignores the failure. The login fixtures and store directories declared after their daemons have the same lifetime problem. Smallest sufficient fix: make fixtures close all database users before dropping directory guards, including during unwinding.

- **[P2] [index.rs:701](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/index.rs:701), shape 3:** the directory guard can disappear while detached writer threads still use its path. Concrete failing schedule: the first writer panics, `w.join().unwrap()` unwinds, the directory is removed, and the second writer resumes inside `write_temp()`, whose `create_dir_all()` recreates it and writes `accounts.json`. This shares the preceding finding’s cause: directory ownership does not cover every file user’s lifetime. Smallest fix here: use scoped threads so all workers finish before the directory guard drops; audit the raw thread spawns in the converted annotation tests too.

- **[P2] [diagnostics.rs:29](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/diagnostics.rs:29), shapes 1 and 3:** permission restoration remains success-only, so `TempDir` cannot clean a populated directory left non-writable by a panic. For example, an unexpected error from `rotate_if_over()` panics at line 30 while the directory is mode `0500`; recursive deletion then fails for an ordinary Unix user. The same fault occurs if `valid_access_token(...).await.unwrap()` fails between `freeze(true)` and `freeze(false)` in the refresh tests. Smallest fix: restore permissions through a guard that drops before the directory owner.

- **Not checked:** no builds, tests, gate stages, or platform executions were performed, as requested. Cross-platform execution results, the resolved feature graph for C84, and the historical disk-usage figures are not verified. Static inspection confirms the frozen process harness and production `world.rs` temp-directory uses are unchanged; the search tests requiring actual files retain them. No goldens or reference documents changed.

- **Nitpicks:** none; these failures directly contradict the requested cleanup guarantees.

- **Brief:** no brief was supplied; the replacement done-when lines were sufficient.
- **Plan:** no plan was supplied or used as evidence.
- **Prompt:** explicitly exempt the frozen process harness from the PID/counter ban, since its required unchanged implementation uses both.
