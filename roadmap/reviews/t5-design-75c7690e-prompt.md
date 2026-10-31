You are the outside reviewer of a DESIGN BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-design.md as cut a third time at 75c7690e, the design for t5 (#141: the daemon's tests onto the five-call door, inside the time seam). Read it with `git show 75c7690e` and the file as it stands. This is a short confirming review: the second cold review (of 189792f5, saved verbatim at roadmap/reviews/t5-design-189792f5.md with its prompt beside; the first review of 6284d681 beside them) found ten of the first review's thirteen findings answered or soundly narrowed and raised seven; this cut answers the seven, and the brief's foot says how. Read; do not build or run anything; the tree is read-only to you. The inputs are the ones the earlier prompts name; tools/bothy.py's entry rule (one bullet, under 800 bytes, a property, named pins or "nothing yet") governs §5.

Your lens:
1. Did this cut answer each of the second review's seven findings in substance? For each: answered, answered narrower than proposed (and whether the narrowing is sound against the tree), or not answered. In particular: the auth-change notification (§2.1) against login.rs's transitions (finish_auth_flow_error at 472, the success after the profile job, the logout) and protocol.rs:455; the responses.rs decision (kept as the component table, three boundary twins, no retirement) against the five tests by line and the C154 rule; round 2 moving every Life caller against credentials.rs 564/781 and harness.rs 633; the boundary rule without an exception (§3's paragraph) against the ruling's text; the scratch journal against daemon.rs:1232, rails.rs 447–480 and harness.rs:506; the ruling's byte count (count the bullet alone, unwrapped, through its *Pinned:* clause) and form.
2. Anything this cut introduced that contradicts another section, a round that now needs a later one, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether round 1's build brief may be written from it after the owner rules on §5.
- The seven findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the section, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- The ruling's byte count.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 1's build brief may be written from this design as it stands, in one line.
