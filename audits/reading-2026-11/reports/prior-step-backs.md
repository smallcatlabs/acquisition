Read-only work only: I ran `git`, `gh` (reads), `wc`, `awk`, `bothy list|pack` and an awk pass over the gitignored `runs/gate/verdicts.tsv`. I wrote no scripts.

## 1. Proposals and what happened to each

| Proposal | Where | Disposition | Cite |
|---|---|---|---|
| Stop `pack` repeating AGENTS.md | gov. report 1 §6 (at `8e04545a`) | **Built.** | `4ea724fc`; `tools/bothy.py:81` |
| Entry limit fails only for new or changed entries | gov. report 1 §6 | **Built** as a shape check with a baseline that may only shrink (#226). | `4ea724fc` |
| `pack --path` should handle root files like Cargo.toml | #228 sitting | **Built.** Today it prints 31,546 B. | #236, `25e4d730` |
| Add `.github` to the uncited-id scan | gov. report 1 | **Built.** | `docs-check.sh:146` |
| Resolve N-ids (ground-truth claims) like C-ids | gov. report 1 | **Not built**, no answer recorded: grep finds no N-id resolution. | `tools/docs-check.sh`, `tools/bothy.py` |
| Run the breakers in the local `gate.sh --docs` | gov. report 1 | **Not built**, no answer recorded: `grep -c breakers tools/gate.sh` = 0. | — |
| A way for checklist rows to retire | gov. report 3 §6 | **Ruled** as C182. Rows went 77 → 76 (`grep -c '^\| \*\*'` at `8e04545a` and HEAD). | `decisions/process.md:40` |
| The 10 changes from the cold-start walk baseline | report 6 | Mostly **built**: the road cap (C180), commit grant (C171), small-fix path (C173), id-minting and regeneration lines in AGENTS. | seat `README.md`, table "Fates" |
| Split entries that hold more than one ruling? | #233 | **Ruled** as C192 (owner 11-20): mechanism moves to the code when the entry is next amended. | #233 comment |
| Owner quotes leave registry entries | #238 | **Ruled** (owner: "#238 is approved."), built `81cc5a1f`. | #238 |
| "Number only law" (T1/T5) | `reports/investigators/process.md` T1; THESES §5 | **Narrowed by the skeptic** (Codex): the 53/147 split was judged from headlines and left out two of C170's classes, and saved reviews cite the ids. Survived as PORTFOLIO question 6 ("no renumbering, no purge"). **Never put to the owner**: NEXT.md lists it under "Not asked now: … 6". | `dissent/theses-1-5.md` §5; `PORTFOLIO.md:52,316-320` |
| Make pack the single entry instead of the whole area file (E3, A4) | `issues/the-pack-and-the-area-file.md`; NEXT A4 | **Weakened by the skeptic**: headlines drop qualifications (`bothy.py:773`), so this is a workflow change that needs review. **Pending**: the issue was never filed (no hit in `gh issue list --search pack`), and AGENTS.md still asks for both. | `next/dissent.md:15` |
| Turn spelling greps into clippy/cargo-deny rules (T3, E2) | investigator T3; ci-tools H1 | **Pending.** #196 has been open since 11-09; briefed 11-11 from #230's spike. | #196 |
| Fold the gate-stage rulings into one; move process lore into skills | governance-context §3 | **No answer found.** | — |
| Brief reviews: one round only, saved inside the answering commit | meta step-anatomy; meta change 3 | **Weakened by the skeptic** (round-2 items were real). | `meta/dissent.md` item 15 |
| Move registry wording to the owner; reserve to him everything a person sees | meta change 2 | **Killed by the skeptic**: split plain-language questions from the transfer of authority. | `meta/dissent.md:61-63` |
| "Every step-back ends in more rules" | meta | **Weakened**: `8bd924bb` deleted 1,050 lines and added 648. | `meta/dissent.md` item 12 |
| "Review only adds" | meta | **Killed by the skeptic**: `BRIEF-REVIEW-PROMPT.md:20` already asks for a simpler shape. | `meta/dissent.md` item 10 |
| Re-measure the governance rebuild | #220 | **Open**, due after 11-30. Its gate-stage questions were ruled early as C196–C199. | #220 comments |

## 2. Measured numbers not in the files you read

**The cold-start walk baseline** (report 6 at `8e04545a`, 2026-11-09, tree `73e7e77c`):
- **What ran:** a blind Opus subagent, read-only. For four tasks it walked the path the rules prescribe up to the first edit, without editing, and logged each file with its bytes and the pointer that sent it there.
- **The four tasks:** fix #200; the locked-store refusal's wording; start the next step; a new boundary ruling.
- **Columns, in KB:** auto-loaded + required by a rule + spent resolving ambiguity:
  - Task 1: 16.7 + 45.6 + 33
  - Task 2: 16.7 + 44 + 12
  - Task 3: 16.7 + 59.3 + 8 (it read the whole 59,270 B road file to name the next step)
  - Task 4: 16.7 + 37.5 + 2
- **Caveat:** this runner had the memory index in context. It says it did not rely on it.

**#228's sitting** (`audits/governance-2026-11/seat/README.md`, 2026-11-10, tree `517836f8`):
- **Setup:** headless `claude -p --model opus` in a clone with no memory, under read-only rails. 41 calls, 248,731 ms. 13 byte counts recomputed, all matched.
- **Results:**
  - Task 1: 19.1 + 50.0 + 8.1
  - Task 2: 19.1 + ~57 + ~14 (up 13 KB, from reading a second pack)
  - Task 3: 19.1 + 14.9 + 6 (named the next step from 20.9 KB beyond auto-loaded, against the 25 KB done-when)
  - Task 4: 19.1 + ~46 + ~6
- **Stops:** 14 new ones. Each was fixed or filed (#236–#239).
- **Its own verdict:** "About a fifth … spent resolving ambiguity" (`REPORT.md:14`).

**Registry by C170 class** (investigator, judgment from headlines, `reg.py` not committed):
- Law: 53, ~35 KB
- Design: 60, 43.5 KB
- Mechanism: 54, ~35 KB
- Process: 22, 13 KB
- Restates a check: 11, 6 KB

governance-context tags the 15 newest separately: 7 restate a check, 2 lore, 5 product, 1 scope.

**Registry over time:**
- 120 entries / 64,397 B with none over 800 B (10-20) → 169 entries / 115,414 B with 25 over (11-09) (gov. report 1).
- Now 200 entries (`bothy list`).
- The shape baseline is still 47 rows, unchanged since `4ea724fc`. No baseline entry has been amended since C192: 0 hits from my `git log --since=2026-11-20 -G'**Cn —'` loop. So C192 has had nothing to act on, rather than failing.

**Pack sizes now** (`bothy pack --path … | wc -c`):
- `web/page.rs`: 25,351
- `daemon/src/gate.rs`: 4,491, against 31,392 on 11-09 before the reshape

**Reading before a first edit, per the written instructions:**
- Investigator: 62.7 / 88 (111 with lib.rs's doc) / 102 KB
- governance-context: ~70 / 118 / 159 KB
- At the reset (10-20), the AGENTS budget was 14,000 B and the checklist draft 17.6 KB (`0704a8f7:roadmap/governance/DESIGN.md:15,110`). Now AGENTS.md is 20,802 B and REVIEW-CHECKLIST.md is 26,836 B at HEAD.

**where-tokens-go** (meta):
- Removing 102 KB of documents from every call would save about 6% ($5,425 → $5,122).
- What makes up a first call (median 77.6k tokens): tool schemas 123 KB (the Artifact tool alone 37 KB), system prompt 33 KB, instruction files 23 KB, skill listing 19 KB (the harness's listing, not just the seven project skills).
- Content shares: instruction files 4.5%, skill listing 3.6%, shell reads 26.8%.
- The skeptic: "6% estimates one billing component, not the reading path's total cost."

**sessions-read-whole** (meta):
- `process.md` was read in 96 of 100 session files and the road in 88. `web/mod.rs` was re-read in 49.
- Worker prompts: median 4,468 characters.
- Byte-cap trimming showed up in 10 of 12 sessions.

**Guards:**
- My `awk` over `runs/gate/verdicts.tsv` (assuming column 6 is the exit status and column 7 is seconds):

  | Stage | Failed / rows |
  |---|---|
  | arch | 2 / 483 |
  | docs | 7 / 483 |
  | bothy | 1 / 483 |
  | trade | 0 / 483 |

  These are the investigator's counts at 447 runs, so nothing has failed since. Mean seconds: arch 2.4, docs 6.9, bothy 0.2.
- Breakers: 147 / 53 / 29 / 21 / 8 cases (arch / docs / gate / hooks / trade). The breakers job takes ~11 min against the gate's ~4. `arch-check-breakers` alone takes 439 s (ci-tools-security).
- Tools, hooks and CI draw 4.58 findings per thousand lines changed, against 1.53 for daemon source (investigator).
- `docs-check.sh` was rewritten in place across 91 commits (history-hindsight).

**Memory:**
- 86,981 B in 32 files before #229, down to 35,460 B in 20 after (`7795a78f`).
- **Now 56,610 B in 25 files** (my `wc -c` over the memory directory's `*.md`, 11-26).

**Skills:** 52,676 B of SKILL.md across 7 skills; seat (12.6 KB) and live-run (12.6 KB) are the largest.

**Reviews:** 576 files in `roadmap/reviews/`, 105 of them `-prompt.md` copies. Report 2 judged that those prompts anchor the reviewer on the author's own claims.

## 3. The owner's words

- 10-18: "keep simplicity, usability, and elegance as guiding lights. Patching edge cases and adding mechanisms … should only be done in service to the goal of making the system better and easier for you to use." (memory, `simplicity-over-mechanism.md`)
- 10-20: "documentation is a scattered mess … testing … probably needlessly complex"; 11-10: "I suspect it has accumulated some cruft"; 11-25: "question if what we thought we needed when we started is not what we actually needed" (`meta/reports/owner-interface.md:84-87`)
- 11-09, on the rebuild: "Go ahead with all of what you've got in mind." (`8bd924bb`); "I have read and approve #227 except for C172." (#227)
- 11-20, C192 and C179: "i agree with your 3 proposed rulings" (#233, #221)
- 11-25: "When agents bring me complex questions with terms that make more sense to other agents than me, I spend energy mainly to understand the decision" (memory, `owner-wants-decisions-translated.md`)
- Undated: "the documentation is too technical and complex for me to understand well enough to make a good decision" (owner-interface:78)

## 4. What others do (from `outward-practice.md` and `outward-claude-code.md`)

- **Anthropic, Jul 2026** (vendor, first-hand): cutting over 80% of the system prompt cost "no measurable loss". Keep CLAUDE.md light and about gotchas; use a tree of files loaded on demand.
- **OpenAI harness engineering** (first-hand, one team): AGENTS.md of about 100 lines as a table of contents, with `docs/` as the system of record; background "doc-gardening" agents.
- **ETH Zurich, arXiv 2602.11988** (study): developer-written context files gave about +4%, LLM-written ones slightly hurt; they raise exploration and cost.
- **McMillan, arXiv 2605.10039** (study, 1,650 sessions): file size and contradictions had no detectable effect on adherence; drift within a session did.
- **IFScale** (study, synthetic task): adherence decays as the number of simultaneous instructions grows.
- **O'Reilly, Oct 2026** (anecdote): agents follow obsolete ADRs rigidly; state a rule once and let agents challenge it.
- **Claude Code docs** (vendor):
  - `.claude/rules/*.md` with `paths:` load on file read and are summarised away by compaction.
  - Skills have `paths`, `context: fork`, and a 500-line advice; every skill listing costs context each turn.
  - Hooks replace rules that depend on discipline.
- **Nobody tested** a required reading list of 60–100 KB (outward-practice:38).

## 5. Open questions and "not checked"

- PORTFOLIO question 6 and the A4 reading rule: unasked, and the drafted issue is unfiled.
- #220's re-measure (cold agent reading at 56–95 KB) is due after 11-30.
- Not checked by any audit:
  - tokens actually read by real sessions per task (investigator §5; governance-context §5);
  - whether dense rulings cause wrong edits;
  - whether breakers ever caught a real regression (would need a read of 209 commits);
  - which arch check fired on 11-12;
  - what kinds of findings brief reviews raise.
- Reviewer family: 418 of 460 review files name no reviewer, so C177 cannot be checked per file.
- Not answered: the meta audit's "records written for readers who never come".

## 6. Contradictions and drift

- **AGENTS.md** still tells an agent to read the area file whole *and* run pack. Each pack is a subset of that file.
- **PORTFOLIO question 6 overlaps C192**, ruled 5 days earlier, for mechanism. Design entries are the only new part.
- **Memory has regrown** about 60% since #229's close (35,460 B → 56,610 B). I did not re-sort its content against C183.
- **Counts that disagree:**
  - "Pinned: nothing yet" in process.md: 16 of 24 (governance-context) against 18 (investigator).
  - census-docs counts 182 entries; there are 200.
  - Briefs near the 8,000 B cap: "16 of 29 within 104 B" against "21 of 29 at 7,500–8,000, 3 over" (step-anatomy).
- **"Stable" checklist:** governance-context calls REVIEW-CHECKLIST.md stable, but one row has gone since 11-09, and the working tree carries one uncommitted added line.
- **The reset commit:** `0704a8f7` holds only the reset's ledger rows. The reset's design (DESIGN, ENTRY-RULE, PARKS) is under `roadmap/governance/` at that commit, deleted by `cc6012db`.

**Not done:** I did not read whole-review `NOTES.md`, `dissent/theses-6-10.md`, the other drafted issues beyond grep hits, gov. reports 2 and 3 in full, or `next/reports/law.md`.
