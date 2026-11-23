2576d17f^..911ed6eb — the brief’s implementation done-when holds by inspection, with two P3 wording findings.

- **P3, shape 1 — [vocabulary.rs:371](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/vocabulary.rs:371):** `2576d17f`’s message counts 14,920 stats, but this loop visits `(category, id)` pairs, keeping multiple texts together. The fixed table distinguishes 14,700 pairs from 14,920 captured text entries. Correct the denominator in a follow-up commit message using the iterator’s unit.
- **P3, shape 1 — [vocabulary.rs:268](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/vocabulary.rs:268):** “Once per entry in a process” exceeds the cache’s guarantee. Two concurrent client requests for `stats?values=true&q=item+level` can both miss the cache before either inserts its result and therefore translate the same entry twice. Describe results as cached after computation, updating the sibling claims in the module doc, `values` doc and commit record.

- Execution-dependent done-when checks—browser behavior, tests, fixture regeneration and gate status—**not verified**, because execution was prohibited.
- Both findings are wording nitpicks; neither establishes an incorrect search result.
- Brief: no impediment encountered. Plan: no change suggested. Prompt: no change suggested.
