1562eb27..a7395a13 — the brief’s done-when does not fully hold: both item commits and pins exist, but facts consistency and account-remedy coverage remain incomplete.

Findings, most severe first:

- **P2 — `crates/acquisition-search/src/tabs.rs:214`, `crates/acquisition-store/src/lib.rs:2239`: rows/counts and coverage describe different snapshots.** With valid persisted input, a daemon fetch committed between `store.tabs()` and `store.coverage()` can leave a row saying “never fetched” while the facts line names that tab’s fetch; status likewise collects counts before coverage, and its refused count separately from its newest refusal. Smallest fix: assemble each document’s database reads under one store-owned read transaction, using one answer time, and pin a write between the component reads.

- **P2 — `crates/acquisition-search/src/tabs.rs:220` — shape 7: the two reads also disagree about live membership without any concurrent write.** After valid upstream listings retire a map/unique parent, ingest preserves its substash rows with their own `removed_at` null; `Store::tabs()` includes them, while coverage’s `TAB_GONE` excludes them through their parent. Thus tabs can count a never-fetched location that store-wide facts exclude. Smallest fix: share the parent-aware live-location predicate between the rows, counts and coverage; pin the retained-substash case.

- **P2 — `crates/acquisition-cli/src/store_cmd.rs:361`, `crates/acquisition-cli/src/web/mod.rs:593` — shape 3: empty leagues skip the facts line.** Persisted input containing a successful empty stash listing produces JSON with `list_seen`, but CLI text and the refresh page omit that freshness information entirely. Smallest fix: render the shared facts line on the empty branch too; pin an empty fetched listing separately from an unseen listing.

- **P2 — `crates/acquisition-cli/src/main.rs:1136`, `crates/acquisition-cli/src/search_cmd.rs:678` — shape 3: the account remedy is restricted to one fault variant and one reporting path.** With Alice and Bob known, client input `acq --account Missing tabs` lists known names but gets no ready commands; a valid account-less `search --request request.json` prints its ambiguous-account refusal through `AlreadyReported`, bypassing the new renderer altogether. Both JSON paths already offer account alternatives. Smallest fix: render account alternatives from the shared refusal document on every text-reporting path, preserving the existing kinds and correctly spelling request-based search offers.

- **P3 — `crates/acquisition-cli/tests/goldens/seat_faults/account_to_pick.txt:16` — shapes 3 and 5: capping commands leaves the refusal itself unbounded.** The twelve-account golden prints all twelve names above ten commands; a larger persisted account index still produces an arbitrarily long opening sentence. Smallest fix: bound the displayed names and count the remainder while retaining the full contract data. This is a preserved sibling, and changing it requires resolving the instruction that the sentence remain unchanged.

What I did not check and why:

- Execution, gate/doc-gate results, test counts, measured tool sizes and shell execution: **not verified**, as requested.
- The unavailable seat-call outputs; I judged their wording against the brief’s quotations.

Nitpicks: only the unbounded account header, because more than ten accounts is unlikely in ordinary use.

Brief: clarify whether preserving the refusal sentence deliberately exempts its name list from the bound.

Plan: no separate plan was supplied.

Prompt: “both references changed” presupposes a CLI-reference diff; its absence is reasonable here because clap help did not change.
