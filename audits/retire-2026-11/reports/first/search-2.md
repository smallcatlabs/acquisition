### C93 — Absent is false; undecided is
- class: design — an agent or user asking a search gets an answer that never reports a no where the evidence was unread, with undecided counted apart from known absence; the rule lives in the search crate and every frontend inherits it.
- clauses:
  1. P · Y · crates/acquisition-search/tests/sockets.rs:168,173: `sockets=0` returns only `empty`, and `-has:sockets` returns `ring`, the item that lacks sockets (it is lacked, not matched); unread.rs:110: `plain` matches `-is:crafted`
  2. P · Y · crates/acquisition-search/tests/answer.rs:118-133: undecided_reasons[i] has `unread`, `example`, `hint` for r5 and r6; unread.rs:114-121: `unread` and `problem` are named
  3. P · Y · crates/acquisition-search/tests/answer.rs:94-109: per-term counts keep lacked (r7, [5,1,1,0]) and undecided (r5, [6,0,0,1]) in separate slots; total.matched is separate from total.undecided.count (118-119)
  4. P · Y · crates/acquisition-search/tests/sockets.rs:168 (`sockets=0` does not match the socketless ring); pseudo.rs:197-201 (`pseudo.total_res=0` and `<60` match 0); ranged.rs:498-500 (`<15.5` on a lacked total gives [0,1,1,0])
  5. P · Y · crates/acquisition-search/tests/answer.rs:343-344: a readable miss beside an unread implicit array leaves `life>=96` undecided (0,1), not failed; unread.rs:198 (an unread hybrid is no absence)
  6. P · Y · crates/acquisition-search/tests/unread.rs:86-99: with unread flags, the text and number terms still decide (1,0) and `-line(template:strength)` stays decided; answer.rs:94-109: only the term that needed r5's class or r6's array is undecided
  7. P · Y · crates/acquisition-search/tests/answer.rs:322-330: and/or/not over an undecided term; 118-120: r5 and r6 are undecided at the root, not in `ids`, counted in total.undecided
  8. P · Y · crates/acquisition-search/tests/unread.rs:35-55 (routes_partition: the four kinds matched/failed/lacked/undecided each route to their count, and together they partition the scope; used at 147, 191, 384, 656); answer.rs:94-109 gives exact four-count arrays
- unheld: none
- reason: Prop · crates/acquisition-search/tests/unread.rs:1-3 (property only); src/answer.rs:204 and src/derive.rs:112,119,458 cite it in fragments; the *Why:* (an unknown count nonzero on every query stops being read) appears nowhere in code · moves to crates/acquisition-search/tests/unread.rs:1
- fate: RM
- confidence: high · did not read generated_completion.rs, generated_routes.rs or the many other c93_ tests in rows.rs, ranged.rs and category.rs, which add to the evidence above

### C94 — A named total is a sum
- class: design — an agent asking `pseudo.<name>` gets one defined sum that answers, as a range with low/high/avg where its lines are ranged, and is told when it is incomplete or unavailable.
- clauses:
  1. P · Y · crates/acquisition-search/tests/pseudo.rs:157-160: ring_a's total_res is 124 (40 + 3×20 + 2×12) and is shown; "defined once as reviewed reference data" restates C68
  2. M · Y · pseudo.rs:157-160: the weighted value 124 would break without the row's weights; src/totals.rs:948-962 (the definition prints `3 × arg1 of line(...)`); stated at src/totals.rs:12-18
  3. M · N · no shipped non-ranged total has a slotless row (an awk over reference/totals-v6.toml found none), and no test of `eval::count` through a row (looked in tests/pseudo.rs, tests/ranged.rs and the src/totals.rs unit tests); stated at src/totals.rs:17-18 and in the header of reference/totals-v6.toml:20-21
  4. P · Y · crates/acquisition-search/tests/ranged.rs:265-375: 1118 expected-answer cases, each checked for low/high/avg/bare against the CSV, with the bare name equal to avg; ranged.rs:1158-1180: any slot word except low/high/avg is `slot_unknown`
  5. M · R · C68 (reference tables): it restates clause 1's "defined once"
  6. P · Y · crates/acquisition-search/tests/pseudo.rs:181,190-196: ring_b (unread implicit) is undecided, not a total; pseudo.rs:205-225: poe2 has no definition, so the total is undecided with the reason "no totals table for realm poe2", and `=0` is undecided, not matched
  7. P · Y · crates/acquisition-search/tests/pseudo.rs:658-690 (`describe` lists every computed value with its definition); src/totals.rs:1045 `the_loader_refuses_what_a_reviewer_would` (unit test: a bad table is a TotalsTableError, never a partial table)
- unheld: 3 cut · para · no shipped row is slotless outside the ranged family (totals-v6.toml), so no caller relies on it; it stays in the src/totals.rs:17-18 doc
- reason: Prop · crates/acquisition-search/src/totals.rs:1-3 gives "one meaning on every surface" but not the *Why:* (a total that may decline fails the everyday question) · moves to crates/acquisition-search/tests/pseudo.rs:140 (doc of `c94_a_total_has_three_statuses_and_a_total_of_nothing_is_lacked`)
- fate: RM
- confidence: medium · read the ranged expected-answers loop but not its CSV, so slot-wise summing over several lines is taken from the CSV having 1118 cases (not counted per case); did not read pseudo.rs:900-1240 or the rest of ranged.rs

### C95 — The counts view has two shapes
- class: design — an agent asking for counts gets independent facets or one crossed table of two keys, with at most one sum, and gets an error for anything else.
- clauses:
  1. P · Y · crates/acquisition-search/src/answer.rs:538-543: the `Shape` enum is one of Rows/Counts/Cross/Explain, so two views are unwritable; `Counts` (615) is `deny_unknown_fields` with one optional `sum`; tests/counts.rs:272-310 (three keys give three tables); 682 (crossed table)
  2. P · Y · the same type: `sum` is one string bound as one number per item (src/bind.rs:921); tests/counts.rs:825-837 refuses a non-number (`operator_mismatch`) and a raw projection; anything more would need a product change to the type, not a respelling
  3. P · Y · crates/acquisition-search/tests/counts.rs:272-310 (independent facets); counts.rs:821-824: cross [tab] gives "two keys" and cross [tab,line] gives "two fields", both `view` errors
  4. P · Y · crates/acquisition-search/tests/counts.rs:459-484: the stack sum has `lacking: 1` and adds nothing; the unread item gives `incomplete: true, unread: 1`; 498-502: an item sum of nothing is 0, lacking 0
  5. P · Y · crates/acquisition-search/tests/counts.rs:829-832: a raw `line(...).arg1` sum is a `view` error offering `sum(...)`; combining views, sorting counts and a standalone sum are unwritable at the crate's type (answer.rs:538, 615), and the CLI refuses each at crates/acquisition-cli/tests/search_json.rs:939-944 (`--sum` alone, `--count --sort`, `--count --cross`, all `view`)
- unheld: none
- reason: Prop · crates/acquisition-search/src/counts.rs:11-24 states the property in full; of the *Why:* only "never an implicit choice of its largest occurrence" (18) is there, not that counts and totals are what is asked for · moves to crates/acquisition-search/tests/counts.rs:1
- fate: RM
- confidence: high · did not read counts.rs:682-747 (the crossed-table test) or the MCP tool's mapping to `Shape`

### C105 — A count accounts for every
- class: design — an agent reading a count sees every match in some bucket: a value, `none` or `undecided`, with a one-value key's buckets summing to the total.
- clauses:
  1. P · Y · crates/acquisition-search/tests/counts.rs:345-347 (the rarity buckets sum to 14 = total.matched), with each bucket's members followed by its route (209-224); 1298-1300 (class: 10)
  2. P · Y · crates/acquisition-search/tests/counts.rs:328-334: the gem and currency are under `none`, term `-has:rarity`
  3. P · Y · crates/acquisition-search/tests/counts.rs:336-344: r3 is in `undecided(rarity)` and not in a value; the tally is exactly `[{unread, items}]`, so a reason field added there would fail
  4. P · Y · crates/acquisition-search/tests/counts.rs:518-558: a multi-value `line:resist` key over 6 matches puts every match in a value, `none` (belt) or `undecided` (amu, blank), with members routed; tests/generated_cross_checks.rs:44 (proptest, by category and class; not read)
  5. P · Y · crates/acquisition-search/tests/counts.rs:345-347 and 1298-1300: the buckets of a one-value key sum exactly to total.matched
- unheld: none
- reason: Prop · crates/acquisition-search/src/counts.rs:25-27 and 33-44 state the property and the two invariants, not the *Why:* (inventing a value or dropping an item breaks C93 or the total) · moves to crates/acquisition-search/tests/counts.rs:310 (doc of `c105_a_count_accounts_for_every_matching_item_and_sums_to_the_total`)
- fate: RM
- confidence: high · did not read the generated_cross_checks.rs body or the category.rs/class.rs C105 tests

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C93 | design | 8 | 0 | 8/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C94 | design | 4 | 3 | 5/0/1/0/1 | Prop | RM | 0/0/1 | 0/0 | medium |
| C95 | design | 5 | 0 | 5/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C105 | design | 5 | 0 | 5/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |

## What I did not check
- Proptest suites (generated_completion.rs, generated_routes.rs, generated_cross_checks.rs, generated_rows.rs): read only their headers or names.
- The 1118-row ranged-expected.csv: I did not count how many cases sum several ranged lines, so C94 clause 4's slot-wise summing rests on the loop's exact low/high comparison over that CSV.
- How the MCP frontend builds a counts `Shape`. I relied on the shared type making combined views and a standalone sum unwritable.
- Whether every frontend's describe output shows a total's definition. I read only the search crate's `describe` test.

## Noticed
- C95 step 2: the entry says "there is no grand total", but the counts view answers with a view-wide `counts.sum` (`{"name":"stack","value":55,...}`, crates/acquisition-search/tests/counts.rs:461). src/counts.rs:15 says "no standalone grand total", which is narrower. Read literally, the entry's words are wider than the code.
- C94 step 2: clause 3 (a slotless row counts the line) is a table semantics that no shipped non-ranged total uses and no test exercises. The loader accepts it (src/totals.rs:603) and the evaluator implements it (src/pseudo.rs:740), so it is unheld, live code.
- C94 step 1: clause 1 mixes a promise ("a sum that answers") with a restatement of C68 ("defined once as reviewed reference data"). Splitting by clause number makes the P/M call lossy; I took it as P.
