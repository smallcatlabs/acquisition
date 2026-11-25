## Hindsight slice: what the history says

### 1. Model
The Rust history starts in `spikes/rust-playground/` and moved to the root at `e7b813b8` (2026-10-12). Any per-file count has to strip that prefix, or it misses everything before mid-October. Many search files show a deleted/added ratio near 0.50, but that comes from the move, not from rewriting. Review records exist from 2026-10-21 only. Phase 0's close deleted the earlier ones (71 deleted review paths), so August and September can be read only from commit messages. The work runs in steps. A step is a brief, then a build, then one cold review per round (`roadmap/reviews/<step>[-fixes]-<sha>.md`), then a close commit with a `Step:` trailer. Two formats coexist. Older reviews number their findings and do not grade them. Newer ones grade P1/P2/P3 and cite the numbered shapes in `roadmap/briefs/REVIEW-PROMPT.md`. Removal is episodic: a cold assessment counts findings, the owner rules, and a batch of cuts lands (#293, 11-20). After that the tree grows again. Most of what the owner says on record is a one-line approval. His few substantive words are always about simplicity.

### 2. Measurements
My scripts are in the scratchpad: `churn.py`, `churn2.py`, `rev.py`, `rev2.py`, over `git log --numstat --no-renames --since=2026-07-01`.

| Measure | Value | Command |
|---|---|---|
| Top churn × size (excluding reference data and goldens) | cli/tests/web.rs (120 commits, +9.6k/−1.2k, 8.3k now); daemon/src/daemon.rs (136, +21.9k/−20.1k, 1.8k); store/src/lib.rs (116, +11.5k/−8.3k, 3.2k); search/src/answer.rs (109); cli/src/web/page.rs (84, 5.6k); plan/src/lib.rs (85, ratio 0.77); cli/src/search_cmd.rs (142); cli/src/web/mod.rs (93); verbs/tests/documents.rs; cli/tests/search_json.rs; ratelimit/tests/spec.rs; client/src/client.rs | `churn.py` |
| Rewritten in place (only edits of 500 lines or fewer, so splits and moves are excluded) | plan/src/currency.rs 0.74; tools/docs-check.sh 0.70 (91 commits, now 426 lines); mcp/tests/plan_loop.rs 0.67; cli/tests/world.rs 0.67; store/src/index.rs 0.60; the mcp and daemon test harnesses 0.60; client/tests/contract.rs 0.58 | `churn2.py` |
| Mean build reviews per step | 2.03 before 11-10 (101 steps), 1.59 after (83 steps) | `rev2.py` |
| Steps with ≥3 build reviews | 15/101 (15%) before, 14/83 (17%) after. The tail has not fallen. | `rev2.py` |
| Commits naming a second, third or later review round, per week | W44 113, W45 66, W46 21, W47 20, W48 11 | grep over subjects |
| Reviews / findings per kLOC changed since 10-21 | tools 3.1 / 3.7; search 1.15 / 1.9; mcp 0.94 / 1.5; cli 0.8 / 1.3; verbs 0.78 / 1.5; daemon 0.61 / 0.95; store 0.43 / 0.5; plan 0.12 / 0.09 | `rev2.py` + numstat |
| Commits touching only process, roadmap or reviews | Sep 42%, Oct 41%, Nov 1–9 57%, Nov 10–24 55% | commit classifier |
| Commits touching product source, per day | Oct ~17, Nov 1–9 ~19, Nov 10–24 ~12 | same |
| P1/P2 findings naming account, realm or league | Oct 23/113, Nov 1–9 23/70, Nov 10–24 32/113 | regex over review files |
| Crates since the #293 cuts (41684e4c → HEAD) | source 115,720 → 119,553; tests 128,561 → 133,261 | `git show` + `wc` |

**Built, then removed** (`git show --shortstat -- crates`):

| Feature | Alive | Built (lines) | Removed (lines) | Review cost |
|---|---|---|---|---|
| Main account (#246) | under 1 day (11-11) | +1,942 | −1,931 (`fd7fc5fb`) | 5 reviews, 4 P1 |
| C190 closed-for-good (#288 → #291) | same day (11-18) | +2,018 | −919 (`9ca22041`) | 6 reviews |
| Quote estimate over a closed route | 2 days | — | −187 | in 4 of those 6 reviews |
| The query's own order (#38) | 9 days | — | −548 / +296 | 14 findings |
| `acq pull` | 5 days (08-24 → 08-29) | — | — | its `store import` replay lived until 11-20 |
| Terminal dashboard | 92 days | — | −808 | — |
| Migration ladder below version 8 | — | — | −799 | — |
| Ad-hoc `refresh` kind | — | — | −245 (10-25) | — |
| MCP structured copy | — | — | (10-28) | — |

### 3. Where a deep review would pay (ranked)

**1. Scope (account × realm × league) as one value, resolved once, with no defaults.**
- **Evidence:** About a quarter of P1/P2 findings in every period name account, realm or league, and the share is not falling after the main account was backed out. The account default went, but `Realm::DEFAULT` is applied at 12 sites, including the daemon (`daemon/kinds.rs`). `DEFAULT_LEAGUE` (`plan/src/pricing.rs:297`) is used at 22 sites across cli, mcp and web. There are three realm types: `protocol/src/realm.rs:75`, `search/src/corpus.rs:125` (`All | One(String)`) and `store/src/corpus.rs:144` (`RealmScope`). Partly the C1 edges force this, because the store links nothing.
- **Verified:** the greps and the three types.
- **Question:** Is "Standard / pc unless named" the main account again, at smaller scale?
- **Payoff:** It retires the largest persistent finding class. **Confidence:** medium-high.

**2. Remedies as structured data, rendered by one function per surface.**
- **Evidence:** Shapes 4 ("say what to do") and 6 ("the printed command must run") feed each other. A printed command that drops context is still a finding after its checklist row: 2g-r2 10-27, seat-issues-b 11-04, 97-own-terms 11-05, the pre-push hook 11-10, describe-slices' quoting 11-01. `cli/src/printed.rs` exists, but remedies are still composed in many places.
- **Payoff:** It removes a review class and helps agents, who want a verb plus arguments, not shell text. **Confidence:** medium.

**3. The unreadable state above the store (assessment candidate 6).**
- **Evidence:** Its 24 findings were the largest pattern in the assessment, and 402 lines in crates still name it. Candidates 6–10 got no comment on #293 and no issue; only cuts 1–5 were built. This is an audit finding nothing acted on, as are priority (149 source lines), the twins and persist-check (#287 open).
- **Payoff:** Concepts removed across store, plan, verbs and frontends. **Confidence:** medium.

**4. The governance tooling is the least reliable code.**
- **Evidence:** tools, hooks and workflows draw 3.7 findings per kLOC changed, about 4× the daemon and 40× the plan crate. docs-check.sh was rewritten in place across 91 commits. tracer-rung.sh has ratio 0.81. `.githooks/pre-push` drew 7 findings, one of them a P1.
- **Question:** Which checks justify what they cost in reviews? Can the spelling-sensitive greps become typed checks, or be deleted? (The retrospective §6 raised this; nothing was built.)
- **Confidence:** high on the measurement; the remedy is judgment.

**5. The web layer is where the size goes.**
- **Evidence:** `cli/src/web` is 12.3k lines of Rust, including `page.rs` at 5,630. It has grown +9.4k/−1.9k since 11-20. web/mod.rs leads findings by file. The #293 cuts (~3k lines) were outgrown within 4 days.
- **Question:** Should HTML rendering live in the CLI crate as format strings, or in templates or a crate of its own?
- **Payoff:** Concentrated churn, which is where any reduction would pay most. **Confidence:** medium.

**Does the wider history agree with the assessment's cause ("a default or a state that saved a user one decision")?** Yes, with one addition. `acq pull`, the structured copy, the inferred order and the closed-for-good state were each a convenience that every reader then had to handle as a case. What is still in the tree with that shape: the realm and league defaults, priority, the unreadable states above the store, and the default limits (#314 found 20 against 10).

### 4. Low yield / already covered
- History in comments: done in cut 5 (43,413 → 42,642 comment lines).
- The terminal dashboard and the migrations: cut.
- The language choice: the retrospective already covered it.
- The plan crate: lowest defect density.
- Saved review prompts: the assessment already declined these.

### 5. Blind spots
- August and September findings cannot be classified from files, because the records were deleted at phase 0.
- My area attribution uses only the reviewed head commit, so ranges are undercounted. The keyword classification is a regex, which is my judgment.
- No one has measured the owner's time: about 146 quoted owner sentences, most of them approvals.
- #220 (re-measure the governance rebuild) falls due after 11-30.
- Where the hand-sampled findings point: of 30 reviews sampled (10-21 → 11-23), the persistent causes are:
  - a rule enforced at one door while another door exists (parser versus tree #273, `RefusedParams`, redaction spellings, the arch-check alias, the account read twice);
  - claims wider than the code;
  - tests whose oracle cannot fail.

  The first is not a checklist row as such; it is spread across "one spelling", "one fact read twice" and "a meaning read off the syntax".

### 6. Surprises
- 872 commits in one week (W44).
- After the governance rebuild, the share of steps needing 3 or more rounds rose slightly, even though the mean fell.
- Since 11-20 the crates gained +18.0k and lost −5.8k lines.

### What the owner keeps asking for
- "is there a simpler elegant way"
- "spiraling into some process complexity"
- "We should hold ourselves accountable to the goal of a simple, better acquisition more than to our past decision"
- "not patching hypothetical edge cases and instead looking for underlying/simpler patterns"
- build, observe, iterate
- the C++/trade-site layout

A valuable finding, to him, removes a concept and says why the edge cases go away.

### Five step-backs
1. Scope as one value, with no defaults.
2. Structured remedies.
3. The unreadable state collapsed to the store's one refusal.
4. A cost-per-check audit of the governance tooling.
5. A shape for the web renderer.

### The unwritten lesson (judgment)
Findings have only ever been counted per step and per file, never per line changed. Per line, the machinery built to keep agents safe (hooks, gate stages, driver scripts, docs-check) is the most defective and most rewritten code in the tree. Each new check buys a standing review tax. Meanwhile removal arrives in batches, and the GUI work had outgrown the batch within days. The project has a rule for retiring tests (C154) but no size or check budget.
