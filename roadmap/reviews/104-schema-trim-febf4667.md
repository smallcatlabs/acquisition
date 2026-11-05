febf4667 — the brief’s done-when is not fully met: the explicit-null compatibility pin is missing.

- **[P3] roadmap/briefs/104-schema-trim.md:81 — Missing required pin.** No existing MCP test sends an explicit `null` as an optional argument and checks that it behaves as absent. The concrete client input is `list_jobs {"expand":null}`; a future reader that refuses it would escape this coverage. Extend an existing MCP test to compare that request with `list_jobs {}`.

- **Not checked:** execution, reported byte measurements, and gate results are **not verified**; this was the requested read-only review. No production code changed, and I found no runtime regression in the affected callers.
- **Nitpicks:** the finding is a low-risk completeness nitpick because the argument reader remains unchanged.
- **Brief:** clarify whether the explicit-null pin remains mandatory when declining the trim requires reverting Do 2.
- **Plan:** no separate plan was supplied.
- **Prompt:** no changes suggested.
