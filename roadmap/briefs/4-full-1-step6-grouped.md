# 4-full-1, step 6.5b — The grouped table

## Goal
At `acq web` a place page takes a query. "Stash tabs" and
"Characters" then list the league's places that hold a match, each
with its count, in stash order, the rest folded into one counted
line, and a place opened lists its items under its head as the
table's rows: the old app's tree. The list's filter is the request's,
and the never-fetched tabs are one of its views.

## Inputs
- `4-full-1-step6-design.md` §1, §3, §4, §6; `4-full-1-step6-places.md`
  and its close (`git log --grep='^Step: 4-full-1-step6-places'`);
  `4-full-1-step6-table.md`.
- `audits/web-2026-11/spike/README.md`, case G1: an open tab.
- #244, #269, #194 (its second half), #294.
- C16 (the URL names the state), C46 (semantics in Rust), C48 (no
  cached search), C98 (one basis), C187 (the browser check).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **One build**, three commits at least: the document, the filter,
   the query on the pages. Both are fields of one request, with one
   maker and one golden: two builds would gate and review it twice.
2. **The document** (`places.rs`) takes a filter and a query beside
   its league, and still lists every place. A place is *listed* or
   *folded*: listed where it passes the filter and, under a query,
   holds a match, or where it holds a listed place. Each level
   counts what it folds, and of that the never fetched. The filter
   is `name` (words the name holds, case aside), `type` (a tab's
   type, whole) and `fetched` (`never`). The query is a text,
   refused as the search refuses it. Under one the document carries
   the canonical text, the basis, the query's own request and its
   `total`, the matches in this league's places, and each place's
   `matched`. A place's `request` is the query and the place's
   terms; its `matched` is that request's `total.matched`, a
   folder's its places' sum. With neither field it is 6.5a's.
3. **The page's fields**: `text` (absent or empty: no query), `name`,
   `type`, `fetched`, `view.value`, `view.sort` and `view.desc`,
   beside `open` and the ticks in the one GET form, in
   `place_fields`' order, all kept by a toggle's 303. The selects'
   form carries all but the ticks and opens. The filter's controls
   replace the boxes and are sent; `filter.js` retires. A level's
   fold is one line after its rows, its never-fetched count a link
   to the page under `fetched=never` alone. A tick the page does not
   draw, folded or under a closed parent, travels hidden and is
   counted in words.
4. **An opened place is `open`, the field a parent has.** An open
   place shows what it holds: places where it holds places, else its
   items, under a query its matches. One rule with or without a
   query; the opener, its 303 and its anchor stand.
5. **A window inside a group.** Each open place's items are one
   segment of the page's one `.window`: its route `rows?next=` with
   the token of the place's window request, its count that request's
   matches, its base 0, its first block the server's
   (`Web::window`). A head and a fold line are flow between
   segments, in none. A place's request takes the view the URL
   names, so a sort orders the rows inside each place and the places
   keep stash order. The item columns' heads stand once, above the
   places, where a place lists rows; a value's head is `view_link`'s,
   which takes this route: no second maker. A page is made over one
   corpus at one basis, with no cache. `rows` and `window.js` stand,
   save one case: if sizing the segments moves the URL's anchor out
   of view, the script seeks it once after sizing, said in the
   commit.
6. **A column is the text an answer's `values` prints** (#294). A
   page carries and compares only value texts the search crate
   printed, and joins no two. The segments' columns agree because a
   place's terms compare no number (`values.rs`). #294 is not built
   here: two spellings of one question becoming one value changes
   what a query answers, which is the owner's. Whatever he rules
   moves texts in the search crate's one printer, and no page.
7. **The ways between.** A rows answer links "by stash tab" and "by
   character": the place page of the league the query names (one
   method beside `tabs_pointer`), else the page's default, with the
   canonical text and the view; none where `asked_by_the_form` gives
   none. The page says the matches outside its places and links the
   query's own request as a table. The scope's tabs link gains
   `fetched=never`, added by the page: no field on a `Way`.

## 6.5c's question: a card under a row, and one row height
**An opened item cuts its segment in two.** The script's arithmetic
is a segment's: rows of one height from `data-base`, which it reads
today. A card is no row and sits in no segment: the server ends the
segment after the opened row, writes the card in the page's flow and
starts another segment at the next row, with its base and the same
route, as G1 drew heads around an open tab. So a segment keeps one
height and a card has any. The URL names an opened item by its id
alone (C16); where it stands in the order is the search crate's to
say, beside the window's maker, and an id the order lacks opens
nothing. The opener is a link to the page with the id added or taken
out, at the row's anchor (#270); the server renders the block that
holds the row, so no script measures a card.

## Find first
- `git grep -n -E 'filter\.js|col-filter|fn filter_row|FILTER' -- crates`:
  the list filter.
- `git grep -n -E 'Places::(tabs|characters)\(|fn place_terms|fn tabs_pointer|fn league_terms|Way::Tabs' -- crates`.
- `git grep -n -E 'fn (window|view_link|rows_links|place_fields|place_rows|rows_table)\(' -- crates/acquisition-cli/src/web`.
- `git grep -l -E 'GET (tabs|characters)' -- crates/acquisition-cli/tests`.
- `git grep -n -E 'fn opening_a_far_parent|pub const SCRIPTS' -- crates`.

## Done when
- Search crate, over 6.5a's fixture and a query: each place's
  `matched` is its request's `total.matched`, a folder's its places'
  sum, the top level's sum the document's. Each place's request
  answers the query's own `values`, `sort` and `desc`. Each filter
  field and the query fold what Decision 2 says, each level's counts
  with them. A query the search refuses is refused (tests).
- Goldens hold a tabs page under a query (a listed head, a fold
  line, an open place's segment and the columns' heads, an open
  parent), one under `fetched=never`, a characters page under a query.
- A test follows the answer's two links, the fold's, the scope's, a
  value head's and a toggle: each lands on the page Decision 3
  names, every field kept. A tick on a folded place reaches the
  plan. A segment's route answers, as JSON, its place's request's
  rows at the page's basis.
- A browser test, a query and two places open, the first several
  blocks long: the rows in view under each head are, id for id, that
  place's route's JSON at those offsets; after the second's opener
  is clicked, its head is in view once the segments are sized, and a
  box ticked before is ticked.
- No page names `filter.js`; each golden and test that held it is
  changed, or retired with its line (C154, a retirement names its
  holder). The module docs say the fields.
- The close closes #269, says what is left of #244, #194 and #294,
  and names the Gate (Linux) and Gate (Windows) runs.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- 6.5c's card and ways; 6.5d; the form's reshaping; the look (#285).
- #294's fix; a CLI flag or an MCP tool for the document; a field on
  a `Way`; a request key; a cache; a windowed head.

## Rules
`TEMPLATE.md`, and `4-full-1-step6-5-run.md`, "Rules for every
session of the run": the plan first, naming the document's keys.
The round is reviewed cold (C175, what is reviewed).

## Report
`TEMPLATE.md`.
