### C102 — The digest's limits register is inherited
- class: design — an agent reading an answer is told, in the answer, what the search cannot know or do
- clauses:
  1. P · P · `crates/acquisition-cli/tests/goldens/search_json/describe_at_a_terminal.txt:328-334` (test `search_json.rs:1344`) holds the six limits `describe.rs` LIMITS prints, word for word. Nothing compares that list to the register. The register's S14 (a map's area) and S67 (a trade number left undecided) are printed nowhere in `acquisition-search/src`, so "inherited" holds for six of nine, not "all but one".
  2. M · N · nothing compares LIMITS to `search/DIGEST.md`. S53's printed words (`describe.rs:200`) are not the register's (`DIGEST.md:232`). The golden holds the printed words, not where they came from.
  3. P · N · looked in `tests/pseudo.rs`, `src/totals.rs` and `tests/acceptance.rs`: no case puts a reworded template under a shipped total and asserts the item lacks the total.
  4. P · U · this clause says the search stays silent about something; no test can fail on a capability that was never built.
  5. P · Y · `crates/acquisition-search/tests/percentile.rs:1386` asserts the percentile field's `describe` text contains "inexact" (it also states it at `pseudo.rs:399`).
  6. M · R · C106 (no judgment)
  7. P · P · Fuzzy matching is held: `acceptance.rs:664-679` asserts a template nothing carries matches 0, gets suggestions and the S107 wording. Fetching and acting on the stash are held: `tools/arch-check.sh:486-487` forbids the search from linking the daemon, the client, reqwest or tokio. Judging legacy is R (C107). Valuing is held by one answer only: `acceptance.rs:1193-1240` checks that a single priced item shows no worth. Recovering the mod is held only by S52's wording (`acceptance.rs:887-893`). The valuing and mod-recovery halves are the loose ones.
- unheld: 1 complex · wrong · the clause's meaning is unclear: the register holds limits the build does not print and words it changed, so either a check against DIGEST.md or a rewording of the entry has to come first
  2 cut · para · `describe.rs:179` doc states it; no frontend or caller reads DIGEST.md; the golden holds the words that are printed
  3 simple · para · `tests/pseudo.rs` fixture table: an item carrying a reworded line lacks the total
  7 simple · wrong · `tests/acceptance.rs` `oq6_`/`oq2_` printed-text checks: `describe(["all"])` names no field or term of worth or of a mod's tier or affix
- reason: Prop · crates/acquisition-search/src/describe.rs:179 · moves to crates/acquisition-search/tests/acceptance.rs:1
- fate: NT · complex
- confidence: medium · I did not read the whole golden, `acceptance.rs` past what I quoted, or `answer.rs:356`

### C107 — Legacy is the user's word; a
- class: design — no field, term or output of the search calls an item legacy
- clauses:
  1. P · P · `crates/acquisition-search/tests/acceptance.rs:333-337`: the describe text may say "legacy is" only as the S178 limit, and one item's `show` must not say "legacy". A field or term named legacy would pass unless its description happened to contain "legacy is".
  2. P · U · this clause allows a future field; the variant-label field is parked (`src/lib.rs:311-312`)
  3. P · P · same lines: only one answer, one `show` and the describe text are checked. Other items' answers, term names and field names are not.
  4. P · U · unbuilt; the variant-label field is parked
  5. P · U · unbuilt, because clause 4 is
- unheld: 1, 3 simple · wrong · `tests/acceptance.rs` `oq2_a_unique_by_name_its_place_and_every_line_through_show`: assert that `describe(["all"])`'s field and term names, and an answer over the whole fixture, say no "legacy" outside the S178 limit
- reason: Y · crates/acquisition-search/src/lib.rs:310 · moves to crates/acquisition-search/tests/acceptance.rs:266
- fate: NT · simple
- confidence: medium · I did not read `src/bind.rs`'s field list whole

### C117 — GGG's facts about a tab are
- class: design — an agent can search GGG's tab flags as fields, and the tool never claims what the site lists
- clauses:
  1. P · P · `crates/acquisition-search/tests/seat_faults.rs:1683-1692` holds that the facts are fields. "Says nothing of what the site lists" is held only at `:1712-1720`, which checks that three `describe fields` entries do not contain "site". An answer, a count or another output that says it would pass.
  2. P · Y · `seat_faults.rs:1683-1692` asserts `tab.public`, `tab.remove_only` and `price.side` each as true, false and lacked: a substash takes its tab's value, a character's item lacks it, no price means no side. `crates/acquisition-cli/tests/seat_faults.rs:3376-3383` runs `tab.remove_only` end to end.
  3. P · U · a rule on the agent's own inference; no output of the tool to fail
- unheld: 1 simple · wrong · search `tests/seat_faults.rs` `ggg_s_facts_about_a_tab_and_a_prices_side_are_fields`: assert the answers and the count it already builds carry no word of the site or of listing
- reason: Y · crates/acquisition-search/tests/seat_faults.rs:1576 · moves to crates/acquisition-search/tests/seat_faults.rs:1576
- fate: NT · simple
- confidence: high · I did not read `src/eval.rs` around its C117 citations

### C118 — An answer says how old its
- class: design — the shape of an answer: how old each row, the matches and the scope are, and what was never fetched
- clauses:
  1. P · Y · held by clauses 2–5 together, cited below
  2. P · Y · `crates/acquisition-search/tests/seat_faults.rs:1478-1491` asserts each row's `place.fetched_at` is its location's (a tab's, a page's own, a character's), and `show`'s too; `acquisition-mcp/tests/search_tools.rs:1175` holds it at the MCP
  3. P · Y · `crates/acquisition-search/tests/answer.rs:1225-1250` asserts that `total` carries places, oldest and newest fetch, their locations and `list_seen` with ages, on rows, limited rows, counts and cross views (the C162 tests)
  4. P · Y · `answer.rs:1250-1257` asserts `scope.oldest_fetch` is the older location that holds no match, kept apart from `total`'s
  5. P · Y · `seat_faults.rs:1563-1570` asserts `never_fetched_by_type`, most never fetched first, characters as a type. `crates/acquisition-cli/tests/seat_faults.rs:1683-1690` has the transcript facts "MapStash 3 of 3" and "2 more in 2 other types". The pointer to `tabs` is held at `acquisition-mcp/tests/search_tools.rs:1206-1214` and `cli seat_faults.rs:3578-3604` (the `locations` view is gone).
  6. P · U · deferred "until a seat asks"; nothing can fail on it
- unheld: none
- reason: Cite · crates/acquisition-search/tests/seat_faults.rs:1417, crates/acquisition-search/tests/answer.rs:1215 · moves to crates/acquisition-search/tests/answer.rs:1215
- fate: NT · none (derived by "Otherwise": the P clauses are Y and one U, so neither rule 2 nor rule 3 applies)
- confidence: medium · I did not read the CLI golden that holds the scope line whole, so I did not see the CLI line itself point at `tabs`

### C125 — A table drawn from the game's
- class: check — the subject is `game-diff.py --check` and the gate's trade stage: what refuses a table drifted from its pin
- clauses:
  1. P · Y · `tools/trade-check.sh:63` runs the `drift` line. `tools/game-diff.py:309-313` refuses a clone that is not at the pin, staged by the self-test (`:422`) and by `tools/trade-check-breakers.sh:104`. "Never typed" is held, where a clone is present, by trade-check's clone tables (check 9, the retired C198). `crates/acquisition-search/tests/class.rs:105-106` asserts the shipped table names `52980e52` and game `3.30.0.2.2`.
  2. M · P · the self-test stages a clone off PINS (`game-diff.py:422`), a header's game not the clone's (`:427`) and a lagging export (`:435`). A header whose pin is not the clone's (`:316-317`) is staged nowhere, neither in the self-test nor in `trade-check-breakers.sh`.
  3. P · Y · `game-diff.py:418`: with no clone the check passes, and the line says "no poe1 clone"
  4. P · P · a generator off the pin is refused (`trade-check-breakers.sh:104`). "In the same commit" rests on the drift line's header-pin refusal, which nothing stages. The bump order is a step people follow.
- unheld: 2 simple · para · `tools/game-diff.py` `self_test` drift cases: a header naming another pin fails
  4 simple · para · the same case
- reason: Prop · tools/game-diff.py:3 · moves to tools/game-diff.py:30
- fate: NT · simple
- confidence: medium · I did not read `tools/sources.py`'s `require` or the generators' own refusals, only the breaker row that stages them

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C102 | design | 5 | 2 | 1/2/2/1/1 | Prop | NT | 2/1/1 | 2/1 | medium |
| C107 | design | 5 | 0 | 0/2/0/3/0 | Y | NT | 2/0/0 | 2/0 | medium |
| C117 | design | 3 | 0 | 1/1/0/1/0 | Y | NT | 1/0/0 | 1/0 | high |
| C118 | design | 6 | 0 | 5/0/0/1/0 | Cite | NT | 0/0/0 | 0/0 | medium |
| C125 | check | 3 | 1 | 2/2/0/0/0 | Prop | NT | 2/0/0 | 0/2 | medium |

## What I did not check
- The CLI goldens for the C118 scope line, and for C102's limits past lines 328-334.
- `tools/sources.py`'s `require` and each generator's refusal off the pin (C125, clause 4). I relied on the breaker row.
- Whether an answer anywhere uses the word "legacy" outside `show` and `describe` (C107).
- The trade translation's own treatment of S67 (C102, clause 1): I searched only `acquisition-search/src`.

## Noticed
- C102, step 2: the entry says the register is inherited "with one exception", but `describe.rs` LIMITS prints six of the register's nine limits. S14 and S67 are printed nowhere, besides S111, which is the named exception. S53's printed words differ from `search/DIGEST.md:232`, so "the register's wording" is not what the code does.
- C118, step 2: *Pinned:* calls the subject "Unbuilt" (#155, #156), but the subject is built and held: `total` carries the matches' fetch span (`tests/answer.rs:1142-1257`, the C162 tests).
- C118, step 5 (fate): a session's entry whose P clauses are all Y except one U (a deferral) fits neither rule 2 nor rule 3, and falls to rule 4's "Otherwise" as NT with nothing to build. The rules do not decide such an entry. C102's clause 4 and C117's clause 3 are the same shape of U.
- C125, step 2: the drift line's refusal of a header naming another pin (`game-diff.py:316-317`) is never staged, although `trade-check-breakers.sh:8` says the drift line's refusals are staged by the self-test.
- C117, step 4: the reason sits at a test's doc (`seat_faults.rs:1576`), not at product code. If the entry retired, that test doc would be its home.
