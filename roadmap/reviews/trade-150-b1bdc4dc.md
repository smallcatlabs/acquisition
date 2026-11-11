b1bdc4dc — the issue’s done-when does not fully hold.

- **P1 — [translate.rs:1655](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/trade/translate.rs:1655), shape 1:** Replacing the entire landing with `nothing()` discards ambiguity that still affects an `if` group’s presence test. Concrete client input: an `if` filter on `explicit.stat_889691035` with `min:5,max:0`. A persisted item carrying only `4% increased Attack Speed per 25 Dexterity` is excluded by the generalized presence predicate, although the requested id names the per-10 stat; the report nevertheless claims exact fidelity. Preserve presence-related ambiguity and its explanation under `if`, and pin this collision case.

- **P2 — [answer.rs:1367](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:1367), shape 2:** The required note naming the inverted pair disappears before reaching the user. A client trade filter `ilvl:{"min":85,"max":80}` becomes an exact `false()` clause, so `ClauseOut` strips its input, rows and notes; the shared text renderer also excludes exact clauses. Preserve and render inversion explanations, and pin the final answer’s note rather than only the intermediate translation.

- Execution evidence: **not verified**, as requested. Live site behavior and external issue updates were not checked; review used the committed tree and supplied issue.
- Nitpicks for unlikely cases: none.
- Brief/plan/prompt: the supplied issue was sufficient; no changes proposed.
