4e53584d — both original findings are fixed and pinned; the fix-round’s source-level done-when holds.

- Findings: none.
  - `--describe=~resist` is portable under the four declared shell syntaxes. The new `error_kind.rs` test confirms help contains that example, passes it unquoted through each shell `shells_here()` returns, and checks successful output echoing `~resist`.
  - Help, CLI reference, terminal guidance, README and golden were carried together. `readme_tour.rs` resolves the new synopsis to `search` and `--describe`; the optional attached alternatives accurately describe its forms.
  - The three duplicate wording pins are removed. The retained bad-pattern sentence and MCP addition have no golden holder, so their literal assertions are justified by C153’s explicit allowance; neither requires another golden merely for that sentence.
  - The `# holds` literals in `search_json.rs` and their generated golden lines constitute **one holder**: the literals supply the golden’s checked facts, rather than independently pinning another surface.
  - The golden’s single-quoted command labels render recorded argument arrays; they are not guidance emitted by Acquisition. Every changed golden/reference line follows a source change.

- Execution evidence—gate status, test counts, measurements and shell round trips—is **not verified**; execution was prohibited. Persisted and upstream inputs, persistence, cancellation and network gates were not exercised; this diff changes no such paths.

- Nitpicks: none.

- Brief: Do 6 should permit portable or shell-specific examples instead of requiring single quotes.
- Plan: no obstruction.
- Prompt: no change needed.
