# Codex review: step h3, `37fae0e5^..cc6f8dc5`

The outside review of the two h3 commits (`roadmap/briefs/h3-build.md`), run 2026-10-26 by the committing session
(`codex exec --sandbox read-only`, the prompt `roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner reads them before any fix is built.

37fae0e5^..cc6f8dc5: the brief’s done-when does not fully hold; execution-dependent criteria are **not verified**.

Findings, most severe first:

1. **P2 — [client.rs:1633](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1633), shape 1:** `winner_listens` checks the deadline before `reach().await`, so it can accept a listener after expiry or wait beyond its stated bound. **Failing schedule:** a Windows pipe connection starts just before expiry, retries while busy, then succeeds after expiry; the helper returns `true`. **Smallest fix:** bound `reach` with `timeout_at` and reject late success, as the handshake already does.

2. **P2 — [frontend.rs:223](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/frontend.rs:223), shape 6:** the generated start command can select the environment’s world instead of the frontend’s captured world. **Failing input:** construct a mock frontend over `default_root()` while `ACQ_STORE_DIR` names scratch world A; the printed command omits `ACQ_STORE_DIR` and starts in A when copied into that environment. **Smallest fix:** always include the captured root in the command, including the default root.

3. **P2 — [contract.rs:719](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/tests/contract.rs:719), shape 1:** the successful spawn-race pin still depends on machine speed, contrary to C137 and the brief’s “no wall-clock bound.” **Failing schedule:** after the loser is reaped, delay the winner’s startup beyond the isolation list’s 60-second lookup deadline; correct code returns `SpawnFailed` and the test fails. The explicit deadlines at lines 699 and 824 share this cause; calling the latter a hang bound does not remove its scheduling dependency. **Smallest fix:** inject the lookup clock and synchronize winner/reaper transitions explicitly, leaving hang termination to the test runner.

4. **P3 — [client.rs:1271](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1271):** finding the winner on the outer loop’s final iteration returns an error without handshaking that winner. **Failing schedule:** observe the loser’s exit on iteration 100, then find the winner within `winner_listens`; `continue` exhausts the loop and produces `Transport`. **Smallest fix:** carry the reached connection directly into the handshake path, independently of the startup polling budget.

What I did not check: builds, tests, gate stages or runtime/platform behavior, because execution was prohibited; green arch-check and gate results remain **not verified**. The two-world coverage is present by inspection. The choke point, single gate, dependency manifests and arch-check rules are unchanged; I did not re-audit their untouched implementations. No goldens or reference files changed.

Nitpicks: none; finding 4 requires an uncommon boundary schedule but loses a reachable winner.

Brief: distinguish deterministic lifecycle assertions from runner-level hang protection explicitly.

Plan: no change suggested.

Prompt: no change suggested.
