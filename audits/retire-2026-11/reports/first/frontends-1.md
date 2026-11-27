### C11 — The CLI's output is structured: `--json`
- class: design — an agent at the CLI (and the MCP, for conflicts) is promised one JSON document on stdout, failures included, with stable exit codes.
- clauses:
  1. P · P · `--json` is a clap global (`crates/acquisition-cli/src/cli.rs:36`), so every verb accepts it, and every error goes through one path (`crates/acquisition-cli/src/main.rs:87-100`: under `--json`, `error_report` is printed to stdout and the process exits 1). Not held: nothing runs every verb's success answer under `--json` to check it is a JSON document. A verb that prints text under `--json` would pass.
  2. P · Y · `crates/acquisition-cli/tests/search_json.rs:976-988`: for each of about 30 refused commands, `--json` exits 1 with exactly one document on stdout (`sole_json`), carrying `kind` and a string `error`; without `--json`, stdout is empty and the error goes to stderr. All of it goes through the single path in `main.rs:89`.
  3. P · Y · `crates/acquisition-cli/tests/seat_faults.rs:524-525` and `:556`: a failed fetch job exits 1 with `outcome: failure`, both under `--json` and in text. `:604-614`: a cancelled job exits 1 in both modes, with empty stdout in text mode.
  4. P · Y · `crates/acquisition-cli/tests/seat_faults.rs:197-225`: a command line the parser refuses exits 2 with one document, `kind: "usage"`, and nothing on stderr; text mode keeps clap's own output, and help is not a failure.
  5. P · Y · `crates/acquisition-verbs/tests/documents.rs:2643-2645`: the shared refusal shape gives `kind: conflict` with `revision`. MCP: `crates/acquisition-mcp/tests/plan_loop.rs:86-93` (`conflict`, `revision`, `current.revision`). CLI: `crates/acquisition-cli/tests/seat_faults.rs:4165` and `price_json.rs:532`.
- unheld: 1 complex · para · it needs a new table of every verb with arguments and a world to run in; the reference walk in `cli_reference.rs` only reads help.
- reason: Cite · the id is cited in fragments: `crates/acquisition-verbs/src/refusal.rs:1`, `crates/acquisition-cli/src/terminal.rs:38,274`, `crates/acquisition-cli/src/jobs_cmd.rs:425`. The reason ("the CLI is itself an API") appears only in a test module doc, `crates/acquisition-cli/tests/plan_json.rs:5`. · moves to `crates/acquisition-cli/tests/search_json.rs:922` (the `c11_` test, which has no doc yet)
- fate: NT · complex
- confidence: medium · I did not read `price_story.rs`, `world.rs:937`, or the web page's refusal path.

### C53 — Output has three levels: default text
- class: cross — the readers of every surface are promised a decision view, the bound, ages in words, and text that is a function of the JSON. The bound is held once (`acquisition-store/src/bound.rs`), but each frontend's text and each maker's ages are kept separately. A new frontend (the GUI to come) could print text that the JSON does not determine, or an age with no words beside it, and no existing test would fail.
- clauses:
  1. P · P · `crates/acquisition-cli/src/plan_cmd.rs:2241-2267`: the grouped plan text against its `--expand` form. `crates/acquisition-mcp/tests/pricing_tools.rs:145+`: MCP `expand` equals the CLI's `--expand`. Each is one verb's test; no test walks every verb.
  2. P · P · `crates/acquisition-cli/tests/search_json.rs:676-686`: for search only, the text says every block and count the JSON says. This is checked per maker; nothing makes text a function of the JSON structurally.
  3. M · R · restates C38: the plan envelope is the authorization. `crates/acquisition-plan/src/lib.rs:766` says the same at the code.
  4. P · P · `crates/acquisition-cli/src/price_cmd.rs:1438-1468` (status opens with one answer line and ends with `next:`) and `plan_cmd.rs:2249-2250`. Each is one verb's test; a new maker would go unchecked.
  5. P · Y · `crates/acquisition-cli/tests/web.rs:6714-6720`: eleven saved searches list as ten, with "10 of 11 listed"; the literal 10 is asserted.
  6. P · Y · `crates/acquisition-store/unit/tests/bound.rs:8-45`: the default bound, a `limit` above or below it (zero included), `expand` gives the whole list, and `left_out` is the difference. `crates/acquisition-search/tests/answer.rs:1084-1087`: a continuation's `next.request` is present.
  7. P · P · `crates/acquisition-search/tests/rows.rs:1366` (an item is carried whole) and `surface_faults.rs:608` (a describe section is whole). I did not find a test that a plan's actions or a shop page's text stay whole past ten.
  8. M · N · no check stops a maker from cutting by a number of its own. The rule is stated at the code: `crates/acquisition-store/src/bound.rs:1-3` and `crates/acquisition-search/src/answer.rs:374`.
  9. P · P · refusals pass through one shape (`crates/acquisition-verbs/src/refusal.rs:1`), and per-refusal goldens hold the wording. Nothing checks that a refusal names its target, cause and evidence.
  10. P · P · `crates/acquisition-search/tests/surface_faults.rs:889` (a pattern that matches nothing offers the index) and `price_cmd.rs:1445` (zeroes named). Checked per maker only.
  11. P · P · `crates/acquisition-store/unit/tests/age.rs:65-90` (each row a maker ages carries `_age` beside its epoch), plus about 15 `c53_` age tests in plan, verbs and search. Each covers one maker; nothing walks every age a text prints.
  12. P · N · I grepped the CLI and the test harnesses for ESC bytes and tty checks and found no assertion. `crates/acquisition-cli/src/jobs_cmd.rs:703-710` prints cursor escapes to stdout with no tty check (see Noticed).
  13. P · U · a rule on future changes. Any golden can be regenerated, so no check over the tree could fail on it.
  14. P · Y · `crates/acquisition-cli/src/store_cmd.rs:1365-1369`: the header ends with ` id` and the row ends with the tab id.
  15. P · Y · `crates/acquisition-mcp/tests/pricing_tools.rs:145+` and `search_tools.rs:349`: the MCP tool's document equals the CLI's `--json`, with `expand` mapped to `--expand`. The single text block is at `crates/acquisition-mcp/src/told.rs:475-487`.
  16. P · Y · `crates/acquisition-search/tests/surface_faults.rs:608-654`: the index is every entry at one line, and `all` is the whole. `crates/acquisition-cli/tests/seat_faults.rs:854-864`: bare JSON is the index and `all` is whole. `crates/acquisition-mcp/tests/search_tools.rs:317-349`: the same on the MCP.
  17. P · U · this qualifies clause 13, a rule on future changes; no check could hold it.
  18. P · Y · `crates/acquisition-search/tests/rows.rs:1356-1371`: asking `ITEM_ROWS+50` rows under `item` returns `ITEM_ROWS`, with `limit` = `ITEM_ROWS`, `limit_asked` = 250 and `left_out` = 1. `crates/acquisition-cli/tests/cli_reference.rs:151` ties the help text to the constant.
  19. P · Y · `crates/acquisition-search/tests/rows.rs:1372-1379`: the continuation asks for `ITEM_ROWS` and returns the last row whole.
- unheld:
  1 complex · para · it needs a table of every verb's two views; there is none.
  2 complex · wrong · it needs either a renderer driven by the JSON, or a check on every maker of text against its JSON.
  4 complex · para · this is a writing rule over every maker's text; only per-verb goldens could hold it.
  7 simple · wrong · `crates/acquisition-plan/unit/tests/listing.rs` or `shop.rs`: a plan with more than ten actions, or a render with more than ten pages, that is not cut.
  8 cut · this is mechanism. `bound.rs:1-3` already states it, and no frontend relies on the sentence.
  9 complex · para · the rule's meaning (what counts as target, cause and evidence) is not something a check can test.
  10 complex · wrong · no shared shape for "nothing" exists to check.
  11 complex · para · it needs a walk over every age each text prints, on every surface.
  12 simple · para · add a "no ESC byte" assertion to `crates/acquisition-client/tests/harness/mod.rs` `sole_json`/`text`, or to the CLI's `Transcript`.
- reason: Cite · `crates/acquisition-store/src/bound.rs:1-16` states the bound's property, `crates/acquisition-cli/src/price_cmd.rs:5` the three levels, `crates/acquisition-search/src/describe.rs:976` the two densities. No comment gives the reason (decisions are made from the first screen; a 169 KB default). · moves to `crates/acquisition-store/src/bound.rs:1`
- fate: SH-KO
- confidence: medium · I read about half of the roughly 35 `c53_` tests. I did not read the plan and shop never-cut paths, the web page renderers, or the MCP `told_as` door beyond `told.rs:475`. The class could be argued as design instead (see Noticed).

### C149 — Every answer and every refusal is
- class: design — an agent reading any answer is promised one JSON object, with counts beside rows.
- clauses:
  1. P · P · Refusals: one function, `crates/acquisition-verbs/src/refusal.rs:1-5`. MCP answers: one door that refuses anything that is not an object (`crates/acquisition-mcp/src/told.rs:480-483`; nothing else builds a `CallToolResult::success`). Verb documents: `crates/acquisition-verbs/tests/documents.rs:1660-1776`, 14 documents, each asserted `is_object`. The CLI's success path is held only for the verbs listed in `crates/acquisition-cli/tests/store_answers.rs:407-455`; a new CLI verb printing a bare array would pass.
  2. P · Y · `crates/acquisition-mcp/src/told.rs:480`: the one door every tool's answer passes refuses a non-object. `crates/acquisition-mcp/tests/search_tools.rs:1429-1468` asserts objects with their counts.
  3. M · R · restates C53 ("text is a function of JSON") with C146's words.
  4. M · N · this is history: the one non-additive change, taken at the alpha. Nothing could hold it, and nothing relies on it as a promise.
- unheld:
  1 simple · para · `crates/acquisition-client/tests/harness/mod.rs:810` `sole_json`: assert that the document is an object, which covers every `--json` answer any test reads.
  4 cut · the commit holds this history, and the code reads nothing from it.
- reason: Prop · `crates/acquisition-mcp/src/told.rs:477-479` (the property, and why it sits at one door; not the seat's miscount) · moves to `crates/acquisition-verbs/tests/documents.rs:1652`
- fate: NT · simple
- confidence: high · I did not read the remaining `c149_` count tests (`events.rs:346`, `characters.rs:103`, `jobs_cmd.rs:772`).

### C115 — An answer leads with what bears
- class: design — an agent reading a search or `tabs` answer is promised the total and the view first, and routes only where they are useful.
- clauses:
  1. P · Y · `crates/acquisition-mcp/tests/search_tools.rs:904-942`: the key order is query, scope, total, rows, view, terms, basis, for both a rows answer and a counts answer. The order is declared once, at `crates/acquisition-search/src/answer.rs:1439`.
  2. P · Y · `crates/acquisition-search/tests/answer.rs:825-835`: terms keep their blocks, with no `request` on matched, failed or lacked unless asked. The terms block is present at `search_tools.rs:913`. The part about C93's meaning is R (C93).
  3. P · P · `crates/acquisition-search/tests/answer.rs:815-848` and `:786-811` hold the code's rule: even when routes are not asked, the routes a next step hangs on (undecided, together, a placed bucket's none or undecided) are carried. That rule is narrower than the clause's "only when routes are asked for" (see Noticed).
  4. P · Y · `crates/acquisition-mcp/tests/search_tools.rs:952-995`: `tabs` text starts with its counts, holds ten rows, `expand` gives 41, `limit` 25 leaves out 16, and each equals `acq tabs --json`.
- unheld: 3 complex · para · the clause's meaning is unclear: the code deliberately carries some routes unasked, so the text needs amending before a test can hold it.
- reason: Cite · `crates/acquisition-search/src/answer.rs:1439-1443` gives the order and the "reader scrolls for total" half of the reason. The size half is in the test doc at `crates/acquisition-search/tests/answer.rs:774-780`. `tabs.rs:1` and `told.rs:465-474` hold fragments. · moves to `crates/acquisition-search/src/answer.rs:1439`
- fate: NT · complex
- confidence: medium · I did not read `seat_faults.rs:3149-3247` (the CLI's text order and routes) or `search_json.rs:3078`.

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C11 | design | 5 | 0 | 4/1/0/0/0 | Cite | NT | 0/1/0 | 0/1 | medium |
| C53 | cross | 17 | 2 | 7/7/2/2/1 | Cite | SH-KO | 2/6/1 | 3/5 | medium |
| C149 | design | 2 | 2 | 1/1/1/0/1 | Prop | NT | 1/0/1 | 0/1 | high |
| C115 | design | 4 | 0 | 3/1/0/0/0 | Cite | NT | 0/1/0 | 0/1 | medium |

## What I did not check
- About half of C53's `c53_` tests: the plan listing and shop age tests, `coverage.rs`, `characters.rs`, `events.rs`, `documents.rs:221/472/610/1388`, and `price_cmd.rs:1759/2205`.
- Whether any test checks that a plan's actions or a shop render's pages stay whole past ten (C53, clause 7).
- The web frontend's renderers for C53 and C149 (only `web.rs:6703` was read), and whether the web's JSON passes through a single object door.
- For C11 clause 3, a failed job is held through one case (a stash 404). I did not look for a test of a parent job's failure.
- CI workflows. I relied on the tests named running in the gate's nextest stage.

## Noticed
- **C53, step 2, clause 12:** `acq demo` (`crates/acquisition-cli/src/run.rs:557` → `crates/acquisition-cli/src/jobs_cmd.rs:703-710`) prints ANSI cursor escapes to stdout without checking for a tty. This breaks "Escapes only on a tty". The progress line at `jobs_cmd.rs:228-252` does check.
- **C115, step 2, clause 3:** the entry says JSON carries a count's request "only when routes are asked for". The code and its tests (`crates/acquisition-search/src/answer.rs:148`, `crates/acquisition-search/tests/answer.rs:774-848`) deliberately carry the routes a next step hangs on even when routes are not asked. The entry is wider than the code.
- **C53, step 5:** the rule lets this entry be either `design` (the shape and words of answers) or `cross` (a new frontend could print text not derivable from the JSON, or ages with no words, and nothing would fail). I chose `cross` by the brief's test. Under `design`, the fate would be NT complex. The bound in `acquisition-store` is shared, but text as a function of JSON is not.
- **C149, step 1, clause 4:** it records an event (the one non-additive change at the alpha). AGENTS.md sends history to `HISTORY.md` or `git log`, not to current documents.
- **C11, Pinned:** the *Pinned:* names `seat_faults.rs` as a whole file. The holders that actually check the clauses are the specific tests listed above, and clause 5 is held mainly in `acquisition-verbs` and `acquisition-mcp`, neither of which the *Pinned:* names.
