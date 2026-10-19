# The trade import — closed record

Step 3 of `brainstorming-notes/36-the-road-to-the-agent-release.md`:
a trade site search URL, its id or its query JSON as a third spelling
of a query — decoded with no network, written as the language's text,
run and labelled clause by clause (C113) — and every numeric property
the site's filters read a field by its displayed name (C114). Opened
2026-10-15 at `6745e5ba`, closed 2026-10-18. This file is the
permanent short record; the open record, the brief (`search/TRADE-PLAN.md`)
and note 38, the plan of the work after the seat, are whole at
`16bba23e`; the brief's steps 0 to 7 at `77666ea4` and step 8 at
`6e31d630`. Nothing here is a second authority.

No further step of the trade import is authorized by this record: the
outbound half and PoE2 are parks (`decisions/trade.md`), and new work
at the boundary opens under its own record.

## Authority

The rulings are `decisions/trade.md` (C99, C113, C114, C119, the
contract detail K5 and K6), the language's `decisions/search.md`, and
the answer's words `decisions/frontends.md` (C115, C116); the
mechanism is the module docs of `trade.rs`, `trade/`, `properties.rs`
and `answer.rs`; the oracle is `tests/trade-oracle.toml`, every expected
disagreement under a label; the sitting is `tools/trade-sheet.py`'s
rounds t to x. The search slice's rules 1 to 10 (`SEARCH-SLICE.md`,
"The plan's rules") bound every step and bind every change here.

## Final state

- The tables: the catalogue `trade-catalogue-v1.toml` (each ambiguous
  row that admits another reading carries it, `other`), the stat table
  `trade-stats-v1.toml` (the capture of 2026-09-12 as its version), the
  game's forms `trade-game-forms-v1.toml` (2,687 forms from 6,841 joined
  texts) and the hand forms `trade-forms-v1.toml`, the properties
  `properties-v1.toml`.
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
  outbound half and PoE2, a named stat value (fired, queued), an other
  reading for a bound within one and for a map tier (fired, queued),
  user-authored weighted sums, a map tier table, a gem-set level, the
  sat rounds as data, the capture tests rebuilt from the body.

## Step ledger

| Step | Commits | Where it lives |
| --- | --- | --- |
| opened | `6745e5ba` | C113, C114, K6 ruled; the raw captures restored |
| 0 · the catalogue | `cc7f07da`, `d6e312ba` | `reference/trade-catalogue-v1.toml`, `trade/catalogue.rs`; `tests/trade_catalogue.rs` |
| 1 · decode | `79a72db3`; `7ea3c309`, `8641cd4b` | `trade.rs`; `tests/trade_decode.rs` |
| 2 · the stat table | `ca819bac`; `dfa18980` | `tools/trade-stats-table.py`, `reference/trade-stats-v1.toml`, `trade/stats.rs`; `tests/trade_stats.rs` |
| 3 · the property fields | `04b510c0`, `59f9aa1d`, `84cd833d`; `ec062843`, `2f1f1d83` | `properties.rs`, `reference/properties-v1.toml`, `pseudo.rs`; `tests/properties.rs`, `tests/trade_dps.rs`; the M3 row |
| 4 · the translation | `19b0baf5`; `c1e674a5`, `fa9b0edd`, `9e891e37` | `trade/translate.rs`; `tests/trade_translate.rs`, `tests/trade-translate.toml` |
| 5 · the surface | `04b43ea6`; `56b3c5e9` | `answer.rs`, the CLI's `search_cmd.rs`, the MCP's `main.rs`; `tests/search_json.rs`, the MCP's tests |
| 6 · the oracle | `34a99e38`; `e2900667` | `tests/trade_oracle.rs`, `tests/trade-oracle.toml`; `tools/trade-captures.py`, `search/pseudo-stats/data/bodies.json` |
| 7 · the sitting | rounds t–w `7032f767`, `e062401e`, `6af5b846`; read `2acda56a`, `bef2ba58`; the clones' tracks `606b4a17`, `555c4f58`, `373c541a`, `0ff8e09c`; 7e `0e91048d`, `39725d10`; 7f `8f293ff3`, `6fdcc581`, `80608255`; closed `77666ea4` | `tools/trade-sheet.py`, `search/pseudo-stats/MANIFEST.md`; `trade/forms.rs`, `tools/trade-forms-table.py`, the two forms tables, `tests/trade_forms.rs`; note 37 |
| 8 · the seat | `388406ef`–`b24fc48b` | `audits/trade-2026-10/seat/README.md`; `seat_faults.rs` (the CLI's and the search crate's), `search_tools.rs`; the catalogue's `says` |
| 8a · the answer's weight (C115) | `e57db22b`–`91d17206` | `answer.rs`'s order, `view.routes`, `tabs` counts first |
| 8b · coverage (C118) | `d483a199`–`07b9a090` | `place.fetched_at`, the scope's never fetched by type and its `tabs` pointer |
| 8c · the stake of an ambiguous clause (C119) | `5dbbad12`–`39d06f5d` | `trade/translate.rs` (where a reading applies), `answer.rs` (what it adds and drops), the catalogue's `other`; the seat's T4–T6 rows in `tests/trade-translate.toml` |
| 8d · the tab facts and the ids (C117, C116) | `6b9206fe`–`165236e5` | `tab.public`, `tab.remove_only`, `price.side`; `acquisition-cli/tests/no_ids.rs` |
| 8e · round x | `48751e67`–`39b18b32` | rounds x01–x18 in `tools/trade-sheet.py`; `map_blighted`, `map_uberblighted` exact; a written digit; two numbers by their mean |
| 8f · the re-sit | `92773808`–`16bba23e` | `audits/trade-2026-10/re-sit/README.md`; `tabs --type --never-fetched` and seven small fixes |
| 9 · the close | this commit | this record; the brief and note 38 deleted; `search/README.md`'s tracks |

## Shapes of fault — the review checklist for the translation

The search slice's table applies whole, and two of its rows name this
build among their returns ("A second maker", "A claim the code did not
make"). These are the shapes this build added, each a fault that came
back until it was named.

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **Evidence read off whatever is on disk.** A generator wrote what the files present gave it: a script without its clone turned 16 grouping members out; an extract over a restore without t01 wrote 1,149 searches for 1,150; a local capture joined to the committed extract by place and base took a replaced capture's marks; the extract's two files paired across generations by their counts | three outside reviews in a row (`606b4a17`, `555c4f58`, `373c541a`) | every input declared and held to its record before any is read (`tools/sources.py`: a clone at its pin, a raw directory at its inventory); raw read by the extract alone; the extract's two files stamped with their raw and paired in one place (`trade_query.fetched`, `tests/trade_oracle.rs`); `tools/docs-check.sh` §9 |
| **A reading applied past the shape it asked.** A sitting's or a source's answer to one shape built for every clause that looks like it: `#% chance to X` elided for every such text where the game elides 212 of 372; every string of a game entry a form; t38's unguarded weighted sum taken for a guarded one | 7e and its two reviews (`0e91048d`, `39725d10`, `8f293ff3`); C119's other readings | a reading applies where the clause has every property of the shape asked, any other keeping its status; the forms' identity rule (`one_stat`) and kinds; a refusal kept for a guarded sum; no expected disagreement under an exact clause (`tests/trade_oracle.rs`) |
| **A group's statuses computed where the group is written, not where it is read** | step 4's reviews (`c1e674a5`, `fa9b0edd`) | every group bound alone, every node a clause lands bound alone (`80608255`); the structural guard, every clause that runs owning one term path inside its group |
| **A scope read one level deep.** The league a query fixes, or what a fetch counts, read off the top of the tree or a row count | note 38's step 2 review (`8b4446f4`), the re-sit's review 2 (`e9f5330b`) | `fixed_league` reads through nested conjunctions; a fetch of no rows is a fetch |

## What the oracle, the sitting and the seats taught

A number here was measured, never recalled; each is its source's.

- The oracle (step 6): rebuilt from the extract, 3,438 of 4,733 items
  missed; as the site sent them, 497, every one failing one top-level
  clause — 377 a `not` on an eldritch form's id, 35 the owner's totals
  rulings, 85 whether the site's stat is a line or the item's stat
  however displayed. After 7f: 765 searches, 7,185 items, 6,587 matched,
  598 missed in 65 rows, every row under a label or a `differs`.
- The forms (7a, 7e): of 10,930 captured lines with an id, 10,428 match
  their id's term; `#% chance to X` elides at 100 % for 212 of 372
  texts; 316 of 2,235 texts with a sign word have no opposite display.
- The property fields (step 3): the defences against 1,313 `extended`
  rows over 4,819 listings; `map_tier` on 1,105 maps over 35 bases; the
  damage at 20 % quality within one of the site's at the mean for 902,
  181 and 12 of 1,095 weapons; M3's move 478 against 477.
- The sitting (rounds t to x): t 57 searches, u 104, v 109 (the
  park's questions), w what t to v left undecided, x 18 (the seat's
  queue). The category table
  answers all 1,214 captured rows as the site does; the percentile's
  rule reproduces 1,182 of 1,190 distinct items.
- The seat and the re-sit: eight runs and then four pasted every URL
  and got a translated, labelled, running query first time; every fault
  sat at the answer's edges — weight, coverage, the stake of an
  ambiguity, a tab's facts, ids. At the re-sit, every run answered the
  never-fetched unique from the scope line and settled each clause that
  carries another reading from "adds 0, drops 0"; the armour clause,
  which carries none, cost three runs two or three hand checks each; no run spent a call
  on the terms block, and each re-read it once.

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
  (Recommended)"; the map search by tier and blighted: "yes, agreed,
  and record what we found before proceeding".
- The seat's six rulings (2026-10-18): "I accept all 6. Go ahead."
- Round x's six findings (2026-10-18): "accepted", then "accepted" for
  the sixth, the written digit.
- The re-sit's triage (2026-10-18): "I agree with your division …
  except let's build the never-fetched tabs list, since that helps with
  refreshes".
- The terms block at the close (2026-10-18): "Measured; park
  (Recommended)".

## Where the open observations went

| Observation | Went to |
| --- | --- |
| A bound on a stat with an opposite-sign or numberless form | the named stat value park, fired and queued (`decisions/trade.md`) |
| A scourge stat in a weighted sum (e043) | the oracle's label `stat.scourge.summed`; C119's other reading (T5 adds 0, drops 0); the user-authored weighted sums park |
| `blighted-base`: the site's Base reads `Blighted Map (Tier 13)` as `Map (Tier 13)` | dropped: the oracle's label `query.type.map` holds it, and the site's form offers no tiered map Base, so no person or seat can make the search |
| A Blight-ravaged map under `map_blighted` | ruled at round x (x01–x03): the site counts none, `map_blighted` exact |
| A `Level` requirement a socketed gem sets | the gem-set level park (`decisions/trade.md`), `reqlevel` exact as displayed |
| Which number a text of two numbers compares | ruled at round x (x14, x15): by their mean |
| Which number a text of three or more compares; grafts, listed in no league; Maven's Invitations, listed and no item sent; the percentile lines and S4 round v did not reach | the sitting park, its remainder (`decisions/search.md`, "What no sitting has asked the site") |
| Two ranged cells round v did not reach | asked at round x (x10, x11), ten agreeing items each |
| The crucible mark (t54 did not part its readings) | dropped: only a `not` over the site's 2,492 crucible ids could part them, past what the site takes; the catalogue row stays ambiguous |
| A map whose base names no tier | the map tier table park |
| The armour band's other reading, measured by hand at the re-sit | the other reading park for a bound within one, fired and queued (`decisions/trade.md`) |
| The page's empty default stat group | its park (`decisions/trade.md`), by design |
| The terms block, re-read once by every re-sit run | C115's measurement and a frontends park (`decisions/frontends.md`) |
| The sat rounds as data | its park; the brief's close dropped from its trigger, a sequence point being no trigger |
| The re-sit's faults past the alpha | the parks its record names, each by its fate (`audits/trade-2026-10/re-sit/README.md`, "The faults") |
