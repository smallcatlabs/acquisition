# Investigator's report: search-kernel

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/search-kernel.md`.

## Search kernel, tables and the trade loop: investigator report

Everything I ran used the copy of the owner's store through `<scratch>/search-kernel/acq.sh`, the preamble's command with `--account 'GERWARIC#7694'`. I ran no cargo. Timings are noisy because other readers were working: a debug search over 44,090 items took about 1.1–1.3 s (`time acq.sh search --realm pc 'class:rings pseudo.total_life>=50' --limit 0`). **V**

### 1. The picture

**Lines by class.** Counted with `wc -l` per module, test modules included. Production lines exclude everything after `#[cfg(test)]`. **V**

| Class | Modules | Lines | Production |
|---|---|---|---|
| Kernel (the language) | tree, parse, print, json, bind, eval, group, exact, template, error, values | 9,091 | 8,525 |
| Derivation (an item from its body) | derive, sockets, properties, corpus, places, price, tabs | 5,919 | 5,692 |
| Tables (data about the game) | class, category, defences, totals, pseudo | 6,239 | 5,233 |
| Answer | answer, counts, describe, show, lib | 8,219 | 8,100 |
| Trade | trade.rs, trade/* | 5,540 | 5,540 |

`pseudo.rs` is half computed-value semantics (485–860) and half derived fields (dps and the percentile, 856–1720). **V**

**What pays for each concept.**

| Concept | What pays for it |
|---|---|
| matched / failed / lacked / undecided | C93. Lacked is large on real data: `pseudo.total_fire_res>=30` has 41,728 lacked. Undecided was 0 in all 15 queries I ran. **V** |
| `holds` | The site's `count` group, and a player's "two of three resistances" (`class:boots holds(...)>=2` gave 48 matches). **V** |
| `undecided()` | C93's route to the open items. A player never typed one. **I** |
| `line(…)`, slots | C92: one occurrence, and a mod displayed over several rows. Essential. |
| `sum` | C92, and every bound the translation writes (`sum(line(T source=…).arg1)`). **V** |
| Totals and readings | The site's pseudo stats: 82 totals and 4 readings, 420 rows. **V** |
| Views (rows, counts, cross, explain) | The counts view is the vocabulary read (C97), which is agent discovery. |
| Routes and the continuation | Invariants 4 and 5, and C100. |
| The 13-part basis | C98, no cache. |

**Vocabulary (question 3).** The language did well:

- `has:note`
- `level>=20 quality>=20` (gems)
- `--count map_tier`
- `rarity=unique -has:priced`
- the bare `-priced` refusal offered `-has:priced` as its way on

It made the obvious hard in four ways:

- **`life>=90` is refused.** Its near names are `level`, `links` and `pseudo.life_regen`. `pseudo.life` is refused too, and its near names miss `pseudo.total_life`, because `bind.rs:530-556` cuts to the first 3, ordered alphabetically within a score. **V**
- **"Best life+res ring" cannot be asked.** `--sort 'pseudo.total_life + pseudo.total_ele_res'` fails with "expected the value to end, found `+`". **V**
- **One of two spellings is refused.** `line("#% increased Movement Speed")>=25` is refused, while `"#% increased Movement Speed">=25` lowers to the same term. **V**
- **The vocabulary is the site's, not a player's.** The computed-values block is 13.9 KB of the 34.6 KB `--describe` index, and every row restates a site id (`pseudo_total_…`). Fields are 8.3 KB. The grammar itself is about 3 KB. **V** (block sizes measured)

What an agent pays to learn is vocabulary, not syntax. **I**

**`answer.rs` (question 6).** **V**

- a 332-line doc
- request types and their binding, about 380–1437
- about 40 serde output types, 1437–2160
- the answering, including the 427-line `unaged`
- routes, offers and the zero block, about 3213–3700
- quoting for four shells, 3938–4222; it reads `ACQ_SHELL` in a library

The smallest answer type is roughly what exists: `{query{text,tree,trade?}, scope⊃basis, total, view: Rows|Counts|Explain, terms[path, term, 4×Count{n, route}], next?}`. Little of the shape can go. The file should be four modules, and the quoting belongs to the CLI. **I**

**The trade loop's reach (question 4).** It is not a side feature:

- `acq web`'s form is generated from the site's filter dump, folds into the site's JSON and runs through the translation (`crates/acquisition-cli/src/web/form.rs:1-14`). **V** The translation is the GUI's query compiler.
- `--trade` is the README's headline feature (`README.md:5`). **V**
- The totals, the categories and the percentile were all decided by sittings.

### 2. Theses

**Thesis 1: There is one value expression, and #316 and #46 are rows of it, not new kinds.**

The evidence that it already exists:

- A total is literally `Σ w·sum(line(T).slot)`. `--describe pseudo.total_life` prints "1 × arg1 of line("# to maximum Life") + 0.5 × arg1 of line("# to Strength") …". **V**
- 45 of the 420 rows are a `reduced` form at weight −1 (`grep -c "weight = -1" totals-v6.toml`). That is #46's `stat()` already shipped, inside the table, for five stats. **V**
- `translate.rs:2633-2680` (`weighted`) refuses a site weighted sum for five reasons: no line, own bound, no weight, different weights, no slot. Three of those exist only because the language has no weights. **V**

The type, in about a dozen lines:

```
enum Val { Field(Thing),                                  // incl. derived: dps, percentile, q20 (non-linear, stay)
           Lines { sel: Member, slot: Option<Slot>, agg: Agg },   // Agg = One (C92 occurrence) | Max (sort) | Sum | Count
           Linear { terms: Vec<(Weight /*halves, signed*/, Val)>,
                    nothing: Zero | Lacked,               // the ONE real difference: sum() vs total
                    when: Option<(Node, Val)> },          // the not_on guard: 27 table lines (grep -c not_on)
           Reading { kind: Count | Least, of: Vec<Val> } }
enum Got { Value(Exact), Lacked, Open { shown: Option<Exact>, range: Option<(Exact,Exact)> } }
fn value(&Val, &Held) -> Got;  fn compare(Got, &NumTest) -> Outcome;   // one evaluator
names: name -> (Val as language text, site id, provenance, limit)          // the reviewed table
```

**What collapses:**

- `ValueRef` (4 variants), `SortKey` (4), and `Atom::{Number, Sum, Pseudo}` with `BProbe::Value`
- four result types: `Scalar` (eval.rs:1524), `Valued` (pseudo.rs:485), the `(Exact, bool)` from `eval::sum`, and the socket interval
- `pseudo.rs` 485–855, about 370 lines of total, count and least evaluation
- `TotalsTable::parse`, 250 lines (totals.rs:482-735), and its 80-line error enum: a definition would be parsed and bound by the language's own reader
- the weighted refusals in `translate.rs`

The estimate is 700–1,000 production lines. **I**, from the ranges above.

**Why the edge cases go with it.** The incomplete, lacked and interval rules are said once, in `compare`. C93's three values come from `Got`.

**The smallest change.**

- `pseudo.total_fire_res + pseudo.total_cold_res >= 60` goes from a parse error to an answer.
- In the sitting set, 14 site searches refused with "different weights" would translate (tally below). **V**
- No existing answer changes, provided the totals rewritten as expressions reproduce the rows. That is held by `tests/pseudo.rs`, `tests/ranged.rs` and `tests/trade_oracle.rs`.

**What makes the kinds really different.** Only three things, and none is a kind:

1. **What nothing is worth.** A total of nothing, including lines that cancel, is lacked; a `sum` of nothing is zero. This is a field of `Linear`.
2. **"Reviewed".** It is provenance of a *name*, not a property of the arithmetic. C94's "nothing else defines a total" survives as "only the table names a total".
3. **The site's rounding** (life regeneration). It is a printed limit, as it is now.

Readings and derived fields really are non-linear, and they stay.

**Rulings touched:**

- C92 holds.
- C94 is reworded: a total is a named expression in the table.
- C95 holds: per-item arithmetic is not cross-item aggregation.
- C109: depth counts nested values.
- `decisions/trade.md`'s "Not doing" line on #316 is dissolved.
- #46 is parked no longer.

**Costs and gains.** Security: none. Performance: the same. Function: gains "best by two values". Agent's ease: gains, because the definition `--describe` prints becomes typeable text.

**Strongest objection.** The `when` guard (category-conditional rows) puts an item predicate inside a value. Keeping it table-only is a wart. A second objection: user arithmetic invites asking for `*`. Refuse it: weights are constants.

**Do this before #316 is built as a third special case.**

**Thesis 2: Stop sitting for exactness; the measure that matters is the stake on the user's store, and the answer mostly lacks it.**

I ran 467 of the 1,255 sitting searches as `--trade-file` against the copy (`<scratch>/search-kernel/tally.py`). They were rounds R to j (327 runs) plus every third search of rounds m to zg (140 runs). Beast rounds k and l were excluded. **V**

| Measure | Result |
|---|---|
| Fidelity | 229 exact, 217 ambiguous, 21 partial |
| Ambiguous clauses | 1,236 |
| …carrying no other reading (C119), so no stake at all | 1,114 (90%) |
| …carrying a stake | 122, every one "adds 0, drops 0" |
| Commonest ambiguity | `stat.text.local` (563), then `stat.text.forms` (434) |

The #46 issue records the same: "No answer differed."

**What goes:**

- further rounds for ambiguity rows (the last three rounds settled bounds of 0 and abyssal sockets)
- the round recomposition in `trade-sheet.py --verify`, about 2,500 lines per the scout
- "ambiguous" as a bare label

In its place: a stake for the local twin and for forms (C119's existing machinery, extended). That lets go of those two rows' sittings, and of an "ambiguous" status wherever the stake is 0.

**Smallest change.** A pasted URL bounding `#% increased Attack Speed (Local)` with no category filter would say "adds N rings" instead of a caveat.

**Invariants.** None touched. C79 and C120 are unchanged; fewer human clicks are needed.

**The oracle.** It *is* the project's best verification asset: an independent implementation on real data. Keep `tests/trade_oracle.rs`.

**Objection.** These sittings are probes the owner composed, not players' searches, and a single store has few twins. A zero stake here is weak evidence for other stores.

**Thesis 3 (must stay): no SQL surface. The typed read for agents already exists.**

- C104 already accepts and returns the tree as JSON on the MCP (`MCP-REFERENCE.md:195`). **V**
- SQL over lines would re-decide C93 in every query an agent writes, a second semantics against C46. It would also lose routes, the per-term counts and the translation's target.
- The agent's cost is the vocabulary (13.9 KB of names), which SQL leaves untouched.
- What would go: parse, print and json, about 2.3k lines. Not worth it.

**Objection.** Agents write SQL fluently and will reach for it. The answer is player names (below), not a second door.

**A corollary of thesis 1.** Fields and totals become one namespace and the `pseudo.` prefix goes (`is_computed`, `HasComputed`, the `Test` versus `Compare` split). That is the cure for `life>=90`: name the total `life`. This touches the owner's naming of `pseudo`, so it is his to rule.

### 3. What I would defend

The four counts per term, lacked apart from undecided, even though undecided was 0 in every query. C93's own Why predicts exactly this, and the lacked count (40,069 for `pseudo.total_life`) is what tells a player the term's denominator.

### 4. Where the old way is right

- C48: no raw SQL and no cache.
- The catalogue's row-template design.
- The oracle as the source of the totals.
- `include_str!` tables cited in the basis.

### 5. What I did not check

- 788 sitting searches were not run: beast rounds k and l (474), and the m–zg searches outside the one-in-three sample.
- Players' real URLs.
- The cost of first use of the 1.35 MB stats table.
- Whether `TotalsTable` checks map one to one onto binder errors.
- Line by line: `counts.rs`, `group.rs` and `describe.rs`.

### Where I part from the scouts

- **search-trade H1.** Agreed, but `k·value | value+value` is too wide. The type is linear over line sums with a "what nothing is worth" flag; non-linear values stay fields. The scout also missed that #46 already ships as the 45 weight −1 rows, and that the code cost is the four evaluators, not the tree.
- **search-trade H3.** Agreed on proportion. The number to cite is a measured 0 nonzero stakes, not 64% exact.
- **design-theses T3.** I part. The form is not a second language. It is a one-way input spelling, and every answer, saved search and route carries the language's text, so the language is already the model. Thesis 1 makes the outbound translation nearly a bijection for stat sums, which is what morning 1's handoff needs.
- **code-craft.** Agreed: split `answer.rs`, and move the quoting out of the library.

### 6. Small finds

- `bind.rs:530-556`: near names are cut to 3 after an alphabetical sort within a score, so `life` never offers `pseudo.total_life`.
- `line("T")>=25` is refused on a one-number template that its shorthand `"T">=25` accepts.
- `--sort` takes no combination of values, so a player has no way to ask for "best".
- 90% of ambiguous clauses carry no stake (C119 is unfinished for `stat.text.local` and `stat.text.forms`).
- `answer.rs:3938-4222`: shell quoting in a library reads environment variables.
- The `describe computed` rows repeat the site id 82 times, about 3 KB.
- There are four value-result types for one notion: `eval.rs:1524`, `pseudo.rs:485`, `sockets.rs:161`, `places.rs:253`.
