`042272f9^..b382f8b6` — source-level done-when conditions hold; the required close’s kept-name explanations and before/after counts are absent from this range.

- Findings: none. The moves preserve behavior, update callers, relocate the unit tests, and leave goldens, references, dependencies, and the architecture check unchanged.
- Not checked: compilation, tests, gate, and CI — not verified, as execution was prohibited.
- Nitpicks: none.
- Brief: narrow the first search to code references and imports; its current pattern also matches ordinary prose containing “run,” “terminal,” or “printed.”
- Plan: no change suggested.
- Prompt: include the close commit when asking whether close-only accounting has landed.
