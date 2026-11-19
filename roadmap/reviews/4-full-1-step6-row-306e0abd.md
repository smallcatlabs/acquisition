1db48f59^..306e0abd — the brief’s done-when does not fully hold: two implementation gaps remain, and the required close is absent from the range.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:1475`:** The refused-query box renders “Matches” but never loads its count script, because `failed_then` uses a head with `count: false`. Concrete client input: visit `search?realm=pc&text=class%3Arings%20sort%3Alife`, then correct the text to `class:rings`; the count remains “–” after edits. Enable the count script when rendering this box and extend the existing browser check to cover correction after refusal.
- **P3 — `crates/acquisition-cli/src/web/mod.rs:1694`:** A continuation’s sort links discard every retained `item`, contrary to Decision 1’s preservation rule. Concrete client input: `search?next=<valid current-basis token>&item=r1`, where the token requests table rows with an `ilvl` column; clicking that column’s head closes the card because this branch carries only saved-search fields. Carry retained `item` fields into the generated URL and cover this branch in the existing reorder test.

Execution, gate statuses, test results and documentation generation: **not verified**, as requested; this was a read-only review. External issue closures were not checked.

Nitpicks: the continuation-only finding is low-frequency because ordinary table navigation does not use continuation tokens.

Brief/prompt change: distinguish build completion from step closure, or include the close commit in the reviewed range.
