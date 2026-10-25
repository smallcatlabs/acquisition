# Codex review: the 4b sitting round, `341bf94c^..d1bca992`

Launched 2026-10-25 by the session that committed the round, at the
owner's word (`codex exec --sandbox read-only`, model gpt-6.1-sol,
`roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner
reads them before any fix is built.

---

`341bf94c^..d1bca992`: the brief’s done-when does not fully hold; execution-dependent claims are **not verified**.

Findings, most severe first:

1. **P2 — [lib.rs:1900](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-plan/src/lib.rs:1900): existing stash listings never refresh through selection plans.** After listing tab A, add tab B in-game and repeatedly run `acq refresh --all`: every plan continues fetching A alone because `ListStashes` is emitted only when no listing exists. The former refresh kind listed on every run. Include listing refreshes in selection plans and explain that newly discovered tabs require the next plan.

2. **P2 — [lib.rs:1931](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-plan/src/lib.rs:1931), shapes 1 and 2: a fresh explicit selection can be suppressed before its first application.** Fetch a tab, then compile another selection during the same second: `unspent` treats that earlier fetch as spending the new plan, contradicting “each named tab fetched however fresh.” The new integration test’s `next_second()` calls avoid this case. Distinguish fetches already present at compilation from subsequent fetches using response identity or a snapshot watermark, preserving replay suppression.

3. **P2 — [page.rs:149](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/page.rs:149), shape 2: the requested filter-group enable controls are absent.** Enter an Armour minimum and collapse Armour Filters: the condition remains active, and there is no control to disable it while retaining its value. A `details` disclosure does not implement Do 4’s enable toggle. Emit the existing trade group’s `disabled` field through a separate control and pin its folded result.

4. **P2 — [mod.rs:603](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-cli/src/web/mod.rs:603): refresh-page grouping crosses the brief’s renderer boundary.** Given a map tab and its substashes, the frontend computes membership, filters the flat rows, and constructs groups itself; the binding rules explicitly prohibit grouping in the page. Produce the grouped document in the shared layer, pin that document there, and render its groups directly.

5. **P3 — [lib.rs:68](/Users/tom/Development/GitHub/smallcatlabs/acquisition-4b/crates/acquisition-plan/src/lib.rs:68), shape 3: the crate’s introductory contract still says every plan derives from stored policy and carries its revision.** Reading it now incorrectly rules out the selection plans this range introduces; `decisions/plans.md` also still calls the one-door work “Unbuilt.” Update these sibling descriptions to identify the two authorization bases and the completed scope.

What I did not check and why:

- Builds, tests, the quality gate, rendered browser behavior, and live traffic were not run, as requested; their done-when claims are **not verified**. Static tracing found no new limiter or global-gate bypass.
- Visual agreement with the owner’s page capture and the journal’s underlying counts are **not verified** from committed evidence. The ledger explicitly records the 180-versus-245 discrepancy.
- The literal “three callers” condition is unmet: the shared list parser has CLI and page callers. Keeping MCP array entries whole is consistent with its documented contract.
- The halt rendering exists, but the added jobs golden exercises a completed queue without a halt.

Nitpicks: finding 5 is documentation cleanup; I would not dismiss findings 1–4 as unlikely-case nitpicks.

Brief: specify selection-list freshness and same-second selection behavior explicitly.

Plan: reconcile the renderer-only boundary with grouped document production before implementation.

Prompt: the range notation was clear and did not obstruct the review.
