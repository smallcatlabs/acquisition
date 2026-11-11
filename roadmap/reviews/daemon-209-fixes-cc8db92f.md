e8971a0c^..cc8db92f — the issue’s done-when holds only partially; the final restore design introduces two regressions.

- **P1 — [queue.rs:452](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:452):** The tainted-tree arm ignores persistence failure without updating `fault_on_disk`, allowing deletion of a fault marker whose job remains running on disk. Concrete persisted input: running parent 11, unknown-state child 12, running child 14, and `fault.json` naming 14; let the parent’s failure write succeed and child 14’s fail. The marker is removed; after restart, only child 14 loads, so it can replay the job that previously faulted, contrary to C134. Include this arm’s write result in `fault_on_disk` and extend the existing marker-survival test with this tree.

- **P2 — [queue.rs:446](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:446), shape 10:** The new parent failure omits the child tally required by C23, and the replacement test drops the retired test’s failed-id assertion. With the commit’s persisted fixture—parent 11, unreadable child 12, waiting sibling 14—the parent answers `children: None`; consequently, the verbs layer returns no failed-child details. Construct parent failures with a tally from the raw child relationships, preserving unreadable dependency ids, and assert that tally in the replacement test.

Execution, gate status, test counts and runtime interleavings: **not verified**, as requested for this read-only review.

Nitpicks: none; both findings affect behavior, although the first requires an uncommon combination of persisted input and write failure.

Issue: specify how unreadable-tree recovery preserves parent summaries and fault-marker durability.

Plan: no separate plan was supplied.

Prompt: no changes needed.
