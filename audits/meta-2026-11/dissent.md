# The skeptic on the draft

Codex (`codex-cli 0.171.0`, read-only sandbox), a model outside the author's family (C177), on 2026-11-25, given `README.md` as first drafted, this charge and nothing of the author's reasoning: try to break it; for each of its eight verdicts, its four claims that no one had named, each of the three changes, the owner's act and each of the five points for `NEXT.md`, check the claim against the repository by path and line; give the smallest fact, ruling, input or case that makes it wrong, larger than stated, mis-measured or harmful; say whether each change removes more than it adds and whether its number and its trial can show what the draft says; name what a change would lose that the draft does not admit; then say what the draft as a whole misses and whether a change at this level is worth making before the programme at all; end each item SURVIVES, WEAKENED or DIES. The answer is whole, with one local path prefix removed from two links. It quotes the draft's line numbers, not this README's. What it changed is the README's "After the skeptic".

The draft supports a small trial of fewer sequencing asks. It does **not** establish that its three changes collectively remove more than they add. Several categorical claims contradict its own sources.

Read-only inspection only; no builds, gate, `acq`, or network. Below, `M` means `audits/meta-2026-11/`; `W` means `audits/whole-review-2026-11/`. Citations give repository paths and lines.

1. **Verdict: process weight does not limit what lands.**  
   `M/reports/where-time-goes.md:119–124` admits that active time was never split between ceremony and product work. Commits/hour varies from 2.2 to 5.0; it measures neither completed value nor owner effort. Process can also cause the waits excluded from “active” time.  
   **WEAKENED — fewer active hours explain much of the weekly decline; they do not exonerate process.**

2. **Verdict: the reading path is not the method’s cost.**  
   `M/scripts/tokens/analyse.py:145–147` subtracts cached-token charges while holding every call and output fixed. It cannot measure time spent interpreting documents, conflicting instructions, extra calls, or mistakes induced by them. The 68% figure locates expenditure; it does not identify avoidable expenditure.  
   **WEAKENED — 6% estimates one billing component, not the reading path’s total cost.**

3. **Verdict: the owner would approve most decisions anyway.**  
   `M/reports/owner-interface.md:55` records an approved main-account design subsequently reversed. At `:96`, the reader explicitly says approval measures trust, not review. No reopen event also cannot establish informed acceptance when delegated decisions stood unread.  
   **WEAKENED — observed non-reversal supports delegation cautiously, not equivalence between approval and useful judgment.**

4. **Verdict: one final review cannot replace fix reviews.**  
   The substantive case holds, but the draft strengthens its measurement: `M/scripts/step/nonconv.py:32` accepts locations **within fifteen lines** of a changed hunk, not necessarily inside changed lines. Proximity alone cannot establish that the fix introduced the fault.  
   **WEAKENED — keep fix reviews; correct the location claim and separate newly introduced from newly discovered defects.**

5. **Verdict: the missing instrument is an outside user.**  
   Issue authorship identifies the filer, not the source. `audits/release-2026-10/cpp-issues.md:7–14` already maps outside requests to Rust behavior. The headless request itself contradicts “nobody outside has ever told this project anything.” `M/reports/product-frame.md:103` also admits downloads may include owner VMs.  
   **WEAKENED — missing observed Rust use is credible; absence of outside input or presence of 23 users is not established.**

6. **Verdict: Rust is not worth questioning.**  
   Nine commit-message mentions of ownership trouble are a weak measure of language cost. `M/reports/technology.md:22` acknowledges substantially different iteration latency elsewhere; `M/README.md:383–385` leaves the daemon split, SQLite, and frontend count unexamined.  
   **WEAKENED — retaining Rust is reasonable; this investigation does not settle the architecture’s opportunity cost.**

7. **Verdict: strings are not the pages’ cost.**  
   Golden bytes measure stored representation, not maintenance effort. `M/reports/technology.md:36–41` identifies nearly 300 manual escaping calls and an unaudited convention; `:53` says typed markup would remove it. That is a cost independent of navigation granularity.  
   **WEAKENED — whole-page tests cause churn, but strings carry a separate correctness burden.**

8. **Verdict: the project does not guard absent users excessively.**  
   `M/reports/absent-users.md:93–98` itself estimates removable compatibility work at 1–2k test/golden lines and a sixth of findings. Small production line count cannot refute disproportionate review or decision cost. Conversely, unknown-stamp refusal protects the owner today (`:106`).  
   **WEAKENED — the compatibility purge already happened; the broader work-cost hypothesis remains open.**

9. **Unnamed claim: commits/week was the wrong measure.**  
   `M/scripts/commits.py:12–20` calls any source-touching commit “product,” including comments and internal tests. Folding saved reviews into response commits will improve the proposed ratios without changing work. `W/reports/history-hindsight.md:17` already compared process-only shares before and after the rebuild.  
   **WEAKENED — hours improve normalization, but neither commit measure establishes delivered value or answers issue 220 conclusively.**

10. **Unnamed claim: review only adds.**  
    [BRIEF-REVIEW-PROMPT.md:20](../../roadmap/briefs/BRIEF-REVIEW-PROMPT.md:20) explicitly asks for a simpler shape and reuse. `roadmap/reviews/4-full-1-form-editor-brief-817be04d.md:7` proposes retiring a recorder requirement. `W/reports/design-theses.md:96` had already named the ratchet. “Smallest sufficient fix” does not inherently contradict fixing by one rule.  
    **DIES — subtraction is already requested and sometimes proposed; the narrower problem is whether builders act on it.**

11. **Unnamed claim: player-facing choices belong to nobody.**  
    [process.md:7](../../decisions/process.md:7) says sessions decide **everything else**. Player choices therefore have an owner: the session, except where another reserved class applies. The defect alleged is excessive delegation, not an ownership hole.  
    **DIES — the proposed transfer may be defensible, but its diagnosis misreads the governing sentence.**

12. **Unnamed claim: every step-back ends in more rules.**  
    Git commit `8bd924bb` added process rulings while deleting 1,050 lines and adding 648; the roadmap alone lost substantial repetition. `W/reports/investigators/process.md:69` records a ruling shrinking in place. Counting identifiers misses both consolidation and behavioral retirement.  
    **WEAKENED — rule accumulation is real, but increasing rule count does not demonstrate increasing burden.**

13. **Change 1: one approval per series.**  
    Likely net subtraction for a bounded, approved series. But `M/scripts/owner/shares.py:9–16` counts short “fix,” “cut,” “write,” and even “thanks” messages as go-aheads; 35% is not the removable sequencing share. The trial can reveal repeated asks and wake-up failures, not sustained unattended reliability. Unadmitted loss: convenient checkpoints for changed priorities and early owner correction; the final report does not replace visibility during work.  
    **SURVIVES — authorize the bounded trial, but measure actual sequencing asks and preserve observable progress.**

14. **Change 2: transfer registry wording, reserve everything a person sees.**  
    This probably adds more owner work than it removes. It reserves almost every visible change while retaining session ownership of verbs, flags, and answer wording—an overlap with `process.md:7`. `M/scripts/owner/typed.py:16` detects question-backs by `?`, not misunderstanding. Answering four rewritten questions cannot test the new workload or delegation safety. Unadmitted loss: quick, coherent frontend iteration. Tests cannot certify that a registry translation faithfully expresses an owner’s intention.  
    **DIES — split plain-language questions from the much broader transfer of authority before trialing either.**

15. **Change 3: subtractive review, one brief round, combined commits.**  
    These are three separable changes. Second brief rounds found concrete problems: source selection and count preservation (`roadmap/reviews/4-full-1-form-editor-brief-817be04d.md:3–7`). Combining review commits removes commits, not review work; 1.9→0.75 partly rewards repackaging. A1 and dependent A2 are different tasks, so their comparison cannot attribute outcomes. “The omitted round would have caught it” is unknowable without that round. Unadmitted losses: early correction before implementation and independently committed evidence of unanswered findings. Subtractive reading adds judgment work.  
    **WEAKENED — combining commits looks economical; net benefit from dropping brief review remains unproven, and the bundled trial cannot isolate it.**

16. **Owner’s act: invite the headless requester.**  
    Worth doing, but `audits/release-2026-10/cpp-issues.md:8` already drafted an explicit feedback invitation; `M/reports/product-frame.md:82` says only a link was posted. The actual contact history is not established locally. A reply relayed by Tom leaves “outside-authored issues” at zero despite success. Alpha.1 findings also need separation from current behavior.  
    **SURVIVES — one purposeful invitation is cheap; measure useful feedback and task completion, not issue authorship.**

17. **NEXT: one lane.**  
    `W/next/lane-trial.md:24–35` admits failed test compilation, no executed tests, and untried gate/integration behavior. That supports avoiding another lane now. Historical low concurrency does not prove parallelism has no value; a separate lane could advance A3 while A1 waits.  
    **SURVIVES — one lane removes unvalidated coordination cost; retain parallelism as an option, not an unwanted-throughput category.**

18. **NEXT: seven decisions become four questions.**  
    The prototype remains an additional owner decision. A general series approval does not necessarily authorize the 33 issue writes: `W/NEXT.md:481–482` explicitly requires worker grants. Nor does a frontend-only trial establish that changing the cross-cutting reading instruction is harmless. The renamed-account wording obscures that the old file remains recoverable.  
    **WEAKENED — clearer questions help, but this does not actually settle or eliminate all seven decisions.**

19. **NEXT: A2’s task list goes to the owner first.**  
    `roadmap/briefs/4-full-design-prompt.md:13` says he lacks a solid feel for what users want. He can reject irrelevant tasks, but cannot supply the missing outside signal. Requiring his approval also conflicts with the intended reduction in asks.  
    **WEAKENED — invite lightweight correction; preserve tasks discovered through real use and measure correct answers alongside calls.**

20. **NEXT: backup before B6.**  
    The priority is sensible; “no backup can be made at all” overstates “no product verb exposes one.” `crates/acquisition-store/src/annotations.rs:1384` and `user.rs:191` expose backup APIs. A useful backup needs scope, consistency, and a demonstrated restore; its being smaller than B6 is unmeasured. Backup alone does not preserve normal access after rename.  
    **WEAKENED — provide recoverability promptly, without treating it as a substitute for identity repair.**

21. **NEXT: B3 and B4 proceed.**  
    Moving command-free words is robust. A projector is less universal: `W/next/reports/pages.md:77–84` lists lost classes, hooks, titles, open state, nesting, and order. Its 156 markup-shaped promises require judgment (`:104`), not merely regeneration. The immediate escaping test is independently justified.  
    **WEAKENED — proceed with escaping and narrow moves; validate projection coverage before committing to the tenfold byte target.**

22. **The draft as a whole; act before the programme?**  
    It confuses cheaper bookkeeping with less work, absence of reversal with informed acceptance, and rule count with burden. Its own evidence points toward delayed use and weak visibility, yet it defers visibility and would widen owner approvals. Reverting sentences cannot undo time spent, deployed behavior, or lost data. Make the bounded sequencing trial and immediate protection fixes now; do not make this governance package a prerequisite for learning from A1/A2 and actual use.  
    **WEAKENED — a small operational adjustment is worth making first; another broad method redesign is not yet earned.**