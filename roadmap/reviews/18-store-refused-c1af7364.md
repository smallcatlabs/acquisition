c1af7364 — the brief’s done-when does not fully hold: the MCP can alter the body it promises to return verbatim.

- **[P2] crates/acquisition-mcp/src/main.rs:868:** `RefusedParams` silently ignores unknown client arguments, allowing a successful answer for a different request. With Alice as the sole known account, `{"acount":"Bob#5678","id":1}` reads Alice’s body instead of refusing the misspelled selector. Add `#[serde(deny_unknown_fields)]` and pin the adapter’s `arguments` refusal.

- **[P3, shape 1] crates/acquisition-mcp/src/main.rs:1807:** The returned body passes through `told` → `spoken`, which treats nested `hint` and `why` strings as frontend prose. A persisted upstream refusal body such as `{"stash":{"id":"t1","items":false},"hint":"--limit"}` gains an MCP spelling suffix inside `body.hint`, corrupting the evidence presented to the reader. Exempt the `body` subtree from rewriting and pin whole-body equality through the tool.

- **Not checked:** Build, tests, gate exit status, and runtime size checks are **not verified**, as execution was prohibited. Static review found the shared one-body function, list/count wiring, sibling wording, reference changes, and requested pins present; no new daemon or network path appears.

- **Nitpicks:** Finding 2 is unlikely with today’s upstream field names and contents; I would call it a practical edge-case nitpick. Finding 1 can arise from ordinary client typos.

- **Brief:** Add a whole-body round-trip assertion to the adapter pin.
- **Plan:** Nothing obstructed this review.
- **Prompt:** No change suggested.
