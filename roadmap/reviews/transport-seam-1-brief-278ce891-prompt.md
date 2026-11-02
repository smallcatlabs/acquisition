You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/transport-seam-1.md as cut a second time at 278ce891, step 1 of the transport seam's build. Read it with `git show 278ce891` and the file as it stands. One cold review before it (of dad73240, saved verbatim at roadmap/reviews/transport-seam-1-brief-dad73240.md with its prompt beside) found five findings; this cut answers them, and the brief's foot ("Taken from the reviews") says how. The design is roadmap/briefs/transport-seam-design.md (§6 step 1; §7 ruled); the ruling is C161 in decisions/network.md; invariant 1 is in AGENTS.md. Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is each of the first review's five findings answered in substance? In particular finding 1: with `ratelimit/transport.rs` declared as a child module of `choke` through `#[path]` in `crates/acquisition-daemon/src/ratelimit/choke.rs`, the production type, its constructor and its client private to `choke` and its child, and the neutral types re-exported to the crate, can any production path outside the choke point construct or reach the production transport, and does the `#[path]` placement resolve to the file the brief names and stay inside `.cargo/mutants.toml`'s glob. And finding 2: are the HEAD path's lines (crates/acquisition-daemon/src/ratelimit/choke.rs 564–568, 618, 660–665, 679, 720–724) read right.
2. Anything this cut introduced that contradicts another section, the design, the registry or the tree, or a citation that does not match the tree.
3. Whether a worker reading only this brief and what it names can build the step in the order given, each commit gated.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The five findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
