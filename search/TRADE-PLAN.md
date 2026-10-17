# Trade-URL import, inbound, PoE1 — the brief

Step 3 of `brainstorming-notes/36-the-road-to-the-agent-release.md`,
opened 2026-10-15 under C113 and C114 (`decisions/trade.md`, with the
contract detail K6); closes by 2026-11-15. This file is the brief the
build runs under, in the mold of `search/BUILD-PLAN.md` at `453a4429`:
the steps, each with its acceptance, what the build owes, and what it
leaves out. It is deleted at the close and cited at its commit; the
record is `TRADE-SLICE.md`, opened 2026-10-17. The slice's
rules 1 to 10 (`SEARCH-SLICE.md`, "The plan's rules") bind every step
below as they bound the search's, and its shapes of fault, with the
record's, are the review's checklist. Reviewed from outside before the
build (Astra with Codex, 2026-10-15, seven findings): the catalogue as
a table, the group rule, the fidelity word, the scope rule and step 4's
witnesses are what that review asked for.

Steps 0 to 7 are closed (`77666ea4`, the last commit that holds the
brief whole: their text and acceptance, what each step opened, what
step 7 found, the choice before 7e and what 7e and 7f built); the
record holds their commits, what they taught and the owner's verdicts;
the seat and the close stay below, with what the closed steps left
open.

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
the next. Steps 0 to 7: the brief at `77666ea4`.

8. **The seat.** The third of the search's, by the seat skill, under
   `audits/trade-2026-10/seat/PLAN.md` (both surfaces, serial, a
   debrief after each report): a URL
   from a friend, what in my stash matches, in a person's words — one
   of them bounding an `increased` stat, so the named stat value's park
   (`decisions/trade.md`) meets its trigger or does not, on evidence;
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

## Open for the seat (what steps 0 to 7 left, each where it stands)

- A bound on a stat with an opposite-sign or numberless form is
  ambiguous by `stat.text.forms`, naming the named stat value's park
  (`decisions/trade.md`); the seat's `increased` ask meets its trigger
  or does not, on evidence.
- `blighted-base` (`query.type.map`) and `scourge-in-sum`
  (`stat.scourge.summed`, e043): `tests/trade-oracle.toml`, labelled;
  the seat's map by its Base and weighted sum over a scourge stat meet
  a reader.
- Each its note's, for the next sitting: the crucible mark (t54 did not
  part its readings); which number a many-numbered text compares;
  grafts, listed in no league; Maven's Invitations, listed and no item
  sent (`map.invitation` met no item the oracle could read); two ranged
  cells and two percentile groups round v did not reach ("what no
  sitting has asked", `decisions/search.md`).
- The map tier table: parked (`decisions/trade.md`; the owner,
  2026-10-16: "Let's do a tier table for maps, either now or parked for
  later based on your judgement"); a map whose base names no tier is
  ambiguous under `map_tier` until it is built.
