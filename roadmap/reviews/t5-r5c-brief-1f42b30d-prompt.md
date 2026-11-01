You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r5c.md as re-cut at 1f42b30d, round 5c of t5 (#141), the sweep. Read it with `git show 1f42b30d` (which also narrows C159 in decisions/network.md) and the file as it stands. The first cold review (of 4e6c330c, saved verbatim at roadmap/reviews/t5-r5c-brief-4e6c330c.md with its prompt beside) found seven findings, two of them nitpicks; this cut answers each, and the brief's foot ("Taken from the review") says how. The design is roadmap/briefs/t5-design.md (C158 and C45 in decisions/daemon.md; the design's §5 ruling on the exempt-route list, now C159 in decisions/network.md); the door is crates/acquisition-daemon/src/daemon/tests/door.rs. Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is each of the seven findings answered in substance, not only in words? In particular finding 1 (C159 as narrowed, decisions/network.md: against the design's §5 and its ruling, "the client keeps its copy under a check that compares it with NO_PROBE_ROUTES composed with the token rule"; is the entry now the ruling and no wider; its form against tools/bothy.py's docstring; its length under 800 bytes); finding 2 (the marker's absence held by exit.rs:1665's component test: does that test check the marker's removal after a successful restore, and do the three scenarios as drawn now read nothing but the protocol, the journal and the ledger); finding 3 (the Windows pipe name derived from the test's directory, as world.rs:403 derives it: is that buildable inside `run` before the daemon is built, daemon/endpoint.rs:56); finding 4 (exit.rs:386's sleep retired as redundant: is the component's `pick_runnable` assertion the property it stood before); finding 5 (the three kinds; C42's Pinned line in decisions/plans.md following the rows at the close); findings 6 and 7.
2. Anything this cut introduced that contradicts another section, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The seven findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 5c may start as drawn, in one line.
