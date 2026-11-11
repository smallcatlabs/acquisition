# 4-full-1, step 6 — The view: values over places

A Fable design, 2026-11-11, for step 6 of `4-full-1-design.md` (§7),
at the owner's word after his morning at `acq web`
(`audits/web-2026-11/sitting/README.md`). Ruled by him on 2026-11-11
("Approved as designed."), with his answers to §7's three questions
recorded there; nothing here is built, and the briefs follow.
Read-only at `f19db25b`; nothing was run.

What was read: the sitting's record; `4-full-1-design.md` whole;
`GUI-NOTES.md` §6 and §7; issues #244, #248, #194, #189, #182, #245,
#186, #190, #191, #38; the search crate's `answer.rs` (the request,
the rows view, a row, `Sorted`), `tree.rs` (`ValueRef`), `counts.rs`'s
doc, `tabs.rs`, `show.rs`; the store's `TabRow`; the web module's
routes and its refresh page.

His word on the shape of an answer: "I don't want this to turn into a
long checklist of features to bolt on. I want to think about how those
needs can be met by a reframing or elegant abstraction."

## 1. The one shape

**A view is a query, a list of values, and a grouping; rows are items,
columns are values, groups are places.** The URL names all three
(C16), and every act at the table is a change to one of them.

| What he asked | What it is in this shape |
| --- | --- |
| Sort by a characteristic of the search (stops 3, 4) | The query's values are the first columns; a column's head is a link that sorts by it |
| A value the search did not name (stop 4, 9) | Any number a card shows is a link that adds its value as a column and sorts by it, the site's gesture |
| "I didn't know how to fill out the sort textbox" (stop 3) | The box leaves the form; nothing is typed to sort |
| Thousands of rows, as the old app's tree (the sitting's last section) | The table grouped by place: a head row per stash tab or character, its rows under it |
| "Stash Tabs" and "Characters" in place of "Refresh" (stop 2) | The same table with no query: every place, with its facts and its tick |
| "A stash tab is a thing I browse" | A place opened is the search scoped to it |
| The live count (stop 5, ruled) | One number beside the Search button, asked as the form changes |

## 2. A column is a value

A value already filters, sorts and sums: a number field, a line's
slot, a sum, a computed value (`tree.rs`, `ValueRef`). A row already
carries one, the sort's, as `Sorted { value, status, shows }`.

- **The rows view names values.** `Rows` gains `values`, a list of
  value texts as `sort` takes one. Each row carries them in that order,
  each in the shape `sort` has today. `sort` stays and names any value;
  a sort the list does not hold joins it.
- **The list fills from the query.** With none named, the values are
  the ones the query compares, in the query's order, and the answer
  says the list it used. So a table "with only columns related to" the
  result is the default, not a setting. Before them the table always
  shows the columns he ruled (§7): name, base, price, the price's age,
  location, the location's age.
- **The query's own value is the default order** (#38): a query that
  compares exactly one value, asked with no sort, is ordered by it,
  best first by its operator, and the answer names it. Several values
  give columns and no default order.
- **A card's numbers are links.** For each number a shown item
  carries (a property, a line's slot, a computed total), the document
  says the value text that reads it, made by the search crate, so no
  page composes query text (C46). A card renders it as a link that adds
  the value and sorts by it; a column's head sorts and flips.
- **Cards and the table are one view.** The card marks the view's
  values; `view.fields` chooses the presentation, as today.
- **An agent gains the same.** Three named values on each row cost a
  few bytes, where `item` costs the whole item.
- **What the matches carry** is the `line` count view that exists; the
  table's head links it, and each of its rows is a link that adds its
  value. No new document.

## 3. A group is a place

The grouped table is two documents the tool nearly has.

- **The places.** The listing the refresh page reads today (tabs in
  stash order, folders holding their tabs, a map or unique tab its
  pages; characters) with each place's facts as `TabRow` holds them:
  type, item count, the age of its last fetch or "never fetched". It
  gains `matched`: how many of the query's matches the place holds,
  and the request for them (the query and the place's term, as a count
  bucket's route is made). The search crate holds the listing and the
  counts, so it makes both: one maker.
- **A place's rows.** The rows view scoped to that request, in the
  view's order.

What follows from it:

- **No query: the place pages** (#248). "Stash tabs" and "Characters"
  replace "Refresh" in the navigation. Each head row has its tick;
  "Refresh ticked" makes the plan, as the refresh page does now.
- **A query: the old app's tree.** Places holding no match fold into
  one counted line; an opened place lists its matches.
- **Never fetched is a fact of the place** (#194's view, #191). The
  head row tells "never fetched" from "fetched, and it has none",
  which `TabRow` already holds. So the plan needs no new field: #191's
  schema half is answered here, and the plan's sentence stays as
  `dbced652` left it.
- **The item's place is a link** to that place opened (#194), and an
  item opens as its card under its row, so the answer is never left.
- **A fresh account** (#189): a league with no place listed shows the
  two listing actions where the list would be; refreshing ticked
  places needs no policy; "under the policy" shows when one exists.
- **After an apply** the reader is back at the places, whose facts say
  what the fetch brought (#182's "4 tabs, 0 items").
- **Flat** is the other grouping: one order across every place, the
  place a column, the old app's By-Item.

**A window and a continuation under a group.** Flat, a window is
`after` and `limit`, as built. Grouped, the head rows carry their
counts, so the scroll's height is known before a row loads; a window
is asked within one place and a continuation never crosses a place. A
sort orders the rows inside each place, and the places keep stash
order. A window across a moved basis is refused (C98) and the page
loads its URL again.

## 4. The table's mechanism (the spike's)

- **Fixed row height, and a window script that owns no order.** It
  reads the scroll, asks the server for the rows in view and keeps a
  few pages. With one height the arithmetic needs no library, so the
  spike writes it by hand first, and takes a vendored virtualiser only
  if the hand-written one misses a frame. This departs from §7's
  "vendored, pinned virtualiser file first".
- **The spike, one day, in Brave**, over the measurement copy
  (`search/MEASUREMENTS.md`): scroll frame time and DOM size flat at
  22,721 rows and grouped at 3,347 head rows; and the browser-driven
  check's driver, as §7 says.
- **The count** (#245): one read-only route answering the form's
  match count, one script writing it beside the button.
- The no-state scan covers each new script. The refresh page's list
  filter becomes a field of the places request, so a window honours it.

## 5. The build, least uncertain first

1. **Values on rows** (search crate, CLI, MCP): `values`, the default
   list and order from the query. No page changes. Closes #38.
2. **The count** (#245).
3. **The spike** of §4.
4. **The flat table**: columns from values, heads that sort, card
   numbers as links, "Sort by" gone from the form; #186's one line by
   the save form; a refused query keeps its form (#190), since the
   page is always the form and its answer.
5. **The places and the grouped table**: the places document, the two
   navigation entries, ticks to the plan, the listing actions; #248,
   #194, #189; #191 closed as answered.

After it, each its own step: the plan and jobs pages from their
documents (#182), then the form's reshaping.

## 6. What not to do

- Sort, group or filter in a script (C46).
- Compose a value's text in a page.
- Default columns chosen by item class: a second vocabulary.
- A saved view: his word, until sorting is a click and he asks again.
- Live results as he types: unruled, waiting on the count.
- A plan schema change for #191.

## 7. Drafts for the owner

Each was his. Ruled 2026-11-11: "Approved as designed." for the five
drafts below as recommended, and of the three patterns:

- The columns always there: "Let's start with name, base, price, age
  of price, location, age of location. There will be more here--e.g.
  adding one search term from a trade group might automatically enable
  the display of related terms, but this might want to be it's own
  separate feature added later." The later feature is its own issue.
  Which time the price's age reads is the build step's to find in the
  price document, and his to say where the document holds none.
- "Places stay in stash order."
- "open as a card under it's row. (eventually this may become a side
  panel or popup or some other UI element, but a card under the row is
  acceptable to start with."

As drafted:

- **The view's shape**: §1's sentence.
- **The navigation and routes** (C138): "Stash tabs" and "Characters"
  replace "Refresh"; routes `tabs`, `characters` and `count` join, and
  `refresh` retires. Recommended as listed.
- **#191**: no plan schema change; the place says it. Recommended.
- **The default order from the query** (#38): it changes the row order
  of a CLI or MCP answer that names no sort and compares one value.
  Agent-facing, so the session's, raised because existing answers
  reorder. Recommended.
- **The window script by hand first**. Recommended.
- **Familiar patterns this would cut**, asked before cutting:
  - The old app's fixed columns. Here a table with no query shows the
    name and nothing else until a value is added. Is there a column he
    expects always there (price, item level)?
  - Places stay in stash order under a sort, as the old app's. Or
    should the best place rise?
  - An item opens as a card under its row. Or as its own page, as now?

Agent-facing design, the session's, for his review: `values` and the
row's shape; the default list; the value text on a shown number; the
places document's `matched`; a window within a place.
