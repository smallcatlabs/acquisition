85c74395 — the fix-round done-when does not fully hold.

- **P2 — [counted_tasks.rs:1142](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:1142), shapes 1/2:** Task 9 can establish “a read after the write says 10 chaos” from the CLI write receipt. Over the fixed persisted world, `price set` already prints `item/r1: 10 chaos`; checking every answer therefore leaves the row `reached` even if the subsequent successful `price show` reports another price. Require this fact from the post-write read, associated with its target, and pin a contradictory read.

- **P2 — [counted_tasks.rs:1095](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:1095), shape 1:** Task 8 substitutes an all-realms scope for the required statement that PoE2 was answered or left out. A frontend answer over the fixed persisted world can retain `scope.realm: "all"` while dropping PoE2’s undecided explanation, and this wanted-fact check still passes. Require evidence of PoE2’s disposition; merely naming the selected scope cannot establish it.

- **P2 — [counted_tasks.rs:1420](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/counted_tasks.rs:1420), shape 2:** The MCP Hex Band negative case omits the identity the predicate actually matches. The fixture names Hex Band, but `hex_band_undecided()` looks for `r3`, so the assertion passes without testing subject/status association. Include `id: "r3"` on the matched row and another item’s identity in the undecided example; pin rejection despite both required words being present.

- **Not checked:** Execution-dependent requirements, measured counts, and gate status are **not verified**; execution was prohibited.
- **Nitpicks:** None; these affect the measurement’s meaning or its required pins.
- **Brief:** Distinguish accumulated facts from facts that must come from a particular later read.
- **Plan:** No separate plan was supplied.
- **Prompt:** No changes suggested.
