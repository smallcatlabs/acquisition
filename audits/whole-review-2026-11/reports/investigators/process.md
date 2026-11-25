# Investigator's report: process

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/process.md`.

## Track: the process, measured (investigator report)

My scripts are in `<scratch>/process/`: `reviews.py`, `briefs.py`, `sample.py`, `commits.py`, `perline.py`, `reg.py`. I ran no cargo, no gate and no `acq`. Timings are not used.

### 1. The picture

| Noun | Size (V) | What pays for it |
|---|---|---|
| Registry | 200 entries, 133 KB of entry text. It grew from 105 entries and 116 KB on 10-19 to 192 KB today (`git ls-tree -l` per week). C1–C200 are all live and none has ever been withdrawn (`reg.py`; `bothy.py list`). | The ~53 law entries. The rest is unpaid weight (thesis 1). |
| Cold reviews | 460 reviews (198 build, 139 fix, 123 brief), 1.43 MB, since 10-21 (`reviews.py`) | First-round build reviews: 56% find a P1 or P2 |
| Brief reviews | 123 reviews, 659 KB, which is 85% of the build reviews' bytes. Before W46 they graded almost nothing; since W46 they raised 130 "changes what gets built" items in 40 reviews (`briefs.py`). | Edits to the brief's own prose |
| Guards | About 5,080 lines: arch 1,183+504, bothy 835+181, docs 426+235, gate 366+252, hooks and breakers 419, mutants-compare 483, trade-check 195 (`wc -l`) | Check 5 and check 15 hold invariant 1. The rest rarely fire: in 447 local gate runs, arch failed 2 times, docs 7, bothy 1, trade 0 (`runs/gate/verdicts.tsv`). |
| Steps and closes | 79 `Step:` closes | The ledger in git log |
| Owner's queue | 23 `trigger:ruling` issues, all closed, median 0.46 days, p90 11.5 days | His time on rulings |

**Where the commits went since 10-12** (V, `commits.py`; 2,540 commits, each classed by the paths it touches):

| Class | Commits | Lines changed |
|---|---|---|
| Product src (plus anything) | 862 (34%) | src 182k, product data 170k |
| Tests only | 170 | 211k |
| Tools, hooks, CI | 147 | 18k |
| Rulings and skills | 179 | 5.3k |
| Roadmap and briefs | 587 | 74k |
| Saving a review only | 439 | 15k |
| Audits and research | 89 | 508k |

By week, product commits went 277 (W44), 139, 66, 74, 45. Roadmap plus review commits stayed at or above product commits from W45 on.

**Review yield** (V, `reviews.py`):

| Week | Build reviews | P1 / P2 / P3 | With no grade |
|---|---|---|---|
| W44 | 136 | 5 / 121 / 76 | 42 |
| W45 | 60 | 3 / 49 / 38 | 25 |
| W46 | 60 | 14 / 50 / 32 | 16 |
| W47 | 47 | 5 / 31 / 43 | 8 |
| W48 | 29 | 0 / 16 / 23 | 9 |

By round of one step, the share of reviews finding a P1 or P2: round 1, 110 of 196 (56%); round 2, 36 of 103 (35%); round 3 and later, 21 of 38 (55%). Ten P1s were found in round 2 or later. Many of those faults were in the fix itself, for example 103-fixes-371bb1bb (a Windows compile P1 introduced by a fix round).

**What kind of finding** (40 sampled at random with seed 7, `sample.py`, my judgment, so I):

| Kind | Count |
|---|---|
| (a) Behaviour a user or GGG would notice | 19, of which about 7 are reachable with one account |
| (b) Claim wider than the code | 6 |
| (c) Test oracle that cannot fail | 3 |
| (d) Fault in a tool, hook or rig | 8 |
| (e) Other (flakes, platform, process) | 4 |

**Other measures:**
- **Findings per thousand changed lines** (V, `perline.py`): tools/hooks/CI 4.58, daemon src 1.53, cli src 1.39, search src 0.52. That is 3×, not the scouts' 4×.
- **Reviewer family:** 418 of 460 review files name no reviewer in their first lines (V). C177 asks for a mark only when the author's own family reviewed, so per-file provenance cannot be checked.
- **Registry by C170's classes** (I, from reading the 200 headlines; `reg.py` holds the tags):

  | Class | Entries | Bytes |
  |---|---|---|
  | Law | 53 | ~35 KB |
  | Design a session may decide | 60 | 43.5 KB |
  | Mechanism | 54 | ~35 KB |
  | Process | 22 | 13 KB |
  | Restates a check or hook | 11 | 6 KB |

  33 entries say "Pinned: nothing yet": 18 process, 7 law (C79, C110, C120, C155, C189, C191, plus C15, which I count as mechanism). One entry shrank in place: C190 lost a state (`8109cd86`). Since 10-12, 1,227 lines were added to `decisions/` and `AGENTS.md` and 979 deleted.
- **Reading path before a first edit**, following `AGENTS.md` literally (V, `wc -c`):
  - The common part: `AGENTS.md` 20.8 KB, `roadmap/README.md` 4.5 KB and `process.md` 14.8 KB, together 40 KB.
  - Limiter fix: plus network.md, the checklist section and two module docs, **62.7 KB**. Its pack is 6.8 KB.
  - Search query word: plus search.md, trade.md, `search/README.md`, two checklist sections and parse.rs's doc, **88 KB**, or 111 KB with lib.rs's doc. Its pack is 8.6 KB.
  - GUI page: plus frontends.md, its checklist section, page.rs's doc and web/mod.rs's 34 KB doc, **102 KB**. Its pack is 25.4 KB.
  - Each pack is a subset of the area files the agent is also told to read whole, so its overlap is 100%.
- **The owner's attention** (V, `gh issue list --label trigger:ruling --state all`): of 23 rulings:
  - 12 were process or registry wording.
  - 8 were frontend matters, 5 of which C170 assigns to sessions (#245, #266, #268, #271, #295).
  - 2 corrected registry sentences.
  - 0 touched an invariant or the live API.

  Commit messages carry 136 distinct quotes from him; at least 49 open with "yes", "accept" or "approved".

### 2. Theses

**T1. Number only law; let the other ~147 entries live where they bind.**
- **What goes:** 147 of 200 ids, about 98 KB of 133 KB (`reg.py`). Also bothy's shape lint (835+181 lines, about 800 of them) and docs-check's registry clauses.
  - Design entries move into clap or MCP help and goldens, where an agent meets them.
  - Mechanism entries move into the module doc. C192 already says so; nothing moved.
  - Process entries move into the two skills, which already restate C176, C178 and C179.
  - Entries that restate a check move into that check's header.
- **Why the edge cases go:** 2 of 23 rulings and about 19 of the assessment's 71 "wider than the code" findings were registry or document sentences drifting from code. A sentence that sits beside its code drifts less, and a sentence that does not exist cannot drift.
- **Smallest input that changes:** a session adding a query word writes help text and a golden, and no Cn.
- **Invariants:** none of the five lists a registry entry among its pins (`AGENTS.md`, "Invariants"). Every law entry stays numbered. C170's classes become the numbered set itself.
- **Cost and gain:**
  - Security: unchanged.
  - Performance: none.
  - Function: none.
  - An agent's ease: the reading path drops from 63–102 KB to the law entries plus the module doc it is already in.
- **Strongest objection:** about 4,200 citations of non-law ids in crates and tools (`git grep -o -P '(?<![A-Za-z0-9_])[Cc]\d{1,3}(?![\d\w])'`, a rough count that includes test names) must be rewritten or dropped. And the design entries are what kept three frontends' wording in step. My answer is that the goldens (C122) already hold that.

**T2. Review the result, not the diff, for agent-facing work, and keep cold diff review for law code.**
- **What goes:** brief reviews except for law-touching designs (most of the 123 reviews and 659 KB). Fix-round diff reviews on agent-facing steps are replaced by one seat on the finished step. Of the 337 build and fix reviews, findings cite cli, mcp, search, verbs and plan code 247 times against 106 for daemon, store and client source (`perline.py`).
- **Why the edge cases go:** a diff review cannot ask whether the concept should exist.
  - The main account drew 5 reviews and 17 findings, 4 of them P1, before it was backed out (−1,931 lines).
  - C190's closed-for-good state drew 6 reviews before it was removed.
  - In both cases the reviews made the concept heavier.
  - Class (a) findings are what a seat surfaces. Classes (b) and (c) shrink with the prose layer.
- **Smallest input:** a GUI step closes after one cold review plus a seat run, and no fix-round review.
- **Invariants:** a diff touching `acquisition-daemon`, `-transport`, the store's intent API, `auth` or `.githooks` keeps the C175/C176 rounds. C177 stays.
- **Cost and gain:**
  - Security: equal, because law paths are still reviewed.
  - Function: a risk of more P3s shipping.
  - An agent's ease: fewer stops.
  - Two independent builds compared is worth it only where a design choice is open, as with the main account.
- **Strongest objection:** round 2 and later still found 10 P1s, and some were in client or CLI code. My answer is that a seat would catch the ones a user meets. The ones it would miss are the latent cases a one-account user never meets.

**T3. Retire the spelling greps the compiler can hold.**
- **What goes:** arch-check checks 7, 10, 12 and 14, and check 5's HTTP-client sub-rule:
  - checks 10, 12 and 14 become `disallowed-methods`, the shape `clippy.toml` already uses; #196 is open on this;
  - check 7 becomes `selector_tree` as `pub(crate)`, as the semantic-model audit said;
  - check 5's HTTP-client rule becomes cargo-deny `bans.deny` with `wrappers`, which already runs in CI.

  Checks 8 and 9 (test layout) are withdrawn as hygiene, "held by nothing". docs-check checks 4, 5 and 8 go. Together this is roughly half of arch-check's 147 breaker cases and of the 7.3 minutes the arch breakers take in CI (I, from ci-tools-security).
- **Why the edge cases go:** the review findings against these checks were spelling escapes (151-stuck-verdict P2 on `use tokio::time`, transport-seam-1-fixes on a raw `extern crate`). A type cannot be misspelled.
- **Invariant 1** stays held by check 5's member edges, check 15, and spec.rs's c161 tests.
- **Cost:** a warm clippy does not re-lint when only `clippy.toml` changes (#230).
- **Strongest objection:** cargo-deny is not in the local gate, and a push lands on a local green. Check 5's edges must therefore stay as the local holder until deny runs locally.

### 3. What I would defend

The capped state file and the `Step:` close commit (C178, C180) are cheap, they worked, and the scouts agree. Round 2 and 3 cold reviews of law code also earn their place: c135, the rejected bearer coming back after a restart, was found as a P1 in round 2.

### 4. Where the old way is right

- Invariants pinned as tests at the send journal.
- Check 5's dependency edges.
- The owner's queue, answered in a median of 0.46 days.

### 5. Not checked

- Brief-review findings by kind; only build findings were sampled.
- Which arch check fired on 11-12 (`08f485c4`, `2b282f4a`).
- Whether the `tracer-rung.sh` policy write against his store (process-286, P1) reached him as a ruling.
- Tokens actually read by real sessions.

### 6. Why improvements don't land, and where I part from the scouts

**Why improvements don't land (V):**
- The semantic-model audit's bug issues (#205, #208, #209, #211) closed within days. Its improvement issues (#206, #210, #212, #213, #214) and #196 have been open, labelled `trigger:step`, since 11-09.
- `trigger:step` waits for "the next step in its area", and the road's "Next" holds only feature work: GUI, search, release. A daemon or store improvement never gets a step.
- The assessment's cuts 1–5 landed because he named them. Candidates 6–10 have no issue of their own (`gh issue list --search`).
- The governance audit's 11 proposals all landed because each was put to him.

So what lands is what has a failing input or the owner's word. "Smaller" has no done-when a reviewer can check.

**Where I part from the scouts:**
- **history-hindsight:** tools draw 3×, not 4×.
- **census-docs:** it counts 182 entries; the registry has 200, because AGENTS.md holds 18 more.
- **governance-context:** it calls brief reviews the largest ceremony with low yield. By bytes it is close to the build reviews, and since W46 brief reviews do change briefs. That is cost, not zero yield.
- **ci-tools-security:** right on H1. On H2, breakers are only the proof a grep needs: once a check becomes a type, its breaker goes with it.

### 7. Small finds

- C79, C110, C120, C155, C189 and C191 (law) are pinned by nothing.
- Assessment candidates 6–10 live only in #293's body.
- 17 briefs had 3 or more review rounds; three overran C176's cap on 11-11, around #264.
- `trade-check` has failed 0 times in 447 runs.
- `AGENTS.md` asks for both the whole area file and the pack, which is the same text read twice.
