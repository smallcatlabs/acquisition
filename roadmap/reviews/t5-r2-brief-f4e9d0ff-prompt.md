You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r2.md as cut a third time at f4e9d0ff, round 2 of t5 (#141). Read it with `git show f4e9d0ff` and the file as it stands. This is a short confirming review: the second cold review (of 437fec6d, saved verbatim at roadmap/reviews/t5-r2-brief-437fec6d.md with its prompt beside; the first review beside them) found eight of eleven findings answered and raised five; this cut answers the five, and the brief's foot says how. Read; do not build or run anything; the tree is read-only to you. The brief's commits changed no code, so every line it cites is checkable at f4e9d0ff.

Your lens:
1. Are the five findings answered in substance? In particular: the code-exchange scenario's evidence (the POST in the journal; the profile left Waiting through request(Status) after the leave) against login.rs:229, dispatch.rs:69 and server.rs:1077; the first-look synchronization for the blocked-worker (exit.rs:388–446) and signal (1099–1149) tests against the driver's look (server.rs:916–929) and the parking clock's registration (harness.rs:201–203), including the case where the driver finds quiet on its first look and leaves without parking; the retirement allowlist against every settle()/spawn_drain call in exit.rs today (list them); the three fixture citations.
2. Anything this cut introduced that contradicts another section, a Do that now needs a later round, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The five findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 2 may start as drawn, in one line.
