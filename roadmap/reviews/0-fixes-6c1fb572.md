# Review of phase 0's fix round, 3f80f0e9..6c1fb572

Codex (gpt-6.1-sol, high reasoning), launched 2026-10-20 from the agent shell with `codex exec --sandbox read-only` and the prompt at `roadmap/briefs/REVIEW-PROMPT.md`, naming the range as one step and the parents holding the deleted review files; 146,685 tokens. The range was written `3f80f0e9..6c1fb572`, which in git's sense leaves out 3f80f0e9 itself; the launcher meant it inclusive, and the reviewer's last line says it read it so. Its answer is verbatim below. No fix is built before the owner reads it.

3f80f0e9..6c1fb572 — done-when **not fully verified**: fixes and owner acceptance of 0a’s revised bound are recorded; validation and test gaps remain below, and execution-dependent criteria are **not verified**.

Findings, most severe first:

1. **P2 — crates/acquisition-mcp/tests/tool_refusals.rs:236:** The shutdown regression test waits indefinitely and owns an unguarded child, repeating the daemon-split checklist’s unbounded-wait fault. Input: the existing `not JSON\n` followed by EOF, with a regression that stalls server shutdown, hangs the gate forever. Smallest fix: bound the exit/output wait using the harness’s deadline and kill and reap the child on timeout or unwinding.

2. **P3 — crates/acquisition-mcp/src/transport.rs:170 — shape 1:** Reading `id` as any JSON value and treating every object without one as a notification does not validate the recovered envelope. With `D` denoting 128 arrays around `0`, `{"jsonrpc":"2.0","id":{},"method":"tools/call","params":D}` produces a response carrying the invalid `id:{}`, while `{"jsonrpc":"2.0","params":D}` is silently discarded despite lacking a method. These share one cause. Smallest fix: validate the recovered version, method and request-ID type before choosing a tool refusal or notification; invalid envelopes receive `-32600` with a null ID.

3. **P3 — crates/acquisition-mcp/src/transport.rs:155 — shape 1:** `IgnoredAny` does not establish that the complete input is valid UTF-8 JSON because its string-skipping path does not validate UTF-8. Input: a `tools/call` with ID `7` and 128 nested arrays around a quoted raw byte `FF` in `params` receives an `arguments` refusal under ID `7`, rather than `-32700` with a null ID. Smallest fix: validate UTF-8 before accepting the syntax-only pass, and add this byte fixture.

4. **P3 — crates/acquisition-store/src/lib.rs:3261 — shape 2:** Sleeping and checking `is_finished()` does not prove the opener performed its preliminary v8 read before the writer commits v99. Schedule: delay the opener until after the 300 ms sleep; its first read sees v99, and the test passes even with the under-lock reread removed. Smallest fix: synchronize after the preliminary read before permitting the writer to commit.

What I did not check and why: No builds, tests, binaries, scripts, benchmarks or live calls, as instructed. Gate success, timing reproducibility, measurement isolation, platform behavior and runtime schedules are **not verified**. No invariant bypass was identified in the inspected changes and callers.

Nitpicks: Findings 2 and 3 are unlikely with conforming clients and are practical protocol-edge nitpicks. Findings 1 and 4 concern regression-test reliability.

Brief: Define malformed envelope and byte-level inputs explicitly alongside the depth fixtures.

Plan: No additional obstacle identified.

Prompt: Specify whether the first SHA is inclusive; Git’s `A..B` excludes A, although the supplied parent list suggests reviewing its correction too.
