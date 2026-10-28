# Review of #103, fix round 3, 371bb1bb

Codex, `codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md` plus the commit, 2026-10-28. Verbatim.

371bb1bb — the fix-round done-when does not fully hold: peer failures now reach the test task, but the revised test cannot compile on Windows.

- **P1 — [contract.rs:544](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/tests/contract.rs:544):** `Peer` uses client-side stream halves for an accepted server connection, breaking Windows test compilation. **Concrete trigger:** on Windows, `Listener::accept()` returns `NamedPipeServer`, while `OwnedReadHalf` and `OwnedWriteHalf` wrap `NamedPipeClient`; both field initializers therefore have incompatible types. Unix hides this because both endpoints are `UnixStream`. **Smallest fix:** declare the peer’s fields as `BufReader<ReadHalf<endpoint::Accepted>>` and `WriteHalf<endpoint::Accepted>`. This is a platform type failure, independent of client, persisted, or upstream input.
- **Not checked:** Builds, tests, gate statuses and test counts are **not verified** because execution was prohibited.
- **Nitpicks:** None; the finding blocks the Windows test build.
- **Brief:** No change suggested.
- **Plan:** No additional impediment.
- **Prompt:** No change suggested.
