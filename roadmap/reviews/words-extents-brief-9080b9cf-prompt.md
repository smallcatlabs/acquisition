You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/words-extents.md, re-cut at 9080b9cf. The first cut (36c4dc9b) was reviewed cold (roadmap/reviews/words-extents-brief-36c4dc9b.md, its prompt beside: nine findings, two nitpicks); this cut answers each, and the brief's foot ("Taken from the review") says how. Read it with `git show 9080b9cf` and the file as it stands. It is round 6 of the answer-facts group (roadmap/reviews/answer-facts-map.md §3 Root 3, §10 S6 to S10), the words round, under C146 read precisely with no new ruling. Rounds 1, 2, 4 and 5 landed; round 3 (every-epoch-words.md) adds JSON words and no text. The rulings: C146, C53, C149, C116, C122, C154, C77, C3, C81, C117. The brief's form is roadmap/briefs/TEMPLATE.md; the rhythm roadmap/README.md "A step". Read; do not build or run anything; the tree is read-only to you.

Your lens, in this order. First, the nine findings: is each answered in substance (one line each: answered / narrower, sound or not / not answered)? Then:
1. Each item's words in Input against its renderer and the document's own fields (C53, C146, C116): any overclaim, any invented fact, any JSON key renamed or removed.
2. Item 3's three contexts for one summary_text (crates/acquisition-cli/src/main.rs:122, :2982, :3060): can each get its own words without three copies of one sentence (C153, a surface's wording held by one golden)?
3. Item 14's three branches against price_cmd.rs:366–376 and the facts block round 4 added (facts.places, facts.fetched): well-defined and pinned?
4. Item 13's four-surface pin on a seeded fixture with a remove-only tab: which goldens and tests change, and does the raw name still decide tab.remove_only (corpus.rs:499, C117)?
5. The holders: any test or golden still missing under a grep by each item's words; the MCP bounds the gloss edits touch.
6. Whether the step is one round for one worker as drawn, in the order given, each commit gated.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The nine findings: one line each.
- Findings, most severe first, each: the item or Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change to the brief sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether the step may start as drawn, in one line.
