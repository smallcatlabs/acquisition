## Verdict

**NARROWED.** Brief rounds after the first, and the separate commits that save reviews, can go. A single cold review of the finished step cannot replace the fix-round reviews.
**Deciding number:** since 2026-11-10, 33 of 113 P1/P2 build findings (29%) came in round 2 or later. 26 of the 27 that cite a location sit inside lines the fix commit itself changed. Under "one review, then the gate", those ship, and they include 4 P1s in product code that still stands (sort ×2, daemon-209 ×2).
**Already true:** small fixes do not carry the six moves. A small fix is 1–2 commits, and its review is folded into the close.

## 1. Ten steps end to end (all closed 11-17 to 11-24)

Commands: `steps.py` (commits by the paths they touch, `git show --numstat`) and `reviews.py`/`rows.json` (the review rows). "Brief items" counts "changes what gets built" per round. The effect column is my judgment from reading the findings.

| Step (class) | Lead | Product / brief / review / close / other commits | Brief bytes | Brief rounds (items) | Build rounds P1/P2/P3 | What the findings changed | Src lines |
|---|---|---|---|---|---|---|---|
| web-312 (small) | 0h | 0/0/0/1/0 (13 test lines inside the close) | – | – | none (test-only, C175) | – | 0 |
| search-307 (small) | 0h | 1/0/0/1/0 | – | – | 0/0/0 | – | 93 |
| web-308 (small) | 0h | 1/0/0/1/0 | – | – | 0/0/0 | – | 17 |
| daemon-320 (ord.) | 0h | 1/0/0/1/0 | – | – | 0/0/0 | test pins only, so C175 asked no review; it was reviewed anyway | 34 |
| form-landing (ord.) | 1.1h | 2/3/4/1/0 | 7,947 | 2 (4, 2) | 0/2/0 → 0/0/0 | 2 behaviour (opened rows lost on return; form missing after a save conflict) | 1,111 |
| form-values (ord.) | 0.9h | 4/3/3/1/0 | 7,994 | 2 (2, 1) | 0/0/2 | words only | 686 |
| network-288 (ord.) | 1.3h | 4/0/3/1/1 | – | – | 0/3/2 → 0/2/0 → 0/0/0 | 3 behaviour, then the "closed for good" state the fixes hardened was deleted 37 min later (network-291) | 1,419 |
| step6-places (design) | 1.6h | 4/2/3/1/0 | 7,998 | 1 (3, incl. the cross-league tab tick) | 0/0/3 → 0/0/1 | words and tests only | 1,587 |
| step6-grouped (design) | 9.8h, overnight | 6/4/5/1/0 | 7,989 | 2 (4, 2) | 1/2/0 → 0/1/0 → 0/0/0 | 3 behaviour (one page mixing store revisions, P1; league-case link; retired toggle, found in the fix) | 2,063 |
| form-editor (design) | 0.9h, plus the series' design brief (5 commits, 6 items) | 4/3/5/1/0 | 7,983 | 2 (4, 3) | 0/1/2 → 0/1/2 → 0/0/0 | 2 behaviour (the second was in the fix) | 2,128 |

- **Process commits per product commit:** small fixes about 1:1, from the close alone. The six non-small steps: 45 process to 24 product, about 1.9:1. Design steps run no heavier than ordinary ones.
- **Elapsed time is not the cost.** A step takes about an hour, and each review round about 10 minutes of wall clock. The cost is tokens, commits and review bytes.

## 2. P1/P2 rate per review

From `agg.py` over `rows.json`: 457 graded review files. A finding is a bullet starting **P1**, **P2** or **P3**.

| ISO week | Build+fix reviews | With P1/P2 | Brief reviews | With "changes what gets built" |
|---|---|---|---|---|
| W44 | 135 | 54% | 48 | 0% (ungraded) |
| W45 | 58 | 43% | 33 | 3% |
| W46 | 60 | 58% | 11 | 90% |
| W47 | 47 | 46% | 12 | 100% |
| W48, partial | 29 | 34% | 19 | 94% |

**By round, since W46** (my grouping: build and fix rounds of one step, in commit order): round 1, 53%; round 2, 48%; round 3 and later, 26%. The earlier report's "55% at round 3 and later" does not reproduce under this grouping. Over all time it is 34% at round 3 and 28% at round 4 or later.

**By author model of the reviewed commit** (joined on `Assisted-by:`), since 11-10:

| Reviews of | Fable | Opus |
|---|---|---|
| Builds and fixes | 53% have a P1/P2 (n=13, 3 P1) | 49% (n=119, 16 P1) |
| Briefs | 93% have an item, 3.4 items each | 95%, 3.2 items each |

There is no material difference. The review data neither supports nor refutes C172's split of roles.

## 3. Thirty P1/P2 findings since 11-10 (judgment)

From `sample.py`: seed 11, spread across steps, out of 114.

| Class | n |
|---|---|
| (a) caught anyway by compiler, test, gate or CI | 3 |
| (b) caught only by a cold read of the diff | 17 |
| (c) a fault in a document, tool or rig the method itself requires | 9 |
| (d) arguable | 1 |

- **Of the 17 in (b), a blind agent in a mock world would plausibly have hit 5:** #3, #7, #15, #20, #26.
- **The other 12** need crafted persisted state, races, fault injection or exotic queries. They include a fail-open account choice (#21), advice to delete a valid store (#30), and two queue P1s (#27).
- **Nine of 30 come from the method's own surfaces:** the checklist, the tracer driver, hooks, the trade sheet, the spike rig, and a brief's done-when.

## 4. Brief reviews (`briefs.py`, `briefeffect.py`)

**I read 15 brief rounds** (places; plan-jobs ×2; row ×2; form-design; form-source; editor ×2; rows ×2; rides ×2; landing ×2). By my judgment their visible items are about 25 behaviour or design gaps, about 10 done-when checks that could pass without the behaviour, and about 2 brief-text faults such as discovery commands that miss instances. **So brief reviews do change what gets built.**

**A brief review almost never says "ready".** Since 11-10, round 1 raised items in 22 of 22 reviews, round 2 in 14 of 15, round 3 in 2 of 3. C176 then builds anyway. As a readiness check it carries no information; only its items do.

**Reviewing the brief does not lower what the build review finds.**

| Steps since 11-10 | Round-1 build reviews with P1/P2 | P1+P2 per step | Round-1 P1s |
|---|---|---|---|
| Brief reviewed (21) | 61% | 1.38 | 0 |
| No brief review (63) | 52% | 1.33 | 12 |

The zero P1s are confounded: every brief-reviewed step is a GUI form step. One counter-case: the form design's brief review flagged hidden CSS controls keeping their values, and the build still shipped the fault (form-rows P2, found by the build review).

**Cost:** since 11-10, brief reviews are 139 KB against 188 KB for build and fix reviews. A round costs a review commit and an answer commit, and about 5–10 minutes.

## 5. What reviews cost beyond the reviewer's run

From `git log --since=2026-11-10`:

- 601 commits, 68 `Step:` closes.
- 158 commits only save a review (26%); about 77 are brief commits.
- 51 of 84 reviewed steps had one build round, 19 had two, 14 had three or more.

## 6. Do reviews converge?

From `nonconv.py`:

- **Later rounds review the fix.** 26 of the 27 located round-2+ P1/P2 findings lie within ±15 lines of hunks the reviewed fix changed.
- **They converge to closure.** Only 5 of 33 multi-round steps ended on a round that still found a P1/P2.
- **Much of what they catch is in tools or dead concepts.** Of the 33 later-round findings, 19 are in tools or the spike rig, or in concepts later deleted: the main account (6), network-288's closed-for-good state, the tracer (5), the spike rig (4) and the hooks (2). 14 are in product code that still stands, 4 of them P1.

**The unit is not wrong; the fix is.** `REVIEW-PROMPT.md:69` asks for "the smallest fix you believe sufficient" (in 314 review files). C173 says a finding is fixed by one rule, never patched case by case. The prompt pulls toward the patch, and the next round finds the patch's edge. Sort's round-1 P1 drew two new P1s in its fix.

## Answer

**What earns its keep:**
- First build review: 71% of P1/P2 (80 of 113).
- Fix-round reviews: 33 findings, 4 standing product P1s.
- Brief round 1: real design gaps, including the cross-league tab tick.
- The gate.

**What does not:**
- Brief round 2 and later: 37 items in 15 reviews, from a check that always finds something, and the build goes ahead regardless.
- A separate commit to save each review: 26% of commits.
- Reviewing test-only rounds: daemon-320 and gate stages, against C175's own words.

**One change** (needs the owner's ruling on C176 and C177):
- One brief review round, not two.
- A review is saved in the commit that answers it. Its header already names the reviewed commit, so provenance holds.
- In the six non-small traced steps, process commits fall from 45 to about 18.

**What it loses:** the 37 brief round-2 items. Examples are the switch sending a count despite the no-request rule, an evicted opened row's look, and `saved/open` left unclassified. I judge that a build round 1 would catch most of these; I did not verify it.

**Second change:** strike "smallest fix" from the build prompt and ask instead for "the rule the fix must hold". This targets the 26-of-27 non-convergence. It is untested.

## What nobody named

- **Briefs fill their cap.** 21 of 29 briefs since C174 are 7,500–8,000 bytes, and 3 are over. The mean before C174 was 16.9 KB. The 8,000-byte line sets their length, not their scope.
- **Review rounds polish concepts that then die.** The main account and the closed-for-good state took 8 of the 33 later-round findings before they were removed. No step has a "should this exist" check, a gap the earlier report noted. The data shows it costs mostly fix-round reviews.
- **Fable versus Opus makes no difference in review yield.** C172's rationale is not visible in what the reviews find.

## Not checked

- Token or cost data.
- Whether the brief round-2 items would have been caught at the build.
- The per-step counts for steps before 11-10.
- A full read of more than 30 findings.
- Round grouping across renamed step keys (`-fixes` and `-brief` were stripped).
- About 99 short build/fix reviews with no grade, counted as "no P1/P2".

**A slip to own:** `reviews.py` first wrote `rows.json` into the repo root. I moved it to the scratchpad at once (`git status` was clean afterwards). Nothing else in the repo was touched.

## Scripts

All in the session's scratchpad, under `<scratchpad>/reader-d/`:
- `reviews.py`
- `agg.py`
- `steps.py`
- `sample.py` and its output `sample.txt`
- `nonconv.py`
- `perstep.py`
- `briefs.py` and its output `briefs.txt`
- `briefeffect.py`
- `rows.json`

**Appendix, the 30 judgments** (file, severity, class; ✓ = a seat would hit it):
1. grouped-fixes-fc0f2f63, P2, b
2. frontends-211-096d6897, P2, b
3. store-279-23370554, P2, b ✓
4. step6-check-45f45de7, P2, c
5. process-286-fixes-a387d438, P1, c
6. step6-table-fixes-57713465, P2, a
7. form-place-0e3a1dc0, P2, b ✓
8. trade-150-fixes-23f225c1, P2, b
9. daemon-261-de4a3814, P2, b
10. sitting-round-49c70542, P2, b
11. gate-four-rulings-27efaeed, P2, a
12. process-229-7795a78f, P2, c
13. step6-sort-4a1e21c3, P1, b
14. form-link-19015729, P2, c
15. daemon-178-9ce48bb3, P2, b ✓
16. process-240-fixes-b47a6203, P2, c
17. search-276-3409c037, P1, b
18. step6-values-fixes-849ccbab, P2, d
19. 273-4d0f824e, P1, b
20. form-rows-a17bec8a, P2, b ✓
21. frontends-246-a3bc5c38, P1, b
22. gate-321-15281cc4, P2, c
23. step6-spike-fb47e47b, P2, c
24. sheet-zf-423547d0, P2, c
25. sitting-282-393024b8, P2, b
26. form-editor-fixes-310862a4, P2, b ✓
27. daemon-209-fixes-dd5d3937, P1, b
28. frontends-203-6f03e67e, P2, a
29. frontends-246-backout-a52a0c44, P2, c
30. cut-293-1-bd05048b, P2, b
