You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r1.md as cut a third time at 61052e5a, round 1 of t5 (#141). Read it with `git show 61052e5a` and the file as it stands. This is a short confirming review: the second cold review (of 5154fd1d, saved verbatim at roadmap/reviews/t5-r1-brief-5154fd1d.md with its prompt beside; the first review beside them) found five of six findings answered and raised two: the auth-change inventory missed the bearer rejection's two publications (refresh.rs:249–252 and daemon.rs:661–668; refresh.rs:317–323) and done-when 4 named the superseded API. This cut answers both, and the brief's foot says how. Read; do not build or run anything; the tree is read-only to you.

Your lens:
1. Are the two findings answered in substance? Is the inventory now complete against every site that changes what AuthStatus answers (server.rs:65–86: pending, the result, the sessions' names, expiry, keyring status, restoring), and is "notified outside the shared hold" buildable at each site (the publication under the lock, the notification after it)?
2. Anything this cut introduced that contradicts another section or the design; a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The two findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 1 may start as drawn, in one line.
