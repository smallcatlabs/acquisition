# Codex second look, round A fix round f3c70bde (read-only, 122,043 tokens)

f3c70bde — done-when **not verified**: no brief is named or committed; the bounded-refusal fix remains incomplete.

- **[P2] `crates/acquisition-daemon/src/daemon/server.rs:188` and `crates/acquisition-daemon/src/daemon/login.rs:392`, shapes 3, 5:** Two sibling refusal paths still echo unbounded client values, losing the structured error for requests that fit the request frame. Let `S` contain 20 MiB of backslashes: after hello, `{"req":"status","id":S}` makes serde’s wrong-type error quote the entire string; `{"req":"auth_logout","account":S}`, with at least one indexed account, forwards `Resolve::NotFound`’s entire debug-quoted selector. Each request occupies roughly 40 MiB, while either serialized error exceeds 80 MiB and the client’s 64 MiB response bound. Both share one cause: forwarding formatted errors outside the new bounded-description path. Smallest fix: bound serde error text before responding and describe the unmatched logout selector; add framed-request coverage for both. The new test calls `handle_request` directly and covers only submits, so it cannot expose these paths.

- **Not checked:** Builds, tests, quality gates, runtime behavior, and live traffic, as instructed. The redirect and authentication-order fixes assigned to stacked round C were outside this commit and were not reassessed. No goldens or reference files changed.

- **Nitpicks:** Neither remaining path is likely during ordinary use; both require unusually large malformed input. I would retain the grouped finding because it reproduces the protocol-boundary failure this round addresses.

- **Brief:** Commit a fix-round brief with explicit done-when criteria.
- **Plan:** No further change suggested.
- **Prompt:** Explicitly permit the prior findings to serve as acceptance criteria when a fix round has no brief.
