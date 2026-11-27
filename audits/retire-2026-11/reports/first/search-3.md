### C96 — Place is terms, but the
- class: design — a user or agent at search is told that the realm is the search's scope, resolved and printed back, while place values are terms they type
- clauses:
  1. P · Y · `crates/acquisition-search/tests/language.toml:341-349` (run by `language.rs:185`): `realm:pc …` and `-realm:pc` are refused as `realm_is_scope`. `tests/answer.rs:518-526`: a store with two realms and no realm named gives `realm_needed`
  2. P · Y · `tests/language.toml:326-328`: `league:`, `tab:`, `character:` and `container:` parse and print back as terms. `tests/membership.rs:511-513` and `:576-581`: `league=` selects as a term
  3. P · Y · `tests/answer.rs:52-53` and `:530`: `scope.realm` is printed in the answer. `:544-548`: a route carries the resolved realm in `scope`. A realm cannot sit in a branch because the term is refused (clause 1's refusal)
  4. P · Y · `tests/answer.rs:518-526`: the error is `realm_needed` and lists `realm pc · realm xbox · realm all`. `:527-532`: an `all` scope answers over both realms (8 items)
  5. P · Y · `tests/membership.rs:107-114`: the default membership is `live`. `tests/answer.rs:990-996`: with no league term, the scope covers both leagues (`Allflame`, `Standard`)
  6. P · Y · `tests/answer.rs:49-79`: the scope block asserts items, realm, leagues, `never_fetched` and `oldest_fetch`, with its location
- unheld: none
- reason: Cite · `crates/acquisition-search/src/corpus.rs:123` (states "the scope, never a term" but not the why: a template and a total mean different things in two games) · moves to `crates/acquisition-search/tests/answer.rs:500`
- fate: RM
- confidence: high · I did not read the CLI's or the verbs' saved-search realm handling (`saved.rs:46`)

### C97 — The vocabulary is a read of
- class: design — a human or agent finds the language's words by counting by `line` under their query, and the closed lists are in the help
- clauses:
  1. P · Y · `crates/acquisition-search/tests/counts.rs:199-201,511-514`: the vocabulary is a `counts` view on the same `answer()` call over the loaded corpus
  2. P · Y · `tests/counts.rs:520-529`: buckets are ranked by count. `:543-550`: a range per slot and kinds (`sources`, `flags`). `:626-658`: under `all`, one row per realm, each with its realm
  3. P · P · `tests/counts.rs:513`: two texts in one call. Every row's `term` is asserted as a string for sample rows (`:534-536,552,567`), and the row's route `request` is followed by `members()` (`:209-224`). Loose half: no test pastes the printed `term` under the query and runs it, so a term that does not select its row passes while its route is right
  4. P · Y · `crates/acquisition-search/src/describe.rs:27` prints the same constants `bind` parses (`ITEM_FLAGS`, `LINE_FLAGS`, `SOURCES`, `FIELDS`). `tests/pseudo.rs:658-690`: every computed value is listed with its definition and kind. `tests/surface_faults.rs:190-237,552-563`: the composition and value words, and the sections are the answer's keys
  5. P · N · Looked at `crates/acquisition-cli/src/web/mod.rs:2830-2868`: the web form's stat finder and its "Add a column" finder use `trade::vocabulary` (`crates/acquisition-search/src/trade/vocabulary.rs:1-6`), which reads the trade stat table, not this corpus. No test or check keeps autocomplete to the one read
- unheld:
  3 simple · wrong · `crates/acquisition-search/tests/counts.rs`, a paste helper beside `members()`: run `query + " " + term` through `view()` and compare its ids with the route's
  5 complex · para · the clause's meaning is unclear now that the web picker has a second vocabulary read over the stat table; a ruling or a rewording comes before any test
- reason: Cite · `crates/acquisition-search/src/counts.rs:1,28` (states the property in fragments; the why, that a human's autocomplete and an agent's discovery are one need, is not stated) · moves to `crates/acquisition-search/tests/counts.rs:506`
- fate: NT · complex
- confidence: medium · I did not read `rows.rs:1067` or check that every `FIELDS` entry reaches `--describe` without a filter

### C98 — No search service: a consumer holds
- class: cross — every answer is labelled with one basis, and a held corpus is reused only after the check. A resident frontend (the GUI's server to come, or any process holding an `Arc<Corpus>` outside `Holder`) could answer from a stale corpus and no test or check would fail. No check walks the frontends; `acq web` uses `Holder` (`crates/acquisition-cli/src/web/mod.rs:550,1249`)
- clauses:
  1. P · Y · `crates/acquisition-search/tests/answer.rs:598`: the answer's `basis` is the held corpus's basis, serialized whole. The clause's opening words ("no search service") restate C48 (no cached search)
  2. P · Y · `crates/acquisition-search/src/corpus.rs:168-199`, the `Basis` type: store, account, build, snapshot, derivation, each table's version and intent. `tests/answer.rs:614-639`: store and account. `tests/answer.rs:81-88`: the snapshot and the table versions. `tests/price.rs:494`: intent. `tests/next.rs:578`: build
  3. P · Y · `tests/answer.rs:650-692`: `Holder` reuses one `Arc` while nothing changes, and loads anew when a tab is recorded or the membership or store changes. `crates/acquisition-search/src/corpus.rs:856-863`: `is_current` compares the whole `Basis` for equality, so a component added to the type is compared too
  4. P · Y · `tests/answer.rs:575-599`: after a fetch lands, a held corpus still answers at its basis (7 items, its basis); `is_current` turns false and the next load counts 8
  5. M · R · restates the "Not doing" line at `decisions/search.md:42` (a persisted projection, not until the 500 ms trigger)
- unheld: none
- reason: Prop · `crates/acquisition-search/src/corpus.rs:871-884` (the `Holder` doc states the check before every answer and that nothing is kept across processes, but not the why, structure over an event) · moves to `crates/acquisition-search/tests/answer.rs:641`
- fate: SH-KO
- confidence: medium · the class turns on whether the opt-in `Holder` counts as "one shared crate every frontend inherits". I did not read `acq-mcp`'s corpus holding or the store's `unit/tests/corpus.rs` snapshot tests

### C100 — There is one answer shape: the
- class: design — an agent or user gets one answer shape: compact rows, ids that work back, continuations bound to a basis, and the effective price
- clauses:
  1. P · Y · `crates/acquisition-search/tests/answer.rs:36-169`: the worked example asserts query (text and tree), scope, basis, terms, total and rows; `:1084-1088`: `next` is a request. There is one `Answer` type in `src/answer.rs`
  2. P · P · `tests/answer.rs:135-156`: id, base, rarity, place by name and id, and the touched lines with their values. `tests/rows.rs:1242`, `:1404`, `:1452-1458`: named fields work, and an unasked row has no `icon` and the view names no `fields`. Loose half: no assertion that a default row lacks `shown` (the item), and `name` is not asserted
  3. P · Y · The `Row` type (`crates/acquisition-search/src/answer.rs:2018-2047`) has no body field. `tests/acceptance.rs:900-934`: `show` gives the body only on request. `tests/against.rs` exists for the why-not (not read)
  4. P · P · `tests/acceptance.rs:781-805`: every row's item id is accepted back by `id:` and by `show`. Location ids `g1`, `c1` and `m1` are hand-picked. Loose half: ids printed outside rows (a scope's `oldest_fetch_location.id`, a count bucket's `id`) are never fed back
  5. P · Y · `tests/next.rs:178-219`: a continuation carries the query, its values and its sort, and its pages equal the whole answer. `:225-243` and `:503-560`: it is refused as `basis_changed` across a fetch. `:578-597`: it is refused across a build
  6. P · Y · `tests/price.rs:220-271`: `priced` is the effective statement, the game's on a tie, with price and side on the row. Read-only: the search takes `&Annotations`
- unheld:
  2 simple · para · `crates/acquisition-search/tests/rows.rs:1452`: also assert that an unasked row has no `shown` and carries `name`
  4 simple · wrong · `crates/acquisition-search/tests/acceptance.rs:781`, `aq4`: also loop over the scope's fetch-location ids and the place ids rows print, through `id:`
- reason: Prop · `crates/acquisition-search/src/answer.rs:1-4` (states the one shape but not the why: an answer that carries its evidence needs no second call and no body) · moves to `crates/acquisition-search/tests/answer.rs:35`
- fate: NT · simple
- confidence: medium · I did not read `against.rs`, `membership.rs:265` or `depth.rs` (these may hold more of clause 4)

### C101 — A missing reach is a field
- class: design — a user or agent reaches what an item holds through named fields and derived values, never a raw path into the body
- clauses:
  1. P · U · a rule on future language design. No test over this tree could fail on it
  2. P · N · Looked at the error kinds in `tests/language.toml`, `NOT_BUILT` (`src/bind.rs:136-147`), and searched the tests for a refused body path or collection syntax. Nothing refuses one by test; the fields list is closed (`src/bind.rs:305`). `src/properties.rs:7` states "No general accessor"
  3. P · Y · `crates/acquisition-search/tests/sockets.rs:202-256`: a colour over the whole item. `:263-317`: `linked( … )` within one group. `:325-369`: an abyssal socket by its word, over the item and in a group, and never two in one group
  4. P · Y · `tests/pseudo.rs:658-690`: computed values are listed with a kind, `derived` apart from `total`, and the totals header counts 82 totals and 4 readings. `tests/percentile.rs:1369-1392`: `defence_pct` is `derived` and its definition is printed. "Never a total row" also leans on C94 ("nothing else defines a total")
  5. P · Y · `tests/trade_catalogue.rs:141-181`: every filter, stat category and group type the captures list has a catalogue row. `:260` (by the module doc at `src/trade/catalogue.rs:12-20`): the loader refuses an unsupported row without a note
  6. P · U · a rule on people (who records a change to what a user types). See Noticed
- unheld:
  2 simple · para · `crates/acquisition-search/tests/language.toml` (or `tests/refusal.rs`): a case refusing `explicitMods[0]` or `body.ilvl>=1` by kind
- reason: Cite · `crates/acquisition-search/src/properties.rs:7`, `src/pseudo.rs:37,322` (the property in fragments; the why, that reach is the derivation's and a derived field is the cheap change, is not stated) · moves to `crates/acquisition-search/src/pseudo.rs:322`
- fate: NT · simple
- confidence: medium · I did not read `trade_catalogue.rs:260`'s body, `derive.rs:639` or `tests/search_json.rs:1953`

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C96 | design | 6 | 0 | 6/0/0/0/0 | Cite | RM | 0/0/0 | 0/0 | high |
| C97 | design | 5 | 0 | 3/1/1/0/0 | Cite | NT | 1/1/0 | 1/1 | medium |
| C98 | cross | 4 | 1 | 4/0/0/0/1 | Prop | SH-KO | 0/0/0 | 0/0 | medium |
| C100 | design | 6 | 0 | 4/2/0/0/0 | Prop | NT | 2/0/0 | 1/1 | medium |
| C101 | design | 6 | 0 | 3/0/1/2/0 | Cite | NT | 1/0/0 | 0/1 | medium |

## What I did not check
- Whether every `FIELDS` entry reaches `--describe` (C97 clause 4 rests on shared constants, not a completeness test).
- `against.rs`, `depth.rs` and `membership.rs:265`, which may hold more of C100 clauses 3 and 4.
- How `acq-mcp` holds its corpus (C98's class): I confirmed `Holder` only in `acq web`.
- Whether unknown-field refusals are pinned somewhere outside the search crate's test names I searched (C101 clause 2).
- The `trade_catalogue.rs:260` loader-refusal test body (C101 clause 5 leans on it through the module doc).

## Noticed
- C97, step 2 (clause 5): the entry says autocomplete is "this one read" of the corpus. But `acq web`'s stat finder and its "Add a column" finder (`crates/acquisition-cli/src/web/mod.rs:2830`) read `trade::vocabulary`, which lists the trade stat table, not the corpus. The entry's text and the code disagree.
- C101, step 5 (clause 6): "a change to what a user types is recorded as a ruling before the code, under the owner's delegation" contradicts C170 and `AGENTS.md` ("Agent-facing design … a query word … is the session's, recorded in the commit that builds it"). Taken literally, clause 6 is a `rule` promise and would make C101 owner's (SH-KO/KO) under "take the owner's one". I classed it `design` because C170 overrides the clause. The class rule does not decide which reading wins when an entry's clause is stale against a later ruling.
- C98, step 5: "not `cross` if the behaviour lives in one shared crate every frontend inherits" does not decide an opt-in shared holder (`Holder`) that a frontend may bypass. I chose `cross`; `design` or `mech` would make it retire-eligible.
- C98, clause 1 begins with C48's own words ("no search service"), so it is partly a restatement inside a P clause; the clause rule gives one code per clause.
- C96's *Pinned:* names only `answer.rs`. The "never a term" half is held in `tests/language.toml:341-349`, which the *Pinned:* does not name.
