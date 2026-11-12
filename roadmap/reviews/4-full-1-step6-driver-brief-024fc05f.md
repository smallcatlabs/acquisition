`roadmap/briefs/4-full-1-step6-driver.md` at `024fc05f`: a builder needs the measurement contract clarified before starting.

- **Decisions 5 and 7 — changes what gets built.** The measurements can omit part of what a candidate adds to the gate. Decision 5 times only `cargo nextest … --no-run` after `cargo build --workspace`; the CLI manifest explains that feature pins make additional features compile in the plain build too, and “Find first” anticipates candidates needing pins. Every done-when could be met while that added build cost goes unreported. **Smallest change:** time the preceding workspace build separately and include both deltas in the projected cost.

- **Decision 5 — changes what gets built.** The timed comparison has no tolerance or repeat policy for compilation measurements. `TEMPLATE.md`, “Done when,” requires a tolerance before comparing timed runs, but the brief simply declares the candidate-minus-control difference to be its added cost; only check execution gets repeated. **Smallest change:** specify the compilation sampling method and tolerance before measurement, and report differences within that tolerance as unresolved.

- **Inputs and Decision 1 — wording.** The prerequisite’s completion removes the copy this brief says remains running. `4-full-1-step6-spike.md`, “Done when,” removes and prunes its copy after the owner’s sitting, whereas this brief says that step is “built first” and preserves its running server for the sitting. The builder must guess the handoff point. **Smallest change:** say whether this step follows the first headless commit or the completed spike; if the latter, remove the retained-copy assumption.

- **Done when, first bullet — wording.** Filing the drafted decisions is not checkable from the resulting commit because no committed issue references are required. The bullet delegates filing to the launching session, while `decisions/process.md`’s owner-queue rule requires one issue per decision. **Smallest change:** require the record or commit message to name each filed issue, including the next-question issue for an inconclusive result.

I did not build, run applications, execute admission checks, inspect external candidate documentation, or query CI; this was a read-only review of the committed tree. All five “Find first” commands found their advertised targets. The prerequisite spike record is absent at this commit, so its eventual patch and launch could not be checked.

The unspecified handoff between the two spike briefs got in the way; the prompt itself did not.
