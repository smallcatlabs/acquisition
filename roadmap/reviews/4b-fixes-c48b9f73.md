# Codex review: the 4b fix round, `0b1fc1ff^..c48b9f73`

Launched 2026-10-25 by the session that committed the fix round, at
the owner's word (`codex exec --sandbox read-only`, model gpt-6.1-sol,
`roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner
reads them before any fix is built.

---

`0b1fc1ff^..c48b9f73`: the brief’s done-when does not hold; the owner’s sitting remains pending, execution-dependent criteria are **not verified**, and one defect remains.

Findings, most severe first:

1. **[P2] crates/acquisition-cli/src/web/form.rs:262 — shape 7:** Selecting counts silently discards a supplied sort, bypassing the shared view-selection rule that refuses this combination. Input: `search?realm=pc&view.count=rarity&view.sort=ilvl` produces counts, whereas the equivalent CLI request is refused because sorting applies only to rows. Construct the view through `Shape::of`, as the CLI does, and test this combination; the refusal should tell the reader to clear the sort or the count key.

What I did not check and why: no builds, tests, gate stages, cargo-deny, README-tour execution, browser interactions, or live traffic, as instructed. Their execution-dependent done-when criteria are **not verified**. The picker’s behavior was inspected statically; its acknowledged lack of an executable pin remains. The commit records the owner’s sitting as still ahead.

Nitpicks: none; the finding is reachable through the ordinary form.

Brief: distinguish the fix round’s acceptance criteria from the full build’s still-pending sitting and execution criteria.

Plan: reuse the shared view-selection function so new adapters inherit its refusals.

Prompt: explicitly distinguish committed test coverage from evidence that those tests pass.
