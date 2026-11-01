You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/time-residue.md, its first cut at 64c91def, the step that closes #152 and the daemon and client rows of #131. Read it with `git show 64c91def` and the file as it stands; the retrospective it comes from is roadmap/reviews/time-seam-retrospective.md (§10 rows E1, E2, E3, C1); the rulings are C158, C137 and C160 in decisions/daemon.md; the step before it is roadmap/briefs/151-stuck-verdict.md (closed). Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Is the brief buildable as drawn against the tree? Every file:line it cites, read against the file; the seventeen spec sites and whether `ChokePoint::with_clock_and_rails` can take a parking clock from a crate-level module the spec can see (`ratelimit/tests/spec.rs` has `use super::*`); whether the two `wait > 5 s` tests can assert an exact wait from their own headers; the nine client waits and the harness's ceilings; the three constructors and the 43 sites; the mock's windows and where a divisor read once at start would apply.
2. Do 4's measurement and stop rule: is "every window's period and restriction divided by ten" safe for the mock's restriction logic (N23's windows, a 429's restriction, the token policy), and can a golden moved by a draining window be told from one moved by the masked ETA; is the stop rule's threshold and its fallback (the knob landing with the isolation list leaving it unset) sound.
3. Do 3's reader: can a report-only script tell door, boundary, sentence and neither apart from the source alone, including a test over a row vector; does a `Boundary` first argument on the constructors meet #152's "Closes when" as the owner read it (a value at the constructor meets "names its boundary").
4. Done-when: each line checkable from the commit alone, its tree or its message; anything asked of a commit that only execution shows.
5. Anything the brief cites from an issue, a ledger row or the retrospective that the tree contradicts.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
