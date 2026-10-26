# Codex review of 893677a3 (the fix round for tempdir-b350125b)

`codex exec --sandbox read-only`, codex-cli 0.164.0, roadmap/briefs/REVIEW-PROMPT.md plus three done-when lines (no brief), 2026-10-26. Verbatim:

893677a3 — done-when does not hold: permission cleanup remains incomplete, and scoped racers can deadlock on thread-creation failure; execution-dependent checks are not verified.

- **[P2] [store world.rs:1205](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/world.rs:1205), [CLI world.rs:644](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/world.rs:644), shapes 1 and 3:** Both sites still depend on a successful intervening call to restore directory permissions. In the converted store test, a regression that leaves `"shut"` at `0000` makes the assertion fail and prevents recursive cleanup from opening that directory. In the explicitly retained CLI case, a failed spawn makes `acq_in()` panic before restoration, leaving a populated `0555` directory. These share one cause: permission restoration lacks an unwind guard. Add local restoration guards, keeping the store guard alive through its mode assertion so restoration cannot hide a regression; the shared process harness can remain unchanged.

- **[P3] [store world.rs:1402](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/world.rs:1402):** Scoped joining introduces a deadlock if the second racer cannot be spawned. Concrete schedule: the first thread reaches `Barrier::wait()`, the second `s.spawn()` panics because thread creation fails, and scope unwinding waits forever for the first thread’s missing barrier partner. Use fallible scoped spawning and release or cancel the first racer before propagating the failure, preserving the barrier on successful startup.

- **Not checked:** No builds, tests, gate stages, panic experiments, or platform executions, as requested; the reported test count and cleanup results are not verified. Static inspection supports the changed annotation lock lifetimes and permission guards. `TestPath` documents the known Windows lifetime limitation, the unreadable-file exception is justified, and no goldens or references changed.

- **Nitpicks:** The second finding is an unlikely resource-exhaustion edge, though it contradicts the explicit no-deadlock requirement. The permission omissions repeat the cleanup fault this round targets.

- **Brief:** The supplied done-when lines provided sufficient scope.
- **Plan:** No plan was supplied or used as evidence.
- **Prompt:** No change needed; the process-harness exemption is explicit.
