23f225c1 — the fix round’s done-when does not fully hold: the earlier P1 is fixed; P2 remains incomplete.

- **P2 — [answer.rs:1362](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:1362), shapes 2 and 3:** An exact inverted stat under `if` still loses its explanation because preservation and rendering recognize only a whole term equal to `false()`. Concrete **client input**: `{"stats":[{"type":"if","filters":[{"id":"implicit.stat_3299347043","value":{"min":5,"max":0}}]}]}`. Its clause becomes `-line("# to maximum Life" source=implicit) or false()`, so `ClauseOut` strips the pair’s note, and [search_cmd.rs:1007](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:1007) excludes it from both CLI and page explanations. These share one cause: using the composed term’s spelling to identify inversion. Preserve and list clauses carrying `query.inverted` regardless of their composed term, and pin this case through the final answer.

- What I did not check: execution evidence is **not verified**; no builds, tests, or application runs were performed under the read-only instruction. Live site behavior was taken from the supplied owner confirmation.
- Nitpicks for unlikely cases: none.
- Brief, plan, or prompt changes: none; the saved review clearly defined this fix round’s obligations.
