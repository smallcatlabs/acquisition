# 4-full-1-answer — the answer and item pages from their documents

Step 4 of 4-full part 1 (`roadmap/briefs/4-full-1-design.md` §4 "Ways
on through `Paged`" and §7 step 4; GitHub issue #176), for an Opus
worker that has not seen the design. Written against the tree at
`35cf33a7`, after step 3 (`roadmap/briefs/4-full-1-pages.md`) closed
on 2026-11-07, the pages' first sitting round, the trade site box
(`34751f0e`) and the blind page seat's round (`2fcd2cfc`, its close
`35cf33a7`), each a row at the foot of `roadmap/LEDGER.md`. The
orchestrating session asked for this brief on 2026-11-08, cut by a
session that read the design on `main` and the tree, nothing else of
the group. Cut again on 2026-11-08 under the cold review at `cacfe12e`
(`roadmap/reviews/4-full-1-answer-brief-cacfe12e.md`), the foot saying
what each finding changed. Where the design and the tree disagree the brief takes the
tree's fact and says so (Input). Where the design is silent on a
page's words or shape, this brief decides, and the foot lists each
decision for the owner.

## Goal

When this step is done the answer page and the item page of `acq web`
render their documents themselves, as HTML, and print no command line
and no command-line flag anywhere in them. Today `page::answer` prints
the search's opening and everything after the rows through the CLI's
own text (`search_cmd::answer_head`, `answer_tail`), and `page::item`
prints `show` through `search_cmd::shown_text`, so a player at the
page reads `acq search --account … --explain 1` where a link should
stand, and a price line that says "why: acq price show …". After it,
every request the answer document carries is either a link to the page
that takes it, spelled through the one shape a page has for its ways
on (`Paged`, the page's `Surface`), or, where no page takes it, its
count or its words alone; the item page's price line says the price
and why there is none, and links nothing. The text renderer (the
CLI's) is unchanged, byte for byte: the two renderers read one
document, each with its own golden, and the words a fact is said in
are made once. Closes #176, by its "Closes when": the pages spell the
search's values through `Paged`, not the CLI's text. It is the step
the owner's other page issues wait on (#180, #181, #183, #184, #185 say
"after step 4"), and the shape #182 repeats for the jobs and refresh
pages.

## Read

- `AGENTS.md`, `roadmap/README.md` "A step", and `TEMPLATE.md`.
- `roadmap/briefs/4-full-1-design.md` §1 "The pages print the CLI's
  text", §2 "Every document with its version" and "What pins it", §4
  whole (its last paragraph is this step; its first two are step 5's),
  §6 ("Cards composed from `Held.item` in the page … two makers"), §7
  steps 4 and 5.
- `decisions/frontends.md`: C16 (the page is presentation: it renders
  the documents the verbs produce and composes nothing; every document
  it holds carries its basis), C143 (a shared text speaks the request's
  own words, never a frontend's flag; a way on is a value each surface
  spells), C53 (output levels; text is a function of the JSON), C115
  (the total and the view before the terms; a count's request only
  where routes are asked), C116 (no ruling id in user text), C138 (the
  listener: no new route is the session's), C140 (item art is the
  browser's), C146 (a count says what it counts), C149. `AGENTS.md`
  C46 (shared semantics in Rust; a surface that evaluates for itself is
  a second semantics), C122 (a behaviour pinned once at the lowest
  crate; wording a golden naming its facts), C153 (a finding joins the
  surface that holds it), C154 (a retirement names its holder).
- `python3 -I tools/bothy.py pack --path crates/acquisition-cli/src/web/page.rs`,
  then:
  - `crates/acquisition-cli/src/web/page.rs`: the module doc (it says
    the pages render "with the CLI's own words where the CLI has them —
    the answer's opening and its terms, `show`"; this step corrects
    it); `answer` (the count, the badges, `answer_head` in a `pre`, the
    rows table, the continuation link, `answer_tail` in a `pre`, the
    query box, the save forms); `item` (the art, the badges,
    `shown_text` in a `pre`); `answer_badges`, `item_badges`;
    `refused` (how a page renders a way on: a string `request` a link,
    a `post` object a form); `Head::with`.
  - `crates/acquisition-cli/src/web/mod.rs`: the module doc's bullet
    "A refusal's ways on are the pages' own"; `Web::link`;
    `asked_by_the_form` (folds a request back to the form's fields, or
    answers none; read its doc for what it refuses: a site search, a
    tree, another account, a crossed table, a sum, an explained node);
    `Paged` and its `request` (the arms for `Way::Search`,
    `Way::Tabs`, `Way::RefreshPlan` and the saved ways; `_ => return
    None`); `search` (one `answer` call, rendered as the page or
    answered as JSON); `item`.
  - `crates/acquisition-cli/src/web/form.rs`: `emitted` (the names the
    fold takes: no `view.routes`, no explain, no context), `fold`.
  - `crates/acquisition-verbs/src/refusal.rs`: `Way` (`Search`, `Tabs`),
    `Surface` (`request`).
  - `crates/acquisition-search/src/answer.rs`: the module doc's
    paragraphs on routes, the zero answer and continuations; `Answer`
    and every type under it to `Suggestion`; `Request::token` and
    `Request::from_token` (a token is a request with its place, rows
    alone); `Answer::tabs_pointer`, `Answer::account_named`; `Router`
    (every route is a rows request under the answer's scope, except a
    value list's rest, a counts request); `crates/acquisition-search/src/counts.rs`:
    `CountsOut`, `Table`, `Bucket`, `Label`, `OfKind`, `CrossOut`,
    `Cell`, `Margin`; `crates/acquisition-search/src/show.rs`: `Shown`,
    `ShownLine`, `Against`.
  - `crates/acquisition-cli/src/search_cmd.rs`: `answer_head`,
    `answer_tail`, `answer_more`, `row_shows`, `trade_text`,
    `shown_text`, `basis_text`, and the helpers they call
    (`place_text`, `span_words`, `title`, `evidence_text`,
    `label_text`, `summed_text`, `sum_of_text`, `bucket_text`,
    `table_text`, `cross_text`); `crates/acquisition-cli/src/printed.rs`
    (`tabs_command`, `show_command`, `opening`: the command lines the
    text prints).
- `crates/acquisition-cli/tests/`: `web.rs` (`seed`, `offline_web`,
  `ring_search`, `query_string`, `the_fold_answers_as_acq_search_trade_does`,
  `trade_answers_as_the_cli`,
  `a_trade_url_in_the_box_answers_as_acq_search_trade_does`,
  `the_query_box_sorts_and_pages_a_typed_query`,
  `every_search_page_is_a_wording_golden`,
  `the_moved_world_renders_with_a_badge`, `offered_links`,
  `link_saying`, `main_of`, `saved_parts`, `follow`, `json_at`,
  `c16_every_page_s_basis_line_is_its_documents_own`,
  `the_saved_pages_are_a_wording_golden`, and how
  `the_form_renders_the_realm_it_is_asked_for` stages a stash listing
  with `Store::record`); `harness/mod.rs` (`Pages::page`,
  `page_golden`, `normalise`, `age_rule`, `basis_rule`, which masks a
  line starting `basis   store `); the goldens under
  `tests/goldens/web/`, `search_pages.txt` first; `no_ids.rs`; the
  CLI's text goldens under `tests/goldens/search_json/` and
  `tests/goldens/seat_faults/` (the text renderer's, which this step
  must leave as they are).
- `CLI-REFERENCE.md` `## acq web` (its first sentence, "a page over
  loopback that renders what these verbs print").
- Issues #176 (this step), #190 (Not in scope says why it is left),
  #182 (the same fault on the jobs and refresh pages, the next step).

## Input

**Fixed.** The answer document and the item document as the search
crate makes them today: this step changes no field, no request, no
JSON byte (`Answer`, `Shown`; C53, JSON changes are additive, and none
is needed). The routes the owner ruled (C138, §8 of the design): no
new route. The fold (`form.rs`): no new field. The text renderer's
output: every CLI golden unchanged.

**The inventory: every request the answer document carries**, read
from the tree this brief was written against, in `answer.rs` and
`counts.rs`, and what a page does with
each today and after this step. A request reaches the JSON under a key
named `request`, on the object that holds it: a `Route` is flattened
into its `Count`, and a `Count` into a `Bucket`, an `OfKind`, a
`Cell`, a `Together`, a `Removed`, so the `request` sits on that
object, beside a `count` that is a number. "The holder" below is that
object, and its JSON pointer (RFC 6901) is the way's name in the
page's markup (Do 3). The rule (the design's §4): a way on is a link
where the page's `Surface` spells it (`Paged::request` of
`Way::Search(request)`, which is `asked_by_the_form`), and its count
or its words alone where it answers none. A count whose holder carries
no `request` is a plain number, no way: by default an answer carries
only the routes a next step hangs on (C115; `answer.rs`, the module
doc; `answer_tail`'s sort into `first` and `rest`), so a value
bucket's count, a vocabulary row's kinds and a term's matched where
something matched carry none unless every route was asked, which no
page asks.

| # | The request | Its holder (pointer) | Its shape | After this step |
| --- | --- | --- | --- | --- |
| 1 | the continuation | `/view/rows/next` (`Next`) | rows, with a place: `view.rows.after` and a `basis` | a link, `search?next=<Request::token>`, absolute (Do 3) |
| 2 | a term's counts | `/terms/<i>/matched`, `/failed`, `/lacked`, `/undecided` (`TermOut`'s `Count`s) | rows | a link on the count, where the count carries a request |
| 3 | a term's only-together | `/terms/<i>/together` (`Together`) | rows | the same |
| 4 | a removed id | `/terms/<i>/removed` (`Removed`) | rows, membership `all` | a link (the fold takes `membership` since step 3) |
| 5 | a value list's rest | `/terms/<i>/resolved/rest` (`Resolved`'s `Route`) | counts, one key | a link where the form's count field reads the key, else the number of values alone |
| 6 | the root's undecided | `/total/undecided` (`Total`) | rows | a link on the count |
| 7 | a count's buckets | `/view/counts/tables/<t>/buckets/<b>`, its `/sources/<k>` and `/flags/<k>` (`Bucket`, `OfKind`) | rows | a link on the count where it carries a request (by default a key's `none` and `undecided` buckets) |
| 8 | a crossed cell and its margins | `/view/cross/cells/<c>`, `/view/cross/margins/<m>/none`, `/undecided` (`Cell`, `Margin`) | rows | the same |
| 9 | an explained node, forced | `/view/explain/forced_true/matched`, `/undecided`, and `forced_false`'s (`Forced`) | rows | the same |
| 10 | an ambiguous clause's other reading | `/query/trade/clauses/<c>/other/adds`, `/drops` (`Stake`; under `set_aside` the same) | rows | the same |
| 11 | the zero answer's undecided | `/zero/undecided` (`Zero`) | rows | the same |
| 12 | the zero answer's explain offers | `/zero/offers/explain/<i>` (`ExplainOffer`) | an explained node | the words alone: the part's path and its node (no page asks explain) |
| 13 | the zero answer's corpus offer | `/zero/offers/context` (`Offer`) | the corpus context | the words alone (no page asks the corpus) |

Beside the document's requests, one way on the page derives as the
CLI's text does: the scope's never-fetched tabs, `Answer::tabs_pointer`
(a function of the answer's JSON, not a field), today printed as
`printed::tabs_command`; after this step `Way::Tabs` through `Paged`,
a link to `refresh?realm=…&league=…` (the design's §4: "the scope
block's never-fetched list links the league's tabs page"). Its fields
come from the `TabsPointer`: `account` as it stands (it is
`Answer::account_named()` already), `realm` a `String` parsed into the
protocol's `Realm` that `Way::Tabs` takes (the pointer exists only
under one realm, so the parse cannot meet `all`; a word it refuses is
no way, as `Paged` answers none), `league`; its `other_leagues` is said
beside the link in words ("N other leagues hold never-fetched tabs",
the text's, made once, Do 2). The way stands where `tabs_pointer()` is
some, and nowhere else: not under every realm, and not for a league
the query excludes, even where the scope has never-fetched tabs.

**What a page can reach.** The fold asks a rows view or a counts view
(`form.rs`, `view.count`), and a token is a rows request with its place
(`Request::from_token` refuses any other), so no URL a page makes
answers a crossed view (8), an explained node (9) or the corpus context
(`set_aside`). The renderer renders them all (its match on `ViewOut` is
exhaustive) under the same rule; no golden reaches them (the foot
raises it). Item ids the document carries, which are no request — a
row's `id`, an undecided reason's `example.id` — link the item page,
`item/<id>`, which takes an id (a row's name does so today).

**The item document carries no request.** `Shown` holds no `Route`, no
`Request`; the "why" on the CLI's price line is a command the text
composes (`printed::opening("price show", …)`). The page prints the
price and the reason, and links nothing (the design's §4: "the item
page's price line has no link").

**Where the design and the tree disagree.**
- The design's §7 step 4 retires "the four `acq …` lines of
  `search_pages.txt`". At `7a72de85`, the design's tree, that golden
  held four (an answer's routes, three, and the item's why line); at
  `35cf33a7` it holds eleven `acq …` lines on answer and item pages
  besides its header (`grep -n "acq " crates/acquisition-cli/tests/goldens/web/search_pages.txt`:
  lines 75, 111, 113, 185, 186, 188, 194, 234, 236, 306 (a `# holds`
  fact) and 319): the trade site box added a zero answer's offers and
  a site search's routes. Two more goldens hold answer and item pages
  that print them: `saved_pages.txt` line 64 (a route) and
  `moved_world.txt` lines 19 and 45 (the why line) and 121 and 123
  (routes). The brief takes the tree's count: every `acq …` line on an
  answer or item page in the three goldens.
- The design's §4 puts the default `fields: [icon, item]` and the card
  on the answer page; that is step 5 (#98), and the design's own §7
  says so. This step keeps the table of rows as it is.

**The tests the step changes**, each found by grepping `web.rs` for the
CLI's layout (`grep -n '"total   \|"query   \|count   \|class   \|why: \|NOT RUN\|reported  \|from a URL\|from the query JSON' crates/acquisition-cli/tests/web.rs`,
and `grep -n 'link_saying' …`): a fact asserted on an answer or item
page in the text's layout, which the HTML renders in its own.
- `a_trade_url_in_the_box_answers_as_acq_search_trade_does` (`web.rs`
  lines 419 to 421): `"trade   from a URL: PARTIAL — 2 clauses not run"`,
  `"NOT RUN   stats.1    A crucible tree path is not read"`, `"reported
  status"`.
- `every_search_page_is_a_wording_golden`: lines 633 and 634 (`"from
  the query JSON: exact"`, `"total   2 matches"`), 660 and 661 (`"query
  class:rings …"`, `"total   1 match"`), 680 to 682 (`"from a URL:
  PARTIAL"`, `"NOT RUN   stats.1 "`, `"reported  status"`), 714 and 715
  (`"from the query JSON: exact"`, `"reported  sort  The site's sort is
  reported and not applied"`), 746 (`"count   rarity · 2 values"`, `"4
  rare"`, `"1  unique"`), 766 and 768 (`"class   Rings"`, `"why: acq
  price show --account 'Alice#1234' --realm pc item/r1"`), and the loop
  at 781 to 786 (`"count   rarity"`, `"count   line:resist"`, asserted
  on each answer a refusal's link reaches).
- `the_moved_world_renders_with_a_badge` (lines 962, 977): `"PARTIAL"`,
  `"stats.0.0"`, `"undecided"` — each a word the HTML keeps; confirm on
  the regenerated page.
- `every_search_page_is_a_wording_golden`, line 742: `assert!(!counted.body.contains("<table>"), "a count shows no rows")`
  fails once a count renders an HTML table. It becomes: no
  `class="results"` on the counted page, and each count table carries
  its own class (Do 4).
- `every_search_page_is_a_wording_golden`, line 655: `!edited.body.contains("trade   from")`
  (an edited text drops the trade input) passes on any HTML page. It is
  rewritten against the HTML's own mark of a translation (the trade
  section's class, Do 4): absent on the edited answer, present on the
  ring search's.
- `every_search_page_is_a_wording_golden`, lines 757 to 760: the item
  page's `!item.body.contains("--account u-web")` and its comment (the
  why line named the account by the basis's uuid, the page seat's fault
  13, `2fcd2cfc`). The why line goes, and the property with it: the
  assertion is deleted, and with it the plumbing that served only that
  line, `page::item`'s `account` parameter and the `item` handler's
  `account.entry.username.clone()`. The commit says the property is
  gone, not moved (no holder to name: nothing on the page names an
  account any more).
- `c16_every_page_s_basis_line_is_its_documents_own` (lines 1812 to
  1817) and `saved_open_runs_the_rows_search_and_writes_nothing`
  (lines 2395 to 2403): each finds the continuation with
  `link_saying(…, "more")`, which reads the last `href="` before
  `">more</a>"`, and follows it with the token put in front by the
  test (`at(&more)`, `format!("/{token}/{more}")`). The continuation
  becomes absolute (Do 3), so both follow `more` as it stands: lines
  1813, 1816, 2399 and 2402 drop their prefix. A way's `<a>` keeps
  `href` its last attribute and the words "more".
- The comment on `saved_parts` ("its own tail keeps the CLI's lines
  until step 4") becomes false and is corrected.
- The goldens that regenerate: `search_pages.txt`, `moved_world.txt`,
  `saved_pages.txt` (eight answer pages: its `GET search?<…>` entries),
  and the new one Do 7 adds. `search_form.txt`, `console_realm.txt`,
  `jobs_halted.txt` and `refresh_loop.txt` hold no answer or item page
  and must not change; the `acq …` lines in the last three are the
  refresh and jobs pages', #182's.
Besides `web.rs` and its harness (`harness/mod.rs`, `Pages`), one test
reads the web pages: `no_ids.rs`'s
`c116_no_user_facing_text_cites_a_record_id` sweeps every file under
`tests/goldens/web` and `src/web/assets`, so every label and every
class this step adds passes that sweep (no letter followed by digits
standing as a ruling's id). The JSON every page answers is unchanged,
so `the_fold_answers_as_acq_search_trade_does`,
`trade_answers_as_the_cli` and `the_query_box_sorts_and_pages_a_typed_query`
stand as they are.

**The facts the tests stand on.** `web.rs`'s `seed` as it is (no new
seed function, #142's rule). In `seed`'s world, unchanged, the pages
`every_search_page_is_a_wording_golden` records today, and three it
gains, each reachable there: rows left out (`view.limit=1`, the
continuation, row 1, as the basis test asks); a count with a bucket
that carries a request (`view.count=class`: the item `new`, Wombgift
Chalice, is in the `undecided` bucket, which carries its route by
default, row 7); the zero answer (the xbox URL page: two explain
offers, the corpus offer, rows 12 and 13). A world staged apart, so no
page of the existing goldens moves: a store moved under the server
reloads its corpus (`mod.rs`'s module doc, "One corpus, held"), and a
staging in `seed`'s shared world would change the scope and totals of
every page recorded after it. That world is a fresh `offline_web` in a
new test (Do 7), staged with `Store::record` inline as
`the_form_renders_the_realm_it_is_asked_for` stages a listing, in this
order, each page recorded after the staging it needs and read in the
light of the ones before it: a never-fetched tab (a pc stash listing
naming a tab no fetch has read); a removed id (a later fetch of a tab
that no longer holds an item, then a term `id:` of it under `live`);
a value list cut with its rest (a value list carries `rest` only
past `bound::LISTED`, ten values, `answer.rs`'s `rest_of`, and only for
a term `rest_of` keys, a field test or a line group with one
template; `seed`'s seven items cannot make one, so the staging writes
eleven or more items carrying eleven or more distinct values of one,
the build naming the term). An ambiguous clause with a stake (row 10)
is recorded where the build finds a form filter whose fate is
`ambiguous` (`search_form.txt` marks them) and an item over which its
other reading adds or drops; where none can be staged without a new
seed function, the report says why and the way test covers the row
by no page (the foot raises it).

**The last push's CI**: `Gate (Linux)` run 42490674447 and `Gate
(Windows)` run 42490674446, both on `35cf33a7`, both `success`.

## Do

1. **No ruling is needed.** No route, no fold field, no document field,
   no verbs change. Every word and shape the brief leaves open is the
   step's own design (Rules).
2. **One maker per wording, one enumeration of the counts; the layouts
   apart.** Where the HTML says a fact the CLI's text says, both call
   one function that makes the words, factored out of `search_cmd.rs`
   where the words are made today inside a line's `format!`; the label
   column, the padding and the line breaks are the text's, the elements
   the HTML's. The counts an answer carries are enumerated once: the
   walk inside `answer_tail` that sorts every `Count` (with its label,
   such as "term 1 undecided" or "clause 0 read the other way, adds")
   into `first` and `rest` and counts the non-zero ones that carry no
   route is factored into one function both renderers call; the text
   prints its routes block and "(N more: --routes)" from it, the page
   its number of counts left without a route (Do 4), so the page
   composes no number. It lives beside `answer_tail` in the CLI crate
   (a frontend's presentation, two layouts of one walk); a builder who
   finds it is semantics another frontend needs moves it to the search
   crate and says so. A function that makes a command line or names a
   flag (`printed::tabs_command`, `printed::show_command`,
   `printed::opening`, a request's `command()`, the words `--routes`,
   `--limit`, `--explain`, `--context`) stays the text's alone; where
   today one function makes a fact's words and its command together
   (`row_shows`'s `more`, `answer_more`, the never-fetched line, the
   zero offers, a table's "a larger --limit returns the rest", the
   price line's why), it is split so the text adds the command and the
   page adds its link or its words. Every CLI text golden (every golden
   under `tests/goldens/` outside `web/`) is unchanged by the step, byte
   for byte.
3. **The ways on, through `Paged`.** The answer page spells every
   request of the inventory (Input, rows 1 to 13) as
   `Way::Search(request)` through `Paged::request`, and the
   never-fetched tabs as `Way::Tabs` through it. One function in
   `page.rs` renders a way: where `Paged` answers a link, an `<a>`
   whose words are the count or the label it stands for; where it
   answers none, those words in a `<span>`. Both carry `class="way"`
   and `data-way="<the JSON pointer of the request's holder>"` (Input's
   table: `/terms/1/undecided`, `/view/counts/tables/0/buckets/2`,
   `/terms/0/removed`, `/terms/0/resolved/rest`, `/view/rows/next`,
   `/zero/offers/explain/0`; the tabs way `data-way="tabs"`), with
   `href` the `<a>`'s last attribute (`link_saying` reads it so). A
   count whose holder carries no `request` is a number with no class.
   `Paged::request`'s `Way::Search` arm takes a continuation first: a
   request with a place, rows `after` and a `basis` both present (the
   test `Request::from_token` makes; `place()` is private to the search
   crate, so the arm reads `request.basis` and the rows view's `after`,
   both public), is `Web::link("search", [("next", token)])`, absolute
   under the token like every other `Paged` link (a relative link on a
   refusal served at `saved/open` would resolve under `saved/`); any
   other request goes to `asked_by_the_form` as today. Its words are
   "more", and the answer page appends `saved` and `saved_revision` to
   it where it has them, as today. A refusal whose offered search
   carries a place gains the same link: the build reads every place a
   `Way::Search` is made (`grep -rn "Way::Search" crates/acquisition-verbs/src`
   finds two, one of them over the search crate's offered requests,
   `Offered` in `crates/acquisition-search/src/error.rs`) and says in
   the report whether any carries one, and what its link answers when
   followed.
4. **The answer page renders `Answer` itself**, in C115's order, with no
   `pre` of the CLI's text:
   - above the answer, as today: the saved notice, the realm notice,
     the count (`<p class="count">`), the badges;
   - the opening: a site search's translation in a section of its own
     class (its source and fidelity sentence, the "over" sentence, then
     a table of the clauses the text lists — not run, ambiguous,
     reported — each its status, its `at` and what it says, an ambiguous
     clause's other reading with its adds and drops as ways, row 10;
     every such clause the document lists, not the text's first ten,
     since the text's "… and N more: the JSON's query.trade.clauses"
     points at a JSON the page does not show); the query (the text, or
     the words for every item in scope); the set-aside query where there
     is one; the scope (account, realm, membership and its counts, the
     fetched and never-fetched locations by type, the list ages, the
     tabs way with its other leagues, the realm the store does not
     hold); the total with its span and its undecided way (row 6), the
     never-fetched sentence, the `before` line where there is one;
   - the view: the rows table as today (`class="results"`), each row's
     "N more of term P" and "N more sorted" a link to `item/<id>` in
     place of the `acq show` line, and the continuation (row 1); or a
     count's tables, an HTML table per key with a class of its own
     (not `results`), each bucket its label, its count (a way where it
     carries a request, row 7), its sum, slots and tally, a vocabulary
     row's term and its kinds with their counts (each a way where it
     carries one), a value's `needs`; the "N shown" words without a
     flag (the page's control is **Show**); or a crossed table (row 8)
     or an explained node (row 9) under the same rule;
   - the zero answer: each selector nothing carries with its
     suggestions (a suggestion's term is words, not a way), the explain
     offers (row 12) each its path and node in words and the number of
     the rest, the corpus offer (row 13) in words;
   - the undecided reasons, each its unread part, its example's name as
     a link to `item/<example id>`, its problem and hint, the rest
     counted;
   - the terms: an HTML table, a row per term (its path, its term, its
     matched, failed, lacked and undecided counts and its only-together
     count, each a way where it carries a request, rows 2 and 3), its
     value list beneath it (every value the document lists, ten at most,
     each with its count, not the text's first five; the rest, the
     document's `more`, a way where `rest` stands, row 5, else its
     number), its removed note (when it left, and the way, row 4, or
     that it held no item);
   - the counts left without a route (Do 2's enumeration): their number
     in words, the text's "(5 more: --routes)" without the flag, since
     no page asks every route (the fold has no `view.routes`);
   - the basis at the foot, in the words `basis_text` makes (the
     handoff's quote: the design's §3, "the basis on every answer is
     what each quotes to the other"), masked by the harness as the
     text's basis line is (Do 7);
   - the query box and the save forms, as today.
5. **The item page renders `Shown` itself**, by the same rule: the art
   and the badges as today; the title with its rarity and frame; then
   each part `shown_text` prints (id, removed, place, base, class,
   category with its open ids, price, stack, note, flags, sockets, the
   shown properties, the lines with their source, flags, template,
   slots, rows and limit, unread, `against` where present, the basis),
   as HTML, the words made once (Do 2). The price line says the price
   with its side and source, or none with the reason, or undecided with
   the problem, and nothing after: no link, no command, no "why".
   `page::item` loses its `account` parameter and the `item` handler
   its username (Input). The body is not on the page (the route asks
   none).
6. **The words that correct.** `page.rs`'s module doc (the pages render
   their documents as HTML, the words made once with the text's, and
   print no command line), `answer`'s doc (`page.rs`, above `answer`)
   and `item`'s; `search_cmd.rs`'s docs of `answer_head` ("what the web
   UI's answer page states above its table, word for word") and
   `answer_tail` ("what the web UI's answer page states under its
   table"), each then the text's alone and private where nothing else
   calls it; `mod.rs`'s bullet "A refusal's ways on are the pages' own"
   gains the answer's ways (every request the answer carries spelled
   through `Paged`, a continuation as `search?next=`, the count or the
   words where no page takes one) and keeps its last sentence true (the
   refresh and jobs pages still print the CLI's command lines, #182);
   `saved_parts`'s comment in `web.rs`; `acq web`'s help, "renders what
   these verbs print", becomes "renders the documents these verbs
   answer" (or the step's words for that fact), `CLI-REFERENCE.md`
   regenerated under `ACQ_UPDATE_FIXTURES=1`. README line 203 says
   nothing this step makes false; it stays.
7. **The goldens.** `search_pages.txt`, `moved_world.txt` and
   `saved_pages.txt` regenerate under `ACQ_UPDATE_FIXTURES=1`; each diff
   is read and holds the answer and item pages' new markup, the three
   pages Input adds to `every_search_page_is_a_wording_golden`, and
   nothing else (no form page, no refusal, no saved list changes). Each
   `# holds` fact Input lists is rewritten to name the same fact as the
   HTML says it, and each answer page records, where the answer has
   one: the total; the query; a term's undecided count as a link; the
   continuation; a bucket's way; on the xbox page the two explain
   offers and the corpus offer as words with no link; on the item page
   the price line with no "why". A new test (named for the property,
   say "the ways no seeded page reaches are a wording golden") records
   the staged world of Input into a new golden under
   `tests/goldens/web/` (say `answer_ways`) through `page_golden`: the
   never-fetched tab with the tabs link and its other leagues, the
   removed id with its way, the cut list with its rest, and the
   ambiguous clause where staged. Any value seen to differ between two
   runs is masked by a named rule in `harness/mod.rs` beside
   `basis_rule` (which reads a line that starts `basis   store `, so a
   basis in HTML needs the rule or a line that starts so), each named in
   the report. The run that regenerates is followed by a plain run
   before any commit (the sweep of Do 8 reads the files the goldens'
   tests write).
8. **Pins, what the adapter adds** (C122), in `web.rs` beside the tests
   they join (C153):
   - **Every way, where it stands** (a new test, named for the
     property, say "every request an answer carries is a way on its
     page"): over every answer page the two golden tests of Do 7 record
     with a status of 200, served again, the test walks the page's JSON
     document (same URL, JSON asked) for every object holding a
     `request`, skipping `query.trade.clauses[].site`, and finds on the
     HTML page exactly one `class="way"` element whose `data-way` is
     that object's pointer, and no `class="way"` element whose pointer
     names none (the tabs way apart: present exactly where
     `tabs_pointer()` would be some, which the test reads as the JSON's
     scope under one realm holding a never-fetched tab in a league the
     query does not exclude, and checked on the staged page and absent
     on a seeded one). For each `<a class="way">`, followed with JSON
     asked: it answers `200`; where its holder has a `count`, the
     answer's `total.matched` equals it (a route returns exactly the
     members counted); where it is `/view/rows/next`, the answer's
     `view.rows.after` equals the first page's `returned`; and its
     `query.text`, `scope.realm` and `scope.membership` equal the
     request's. For each `<span class="way">`, its pointer is
     `/zero/offers/explain/<i>`, `/zero/offers/context`, or a `rest`
     whose key the build names as one the form's count field cannot
     read. A recorded page per inventory row a page reaches: rows 1 to
     7, 11 to 13, and 10 where staged (Input); rows 8 and 9 reach no
     page.
   - **No command on an answer or item page** (a new test, say "an
     answer or item page prints no command line and no flag"): every
     page headed `GET search` or `GET item/` in every golden under
     `tests/goldens/web/` that answered 200, read from the files as
     `no_ids.rs` reads them, holds no `acq ` and no ` --` followed by a
     letter. Once, over the files: the pages served are the goldens'.
     It holds C143 on these pages; the refresh and jobs pages are not
     read (#182).
   - **One document, two renderers**: already pinned, and named in the
     close: the page's JSON equals the CLI's (`the_fold_answers_as_acq_search_trade_does`,
     `trade_answers_as_the_cli`, `the_query_box_sorts_and_pages_a_typed_query`),
     the page's basis line equals the JSON's basis
     (`c16_every_page_s_basis_line_is_its_documents_own`), the
     document's semantics in the search crate's tests, the text's
     wording in the CLI's goldens, the HTML's in the web goldens. No new
     test compares the two layouts (C122: the behaviour shared is the
     document's, held below the adapter).
   - Every test under C137 (no wall-clock bound).
9. **The gate and the commits.** `tools/gate.sh` green on the step's
   tree; one commit per concern or one for the step, the last naming
   this brief. The commit that changes the goldens carries the
   retirement line in C154's form (C154, a retirement names what now
   holds it): "retired the `acq …` lines, the `(N more: --routes)`
   flags and the `why: acq price show …` fact on the answer and item
   pages of `search_pages.txt`, `saved_pages.txt` and
   `moved_world.txt`; held by `every_search_page_is_a_wording_golden`,
   `the_saved_pages_are_a_wording_golden` and
   `the_moved_world_renders_with_a_badge` with their goldens
   regenerated, which hold the links in their place, and by the
   no-command test (Do 8)", naming the new test as built; and a line
   saying the item page's account check (lines 757 to 760) is deleted
   with the why line it guarded, its property gone. The push and the
   review are the orchestrating session's.

## Done when

- `grep -rn "search_cmd::\|printed::\|command()" crates/acquisition-cli/src/web/`
  shows only word-making functions and `search_cmd::trade_realm` (the
  URL reader `form.rs` and `mod.rs` call), each named in the commit
  message; `page.rs` calls none of `answer_head`, `answer_tail`,
  `answer_more`, `shown_text`.
- Every golden under `crates/acquisition-cli/tests/goldens/` outside
  `web/` is unchanged in the step's commits.
- `Paged::request`'s `Way::Search` arm links a request with a place as
  `search?next=`, absolute under the token; the answer page's every
  way goes through `Paged::request`, and the tabs way is `Way::Tabs`.
- `search_pages.txt`, `saved_pages.txt`, `moved_world.txt` and the new
  golden hold no `acq ` and no ` --` followed by a letter on any page
  headed `GET search` or `GET item/`; `search_form.txt`,
  `console_realm.txt`, `jobs_halted.txt` and `refresh_loop.txt` are
  unchanged.
- `web.rs` holds the two pins of Do 8 and the golden test of Do 7,
  under the names the commit gives them; no new seed function; the
  tests Input names are changed as it says.
- The module docs and comments Do 6 names say what the tree does;
  `CLI-REFERENCE.md` regenerated; `cli_reference.rs` green.
- The retirement lines of Do 9 are in a commit message in C154's form.
- Issue #176's "Closes when" is met, and the close says so, naming
  where each remainder of the issue's title and body lives: the refresh
  and jobs pages' command lines and the refusal words a page prints
  (`failure_words`, `console_realm.txt`'s "run `acq stashes …`") are
  #182's, a refusal keeping its input #190's; the orchestrating session
  closes the issue with a comment saying the same.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Cards, `fields: [icon, item]`, `RowField::Item`, the one `show`
  maker, the CLI's `--fields`: step 5 (#98). The rows table stays.
- The jobs page, the refresh page, the plan page and their command
  lines: #182, the step after this one.
- **#190, a refusal page keeping its input** (the query box with the
  refused text): left. It is a design over every refusal page — which
  refusals carry which form back, and how the CLI's words read on a
  page, as the issue itself says ("a design question for the answer and
  item pages step (#176) or the one after; not a wording round") — and
  this step renders two documents and changes no refusal. Its natural
  home is #182's step, which renders the remaining pages from their
  documents and meets the refusal words on them (`failure_words`, "run
  `acq stashes …`"); the foot raises it for the owner.
- A link where no page takes the request: explain, the corpus context,
  every route (`view.routes`), a crossed table. A new route or fold
  field is the owner's or a design's (C138); the page says the count or
  the words.
- A link on the item page's price line, or a price page (C168: no
  annotation page until its page is designed).
- The save form's dropped sort (#186), the quality box (#180), the
  unsupported-fields switch (#181), the stat finder (#183), stat rows
  and groups (#184), abyssal sockets (#185).
- Any change to the answer's or the item's JSON, the search crate, the
  verbs, the MCP.
- The CLI's text: unchanged; its goldens are its pins.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push. (Binds your work.)
- Every `acq` you run is in a mock world (the mock-session skill):
  `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch; never a
  bare `acq`. Nothing in this step needs a daemon. (Binds your work.)
- A page's word or label this brief leaves open is the step's own
  design, named in the report and in the commit that builds it; a
  label is presentation, the words of a fact the document's (the
  design's §4). (Binds the thing you build.)
- A choice that would add a route, a fold field, a document field or
  change a ruling stops the step and comes to the orchestrating
  session.
- A sentence of this brief the tree disproves is corrected in the
  report, not silently built around.

## Report

`TEMPLATE.md`. Add, under 3, the four goldens' diffs summarised
(lines added and removed per golden), the inventory rows each recorded
page shows as a link and as words, and each masking rule Do 7 needed;
and ask, under 5, for the way test seen failing twice: once with one
route's link pointing at another count's request, once with a way
dropped from the page.

## For the owner's review

Page design decided here, the session's (the design names the rule
and leaves these open):
- A count that carries a route is the link, where it stands (the
  terms table, a bucket, the total's undecided, a clause's adds and
  drops); the CLI's separate "routes" block has no page counterpart.
- The counts whose routes the document leaves out are said as a number
  with no link: no page asks every route.
- A way is marked `class="way"` with `data-way`, the JSON pointer of
  the object holding its request in the document: markup that names no
  value, read by the way test to tie each link to its request.
- The page lists every value a value list's document lists (ten at
  most), not the text's five, and every clause of a site search the
  document lists in the statuses the text lists, not the text's ten:
  the text cuts for a terminal and points at the JSON, which the page
  does not show.
- The item page names no account: the why line, its only use, goes.
- `Paged` links a continuation, `search?next=`, the one way a
  continuation is spelled on a page, refusals included.
- An undecided reason's example and a row's "more" lines link the item
  page, as a row's name does.
- The basis stays at the foot of the answer and item pages in the
  text's words, beside the head's basis line: the words a person
  quotes to an agent.
- The help's first sentence: the pages render the documents the verbs
  answer.

Raised, for the owner:
- **#190 left to #182's step** (Not in scope says why); the owner may
  rule it into this one.
- **Views no page reaches.** A crossed table, an explained node and the
  corpus context render under the rule, and no page URL asks one (the
  fold asks rows or one count; a token is a rows request), so no golden
  holds their HTML. A test that reaches them would need a route or a
  fold field, the owner's (C138).
- **An ambiguous clause's stake may reach no golden.** Row 10 needs a
  form filter the site reads ambiguously and an item its other reading
  moves; where the build cannot stage one without a new seed function,
  the row is held by the way test's rule alone.
- **The design's four lines are eleven.** The trade site box added
  answers whose offers and routes printed commands after the design was
  read; the retirement names the tree's.

## Cuts

Cut 1, 2026-11-08, at `35cf33a7`.

Cut 2, 2026-11-08, under the cold review at `cacfe12e`
(`roadmap/reviews/4-full-1-answer-brief-cacfe12e.md`, sixteen
findings, each taken): 1 the continuation absolute under the token,
the two tests that prefix it named with their lines (Do 3, Input); 2
the counted page's `<table>` check, the edited text's trade check and
the item's account check named, the last deleted with its plumbing
(Input, Do 5, Do 9); 3 the continuation, a routed bucket and the
ambiguous clause recorded, Do 7's list and Do 8's one (Input, Do 7, Do
8); 4 the staged facts in a world and a golden of their own, the cut
list over eleven staged values (Input, Do 7); 5 a count a way only
where it carries a request, the pointer the holder's, the table's
column the pointer (Input, Do 3, Do 4); 6 the counts enumerated once
for both renderers (Do 2); 7 a continuation is a request with a place,
`after` and a basis (Do 3); 8 the flag lines retired and swept, the
grep recursive with `trade_realm` allowed (Do 9, done-when); 9 four
goldens unchanged, `search_form.txt` among them (Input, done-when); 10
"where the answer has one" (Do 7); 11 `no_ids.rs` named as the other
reader (Input); 12 `answer_head`'s and `answer_tail`'s docs (Do 6); 13
the tabs way's realm parsed, its other leagues said, its absence read
from `tabs_pointer()` (Input); 14 every listed value and clause on the
page, a decision for the owner (Do 4, the foot); 15 the no-command
sweep once, over the files, the regenerating run followed by a plain
one (Do 7, Do 8); 16 the close names the issue's remainders (done-when).
