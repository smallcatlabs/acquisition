`roadmap/briefs/151-stuck-verdict.md` at `132a43ef`, unchanged in the working tree: **not buildable as drawn.**

- **Original finding 1 — narrower, not fully sound:** the accounting rule is implementable for verdict-armed waits on the hook-owning door, but C160’s unqualified “a missed one cannot happen” exceeds Do 5’s explicit `beside` exception.
- **Original finding 2 — narrower, substantially sound:** the representative logs support the three causes; the parse wait can become unanswerable with nothing pending, and `settle` can expose the zero-wait mutant, but the census and exclusion outcome remain inconsistent.
- **Original finding 3 — answered:** local `Expired`, future-first biased selection, unchanged result arms, and cloned callback clocks address it.
- **Original finding 4 — answered:** the registry entry is 791 bytes excluding its newline, with the inventory removed.
- **Original finding 5 — not answered:** the revised reader handles today’s files but still admits production code after a test module.
- **Original finding 6 — answered:** built and stopped outcomes now have separate completion requirements.
- **Original finding 7 — answered as to evidence placement:** summaries belong in commit messages, although Do 2 cannot collect everything those summaries require.
- **Original finding 8 — answered:** the selected verdict arm now panics.

Findings, most severe first:

1. **Do 6 — Taking the last column-zero brace does not identify the test module’s closing brace.** At [brief:384](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:384), append a multiline production function containing `tokio::time::sleep` after `auth.rs:938`, ending with its own column-zero `}`: the prescribed reader absorbs that function into the exemption. Today’s `auth.rs:647–648` and `ratelimit.rs:44–50` are handled correctly, but that does not establish the boundary. **Smallest change:** use the first standalone column-zero `}` after the opener and require it to end the file, refusing ambiguous shapes; make the appended-item breaker a multiline production function containing a timer, and require Check 12’s specific refusal.

2. **Do 1 / Do 5 — The registry still promises more coverage than the brief supplies.** [C160, daemon.md:26](/Users/tom/Development/GitHub/smallcatlabs/acquisition/decisions/daemon.md:26) says a missed verdict cannot happen, while [brief:341–344](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:341) expressly permits a `beside` wait to reach `HANG`; that door has no hook of its own (`door.rs:511–525`). Do 4’s passing-run measurement cannot establish coverage of a later stalled wait on that door. **Smallest change:** qualify C160 and the Goal to verdict-armed waits on the hook-owning door, explicitly retaining `beside` outside that guarantee.

3. **Do 2 / done-when 3 — End-of-scenario totals cannot reveal a killed scenario’s last hook branch.** [Brief:259–275](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:259) specifies printing only when `run` ends, but the representative parse-mutant log ends with SIGTERM at line 2642; the hung scenario never reaches that print. Moreover, class (c)’s component test has no hook to report. **Smallest change:** specify flushed, opt-in park diagnostics carrying the scenario and active wait before termination, and record class (c) as “no hook; runnable yield loop.”

4. **Do 5 / done-when 4 — Calling the sleepers test a scenario while asserting `sleepers() == 2` contradicts C158’s observation boundary.** [Brief:367–373](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:367) retains a clock-internal assertion, whereas `decisions/daemon.md:25` limits scenarios to the protocol and journal. The existing hook component test already demonstrates using the door runtime without becoming a scenario (`dispatch.rs:2458–2510`). **Smallest change:** retain this as a component test on the door runtime, naming its parking-clock boundary and using `settle`; update its completion and retirement wording accordingly.

5. **Do 5 / post-step done-when — Excluded occupancy mutants will not still run to timeout.** [Brief:361–366](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:361) requires `exclude_re` entries, but [brief:470–471](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:470) says those four still time out in the closing run. Function-based patterns can precisely select these four replacements, and the timeout rationale holds for the suite’s final result; individual failures can already be printed before that timeout. **Smallest change:** expect four excluded/removed mutants and the retained stress timeout; describe the reason as preventing a completed caught result, rather than preventing failure reporting.

6. **Do 5’s Input census — Class (a)’s “34 mutants at least” double-counts the declared overlap.** [Brief:200–214](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:200) lists 49 memberships, explicitly counting `policy.rs:333:32` in two rows; 48 unique mutants minus four class (b) and eleven class (c) leaves 33 class (a). **Smallest change:** mark the overlapping row membership explicitly and change the unique class total to 33.

7. **Done-when 1 — The cited revision contains the superseded ruling.** [Brief:425–426](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:425) names `b67776d7`, whose C160 still claims quiescence proves no future progress. **Smallest change:** cite `132a43ef` for the narrowed entry.

What I did not check and why: no compilation, tests, gates, breakers, mutant runs, or measurements, as instructed. I checked saved representative mutant logs and related families, but did not independently recompute the complete 48-mutant comparison or fetch current GitHub issues/CI.

Which findings I would call nitpicks: findings 6 and 7 above. The first review’s two nitpicks are corrected.

**The step may not start as drawn; correct the coverage, reader, measurement, and component-boundary requirements first.**
