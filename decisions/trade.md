# Decisions — The trade boundary: the translation, the site's fields, the stat id

Part of the decision registry (`CONTEXT.md`, "Decisions — the registry", whose rules apply unchanged): the rulings that bind the trade boundary — the `trade` modules and `properties.rs` of `acquisition-search`, their reference tables, the oracle and the sitting's tools. Split from `decisions/search.md` 2026-10-17 on the owner's acceptance, every line moved verbatim with its date; the search's lines bind the text the translation writes, so read both, with `search/README.md`, `search/DIGEST.md` (the `S` ids, by id) and the brief `search/TRADE-PLAN.md` while it is open. The contract detail no code has taken is "Contract detail" at the foot of this file, binding with its line.

**Standing.** As `decisions/search.md`'s: every line stands and changes by the owner's ruling alone, ruled in his own words, quoted on the line or in the module doc it bears on; a line found wanting is a park with its trigger, brought to him.

## The boundary and the translation

- **C99 — Direction. The trade boundary is a translation with a report, in both directions, and the stat id lives only here.** A site URL decodes with no network (S56). Each clause lands as exact, ambiguous, unsupported or listing-only, and a nonempty remainder means the result is never run as equivalent; a clause the published model cannot represent stays in the remainder. The site's bound on a ranged line is its average (S47) and translates to `avg` on the line (C92); an undecided pair is shown with both candidates (S67). A line with no stat id cannot cross (S155). *Why:* the report makes a missing mapping honest; it does not supply one. *Evidence:* S26, S41, S47, S56, S57, S67, S106. 2026-09-17.
- **C113 — Inbound, PoE1: a site search URL, its id or its query JSON is a third spelling of a query — decoded with no network, written as the language's text, run and labelled.** Owner, 2026-10-15: "run and label"; "accept the raw query json, too". Exact and ambiguous clauses run, the rest are reported; the remainder is the unsupported, and nonempty it makes the answer say it is not the site's. The URL's realm is the scope (C96): `/trade/search/<league>/…` is pc, `…/(xbox|sony)/<league>/…` a console; the league is a term; a PoE2 URL is refused by name. The stat vocabulary is reference data (C106) read by the translation alone (C90). *Why:* the text is what a person would have typed and can edit; a label is read, a refusal is a round trip. *Details:* K6; `translate.rs`.
- **C114 — Every numeric property the site's filters read is a field, named by its displayed name, defined once in one table.** Owner, 2026-10-15: "build all using displayed names". Quality, gem level, map tier and the rest of the site's bridge (S55), each a field over the displayed property, reviewed reference data (C106), listed in `--describe fields`; no general accessor (C101). The quality-normalised defences — `ar`, `ev`, `es`, `ward` at 20 % quality (S26) — are built in the same step (owner: "in this step"). *Why:* 25 of the site's 93 filters read a property the search reached only through `text:`. *Details:* `properties.rs`, and `reference/properties-v1.toml`'s header. *Pinned:* `acquisition-search/tests/properties.rs`.

## Parked (do not build yet; each with its trigger)

Scope this area has deferred, under the rules of `CONTEXT.md`, "Parked".

- The trade translation, the outbound half and PoE2 (C99, C113) → `trade.rs` in the search crate, beside the inbound half note 36's step 3 builds under `search/TRADE-PLAN.md` (opened 2026-10-15, rewritten at its close). Trigger: a consumer who needs a search sent to the site, or PoE2 on the trade site in use.
- The sitting's sat rounds as data (the committed sheet the record, `tools/trade-sheet.py` composing only the next round against it, the earlier rounds' code left to history) → `tools/trade-sheet.py`; until then every round's code reproduces its rows and the gate holds it (`--verify`). Trigger: the trade brief's close (step 9), or a round whose input cannot be frozen.
- The capture tests that rebuild an item from the extract's selection (`tests/pseudo.rs`, `tests/properties.rs`, `tests/ranged.rs`, each a header the capture lacks filled in, `ilvl` 84 among it) → read the item as the site sent it, `search/pseudo-stats/data/bodies.json`, as the trade oracle does. Trigger: a capture test whose answer turns on a key the extract drops.
- A map tier table for bases naming no tier (u065) → reference data read by `map_tier` (C101). Trigger: the owner registering a source of map tiers (C79).
- A named stat value, `stat("…")`: the item's stat however the game displays it — the opposite word below nothing, a numberless form at its number, summed, lacked when none — exact where the translation labels such a bound ambiguous, and typeable → the query language, one value kind beside `sum` and `pseudo.`, over the translation's forms table (`brainstorming-notes/37-the-stat-value.md`, 2026-10-17: the translation alone ships). Trigger: a seat or a recorded question bounding a stat the translation labels so for its forms, or the outbound half's trigger firing.
- User-authored weighted sums (an unnamed total; operands anything with a value, a definition and a status) → the query language, beside `sum`; range weights, negative weights on ranges and guarded trade groups need their own definition first. Trigger: a recorded question a named total and `sum` cannot serve, or the trade translation meeting a weighted group (C99).

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
