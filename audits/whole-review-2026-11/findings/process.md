# The process, measured

The plan's D7, with E2 and E3. Investigator's report: `reports/investigators/process.md`. Portfolio: proposals 4 and 5.

## What was found

| Measure | Number |
| --- | --- |
| Commits since 2026-10-12, by what they touch | product source 862 (34%); tests only 170; tools and hooks 147; rulings and skills 179; roadmap and briefs 587; saving a review only 439; audits and research 89 |
| Product commits per week, W44 to W48 | 277, 139, 66, 74, 45 |
| Cold reviews since 2026-10-21 | 460: 198 build, 139 fix, 123 brief; 1.43 MB |
| Reviews finding a P1 or P2, by round of one step | first 56%, second 35%, third and later 55% |
| 40 findings sampled, by kind | 19 behaviour a user or GGG would notice; 6 a claim wider than the code; 3 a test that cannot fail; 8 a tool or rig fault; 4 other |
| Findings per thousand changed lines | tools and hooks 4.58; daemon 1.53; CLI 1.39; search 0.52 |
| The registry, by C170's own classes | law 53; design a session may decide 60; mechanism 54; process 22; restates a check 11 |
| Guards | about 5,080 lines; in 447 local gate runs arch failed 2 times, docs 7, bothy 1, trade 0 |
| Reading path before a first edit | 63 to 102 KB, 40 KB of it common to every task |

## Checked by the orchestrator

- `gh issue list --label trigger:ruling --state all`: 23, none open. The investigator's reading of them (12 on process or registry wording, 8 on frontends, none on an invariant or the live API) was not re-read issue by issue.
- 86 issues open, 36 of them `trigger:step`. #196, #206, #210, #212, #213 and #214, the semantic-model audit's improvements, were opened 2026-11-09 and are still open under that label.
- 565 files under `roadmap/reviews`. `AGENTS.md`, `process.md` and the road are 40,064 bytes together.
- Not re-run: the weekly yield table, the registry tagging (the investigator's judgment over 200 headlines), and the per-line finding rates.

## Why improvements do not land

What lands has a failing input or the owner's word. The semantic-model audit's bugs closed within days; its improvements wait under `trigger:step` for "the next step in its area", and "Next" holds only features. The assessment's five cuts landed because he named them; its candidates 6 to 10 have no issue. "Smaller" has no done-when a reviewer of a diff can check.

## What a cold review of a diff cannot see

Whether the concept should exist. The main account drew five reviews and seventeen findings before it was backed out; the closed-for-good route drew six. In both the reviews made the concept heavier.

## Where this parts from the plan

- Tools draw three times the daemon's findings per line, not four.
- Brief reviews are not low-yield ceremony: since W46 they do change briefs. They are cost: 85% of the build reviews' bytes.
- The census counted 182 registry entries; there are 200.

## Position

The orchestrator briefed this investigator with the question of reviewing a result instead of a diff, so its agreement on that point is not independent. Its numbers are.

**Number only law.** The 53 entries in the owner's classes stay numbered. The other 147 move to where they bind and lose their ids. C192 already says so for mechanism; nothing moved.

**Two methods.** A diff that touches the code holding an invariant keeps its cold reviews; second rounds there found real P1s. Agent-facing work is checked by its use.

**A standing slot for removal** in "Next".

**Retire the spelling greps a type can hold**: four arch checks become clippy lines or a visibility (#196), with their breakers.

## Route

Proposals 4 and 5. Issues drafted: `the-pack-and-the-area-file`, `found-by-the-planning-read`, `c1-omits-the-client-store-edge`.

## After the skeptic

**"Two speeds" dies** (`dissent/theses-1-5.md`, thesis 4), and with it the position above called "Two methods".

- The line between law and shell cannot be drawn by crate. Exact authorization (C38), the choice of account (C121) and the listener's security (C138) live in the frontends. A brief review found that a tab id held in two leagues could authorize the other league's tab (`roadmap/reviews/4-full-1-step6-places-brief-92f0be8e.md`); no seat would have been built to hold that collision.
- "One method applied alike" was wrong: a small fix needs no brief, and rounds that change only tests or documents get no cold review (C173, C175).
- "Next holds only features" was wrong: the road names the test-shape work and the facts door (#212). Five of the six improvements cited are on no road.
- Ten P1s were found in second and later rounds.

**"Number only law" is narrowed** (thesis 5). C170's classes include the registry's own words and the gate's stages, which the 53-to-147 split left out, and the split is one reader's judgment of headlines. Saved reviews cite the ids and are kept verbatim, so an id must stay resolvable after its text moves. A promise that binds every door (additive JSON, for one) needs a home with authority over all of them, which one door's help text is not.

What stands is what was measured: where the commits and the reviews went, what the reviews found, how long improvements have waited, and what the owner's rulings were about.
