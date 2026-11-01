You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/time-residue.md as re-cut at a7904b32, the step that closes #152 and the daemon and client rows of #131. Read it with `git show a7904b32` and the file as it stands. The first cold review (of 64c91def, saved verbatim at roadmap/reviews/time-residue-brief-64c91def.md with its prompt beside) found nine findings, two nitpicks; this cut answers each, and the brief's foot ("Taken from the review") says how. The rulings are C158, C137 and C160 in decisions/daemon.md; the retrospective it comes from is roadmap/reviews/time-seam-retrospective.md §10. Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is each of the nine findings answered in substance, not only in words? In particular finding 1 (a GET pinned and polled once through poll_fn before the trip: does one poll of the choke point's GET future register it at the gate's oneshot deterministically, spec.rs:2063 and choke.rs:409, gate.rs:139); finding 3 (start_daemon_with's per-daemon env at harness/mod.rs:440–444 applied after isolate_at: does it override an isolation-list value for that daemon, and is new_league.rs:48 reachable by it); finding 5 (the reader's rules as Do 3 states them: can a script apply them to the files as they stand, including list.rs:50, exit.rs:77, exit.rs:1095, responses.rs:2, login.rs:73 and sessions.rs:193, and does "zero neither" then follow from Do 3's edits); finding 6 (the five completion bounds' three classes and a spec-visible HANG); finding 7 (the conditional Goal and the rehearsal); finding 9 (the gate line).
2. Anything this cut introduced that contradicts another section, the registry or the tree, or a citation that does not match the tree.
3. Whether a worker reading only this brief and what it names can build the step in the order given, each commit gated, and whether the stop rule of Do 4 can be followed as written.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The nine findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
