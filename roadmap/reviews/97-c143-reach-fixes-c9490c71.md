# Review of 97-c143-reach fix round 2 at c9490c71^..77688949

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

- `c9490c71^..77688949` — both prior findings are fixed by one rule each and pinned; the fix round’s code requirements hold.

- **[P3] [crates/acquisition-mcp/tests/harness/mod.rs:399](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/tests/harness/mod.rs:399) — shape 1.** The widened checker treats copied facts as generated prose, producing false failures for valid persisted input. A stash tab named `--sale`, received from GGG and stored normally, reaches `rows[].place.name`; the checker now rejects an otherwise correct MCP search answer. Smallest fix: exempt copied fact fields by document path, preserving checks on generated names such as `describe` entries, and pin this valid echo beside the prose breaker cases.

- **Not checked:** gate status, test execution, and claimed red/green results are **not verified**; execution was prohibited.
- **Nitpicks:** the new P3 finding concerns uncommon tab names and affects the test harness, so I would call it a nitpick.
- **Brief:** no obstacle or change suggested.
- **Plan:** no separate worker plan was supplied or used.
- **Prompt:** no obstacle or change suggested.
