# The test read — search

322 rows of `data/inventory-search.tsv`, one row each in `data/read-search.tsv`, in the inventory's order; 35 rows in `data/cases-search.tsv`.
Classes: boundary 310 · mechanism 5 · harness 4 · cannot-fail 1 · weak 1 · duplicate 1.
Confidence: high 275 · medium 47 · low 0. Every medium row says why in its `note`.
Nothing was run: every answer is from the source, and a case count from the line that sets it.
Not read line by line: the generators of `tests/common/generated.rs` (lines 73–984, 1100–2424); what the six generated properties reach is their own fixed cases' and docs' word.
Five fixtures were skimmed, their asserts read: `seat_faults.rs:897, 984, 1046`, `sockets.rs:526, 603` (medium).

## cannot-fail, weak, duplicate

- **cannot-fail** `tests/language.rs:245` `every_construct_of_the_reference_has_a_case_and_every_case_a_construct` — compares the test file's own `CONSTRUCTS` list with the test's corpus `tests/language.toml` (`language.rs:35-132, 247-258`); no product code is asked. A harness check in kind: a construct added to the language with no case passes it.
- **weak** `tests/rows.rs:705` `what_a_slot_is_read_beside_costs_the_groups_size` — its bound is 20 s of real time (`rows.rs:736`), and by its own doc a cost that doubles by the group passes, the depth limit holding it near a second (`rows.rs:698-703`); only a cost that quadruples fails.
- **duplicate** `tests/generated_completion.rs:441` `how_often_an_undecided_answer_is_the_same_under_every_completion_tried` — `#[ignore]` (`:440`), so it never runs in the gate; by hand it calls the same `checked` as `:433` over 2000 deterministic cases and prints a tally. A measurement, kept or cut as one.

## mechanism (all five; each also names what holds the rule through an answer)

- `src/pseudo.rs:1852` `a_reading_gives_reasons_only_where_it_is_open` — a hand-built private `Held`, the private `unread_of`, and a case its own doc says no answer asks for (`:1848-1850`). The most disposable.
- `src/sockets.rs:435` `a_comparison_over_an_interval_…` — the private `Counted::truth`, variant by variant; `tests/sockets.rs:157` and `tests/pseudo.rs:1655` hold the interval rule.
- `src/pseudo.rs:1799` `a_half_weight_is_exact_and_a_total_is_never_rounded` — `Held` and `Place` built field by field (`:1739-1772`); `tests/pseudo.rs:145, 885` hold the statuses.
- `src/pseudo.rs:1894` `a_ranged_total_and_a_counting_row` — the same build; `tests/ranged.rs:264, 467` hold the ranged half. The slotless counting row was not found asked anywhere else.
- `src/pseudo.rs:1913` `dps_and_pdps_read_the_displayed_properties` — the same build; `tests/pseudo.rs:241, 296` hold most. The `1e34` and product-of-decimals reasons (`:1966-1979`) are asked only here: cut it and they are held by `src/exact.rs:331, 400` alone.

## Read beside the classes

- **Subsumed, not duplicate:** `src/class.rs:735` fails on nothing `tests/class.rs:98` does not; `tests/seat_faults.rs:636` checks three substrings of a sentence `tests/refusal.rs:120` compares whole.
- **A hand count and its pin disagree:** `tests/sockets.rs:379` works a sum as 17, `:383` asserts 18.
- **A half with no case left:** `tests/refusal.rs:48` asserts no construct is refused as not built (`:98`); the four names still on the list are flags, the CLI's to refuse.
- **Wall-clock bounds:** `tests/rows.rs:736, 999`, 20 s of real time each — a loaded machine can turn them red with no product change.
- **Wording pinned by substring or whole** (medium, each noted): the help's and the loaders' texts in `category.rs:1344`, `percentile.rs:1343`, `pseudo.rs:645, 1861`, `ranged.rs:1067`, `counts.rs:778`, `refusal.rs:120`.
- **Unmeasured in the gate:** how many generated cases are refused and so compared only as errors (`generated_equivalence.rs`, `generated_explain.rs`, `next.rs:356`, the cross-checks' `if let Ok`); only the ignored tally prints `erred`.

## The speed lever (`data/cases-search.tsv`)

- Eighteen property tests. In the gate: completion 256, equivalence 256, routes 512, cross-checks 192, rows 4 × 192, explain 48, `next.rs` 48, `derive.rs` 2 × 256, `language.rs` 4 × 2000, `src/bind.rs` 2 × 4000.
- **The knob exists.** `cases()` (`tests/common/generated.rs:1093`) reads `PROPTEST_CASES` for completion, cross-checks, equivalence, explain, rows and `next.rs`. Routes' literal 512, `language.rs`'s `with_cases(2000)` and `bind.rs`'s 4000 do not go through it — but proptest 1.11.0's own macro applies the environment over an explicit config (`sugar.rs:160`, `test_runner/config.rs:86`, read in the vendored source), so `PROPTEST_CASES=N` sets every one of the eighteen to N. Medium confidence: read, never run.
- **So it is one number, not a scale.** Set to 32 it also takes `language.rs` from 2000 to 32 and `bind.rs` from 4000 to 32, the cheap ones. A local/CI split that keeps their proportions needs `cases()` to scale its gate number rather than replace it, and routes, `language.rs` and `bind.rs` to call it. Nothing in `.github`, the root manifest or a `.cargo` config sets the variable today.
- **A case is not one ask.** Completion stores 16 bodies and asks 1 probe, the query, its routes and 14 sorts; a cross-check case builds some nine fixture stores and asks 16 probe-and-sort pairs before the copies; a routes case asks three queries and follows every count; a rows case of `c90_…` builds two stores a query.
- **The tables are the other weight, with no knob:** `category.rs:245` (553 rows × 82 ids × 2 queries), `ranged.rs:264` (598 cases × 21 totals, 7 queries a total a realm), `percentile.rs:366` (4898 items in one store), `ranged.rs:840` (459 cells), `pseudo.rs:1489` (110 queries). Each is a committed table held whole; none is generated.
