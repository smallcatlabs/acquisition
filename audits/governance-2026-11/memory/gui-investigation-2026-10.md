---
name: gui-investigation-2026-10
description: "GUI investigation 2026-10-18/19 is recorded at roadmap/GUI-NOTES.md as phase 4's second input; owner ruled loopback web UI first, Tauri later"
metadata:
  type: project
---

The GUI investigation (trade-site replica over the stash, then a C++-style dense table) is written whole at `roadmap/GUI-NOTES.md` (2026-10-19), outside the routing conventions by the owner's instruction, because a governance rework (`roadmap/README.md`, phase 1) is under way. The roadmap names it as phase 4's second input. Owner rulings already in force there: loopback web UI first on the shared function layer, Tauri a later wrapper, the corpus held in `acq-mcp`.

**Why:** the file's section 9 is a ten-question ruling packet for the phase 4 design session; section 8 orders the foundation work (measure the held-corpus ask and steady memory first, MCP holds its corpus, incremental refresh over `item_events` pinned by equality with a whole load, rows carrying item fields, memory diet only if measured, persisted projection last).

**How to apply:** a phase 4 design session reads `roadmap/README.md`, then `roadmap/GUI-NOTES.md`, and rules the packet before anything is built. Do not restate M1–M7 or create a `GUI-SLICE.md` before the owner opens the slice. See [[simplicity-over-mechanism]].
