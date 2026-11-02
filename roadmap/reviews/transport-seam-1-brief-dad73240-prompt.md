You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/transport-seam-1.md, its first cut at dad73240, step 1 of the transport seam's build. Read it with `git show dad73240` and the file as it stands. The design it comes from is roadmap/briefs/transport-seam-design.md (§2, §3, §4, §5, §6 step 1; §7 ruled by the owner on 2026-11-01); the ruling is C161 in decisions/network.md; invariant 1 is in AGENTS.md. Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is the brief buildable as drawn against the tree? Every file:line it cites, read against the file (crates/acquisition-daemon/src/ratelimit/choke.rs, auth.rs, ratelimit/policy.rs, daemon/kinds.rs, ratelimit.rs, mockggg.rs, ratelimit/tests/spec.rs, daemon.rs, tools/arch-check.sh check 12, tools/arch-check-breakers.sh); whether the four send sites can each build a prepared request and await a transport while the limiter still observes the headers before the body and the gate's permit still spans the body (C21); whether `observe` and `finish_send` can take the new answer type with every journal field and word unchanged.
2. "No behaviour change": every place the brief claims the wire, the journal, the words or the pacing stay as they are, held against what reqwest does today (its default headers against a request header, `.form()`, `bearer_auth`, a builder error, `text()`), and the one change of words it allows (a token that is no header value). Is any other behaviour changed that the brief does not name?
3. Invariant 1's structural form after this step: the choke point owning the transport privately, the production transport owning the client privately, `with_transport` under `#[cfg(test)]`, check 13 and its rename refusals. Can a production path reach the provider without the choke point and the gate once this lands, by a route check 13 does not see?
4. Do 3's table and the five kept wire tests: does the table hold what §2 asks of the prepared request's real-wiring test, and is the in-process run evidence of anything the TCP run does not hold?
5. Done-when: each line checkable from the commit alone, its tree or its message; anything asked of a commit that only execution shows; the counts (20 lines before, 8 after Do 1) against the tree.
6. The narrowing in Not in scope (the builder's transport option moved to step 2): sound, or does it break "no step needs a later one" in the design's §6?

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
