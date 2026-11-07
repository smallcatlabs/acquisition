# 4-full-1 — Morning 1: the page, the saved search and the handoff

A Fable design, 2026-11-06, part 1 of `4-full-design-prompt.md`, at
the owner's word of that day (C16 amended, C167, the two parks; the
timeline disregarded). Nothing here is built or ruled: §8 is drafts
for the owner, and the build briefs follow his word. Read-only at
`7a72de85`; nothing was run, no mock, no `acq`, no subagent. The
prompt file was printed whole, so parts 2 and 3 were seen; nothing of
them is designed here. Cut once, 2026-11-06, under the cold review at
`2db2a07d` (`roadmap/reviews/4-full-1-design-2db2a07d.md`): the foot
says what each finding changed, and three questions the orchestrating
session put apart from the review are answered where they fall (§2,
§3).

What was read: the prompt's head and part 1, its critique,
`4-0-design.md` (the form), `GLOSSARY.md`, `roadmap/README.md`,
`roadmap/TEST-SHAPE.md` §2–§3, `GUI-NOTES.md` §4, §6, §7, `4a-design.md`,
`4b-build.md`, the night run of 2026-11-04; the search, store,
frontends, trade, plans and pricing rulings whole; the web module and
`acquisition-cli/tests/web.rs` with its goldens; the search crate's
`answer.rs`, `show.rs`, `derive.rs`, `corpus.rs`; the store's
`annotations.rs` and `world.rs`; the verbs' `frontend.rs`, `refusal.rs`,
`receipt.rs`; the MCP's module doc and tools; the two references;
`tools/arch-check.sh`'s door grep; issues #42, #98, #176, #19, #159 and
the open list; for the cut, `search_json.rs`, `surface_size.rs`,
`search_cmd.rs`'s routes and offers, `search/MEASUREMENTS.md`.

## 1. What the tree says

**The page is already its URL.** `acq web`'s search is a GET whose
query string is the form's own fields, folded by the server into the
trade envelope `Asked::read` reads (`web/mod.rs`, `search`; `form.rs`,
`fold`); a continuation is `search?next=<token>`, and the token is the
request with its basis (`Request::token`). The one script is
the picker, 49 lines, which fetches `stats?q=`, keeps a counter of asks
in flight, and writes an id into an empty row (`assets/picker.js`); the
content policy admits no other. No page stores anything between
navigations. C16's state sentence is true of 4-lite by its size, and
pinned by nothing. The query box resubmits the realm and the text alone
(`page.rs`, `query_box`); the fold reads neither `membership` nor
`view.fields` (`form.rs`, `emitted`).

**The pages print the CLI's text.** `page::answer` renders the head
and the tail through `search_cmd::answer_head` and `answer_tail`, and
`page::item` through `shown_text`; the search-pages golden carries four
`acq …` command lines (`goldens/web/search_pages.txt`). The CLI's
answer carries more requests than the terms' routes (`search_cmd.rs`;
§4 lists them). The refusals are already the page's own: `Paged`
spells a `Way` as a link where a page takes it, through
`asked_by_the_form`, which folds a request back to the form's fields
and offers nothing it cannot fold (#97's build). The harness records a
page's `<main>` alone (`harness/mod.rs`, `Pages::page`).

**Rows name fields, one so far.** `Rows { limit, sort, desc, after,
fields }` with `RowField::Icon`, made where `show` makes the icon
(`derive.rs`, `Item::icon`; `rows.rs`, the `c140_` test). The CLI
refuses `--fields` as not built, pinned twice in `search_json.rs`.
`Held { item, place, class, category, price }` is what a corpus holds
per item; `Shown` is the same five with the basis, the lines with
their slots, `against` and the body; `read_one` builds a `Shown` from
a raw row, so the maker is one function the rows cannot yet call.

**The basis is the version, and the holder checks it.** `Basis` names
the store, the account, the build, the facts revision, every table and
the intent revision (`corpus.rs`); `Holder::corpus` compares it before
every answer and reloads whole when it moved; a continuation across a
change is refused (C98, C100). The intent revision is a sum over every
annotation row, which is why C167 puts the saved search in a file of
its own.

**The store's shape for intent is generic.** `Annotations::open(path)`
opens any file, the uuid optional metadata; `open_for(dir, uuid)` names
the path and binds the uuid, and `open_read_only_for` requires one.
`put` runs the kind's strict parse (`IntentValue`, C66), the writer's
rules, then compare-and-swap; `delete` names an exact revision and
tombstones; `list` names an unreadable row (C151); `revision` is one
statement; `export` is a snapshot. The `c35_` tests pin the file's
mechanics: the opening race, a foreign writer, synchronous mode. A
world's durable state sits at `<root>/<provider>/` beside the account
files (`world.rs`), and `Frontend::dir` is that directory.

**The frontends share one shape per write**: the receipt (`receipt.rs`:
`written`, `prior`, `revision`, `undo` or `no_undo`) and the refusal
with `offers` each surface spells (`refusal.rs`). The MCP opens per
call, selects the account by `scope.account` alone, takes `if_revision`
on writes (C52), refuses unknown arguments, and its first turn is held
to 38,000 bytes, measured at 37,824 (`surface_size.rs`).

## 2. The page: what it holds, what it may not, the one feed

**Two kinds of state.** Document state is what a page shows: the
documents it rendered, each with the revisions it was made at, and the
URL that names them; the server rebuilds all of it from the URL, which
is C16's sentence. Transient state is what a page holds inside one life
and a reload discards: unsent input, the picker's in-flight counter and
list. It never labels a document, never outlives a navigation and is
never a version. §8 offers the owner this reading of "state".

**What a page may not hold:** browser storage, a cookie, `pushState`,
rows that outlive a navigation, a sequence number of its own, a value
computed from a document. A presentation choice (cards or the table, a
sort) is a URL field; `next` is the request whole.

**Identity.** `search?<the form's fields>` and `search?next=<token>`
stay the one answer route; `item/{id}` and `saved` (§3) join `/`,
`refresh`, `plan`, `jobs`. A page field that is not a request field
(`saved=<name>`) is read apart from the fold; the fold itself gains
`membership` and `view.fields`, and `asked_by_the_form` folds both
back, so its equality holds over every field a page can ask.

**Every document with its version, copied, never composed.** Each
page carries in its head one `meta` line holding the version of each
document it rendered as that document serializes it: the answer's and
the item's `basis` object as serde writes it, the bytes the JSON
answer carries; the form page's league list under the facts revision
the document names; the saved list's `revision` as the list document
answers it (C167). The page composes no number and summarises nothing:
the line is the document's own version, so it is the basis and not a
second one. That line is what part 3's feed compares. A page re-asks
by loading its URL; the feed, when it comes, says which revision
moved. A page built so needs no change when the feed arrives.

**What pins it.** The goldens hold every page and every document
(`web.rs`, `page_golden`, the JSON equality with the CLI); step 3
widens `Pages::page` to record the head's `meta` line beside `<main>`,
and a test compares that line with the `basis` of the document the
same URL answers as JSON. The no-state scan, step 3: a test reads each
served script for the names of the storage, cookie and history APIs
and refuses one. What it holds: that no served script names those
APIs, a lexical property of the bytes the server serves. What it
cannot hold: a
cache kept in a closure or the DOM across fetches, state the URL omits,
a generation a script keeps for itself; those are behaviour, and stay
unpinned until the browser-driven check exists, with a review-checklist
row until then. That check is for a presentation rule inside a script
(the picker's one today, the virtualiser's window at step 6), and the
frontends park holds it until the second rule: step 6 is the second
rule, and its spike chooses the driver.

**The dense table and the toolchain.** Step 6, after the owner's
morning: a server-windowed rows view keyed by item id (`GUI-NOTES.md`
§6; `after` and `limit` exist, and a window past a moved basis is
refused, which is the honest answer), a vendored, pinned virtualiser
file first, the browser-driven check with it. Nothing more here.

## 3. The saved search

**The kind** (C167, as ruled): scope `search`, key the name, kind
`saved-search`, value `{version: 1, text, scope: {realm, membership}}`.
The text is the canonical text (C104), never a site URL: a trade search
saved is its translation's text. The scope never names an account. The
realm is always present: a save naming none over a store of several is
C96's refusal. A name is one to sixty-four characters after trimming,
no line break, compared exactly; a name is only ever a query value or
a form field, never a path segment, so no name collides with a route.
The view (sort, limit, fields) is the URL's, not the row's (§9).

**The store**: `acquisition-store/src/user.rs`, a `User` newtype over
`Annotations::open` at `<dir>/user.db`, no uuid bound: `open_for(dir)`,
`open_read_only_for(dir)` over an unbound read-only opener factored
from the account one, `open_memory()`, and `put`, `get`, `list`,
`delete`, `revision`, `export` delegated; `list` answers the rows and
the file's revision from one transaction. `SavedSearch` is the kind
(`IntentValue`, version 1, shape alone: the store links no language).
A newtype, so a signature taking `&Annotations` cannot be handed the
user file. No rename door.
The tests: the file's mechanics stay pinned once by the `c35_` tests,
since `User` opens through the one `open`; the API's properties
(compare-and-swap, the tombstone's ABA guard, the revision sum,
provenance, an unreadable row named, the export) become one table in
`unit/tests/annotations.rs` run over an opener for each file, and
`c167_` pins only what the newtype adds: the path, the kind, the scope
word, the one-transaction list. Settled here, not raised (C153).

**The verbs** (`acquisition-verbs/src/saved.rs`): `Frontend::user`
opens the world's file (the door grep gains `User::open_for`); `list`
answers one object, `searches` counted, each row with `name`, `text`,
`scope`, `revision`, `written_via`, `actor`, its times with their words
(C163), and `request`, the search request it is; `set` reads the text
with the language's reader in the row's realm, stores the canonical
text, answers the receipt; `clear` the receipt with `prior`;
`request_of(name)` is what every `--saved` resolves through. `rename`
is two compare-and-swaps in order, each refusable, and its receipt says
which landed: the new name create-only (a name standing refuses before
anything is written), then the old cleared at the revision read; where
the clear is refused the receipt is partial, `written` the new row,
`prior` the old as read, `clear` the offer to clear the old at its
current revision, and `no_undo` saying both names stand; where both
landed, `undo` is the reverse rename at the new row's revision. A name
no row holds is `not_found`, offering the list; a stale `if_revision`
is `conflict` with `current` and the write re-aimed. `Way` gains the
list, the set, the clear and the rename, so every surface spells each
offer and each undo.

**The CLI**: `acq saved` with `list` (the default), `show NAME`,
`set NAME QUERY [--realm] [--membership] [--if-revision]`,
`rename OLD NEW [--if-revision]`, `clear NAME [--if-revision]`,
`--if-revision` meaning what `policy set`'s means; `acq search --saved
NAME` runs it, `--account` and view flags beside it, no word of query,
realm or membership (the row's), as `--next` takes none.

**The MCP**: `saved_searches` (`name` for one), `saved_search_set`
(`name`, `text`, `realm`, `membership`, `if_revision`, required where a
row stands: C52's create-only form), `saved_search_rename`,
`saved_search_clear`, each taking `account` as the pricing tools do;
and `search` takes `saved` with `scope.account` and `view` beside it,
refusing `query`, `scope.realm` and `scope.membership` beside it. Each
answers the CLI's `--json` document, pinned as `pricing_tools.rs` pins.

**The pages.** `saved`: the list, each name a link that opens it, a
rename and a clear form beside each row, the file's revision in the
head. `saved/open?name=…`: a GET that writes nothing and redirects to
`search?realm=…&membership=…&text=…&saved=<name>`. The answer page
gains a save form, the text and scope hidden; the query box carries
the realm, the membership, `saved` and the revision shown, so the loop
survives an edit: with `saved=<name>` the page says the name and its
revision and the form replaces at it; where the row's text no longer
equals the page's query, or the name is gone, it says so. `POST
saved`, `saved/rename`, `saved/clear`: the write, then a redirect to
the list; a conflict renders the refusal page with the re-aimed write
as a form, since a way on that writes is a form where one that reads
is a link. Mutation by POST alone under the token (C138).

**The handoff, both ways, with no new channel.** The agent saves; the
owner opens or reloads the list and it is there, `written_via` saying
the channel. The owner saves; the agent lists and gets the request to
run. Neither learns the other's door: the agent never sees the web URL
or its token, the owner never drives the MCP. The basis on every
answer is what each quotes to the other. What stops the agent saying
something wrong: a name that exists refuses with its row; an edited
text conflicts with the current row; the realm is stored and printed
back; a later build's text is listed, named and refused only when run
(C151, inherited).

## 4. Cards and the pages' own ways on

**Rows that carry every field** (#98's "Closes when"): `RowField::Item`
puts on each row what `Shown` holds of the item: the item without its
lines, the lines with their slots, `class`, `category`, `price`. The
step factors the `Held`-to-`Shown` projection out of `read_one` into
one function `show` and the rows both call, pinned by equality in the
search crate (`rows.rs`: a row's `item` equals `show`'s for the same
id at the same basis). The CLI's `--fields icon,item` is built, its
text each row as `show` prints one. Under `item` the rows view has a
row cap, set by step 5's spike, and the rest is `next`; a single item
is never cut (C53, what an answer is for), and the byte ceiling is the
frame's (C166), whose refusal stands where a capped page would still
pass it, never a page's own.

**The card and the item page.** The answer page asks
`fields: [icon, item]` by default and renders a card per row in the
site's order (name, base, properties, requirements, lines by source
with their flags, sockets, price, place with its age);
`view.fields=icon` in the URL is the compact table. The item page
renders `Shown` by the same card.

**Ways on through `Paged`** (#176's "Closes when"): the answer page
renders `Answer` itself, and every request the document carries is
inventoried: the terms' routes, a count bucket's, a removed id's, a
value list's rest, the zero answer's explain and corpus offers, an
ambiguous clause's other reading, `next`. Each is a link where
`asked_by_the_form` folds it and the count or the words alone where no
page takes it (explain and corpus, which the form does not ask). The
scope block's never-fetched list links the league's tabs page
(`Way::Tabs`); the item page's price line has no link. The cost: two renderers of one document, text and HTML, each with
its golden; the labels are presentation, the words the document's.

## 5. What must exist for morning 1, and what can wait

Must exist: steps 1 to 5 of §7. The owner then sits with an agent, in
his store, under the live-run skill, as 4b's Do 10.

Can wait: the dense table (step 6); a sticky default realm and league
(#19: the saved search carries both; a default kind is added when first
written); describe's derived groups (#159: no surface here types a
group); a saved view; tags; any feed (a reload is the liveness); price
writes and the policy page (part 3); a login from a page (parked).
Group 4, feedback, is out of this part.

## 6. What not to do

- `search?saved=<name>` running the search by name, or a name in a
  path: the answer page would depend on a mutable name; open redirects.
- A `saved` spelling in `QueryInput`: the search crate opens no file
  (C89); the name resolves in the verbs.
- The saved search in the annotation file: ruled out (C167).
- Cards composed from `Held.item` in the page, or a table that sorts:
  two makers (C46).
- A feed, SSE or a meta-refresh on the saved list: part 3; a reload.
- Handing the agent the web URL: the token is the human door (C138).

## 7. The build

From the least uncertain to the bet; each step one brief from
`TEMPLATE.md`, in series on the shared checkout, every test obeying
C137, no new seed function (#142's rule).

1. **The file and its first consumer**: `user.rs`, `SavedSearch`, the
   unbound read-only opener, the one table over both files and the
   `c167_` pins; `saved.rs` with `Frontend::user`, the door grep
   widened; `acq saved` and `--saved`; the references and the tour
   line regenerated. Consumer: the owner at a terminal. Closes #42 by
   its "Fire" line. Retires nothing.
2. **The MCP**: the four tools and `search.saved`; each answer equal to
   the CLI's document; the first turn measured in the step's first
   commit and held under `surface_size.rs`'s 38,000 bytes, the tools'
   texts cut to fit, or a measured ruling brought to the owner before
   the close. Consumer: the agent in morning 1 (a mock seat first).
3. **The saved pages, the save form and the fold's new fields**, under
   §8's first draft: the routes, `Pages::page` recording the `meta`
   line, the goldens, the no-state scan, the meta equality (C16's
   Pinned line filled at the close). Consumer: the owner.
4. **The answer and item pages from their documents** (#176): the
   renderer over the inventory of §4, the two goldens regenerated.
   Closes #176. Retires: the four `acq …` lines of `search_pages.txt`;
   held by that golden regenerated, which holds the links in their
   place.
5. **Cards** (#98), the spike first: `fields: [item]` at limits of 20,
   200 and 1,000 over the measurement copy of the owner's store
   (`search/MEASUREMENTS.md`, 22,721 items, read under `ACQ_STORE_DIR`
   and `ACQ_NO_SPAWN=1` by his grant of 2026-10-19), bytes and
   milliseconds, which set the row cap; then
   `RowField::Item`, the one maker factored, the CLI's `--fields`, the
   card renderer. Closes #98. Retires: `search_json.rs`'s two
   `not_built` assertions on `--fields`; held by a CLI equality case
   there, `--fields icon,item --json` equal to the request's answer.
6. **The dense table**, after the owner's morning: the server-windowed
   rows view, a vendored, pinned virtualiser file, the browser-driven
   check. Spike first, one day: the file over the measurement copy's
   rows in the browsers the owner uses, scroll frame time and DOM size;
   and the check's driver, a headless browser from a Rust test with no
   Node, or a CI-only job, measured by what it adds to the gate. Its
   answer shapes the step's brief or comes to the owner with two shapes.

Mechanism bets: the item field's size (step 5's spike); the virtualiser
without a build step and the check's driver (step 6's).

## 8. Drafts for the owner

Each is his; nothing is entered until he rules.

- **C<next> — A page writes user-scoped intent as the verbs write
  it.** A page writes only through the verbs' door (C46, every
  frontend an adapter), by POST under the listener's token (C138, the
  listener), naming in its form the revision it replaces as the plan
  page carries its envelope (C38, plans are binding), and answers the
  one receipt (C78, a write returns its prior) or the one refusal (C11,
  the failure document); the first kind a page writes is the saved
  search (C167, user-scoped intent); an annotation (a price, the
  policy) is written from no page until its page is designed. *Why:* a
  browser is where a stale view could clobber, and the revision in the
  form makes staleness a refusal, as C52 (no blind replace) does for an
  agent. *Pinned:* `acquisition-cli/tests/web.rs`, the `c<next>_` tests
  over the saved pages. Recommendation: enter as drafted, before step 3.
- **C16, the word "state"** (a clarification, no amendment
  recommended): the sentence binds what survives a navigation; unsent
  input and a script's in-flight work are discarded by a reload and
  label nothing. Recommendation: the mechanism's doc comment carries
  it; his word on whether the sentence gains the words.
- **The web's routes** (new routes are his under C138): `saved` GET
  and POST, `saved/open` GET (a redirect, no write), `saved/rename`
  POST, `saved/clear` POST; a name only ever a query value.
  Recommendation: as listed.
- **`user.db`'s place**: beside the account files, where `world.rs`
  already names the world's durable state; no amendment to C167, the
  path in the module doc. For his word, since the file is in his store.
- **The spike's home for the browser-driven check** (the gate, or
  CI): no draft now; step 6's spike brings the measured shapes.

Agent-facing design, the session's, for his review: the verb and tool
names; the kind's shape and the name rule; the canonical text saved,
never a URL; the view left to the URL; rename as two writes with a
partial receipt; `saved=` as a page field; the fold's new fields; cards
as the default; a way on that writes as a form.

## 9. Raised for review

- The view in the saved row: the owner's morning says whether "rings
  by life" wants its sort saved; a `view` field is the kind's version 2,
  upgraded in memory (C66).
- The web's tests seed a private world (`web.rs`, `seed`), built before
  #142's rule; this part adds no seed, and t6 folds the one there is.
- For part 2: `search --saved` on the MCP opens a third file per
  call; the saved list is per world, so a logout changes nothing there.
- For part 3: the feed line names the user revision beside the basis,
  since the saved list's version is it; a page's `meta` line is what
  the feed is compared with; the resident must see a user-file write
  without invalidating the search basis, however it holds the file;
  the price and policy pages extend §8's first draft by naming their
  kinds; a continuation refused across a moved basis is what makes a
  scrolled window honest.

## Taken from the review (`2db2a07d`, cut 2026-11-06)

1 rename create-only, the clear at the revision read, a partial receipt,
`Way` gains clear and rename (§3); 2 two kinds of state, the meta line
copied never composed, a C16 reading offered (§2, §8); 3 the query box
and the fold carry membership, `saved`, the revision and `view.fields`
(§2, §3); 4 a name is a query value, `saved/open` (§3, §8); 5
`scope.account` beside `saved` (§3); 6 `Pages::page` records the meta
line, the meta equality, the list and its revision in one transaction
(§2, §3, §7); 7 the scan a lexical guard with what it cannot hold (§2);
8 every request the answer carries inventoried (§4); 9 the spike first
over the measurement copy, a row cap apart from the frame's ceiling
(§4, §7); 10 the 38,000-byte bound in step 2 (§7); 11 the retirements
name the `search_json.rs` assertions and a CLI holder (§7); 12
`open_for` binds the uuid, an unbound read-only opener (§1, §3); 13
glosses and a named pin, the C167 amendment dropped (§8); 14 the
resident line raises the requirement alone (§9).
