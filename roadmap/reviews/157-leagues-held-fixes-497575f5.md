497575f5 — the fix-round’s code and pin requirements hold by inspection.

- Findings: none. The shared reader handles malformed persisted policy rows across the affected carriers; unavailable holdings remain distinct from empty in CLI text and MCP JSON; both documents join the walk. Shop render’s changed refusal behavior follows the same policy-read fix. No unrelated changes found.
- Not checked: gate, tests and runtime behavior — **not verified**, per the read-only review instruction.
- Nitpicks: none.
- Brief: no obstacle. Plan: no change suggested. Prompt: no change suggested.
