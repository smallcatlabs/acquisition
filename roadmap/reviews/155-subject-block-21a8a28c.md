`0d17cdaa^..21a8a28c` — done-when does not fully hold: the required two-league character case under `removed` is not staged.

- **[crates/acquisition-search/tests/answer.rs:1129](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/tests/answer.rs:1129), shape 2:** The `Removed` iteration tests one historical league, so it cannot pin deduplication across leagues. Persisted input contains removed `gone` in Settlers and live `kept` in Standard; the client’s `membership=removed` excludes `kept`. Smallest fix: seed another removed item last seen in Standard, assert that both leagues matched, then assert one place.

- **[crates/acquisition-search/src/describe.rs:247](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:247), shape 1:** “The fetch fields are null” overstates the serialized shape. For persisted retired substashes with cleared fetches—the fixture at `tests/answer.rs:1247`—only the fetch epochs are null; their age and location keys are absent, while listing fields remain. Smallest fix: state those distinctions explicitly.

- The remaining requested source checks hold: scope serialization preserves its attributes and field order; the fold keys by `coordinate` and retains a span whenever matches exist; text forms use total’s fields; complete-span assertions cover the three views, a limited request and MCP serialization. All regenerated lines fit the allowed kinds, and describe bounds are unchanged.
- Execution evidence, including builds, tests, gate results and reported measurements: **not verified**, per the read-only review instruction.
- Nitpicks: neither finding.
- Brief: Do 4’s “fetch fields null” wording should match Do 2’s precise null-versus-absent contract.
- Plan: no additional obstacle.
- Prompt: no obstacle.
