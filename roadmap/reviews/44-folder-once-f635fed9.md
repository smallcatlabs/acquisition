f635fed9 — the brief’s two source-level done-when conditions hold; overall completion depends on the gate result.

- Findings: none. The comparisons preserve existing behavior, and all three SQL statements bind the constant at the correct parameter positions, including through the affected callers.
- Gate exit status, test count, and runtime behavior: not verified; this review was read-only, with no builds or tests, as instructed.
- Nitpicks: no findings to classify.
- Brief: change “two SQL statements” in Goal to “three,” matching Input and the commit.
- Plan: no changes suggested.
- Prompt: no changes suggested.
