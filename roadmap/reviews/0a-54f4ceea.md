# Review of 0a at 54f4ceea

Codex (gpt-6.1-sol, high reasoning), launched 2026-10-20 from the agent shell with `codex exec --sandbox read-only` and the prompt at `roadmap/briefs/REVIEW-PROMPT.md`, naming the range 48d79572..54f4ceea as one step; 111,965 tokens. Its answer is verbatim below. No fix is built before the owner reads it.

54f4ceea, reviewing 48d79572..54f4ceea: the brief’s done-when does not yet hold—the recorded ratio is 1.34×, and bound renegotiation remains pending; execution-dependent claims are not verified.

Findings, most severe first:

1. [crates/acquisition-store/src/lib.rs:3253](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/lib.rs:3253) — **Shapes 1, 2:** the claimed two-opener migration test exercises contention without another opener migrating. The writer commits no changes while the opener waits; replacing the under-lock version reread with the preliminary version would still pass. Synchronize the loser after its preliminary read, have the writer commit a version change, and assert the loser observes it—for example, a stamp of 99 must produce `SchemaTooNew`.

2. [crates/acquisition-store/src/corpus.rs:19](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/corpus.rs:19) — **Shape 3:** the module’s “As built” description still attributes streaming order to `items_location` and a per-location sort. The committed query-plan evidence says the live read now uses `items_live_at` without that sort; only the all-items read retains the old behavior. Distinguish the two reads in this paragraph.

3. [search/M3-LEDGER.md:10](/Users/tom/Development/GitHub/smallcatlabs/acquisition/search/M3-LEDGER.md:10) — **Shape 1:** “90,890 removed items added” confuses the resulting total with the addition. Four copies of 22,721 live rows add 90,884 rows; the original six make 90,890 removed overall. Change the wording to “90,890 removed items in total.”

What I did not check and why: No builds, tests, scripts, SQL execution, profiling, or live calls, as instructed. Gate success, actual timings, measurement isolation, and the profiling attribution are **not verified**; I assessed their recorded evidence and the code statically.

Nitpicks: Findings 2 and 3 are documentation nitpicks without runtime consequences. Finding 1 is a substantive coverage gap.

Brief: It assumes an existing facts-file two-opener race test; explicitly require creating one if absent.

Plan: The roadmap appropriately leaves this step open pending the owner’s bound decision.

Prompt: Nothing impeded this review; naming the complete range removed ambiguity about its scope.
