2d751d1f^..aff30846 — the brief’s done-when does not fully hold; execution-dependent criteria are not verified.

Findings, most severe first:

- **[P2] crates/acquisition-mcp/tests/search_tools.rs:929 — shape 2:** The fixture still does not reliably catch the named default-window regression. Changing the default to 12 hours leaves every assertion satisfied; changing it to one hour also passes if seeding and the request occur within the same Unix second, because `events_since` includes the cutoff timestamp. Seed events 23 and 25 hours ago and update the expected timestamps, retaining the explicit 72-hour comparison.

- **[P3] tools/arch-check.sh:384 — shape 1:** Converting declared test paths to a set hides multiple Cargo targets using the same root. Add a second CLI `[[test]]` entry named `main_copy` with `path = "tests/main.rs"`: the layout check accepts it although Cargo builds and runs the suite twice. Reject repeated normalized paths before converting them to a set, and add that breaker.

What I did not check and why:

- Builds, either test runner, the 24-binary count, gate stages, breaker execution, artifact hashes, timings, and Windows runtime behavior are **not verified**: this review was restricted to reading. Commit-message reports were not treated as execution evidence.
- The earlier migration’s unchanged deletions, thins, and goldens were not exhaustively re-reviewed; this review covered the requested fix range and affected callers. No golden or reference files change in this range.

Nitpicks:

- The duplicate-target finding is a nitpick for an unlikely manifest mistake. The default-window finding concerns the regression this round explicitly promises to catch.

Brief, plan, and prompt:

- **Brief:** Give fix rounds a focused done-when list distinguishing their obligations from the original seven-step migration.
- **Plan:** Express the layout correspondence as one Cargo target per inferred root, including multiplicity.
- **Prompt:** No change suggested.
