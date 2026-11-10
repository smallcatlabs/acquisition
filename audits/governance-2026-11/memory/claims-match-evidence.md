---
name: claims-match-evidence
description: "Every number, hash, count or universal claim in a message, brief or report comes from a command run in an earlier call of the same turn, with its command named; recurred 2026-10-17, 10-19, 10-21, 10-25, 10-28, 10-30 (twice), 11-01"
metadata:
  type: feedback
---

The most repeated fault across sessions: writing from intent or recall rather than from the artifact. Every review found it in my own text.

**Shapes it took:**
- A size, count, hash or digest written in the same command as its measurement, or as a placeholder ("measured later"), or invented before the script printed it.
- A count in a brief from an ad-hoc grep ("about 55"; the worker's named regex found 19), or recalled ("nine" rows, eleven were; "three callers", two).
- "X already has Y" or a constant's value stated from memory, disproved by the worker in one grep.
- A universal word ("every command", "unchanged since") wider than the code held.
- A fix whose check staged the cheapest failure, not the one the finding named.
- A sibling sentence left standing, or inverted, after an edit.
- Routing an observation from a record's summary instead of its source (the cited calls, the oracle's `home`).

**How to apply:**
- Measure in one call, read the output, write the message in a later call. A hash or digest is pasted from output.
- A count in a brief names the command that produced it, so the worker can reproduce it; a count whose matcher is arguable is left out.
- Run every grep a brief cites and paste the hits; quote constants with their line.
- A universal word earns its place by an enumeration done first.
- After editing a claim, grep the document for its key words and read the section whole.
- Check a close message against the tree before committing it. After renumbering any ledger, grep for the old id.

**Why:** the owner reads the history as the record, and the worker and reviewer disprove a wrong literal first, costing a round.

Related: [[verify-before-deferring]], [[raise-what-you-judged-narrow]], [[external-review]].

- **A classification by file name, not by reading (2026-11-01, #151):** I called three daemon tests "a door wait" in a brief's table and on the issue because they live in `dispatch.rs`, without opening them; they are component tests that drive their own clocks, so the verdict I promised never races them. The design's cold review caught it and the next orchestrator raised it. A table row that classifies a test opens the test; "a door wait" is a fact read from the body (`Door::open`, an `await_*` of the door), never inferred from the file.
