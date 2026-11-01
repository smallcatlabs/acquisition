`roadmap/briefs/151-stuck-verdict.md` at `2e0fa403`, unchanged in the working tree: **buildable as drawn, with three nonblocking wording/citation findings.**

- **Second-review finding 1 — answered:** all six inline test modules satisfy the reader’s rule today; none contains an earlier standalone column-zero `}` or has content after its close. The appended-function breaker distinguishes the corrected boundary.
- **Finding 2 — answered:** C160 is 793 bytes; its hook-owning-door guarantee and `beside` exception agree with the Goal and Do 5.
- **Finding 3 — answered:** flushed park-level counters survive a killed scenario; class (c)’s “no hook; a runnable yield loop” record is appropriate.
- **Finding 4 — answered:** the sleepers test remains a component test, names its parking-clock boundary and uses the door runtime consistently with the blocking-count probe.
- **Finding 5 — answered:** exclusions become removals in the compare tool; the revised reason correctly distinguishes printed failures from a completed result.
- **Finding 6 — answered:** class (a) is 33 unique mutants, with the overlapping membership marked.
- **Finding 7 — answered:** done-when 1 now identifies the ruling at this cut.

Findings, most severe first:

1. **Done-when 3 — Its unconditional demand for a hook branch for all three mutants retains the class (c) contradiction that Do 2 now resolves.** Concrete reading: [brief:457](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:457) versus the explicit exception at [brief:285](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:285); the Report repeats the unconditional wording at line 547. **Smallest change:** repeat the class (c) “no hook; a runnable yield loop” exception in those two requirements.

2. **Do 1 / Input — The brief retains the superseded current size and leaves Do 1’s account of the ruling at its first-review wording.** Concrete reading: [brief:188](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:188) says 791 bytes; [brief:255](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:255) repeats the unqualified missed-verdict sentence, while [daemon.md:26](/Users/tom/Development/GitHub/smallcatlabs/acquisition/decisions/daemon.md:26) is now qualified and 793 bytes. **Smallest change:** correct Input’s size and add the second-review qualification to Do 1; preserve historical sizes where explicitly historical.

3. **Do 5 — Its citation for the hook’s last branch points after that branch.** Concrete reading: [brief:339](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/151-stuck-verdict.md:339) cites `door.rs:233–235`, but the branch starts at [door.rs:230](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:230), with the counter increment at 231. **Smallest change:** correct the citation to `230–231`.

What I did not check and why: no compilation, tests, gates, breakers, mutant execution or runtime measurements, as instructed. I read the three representative saved mutant logs and occupancy timeout listings, but did not independently recompute the complete 48-mutant comparison or fetch current GitHub issues/CI.

Which findings I would call nitpicks: all three above; none prevents implementation or following the stop rule.

**The step may start as drawn, with each commit gated and Do 4’s stop rule binding.**
