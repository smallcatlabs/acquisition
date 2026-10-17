# Trade-URL import, inbound, PoE1 — the brief

Step 3 of `brainstorming-notes/36-the-road-to-the-agent-release.md`,
opened 2026-10-15 under C113 and C114 (`decisions/search.md`, with the
contract detail K6); closes by 2026-11-15. This file is the brief the
build runs under, in the mold of `search/BUILD-PLAN.md` at `453a4429`:
the steps, each with its acceptance, what the build owes, and what it
leaves out. It is deleted at the close and cited at its commit; the
record is `TRADE-SLICE.md`, opened 2026-10-17. The slice's
rules 1 to 10 (`SEARCH-SLICE.md`, "The plan's rules") bind every step
below as they bound the search's, and its shapes of fault are the
review's checklist. Reviewed from outside before the build (Astra with
Codex, 2026-10-15, seven findings): the catalogue as a table, the group
rule, the fidelity word, the scope rule and the witnesses below are what
that review asked for.

## What is built

A site search URL is a request: its realm the scope, its league a term,
the rest the query; a sort rides only the JSON envelope and is reported,
never applied (K6). The URL, its id, or its
query JSON becomes the query text a person would have typed, with a
report per clause, and runs over the local store labelled with that
report (C113). The text is the product: it is parsed by the one parser,
printed by the one printer, and the answer carries the site's clauses,
the text and the tree. No new verb, no new tree node, no second
evaluator. The property fields and the 20 %-quality defences (C114) are
the one addition to the language, each a named field over a displayed
property, defined once in a table.

Not built: the outbound half, PoE2, saved queries, weighted sums beyond
the one-weight case, a Rust composer of a site id (the tests decode; the
Python composer stays the sitting's), any encoder of a tree into a site
query. Nothing here touches the network: the site's vocabulary is the
committed capture of 2026-09-12 and every oracle is a committed capture
(C79; `SURFACES.md`).

## Inputs

| Input | Where | Use |
| --- | --- | --- |
| The site's stat vocabulary | `search/trade-query/data/stats-2026-09-12.json` | the stat table, generated |
| The site's filters and grammar | `search/trade-query/data/filters-2026-09-12.json`, `grammar.json` | the catalogue table, by hand |
| The owner's eight searches (q1, q2, q2b, q3, q3b, q4, q5, q6) | `search/trade-query/data/fetch-census.json` (requests, 70 fetched items; q2b found none); the ids from `raw/searches/*-search.json`, local | the decode fixture; the acceptance set |
| 864 sitting captures: 451 with ten fetched items, 78 with one to nine, 335 with none | `search/pseudo-stats/data/captures.json` | the oracle (the 451 and the 78); the 20 %-quality figures (`extended.ar|ev|es|ward`: 1,313 rows over the 4,819 listings fetched, measured at step 3 where this said 812) |
| The site's property bridge (S55) | `grammar.json`, `property_type_to_field` | the property table |
| The fetched items as the site sent them, scrubbed | `search/pseudo-stats/data/bodies.json` (`tools/trade-captures.py`; 4,819 items, and the census's 70) | the oracle's items (step 6) |

The raw captures were restored to `search/*/raw/` from the pre-swap
clone on 2026-10-15, hashes checked against the manifests;
`tools/trade-sheet.py --self-test` composes all eight ids.

## Steps

Each step lands green on the gate and is reviewed from outside before
the next, as the search's were. Steps 0 to 2 are data; the language
grows at 3 so that 4's text parses on the build that writes it.

0. **The catalogue.** `reference/trade-catalogue-v1.toml`, by hand:
   one row per site filter (93, from the filters capture) and per stat
   category (14), each with its disposition — a field, a flag, a line
   source or flag, a total, listing-only, unsupported — the field or
   term it lands as, and the note the report prints. A disposition the
   rows cannot state stops the step (rule 4). The translation reads it,
   `--describe trade` prints it, and one test walks it: every row has a
   fixture at step 4. *Acceptance:* the table is reviewed as a diff; the
   loader's check that every named field exists arms at step 3, when
   the fields it names land.
1. **Decode.** `trade.rs`: a URL in the site's two forms
   (`/trade/search/<league>/<id>`, `…/(xbox|sony)/<league>/<id>`), a
   bare id, or the query JSON (envelope or query), to one query object
   and the scope a URL carries; a PoE2 URL refused by name; a malformed
   id a structured error (C47). *Acceptance:* the eight captured ids
   decode to exactly their requests (the fixture holds the ids, which
   are the queries and carry no seller data); S56 holds in Rust.
2. **The stat table.** `tools/trade-stats-table.py` writes
   `reference/trade-stats-v1.toml` from the dated capture: id to
   displayed text for the six hash categories, the capture date as its
   version, cited in the basis of any answer that used it. The loader
   checks what a reviewer would (every id once, every text non-empty,
   the version this build expects). *Acceptance:* the table
   regenerates byte-for-byte; the loader refuses each defect.
3. **The property fields (C114).** `reference/properties-v1.toml`: each
   field by its displayed name, the property it reads, which value and
   its unit; one reader; `--describe fields` lists them with
   definitions; a property the item lacks is lacked, one unread is
   unread at its grain (rule 8). The 20 %-quality defences the same
   way, by S26's rule; and one derived field over the base type,
   `map_tier`, the `(Tier N)` a map's base carries — on the census's
   copy 1,105 maps over 35 bases, 48 of them unique maps whose base
   carries it too, where this said 643 of 643 and a unique map none
   (measured at step 3); a base with none lacks it. *Acceptance:* every field asked every way a
   number is (compare, sort, sum, count, undecided); the defences
   against the captures' `extended` figures — 1,313 rows — and the 37 of
   the fetch census; M3 rerun, the load's move measured and recorded.
4. **The translation.** `trade.rs`: query object to a request and a
   report, clause by clause as the catalogue and K6 state — status, the
   term written, the note — and the fidelity word. A stat group is
   translated whole or not at all, `and` excepted. Text is written
   through the printer's quoting and parsed back by the parser before
   it is returned, so what the answer prints is canonical.
   *Acceptance:* a fixture per catalogue row and per group type, the
   expected text worked by hand first; a restrictive translation's
   fixture carries a matching item and a failing item that tell it
   from a weaker one (a dropped bound, a lost source or flag, a
   negation, a count); a tautology's (an `if` with no bound, as R1
   carries) is tested as the no-op it is, and a reported-only clause
   (unsupported, disabled, listing-only) as absent from the text and
   present in the report; the eight owner searches translate to their
   expected texts; the structural guard — every clause that runs owns
   one term path inside its group, a tautology its own, and every path
   is a clause's — holds on every fixture; a generated site query (the properties' generators
   gain one: filters, groups, ids drawn from the table) translates to
   text that parses, and its print re-parses to the same tree.
5. **The surface.** `acq search --trade <URL|ID>` and
   `--trade-file <FILE|->` in place of a query; `--realm` beside a URL
   is a conflict; the MCP's `query` takes `{"trade": <string or
   object>}` beside `text` and `tree`; the answer's `query` block
   carries `trade` — source, clauses with statuses, remainder,
   `fidelity`; the text rendering opens with the translation's one
   line and the remainder, loud. `--print-request` is unchanged: the
   request it prints carries the site search beside the text it
   translates to, and `--request` reads it back, translating again.
   `--describe trade` is the catalogue.
   *Acceptance:* `tests/search_json.rs` and the MCP's tests pin the
   JSON; the references regenerate; the tour gains one line.
6. **The oracle: missed matches.** `tests/trade_oracle.rs`: for every
   capture with fetched items whose translation has no remainder, the
   items entered through the store's ingest as `tests/pseudo.rs`
   enters them, the translated query run, every item matched. The
   captures hold what the site returned and never what it excluded,
   so this checks missed matches alone; excess matches are step 4's
   guard and fixtures. An item that does not match is a finding: a
   clause the site reads otherwise, fixed in the translation or filed
   for the sitting, never a tolerance, and never an ambiguous clause
   read as exact to pass. The eight owner searches the same.
   *Acceptance:* 0 disagreements over the 451 and the 78, or each one
   a named row of the sitting below; the 335 with no items excluded by
   name. *As opened* (below, "What step 6 opened"): each disagreement a
   row whose finding is a fact of the captures, the owner's ruling or a
   question for the sitting, the set exact; an undecided item a
   disagreement of its own kind.
7. **The sitting.** Composed after step 6 from its disagreements — the
   `asks` of `tests/trade-oracle.toml`'s sitting findings — and
   K6's "what no sitting has asked", by the site-sitting skill, each
   search with its mutant: round t of `tools/trade-sheet.py`, 57
   searches (7a), the forms' presence shown by the captured ids and
   their numbers asked a kind of relation at a time. Beside it, round
   u, 107 searches of coverage — a category or a filter row no capture
   reached, 53 of 82 categories and 64 of 94 rows that run having met
   no item the site returned, so that the oracle reads the translation
   on what the owner's store does not hold — and round v, 109, the
   park's questions (owner, 2026-10-16: "yes let's fold them in. i'm ok
   doing the larger ones as well"). The owner sits
   them; the captures read; K6 and
   the catalogue amended and the "what no sitting has asked" park takes
   its rows off. Only where the intended meaning is still unsettled.
8. **The seat.** The third of the search's, by the seat skill: a URL
   from a friend, what in my stash matches, in a person's words — one
   of them bounding an `increased` stat, so the named stat value's park
   (`decisions/search.md`) meets its trigger or does not, on evidence;
   and, where a person would, a map searched by its Base and a weighted
   sum over a scourge stat, so the two questions 7f labelled
   (`blighted-base`, `scourge-in-sum`: `tests/trade-oracle.toml`) meet a
   reader;
   the fix round inside the step (V6), its faults held by
   `tests/seat_faults.rs`, anything new a park with a trigger.
9. **Close.** `TRADE-SLICE.md` gains the seat's rows and is cut to
   its closed form; the park entries rewritten (`decisions/trade.md`,
   `decisions/search.md`); `search/README.md`'s tracks table; this
   file deleted; the session-close skill.

## What the build owes

- Every count in a fixture worked by hand first (rule 3), and every
  count this brief states measured again by script before a fixture
  rests on it: step 2 found the brief's "every id once" untrue of the
  capture, step 3 its 812 rows (1,313) and 643 of 643 maps (1,105,
  48 unique), its 37 rows and 40 fields holding, and step 4 the
  catalogue's 230 explicit texts two ids share (227).
- M3 rerun after step 3, on the copy, the row in `search/M3-LEDGER.md`.
- A rule the catalogue does not state stops the step (rule 4): what a
  clause *means* comes to the owner or to the sitting; a name — a
  field's spelling, a flag — is the builder's and `--describe` prints
  it.
- A sat search is never composed again otherwise: the sheet is appended
  to and never edited, a round is composed from frozen input and never
  from a column the sitting's reading updates, and the gate holds the
  sheet to its script (`tools/trade-sheet.py --verify`; round v broke
  the rule `tools/trade_rows.py` keeps for its versions, the outside
  review of step 7).
- An answer never prints a command the same build refuses (rule 5):
  the translated text is parsed before it is shown.
- The review's checklist: `SEARCH-SLICE.md`, "Findings". Two shapes to
  expect here: a meaning read off the syntax (a group's statuses
  computed where the group is written, not where it is read), and a
  claim the code did not make (a status "exact" no capture has shown;
  a count stated before it was measured — this brief's first draft
  said 447 captures where the file holds 864).

## Open at the opening

Step 4 brought the openings to the owner (2026-10-15) and settled each;
every one is a row of the step 7 sitting, its words in K6 or in its
catalogue row's note.

- K6's six — an explicit id on a crafted or fractured line, a `not`
  group with a value, which number a many-numbered template compares,
  `term`, `uniquefoil`, the site's dps and quality: ambiguous until the
  sitting (owner: "No — all stay ambiguous").
- An id the site lists with two texts in one category (172 ids, 229
  pairs, measured again): unsupported until the sitting (owner:
  "Unsupported until the sitting"); the catalogue's `stat.text.two`.
- `weapon_filters.crit` on a gem (owner: "Let's use a sitting to resolve
  this so that it's aligned with the trade site searches."),
  `map_filters.area_level` on a vessel's `Monster Level` and the nine
  heist skills on a brooch's or cloak's `Level {0} in {1}` (owner: "Both
  ambiguous"): ambiguous, each note naming the wording the field leaves
  out.

What step 4 opened:

- A number the site's text writes as a digit (`…Phasing for 4 seconds
  on Kill`, `1 Added Passive Skill is …`; 1,613 of the table's 14,068
  pairs, which the binder refused until it was read) is taken as the
  line's own, the slot fixed to it (`arg2=4`), the bounds on the text's
  own `#`: the builder's reading, which none of the 864 captures shows,
  so ambiguous and a row for the sitting (owner, 2026-10-16: "i agree
  with your recommendation with the sitting row"); the catalogue's
  `stat.text.numbers`.
- A `count` or weighted group with no bound of its own, or with no
  filter the site leaves enabled, is unsupported: what the site makes of
  one is unasked — a row for the sitting. An `and`, `not` or `if` with
  none restricts nothing and writes nothing.
- q2 and q2b name the two ids of `+#% chance to Suppress Spell Damage`;
  the site answered 10,000 listings to one and none to the other (the
  fetch census), so the two are not alike there, and the translation
  writes both as one line, ambiguous (S66): a row for the sitting, and
  step 6's excess, which it cannot see.
- Every pseudo filter of the 864 captures is unbounded — 112 in `and`
  groups, 37 in `not`, 21 in `if` of captures with fetched items — and
  an unbounded pseudo is its presence, `has:` (C94; K6), so step 6 reads
  them rather than excluding them by their remainder.

What step 5 opened:

- What a printed request and a continuation carry beside the text: the
  site search itself, translated again at every read and the text held
  to it, where this said the report, which a caller would carry as a
  label, stale or edited (owner, 2026-10-16, choosing "Site search,
  re-translated"). The report is the answer's alone, and the tables'
  versions are the report's, the basis being the corpus's.

What step 6 opened (measured 2026-10-16, before the oracle was written;
the owner, verbatim: "I'm inclined to accept your recommendations, with
the caveat that I prefer using sittings to answer questions so that we
don't have to guess or over-generalize our design", then "yes, this
approach sounds good"):

- The extract's items lose what the search reads (the bestiary text,
  the item level): rebuilt from it, 3,438 of 4,733 items missed; as the
  site sent them, 497. The oracle reads `bodies.json` alone.
- Of the 497, every item fails one top-level clause, so a row names its
  clause. 377 are a fact the pseudo track found (a `not` on an eldritch
  form's id lets items through): the clause is ambiguous by
  `stat.text.eldritch`, the step's one translation change. 35 are the
  owner's totals rulings at 9c. The other 85 — the `reduced` spelling,
  `No Physical Damage`, q6's two lines that meet a bound together, a
  scourge line in a sum — are one question, whether the site's stat is a
  line or the item's stat however displayed, possibly summed; each a
  sitting row, and nothing built until the sitting answers. The
  material for that answer, measured at 7a through the translation and
  the deriver (each captured line asked the term its id translates to):
  of 10,930 captured lines with an id, 10,428 match their id's term,
  100 lie under ids the translation leaves unsupported, and 402 are
  missed, 72 displayed forms under 71 ids — the opposite word (35
  forms: `reduced` for `increased`, the inverse, `Lucky`), `No
  Physical Damage` (2), and a number elided or spelled (a 100 % chance
  dropped, `an` for one, singular for plural) or another text. Where
  this said 67 pairs and 401 lines it was a script's own templating.
- Until step 7, a clause on an `increased` id reports exact while the
  oracle's rows show it missing `reduced` lines: said here and in the
  rows, not labelled, since a label rule now would be a design the
  sitting replaces.

What step 7 found (the sitting, rounds t to w, sat and read 2026-10-16;
the rows are `tests/trade-oracle.toml`'s, each finding stating what 7e
builds; the owner, verbatim: "I accept all your recommendations."):

- A site stat is the item's stat however displayed: a `reduced`
  spelling under its `increased` id below nothing, and the inverse
  (t01–t04); an elided 100 % as 100 (t10), `an` as one (t12); a plural
  the same number (t27). An explicit id holds the stat's crafted and
  fractured lines, and a bound compares their sum (t13–t20; q6's item at
  157 and not 158). `No Physical Damage`'s number is hidden — about
  -120, and not one number (t07–t09, w01–w05) — so presence alone.
- The property filters read more widely than the fields: `dps` and
  `pdps` at 20 % quality (t41, t42), `area_level` a vessel's `Monster
  Level` (t46), a heist skill a lockpick's `Level n in …` (t48),
  `quality` a catalyst's (t49); `crit` reads no gem (t44). Each field
  follows the site. Of a property the item lacks: no requirement meets
  any maximum (u061), a gem at its maximum level is at 100 % (u090).
- The smaller clauses: the free text is a whole name or base (t50,
  t51, w06, w07); a `count` with no bound restricts nothing (t37), a
  weighted sum with none needs one stat present (t38); a `not` with a
  value excludes the bound's meeting (t24); `uniquefoil` is the relic
  mark (t52); `map_blighted` the `Blighted` prefix (t55, t56); a key a
  site object sends null is absent (u059, the translation's reader).
- Coverage (round u, 104 searches of 53 categories and 64 rows no
  capture had reached) found the translation sound but for the three
  absent properties above; the park's questions (round v) agree with
  the tables but two ranged cells and two percentile groups unreached;
  a crucible defence line is the item's own (J33's question); the site
  gives a percentile to an item showing no defence, which stays
  lacked (the owner's acceptance). Open: e043's weighted sum of 28; the
  crucible mark (t54 did not part its readings); grafts, listed in no
  league; Maven's Invitations, listed (4,811) and no item sent (u025,
  v092, v093: a null for every listing fetched), so `map.invitation`
  met no item the oracle could read. The tracks
  whose scripts need the clones were read over the same captures once
  the clones were back (`poe1`, awakened-poe-trade, PathOfBuilding): the
  category table answers all 1,214 captured rows as the site does (five
  partial on class, the kind Xoph's Breachstone is), and four grouping
  cells meta.ts alone held are the site's now (t41, t42, v108); the
  percentile's stated rule reproduces 1,182 of 1,190 distinct items, the
  eight each a known reason; the defence twins separate as before.
- The map tier table: parked (the owner, 2026-10-16: "Let's do a tier
  table for maps, either now or parked for later based on your
  judgement"), no registered source holding a tier its base does not
  name; such a map is ambiguous under `map_tier` until it is built.

**The choice before 7e** (decided 2026-10-17; the deliberation is
`brainstorming-notes/37-the-stat-value.md`). `sum( … )` adds the
numbers of the lines it selects, with no sign and no constant, so a
bound on a stat with an opposite-sign form (`reduced`) or a numberless
one (an elided 100 %, `an`) has no exact text. Presence over every form,
the explicit sum and the same-sign forms are exact today. The owner
chose the translation alone for the release, and parked a named stat
value, `stat("…")`, with its trigger (`decisions/search.md`, "Parked").
What 7e builds, under that choice:

- A bound on a stat is `sum(line(T source=…).arg1)` over the sources
  the id admits (the stat-summed finding), with the line's presence
  beside a maximum, since a sum of nothing is zero (C95).
- A stat with a numberless or an opposite-sign form is ambiguous with
  a note that says what would make it exact and names the park; the
  bound stays on its numbered, same-sign forms; no alternative is
  computed from the bound, so the text means the same after an edit.
  `No Physical Damage`, and any form no sitting read, is presence
  alone.
- The forms are data, once: a forms table beside the catalogue —
  which texts have an opposite word, which elide to a numberless
  display, the exceptions — reference data (C106) read by the
  translation alone, so the park lifts it rather than finds it again.
  One test walks the vocabulary: a `#% chance to X` whose `X` is itself
  a listed stat (7 of 372), a `have #% chance to` form likewise (2 of
  286), a text carrying both sign words (4), each refused by name.
- The oracle's rows these leave ambiguous stay in the set under their
  finding; a row whose clause the build makes exact leaves it.

The material: the forms measured at 7a (72 under 71 ids, through the
translation and the deriver), the sitting's readings above, the totals
table's rows with their weights (`totals.rs`, *a weight*), and
`tree::sum_of_lines`.

**What 7e built** (2026-10-17; the oracle's rows, `tests/trade-oracle.toml`):

- Every `ruling` finding as it states, under the choice above, but the
  two the search counts against the site by the owner's earlier rulings
  (the twin, the socketed gems): a bound is the stat's sum over the
  lines its id admits, the line's presence beside a maximum — and beside
  a minimum at or below zero, the same reason the brief gives, which its
  rule did not reach (w03's `min` -120 would otherwise meet every item
  lacking the stat); presence over every form; a bound on a stat with a
  form no sum holds ambiguous by `stat.text.forms`, naming the park; the
  fields as the site reads them (`area_level`, the heist skills,
  `quality`, `experience`), the damage at 20 % quality
  (`pseudo.pdps_q20`, `pseudo.dps_q20`: within one of the site's at the
  mean, 902, 181 and 12 of the captures' 1,095 weapons), no requirement
  under a maximum, `map_tier` ambiguous. 101 items left the set in 17
  rows and none came in: 6,547 of 7,135 matched, 588 missed in 64 rows,
  what stays ambiguous held to its row by the test.
- The forms. The brief's blanket rules, measured against the PoE1
  export's stat descriptions during the step, named forms the game never
  displays: `#% chance to X` elides at 100 % for 212 of the 372 texts the
  export joins and not for 160 (`Suppress Spell Damage`, in the owner's
  q2), and 316 of 2,235 joined texts with a sign word have no opposite
  display. The owner, 2026-10-17, choosing among three: "Game data, rules
  beside (Recommended)". So the game's forms are generated
  (`tools/trade-forms-table.py`, `reference/trade-game-forms-v1.toml`:
  6,841 texts joined, 2,712 forms), the rules stand for the texts the
  export joins nowhere, and `reference/trade-forms-v1.toml` holds them,
  the 14 forms the captures show that neither gives, and 14 refusals by
  name — every form that is a listed text, 6 the game's, 1 a row's, 7 by
  the `have` elisions; none by `#% chance to`: every text whose elided
  form would be a listed one is joined, and the game elides none of them. Every one of the captures' 21,616 lines under a
  hash id is met by its id's presence term but 56 under an id left
  unsupported and 5 under a refusal (`tests/trade_forms.rs`).
- Its outside review (three findings): the generator took every string
  of a game entry for one stat's, and so read an enumeration's other
  values (Legion beside Blight) and another stat's flag (a warcry's
  infinite Power beside its counting) as forms, matched and called exact;
  now two strings are one stat's where every stat their conditions part
  on is a number one of them shows, `negated` read (2,687 forms, the
  witnesses fixtures), and a wording the game data gives that no capture
  shows labels its clause `stat.text.wording` — matched, never vouched
  for (the owner's choice, 2026-10-17); one capture's form it dropped is a
  hand row again (15). The damage at 20 % ignored `Quality does not
  increase Physical Damage`, which every capture carried at 20 % or more:
  read now as the defences read their enchant, and asked below 20 %. The
  third, the findings below, is step 7f's.
- Step 7f (2026-10-17; the owner: "yes, go ahead, including 7f") built
  what "What step 7 found" settled outside the oracle's rows: the free
  text a whole name or base, `(name=T or base=T)` (w06, w07; t50, t51); a
  `count` with no bound the site's no-op (t37), a weighted sum with none
  one of its stats present (t38); a `not` with a value exact, the bound's
  meeting excluded (t24); `uniquefoil` the relic mark (t52);
  `map_blighted` the `Blighted` prefix (t55, t56), its `uberblighted`
  unasked; a key a site object sends null absent (u059); `crit` no gem's,
  `-category:gem` (t44); and J33's crucible line the item's own in the
  percentile (t54: 59; the copy 2,250 and 98). The catalogue 143 rows, 80
  exact, 18 ambiguous, 36 unsupported. Five partial searches now run, 50
  items, 40 matched; t55's ten are a new question, `blighted-base`: the
  site's Base reads `Blighted Map (Tier 13)` as `Map (Tier 13)`. The oracle
  765 searches, 7,185 items, 6,587 matched, 598 missed in 65 rows. Left as
  they were, each its note's: the crucible mark (t54 did not part its
  readings), which number a many-numbered text compares, the scourge line
  in a sum (e043), and the step-7 opens above.
- Its outside review (three findings): an unbounded weighted sum was read
  as presence even where a stat carried its own bound, which t38 never
  asked — that one keeps K5's refusal; `blighted-base` stood under a Base
  reported exact — the oracle now holds every expected disagreement under
  a label but the rulings by which the owner differs from the site
  (`differs`), so the Base naming a map's tier is ambiguous
  (`query.type.map`) and so is a scourge stat in a weighted sum
  (`stat.scourge.summed`, e043); and a key sent null was absent at one
  reader and not others — now taken out once, where the query enters the
  translation, and a generated query with a null beside every key
  translates as itself. The catalogue 145 rows, 80 exact, 20 ambiguous.
  Its review found a stat's reversed bounds aborting the translation, the
  sum built after the binder had read the line; the node is bound alone
  now, as every node a clause lands is (`80608255`). The outside review
  approved the round, 2026-10-17: step 7 is closed, the seat next.