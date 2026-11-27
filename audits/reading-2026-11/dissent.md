Skeptic's read of `audits/reading-2026-11/README.md`. I made no edits. I ran `bothy list`, `bothy pack --help` and `pack --path` on two tools files, `parts.py`, `derive.py`, and a `git ls-tree` size sum.

**1. Numbers the reports don't support as stated**
- **"Reading a rule did not keep it" (point 3).** findings-against-rules.md:36 shows only that the 7 rules were in the file builders were told to read. It never matched the 45 findings to session transcripts. what-builders-read.md:51 says 28 of 40 builders never read that file. So the data fits "not reading let the fault through" at least as well as the page's reading.
- **"9 broke a ruling."** The R class includes checklist rows (line 15, #38/#40). The report also double-classes #4 (C154), #16 (C99) and #9/#33 (C35) (line 81). Counted that way, rulings broken could be about 13.
- **"442 rulings … 48 engaged, 11%."** 442 is a sum over six steps, so it counts AGENTS' 18 and process.md's 24 again for each step. It is not 442 distinct rulings. "Engaged" means cited by id. The report says rules followed without a cited id are missed (reading-precision.md:60). C188 and C200 were written by the step itself, and several ids are marked † judgment. The 51–151 KB includes briefs, issues, skills and a design doc, not only rules.
- **"searched crates/ … 319 times."** The target breakdown adds up to 400 and is labelled "all builder Bash greps" (what-builders-read.md:82). The C-id greps number 373. So 319 may include greps for things other than ids.
- **"Nobody read the checklist whole."** `AGENTS.md` asks for "its section". 9 builders read it by range before the first edit (what-builders-read.md:24), which is following the rule. The frame table calls it a failure.
- **"70 … name the test that now holds them."** The report says "name a test" (standing-instructions.md:25) and did not check that the named tests hold the row (line 177). The page's own change 3 cites a false holder claim.
- **"26 of 30 … reason written nowhere but the entry."** The reader looked only at the doc comments that cite the id (registry-sample.md:112), not at commits, briefs or reviews. It also counts C169 as partly, so the figure is 25–26.
- **"25 listed" instructions "in two to five places."** 25 is the report's cap ("top 25", :31), not a count. #12 has more than five places (56 briefs).
- **"99 of 138 reviews use one" shape.** The report says "mention" (:67). It also warns that reviewers are given the shapes, which tilts what they tag (reading-precision.md:62). The page drops that caveat.
- **"Half of recent entries restate a check."** This is an earlier audit's judgment of the 15 newest entries at that time (prior-step-backs.md:56), before C196–C200 were added.
- **"about 10k" tokens of governance.** That figure leaves out the auto-loaded `AGENTS.md` (20.8 KB) and launch prompts.
- **"by none of them for decisions/daemon.md."** No count of daemon-area builders is given.

The other figures check out: 226/319 KB, the parts shares (17/47/19/17 of 127,587 B), 156/167, and the 33 uncited ids (C193 is correct).

**2. Where inferences outrun evidence**
- **Point 4, "invisible."** 24 of the 33 uncited ids are process rulings. 36 of 40 builders do read those, in `process.md`. Change 2 removes `process.md` from builder reading, which creates the invisibility the page warns about. The top grep trigger is "id seen in the previous result" (107), so ids often reach builders through briefs and reviews.
- **Change 1.** The proposed measure, shape-tagged findings per review, cannot tell fewer faults apart from builders writing to the shapes. The shapes are not unseen today. Frontends checklist rows repeat shapes 1–3 (reading-precision.md:41). `cold-review/SKILL.md:38`, read by 20 builders, names `REVIEW-PROMPT.md`. So "No document a builder reads points at them" is false. Exposure has existed, with no measured effect.
- **Change 2, "five sentences."** The page's own source lists 12 things needed at the start (standing-instructions.md:121-133). They include C170, ruling-before-code, no history and C183. The safety claim rests on a pack run at HEAD, not at the step (reading-precision.md:61), and it needs the tests the builder has not written yet. Its measure (9 of 45, ±14 points) cannot detect a change.
- **Change 3, "costs nothing and claims nothing."** It leaves 70 rows standing as live reviewer instructions (REVIEW-PROMPT.md:17-18) whose holders are unverified. It also reverses C182's retirement path (process.md:40) without naming C182. A "twelfth line in the review prompt" moves the problem into a document that has no lifetime either and that change 1 makes builder reading.
- **Change 4 and C192.** C192 is about doc comments on code, shed "when next amended" (process.md:41). A skill is not that code. The counts in C176 (three, two, one) are the ruling's property, not its mechanism. `process.md:3` already says mechanics live in the skills, so C192 is the wrong authority to cite.
- **The owner's part.** `AGENTS.md`'s hard rule makes "process" his. Under it, changes 1, 3, 4 and 6 are his, not only change 2. Even under C170, change 6 adds meaning to C182 ("He rules on what a ruling means", process.md:7). Change 4 rearranges C196–C200, which are in his gate-stages class. Corrections 1 and 2 shrink his stated authority, not only correction 1.

**3. Claims about the current tree**
- **`AGENTS.md`** says "read its rulings file", the checklist section and the module doc. It does not say "whole". It mentions pack only as what "prints what to read". The page also drops the module doc.
- **`roadmap/README.md:96-97`** sends a session to the road, then `process.md`, then the brief. It never names the area file. Builders followed that path: road 29, `process.md` 36, area file 11. The page reads the two paths disagreeing as builders skipping a rule.
- **`TEMPLATE.md:79-80`** does allow commits after targeted tests. Whether that contradicts C171 is arguable: C171 reads as a grant to commit on a green gate and to push only gated work. "Builders do what the template says" has no count in any report.
- **`pack`** takes one `--path` (usage line confirmed). `pack --path tools/arch-check.sh` refuses ("no area governs"). But `tools/gate.sh` resolves to process, so "a file under `tools/`" is overbroad. "117 KB by repetition" has no measured repeated share.
- **`REVIEW-PROMPT.md:17-18`** does send the reviewer to the area's checklist rows, as the page says.

**4. Re-proposals the page doesn't flag**
- **Change 4** is "move process lore into skills" (governance-context §3), which was never answered (prior-step-backs.md:20). The page does not say this.
- **The title and point 6 ("the rules only grow", "Nothing drains")** repeat the meta audit's "every step-back ends in more rules". The skeptic weakened that claim with `8bd924bb`: −1,050/+648 lines (prior-step-backs.md:23).
- **Change 3** cuts across C182, ruled from gov. report 3's retirement proposal (prior-step-backs.md:13).
- **Change 2's** earlier skeptic objection (`next/dissent.md` item 4) is acknowledged, but the page does not mention its demand for a reviewed workflow decision.

**5. If only one could stand**
- **Drop change 4.** It strips the substance out of C176 under a ruling that does not cover it, and re-proposes an unanswered idea without saying so.
- **Keep the finding that knowledge needed and not delivered was neighbouring code, test harnesses and outside tools, almost never the registry** (reading-precision.md:36). It is the one measured result that says where a builder's attention actually pays.

**Not checked:** the session transcripts or `sample.json`; "sixth step back in ten weeks"; the 214 "door" lines; the C175 wording in "Procedures"; `search/README.md`; the briefs' glosses of breached ids; `dissent/theses-1-5.md` in full.
