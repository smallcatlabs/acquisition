Report: the ruling registry and the tools that enforce it. Read-only. Every number below comes from the command named beside it. "Judgment" marks opinion.

## 1. The five findings that matter most

1. **The registry only grows, and the 800-byte entry limit stopped holding once it became a note.** It went from 120 entries and 64,397 bytes with none over 800 (2026-10-20) to 169 entries and 115,414 bytes with 25 over (2026-11-09). The over-limit count was 0 on 10-26, the day the limit became "note, never fail" (commit 5eda514f), then 15 on 11-01 and 22 on 11-05. No id has ever been retired: ids 1–169 are all present and none is marked superseded.
2. **The overflow is mostly test inventories in `*Pinned:*`, built up by build commits that never quote the owner.** C165 was 628 bytes at its ruling commit (96d9205e). It reached 2,995 bytes after seven build commits the same day, and none of those seven mentions the owner. That breaks bothy.py's entry rule 7: an entry changes only by his ruling, and the commit quotes him. Across the 25 over-limit entries, 31% of the bytes are `*Pinned:*`, and 15 of the 25 would fit if `*Pinned:*` were short.
3. **The owner's words live in at least three places, and none of them is the registry.**
   - `roadmap/README.md` "The owner's rulings for this road": 23,230 bytes, dated quotes with commits.
   - Commit messages: 200 lines containing "owner's word" since the reset.
   - `roadmap/LEDGER.md`: 27 lines.
   - Two entries also carry owner quotes the no-history rule forbids (C53 and C156, below), and docs-check's history-marker scan does not catch them.
4. **The structural checks hold; the spelling greps keep getting chased.**
   - Check 13 (reqwest held to one daemon file) was retired after "three review rounds in a row found another spelling" (commit baf625c9). The owner then chose a dependency edge.
   - Check 12 needed two commits for new spellings (68cb9448, 68b9feb1). Check 14 drew "misses ordinary aliases" (`roadmap/reviews/175-one-test-clock-96d75338.md:9`). arch-check.sh itself records "Six review rounds" on check 5's reader.
   - Since 11-01, docs-check, bothy and trade-check have had zero commits. arch-check has had 9.
5. **The citation graph is real and dense, but its biggest reader is the roadmap, not the code.** Every one of the 169 ids is cited outside the registry. Across all of them, the roadmap holds 4,191 occurrences against 2,989 in src and 1,734 in tests. Eleven ids get at least 80% of their citations from the roadmap. Half the cross-cutting rulings (9 of 18) say `Pinned: nothing yet`.

## 2. Measured facts

**Inventory** (my python parse of `AGENTS.md` and `decisions/*.md` bullets; it agrees with `bothy.py check`: "169 decisions in 10 files, 38 not-doing lines, 3 principles folding 6 ids, 5 invariants")

| Area | Entries | `Pinned: nothing yet` | Not-doing lines |
|---|---|---|---|
| AGENTS.md (cross-cutting) | 18 | 9 | – |
| daemon | 38 | 3 | 13 |
| search | 22 | 0 | 4 |
| network | 21 | 2 | 2 |
| frontends | 18 | 1 | 8 |
| store | 18 | 5 | 2 |
| pricing | 15 | 3 | 2 |
| plans | 12 | 0 | 4 |
| trade | 6 | 0 | 3 |
| ci | 1 | 1 | 0 |
| **Total** | **169** | **24** | **38** |

- Highest id is C169 and there are no gaps.
- Entry sizes: median 675 bytes, mean 682, maximum 2,994 (C165). 25 entries are over 800 bytes and 44 sit within 10% of it (720–800).
- `Unbuilt`: 6 (C71, C73, C78, C118, C162, C163).
- The brief says 19 cross-cutting rulings. There are 18, and docs-check prints "18 always-loaded decisions (limit 18)", so AGENTS.md is at its cap.

**docs-check today** (`tools/docs-check.sh`, exit 0)
- "AGENTS.md 16689 > 16600 bytes — over its budget"
- 25 "over the entry limit" notes
- "uncited decisions … C155"
- "767 cited commits", "616 document names", "62 goldens"

**bothy check** (`python3 -I tools/bothy.py check`, exit 0)
- One note: "no module doc names decisions/ci.md".

**Citations outside the registry** (python over `git ls-files` *.rs, *.md, *.sh, *.py, *.toml, *.yml; excludes `/data/`, `spikes/` and the registry; occurrences)
- Every id is cited at least once. Lowest: C56 (1) and C57 (2), both cited in `acquisition-store/src/lib.rs` docs.
- Seven ids have no src or tests citation: C15, C20, C73, C110, C125, C154, C155.
- Median 39 occurrences per id, in 20 files.
- Top cited: C53 570 (217 src, 243 roadmap), C122 250 (184 roadmap), C137 225, C84 203, C146 195, C106 195 (130 src), C158 183, C154 179 (177 roadmap, 0 code), C1 177.
- Search-corpus data inflates raw counts: C101 appears 1,538 times in `percentile-expected.csv`, so a naive grep misleads.

**Cross-cutting ruling ranks** (same scan; rank out of 169)

| Id | Rank | Occurrences |
|---|---|---|
| C122 | 2 | 250 |
| C154 | 8 | 179 |
| C1 | 9 | 177 |
| C46 | 14 | 144 |
| C35 | 26 | 113 |
| C153 | 33 | 103 |
| C47 | 39 | 92 |
| C38 | 51 | 71 |
| C12 | 67 | 52 |
| C34 | 93 | 32 |
| C48 | 95 | 31 |
| C14 | 96 | 30 |
| C79 | 99 | 29 |
| C4 | 110 | 24 |
| C2 | 121 | 19 |
| C120 | 143 | 11 |
| C15 | 150 | 8 |
| C110 | 151 | 8, roadmap only |

- No cross-cutting id is duplicated as an entry in an area file. My parse found 169 unique ids in 169 entries.
- At least 13 area rulings out-rank C2, C4, C15, C110 and C120: C53, C137, C84, C146, C106, C158, C144, C143, C160, C121, C148, C161, C93.

**P-ids and N-ids**
- P-ids are cited mostly from the roadmap: P2 has 218 roadmap occurrences of 250, P3 has 167 of 177, and P7 has 0. bothy resolves P-ids.
- N-ids: 52 defined (N1–N52 in `docs/design/network-ground-truth.md`). 42 distinct N-ids are cited outside it (464 occurrences, 118 in src).
- Neither docs-check nor bothy resolves N citations. `grep N[0-9] tools/docs-check.sh tools/bothy.py` returns nothing. `N0` is cited somewhere outside and nothing checks it.

**Tools** (`wc -l`, `wc -c`, `git log --oneline [--since=2026-10-19] -- <tool>`, and `run_case`/`expect` counts in the breakers)

| Check | Property | Kind | Lines | Commits since 10-19 | Breaker cases |
|---|---|---|---|---|---|
| docs-check 1, budget | AGENTS.md ≤ 16,600 bytes (note only) | size | ~15 | 29 for the whole tool | ~39 run_case for the whole tool |
| docs-check 2, registry | history markers, cross-cutting cap, entry length (note), cited ids real, uncited listed | textual | ~95 | – | – |
| docs-check 2b | cited hashes exist; a cited path existed at its commit | structural (git) | – | – | – |
| docs-check 3–7 | stale identifiers, README form, hooks, doc paths, goldens | textual | – | – | – |
| bothy check | cross-refs resolve, Why/Pinned present, not-doing form, registry paths exist | textual | 560 for the whole script | 6 | 24 expect |
| arch-check 5 | crate edges from cargo metadata, read twice by two jq programs | **structural** | ~416 | 23 for the whole tool | ~120 run_case for the whole tool |
| arch-check 5, sub-greps | intent API, frontend "door" names, TLS option words | textual | – | – | – |
| arch-check 7 | one reader of a group's tree | textual | ~28 | – | – |
| arch-check 8 | test layout | textual/file | ~58 | – | – |
| arch-check 9 | spawn via the harness | textual | ~43 | – | – |
| arch-check 10 | one `spawn_blocking` | textual | ~51 | – | – |
| arch-check 11 | one exempt-route set | textual, parsed | ~115 | – | – |
| arch-check 12 | tokio timers through the clock | textual, spelling-chased | ~119 | – | – |
| arch-check 14 | one test clock | textual, spelling-chased | ~36 | – | – |
| trade-check | runs the sitting scripts' self-tests | behavioural | 69 | 4 | ~5 run_case |

**Sizes**
- The four checks total 110,892 bytes. Their breakers total 66,224 bytes.
- All eight `*-breakers.sh` files total 96,638 bytes (`wc -c tools/*breakers*.sh`).

**Where the breakers run**
- `tools/gate.sh` does not run them: grep finds no "breakers" in it.
- CI runs them only in gate-linux.yml's `breakers` job (lines 189–231).
- `sign-acqd-breakers.sh` runs weekly (weekly.yml:86).

**Caught real faults (recorded)**
- bothy `pack` caught nine unrouted verbs modules (commit 797d1116, #114).
- I found no other commit or review recording a check catching a fault in work being built. Judgment: such catches happen before commit and leave no trace, so "none recorded" is not "none caught".
- Most recorded check history is the reverse: reviews finding the check's own holes.

**`bothy.py pack --path`** (`… | wc -c`)

| File | Bytes | Entries | Areas |
|---|---|---|---|
| `store/src/corpus.rs` | 48,400 | 58 | store + search |
| `daemon/src/gate.rs` | 31,392 | 39 | network |
| `cli/src/main.rs` | 33,453 | 36 | frontends |

- About 9.5 KB of each pack repeats AGENTS.md (invariants, principles and 7,388 bytes of cross-cutting rulings), which every session has already loaded.
- The corpus.rs pack also carries an 8,964-byte "Where each ruling is held" file:line list.
- `pack --path crates/acquisition-daemon/src/limiter.rs`, a file that does not exist, printed 74,865 bytes instead of refusing. The output is a usable reading list but a long one. Judgment: as a pre-read it costs 30–75 KB per file.

**Amended rulings: how many commits to see why an entry reads as it does** (`git log -G'\*\*C<n> —' -- decisions AGENTS.md`)
- C1: 4 commits. The latest, baf625c9, quotes "(b) Own crate".
- C53: 5 commits, growing 713 → 1,560 → 2,306 → 2,459 bytes. Each amendment's commit names the owner.
- C165: 8 commits, 7 of them build commits with no owner mention.
- Reconstructing the why also needs `roadmap/README.md`'s list and the briefs.

## 3. What earns its keep

- **arch-check 5's cargo-metadata graph.** It is structural, two jq readers cross-check each other, and ~120 breaker cases back it. The project's own response to a spelling chase was to move a property onto it (check 13 retired, baf625c9). That is the principle "structure over discipline" working.
- **C154's retirement-line practice.** It is `Pinned: nothing yet` but used heavily: 331 `retired …; held by` lines in 96 commits since 10-29 (`git log --format=%B | grep -cE 'retired .*; held by'`).
- **bothy check and docs-check 2b/6.** 767 cited commits and 616 document names are verified on every gate. They are stable: no commits since 11-01.
- **The citation density.** Every ruling is reachable from code or docs. `bothy show` and pack's "where held" list make the id-to-code join cheap.
- **Gloss-on-id and one-bullet form.** In my 8-entry sample, entries are readable without opening other ids.

## 4. Friction, cruft or vestige

**Cold-reader sample.** I picked 8 entries with Python `random.seed(2026); random.sample(ids, 8)`: C31, C82, C129, C132, C166, C27, C58, C154.
- Actionable alone: C27, C166, C129 (dense, but every term is defined) and C58.
- Leans on unglossed working nouns: C31, C82 and C132 ("world", "real-mode", "external concurrency", "a design event").
- Mixed: C154. Its *Why* is history ("had no procedure … so nothing was retired"), which rule 5 forbids in spirit.
- Best: C27 at 473 bytes, "A client that disappears leaves its jobs running … *Why:* the sends are committed either way."
- Worst in the sample: C132 at 797 bytes, which uses "external concurrency" and "real-mode" unglossed and is pinned by a test named `c83_…`. Outside the sample, C165's `*Pinned:*` is a ~2.3 KB test list (decisions/daemon.md:13).

**Other friction**
- The entry limit is a note that 25 entries ignore (§1.1). Judgment: it now signals nothing.
- AGENTS.md is over its budget (16,689 against 16,600) and at the cross-cutting cap, with the overage as a standing note.
- Each pack prescribes 30–75 KB, a fifth of it already loaded.
- Breakers do not run in the local gate, so a check edit is proven only in CI.
- The textual greps in arch-check 7–14 are where the review cycles go. Since 11-01, arch-check has had 9 commits and the other three tools none.

## 5. Contradictions and stale statements

- `decisions/frontends.md:11` (C53): "(the owner's word: "Yes, the index.")". The same file's line 23 (C156) says "Under the owner's word." Both contradict bothy.py:36 ("no owner's quote"), and docs-check reports "none carrying a history marker".
- `tools/bothy.py:41` (rule 7, the owner quoted on every entry change): C165's seven build commits changed it without him.
- `tools/docs-check.sh:4` says "Six checks" but lists 1–7 plus 2b.
- docs-check's "uncited decisions: C155" is false. C155 is cited at `.github/workflows/gate-linux.yml:32`, `:187` and `gate-windows.yml:29`. The uncited scan does not read workflows, though 2b's commit scan does (6faeb2f8).
- The brief's "19 cross-cutting" and "governance reset 2026-10-19": there are 18 entries, and 0704a8f7 is dated 2026-10-20 (`git show -s --date=short`).
- `bothy pack --path` routes a non-existent path instead of refusing it.

## 6. Ideas

- Make `*Pinned:*` name one test or prefix (the rule's own text), with the list kept in the code via `bothy show`.
- Make the entry limit fail again for new or changed entries only.
- Drop AGENTS.md's sections from `pack` output, since every session has them already.
- Add `.github/` to docs-check's uncited scan; resolve N-ids as C-ids are resolved.
- Run the four registry-tool breakers in `gate.sh --docs`. They take seconds.
