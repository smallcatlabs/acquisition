## How I picked
I took the steps from `git log --grep='^Step: ' --since=2026-11-10`, then checked `ls roadmap/reviews | grep …` to keep only steps with a saved build review. I picked one step per required area:
- **network-288** (daemon/network): its issue is the brief, with three build reviews.
- **store-277** (store/search): has a brief, a brief review and a build review.
- **search-307** (search): issue only, one review.
- **frontends-304** (`acq web`, but it carries the frontend rules): issue only, one review.
- **4-full-1-form-rows** (web GUI): a brief, two brief reviews and two build reviews.
- **gate-321** (tools): an issue and two reviews.

Rejected: next-a3, because network-288 had more findings. The 6.5x steps and #293 cuts, because their close subjects are "roadmap:" and they have few reviews. The seat steps, which are not builds. frontends-304 is web code, so the CLI/MCP slot is only partly covered. I used store-277 for "store or plan"; no plans step was sampled.

## How each number was made
- **Changed files:** `git show --stat` on each build and fix commit.
- **Delivered bytes:** at each step's first commit (`git show <c>^:<path> | wc -c`) for `AGENTS.md`, `roadmap/README.md`, `decisions/process.md`, each area file, and the `REVIEW-CHECKLIST.md` sections (awk over `## ` headings). Plus `cold-review` and `session-close` (8,530 B), the brief, the issue body (`gh issue view --json body | wc -c`), and the design doc where the brief names it.
- **Entries delivered whole:** `grep -cE '^- \*\*C[0-9]+'` on the delivered area files at the step's commit, plus `AGENTS.md` (18) and `process.md`.
- **Engaged entries:** C-ids grepped from commit messages, added diff lines, the brief, the reviews and the issue (script `ids.sh`). Ids marked † are my judgment. I dropped C169 in gate-321: it appears only as a context line in `mutants.toml`.
- **pack:** `pack --path` run at HEAD over every changed `.rs/.js/.css/.sh/.toml` file (`pack.py`, `packall.zsh`).

All scripts are in <scratch>/readers/precision/

## One row per step

| step | area | files changed | delivered bytes | entries delivered whole | entries engaged | rules breached (in delivered reading?) | needed, not delivered |
|---|---|---|---|---|---|---|---|
| network-288 | network + daemon (also touched cli, verbs, protocol, store) | 46 (4 commits) | ~151 KB (daemon, network, frontends and store files, plus 4 checklist sections at 18 KB) | 134 | 18: C1 C10 C19 C22 C24 C25 C26 C32 C34 C46 C53 C87 C116 C129 C153 C176 C189 C190 | sibling sentence ×3, claim wider than evidence, nearest failure (journal vs log), pin retired below what it held: all are Frontends checklist rows (y). Token-route loop: C24/C129 (y) | The send journal versus the daemon log (`TESTING-NOTES.md` §"The journal is the contract surface"; `AGENTS.md` only points at it). Parent and deep-stash tree behaviour (code in `verbs/jobs.rs` and `queue.rs`; C23 states only the property). `quote.rs`'s branch for a route with no learned policy. Ground truth N27/N60. The plan agreed with the orchestrator, never committed. |
| store-277 | store + search | 16 | ~118 KB | 94 | 10: C53 C54 C96 C108 C122 C146 C153 C163 C174 C188 (C188 was written by the step) | Build review clean. Brief review: C54 revive, C96 all-realms, C174 (y, all delivered but missed by the brief's author). The C163 walk (n: the test's `populated()` in `verbs/tests/documents.rs`) | The two render sites (`search_cmd.rs`, `web/page.rs`). The test doc walk. `TEMPLATE.md`'s CI commands. |
| search-307 | search | 1 (`bind.rs`) | ~70 KB | 59 | 2† (C47, C175) | none | The regex compile cost and its size limit (outside knowledge). The measurement command, which the author invented. |
| frontends-304 | web/frontends | 6 | ~75 KB | 57 | 5 (C175; †C11 C143 C154 C122 from the retirement lines) | claim wider than evidence (y, Frontends checklist row) | The earlier review rounds that brought in the boxes (form-editor fixes 310862a4, a6acf78c: other reviews' files). |
| form-rows | web GUI | 21 | ~105 KB (with 23 KB design doc) | 57 | 9: C1 C46 C137 C138 C154 C172 C187 C194 †C16 | nearest failure and retirement (y). A hidden number box blocking submit through the browser's own validation (n: no rule anywhere) | `trade/translate.rs::weighted` limits (other area's code; found by the brief review). The test harness's Fetch interception (`tests/browser.rs`). #184's comments (never committed). C137 (daemon) and C194 (store) are not in frontends' area file. |
| gate-321 | tools/gate | 4 | ~51 KB | 41 | 4: C200 (written by the step) †C175 †C176 †C154 | claim wider than evidence ×2 and an escape read wrong (n: tools has no checklist section; the shapes reach only the reviewer, through `REVIEW-PROMPT.md`) | `cargo-mutants` `exclude_re` and regex escape meaning (outside knowledge). The breaker-suite convention (`tools/arch-check-breakers.sh`). Telling comments from code at a position. |

## Totals
- **Precision by entries:** 48 / 442 = **10.9%**. Per step: 13%, 11%, 3%, 9%, 16%, 10%.
- **Precision by bytes:** engaged entries sum to 37.5 KB (`grep` of each bullet at HEAD, `wc -c`) out of about 570 KB delivered = **~6.6%**. This is a lower bound: checklist rows, skills and briefs are not counted as engaged bytes, though the two skills and the briefs were engaged in full.
- **Review findings:** 13 across the build reviews; 10 carry a review-prompt shape number. 8 of those 10 were in delivered reading, but only because the Frontends checklist section repeats shapes 1–3. gate-321's two were not delivered. All 3 untagged findings were plain code knowledge.
- **Most often needed and not delivered** (judgment): facts living in code and tests outside the files being changed. That covers another module's limits, a test harness's mechanics and a second render site. Next came outside tool semantics (regex, `cargo-mutants`, browser form validation). The registry was almost never the gap. Every engaged entry except C137 and C194 in form-rows was in the area-file reading, and `pack --path` covers those two.

## Over-delivered and what did the work
Three most over-delivered (bytes against what was engaged):
1. **`decisions/daemon.md`** (30.7 KB) for network-288: 1 of 38 entries engaged (C10).
2. **`REVIEW-CHECKLIST.md` sections**: network-288 got four sections (18 KB, 48 rows), and about 3 rows did work. For a builder they mostly repeat `REVIEW-PROMPT.md`'s shapes.
3. **`decisions/frontends.md`** (19–20 KB), delivered to four steps. It engaged 2–3 entries in each; store-277 engaged C53, C146 and C163, and network-288 engaged C53 and C116. `store.md` for network-288 engaged 0 of 19.

Three kinds of knowledge that did the most work:
1. **The brief's or issue's done-when, and the entries it names.** The form-rows brief named exactly C46, C187, C138 and C154. Every review judged against it. It lives in `roadmap/briefs/` and the issues.
2. **The fault shapes.** `roadmap/briefs/REVIEW-PROMPT.md`, copied in part as Frontends checklist rows. They named 10 of 13 findings.
3. **Code-level facts in neighbouring modules and tests.** `translate.rs`, `documents.rs`, `quote.rs`, the breaker suite. Reviewers found these; no instruction sends a builder to them.

## Would `pack --path` (at HEAD) have hidden an engaged entry behind a headline?
- **Packing every changed file, tests included:** no engaged entry is hidden. The one failure is gate-321: `pack --path tools/arch-check.sh` answers "No area governs tools/arch-check.sh", so it delivers nothing.
- **Packing only the main product files** (the "module you will change"):
  - **network-288:** C22, C26 and C32 drop to headlines. Packing rails, choke, server, quote, dispatch, jobs, jobs_cmd and status leaves them as headlines; only the test files cite them.
  - **form-rows:** C187 (the browser-check rule, the step's central constraint) and C137 drop to headlines. `page.rs`, `form.rs`, `fill.rs`, `mod.rs` and `lists.js` don't cite them; `tests/browser.rs` does.
- Cross-cutting and process entries are left out by design: pack assumes `AGENTS.md` is loaded. That is correct for the cross-cutting ones. The process ones come only through `roadmap/README.md`.
- Pack's size grows fast: 548 KB summed over network-288's 34 files, 117 KB over its 8 main files.

## What this sample cannot show
- **What builders actually read.** I read no session transcripts, so "delivered" means what the instructions would send, not what was opened.
- **Whether reading prevented faults.** There is no counterfactual, and a clean review may come from reading that prevented a fault and left no trace.
- **Incomplete engagement.** Rules followed without an id cited are missed, except where I marked judgment (†). N-claims and module-doc reading were not measured. I read issue comments only for #288's C-ids.
- **Pack at the time.** Pack was run at HEAD, not at each step; the entries and file citations have changed since.
- **Bias in the breach count.** Reviewers are given the shapes, which tilts breaches toward shapes.
- **Small, recent sample.** Six steps over two weeks, with no plans or MCP-only step. The brief writer's reading is mixed with the builder's: store-277's faults were all at the brief.
- **Not checked:** module-doc byte counts, `mock-session` use, and CI.
