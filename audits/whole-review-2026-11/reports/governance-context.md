## 1. Model

Governance is a large prose layer that agents load before they act. `AGENTS.md` (20.8 KB, loaded every session) points to 11 area files of rulings (`decisions/*.md`, 200 entries, C1–C200). `tools/bothy.py` lints their shape and `pack --path` prints the rulings that bear on one file. Process is a fixed cycle (C173): issue or brief (capped at 8,000 B), build, gate, a cold review by Codex saved whole to `roadmap/reviews/`, then a close commit carrying a `Step:` trailer. A road file under a byte cap holds current state. Seven skills hold the procedures.

The rebuild of 2026-11-09 (C170–C185) replaced byte caps with shape checks and turned some rules into hooks and checks. Where it applied a **hard cap with a check**, it worked: the road file stays at 4.4 KB and the ledger is retired. Where it relied on **"retire when proven"**, nothing happened: no entry has ever been retired, and the shape baseline has not shrunk by one row. The new pattern is that each new check also gets a new ruling id. The cost to an agent has moved from the registry into **module docs** (`# As built` sections) and the **pack output**, which duplicates the area file the agent is also told to read in full. The owner's ruling queue is empty and the tracker agrees with the road.

## 2. Measurements

| Measure | Value | Command / source |
|---|---|---|
| Commits since 11-10 | 586; 342 (58%) touch only governance docs; 290 (49%) touch only `roadmap/` (160 save reviews, 69 briefs, 51 road edits); 206 (35%) touch `crates/`. The audit's window: 49% / 43% / 42% | `git log --since=2026-11-10 --no-merges --format='@%h' --name-only`, classed by path |
| Line churn since 11-10 | governance +9.1K/−2.7K lines; crates +97.7K/−67.4K | `git log --numstat` |
| Registry | 200 entries, 0 ever retired (C1–C200 all live). 37 over 800 B (audit: 25 of 169). `shape-baseline.tsv` still 47 rows, one commit (11-09), never shrunk | `bothy.py list`; python over the `^- **C` lines; `git log -- decisions/shape-baseline.tsv` |
| "Pinned: nothing yet" | 33/200, of which 16 of 24 in `process.md` and 7 of 18 in `AGENTS.md`; 6 marked "Unbuilt" | `grep 'Pinned:[^.]*nothing yet'` |
| `roadmap/` vs production source | 4.11 MB → 4.69 MB (+578 KB in 14 days) vs non-test `.rs` 4.40 → 4.90 MB (+498 KB) | `git ls-tree -r -l <rev>` at 11-10 and HEAD |
| Road file | 4,376 B (5,818 at 11-23) | `wc -c roadmap/README.md` |
| `AGENTS.md` | 19,124 → 20,802 B | `git cat-file -s` |
| Reviews since 11-10 | 162 review files, 29 briefs, 64 closes (79 ever) | `git log --diff-filter=A`; `git log --grep='^Step: '` |
| Review yield since 11-06 | 94 steps, 144 build reviews, 60 brief reviews (321 items); P1 20, P2 104, P3 94 | `python3 -I audits/assessment-6-5/findings.py` (read-only, git only) |
| P1+P2 per build review by ISO week | w44 0.96, w45 0.83, w46 1.07, w47 0.77, w48 0.55 (2 days). Share with no finding in w48: 41% | my script over the same regex, in scratchpad `wk.py` |
| Brief sizes since 11-10 | 16 of 29 fall between 7,896 and 7,999 B, just under the 8,000 cap | `wc -c` on the added briefs |
| Doc comments in `crates/` | `//!` 582 KB, `///` 1.74 MB. `web/mod.rs` module doc 9.7 KB → 32.6 KB in 14 days; `search/lib.rs` 23 KB | `grep '^\s*//!' \| wc -c` |
| Issues | 86 open, all labelled; median age 14 d, none over 32 d; 0 open `trigger:ruling`; 1 "Taken under delegation" ever | `gh issue list --state all` |

## 3. Where a deep review would pay (ranked)

**1. Every capped measure is being written to the cap.** *(Verified. High confidence.)*
- **Evidence:** 16 of 29 briefs land at 7,9xx B against the 8,000 B cap (C174).
  - Entries meet the six-clause limit by chaining semicolons. C148 is about 1 KB of mechanism (a credentials lock, a blocking thread, "the fifth landing") that passes `check`.
  - The baseline "only shrinks", but no row has been removed, and C192's "shed when next amended" has produced nothing yet.
- **Question:** are the caps trimming words while leaving the substance, or the facts, intact? Would a reading-cost measure work better, such as bytes an agent is told to read per task?
- **Read:** `tools/bothy.py` docstring rules 6 and 8; `decisions/process.md` C174 and C192; recent briefs.
- **Payoff:** fewer concepts, honest measures.

**2. The reading path is 70–160 KB before a first edit, and roughly 15–25% of it is relevant** *(judgment)*.
- **Totals**, following AGENTS.md literally (area file, checklist section, module doc, pack, process.md, road, brief):
  - Limiter fix: about 70 KB (17.5K tokens). The module doc also names the 74 KB `network-ground-truth.md` as the "source of truth".
  - New search query word: about 118 KB, including `search/README.md` (13.5 KB) and `lib.rs`'s 23 KB module doc.
  - GUI page change: about 159 KB, including `4-full-1-design.md` (23.8 KB), the `web/mod.rs` module doc (34 KB) and pack (25.4 KB). Not counted: `GUI-NOTES.md` (40 KB) and the seat skill (12.6 KB).
- **Where it fails:**
  - pack already carries the cited entries in full plus the area's headlines, yet the agent is also told to read the whole area file. That is duplication by instruction.
  - `process.md` (14.8 KB) is required even for a non-step fix.
  - `# As built` module docs restate rulings: `web/mod.rs` quotes C138 "(verbatim)", and there are 12 `# As built` sections.
- **Where it works:** pack itself, per-area files, skills loaded on demand, and the capped road.
- **What would halve it** *(judgment)*: make pack the single entry point, read process.md only at step start and close, and keep module docs to property plus pointer.
- **Read:** `AGENTS.md:105-128`, the pack output for `crates/acquisition-cli/src/web/page.rs`, and `web/mod.rs:1-480`.

**3. The registry accretes: each check also gets a ruling id.** *(Verified 0 retired. Classification is judgment.)*
- C182 says a rule is added only where structure cannot hold it. Of the 15 newest (C186–C200):
  - **Restate a check or hook that already exists (7):** C186, C187, C196, C197, C198, C199, C200. The structure was built, and a rule was added too.
  - **Process lore (2):** C189, C192.
  - **Product properties (5):** C188, C190, C193, C194, C195.
  - **Scope (1):** C191.
- **What half-size would look like:**
  - Fold the gate-stage rulings into one entry: "the gate's stages are the owner's; each stage's script states its property".
  - Move process lore that has a skill into that skill, since the skills already restate C176, C178, C179 and C154.
  - Delete "Pinned: nothing yet" process entries that only describe the procedure.
  - Make mechanism-heavy entries (C53 at 2,474 B, C16, C156, C157, C148) doc comments.
- **Payoff:** about 40–60 entries and about 40 KB *(estimate)*.

**4. Cold-review yield is falling and shifting to P3; brief reviews are the largest ceremony.**
- **Evidence:**
  - Brief reviews: 60 reviews, 321 items, against 124 P1/P2 from build reviews.
  - Five `cut-293-5-b*` reviews each found only P3s.
  - Retirement lines at closes: about 8 of 60 closes list them; most say "none" or put them "in prose".
- **Question:** limit brief review to design briefs, and skip review for mechanical rounds (cuts, regenerations)?
- **Payoff:** about 30% of review commits. Confidence: medium.

**5. Governance commit share rose after the rebuild (58% vs 49%).** In bytes, `roadmap/` still outgrows source. This answers #220 early, but by lines changed, code dominates about 10:1, so the commit count overstates the cost.

## 4. Low yield / already covered

- **The road file:** capped and current; C180 worked.
- **The tracker:** fully labelled, young (none over 32 days), agrees with the road item by item.
- **`roadmap/reviews/` (1.9 MB):** on no reading path.
- **`REVIEW-CHECKLIST.md` (26.8 KB):** stable since 11-10.
- **`HISTORY.md` (4.9 KB):** fine.
- **`RUN-LEDGER.md`:** C189 cut the rows (16 in November); it is 69 KB but read by its tail.

## 5. Blind spots

- **Reviewer independence:** no saved review since 11-10 declares the author's family. Codex is named in 83 of 161 review-saving commits; the other 78 name no reviewer, so C177 cannot be verified per review.
- **Token totals:** I did not test what a real agent actually reads; my totals follow the written instructions.
- **Production-source definition:** the audit's (5.44 MB) is not stated, so my source figure is non-test `.rs` and not directly comparable.
- **Prose density:** I did not measure whether dense rulings cause wrong edits; no data links review findings to misreadings of rulings.

## 6. Surprises

- **"No UI" vs `acq web`:** the README opens with an author's note saying "there's no UI". The same file's tour (`README.md:207`) documents `acq web`, a full search, saved-search and places page set.
- **No human path:** `CONTRIBUTING.md` sends people to the agent-dense `AGENTS.md`; the only shorter way in is the 52-term `GLOSSARY.md`.
- **Prose (Q3), sampled 20 entries and 3 skills** *(judgment)*:
  - The headlines compress well (C59, C86, C82 read cleanly).
  - Bodies decay into semicolon chains of mechanism (C148, C163, C92).
  - C163's headline, "Every time a document carries has its words beside it", is ambiguous: "time" means timestamp and "words" means an age written out.
  - Glosses like "(C98, the basis)" help, but an entry like C163 still needs C98, C38 and C53 to parse.
  - The skills are procedural and readable, but they restate process.md (the C176 counts appear in both).
- **One ruling ever taken under delegation:** C179 expected more, so either sessions defer more than intended or they decide without filing.
- **`bothy check`'s only note:** no module doc names `decisions/process.md` or `decisions/ci.md`.

**Top three claims re-checked myself:** brief sizes (`wc -c`), zero retirements plus the unchanged baseline (`bothy list` and `git log`), and the growth of the `web/mod.rs` module doc and its verbatim C138 copy (`git show` at both dates).
