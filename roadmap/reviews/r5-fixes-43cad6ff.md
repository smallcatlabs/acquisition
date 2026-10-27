6f5e0119^..43cad6ff — r5’s done-when is not fully established; execution-dependent criteria remain **not verified**.

- **[P3] `crates/acquisition-mcp/src/main.rs:1042` — shape 1:** “A policy plan carries `notes`” promises a key that serialization omits when both notes are empty. Concrete input: set a tabs-only Standard policy in a store with no fetched league list, then call `refresh_plan({})`; the answer has no `notes`. Smallest fix: say “Only policy plans can carry `notes`; selection plans carry none,” and regenerate `MCP-REFERENCE.md:134`, which shares this cause.
- **Not checked:** Builds, tests, binaries, reference regeneration, mock apply loops, or gate stages, as instructed. Their execution-dependent acceptance lines are **not verified**; these commits contain no gate exit-status table. Static inspection found no additional behavioral defects or invariant bypasses in the changed paths and affected callers.
- **Nitpicks:** Finding 1 is a wording nitpick; its impact requires reading the new clause as guaranteeing the key’s presence.
- **Brief:** Separate source-coverage requirements from execution evidence in the done-when list.
- **Plan:** No change suggested; the shared selection compiler is the appropriate place for realm validation.
- **Prompt:** Specify where execution evidence should be committed if the reviewer must assess it from commits alone.
