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
the group. Where the design and the tree disagree the brief takes the
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
named `request`, alone or flattened beside a `count` (a `Route`
flattened into a `Count`). The rule (the design's §4): a way on is a
link where the page's `Surface` spells it (`Paged::request` of
`Way::Search(request)`, which is `asked_by_the_form`), and its count
or its words alone where it answers none.

| # | The request | Its field | Its shape | After this step |
| --- | --- | --- | --- | --- |
| 1 | the continuation | `view.rows.next.request` (`RowsOut`, `Next`) | rows, with `basis` and a place | a link, `search?next=<Request::token>`, as today (Do 3) |
| 2 | a term's counts | `terms[].matched`, `.failed`, `.lacked`, `.undecided` (`TermOut`, each a `Count`) | rows | a link on the count |
| 3 | a term's only-together | `terms[].together` (`Together`) | rows | a link on the count |
| 4 | a removed id | `terms[].removed` (`Removed`) | rows, membership `all` | a link (the fold takes `membership` since step 3) |
| 5 | a value list's rest | `terms[].resolved.rest` (`Resolved`) | counts, one key | a link where the form's count field reads the key, else the count of values alone |
| 6 | the root's undecided | `total.undecided` (`Total`) | rows | a link on the count |
| 7 | a count's buckets | `view.counts.tables[].buckets[].count`, `.sources[].count`, `.flags[].count` (`Bucket`, `OfKind`) | rows | a link on the count |
| 8 | a crossed cell and its margins | `view.cross.cells[].count`, `.margins[].none`, `.undecided` (`Cell`, `Margin`) | rows | a link on the count |
| 9 | an explained node, forced | `view.explain.forced_true` and `forced_false`, each `.matched` and `.undecided` (`Forced`) | rows | a link on the count |
| 10 | an ambiguous clause's other reading | `query.trade.clauses[].other.adds`, `.drops` (`Stake`; and under `set_aside` the same) | rows | a link on the count |
| 11 | the zero answer's undecided | `zero.undecided` (`Zero`) | rows | a link on the count |
| 12 | the zero answer's explain offers | `zero.offers.explain[].request` (`ExplainOffer`) | an explained node | the words alone: the part's path and its node (no page asks explain) |
| 13 | the zero answer's corpus offer | `zero.offers.context.request` (`Offer`) | the corpus context | the words alone (no page asks the corpus) |

Beside the document's requests, one way on the page derives as the
CLI's text does: the scope's never-fetched tabs, `Answer::tabs_pointer`
(a function of the answer's JSON, not a field), today printed as
`printed::tabs_command`; after this step `Way::Tabs { account:
Answer::account_named(), realm, league }` through `Paged`, a link to
`refresh?realm=…&league=…` (the design's §4: "the scope block's
never-fetched list links the league's tabs page").

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
- `c16_every_page_s_basis_line_is_its_documents_own` (line 1812):
  `link_saying(&first.body, "more")` finds the continuation by its
  words, `">more</a>"`, and its `href` by the last `href="` before
  them; a way's link keeps `href` its last attribute and the words
  "more" (Do 3).
- The comment on `saved_parts` ("its own tail keeps the CLI's lines
  until step 4") becomes false and is corrected.
- The goldens that regenerate: `search_pages.txt`, `moved_world.txt`,
  `saved_pages.txt` (eight answer pages: its `GET search?<…>` entries).
  `console_realm.txt`, `jobs_halted.txt` and `refresh_loop.txt` hold no
  answer or item page and must not change; their `acq …` lines are the
  refresh and jobs pages', #182's.
No test outside `web.rs` reads a web page (`grep -ln "WebUi\|offline_web" crates/*/tests/*.rs`
names `web.rs` and the harness alone); the JSON every page answers is
unchanged, so `the_fold_answers_as_acq_search_trade_does`,
`trade_answers_as_the_cli` and `the_query_box_sorts_and_pages_a_typed_query`
stand as they are.

**The facts the tests stand on**: `web.rs`'s `seed` as it is (no new
seed function, #142's rule); a world where the scope holds a
never-fetched tab, staged in the test with `Store::record` as
`the_form_renders_the_realm_it_is_asked_for` stages a stash listing; a
zero answer (the xbox URL page has one, with two explain offers and the
corpus offer); a term whose value list is cut (a `class:` or a
`line~` term over enough values, or the build says which seed value
makes one); a removed id (`seed`'s world under membership `live`, a
term `id:` of an item a later listing dropped, staged with
`Store::record` if `seed` holds none); a counted answer (rarity, as
today); an answer with rows left out (`view.limit=1`, as the basis
test asks).

**The last push's CI**: `Gate (Linux)` run 42490674447 and `Gate
(Windows)` run 42490674446, both on `35cf33a7`, both `success`.

## Do

1. **No ruling is needed.** No route, no fold field, no document field,
   no verbs change. Every word and shape the brief leaves open is the
   step's own design (Rules).
2. **One maker per wording; the layouts apart.** Where the HTML says a
   fact the CLI's text says, both call one function that makes the
   words, factored out of `search_cmd.rs` where the words are made
   today inside a line's `format!`; the label column, the padding and
   the line breaks are the text's, the elements the HTML's. A function
   that makes a command line or names a flag (`printed::tabs_command`,
   `printed::show_command`, `printed::opening`, `Next`'s `command()`,
   a route's `command()`, the words `--routes`, `--limit`, `--explain`,
   `--context`) stays the text's alone; where today one function makes
   a fact's words and its command together (`row_shows`'s `more`,
   `answer_more`, the never-fetched line, the zero offers, a table's
   "a larger --limit returns the rest", the price line's why), it is
   split so the text adds the command and the page adds its link or
   its words. Every CLI golden (`tests/goldens/search_json/`,
   `tests/goldens/seat_faults/`, `tests/goldens/journey/`, and every
   other text golden under `tests/goldens/`) is unchanged by the step,
   byte for byte.
3. **The ways on, through `Paged`.** The answer page spells every
   request of the inventory (Input, rows 1 to 13) as
   `Way::Search(request)` through `Paged::request`, and the
   never-fetched tabs as `Way::Tabs` through it; one function in
   `page.rs` (or `mod.rs`) renders a way: where `Paged` answers a link,
   an `<a>` whose words are the count or the label it stands for; where
   it answers none, those words in a `<span>`. Both carry
   `class="way"` and `data-way="<the JSON pointer of the request in the
   document>"` (RFC 6901, `/terms/1/undecided`, `/view/rows/next`; the
   tabs way `data-way="tabs"`), with `href` the `<a>`'s last attribute
   (`link_saying` reads it so). `Paged::request`'s `Way::Search` arm
   takes a continuation: a request with a `basis` is the link
   `search?next=<Request::token>`, before `asked_by_the_form` (which
   answers none for one: its place and basis are no form field); its
   words are "more", as today, and the answer page appends `saved` and
   `saved_revision` to it where it has them, as today. A refusal whose
   offered search carries a basis gains the same link: the build reads
   every place a `Way::Search` is made (`grep -rn "Way::Search" crates/acquisition-verbs/src`
   finds two, one of them over the search crate's offered requests,
   `Offered` in `crates/acquisition-search/src/error.rs`) and says in
   the report whether any carries a basis, and what its link answers
   when followed.
4. **The answer page renders `Answer` itself**, in C115's order, with no
   `pre` of the CLI's text:
   - above the answer, as today: the saved notice, the realm notice,
     the count (`<p class="count">`), the badges;
   - the opening: a site search's translation (its source and fidelity
     sentence, the "over" sentence, then a table of the clauses the
     text lists — not run, ambiguous, reported — each its status, its
     `at` and what it says, an ambiguous clause's other reading with its
     adds and drops as ways (row 10), the first ten of each status and
     the rest counted); the query (the text, or the words for every
     item in scope); the set-aside query where there is one; the scope
     (account, realm, membership and its counts, the fetched and
     never-fetched locations by type, the list ages, the tabs way, the
     realm the store does not hold); the total with its span and its
     undecided way (row 6), the never-fetched sentence, the `before`
     line where there is one;
   - the view: the rows table as today, each row's "N more of term P"
     and "N more sorted" a link to `item/<id>` in place of the `acq
     show` line, and the continuation (row 1); or a count's tables, an
     HTML table per key, each bucket its label, its count as a way (row
     7), its sum, slots and tally, a vocabulary row's term and its kinds
     each with its count as a way, a value's `needs`; the "N shown"
     words without a flag (the page's control is **Show**); or a
     crossed table (row 8) or an explained node (row 9) under the same
     rule;
   - the zero answer: each selector nothing carries with its
     suggestions (a suggestion's term is words, not a way), the explain
     offers (row 12) each its path and node in words and the count of
     the rest, the corpus offer (row 13) in words;
   - the undecided reasons, each its unread part, its example's name as
     a link to `item/<example id>`, its problem and hint, the rest
     counted;
   - the terms: an HTML table, a row per term (its path, its term, its
     matched, failed, lacked and undecided counts and its only-together
     count, each a way where it carries a route, rows 2 and 3), its
     value list beneath it (the values with their counts, the rest a
     way, row 5), its removed note (when it left, and the way, row 4,
     or that it held no item);
   - the counts that carry a request the document leaves out (C115:
     unasked, an answer carries only the routes a next step hangs on):
     their number in words, the CLI's "(5 more: --routes)" without the
     flag, since no page asks every route (the fold has no
     `view.routes`);
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
   the problem, and nothing after: no link, no command, no "why". The
   body is not on the page (the route asks none).
6. **The words that correct.** `page.rs`'s module doc: the pages render
   their documents as HTML, the words made once with the text's, and
   print no command line; `answer`'s and `item`'s docs; `mod.rs`'s
   bullet "A refusal's ways on are the pages' own" gains the answer's
   ways (every request the answer carries spelled through `Paged`, a
   continuation as `search?next=`, the count or the words where no page
   takes one) and keeps its last sentence true (the refresh and jobs
   pages still print the CLI's command lines, #182); `saved_parts`'s
   comment in `web.rs`; `acq web`'s help, "renders what these verbs
   print", becomes "renders the documents these verbs answer" (or the
   step's words for that fact), `CLI-REFERENCE.md` regenerated under
   `ACQ_UPDATE_FIXTURES=1`. README line 203 says nothing this step
   makes false; it stays.
7. **The goldens.** `search_pages.txt`, `moved_world.txt` and
   `saved_pages.txt` regenerate under `ACQ_UPDATE_FIXTURES=1`; each diff
   is read and holds the answer and item pages' new markup and nothing
   else (no form page, no refusal, no saved list changes). Each `#
   holds` fact Input lists is rewritten to name the same fact as the
   HTML says it, and each answer page records at least: the total; the
   query; a term's undecided count as a link; on the xbox page the two
   explain offers and the corpus offer as words with no link; on the
   item page the price line with no "why"; on a page with a
   never-fetched tab, the tabs link. Any value seen to differ between
   two runs is masked by a named rule in `harness/mod.rs` beside
   `basis_rule` (which reads a line that starts `basis   store `, so a
   basis in HTML needs the rule or a line that starts so), each named
   in the report. A new answer page joins `search_pages.txt` for each
   inventory row a page can reach that no recorded page shows today
   (a never-fetched tab, a cut value list with its rest, a removed id),
   recorded in `every_search_page_is_a_wording_golden`.
8. **Pins, what the adapter adds** (C122), in `web.rs` beside the tests
   they join (C153):
   - **Every way, where it stands** (a new test, named for the
     property, say "every request an answer carries is a way on its
     page"): for each answer page Do 7 records (the ring search, the
     edited text, a site URL, the xbox zero, the sorted JSON, the count,
     the never-fetched page, the cut list, the removed id, a page with
     rows left out), the test walks the page's JSON document (same URL,
     JSON asked) for every value under a key `request`, skipping
     `query.trade.clauses[].site`, and finds on the HTML page exactly
     one `class="way"` element whose `data-way` is that value's parent
     pointer, and no `class="way"` element whose pointer names none
     (the tabs way apart, asserted where the page has a never-fetched
     tab and absent where it has none). For each `<a class="way">`,
     followed with JSON asked: it answers `200`; where its pointer
     names a count, the answer's `total.matched` equals that count
     (a route returns exactly the members counted); where it names
     `next`, the answer's rows are the next ones (`view.rows.after`
     equals the first's `returned`); and its `query.text`, `scope.realm`
     and `scope.membership` equal the request's. For each `<span
     class="way">`, `Paged` would answer none: the request is an
     explained node, the corpus, or a counts request the form cannot
     read (asserted by its pointer's kind: `/zero/offers/explain/…`,
     `/zero/offers/context`, or a `rest` whose key the build names).
   - **No command on an answer or item page** (a new test, say "an
     answer or item page prints no command line and no flag"): every
     page headed `GET search` or `GET item/` in every golden under
     `tests/goldens/web/` that answered `200`, read from the files as
     `no_ids.rs` reads them, holds no `acq ` and no ` --` followed by a
     letter; and the same over the pages the way test above visits, as
     served. It holds C143 on these pages; the refresh and jobs pages
     are not read (#182).
   - **One document, two renderers**: already pinned, and named in the
     close: the page's JSON equals the CLI's (`the_fold_answers_as_acq_search_trade_does`,
     `trade_answers_as_the_cli`, `the_query_box_sorts_and_pages_a_typed_query`),
     the page's basis line equals the JSON's basis
     (`c16_every_page_s_basis_line_is_its_documents_own`), the
     document's semantics in the search crate's tests, and the text's
     wording in the CLI's goldens; the HTML's wording in the web
     goldens. No new test compares the two layouts (C122: the
     behaviour shared is the document's, held below the adapter).
   - Every test under C137 (no wall-clock bound).
9. **The gate and the commits.** `tools/gate.sh` green on the step's
   tree; one commit per concern or one for the step, the last naming
   this brief. The commit that changes the goldens carries the
   retirement line in C154's form (C154, a retirement names what now
   holds it): "retired the `acq …` lines and the `why: acq price show
   …` fact on the answer and item pages of `search_pages.txt`,
   `saved_pages.txt` and `moved_world.txt`; held by
   `every_search_page_is_a_wording_golden`,
   `the_saved_pages_are_a_wording_golden` and
   `the_moved_world_renders_with_a_badge` with their goldens
   regenerated, which hold the links in their place, and by the no-command
   test (Do 8)", naming the new test as built. The push and the review
   are the orchestrating session's.

## Done when

- `crates/acquisition-cli/src/web/page.rs` calls none of
  `search_cmd::answer_head`, `answer_tail`, `answer_more`,
  `shown_text`, nothing in `printed.rs`, and no route's or `Next`'s
  `command()`; `grep -n "search_cmd::\|printed::\|command()" crates/acquisition-cli/src/web/`
  shows only word-making functions, each named in the commit message.
- Every text golden under `crates/acquisition-cli/tests/goldens/`
  outside `web/` is unchanged in the step's commits.
- `Paged::request`'s `Way::Search` arm links a continuation as
  `search?next=`; the answer page's every way goes through
  `Paged::request`, and the tabs way is `Way::Tabs`.
- `search_pages.txt`, `saved_pages.txt` and `moved_world.txt` hold no
  `acq ` on any page headed `GET search` or `GET item/`; the other
  three web goldens are unchanged.
- `web.rs` holds the two new tests of Do 8, under the names the
  commit gives them; no new seed function.
- The module docs and comments Do 6 names say what the tree does;
  `CLI-REFERENCE.md` regenerated; `cli_reference.rs` green.
- The retirement line of Do 9 is in a commit message in C154's form.
- Issue #176's "Closes when" is met, and the close says so; the issue
  is closed by the orchestrating session.
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

`TEMPLATE.md`. Add, under 3, the three goldens' diffs summarised
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
  its request in the document: markup that names no value, read by the
  way test to tie each link to its request.
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
- **The design's four lines are eleven.** The trade site box added
  answers whose offers and routes printed commands after the design was
  read; the retirement names the tree's.

## Cuts

Cut 1, 2026-11-08, at `35cf33a7`.
