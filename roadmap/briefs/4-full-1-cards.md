# 4-full-1-cards — search results as cards, from rows that carry the item

Step 5 of 4-full part 1 (`roadmap/briefs/4-full-1-design.md` §4 "Rows
that carry every field" and "The card and the item page", §7 step 5;
GitHub issue #98), for an Opus worker that has not seen the design.
Written against the tree at `4b28dad1`, after step 4
(`roadmap/briefs/4-full-1-answer.md`) closed on 2026-11-08, its row the
last of `roadmap/LEDGER.md`. Cut by a session that read the design on
`main` and the tree, nothing else of the group. The spike the design
puts first was run by this session, before the brief, and its numbers
are the Input; the build measures the built rows once more (Do 2).
Where the design and the tree disagree the brief takes the tree's fact
and says so (Input). Where the design is silent on a card's words or
layout, this brief decides, and the foot lists each decision for the
owner.

## Goal

When this step is done a rows view may ask each row to carry the item
whole — `fields: ["item"]` beside `icon` — and what a row then carries is
made by the one function `show` uses, so a row's item, its lines with
their slots, its class, its category and its price equal `show`'s for
the same id at the same basis, pinned so in the search crate. Under
`item` a rows answer holds at most 200 rows, the rest its continuation,
an item never cut. The CLI's `--fields` is built: `acq search …
--fields icon,item` answers so in JSON and prints each row's item as
`show` prints one. The answer page of `acq web` asks for the item by
default and renders a card per row in the trade site's order — name,
base, properties and requirements, lines by source with their flags,
sockets, price, place with its age — and `view.fields=icon` in its URL
renders the compact table it has today; the item page renders `show`'s
document through the same card. Today the page has only the rows'
compact fields (id, name, base, rarity, place, the lines the query
touched), so a card would have to be composed from a second maker, which
C46 (shared semantics in Rust) forbids. Closes #98 by its "Closes when":
a results page renders cards from rows that carry every field, made by
the one maker `show` uses, with no second evaluator in the page.

## Read

- `AGENTS.md`, `roadmap/README.md` "A step", and
  `roadmap/briefs/TEMPLATE.md`.
- `roadmap/briefs/4-full-1-design.md` §2 ("What a page may not hold": a
  presentation choice is a URL field; "Identity": the fold gains
  `view.fields`), §4 whole (its first two paragraphs are this step; its
  third was step 4's), §6 ("Cards composed from `Held.item` in the page
  … two makers"), §7 step 5. `roadmap/GUI-NOTES.md` §7 (one maker with
  `show`; why `show` per row is no answer: a full read an item).
- `decisions/search.md`: C100 (one answer shape; a row compact by
  default, the caller may name fields; the body only through `show`),
  C103 (one deriver per fact), C108 (the corpus read). 
  `decisions/frontends.md`: C16 (the page renders documents and composes
  nothing; a presentation choice is the URL's), C53 (output levels: ten
  listed by default, what an answer is for never cut, the number held
  once, JSON changes additive), C115 (the total and the view first),
  C116 (no ruling id in user text), C138 (no new route is the
  session's), C140 (item art is the browser's), C143 (a shared text
  speaks the request's words, never a flag), C163 (every time a document
  carries has its words beside it). `decisions/daemon.md` C166 (the job
  list fits its frame; read for what it bounds, Input says why it does
  not bound this). `AGENTS.md` C46, C122 (pinned once at the lowest
  crate; wording a golden), C153 (a finding joins the surface that
  holds it), C154 (a retirement names its holder).
- `REVIEW-CHECKLIST.md`, "Search" ("A second maker", "A block past its
  bound") and "Frontends and the surface an agent meets" ("A document's
  key order lost through a JSON value", "A behaviour pinned at three
  layers", "Page state the scan cannot see").
- `python3 -I tools/bothy.py pack --path crates/acquisition-search/src/show.rs`
  and the same for `crates/acquisition-cli/src/web/page.rs`, then:
  - `crates/acquisition-search/src/show.rs`: the module doc; `Shown`
    (line 69) and `without_lines`; `ShownLine`; `shown` and `read_one`
    (line 252: how one item is read, placed, derived, classed, priced and
    projected — the lines taken out of the item into `ShownLine`s with
    their slots and limit, the category made by
    `categories.shown(&item, &class, &categories.of(&item, &class))`,
    `place.public` set from the price join); `Shown::aged` (line 238).
  - `crates/acquisition-search/src/corpus.rs`: `Held` (line 357: `item`,
    `place`, `class`, `category` — the ids, `Category`, not `show`'s
    `category::Shown` — and `price`); `Corpus::read` (how each `Held` is
    made, the same `placed`, `derive`, `celled`, `tab_type_unread`,
    `classify` and `categories.of` as `read_one`; then `name_sockets`
    and the price join over the realm's live leagues);
    `name_sockets` (line 977).
  - `crates/acquisition-search/src/answer.rs`: the module doc's
    paragraphs on rows and continuations; `Rows` (line 599) and
    `RowField` (line 621); `Shape::of` (its refusals: a sort beside a
    count, explain beside a view); `Shape::bound` (`limit:
    rows.limit.unwrap_or(LISTED)`); `BoundView::Rows`; `RowsOut` and `Row`
    (lines 1485 to 1530); the rows' making in `answer` (from "the view:
    counts of the matches, or the rows", the `icon` flag, the `Row`
    literal, `next` with `fields: fields.to_vec()`); `Answer::aged`
    (line 1642).
  - `crates/acquisition-search/src/bind.rs`: `NOT_BUILT` (its `fields`
    entry, "the caller names a row's fields"; "an entry leaves in the
    commit that builds its construct").
  - `crates/acquisition-search/src/describe.rs`: the `answer` section's
    `rows` and `view` keys (what an agent reads of a row).
  - `crates/acquisition-cli/src/search_cmd.rs`: `SearchArgs::fields`
    (line 197) and its doc; `search` (the `unbuilt` refusal at line 339,
    the `typed` list a continuation and a request stand alone beside,
    the `Rows` literal with `fields: Vec::new()` near line 557);
    `answer_text_of`, `row_text`, `row_shows`; `show_item`; `shown_text`
    (line 2403) and every word function it and `page::item` call:
    `shown_title`, `shown_removed_words`, `class_words`,
    `category_words`, `open_words`, `shown_place_words`, `price_words`,
    `sockets_words`, `shown_properties`, `line_kind_words`,
    `line_template_quoted`, `line_slots_words`, `line_row_quoted`,
    `line_row_slots_words`, `place_text`, `title`, `UNREAD_NOTHING`.
    Three of them take a whole `Shown` (`shown_title`,
    `shown_place_words`, `price_words`) where they read only part of it.
  - `crates/acquisition-cli/src/web/page.rs`: the module doc; `answer`;
    `rows_table` (the compact table, `class="results"`, its "card's
    order" comment); `item` (the art, the badges, the fact table
    `class="shown"`, the lines table `class="lines"`, against, the basis
    foot); `item_badges`; `query_box`; `basis_foot`.
  - `crates/acquisition-cli/src/web/form.rs`: the module doc's names;
    `LIMITS` (line 68: 20, 50, 100, 200); `emitted`; `Folded::request`
    (it asks `fields: vec![RowField::Icon]` for every view); `fold`; its
    unit tests.
  - `crates/acquisition-cli/src/web/mod.rs`: `asked_by_the_form` (line
    548; its last lines replace an offered request's row fields with the
    form's own before comparing); `search` (line 884); `item`.
- The tests: `crates/acquisition-search/tests/rows.rs`
  (`c140_a_row_carries_the_icon_when_the_view_names_it`, line 1243; the
  fixture helpers it uses from `tests/common/mod.rs`: `store`,
  `list_tabs`, `fetch_tab`, `item`, `view`, `load`, `load_with`, `show`,
  `answer`, `intent`); `tests/price.rs` (how a price is written for a
  test); `tests/explain.rs` (the view-refusal table near line 825);
  `tests/refusal.rs` (`the_list_the_walk_checks_is_the_list_describe_prints`);
  `tests/surface_faults.rs` (line 724, `["fields", "show basis"]`).
  `crates/acquisition-cli/tests/search_json.rs`:
  `c11_a_failure_is_structured_and_a_later_steps_flag_is_refused_by_name`
  (line 889 and lines 952 to 958, the two `not_built` assertions this
  step retires), `step_10_next_request_and_print_request_at_a_terminal`
  (its `--print-request` table, from line 2088),
  `describe_and_show_print_json_whole_and_text_from_it` (its golden
  `an_items_detail`), `describe_at_a_terminal` (its golden, line 318
  lists `fields (--fields)` as not built). `crates/acquisition-cli/tests/web.rs`
  and `harness/mod.rs` (`Pages`, `page_golden`, `normalise`), every test
  Input names, and the goldens under `tests/goldens/web/`.
  `crates/acquisition-mcp/tests/surface_size.rs` and
  `search_tools.rs` (the `describe` sizes near line 729: `answer` under
  7,500 bytes).
- `search/MEASUREMENTS.md`, its head and M3 and M8 (how a measurement
  here is taken and recorded: the copy, the command, the numbers as
  measured); `crates/acquisition-search/examples/copy-as-store.rs`.
- Issue #98.

## Input

**The spike: what `fields: [item]` adds, measured before it exists.**
The design's spike asks `fields: [item]` at limits of 20, 200 and 1,000
over the measurement copy, bytes and milliseconds, to set the row cap.
`RowField::Item` does not exist, so this session measured what it will
carry: for each row of the empty query's answer in pc at limit 1,000
(the store's order), `show` of that id, its five keys `item`, `class`,
`category`, `price` and `lines` taken from `show`'s JSON and serialized
as one object — exactly what a row will carry under `item`, once the
equality Do 3 pins holds — summed over the first N rows, beside the
compact answer at N (`fields: ["icon"]`, what the page asks today). The
time: the compact answer asked over a corpus held in memory (the web's
and the MCP's case, C98's holder), serialized, and the serialization of
the N items' five keys, each the median of eleven. Its bias: the bytes
are exact by construction (`show`'s own document); the time leaves out
the projection itself — a clone of each held item, its lines' slots and
its category's words — since its function does not exist yet, and every
renderer (the CLI's text, the page's HTML, a browser's layout).

The copy: a fresh `sqlite3 -readonly <source> ".backup <scratch>/copy.db"`
of `spike-GERWARIC_7694-2026-09-13.db`, the M3 copy, which is not in
this tree's `search/item-facts/raw/` (the root swap left `raw/`
behind; it survives in the pre-swap clone,
`/Users/tom/Development/GitHub/gerwaric/acquisition/spikes/rust-playground/search/item-facts/raw/`),
its SHA-256 `fbae8786…bccfda7` the same before and after the backup.
The program: a scratch crate linking `acquisition-search` and
`acquisition-store` by path at `4b28dad1`, release profile, opening the
backup with `Store::open` and an empty intent file in memory
(`Annotations::open_memory_for`), never `acq`, no daemon, no store
directory of the owner's; its source and its output are in the group's
notes folder (`4-full-1-cards-spike-main.rs`, `-run.txt`, beside the
brief's session notes). Apple M4, rustc 1.99.0, 2026-11-08, run alone.

| N | compact answer, compact JSON (pretty) | the items' five keys, compact (pretty) | a row's five keys, mean | answer and its JSON, held corpus | the five keys' JSON |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 20 | 15,188 B (20,419) | 38,315 B (57,281) | 1,916 B | 3.0 ms | 0.0 ms |
| 200 | 128,387 B (167,328) | 438,085 B (676,521) | 2,190 B | 3.1 ms | 0.4 ms |
| 1,000 | 635,140 B (821,973) | 2,266,863 B (3,510,376) | 2,267 B | 3.9 ms | 2.1 ms |

A row's five keys over the 1,000: min 986 B, median 2,237 B, p90
3,043 B, max 5,449 B. The corpus's load, once per basis: 458 ms for the
22,623 live pc items (of the copy's 22,721; M3's count). Each `show`
was 86 ms (85.7 s for the 1,000), a full read an item: what
`GUI-NOTES.md` §7 said a card per `show` would cost.

**The cap it sets: 200 rows under `item`.** Time does not bind: over a
held corpus the answer and its JSON at 1,000 rows with the items is
under 6 ms here, and the CLI's ask is its load. Bytes do: a row's item
is about 2.2 KB beside a compact row's 0.64 KB, so a page of 1,000
cards is 2.9 MB of JSON before its HTML, and 200 is 0.57 MB. 200 is the
largest number of rows the answer page's Show control offers today
(`form.rs`, `LIMITS`), so no page choice is cut; a thousand rows at
once is step 6's server-windowed table, not a page of cards. The cap is
a count, held once in the search crate beside the rows (C53, the number
held once), never a byte measure.

**Where the design and the tree disagree.**
- The design's §4: "the byte ceiling is the frame's (C166), whose
  refusal stands where a capped page would still pass it". C166 bounds
  the daemon's IPC frames (`acquisition-protocol/src/protocol.rs`,
  `MAX_FRAME_BYTES`, 64 MiB a line), and no search answer crosses one:
  the CLI, the web server and the MCP each answer a search in their own
  process over the store's read (C12's second surface). So no byte
  ceiling stands over an answer today, and this step adds none: the
  cap is the bound, and the brief raises the absence for the owner.
- The design's §1: "`Held { item, place, class, category, price }` …
  `Shown` is the same five with the basis". `Held.category` is the ids
  (`Category`); `Shown.category` is `category::Shown`, the ids with the
  bucket and every id left open with why, made from the ids by
  `categories.shown`. The one maker makes the second from the first.
- The design's equality, "a row's `item` equals `show`'s": `Shown.item`
  is the item without its lines, which follow with their slots, beside
  `class`, `category` and `price`; the pin compares those five keys
  (Do 3). A row's `place` stays its own and is not compared: the corpus
  names what a socketed item sits in (`name_sockets`), `show` gives its
  id alone (its module doc), so the two places differ for a socketed
  item by design, and a card says the parent's name on the answer page
  and its id on the item page (the foot raises it).
- The design names the copy at `search/item-facts/raw/`; it is not in
  this tree (above). The build's measurement (Do 2) takes its backup
  from the same source.

**What an agent meets.** The MCP's `search` reads its request with the
search crate's reader (`acquisition-mcp/src/main.rs`, `RequestParams`),
so `view.rows.fields: ["item"]` reaches it the commit `RowField::Item`
lands, with the cap. No tool's text names `fields` today (the `view`
parameter's description lists `limit`, `sort`, `desc`), and this step
changes no MCP text: the first turn's bound in `surface_size.rs`
(43,000 bytes) holds what it holds now. `describe`'s `answer` section,
where an agent reads a row's keys, is held under 7,500 bytes by
`search_tools.rs`, last measured at 7,401: one clause naming the fields
would pass it, so this step adds none and the foot raises it.

**The tests and goldens the step changes**, each found by grepping
(`grep -n 'remove("icon")\|<th>Price\|class=\\"results\\"\|not_built' crates/acquisition-cli/tests/web.rs crates/acquisition-cli/tests/search_json.rs`,
`grep -rn '"fields"' crates/acquisition-search/tests`, and the goldens'
`GET search` and `GET item/` entries):
- The page's JSON compared with the CLI's, the page's art taken out of
  each row first (`row.as_object_mut().unwrap().remove("icon")`): `web.rs`
  lines 373 and 374 (`the_fold_answers_as_acq_search_trade_does`), 392
  and 393 (`trade_answers_as_the_cli`, called by
  `a_trade_url_in_the_box_answers_as_acq_search_trade_does`), 578
  (`the_query_box_sorts_and_pages_a_typed_query`) and 2916
  (`saved_open_runs_the_rows_search_and_writes_nothing`). The page will
  ask `[icon, item]`, and the CLI can now ask the same: each CLI side
  gains `--fields icon,item` and the removal goes, so the page's JSON
  equals the CLI's whole, the art and the item included (Do 8). The
  view a zero's context offer carries stays taken out of both in
  `trade_answers_as_the_cli`, as its comment says.
- `every_search_page_is_a_wording_golden`: the ring search's `# holds`
  `"alt=\"Doom Loop Two-Stone Ring\""`, `"class=\"name rarity-rare\""`
  and `"+95 to maximum Life"` (lines 638, 639 and 642) stand on a card (Do 6
  keeps the art's `alt` and the name's class); line 763,
  `!counted.body.contains("class=\"results\"")` (a count shows no rows),
  gains the card's class beside it; the item page's holds at lines 843
  and 845 (`"<th>Class</th><td>Rings</td>"` stands in the facts table;
  `"<th>Price</th><td>none ("` moves to the card's price) and the price
  slice at line 851 (`find("<th>Price</th>")` to `find("</tr>")`, "no
  why, no link") are rewritten to the card's price element, the same
  property.
- `c11_a_failure_is_structured_and_a_later_steps_flag_is_refused_by_name`:
  line 889, `(vec!["search", "--fields", "name"], "not_built")`, and
  lines 952 to 958, the `not built: fields (waits for a consumer that
  asks)` wording: retired (Do 9).
- `describe_at_a_terminal`'s golden, line 318: the `fields (--fields)`
  row of the not-built list leaves with the entry (Do 4).
- `surface_faults.rs` line 724: `["fields", "show basis"]`, the
  constructs `describe` answers though listed as not built; `fields`
  leaves the list, so the allowance and its comment name `show basis`
  alone.
- The goldens that regenerate: `search_pages.txt` (nine answer pages
  of its twelve `GET search` entries, three being refusals, and its item
  page `GET item/r1`; `GET item/t1` is a refusal), `saved_pages.txt`
  (eight answer pages), `moved_world.txt` (two answer pages and the two
  item pages), `answer_ways.txt` (six answer pages); and
  `search_json/describe_at_a_terminal.txt`,
  `search_json/an_items_detail.txt` (Do 8 adds a case to it).
  `search_form.txt`, `console_realm.txt`, `jobs_halted.txt` and
  `refresh_loop.txt` hold no answer or item page and must not change.
  `CLI-REFERENCE.md` regenerates (`--fields`'s help); `MCP-REFERENCE.md`
  must not change.
- Besides these, `no_ids.rs`'s sweep reads every web golden (no class or
  label the card adds may read as a ruling's id), and the way check
  (`every_request_an_answer_carries_is_a_way_on_its_page`, `web.rs` line
  1175) walks every object in an answer's JSON holding a `request`: the
  rows' new keys hold none (`grep -n request crates/acquisition-search/src/price.rs crates/acquisition-search/src/category.rs crates/acquisition-search/src/class.rs`
  finds one doc comment and no field), so it stands unchanged.

**The last push's CI**: `Gate (Windows)` run 42498341755 on `4b28dad1`,
`success`. `Gate (Linux)` run 42498341788 on `4b28dad1`, `failure`: one
test, `web::the_refresh_loop_runs_through_the_pages`, whose golden
`refresh_loop.txt` records the plan page's basis line with the stash
listing's `response_id` 4 where Linux renders 5; the same test failed
the same way on `7c1ce05a` (run 42496970974), and `098f70c6` was green
(run 42495576834). No issue holds it (`gh issue list --label trigger:ci`
lists #179 and #150). It is not this step's: the refresh page and its
golden are outside it, and this step's Done when requires the golden
unchanged. The orchestrating session files it as `trigger:ci`,
`area:frontends`, before the build starts; the step goes on.

## Do

1. **No ruling is needed.** C100 says the caller may name fields; the
   cap is a count the design gave to the spike; the fold's
   `view.fields` is the design's (§2), its routes unchanged (C138). No
   document field changes meaning: every JSON change is a key added
   (C53, JSON changes additive). The words and the card's layout this
   brief leaves open are the step's own design (Rules).
2. **The one maker, `show`'s projection factored out of `read_one`**, in
   `show.rs`: one function that takes a `Held` and makes what `show`
   carries of the item — the item without its lines; the lines, each
   with its slots and its limit; the class; the category as `show` says
   it (`categories.shown` over `categories.of`); the price — and one
   function that ages it (the item's first and last sighting and its
   removal, in words at `now`, as `Shown::aged` does today). Suggested
   shape: a struct of the five (say `ShownItem`), serialized flat with
   the keys `item`, `class`, `category`, `price`, `lines` in that order,
   `item` through `without_lines`; `Shown` holds it flattened between
   `place` and `against`, so `show`'s JSON is byte for byte what it is
   today (every test and golden reading `show`'s JSON or text unchanged
   is the proof). `read_one` builds the `Held` it already builds for
   `against` and calls the maker; nothing else in `show.rs` makes any of
   the five. A key order that matters goes through a struct, never a
   `serde_json::Value` (the checklist's row).
   Then **the measurement, built**: an example,
   `crates/acquisition-search/examples/rows-item.rs`, that opens a copy
   as `copy-as-store.rs` does (a path argument, never a store
   directory), holds the corpus of pc, and for 20, 200 and 1,000 rows of
   the empty query asks `fields: ["icon", "item"]` with the cap set
   aside for the run (a constant the example passes, or a limit the
   build lets the example ask; the worker's choice, named in the
   commit), printing the answer's compact JSON bytes and the median of
   eleven asks of the answer and its JSON. Run over a fresh
   `sqlite3 -readonly … ".backup …"` of the same source under the
   worker's scratch, release, alone; the numbers go to
   `search/MEASUREMENTS.md` as M9, its verdict first, in the form of M8
   (the command, the copy, the date, the machine, the numbers). The cap
   stands where the 200-row answer is within a tenth of Input's
   566,472 bytes (the compact answer's and the five keys' sum) and its
   time under 50 ms; otherwise the build stops before Do 3's cap and
   brings the numbers to the orchestrating session.
3. **`RowField::Item` and the cap**, in `answer.rs`:
   - `RowField` gains `Item` (`"item"`): each row carries what Do 2's
     maker makes of its `Held`, flat after the row's own keys, after
     `sort`: `item`, `class`, `category`, `price`, `lines`, absent where
     the view did not ask. `Answer::aged` ages each row's item by Do 2's
     one function. The compact keys stay (`name`, `base`, `rarity`
     repeat inside `item`; C53 additive).
   - `RowField` gains one reader of its words for the frontends (say
     `RowField::parse`), refusing a word that is no field with a
     structured error of kind `unknown_name` whose words name both
     fields, drafted: "`price` is no field a row carries: `icon`, the
     URL of the item's art, or `item`, the item as `show` gives it".
     The request's JSON reader stays serde's (an unknown field is
     refused there today, `c140_`'s last assertion).
   - The cap, held once beside the rows (say `pub const ITEM_ROWS:
     usize = 200`, its doc citing M9): under `item`, the rows view
     answers at most that many rows, whatever `limit` asks; the rest are
     `left_out`, and `next` continues them at the applied limit, asking
     the same fields. An item is never cut: every row carries its five
     keys whole (C53, what an answer is for).
   - `RowsOut` gains `fields`, the fields the rows carry as asked,
     absent where none (`skip_serializing_if` empty), beside `sort` and
     `desc`: the view as it ran, so a reader sees why `limit` is 200
     where it asked more, and a page reads its presentation from the
     document it renders (Do 6).
   - `Shape::of` refuses `fields` beside a count, a crossed table or
     explain, as it refuses a sort there (kind `view`), drafted:
     "`fields` names what each row carries, and a count shows none: a
     table counts items and lists none" (and in the explain branch's
     list, `fields` beside `count`, `cross`, `sum`, `sort`).
   - The docs: `Rows::fields` ("`icon` alone" today), `RowField`'s, the
     module doc's paragraph on rows, `derive.rs`'s `icon` doc where it
     says what copies it, `show.rs`'s module doc (the projection is the
     one maker a rows view calls).
4. **The not-built list.** `NOT_BUILT` loses its `fields` entry, in the
   commit that builds it (`bind.rs`'s rule). `--describe`'s not-built
   list follows (the golden, Input); `surface_faults.rs`'s allowance is
   trimmed (Input).
5. **The CLI's `--fields`**, in `search_cmd.rs`:
   - `--fields NAME,…` read by Do 3's reader, each name once (a name
     twice is one), into the rows view's `fields`; an unknown name is
     that refusal. Beside `--count`, `--cross` or `--explain` it is
     `Shape::of`'s refusal. It joins the `typed` list a continuation
     and a request stand alone beside (a token and a request carry their
     view whole). Beside `--saved` it stands, as `--limit` and `--sort`
     do. The `unbuilt` check keeps `rebind` alone.
   - The help, drafted: "What each row carries beside its compact
     fields, by name: `icon`, the URL of the item's art; `item`, the
     item as `show` gives it — its lines, class, category and price.
     Under `item` an answer holds at most 200 rows; the rest are its
     continuation." `CLI-REFERENCE.md` regenerated under
     `ACQ_UPDATE_FIXTURES=1`.
   - The text (C53, a function of the JSON): a row prints its head, its
     shows and its `id` line as today; under `icon` an `icon` line after
     the `id` line, `            icon <url>`, or `            icon none`
     where the body names none; under `item` then the item as `show`
     prints it, from its `base` line to its `unread` lines, unindented,
     the price line with its why command as `show` prints it, and a
     blank line after. Not printed again: the title, `id`, `removed` and
     `place` lines (the row's head says them) and the basis (the
     answer's foot says it once). `shown_text` is split so the item's
     block is one function both call; the why command names the account
     as every command an answer prints does (`Answer::account_named`).
6. **The card**, in `page.rs`, one function rendering one item from
   what Do 2's maker makes and its place, with no word of its own that a
   word function in `search_cmd.rs` makes (one maker per wording): the
   three word functions that take a whole `Shown` and read part of it
   (`shown_title`, `shown_place_words`, `price_words`) are retyped to
   the part they read, so a row's item and `show`'s are said by the same
   calls. The card, in the site's order, each element with a class of
   its own:
   - the art (`<img src alt>` as today, the `alt` the title), the
     item's badges (`item_badges`, retyped to the item's parts: class
     undecided, unread, flags unknown or unread);
   - the name, `shown_title`'s words, `class="name rarity-<rarity>"`,
     a link to `item/<id>` on the answer page and none on the item page;
   - the base;
   - the properties, the item level and the requirements, each a line,
     `shown_properties`'s lines in its order;
   - the lines, grouped by source in the item's own order (a group per
     run of one source, no sort: the page composes no order), each its
     text, a row break shown as a line break, and its kind
     (`line_kind_words`: the source and the flags);
   - the flags (`is`, the item's `flags` joined as the text joins them),
     where there are any;
   - the sockets (`sockets_words`), where there are any;
   - the price (`price_words`: the price and its side, or none with the
     reason, or undecided with the problem; no link, no command, no
     why);
   - the place with its age: `shown_place_words` (realm, then
     `place_text`, whose words carry the fetch's age), and where the item
     left, `shown_removed_words`.
   The answer page under `[icon, item]` renders a card per row in the
   answer's order, each followed by what the row shows of the query
   (`row_shows`, its "more" pieces linking `item/<id>` as the table's
   do) — `<div class="cards">` around them, no `class="results"` table;
   under `view.fields=icon` the compact table as today; the card is
   chosen by the document's `view.rows.fields` (Do 3), never by the
   URL. The item page renders the card, then a fact table
   (`class="shown"`) of what the card leaves out — Id, Class, Category
   with its open ids, Stack, Note, Unread — then the lines table with
   templates, slots, rows and limits, against, and the basis foot, as
   today. The CSS for the card goes in the page's one stylesheet,
   `assets/page.css`, beside the table's.
7. **The fold's `view.fields` and the presentation link.**
   - `form.rs`: `view.fields` joins the names `emitted` lists, read by
     Do 3's reader as a list (`icon`, or `icon,item`); `Folded::request`
     asks `[icon, item]` where no `view.fields` was sent and the view is
     rows, the list sent where one was, and no fields for a count (Do
     3's refusal would refuse them). The module doc's list of names gains
     it. Unit tests beside the fold's: absent, `icon`, `icon,item`, an
     unknown word refused as a usage error, `view.fields` beside
     `view.count` refused.
   - `mod.rs`: `asked_by_the_form` folds a rows request's fields back:
     exactly `[icon]` is `view.fields=icon`; `[icon, item]` and none
     (a route carries none) are the page's default and add no field; any
     other list is no form's (`None`, so no link). Its last lines, which
     replace an offered request's fields with the form's, keep doing so
     for the comparison.
   - The query box carries `view.fields` hidden where the answer's
     `view.rows.fields` is `[icon]`, so an edited query keeps the table.
   - The answer page reached by the form's fields (not a continuation:
     a token holds its view whole) carries one link to the same answer
     in the other presentation: the page's own URL, its fields as sent,
     with `view.fields=icon` added, or removed. The handler passes it to
     `page::answer` (the URL's fields are the handler's; the page builds
     no request). Its words, drafted: "as a table" and "as cards".
8. **Pins, each at the lowest crate that holds it** (C122), beside the
   tests they join (C153):
   - **The one maker, by equality** (`rows.rs`, a test named for the
     property, say `c100_a_rows_item_is_the_one_show_makes`): over a
     fixture holding at least a mod of several rows (the file's
     `stash()` has them), an item priced by a price written to the
     intent file (as `tests/price.rs` writes one), an item whose class
     is undecided, and, under membership `all`, a removed item: every
     row of the answer under `fields: ["icon", "item"]` has keys
     `item`, `class`, `category`, `price`, `lines` equal to `show`'s for
     its id, with the same intent file, both aged at one `now`
     (`Answer::aged`, `Shown::aged`), and the answer's `basis` equal to
     each `show`'s. Seen failing once with the row's category made from
     the ids alone (the report says so).
   - **The cap** (`rows.rs`, joining `c140_`'s neighbour, say
     `an_item_row_view_holds_at_most_the_cap_and_continues`): a fixture
     of `ITEM_ROWS + 1` items built inline (no new seed function); a
     view asking `limit` past the cap under `item` returns the cap's
     rows, `left_out` 1, `view.rows.limit` the cap, `view.rows.fields`
     as asked, and its `next` followed returns the last row with its
     item; the same view without `item` returns every row asked.
   - **The refusals**: `fields` beside a count in `explain.rs`'s view
     table (Do 3); an unknown field word where `RowField`'s reader is
     tested in the search crate; the CLI's `c11_` table gains
     `--fields name` (kind `unknown_name`) and `--count tab --fields
     item` (kind `view`), the adapter's refusal shape.
   - **The CLI's equality** (the retirement's holder):
     `step_10_next_request_and_print_request_at_a_terminal`'s
     `--print-request` table gains a row, `search --realm pc LIFE
     --fields icon,item`, and asserts the printed request's
     `view.rows.fields` is `["icon", "item"]`: the typed `--json` answer
     equals the request's answer, as every row of that table does.
   - **The CLI's text**: `describe_and_show_print_json_whole_and_text_from_it`'s
     transcript gains `search --realm pc LIFE --fields icon,item --limit
     2`, its golden `an_items_detail` naming, in `# holds` facts, a
     row's `icon` line and the item's `class` and `lines` lines.
   - **The page**: the four JSON equalities of Input compare whole with
     `--fields icon,item`; `every_search_page_is_a_wording_golden`
     records the ring search as cards (holds: the card's class, a line's
     kind, the price's element, the place's age) and gains one page, the
     same search under `view.fields=icon`, holding `class="results"` and
     the "as cards" link, followed for its status, 200; the item page's
     holds as Input says. The way check, the no-command test and the
     basis test stand as they are and run on every page they run on
     today.
   - Every test under C137 (no wall-clock bound): the cap's pin counts
     rows, and M9's numbers are the record's, never asserted.
9. **The gate and the commits.** `tools/gate.sh` green on the step's
   tree; one commit per concern, the last naming this brief and setting
   nothing in the roadmap (the close is the orchestrating session's).
   The commit that builds `--fields` carries, in C154's form (a
   retirement names what now holds it): "retired the `--fields` row of
   `c11_a_failure_is_structured_and_a_later_steps_flag_is_refused_by_name`
   (kind `not_built`) and its `not built: fields` wording; held by the
   `--fields icon,item` row of
   `step_10_next_request_and_print_request_at_a_terminal`'s
   `--print-request` table, its typed answer equal to its request's".
   The commit that changes the goldens names each golden and what moved
   in it. The push and the review are the orchestrating session's.

## Done when

- `grep -n '"fields"' crates/acquisition-search/src/bind.rs` finds no
  `NOT_BUILT` entry; `RowField` has `Icon` and `Item`; the cap is one
  constant in the search crate, 200, its doc citing M9.
- `show.rs` makes the five keys in one function that `read_one` and the
  rows view both call; no other function in the workspace builds a
  `ShownLine`, calls `categories.shown`, or assembles a row's item
  (`grep -rn "ShownLine {\|categories.shown\|\.shown(" crates/*/src`
  finds the maker alone).
- `rows.rs` holds the equality pin and the cap's pin under the names the
  commit gives them; `explain.rs`'s view table and the CLI's `c11_`
  table hold the new refusals.
- `search/MEASUREMENTS.md` holds M9 with its command and numbers, and
  `crates/acquisition-search/examples/rows-item.rs` builds.
- Every golden and test reading `show`'s JSON is unchanged; the CLI
  goldens that move are `describe_at_a_terminal.txt` (the not-built row)
  and `an_items_detail.txt` (the added case) alone.
- `acq search … --fields icon,item --json` equals the answer of the
  request `--print-request` prints, in the table named; the retirement
  line of Do 9 is in a commit message.
- The web goldens that move are `search_pages.txt`, `saved_pages.txt`,
  `moved_world.txt` and `answer_ways.txt`; `search_form.txt`,
  `console_realm.txt`, `jobs_halted.txt` and `refresh_loop.txt` are
  unchanged; no `remove("icon")` is left in `web.rs`.
- `page.rs` renders a card per row from the row's own keys and the item
  page through the same function; it calls no function that takes a
  whole `Shown` for a word it could take from the item's parts; no
  `<table class="results">` on a default answer page, one under
  `view.fields=icon`.
- `MCP-REFERENCE.md` and `surface_size.rs` are unchanged.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Step 6: the dense table, a server-windowed rows view, a virtualiser,
  the browser-driven check. A thousand rows on a page is its question.
- A new route (C138): the cards are the answer page's; the toggle is a
  URL field of the one route.
- Any MCP text, a `describe` clause naming the fields, or a raised
  bound (the foot raises them).
- A byte ceiling over an answer (the foot raises the absence).
- A card sorted, filtered or grouped by the page beyond the run of one
  source in the item's order; a price link or a price page (C168: no
  annotation page until it is designed).
- Saving the view in a saved search (the design's §9: version 2 of the
  kind); the save form's dropped sort (#186); the owner's page issues
  #180, #181, #183 to #185, #190; the jobs and refresh pages (#182).
- The red Linux run (Input): filed, not fixed here.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push. (Binds your work.)
- Every `acq` you run is in a mock world (the mock-session skill):
  `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch; never a bare
  `acq`. The measurement opens a fresh backup by path under your
  scratch, never the source, never a store under `~/Library`. Nothing
  in this step needs a daemon. (Binds your work.)
- A word, a class or a layout this brief leaves open is the step's own
  design, named in the report and in the commit that builds it; a label
  is presentation, the words of a fact the word function's. (Binds the
  thing you build.)
- A choice that would add a route, change a document field's meaning,
  touch the MCP's texts or need a ruling stops the step and comes to the
  orchestrating session. (Binds your work.)
- A sentence of this brief the tree disproves is corrected in the
  report, not silently built around. (Binds your work.)

## Report

`TEMPLATE.md`. Add, under 3, M9's table beside Input's and the cap's
verdict; the web goldens' diffs summarised (lines added and removed per
golden); and under 5, the equality pin seen failing as Do 8 asks.

## For the owner's review

Agent-facing design decided here, the session's:
- **The cap, 200 rows under `item`**, the rest a continuation, set by
  bytes (time does not bind), equal to the page's largest Show choice;
  a larger `limit` is lowered to it, said by `view.rows.limit`, never
  refused.
- **A row's item is five flat keys**, `item`, `class`, `category`,
  `price`, `lines`, `show`'s own, after the compact keys; the compact
  keys stay.
- **`view.rows.fields`** echoes the fields asked, as `sort` and `desc`
  do.
- **`fields` beside a count is refused**, as a sort there is; an
  unknown field word is `unknown_name`, its words drafted in Do 3.
- **The CLI's text under `item`**: each row's head, shows and id, then
  `show`'s lines from `base` to `unread`; under `icon` an `icon` line.
- **The card's order and content** (Do 6): art and badges, name, base,
  properties with item level and requirements, lines by source with
  their kinds, flags, sockets, price, place with its age; the row's
  matched lines under it on the answer page; the item page's card then
  a facts table of the rest.
- **Cards the page's default**, `view.fields=icon` the table, and a
  link between the two on every answer page the form reached ("as a
  table", "as cards"); none on a continuation's page.
- **A route opens as cards**, whatever presentation the page it was on
  had: a route carries no fields, and the fold's default is cards.

Raised, for the owner:
- **No byte ceiling over a search answer.** The design cites C166's
  frame; no search answer crosses a frame. The cap bounds a page of
  cards by count; a single item past 5 KB (the copy's largest is
  5,449 B) is never cut. Whether an answer wants a byte bound is a
  ruling.
- **An agent cannot learn of `fields`.** The MCP's reader takes it, and
  no text says so: a `view` parameter clause would come out of the first
  turn's 43,000 bytes (42,089 measured at step 2) and a `describe`
  clause out of the answer section's 7,500 (7,401 last measured). Which,
  if either, is the owner's.
- **A socketed item's place** names its parent on the answer page's
  card (the corpus names it) and gives the parent's id on the item
  page's (`show` reads one item): one card, two inputs.
- **The red Linux gate** on `4b28dad1` and `7c1ce05a`
  (`the_refresh_loop_runs_through_the_pages`, the plan page's stash
  listing response 5 where the golden holds 4): to be filed.

## Cuts

Cut 1, 2026-11-08, at `4b28dad1`.
