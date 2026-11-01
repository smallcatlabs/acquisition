You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r3.md as re-cut at 582e9569, round 3 of t5 (#141). Read it with `git show 582e9569` and the file as it stands. It was re-cut from a first cold review of the earlier draft (66086b68), saved verbatim at roadmap/reviews/t5-r3-brief-66086b68.md with its prompt beside it; the brief's foot ("Taken from the review") says what each of the five findings changed. The design is roadmap/briefs/t5-design.md (ruled 2026-10-31, C158); rounds 1 and 2 closed at 37275136 and 000b104c. Read; do not build or run anything; the tree is read-only to you. The brief's commits changed no code, so every line it cites is checkable at 582e9569.

Your lens, in this order:
1. Did the re-cut answer each of the five findings in substance? For each: answered, answered narrower than proposed (and whether the narrowing is sound against the tree), or not answered. In particular: the runner's one subscription before any submit, counting terminal events by id and rejecting a lag, against door.rs's events receiver and the broadcast's capacity (daemon.rs's events channel, the builder's events_capacity); the step model (batches, awaited states, cancel, reset, status and dashboard reads, a second lifetime while the first door is live) against DoorBuilder::run owning its runtime (door.rs:359: can a second door be opened inside the first's scenario on the same runtime, with Server::None and the same state_path, and finished there?) and against rails.rs:698's reset clearing the marks; the 1015 block's post-reset fetch whose journal wait is C19's price for a Retry-After of 30 (is the limiter's hold intact after ResetTripwire, or does the reset clear it too: rails.rs:698 and the choke's hold state); the projection's ok and the forbids over the journal text, Dashboard.errors and a lifetime's halt.
2. Anything this cut introduced that contradicts another section, a Do that now needs a later round, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The five findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 3 may start as drawn, in one line.
