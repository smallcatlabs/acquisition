# p0 fix-round review, 16ef9210

Codex, read-only, launched by the committing session with `roadmap/briefs/REVIEW-PROMPT.md`; its final answer verbatim.

16ef9210 — done-when partially holds: three findings are fixed; stage validation retains a false-green path.

- **[P3] tools/gate.sh:53 — shape 1:** Validation trusts whether `selected` exists, so an inherited environment variable can bypass rejection of an unknown stage. Concrete input: `selected=diff tools/gate.sh --only bogus` selects `diff` and can print green instead of rejecting `bogus`; the fallback sees a valid stage and cannot catch this. Smallest fix: unset `selected` before argument parsing.

Not checked: builds, tests, gate, breakers, or command execution, as instructed; the commit’s reported exits remain **not verified**. No goldens or reference files changed.

Nitpicks: the inherited-variable finding is unlikely in ordinary use and qualifies as defensive hardening, although it permits a false gate verdict.

Brief: its “stages that read no Rust” description should say “stages that build no Rust,” matching the corrected guidance.

Plan: no separate plan was supplied; the brief and prior findings established the scope.

Prompt: no change needed.
