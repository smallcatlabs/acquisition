# 4-full-1, step 6.5c — An item under its row, and the row's ways

## Goal
At `acq web` a table's row opens its item as a card under it, the
reader kept in place. The card's place links that place opened, and
the query's outcome on it, term by term, is one link away. A refused
query keeps its box, and the save form says what a save leaves
behind.

## Inputs
- `4-full-1-step6-design.md` §2, §3, §4, §7.
- `4-full-1-step6-grouped.md` (6.5b): Decision 5, and "6.5c: a card
  under a row, and one row height", the designer's, taken as written.
- `4-full-1-step6-places.md` (6.5a): the GET form, `toggle`, anchors.
- #270, #194, #186, #190.
- C16 (the URL names the state), C46 (semantics in Rust), C98 (one
  basis), C167 (a saved search is text and scope), C187 (browser
  check).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **`item` names an opened item**, by its id, once for each, as
   `open` names a place. `search`, `tabs` and `characters` take it
   off before the fold, as `opened_from` takes `saved`. One no order
   of the page holds is dropped, in the page and every URL it
   writes. A change to the view keeps each `item`; the query box,
   the save form and "as cards" drop them. Cards cut nothing.
2. **One maker of the order.** The search crate gives each id's
   index in a rows request's whole order, `after` and `limit` aside,
   or none: a function beside `answer`, never a key, sharing the
   matching and the order factored out of `unaged`.
3. **The cut** (6.5b's section). A segment holding an opened row
   ends at it; the card stands in the page's flow; a segment of the
   same route starts at the next row. The server renders each
   segment's first block and the block holding each opened row, at
   one basis. `rows` stands, and no row is marked open: its card,
   in the flow and never fetched again, is the mark and holds a
   "Close" button. One renderer makes the segments and cuts, for the
   flat table and each open place of 6.5b.
4. **The opener is 6.5a's toggle**, not a link: a link drops a tick
   changed since the load, and a fetched row has no page's fields. A
   row's name is a button sending `toggle_item` with its id, as the
   card's "Close" does, inside the page's one GET form: the place
   pages', and on `search` one around the table holding the URL's
   fields hidden. It is answered 303 to the page with the id added,
   or taken out where open, at the row's anchor (an `id` on each
   row, made beside `place_anchor`).
5. **The card** is `page::card` of the block's row (its `item`,
   which the window asks), with the page's view links: its numbers
   add columns. No second read.
6. **The place is a link** on every card and on the item page: the
   place page with the place and each holding it `open`, at its
   anchor, no query. A function of `Places` gives the holders, top
   first; a place it lacks is words. *Not taken:* the query on it:
   #194 asks for the tab and its neighbours, and 6.5b's answer links
   the query by place.
7. **The query against it.** `item/<id>` takes `against`, a query's
   text, as `acq show --against` does, through `show::against`; its
   JSON is that document. The card under a row links it with the
   answer's canonical text. The way back is the row the card stands
   under (§3: "the answer is never left").
8. **The save note** (#186). Where the query box carries a part of
   the view the URL named (an order, values, the table by its
   select), a line by the save form says the search is saved without
   it, read from what the box carries, one list.
9. **A refused query keeps its box** (#190). A refused `GET search`
   whose URL names `text` renders the refusal, then the query box
   holding that text and the view's fields as sent. JSON and status
   stay.

## Where a toggle lands (the designer's, 2026-11-19)
**The 303 names the row as `at`,** its id, beside the fragment. The
server renders the block holding the `at` row as it does an opened
row's, by Decision 2's index, so a closed card's row is there for
its anchor. `at` is one load's landing, never state: no link or form
carries it, and one no order holds is ignored. 6.5b's anchor seek
must come before the script's first fill, which drops a far block;
if built after, it moves.

## Find first
- `git log --grep='^Step: 4-full-1-step6-grouped' -1`;
  `git grep -n -E 'class=\\?"seg|data-base' -- crates/acquisition-cli/src/web`:
  6.5b as built, where segments are written.
- `git grep -n -E 'fn (unaged|answer)\(|let mut order|pub fn against' -- crates/acquisition-search/src`.
- `git grep -n -E 'pub fn (every|tabs|characters)\(' -- crates/acquisition-search/src/places.rs`.
- `git grep -n -E 'fn (opened_from|window_request|view_link|rows_links|failed|item|search|rows)\(|"toggle"' -- crates/acquisition-cli/src/web/mod.rs`.
- `git grep -n -E 'fn (rows_table|window_rows|card|item|save_forms|query_box|place_anchor)\(' -- crates/acquisition-cli/src/web/page.rs`.
- `git grep -n -E '"(item|toggle_item)"|[?&]item=' -- crates/acquisition-cli/src/web`:
  are the names free.
- `git grep -l -E 'GET (item|search|tabs|characters|rows)|save-note' -- crates/acquisition-cli/tests`;
  `git grep -n -E 'fn (opening_a_far_parent|the_window_shows)|pub fn start_browser' -- crates`:
  these pages' tests and goldens, the browser tests.

## Done when
- Search crate: each id's index, asked of `answer` with `after` that
  index and `limit` one, gives that row; an id outside the order
  gives none; under a sort and in the store's order (tests).
- Goldens: a table with two opened rows (three segments, the cards
  between, each with its "Close"), a tabs page's open place under a
  query with an opened row, the item page under `against`, a refused
  table query naming an order and values, its box carrying all of
  them, the save note on a sorted table and on a table alone.
- A test toggles a row open, then a head's link reorders and it
  stays open at its new index; toggled again, it closes: the 303
  names `at`, the page renders that row's block, and no URL the page
  writes carries `at`. An `item` no order holds is dropped; "as
  cards" and the query box carry none.
- A test follows a card's place to its page: the place and its
  holders open, its items listed; a tab in a folder, a map tab's
  page and a character each. A test follows the card's query link:
  it carries the answer's canonical text, and its JSON is
  `acq show <id> --against` JSON.
- A browser test on a tabs page several blocks long: a box ticked,
  then a far row the script fetched opened; the row is in view, its
  card under it, the tick kept, and the rows under the card are, id
  for id, the route's JSON at those offsets. Closed: the row in
  view, no card, the tick kept.
- The module doc says `item`, the cut and the card's ways; a
  retired test has its line (C154, a retirement names its holder). The
  close closes #270 and #186, closes #194 or says what 6.5b left,
  and says what is left of #190.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- A card in the cards presentation; a side panel (§7).
- A saved search with a view (#186's parked half); the big form's,
  the save form's and the plan's refusals (#190's rest).
- The form's reshaping, the look (#285), a cache (C48), a CLI flag
  or an MCP tool.

## Rules
`TEMPLATE.md`, and `4-full-1-step6-5-run.md`, "Rules for every
session of the run". The plan names the index function's signature.
Three commits at least: the index, the cut and card, the item page
with the box and the note.

## Report
`TEMPLATE.md`.

## Taken from the review
`roadmap/reviews/4-full-1-step6-row-brief-5e4d34e3.md`: 2 and 3 by
one rule, the opener as 6.5a's toggle; 4, `text`; 5, 6 and 7 in
Done when and Find first; 8, C167. `…-4bb804d9.md`: 2, no row is
marked (Decision 3); 3 and 4 in Done when and Find first. Each
round's 1 is answered above. Not reviewed again (C176).
