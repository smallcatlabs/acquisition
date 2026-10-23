# Codex review of round D (b1d9cf3c..cf2137ef), 2026-10-23

The final message verbatim.

cf2137ef (range b1d9cf3c..cf2137ef): C50 and C136 hold by static inspection; execution-dependent completion claims are **not verified**.

- **Low — [queue.rs:10](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:10), shapes 1 and 3:** The module documentation still says “`waiting` jobs resume,” although the function documentation now names the exceptions. Concrete counterexample: a waiting `profile` row submitted by `daemon` now fails at restore. Smallest fix: replace the broad replay description with a reference to `restore_jobs`’s documentation.
- **Behavioural findings:** None found. No remaining path was identified that dispatches a previous lifetime’s login fetch: restore terminalizes it before dispatch starts; cancellation prevents replay; a failed persistence write blocks dispatch. Ordinary frontend profile jobs retain their replay behaviour, and the client door reserves the daemon’s submitter before allocating an id.
- **Not checked:** Builds, tests, the quality gate, actual shutdown/restart timing, wire traffic, or keychain behaviour, as instructed. The commit messages’ execution results remain unverified. The new regression test seeds the named waiting row directly; it does not reproduce the stop race or cover the running-row variant. No goldens or reference files changed.
- **Nitpicks:** The sole documentation finding is a nonblocking nitpick; it does not enable a send.
- **Brief:** The rulings and commit messages supplied a sufficiently concrete substitute for a brief.
- **Plan:** No separate plan was supplied; none was needed to assess this range.
- **Prompt:** No obstruction; the explicit stopping rule made the review’s scope clear.
