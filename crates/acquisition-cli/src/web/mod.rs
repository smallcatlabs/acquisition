//! `acq web` (C139, the web UI is a verb of the CLI crate): a loopback
//! web UI that is a renderer, not an app (C16). Every page renders the
//! documents a verb already produces, and the server opens no store,
//! picks no account and compiles no plan itself: it calls the verbs
//! (C46; `tools/arch-check.sh`'s door grep reads this module). It writes
//! one kind, the saved search (C167), and that only through the verbs
//! that write it for every frontend (C168).
//!
//! # As built
//!
//! - **C138, the listener** (verbatim): The web UI's listener binds
//!   127.0.0.1 on a random port, mints a per-launch token that every
//!   link and form action carries, refuses a request whose Host or
//!   Origin is not its own, serves no CORS, and mutates only by POST. Its
//!   routes are private to its page and unversioned, carrying the verbs'
//!   own documents, so they are no third surface (C12, two surfaces).
//!   Here: the token is every route's first path segment, so a request
//!   without it meets no route; the guard compares the Host header with
//!   the bound address and an Origin, where one is sent, with the
//!   server's own, before any route runs; each mutation — apply, the
//!   league list's fetch, a saved search's write and its clear, the
//!   listing of a league's stash tabs or characters — is a POST, the
//!   first four on routes that are POST alone, so a GET on them is refused
//!   by its method, and the listings on the place pages' own routes, whose
//!   GET reads and writes nothing (#284); no
//!   response carries an `Access-Control-*` header. Every response says
//!   `Referrer-Policy: same-origin`: the browser's fetch of an item's art
//!   (C140) carries no referrer, so the token never reaches the image
//!   host, while a form posted to the server itself carries its real
//!   Origin — under `no-referrer` the Fetch standard serialises a POST's
//!   Origin as `null`, which the guard refuses. A content policy admits
//!   images from that host alone and scripts and styles from the server
//!   alone.
//! - **The lifecycle is Ctrl-C**: no lock, no reopen, no idle exit — a
//!   server launched from a terminal is a dev server. The URL is the first
//!   line on stdout (`--json`: `{"url": …}`, as `auth` prints its URL);
//!   `--no-browser` prints it alone.
//! - **The door is the human's** (`Frontend::human("web")`, C88): a
//!   submit from a page reaches the daemon as every CLI verb does, and
//!   its jobs say `web` as their channel (C65).
//! - **One account, named on every call**: the server resolves the
//!   account it serves once (C121), and every read and every submit
//!   names it — a selection's jobs, an apply's — so a login that comes
//!   or goes while the server runs never moves a page's submit to
//!   another account than the one the page shows. Once is at its start, or,
//!   where no account is on disk then, at the first request that finds one:
//!   until then every page is the `not_logged_in` refusal with its remedy,
//!   so a person who ran `acq web` to look at the tool meets the page and
//!   signs in; from then on the account found is held for the server's
//!   life. Only a server started with no account named waits so: one named
//!   (`--account`) is the command's choice, refused before the bind
//!   whatever the refusal — `not_logged_in` over an empty index too, since
//!   a login later could make another account than the one named — as is
//!   any refusal with none named but none (several accounts).
//! - **One corpus, held** (C98, the consumer holds the corpus and checks
//!   currency before every answer): the server holds the last corpus it
//!   loaded for the account it serves in the search crate's `Holder`,
//!   the one `acq-mcp` holds too, which checks `is_current` before every
//!   answer; a stale one, or one of another scope, is loaded again and
//!   swapped. No answer loads per request.
//! - **A page's document as JSON**: a request that accepts
//!   `application/json` is answered with the document the page renders —
//!   the answer, the item, the plan, the jobs — which is how the tests
//!   compare a page with the CLI's `--json` (C11).
//! - **A refusal's ways on are the pages' own**: a refusal is the one
//!   refusal shape spelled by [`Paged`], the page's `Surface`, and the page
//!   renders that document — its words, its readings, then each offer
//!   saying what it is: a read as a link to the route that takes it, a
//!   write as a form posting the fields its route takes with one button in
//!   the offer's words. A search is a link where the page asks it — its
//!   text, or its site search as one word ([`Web::asked_by_the_form`]) —
//!   and those fields fold back to the request offered; a league's plan, or its tabs, a link to
//!   that page; the saved ways — the list, an opening, a write, a clear —
//!   as the C168 section below spells them; any other way on is no page's,
//!   and nothing on a page stands for it. The plan page prints no command
//!   to type next: its Apply button is that way on. An answer's ways on are
//!   spelled the same way: every request the answer carries — a count's
//!   route, a removed id's, a value list's rest, a zero answer's offers —
//!   and the scope's never-fetched tabs go through [`Paged`], a link where
//!   a page takes one and the count or the words where none does; a
//!   continuation, a request with its place, is `search?next=` and its
//!   token, on an answer or a refusal alike. A refusal's words are still
//!   the CLI's (`failure_words`), in the one `pre` a page holds; every
//!   page's own text is its document in a player's words (#182).
//! - **The plan and jobs pages**: `GET plan` renders the `Planned`
//!   document, `POST apply` an apply that sent nothing, and `GET jobs` the
//!   `jobs` document, each in [`page`] and none through the CLI's text;
//!   their JSON is the document unchanged. A submit lands at its job: `POST
//!   apply`, the listings and the league list's fetch answer `303` to
//!   `jobs?job=<id>` at the row's anchor, carrying the `from` the submit
//!   handed on, which the jobs page draws as its way back (the landing
//!   rule, below). `job` names one job as `open` names a parent: read by id,
//!   listed or not, its row marked, one the list leaves out above the list,
//!   and a failed or cancelled one's result under it, its failure said by
//!   its kind and its offers spelled by [`Paged`], which spells a job's
//!   result as that job opened. The daemon's `error` and the way on's words
//!   can name a command and stand in the JSON alone. Only a quote the
//!   daemon refused for want of a session says "not signed in".
//! - **The place pages**: "Stash tabs" (`GET tabs`) and "Characters" (`GET
//!   characters`) render the search crate's places document of the URL's
//!   realm and league. The realm and league selects are a form of their own
//!   and carry no tick: one id can stand in two leagues. The list and its
//!   buttons are one GET form: `open` names each open place, `tab` or
//!   `character` each ticked place, and the server renders both from the
//!   URL (C16). A place's opener sends `toggle`, answered `303` to the page
//!   with `open` changed, every tick kept and no `toggle`, at the parent's
//!   head's anchor, so the reader stays where they were. A tick under a
//!   closed parent travels as a hidden field, counted in words on its head.
//!   An `open` or a tick that names no place of the league is dropped, in
//!   the page and in every URL it writes. Heads are never windowed. A
//!   place's name is its request as a table, spelled by [`Paged`]; a
//!   folder's is words. "Refresh ticked" and "Plan under the policy" send
//!   `refresh` (`ticked`, `policy`), a field of the page itself, which
//!   answers `303` to `plan` with the selection as sent and the page as
//!   sent as its `from`, so the place form never leaves its page; "Plan
//!   under the policy" shows only where the policy names the league (C168:
//!   no page writes a policy). "List this league's stash tabs" (`POST
//!   tabs`) and "List characters" (`POST characters`) submit the `stashes`
//!   and `characters` jobs as `POST leagues` submits `leagues`, each
//!   offered only where its realm's endpoints take the realm
//!   (`Family::accepts`). A league with nothing listed says so in words
//!   where the table would be, beside the listing.
//! - **The landing rule** (`from`, [`from`]): a request that leaves its
//!   page carries that page, and what answers it leads back to the page
//!   holding what was sent. A route is a place (the search page, `tabs`,
//!   `characters`, `saved`, `jobs`, an item's page) or a step (`plan`,
//!   `saved/open`, the clear's ask, every POST): a place's `from` is
//!   itself, and one its URL carries is only its way back, drawn above its
//!   body; a step hands on the `from` it was given. Every route takes
//!   `from` out of its fields first, so no fold, verb or JSON answer meets
//!   it. Every form that posts is made by [`page::post_form`] and every
//!   link to a step carries it ([`from::onward`], [`Paged`]). Landed, a
//!   leaving request answers `303` to the next page carrying `from`, or to
//!   that page as sent where `from` is the page it lands on ([`Web::landed`]);
//!   refused, the refusal in the shared words, then the save form as sent
//!   where the write was a save and the refusal offers no write of its own,
//!   then the way back ([`Web::failed`]).
//! - **The filter**: `name` (words the name holds), `type` (a tab's type)
//!   and `fetched` (`never`) are the places document's filter, read by it
//!   and refused in its words. Its controls stand under the column heads in
//!   the table's form, whose first button, "Show", sends the form to the
//!   page again, so Enter in a box keeps every tick and open and never
//!   plans; the selects' form carries every field but the ticks and the
//!   opens, hidden. A place's fields stand in one order in every link and
//!   redirect ([`place_fields`]), a toggle's 303 among them. A place the
//!   document does not list is not drawn: its ticks travel hidden, and its
//!   level's one fold line after its rows counts it, those never fetched a
//!   link to the page under `fetched=never` alone, every open and tick
//!   kept.
//! - **The grouped table**: `text` is the places document's query,
//!   `view.value`, `view.sort` and `view.desc` the view every place's
//!   request takes, with a query or without, carried hidden in the list's
//!   form; its box stands before "Show". The list is the page's one
//!   `.window`, inside the form: each place's head, fold line and
//!   own-matches line in the flow, and each open place that holds no places
//!   one segment of its items — its route the token of its request's first
//!   window, its count that window's own `total.matched`, its base 0, its
//!   first block the server's — and none where it holds none. Where the URL
//!   opens a place, the document and every segment are read inside one
//!   `places::steady`, which reads both again where the store moved, so the
//!   page stands at one basis. Every segment is asked under one list of
//!   values, the union of what the open places' answers carry, since a
//!   place's default list follows its matches (a currency tab's `stack`):
//!   a place whose answer carries another is asked again naming the
//!   union, and the page's own fields name none of it. The item columns'
//!   heads stand once above the
//!   places where a segment does, a value's head the page ordered by it
//!   through the one maker of a change to the view, so a sort orders the
//!   rows inside each place and the places keep their order. `window.js` is
//!   the table's, unchanged: it sizes the segments while the page is
//!   parsed, before the browser scrolls to a toggle's anchor, so the head
//!   lands in view
//!   (`a_grouped_page_shows_each_open_places_rows_and_keeps_its_place`).
//!   Under a query each head says its matches beside its items, and the
//!   page the league's matches beside the query's own, linking every match
//!   as a table, and where the rest stand (the document's `elsewhere`, each
//!   kind and league a link to its place page under the same query and
//!   view). A rows answer links "by stash tab" and "by character", the
//!   place page of the league its query fixes or the default's, with its
//!   canonical text and view ([`by_place`]), none under all realms, another
//!   membership than live, or a request the form cannot ask; the scope's
//!   tabs link names `fetched=never`.
//! - **C16, what a page holds** (the owner's reading of the ruling's word
//!   "state"): document state is what a page shows, rebuilt from its URL;
//!   transient state — input not yet sent, a finder's asks in flight, the
//!   count beside a search's button and its ask in flight — is discarded by
//!   a reload and labels nothing. A page holds no browser storage, no
//!   cookie, no history entry of its own and no value computed from a
//!   document. Every document a page renders that carries a version — the
//!   answer's and the item's `basis`, the plan's `basis`, the saved list's
//!   `revision`, the file's, and the saved row's `revision`, an answer's
//!   opened row or a conflict's `current` — is copied into one `<meta
//!   name="basis">` line in the head, a JSON object keyed by the document,
//!   its values the documents' own fields ([`page::Head`]): a place page's
//!   under a query its places document's `basis` (`places`), and where a
//!   place is open its segment's window's (`rows`), one basis; a page whose
//!   documents carry none (the form's league list, a place page with no
//!   query and nothing open, the jobs, a refusal without `current`) carries
//!   no line. The scan holds a list of words at the tests' recording seam:
//!   every page a golden records sets no cookie and names only scripts on
//!   the harness's one list (`SCRIPTS`, `tests/harness/mod.rs`), and
//!   `c16_no_served_script_names_a_storage_cookie_or_history_api`
//!   (`tests/web.rs`) fetches each script on it and refuses one that
//!   contains `localStorage`, `sessionStorage`, `indexedDB`, `caches`,
//!   `cookie`, `history`, `serviceWorker` or `.storage`. The property is
//!   that list, lexical: an API reached by another spelling, a cache held
//!   in a closure or the DOM, state the URL omits, a generation a script
//!   keeps for itself is the review's (`REVIEW-CHECKLIST.md`, the
//!   frontends' rows), until a browser-driven check exists.
//! - **C168, the saved pages**: `GET saved` is [`saved::list`]'s document,
//!   ten rows and the whole under `expand=true`, each row with a link that
//!   opens it and its rename form naming the revision listed and a Clear
//!   link. A clear asks first (the owner: "Clear should ask first"): `GET
//!   saved/clear?id=` takes an id alone, writes nothing, and shows the row
//!   with the clear's form at the revision shown and a link back to the
//!   list, its basis keyed `saved_search`; an id no row has is refused,
//!   offering the list, and a row this build cannot read is asked about
//!   like any other, a clear being all a page can do with it. `GET
//!   saved/open?id=` takes an id alone and writes nothing: the row
//!   [`saved::show`] finds is used only where its id is the URL's (a name
//!   may have an id's shape; one rule with the answer page's), and the
//!   answer is `303` to the search its request folds to, carrying `saved`
//!   and `saved_revision`, the row's revision as opened. `POST saved` takes
//!   `id`, `if_revision`, `name`, `text`, `realm` and `membership` into the
//!   verbs' one `Set` under `Expect::or(if_revision, Expect::Create)`, and
//!   `POST saved/clear` `id` and `if_revision`, both required, under
//!   `Expect::Exactly`: a field is present where the body names it, empty
//!   or not, and no page writes at `Expect::Current` (a browser is where a
//!   stale view could clobber). A write answers the browser `303` to the
//!   list, so a reload asks the list again and never posts again (the
//!   owner: "Redirect to the list"); its receipt is the JSON answer, the
//!   undo spelled as the page's form, and no page renders a receipt. A
//!   write way is spelled `{"post", "fields"}` and rendered as a button; a
//!   read way as a link. An answer opened from a saved search says so above
//!   it, and saves over that row at the revision its URL names, so a write
//!   between is refused as stale and offered again over the row as it
//!   stands.
//! - **The count**: every form that submits to `search` carries
//!   `data-count` and, beside its button, the words "Matches:" and an
//!   `<output>`, then a hidden undecided part with its own, found among
//!   the form's elements, so the site box's, owned by its form through
//!   `form=`, stand in the search form's bar. `count.js` hears the page's
//!   `input` and `change` on the document, each for its target's form, or,
//!   for a target with none (the stat pane's notice), the form it stands
//!   in, and asks `GET count` with the fields the form would send, 300 ms
//!   after the last such event that leaves those fields other than the
//!   ones it last asked (the page's own, at its load), a new ask
//!   aborting the one in flight, and writes the answer's two numbers alone,
//!   a dash before and between; a refusal's words are the output's title. A
//!   control no form sends, a finder's box, changes no field and asks
//!   nothing. `count` folds as `search` does and answers the answer
//!   document with no row and no order ([`count`]), so its number is the
//!   one Search would give now. The stat pane's script announces an add or
//!   a remove with a bubbling `change`; neither script names the other.
//! - **Rows and groups** (`roadmap/briefs/4-full-1-form-rows.md`): the stat
//!   pane is lists of items the server rendered, each row by one maker
//!   ([`page::stat_row`]), and `assets/lists.js` copies and removes them
//!   with one click listener on the pane: an add control (`data-add`)
//!   copies the `<template>` it holds, or the one its attribute names, into
//!   the item list (`data-items`) of its nearest list (`data-list`); a
//!   remove control (`data-remove`) removes its nearest item
//!   (`data-item`). The item list that changed announces it with a bubbling
//!   `change`, and an add's copy gives the focus to its element marked
//!   `data-focus`. A group is an item of the pane's list and a list of its
//!   own, ended by its finder (`data-finder`): a box no form sends, which
//!   asks the route its attribute names (`stats?q=`, written by the
//!   server; the script appends the words) and puts the answer, HTML, in
//!   its listbox, its first option marked (`aria-selected`). A finder is
//!   heard wherever it stands: its listeners are the document's. Arrows move the
//!   mark; Enter clicks the marked option and never submits; Escape, and a
//!   click on an option after its add, empty the finder, its box and its
//!   list, and abort its ask in flight. `stats` answers the vocabulary
//!   document as JSON and otherwise the finder's options
//!   ([`page::stat_options`]), each an add control holding its row's
//!   template. What a group's type shows of its bound and its rows'
//!   weights is the stylesheet's, over the type's checked option, which
//!   lists the fields the type takes ([`page::type_takes`]). The script
//!   names no field and writes no word or value a form sends (C46).
//! - **The value finder** (`roadmap/briefs/4-full-1-form-values.md`):
//!   "Add a column" ([`page::value_finder`]) stands inside each form that
//!   carries the page's view, after that form's own button — the search
//!   form and the query box, and a place page's list form beside its item
//!   heads, where they stand — and asks `stats?values=true&q=`, the value
//!   listing (`acquisition_search::trade::vocabulary::values`): every number
//!   field the form renders as a box and every stat of the vocabulary that
//!   the translation compares a value for, by the site's words, JSON where
//!   asked. Each option is a submit button of the finder's form
//!   ([`page::value_options`]) sending `view.by` with the value's text, so
//!   a pick sends that form as it stands with one field more. `view.by` is
//!   the page's: [`search`] and [`places`] take it out of their fields
//!   first ([`picked`]), as the switch is, so no fold, document or link
//!   meets it, and answer `303` to the page ordered by it through the one
//!   maker of a change to the view, over the rows the search as sent
//!   answers (a place page's: those its heads are drawn from,
//!   [`page::heads_rows`]), to the URL that value's head writes. Beside a
//!   count it is let go, the `303` the page less it; past a full list the
//!   change names the longer list, so the page lands on the search's own
//!   refusal of it ([`view_picked`]); a search refused as sent is that
//!   refusal; a place page that draws no heads refuses it in words.
//! - **The table**: an answer asked as a table (`view.fields=icon`) is
//!   every matching row in one scrolling page at one row height, the height
//!   and the block's rows written on the table ([`page::ROW_HEIGHT`],
//!   [`page::BLOCK_ROWS`]). Its line says how many items it holds and in
//!   what order; `view.limit` is the cards' and is not read for it. One
//!   maker gives a window's rows ([`Web::window`], [`window_request`]): the
//!   page's request at its answer's basis, its place the window's offset
//!   into the whole order and its limit the window's count, `item` among
//!   its fields for the price. The page answers its request and its first
//!   block over one corpus, so both stand at one basis, and renders that
//!   block, which `window.js` takes as one it holds; the script asks the
//!   rest as they near the viewport. `GET
//!   rows?next=<token>&offset=<n>&count=<n>` answers a window, the token
//!   the first window's request: the rows alone as HTML, the window
//!   request's answer document as JSON ([`rows`]). It refuses with 400, in
//!   the one refusal shape, a token no answer printed, an offset or a count
//!   that is no whole number, a count of none or past the item rows' bound,
//!   and a window that runs past the rows; a basis moved since the page is
//!   the search's refusal with 409, and the script loads the page again. A
//!   table reached by a continuation and loaded again after its basis moved
//!   meets the continuation's own refusal of a moved basis, as a cards page
//!   does.
//! - **The view's links**: nothing is typed to sort. The URL names the
//!   rows' values by `view.value`, one each in order as the CLI's
//!   `--value`, beside `view.sort` and `view.desc`; no control of a form
//!   types the three ([`form::UNTYPED`]): both editors carry what the URL
//!   named of them hidden, the query box a count's keys too
//!   (`page::view_hidden`, the one renderer), so an edited search keeps
//!   them; a search sent with a count lets go of them, a count showing no
//!   rows (`form.rs`, the fold; #304). One
//!   maker gives a change to the view ([`view_link`]): the same page
//!   ordered by one value, largest first or the other way where the rows
//!   are ordered by it, the value joining the answer's list where the list
//!   lacks it and no link where the list is full; a value picked by name
//!   (the value finder, below) lands on that same link. Each value's head of the
//!   table is that link, the ordered one marked by an arrow and by words a
//!   screen reader gets. So is each number of an answer's card that a value
//!   reads (`page::card`): a line whole by its `value`, the value a trade
//!   filter on its stat compares, so a stat the form filtered lands on its
//!   own column; a displayed row's number by the field it shows, the row in
//!   the search crate's pieces (`acquisition_search::show::Displays`),
//!   never read again from its text. A number whose value the view holds is
//!   marked `valued`, the sort's as its head is; the item page has no view,
//!   and its numbers are words.
//! - **An item under its row**: `item` names an opened row by its item's
//!   id, each once, as `open` names a place. `search`, `tabs` and
//!   `characters` take it off before the fold ([`opening`]), with `at` and
//!   `toggle_item`. The rows a page keeps are those the search crate places
//!   in its order (`acquisition_search::indices`, the one maker of the
//!   order): the table's request on `search`, each drawn open place's on a
//!   place page; one no order holds is dropped, in the page and every URL
//!   it writes, and a page of cards keeps none (cards cut nothing). A
//!   change to the view keeps each kept row; "as cards", the query box, the
//!   save form, the selects' form and the ways to the place pages carry
//!   none. **The cut** ([`cut`], one maker, and `page::segments`, one
//!   renderer, for the flat table and each open place): a segment ends at
//!   an opened row, the row's card stands in the page's flow after it, and
//!   a segment of the same route goes on from the next row, its base that
//!   row's index; the server renders each segment's first block, the block
//!   holding its opened row and the block holding `at`, every one over the
//!   page's one corpus, so one basis. No row is marked open: its card is
//!   the mark, with its "Close". **The opener** is a toggle, as a place's
//!   is: a row's name is a button sending `toggle_item` with its id inside
//!   the page's one GET form — the place pages' list form, which carries
//!   the kept rows hidden, and on `search` a form around the table holding
//!   the page's fields hidden — answered `303` to the page with the id
//!   added, or taken out where open, and `at`, at the row's anchor
//!   (`page::row_anchor`). `at` is one load's landing, never state: no form
//!   writes it and no link carries a page's own on, and one no order holds
//!   is ignored. `window.js` seeks the URL's anchor (a row, or the answer
//!   a search lands at) once its segments are sized and before its first
//!   fill, so the fill keeps the block the server rendered for it; a row's anchor stands below the
//!   sticky heads by its scroll margin. **The card** is `page::card` of the
//!   row's `item`, which every window asks, with the page's view links, so
//!   its numbers add columns. **Its place**, on every card and on the item
//!   page, is a link ([`Web::place_links`]): the place page of its realm
//!   and league with the place and each holding it open
//!   (`Places::holders`), no query, landing at the card's own row (`at`,
//!   the one link that writes one, for another page's one load), or at the
//!   place's head for a removed item, which its live rows lack; a place no
//!   places document lists stands as words. **Its query**: an opened row's
//!   card links `item/<id>?against=` the page's canonical text, where it
//!   has one; [`item`] takes `against` as `acq show --against` does,
//!   through `show::against`, and its JSON is that document. The way back
//!   is the row the card stands under.
//! - **One page** (`roadmap/briefs/4-full-1-form-design.md` §2): `.` and
//!   `search` render the editors ([`page::editor`]) — what the form's
//!   envelope holds and no control sends, named; the search form, the site
//!   box in its bar after the item search, its own form's by `form=`
//!   (`4-full-1-form-rides.md`, Decision 5); the query box — then the answer or the refusal at the anchor
//!   `answer`, which every form that submits to `search` names in its
//!   action. The source says which editor is filled ([`Web::editors`]): a
//!   site search, the form's own envelope among them, is decoded and fills
//!   the form (`fill.rs`), the query box holding the answer's canonical
//!   text, closed; a text fills the query box, open, over an empty form; a
//!   continuation is read by its token's request, a site search first where
//!   a text stands beside it. Both editors carry one `page::BoxView` — the
//!   order and values the URL named, hidden, the table, a count's keys,
//!   the rows' or the count's number — made from an answer's rows or a
//!   request's count, and the saved search the page was opened from; a line by the
//!   save form names each part of the view a save leaves behind, a saved
//!   search being its query and its scope (C167). So a saved form search
//!   and a saved site search open with the form filled. A refused search
//!   (#190) is the same page, its status and its JSON the refusal's, its
//!   editors filled from the nearest thing read: the envelope the fold or
//!   the decode gave, the site box holding a search the decode refused,
//!   the query box a text and the view as sent, the form empty where the
//!   fold refused.
//! - **What rides on the form** (`roadmap/briefs/4-full-1-form-rides.md`):
//!   the armour panel's quality box, "assume 20% quality", ticked by
//!   default (C142, the defences' reading beside the query); a fate said
//!   only where it is not exact; the site box in the bar; Clear beside
//!   Search, a link to `.` with the scope alone, which [`home`] reads; and
//!   at the page's foot, after the answer, the switch ([`page::switch`]),
//!   tied to the search form by `form=`, with the line that says what a
//!   mark means. The server marks `data-unrun` a control, a select's
//!   option, a group type's option, its "Add Stat Group" control and a
//!   panel all of whose controls do not run (a fate neither exact nor
//!   ambiguous); the stylesheet hides a marked one while the switch is
//!   unticked, unless among its controls a text or number box is not
//!   showing its placeholder or a select stands off its first option, and
//!   a marked option unless it is chosen. No script hides anything. The
//!   switch is the page's, marked `data-page`: [`home`], [`search`] and
//!   [`count`] take `all_fields` out of the fields first, so no fold,
//!   link or hidden field meets it, and `count.js` leaves it out of what
//!   it asks. Nothing remembers it (#181).

mod fill;
mod form;
mod from;
mod page;

use std::sync::{Arc, OnceLock};

use acquisition_plan::RefreshPlan;
use acquisition_plan::pricing::Expect;
use acquisition_protocol::protocol::ListState;
use acquisition_protocol::realm::Realm;
use acquisition_search::answer::{Asked, Request};
use acquisition_search::places::{Kind, Places};
use acquisition_search::{Holder, SearchError};
use acquisition_store::SavedQuery;
use acquisition_verbs::refusal::{Pointed, Way};
use acquisition_verbs::saved::{self, SavedRow};

use crate::saved_cmd::trade_word;
use acquisition_verbs::{Account, Apply, Fault, Frontend, Jobs};

use crate::terminal::plural;
use anyhow::Result;
use axum::Router;
use axum::extract::{Form, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde_json::{Value, json};

use crate::run::Ctx;
use crate::store_cmd;
use from::{FromPage, Here, Place};
use page::{Head, Opened, OpenedRow, SavedLinks, enc, esc, refusal};

/// The channel every write and submit from a page carries (C65).
const VIA: &str = "web";

/// The stylesheet, the stat pane's script, the count's and the table's
/// window, served as written.
const CSS: &str = include_str!("assets/page.css");
const LISTS: &str = include_str!("assets/lists.js");
const COUNT: &str = include_str!("assets/count.js");
const WINDOW: &str = include_str!("assets/window.js");

/// How many stats a finder lists for one ask; the rest are counted.
const PICKED: usize = 50;

/// How often the jobs page reloads itself while a job is not done.
const JOBS_REFRESH: u32 = 3;

/// The content policy every response carries: the server's own scripts
/// and styles, images from GGG's CDN alone (C140), no frame, no other
/// form target.
const POLICY: &str = "default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src https://web.poecdn.com; form-action 'self'; frame-ancestors 'none'; base-uri 'none'";

/// What the server holds for its whole life.
struct Web {
    cx: Ctx,
    /// The account served: set at the start, or at the first request
    /// that finds one, and never moved after (the module doc).
    account: OnceLock<Account>,
    token: String,
    host: String,
    origin: String,
    form: form::Form,
    held: Holder,
}

/// Why a page could not render its document: a search refusal in the
/// search's words, anything else in the CLI's.
enum Failure {
    Search(SearchError),
    /// A request the pages never make: a usage error (C11's `usage`).
    Usage(String),
    Other(anyhow::Error),
}

impl From<SearchError> for Failure {
    fn from(e: SearchError) -> Failure {
        Failure::Search(e)
    }
}

impl From<acquisition_search::LanguageError> for Failure {
    fn from(e: acquisition_search::LanguageError) -> Failure {
        Failure::Search(e.into())
    }
}

impl From<Fault> for Failure {
    fn from(e: Fault) -> Failure {
        Failure::Other(e.into())
    }
}

impl From<anyhow::Error> for Failure {
    fn from(e: anyhow::Error) -> Failure {
        Failure::Other(e)
    }
}

/// `acq web`: bind, mint, print, open, serve until Ctrl-C.
pub(crate) async fn web(account: Option<String>, no_browser: bool, json: bool) -> Result<()> {
    let cx = Ctx {
        frontend: Frontend::human(VIA)?,
        account,
    };
    // no account on disk with none named is a page, every other
    // refusal the command's
    let served = match cx.account() {
        Ok(account) => OnceLock::from(account),
        Err(Fault::NoAccounts { .. }) if cx.account.is_none() => OnceLock::new(),
        Err(fault) => return Err(fault.into()),
    };
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0)).await?;
    let port = listener.local_addr()?.port();
    let token = mint()?;
    let url = format!("http://127.0.0.1:{port}/{token}/");
    let web = Arc::new(Web {
        cx,
        account: served,
        host: format!("127.0.0.1:{port}"),
        origin: format!("http://127.0.0.1:{port}"),
        token: token.clone(),
        form: form::Form::load()?,
        held: Holder::new(),
    });
    if json {
        println!("{}", json!({ "url": url }));
    } else {
        println!("{url}");
    }
    use std::io::Write as _;
    std::io::stdout().flush().ok();
    // as `auth` does: `--json` changes what is printed, never whether the
    // browser opens; stdout under `--json` is the one document
    if !no_browser {
        let opened = crate::auth_cmd::open_browser(&url);
        if !json {
            println!(
                "({} Ctrl-C stops the server)",
                if opened {
                    "opened in your browser;"
                } else {
                    "open it in your browser;"
                }
            );
            std::io::stdout().flush().ok();
        }
    }
    let at = |path: &str| format!("/{token}{path}");
    let app = Router::new()
        .route(&at("/"), get(home))
        .route(&at("/search"), get(search))
        .route(&at("/item/{id}"), get(item))
        .route(&at("/stats"), get(stats))
        .route(&at("/count"), get(count))
        .route(&at("/rows"), get(rows))
        .route(&at("/tabs"), get(tabs).post(list_tabs))
        .route(&at("/characters"), get(characters).post(list_characters))
        .route(&at("/plan"), get(plan))
        .route(&at("/apply"), post(apply))
        .route(&at("/leagues"), post(fetch_leagues))
        .route(&at("/jobs"), get(jobs))
        .route(&at("/saved"), get(saved_list).post(saved_set))
        .route(&at("/saved/open"), get(saved_open))
        .route(&at("/saved/clear"), get(saved_ask).post(saved_clear))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&web),
            with_account,
        ))
        .route(&at("/assets/page.css"), get(css))
        .route(&at("/assets/lists.js"), get(lists_script))
        .route(&at("/assets/count.js"), get(count_script))
        .route(&at("/assets/window.js"), get(window_script))
        .fallback(no_such_page)
        .layer(middleware::from_fn_with_state(Arc::clone(&web), guard))
        .with_state(web);
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

/// A per-launch token: 128 random bits, as hex.
fn mint() -> Result<String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| anyhow::anyhow!("no randomness for the token: {e}"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// C138, before any route: the Host must be the bound address and an
/// Origin, where one is sent, the server's own. Every response then
/// carries the referrer and content policies, and no CORS header.
async fn guard(
    State(web): State<Arc<Web>>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    // read in a block of its own: a borrow of the request (whose body is
    // not `Sync`) held across the await would make the guard's future
    // not `Send`
    let (host, origin) = {
        let said = |name: header::HeaderName| {
            request
                .headers()
                .get(name)
                .map(|v| v.to_str().unwrap_or("\u{fffd}").to_string())
        };
        (said(header::HOST), said(header::ORIGIN))
    };
    let mut response = if host.as_deref() != Some(web.host.as_str()) {
        refused(
            StatusCode::FORBIDDEN,
            &format!(
                "refused: this server answers only at {}, the address it printed",
                web.origin
            ),
        )
    } else if origin.is_some_and(|o| o != web.origin) {
        refused(
            StatusCode::FORBIDDEN,
            "refused: a page of another site sent this request",
        )
    } else {
        next.run(request).await
    };
    let headers = response.headers_mut();
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(POLICY),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

/// Every page but the assets, after the guard: the account served,
/// resolved here if the server has none yet; while none is found the
/// page is its refusal (the module doc, "One account").
async fn with_account(
    State(web): State<Arc<Web>>,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    match blocking(&web, |web| web.served().map(|_| ())).await {
        Ok(()) => next.run(request).await,
        Err(fault) => web.failed(
            "Account",
            &Failure::from(fault),
            request.headers(),
            &Here::none(),
        ),
    }
}

fn refused(status: StatusCode, words: &str) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        format!("{words}\n"),
    )
        .into_response()
}

/// A path no route holds — a request without the token among them.
async fn no_such_page() -> Response {
    refused(StatusCode::NOT_FOUND, "no such page")
}

fn wants_json(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|a| a.contains("application/json"))
}

fn html(status: StatusCode, body: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        body,
    )
        .into_response()
}

fn document(value: &impl serde::Serialize) -> Response {
    match serde_json::to_string_pretty(value) {
        Ok(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response(),
        Err(e) => refused(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

impl Web {
    /// The account this server serves: the one it holds, else the one
    /// the index resolves now, held from then on — a request racing this
    /// one to it serves whichever was held first.
    fn served(&self) -> Result<&Account, Fault> {
        if let Some(account) = self.account.get() {
            return Ok(account);
        }
        let found = self.cx.account()?;
        Ok(self.account.get_or_init(|| found))
    }

    /// The name of the account this server serves, none before one is
    /// found: what every page's nav bar names.
    fn account_name(&self) -> Option<&str> {
        self.account.get().map(|a| a.entry.username.as_str())
    }

    fn page(&self, title: &str, head: Head, body: &str) -> Response {
        html(
            StatusCode::OK,
            page::page(&self.token, title, self.account_name(), head, body),
        )
    }

    /// A refusal's document, in the one refusal shape as a page spells it
    /// ([`Paged`]), its links to a step carrying `from` where the page
    /// renders it (none in a JSON answer: `from` is the page's).
    fn refusal(&self, failure: &Failure, from: Option<&FromPage>) -> Value {
        let paged = Paged { web: self, from };
        match failure {
            Failure::Usage(words) => json!({ "error": words, "kind": "usage" }),
            Failure::Search(e) => acquisition_verbs::refusal::of_search(e, &paged),
            Failure::Other(e) => acquisition_verbs::refusal::document(e, &paged),
        }
    }

    /// The way back `here` draws, where it has one.
    fn back(&self, here: &Here) -> Option<page::Back> {
        here.back.as_ref().map(|back| page::Back {
            href: back.href(self),
            words: back.place().words(),
        })
    }

    /// A page drawn with `here`'s way back.
    fn page_at(&self, title: &str, head: Head, body: &str, here: &Here) -> Response {
        let head = Head {
            back: self.back(here),
            ..head
        };
        self.page(title, head, body)
    }

    /// A leaving request landed (the landing rule): `303` to `route` with
    /// `fields` at `anchor`, carrying the `from` the request handed on —
    /// or, where that `from` is the very place `route` names with no field
    /// of its own, to that page as sent, its own way back left out.
    fn landed(
        &self,
        route: &str,
        fields: &[(&str, String)],
        anchor: Option<String>,
        here: &Here,
    ) -> Response {
        let to = match here.onward() {
            Some(from) if fields.is_empty() && from.is_route(route) => from.href(self),
            from => from::onward(&self.link(route, fields), from),
        };
        match anchor {
            Some(anchor) => see_other(&format!("{to}#{anchor}")),
            None => see_other(&to),
        }
    }

    /// A refusal as a page, or as its document where JSON is asked (C11):
    /// the page renders the document (C16).
    /// Its way back is `here`'s, and its offers post `here`'s `from`: a
    /// refused leaving request's refusal, then that way back (the landing
    /// rule).
    fn failed(&self, title: &str, failure: &Failure, headers: &HeaderMap, here: &Here) -> Response {
        self.failed_as_sent(title, failure, headers, here, "")
    }

    /// [`Web::failed`], with `sent` after the refusal: the form as sent,
    /// where the route has one a reader fills.
    fn failed_as_sent(
        &self,
        title: &str,
        failure: &Failure,
        headers: &HeaderMap,
        here: &Here,
        sent: &str,
    ) -> Response {
        if wants_json(headers) {
            return self.refused_document(failure);
        }
        let doc = self.refusal(failure, here.onward());
        // a conflict's page renders the row as it stands: its revision is
        // the page's basis
        let head = match page::saved_current(&doc) {
            Some(row) => Head::default().with("saved_search", &row["revision"]),
            None => Head::default(),
        };
        let head = Head {
            back: self.back(here),
            ..head
        };
        html(
            StatusCode::BAD_REQUEST,
            page::page(
                &self.token,
                title,
                self.account_name(),
                head,
                &format!("{}{sent}", page::refused(&doc, here.onward())),
            ),
        )
    }

    /// A refused search: its editors as `editors` renders them, filled from
    /// what was sent, then the refusal at the answer's anchor, then `foot`,
    /// the page's switch; JSON and status the refusal's.
    fn failed_under(
        &self,
        title: &str,
        failure: &Failure,
        headers: &HeaderMap,
        editors: &str,
        foot: &str,
        here: &Here,
    ) -> Response {
        if wants_json(headers) {
            return self.refused_document(failure);
        }
        let doc = self.refusal(failure, here.onward());
        html(
            StatusCode::BAD_REQUEST,
            page::page(
                &self.token,
                title,
                self.account_name(),
                Head {
                    lists: true,
                    count: true,
                    back: self.back(here),
                    ..Head::default()
                },
                &format!(
                    "{editors}<div id=\"{}\">\n{}</div>\n{foot}",
                    page::ANSWER,
                    page::refused(&doc, here.onward())
                ),
            ),
        )
    }

    /// The editors a search page renders above its answer or its refusal,
    /// filled from its source (the one page, `4-full-1-form-design.md` §2):
    /// a site search, the form's envelope among them, decoded, fills the
    /// form, the query box holding the answer's canonical text, closed; one
    /// whose decode refused stands in the site box as sent; a text, or a
    /// tree, fills the query box, open, over an empty form — the answer's
    /// canonical text, or the text as sent where it was refused; and where
    /// nothing was read, the fold having refused, the form is empty and the
    /// query box holds a text the URL names. The scope is the answer's, else
    /// the reading's, else the URL's; the view is the answer's rows', else
    /// the URL's (`page::BoxView`); the saved search is the URL's.
    fn editors(
        &self,
        sent: &Fields,
        source: Option<&Source>,
        answer: Option<&acquisition_search::Answer>,
    ) -> page::Filled {
        use acquisition_search::answer::ViewOut;
        let named = |name: &str| pick(sent, name).map(str::to_string);
        let mut filled = page::Filled::empty(Realm::DEFAULT.as_str());
        filled.saved = pick(sent, "saved")
            .zip(pick(sent, "saved_revision").and_then(|at| at.parse::<i64>().ok()))
            .map(|(id, at)| (id.to_string(), at));
        filled.view = match (
            answer.map(|a| &a.view),
            source.and_then(|s| s.counted.clone()),
        ) {
            (Some(ViewOut::Rows(rows)), _) => page::BoxView::of(rows),
            (_, Some((keys, limit))) => page::BoxView::counted(keys, limit),
            _ => page::BoxView::sent(sent),
        };
        filled.membership = answer
            .map(|a| a.scope.membership.as_str().to_string())
            .or_else(|| source.and_then(|s| s.membership.clone()))
            .or_else(|| named("membership"));
        let mut realm = answer
            .map(|a| a.scope.realm.as_str().to_string())
            .or_else(|| source.and_then(|s| s.realm.clone()))
            .or_else(|| named("realm"));
        let canonical = answer.map(|a| a.query.text.clone());
        match source.map(|s| &s.query) {
            Some(
                query @ acquisition_search::answer::QueryInput {
                    trade: Some(trade), ..
                },
            ) => {
                let decoded = match trade {
                    Value::String(word) => acquisition_search::trade::decode(word),
                    value => acquisition_search::trade::decode_json(value),
                };
                match decoded {
                    Ok(decoded) => {
                        // a URL's realm is the search's (#188)
                        realm = decoded.realm.clone().or(realm);
                        filled.fill = fill::Fill::decoded(&decoded);
                        filled.text = canonical.unwrap_or_default();
                        filled.text_open = false;
                    }
                    Err(_) => filled.trade = query.trade_word().unwrap_or_default(),
                }
            }
            Some(query) => {
                filled.text = canonical.or_else(|| query.text.clone()).unwrap_or_default();
            }
            None => filled.text = named("text").unwrap_or_default(),
        }
        if let Some(realm) = realm {
            filled.realm = realm;
        }
        filled
    }

    /// A refusal as its document, whatever was asked: what a page's
    /// refusal answers where JSON is asked, and the count's every refusal.
    fn refused_document(&self, failure: &Failure) -> Response {
        (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            self.refusal(failure, None).to_string(),
        )
            .into_response()
    }

    /// Whether `account` names the account this server serves, by its
    /// name or its uuid (a store's read names it by the uuid).
    fn serves(&self, account: &str) -> bool {
        self.account.get().is_some_and(|served| {
            served.entry.username == account || served.entry.uuid.as_deref() == Some(account)
        })
    }

    /// `path` on this server with `fields` as its query: a link a page
    /// prints, the token first as every link's is (C138).
    fn link(&self, path: &str, fields: &[(&str, String)]) -> String {
        let query: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
            .collect();
        match query.is_empty() {
            true => format!("/{}/{path}", self.token),
            false => format!("/{}/{path}?{}", self.token, query.join("&")),
        }
    }

    /// A link a page writes, relative to the pages' token or absolute
    /// under it, as an absolute path under the token: what a `303` names.
    fn under_token(&self, href: &str) -> String {
        match href.starts_with('/') {
            true => href.to_string(),
            false => format!("/{}/{href}", self.token),
        }
    }

    /// The saved pages' links and forms, absolute under the token.
    fn saved_links(&self) -> SavedLinks {
        SavedLinks {
            list: self.link("saved", &[]),
            every: self.link("saved", &[("expand", "true".to_string())]),
            write: self.link("saved", &[]),
            clear: self.link("saved/clear", &[]),
        }
    }

    /// The saved search at exactly `id`, as [`saved::show`] finds it, or
    /// none where no row stands at that id: `show` reads an id or a name,
    /// and a name may have an id's shape, so a row found by its name under
    /// another id — or several rows of that name — is no row at `id`
    /// (one rule for every page that resolves a URL's id). Any other
    /// failure is the page's refusal.
    fn saved_at(&self, id: &str) -> Result<Option<SavedRow>, Failure> {
        match saved::show(&self.cx.frontend, id) {
            Ok(row) if row.id() == id => Ok(Some(row)),
            Ok(_) => Ok(None),
            Err(e) => match e.chain().find_map(|c| c.downcast_ref::<Pointed>()) {
                Some(p)
                    if matches!(p.kind, "saved_search_not_found" | "ambiguous_saved_search") =>
                {
                    Ok(None)
                }
                _ => Err(Failure::Other(e)),
            },
        }
    }

    /// A search request as a page's fields — its realm, its query's one
    /// editor, and every view field the form asks: its rows' limit, order
    /// and values named, or its count's keys and limit — kept only where the
    /// fold reads them back to that request, as the account this server
    /// serves: what a page can ask. The query is its site search as one word
    /// (`trade`), which the page fills the form from (the one page,
    /// `4-full-1-form-design.md` §2), a text beside it being the translation
    /// the read verified; else its text. A URL's realm is the search's, so
    /// none is carried beside one that names it. Its rows' fields are
    /// `view.fields=icon` where they are the art alone, the table, and no
    /// field where they are the art and the item or none, the page's
    /// default, cards; any other list is no form's. Its membership is a
    /// field only where it is not `live`, the form's default, so every link
    /// made before the form carried one stands as it was. None for any other
    /// (a tree, another account or view: a crossed table, a sum, an explained
    /// node): no page takes it (else a count is dropped).
    fn asked_by_the_form(&self, request: &Request) -> Option<Vec<(&'static str, String)>> {
        if !request
            .scope
            .account
            .as_deref()
            .is_none_or(|a| self.serves(a))
        {
            return None;
        }
        // the query's one editor: its site search as one word, which the
        // page fills the form from, a text beside it being its translation
        // (the read refuses any other); or its text. A tree is no box's
        let query = &request.query;
        let asked = match (&query.text, query.trade_word(), &query.tree) {
            (_, Some(word), None) => ("trade", word),
            (Some(text), None, None) => ("text", text.clone()),
            _ => return None,
        };
        let mut fields = Vec::new();
        // a URL's realm is the search's: the page carries none beside it
        if let Some(realm) = &request.scope.realm
            && (asked.0 == "text" || crate::search_cmd::trade_realm(&asked.1).is_none())
        {
            fields.push(("realm", realm.as_str().to_string()));
        }
        if request.scope.membership != acquisition_search::answer::Membership::Live {
            fields.push(("membership", request.scope.membership.as_str().to_string()));
        }
        fields.push(asked);
        match &request.view.shape {
            acquisition_search::answer::Shape::Rows(rows) => {
                if let Some(limit) = rows.limit {
                    fields.push(("view.limit", limit.to_string()));
                }
                if let Some(sort) = &rows.sort {
                    fields.push(("view.sort", sort.clone()));
                }
                if rows.desc {
                    fields.push(("view.desc", "true".to_string()));
                }
                // the values named, one field each in their order; the
                // query's own are the request's to fill again, so unnamed
                for value in &rows.values {
                    fields.push(("view.value", value.clone()));
                }
                // the row's fields: the table is `view.fields=icon`; the
                // cards, and none (a route carries none), the page's
                // default; any other list is no form's
                use acquisition_search::answer::RowField;
                match rows.fields.as_slice() {
                    [RowField::Icon] => fields.push(("view.fields", "icon".to_string())),
                    [] | [RowField::Icon, RowField::Item] => {}
                    _ => return None,
                }
            }
            // the one field the form reads a count from: each list as
            // `--count` takes it, joined, since the field's list reads a
            // narrowing's own `line:` after another's; an order it cannot
            // read (a field after a narrowing) folds back to another count
            // and is offered nowhere
            acquisition_search::answer::Shape::Counts(counts) => {
                fields.push(("view.count", counts.lists().join(",")));
                if let Some(limit) = counts.limit {
                    fields.push(("view.limit", limit.to_string()));
                }
            }
            _ => {}
        }
        let pairs: Vec<(String, String)> = fields
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        let folded = form::fold(&self.form, &pairs).ok()?;
        let asked = Asked::read(&folded.request().ok()?).ok()?.request;
        // the page's rows carry the fields the form asks, and its account
        // is the one served; a count's keys are compared as the key's one reader
        // reads them, never by their letters (`Rarity` is `rarity`)
        let mut offered = request.clone();
        offered.scope.account = None;
        if let (
            acquisition_search::answer::Shape::Rows(rows),
            acquisition_search::answer::Shape::Rows(own),
        ) = (&mut offered.view.shape, &asked.view.shape)
        {
            rows.fields = own.fields.clone();
        }
        (as_asked(read_keys(asked)) == as_asked(read_keys(offered))).then_some(fields)
    }

    /// The request answered over the held corpus (C98): checked first,
    /// then the holder's corpus, current and of the request's scope.
    fn answer(&self, request: &Request) -> Result<acquisition_search::Answer, Failure> {
        let corpus = self.corpus(request)?;
        Ok(acquisition_search::answer(&corpus, request)?)
    }

    /// The holder's corpus for `request`, checked first: current, and of
    /// the request's scope. A page that answers two requests of one scope
    /// answers both over the one corpus this returns, at one basis.
    fn corpus(&self, request: &Request) -> Result<Arc<acquisition_search::Corpus>, Failure> {
        request.check()?;
        let opened = self.served()?.open()?;
        Ok(self.held.corpus(request, &opened.facts, &opened.intent)?)
    }

    /// A window of a table's rows (the table's design, `rows`): the page's
    /// request at its answer's `basis`, from row `offset` of its order,
    /// `count` of them, each carrying its item for its price — the one
    /// maker the `rows` route and the page's first block call, over the
    /// corpus each takes ([`window_request`]).
    fn window(
        &self,
        corpus: &acquisition_search::Corpus,
        page: &Request,
        basis: &acquisition_search::Basis,
        offset: usize,
        count: usize,
    ) -> Result<acquisition_search::Answer, Failure> {
        Ok(acquisition_search::answer(
            corpus,
            &window_request(page, basis, offset, count),
        )?)
    }

    /// Each place's link as a card shows it: the
    /// place page of its realm and league, its kind's, with the place and
    /// each place holding it open ([`Places::holders`]), no query, and no
    /// landing, which the card adds for its own item ([`page::card`]);
    /// none where that league's places document lacks it. Each realm,
    /// league and kind's document is read once.
    fn place_links(
        &self,
        shown: &[&acquisition_search::corpus::Place],
    ) -> Result<std::collections::HashMap<PlaceKey, String>, Failure> {
        let mut links = std::collections::HashMap::new();
        if shown.is_empty() {
            return Ok(links);
        }
        let opened = self.served()?.open()?;
        let at = acquisition_search::places::Reading {
            store: &opened.facts,
            intent: &opened.intent,
            holder: &self.held,
        };
        let now = acquisition_store::now();
        let mut read: std::collections::HashMap<(Listed, Realm, &str), Places> =
            std::collections::HashMap::new();
        for place in shown {
            let listed = match place.kind.as_str() {
                "stash" => Listed::Tabs,
                "character" => Listed::Characters,
                _ => continue,
            };
            let (Some(realm), Some(league)) = (Realm::parse(&place.realm), place.league.as_deref())
            else {
                continue;
            };
            let places = match read.entry((listed, realm, league)) {
                std::collections::hash_map::Entry::Occupied(held) => held.into_mut(),
                std::collections::hash_map::Entry::Vacant(slot) => {
                    let asking = acquisition_search::places::Asking::default();
                    slot.insert(match listed {
                        Listed::Tabs => Places::tabs(at, realm.as_str(), league, &asking, now)?,
                        Listed::Characters => {
                            Places::characters(at, realm.as_str(), league, &asking, now)?
                        }
                    })
                }
            };
            let Some(holders) = places.holders(&place.id) else {
                continue;
            };
            let open: Vec<String> = holders.into_iter().map(str::to_string).collect();
            let href = self.link(
                listed.route(),
                &place_fields(
                    listed,
                    realm,
                    league,
                    &Shown::default(),
                    Marked {
                        open: &open,
                        ..Marked::default()
                    },
                ),
            );
            links.insert(place_key(place), href);
        }
        Ok(links)
    }

    /// The leagues of `realm` the store holds tabs or characters of, as it
    /// spells them, through the verbs' read (C46).
    fn held_leagues(&self, realm: Realm) -> Result<Vec<String>, Failure> {
        Ok(acquisition_verbs::LeaguesHeld::read(
            &self.served()?.facts()?,
            &acquisition_verbs::leagues::PolicyRead::of(None),
            realm,
            None,
            true,
        )?
        .leagues
        .into_iter()
        .filter_map(|l| l.league)
        .collect())
    }

    /// The store's league list (the `leagues` job's latest answer), whole,
    /// each league marked with whether this realm has tabs of it on disk:
    /// what both pages' league control lists, through the verbs' two
    /// reads (C46), the list and what the realm holds; a word that is no
    /// realm holds nothing. Where no list was fetched, the leagues the
    /// realm holds places in; none where it holds none either.
    fn leagues(&self, realm: &str) -> Result<Option<Vec<(String, bool)>>, Failure> {
        let account = self.served()?;
        let list = account.league_list(true)?;
        let held = match Realm::parse(realm) {
            Some(realm) => {
                acquisition_verbs::LeaguesHeld::read(
                    &account.facts()?,
                    &acquisition_verbs::leagues::PolicyRead::of(None),
                    realm,
                    None,
                    true,
                )?
                .leagues
            }
            None => Vec::new(),
        };
        // the league list never fetched (a store with no session cannot
        // fetch one): the leagues the store holds places in, so a page
        // over a copied store still moves between its leagues; none where
        // it holds none
        if list.fetched_at.is_none() {
            let leagues: Vec<(String, bool)> = held
                .into_iter()
                .filter_map(|h| h.league.map(|league| (league, h.tabs > 0)))
                .collect();
            return Ok((!leagues.is_empty()).then_some(leagues));
        }
        Ok(Some(
            list.leagues
                .into_iter()
                .map(|l| {
                    let on_disk = held
                        .iter()
                        .any(|h| h.league.as_deref() == Some(l.id.as_str()) && h.tabs > 0);
                    (l.id, on_disk)
                })
                .collect(),
        ))
    }
}

/// A page's side of the one refusal shape (each surface spells its own
/// ways on): its words the CLI's, and a way on a link to the page
/// that takes it — a search the form can ask, its own fields; a league's
/// plan or its tabs and characters; the saved list, whole where asked
/// and for one row's show (no page shows one row alone); a saved search
/// opened — for the account this server serves; or, for a saved search's
/// write or clear, a form: `{"post": <the route>, "fields": {…}}`, the
/// fields that route takes as strings, `if_revision` `0` for a create at
/// an id. A way no page takes is offered nowhere on a page: another
/// account, a price or policy write, a job, `describe`, a search the form
/// cannot ask, a saved search written at the revision read. A link to a
/// step (a league's plan, a saved search opened) carries `from`, the
/// page's, where the page renders it (the landing rule); none in a JSON
/// answer.
struct Paged<'a> {
    web: &'a Web,
    from: Option<&'a FromPage>,
}

impl acquisition_verbs::refusal::Surface for Paged<'_> {
    /// The CLI's words, but where the refusal's offers are its remedy and
    /// a page takes each (a plan whose policy moved, a league the policy
    /// does not cover among ten or fewer): the cause alone, its ways on the
    /// links beside it.
    fn words(&self, e: &anyhow::Error) -> String {
        crate::terminal::failure_words_by(e, |fault| match fault {
            Fault::Replan { .. } => fault.to_string(),
            Fault::NotCovered { covered, .. }
                if (1..=acquisition_store::bound::LISTED).contains(&covered.len()) =>
            {
                fault.to_string()
            }
            _ => crate::terminal::fault_words(fault),
        })
    }

    fn request(
        &self,
        way: &acquisition_verbs::refusal::Way,
    ) -> Option<(Value, Option<&'static str>)> {
        use acquisition_verbs::refusal::Way;
        let league = |realm: &Realm, league: &str| {
            vec![
                ("realm", realm.as_str().to_string()),
                ("league", league.to_string()),
            ]
        };
        let link = match way {
            // a continuation, a request with its place, is the one way a
            // page spells it: its token, for the account served
            Way::Search(request) if continues(request) => {
                if !request
                    .scope
                    .account
                    .as_deref()
                    .is_none_or(|a| self.web.serves(a))
                {
                    return None;
                }
                self.web.link("search", &[("next", request.token())])
            }
            Way::Search(request) => self
                .web
                .link("search", &self.web.asked_by_the_form(request)?),
            Way::RefreshPlan {
                account,
                realm,
                league: named,
            } if self.web.serves(account) => {
                from::onward(&self.web.link("plan", &league(realm, named)), self.from)
            }
            Way::Tabs {
                account,
                realm,
                league: named,
            } if self.web.serves(account) => self.web.link("tabs", &league(realm, named)),
            // a job's result, under its row on the jobs page
            Way::Result { id } => format!(
                "{}#{}",
                self.web.link("jobs", &[("job", id.to_string())]),
                page::job_anchor(*id)
            ),
            Way::SavedSearches { expand: false } => self.web.link("saved", &[]),
            Way::SavedSearches { expand: true } | Way::SavedSearchShow { .. } => {
                self.web.link("saved", &[("expand", "true".to_string())])
            }
            Way::SavedSearchRun { id, account }
                if account.as_deref().is_none_or(|a| self.web.serves(a)) =>
            {
                from::onward(
                    &self.web.link("saved/open", &[("id", id.clone())]),
                    self.from,
                )
            }
            Way::SavedSearchSet(set) => {
                let mut fields = serde_json::Map::new();
                if let Some(id) = &set.id {
                    fields.insert("id".into(), json!(id));
                }
                match set.expect {
                    Expect::Exactly(revision) => {
                        fields.insert("if_revision".into(), json!(revision.to_string()));
                    }
                    Expect::Create if set.id.is_some() => {
                        fields.insert("if_revision".into(), json!("0"));
                    }
                    Expect::Create => {}
                    // a page never writes at the revision read (C168)
                    Expect::Current => return None,
                }
                for (key, value) in [
                    ("name", &set.name),
                    ("realm", &set.realm),
                    ("membership", &set.membership),
                ] {
                    if let Some(value) = value {
                        fields.insert(key.into(), json!(value));
                    }
                }
                // the source as the save form carries it (C194)
                match &set.query {
                    Some(SavedQuery::Text(text)) => {
                        fields.insert("text".into(), json!(text));
                    }
                    Some(SavedQuery::Trade(trade)) => {
                        fields.insert("trade".into(), json!(trade_word(trade)));
                    }
                    None => {}
                }
                return Some((
                    json!({ "post": self.web.link("saved", &[]), "fields": fields }),
                    None,
                ));
            }
            Way::SavedSearchClear { id, if_revision } => {
                return Some((
                    json!({
                        "post": self.web.link("saved/clear", &[]),
                        "fields": { "id": id, "if_revision": if_revision.to_string() },
                    }),
                    None,
                ));
            }
            _ => return None,
        };
        Some((json!(link), None))
    }
}

/// Whether `request` is a continuation: a rows request with its place,
/// the rows it starts after and the basis that place is in, as
/// `Request::from_token` takes one.
fn continues(request: &Request) -> bool {
    request.basis.is_some()
        && matches!(
            &request.view.shape,
            acquisition_search::answer::Shape::Rows(rows) if rows.after.is_some()
        )
}

/// The request a window of a table's rows asks: the page's request at
/// `basis`, its place `offset` rows into the order and its limit
/// `count`, `item` among the row's fields beside the page's own (the
/// table's `icon`), so each row carries its price. The offset counts from
/// the order's first row whatever place the page's request held: a table
/// lists the whole order.
fn window_request(
    page: &Request,
    basis: &acquisition_search::Basis,
    offset: usize,
    count: usize,
) -> Request {
    use acquisition_search::answer::{RowField, Shape};
    let mut request = page.clone();
    request.basis = Some(basis.clone());
    if let Shape::Rows(rows) = &mut request.view.shape {
        rows.after = Some(offset);
        rows.limit = Some(count);
        if !rows.fields.contains(&RowField::Item) {
            rows.fields.push(RowField::Item);
        }
    }
    request
}

/// `request` with its site search as the page reads one: no text beside
/// it, a JSON string as its object, and no realm beside a URL that names
/// the realm the scope holds, which is where the URL is searched (K6). Two
/// spellings of one site search compare equal.
fn as_asked(mut request: Request) -> Request {
    // the text beside a site search is its translation, verified by the
    // read: the site search is what was asked
    if request.query.trade.is_some() {
        request.query.text = None;
    }
    if let Some(Value::String(word)) = &request.query.trade {
        if let Ok(object @ Value::Object(_)) = serde_json::from_str::<Value>(word) {
            request.query.trade = Some(object);
        } else if let Some(url) = crate::search_cmd::trade_realm(word)
            && request.scope.realm.as_ref().map(|r| r.as_str()) == Some(url.as_str())
        {
            request.scope.realm = None;
        }
    }
    request
}

/// `request` with each count key as the key's one reader spells it
/// (`Counts::lists`, through `bind::bind_key`) and the form's list reads it
/// back (`counts::count_keys`): two spellings of one key compare equal.
fn read_keys(mut request: Request) -> Request {
    if let acquisition_search::answer::Shape::Counts(counts) = &mut request.view.shape {
        counts.keys = counts
            .keys
            .iter()
            .map(|key| {
                let alone = acquisition_search::answer::Counts {
                    keys: vec![key.clone()],
                    sum: None,
                    limit: None,
                };
                acquisition_search::counts::count_keys(&alone.lists().join(","))
                    .ok()
                    .and_then(|read| read.into_iter().next())
                    .unwrap_or_else(|| key.clone())
            })
            .collect();
    }
    request
}

/// Run a page's blocking work — store reads, a corpus load — off the
/// server's threads.
async fn blocking<T: Send + 'static>(
    web: &Arc<Web>,
    work: impl FnOnce(&Web) -> T + Send + 'static,
) -> T {
    let web = Arc::clone(web);
    match tokio::task::spawn_blocking(move || work(&web)).await {
        Ok(value) => value,
        Err(e) => std::panic::resume_unwind(e.into_panic()),
    }
}

fn pick<'a>(fields: &'a [(String, String)], name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
        .filter(|v| !v.is_empty())
}

fn realm_of(fields: &[(String, String)]) -> Result<Realm, Failure> {
    match pick(fields, "realm") {
        None => Ok(Realm::DEFAULT),
        Some(word) => {
            acquisition_verbs::realm(word).map_err(|e| Failure::Other(anyhow::anyhow!(e)))
        }
    }
}

// ---- the search pages ---------------------------------------------------------------------------

/// The form, its league list the counts by league over the corpus, of
/// the realm and the membership the URL names (Clear's link names them).
async fn home(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    // `from`, then the switch, first: neither is the search's
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed("Search", &failure, &headers, &Here::none()),
    };
    let (fields, all_fields) = switched(fields);
    let here = Here::place(FromPage::here(Place::Home, &fields), back);
    let realm = pick(&fields, "realm").unwrap_or("pc").to_string();
    let mut filled = page::Filled::empty(&realm);
    // the scope's membership, as `search` reads it: the word sent, which
    // the form's select shows (Clear carries it)
    filled.membership = pick(&fields, "membership").map(str::to_string);
    let (leagues, said) = blocking(&web, move |web| match web.leagues(&realm) {
        Ok(leagues) => (leagues, Ok(())),
        Err(failure) => (None, Err(failure)),
    })
    .await;
    let said = match said {
        Ok(()) => String::new(),
        Err(failure) => page::refused(&web.refusal(&failure, here.onward()), here.onward()),
    };
    let body = format!(
        "{said}{}{}",
        page::editor(&web.form, &filled, leagues.as_deref(), here.onward()),
        page::switch(all_fields)
    );
    web.page_at(
        "Search",
        Head {
            lists: true,
            count: true,
            ..Head::default()
        },
        &body,
        &here,
    )
}

/// The answer: a form submission folded into the trade JSON, the
/// query box's text or the trade box's site search (#188), or a
/// continuation's `next`, answered over the held corpus. A site search
/// whose URL names another realm than the one the page carried runs in
/// the URL's, and the page says so above the answer. With `saved` and
/// `saved_revision`, read apart from the
/// fold, the page says which saved search it was opened from and saves
/// over it at that revision; the JSON is the answer alone.
async fn search(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let json = wants_json(&headers);
    // `from`, then the switch, the page's own, first: nothing the page
    // reads, links or carries meets them
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed("Answer", &failure, &headers, &Here::none()),
    };
    let (fields, all_fields) = switched(fields);
    // a value picked by name, the page's too: it orders the page the
    // search as sent answers, and the search never meets it
    let (fields, by) = picked(fields);
    // the opened rows, the landing and a row's toggle, taken off before
    // the fold as `saved` is
    let (fields, opening) = opening(fields);
    if let Some((items, id)) = opening.toggled() {
        let mut to: Vec<(&str, String)> = fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        to.extend(items.into_iter().map(|i| ("item", i)));
        to.push(("at", id.to_string()));
        return see_other(&format!(
            "{}#{}",
            web.link("search", &to),
            page::row_anchor(id)
        ));
    }
    let sent = fields.clone();
    let editing = fields.clone();
    let sent_items = opening.items.clone();
    let (asked, filled, leagues) = blocking(&web, move |web| {
        // the query as read, the fold's or a token's: what the editors are
        // filled from, answered or refused
        let mut read: Option<Source> = None;
        let asked = (|| -> Result<_, Failure> {
            let (fields, from) = opened_from(fields)?;
            // the realm the page carried beside a site search whose URL names
            // another: not read, and said above the answer (#188)
            let mut carried = None;
            let mut continued = None;
            let request = match pick(&fields, "next") {
                Some(token) => {
                    let request = Request::from_token(token)?;
                    read = Some(Source::of(&request));
                    continued = Some(request.clone());
                    request
                }
                None => {
                    let folded = form::fold(&web.form, &fields).map_err(Failure::Usage)?;
                    read = Some(Source::folded(&folded));
                    if let (Some(trade), Some(realm)) = (&folded.trade, &folded.realm)
                        && let Some(url) = crate::search_cmd::trade_realm(trade)
                        && Realm::parse(realm).is_none_or(|r| r.as_str() != url)
                    {
                        carried = Some((realm.clone(), url));
                    }
                    let request = Asked::read(&folded.request()?)?.request;
                    read = Some(Source::of(&request));
                    request
                }
            };
            let corpus = web.corpus(&request)?;
            let answer = acquisition_search::answer(&corpus, &request)?;
            // a table's first block, over the same corpus: at the answer's
            // basis by construction, its route the first window's request
            // the opened rows a table holds, each at its index in the order:
            // one no order holds is dropped, and a page of cards holds none
            let mut kept = Vec::new();
            let window = match &answer.view {
                acquisition_search::answer::ViewOut::Rows(rows)
                    if !rows
                        .fields
                        .contains(&acquisition_search::answer::RowField::Item)
                        && answer.total.matched > 0 =>
                {
                    let count = answer.total.matched.min(page::BLOCK_ROWS);
                    let token = window_request(&request, &answer.basis, 0, count).token();
                    let (opened, at) = opening.held(&corpus, &request)?;
                    kept = opened.iter().map(|(id, _)| id.clone()).collect();
                    let at_row: Vec<usize> = opened.iter().map(|(_, n)| *n).collect();
                    Some(page::Window {
                        route: web.link("rows", &[("next", token)]),
                        segments: cut(
                            &corpus,
                            &request,
                            &answer.basis,
                            answer.total.matched,
                            &at_row,
                            at,
                        )?,
                    })
                }
                _ => None,
            };
            // each card's place, a link where a place page lists it: the
            // cards' rows, or the table's opened ones
            let shown: Vec<&acquisition_search::corpus::Place> = match &window {
                Some(window) => window
                    .segments
                    .iter()
                    .filter(|s| s.opened)
                    .flat_map(|s| s.blocks.iter().flat_map(|(_, b)| &b.rows))
                    .map(|row| &row.place)
                    .collect(),
                None => answer.rows.iter().map(|row| &row.place).collect(),
            };
            let placed = if json {
                std::collections::HashMap::new()
            } else {
                web.place_links(&shown)?
            };
            let opened = match from {
                Some((id, at)) if !json => {
                    let row = match web.saved_at(&id)? {
                        Some(SavedRow::Read(row)) => OpenedRow::Read(row),
                        Some(SavedRow::Unreadable(_)) => OpenedRow::Unreadable,
                        None => OpenedRow::Gone,
                    };
                    Some(Opened { id, at, row })
                }
                _ => None,
            };
            // the realm's leagues on disk, where a rows answer of one realm's
            // live items links its place pages
            let held = match (&answer.view, &answer.scope.realm) {
                (
                    acquisition_search::answer::ViewOut::Rows(_),
                    acquisition_search::Realm::One(realm),
                ) if !json => match Realm::parse(realm) {
                    Some(realm) => web.held_leagues(realm)?,
                    None => Vec::new(),
                },
                _ => Vec::new(),
            };
            Ok((
                answer, window, opened, carried, continued, request, held, kept, placed,
            ))
        })();
        let filled = web.editors(&editing, read.as_ref(), asked.as_ref().ok().map(|a| &a.0));
        // the form's league list, the realm's; none where it cannot be read,
        // the page's answer or refusal standing whatever it is
        let leagues = if json {
            None
        } else {
            web.leagues(&filled.realm).ok().flatten()
        };
        (asked, filled, leagues)
    })
    .await;
    // a value picked by name (`view.by`): answered `303` to the page
    // ordered by it as a head's link writes it, through the one maker of
    // a change to the view, over the rows the search as sent answers; past
    // a full list, to the search's own refusal of the longer one. Beside a
    // count it is let go as the carried order is (#304): the page less it.
    // A search refused as sent is that refusal, the pick let go.
    if let (Some(by), Ok((answer, _, _, _, continued, _, _, kept, _))) = (&by, &asked) {
        let sent: Fields = sent
            .iter()
            .cloned()
            .chain(kept.iter().map(|id| ("item".to_string(), id.clone())))
            .collect();
        let to = match &answer.view {
            acquisition_search::answer::ViewOut::Rows(rows) => {
                match view_link(&web, &sent, continued.as_ref(), view_picked(rows, by)) {
                    Some(link) => web.under_token(&link.href),
                    None => {
                        let failure = Failure::Usage(
                            "this page's search is a continuation the form cannot ask, so no \
                             value orders it: open the search from its start, then add the \
                             column"
                                .to_string(),
                        );
                        return web.failed("Answer", &failure, &headers, &Here::none());
                    }
                }
            }
            _ => {
                let fields: Vec<(&str, String)> =
                    sent.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
                web.link("search", &fields)
            }
        };
        return see_other(&to);
    }
    // the page itself, as its every link writes it — its fields as sent and
    // the opened rows it keeps (or, refused, those its URL opens) — is what
    // its leaving forms post as `from`, as a place page's is
    let items = match &asked {
        Ok((.., kept, _)) => kept.clone(),
        Err(_) => sent_items,
    };
    let itself: Fields = sent
        .iter()
        .cloned()
        .chain(items.into_iter().map(|id| ("item".to_string(), id)))
        .collect();
    let here = Here::place(FromPage::here(Place::Search, &itself), back);
    let editors = page::editor(&web.form, &filled, leagues.as_deref(), here.onward());
    match asked {
        Ok((answer, ..)) if json => document(&answer),
        Ok((answer, window, opened, carried, continued, request, held, kept, placed)) => {
            // the fields every link of the page writes: as sent, the
            // opened rows the table holds, no landing
            let sent: Fields = sent
                .into_iter()
                .chain(kept.into_iter().map(|id| ("item".to_string(), id)))
                .collect();
            let mut head = Head {
                lists: true,
                count: true,
                window: window.is_some(),
                ..Head::default()
            }
            .with("answer", &answer.basis);
            if let Some(Opened {
                row: OpenedRow::Read(row),
                ..
            }) = &opened
            {
                head = head.with("saved_search", row.revision);
            }
            let said = carried
                .map(|(page, url)| {
                    format!(
                        "<p class=\"notice\">{}</p>\n",
                        esc(&format!(
                            "The site search's URL names its realm, {url}: it ran there, not in \
                             {page}, the realm the page showed."
                        ))
                    )
                })
                .unwrap_or_default();
            let mut rows_links = rows_links(&web, &sent, continued.as_ref(), &answer);
            rows_links.places = by_place(&web, &request, &answer, &held);
            let view_links = |value: &str| match &answer.view {
                acquisition_search::answer::ViewOut::Rows(rows) => {
                    view_link(&web, &sent, continued.as_ref(), view_change(rows, value)?)
                }
                _ => None,
            };
            web.page_at(
                "Answer",
                head,
                &format!(
                    "{said}{editors}<div id=\"{}\">\n{}</div>\n{}",
                    page::ANSWER,
                    page::answer(
                        &answer,
                        page::Saving {
                            source: &request.query,
                            opened: opened.as_ref(),
                            view: &filled.view,
                            from: here.onward(),
                        },
                        &web.saved_links(),
                        &|way| {
                            use acquisition_verbs::refusal::Surface as _;
                            Paged {
                                web: &web,
                                from: here.onward(),
                            }
                            .request(way)
                            .and_then(|(link, _)| link.as_str().map(str::to_string))
                        },
                        &rows_links,
                        &view_links,
                        &page::Shows {
                            window: window.as_ref(),
                            placed: &|place| placed.get(&place_key(place)).cloned(),
                            fields: &sent,
                        },
                    ),
                    page::switch(all_fields)
                ),
                &here,
            )
        }
        // a refused search is the same page, its editors filled from what
        // was sent (#190): its status and its JSON the refusal's
        Err(failure) => web.failed_under(
            "Answer",
            &failure,
            &headers,
            &editors,
            &page::switch(all_fields),
            &here,
        ),
    }
}

/// A request's fields as a page sends them, in order.
type Fields = Vec<(String, String)>;

/// What a search page's editors are filled from (the one page,
/// `4-full-1-form-design.md` §2): the query as read — the form's fields
/// folded, or a request read from them or from a continuation's token — and
/// the words of its scope and its count.
struct Source {
    query: acquisition_search::answer::QueryInput,
    realm: Option<String>,
    membership: Option<String>,
    /// A count's keys and limit, where the request asks a count.
    counted: Option<(String, Option<usize>)>,
}

impl Source {
    /// The fold's reading: the query box's text, the site box's search, or
    /// the form's envelope, and the scope the page carried.
    fn folded(folded: &form::Folded) -> Source {
        use acquisition_search::answer::QueryInput;
        Source {
            query: match (&folded.text, &folded.trade) {
                (Some(text), _) => QueryInput {
                    text: Some(text.clone()),
                    ..QueryInput::default()
                },
                (None, Some(trade)) => QueryInput {
                    trade: Some(Value::String(trade.clone())),
                    ..QueryInput::default()
                },
                (None, None) => QueryInput {
                    trade: Some(folded.envelope.clone()),
                    ..QueryInput::default()
                },
            },
            realm: folded.realm.clone(),
            membership: folded.membership.clone(),
            counted: None,
        }
    }

    /// A request's reading: its query, its scope, a count's keys as the
    /// form's one field reads them.
    fn of(request: &Request) -> Source {
        Source {
            query: request.query.clone(),
            realm: request.scope.realm.as_ref().map(|r| r.as_str().to_string()),
            membership: Some(request.scope.membership.as_str().to_string()),
            counted: match &request.view.shape {
                acquisition_search::answer::Shape::Counts(counts) => {
                    Some((counts.lists().join(","), counts.limit))
                }
                _ => None,
            },
        }
    }
}

/// The plain links beside a rows answer that has a row: the same answer
/// the other way — cards or the table — and, on a continuation's page,
/// the same search from its first rows. Reached by the form's fields, the
/// other way is the page's own URL, its fields as sent, with
/// `view.fields=icon` added where it shows cards and taken out where it
/// shows the table. Reached by a continuation, whose token holds its
/// view whole, both are the continued request from its start — its place
/// and basis dropped, the other way's fields swapped — spelt as the form
/// would ask it, the saved search the page was opened from carried; none
/// where the form cannot ask it (a site search, which the form reads as
/// its translated text). None where the rows carry another list than
/// the page's two, which no form asks.
fn rows_links(
    web: &Web,
    sent: &Fields,
    continued: Option<&Request>,
    answer: &acquisition_search::Answer,
) -> page::RowsLinks {
    use acquisition_search::answer::{RowField, Shape, ViewOut};
    let none = page::RowsLinks::default();
    let ViewOut::Rows(rows) = &answer.view else {
        return none;
    };
    if answer.rows.is_empty() {
        return none;
    }
    let (other, words) = match rows.fields.as_slice() {
        [RowField::Icon, RowField::Item] => (vec![RowField::Icon], "as a table"),
        [RowField::Icon] => (vec![RowField::Icon, RowField::Item], "as cards"),
        _ => return none,
    };
    let Some(continued) = continued else {
        // cards cut nothing: the other way names no opened row
        let mut fields: Vec<(&str, &str)> = sent
            .iter()
            .filter(|(k, _)| k != "view.fields" && k != "item")
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        if other == [RowField::Icon] {
            fields.push(("view.fields", "icon"));
        }
        let query: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", page::enc(k), page::enc(v)))
            .collect();
        return page::RowsLinks {
            other: Some(page::Presentation {
                href: format!("search?{}", query.join("&")),
                words,
            }),
            first: None,
            places: Vec::new(),
        };
    };
    // the continued search from its start, as the form would ask it, the
    // saved search it was opened from carried as every link of it is
    // cards cut nothing: the other way and the first rows carry no row
    let saved = carried_on(sent, false);
    let spelt = |request: Request| {
        web.asked_by_the_form(&request)
            .map(|fields| format!("{}{saved}", web.link("search", &fields)))
    };
    let mut first = continued.clone();
    first.basis = None;
    let Shape::Rows(view) = &mut first.view.shape else {
        return none;
    };
    view.after = None;
    let mut swapped = first.clone();
    if let Shape::Rows(view) = &mut swapped.view.shape {
        view.fields = other;
    }
    page::RowsLinks {
        other: spelt(swapped).map(|href| page::Presentation { href, words }),
        first: spelt(first),
        places: Vec::new(),
    }
}

/// The ways from a rows answer to the place pages: "by stash tab" and "by
/// character", each the place page of the one league the query fixes, else
/// the default's, with the canonical text and the view. None where the form
/// cannot ask the request, under all realms, or under a membership other
/// than live: a place page is one realm's live items. A league is named as
/// the store holds it (`held`, the realm's leagues on disk).
fn by_place(
    web: &Web,
    request: &Request,
    answer: &acquisition_search::Answer,
    held: &[String],
) -> Vec<page::Presentation> {
    use acquisition_search::answer::{Membership, Shape, ViewOut};
    let (ViewOut::Rows(_), Shape::Rows(view)) = (&answer.view, &request.view.shape) else {
        return Vec::new();
    };
    let acquisition_search::Realm::One(realm) = &answer.scope.realm else {
        return Vec::new();
    };
    if answer.scope.membership != Membership::Live || web.asked_by_the_form(request).is_none() {
        return Vec::new();
    }
    let Some(realm) = Realm::parse(realm) else {
        return Vec::new();
    };
    // the store's spelling of a league the query named in another case
    let league = acquisition_search::places::held_spelling(
        answer
            .fixed_league()
            .unwrap_or(acquisition_plan::DEFAULT_LEAGUE),
        held,
    );
    let shown = Shown {
        text: Some(answer.query.text.clone()),
        view: acquisition_search::answer::Rows {
            values: view.values.clone(),
            sort: view.sort.clone(),
            desc: view.desc,
            ..Default::default()
        },
        ..Shown::default()
    };
    [
        (Listed::Tabs, "by stash tab"),
        (Listed::Characters, "by character"),
    ]
    .into_iter()
    .map(|(listed, words)| page::Presentation {
        href: web.link(
            listed.route(),
            &place_fields(listed, realm, league, &shown, Marked::default()),
        ),
        words,
    })
    .collect()
}

/// The one maker of a change to the view: the same page ordered by `value`,
/// largest first, or the other way where its rows are already ordered by
/// it. A value among the answer's values leaves the list as the page's URL
/// names it; another names the answer's list with it added at its end,
/// which the sort then orders by. A value outside a full list ([`VALUES`])
/// gets no link, since the search refuses a list past it. Spelt as
/// [`rows_links`] spells the other presentation: reached by the form's
/// fields, the page's own URL, its fields as sent with the view's order and
/// list replaced; reached by a continuation, the continued search from its
/// start as the form would ask it, the saved search the page was opened
/// from carried; none where the form cannot ask it. Heads and a card's
/// numbers both call it.
///
/// [`VALUES`]: acquisition_search::answer::VALUES
fn view_link(
    web: &Web,
    sent: &Fields,
    continued: Option<&Request>,
    change: ViewChange,
) -> Option<page::ViewLink> {
    use acquisition_search::answer::Shape;
    let ViewChange {
        sort: value,
        desc,
        values,
        held,
        ordered,
    } = change;
    let value = value.as_str();
    let listed = |list: &mut Vec<String>| {
        if let Some(values) = &values {
            list.clone_from(values);
        }
    };
    let href = match continued {
        None => {
            let mut fields: Vec<(&str, String)> = sent
                .iter()
                .filter(|(k, _)| {
                    !matches!(k.as_str(), "view.sort" | "view.desc") && (held || k != "view.value")
                })
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();
            fields.push(("view.sort", value.to_string()));
            if desc {
                fields.push(("view.desc", "true".to_string()));
            }
            let mut values = Vec::new();
            listed(&mut values);
            fields.extend(values.into_iter().map(|v| ("view.value", v)));
            let query: Vec<String> = fields
                .iter()
                .map(|(k, v)| format!("{}={}", page::enc(k), page::enc(v)))
                .collect();
            format!("search?{}", query.join("&"))
        }
        Some(continued) => {
            let mut request = continued.clone();
            request.basis = None;
            let Shape::Rows(view) = &mut request.view.shape else {
                return None;
            };
            view.after = None;
            view.sort = Some(value.to_string());
            view.desc = desc;
            listed(&mut view.values);
            // a change to the view keeps each opened row
            let saved = carried_on(sent, true);
            format!(
                "{}{saved}",
                web.link("search", &web.asked_by_the_form(&request)?)
            )
        }
    };
    Some(page::ViewLink {
        href,
        held,
        ordered,
    })
}

/// A change to the view by one value: the rows
/// ordered by it, largest first or the other way where they are ordered
/// by it so, and the list of values to name where the view lacks it —
/// the answer's with it added at the end. None where the list is full
/// ([`VALUES`]). [`view_link`] spells it for an answer, a place page
/// for its own fields.
///
/// [`VALUES`]: acquisition_search::answer::VALUES
struct ViewChange {
    sort: String,
    desc: bool,
    values: Option<Vec<String>>,
    held: bool,
    ordered: Option<bool>,
}

fn view_change(rows: &acquisition_search::answer::RowsOut, value: &str) -> Option<ViewChange> {
    let held = rows.values.iter().any(|v| v == value);
    if !held && rows.values.len() >= acquisition_search::answer::VALUES {
        return None;
    }
    Some(view_picked(rows, value))
}

/// A pick's change (`view.by`, the value finder): [`view_change`], and
/// past a full list the same change all the same, the list one longer
/// than a row carries, so the page it lands on is the search's own
/// refusal of that list (C143), never the page unchanged.
fn view_picked(rows: &acquisition_search::answer::RowsOut, value: &str) -> ViewChange {
    let held = rows.values.iter().any(|v| v == value);
    let ordered = (rows.sort.as_deref() == Some(value)).then_some(rows.desc);
    ViewChange {
        sort: value.to_string(),
        // largest first, or the other way where the rows are ordered by it so
        desc: ordered.is_none_or(|desc| !desc),
        values: (!held).then(|| {
            let mut list = rows.values.clone();
            list.push(value.to_string());
            list
        }),
        held,
        ordered,
    }
}

/// The value a page is to be ordered by (`view.by`, [`page::VIEW_BY`])
/// taken out of its fields, the first thing `search` and `places` do with
/// the switch and `from`, so no fold, link or hidden field meets it: the
/// last sent, where one was.
fn picked(fields: Fields) -> (Fields, Option<String>) {
    let (by, fields): (Fields, Fields) = fields.into_iter().partition(|(k, _)| k == page::VIEW_BY);
    (fields, by.into_iter().next_back().map(|(_, v)| v))
}

/// What a link spelt from a continuation carries of the page beside its
/// request, as query fields to append: the saved search it was opened
/// from, and, where `rows`, the opened rows the page keeps. One list for
/// every such link, so no link of the page drops what another keeps.
fn carried_on(sent: &Fields, rows: bool) -> String {
    sent.iter()
        .filter(|(k, _)| k == "saved" || k == "saved_revision" || (rows && k == "item"))
        .map(|(k, v)| format!("&{}={}", page::enc(k), page::enc(v)))
        .collect()
}

/// The switch ([`page::switch`]) taken out of a page's fields, the first
/// thing `home`, `search` and `count` do, so no fold, link or hidden field
/// of the page meets it; ticked where any of it is `true`, its one value.
/// It is the page's, never the search's: no other value is refused.
fn switched(fields: Fields) -> (Fields, bool) {
    let (switch, fields): (Fields, Fields) =
        fields.into_iter().partition(|(k, _)| k == page::ALL_FIELDS);
    (fields, switch.iter().any(|(_, v)| v == "true"))
}

/// `saved` and `saved_revision` taken out of a search's fields, which the
/// fold would refuse: together or not at all, `saved` an id's shape and
/// the revision a positive integer, else a usage refusal.
fn opened_from(fields: Vec<(String, String)>) -> Result<(Fields, Option<(String, i64)>), Failure> {
    let (opened, fields): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .partition(|(k, _)| k == "saved" || k == "saved_revision");
    let named = |name: &str| {
        opened
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.clone())
    };
    let from = match (named("saved"), named("saved_revision")) {
        (None, None) => None,
        (Some(id), Some(at)) => {
            if !saved::is_id(&id) {
                return Err(Failure::Usage(format!(
                    "{id:?} is no saved search's id: a page opens a saved search by its id, \
                     twelve hex characters"
                )));
            }
            match at.parse::<i64>() {
                Ok(at) if at > 0 => Some((id, at)),
                _ => {
                    return Err(Failure::Usage(format!(
                        "{at:?} is no revision: the revision a saved search was opened at is \
                         a positive whole number"
                    )));
                }
            }
        }
        _ => {
            return Err(Failure::Usage(
                "a saved search is opened by its id and the revision it was opened at, the \
                 two together"
                    .to_string(),
            ));
        }
    };
    Ok((fields, from))
}

/// What a page's URL opens of its rows: each
/// opened row by its item's id (`item`, each once, in the URL's order),
/// the row one load lands at (`at`, never carried on), and a row's toggle
/// (`toggle_item`). `search`, `tabs` and `characters` take them off their
/// fields before reading the rest.
struct Opening {
    items: Vec<String>,
    at: Option<String>,
    toggle: Option<String>,
}

fn opening(fields: Fields) -> (Fields, Opening) {
    let mut opening = Opening {
        items: Vec::new(),
        at: None,
        toggle: None,
    };
    let mut rest = Vec::new();
    for (k, v) in fields {
        match k.as_str() {
            "item" if !v.is_empty() => {
                if !opening.items.contains(&v) {
                    opening.items.push(v);
                }
            }
            "at" if !v.is_empty() => opening.at = Some(v),
            "toggle_item" if !v.is_empty() => opening.toggle = Some(v),
            "item" | "at" | "toggle_item" => {}
            _ => rest.push((k, v)),
        }
    }
    (rest, opening)
}

impl Opening {
    /// A row's toggle: the opened rows with its id taken out where it is
    /// open, added at the end where it is not, and the id the answer
    /// lands at; none where nothing was toggled.
    fn toggled(&self) -> Option<(Vec<String>, &str)> {
        let id = self.toggle.as_deref()?;
        let mut items = self.items.clone();
        match items.iter().position(|i| i == id) {
            Some(at) => {
                items.remove(at);
            }
            None => items.push(id.to_string()),
        }
        Some((items, id))
    }

    /// The opened rows the order of `table` holds, each with its index,
    /// in the URL's order, and the landing row's index where the order
    /// holds it: one order, the search crate's.
    fn held(
        &self,
        corpus: &acquisition_search::Corpus,
        table: &Request,
    ) -> Result<Held, SearchError> {
        let mut ids: Vec<&str> = self.items.iter().map(String::as_str).collect();
        ids.extend(self.at.as_deref());
        if ids.is_empty() {
            return Ok((Vec::new(), None));
        }
        let at = acquisition_search::indices(corpus, table, &ids)?;
        let opened = self
            .items
            .iter()
            .zip(&at)
            .filter_map(|(id, n)| n.map(|n| (id.clone(), n)))
            .collect();
        let landing = self.at.as_ref().and_then(|_| at.last().copied().flatten());
        Ok((opened, landing))
    }
}

/// The opened rows an order holds, each with its index, and the landing
/// row's index.
type Held = (Vec<(String, usize)>, Option<usize>);

/// A window's segments over `corpus`: the order of
/// `table`'s `count` rows cut after each opened row, and in each segment
/// the blocks the server renders — its first, the block holding its
/// opened row, and the block holding the landing row `at` — each the
/// window [`window_request`] asks at `basis`, so every block stands at the
/// page's basis.
fn cut(
    corpus: &acquisition_search::Corpus,
    table: &Request,
    basis: &acquisition_search::Basis,
    count: usize,
    opened: &[usize],
    at: Option<usize>,
) -> Result<Vec<page::Segment>, SearchError> {
    let mut ends: Vec<usize> = opened
        .iter()
        .filter(|n| **n < count)
        .map(|n| n + 1)
        .collect();
    ends.sort_unstable();
    ends.dedup();
    let mut bounds: Vec<(usize, bool)> = ends.iter().map(|end| (*end, true)).collect();
    if ends.last() != Some(&count) {
        bounds.push((count, false));
    }
    let block = page::BLOCK_ROWS;
    let mut segments = Vec::new();
    let mut base = 0;
    for (end, opened) in bounds {
        let mut wanted = vec![0];
        if opened {
            wanted.push((end - 1 - base) / block);
        }
        if let Some(at) = at.filter(|at| (base..end).contains(at)) {
            wanted.push((at - base) / block);
        }
        wanted.sort_unstable();
        wanted.dedup();
        let mut blocks = Vec::new();
        for b in wanted {
            let offset = base + b * block;
            let rows = block.min(end - offset);
            blocks.push((
                b,
                acquisition_search::answer(corpus, &window_request(table, basis, offset, rows))?,
            ));
        }
        segments.push(page::Segment {
            base,
            count: end - base,
            blocks,
            opened,
        });
        base = end;
    }
    Ok(segments)
}

/// A place as a card names it: the key [`Web::place_links`] answers by.
type PlaceKey = (String, String, Option<String>, String);

fn place_key(place: &acquisition_search::corpus::Place) -> PlaceKey {
    (
        place.realm.clone(),
        place.kind.clone(),
        place.league.clone(),
        place.id.clone(),
    )
}

/// One item: `show`'s document, with its art and its badges. With
/// `against`, a query's text, the document is
/// `show --against`'s, through `show::against`: the item, and beside it
/// the query's outcome on it, term by term.
async fn item(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Query(fields): Query<Fields>,
) -> Response {
    let json = wants_json(&headers);
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed("Item", &failure, &headers, &Here::none()),
    };
    let here = Here::place(FromPage::here(Place::Item(id.clone()), &fields), back);
    let against = fields
        .iter()
        .find(|(k, _)| k == "against")
        .map(|(_, text)| acquisition_search::answer::QueryInput {
            trade: None,
            text: Some(text.trim().to_string()),
            tree: None,
        });
    let shown = blocking(&web, move |web| -> Result<_, Failure> {
        let account = web.served()?;
        let opened = account.open()?;
        let shown = match &against {
            Some(query) => {
                acquisition_search::show::against(&opened.facts, &opened.intent, &id, false, query)?
            }
            None => acquisition_search::show(&opened.facts, &opened.intent, &id, false)?,
        };
        let placed = match json {
            true => None,
            false => web
                .place_links(&[&shown.place])?
                .remove(&place_key(&shown.place)),
        };
        Ok((shown, placed))
    })
    .await;
    match shown {
        Ok((shown, _)) if json => document(&shown),
        Ok((shown, placed)) => web.page_at(
            "Item",
            Head::default().with("item", &shown.basis),
            &page::item(&shown, placed.as_deref()),
            &here,
        ),
        Err(failure) => web.failed("Item", &failure, &headers, &here),
    }
}

/// A window of a table's rows (the module doc, "The table"): the
/// request a table's segment names as `next`, its rows from `offset`,
/// `count` of them, made by [`Web::window`] at the token's basis. HTML is
/// the rows alone, which the window script places; JSON is the answer
/// document of the window's request. Refused, in the one refusal shape
/// and as the pages are, with 400: a token no answer printed; an offset
/// or a count that is no whole number; a count of none or past the item
/// rows' bound; a window that does not lie inside the rows, nothing
/// clipped in silence. A basis moved since the page is the search's own
/// refusal with 409, which the script answers by loading the page again.
async fn rows(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Fields>,
) -> Response {
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        let request = Request::from_token(pick(&fields, "next").unwrap_or(""))?;
        let number = |name: &str, what: &str| {
            pick(&fields, name)
                .and_then(|n| n.parse::<usize>().ok())
                .ok_or_else(|| Failure::Usage(format!("`{name}` is {what}: a whole number")))
        };
        let offset = number("offset", "the row a window starts at, counted from the first as 0")?;
        let count = number("count", "how many rows a window holds")?;
        let bound = acquisition_store::bound::ITEM_ROWS;
        if !(1..=bound).contains(&count) {
            return Err(Failure::Usage(format!(
                "a window holds from 1 to {bound} rows, each with its item, and this one asks {count}"
            )));
        }
        let Some(basis) = request.basis.clone() else {
            // `from_token` reads only a request with its place and basis
            return Err(Failure::Usage("a window continues an answer at its basis".to_string()));
        };
        let corpus = web.corpus(&request)?;
        let answer = web.window(&corpus, &request, &basis, offset, count)?;
        let matched = answer.total.matched;
        // the window from its offset, its count beside what is left: no
        // sum of two numbers a request names, which could pass the largest
        if offset >= matched || count > matched - offset {
            return Err(Failure::Usage(format!(
                "a window lies inside the rows: {count} from row {offset} runs past the {} the answer holds",
                plural(matched, "row", "rows")
            )));
        }
        Ok(answer)
    })
    .await;
    match asked {
        Ok(answer) if wants_json(&headers) => document(&answer),
        Ok(answer) => html(
            StatusCode::OK,
            page::window_rows(&answer, acquisition_store::now()),
        ),
        Err(failure) => {
            let doc = web.refusal(&failure, None);
            let status = match doc["kind"].as_str() {
                Some("basis_changed") => StatusCode::CONFLICT,
                _ => StatusCode::BAD_REQUEST,
            };
            if wants_json(&headers) {
                (
                    status,
                    [(header::CONTENT_TYPE, "application/json")],
                    doc.to_string(),
                )
                    .into_response()
            } else {
                html(status, page::refused(&doc, None))
            }
        }
    }
}

/// The count beside a search's button:
/// a form's fields folded as [`search`] folds them, the saved search an
/// answer was opened from set aside, and answered with no row and no
/// order ([`form::fold_count`]): the answer document, JSON whatever
/// was asked, its `total` the number `count.js` writes. A refusal is the
/// search's refusal document. Asked with no basis, it is the store's
/// count now, never compared with the page's answer.
async fn count(State(web): State<Arc<Web>>, Query(fields): Query<Fields>) -> Response {
    let (fields, _) = switched(fields);
    let counted = blocking(&web, move |web| -> Result<_, Failure> {
        let (fields, _) = opened_from(fields)?;
        let folded = form::fold_count(&web.form, &fields).map_err(Failure::Usage)?;
        let request = Asked::read(&folded.request()?)?.request;
        web.answer(&request)
    })
    .await;
    match counted {
        Ok(answer) => document(&answer),
        Err(failure) => web.refused_document(&failure),
    }
}

/// A finder's listing, by the words asked: the stat vocabulary, its
/// document where JSON is asked, else the stat finder's options as HTML,
/// each holding the template of its row ([`page::stat_options`]); or,
/// asked for values (`values=true`), the value listing, its document or
/// the value finder's options, each a submit button
/// ([`page::value_options`]). One route, one field (C138, the listener).
async fn stats(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    use acquisition_search::trade::vocabulary;
    let ask = pick(&fields, "q").unwrap_or_default().to_string();
    let json = wants_json(&headers);
    let listed = match pick(&fields, "values") {
        None => {
            blocking(&web, move |_| {
                vocabulary::vocabulary(&ask, PICKED).map(|listed| match json {
                    true => document(&listed),
                    false => html(StatusCode::OK, page::stat_options(&listed)),
                })
            })
            .await
        }
        Some("true") => {
            blocking(&web, move |_| {
                vocabulary::values(&ask, PICKED).map(|listed| match json {
                    true => document(&listed),
                    false => html(StatusCode::OK, page::value_options(&listed)),
                })
            })
            .await
        }
        Some(other) => {
            let failure = Failure::Usage(format!(
                "`values` {other:?} is no listing a finder asks: `true` lists the values a \
                 column is added by, and a listing without it the stats"
            ));
            return web.failed("Stats", &failure, &headers, &Here::none());
        }
    };
    match listed {
        Ok(answered) => answered,
        Err(e) => web.failed("Stats", &Failure::from(e), &headers, &Here::none()),
    }
}

// ---- the place pages ----------------------------------------------------------------------------

/// Which places a place page lists: a league's stash tabs or
/// its characters, each page its own route, ticks and listing job.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Listed {
    Tabs,
    Characters,
}

impl Listed {
    fn route(self) -> &'static str {
        match self {
            Listed::Tabs => "tabs",
            Listed::Characters => "characters",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Listed::Tabs => "Stash tabs",
            Listed::Characters => "Characters",
        }
    }

    /// The field a ticked place is sent as, the plan's selection field.
    fn tick(self) -> &'static str {
        match self {
            Listed::Tabs => "tab",
            Listed::Characters => "character",
        }
    }
}

async fn tabs(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    places(web, headers, fields, Listed::Tabs).await
}

async fn characters(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    places(web, headers, fields, Listed::Characters).await
}

/// A place page: the places
/// document of the URL's realm and league, asked its query and filter,
/// the places the URL opens open and the places it ticks ticked. A
/// `toggle` is the opener of one place: answered 303 to the page with
/// that place opened or closed, at its head, every tick kept. A tick or
/// an open that names no place of this league is dropped, here and in
/// every URL the page writes. Where the URL opens a place, the document
/// and every open place's segment are read inside one `steady`, so the
/// page stands at one basis, which its basis line says.
async fn places(
    web: Arc<Web>,
    headers: HeaderMap,
    fields: Vec<(String, String)>,
    listed: Listed,
) -> Response {
    let title = listed.title();
    // `from` first: the page's way back, never the places' document's
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed(title, &failure, &headers, &Here::none()),
    };
    // a value picked by name, the page's own: it orders the rows the heads
    // are drawn from, and no places document or segment meets it
    let (fields, by) = picked(fields);
    // the page itself as sent, less what one load reads (a toggle, a
    // landing), is what its leaving forms post as `from`
    let itself: Fields = fields
        .iter()
        .filter(|(k, _)| !matches!(k.as_str(), "toggle" | "toggle_item" | "at" | "refresh"))
        .cloned()
        .collect();
    let place = match listed {
        Listed::Tabs => Place::Tabs,
        Listed::Characters => Place::Characters,
    };
    let here = Here::place(FromPage::here(place.clone(), &itself), back);
    // "Refresh ticked" and "Plan under the policy" are this page's own
    // field, `refresh` (the landing rule: the place form never leaves): answered
    // `303` to `plan` with the selection as sent and this page as `from`
    if let Some(asked) = pick(&fields, "refresh") {
        let mut to: Vec<(&str, String)> = fields
            .iter()
            .filter(|(k, _)| {
                matches!(k.as_str(), "realm" | "league" | "deep") || k == listed.tick()
            })
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        match asked {
            "ticked" => to.push(("selected", "yes".to_string())),
            "policy" => to.retain(|(k, _)| matches!(*k, "realm" | "league")),
            other => {
                let failure = Failure::Usage(format!(
                    "`refresh` {other:?} is no plan a place page asks: `ticked`, the ticked \
                     places, or `policy`, the league's policy"
                ));
                return web.failed(title, &failure, &headers, &here);
            }
        }
        return web.landed("plan", &to, None, &here);
    }
    // the opened rows, the landing and a row's toggle
    let (fields, opening) = opening(fields);
    let toggled = opening.toggled().map(|(items, id)| (items, id.to_string()));
    let sent_items = opening.items.clone();
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed(title, &failure, &headers, &here),
    };
    let league = pick(&fields, "league")
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    let shown = match Shown::read(&fields) {
        Ok(shown) => shown,
        Err(failure) => return web.failed(title, &failure, &headers, &here),
    };
    let asking = shown.asking();
    let sent = fields.clone();
    let read = blocking(&web, move |web| -> Result<_, Failure> {
        let account = web.served()?;
        let now = acquisition_store::now();
        let opened = account.open()?;
        let at = acquisition_search::places::Reading {
            store: &opened.facts,
            intent: &opened.intent,
            holder: &web.held,
        };
        let make = || match listed {
            Listed::Tabs => Places::tabs(at, realm.as_str(), &league, &asking, now),
            Listed::Characters => Places::characters(at, realm.as_str(), &league, &asking, now),
        };
        let (places, windows, kept) = if sent.iter().any(|(k, _)| k == "open") {
            let scope = Request {
                scope: acquisition_search::answer::Scope {
                    account: None,
                    realm: Some(acquisition_search::Realm::One(realm.as_str().to_string())),
                    membership: acquisition_search::answer::Membership::Live,
                },
                ..Request::default()
            };
            acquisition_search::places::steady(at, &scope, |corpus| {
                let places = make()?;
                let openable = openable(&places);
                // a toggle that names a place answers with a redirect and
                // draws no segment; one that names none is dropped, as the
                // redirect drops it
                if pick(&sent, "toggle").is_some_and(|t| openable.contains(&t))
                    || opening.toggle.is_some()
                {
                    return Ok((places, std::collections::HashMap::new(), Vec::new()));
                }
                let open = named_places(&sent, "open", &openable);
                let (windows, kept) = windows_over(corpus, &places, &open, &opening)?;
                let windows = windows
                    .into_iter()
                    .map(|(id, (token, segments))| {
                        (
                            id,
                            page::Window {
                                route: web.link("rows", &[("next", token)]),
                                segments,
                            },
                        )
                    })
                    .collect();
                Ok((places, windows, kept))
            })?
            .1
        } else {
            (make()?, std::collections::HashMap::new(), Vec::new())
        };
        // a league with none: the terminal's cause, never its command
        let none = match (listed, places.places.is_empty()) {
            (Listed::Characters, true) => Some(store_cmd::no_characters_cause(
                Some(realm),
                Some(&league),
                &opened.facts,
            )?),
            _ => None,
        };
        let policy =
            acquisition_verbs::leagues::PolicyRead::of_account(account).names(realm, &league);
        let leagues = web.leagues(realm.as_str())?;
        Ok((league, places, windows, kept, none, policy, leagues))
    })
    .await;
    let (league, places, windows, kept, none, policy, leagues) = match read {
        Ok(read) => read,
        Err(failure) => return web.failed(title, &failure, &headers, &here),
    };
    if wants_json(&headers) {
        return document(&places);
    }
    let openable = openable(&places);
    let tickable: Vec<&str> = places
        .every()
        .into_iter()
        .filter(|p| matches!(p.place, Kind::Tab | Kind::Character))
        .map(|p| p.id())
        .collect();
    let mut open = named_places(&fields, "open", &openable);
    let ticked = named_places(&fields, listed.tick(), &tickable);
    let deep = listed == Listed::Tabs && pick(&fields, "deep").is_some();
    if let Some(toggle) = pick(&fields, "toggle").filter(|t| openable.contains(t)) {
        match open.iter().position(|o| o == toggle) {
            Some(at) => {
                open.remove(at);
            }
            None => open.push(toggle.to_string()),
        }
        let at = web.link(
            listed.route(),
            &place_fields(
                listed,
                realm,
                &league,
                &shown,
                Marked {
                    deep,
                    open: &open,
                    items: &sent_items,
                    ticked: &ticked,
                },
            ),
        );
        return see_other(&format!("{at}#{}", page::place_anchor(toggle)));
    }
    // a row's toggle: the page with the row opened or closed, landing at
    // it (`at`, one load's, written by no link or form)
    if let Some((items, id)) = &toggled {
        let mut to = place_fields(
            listed,
            realm,
            &league,
            &shown,
            Marked {
                deep,
                open: &open,
                items,
                ticked: &ticked,
            },
        );
        to.push(("at", id.clone()));
        return see_other(&format!(
            "{}#{}",
            web.link(listed.route(), &to),
            page::row_anchor(id)
        ));
    }
    // the page as it draws itself, a tick or an open that names no place
    // dropped, is what its leaving forms post as `from`
    let itself: Fields = place_fields(
        listed,
        realm,
        &league,
        &shown,
        Marked {
            deep,
            open: &open,
            items: &kept,
            ticked: &ticked,
        },
    )
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    let here = Here::place(FromPage::here(place, &itself), here.back);
    let spell = |way: &acquisition_verbs::refusal::Way| {
        use acquisition_verbs::refusal::Surface as _;
        Paged {
            web: &web,
            from: here.onward(),
        }
        .request(way)
        .and_then(|(link, _)| link.as_str().map(str::to_string))
    };
    // what the selects' form carries: every field but the ticks and the
    // opens, since one id can stand in two leagues
    let carried: Vec<(&'static str, String)> = place_fields(
        listed,
        realm,
        &league,
        &shown,
        Marked {
            deep,
            ..Marked::default()
        },
    )
    .into_iter()
    .filter(|(k, _)| !matches!(*k, "realm" | "league"))
    .collect();
    // a fold's never-fetched count: the page under `fetched=never` alone,
    // every open and tick kept
    let never = Shown {
        filter: acquisition_search::places::Filter {
            fetched: Some(acquisition_search::places::Fetched::Never),
            ..Default::default()
        },
        ..Shown::default()
    };
    let never = web.link(
        listed.route(),
        &place_fields(
            listed,
            realm,
            &league,
            &never,
            Marked {
                deep,
                open: &open,
                items: &kept,
                ticked: &ticked,
            },
        ),
    );
    // a value's head: the page ordered by it, through the one maker of a
    // change to the view, every open and tick kept
    let changed_link = |change: ViewChange| {
        let mut changed = shown.clone();
        changed.view.sort = Some(change.sort);
        changed.view.desc = change.desc;
        if let Some(values) = change.values {
            changed.view.values = values;
        }
        page::ViewLink {
            href: web.link(
                listed.route(),
                &place_fields(
                    listed,
                    realm,
                    &league,
                    &changed,
                    Marked {
                        deep,
                        open: &open,
                        items: &kept,
                        ticked: &ticked,
                    },
                ),
            ),
            held: change.held,
            ordered: change.ordered,
        }
    };
    let view_links = |rows: &acquisition_search::answer::RowsOut, value: &str| {
        Some(changed_link(view_change(rows, value)?))
    };
    // a value picked by name (`view.by`): answered `303` to the page
    // ordered by it as a head's link writes it, over the rows the heads
    // are drawn from; past a full list, to the search's own refusal of the
    // longer one. A page that draws no heads has no rows to order: refused.
    if let Some(by) = &by {
        return match page::heads_rows(&places, &windows) {
            Some(rows) => see_other(&changed_link(view_picked(rows, by)).href),
            None => {
                let failure = Failure::Usage(
                    "no open place on this page lists items, so no rows stand to be ordered \
                     by a value: open a tab or a character that holds items, then add the \
                     column"
                        .to_string(),
                );
                web.failed(title, &failure, &headers, &here)
            }
        };
    }
    let hidden_view: Vec<(&'static str, String)> = shown.view_fields();
    // an opened row's place: this page with it and its holders open, no
    // query, the card adding its landing
    let placed = |place: &acquisition_search::corpus::Place| {
        let kind = match listed {
            Listed::Tabs => "stash",
            Listed::Characters => "character",
        };
        if place.kind != kind
            || place.realm != realm.as_str()
            || place.league.as_deref() != Some(league.as_str())
        {
            return None;
        }
        let open: Vec<String> = places
            .holders(&place.id)?
            .into_iter()
            .map(str::to_string)
            .collect();
        Some(web.link(
            listed.route(),
            &place_fields(
                listed,
                realm,
                &league,
                &Shown::default(),
                Marked {
                    open: &open,
                    ..Marked::default()
                },
            ),
        ))
    };
    // where the query's other matches stand: each kind and league a link to
    // its place page under the same query and view, the characters with no
    // league words alone
    let elsewhere: Vec<Option<String>> = places
        .query
        .iter()
        .flat_map(|q| &q.elsewhere)
        .map(|e| {
            let there = match e.place.as_str() {
                "stash" => Listed::Tabs,
                _ => Listed::Characters,
            };
            e.league.as_deref().map(|league| {
                web.link(
                    there.route(),
                    &place_fields(
                        there,
                        realm,
                        league,
                        &Shown {
                            text: shown.text.clone(),
                            view: shown.view.clone(),
                            ..Shown::default()
                        },
                        Marked::default(),
                    ),
                )
            })
        })
        .collect();
    let body = page::places(&page::PlacesPage {
        listed: listed.route(),
        onward: here.onward(),
        title,
        tick: listed.tick(),
        realm,
        league: &league,
        leagues: leagues.as_deref(),
        places: &places,
        none: none.as_deref(),
        open: &open,
        ticked: &ticked,
        deep,
        filter: &shown.filter,
        text: shown.text.as_deref(),
        view_fields: &hidden_view,
        carried: &carried,
        never: &never,
        windows: &windows,
        view_links: &view_links,
        items: &kept,
        placed: &placed,
        elsewhere: &elsewhere,
        policy,
        spell: &spell,
        now: acquisition_store::now(),
    });
    // C16: the documents the page renders that carry a version, each its
    // own field — the places document's basis under a query, and a
    // segment's window's, which its route answers; one basis by `steady`
    // the finders' script where "Add a column" stands, beside the heads
    let mut head = Head {
        window: !windows.is_empty(),
        lists: page::heads_rows(&places, &windows).is_some(),
        ..Head::default()
    };
    if let Some(basis) = &places.basis {
        head = head.with("places", basis);
    }
    if let Some(basis) = places
        .every()
        .into_iter()
        .find_map(|p| windows.get(p.id()))
        .and_then(page::Window::basis)
    {
        head = head.with("rows", basis);
    }
    web.page_at(title, head, &body, &here)
}

/// The places a page can open: those that show something, their places
/// or their items.
fn openable(places: &Places) -> Vec<&str> {
    places
        .every()
        .into_iter()
        .filter(|p| p.holds.is_some() || p.request.is_some())
        .map(|p| p.id())
        .collect()
}

/// The ids a place page's URL names under `name` that `among` holds, each
/// once, in the URL's order: a name of no place of the league is dropped.
fn named_places(fields: &[(String, String)], name: &str, among: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (k, v) in fields {
        if k == name && among.contains(&v.as_str()) && !out.contains(v) {
            out.push(v.clone());
        }
    }
    out
}

/// Each open place the page draws that shows items, as segments of the
/// page's one window over `corpus`, every place asked under one list of
/// values, the union of what their answers carry (#296: the default
/// list depends on a place's matches, and the heads stand once above
/// the places): the token of its request's first
/// window, its count that window's own `total.matched`, cut after each
/// row `opening` opens there, the blocks the server's; none where
/// it holds none. Beside them, the opened rows some drawn place's order
/// holds, in the URL's order: the rest are dropped.
#[allow(clippy::type_complexity)]
fn windows_over(
    corpus: &acquisition_search::Corpus,
    places: &Places,
    open: &[String],
    opening: &Opening,
) -> Result<
    (
        std::collections::HashMap<String, (String, Vec<page::Segment>)>,
        Vec<String>,
    ),
    SearchError,
> {
    let mut windows = std::collections::HashMap::new();
    let mut held: Vec<String> = Vec::new();
    // each drawn place's table and its first row's answer
    let mut asked = Vec::new();
    for place in drawn_items(&places.places, open) {
        let Some(mut table) = place.request.clone() else {
            continue;
        };
        if let acquisition_search::answer::Shape::Rows(rows) = &mut table.view.shape {
            rows.fields = vec![acquisition_search::answer::RowField::Icon];
        }
        let first =
            acquisition_search::answer(corpus, &window_request(&table, &corpus.basis, 0, 1))?;
        if first.total.matched == 0 {
            continue;
        }
        asked.push((place, table, first));
    }
    // one list over every drawn place, as the one head above them says
    // it: the first's order, then what another adds at the end. A place
    // whose answer carries another list is asked again naming that one,
    // so its window and every block of it carry the same cells; the
    // page's own fields name none of it
    let mut list: Vec<String> = Vec::new();
    for (_, _, first) in &asked {
        for value in values_of(first) {
            if !list.contains(value) {
                list.push(value.clone());
            }
        }
    }
    for (place, mut table, mut first) in asked {
        if values_of(&first) != list.as_slice() {
            if let acquisition_search::answer::Shape::Rows(rows) = &mut table.view.shape {
                rows.values = list.clone();
            }
            first =
                acquisition_search::answer(corpus, &window_request(&table, &corpus.basis, 0, 1))?;
        }
        let count = first.total.matched;
        let (opened, at) = opening.held(corpus, &table)?;
        let rows: Vec<usize> = opened.iter().map(|(_, n)| *n).collect();
        held.extend(opened.into_iter().map(|(id, _)| id));
        let token = window_request(&table, &first.basis, 0, count.min(page::BLOCK_ROWS)).token();
        windows.insert(
            place.id().to_string(),
            (token, cut(corpus, &table, &first.basis, count, &rows, at)?),
        );
    }
    let kept = opening
        .items
        .iter()
        .filter(|id| held.contains(id))
        .cloned()
        .collect();
    Ok((windows, kept))
}

/// The values an answer's rows carry, none where it lists no rows.
fn values_of(answer: &acquisition_search::Answer) -> &[String] {
    match &answer.view {
        acquisition_search::answer::ViewOut::Rows(rows) => &rows.values,
        _ => &[],
    }
}

/// The open places a page draws that show their items: listed, every
/// place above them open, and holding no places of their own, in order.
fn drawn_items<'a>(
    places: &'a [acquisition_search::places::Place],
    open: &[String],
) -> Vec<&'a acquisition_search::places::Place> {
    let mut out = Vec::new();
    for place in places.iter().filter(|p| p.is_listed()) {
        if !open.iter().any(|o| o == place.id()) {
            continue;
        }
        if place.places.is_empty() {
            out.push(place);
        } else {
            out.extend(drawn_items(&place.places, open));
        }
    }
    out
}

/// What a place page's URL asks of its places beside the opens and the
/// ticks: the query's text, the filter, read as the places
/// document reads it, and the view every place's request takes.
#[derive(Clone, Default)]
struct Shown {
    text: Option<String>,
    filter: acquisition_search::places::Filter,
    view: acquisition_search::answer::Rows,
}

impl Shown {
    /// The fields of a place page's URL: a filter field the document
    /// refuses is the page's refusal.
    fn read(fields: &[(String, String)]) -> Result<Shown, Failure> {
        Ok(Shown {
            text: pick(fields, "text").map(str::to_string),
            filter: acquisition_search::places::Filter::read(
                pick(fields, "name"),
                pick(fields, "type"),
                pick(fields, "fetched"),
            )?,
            view: acquisition_search::answer::Rows {
                values: fields
                    .iter()
                    .filter(|(k, v)| k == "view.value" && !v.is_empty())
                    .map(|(_, v)| v.clone())
                    .collect(),
                sort: pick(fields, "view.sort").map(str::to_string),
                desc: pick(fields, "view.desc") == Some("true"),
                ..Default::default()
            },
        })
    }

    /// What the places document is asked.
    fn asking(&self) -> acquisition_search::places::Asking {
        acquisition_search::places::Asking {
            filter: self.filter.clone(),
            query: self.text.clone(),
            view: self.view.clone(),
        }
    }

    /// The view's fields, in their order: each value, the sort, the order.
    fn view_fields(&self) -> Vec<(&'static str, String)> {
        let mut fields: Vec<(&'static str, String)> = self
            .view
            .values
            .iter()
            .map(|v| ("view.value", v.clone()))
            .collect();
        fields.extend(self.view.sort.iter().map(|s| ("view.sort", s.clone())));
        if self.view.desc {
            fields.push(("view.desc", "true".to_string()));
        }
        fields
    }
}

/// What a place page's URL marks beside what it asks: the substashes
/// box, the open places, the opened rows and the ticks.
#[derive(Clone, Copy, Default)]
struct Marked<'a> {
    deep: bool,
    open: &'a [String],
    items: &'a [String],
    ticked: &'a [String],
}

/// A place page's URL fields, in the one order every link and redirect
/// writes them: realm, league, the query's text, the filter's fields, the
/// view's, the substashes box, each open place, each opened row, each
/// tick.
fn place_fields(
    listed: Listed,
    realm: Realm,
    league: &str,
    shown: &Shown,
    marked: Marked,
) -> Vec<(&'static str, String)> {
    let Marked {
        deep,
        open,
        items,
        ticked,
    } = marked;
    let mut fields = vec![
        ("realm", realm.as_str().to_string()),
        ("league", league.to_string()),
    ];
    fields.extend(shown.text.iter().map(|t| ("text", t.clone())));
    let filter = &shown.filter;
    fields.extend(filter.name.iter().map(|n| ("name", n.clone())));
    fields.extend(filter.r#type.iter().map(|t| ("type", t.clone())));
    if filter.fetched.is_some() {
        fields.push(("fetched", "never".to_string()));
    }
    fields.extend(shown.view_fields());
    if deep {
        fields.push(("deep", "true".to_string()));
    }
    fields.extend(open.iter().map(|o| ("open", o.clone())));
    fields.extend(items.iter().map(|i| ("item", i.clone())));
    fields.extend(ticked.iter().map(|t| (listed.tick(), t.clone())));
    fields
}

async fn list_tabs(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    list(web, headers, fields, Listed::Tabs).await
}

async fn list_characters(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    list(web, headers, fields, Listed::Characters).await
}

/// "List this league's stash tabs" and "List characters" (#284):
/// the `stashes` job for the form's realm and league, or the `characters`
/// job for its realm, as `acq stashes` and `acq characters` submit them,
/// through the human door as the served account, landing on the jobs
/// page as "Fetch the league list" does.
async fn list(
    web: Arc<Web>,
    headers: HeaderMap,
    fields: Vec<(String, String)>,
    listed: Listed,
) -> Response {
    // a step: it hands on the `from` it was given
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed(listed.title(), &failure, &headers, &Here::none()),
    };
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed(listed.title(), &failure, &headers, &here),
    };
    let league = pick(&fields, "league")
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    let (kind, params) = match listed {
        Listed::Tabs => (
            "stashes",
            json!({ "realm": realm.as_str(), "league": league }),
        ),
        Listed::Characters => ("characters", json!({ "realm": realm.as_str() })),
    };
    let submitted = async {
        let mut client = crate::run::connect(&web.cx).await?;
        let id = acquisition_verbs::jobs::submit(
            &web.cx.frontend,
            &mut client,
            kind,
            params,
            0,
            Some(web.served()?.entry.username.as_str()),
        )
        .await?;
        anyhow::Ok(id)
    }
    .await;
    match submitted {
        Ok(id) => to_job(&web, id, &here),
        Err(e) => web.failed(listed.title(), &Failure::Other(e), &headers, &here),
    }
}

/// "Fetch the league list": the `leagues`
/// job, as `acq leagues` submits it — one request, no probe — through the
/// human door and as the served account, landing on the jobs page with
/// the way back to the page it was fetched from. It takes no field but
/// `from`.
async fn fetch_leagues(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let here = match from::taken(fields) {
        Ok((fields, given)) => {
            let here = Here::step(given);
            if let Err(failure) = taken(&fields, &[], "the league list's fetch") {
                return web.failed("Leagues", &failure, &headers, &here);
            }
            here
        }
        Err(failure) => return web.failed("Leagues", &failure, &headers, &Here::none()),
    };
    let submitted = async {
        let mut client = crate::run::connect(&web.cx).await?;
        let id = acquisition_verbs::jobs::submit(
            &web.cx.frontend,
            &mut client,
            "leagues",
            json!({}),
            0,
            Some(web.served()?.entry.username.as_str()),
        )
        .await?;
        anyhow::Ok(id)
    }
    .await;
    match submitted {
        Ok(id) => to_job(&web, id, &here),
        Err(e) => web.failed("Leagues", &Failure::Other(e), &headers, &here),
    }
}

/// A submit lands at its job: `303` to the jobs page naming the job, at
/// its row, carrying the `from` the submit handed on, which the jobs page
/// draws as its way back. It decides nothing else: the way back is the
/// page the reader left, never read back out of the job.
fn to_job(web: &Web, id: u64, here: &Here) -> Response {
    web.landed(
        "jobs",
        &[("job", id.to_string())],
        Some(page::job_anchor(id)),
        here,
    )
}

/// The plan page: a selection's plan or the stored policy's,
/// rendered from its document ([`page::plan`]), the envelope
/// whole in a hidden field beside "Apply".
async fn plan(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    // a step: it hands on the `from` it was given, and draws it as its way
    // back, answered or refused
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Plan", &failure, &headers, &Here::none()),
    };
    let realm = match realm_of(&fields) {
        Ok(realm) => realm,
        Err(failure) => return web.failed("Plan", &failure, &headers, &here),
    };
    let league = pick(&fields, "league")
        .unwrap_or(acquisition_plan::DEFAULT_LEAGUE)
        .to_string();
    // "Refresh selected" (C44): the ticked tabs and characters
    // compiled into a selection plan; "Plan under the policy": the stored
    // policy's. Either lands here, and apply is the one apply route.
    let selection = pick(&fields, "selected").map(|_| {
        let named = |name: &str| -> Vec<String> {
            fields
                .iter()
                .filter(|(k, v)| k == name && !v.is_empty())
                .map(|(_, v)| v.clone())
                .collect()
        };
        acquisition_plan::PlanSelection {
            tabs: Some(acquisition_plan::Selection::Ids(named("tab")))
                .filter(|t| !matches!(t, acquisition_plan::Selection::Ids(ids) if ids.is_empty())),
            characters: named("character"),
            deep: pick(&fields, "deep").is_some(),
        }
    });
    if let Some(selection) = &selection
        && selection.tabs.is_none()
        && selection.characters.is_empty()
    {
        let failure = Failure::Other(anyhow::anyhow!(
            "nothing is selected: tick a tab or a character to refresh it"
        ));
        return web.failed("Plan", &failure, &headers, &here);
    }
    let account = match web.served() {
        Ok(account) => account,
        Err(fault) => return web.failed("Plan", &Failure::from(fault), &headers, &here),
    };
    let planned = async {
        let mut opened = account.open()?;
        match &selection {
            Some(selection) => opened.plan_selection(realm, &league, selection).await,
            None => opened.plan(realm, &league).await,
        }
    }
    .await;
    let planned = match planned {
        Ok(planned) => planned,
        Err(fault) => return web.failed("Plan", &Failure::from(fault), &headers, &here),
    };
    if wants_json(&headers) {
        return document(&planned);
    }
    // the instant the plan's ages were said at
    let now = planned.ages.at.unwrap_or_else(acquisition_store::now);
    let mut body = page::plan(&planned, now);
    if !planned.plan.actions.is_empty() {
        match serde_json::to_string(&planned.plan) {
            Ok(envelope) => body.push_str(&format!(
                "{}\n",
                page::post_form("apply", here.onward(), [("plan", envelope)], "", "Apply")
            )),
            Err(e) => body.push_str(&refusal(&e.to_string())),
        }
    }
    web.page_at(
        "Plan",
        Head::default().with("plan", &planned.plan.basis),
        &body,
        &here,
    )
}

/// "Apply": the envelope posted back whole to `Opened::apply`, as
/// `--apply=FILE` reads one. A moved policy is refused in the words C44
/// gives it, a spent plan sends nothing and says so (C111), and a
/// submitted one goes to the jobs page.
async fn apply(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    // a step: it hands on the `from` the plan page posted
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Apply", &failure, &headers, &Here::none()),
    };
    let applied = async {
        let text = pick(&fields, "plan").ok_or_else(|| {
            anyhow::anyhow!("no plan was posted: apply posts the envelope a plan page holds")
        })?;
        let value: Value = serde_json::from_str(text)?;
        let plan = RefreshPlan::from_value(&value)?;
        let mut opened = web.served()?.open()?;
        let mut slot = None;
        let applied = opened.apply(&plan, None, &mut slot).await;
        if let Some(client) = &slot {
            crate::note_spawn(client);
        }
        anyhow::Ok((applied?, plan))
    }
    .await;
    match applied {
        Ok((Apply::Submitted(submitted), _)) => to_job(&web, submitted.job_id, &here),
        Ok((done, _)) if wants_json(&headers) => document(&done),
        Ok((done, plan)) => web.page_at(
            "Apply",
            Head::default(),
            &page::applied(
                &done,
                &plan_again(&web, &plan, here.onward()),
                acquisition_store::now(),
            ),
            &here,
        ),
        Err(e) => web.failed("Apply", &Failure::Other(e), &headers, &here),
    }
}

/// The plan `plan` was compiled from, compiled again: its realm and league,
/// and the selection it names, ticked as the place pages send one; a link
/// to a step, carrying `from`.
fn plan_again(web: &Web, plan: &RefreshPlan, from: Option<&FromPage>) -> String {
    let mut fields = vec![
        ("realm", plan.realm.as_str().to_string()),
        ("league", plan.league.clone()),
    ];
    if let Some(selection) = &plan.basis.selection {
        fields.push(("selected", "yes".to_string()));
        if let Some(acquisition_plan::Selection::Ids(ids)) = &selection.tabs {
            fields.extend(ids.iter().map(|id| ("tab", id.clone())));
        }
        fields.extend(
            selection
                .characters
                .iter()
                .map(|id| ("character", id.clone())),
        );
        if selection.deep {
            fields.push(("deep", "true".to_string()));
        }
    }
    from::onward(&web.link("plan", &fields), from)
}

/// The jobs: the `jobs` document rendered in words
/// ([`page::jobs`]), reloading itself while a job is not done; `job`
/// names one job, read by id whether the list holds it or not, its
/// result shown where it failed or was cancelled; `from`, the page a
/// submit left, is its one way back. It observes and never spawns, and its
/// JSON is the `jobs` document whatever the URL names.
async fn jobs(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed("Jobs", &failure, &headers, &Here::none()),
    };
    let here = Here::place(FromPage::here(Place::Jobs, &fields), back);
    let opened = match pick(&fields, "job").map(|word| {
        word.parse::<u64>().map_err(|_| {
            Failure::Usage(format!(
                "`job` {word:?} is no job: it names one job by its number"
            ))
        })
    }) {
        None => None,
        Some(Ok(id)) => Some(id),
        Some(Err(failure)) => return web.failed("Jobs", &failure, &headers, &here),
    };
    let listed = match web.cx.frontend.jobs(ListState::All, false).await {
        Ok(listed) => listed,
        Err(fault) => return web.failed("Jobs", &Failure::from(fault), &headers, &here),
    };
    if wants_json(&headers) {
        return document(&listed);
    }
    let (running, rails) = match &listed {
        Jobs::Live(jobs) => {
            // the daemon's halts first, so a queue that stands still says
            // why
            let rails = match web.cx.frontend.daemon().await {
                Ok(acquisition_verbs::DaemonReport::Running { status, .. }) => match *status {
                    acquisition_protocol::protocol::Response::DaemonStatus { rails, .. } => {
                        Some(rails)
                    }
                    _ => None,
                },
                _ => None,
            };
            (
                jobs.counts.by_state.running + jobs.counts.by_state.waiting > 0,
                rails,
            )
        }
        Jobs::Absent { .. } => (false, None),
    };
    let opened = match opened {
        Some(id) => match opened_job(&web, &listed, id, here.onward()).await {
            Ok(opened) => Some(opened),
            Err(fault) => return web.failed("Jobs", &Failure::from(fault), &headers, &here),
        },
        None => None,
    };
    // with none served yet, every job's account is named
    let served = web
        .served()
        .map(|account| account.entry.username.clone())
        .unwrap_or_default();
    let spell = |way: &acquisition_verbs::refusal::Way| {
        use acquisition_verbs::refusal::Surface as _;
        Paged {
            web: &web,
            from: here.onward(),
        }
        .request(way)
        .and_then(|(link, _)| link.as_str().map(str::to_string))
    };
    let head = Head {
        refresh: running.then_some(JOBS_REFRESH),
        ..Head::default()
    };
    web.page_at(
        "Jobs",
        head,
        &page::jobs(&page::JobsPage {
            jobs: &listed,
            rails: rails.as_ref(),
            opened: opened.as_ref(),
            account: &served,
            spell: &spell,
            now: acquisition_store::now(),
        }),
        &here,
    )
}

/// The job `id` as the jobs page opens it: read by id where
/// the list leaves it out, and its result where it failed or was
/// cancelled, its offers spelled by [`Paged`].
async fn opened_job(
    web: &Web,
    listed: &Jobs,
    id: u64,
    from: Option<&FromPage>,
) -> Result<page::OpenedJob, Fault> {
    use acquisition_protocol::job::JobState;
    let row = match listed {
        Jobs::Live(jobs) => jobs.rows.iter().find(|j| j.id == id).map(|j| j.state),
        Jobs::Absent { persisted } => persisted
            .as_ref()
            .and_then(|p| p.jobs.iter().find(|j| j.id == id))
            .map(|_| JobState::Waiting),
    };
    let job = match row {
        Some(_) => None,
        None => Some(web.cx.frontend.job(id).await?),
    };
    let finished = match (&row, &job) {
        (Some(state), _) => matches!(state, JobState::Failed | JobState::Cancelled),
        (None, Some(acquisition_verbs::jobs::Job::Live(job))) => {
            matches!(job.state, JobState::Failed | JobState::Cancelled)
        }
        // the ledger holds a job's outcome once it is final; one with
        // none is unfinished, and has no result to show
        (None, Some(acquisition_verbs::jobs::Job::Absent { .. })) => true,
        (None, None) => false,
    };
    let result = match finished {
        true => match web.cx.frontend.result(id).await {
            Ok(result) => Some(result.offered(&Paged { web, from })),
            Err(Fault::Unfinished { .. }) => None,
            Err(fault) => return Err(fault),
        },
        false => None,
    };
    // a success read from the ledger has no result to show
    let result = result.filter(|r| {
        !matches!(
            r.outcome,
            acquisition_protocol::job::Outcome::Success { .. }
        )
    });
    Ok(page::OpenedJob { id, job, result })
}

// ---- the saved pages (C168) --------------------------------------------------------------------

/// A field a saved route was sent, present where the body or the query
/// names it, empty or not (the text `""` is a query).
fn sent<'a>(fields: &'a [(String, String)], name: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// The fields a saved route takes, or a usage refusal naming the first
/// it does not.
fn taken(fields: &[(String, String)], names: &[&str], route: &str) -> Result<(), Failure> {
    match fields.iter().find(|(k, _)| !names.contains(&k.as_str())) {
        Some((name, _)) => Err(Failure::Usage(format!(
            "`{name}` is no field of {route}: it takes {}",
            names
                .iter()
                .map(|n| format!("`{n}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ))),
        None => Ok(()),
    }
}

/// A sent `if_revision`, read as a whole number, or a usage refusal.
fn revision_sent(fields: &[(String, String)]) -> Result<Option<i64>, Failure> {
    sent(fields, "if_revision")
        .map(|word| {
            word.trim().parse::<i64>().map_err(|_| {
                Failure::Usage(format!(
                    "`if_revision` {word:?} is no revision: it names the revision written over \
                     as a whole number"
                ))
            })
        })
        .transpose()
}

/// The saved searches: ten, or every one under `expand=true`.
async fn saved_list(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let (fields, back) = match from::taken(fields) {
        Ok(taken) => taken,
        Err(failure) => return web.failed("Saved searches", &failure, &headers, &Here::none()),
    };
    let here = Here::place(FromPage::here(Place::Saved, &fields), back);
    let listed = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["expand"], "the saved searches' list")?;
        let expand = match sent(&fields, "expand") {
            None => false,
            Some("true") => true,
            Some(other) => {
                return Err(Failure::Usage(format!(
                    "`expand` {other:?} is no value it takes: `expand=true` lists every saved \
                     search"
                )));
            }
        };
        Ok(saved::list(&web.cx.frontend, None, expand)?)
    })
    .await;
    match listed {
        Ok(listed) if wants_json(&headers) => document(&listed),
        Ok(listed) => web.page_at(
            "Saved searches",
            Head::default().with("saved_searches", listed.revision),
            &page::saved_list(
                &listed,
                &web.saved_links(),
                &|id| {
                    from::onward(
                        &web.link("saved/open", &[("id", id.to_string())]),
                        here.onward(),
                    )
                },
                here.onward(),
            ),
            &here,
        ),
        Err(failure) => web.failed("Saved searches", &failure, &headers, &here),
    }
}

/// A saved search opened: `303` to the search its request folds
/// to, carrying its id and the revision it was opened at, and the `from`
/// it was given (a step). It writes nothing.
async fn saved_open(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Saved search", &failure, &headers, &Here::none()),
    };
    let opened = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id"], "a saved search's opening")?;
        let id = sent(&fields, "id").unwrap_or_default().to_string();
        if !saved::is_id(&id) {
            return Err(Failure::Usage(format!(
                "{id:?} is no saved search's id: a page opens a saved search by its id, twelve \
                 hex characters, as the saved searches list it"
            )));
        }
        let row = match web.saved_at(&id)? {
            // a source this build cannot read or translate: listed, and
            // refused when opened, in the page's words beside the verbs'
            Some(SavedRow::Read(row)) if row.fault.is_some() => {
                return Err(listed(
                    "refused",
                    format!(
                        "saved search {} at revision {} asks what this build cannot read ({}); \
                         a page cannot open it",
                        row.id,
                        row.revision,
                        row.fault.as_deref().unwrap_or_default()
                    ),
                ));
            }
            Some(SavedRow::Read(row)) => row,
            // a page refuses from the row it read, never from a second
            // read: the words are the page's, beside the verbs' own in
            // `saved::request_of`, whose maker is private
            Some(SavedRow::Unreadable(row)) => {
                return Err(listed(
                    "refused",
                    format!(
                        "saved search {} at revision {} holds a value this build cannot read \
                         ({}); a page cannot open it",
                        row.id, row.revision, row.unreadable
                    ),
                ));
            }
            None => {
                return Err(listed(
                    "saved_search_not_found",
                    format!("no saved search has the id {id}"),
                ));
            }
        };
        // a text or a site search opens as the page asks one (C194), and
        // the page fills its editor from it
        let Some(mut asked) = web.asked_by_the_form(&row.request) else {
            return Err(Failure::Other(anyhow::anyhow!(
                "saved search {id} is no search a page can ask: open it with an agent's search \
                 or the terminal's"
            )));
        };
        asked.push(("saved", id));
        asked.push(("saved_revision", row.revision.to_string()));
        Ok(asked)
    })
    .await;
    match opened {
        Ok(asked) => web.landed("search", &asked, None, &here),
        Err(failure) => web.failed("Saved search", &failure, &headers, &here),
    }
}

/// A saved route's refusal of the row it read, in the page's words, the
/// list its one way on.
fn listed(kind: &'static str, message: String) -> Failure {
    Failure::Other(
        Pointed {
            kind,
            message,
            ways: vec![(
                Way::SavedSearches { expand: false },
                "the saved searches, each with its id".to_string(),
            )],
            fields: serde_json::Map::new(),
        }
        .into(),
    )
}

/// A saved search written: the verbs' one `Set`, its revision
/// under `Expect::or(if_revision, Expect::Create)`.
async fn saved_set(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    // a step: it hands on the `from` the page posted
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Saved searches", &failure, &headers, &Here::none()),
    };
    let as_posted = fields.clone();
    let written = blocking(&web, move |web| -> Result<_, Failure> {
        taken(
            &fields,
            &[
                "id",
                "if_revision",
                "name",
                "text",
                "trade",
                "realm",
                "membership",
            ],
            "a saved search's write",
        )?;
        let field = |name: &str| sent(&fields, name).map(str::to_string);
        // the source, one of the two (C194): a site search as sent, which
        // the verbs store as given, a JSON text as its object
        let query = match (field("text"), field("trade")) {
            (Some(_), Some(_)) => {
                return Err(Failure::Usage(
                    "a saved search's write sends `text` or `trade`, never both: a search is a \
                     query or a site search"
                        .to_string(),
                ));
            }
            (Some(text), None) => Some(SavedQuery::Text(text)),
            (None, Some(trade)) => Some(SavedQuery::Trade(Value::String(trade))),
            (None, None) => None,
        };
        let set = saved::Set {
            id: field("id"),
            expect: Expect::or(revision_sent(&fields)?, Expect::Create),
            name: field("name"),
            query,
            realm: field("realm"),
            membership: field("membership"),
        };
        let account = web.served()?.entry.username.clone();
        Ok(saved::set(
            &web.cx.frontend,
            Some(&account),
            &set,
            &mut saved::mint,
        )?)
    })
    .await;
    web.written(written, &headers, &here, Some(&as_posted))
}

/// A clear asked first (the owner: "Clear should ask first"): the row at
/// the URL's id, shown with the clear's form at its revision, its basis
/// keyed `saved_search`. It writes nothing, so a reload of it writes
/// nothing. An id no row has is refused from the row in hand, offering the
/// list, as an opening is.
async fn saved_ask(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Query(fields): Query<Vec<(String, String)>>,
) -> Response {
    // a step: it hands on the `from` it was given to the clear's form
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Saved search", &failure, &headers, &Here::none()),
    };
    let asked = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id"], "a saved search's clear, asked first")?;
        let id = sent(&fields, "id").unwrap_or_default().to_string();
        if !saved::is_id(&id) {
            return Err(Failure::Usage(format!(
                "{id:?} is no saved search's id: a page clears a saved search by its id, twelve \
                 hex characters, as the saved searches list it"
            )));
        }
        match web.saved_at(&id)? {
            Some(row) => Ok(row),
            None => Err(listed(
                "saved_search_not_found",
                format!("no saved search has the id {id}"),
            )),
        }
    })
    .await;
    match asked {
        Ok(row) if wants_json(&headers) => document(&row),
        Ok(row) => {
            let doc = serde_json::to_value(&row).unwrap_or_default();
            web.page_at(
                "Clear a saved search",
                Head::default().with("saved_search", row.revision()),
                &page::saved_ask(&doc, &web.saved_links(), here.onward()),
                &here,
            )
        }
        Err(failure) => web.failed("Saved search", &failure, &headers, &here),
    }
}

/// A saved search cleared at exactly the revision named.
async fn saved_clear(
    State(web): State<Arc<Web>>,
    headers: HeaderMap,
    Form(fields): Form<Vec<(String, String)>>,
) -> Response {
    let (fields, here) = match from::taken(fields) {
        Ok((fields, given)) => (fields, Here::step(given)),
        Err(failure) => return web.failed("Saved searches", &failure, &headers, &Here::none()),
    };
    let written = blocking(&web, move |web| -> Result<_, Failure> {
        taken(&fields, &["id", "if_revision"], "a saved search's clear")?;
        let id = sent(&fields, "id")
            .ok_or_else(|| Failure::Usage("a clear names the saved search's `id`".to_string()))?;
        let revision = revision_sent(&fields)?.ok_or_else(|| {
            Failure::Usage("a clear names the revision it clears, `if_revision`".to_string())
        })?;
        Ok(saved::clear(
            &web.cx.frontend,
            id,
            Expect::Exactly(revision),
        )?)
    })
    .await;
    web.written(written, &headers, &here, None)
}

impl Web {
    /// A write's answer (the owner: "Redirect to the list"): the
    /// receipt where JSON is asked, its undo spelled as a page's form;
    /// else landed on the list with the way back to the page the write
    /// left. Refused, the refusal, then the save form as `sent`, where the
    /// write was a save and the refusal offers no write of its own (a
    /// conflict's offer is the write as sent, re-aimed), then the way back.
    fn written(
        &self,
        written: Result<saved::SavedWritten, Failure>,
        headers: &HeaderMap,
        here: &Here,
        sent: Option<&Fields>,
    ) -> Response {
        match written {
            Ok(written) if wants_json(headers) => document(&acquisition_verbs::receipt::saved(
                &written,
                &Paged {
                    web: self,
                    from: None,
                },
            )),
            Ok(_) => self.landed("saved", &[], None, here),
            Err(failure) => {
                let offers_a_write = self.refusal(&failure, None)["offers"]
                    .as_array()
                    .is_some_and(|offers| {
                        offers.iter().any(|o| o["request"].get("post").is_some())
                    });
                let form = match sent {
                    Some(sent) if !offers_a_write => as_sent(self, sent, here),
                    _ => String::new(),
                };
                self.failed_as_sent("Saved searches", &failure, headers, here, &form)
            }
        }
    }
}

/// A save's form as `sent`, by the save form's one maker: its source,
/// scope and row hidden, its name in the box as sent, posting `from`.
fn as_sent(web: &Web, sent: &Fields, here: &Here) -> String {
    let hidden = sent
        .iter()
        .filter(|(k, _)| k != "name")
        .map(|(k, v)| (k.as_str(), v.clone()));
    format!(
        "<div class=\"save\">\n{}\n</div>\n",
        page::save_form(
            &web.saved_links().write,
            here.onward(),
            hidden,
            sent_name(sent),
            "Save",
        )
    )
}

fn sent_name(sent: &Fields) -> &str {
    sent.iter()
        .find(|(k, _)| k == "name")
        .map(|(_, v)| v.as_str())
        .unwrap_or_default()
}

/// `303 See Other` to `location`, an absolute path under the token.
fn see_other(location: &str) -> Response {
    let mut response = StatusCode::SEE_OTHER.into_response();
    if let Ok(location) = HeaderValue::from_str(location) {
        response.headers_mut().insert(header::LOCATION, location);
    }
    response
}

async fn css() -> Response {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], CSS).into_response()
}

async fn lists_script() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        LISTS,
    )
        .into_response()
}

async fn window_script() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        WINDOW,
    )
        .into_response()
}

async fn count_script() -> Response {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        COUNT,
    )
        .into_response()
}
