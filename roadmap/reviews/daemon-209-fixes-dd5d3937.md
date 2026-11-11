dd5d3937 — the fix round’s done-when holds only partially; nested restore and eviction still mishandle unreadable children.

- **P1 — [queue.rs:74](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:74), shape 2:** `unreadable_child` preserves the row’s deferred success, allowing a later descendant completion to reverse the synthetic failure and overwrite the unreadable row. Concrete persisted input: running parent 11 holds success; child 12 has state `paused` and deferred success; waiting grandchild 13 belongs to 12; waiting sibling 14 belongs to 11. If 13 completes before 14, `maybe_finish_parent(12)` consumes the inherited success and persists 12 as done; completion of 14 then makes 11 succeed. Keep an unreadable nonleaf’s failure held until its descendants settle, prevent persistence of its substituted row, and pin this nested ordering.

- **P2 — [queue.rs:390](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:390), shape 7:** Eviction treats synthetic failures as persisted entries, so the list’s counts disagree with its rows. With this commit’s own persisted fixture, after parents 11 and 21 conclude, `List` reports five jobs—four failed and one done—but returns four rows: two failed and two done; child 12 disappears and child 22 reappears as done. Both findings share one cause: a memory-only substitute receives an ordinary persisted entry’s lifecycle. Retain trees containing substitutes in memory, or preserve their exceptional state through subsequent reads, and assert list rows against counts after parent completion.

Execution, gate status, test counts and runtime interleavings: **not verified**, because this review was restricted to reading.

Nitpicks: none; both findings affect behavior, although the nested case requires unusual persisted input.

Issue: its proposal does not address unreadable children that themselves have descendants.

Plan: no separate plan was supplied.

Prompt: no changes needed.
