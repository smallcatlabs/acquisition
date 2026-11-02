2d855904 — both prior findings are fixed and pinned, but the fix round does not fully hold because its new regression test introduces a concurrency defect.

- **P2 — [mockggg.rs:1135](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/mockggg.rs:1135), shape 1:** The `unsafe set_var` assumes per-test process isolation that the supported `cargo test` fallback does not provide. Concrete execution: `cargo test -p acquisition-daemon --lib` on Unix runs this mutation alongside other tests and their native libraries, invalidating its safety justification against concurrent environment reads. This concerns process-configuration input, not client, persisted, or upstream input. Smallest fix: run the probe in a filtered subprocess with the variable supplied through `Command::env`, eliminating the in-process mutation.

- Execution evidence—gate statuses, test results, timings, and census output—**not verified**: no builds, tests, or repository scripts were executed, as requested.

- Nitpicks: none; the finding affects the less-used fallback, but its unsafe precondition still requires justification.

- Brief: no obstacle.
- Plan: no obstacle.
- Prompt: no change suggested.
