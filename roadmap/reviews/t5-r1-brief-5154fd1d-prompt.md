You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r1.md as re-cut at 5154fd1d, round 1 of t5 (#141). Read it with `git show 5154fd1d` and the file as it stands. This is a short confirming review: the first cold review (of 7fcf56be, saved verbatim at roadmap/reviews/t5-r1-brief-7fcf56be.md with its prompt beside) found six findings, one a nitpick; this cut answers the six, and the brief's foot ("Taken from the review") says how. The design is roadmap/briefs/t5-design.md (ruled 2026-10-31, C158 in decisions/daemon.md); the spike's code is on branch worktree-agent-a3d9540486fed65ea (read with `git show worktree-agent-a3d9540486fed65ea:<path>`). Read; do not build or run anything; the tree is read-only to you.

Your lens:
1. Did the re-cut answer each of the six findings in substance? For each: answered, answered narrower than proposed (and whether the narrowing is sound against the tree), or not answered. In particular: DoorBuilder::run building the runtime first and constructing inside block_on (against harness.rs:351–358, 960–967 and the branch's dispatch.rs:2293–2317); await_terminal reading through Request::Status and Request::Result (server.rs:290–323) rather than held (harness.rs:563–574); the auth-change notification's listed publications (login.rs:125–129, 301, 346, 364, 412, 477, 341–364; 508–591; start.rs:627–632; refresh.rs 574–595, 627–630; daemon.rs:633–635) against every site that changes what AuthStatus answers (server.rs:65–77); await_auth's predicate; the done-when greps against check 9's comment filter (arch-check.sh:433–434).
2. Anything this cut introduced that contradicts another section or the design, a Do that now needs a later round, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The six findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 1 may start as drawn, in one line.
