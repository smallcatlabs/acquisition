You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/time-residue.md as cut a third time at bcd7c96b, the step that closes #152 and the daemon and client rows of #131. Read it with `git show bcd7c96b` and the file as it stands. Two cold reviews before it (of 64c91def and a7904b32, saved verbatim at roadmap/reviews/time-residue-brief-<sha>.md with their prompts beside) found nine and then three findings; this cut answers the second review's three, and the brief's foot ("Taken from the reviews") says how. The rulings are C158, C137 and C160 in decisions/daemon.md. Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is each of the second review's three findings answered in substance? In particular finding 1: with helper chains followed to any depth within a file, cycle-safe, a forwarded Boundary resolved and a helper's own sentence covering no caller, does "zero neither" follow from Do 3's edits for every attributed test under crates/acquisition-daemon/src/daemon/tests (credentials.rs:649 through restoring_daemon and restoring_daemon_on; exit.rs:1227 and 1232 through the helper at 1194, exit_daemon and exit_daemon_on; list.rs:50; login.rs:73 and sessions.rs:193; exit.rs:1095; responses.rs; door.rs:1367), or is there a test the rules still leave unclassified.
2. Anything this cut introduced that contradicts another section, the registry or the tree, or a citation that does not match the tree.
3. Whether a worker reading only this brief and what it names can build the step in the order given, each commit gated, and follow Do 4's stop rule as written.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The three findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
