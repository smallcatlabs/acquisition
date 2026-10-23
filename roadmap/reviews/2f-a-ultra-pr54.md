# Ultrareview of PR #54 (round A), 2026-10-22: free run 1 of 3

Two findings, both nits, both verified by the review itself. Quoted verbatim from the cloud review's output.

1. **`crates/acquisition-daemon/src/daemon/kinds.rs:595-600` (nit).** The sleep runner re-implements the `seconds` bound check and its message instead of calling `check_sleep` from the kind table, so the bound now lives in two places. The two copies already differ: the door refuses a non-number `seconds` (such as a string), but the runner silently uses 3.0. A simpler version would have the runner call `check_sleep(&params)`, or the row's `check`, and fail with its words.
2. **`crates/acquisition-daemon/src/daemon/routes.rs:182-189` (nit).** Each path param is checked twice per render. `route_for` first runs `admit_segments`, which calls `is_one_segment` on every segment param, and then `with_segments` calls `is_one_segment` again on the same values. That happens on the dispatcher's hot path, and it means the one-segment rule is enforced from two places that must stay in sync. A simpler form: keep `admit_segments` as the single gate and have `with_segments` only encode.
