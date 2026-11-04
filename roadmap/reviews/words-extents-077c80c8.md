077c80c8 — done-when does not fully hold: one naming sibling remains, and item 9’s corrected wording lacks a pin.

- **[P3] [corpus.rs:478](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:478), shapes 3/7:** A substash’s parent bypasses the shared naming rule. Persisted input with a fetched `UniqueStash` named `" (Remove-only)"` and a fetched child page produces `name: null` for that parent in the scope, but `place.parent.name: " (Remove-only)"` on the row, which prints the mark. Use `location_name(&p.name)` here and extend the existing unique-page fixture to pin both representations.
- **[P3] [describe.rs:239](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:239), shape 2:** Item 9’s corrected extent is printed but unpinned. Restoring “the live items in the fetched ones” would satisfy the existing description assertions, which check only the unchanged introduction and pointer; the golden also retains only the introduction. Add an assertion for “the live items at every location of the type, fetched or not” in an existing search description test.

- **Not checked:** Execution evidence—including the gate, test counts and measured MCP sizes—was not verified; this was the requested read-only review. Bounds remain unchanged in source.
- **Nitpicks:** The unnamed remove-only parent case is unlikely in practice; I would call the first finding a nitpick. The missing wording pin is an explicit done-when gap.
- **Brief:** No change suggested.
- **Plan:** No separate plan was supplied or assessed.
- **Prompt:** No change suggested.
