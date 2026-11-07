# 4-full-1 — Morning 1: the page, the saved search and the handoff

A Fable design, 2026-11-06, part 1 of `4-full-design-prompt.md`, at
the owner's word of that day (C16 amended, C167, the two parks; the
timeline disregarded). Nothing here is built or ruled: §8 is drafts
for the owner, and the build briefs follow his word. Read-only at
`7a72de85`; nothing was run, no mock, no `acq`, no subagent. The
prompt file was printed whole, so parts 2 and 3 were seen; nothing of
them is designed here, and what this part found for them is in §9.

What was read: the prompt's head and part 1, its critique,
`4-0-design.md` (the form), `GLOSSARY.md`, `roadmap/README.md`,
`roadmap/TEST-SHAPE.md` §2–§3, `GUI-NOTES.md` §4, §6, §7, `4a-design.md`,
`4b-build.md`, the night run of 2026-11-04; `decisions/search.md`,
`store.md`, `frontends.md`, `trade.md`, `plans.md`, `pricing.md` whole;
the web module (`web/mod.rs`, `page.rs`, `form.rs`, the three assets)
and `acquisition-cli/tests/web.rs` with its goldens; the search crate's
`answer.rs`, `show.rs`, `derive.rs`, `corpus.rs` (`Basis`,
`Corpus::is_current`, `Holder`); the store's `annotations.rs` and
`world.rs`; the verbs' `frontend.rs`, `refusal.rs`, `receipt.rs`; the
MCP's module doc and its `search`, `show`, `describe` and pricing tools;
`CLI-REFERENCE.md` and `MCP-REFERENCE.md`; `tools/arch-check.sh`'s door
grep; issues #42, #98, #176, #19, #159 and the open list.

## 1. What the tree says

**The page is already its URL.** `acq web`'s search is a GET whose
query string is the form's own fields, folded by the server into the
trade envelope `Asked::read` reads (`web/mod.rs`, `search`; `form.rs`,
`fold`); a continuation is `search?next=<token>`, and the token is the
request with its basis (`Request::token`). The item page is
`item/{id}`. The one script is the picker, 49 lines, which fetches
`stats?q=` and writes an id into an empty row (`assets/picker.js`); the
content policy admits no other script. No page stores anything: no
cookie, no browser storage, no history call. C16's state sentence is
true of 4-lite by accident of its size, and pinned by nothing.

**The pages print the CLI's text.** `page::answer` renders the head
and the tail through `search_cmd::answer_head` and `answer_tail`, and
`page::item` through `shown_text`; the search-pages golden carries four
`acq …` command lines (`goldens/web/search_pages.txt`, the routes and
the price line). The refusals are already the page's own: `Paged`
spells a `Way` as a link where a page takes it, through
`asked_by_the_form`, which folds a request back to the form's fields
and offers nothing it cannot fold (#97's build). That is the machinery
#176 needs, built for refusals and not yet applied to answers.

**Rows name fields, one so far.** `Rows { limit, sort, desc, after,
fields }` with `RowField::Icon`, made where `show` makes the icon
(`derive.rs`, `Item::icon`; `rows.rs`, the `c140_` test). The CLI's
`--fields` refuses as "not built, waiting for a consumer that asks"
(`search_cmd.rs`). `Held { item, place, class, category, price }` is
what a corpus holds per item; `Shown` is the same five with the basis,
the lines with their slots, `against` and the body (`show.rs`);
`read_one` builds a `Shown` from a raw row, so the maker is one
function the rows cannot yet call.

**The basis is the version, and the holder checks it.** `Basis` names
the store, the account, the build, the facts revision, every table and
the intent revision (`corpus.rs`); `Holder::corpus` compares it before
every answer and reloads whole when it moved; a continuation across a
change is refused (C98, C100). The intent revision is a sum over every
annotation row (`Annotations::revision`), which is why C167 puts the
saved search in a file of its own.

**The store's shape for intent is generic.** `Annotations::open(path)`
opens any file; `open_for(dir, uuid)` only names the path. `put` runs
the kind's strict parse (`IntentValue`, C66), the writer's rules, then
compare-and-swap; `delete` tombstones; `list` names an unreadable row
(C151); `revision` is one statement; `export` is a snapshot. A world's
durable state sits at `<root>/<provider>/` beside the account files
(`world.rs`, "Durable state in the world"), and the verbs' `Frontend::dir`
is that directory (`frontend.rs`, `account_path`).

**The frontends share one shape per write.** `Account::open` gives
`Opened { facts, intent }`; every write answers the one receipt
(`receipt.rs`: `written`, `prior`, `revision`, `undo`); every refusal is
one document with `offers` each surface spells (`refusal.rs`, `Way`,
`Surface`). The MCP opens per call and holds the search crate's
`Holder`; its write tools take `if_revision` and refuse a blind replace
(C52); every tool refuses an argument it does not take (`Args`).

## 2. The page: what it holds, what it may not, the one feed

**What a page holds.** Its URL; the documents it rendered, each with
the revisions it was made at; the unsent input of its forms; the
picker's transient list. Nothing else: no browser storage, no cookie,
no `pushState`, no copy of rows that outlives a navigation, no
sequence number, no value a page computed from a document. A
presentation choice (cards or the table, a sort) is a URL field; a
request's identity is the form's fields as the fold reads them, and
`next` is the request whole with its basis.

**Identity.** `search?<the form's fields>` and `search?next=<token>`
stay the one answer route; `item/{id}`, `saved` (§3) join `/`,
`refresh`, `plan`, `jobs`. A page field that is not a request field
(`saved=<name>`, §3) is read apart from the fold, so
`asked_by_the_form` never sees it and a route still folds back.

**Every document with its version.** The answer and the item carry
`basis`; the league list on the form page is the facts revision's; the
saved list (§3) is the user revision's (C167, a revision no answer's
basis carries). The rule for every page: it prints, in its head, the
revisions its documents were made at, as one `meta` line a script can
read and a golden holds. That line is what part 3's feed compares.
Nothing else is reserved: a page re-asks by loading its own URL, which
rebuilds everything it holds; the feed, when it comes, says which
revision moved and the page reloads. A page built so needs no change
when the feed arrives, and a page that would is wrong now.

**What pins it.** The goldens hold every page and every document
(`web.rs`, `page_golden`, the JSON equality with the CLI). The state
sentence gets its check at step 3: a test in `web.rs` reads each served
script for the names of the storage and history APIs and refuses one,
beside the content policy, which already admits no outside script; and
every golden page carries the `meta` line. A rule that is presentation
inside a script (the picker's one today, the virtualiser's window at
step 6) gets a browser-driven check, and the frontends park holds that
until the second rule exists: step 6 is the second rule, and its spike
chooses the driver (§7).

**The dense table and the toolchain.** Step 6, after the owner's
morning: a server-windowed rows view keyed by item id (`GUI-NOTES.md`
§6; `after` and `limit` already exist, and a window past a moved basis
is refused, which is the honest answer), a vendored, pinned virtualiser
file first, the browser-driven check with it. The spike decides
whether the park holds. Nothing more about them here.

## 3. The saved search

**The kind** (C167, as ruled): scope `search`, key the name, kind
`saved-search`, value `{version: 1, text, scope: {realm, membership}}`.
The text is the canonical text (C104), written as the language prints
it back, never a site URL: a trade search saved is its translation's
text, this build's report left behind. The scope never names an
account. The realm is always present: a save over a store of several
realms that names none is C96's refusal. A name is one to sixty-four
characters after trimming, no line break, compared exactly. The view
(sort, limit, fields) is the URL's, not the row's (§9).

**The store** (C35's shape over C167's file): `acquisition-store/src/user.rs`,
a `User` newtype over `Annotations::open` at `<dir>/user.db`, with
`open_for(dir)`, `open_read_only_for(dir)`, `open_memory()`, and `put`,
`get`, `list`, `delete`, `revision`, `export` delegated; `SavedSearch`
as the kind (`IntentValue`, version 1, shape checked, nothing more: the
store links no language). A newtype, so a signature taking
`&Annotations` cannot be handed the user file. No rename door: a rename
is two writes in the verbs (below). The tests are the `c35_` tests'
shape over the second file (compare-and-swap, the tombstone's ABA
guard, the revision sum, provenance, an unreadable row named, the
export), named `c167_`.

**The verbs** (`acquisition-verbs/src/saved.rs`): `Frontend::user`
opens the world's file (the door grep gains `User::open_for`); `list`
answers one object, `searches` counted, each row with `name`, `text`,
`scope`, `revision`, `written_via`, `actor`, its times with their
words (C163), and `request`, the search request it is, ready to send
back; `set` reads the text with the language's reader in the row's
realm (an authoring error refuses before the file is touched), stores
the canonical text, and answers the receipt; `clear` the receipt with
`prior`; `rename` creates the new name first and clears the old, so an
interruption leaves both names and never none, and answers one receipt
whose `undo` is the reverse rename; `request_of(name)` is what every
`--saved` resolves through. A name no row holds is `not_found`,
offering the list; a stale `if_revision` is `conflict` with `current`
and the write re-aimed (`Retry`'s shape). `Way` gains the list and the
set, so every surface spells both.

**The CLI**: `acq saved` with `list` (the default), `show NAME`,
`set NAME QUERY [--realm] [--membership] [--if-revision]`,
`rename OLD NEW [--if-revision]`, `clear NAME [--if-revision]`,
`--if-revision` meaning what `policy set`'s means; `acq search --saved
NAME` runs it, view flags beside it and no word of scope or query, as
`--next` takes none. `--json` on each is the document (C11).

**The MCP**: `saved_searches` (`name` for one, each row carrying its
`request`), `saved_search_set` (`name`, `text`, `realm`, `membership`,
`if_revision`, required where a row stands: C52's create-only form),
`saved_search_rename`, `saved_search_clear`; and `search` takes `saved`
in place of `query` and `scope`, a view beside it. Each answers the
CLI's `--json` document, pinned as the pricing tools are
(`pricing_tools.rs`).

**The pages.** `saved`: the list, each name a link that opens it, a
rename form and a clear form beside each row, the file's revision in
the head. `saved/{name}`: a GET that writes nothing and redirects to
`search?realm=…&membership=…&text=…&saved=<name>`, so the answer page
is the one answer page and its identity its request. The answer page
gains a save form: a name box with the known names as its list, the
text and scope hidden, posted to `saved`; where the URL carries
`saved=<name>` the page says the name and its revision and the form
replaces at that revision, and where the row's text no longer equals
the page's query, or the name is gone, it says so with the row's text
or the list. `POST saved`, `saved/rename`, `saved/clear`: the write,
then a redirect to the list; a conflict renders the refusal page with
the re-aimed write as a form, since a way on that writes is a form
where a way on that reads is a link. Mutation by POST alone under the
token (C138). Editing is the loop: open, edit the box, run, save at the
revision.

**The handoff, both ways, with no new channel.** The agent saves a
search; the owner opens the list, or reloads it, and it is there, with
`written_via` saying the agent's channel. The owner saves one; the
agent lists and gets the request to run. Neither learns the other's
door: the agent never sees the web URL or its token, the owner never
drives the MCP. The basis on every answer is what each quotes to the
other, so "I see 41, you see 40" resolves to a moved revision in words.
What would make the agent say something wrong, and what stops it: a
name that already exists (refused with its row and revision); a text
the agent edited from memory (the canonical text is stored, and a
conflict names the current row); a search saved under the agent's own
realm assumption (the realm is stored, resolved, and printed back);
the saved text of a later build (listed and named, refused only when
run, never dropped: C151's rule, inherited).

## 4. Cards and the pages' own ways on

**What exists:** the compact row with its icon, the table, `next`,
`show` as a page, `Paged` and `asked_by_the_form`, the badges.

**Rows that carry every field** (#98's "Closes when"): `RowField::Item`
puts on each row what `Shown` holds of the item: the item without its
lines, the lines with their slots, `class`, `category`, `price`. The
step factors the `Held`-to-`Shown` projection out of `read_one` into
one function `show` and the rows both call: one maker, pinned in the
search crate by equality (`rows.rs`, a row's `item` equals `show`'s for
the same id at the same basis). The CLI's `--fields icon,item` is built
(the consumer asked), its text each row as `show` prints one; the MCP's
`search` already takes `view.rows.fields`. A card row is the item
whole, so the rows limit under `item` is bounded in the frame's spirit
(C166) and the rest is `next`; step 5's spike measures where.

**The card and the item page.** The answer page asks
`fields: [icon, item]` by default and renders a card per row in the
site's order (the name in its rarity's colour, the base, properties,
requirements, lines by source with their flags, sockets, the price,
the place with its age); `view.fields=icon` in the URL is the compact
table. The item page renders `Shown` by the same card and the lines in
`show`'s order. Nothing is composed in the page: every value is a
field the document carries.

**Ways on through `Paged`** (#176's "Closes when"): the answer page
renders `Answer` itself: the query line, the scope block with its
never-fetched list as a link to the league's tabs page (`Way::Tabs`),
the total, the rows, `next`, the terms block with each count's route a
link where the form folds it (`Way::Search` through
`asked_by_the_form`) and the count alone where it does not, the basis
line. The item page: the place, the price with no link (no page takes
`price show`), the lines. `answer_head`, `answer_tail` and `shown_text`
stay the CLI's. The cost, said plainly: two renderers of one document,
text and HTML, each with its golden; the labels are presentation, the
words come from the document, and a label that is semantics is a
document change first.

## 5. What must exist for morning 1, and what can wait

Must exist: steps 1 to 5 of §7: the file and its kind; `acq saved`
and `--saved`; the four tools and `search.saved`; the saved pages and
the save form; the answer page from its document with cards and its
ways on. The owner then sits with an agent, in his store, under the
live-run skill, as 4b's Do 10.

Can wait: the dense table, the By-Tab tree and the virtualiser (step
6, after the morning); a sticky default realm and league (#19: the
saved search carries both, the form defaults to `pc`, and a default
kind is added when first written); describe's derived groups (#159:
the picker lists stats, no surface here types a group); a saved view;
tags and names on items; any feed (a reload is the liveness: the
prompt's word); price writes and the policy page (part 3); a login
from a page (parked). Group 4, feedback, is out of this part.

## 6. What not to do

- `search?saved=<name>` running the search by name: the answer page
  would depend on two files and a mutable name; open redirects.
- A `saved` spelling in `QueryInput`: the search crate opens no file
  (C89); the name resolves in the verbs, the request runs as any.
- The saved search in the annotation file: ruled out (C167).
- A rename door in the store, or a transaction across two names:
  two writes in the verbs, the new name first, until a kind needs more.
- The view in the row: a saved search is a query, its presentation the
  URL's; raised for the owner's morning (§9).
- Cards composed from `Held.item` in the page, or a table that sorts:
  two makers (C46).
- A feed, SSE or a meta-refresh on the saved list: part 3; a reload.
- Handing the agent the web URL, or a tool that opens a browser: the
  token is the human door (C138), and the agent cannot see a page.
- "Open on the trade site": the outbound encoder stays parked.
- A framework, a build step, a page-side store: the parks and C16.

## 7. The build

Ordered from the least uncertain to the bet; each step one brief from
`TEMPLATE.md`, in series on the shared checkout, every test obeying
C137, no new seed function (#142's rule: rows in the mock's one world).

1. **The file and its first consumer**: `user.rs`, `SavedSearch`, the
   `c167_` tests; `saved.rs` in the verbs with `Frontend::user`, the
   door grep widened; `acq saved` and `acq search --saved`;
   `CLI-REFERENCE.md` and the README tour line regenerated. Consumer:
   the owner at a terminal. Closes #42 by its "Fire" line (the first
   user-scoped kind written). Retires: `search_cmd`'s unbuilt refusal
   of `--fields` waits for step 5; nothing here.
2. **The MCP**: the four tools and `search.saved`; each answer equal to
   the CLI's document; the first-turn size measured against #104's
   37,824 bytes and said in the close. Consumer: the agent in morning 1
   (a mock seat under the seat skill before the morning).
3. **The saved pages and the save form**, under §8's first draft: the
   routes, the goldens, the no-state check and the `meta` line on every
   page (C16's Pinned line filled at the close). Consumer: the owner.
4. **The answer and item pages from their documents** (#176): the
   renderer, the ways on through `Paged`, the two goldens regenerated.
   Closes #176 by its "Closes when". Retires: the four `acq …` lines of
   `search_pages.txt`; held by that golden regenerated and `no_ids.rs`,
   which reads every golden.
5. **Cards** (#98): `RowField::Item`, the one maker factored, the CLI's
   `--fields`, the card renderer, the bound on `limit` under `item`.
   Closes #98. Retires: `search_cmd`'s unbuilt refusal of `--fields`;
   held by the rows test that pins `item`. Spike, inside the step: the
   size and time of `fields: [item]` at limits of 20 and 200 over the
   mock's 22,000 items, which sets the bound.
6. **The dense table**, after the owner's morning: the server-windowed
   rows view, a vendored, pinned virtualiser file, the browser-driven
   check. Spike first, one day: the file over 22,000 rows in the
   browsers the owner uses, scroll frame time and DOM size; and the
   check's driver, a headless browser driven from a Rust test with no
   Node, or a CI-only job, measured by what it adds to the gate. Its
   answer shapes the step's brief or comes to the owner with two shapes
   (the park: no framework unless the spike shows the need).

Mechanism bets: the item field's size (step 5's spike); the virtualiser
without a build step and the check's driver (step 6's); the rest is a
shape the tree has.

## 8. Drafts for the owner

Each is his; nothing is entered until he rules.

- **C<next> — A page writes user-scoped intent as the verbs write
  it.** A page writes only through the verbs' door (C46, every
  frontend an adapter), by POST under the listener's token (C138, the
  listener), naming in its form the revision it replaces as the plan
  page carries its envelope (C38, plans are binding), and answers the
  one receipt (C78) or the one refusal (C11); the first kind a page
  writes is the saved search (C167, user-scoped intent); an annotation
  (a price, the policy) is written from no page until its page is
  designed. *Why:* a browser is where a stale view could clobber, and
  the revision in the form makes staleness a refusal, as C52 does for
  an agent. *Pinned:* `acquisition-cli/tests/web.rs`, the saved pages.
  Recommendation: enter as drafted, before step 3.
- **C167, one sentence added (amendment):** "`user.db` sits beside the
  account files, where the world keeps its durable state." *Why:* the
  file is in his store, and the daemon's and the rails' are there.
  Recommendation: enter; the mechanism's doc comment carries the path.
- **The web's routes** (new routes are his under C138): `saved` GET
  and POST, `saved/{name}` GET (a redirect, no write), `saved/rename`
  POST, `saved/clear` POST. Recommendation: as listed.
- **C16's Pinned line**, no ruling: at step 3's close, the state
  sentence names the no-state check and the `meta` line.
- **The spike's home for the browser-driven check** (the gate, or
  CI): no draft now; step 6's spike brings the measured shapes.

Agent-facing design this session made, the session's under the
delegation and listed for his review: the verb and tool names above;
the kind's shape and the name rule; the canonical text saved, never a
URL; the view left to the URL; rename as two writes, new first;
`saved=` as a page field apart from the request; cards as the answer
page's default with `view.fields` in the URL; a way on that writes
rendered as a form; `--if-revision` on the CLI meaning `policy set`'s.

## 9. Raised for review

- Nothing was run: the mock's behaviour is read from the tree and the
  goldens; the spikes are where mechanism meets it.
- The view in the saved row: the owner's morning will say whether
  "rings by life" wants its sort saved; a `view` field on the kind is
  a version 2, upgraded in memory (C66). Pressed on by a reviewer.
- Two renderers of one document (§4): a reviewer may ask that the
  labels come from one table both renderers read; the design says a
  label is presentation and takes the cost.
- The `c167_` tests as copies of `c35_`'s over a second file: C153
  asks for one table over both files where the shape is one; step 1's
  brief should try that first.
- The web's tests seed a private world (`web.rs`, `seed`), built before
  #142's rule; this part adds no seed, and t6 folds the one there is.
- For part 2: a `search --saved` on the MCP opens a third file per
  call; the resident, when it exists, holds a `User` beside the
  `Holder`. The account is a lifecycle state (the head); the saved
  list is per world, so a logout changes nothing on that page.
- For part 3: the feed line names the user revision beside the basis,
  since the saved list's version is it; a page's `meta` line is what
  the feed is compared with; the price and policy pages extend §8's
  first draft by naming their kinds; the reload-as-liveness of morning
  1 is what the feed replaces, and nothing a page holds changes.
- For part 3: a continuation refused across a moved basis is what makes
  a scrolled window honest; the queue view is part 2's wire, untouched.
