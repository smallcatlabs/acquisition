### C99 — The trade boundary is a translation
- class: edge — the stat id and the site's vocabulary live at the trade boundary alone, and what crosses it carries a report. A user is also promised that a partial translation is never run as the site's search. The entry has promises in two classes and takes the owner's.
- clauses:
  1. P · P · the report half is held: `crates/acquisition-search/tests/trade_translate.rs:242-272` (`holds`: the remainder equals the unsupported clauses, the fidelity is the worst status, and the guard holds). "The stat id lives only here" is held by nothing. I looked in the search, cli and mcp tests and found no assertion that a stat id stays out of the text, the answer or the store. The `no_ids.rs:320` citation is a literal in a sweep's self-test, not a holder. "Both directions" is half unbuilt (clause 8).
  2. P · P · decoding works offline in every decode test (`trade_decode.rs:44-114`). `tools/arch-check.sh:251` (check 5) refuses any HTTP client crate outside the transport. Loose half: nothing refuses `std::net` in the search crate (`git grep` found no guard).
  3. P · Y · `trade_translate.rs:410-444` (`worked_by_hand`) asserts every fixture clause's status: exact, ambiguous, unsupported or listing_only. `holds` (`:256-271`) asserts that a nonempty remainder makes the fidelity `partial`. `crates/acquisition-cli/tests/search_json.rs:2664` asserts the text "so this is not the site's search".
  4. P · Y · the fixture "an id the site lists with two texts in one category" (`trade-translate.toml`, about line 981) is asserted unsupported, with empty text and fidelity partial, by `worked_by_hand`.
  5. P · Y · `trade-translate.toml:893-895` ("a ranged local line … the bound on avg"): the text `sum(line("Adds # to # Physical Damage" …).avg)>=20` is asserted at `trade_translate.rs:413`.
  6. P · Y · `trade_translate.rs:335-354` asserts the collision row and the note "the text's other id: explicit.stat_2241560081". The fixture at `trade-translate.toml:966-971` asserts that the line matches both texts.
  7. P · U · outbound is unbuilt, so no line crosses.
  8. M · U · it restates the "Not doing" line of `decisions/trade.md`.
- unheld:
  - 1 simple · para · `trade_translate.rs`, proptest `c113_a_generated_site_query_translates_and_reads_back`: assert that the text and the answer carry no `<category>.stat_<n>`.
  - 2 complex · ggg · needs a new check over std's sockets in the search crate; the dependency edge does not reach them.
- reason: Cite · `crates/acquisition-search/src/trade/translate.rs:1-3`, `crates/acquisition-search/src/trade.rs:1-3`, `crates/acquisition-search/src/trade/stats.rs:1-6` (fragments; the *Why:*, "the report makes a missing mapping honest", appears nowhere) · moves to `crates/acquisition-search/tests/trade_translate.rs:239` (doc of `holds`)
- fate: SH-KO
- confidence: medium · I read the fixture file only at the lines cited, not all 80+ cases. I did not read `trade_oracle.rs`.

### C113 — Inbound, PoE1: a site search URL,
- class: scope — the product reads PoE1 site searches and refuses PoE2 ones. A user is also promised that a site search is a query spelling that is run and labelled (design). The entry has promises in two classes and takes the owner's.
- clauses:
  1. P · P · decoded, written as text, run and labelled: `trade_decode.rs:44-114`; `trade_translate.rs:308-313,410-476` (text, scope and statuses against the hand-worked fixtures, and the text asked of items); `search_json.rs:2622-2666` (runs, `fidelity`, the labelled text). Loose half: "no network", as C99.2.
  2. P · Y · `trade_translate.rs:410-476` (exact and ambiguous clauses write their terms, the rest none); `search_json.rs:2664` (a remainder makes the answer say "not the site's search").
  3. P · Y · `trade_decode.rs:69-113` (pc, xbox and sony realms from the path); `against.rs:340-372` (an xbox URL asked in pc is refused, naming both realms); an xbox fixture realm (`trade-translate.toml:38-40`) asserted at `trade_translate.rs:414`.
  4. P · Y · `trade_decode.rs:259-267` (the league is an exact clause and `league="Hardcore Standard"` is in the text).
  5. P · Y · `trade_decode.rs:119-136` (a `/trade2/` URL and the `poe2` realm are refused with "PoE1 only", kind `trade`).
  6. M · N · it restates C106 (admitted convention) and C90 (line identity). No check confines the stat table's readers: `trade/vocabulary.rs` reads it for `acq web`'s picker (`web/mod.rs:2835`, `web/page.rs:1296`). It is stated at `crates/acquisition-search/src/trade/stats.rs:1-6`.
- unheld:
  - 1 complex · ggg · the no-network half needs a new check over std's sockets, as C99.2.
  - 6 cut · para · M, and no frontend relies on who reads the table; the stat-table doc carries it.
- reason: Prop · `crates/acquisition-search/src/trade/translate.rs:1-3` (the text a person would have typed; the label and round-trip reason are absent) · moves to `crates/acquisition-search/tests/trade_translate.rs:304`
- fate: SH-KO
- confidence: medium · the class turns on whether the PoE2 refusal is read as scope or as the "Not doing" line's (see Noticed). I did not read `answer.rs`'s URL-to-scope code itself.

### C114 — Every numeric property the site's filters
- class: design — the query words (field names) a user or an agent types for the site's numeric filters.
- clauses:
  1. P · P · `trade_catalogue.rs:188-249`: every field the catalogue names binds, and the fields beyond the language's own equal the property table plus three named fields. `properties.rs:122-132`: the loader refuses a duplicate name. Loose half: nothing asserts that every numeric property filter of the site lands a field rather than an unsupported row, or that the name is the displayed one.
  2. P · P · `properties.rs:27-63`: every table field is in `--describe fields`, with its definition and table version. `properties.rs:724-815`: each field is asked every way a number is. Loose half: "no general accessor" restates C101 (fields, not doors). There is no construct to refuse, so no test fails on it.
  3. P · Y · `properties.rs:347-389` (specs for `armour.q20`, `evasion_rating.q20`, `energy_shield.q20` and `ward.q20`, run by `c114_every_field_is_asked_every_way_a_number_is`); `properties.rs:823-845` (displayed beside q20, worked values).
- unheld:
  - 1 simple · para · `crates/acquisition-search/tests/trade_catalogue.rs`, `c114_every_field_the_catalogue_names_binds`: assert that no `minmax` row over a property is unsupported.
  - 2 complex · para · the no-accessor half is a rule on future design, and its meaning as a testable property is unclear.
- reason: Prop · `crates/acquisition-search/src/properties.rs:1-8` (states the property; the *Why:*, "reached only through `text:`", is absent) · moves to `crates/acquisition-search/tests/properties.rs:20`
- fate: NT · complex
- confidence: medium · I read `properties.rs` tests 1-160 and 715-875; I did not read the `s26_` captured-figure test body.

### C119 — An ambiguous clause says what is
- class: design — the shape of the answer: what an ambiguous trade clause tells an agent, with counts and routes.
- clauses:
  1. P · Y · `trade_translate.rs:489-556`: each fixture's `other.adds` and `other.drops` count equals the hand-worked items, each route returns exactly those ids, and at least 5 stakes are read. `:557-577`: every catalogue row carrying a reading is read by a fixture.
  2. M · Y · `trade_translate.rs:493-503` (readings equal the hand-worked fragments); `trade_catalogue.rs:488-536` (the loader refuses a reading on a non-ambiguous row, an unknown placeholder or a quoted one). Stated at `crates/acquisition-search/src/answer.rs:163-177`.
  3. P · Y · `trade_translate.rs:503` (the readings map equals the expected one exactly, so a clause with none carries no `other`); `:537-538` (a zero count carries no request).
- unheld: none
- reason: Prop · `crates/acquisition-search/src/answer.rs:163-177` (the *Why:*, "runs measured each ambiguity by hand", is only in `crates/acquisition-cli/tests/seat_faults.rs:3250`'s test doc) · moves to `crates/acquisition-search/tests/trade_translate.rs:478`
- fate: RM
- confidence: high · I did not read `seat_faults.rs`'s test body beyond its doc.

### C141 — A trade query JSON may carry
- class: design — the envelope shape an agent or a page writes: the league beside the query.
- clauses:
  1. P · Y · `trade_decode.rs:243-267` (the envelope `league` is decoded, translated as an exact `url.league` clause and is a term of the text); `:280-291` (null is absent, a bare query's `league` is the query's, and a league that is not text or is empty is refused).
  2. P · Y · `trade_decode.rs:252-255,269-278` (an envelope `realm` does not fill the realm; it is carried in `beside` and reported in the remainder as `envelope.realm`).
- unheld: none
- reason: Prop · `crates/acquisition-search/src/trade.rs:42-48` (the ruling verbatim, with no *Why:*) · moves to `crates/acquisition-search/tests/trade_decode.rs:237`
- fate: RM
- confidence: high · nothing material left unread.

### C142 — A trade envelope may name the
- class: design — an envelope option (a query word) that a page checkbox and `--trade` send.
- clauses:
  1. P · P · `trade_translate.rs:1024-1070`: `displayed` lands `armour>=330`, exact, and the answer matches 0 versus 1. Loose half: only Armour of the four `DEFENCE_ROWS` (`translate.rs:667-672`) is tested. Evasion, Energy Shield and Ward are not.
  2. P · Y · `trade_decode.rs:449-461` (`"q20"`, null and absent all read Q20); `trade_translate.rs:1044` (null lands `armour.q20`, ambiguous).
  3. P · P · the text names the field (`trade_translate.rs:1049`), and the displayed clause's note names `armour` (`:1056-1062`). Loose half: no other defence's label is asserted, and the q20 clause's label is not asserted to name `armour.q20`.
  4. P · Y · `trade_decode.rs:454-456` (`defences` is read beside the query and does not go to `beside`); `trade_translate.rs:1145-1157` (with league, defences, sort and realm together, only `envelope.realm` is in the remainder).
- unheld:
  - 1 simple · wrong · `crates/acquisition-search/tests/trade_translate.rs`, the table in `c142_the_defences_reading_lands_the_field_it_names`, or envelope `query` cases in `trade-translate.toml` for `ev`, `es` and `ward`.
  - 3 simple · wrong · the same table: assert each clause's note or label names its field.
- reason: Prop · `crates/acquisition-search/src/trade.rs:49-56` (the ruling verbatim, with no *Why:*); `crates/acquisition-cli/src/web/page.rs:1017` gives half a reason · moves to `crates/acquisition-search/tests/trade_translate.rs:1017`
- fate: NT · simple
- confidence: high · I did not read the web form tests that cite C142 (`form.rs:713,960`).

### C195 — A search held as the site's
- class: design — the link an answer carries back to the site: what it encodes and what it names as left out. The C79 clause (governed surfaces) restates a cross-cutting ruling.
- clauses:
  1. P · Y · `trade_decode.rs:507-529` (proptest: a composed link decodes to the same query, realm and league); `trade_translate.rs:1101-1131` (an answer's link decodes to the query as read, a nested null kept, no status added).
  2. P · Y · `trade_translate.rs:1145-1164` (`defences`, `sort` and `realm` beside the query are left out; the link's query is the query object alone).
  3. P · Y · `trade_translate.rs:1155-1158` (`left_out` names `defences`, `sort` and `realm`); `crates/acquisition-cli/tests/web.rs:885-902` (each league option's link decodes to the same query in that league); `trade_decode.rs:536-583` (no link, with why: no league, every realm, PoE2, size limits).
  4. M · P · it restates C79. `web.rs:861-883` asserts the one outbound anchor opens `_blank`/`noopener` under `Referrer-Policy: same-origin`, and arch-check 5 refuses an HTTP client. Loose half: `std::net`. Stated at `crates/acquisition-search/src/trade.rs:86-87`.
  5. P · Y · `search_json.rs:2722` and `:2924` (a request in the language carries no `query.trade`, hence no link); `trade_translate.rs:1205-1212` (describe says it).
- unheld:
  - 4 cut · para · M, restating C79, which is cross-cutting in `AGENTS.md`; it is stated at the code.
- reason: Prop · `crates/acquisition-search/src/trade.rs:72-87` (the decode run backwards, "so the site is asked what it was asked"; the other-league half of the *Why:* is only in the page's words at `crates/acquisition-cli/src/web/page.rs:2134`) · moves to `crates/acquisition-search/tests/trade_decode.rs:502`
- fate: RM
- confidence: medium · I did not read `browser.rs:910` (the league select driven in a browser) or the link's rendering in `search_cmd.rs:1013`.

## Rows
| id | class | P | M | held Y/P/N/U | reason | fate | scope s/c/x | worth own/para | conf |
| C99 | edge | 7 | 1 | 4/2/0/2 | Cite | SH-KO | 1/1/0 | 1/1 | medium |
| C113 | scope | 5 | 1 | 4/1/1/0 | Prop | SH-KO | 0/1/1 | 1/0 | medium |
| C114 | design | 3 | 0 | 1/2/0/0 | Prop | NT | 1/1/0 | 0/2 | medium |
| C119 | design | 2 | 1 | 3/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C141 | design | 2 | 0 | 2/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C142 | design | 4 | 0 | 2/2/0/0 | Prop | NT | 2/0/0 | 2/0 | high |
| C195 | design | 4 | 1 | 4/1/0/0 | Prop | RM | 0/0/1 | 0/0 | medium |

## What I did not check
- I did not read `trade-translate.toml` in full (about 1,500 lines); I read only the cases cited.
- I did not read `trade_oracle.rs`, `trade_forms.rs`, `trade_stats.rs`, `trade_dps.rs` or the MCP tests citing C113 and C114 (`search_tools.rs:663-744`).
- I did not read `browser.rs:910` or the web form tests citing C141 and C142 (`form.rs`, `fill.rs` tests).
- I did not run anything. "Runs in the gate" assumes the files named are nextest targets of `tests/main.rs`.
- I did not open `tools/arch-check.sh` check 5 beyond the lines on HTTP clients.

## Noticed
- C113 clause 6 and the doc at `crates/acquisition-search/src/trade/stats.rs:1-6` both say only the translation reads the stat table. `trade/vocabulary.rs` reads it too (`StatTable::entries`) for `acq web`'s picker, through `web/mod.rs:2835` and `web/page.rs:1296`. They agree only if the vocabulary counts as the translation.
- C99: nothing holds "the stat id lives only here". The `show` citation `crates/acquisition-cli/tests/no_ids.rs:320` is a sample literal in a sweep's self-test, not a holder.
- C99 headline: "in both directions", beside C195's built link back to the site, reads as if outbound were built. "Not doing" says only the language-to-site half is unbuilt. Step 1 is affected: clause 8 restates "Not doing".
- C142's *Pinned:* `trade_translate.rs` tests only Armour of the four filters the ruling names (step 2).
- Step 2 rule: a dependency edge (arch-check 5) refuses every HTTP client crate, but `std::net` is a respelling it does not see. The brief's Y line ("an edge makes the breach unwritable") and its P line ("a respelling would pass") do not settle "no network" (C99.2, C113.1, C195.4). I judged it P.
- Step 5 rule: C113's PoE2 refusal could be read as scope or as the "Not doing" line restated; read as the latter, C113 is design and NT complex. C195 touches scope and ggg only through C79, which I judged a restatement (M), so it is design. Two readers may class both differently.
- The doc of `trade_translate.rs:626-637` and the comment at `properties.rs:20` ("39 fields", asserted 44 counting the displayed four and `atlas_drops`) carry counts. They are not wrong, but they are a size in a comment.
