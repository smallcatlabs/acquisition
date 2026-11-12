91eaf498 — the reported overflow is fixed by one shared rule and pinned; the fix round’s implementation done-when holds, with one minor retirement-coverage gap.

- **P3 — `crates/acquisition-search/tests/rows.rs:1873` (shape 10):** The replacement test discards the refusal’s stable kind, losing coverage held by the retired assertions. For client input containing thirteen valid named values, returning `unknown_name` instead of `view` would now pass. Smallest fix: preserve the error kind and assert `ErrorKind::View` for each refused case.

Execution, gate status, test count and documentation checks: **not verified**, as requested. No build, tests or application ran; unchanged daemon transport, concurrency and persistence internals were outside the affected paths. The changed logic consumes client requests, without adding persisted-input or upstream-response handling.

Nitpicks: the P3 finding above is a narrow test-coverage regression; the production refusal currently uses the correct kind.

Brief: no change suggested for this fix round. Plan: no change suggested. Prompt: no obstruction.
