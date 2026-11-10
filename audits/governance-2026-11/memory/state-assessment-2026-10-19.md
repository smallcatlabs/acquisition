---
name: state-assessment-2026-10-19
description: "Where the project's running state lives: the tree (roadmap README, LEDGER.md, HISTORY.md, milestones), never this note"
metadata:
  type: project
---

Read the state from the tree, in this order: `roadmap/README.md` ("Current step" and "Pull-forward" say what is next; "The owner's rulings for this road"), `roadmap/LEDGER.md` (one row per closed step), `HISTORY.md` (one line per slice), and the GitHub milestones (League 3.30 and alpha.2; PoE2 launch; 4-full, the GUI), which hold every open issue and date. The dated assessments are `roadmap/STATE-2026-10-19.md` and `STATE-2026-10-26.md`; the test-system read is `roadmap/TEST-SHAPE.md`.

**Why:** this note was once a 29 KB running log, and the memory folder later grew a handoff note per group; both duplicated the ledger and went stale within days.

**How to apply:** never record a step's status, a plan's order or what is pushed in memory; a close writes its row in the tree. Check `git log origin/main..main` for what is unpushed.
