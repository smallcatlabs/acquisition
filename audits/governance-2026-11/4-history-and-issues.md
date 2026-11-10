## The quantitative record: what git history and the issue tracker show about effort and process since the 2026-10-19 reset

Scope: 1,670 non-merge commits from 2026-10-19 00:00 to now, plus the 338 commits of 2026-10-05..18 for contrast. Every number below names the command that produced it. Scratch scripts are at `classify.py` and `msg.py` in the session's scratch directory.

Note on the reset commit: `git log -1 --format='%ad %s' 0704a8f7` gives **2026-10-20**, "roadmap 1e: the ledger rows…". That is the close of the governance phase. The roadmap commit that opened it is a987d02f, dated 2026-10-19. ISO week W43 begins on Mon 2026-10-19, so W43 is the first week after the reset. W46 is today only.

### 1. The five findings that matter most

1. **About half of all commits since the reset touch only governance docs, up from 6% before it.** Governance-only commits: W42 14 of 235 (6%), W43 122 of 369 (33%), W44 445 of 872 (51%), W45 221 of 418 (53%). Most of these touch only `roadmap/`: 685 of 1,670 (41%). Commits touching `crates/` at all: 702 of 1,670 (42%).
2. **Code still gets most of the added lines.** `crates/**/src` plus `crates/**/tests` took 64.7% of the 261,526 lines added since 10-19. `roadmap/` took 24.4%, and `decisions/` took 0.2%. The roadmap tree is now 4.15 MB: briefs 2.18 MB in 127 files, reviews 1.56 MB in 388 files. Production source under `crates/**/src` is 5.40 MB.
3. **Review is the dominant rhythm.** 570 of 1,670 subjects (34%) mention review or findings, and 437 review files were added. In the steps I checked, roadmap-only commits outnumber commits touching crates by 2 to 4 times: t5 had 80 to 28, 4-full 69 to 21, transport seam 39 to 19.
4. **The AGENTS.md byte budget is not a constraint in practice.** It was raised seven times, from 8,000 to 16,600 (more than doubled in 22 days). On 10-26 it was made a non-failing note (5eda514f). Today the file is 16,689 bytes, over its own budget. Budget commits mostly raise the number; actual cuts are rare and small.
5. **The issue tracker carries two different populations.**
   - In-flight work closes fast: median 1.12 days, 56 of 119 closed the same day.
   - 34 of the 78 open issues are 2026-10-22 bulk stubs, median 234 bytes, that point at a 5862a1e2 snapshot.
   - The external triggers mean little in practice. All 197 issues were authored by `gerwaric`, and `trigger:user` is applied to review findings and process items.
   - No `consumer` issue has closed because its trigger fired.

### 2. Measured facts

**Commits and classes per ISO week.** Command: `git log --since=2026-10-05 --no-merges --format='@@@%h|%ad|%s' --date=short --numstat`, then classify.py. A commit can fall in several classes; tests means `crates/**/tests/**`.

| Week | Commits | code | tests | gov | tooling | audits | userdocs | only gov | only roadmap | only roadmap/reviews |
|---|---|---|---|---|---|---|---|---|---|---|
| W41 (pre) | 103 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| W42 (pre) | 235 | 121 | 102 | 84 | 44 | 50 | 50 | 14 | 0 | 0 |
| W43 | 369 | 180 | 137 | 208 | 43 | 15 | 55 | 122 | 94 | 37 |
| W44 | 872 | 349 | 267 | 545 | 79 | 9 | 70 | 445 | 391 | 146 |
| W45 | 418 | 171 | 156 | 261 | 7 | 14 | 35 | 221 | 197 | 83 |

The pre-reset work lived in `spikes/` until the root swap (e7b813b8, 10-12), so W41 classifies as "other". Daily peaks: 167 commits on 10-27, 168 on 11-04 (`git log --format=%ad --date=short | uniq -c`).

**Added lines per week**, same command:

| Week | src | tests | roadmap | decisions | tooling | audits | total |
|---|---|---|---|---|---|---|---|
| W43 | 26,187 | 28,726 | 12,215 | 298 | 2,028 | 2,725 | 75,535 |
| W44 | 39,007 | 31,491 | 32,474 | 91 | 3,127 | 2,790 | 115,578 |
| W45 | 17,021 | 26,841 | 18,306 | 49 | 445 | 4,940 | 68,322 |

Share of the 261,526 lines added since 10-19:

| Path | Share |
|---|---|
| tests | 33.3% |
| src | 31.4% |
| roadmap (non-review) | 19.9% |
| roadmap/reviews | 4.5% |
| audits | 4.4% |
| tools | 2.2% |
| decisions | 0.2% |

**Commit message bodies** (`git log --format='%x00%h|%ad|%s|%b'`, then msg.py):

| Period | Median | p90 | Max |
|---|---|---|---|
| Pre-reset, all | 1,138 B | 2,542 | 5,545 |
| W43 | 1,140 | 2,737 | 8,146 |
| W44 | 1,054 | 3,715 | 9,519 |
| W45 | 820 | 2,753 | 20,544 |

Total body text since 10-19 is 2.46 MB. The ten longest:

| Bytes | Commit | Subject |
|---|---|---|
| 20,544 | 7aca867d | the mutants measure |
| 11,647 | 39a6d7b7 | describe |
| 9,519 | d2a588bc | MCP texts cut |
| 9,141 | 7c1ce05a | web pages |
| 8,785 | fe331c45 | rails through the door |
| 8,583 | c27557ea | transport fixtures |
| 8,396 | 92bd0816 | restored rows |
| 8,146 | fba1b03e | acq web |
| 8,099 | 665e7203 | lifecycle driver |
| 7,988 | cfb0f3aa | saved search MCP |

What the long ones contain, from reading five:
- **7aca867d** is a no-tree-change commit that carries a pasted cargo-mutants listing: 16 mutants, each with up to 86 failing test names.
- **7c1ce05a** lists dozens of function names moved.
- **39a6d7b7, fe331c45 and d2a588bc** are "Do N" walkthroughs keyed to a brief, with file and line references and the rule each change applies.

My judgment: these are build logs rather than a story a reader would follow.

**Retirement lines.** Commits with `retired …; held by`: 58, holding 339 such lines (`git log -i --grep='retired .*; held by'`; `grep -ic`). They start 2026-10-29, the date of C154 (12a04b24).

Estimate of tests deleted without such a line: I counted `#[test]` / `#[tokio::test]` attributes removed minus added per commit from `git log -p -U0 -- crates`, plus test-file deletions.
- Since C154: 8 commits net-removed 37 tests, and all 8 carry retirement lines. The only deleted file under `tests/` was a golden, in f87e645d.
- Before C154 (10-19..28): 3 commits net-removed 8 tests.

Compliance looks complete.

**AGENTS.md** (`git log --since=… -- AGENTS.md`; `git show "<h>:AGENTS.md" | wc -c`).
- 28 commits touched it since 10-19.
- Size went from 8,374 bytes (6118f449) to 13,774 (d3116d8f, when the registry moved in), 15,046 (a19dbce5), 16,279 (12a04b24), 16,516 (baf625c9), and 16,689 today.
- Budget history, from `git log -G'budget AGENTS.md' -- tools/docs-check.sh` and `git show <h>:tools/docs-check.sh`: 8000 (10-12), 9000 (10-18), 14000 (10-20), 14363 (10-21), 15146 (10-21), 15277 (10-21), 16300 (10-29), 16600 (11-01).
- 5eda514f (10-26) made overage a note, not a failure.
- 5438b84f and d9d882a7 (today) each record that the file is over budget "for the owner to audit later".

Three budget-pressured diffs:
- **a19dbce5** and **12a04b24** removed no lines from AGENTS.md; they raised the budget by the size of the addition plus about 100 bytes.
- **15b0eac2** cut the build-order paragraph (which artifact `cargo test` rewrites) and pointed to the script instead.
- **3668c786** compressed wording to fit `trigger:ci`:
  - Before: "`area:*` (the area file it belongs to) … A feature that waits for someone to ask for it is an issue."
  - After: "`area:*` (its area file) … A feature waiting to be asked for is an issue."

My judgment: the cost to clarity is small but real, since each line is now denser.

**"Corrected" lines.** 19 lines under `roadmap/` match `[Cc]orrected (in place|the same day|20)` (`grep -rE … roadmap | wc -l`), as do 19 lines across 18 commit messages since 10-19. Classes of error in five samples:
- **TEST-SHAPE.md:34:** a wrong measurement reading (runner time taken for wall time). It led to #139 being "closed undone".
- **TEST-SHAPE.md:45:** a false mechanism claim ("inline tests triple a library's compile").
- **TIME-SEAM.md:116:** a premise already false, since the pin already existed.
- **r2-design.md:137:** a wrong design mechanism (the flight stays set) that review caught.
- **time-seam-retrospective.md:349:** a wrong causal hypothesis that a measurement refuted.

None is a typo. Every one is a claim wider than the evidence behind it.

**Rework per step**, from `git log -E --grep=<step>`, counting subjects that match review|fix round|finding, and commits touching crates versus touching only roadmap:

| Step | Commits | Review-ish | Touch crates | Roadmap-only |
|---|---|---|---|---|
| t5 | 124 | 69 | 28 | 80 |
| 4-full | 99 | 48 | 21 | 69 |
| transport seam | 70 | 33 | 19 | 39 |
| r2 | 40 | 13 | 15 | 22 |
| C143 | 35 | 3 | 19 | 13 |

**CI** (`gh run list --branch main --limit 2000 --json …`; 606 runs, history starting 2026-10-22):

| Workflow | W43 | W44 | W45 |
|---|---|---|---|
| Gate (Linux), success / failure / cancelled | 4 / 2 / 1 | 92 / 6 / 15 | 79 / 9 / 72 |
| Gate (Windows), success / failure / cancelled | 4 / 1 / 2 | 71 / 31 / 11 | 109 / 7 / 43 |

Time from a red run to the next green on the same workflow:
- Linux: 13 red episodes, median 0.5 h, max 7.6 h.
- Windows: 16 episodes, median 0.5 h, max 41.5 h, three over 6 h.
- Nothing is red now.

**Issues** (`gh issue list --state all --limit 500 --json …`):
- **Totals:** 197 issues, 119 closed (112 completed, 7 not planned) and 78 open. All authored by `gerwaric`.
- **Opened / closed per week:** W43 68 / 1, W44 78 / 80, W45 32 / 37, W46 19 / 1.
- **Days to close:** median 1.12, p90 9.0.
- **Open by area:** frontends 27, daemon 19, search 18, pricing 9, store 7, network 4, plans 2, trade 2, process 1.
- **Open by trigger:** user 21, seat 14, live 11, step 10, gui 10, consumer 7, ruling 6, ci 2.
- **Trigger hygiene:** no open issue lacks a trigger label. Three carry two (#183, #191, #193).
- **Open by milestone** (`gh api …/milestones?state=all`):

| Milestone | Open | Closed | Due |
|---|---|---|---|
| None | 37 | — | — |
| 4-full, the GUI | 37 | 38 | 12-18 |
| League 3.30 and alpha.2 | 2 | 28 | 11-25 |
| PoE2 launch | 2 | 0 | 12-10 |

- **The three external-event triggers:**
  - **user:** 50 issues, 29 closed. Oldest open is #4 (10-22, 18 days). The closed ones are mostly review findings (#59, #63, #68) and process items (#137, #145, #204), not user reports.
  - **consumer:** 9 issues, 2 closed, neither by the trigger firing. #44's closing comment says "trigger waived" at the owner's word; #82 was built the next day as a step.
  - **gui:** 28 issues, 18 closed. Phase 4 began, so this trigger genuinely fired; #176 closed by 7c1ce05a.
- `ruling` and `step` were created today (5438b84f), which accounts for all 16 of their open issues.

**Ten sampled open issues** (#1, 24, 33, 48, 95, 142, 179, 189, 201, 210; `gh issue view --json body`):
- Body sizes: 188, 289, 446, 763, 1,275, 1,321, 1,591, 1,794, 1,795 and 2,453 bytes.
- Only 3 of the 10 state a done-when. Across all 78 open issues, 18 contain "done when" or "closes when" (`gh issue list --state open --json number,body`).
- The newer issues (#179, #189, #201, #210) cite audits, files and lines, and say what is needed; a worker could start from them.
- The park stubs (#24: "Fire: a 'since' question the returned prior cannot answer." plus a pointer to `CONTEXT.md` at 5862a1e2) cannot be started without a brief.

**PRs** (`gh pr list --state all --limit 100`): 18 in total. 12 are dependabot (8 merged, 3 closed, 1 open). 6 are from `gerwaric`: 5 merged during step 2f (10-22..10-24), and #93 has been open since 10-27. After 10-24 the PR path carries only dependabot; agent work goes straight to main.

### 3. What the numbers say is working

- Code output is high and test-heavy: tests 33.3% and src 31.4% of added lines.
- The retirement rule (C154) is followed: every net test removal since 10-29 carries the line.
- CI regressions are usually fixed within about half an hour, and nothing is red.
- Issues that drive work close fast, and trigger labelling is complete.
- Windows failures fell from 31 in W44 to 7 in W45.
- The corrections fix substantive errors, which suggests review finds real faults.

### 4. What the numbers say costs more than it returns (my judgment)

- **Governance volume.** Half of all commits are governance-only (W44 51%, W45 53%), and `roadmap/` holds 4.15 MB, about three-quarters of the production source's 5.40 MB. Commit bodies add another 2.46 MB. Steps like t5 and 4-full run 2 to 4 roadmap-only commits for every commit that touches crates.
- **The AGENTS.md budget.** Seven raises and a demotion to a note mean it no longer constrains anything. It costs a sentence in many commit messages and an owner audit that keeps getting deferred.
- **The long build logs.** The largest commit bodies are pasted tool output and function inventories (the 20 KB mutants listing), which is more than "the story".
- **The stub backlog and trigger labels.** The 34 bulk stubs (44% of open issues) and `trigger:user` applied without real users make the trigger taxonomy look more informative than it is.
- **Unused milestones.** The 37 open issues with no milestone, and 2 of the 3 milestones holding 2 open issues each, suggest milestones do little planning work.
- **Cancelled CI.** 115 cancelled CI runs in W45 (72 Linux, 43 Windows) mean pushes outpace CI.

### 5. Surprising or contradictory

- AGENTS.md is over its own budget today: 16,689 bytes against 16,600.
- The reset commit 0704a8f7 is dated 10-20, not 10-19.
- `decisions/` received only 440 added lines (0.2%) since the reset, while `roadmap/` received 63,830 (24.4%). The rulings home is small and the process narrative is large.
- #139 was "closed undone" after a correction showed its premise was a misreading.
- 437 review files were added but 388 exist, so about 49 were removed or moved.
- The record suggests the PR path was tried for 2f and then abandoned. I could not find the ruling that ended it in what I read.
