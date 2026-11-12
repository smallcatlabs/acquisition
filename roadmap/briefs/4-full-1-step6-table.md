# 4-full-1, step 6.4b — The windowed table

## Goal
An answer shown "as a table" in `acq web` is the dense table the
spike measured: every matching row in one scrolling page, at one row
height, its rows asked a block at a time as they come into view. It
has the columns the owner ruled and one column for each value the
answer carries.

## Inputs
- `4-full-1-step6-design.md` §2 (a column is a value), §4, and §7's
  rulings: the route `rows`; the fixed columns name, base, price,
  location and the location's age (the price's age left out, #271).
- `audits/web-2026-11/spike/README.md`, the record, and
  `rig-3.patch` beside it, read whole: the code the owner's Brave
  passed.
- C16 (the URL names the state; a page holds nothing a reload must
  keep), C46 (semantics in Rust), C98 (one basis per answer), C138
  (the listener's token and guard), C187 (the browser-driven check).
- `answer.rs`: `Rows`, `RowsOut`, `Row` (its `place`, its `values`,
  and `shown` under `RowField::Item`, the only place a price is),
  `Request::token` and `from_token`.
- `web/mod.rs`: `search`, `rows_links`, `asked_by_the_form`.
  `page.rs`: `answer`, `rows_table`, `Head`. `tests/browser.rs` and
  the harness's `start_browser`, `SCRIPTS`, `no_state`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Start from the rig, and know what it is not.** Take from
   `rig-3.patch` the window script, the window's route, the segment
   and row markup, the stylesheet block. Leave its own page, its
   heads and result routes, the measure, the runner and the probes.
   The rig rendered the least a row needs; Decision 5 says what the
   table's cells must. A change to the script is said in the commit
   with its reason: it is the thing that was measured.
2. **One maker of a window's rows.** A function of the page's
   request, the answer's basis, an offset and a count gives that
   window's rows with their items: the request with `after` and
   `limit` set and `item` among its fields. The `rows` route calls
   it, and so does the page for its first block.
3. **The page's request stays what its URL names.** The table is
   still chosen by the request's fields as today (`icon` alone), so
   `rows_links`, `query_box` and `asked_by_the_form` read as they
   do. `rows_table` takes the answer, for its totals, its values'
   list and its basis, and the first block from Decision 2's maker;
   it does not render the answer's own rows. Its segment's count is
   the rows the request would list in full.
4. **The route.** `rows?next=<token>&offset=<n>&count=<n>` answers
   Decision 2's window as an HTML fragment; where JSON is asked, the
   answer document of that window's request. A count of zero or over
   the item rows' bound, or an offset past the rows, is refused as
   the page's refusals are; nothing is clipped in silence. A moved
   basis is refused with 409 and the script loads the page again.
5. **The columns**, in this order: the name (in its rarity's colour,
   a link to the item's page), the base, the price, the location,
   the location's age; then each of `RowsOut.values`, headed by the
   value's own text. A price is `search_cmd::price_words`' words, so
   one that is open or absent says so. A value's cell is written by
   one function of one `Valued`, taken out of `row_shows` so both
   use it: the number and its status's words as `row_shows` writes
   them (C94, an incomplete subtotal is marked), never a bare number
   where a status stands, and without the evidence beside it, which
   stays the card's. A value the row lacks is an empty cell unless a
   status says why. Heads are text in this part. A cell is one line,
   cut where it overflows.
6. **One row height**, written by the server on the table and read
   by the script and the stylesheet from there.
7. **The first block is the server's, and the script adopts it.**
   The page shows rows before any script runs. On starting, the
   script takes a block the server rendered as one it holds, done,
   and drops it like any other; it never asks that block again while
   it holds it. This is the one change to the measured script.
8. **`view.limit` is the cards'** and is not read for the table,
   whose line says how many rows it holds, with no "more" or "first
   rows" link. The form's controls stay as they are in this part.
9. **The script is served as the others are** (`Head`, `SCRIPTS`,
   the no-state scan).
10. **The check joins.** A test in `browser.rs` loads a table several
    times longer than the bound below. At the start, the middle, the
    end and the start again: the rows the viewport covers are, id for
    id and in order, the rows the route's JSON gives for those
    offsets, each id once; each is drawn at its index times the row
    height; and the rows in the page number at most the blocks three
    viewports span and one more, by the viewport's height, the row
    height and the block's size, which the test reads from the page.
    It also holds the script's two other acts: the first block is
    not asked for while the page holds the server's; and after the
    world's basis moves, the next window asked loads the page again,
    at the new basis. The seeded world gains the rows; no test waits
    against a duration.

## Find first
- `git apply --numstat audits/web-2026-11/spike/rig-3.patch`: what
  it touches; it need not apply whole, since `main` moved.
- `git grep -n "fn rows_table\|fn rows_links\|fn answer\|struct Head" -- crates/acquisition-cli/src/web`:
  what renders the table today and what names a script.
- `git grep -n -E 'class=.{1,2}results|as a table|as cards|first rows|view\.fields' -- crates/acquisition-cli`:
  what holds today's table; trace each golden to its test.
- `git grep -n "ITEM_ROWS\|fn from_token\|fn place" -- crates/acquisition-search/src/answer.rs crates/acquisition-store/src/bound.rs`:
  the bound, the token, a continuation's place.
- `git grep -n "fn seed\|fn offline_web\|fn the_moved_world" -- crates/acquisition-cli/tests/web.rs`:
  the seeded world, and how a test moves its basis.
- `git grep -n "route(&at(" -- crates/acquisition-cli/src/web/mod.rs`:
  the routes; `sed -n 1,190p` of that file is its module doc.
- `git grep -n "fn row_shows\|fn price_words\|fn place_text" -- crates/acquisition-cli/src/search_cmd.rs`:
  the words a value, a price and a place already have.

## Done when
- A table answer's page holds one segment with its row height, its
  count and its route, and its first block's rows, in the columns of
  Decision 5 (goldens, each diff read). An answer whose query
  compares a value shows that value's column.
- `rows` answers a window as a fragment and as JSON. Its JSON equals
  `search?next=` asked with the token of the same request made by
  hand in the test: `after` the offset, `limit` the count, `item`
  among its fields. Its refusals are Decision 4's (tests). The
  module doc says the route.
- A golden holds a table of several values, one with a status, and
  a price that is open or absent: each in its own words, under its
  own head.
- The browser test of Decision 10 passes here, and the close names
  the hosted Gate (Linux) and Gate (Windows) runs with it run.
- Each golden and test that held the old table is changed, or
  retired with its line (C154, a retirement names its holder).
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- Heads that sort, a value added from a card's number, the sort
  box's removal, a URL field for the values: the next part.
- An item opened under its row, the reader's place (#270), #186,
  #190, the look: the part after. The grouped table (6.5, #269).
- The price's age (#272). An icon column. A framework or a crate.
- A change to the search or store crates' API or bounds.

## Rules
`TEMPLATE.md`. Every `acq` and `acq web` runs in a mock world. The
round takes a cold review (C175).

## Report
`TEMPLATE.md`.
