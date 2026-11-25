# The search's kernel, its tables and the trade loop

The plan's D6 and E7. Investigator's report: `reports/investigators/search-kernel.md`. Portfolio: proposals 7 and 8.

## What was found

| Class | Modules | Production lines |
| --- | --- | --- |
| Kernel (the language) | tree, parse, print, json, bind, eval, group, exact, template, error, values | 8,525 |
| Derivation (an item from its body) | derive, sockets, properties, corpus, places, price, tabs | 5,692 |
| Tables (data about the game) | class, category, defences, totals, pseudo | 5,233 |
| Answer | answer, counts, describe, show, lib | 8,100 |
| Trade (translation from the site's query) | trade.rs, trade/* | 5,540 |

The language does well on real questions (`has:note`, gems by level and quality, unpriced uniques, maps by tier). What an agent pays to learn is vocabulary, not syntax: the grammar is about 3 KB of the 34.6 KB description; the computed values are 13.9 KB, each row restating a trade-site id.

**There is one value expression and the code has four.** A total is already a weighted sum of line sums and prints as one. Forty-five table rows carry weight −1, which is #46's request shipped inside the table. Three of the five reasons the translation refuses a site's weighted sum exist only because the language has no weights.

**The trade loop is not a side feature**, but its precision has outrun its stake. Of 467 sitting searches run against the owner's store, 229 were exact, 217 ambiguous, 21 partial; 1,114 of 1,236 ambiguous clauses carry no other reading, and the 122 that do each change the answer by nothing.

## Checked by the orchestrator

- `--sort 'pseudo.total_life + pseudo.total_ele_res'` fails with "expected the value to end, found `+`". A player cannot ask for "best by two things".
- `grep -c "weight = -1" crates/acquisition-search/reference/totals-v6.toml` prints 45.
- `--describe pseudo.total_life` prints the total as `1 × arg1 of line(…) + 0.5 × arg1 of line(…) …`.
- `line("#% increased Movement Speed")>=25` is refused with both right spellings offered; the bare-template form answers 609.
- Not re-run: the 467-search tally (`<scratch>/search-kernel/tally.py`), nor the 788 sitting searches it left out.

## Where this parts from the plan

- The search scout's value grammar (`k·value | value+value`) is too wide. The type is linear over line sums, with one flag for what nothing is worth (a total of nothing is lacked; a sum of nothing is zero). Readings, percentiles and the q20 defences are not linear and stay fields.
- The number to cite for the trade loop is not "64% exact" but "no answer differed".
- A read-only SQL view for agents was the orchestrator's own question. It dies. SQL over lines would re-decide the three-valued logic in every query an agent writes, which is a second semantics; and what costs the agent is the vocabulary, which SQL leaves as it is. The cure is a player's names.

## Position

Build the one value expression before #316 becomes a third special case. It collapses four result types, the totals table's own parser and its error enum, and about 370 lines of `pseudo.rs`: 700 to 1,000 lines. Fields and totals become one namespace, the `pseudo.` prefix goes, and `life>=90` binds. No existing answer changes if the totals rewritten as expressions reproduce their rows, which three existing test files hold.

Stop sitting for exactness. Print the stake for the two commonest ambiguities and call a clause ambiguous only where the stake is not zero. The site's answers remain the project's best oracle.

The four counts per term stay, lacked apart from undecided, though undecided was 0 in every query run: the lacked count is the term's denominator.

`answer.rs` should be four modules, and quoting for four shells does not belong in a library that reads the environment.

## Route

Proposals 7 and 8. The `pseudo.` prefix is the owner's naming. Issues drafted: `near-names-miss-the-total`, `basis-against-omits-properties`.

## After the skeptic

**The value expression is narrowed** (`dissent/theses-6-10.md`, thesis 7). The orchestrator's summary in `THESES.md` was wider than the investigator's type. One occurrence, the largest occurrence and the sum are three different things on an item with two life lines of 40 and 25 (`tests/rows.rs:300`), and C92 keeps them apart; the count and least readings carry an interval and an absence that one flag does not express. So: linear arithmetic over the values the language already has, with their semantics untouched. Dropping the `pseudo.` prefix would break saved query text, which is a durable surface (C104, C194); a player's names can be added, the old ones cannot go.

**"Stop sitting for exactness" dies** (thesis 8). A zero stake on one store is not an exact translation. A clause is ambiguous because of what it means, and the warning is for the inventory that would show the difference, which another account or the next refresh may hold. The tally left out the beast rounds and two thirds of the later ones, and the orchestrator did not re-run it. What stands is narrower and is a fact about proportion, not semantics: in the sample, on the owner's store, no answer differed.
