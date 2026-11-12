# 4-full-1, step 6.4c — Heads that sort, and a value from a card

## Goal
At `acq web` nothing is typed to sort. A value column's head is a
link that orders the rows by it and flips; a number on a card is a
link that adds its value as a column and orders by it; "Sort by" and
"highest first" leave both forms. The page's URL can name the list of
values, which it cannot today.

## Inputs
- `4-full-1-step6-design.md` §1, §2, §6 and §7's rulings.
- `4-full-1-step6-table.md` and its close, `git show -s de5d8247`:
  the table this part changes.
- C16 (the URL names the state), C46 (semantics in Rust: no page
  composes a value's text or orders a row), C53 (JSON is the
  contract, so a new key is additive), C92 (a line's comparison binds
  one occurrence; a total is asked by name).
- `answer.rs`: `Rows` and `RowsOut` (`values`, `sort`, `desc`, the
  two `_from_query` marks, `VALUES`). `show.rs`: `ShownItem::of`,
  `ShownLine`. `print.rs`: `print_value`.
- `web/form.rs`: the fold and the names it takes. `web/mod.rs`:
  `asked_by_the_form`, `rows_links`. `page.rs`: `rows_table`, `card`,
  `query_box`, `search_form`. `search_cmd.rs`: `shown_properties`,
  `row_shows`, `order_words`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **The field.** `view.value`, one value each and given again for
   the next, as the CLI's `--value` (a template may hold a comma, so
   no list). The fold reads it into `Rows.values`,
   `asked_by_the_form` writes it, and a refusal is the search's own.
2. **One maker of a change to the view.** A function of the page's
   fields as sent (as `rows_links` makes the presentation's link, a
   continuation as it does), the answer's `RowsOut` and one value's
   text gives a link: the same page ordered by that value, largest
   first, or the other way where the rows are already ordered by it.
   Where the value is not among the answer's values, the link names
   the answer's list with it added; otherwise it leaves the list as
   the URL has it. Heads and card numbers both call it. A value
   outside a full list gets no link.
3. **Heads.** Each value column's head is that link, in the value's
   text as today. The one the rows are ordered by says which way, in
   a mark and in words a screen reader gets. The five fixed heads
   stay text: none is a value the sort takes. A price's amount is
   one, and would order ten chaos above one divine.
4. **The boxes leave** the search form and the query box. `view.sort`
   and `view.desc` stay fields of the URL. The query box carries the
   sort, the direction and the values the URL named, hidden, as it
   carries the realm: an edited query keeps the columns its reader
   added. What the query gave is never carried, as today.
5. **A shown number says its value** (search crate; agent-facing,
   recorded in its commit). What `show` and a row's `item` carry
   gains, by additive keys from the one maker, the value's text for
   each slot of a line and for each number field the item shows, each
   canonical by `print_value`. A line's slot reads
   `sum(line("<template>" source=<source>).<slot>)`: what the trade
   form's filter on that stat compares, so a click on a stat the form
   filtered lands on its column and adds no twin. *Other answer:*
   `line( … ).<slot>`, the one occurrence. Not taken: the form is how
   the owner searches. Where the keys sit, and how a card places a
   slot's link in a line's words without parsing a text it was given,
   is the plan's to propose.
6. **Cards.** A card writes each such number as Decision 2's link,
   and stays cards. A number whose value the view holds is marked,
   the sort's as its head is.
7. **No script.** Each is a plain link.

## Find first
- `git grep -n -E 'view\.(sort|desc)|Sort by|highest first' -- crates README.md`:
  every form, test, golden and doc that holds the boxes.
- `git grep -n "fn rows_links\|fn asked_by_the_form\|fn query_box\|fn rows_table\|fn card" -- crates/acquisition-cli/src/web`.
- `git grep -n "slots" -- crates/acquisition-search/src crates/acquisition-cli/src crates/acquisition-mcp/src`:
  who reads a shown line, and where `--describe` says an answer's keys.
- `git grep -n -E 'sum\(line\(' -- crates/acquisition-search/src/trade`:
  how the translation writes a stat's value; one writer for both.
- `git grep -n -i -E 'size|bytes' -- crates/acquisition-mcp/tests crates/acquisition-cli/tests/*.rs | head -40`:
  the size bounds an `item` row is held to.

## Done when
- The fold reads `view.value`, and a request naming values is written
  back as fields that fold to the same request (tests).
- A golden holds a table whose value heads are links, the ordered one
  marked. A web test follows a head's link, then the same head's
  again: the first answer is ordered by the value, largest first, the
  second the other way, the list unchanged.
- A web test follows a card's number for a value the view lacks: the
  answer's list is the old list and that value, ordered by it, as
  cards. A full list writes no such link (test).
- Search crate, over the fixtures its tests hold: every value text a
  shown item carries binds as a sort and, asked of that item, is the
  number it stands beside, or their sum where the item holds several
  occurrences it selects. A trade filter on a stat and that stat's
  line on a card give one text (test).
- No page holds a `view.sort` input. An edited query run from the
  query box keeps a named sort and named values (test). Each golden
  and test that held the boxes is changed or retired with its line
  (C154, a retirement names its holder).
- The references and `--describe`'s account of the keys are
  regenerated, each diff read. The module docs say the field and
  the links.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- Dropping a column, a link back to the store's order, a best
  direction for each value: none until the built page is looked at.
- An order for the fixed columns, the price's among them.
- An item opened under its row, the reader's place (#270), #186,
  #190, the look: the next part. The grouped table (6.5, #269).
- A new value in the language. A script.

## Rules
`TEMPLATE.md`, and:
- **The plan first.** Before any edit, run "Find first" and write a
  plan of one screen to `plan.md` in your notes folder: what each
  command found, a line each; the functions you will change; the
  shape you propose for Decision 5's keys and the card's link; the
  tests; anything here the tree disproves. End the turn and wait for
  the answer.
- Two commits at least, in order: the view's URL (Decisions 1 to 4),
  then the shown number (5 and 6). The report says the bytes the new
  keys add to one `item` row and to one `show`.
- Every `acq` and `acq web` runs in a mock world. The round takes a
  cold review (C175, a behaviour change is reviewed cold).
- A push the harness refuses is left and reported, by its commits.

## Report
`TEMPLATE.md`.
