## 1. Model

Search is a pure library over the store's read. The flow: text goes through `parse.rs` (1,607 lines) to a `tree::Node`, then `tree::check`, then `bind.rs` (1,743), where `group.rs` (1,227) gives `line(...)` its meaning. The output is a `Query` that never depends on the corpus. Separately, `Corpus::load` (`corpus.rs`, 1,081) derives every item once (`derive.rs`, 1,443), classes it (`class.rs`, `category.rs`) and joins its price, under a `Basis`. `answer()` evaluates every term on every item (`eval.rs`, 1,577), with `pseudo.rs`/`totals.rs` (3,353) for computed values. It then builds the rows or the counts (`counts.rs`, 1,464), the routes and the offers.

A trade URL is one more spelling of a query. `QueryInput.trade` → `trade::decode` → `translate.rs` writes language text by filling the catalogue's `lands` templates (for example `"attacks_per_second{bounds}"`). It re-parses that text (`bind::parse_query`, translate.rs:1199/2381/2480) and runs it with a per-clause report.

All knowledge is about 2.5 MB of TOML, held as `include_str!` and parsed lazily on first use (`LazyLock`, e.g. `trade/stats.rs:186`). `build.rs` digests sources and tables into the basis's build name.

The tables come from three places:
- **The RePoE `poe1` clone at a pin:** classes, defences, game-forms.
- **Committed captures and extracts:** stats, categories, colours, totals.
- **Hand-kept:** catalogue, forms, properties.

The site knowledge loop runs: sheet composition, the owner clicking each link, recorder captures, split into `captures.json`/`bodies.json`, the oracle test, and finally catalogue rows.

## 2. Measurements

| What | Number | Command |
|---|---|---|
| src / tests / reference | 35,008 (22% comments) / 37,433 .rs + 3,048 / 98,580 lines | `find … \| xargs wc -l`; `grep -cE '^\s*//'` |
| `search/` tracked | 82 files, 1,019,489 lines, 47 MB; working tree 2.4 GB (gitignored raw) | `git ls-files search \| xargs wc -lc`; `du -sh` |
| …of which `pseudo-stats/data/captures.json` | 925,752 lines (91%), 18.3 MB, pretty-printed; `bodies.json` 11.4 MB | same |
| search/ in git history | ~13 MB packed of a 144 MB pack (`deps/`, the C++ era, is 88 MB) | `git rev-list --objects --all \| git cat-file --batch-check` |
| search test binary | 175 MB, holding **8 embedded copies** of captures/bodies (≈126 MB) | `grep -o -a '"_raw": "f954d11b' target/debug/deps/main-841b…` → 8; include sites at properties.rs:1146, ranged.rs:30, trade_dps.rs:24-25, trade_forms.rs:29, trade_oracle.rs:40-41, pseudo.rs:1517 |
| Loop tooling | tools 9,272 py + recorder 1,483 js + track scripts 8,495 (22 files) + skill 192 ≈ 19.4k, against ~5.5k Rust in `trade/` | `wc -l` |
| Sittings | 1,255 searches in 29 rounds; 474 of them (rounds k and l) for beast colours | `csv` count over `search-sheet.csv` by prefix |
| Translation coverage (14,700 category.id pairs) | no bound: 13,669 exact (93%), 811 ambiguous, 220 unsupported; with a min: 9,408 exact (64%), 5,072 ambiguous, 220 unsupported | asserted at `tests/trade_translate.rs:654-690`; **not in MEASUREMENTS.md** |
| Catalogue | 151 rows: 90 exact, 18 ambiguous, 34 unsupported, 9 listing-only; oracle 88 expected disagreements | `grep ^status` |
| Language | Node 11 kinds, Member 6, ValueRef 4, Value 3, 7 operators, 5 field kinds; 26 built-in fields + 44 property fields + 82 totals | `tree.rs:42-226`, `bind.rs:305` |

**What `answer.rs` is made of:** a 332-line module doc and 1,095 comment lines. The code splits into:
- request and view parsing and binding, ≈380–1440;
- serde output shapes, 1445–2178;
- the computation, including a 427-line `unaged` (2566);
- routes, offers and the zero block, 3143–3700;
- shell quoting, 3938–4222.

**What `translate.rs` is made of:** a 200-line doc of rules, then one 1,422-line `impl Translator`. Within it, `stat()` is 387 lines and `group()` 257.

## 3. Where a deep review would pay

**H1: the missing shape is a weighted sum of values (an algebra of values). Confidence: medium-high. Verified.**
- **Evidence.** Totals rows are weighted sums of lines (`totals-v6.toml:61-118`, weights 1 and 2), but the language's `sum(line(..).slot)` has no weight.
- `translate.rs::weighted` (2637) accepts a site Weighted Sum only when every weight is equal and every filter is a line. It divides the bound by the weight, and otherwise returns "its filter … is no line", which is #316.
- `decisions/trade.md` now rules that "a weighted sum of named totals … is to be built (#316)": a third special-case sum.
- #46 `stat("…")` is another value kind still parked.
- In `tree.rs`, `Node::Test{field}` duplicates `Node::Compare{ValueRef::Field}`, `Has`/`Is` are presence tests, and `Member` mirrors `Node`'s boolean algebra.
- **Question.** Would `value := field | pseudo.N | sum(line(…).slot) | stat(…) | k·value | value+value`, with each total defined as such an expression (printable by `--describe`), make #316, #46, K5's weight/weight2 groups and the totals loader one construct? Could predicates collapse to quantifiers (exists/count over a collection) plus `compare(value, op, rhs)`?
- **Files.** `tree.rs`, `totals.rs`, `pseudo.rs`, `translate.rs` (weighted, stat, group), `decisions/search.md` C92/C94/C95, issues #316 and #46.
- **Payoff.** Several concepts removed, the totals table becomes language text, and a few hundred lines of translation special cases go. This is the "ambitious read".

**H2: test-fixture mass. Confidence: high on the duplication, medium on time cost (not measured).**
- **Evidence.** `captures.json` is included 5 times and `bodies.json` 3 times into one test binary of about 175 MB. Under nextest's process-per-test, every oracle, ranged, property or dps test also re-parses an 18 MB JSON. The "1M lines under search/" is one pretty-printed fixture that pollutes `git grep` and every line count.
- **Question.** One shared `LazyLock` in `tests/common`, a compact or compressed encoding, and a measurement of link time and per-test parse time.
- **Payoff.** About 100 MB of link output per gate, plus seconds per gate.

**H3: the sitting loop's cost against what it delivers. Confidence: medium. Judgment on proportionality.**
- **Evidence.** About 19k lines of Python and JS serve about 5.5k lines of translation Rust, plus 1,255 searches the owner ran by hand.
- `trade-sheet.py --verify` (3099-3218) **recomposes every past round** (`round_two` … `round_zg`) and compares it to the committed CSV. About 2,500 lines of one-off round code therefore live forever under the gate. An append-only hash chain, or a check against `git` history, would hold "appended, never edited" without them.
- `percentile-expected.py` (1,600 lines) and `ranged-expected.py` (1,336) are Python second implementations of the semantics, used as spec-first oracles. That is deliberate, but they are frozen beside the Rust.
- Beast colours took 474 searches for a niche table.
- **Question.** Which scripts still run, and what is the marginal value of a sitting? Coverage stands at 64% exact under a bound and is reported nowhere a reader looks.
- **Files.** `tools/trade-sheet.py`, `tools/trade-check.sh`, `search/pseudo-stats/scripts/`, `.claude/skills/site-sitting/SKILL.md`.

**H4: `answer.rs` is four modules, and it holds frontend code. Confidence: medium. Verified.**
- `Shell`, `shell_quoted`, `posix_quoted` and `powershell_quoted` sit in a library and read `ACQ_SHELL`/`MSYSTEM` (answer.rs:3981-3989). The CLI's `printed.rs` and `price_cmd.rs` consume them, and a daemon twin lives in `login.rs`.
- Routes wrap `group.selector_tree` in `Node::Members` themselves (answer.rs:3311, 3329, 3476), past `group.rs`'s claim to be "the one reader".
- **Question.** Split request, out-shapes, compute and routes, and move quoting to the verbs crate or the CLI.

**H5: semantic-model findings left with no route into the tracker. Confidence: high. Verified.**
- `selector_tree` is still `pub` (group.rs:697).
- `Basis::against` still lists 14 parts without `properties` (corpus.rs:232).
- `Line.numbers` is still `Vec<Option<f64>>` (derive.rs:388, 406).
- None of these is filed.
- The canonical-case finding went unacted, then failed CI as #273 (2026-11-12). It was fixed for field and pseudo names (`4d0f824e`, lowercased in `tree.rs:258`), but `has:`/`is:` names are still kept as typed.
- **Lesson.** An audit's "provably equivalent" refactors need a path into issues.

**H6: "twin" means two things.** It names both the defence local/global twin (defences.rs, 32 hits) and the case twin (counts.rs, 24). Assessment candidate 10 (case twins, about 128 source and 109 test lines) is still "not weighed". Confidence: medium.

## 4. Low yield / already covered

- **Determinism and floats:** the semantic audit verified them.
- **Parse, print and tree round-trip:** property-tested.
- **C198 regeneration:** `sources.py` pins, refusal on absence and `game-diff --check` are well made, though the clone checks are skipped on CI by design.
- **Assessment candidate 3 (the query's own order):** done. `Direction`/`default_order` are gone, #294 is closed, and the "Not doing" line was approved on 2026-11-21.
- **`reference/trade-filters-2026-11-17.json`:** a byte-identical copy of the evidence, deliberately held to it by sha256.
- **The catalogue's text-template design** is elegant; don't redesign it.

## 5. Blind spots

- **Runtime cost of first use** of the 1.35 MB stats TOML and 750 KB game-forms TOML in `acq-mcp` and the web finder is unmeasured. The test comment says a debug build "binds the whole table's text in half a minute". #307 added a `Mutex` static cache (`trade/vocabulary.rs:279`), a cache inside the library.
- **Coverage by real use** (the share of pasted URLs that translate whole) is unknown.
- **Test wall time** per file is unknown; I ran nothing.
- **Python oracles going stale:** whether the expected-answer scripts still reproduce their committed CSVs is unchecked (not in the gate).

## 6. Surprises

- The "1M lines" is about 91% one fixture, and it is load-bearing for tests.
- `search/README.md`, the first thing read for the crate, has a status cell of about 3 KB of dated narrative, against the rule that a row "points and never summarizes".
- The crate doc names `printed.rs` as the one maker of a command, but the quoting it depends on lives in `answer.rs`.
- A ruling ("to be built", #316) is adding a special case where a general shape is visibly missing.
- Pack history is dominated by the C++-era `deps/` (88 MB), not by search.

Verified by re-reading: H1 (`weighted()`, the totals weights, the trade.md line), H2 (the include sites and the 8 copies in the binary), H3 (`compose_sheet` and `verify`), H5 (all three lines).
