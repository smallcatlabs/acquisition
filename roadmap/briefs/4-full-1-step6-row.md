# 4-full-1, step 6.5c — An item under its row, and the row's ways

## Goal
At `acq web` a table's row opens its item as a card under the row,
and the reader stays where they were. The card's place is a link to
that place opened, and its query's outcome on it, term by term, is
one link away. A refused search keeps its query box, and the save
form says what a saved search leaves behind. It is step 6.5's last
part (#270, #194, #186, #190).

## Inputs
- `4-full-1-step6-design.md` §2, §3, §4 and §7's ruling ("open as a
  card under it's row").
- `4-full-1-step6-grouped.md` (6.5b): Decision 5, a window inside a
  group, and "6.5c: a card under a row, and one row height", the
  designer's answer, which this brief takes as written.
- `4-full-1-step6-places.md` (6.5a): `open`, the anchor, the drop of
  a field naming nothing.
- #270, #194, #186, #190.
- C16 (the URL names the state), C46 (semantics in Rust), C98 (one
  basis), C168 (a saved search is a query and its scope), C187 (the
  browser check).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **`item` names an opened item**, by its id, once for each, as
   `open` names a place. `search`, `tabs` and `characters` take it
   off before the fold, as `opened_from` takes `saved`. An `item` the
   page's orders do not hold is dropped, in the page and in every
   URL it writes. A link that changes the view keeps each `item`,
   since the server finds each again; the query box, the save form
   and "as cards" drop them. Cards are no table and cut nothing.
2. **One maker of the order.** The search crate gives each id's
   index in a rows request's whole order, `after` and `limit` aside,
   or none: one function beside `answer`, which shares with it the
   matching and the order factored out of `unaged`, and refuses as
   it does. A function, never a key on a request or an answer.
3. **The cut** (6.5b's section). A segment holding an opened row
   ends at it; the card stands in the page's flow; a segment of the
   same route starts at the next row. The server renders each
   segment's first block and the block holding each opened row, at
   one basis, so the anchor lands with no script. Several opened
   rows are several cuts. One renderer makes a run of segments with
   its cuts, for the flat table and for each open place of 6.5b.
4. **The opener.** A row's name links the page with its id added,
   or taken out where open, at the row's anchor: an `id` each row
   carries, one maker beside `place_anchor`. An opened row is marked
   and its link says it closes, in words a screen reader gets.
5. **The card** is the answer's `page::card`, made from the
   block's row (its `item`, which the window asks), with the page's
   view links, so its numbers add columns. No second read.
6. **The place is a link** on every card and on the item page: the
   league's "Stash tabs" or "Characters" page with the place and
   each place holding it `open`, at the place's anchor, no query. A
   function of `Places` gives the holders, top first; a place the
   document does not hold is words. *Not taken:* the query on that
   link: #194 asks for the tab and its neighbours, and 6.5b's answer
   already links the query by place.
7. **The query against it.** `item/<id>` takes `against`, a query's
   text, as `acq show --against` does, through `show::against`; its
   JSON is that document. The card under a row links it with the
   answer's canonical text. The way back to the answer is the row
   the card stands under (§3: "the answer is never left").
8. **The save note** (#186). Where the query box carries a part of
   the view the URL named (an order, values, the table), a line by
   the save form says the search is saved without it, from the one
   list of what the box carries hidden.
9. **A refused search keeps its box** (#190). A refused `GET search`
   whose URL names no `next` renders the refusal, then the query
   box holding the URL's `text` as sent and the fields it carries.
   Its JSON and status stay.

## Open, for the designer
**Where a closed card lands** (C172, a choice of shape). Closing
takes the id out, and the next page does not know which row was
open, so it renders no block holding it and the anchor misses.
Each answer adds a part:
(a) the closer's URL names the row as `at`, whose block the server
renders, dropped from every URL the page writes;
(b) the fragment names the row's index, written by the server, and
the script seeks it after sizing, as 6.5b's anchor seek does. The
build waits on the answer, amended here before the plan.

## Find first
- `git log --grep='^Step: 4-full-1-step6-grouped' -1` and
  `git grep -n -E 'class=\\?"seg|data-base' -- crates/acquisition-cli/src/web`:
  6.5b as built, and where segments are written.
- `git grep -n -E 'fn (unaged|answer)\(|let mut order|pub fn against' -- crates/acquisition-search/src`:
  the order to factor out, and the against read.
- `git grep -n -E 'pub fn every|pub fn tabs\(|pub fn characters\(' -- crates/acquisition-search/src/places.rs`.
- `git grep -n -E 'fn (opened_from|window_request|view_link|rows_links|failed|item|search)\(' -- crates/acquisition-cli/src/web/mod.rs`.
- `git grep -n -E 'fn (rows_table|window_rows|card|item|save_forms|query_box|place_anchor)\(' -- crates/acquisition-cli/src/web/page.rs`.
- `git grep -n -E '"item"|[?&]item=' -- crates/acquisition-cli/src/web`:
  whether the field's name is free.
- `git grep -l -E 'item/|GET search|save-note' -- crates/acquisition-cli/tests`:
  the goldens and tests of these pages.
- `git grep -n -E 'fn opening_a_far_parent|pub fn start_browser' -- crates`.

## Done when
- Search crate: for each id of an order, the index function's index,
  asked of `answer` with `after` that index and `limit` one, gives
  that row; an id outside the order gives none; under a sort and in
  the store's order (tests).
- Goldens hold a table with two opened rows (three segments, the
  cards between, each opened row marked), an open place of a tabs
  page under a query with an opened row, the item page under
  `against`, a refused query with its box, and a sorted table's
  save note.
- A test opens a row, then a head's link reorders and the row stays
  open at its new index; an `item` no order holds is dropped from
  the page and its links; "as cards" and the query box carry none.
- A test follows a card's place to its page: the place and its
  holders open, its items listed. A tab in a folder, a map tab's
  page and a character each (tests).
- `item/<id>?against=` JSON is `acq show <id> --against` JSON
  (test).
- A browser test, on a table several blocks long: a far row's
  opener clicked, the row is in view with its card under it, and
  the rows under the card are, id for id, the route's JSON at those
  offsets. Then its closer clicked: the row is in view, no card.
- The module doc says `item`, the cut and the card's ways; a test
  retired has its line (C154, a retirement names its holder). The
  close closes #270 and #186, closes #194 or says what of it 6.5b
  left, and says what is left of #190.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- A card in the cards presentation; a side panel (§7: "eventually").
- A saved search with a view (#186's parked half); the big form's,
  the save form's and the plan's refusals (#190's rest).
- The search form's reshaping, the look (#285), a cache (C48), a
  CLI flag or an MCP tool for the index.

## Rules
`TEMPLATE.md`, and `4-full-1-step6-5-run.md`, "Rules for every
session of the run". The plan names the index function's signature
and the open question's answer as the brief then holds it. Three
commits at least: the search crate's function, the cut and card,
the item page with the box and the note.

## Report
`TEMPLATE.md`.
