# How agent-built projects are run, late 2026: an outward read

Scope: about 30 web searches and 12 page reads. No query named the project. Confidence marks: **solid** means a primary source with data or a first-hand account at scale; **thin** means one source, vendor-tinted, or secondhand; **anecdote** means one practitioner's word. Many pages the search returned were secondary summaries. I mark those where I could not open the original.

## 1. How work is structured

- **The best-documented account is OpenAI's "harness engineering" (Feb 11, 2026).** About a million lines and about 1,500 merged PRs in five months, with no hand-written code. Codex opens the PRs, reviews them with other agents and often squash-merges its own. Merge gates are minimal and flaky tests are rerun. Recurring background agents open small cleanup and "doc-gardening" PRs. They dropped a weekly human "AI slop" cleanup day (about 20% of each week) and one big AGENTS.md. https://openai.com/index/harness-engineering/ — **solid** (first-hand, one team).
- **Spec-driven development is widespread but contested.** GitHub's Spec Kit is reported near 140k stars (LinkedIn post, secondhand). The critiques: specs go stale, and only "discipline" keeps them in sync (DZone, Jul 2026, https://dzone.com/articles/spec-driven-development). Specs also leave the real bottleneck, review, untouched: Faros reports throughput +33.7% and median review time +441% (cited secondhand). — **thin**.
- **Unattended loops ("Ralph loops")** are on the Thoughtworks Radar (https://www.thoughtworks.com/en-de/radar/techniques/ralph-loop, 2026). Reported wins are greenfield work with a precise "done", and practitioners note non-converging loops. One nightly-loop writer forbids the loop to merge (https://dev.to/pvhee/the-nightly-ralph-loop-14f8). — **anecdote**.
- **Parallel agents in worktrees are the norm in practitioner writing.** Several put the useful ceiling at about 3–5 live sessions before coordination costs more than it buys (dev.to and augmentcode guides, 2026, mostly undated). Steve Yegge's multi-agent orchestrator reached 1.0 on Apr 3, 2026, and users report keeping its issue tracker (Beads) while dropping the orchestrator (https://maggieappleton.com/gastown; https://softwareengineeringdaily.com/2026/02/12/...). — **thin**.
- **Abandoned as overhead** (Pragmatic Engineer essay winners, Nov 24–25, 2026, summarised at https://news.lavx.hu/article/engineers-share-field-reports-on-building-software-with-ai-agents): hand-written PRs (a four-person startup) and routine code review (a solo builder with 12k commits in 10 months). One team made backend review optional and found faster merges but "comprehension dropped". — **anecdote, but recent and many voices**.
- **Trunk versus PR:** I found no comparative evidence. The common position is small, fast PRs, or direct commits with strong automated checks when there is no governance need. — **thin**.

## 2. Review

- **The best data is a single-author preprint** (arXiv 2610.01471, Oct 2026, summarised at https://agentpatterns.ai/verification/cross-model-review-independence/, reviewed 2026-10-05). On code, a **same-model fresh-session reviewer beat every cross-model reviewer on F1** (40.7% vs 37.2%). On prose, cross-model won clearly. One same-model plus one cross-model review together caught 56.7% of planted errors, against 42.7% for two same-model reviews. A third reviewer added 1.3 points. Its recommendation: at most two review calls, then stop, and never a weak second model. About 9% of cross-model false positives came from reviewer bias, such as flagging the generator's features as nonexistent. — **solid-ish** (preprint, Korean artifacts, not peer-reviewed).
- **Non-converging rounds are reported:** 12, then 8, then 5 findings across rounds, run until it "felt done". Commenters say the human must fix the exit criteria before the review starts (https://dev.to/brightgir/the-never-ending-ai-code-review..., May 2026). — **anecdote**.
- **What replaces line review:**
  - *Mechanical enforcement.* OpenAI uses custom linters and structural tests whose error messages tell the agent how to fix the problem. — **solid**.
  - *Property-based testing by agents.* Anthropic's agent found bugs, 56% of its reports were valid, and 32% were worth reporting upstream (https://www.anthropic.com/research/property-based-testing). — **solid**.
  - *Executable checks over prompt rules.* A Nov 23, 2026 release reported that "nearly half of supervisory rules existed only in natural-language prompts", and that a requirement ledger plus host-side acceptance tests caught failures prompt-only supervision missed. Only an aggregator carries it (https://aiagentstore.ai/ai-agent-news/this-week). — **thin**, original not found.
- **"Agent experience" (AX)** is now a named discipline: Microsoft DevAX (Jul and Oct 2026, https://developer.microsoft.com/blog/what-is-agent-experience-ax/), Netlify and O'Reilly. Its method is usability testing with an agent as the user, watching where it gets stuck. — **thin** (vendor-led, no settled method).

## 3. Human in the loop

- **The recurring theme is that the human is "on the loop", not in it.** Work is routed by risk: low-risk changes land alone and the rest go to a person (AWS Builder Center; madewithlove; The New Stack, 2026). Teams tier review by risk and require the author agent to explain its change, its tests and its failure modes (PE essays, Nov 2026). GitHub's Nov 6, 2026 guide keeps exactly one human gate: approving the plan before implementation (https://github.blog/ai-and-ml/github-copilot/beyond-code-generation-a-developers-guide-to-agentic-engineering/). — **thin**.
- **Hashimoto:** check on agents at your own intervals rather than being called by them (Feb 5, 2026, https://mitchellh.com/writing/my-ai-adoption-journey). — **anecdote**.
- **Decision records:** agents treat accepted ADRs as binding and follow obsolete ones rigidly. The author's advice is to state each rule once in its owning record and **explicitly let agents challenge decisions that no longer fit** (O'Reilly Radar, Oct 2, 2026, https://oreilly.com/radar/coding-agents-love-decision-records/). One preprint claims rationale-bearing records cut agent time by 10% (cited secondhand). — **anecdote / thin**.
- **Not found:** any evidence on decide-by-default-with-veto, decision budgets, or batch review of agent-written ADRs as named practices. The web is thin here.

## 4. Context and instruction files

- **Anthropic (Jul 24, 2026):** removing over 80% of Claude Code's system prompt for the current models cost "no measurable loss". Keep CLAUDE.md light and spend it on gotchas. Use a tree of files loaded when needed, not "a central repository for every known practice". Replace prohibitions with goals, and remove rules that conflict. A `/doctor` command right-sizes these files (https://claude.dev/blog/the-new-rules-of-context-engineering-for-claude-5-generation-models/). — **solid** (vendor, first-hand).
- **OpenAI:** an AGENTS.md of about 100 lines as a table of contents, with docs/ as the system of record. — **solid**.
- **ETH Zurich (arXiv 2602.11988, Feb 2026):** developer-written context files gave about +4%, and LLM-written ones slightly hurt. Files caused broader exploration and higher cost, and codebase overviews did not help agents find files. — **solid**.
- **McMillan (arXiv 2605.10039, May 2026)**, 1,650 sessions: file **size, position, architecture and contradictions had no detectable effect** on adherence, and Bayes factors supported null for size. Compliance fell about 5.6% per extra function generated within a session. — **solid** (one study, Sonnet/Opus 4.6).
- **IFScale (NeurIPS 2025):** adherence decays as the number of simultaneous instructions grows. The best model reached 68% at 500 instructions, with a bias toward earlier instructions. — **solid**, but a synthetic task.
- **Net:** the evidence says long rule lists cost tokens and exploration more than they cost adherence. Within-session drift and too many simultaneous constraints matter more. No study tests a 60–100 KB required reading list.

## 5. Model mix and cost

- **Anthropic advisor strategy (Apr 9, 2026):** a Sonnet executor with an Opus advisor scored +2.7 points on SWE-bench Multilingual at 11.9% lower cost. A Haiku executor with an Opus advisor scored 41.2% vs 19.7% on BrowseComp, at 85% less than Sonnet (https://claude.com/blog/the-advisor-strategy). — **solid** (vendor).
- **Routing anecdotes:** Arize routed each phase to a different model and cut a workflow from about $100 to $15–20 per run. A team reported about $2.50 per merged PR, with tiering halving per-request cost. — **anecdote**.
- **Fresh context versus long sessions:** the commonest recommendation is a written handoff into a fresh session. Amp reportedly retired compaction in favour of "handoff". The preprint evidence on compaction is mixed (arXiv 2610.25516). — **thin**.
- **"Cost per merged change" is recommended** by Warp and Sonar as a metric. I found no published longitudinal series of it.

## 6. Measurement

- **Warp (Aug 31, 2026, updated Oct 6) recommends a "running dashboard, not a one-time pilot report"** of five metrics (https://www.warp.dev/articles/metrics-that-prove-ai-coding-agents-are-working):
  - autonomy rate
  - cost per merged PR
  - rollback rate
  - intervention rate
  - cycle time

  It adds wasted-run rate. — **thin** (vendor).
- **A preprint proposes output per reviewer-hour and an escalation rate** (arXiv 2609.04681). — **thin**.
- **LinearB's 2026 benchmark** (8.1M PRs) reports agent PRs wait 5.3× longer for review and are 2.6× larger. — **thin**, secondhand.
- **METR (Feb 24, 2026) could not re-run its productivity RCT.** Developers refused to work without AI, and concurrent agents break per-task timing (https://metr.org/blog/2026-02-24-uplift-update/). — **solid**. Measuring this is genuinely hard.
- I found **no first-hand account of a small or solo project keeping a standing process dashboard.**

## 7. Technology fit

- **Endoh's benchmark** (Claude Code building a mini-Git in 13 languages, 2026): Ruby, Python and JS were fastest and cheapest. Typed languages cost 1.4–2.6× more. Rust varied most and had 2 of the 3 failures in 600 runs (https://dev.to/mame/which-programming-language-is-best-for-claude-code-508a). — **thin** (one task).
- **GitHub and vendors argue typed languages give agents more feedback per edit.** — advocacy.
- **Nolan Lawson's framework bakeoff (DevClass, Nov 4, 2026):** agents did better with server-centric, convention-heavy, concise frameworks than with client-heavy SPAs (https://www.devclass.com/ai-ml/2026/11/04/pokemon-bakeoff-reveals-best-javascript-frameworks-for-ai/5307282). — **thin**.
- **OpenAI wrote its own helper code rather than import some packages**, because in-repo code is easier for agents to reason about. — **anecdote**.
- **Evidence for "mainstream stacks with deep training data win"** is suggestive only.

## 8. Early users

- **The web is thin on agent-built projects specifically.** The general 2026 indie postmortems agree that building is now cheap and distribution is not. Typical reports: "only one of six had real users", and over-engineering before users is a common failure (Indie Hackers; dev.to, 2026). — **anecdote**.
- **The one strong data point is the solo builder in the PE essays** (Nov 2026): he shifted his time from review to product, marketing and support, with 67 releases in 10 months. — **anecdote**.
- **Simulated-user usability agents** (UXAgent, CHI '25; UXCascade, arXiv 2601.15777) are framed as a pilot step, not a replacement for people, with a warning that they produce hallucinated friction. — **solid** that the caveat exists.

## 9. Surprises since autumn 2026

- **Removing instructions is now vendor guidance.** "Rules to judgement" plus a right-sizing command (Anthropic, Jul 2026).
- **Background "gardening" agents on a schedule** open small, fast-reviewable PRs for doc drift and principle violations, in place of periodic human cleanups (OpenAI).
- **Products are built for review by risk tier, and teams report making review optional** (GitHub, PE, Nov 2026).
- **A supervisor agent over a fleet of coding agents** (Cognition, reported Nov 2026, aggregator only) and Meta's reported acquisition of a parallel-agent workspace startup (Nov 4, 2026, secondhand).

## Where the described method looks behind, ahead, and merely different

**Behind**
1. **The 60–100 KB reading list before a first edit.** Both main vendors now prescribe a ~100-line map plus files loaded on demand. ETH found context files mainly raise cost and exploration, and Anthropic's own pruning showed no loss.
2. **Up to three cross-vendor review rounds on code.** The best study says a fresh same-model review is at least as good on code, and a second reviewer is the useful limit. Cross-vendor helps most on prose (briefs, rulings).
3. **Periodic fleet audits of the method instead of standing metrics or scheduled gardening agents.** Others run continuous small cleanup agents and dashboards.
4. **Largest model everywhere.** Executor/advisor routing has vendor data showing cost cut with quality held.
5. **Process records outnumber product commits, with users absent.** The 2026 postmortems name pre-user polish as the common failure.

**Ahead**
1. **Architecture enforced by checks** (dependency-edge checks, docs checks). This matches OpenAI's most-cited practice.
2. **"Seats"**, blind agents at the product. This is AX testing before most teams have a method for it.
3. **Rulings pinned to tests that cite them** (executable over prose). This is what the Nov 2026 supervisory-rules finding recommends.
4. **Briefs reviewed by a second vendor.** Prose is where cross-model review measurably wins.

**Merely different**
1. **Trunk commits without PRs.** No evidence favours either for a solo owner with a strong local gate.
2. **One checkout and one build machine.** Worktree parallelism has a 3–5-session ceiling, and the gain is unproven for a single reviewer.
3. **A numbered ruling registry versus ADRs.** Same idea. The only advice found: one rule stated once, and let agents challenge stale ones.
4. **Rust.** Costs more per agent task in the one benchmark, and catches more at compile time. A trade, not a mistake.

## Three things I would try if I ran it

1. **Cut the always-loaded file to a ~100-line map and measure.**
   - *How:* run the same three past steps with the current reading list and with the map, and compare tokens, wall time and gate failures.
   - *Evidence:* OpenAI's map design (Feb 2026); Anthropic removing over 80% of its prompt with no loss (Jul 2026); ETH's cost finding; McMillan's null result on size, which means cutting is cheap to try and the risk of adherence loss looks low.
2. **Change the review rule.**
   - *How:* code gets one fresh-session same-model review plus at most one cross-vendor pass. The exit criterion is written before the review runs. Cross-vendor review is kept as the default for briefs and rulings.
   - *Evidence:* arXiv 2610.01471 (Oct 2026); the non-convergence reports (May 2026).
3. **Replace the periodic step-back audits with a standing five-number ledger, updated by the close commit.**
   - *The numbers:* owner minutes per step; review findings accepted versus rejected; rework within N steps; tokens per step; outside users or sessions.
   - *Also:* a scheduled small-model "gardening" run that opens one doc-drift fix at a time.
   - *Evidence:* Warp's running-dashboard recommendation (Oct 2026); OpenAI's background gardening agents; METR's finding that one-off measurement breaks under concurrent agents.

**Where the web was thin:** human-decision patterns (veto-by-default, decision budgets), any solo-project process dashboard, trunk-versus-PR outcomes, and early-user routes for agent-built tools. I did not fill these from belief.
