### C90 — A line's identity is (realm, kind,
- class: design — an agent and a user writing a query are told what names a line: its realm, its kind and its displayed template, never a stat id
- clauses:
  1. P · Y · crates/acquisition-search/tests/counts.rs:649-658 lists one template once per realm and routes each row to its realm alone. crates/acquisition-search/tests/derive.rs:96-115 keeps a line's source and flags beside its template. tests/rows.rs:159 names a mod by its template text
  2. P · Y · derive.rs:121-122 asserts the sign is carried in the number (`#% to Chaos Resistance`, -27). derive.rs:358-372 asserts markup is reduced to its display half (brackets, styled braces, `[X::20|Display]`) and that a thousands comma belongs to its number
  3. P · Y · derive.rs:147-152 asserts the slots are ordered arg1/arg2 and the numbers are kept whole. derive.rs:170-174 keeps `(#-#)` as four ordered numbers. An average exists only as `avg` on a ranged line
  4. P · Y · derive.rs:96-115 asserts the source array (explicit/implicit) and the flags (fractured/crafted) per occurrence
  5. P · N · I grepped search tests and src for rename and for one stat under two spellings, and found no test that asserts two texts are two lines. The clause follows only from 1 and 2
  6. M · R · C99 (cited in the clause)
- unheld: 5 simple · para · derive.rs's `item()` helper: derive two bodies with an old and a new text of one mod and assert two templates, or a query on one that finds only its own
- reason: Cite · crates/acquisition-search/src/derive.rs:1, :41, :56; src/defences.rs:37 (stat id); src/lib.rs:211 · moves to crates/acquisition-search/tests/derive.rs:91
- fate: NT · simple
- confidence: medium · I did not read derive.rs's `ring()` body (lines 42-64) to confirm the sign source text. I did not read the proptest at derive.rs:840 or trade_stats.rs past its doc

### C91 — A query is one typed tree
- class: design — a human and an agent hand each other one composable, serializable query value that says how it was understood
- clauses:
  1. P · Y · crates/acquisition-search/tests/language.rs:150-168 asserts a parsed tree survives JSON and the wire form, over every corpus case and over generated trees (language.rs:671-679). src/tree.rs:1-4: the tree carries no account or realm, so it is portable. The "to the site" half restates C99/C113
  2. P · Y · answer.rs:338-341 asserts holds( ) with a lower bound and an upper bound (`<=1`) under undecided. answer.rs:945-958 composes place (league) under or/not. rows.rs:350ff composes line alternatives in a group. language.toml:16-131 covers every composition construct. language.rs:671 round-trips generated trees over every node kind
  3. P · Y · language.toml:61-64 (`composition.refinement`) round-trips the parenthesised old query plus a term, through language.rs:185. src/answer.rs:3272 builds a count's route the same way
  4. P · Y · language.rs:179-181 asserts an ambiguity's kind and its readings exactly (language.toml:68-77, `mixed_and_or`). language.rs:227-241 asserts every reading offered parses. answer.rs:42 asserts the answer prints the canonical query back
- unheld: none
- reason: Prop · crates/acquisition-search/src/tree.rs:1 states the property; neither the C++ app nor the site composes, and the "say how it was understood" reason is not stated · moves to crates/acquisition-search/src/tree.rs:1
- fate: RM
- confidence: high · I did not read rows.rs:350's body past its doc, or against.rs:198

### C104 — The text is carried on every
- class: design — every frontend and agent can exchange a query as text or as a tree, and every answer carries both
- clauses:
  1. P · Y · crates/acquisition-search/tests/answer.rs:42-46 asserts the answer's `query.text` and `query.tree`. Every frontend serializes this one Answer type (shared crate). MCP-REFERENCE.md:195 is held by the mcp_reference golden
  2. P · Y · answer.rs:737-746 sends the answer's tree back as a request and asserts the same answer. depth.rs:98-105 does the same at the depth limit
  3. P · Y · language.rs:150-168 (parse/print fixed point, to_json/from_json, wire) over the corpus; language.rs:671-679 over generated trees; language.rs:311-329 (worked tree to text and back)
  4. P · Y · the one `QueryInput` (src/answer.rs:505, text | tree | trade, deny_unknown_fields) is what each frontend sends. answer.rs:737-760 shows a tree accepted and both returned. verbs/tests/documents.rs:4400 stores the text canonical
- unheld: none
- reason: Prop · crates/acquisition-search/src/answer.rs:504 states the property; the reason (reviewers wrote text, a refinement is the previous query plus a term) is not stated · moves to crates/acquisition-search/tests/language.rs:1
- fate: RM
- confidence: high · I did not read the CLI's search_json tests for the text on the CLI surface; I took it from the shared type

### C109 — A query nests at most 32
- class: design — an agent or user is told how deep a query may nest and gets a structured refusal past it, never an aborted process
- clauses:
  1. P · Y · crates/acquisition-search/tests/depth.rs:82-110 asserts every nesting shape answers at 32 (as text and sent back as a tree). depth.rs:127-140 covers a JSON tree at 32
  2. P · Y · depth.rs:72-78 asserts kind `too_deep` and "at most 32 deep" at 33, at 10,000 (:114-120) and for a tree at 33 and 120 (:137-140). crates/acquisition-mcp/tests/search_tools.rs:243 refuses at 400 and the server answers the next call
  3. P · Y · depth.rs:38-61: parentheses, not, not of a group, and inside or, holds( ), undecided( ), each also inside line( ). Each must answer at 32 and be refused at 33
  4. P · P · depth.rs:147-164 asserts that line( ) and sum( ) (and a named total) cost no level. `linked( )` is never asked, so a linked( ) counted as a level would pass
- unheld: 4 simple · para · add a `linked( … )` leaf to the list in depth.rs:150 (`c109_a_line_or_a_sum_is_a_term_and_never_a_level`)
- reason: Y · crates/acquisition-search/src/tree.rs:492-509 states the limit, what a level is, the stack abort in a resident process and why 32 · moves to crates/acquisition-search/tests/depth.rs:1
- fate: NT · simple
- confidence: high · I did not read parse.rs's or check's counting of a JSON tree beyond the doc

### C92 — A comparison on a line binds
- class: design — a query writer is told that a line comparison means one occurrence, that a total is asked for by name, and how slots, sorting and the together count read
- clauses:
  1. P · Y · crates/acquisition-search/tests/answer.rs:97-100: r2 holds 20 and 75 and fails `arg1>=90`. generated_cross_checks.rs:698-705: a term on a line does not move when every occurrence is written twice
  2. P · Y · held with 1 and 3: no comparison sums implicitly (answer.rs:97-100); a sum is asked by `sum( )` (language.toml:600-603)
  3. P · Y · generated_cross_checks.rs:719-729 asserts a sum doubles when every occurrence is written twice. language.toml:600-603 lowers `sum("T")` to sum of arg1. The named-total half restates C94
  4. P · P · language.rs:292-301 holds the syntax: two groups over one template are never merged. rows.rs:233 shows two rows of one mod conjoined in one group. I found no test where two occurrences each meet one condition of a group and the item must fail, and generated_rows.rs:15-19 excludes the conjoined case from what it asks
  5. P · Y · language.toml:473-498 (`slot_unknown` for low on a one-number line, avg on two pairs, low on `(#-#)`), through language.rs:185. derive.rs:142-178 slots
  6. P · Y · seat_faults.rs:671-688 and :1150-1170 assert `slot_missing` with the slots listed as readings. language.toml:572-580 (comparison), :606-617 (sum), :703-706 (sort)
  7. P · Y · rows.rs:243-249 shows a mod of two rows whole. answer.rs:391-392 shows the sorting occurrence's full text
  8. P · Y · answer.rs:388, 395, 398: r7, with no line, sorts last both ways with status "no satisfying occurrence"
  9. P · Y · answer.rs:100-106: together count 1 for r2 at arg1 >= 90; answer.rs:165 routes it to r2
- unheld: 4 simple · wrong · a case in answer.rs's `worked()` corpus or rows.rs's `stash()`: an item whose fractured life line is 92 and whose crafted one is 20 must fail `line("# to maximum Life" is:crafted arg1>=90)`
- reason: Prop · crates/acquisition-search/src/eval.rs:4-31 states the rules, and src/pseudo.rs:32 states half the reason ("a total has none"); no one comment says an occurrence cannot be recovered from a sum · moves to crates/acquisition-search/tests/answer.rs:358
- fate: NT · simple
- confidence: medium · I did not read rows.rs:350's or :474's bodies, unread.rs:432/558, or ranged.rs beyond its outline

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C90 | design | 5 | 1 | 4/0/1/0/1 | Cite | NT | 1/0/0 | 0/1 | medium |
| C91 | design | 4 | 0 | 4/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C104 | design | 4 | 0 | 4/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C109 | design | 4 | 0 | 3/1/0/0/0 | Y | NT | 1/0/0 | 0/1 | high |
| C92 | design | 9 | 0 | 8/1/0/0/0 | Prop | NT | 1/0/0 | 1/0 | medium |

## What I did not check
- I did not read the "Read first" texts (decisions/process.md C170/C182/C192 and its Retired section, bothy.py's docstring, `git show 51f2e4f9`). I classed from CLASS-BRIEF section 2 alone.
- I did not confirm that the acquisition-search `tests/main.rs` binary runs in every gate stage; I saw only that the modules are declared there.
- I did not read the proptests' generators (language.rs:465-660, derive.rs:808-896) beyond their asserting bodies.
- I did not read the CLI and MCP frontends' search tests for C104 clause 1. I relied on the shared Answer/QueryInput types and the MCP-REFERENCE.md text.

## Noticed
- C90, step 2: clause 1 calls kind part of a line's identity, but the counts vocabulary lists `# to maximum Life` once per realm, with kinds merged (counts.rs:649-655: pc 4, poe2 1). There kind is a sub-bucket and a filter, not an identity coordinate. The entry's text and the code's notion of "a line" differ at the counts surface.
- C90, step 2: *Pinned:* `tests/derive.rs` holds clauses 2-4 only. The realm half of clause 1 is held in tests/counts.rs:626.
- C92, step 2: *Pinned:* names `tests/ranged.rs`, whose module doc is C94's (J7, J29–J31) and never cites C92. C92's holders are tests/answer.rs (358, 97-106), generated_cross_checks.rs, seat_faults.rs (F4) and language.toml's slot cases.
- C109, step 2: the entry and src/tree.rs:494 both say `linked( )` is no level, and depth.rs never asks it.
