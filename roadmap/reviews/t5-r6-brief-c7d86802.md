`roadmap/briefs/t5-r6.md` at `c7d86802` is implementable, but Do 5 still prevents it being buildable as drawn.

1. **Answered.** Cargo-mutants 27.1.0’s `outcomes.json` retains every completed mutant and its outcome, alongside baseline-build entries; the serialized mutant supplies the proposed key. Whole-inventory comparison with duplicate entries is possible (`src/outcome.rs:62`, `106`, `180`; `src/mutant.rs:342`).
2. **Not answered.** The attribute totals are correct—197 tokio plus 28 synchronous—but they are not a scenario census, and “no tables but the limiter’s spec” is false at `a4834369`.
3. **Answered.** Execution evidence is explicitly report-only; the committed close records identities and results, while the commit-checkable requirements concern source changes and unchanged artifacts.
4. **Answered.** The HEAD guard is correctly distinguished as baseline line 599 and current line 604; the limiter locations also match.
5. **Answered.** With the preceding required headers present, missing `x-rate-limit-ip` produces “missing the limits header of rule Ip”; either replacement of `capitalise` produces “a rule outside GGG’s” instead (`policy.rs:77`, `189`, `209`).
6. **Answered in substance.** The unsupported improvement claim is removed, but the replacement still misquotes parts of the ledger.
7. **Answered.** Do 6 explicitly requires the close to reconcile the issue’s wording with the ruling’s documented component boundary (`decisions/daemon.md:25`).
8. **Answered.** The operative instructions correctly describe a description filter and require inspecting its selections; this matches cargo-mutants’ `src/options.rs:480`.

Findings, most severe first:

1. **Do 5 — The fixed before number contradicts the promised consistent row census.** [The brief’s line 143](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r6.md:143) treats 225 test functions as the before scenario count and denies existing tables, but at `a4834369`, `crates/acquisition-daemon/src/daemon/tests/parents.rs:107` defines a ten-row refusal vector consumed at line 153, `dispatch.rs:597` defines four independent retry-header cases, and `exit.rs:657` defines the lifecycle row vector consumed at line 879. These are identifiable by source inspection commands at that commit; attribute grep cannot count their scenarios. **Smallest change:** retain 197/28 solely as attribute totals, delete the fixed before scenario number and “no tables” claim, and require the same source-backed inventory at both commits, expanding independent cases in vectors, arrays and fixture rows while separating component tests and avoiding double-counting table wrappers.

2. **Do 5 / Input — The timing quotation conflates the baseline with the worker’s measurement and truncates the final recorded range.** [The brief’s line 79](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r6.md:79) attributes 40 seconds to the worker’s loaded machine, whereas [the ledger’s round 1 row](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/LEDGER.md:63) distinguishes the brief’s 40-second input, the worker’s 42 seconds under its own load, and the orchestrator’s 22–23 seconds; [round 5c](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/LEDGER.md:69) records 22–23 seconds, rather than only 23. Round 2’s 24–25 seconds is quoted correctly. **Smallest change:** reproduce those distinctions and ranges exactly, retaining the observation-only qualification.

What I did not check and why: No builds, tests, mutants, gate or daemons were run, as requested. I did not inspect or download run artifacts, verify CI execution or independently fetch #141; those remain supplied claims. I checked the attribute totals and counterexamples to the census premise, rather than calculating the complete replacement census.

Nitpicks: new finding 2. Original finding 8 is substantively resolved.

Round 6 should not start as drawn; correct Do 5’s before-census premise first.
