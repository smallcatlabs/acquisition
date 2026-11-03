You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/151-component-waits.md, its second cut at 3530b90c (the first, d5dffbac, reviewed in roadmap/reviews/151-component-waits-brief-d5dffbac.md; check that its seven findings are answered, then review the whole). It aims to turn sixteen mutants that cargo-mutants reports as timeouts into caught ones, by making each test that hangs under them fail where its wait can no longer be answered. Its input is the saved logs of Weekly run 41567365592 under runs/transport-seam-after/ (gitignored; present on this machine), whose per-timeout SIGTERM lines name the tests still running at the cut. The rulings are C122, C137, C154, C158 and C160 (decisions/daemon.md, AGENTS.md); the verdict was built under roadmap/briefs/151-stuck-verdict.md; the transport seam's design row on #151's remainder is roadmap/briefs/transport-seam-design.md §3. Read; do not build or run cargo; the tree is read-only to you. You may run the brief's Python sweep over runs/ and `cargo mutants --list` with its patterns (both read-only).

Your lens, in this order:
1. Is the brief buildable as drawn against the tree at 3530b90c? Every file:line it cites (crates/acquisition-daemon/src/daemon/tests/door.rs, harness.rs, dispatch.rs, bearer.rs, exit.rs, credentials.rs, stress.rs; ratelimit/tests/spec.rs; .cargo/mutants.toml; tools/mutants-compare.py), and the counts it states (77 timeouts, sixteen mutants, the measure listing 16).
2. The classes: is each hung test in the class the brief gives it, read from its runtime and its waits? In particular the three door tests (bearer.rs 474 and 486, door.rs 1707): why might C160's verdict not fire there, and is the brief's guess (a hold the hook counts) consistent with the hook in door.rs?
3. The fail-fast rules of Do 4: is each signal exact (no wall bound under HANG, no count of yields)? Is `now_or_never` sound for the C128 refusal (is the refusal reached before any await in get_bearer and head)? Is the suggested `advance_until` shape (fail once the clock has no next deadline and the awaited state has not changed since the last advance) sound on a plain runtime where a task may still be runnable while no sleeper is parked?
4. Scope: can every class be done with tests, test support and tools only, as Rules and Done-when require; does any likely fix need a production edit the brief forbids?
5. The stop at Do 3 and the oracle: is the stop placed where the unknowns are, and is anything else in the step unknown that the brief treats as known?
6. Done-when: each line checkable from the commits, their tree or their messages; anything stated that the build cannot show.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- Findings, most severe first, each: the Do or done-when number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
