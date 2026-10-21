`9501f750^..78278e0f`: the design needs correction; the build brief’s done-when is not consistently satisfiable as written. Execution-dependent criteria are **not verified**.

Findings, most severe first:

1. **P1 — `roadmap/briefs/2b-build.md:117`, shape 1:** Requiring the shared process harness’s `same_text` in protocol and planner tests conflicts with their dependency boundaries. Including appendix C’s harness requires client, store and Tokio dependencies absent from those consumers; `tools/arch-check.sh:277–288` explicitly forbids the relevant additions. Extract the diff into a dependency-free shared module, re-export it from the harness, and include that module alone in these tests.

2. **P2 — `roadmap/briefs/2b-design.md:393`, shape 1:** The proposed deletions lose coverage of successful MCP `show.against` argument forwarding. Deleting `aq3` and moving the whole site-search test removes the only successful text and trade `against` calls; the retained refusal cases and new malformed-input test still pass if the adapter validates `against` and then always calls plain `show`. Keep a small successful adapter test for both query forms.

3. **P2 — `roadmap/briefs/2b-design.md:185`, shape 2:** Checking facts across an entire transcript cannot preserve a fact separately required from multiple commands. In the age test selected for conversion, removing `fetched 3d ago` from `show` while retaining it in `search` satisfies the transcript’s fact check and permits blind regeneration, whereas today’s assertions reject it. Attach required facts to each command’s output, or retain those individual assertions.

4. **P2 — `roadmap/briefs/2b-build.md:165`:** The universal layout check rejects the layout item 5 explicitly preserves. Verbs retains `tests/documents.rs` beside `daemon.rs`, but `documents.rs` neither calls `session(` nor names `env::set_var`; client also retains only `contract.rs`, without the universally required `main.rs`. Define the permitted primary entry point for each crate and specify where merged scenario modules move; use that same definition in the check and done-when.

5. **P2 — `roadmap/briefs/2b-build.md:204`:** The environment-writer completion check contradicts the centralization it is supposed to verify. Once `session()` applies isolation in the shared harness, recursive grep necessarily finds `env::set_var` there, while converted callers need not contain it; it cannot return exactly the four specified files. Check for the centralized writer and separately verify that its in-process consumers remain isolated binaries.

6. **P2 — `roadmap/briefs/2b-build.md:64`, shape 2:** “More than ten tabs” does not stage the truncation the new test must verify. An eleven-tab fixture meets that instruction, but `Tabs::of` lists everything through thirty tabs (`tabs.rs:35,145`), so expecting ten rows fails on correct behavior and can then be ignored under item 3. Specify more than thirty selected tabs—such as the existing forty-one-tab fixture—and preserve the threshold explicitly.

What I did not check and why: I ran no builds, tests, checks, daemons or benchmarks, as requested. Gate results, timings, binary counts and platform behavior remain **not verified**. No goldens or references changed in these commits, so there was no regenerated fixture diff to assess. The range changes no runtime code.

Nitpicks: none of the findings above; each affects implementation feasibility, retained coverage or completion checks.

Brief: clarify whether transcripts capture only text invocations; “every `acq`” conflicts with the later text-only restriction and would capture changing JSON timestamps.

Plan: no additional objection.

Prompt: no change needed; its distinction between reviewing the design and verifying a future build was useful.
