a0a85406 — both prior findings are fixed and pinned; the full done-when remains conditional on the gate.

- **P3 — [daemon.rs:1035](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon.rs:1035), shape 3:** The field comment still defines the count as closures that have not returned. Concrete contradictory state: the closure has returned and `blocking_probe` holds the observer, but the count remains 1. Change the comment to describe completions not yet observed by the runtime.

- **No behavioral findings:** Detached restores and dropped callers retain the observer; the guard decrements on unwind or cancellation. No current caller aborts the returned handle, so additional abort protection is unnecessary for these callers. The component pin checks the retained count and a suppressed clock advance after closure completion, distinguishing the old decrement timing. The counter reader introduces no reversed lock order. Every deadline-order wait now has `timeout(HANG, …)`. The extra scheduling hop preserves the judge’s admission check, landing serialization and restore publication; no user-facing wording or input-validation boundary changes.

- **What I did not check:** Execution evidence, including the gate and breaker results, is **not verified**; this review was read-only.

- **Nitpicks:** The sole finding is a documentation nitpick; it has no runtime effect.

- **Brief:** Do 2’s “until the closure returns” should say “until a runtime task observes completion.”
- **Plan:** Nothing obstructed this fix-round review.
- **Prompt:** Nothing obstructed the review.
