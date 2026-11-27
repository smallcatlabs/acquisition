## Report: standing instructions outside the per-area registry files

I worked read-only. Scripts are in `<scratch>/readers/machinery/` (`rc.py` sorts checklist rows, `norm.py` counts normative sentences). Byte counts come from `wc -c`.

### 1. Sources

"Proxy" is the number of sentences `norm.py` flags as normative (never, must, only, before, refuse and similar), out of all sentences. It over-counts definitions. "Hand" is my count of separate do or don't units, which is judgment.

| Source | Bytes | Proxy | Hand |
|---|---|---|---|
| AGENTS.md | 20,802 | 54/132 | ~60 (5 invariants, 6 hard rules holding ~12 units, 11 "Where" bullets, 18 rulings of which ~8 bind agent conduct, reading paragraph 4, clone setup 3) |
| decisions/process.md | 15,334 | 60/115 | 24 rulings + 5 Not-doing, ~60 units |
| decisions/ci.md | 1,093 | 4/5 | 1 ruling (C155), ~4 units |
| roadmap/README.md | 5,187 | 20/41 | ~6 (the rest is state) |
| CONTRIBUTING.md | 1,498 | 4/11 | 4 |
| TESTING-NOTES.md | 13,936 | 32/64 | ~12 |
| GLOSSARY.md | 8,136 | 29/59 | 2 (use the specific word instead of "door"; add a term at its second document) |
| REVIEW-CHECKLIST.md | 27,436 | — | header 3 + 77 rows |
| skills: live-run / seat / site-sitting / session-close / mock-session / cold-review / research-track | 12,607 / 12,644 / 11,039 / 4,655 / 4,251 / 4,243 / 3,237 | 52 / 40 / 40 / 18 / 6 / 21 / 12 | ~35 / ~40 / ~30 / ~18 / ~10 / ~18 / ~14 |
| briefs/TEMPLATE, REVIEW-PROMPT, BRIEF-REVIEW-PROMPT | 5,274 / 4,472 / 2,389 | 21 / 16 / 9 | ~25 / ~16 / ~9 |
| search/README.md (rules part) | 13,455 | — | 5 rules + 5 traps + bootstrap, ~12 |
| .githooks (4 files) | 7,103 | — | 3 enforced behaviours |
| tools/gate.sh / docs-check.sh / bothy.py / arch-check.sh | 17,237 / 24,206 / 42,224 / 71,026 | — | machines |

**REVIEW-CHECKLIST rows by section** (`rc.py`): Network 8, Store 11, Pricing 9, Protocol 15, Search 15, Trade 4, Frontends 15, for 77 in all. Of these, 70 name a test, check, script self-test or type in "Held by". Seven name none:
- held by review: lines 22 and 120
- "no test" stated outright: line 52
- a design property with no holder named: lines 18, 86 and 100
- a seat debrief: line 107

### 2. Instructions stated in two or more places (top 25)

The ⚠ marks a pair that disagree or where one copy is stale.

| # | Instruction | Places |
|---|---|---|
| 1 | Real provider is the default; every agent `acq` runs in a mock world | AGENTS:74; mock-session:21-24; live-run:8-12; seat:175 |
| 2 | Build before starting a daemon (build order) | AGENTS:91; session-close:20-28; mock-session:17; live-run:113-123; seat:48 |
| 3 | Gate a commit on exit status, never a filtered grep | AGENTS:97; session-close:10; gate.sh:11-17; CONTRIBUTING:15; seat:211 |
| 4 | Enable the hooks once per clone | AGENTS:205; CONTRIBUTING:25-28; docs-check.sh:24; settings.json:11; each hook's header |
| 5 | `Co-Authored-By` becomes `Assisted-by:` | process:22; session-close:44; AGENTS:206; CONTRIBUTING:25; commit-msg:2-8 |
| 6 | A push stands alone in its command | process:8; session-close:89 |
| 7 | A close is one commit with a `Step:` trailer | process:20; session-close:58-72; AGENTS:171; roadmap/README:101; GLOSSARY:49,83 |
| 8 | Owner decisions go in `trigger:ruling` issues | process:10; AGENTS:167; session-close:78-85; cold-review:87; roadmap/README:73 |
| 9 | Review counts: build 3, brief 2, spike's brief 1 | process:27; cold-review:22-30 |
| 10 | Reviewer from outside the author's model family | process:28; cold-review:34,54 |
| 11 | `retired X; held by Y` | AGENTS:154; process:40; session-close:38,96; TESTING-NOTES:170; REVIEW-PROMPT:40 |
| 12 | One golden per wording, regenerated under `ACQ_UPDATE_FIXTURES` | AGENTS:152,170; TESTING-NOTES:167-181; GLOSSARY:67; seat:94; restated in 56 of 162 briefs (`git grep -l ACQ_UPDATE_FIXTURES -- roadmap/briefs`), though TEMPLATE:9 says "a copied rule drifts" |
| 13 | The road is rewritten at each close and holds no history | process:21; roadmap/README:4; session-close:73; AGENTS:172 |
| 14 | A ruling is recorded in `decisions/` before the code | AGENTS:79,164; process:7 (C170) ⚠ (see section 5) |
| 15 | Agent-facing design belongs to the session | AGENTS:82; process:7; TEMPLATE:33 |
| 16 | A GGG fact is a numbered claim; acquisition-qt only copies | AGENTS:168; live-run:23,190 |
| 17 | A live run is one RUN-LEDGER row; the owner's own use is not a live run | AGENTS:169; process:16; live-run:15-24,185 |
| 18 | No second gate or build at once | AGENTS:95; process:17; seat:48; session-close:14; TEMPLATE:72 |
| 19 | Reading order at a session's start | AGENTS:120-127,172; roadmap/README:96; process:3 |
| 20 | Declined work is a "Not doing" line | AGENTS:166; CONTRIBUTING:36; bothy docstring; GLOSSARY:93 |
| 21 | The journal is the GGG-side contract surface | AGENTS:101; TESTING-NOTES:70 |
| 22 | A deleted text is cited as `<path>` at `<commit>` | AGENTS:171; session-close:36 |
| 23 | Sizes and counts are measured after the last edit | session-close:35; TEMPLATE:74; process:19 |
| 24 | Unset leftover `ACQ_*` variables | mock-session:21; live-run:105; seat:175 |
| 25 | Issue labels `area:*` / `trigger:*` | AGENTS:166; CONTRIBUTING:33; roadmap/README:67 ⚠ (no `area:ci` label exists although `decisions/ci.md` does; checked with `gh label list`) |

### 3. What holds each rule: a machine or discipline

**process.md (24 rulings)**

| Held by | Rulings |
|---|---|
| A machine | C184 (commit-msg hook), C186 (pre-push hook and committer CI), C196 (gate `doc` stage), C197 (`artifacts` stage), C198 (trade-check), C199 (keyring workflow), C200 (arch-check 16) |
| Partly | C180 (only the 6,000-byte cap, docs-check:47; the rewrite at each close is discipline), C185 (no lock in gate.sh; the `tree` and `artifacts` stages catch some of what goes wrong), C192 (bothy enforces the size, not "property versus mechanism") |
| Discipline alone | C170, C171, C172, C173, C174 (the 8,000-byte brief cap is unchecked), C175, C176 (a check is refused under Not doing, line 48), C177, C178, C179, C181, C182, C183, C189 |

Totals: **7 machine, 3 partly, 14 discipline.**

**AGENTS.md: 6 hard rules and 11 "Where things live" bullets**

| Held by | Rules |
|---|---|
| A machine | No id in user-facing text (`crates/acquisition-cli/tests/no_ids.rs`); what a user meets (goldens, `readme_tour.rs`, docs-check 4) |
| Partly | Real provider (client.rs:1155-1160 refuses to spawn a real daemon without a terminal on stderr, but a bare `acq` still opens the owner's store); build order (`artifacts` stage); the gate decides (gate.sh exits non-zero, CI runs after a push, nothing stops the commit itself); rulings first (bothy/docs-check refuse an unknown cited id and enforce the entry form, but not "before the code"); a ruling's home (form checked, placement not); history (docs-check refuses history markers only inside registry entries); the road (cap) |
| Discipline alone | Tests pin behaviour at boundaries; a finding's home; work not done; decisions for the owner; GGG facts; live-run row; working papers; working noun |

Totals for these 17: **2 machine, 7 partly, 8 discipline.**

### 4. Checks that guard the documents themselves

Commit counts are from `git log --since=2026-11-10 --oneline -- <file> | wc -l`.

| Check | What it guards | Checker lines | Breaker | Commits since 11-10 (checker/breaker) |
|---|---|---|---|---|
| docs-check 1 | roadmap/README ≤ 6,000 bytes | 43-55 (13) | docs-check-breakers.sh, 235 lines in total across all its checks | docs-check.sh 1 / breakers 2 |
| docs-check 2 | One bullet per entry, no duplicate ids, ≤18 cross-cutting rulings, no history markers (date, commit, owner's quote, "Amended" and so on), every cited C-id real, uncited ids reported | 56-157 (102) | same | same |
| docs-check 2b | A cited hash exists, and `<path>` at `<hash>` names the path as it was | 158-234 (77) | same | same |
| docs-check 3 | Backticked identifiers in AGENTS, README, GLOSSARY, CHECKLIST and the skills exist in code | 235-279 (45) | same | same |
| docs-check 4 | README tour has one line per verb; every knob has a row | 280-321 (42) | same | same |
| docs-check 5 | hooksPath is set (a note only, never a failure) | 322-330 (9) | same | same |
| docs-check 6 | Backticked `*.md` paths exist | 331-381 (51) | same | same |
| docs-check 7 | Every golden is named by a test | 382-402 (21) | same | same |
| docs-check 8 | AGENTS' list of stages equals gate.sh's `names=(…)` | 403-426 (24) | same | same |
| bothy `check` | Cross-references resolve; Why/Pinned present; headline ≤160 bytes and ≤6 clauses, or within `decisions/shape-baseline.tsv` (50 lines); Pinned names ≤3 existing files; Not-doing lines have a trigger; registry paths exist; module-doc area routing | check() at 550-650, helpers 443-550; file 835 lines | bothy-breakers.sh, 181 lines | bothy.py 3 / breakers 2; baseline 0 |
| commit-msg and prepare-commit-msg hooks | The trailer in an agent's shell | 30 + 13 lines | githooks-breakers.sh, 220 lines | 0 / 0 |
| pre-push and claude-committers | Claude as committer on `main` | ~110 lines | same | 0 |
| gate.sh: unsets `ACQ_UPDATE_FIXTURES`; `tree` stage | Goldens are not rewritten silently; nothing is edited mid-run | 257-286 | gate-breakers.sh, 252 lines | 2 / 2 |
| arch-check | Nothing that constrains governance text. Checks 8 and 9 hold the test layout (C122) | — | — | 3 |

**Commits made to satisfy these checks.** `git log --since=2026-11-10 -E -i --grep='has room|room again|under (its|the) cap|6,?000|state file.s cap'` finds 5. Reading their bodies, 3 are trimming the road to its cap: 931a8acc ("5,703 bytes to 5,250"), 97932e96 ("at its cap of 6000") and d8852684 ("5,960 of 6,000"). A fourth, f40f53a1, trims a brief to the unchecked 8,000-byte cap. Removing history, which is what check 2's markers stand for, took 21 commits under `--grep='#293 cut 5'`. For scale: 41 of 635 commits since 11-10 touched `decisions/` or AGENTS.md, and 263 touched `roadmap/briefs` or `roadmap/reviews`.

### 5. Stale or contradictory material

- **AGENTS:79-81 against C170 (process:7).** AGENTS lists "process, scope" among the owner's classes. C170 lists invariant, boundary, the gate's stages, and the real store or live API, and says "a session writes and amends this registry's entries".
- **bothy.py docstring against C192/C170.** Rule 6 (line ~44) says a second property "takes a new id, by the owner's ruling", and rule 9 says an entry's ruling and Why "change only by his ruling". C192 grants a new id without him, and C170 lets the session amend.
- **TEMPLATE:79-80 against C171 and AGENTS:97.** TEMPLATE says "Each concern may commit after its own targeted tests; the one full gate runs … before the last commit". The other two say a commit is gated on the gate's exit status.
- **search/README.md:20-21.** It says it is "The only place process lives", carries dates ("Set 2026-09-12", "owner, 2026-09-11"), and calls itself evidence. Yet it sits on a reading path: the AGENTS workspace table says "read its README first", and research-track:8 cites its "rules in force".
- **GLOSSARY:11-12** says new text should not use "door". Since 11-10, 214 added `*.md` lines contain "door" (`git log --since=2026-11-10 -p -- '*.md' | grep '^+' | grep -c -w -i 'doors\?'`), and the seat skill alone uses it 28 times.
- **AGENTS "Procedures"** (line 181) states the cold-review trigger more narrowly than C175: it omits the gate, a workflow and the test harness.
- **roadmap/README:98-103** names six dated history files in its "Starting a session" step. They are conditional ("read when a brief points"), but they are on the path.
- **Paths.** Every other path I checked exists, 38 of them. AGENTS cites crate paths without `crates/`, which the checkers resolve as tails; TESTING-NOTES writes `crates/`.

### 6. What must be known at session start versus what can wait for its occasion (judgment)

The 12 instructions an agent needs from the start, because nothing would stop it in time:
1. Never start real-mode `acq`; work in a mock world. A bare `acq` reads the owner's store, and a read is not refused.
2. Invariants 1 and 3 at design time: one choke point, no Cloudflare retry. The checks catch only some shapes, and only after the build.
3. Commit straight to `main` on green and push without asking. This overrides the harness default of branching first and asking before a commit.
4. Which choices are the owner's and which the session's (C170). Asking wrongly or deciding wrongly is invisible.
5. Owner decisions are `trigger:ruling` issues, and no record promises him a later read.
6. Gate on the exit code, never a grep.
7. No second gate or build at once (no lock exists).
8. Record a ruling before the code that depends on it.
9. Where a finding, unfinished work and a GGG fact each belong.
10. No history (dates, steps, reviews) in code or current documents. Only registry entries are checked.
11. The tree governs over memory; a rule learned is written to the tree in the same turn (C183).
12. Read the road, then process.md, then the area's rulings before touching it.

Instructions that could be met at their occasion: the gate command list (gate.sh usage), clone setup (settings.json already does it), the workspace table (`bothy pack --path`), the cross-cutting rulings that arch-check enforces (C1, C2, C12, C46, C122), the hook rulings (their refusal messages print the fix), the gate rulings C196-C200, the review rulings C175-C177 (cold-review), C178 and C180 (session-close), C174 (TEMPLATE), and C189 (live-run).

Rough split, by section bytes (awk):
- **AGENTS.md:** about 45-50% law, the "What it is", Invariants, Hard rules, Where things live and Principles sections (≈8.9 KB) plus perhaps 1 KB of rulings. About 50% occasion: the workspace table (2.0 KB), the gate (1.2 KB) and most of the cross-cutting rulings (7.8 KB).
- **process.md:** about 30% law (C170, C171, C179, C181, C182, C183, C185 plus the intro, ≈4.9 KB) and about 70% occasion.

### 7. Vocabulary

GLOSSARY defines 52 terms (`grep -cE '^- \*\*'`). Files using each sampled term, all files / `*.md` only, by `git grep -l -w -i`:

| Term | All files | `*.md` |
|---|---|---|
| world | 487 | 310 |
| cut | 433 | 309 |
| door | 426 | 304 |
| golden | 385 | 248 |
| basis | 321 | 158 |
| seat | 275 | 203 |
| sitting | 185 | 135 |
| fold | 136 | 94 |
| park | 133 | 96 |
| oracle | 119 | 88 |

Case-insensitive matching inflates the common English words (cut, world, basis, park, fold).

House style rules and where each is stated:
- **No ids in user-facing text:** AGENTS:99 and frontends.md:14 (C116), held by `no_ids.rs`.
- **Ruling entry form** (one bullet; plain declaratives; one Why; Pinned; a gloss on each cited id; no history; headline ≤160 bytes; ≤6 clauses): bothy docstring rules 1-9, held by bothy and docs-check 2.
- **Gloss an id on first use in a brief, report or commit:** TEMPLATE:90-92, not checked.
- **History only in the records:** AGENTS:171 and REVIEW-PROMPT shape 11. Checked only inside registry entries.
- **Measured numbers only:** session-close:35 and TEMPLATE:74. Not checked.

### Five observations about the shape of the whole

1. The machines guard the commit boundary and the registry's form (hooks, the cap, entry shape, ids, paths). The rules about who decides, how a step runs and when to review, the bulk of process.md, are discipline: 14 of 24.
2. Several document checks produce work to satisfy them: three commits trimmed the road to its cap, and 21 commits under #293 cut 5 removed history (the rule docs-check's markers guard). bothy's 50-line `shape-baseline.tsv` records entries still over the shape.
3. Procedure is restated across four layers (AGENTS, process.md, a skill, and the brief template or prompts). The briefs then copy it again: 56 of 162 restate the golden rule.
4. The contradictions above sit where one text paraphrases another's grant of authority: who owns process, who may mint an id, when a commit needs the gate.
5. Working nouns carry much of the meaning. Of the sampled terms, door, world, cut and golden each appear in 248-310 Markdown files, and the one rule meant to retire "door" is not followed.

### What I did not check
- Per-check line counts inside docs-check-breakers.sh, so the breaker column gives only its total.
- Whether every row's named test in REVIEW-CHECKLIST exists (docs-check 3 claims to hold that).
- The bodies of all 20 history-marker grep hits, and the other half of `TESTING-NOTES` claims.
- The other `decisions/*.md` areas, the CI workflows, the `codex` reviewer's availability, and the README.
- I ran no gate or check. Every "held by" verdict comes from reading the checker's source, not from running it.
