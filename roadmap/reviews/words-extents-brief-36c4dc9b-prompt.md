You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/words-extents.md, committed at 36c4dc9b. Read it with `git show 36c4dc9b` and the file as it stands. It is round 6 of a group designed in roadmap/reviews/answer-facts-map.md (§3 Root 3, §10 steps S6 to S10; reviewed twice), the words round: fourteen items, each a count or an age whose words do not name its extent, under C146 read precisely, with no new ruling. Rounds 1, 2, 4 and 5 landed (fb6c5c02; 0d17cdaa^..2977ea2c; 052e0fe4; a6c2b121^..497575f5); round 3 (every-epoch-words.md, C163) adds JSON words and no text and may land before or after. The rulings: C146, C53, C149, C116, C122, C154 (AGENTS.md, decisions/frontends.md); C77 and C3 (decisions/plans.md, decisions/daemon.md); C81 (decisions/pricing.md). The brief's form is roadmap/briefs/TEMPLATE.md; the rhythm roadmap/README.md "A step". Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order:
1. Each of the fourteen items: is the fault real at the cited lines, and are the proposed words (Input) a function of the document's own fields (C53), naming the extent exactly (C146), with no ruling id in them (C116)? Flag any item whose fault is not real, whose words overclaim, or whose JSON change is not additive (a renamed or removed key).
2. Item 11 (the plan's two measures of one age) against C146's bar on renames and C38 (the envelope bound): is making the note's measure "before generated_at" sound, and what does documents.rs:429/:483 pin that it would break?
3. Item 12 (C77's pin): is the test as drawn a pin of C77's property (freshness judged at compile against the window; a later replan refetches what aged), and does the plan crate's compile expose what it needs (crates/acquisition-plan/src/lib.rs, grep max_age_seconds and the freshness test)?
4. Item 13 (the remove-only name): is one rule in the store's FetchedLocation::of (coverage.rs:35–44) consistent with Place's rule (search/corpus.rs:291–325, :301, :449) and C117 (tab.remove_only is a field), and which goldens change?
5. Item 14 and items 1, 3, 6: the exact sentences against their renderers (store_cmd.rs:788–812, :664–672; the jobs text; price_cmd.rs:370–376) and the positional inline tests they move.
6. The holders: any test or golden the brief misses under a grep by each item's words; the MCP bounds (surface_size.rs:32, search_tools.rs's sections) the gloss edits touch.
7. Whether the step as drawn is one round for one worker, built in the order given, each commit gated; or whether it should be cut, and where.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The fourteen items: one line each (real and sound / real, words to change / not real).
- Findings, most severe first, each: the item or Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
