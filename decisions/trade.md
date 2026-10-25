# Decisions — The trade boundary: the translation, the site's fields, the stat id

Part of the decision registry (`AGENTS.md`, "Cross-cutting rulings", whose rules apply unchanged): the rulings that bind the trade boundary — the `trade` modules and `properties.rs` of `acquisition-search`, their reference tables, the oracle and the sitting's tools. The search's lines bind the text the translation writes, so read both, with `search/README.md`, `search/DIGEST.md` (the `S` ids, by id) and `REVIEW-CHECKLIST.md`, "Trade". The contract detail no code has taken is "Contract detail" at the foot of this file, binding with its line.

**Standing.** As `decisions/search.md`'s: every line stands and changes by the owner's ruling alone, and the commit that changes it quotes him; a line found wanting is brought to him.

## The boundary and the translation

- **C99 — The trade boundary is a translation with a report, in both directions, and the stat id lives only here.** A site URL decodes with no network. Each clause lands as exact, ambiguous, unsupported or listing-only, and a nonempty remainder means the result is never run as equivalent; a clause the published model cannot represent stays in the remainder. The site's bound on a ranged line is its average (S47, two-number lines averaged) and translates to `avg` on the line (C92, one occurrence bound); an undecided pair is shown with both candidates. A line with no stat id cannot cross. The outbound half is unbuilt ("Not doing", below). *Why:* the report makes a missing mapping honest; it does not supply one. *Pinned:* `acquisition-search/tests/trade_translate.rs`.
- **C113 — Inbound, PoE1: a site search URL, its id or its query JSON is a third spelling of a query: decoded with no network, written as the language's text, run and labelled.** Exact and ambiguous clauses run, the rest are reported, and a remainder makes the answer say it is not the site's. The URL's realm is the scope (C96, realm as scope): `/trade/search/<league>/…` is pc, `…/(xbox|sony)/<league>/…` a console; its league is a term; a PoE2 URL is refused by name. The stat vocabulary is reference data (C106, admitted convention) only the translation reads (C90, line identity). *Why:* the text is what a person would type and can edit; a label is read, a refusal is a round trip. *Pinned:* `acquisition-search/tests/trade_decode.rs`, `acquisition-search/tests/trade_translate.rs`.
- **C114 — Every numeric property the site's filters read is a field named by its displayed name, defined once in one table.** Quality, gem level, map tier and the rest of the site's bridge (S55, the site's property bridge) are each a field over the displayed property, reviewed reference data (C106, data and convention), listed in `--describe fields`, with no general accessor (C101, fields not doors). The quality-normalised defences, `ar`, `ev`, `es` and `ward` at 20 % quality (S26, the site's normalisation), are fields too. *Why:* many of the site's filters read a property the search reached only through `text:`. *Pinned:* `acquisition-search/tests/properties.rs`.
- **C119 — An ambiguous clause says what is at stake: its row may carry the other reading, and the answer counts the items in scope that reading would add or drop, with a route.** The other reading is a fragment in the language, with the clause's bound where it needs one, applied where the clause has every property of the shape asked; a row with no other reading prints nothing (C93, absent versus undecided). *Why:* the report (C99, the translation's report) should finish its sentence, and runs measured each ambiguity by hand. *Pinned:* `acquisition-search/tests/trade_translate.rs`, `acquisition-search/tests/trade_catalogue.rs`.
- **C141 — A trade query JSON may carry the league beside its query object, `{league, query, sort}`, read as a URL's league is: a term (C113, the third spelling).** The realm stands in the scope as before, never in the envelope, and any other envelope key is still reported unsupported by name. *Why:* a form that emits the site's JSON must scope to one league, and the site carries the league in a URL nobody can compose without an id the site mints. *Pinned:* `acquisition-search/tests/trade_decode.rs`.
- **C142 — A trade envelope may name the reading of the site's four defence filters: `defences: "displayed"` lands Armour, Evasion, Energy Shield and Ward on the displayed fields; absent, or `"q20"`, lands them on the site's normalised ones (C114, the property bridge).** Each clause's label and the translated text name the field used; the option is read beside `league` (C141, the league beside the JSON) and nothing else of the envelope changes. *Why:* the site cannot ask for a defence as the item stands, and one reading named beside the query serves a page's checkbox and `--trade` alike, without a control per field. *Pinned:* nothing yet.

## Not doing

What someone would propose and the project declines, each until the trigger it names or for good; the backlog is the repository's GitHub issues.

- The outbound translation and PoE2 on the trade site: not until a consumer needs a search sent, or PoE2 trade is in use.
- A map tier table for bases naming no tier: not until the owner registers a source (C79).
- User-authored weighted sums: not until a recorded question a named total and `sum` cannot serve.

## Contract detail (binding with its line until code takes it)

**C99 (K5).** `count` carries its minimum and its maximum; `weight`
and `weight2` differ in how a per-stat requirement gates a
contribution (S57), and a weighted group translates only where that
guard has a representation; a defence bound is inexact while the site
normalises quality (S26) and sits in the remainder until a normalised
field exists. Several ids for one line go out as a `count` of at
least one (S106). A site bound on a ranged line is `avg` on the line
(S47), exact once the word exists.

**C113 (K6).** Taken whole by the code at steps 4 and 5 of the brief:
`answer.rs` (URL, scope, sort), `trade.rs` (the id), `translate.rs`
(clauses, groups, the line).
