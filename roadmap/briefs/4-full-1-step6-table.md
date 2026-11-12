# 4-full-1, step 6.4b — The windowed table

## Goal
An answer shown "as a table" in `acq web` is the dense table the
spike measured: every matching row in one scrolling page, at one row
height, its rows asked a block at a time as they come into view. It
has the columns the owner ruled and one column for each value the
answer carries. This is the table that the heads that sort, the
card's numbers and the opened card are then built on, each its own
part after this one has been looked at.

## Inputs
- `4-full-1-step6-design.md` §2 (a column is a value), §4, and §7's
  rulings: the route `rows`; the fixed columns name, base, price,
  location and the location's age (the price's age left out, #271).
- `audits/web-2026-11/spike/README.md`, the record, and
  `rig-3.patch` beside it, read whole: the window script
  (`assets/window.js`), the route that answers a window, the
  segment's markup and its stylesheet block. It is the code the
  owner's Brave passed.
- C16 (the URL names the state; a page holds nothing a reload must
  keep), C46 (semantics in Rust), C98 (one basis per answer), C138
  (the listener's token and guard), C187 (the browser-driven check).
- `answer.rs`: `Rows` (`after`, `limit`, `fields`), `RowsOut`
  (`values`, `returned`, `left_out`), `Row` (`place` with its fetch
  time, `values`, and `shown` under `RowField::Item`, the only place
  a price is), `Request::token` and `from_token`, and the bound on
  rows that carry their item.
- `web/mod.rs`: `search`, `rows_links`, `asked_by_the_form`, `failed`,
  the asset handlers. `page.rs`: `answer`, `rows_table`, `Head`.
  `tests/browser.rs` and the harness's `start_browser`, `SCRIPTS`,
  `no_state`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`; on hosted runners.

## Decisions
1. **Start from the rig.** Take from `rig-3.patch` what the flat
   table is: the window script as it is, the window's route, the
   segment and row markup, the stylesheet block. Leave the rig's own
   page, its heads and result routes, the measure, the runner and
   the probes. A change to the script is said in the commit with its
   reason: it is the thing that was measured.
2. **The table is the answer page's table.** `rows_table` becomes it;
   there is no second page. Its segment is the answer's rows whole,
   its count the rows the request would list in full. The server
   renders the first block in place, so the page shows rows before
   any script runs.
3. **The route.** `rows?next=<token>&offset=<n>&count=<n>` answers
   that window of the token's request at its basis, as an HTML
   fragment of rows; where JSON is asked, the answer document of the
   same window, as `search` gives it. It asks its rows with their
   items whatever the request names, since a price comes only with
   the item. A count of zero or over the item rows' bound, or an
   offset past the rows, is refused as the page's refusals are;
   nothing is clipped in silence. A moved basis is refused with 409
   and the script loads the page again.
4. **The columns**, in this order: the name (in its rarity's colour,
   a link to the item's page), the base, the price, the location, the
   location's age; then each of `RowsOut.values`, headed by the
   value's own text, each row's value under it as the row's `values`
   carry it. The heads are text in this part. A cell is one line,
   cut where it overflows.
5. **One row height**, written by the server on the table and read
   by the script and the stylesheet from there.
6. **What the table does not use.** `view.limit` chooses how many
   cards a page shows and is not read for the table; the table's
   line says how many rows it holds, and carries no "more" or "first
   rows" link. The form's controls stay as they are in this part.
7. **The script is served as the others are**: a `Head` flag, its
   route, `SCRIPTS`, the no-state scan. It names no storage, cookie
   or history API and keeps only the blocks it shows and the asks in
   flight.
8. **The check joins.** A test in `browser.rs` loads a table of more
   rows than several blocks, scrolls to its middle and to its end,
   and each time: the rows the viewport covers are, id for id and in
   order, the rows the route's JSON gives for those offsets; each is
   drawn at its index times the row height; and the rows in the page
   stay under a bound that does not grow with the table. The seeded
   world gains the rows it needs; no test waits against a duration.

## Find first
- `git apply --numstat audits/web-2026-11/spike/rig-3.patch` and
  `git apply --check` of it on `main`: what it touches; it need not
  apply whole, since `main` moved.
- `git grep -n "fn rows_table\|fn rows_links\|fn answer\|struct Head" -- crates/acquisition-cli/src/web`:
  what renders the table today and what names a script.
- `git grep -n -E 'class=.{1,2}results|as a table|as cards|first rows' -- crates/acquisition-cli`:
  every test and golden that holds today's table.
- `git grep -n "ITEM_ROWS\|fn from_token\|fn place" -- crates/acquisition-search/src/answer.rs crates/acquisition-store/src/bound.rs`:
  the bound, the token, a continuation's place.
- `git grep -n "fn seed\|fn offline_web" -- crates/acquisition-cli/tests/web.rs`:
  the seeded world the tests share.
- `git grep -n "route(&at(" -- crates/acquisition-cli/src/web/mod.rs`:
  the routes, and the module doc's list of them.

## Done when
- A table answer's page holds one segment with its row height, its
  count and its route, and its first block's rows, in the columns of
  Decision 4 (goldens, each diff read). An answer whose query
  compares a value shows that value's column.
- `rows` answers a window as a fragment and as JSON; its JSON equals
  `search?next=`'s for the same place; its refusals are Decision 3's
  (tests). The module doc lists the route.
- The no-state scan reads the window script; `SCRIPTS` lists it.
- The browser test of Decision 8 passes here, and the close names
  the hosted runs of Gate (Linux) and Gate (Windows) on the build's
  push with that test run.
- Every golden and test that held the old table is changed or
  retired with its line (C154, a retirement names what holds the
  property).
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- Heads that sort, a value added from a card's number, the sort box's
  removal, a URL field for the values: the next part.
- An item opened as a card under its row, the reader's place (#270),
  #186, #190, the look: the part after.
- The grouped table and the places (step 6.5; #269).
- The price's age (#272). An icon column. A framework or a crate.
- A change to the search or store crates' API or bounds.

## Rules
`TEMPLATE.md`. Every `acq` and `acq web` runs in a mock world. The
round changes behaviour and the shared harness, so it takes a cold
review (C175).

## Report
`TEMPLATE.md`.
