8e0baadb^..197f60a8 — #234’s done-when is partly met: both busy writes have CLI/MCP coverage, but the general fallback rule remains incomplete.

- **P3 — `crates/acquisition-store/src/annotations.rs:610`, shape 1.** The shared fallback claims “nothing was written” even when a price has committed successfully. Concrete persisted input: an unrelated live price row whose `revision` contains nonnumeric text; `price set tab/pc/c1 exact 2 divine` commits, then `covering()` fails while listing prices and embeds this sentence in `counts_unavailable.why`. Remove the unconditional write-status claim from the shared fallback; assert rollback only where the caller knows it occurred, and pin this successful-write/failed-read case.

- **P3 — `crates/acquisition-store/src/annotations.rs:684`, shape 3.** Printing raw database errors still exposes the store names the new rule promises to remove. Concrete persisted input: a schema-v3 file with valid account metadata but no `annotations` table; a valid price write prints “no such table: annotations,” potentially followed by SQL containing scope and key names. Render a user-facing database cause, retain technical diagnostics separately, and extend the existing refusal golden with this case.

- **Not checked:** runtime behavior, tests, gate exit statuses and test counts are **not verified**; execution was prohibited. Both named retirement replacements exist and are included in the gate’s test suite.
- **Nitpicks:** the internal-name wording leak is a prose nitpick under unlikely persisted damage; the false write-status claim can mislead recovery.
- **Issue:** no changes suggested.
- **Plan:** no separate plan was supplied.
- **Prompt:** no changes suggested.
