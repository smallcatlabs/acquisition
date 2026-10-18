# The trade import — the record

Step 3 of `brainstorming-notes/36-the-road-to-the-agent-release.md`:
a trade site search URL, its id or its query JSON as a third spelling
of a query — decoded with no network, written as the language's text,
run and labelled clause by clause (C113) — and every numeric property
the site's filters read a field by its displayed name (C114). Opened
2026-10-15 at `6745e5ba` under the brief `search/TRADE-PLAN.md`; steps
0 to 7 closed at `77666ea4`, where the brief's full text stands; step 8
(the seat) closed 2026-10-17; 8b (what the seat asked of the translation)
was built 2026-10-18, and 9 (the close) is open. This file is its record in the
mold of `PLATFORMS-SLICE.md`, opened before the close on the owner's
acceptance of 2026-10-17 so the seat's rows have a home; the close
fills what is marked open. The search slice's rules 1 to 10
(`SEARCH-SLICE.md`, "The plan's rules") bound every step and bind every
change here; its shapes of fault and the ones below are the review's
checklist. The rulings are `decisions/trade.md` (C99, C113, C114, C119,
the contract detail K5 and K6) and the language's, `decisions/search.md`;
the mechanism is the module docs of `trade.rs`, `trade/` and
`properties.rs`; the oracle is `tests/trade-oracle.toml`, every expected
disagreement under a label; the sitting is `tools/trade-sheet.py`'s
rounds t to w. Nothing here is a second authority.

## Final state (at step 7's close; the close measures again)

- The tables: the catalogue `trade-catalogue-v1.toml` (145 rows, 80
  exact, 20 ambiguous at 7f's review), the stat table
  `trade-stats-v1.toml` (the capture of 2026-09-12 as its version), the
  game's forms `trade-game-forms-v1.toml` (2,687 forms from 6,841 joined
  texts) and the hand forms `trade-forms-v1.toml` (the captures' rows
  and 15 refusals by name), the properties `properties-v1.toml`.
- The code: `trade.rs` (the decoder), `trade/catalogue.rs`,
  `trade/stats.rs`, `trade/forms.rs`, `trade/translate.rs`,
  `properties.rs`; the CLI's `--trade <URL|ID>` and
  `--trade-file <FILE|->`, the MCP's `{"trade": …}`, `--describe
  trade`. The makers `tools/trade-stats-table.py` and
  `tools/trade-forms-table.py`; the sitting's `tools/trade-sheet.py`,
  `tools/trade-split.py`, `tools/trade-captures.py`,
  `tools/trade_query.py` and `tools/sources.py`, each self-tested in
  `tools/docs-check.sh` §9.
- Not built, each parked with its trigger (`decisions/trade.md`): the
  outbound half and PoE2, a named stat value, user-authored weighted
  sums, a map tier table, the sat rounds as data.

## Step ledger

| Step | Commits | Where it lives |
| --- | --- | --- |
| opened | `6745e5ba` | C113, C114, K6 ruled; the raw captures restored |
| 0 · the catalogue | `cc7f07da`; `d6e312ba` (the tier is in the base type) | `reference/trade-catalogue-v1.toml`, `trade/catalogue.rs`; `tests/trade_catalogue.rs` |
| 1 · decode | `79a72db3`; the fix round `7ea3c309`; the second look `8641cd4b` | `trade.rs`; `tests/trade_decode.rs` |
| 2 · the stat table | `ca819bac`; reviewed `dfa18980` | `tools/trade-stats-table.py`, `reference/trade-stats-v1.toml`, `trade/stats.rs`; `tests/trade_stats.rs` |
| 3 · the property fields | the brief before it `04b510c0`; `59f9aa1d`; the owner's verdicts `84cd833d`; reviewed `ec062843`; the brief after it `2f1f1d83` | `properties.rs`, `reference/properties-v1.toml`, `pseudo.rs` (the damage at 20 %); `tests/properties.rs`, `tests/trade_dps.rs`; the M3 row (`search/M3-LEDGER.md`) |
| 4 · the translation | `19b0baf5`; reviewed `c1e674a5`, `fa9b0edd`; K6 into the code `9e891e37` | `trade/translate.rs`; `tests/trade_translate.rs`, `tests/trade-translate.toml`, the generators' site query |
| 5 · the surface | `04b43ea6`; reviewed `56b3c5e9` | `answer.rs`, the CLI's `search_cmd.rs`, the MCP's `main.rs`; `tests/search_json.rs`, the MCP's tests; the references |
| 6 · the oracle | `34a99e38`; reviewed `e2900667` | `tests/trade_oracle.rs`, `tests/trade-oracle.toml`; `tools/trade-captures.py`, `search/pseudo-stats/data/bodies.json` |
| 7 · the sitting | rounds t to v `7032f767`, `e062401e`; round w `6af5b846`; the reading `2acda56a`; reviewed `bef2ba58`; the clones' tracks read `606b4a17`, reviewed `555c4f58`, `373c541a`, `0ff8e09c`; the shape `c14c95ee`; the choice before 7e `33a96108`; 7e `0e91048d`, reviewed `39725d10`; 7f `8f293ff3`, reviewed `6fdcc581`, `80608255`; closed `77666ea4` | `tools/trade-sheet.py` rounds t–w, `search/pseudo-stats/MANIFEST.md`; `trade/forms.rs`, `tools/trade-forms-table.py`, the two forms tables; `tests/trade_forms.rs`; the oracle's rows and labels; `brainstorming-notes/37-the-stat-value.md` |
| 8 · the seat | the plan `388406ef`–`18a3d7b4`; the doors `45c0d377`, `875a6729`, `e459bed7`, `3e84db17`; the briefs `538022d6`, review 1 `157f7078`; the runs `5eea0ac6`–`df03ec95`; the round `a3ad3273`–`1eecf3e0`; closed with the record | `audits/trade-2026-10/seat/README.md`; the seat faults of the CLI and the search crate (`seat_faults.rs`) and the MCP (`search_tools.rs`); the catalogue's `says` |
| 8b · the stake of an ambiguous clause | built 2026-10-18 at `5dbbad12` under C119, `brainstorming-notes/38-the-answer-an-agent-reads.md`, step 3: four rows' other readings — the Blight-ravaged maps both ways, the opposite form under a minimum, a scourge line's sum without it; a bound within one and the map tier parked (`decisions/trade.md`) | `trade/translate.rs` (where a reading applies), `answer.rs` (what it adds and drops), the catalogue's `other`; `tests/trade_translate.rs` and `tests/trade-translate.toml` (the seat's T4, T5, T6 rows), `seat_faults.rs`, `search_tools.rs` |
| 9 · the close | open | this record cut to its closed form; the brief deleted, cited at its last commit; `search/README.md`'s tracks table |

## The parks it fired or filed

- Fired and built in part: the trade translation (the inbound half;
  the outbound half and PoE2 stay parked, `decisions/trade.md`); "what
  no sitting has asked the site" (`decisions/search.md`: round v asked
  the list; two ranged cells and two percentile groups unreached stay
  under the next sitting).
- Filed, each with its trigger (`decisions/trade.md`): a named stat
  value, `stat("…")` (the choice before 7e, note 37); user-authored
  weighted sums; a map tier table for bases naming no tier; the capture
  tests that rebuild an item from the extract's selection; the sitting's
  sat rounds as data.

## Shapes of fault — the review checklist for the translation

The search slice's table applies whole, and two of its rows name this
build among their returns ("A second maker", "A claim the code did not
make"). These are the shapes this build added, each a fault that came
back until it was named.

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **Evidence read off whatever is on disk.** A generator wrote what the files present gave it: a script without its clone turned 16 grouping members out; an extract over a restore without t01 wrote 1,149 searches for 1,150; a local capture joined to the committed extract by an item's place and base took a replaced capture's marks for the old item's; the extract's two files, written one after the other, paired across generations by their counts | the trade import's tooling, three outside reviews in a row (`606b4a17`, `555c4f58`, `373c541a`) | every input declared and held to its record before any is read (`tools/sources.py`: a clone at its pin, a raw directory at its inventory); raw read by the extract alone; the extract's two files stamped with their raw (`_raw`) and paired in one place (`trade_query.fetched`, `tests/trade_oracle.rs`); each self-test in `tools/docs-check.sh` §9 |
| **A reading applied past the shape it asked.** A sitting's or a source's answer to one shape built for every clause that looks like it: `#% chance to X` elided for every such text where the game elides 212 of 372; every string of a game entry a form, an enumeration's other values among them; t38's unguarded weighted sum taken for a guarded one, a filter's own bound dropped | the trade import's 7e and its two reviews (`0e91048d`, `39725d10`, `8f293ff3`) | a reading applies where the clause has every property of the shape asked, any other keeping its status; the forms' identity rule (`one_stat`) and their kinds; a refusal kept for a guarded sum (`tests/trade-translate.toml`); no expected disagreement under an exact clause (`tests/trade_oracle.rs`) |
| **A group's statuses computed where the group is written, not where it is read** — a meaning read off the syntax, the brief's own warning | step 4's reviews (`c1e674a5`, `fa9b0edd`) | every group bound alone, every node a clause lands bound alone (`80608255`); a group's filters are the ones the site leaves enabled; the structural guard, every clause that runs owning one term path inside its group |

## What the oracle and the sitting taught

The numbers are the brief's at `77666ea4` and the oracle file's; a
number here was measured, never recalled.

- The oracle (step 6, measured 2026-10-16): rebuilt from the extract,
  3,438 of 4,733 items missed; as the site sent them, 497, every one
  failing one top-level clause — 377 a fact the pseudo track found (a
  `not` on an eldritch form's id), 35 the owner's totals rulings at 9c,
  85 one question, whether the site's stat is a line or the item's stat
  however displayed.
- The forms (7a, through the translation and the deriver): of 10,930
  captured lines with an id, 10,428 match their id's term, 100 lie
  under unsupported ids, 402 are missed — 72 displayed forms under 71
  ids. Against the game's export (7e): `#% chance to X` elides at 100 %
  for 212 of 372 texts and not for 160; 316 of 2,235 texts with a sign
  word have no opposite display. Every one of the captures' 21,616
  lines under a hash id is met by its id's presence term but 56 under
  an unsupported id and 5 under a refusal.
- After 7f: 765 searches, 7,185 items, 6,587 matched, 598 missed in 65
  rows, every row under a label or a `differs`; five partial searches
  run, 50 items, 40 matched. The damage at 20 % quality is within one
  of the site's at the mean: 902, 181 and 12 of the captures' 1,095
  weapons.
- The property fields (step 3): the defences against 1,313 `extended`
  rows over 4,819 listings; `map_tier` on 1,105 maps over 35 bases, 48
  of them unique; the load's move 478 against 477, medians
  (`search/M3-LEDGER.md`).
- The sitting (rounds t to w, 2026-10-16): t 57 searches; u 104, of 53
  categories and 64 rows no capture had reached; v 109, the park's
  questions. The tracks that need the clones, read over the same
  captures: the category table answers all 1,214 captured rows as the
  site does; the percentile's rule reproduces 1,182 of 1,190 distinct
  items, the eight each a known reason.

## Owner verdicts, verbatim

- K6's six openings (2026-10-15): "No — all stay ambiguous". An id the
  site lists with two texts: "Unsupported until the sitting".
  `weapon_filters.crit` on a gem: "Let's use a sitting to resolve this
  so that it's aligned with the trade site searches." `area_level` and
  the heist skills: "Both ambiguous".
- A number the site's text writes as a digit (2026-10-16): "i agree
  with your recommendation with the sitting row".
- What a printed request carries (2026-10-16): "Site search,
  re-translated".
- Step 6's openings (2026-10-16): "I'm inclined to accept your
  recommendations, with the caveat that I prefer using sittings to
  answer questions so that we don't have to guess or over-generalize
  our design", then "yes, this approach sounds good".
- Rounds u and v (2026-10-16): "yes let's fold them in. i'm ok doing
  the larger ones as well".
- Step 7's findings (2026-10-16): "I accept all your recommendations."
  The map tier table: "Let's do a tier table for maps, either now or
  parked for later based on your judgement".
- The forms' source (2026-10-17): "Game data, rules beside
  (Recommended)". Step 7f: "yes, go ahead, including 7f".
- This record and `decisions/trade.md` (2026-10-17): "i accept your
  recommendations. go ahead".
- The seat's site searches (2026-10-17): "You make them on the site
  (Recommended)"; the map search by tier and blighted, the site's form
  offering no tiered Base: "yes, agreed, and record what we found
  before proceeding".
- Round x's six findings (2026-10-18; note 38, step 5): "accepted", then
  "accepted" for the sixth, the written digit.

## Where the open observations went

| Observation | Went to |
| --- | --- |
| A bound on a stat with an opposite-sign or numberless form | the named stat value park (`decisions/trade.md`); the clause ambiguous by `stat.text.forms`; the seat's `increased` ask |
| e043's weighted sum of 28, a scourge stat in a weighted sum | `stat.scourge.summed`; the user-authored weighted sums park; the seat's `scourge-in-sum` |
| `blighted-base`: the site's Base reads `Blighted Map (Tier 13)` as `Map (Tier 13)` | `query.type.map`; the sitting alone — the site's form offers no tiered map Base (its item list, `search/trade-query/data/items-2026-09-12.json`: 6,005 types, none naming a tier, a map's base the one type `Map`): a person cannot make the shape t55's composed URL asked, so no seat reaches it (2026-10-17) |
| The crucible mark (t54 did not part its readings) | its note, unchanged at round x: only a `not` over the site's 2,492 crucible ids could part the two readings, past what the site takes (220 ids refused) |
| Which number a many-numbered text compares | round x, x04–x07 and x13–x17: a text of exactly two numbers by their average, as a `# to #` line (x14 at 1250–1253 found 252 of 252, x15 at 1254–2400 none); `avg` on any two-number template, `a_two_numbered_stat_is_compared_by_its_average`; three or more stay `stat.text.many`, unasked |
| Grafts, listed in no league; Maven's Invitations, listed and no item sent | its note; `map.invitation` met no item the oracle could read |
| A Blight-ravaged map under `map_blighted`, outside its `Blighted` prefix (the trade seat: 13 at tiers 14–16) | round x, x01–x03: the site counts none; `map_blighted` exact, its other readings gone, `a_blight_ravaged_map_is_not_a_blighted_one` (the owner, 2026-10-18: "accepted") |
| A `Level` requirement GGG marks `(gem)`, read as the item's `reqlevel` (the trade seat: mcp-opus-1 call 39, not shown) | round x, x08 and x18: not shown — every item fetched owed its requirement to its base, whatever its item level (a normal Vaal Regalia of item level 7 requires 68), and no site filter selects an item by its socketed gem; parked in `decisions/trade.md`, `reqlevel` exact as displayed |
| A map whose base names no tier | the map tier table park, `map_tier` ambiguous until it is built |
| Two ranged cells and two percentile groups round v did not reach | the two ranged cells asked at round x (x10, x11: ten agreeing items each, `ranged-expected-copy.csv` "asked"); the percentile lines and S4 are the pseudo track's own round, "what no sitting has asked" (`decisions/search.md`) |
