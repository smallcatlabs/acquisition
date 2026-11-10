---
name: commit-before-each-owner-look
description: "Owner 2026-10-22 — commit at logical points and always before he reviews, so the before/after of his input is in history; commit to main on a green gate and push on my judgment are standing grants (2026-11-09)"
metadata:
  type: feedback
---

Owner (2026-10-22, the 2f fix rounds): make commits at logical points, and always make one before each time he looks at the work. That way his input shows up in history as a before and after. Commit straight to main, never a branch (owner 2026-10-30: "we haven't been using branches so far", keep the GitHub pattern consistent), and gate every commit on `tools/gate.sh`'s exit status.

**Commit and push are standing grants (owner 2026-11-09, the governance audit, asked whether the push grant of 2026-11-03 stands along with commit-to-main on a green gate):** "yes, the push grant and commit to main on green are ok." So: commit to main when the gate is green and push when a logical unit is committed and gated, on my own judgment, without asking first. This replaces the earlier "never push without his word" (2026-10-22). A push the harness classifier refuses is left and reported, never worked around. The grant is to move into the repository's process rulings (`decisions/process.md`, ruled the same day); once it is there, the tree is the source and this note only points at it. In the same message he asked that the commit-msg hook's behaviour be known to every session, "because we've been bitten by that twice": it rewrites Claude's `Co-Authored-By:` line to `Assisted-by:` and adds nothing when the line is absent.

**Why:** it captures what his review changed.

**How to apply:** before handing him a diff, ruling packet or round to look at, commit what exists. His changes then land as later commits.
