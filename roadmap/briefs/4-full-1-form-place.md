# 4-full-1-form-place — A place's line

## Goal
A currency tab reads in its own order with its stacks and rarities, on
its page and at `acq search` (`4-full-1-form-design.md` §8 step 7;
#296). With no sort asked, a place's items stand as they lie, one
order made once for every door; a stack's count is the value `stack`
in the default list; a line carries its rarity in words from one maker
the CLI and the page share. Closes #296.

## Inputs
- #296: the reading of 2026-11-20 and the owner's three rulings.
  `4-full-1-form-design.md` §6's second bullet, §9's second choice,
  §10. Step 6's close, `b1fe747a`.
- C98 (a continuation is a place in one basis's order), C100 (one
  answer shape), C122 (pin once, at the lowest crate), C143 (a text
  speaks the request's words), C146 (a count says what it counts),
  C53 (a JSON name is the contract). #293's cut 3
  (`audits/assessment-6-5/README.md`) left the corpus's order the one
  default order.
- Code: the store's `ITEMS_SQL` and `ALL_ITEMS_SQL` and their
  `ORDER BY` (`acquisition-store/src/corpus.rs`, read only); the search crate's `corpus.rs` (`Corpus::read`,
  `celled`), `answer.rs` (`ordered`, `BoundView::filled`, `unaged`,
  `RowsOut`, `VALUES`, `STORE_ORDER`), `values.rs`, `describe.rs`; the
  CLI's `search_cmd.rs` (`row_text`, `title`, `place_words`,
  `value_cell`, `lacks`) and `web/page.rs` (`window_rows`,
  `item_heads`, `order_way`, `BoxView::of`).
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows) at `ab289d00`: Linux 45240460627 in progress, Windows
  45240460624 success.

## Decisions
The design session's (C172, who designs), each settled:
1. **The default order is made once, where the search crate loads the
   corpus**: in `Corpus::read`, after its loop, one stable sort of
   `items` by (the index of the item's location's first row in the
   read, then its place's cell as `celled` leaves it: row, then
   column; an item with none after those with one, by id). The store hands
   rows grouped by location and by id within one, so a location's
   items stay together, locations keep the store's order, and id breaks
   every tie: a total order, which a continuation's offset needs. The
   place's cell, not the body's, so a worn or socketed item, whose
   `x`/`y` name no cell of its place, stands after. *Declined:* the
   order in the store's SQL by `json_extract` (each body parsed twice
   per search); a position column written at ingest (a schema change
   and a backfill of the owner's store, his class); a page's own
   order, which his ruling refused.
2. **The order's words change with it, held once**: a constant beside
   the sort word, "as they lie in their place, top row first and left
   to right", which every text that says an answer's rows' order
   embeds (the CLI's order line, `order_way`, `describe`'s view entry,
   the refusal of `none` beside `desc`, the docs); `describe` adds
   that an item with no cell stands after, by id. `STORE_ORDER` is
   renamed `PLACE_ORDER`; its word `none` stays. A text about the
   order of locations or of characters keeps its words.
3. **The Stack column is the value `stack` in the default list**:
   where the request names no values, some match carries a stack, and
   the list has room under `VALUES`, `stack` joins at its end (once,
   where the sort is `stack`). It is decided over every match, after
   `matching` in `unaged`, never over the rows shown, so a window's
   columns do not change as it scrolls; `filled` stays a function of
   the query. `values_from_query` keeps its name (C53) and means "the
   default list, none named": true where `stack` alone stands in it,
   so the page's box carries no value back as named. The doc that
   calls the list a pure function of the query and the build says the
   matches too, which the basis fixes.
4. **A row's line has one maker, in the CLI crate**: a function from
   an answer's row to its parts (name, rarity's word where the item
   has one, base, price's words, place, removed, fetched, each value's
   cell), which each door destructures whole, no `..`, so a part a door
   leaves out is named there with why. The page shows the rarity's
   word in the Item cell after the name, as the CLI's head joins it;
   no new head. The CLI's head keeps what it prints: its title holds
   the base, the item under `item` its price, `row_shows` its values,
   and a row carrying the item keeps `shown_title` (its frame with it)
   as its name, a part of the maker.
5. **Each is pinned at the lowest crate that holds it** (C122): the
   order and the default list in the search crate's tests; the maker
   in the CLI crate, its goldens regenerated.

## Find first
- `git grep -n "store's \(stable \)\?order\|STORE_ORDER\|corpus's order" -- crates ':!*/goldens/*'`:
  each text and test of the old order; sort them into an answer's
  rows' (changed) and locations' or characters' (kept).
- `git grep -ln "store's order" -- '*.md' crates/acquisition-cli/tests/goldens`:
  the references, the README and the goldens that say it.
- `git grep -n "corpus\.items\|\.items()" -- crates/acquisition-search/src crates/acquisition-verbs/src crates/acquisition-cli/src crates/acquisition-mcp/src`:
  every reader of the corpus's order; name each that relies on it
  within a location (an example, a first item, a tie).
- `git grep -n 'values_from_query\|\["rows"\]\["values"\]\|rows\.values\b\|rows\["values"\]' -- crates ':!*/goldens/*'`:
  each reader and test of the default list (`view_change` copies it
  as named, as it does the query's values today).
- `git grep -n "FIRST_TURN\|DESCRIPTION\|PARAMETER" crates/acquisition-mcp/tests/surface_size.rs`:
  the MCP's bounds a changed text meets.
- With Decision 1 alone built, `cargo nextest run --workspace --no-fail-fast 2>&1 | grep -E '^ +FAIL'`:
  the tests and goldens that held rows in the old order.

## Done when
- In the search crate, over a seeded tab whose ids and cells disagree,
  with a cell-less item and a socketed one whose body's `x`/`y` would
  sort it first, and a character with a worn item whose body's would:
  unsorted and at `sort: none`, rows by row, then column, the worn,
  socketed and cell-less after by id; a second location's items
  together, locations in the store's order; a continuation continues
  that order. The generated tests (`tests/generated_*.rs`) pass, each
  change named.
- There too: no values named and a match with a stack gives `values`
  `["stack"]`, `values_from_query` true; after a query's values, at the
  end; named values, a full list, or no match with a stack, none; a
  `limit` 1 whose row has no stack still carries it; sort `stack`, once.
- `window_rows` and `row_text` each call the one maker, and neither
  calls `title`, `place_words` or `value_cell` for a row's line itself.
- Goldens, each diff read: a currency tab's place page and `acq search`
  over it, rows in cell order with a `stack` column of counts and the
  rarity's word on a gear row; a page's box after an answer whose list
  is `stack` alone carries no values; a row carrying the item at
  `acq search` titled as before, its frame with it.
- The first Find-first command finds no text saying an answer's rows
  stand in the store's order. `describe`, the module docs of `values.rs`
  and `answer.rs` and `window_rows`'s doc say what is built; the
  references and the README regenerated.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- The store's read, its SQL and its schema; the order of locations
  themselves; a page's own order or a column only the page shows.
- A Rarity head, and the look (#285); the sort word `none`.
- Abyssal sockets and the index (step 8).

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
