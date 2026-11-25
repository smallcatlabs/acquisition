## 1. Model

You don't need the daemon to run a search. `acq search` reads SQLite directly (`search_cmd.rs:2`). Every CLI ask rebuilds the whole corpus. `Corpus::load_of` (`search/src/corpus.rs:606-851`) streams every live body under one snapshot. It parses each body into a `serde_json::Value` (the workspace sets `preserve_order`, so every object is an IndexMap). It then derives each item into owned Strings per line, classifies and categorises it, names sockets and sorts. After that, `price::join` asks SQLite to `json_extract` the same bodies a second time, once per league (`store/src/snapshot.rs:754-758`). The reference TOMLs are `include_str!` and parsed lazily, once per process. The 2 MB of trade tables are parsed only when a query uses them.

Resident consumers (`acq-mcp`, `acq web`) keep one corpus in a `Holder`. Before every answer they call `Corpus::is_current`, which re-reads the full corpus header (`corpus.rs:856-864`). Any committed response moves the revision, and that forces a whole reload: there is no incremental update (#32). Each web request also reopens the store and the intent file. An item card does not use the held corpus. It calls `show`, which scans every row of the store to find one id and then runs a league-wide price join (`show.rs:670,721`).

Evaluation is O(N × terms) plus a sort on every request, including every 50-row scroll window. That is cheap at 22k items, at 4–7 ms.

On the write side, ingest rewrites every item row on every fetch, changed or not. Each statement is prepared again for every item, and there is no `prepare_cached` in `lib.rs`. GGG's rate limit sets the pace of a refresh, so the CPU cost of ingest doesn't matter.

## 2. Measurements

| What | Number | Data, date | Source / command |
|---|---|---|---|
| Corpus | 22,721 items (22,623 pc), 29.9 MB bodies, 51.9 MB file | copy of 2026-09-13 | `search/MEASUREMENTS.md` M1, M7 |
| CLI empty query, release | 440 ms (debug ~2.7 s) | last row `5235d928`, about 2026-10-20 | `tail search/M3-LEDGER.md` |
| Same, removed-heavy copy | 590 ms (over 0a's 1.2× bound) | same | same |
| 5× store (113,635 items) | 2.16–2.33 s, 971 MB; price join 41% of samples | 2026-10-19 | `roadmap/STATE-2026-10-19.md` report 5 |
| Held corpus | 195 MB; +36 MB retained per reload | 2026-10-27 | M8, #95 (open) |
| MCP warm / cold search | 9–15 ms / 430–490 ms | same | M8 |
| Web window, server time | 25–37 ms median: currency check ~22, answer 4–7, open ~2 | 2026-11-11 | python over `audits/web-2026-11/spike/headless-3.jsonl` `windows.each[].server` |
| `show` one item | 110–150 ms (1×), ~0.5 s (5×) | Sept / Oct | MEASUREMENTS seat block; report 5 |
| 200 rows with items | 0.57 MB, 4.7 ms | 2026-11-08 | M9 |
| Gate, warm, M4 | median 65 s (test 39, p90 63) | 228 full runs, 11-10 to 11-25 | python over `runs/gate/verdicts.tsv` |
| CI gate-linux | 11–17 min | latest | `gh run list --workflow gate-linux.yml` |
| Binaries | 20.8 / 18.4 / 9.7 MB, no `[profile.release]` | `5f3ad0d1` | #201 (open) |
| Dependencies | 351 packages in the lock | now | `grep -c '^name = ' Cargo.lock` |

No benchmark, criterion dependency or latency budget runs in the gate or in CI (`grep` of `tools/gate.sh` and `.github/workflows`). The only guards cap complexity: a 20 s bound on a pathological query (`search/tests/rows.rs:735`) and the tiled real-scale fixtures for quadratic text views (`cli/src/real_scale_fixture.rs`). M3 is a hand-run script whose last row is five weeks old. That run predates `acq web`, the trade catalogue and the 3.30 league. Since the 3.30 launch the owner's Standard league is 3,369 tabs plus 3,009 substashes (`RUN-LEDGER.md` tail). His store is probably well past the 22k items every number above is measured on, but nobody has counted it.

## 3. Where a deep review would pay (ranked)

**H1. The currency check reads the whole header to compare two scalars.** Verified: `corpus.rs:856-864`, `Basis::of` at `527-552`, `store/src/corpus.rs:667-711`.
- `Basis::of` uses only `header.revision` and `header.account_uuid`.
- But `is_current` builds the full header: realms, the listings (`json_extract` over every `responses` row, which has no index on `endpoint`), every location, the games (`json_valid` plus `json_extract` over character bodies) and the gone list.
- The cheap one-row `Store::revision()` already exists and is used by `still_at`.
- It is called while the `Holder` mutex is held (`corpus.rs:919-923`), so concurrent windows queue behind it. `places::steady` calls it twice per page (`places.rs:776-783`).
- Measured, it is about 60–90% of a web window's server time (22 of 25–37 ms). It also grows with store age, because `responses` gains ~3k rows per full refresh and the gone list grows until prune (#43) exists.
- M8 saw it and left it "as it is".
- Question: can currency be revision + account + path, held by one test? Payoff: web windows ~3× faster, MCP warm calls a few ms, an age-dependent cost removed, one small function. Confidence high.

**H2. Every body is parsed twice per load.** Verified: `corpus.rs:839`, `snapshot.rs:754-758`.
- The corpus already derives `note` from each body.
- `price::join` then has SQLite `json_extract` note and inventoryId from every body again.
- Report 5 put this at 41% of a 5× load (32% for `read_items` alone). The 9.5 KB-per-item footprint comes from Value-based derivation into owned Strings.
- Question: can the price join take its inputs from the derived items (one fact, one home)? Can the derive parse typed or borrowed instead of into an IndexMap Value?
- Payoff, a judgment: the CLI floor from ~450 to ~200–250 ms, 5× from 2.2 s to ~1 s, and less memory, which also eases #95.
- This is also the only lever for the CLI, which reloads per process by ruling (C98, no service). Confidence medium-high.

**H3. Opening one item costs about a whole-store pass.** Verified: `show.rs:670-721`, `web/mod.rs:2700-2710, 1278-1340`.
- `show` streams every row (`Membership::All`) to find one primary key, then joins prices for the league.
- The web card adds `place_links`, which reads the whole tab listing: about 41 ms for 3,347 tabs (spike rig-1). Rig-2's held listing was not shipped into `places.rs`.
- So a card costs ~150–200 ms at 1× and ~0.6 s at 5×, while scrolling is 25 ms.
- The owner wants C++-style interactivity. Card clicks and live refresh are the gap, not the table.
- Question: should a card come from the held corpus (the `Row` `item` field M9 already makes from `show`'s one maker), with `show` doing a keyed lookup? Confidence high.

**H4. No standing measurement (finding number one).** The cheapest standing measurement would extend the existing `rows-item` and `read-corpus` examples to print the split: open, currency check, load (derive vs price join), answer, show and resident size. Run it at each close over the M3 copy and over a fresh copy of today's store. In CI, run a ratio guard over a synthetic tiled store, the way `real_scale_fixture` tiles one: time per item, or an instruction count, against a generous bound.

**H5. Live refresh against a resident holder.** Each committed tab triggers a whole reload of ~450 ms that retains +36 MB. A 3,055-fetch refresh with the GUI open is the unmeasured worst case (#32, #95 open). This is a design question for part 3, but its numbers should be taken first.

## 4. Low yield / already covered

- **Daemon scans** (`queue_order` per 1 s pacing slice, the child scan in `dispatch.rs:340-385`): still standing from the semantic-model audit, but jobs run at GGG's pace, so the total is milliseconds per refresh.
- **Ingest's per-item prepare and its `status()` count scans**: also bounded by the rate limit.
- **Logs**: bounded at daemon start (`rotate_if_over`).
- **Table parsing**: lazy, and tens of ms in release.
- **Regex compiled per translation**: fixed (#307).
- **Scroll rendering**: the spike passed every case (`audits/web-2026-11/spike`).
- **Build time**: a lean graph, no generated code (the tables are embedded TOML), the gate already tuned (incremental, deps at opt 2). The only open lever is #201 (release profile, tokio features).
- **The persisted projection**: premature until H1–H2 are done.

## 5. Blind spots

- The current size of the owner's store, in items and file size, after 3.30.
- What Value parse, derive, classify and categorise each cost, separately. GUI-NOTES 8.2 move 1 is still unbuilt.
- The steady memory of a long-lived `acq web` during a refresh.
- What `Store::open` costs per web request on a large WAL.
- Test-time distribution per test (nextest timings are never recorded).
- Windows performance, IPC and SQLite there.
- I built and ran nothing. Every number above is someone else's record.

## 6. Surprises

- M8 measured the warm MCP cost and kept the whole-header check. The spike later measured that same check as the dominant web cost, and no issue was filed.
- Report 5 (2026-10-19) found the double parse and `show`'s full scan. Five weeks later both still stand, and neither has an issue.
- The roadmap's goal is "faster", yet the 500 ms budget has had no measurement since 2026-10-20.

**Commands for the orchestrator** (may build):
1. `cargo build --release -p acquisition-search --example rows-item && /usr/bin/time -l target/release/examples/rows-item <copy.db>`, adding the timing split from H4.
2. `target/release/acq --json search --realm pc --limit 0` under `/usr/bin/time -l` over the M3 copy and over a fresh `.backup` of today's store. This is the M3 floor, and it tells you whether the corpus is still the 22k one.
3. The spike runner (`rig-run.py` in `rig-3.patch`) against today's `acq web`, adding an item-card case.
