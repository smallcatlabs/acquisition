ed5161eb — the fix-round done-when does not fully hold.

- **P2 — [counted_tasks.rs:1094](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:1094), shapes 2/3:** Tasks 8 and 9 remove the required evidence instead of checking it; although the message explains these removals, they change what the benchmark measures. Over the fixed persisted world, task 8 now reaches with only PC’s facts even if the answer omits PoE2’s disposition. At [counted_tasks.rs:1134](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:1134), task 9 reaches from a write receipt even if a subsequent read would return another price. Preserve the specified wanted facts and require their appropriate evidence: PoE2’s disposition and a target-associated post-write read.

- **P2 — [counted_tasks.rs:641](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:641), shape 1:** The generic missed-fact text discards the actual wrong answer that this table exists to record. For task 7 over the fixed persisted Hardcore world, “Standard, 1 item” becomes “never spoken of”; changing that answer to another one-digit Standard count can leave both the verdict and rounded bytes unchanged. Record the returned league and count when they differ from the wanted facts, with a pin for that distinction.

- **Not checked:** Execution-dependent requirements, measured counts, and gate status are **not verified**; execution was prohibited.
- **Nitpicks:** None.
- **Brief:** Specify which task requirements may change during a removal round.
- **Plan:** No separate plan was supplied.
- **Prompt:** No changes suggested.
