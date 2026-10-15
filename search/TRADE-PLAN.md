# Trade-URL import, inbound, PoE1 — the brief

Step 3 of `brainstorming-notes/36-the-road-to-the-agent-release.md`,
opened 2026-10-15 under C113 and C114 (`decisions/search.md`, with the
contract detail K6); closes by 2026-11-15. This file is the brief the
build runs under, in the mold of `search/BUILD-PLAN.md` at `453a4429`:
the steps, each with its acceptance, what the build owes, and what it
leaves out. It is deleted at the close and cited at its commit; the
record is `SEARCH-SLICE.md`, which gains the step's rows. The slice's
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
| 864 sitting captures: 451 with ten fetched items, 78 with one to nine, 335 with none | `search/pseudo-stats/data/captures.json` | the oracle (the 451 and the 78); the 20 %-quality figures (`extended.ar|ev|es`, 812 rows) |
| The site's property bridge (S55) | `grammar.json`, `property_type_to_field` | the property table |

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
   way, by S26's rule. *Acceptance:* every field asked every way a
   number is (compare, sort, sum, count, undecided); the defences
   against the captures' `extended` figures — 812 rows — and the 37 of
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
   request it prints carries the translated text and the report, and
   `--request` reads it back. `--describe trade` is the catalogue.
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
   name.
7. **The sitting.** Composed after step 6 from its disagreements and
   K6's "what no sitting has asked", by the site-sitting skill, each
   search with its mutant; the owner sits it; the captures read; K6 and
   the catalogue amended and the "what no sitting has asked" park takes
   its rows off. Only where the intended meaning is still unsettled.
8. **The seat.** The third of the search's, by the seat skill: a URL
   from a friend, what in my stash matches, in a person's words; the
   fix round inside the step (V6), its faults held by
   `tests/seat_faults.rs`, anything new a park with a trigger.
9. **Close.** `SEARCH-SLICE.md` gains the step's rows (its budget is
   24,000 bytes; 21,355 at opening — a record of its own if it does
   not fit); the park entry rewritten; `search/README.md`'s tracks
   table; this file deleted; the session-close skill.

## What the build owes

- Every count in a fixture worked by hand first (rule 3).
- M3 rerun after step 3, on the copy, the row in `search/M3-LEDGER.md`.
- A rule the catalogue does not state stops the step (rule 4): what a
  clause *means* comes to the owner or to the sitting; a name — a
  field's spelling, a flag — is the builder's and `--describe` prints
  it.
- An answer never prints a command the same build refuses (rule 5):
  the translated text is parsed before it is shown.
- The review's checklist: `SEARCH-SLICE.md`, "Findings". Two shapes to
  expect here: a meaning read off the syntax (a group's statuses
  computed where the group is written, not where it is read), and a
  claim the code did not make (a status "exact" no capture has shown;
  a count stated before it was measured — this brief's first draft
  said 447 captures where the file holds 864).

## Open at the opening

- Whether an explicit id matches a crafted or fractured line of the
  same stat on the site; a `not` group with a value; which number a
  many-numbered template compares; `term`; `uniquefoil`; the site's
  dps and quality. Each ambiguous until the sitting (K6).
- The names of the 20 %-quality defence fields: the builder's,
  printed by `--describe`.
