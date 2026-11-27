## Cold-review findings, Nov 10–26: written-rule breaches vs gaps, and how each was answered

Nothing in the repository was changed. All scripts and working files are in `<scratch>/readers/findings/`: `reviews.py` (a copy of the earlier audit's script, changed only to write `rows.json` into scratch), `draw.py`, `next.py`, `files.txt`, `sample.json`.

### Population and sample
- `python3 -I reviews.py` parsed 468 saved reviews. Of these, 138 are build or fix reviews added since 2026-11-10 (88 build, 50 fix). 31 of them have no findings at all.
- `draw.py` splits each review into bullets and keeps those with P1 or P2 in the first 40 characters. That gives a pool of **123 serious findings (19 P1, 104 P2) across 70 reviews**.
- Sampling used seed **20261126**. I shuffled the pool and walked it in order. Each time a review was drawn, every P1 in that review was added too. I stopped at 45 findings: **13 P1 and 32 P2**, from 29 reviews.
- The full list is `sample.json`. The main ones are named in the illustrations below.

### Counts table (45 findings)
| Class | n | Findings (#) |
|---|---|---|
| R, own area | 7 | #7 C183 (process), #21 C99 (trade), #25 C23 (daemon), #29 and #30 C16/C98 page basis (frontends), #37 C187, #43 C121 |
| R, other area | 2 | #38 and #40: C151 (store.md), plus checklist rows "fall-through past an unreadable row" and "blind write over what this build cannot read", applied to a verbs/frontends change |
| W | 9 | 5 breach the step brief's slot/value rule (#11, #12, #13, #45 sort; #1 twelve-value bound); #20 trade-150 brief note; #34 next-a2 decision 2; #35 seat fault 1; #28 the step's own unreadable-outcome refusal, bypassed |
| N | 11 | #3, #14, #15, #17, #18, #27, #31, #36, #39, #42, #44 |
| M | 16 | #2, #4, #5, #6, #8, #9, #10, #16, #19, #22, #23, #24, #26, #32, #33, #41 |

Put together: 18 of 45 (40%) broke something already written. 9 of those were registry entries (R) and 9 were briefs (W). 11 (24%) were product faults no rule covered. 16 (36%) were faults in the project's own tooling, tests and documents (M).

### Answers table
I found each answer by `git log --grep=<review name>` after the commit that saved the review, then read `git show --stat` and the commit body. A finding counts once, under its main answer.

| Answer | n |
|---|---|
| Structure: a test, type, check, breaker or pinned code change | 38 (#39 is "structural, no test": the account is now chosen once) |
| Code fix with no pin | 2 (#32, #33 in `aef881ea`; the owner later had the mechanism removed in `055eb80e`) |
| Registry entry added or amended | **0**. None of the answering commits touches `decisions/`. |
| New or restored checklist row | 1 (#4, `3560ada9`) |
| Doc sentence | 2 (#2: brief amended to want less, `49ee9de6`; #7: the private memory file, recorded only in the commit message `5debe4fa`) |
| Other: research sheet rows | 1 (#26) |
| Nothing ("answered, not changed") | 1 (#31, `e9bcae07` item 3) |

### Class R: was the builder told, and why was it breached?
- **All 7 own-area breaches sit in the file `AGENTS.md` tells a builder to read before touching that area.** The 2 other-area ones (#38, #40) sit in store.md (C151) and the "Pricing and intent" checklist section, while the change was in verbs/frontends. A builder following the instruction literally would not have read those.
- I did not check whether each step's brief glossed the ids.
- **No review says why a rule was broken.** Only 1 of the 9 R reviews names the id (#43 cites C121). The others describe the fault, or cite a reviewer shape or a test name (#29 names `c16_every_page_s_basis_line…`).
- The answering commits don't say why either. `f1f9cade` and `dd5d3937`, for example, describe the fix and the rule beneath it, not how the rule was missed.

### Class M: how much was the written rules themselves being wrong?
Only **2 of 16** M findings are a governance or doc sentence that claimed more than the code does:
- #4: a REVIEW-CHECKLIST retirement claimed tests held a clause they did not. This also breaches C154.
- #26: a `trade-sheet.py` docstring justified leaving a question unasked.

The other 14 are tooling faults:
- tests that cannot fail or that hide a fault: #2, #16, #22, #23, #24
- a harness race: #10
- checkers or hooks: #5, #6, #19, #41
- the live-run driver `tracer-rung.sh`: #8, #9, #32, #33. #9 and #33 could lose persisted intent, which touches C35. I kept them in M because the driver is project tooling.

So the registry was never itself the wrong text in this sample. The checklist was once.

### Eight illustrative findings
1. **R own** #43, saved-247 P2: `--account Alice`, matching two accounts, offered Bob's realm instead of `ambiguous_account` (C121). Fixed with a test, `7ffb7aec`.
2. **R own** #25, daemon-209 P1: a restored parent reported success after dropping an unreadable child (C23 tally rule). Fixed with a test, `dd5d3937`.
3. **R other** #38, frontends-246 P1: the main-account read failed open and ran Bob's job (C151, plus a checklist row in another section). Fixed with a test, `f1f9cade`.
4. **W** #45, step6-sort P1: a card's value differed from its trade filter's slot, which the brief required to be equal. Fixed with a test, `9fe51220`, then failed again as #12 and #13 in the next round.
5. **N** #27, cut-293 P2: the version and table checks read separate snapshots, so a fresh store got the "delete it" remedy. Fixed with a test, `38794514`.
6. **M (doc)** #4, step6-check P2: a checklist retirement claimed more than its tests watch. The clause was restored to the checklist, with no test.
7. **M (hook)** #6, process-240 P1: the pre-push hook's printed rebase drops merge-only edits. Fixed with breakers, `b47a6203`.
8. **M (test)** #24, daemon-209 P2: the parked-job pin passed without the persist it claims to pin. The test was narrowed.

### Step 5: citations and registry churn (all 138 build/fix reviews)
- `xargs grep -l -E '\bC[0-9]{1,3}\b' < files.txt | wc -l` gives **3 of 138** reviews citing any C-id: `4-full-1-form-source-01cdf862`, `daemon-209-fixes-cc8db92f`, `saved-247-0cf74928`.
- The ids cited are C23, C121, C134 and C194, once each. There is no top ten; only 4 distinct ids appear.
- For contrast: 99 of 138 mention a "shape" from `roadmap/briefs/REVIEW-PROMPT.md`, 3 mention the checklist, and 1 mentions AGENTS.md or an invariant.

Registry churn, from `git log --since=2026-11-10 -p -- 'decisions/*.md'`, counting added lines that start `- **C`:

| ISO week | commits | entry lines added | of which new ids* | distinct ids |
|---|---|---|---|---|
| W46 (Nov 10–16) | 10 | 18 | 2 | 13 |
| W47 | 14 | 21 | 8 | 15 |
| W48 (to Nov 26) | 15 | 17 | 5 | 12 |

\*"New" means no removed line for the same id in that commit. Moves between files count as amendments. `bothy.py list` currently ends at C200.

### What these numbers do not show, and what I judged
- **Judged, not counted:**
  - The class of every finding. Where a finding fit two classes I picked one; #4 is M and R(C154), #16 is M and R(C99), #9/#33 are M with C35.
  - That the reviewer's 11 "shapes" are lenses rather than builder rules, so a shape alone did not make a finding W.
  - That the `tracer-rung.sh` and `trade-sheet.py` findings belong in M.
  - That #39 (reading one fact in two places) is N, since only the reviewer's shape 7 names it.
  - Whether a registry text "covered" a fault, which is my reading. For R, I searched the area file and the checklist at the reviewed commit with `git grep <sha>`. For N, I did a few keyword searches per finding, so N may be overstated.
- **Not checked:**
  - findings formatted without P1/P2 in the first 40 characters (the pool may undercount)
  - P3 findings
  - brief reviews
  - whether each brief listed the breached id
  - whether fixes held in later rounds, beyond the #45 to #12/#13 case
  - commits that answer a finding without naming the review
- Reviews almost never cite ids, so the low citation count reflects how the reviewers report as much as how the registry is used. Reviewers are prompted with the shapes and the checklist, not the registry.
- n=45 gives a margin of roughly ±14 points on any one share.
- The whole window is 17 days.
