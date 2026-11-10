---
name: external-review
description: "The Codex review of each step: what the README does not say about launching it, reading it, and when a docs-only round still gets one"
metadata:
  type: feedback
---

The launcher, the wait inside the turn and where findings are saved are `roadmap/README.md` "A step". Beyond that:

- **Close stdin:** a backgrounded `codex exec` without `< /dev/null` printed "Reading additional input from stdin..." and sat idle through a whole review (2026-10-22). Always end it with `< /dev/null`; a minute in, check the output has grown past that line. A `( codex … ) &` wrapper returns at once: wait on the log's true last line, never the wrapper.
- **Read only the answer:** the final message is a few lines of thousands; extract the last block, never the whole transcript.
- **A commit with no brief:** append three or four done-when lines to the standing prompt, written as properties, not as my conclusions.
- **A large step:** review its brief cold before any build (2g, 2026-10-27: the first review found five missed consumers and cut the step into eight rounds).
- **Large docs edits (owner 2026-10-30):** the comments-only exemption does not cover a cut, move or import of prose across many files: "i would like us to review the doc commit for A, since this is an import and larger edit". Both rounds found real faults. Propose the review up front and add to the prompt: nothing cut held mechanism or an unhomed rationale; nothing kept contradicts the registry or code; every id names its rule. A small comment fix gets none.
- **Fixing:** the owner reads the findings and the plan first ("think and let me know what you'd like to do"). Reproduce each finding as a failing test, look for the one rule beneath several, break each fix in turn to see the test fail (back-up copies outside `crates/`), and when fixes become nitpicks for unlikely cases, say which and why rather than building them. A probe that passes after a fix may pass for a different reason; say so.

Related: [[claims-match-evidence]], [[orchestrating-steps]], [[verify-before-deferring]].
