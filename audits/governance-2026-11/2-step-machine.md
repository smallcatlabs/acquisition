STEP-MACHINE AUDIT (roadmap/), read-only, 2026-11-09

## 1. The five findings that matter most

1. **"Current step" has become a history log.** It is 25,972 bytes. Of its 127 sentences, about 81 describe closed work. Only the last sentence (README:915–917) says what to do next, and the section points to `LEDGER.md` 38 times while retelling the same closes. It grew from 114 bytes (10-19) to 3,193 (10-31) to 25,972 (11-09).
2. **The same close is recorded in five to seven places.** One small exception: the five seat-issues rounds A–E (closed 11-04) have no LEDGER row at all. They live only in README prose (0 matches for `#169` in LEDGER.md).
3. **The first cold review of a brief pays off; rounds after the second pay much less.** Round 1 finds about 8 findings and round 2 about 2–3 (heuristic). From round 3 on, most reviews find 0–1, often wording or citation fixes. The exceptions continue a design that is still moving (4-full-1-cards: 12, 9, 8). Seventeen of the 37 reviewed briefs went to 3 or more review rounds, all after the "never a third time" rule landed (10-28/10-29).
4. **Build and fix reviews find real faults cheaply.** In 97 build reviews, regex counting found 6 P1 and 112 P2 tags, at about 2 KB per review. The faults include a P1 memory bound (`56-list-paging-fef5eadb.md`) and fixes that were incomplete (`2g-r2-fixes-a01e748e.md`, `c135-fixes-380fccb1.md`). This is the part of the machine that most clearly earns its keep.
5. **The process text outweighs the code it governs, and nothing is ever cleared out.** roadmap/ is 4.15 MB tracked, against 14.05 MB in crates/ (goldens included). For three sampled steps, the brief, its reviews and its ledger/README text came to 0.4–0.85× the bytes the build added to crates/. The LEDGER folding rule has never been applied: no commit to LEDGER.md has ever deleted more than one line.

## 2. Measured facts

**README by section** (`awk` split on `## `):

| Section | Bytes |
| --- | --- |
| The owner's rulings for this road | 23,230 |
| Roles | 1,263 |
| A step | 2,603 |
| The phases | 1,743 |
| Pull-forward | 2,130 |
| Open | 1,267 |
| Starting a session | 465 |
| Current step | 25,972 |
| Whole file | 59,270 |

**README size over time** (`git show <h>:roadmap/README.md | wc -c`, every ~31st of 307 commits):

| Date | Commit | Total | Rulings | Current step |
| --- | --- | --- | --- | --- |
| 10-19 | a987d02f | 6,018 | 962 | 114 |
| 10-21 | d232cf52 | 38,858 | 962 | 489 |
| 10-24 | e19838f2 | 14,311 | 803 | 606 |
| 10-27 | 1e4c2547 | 28,246 | 3,182 | 544 |
| 10-27 | e60ab401 | 37,729 | 3,182 | 220 |
| 10-28 | b4f8420e | 11,322 | 2,797 | 377 |
| 10-29 | 4af4113c | 14,311 | 5,001 | 881 |
| 10-31 | ef7e4b3f | 23,800 | 10,539 | 3,193 |
| 11-02 | 9fea3a30 | 38,162 | 15,240 | 12,854 |
| 11-04 | 2a250c54 | 46,927 | 15,240 | 21,619 |
| 11-09 | 73e7e77c | 59,270 | 23,230 | 25,972 |

- The file was cut back twice (10-21 → 10-24, 10-27 → 10-28). Since 10-28 it has only grown, by 5.2×.
- 307 commits touch it, all since 10-19 (`git log --since=2026-10-19 -- roadmap/README.md | wc -l`), up to 39 in one day (10-27). Net +2,718/−1,801 lines (numstat sum).
- Inside Current step (`grep -o`): "Next, Phase 3, the owner's league run, at his word" appears 5 times, and "cut N times" phrases 26 times.

**What a fresh session needs from Current step** is the last sentence:
> "Current step: the owner's morning at `acq web` in his own store under the live-run skill; step 6, the dense table, follows the morning, its spike first (§7 step 6)."

**Duplication for three steps** (bytes; README figures from a whitespace-joined python find):

| Place | t5-r3 (10-31) | 157-leagues-held (11-04) | 4-full-1-cards (11-08) |
| --- | --- | --- | --- |
| README Current step | 195 | 400 | 451 |
| README rulings list | 0 | 3 mentions of #157 | 0 |
| LEDGER row | 2,788 | 2,371 | 2,187 |
| HISTORY.md | in the t5 group line | in the answer-facts group line | none yet |
| Close commit message (`git log -1 --format=%B \| wc -c`) | 2,491 (e63ce2c7) | 876 (24ad0bd3) | 182 (247ab353) |
| Brief | 18,486 | 24,694 | 54,823 |
| Brief's own Taken/Cuts sections | yes | yes | 3,237 (Cuts) + 3,329 (For the owner) |
| Review files | 20,116 (8 files, 3 prompts) | 22,197 (6 files, 2 prompts) | 38,242 (4 files) |

- GitHub issues were not measured, because gh is outside my constraints.
- What only one place holds:
  - The LEDGER row holds the review commit ids and the left-open homes.
  - The review files hold the verbatim findings.
  - The brief's "Cuts"/"Taken from the reviews" sections duplicate the review files. There are 25 such sections, 62,252 bytes across 29 briefs (awk).

**LEDGER.md** (python over `^|` rows):
- 96 rows. Bytes per row: min 58, median 1,198, max 4,806 (t5 round 6).
- The "Left open" column is a median 230 bytes, 30,392 of 138,283 bytes in total.
- The large rows hold full measurement narratives: mutant counts, census figures, review-by-review fates, gate test counts. In other words, the story the README says belongs in commit messages, not "a commit range and a close date" (README:42).
- Who reads it (`git grep` outside README and LEDGER): about 20 briefs (mostly "rounds N closed at X, their rows"), HISTORY.md ×3, search/README ×2, one audit. AGENTS.md, the skills and TEMPLATE.md do not reference it. The session-close skill mentions only "write its row".

**Briefs** (`wc -c roadmap/briefs/*.md`):
- 127 files. Sizes: min 3,003, p25 6,712, median 13,286, p75 24,694, max 89,404 (2g-build).
- Template boilerplate is small. Rules plus Report run 243–2,094 bytes per brief (2–10%), because briefs cite `TEMPLATE.md` rather than copy it.
- Read and Do are the bulk. 4-full-1-cards has Input 13,231 and Do 20,764.

**Ten briefs** (`git log --oneline --follow`):

| Brief | Started | Commits | Brief reviews | Bytes |
| --- | --- | --- | --- | --- |
| 2c-build | 10-21 | 3 | 0 | 19,571 |
| 63-store-remainder | 10-28 | 2 | 0 | 23,912 |
| t4 | 10-30 | 1 | 0 | 7,090 |
| 4-0-door | 10-31 | 5 | 4 | 29,272 |
| t5-r3 | 10-31 | 3 | 3 | 18,486 |
| transport-seam-2 | 11-01 | 4 | 3 | 30,014 |
| 155-coverage-fold | 11-03 | 3 | 2 | 21,203 |
| seat-issues-b | 11-04 | 2 | 0 | 6,889 |
| 44-folder-once | 11-04 | 1 | 0 | 3,560 |
| 4-full-1-cards | 11-08 | 4 | 3 | 54,823 |

**Are old briefs read again?** (`git grep`)
- 76 of 127 briefs cite another brief. The most cited are design briefs within the same group: t5-design ×9, 4-0-spike ×6, 4-0-design ×6.
- Code cites briefs 29 times across 20 files (2g-build, 63-store-remainder, 4-full-1-pages).
- decisions/ cites one brief, and REVIEW-CHECKLIST.md cites 3.

**Reviews** (`ls roadmap/reviews` plus a python classifier on file names; findings counted by P-tags or numbered bold heads, over all 281 non-prompt files rather than a 30-file sample; I hand-read 27 files to check the character):

| Kind | Files | P1 | P2 | P3 | Findings (heuristic) | Zero-finding files | Bytes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Brief/design | 93 | 0 | 9 | 3 | 704* | 9 | 680,626 |
| Build | 97 | 6 | 112 | 69 | 218 | 18 | 210,266 |
| Fix round | 91 | 2 | 53 | 47 | 115 | 31 | 136,372 |

\*Brief reviews number their findings and are not P-tagged. Counts in rounds 2 and later include "Answered/Narrower" re-checks; the excluded counts are below.

- There are also 105 `-prompt.md` files, 462,260 bytes, committed from 10-27. They are the author's per-review instructions to the reviewer. Each restates the author's reading of the tree, line citations included, so the "cold" reviewer is anchored on the author's claims. That is a judgment, but see `t5-r3-brief-66086b68-prompt.md`.

**Brief review yield by round** (new findings, "Answered/Narrower" re-checks excluded):

| Round | Reviews | Mean | Median | Zero |
| --- | --- | --- | --- | --- |
| 1 | 37 | 8.0 | 8 | 4 |
| 2 | 31 | 3.3 | 2 | 12 |
| 3 | 17 | 2.6 | 1 | 7 |
| 4 | 4 | 1.2 | – | – |
| 5 | 3 | 0.7 | – | – |
| 6 | 1 | 2 | – | – |

- Per brief: 4-0-life went 8, 4, 1, 0, 0. transport-seam-design went 10, 5, 4, 1, 1.
- Code review rounds: 54 steps had 2 reviews (build + fix), 13 steps had 3–5.
- More than 2 rounds is common: 17 of 37 reviewed briefs, and 13 of 104 steps' code.

**Cost ratio** (`git diff --shortstat`, plus added bytes via `grep '^+' | wc -c`, crates/ only):

| Step | Process bytes (brief + reviews + README + LEDGER) | crates/ lines | crates/ added bytes |
| --- | --- | --- | --- |
| t5-r3 | 41,585 | +1,594/−1,147 | 68,578 |
| 157-leagues-held | 49,662 | +2,521/−181 | 105,514 |
| 4-full-1-cards | 95,703 | +2,267/−375 | 114,456 |
| 44-folder-once | ~5,100 | +73/−43 | 3,507 |
| seat-issues-b | ~25,100 (no LEDGER row) | +1,695/−177 | 78,664 |

For seat-issues-b, the range 935d4f51^..4bd32311 is 14 commits, two of them review commits.

**Totals:**
- `du -sk`: roadmap 5,132 KB; reviews 2,324 KB; briefs 2,388 KB.
- `git ls-files | xargs wc -c`: roadmap 4,147,337 bytes; reviews 1,555,759; briefs 2,179,091.

**Folding and retention:**
- No LEDGER commit has ever removed two or more lines (numstat over 80 commits).
- The phase 0, 1 and readiness rows were written as single rows, not folded later.
- The only retention rule is "deletes nothing / stay there for good" (README:41, 418, 434).

**Dated evidence** (`git grep -l`):
- STATE-2026-10-19 is cited by 10 files, the newest p1/53 from 10-28. No commit since 11-03 adds or removes a citation (`git log -S`).
- STATE-2026-10-26 is cited by 15 files, 0 recent.
- TIME-SEAM is cited by 25 files, 0 recent.
- GUI-NOTES is live: 18 files, 9 recent commits, cited by 4-full-1-cards.
- TEST-SHAPE is cited by 16 files, including TESTING-NOTES.md.
- LANGUAGE-RETROSPECTIVE is cited by nothing, and README:521 does not list it.

## 3. What earns its keep

- **Build and fix cold reviews.** They found 6 P1 and 112 P2 across 97 builds. Fix reviews caught fixes that did not fully land (2g-r2, c135, 56-list-paging's ineffective pin). About 31 of 91 fix reviews came back clean, which is cheap confirmation at a median of about 1.5 KB.
- **Round 1 of a brief review.** About 8 findings, mostly substantive: contradictions with a ruling (`155-subject-block-brief-263ab9db.md` #1), missing test inventories, unbounded lists, wrong citations. Those faults would otherwise surface in the build.
- **TEMPLATE.md.** Its rules are cited, not copied, so boilerplate stays at 2–10% of a brief. Its rules record their origin (e.g. "(t4)"), which is useful lineage.
- **HISTORY.md.** 4,342 bytes for the whole project, one line per group. Lean, and it works.
- **LEDGER's "Left open" column.** Median 230 bytes, and the one place that gives each leftover a home.

## 4. Friction, cruft, vestige

- **README:528–917.** The Current step history log. A fresh session reads about 26 KB to get one sentence, and "Read this file first for any step" (README:6) makes that 59 KB every session.
- **README:13–369.** The rulings list mixes rulings with chronicle. For example, README:345–356 records issue relabelling and an `AGENTS.md` byte-budget note. It largely parallels the decisions/ registry (judgment).
- **The brief re-cut loop.** Seventeen briefs went to 3–6 reviews. From round 3 on the yield is a median of 1 finding, and the briefs grow "Cuts" and "Taken" sections (62 KB total) that duplicate the review files.
- **The 105 `-prompt.md` files** (462 KB). They are committed process scaffolding, and they bias "cold" reviews with the author's reading.
- **LEDGER rows of 3.5–4.8 KB** (rows for t5 r4, r5a, r5b, r6, describe, transport-seam-3). These are commit-message stories copied into a table cell.
- **No retention or folding has ever run.** reviews/ alone is 1.56 MB and only grows.

## 5. Contradictions and stale statements

- **README:419–420** says "nothing is reviewed a third time unless the owner asks". 17 brief artifacts and 13 code steps had 3 or more reviews, and Current step celebrates "cut six times under five cold reviews" (README:681–682).
- **README:42 vs LEDGER.md** (the ruling of 2026-10-26): "a ledger row is a commit range and a close date". The median row is 1,198 bytes, with measurement narrative.
- **README:525–526** says "nothing else here changes except at a close". 307 commits in 22 days.
- **README:494–496** says an Open item "lives here only until the next close". Items from 10-19 to 10-29 (`git blame`) remain after dozens of closes.
- **README:500–501**: "Run tools/tracer-rung.sh … not run since". RUN-LEDGER rows of 2026-11-03 record the tracer driver run twice, and `tools/tracer-rung.sh` was edited for that run (1f31a394). Probably stale; a judgment, since the row does not name the file.
- **README:446**: phase 2 "ahead: 2h … (#38, #46, #47)". Not verifiable without gh; check it.
- **README:450**: phase 4 says "ahead: 4-0 the time seam … after `alpha.2`, 4-full". 4-0 closed (t5 and the transport seam, README:611, 723), and 4-full part 1 steps 1–5 closed before alpha.2 under the ruling at README:252.
- **README:464–469**: Pull-forward item 1 (4-0) is done but still listed, against the rule at README:462.
- **README:470–490**: Pull-forward item 2. The first tranche is done (README:545), t5 is done, and t6 was re-sequenced after 4-full (README:255–257). Still listed.
- **LEDGER.md**: no rows for seat-issues rounds A–E, although the rhythm (README:429) requires a row at each close.
- **README:521**: the dated-evidence list omits LANGUAGE-RETROSPECTIVE-2026-11-09.md.

## 6. Ideas

1. Make Current step one paragraph of what comes next, rewritten (not appended) at each close; the history already lives in LEDGER and git.
2. Cap brief reviews at two, with a third only on the owner's word, and enforce it. The yield curve supports this.
3. Stop committing `-prompt.md` files, or make the prompt not restate the author's reading of the tree.
4. Hold LEDGER rows to range, date and left-open homes, and move the narratives to the close commit message, as the ruling says.
5. Fold or archive reviews/ and briefs/ at each phase close (for example, a tag and a delete), since git keeps them anyway.
