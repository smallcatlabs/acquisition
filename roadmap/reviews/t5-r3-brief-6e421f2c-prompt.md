You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r3.md as cut a third time at 6e421f2c, round 3 of t5 (#141). Read it with `git show 6e421f2c` and the file as it stands. This is a short confirming review: the second cold review (of 582e9569, saved verbatim at roadmap/reviews/t5-r3-brief-582e9569.md with its prompt beside; the first review beside them) found four of five findings answered and raised two: a blanket forbidden-fragment check would fail the challenge row on correct behaviour (the journal keeps the cf-mitigated header), and the report still asked for a replaced column's shape. This cut answers both, and the brief's foot says how. Read; do not build or run anything; the tree is read-only to you.

Your lens:
1. Are the two findings answered in substance? The per-fragment surfaces (Do 1 and the block rows in Do 2) against dispatch.rs 821–830's old assertions (which fragment was excluded from which surface) and the journal's header snapshot (harness.rs:493, policy.rs:427, rails.rs:839): does any row now forbid a fragment on a surface that keeps it, or omit an exclusion the old test made?
2. Anything this cut introduced that contradicts another section, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The two findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 3 may start as drawn, in one line.
