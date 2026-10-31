You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/4-0-door.md as cut a fourth time at 71b89153, the first build step of phase 4-0, the time seam. Read it with `git show 71b89153` and the file as it stands. This is a short confirming review: the third cold review (of 19f771d4, saved verbatim at roadmap/reviews/4-0-door-brief-19f771d4.md with its prompt beside; the two earlier reviews beside them) found the second review's seven findings all answered or soundly narrowed and raised five of its own, the first that the brief's narrowing of the ruled design was unrecorded and prescribed to step 2. This cut answers the five; the same commit records the narrowing in roadmap/briefs/4-0-design.md §4 as a dated paragraph, the owner's ruling of 2026-10-31. Read; do not build or run anything; the tree is read-only to you. The brief's form is roadmap/briefs/TEMPLATE.md; AGENTS.md names the invariants and the cross-cutting rulings. The brief's line numbers were printed from the tree at 71b89153.

Your lens, in this order:
1. Did this cut answer each of the third review's five findings in substance, not in words? For each: answered, answered narrower than proposed (and whether the narrowing is sound against the tree), or not answered. Check "Taken from the reviews" against the Goal, Read, Do and done-when lines it points at, not against itself.
2. The recorded ruling: does the dated paragraph in 4-0-design.md §4 say what the brief builds (Goal, Read, Do 1, Do 3), and does it contradict C157's words (decisions/daemon.md) or anything the registry holds? Is the criterion as now worded (ordering is a decision, or the only pin costs real time; else an effect) consistent with keeping `stop_until` and `look_until` as effects and with putting the hello and the startup in the machine, against the tree (client.rs 1736–1770, 1444–1508, 1624–1700, 1939–1990, and the in-crate tests at 2235, 2271, 2337)?
3. What this cut introduced: the `child` construction input and its row against `open_door`'s seam (client.rs:1310) and the seam tests (2373, 2449); `LookFailed` against `winner_listens` (1939–1955) and the `?` at 1477; the two startup scripts in Do 2 against the consult rule in Do 3 and the loop at 1444–1508; the harness citation (tests/harness/mod.rs:180–182). Any citation that does not match, any row that contradicts Do 1 or Do 3, any new gap.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The five findings: one line each, answered / narrower (sound or not) / not answered.
- The ruling: consistent with the brief and the registry, or not, in one or two lines.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading in the tree (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether step 1 may start as drawn, in one line.
