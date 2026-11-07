# 4-full-1-pages — the saved pages, the save form and the basis line

Step 3 of 4-full part 1 (`roadmap/briefs/4-full-1-design.md` §2, §3
"The pages" and §7 step 3, under §8's first draft as the owner ruled
it on 2026-11-06), for an Opus worker that has not seen the design.
Written against the tree at `7323ff6a`, after step 1
(`roadmap/briefs/4-full-1-saved.md`) and step 2
(`roadmap/briefs/4-full-1-mcp.md`) closed on 2026-11-07
(`roadmap/LEDGER.md`, its last two rows). The owner asked for this
brief on 2026-11-07. Where the design's §3 and step 1's brief differ,
the brief won and §3 says so; this brief builds over what steps 1 and
2 built. Where the design is silent on a page's words or shape, this
brief decides, and the foot lists each decision for the owner.

## Goal

When this step is done the owner, in his browser at `acq web`, keeps
searches: on any answer he types a name and saves the search he is
looking at; on a new **Saved** page he sees every saved search, his
and his agent's, each with its scope, its query, its revision and the
channel that last wrote it, and opens one (a link that runs it),
renames one or clears one, each write answering with what it wrote and
a button that undoes it; an answer opened from a saved search says so
and saves over it at the revision he opened, so an agent's write in
between is refused as stale and offered again over the current one,
never clobbered. Every page that renders a versioned document carries
that document's version in one `<meta name="basis">` line in its head,
copied from the document, so part 3's feed has something to compare;
and a test refuses any served script that names a storage, cookie or
history API. This is morning 1's handoff on the owner's side (the
design, §3, "The handoff"): the agent saves through `acq-mcp` (step
2), the owner reloads **Saved** and it is there, `web` or `mcp` saying
the channel; the owner saves, the agent lists. Every write goes
through the verbs step 1 built (C46, every frontend an adapter; C168,
a page writes user-scoped intent as the verbs write it). Nothing
reaches the daemon.

## Read

- `AGENTS.md`, `roadmap/README.md` "A step" and the rulings bullets
  dated 2026-11-06 and 2026-11-07, and `TEMPLATE.md`.
- `roadmap/briefs/4-full-1-design.md` §2 whole (the two kinds of state,
  what a page may not hold, identity, the version line, what pins it),
  §3 "The kind", "The pages", "The handoff", §6, §7 step 3, §8.
- `roadmap/briefs/4-full-1-saved.md` Do 3 and Do 4 (the verbs' and the
  CLI's contract as built) and `roadmap/briefs/4-full-1-mcp.md` Do 3
  and Do 4 (the MCP's two revision mappings, which this step's POSTs
  take as theirs).
- `decisions/frontends.md`: C16 as amended (a page holds no state its
  URL and one feed line cannot rebuild; its identity is its URL; every
  document carries its basis; the basis is the only version), C168 (a
  page writes user-scoped intent as the verbs write it, by POST under
  the token, naming the revision it replaces, answering the one receipt
  or the one refusal), C138 (the listener: the token, the Host and
  Origin guard, POST alone mutates), C139 (`acq web` a CLI verb,
  `web` its channel), C116 (no ruling id in user text), C121, C143 (a
  way on is a value each surface spells), C149, C163, C53 (ten listed;
  the tabs page's whole list, an audit view), and "Not doing" (a page
  login; the script seam; a front-end build step).
  `decisions/store.md` C167 (the world's `user.db`), C35.
  `decisions/search.md` C96 (the realm is scope), C98 (the basis), C104
  (the canonical text on every surface). `decisions/pricing.md` C65
  (`written_via`), C78 (a write returns its prior). `AGENTS.md` C46,
  C122, C153, C154.
- `python3 -I tools/bothy.py pack --path crates/acquisition-cli/src/web/mod.rs`,
  then `crates/acquisition-cli/src/web/`: `mod.rs` (the module doc;
  `Web`; the router in `web`, where `route_layer(with_account)` gates
  every page but the assets; `guard`; `failed`, `refusal`, `document`,
  `wants_json`; `Web::link`, which builds an absolute `/<token>/path?…`;
  `asked_by_the_form`, which folds a request back to the form's fields
  and answers none where it cannot; `Paged` and its `request`, whose
  `_ => return None` arm covers the five saved ways today; `search`,
  which reads `next` or folds; `item`; `to_jobs`, the one redirect);
  `page.rs` (`Head`, `page`, `refused`, `query_box`, `search_form`,
  `answer`, the continuation link `search?next=…`); `form.rs`
  (`emitted`, the names the fold accepts; `Folded` and its `request`;
  `fold`, which refuses any other name as a usage error);
  `assets/page.html` (the template), `assets/picker.js` (the one
  script).
- `crates/acquisition-verbs/src/saved.rs`: the module doc; `Set`;
  `list(frontend, limit, expand)` → `SavedSearches` (`searches`,
  `total`, `left_out`, `revision`, the file's); `show(frontend,
  id_or_name)` → `SavedRow` (`Read` with `id`, `name`, `text`,
  `scope`, `revision`, the row's, `updated_at_age`, `written_via`,
  `actor`, `request`; or `Unreadable` with `id`, `revision`,
  `unreadable`); `set(frontend, account, &set, &mut mint)`;
  `clear(frontend, id, expect)`; `SavedWritten` and its `undo`;
  `is_id`, `mint`; the refusals `saved_search_not_found` (offering the
  list), `conflict` (with `revision` and `current`, the write offered
  again at the revision that stands, or, for a create meeting a
  standing row, that row's show), `arguments`.
  `crates/acquisition-verbs/src/refusal.rs`: `Way`'s five saved
  variants, `Surface`. `crates/acquisition-verbs/src/receipt.rs`: the
  module doc and `saved`. `crates/acquisition-plan/src/pricing.rs`:
  `Expect`, `Expect::or`.
- `crates/acquisition-cli/src/saved_cmd.rs`: how the CLI adapts the
  same functions (its text is not the page's: it prints command lines,
  which no new page prints).
- `crates/acquisition-cli/tests/`: `main.rs` (one binary); `web.rs`
  (`seed`, `offline_web`, `no_cors`, the `Case` table in
  `c138_the_listener_answers_its_own_pages_alone`,
  `the_fold_answers_as_acq_search_trade_does`, which compares a page's
  JSON with the CLI's after removing each row's `icon`,
  `every_search_page_is_a_wording_golden`, `offered_links`);
  `harness/mod.rs` (`WebUi` with `get`, `get_json`, `post`, `raw`;
  `Reply`; `Pages` and `Pages::page`, which records `<main>` alone and
  masks the token, the address and a plan's envelope; `FORM_PANES`;
  `page_golden`, `panes_golden`; `normalise` and `basis_rule`;
  `Transcript::placeholder`, the rule for a word drawn at random);
  `saved_json.rs` (`put`, which writes a saved search at a fixed id
  through the store's door; `NewerSaved`, with which it stages a row
  this build cannot read; `world`, `id_named`); `no_ids.rs`
  (`c116_no_user_facing_text_cites_a_record_id`, which sweeps every
  file under `tests/goldens/web` and `src/web/assets`); the goldens
  under `tests/goldens/web/` (six: `console_realm`, `jobs_halted`,
  `moved_world`, `refresh_loop`, `search_form`, `search_pages`).
- `tools/arch-check.sh`, the door grep (`User::open_for` among the
  doors no frontend opens). `REVIEW-CHECKLIST.md` "Frontends and the
  surface an agent meets", the script-rule row.
- `README.md` line 203 (`acq web`'s tour line) and `CLI-REFERENCE.md`
  `## acq web` (the verb's help).

## Input

Fixed: the verbs' contract (`saved.rs`, `refusal.rs`, `receipt.rs`;
step 1, closed), the MCP's (step 2, closed), C168, C16 as amended, and
the routes the owner ruled on 2026-11-06: `saved` GET and POST,
`saved/open` GET (a redirect that writes nothing), `saved/clear` POST,
an id only ever a query value. No new route beyond these four.

What the tree holds today, checked at `7323ff6a`: the router has nine
routes behind the account gate and two assets; no page writes intent; `Paged::request`
answers none for every saved way; the template's head carries no
version; no test reads a served script; C16's *Pinned* says "the state
sentence by nothing yet, until 4-full's part 1 finds its check", and
C168's "nothing yet; Unbuilt, the `c168_` tests over the saved pages".
Of the documents the pages render, those that carry a version a page
can copy are the answer (`basis`, `Answer` in
`acquisition-search/src/answer.rs`), the item (`basis`, `Shown` in
`show.rs`), the saved list (`revision`, the file's) and the saved row
(`revision`, the row's); the league list the form page renders
(`LeagueListShown`, `acquisition-verbs/src/leagues.rs`) carries
`fetched_at` and no revision, the tabs, characters and jobs documents
none; the plan carries `basis` with `snapshot_taken_at`, a clock time.

The facts the tests stand on: `web.rs`'s `seed` (one account, pc and
xbox, the rings), `saved_json.rs`'s `put` and `NewerSaved` (made
visible to `web.rs`, never copied) for rows at fixed ids and a row
this build cannot read, and rows written through the
pages and through `acq saved set` in the same world. No new seed
function (#142's rule). The test clock is the harness's one instant
(`ACQ_NOW`), so ages agree between a page and a CLI call in one test.

The failing inputs the tests stage: a `POST saved` naming a field the
route does not take, an `if_revision` that is no number, a nonzero
`if_revision` with no id, a create with an empty name, a write at a
revision that moved since the page was rendered; a `POST saved/clear`
with no `if_revision` and at a revision that moved; `saved/open` with
a name in place of an id and with an id no row holds; `search` with
`saved=` a name; `GET saved/clear`, `POST saved/open`, a `POST saved`
without the token and one from another Origin.

The last push's CI: `Gate (Linux)` run 42389345514 and `Gate
(Windows)` run 42389345529, both on `7323ff6a`, in progress when this
brief was written; the worker reads their conclusions first, and a red
run is named in the report.

## Do

1. **No ruling is owed.** The routes, C168 and C16's reading are the
   owner's of 2026-11-06. Two design sentences the registry overrides
   are settled here, not ruled again: the design's "the write, then a
   redirect to the list" yields to C168's "answers the one receipt"
   (Do 6); the design's rename form carrying "the row's id, revision,
   text and scope hidden" yields to step 1's partial-field `set`, so a
   rename posts the id, the revision and the name alone (Do 4).
2. **The module doc carries C16's reading** (the owner's word: the
   doc comment carries it). `web/mod.rs` gains "C16, what a page
   holds": document state is what a page shows, rebuilt from its URL;
   transient state (unsent input, the picker's asks in flight) is
   discarded by a reload and labels nothing; a page holds no browser
   storage, no cookie, no history entry of its own, no value computed
   from a document; the basis line (Do 3); what the scan holds and
   what it cannot (Do 9). And "C168, the saved pages", stating Do 4 to
   Do 8. The module doc's first paragraph says the server writes
   through the verbs, the saved search its one kind.
3. **The basis line.** `Head` gains the versions the page renders;
   `page` writes them as one line in the head,
   `<meta name="basis" content="…">`, after the viewport line, the
   content one JSON object, HTML-escaped, keyed by the document:
   `answer` the answer's `basis`, `item` the item's `basis`,
   `saved_searches` the list's `revision`, `saved_search` the row's
   `revision`; each value is `serde_json::to_value` of the document's
   own field, never a number the page composes. A page writes the line
   only where it records at least one document: the answer (with
   `saved_search` beside `answer` where Do 7 read a row), the item, the
   saved list. The form, refresh, plan, jobs, receipt and refusal
   pages carry none at this step (the foot says why). A refusal's
   JSON, a receipt's and every other document a route answers are
   unchanged by this.
4. **`GET saved`, the list.** Calls `saved::list(frontend, None,
   true)`: the page lists every row, as the tabs page lists every tab,
   because a row it does not list cannot be renamed or cleared (C53's
   audit view); JSON answers that document whole. The page, titled
   "Saved searches":
   - a head line, `N saved searches · the file at revision R` (one:
     `1 saved search`); with none, `No saved searches yet: save one
     from any search's answer, under its query.`
   - a table, one row per search, in the document's order: **Name**,
     a link to `saved/open?id=<id>`; **Scope**, `pc · live`; **Query**,
     the text; **Revision**; **Last written**, `<updated_at_age> via
     <written_via>`, ` as <actor>` where one is claimed; **Id**; then
     two forms, a rename (a text input `name` holding the name, `id`
     and `if_revision` hidden, button **Rename**) and a clear (`id` and
     `if_revision` hidden, button **Clear**), each posting with the
     row's revision as listed.
   - a row this build cannot read: `?` for the name, `this build cannot
     read it: <why>` across scope and query, its revision and id, and
     the clear form alone.
   The template's `<nav>` gains **Saved** between Search and Refresh.
5. **`GET saved/open?id=<id>`, the redirect.** `id` must have an id's
   shape (`saved::is_id`); a name, or anything else, is a usage
   refusal saying a page opens a saved search by its id. Otherwise it
   calls `saved::show(frontend, id)`: a refusal renders as every
   refusal does (`saved_search_not_found` offering the list); an
   unreadable row is refused saying this build cannot read it, offering
   the list. A readable row's `request` goes through
   `asked_by_the_form` (one rule for every link a page makes to a
   search); the response is `303 See Other` to `search?<those
   fields>&saved=<id>&saved_revision=<the row's revision>`, absolute
   under the token as `to_jobs` makes its location. Where
   `asked_by_the_form` answers none, the route refuses saying the
   saved search is no search a page can ask. It writes nothing: the
   user file's revision is the same before and after.
6. **`POST saved` and `POST saved/clear`, the writes.** Each reads a
   form body (`Form<Vec<(String, String)>>`, as `apply` reads one); an
   empty value is absent, as the fold drops one; a name the route does
   not take is a usage refusal naming it.
   - `saved` takes `id`, `if_revision`, `name`, `text`, `realm`,
     `membership` into the verbs' one `Set`, `expect:
     Expect::or(if_revision, Expect::Create)`, the MCP's mapping, and
     calls `saved::set(frontend, Some(<the served account's name>),
     &set, &mut saved::mint)`. So a page's forms are: no id, a create;
     an id and a revision, a write over the row at exactly that
     revision, fields not named keeping the row's values; an id and `0`,
     the create at the id that a clear's undo is. A page never writes
     at `Expect::Current`: a browser is where a stale view could
     clobber (C168).
   - `saved/clear` takes `id` and `if_revision`, both required, and
     calls `saved::clear(frontend, id, Expect::Exactly(if_revision))`,
     the MCP's mapping; a missing one is a usage refusal naming it.
   - The answer is the receipt, `receipt::saved(&written, &Paged {…})`:
     its document where JSON is asked, else a page titled "Saved"
     rendering it: one sentence of what was written — a create,
     `Saved "<name>" (<realm> · <membership>) as <id>, at revision
     <n>.`; a write over a row, `Wrote "<name>" (…) at revision <n>;
     before, it was "<prior name>" (…): <prior text>, at revision
     <m>.`; a clear, `Cleared "<name>" (…), which stood at revision
     <m>.` — then the undo as a form whose button says the receipt's
     `says` (`Undo: <says>`), or `Nothing puts it back: <no_undo>`;
     then a link **Open it** (`saved/open?id=`) where a row stands, and
     **The saved searches**. A refusal renders as every refusal does
     (Do 8), status 400.
   Every link and form action the new pages print is absolute,
   `/<token>/…` (`Web::link`), never relative: the clear's receipt is
   served at `saved/clear`, where a relative `saved` resolves under
   `saved/`.
7. **The answer page keeps its saved search.** `search` reads `saved`
   and `saved_revision` apart from the fold and removes them before
   folding (the fold refuses any name it did not emit). `saved=` that
   is no id's shape is a usage refusal, as Do 5's. With `saved=<id>`
   the page calls `saved::show` and, above the answer, says one of:
   - `Saved search "<name>" at revision <n>` (`n` the URL's
     `saved_revision`) with a link to the list; and, where the row's
     revision is not `n`, `It has been written since: it is now at
     revision <m>, last written <age> via <via>. Saving over it here is
     refused as stale and offers the write over revision <m>.`; and,
     where the row's text, realm or membership is not the answer's
     (`query.text`, `scope.realm`, `scope.membership`, each compared as
     the two documents carry them, byte for byte), `This answer's
     query is not the saved one's: the saved query is <text> over
     <realm> · <membership>.`;
   - `Saved search <id> is gone: no saved search has that id now.
     Saving here makes a new one.` where `show` refuses
     `saved_search_not_found`;
   - `Saved search <id> holds a value this build cannot read; clear it
     from the saved searches.` where the row is unreadable.
   Under the query box, the **save form** posts to `saved` the answer's
   `query.text`, `scope.realm` and `scope.membership` hidden (the text
   the answer ran, the translation's where the search was the site's
   form; never the box's unsent edit), and a text input `name`. With a
   readable row standing: the input holds its name, `id` and
   `if_revision` (the URL's `saved_revision`) hidden, the button
   **Save over "<name>"**; and a second form, **Save as a new
   search**, the create. Without one: the button **Save this search**,
   the create. The query box and the continuation link (`search?next=…`)
   carry `saved` and `saved_revision` where the page has them, so the
   loop survives an edit and a page turn.
8. **The fold's `membership`, and the page's ways on.**
   - `membership` joins `emitted`; `Folded` carries it into the
     request's `scope.membership` as the word typed, which `Asked::read`
     reads or refuses in the search crate's words (the fold decides
     nothing the reader decides). The search form gains a
     **Membership** select in its search bar, `live` (selected),
     `all`, `removed`; the query box carries the answer's membership
     hidden. `asked_by_the_form` emits `membership` where the request's
     is not `live`, so every link the goldens hold today stands as it
     is, and a request over removed items now folds back where it
     answered none.
   - `Paged::request` spells the saved ways: `SavedSearches` → a link
     to `saved`; `SavedSearchShow { id }` → a link to `saved` (the row
     stands there with its forms; no page shows one row alone);
     `SavedSearchRun { id, account }` → a link to `saved/open?id=`
     where `account` is none or the one served, else none;
     `SavedSearchSet(set)` → a form, unless its expect is `Current`
     (none); `SavedSearchClear { id, if_revision }` → a form. A form is
     spelled in the refusal's and the receipt's JSON as `{"post":
     "/<token>/saved", "fields": {…}}` (`saved/clear` for a clear), the
     fields the route takes, `if_revision` `0` for `Create` at an id;
     `page::refused`, and the receipt page, render a string `request`
     as a link and a `post` object as a form of hidden fields with one
     button saying the offer's `says`. So a conflict's page offers the
     write over the current revision as a button that posts it (a
     write is a form, a read a link).
9. **The no-state scan.** A test in `web.rs` reads every page the step
   records, takes each `<script src=…>` it names, fetches each from the
   server and refuses one whose bytes name a storage, cookie or history
   API: `localStorage`, `sessionStorage`, `indexedDB`, `caches`,
   `cookieStore`, `document.cookie`, `pushState`, `replaceState`; it
   checks too that no response the test met carries `Set-Cookie`, and
   that the scan refuses a script text naming each word (the breaker,
   in the same test). What it cannot hold (a cache in a closure or the
   DOM, state the URL omits, a generation a script keeps for itself)
   is a new row in `REVIEW-CHECKLIST.md` "Frontends and the surface an
   agent meets", until the browser-driven check exists (step 6).
10. **The goldens.** `Pages::page` records the head's basis line, when
    there is one, as the line before `<main>`, under a harness rule
    that masks the basis's `store` and `build` values as `basis_rule`
    masks them in text; any other value seen to differ between two
    runs of the test is masked by a named rule, each named in the
    report. `Pages` gains `placeholder`, `Transcript::placeholder`'s
    rule, for an id a page create drew. A new golden `saved_pages`
    (the list empty, the list with rows written by `put` at fixed ids
    and one this build cannot read, a create from an answer with its
    receipt, a rename and its receipt, a clear and its receipt, the
    answer opened from a row, the same after the row moved, after its
    text changed, after it was cleared, and a conflict's page) holds
    the words of Do 4 to Do 8 as facts a reader must find. The six
    goldens regenerate under `ACQ_UPDATE_FIXTURES=1`; each diff is
    read and holds the basis line on answer and item pages, the
    membership control on form pages, and the save form on answer
    pages, and nothing else.
11. **Pins, what the adapter adds** (C122), in `web.rs` beside the
    tests it joins (C153), the verbs' semantics pinned once in
    `acquisition-verbs/tests/documents.rs` and not again here:
    - **C16, the basis line**: for each recorded page that carries one
      (an answer from the form, a continuation, an edited text, a
      counted answer, an item, the list empty and full, an answer
      opened from a row), each entry equals the field the same URL
      answers as JSON (`basis`, `revision`), and `saved_search` equals
      `acq saved show <id> --json`'s `revision`; the form page and a
      refusal page carry no line.
    - **C16, the scan**: Do 9.
    - **C168, a page writes as the verbs write**: a create from an
      answer, a rename and a clear from the list, each receipt's JSON
      `written` equal to `acq saved show <id> --json` (or after a
      clear, `show` refusing `saved_search_not_found`), its
      `written_via` `web`; each undo form posted as rendered lands
      and puts the prior back; a conflict: the list rendered, the row
      renamed by `acq saved set`, the page's rename posted at the old
      revision refused `conflict` with `current`, nothing written, the
      offered form posted as rendered landing over the current
      revision; Do 6's failing inputs each refused, nothing written
      (the file's revision unchanged).
    - **The routes**: the c138 test's `Case` table gains `GET
      saved/clear` and `POST saved/open` (405), `POST /saved` without
      the token (404) and `POST saved` from another Origin (403);
      `saved/open` answers 303 to the search Do 5 names, the user
      file's revision unchanged, and that search answers as `acq search
      --saved <id> --json` does, each row's `icon` removed (as the fold
      test compares); `saved/open` with a name, and `search` with
      `saved=` a name, are usage refusals.
    - **The handoff**: a row written by `acq saved set` is on the list
      page; a row the page writes is in `acq saved list --json` with
      `written_via` `web`.
    - **The size bound**: eleven rows written by `put` are eleven on
      the list page, and its JSON equals `acq saved list --expand
      --json` (the page asks the whole list; no bound of its own).
    - **The text**: no new page carries `acq ` (no command line); the
      new golden is swept by `no_ids.rs` by existing.
    Every test under C137 (no wall-clock bound).
12. **The words outside the server.** `acq web`'s help names the saved
    searches among its pages (kept, opened, renamed, cleared, saved
    from an answer) and that they are the world's; `CLI-REFERENCE.md`
    regenerated under `ACQ_UPDATE_FIXTURES=1`; README line 203 says the
    same in its own words.
13. **The gate and the commits.** `tools/gate.sh` green on the step's
    tree; one commit per concern or one for the step, the last naming
    this brief. The push and the review are the orchestrating
    session's.

## Done when

- `web/mod.rs`'s router has `saved` (GET and POST), `saved/open` (GET)
  and `saved/clear` (POST), and no other new route.
- No file under `crates/acquisition-cli/src/web/` names `User::`,
  `Annotations` or `user.db`; the saved routes call `saved::list`,
  `saved::show`, `saved::set` and `saved::clear`; `tools/arch-check.sh`
  is green.
- `saved` POST reads its revision through `Expect::or(…,
  Expect::Create)` and `saved/clear` through `Expect::Exactly`; no
  page route writes at `Expect::Current`.
- `assets/page.html` has a placeholder for the basis line and the
  **Saved** link in its `<nav>`; `Head` carries the versions; no
  basis value is computed in `page.rs` or `mod.rs` other than by
  `serde_json::to_value` of a document's field.
- `form.rs`'s `emitted` names `membership`; `Paged::request` has an arm
  for each of the five saved ways and answers none only for
  `SavedSearchSet` under `Expect::Current` and a run as another
  account.
- `web/mod.rs`'s module doc has the C16 and C168 sections of Do 2.
- `crates/acquisition-cli/tests/goldens/web/saved_pages.txt` exists;
  the six goldens' diffs in the step's commits hold only Do 10's three
  additions.
- `web.rs` holds the pins of Do 11, the C16 ones named `c16_…` and the
  write ones `c168_…`; it defines no seed function of its own, and
  `saved_json.rs`'s `put` is the one it calls.
- `REVIEW-CHECKLIST.md` has the scan's row (Do 9).
- `CLI-REFERENCE.md`'s `acq web` and README line 203 name the saved
  searches; `cli_reference.rs` and `readme_tour.rs` are green.
- At the close, C16's *Pinned* names the scan and the basis-line test
  by their names as built in place of "by nothing yet", and C168's
  names the `c168_` tests in place of "nothing yet; Unbuilt";
  `python3 -I tools/bothy.py check` is green.
- Every retirement in a commit message is in C154's form.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- `view.fields` in the fold: step 5, where it first has a second value
  (`item`) and the default changes; at this step the one field is
  `icon`, which the fold already asks (the foot says so).
- Cards and the dense table: steps 5 and 6. The answer and item pages
  rendered from their documents, and the `acq …` lines
  `search_pages.txt` carries today: step 4.
- Any change to the verbs' contract (`saved.rs`, `refusal.rs`'s `Way`,
  `receipt.rs`) or the MCP's: closed. A verbs word that reads wrong on
  a page is raised, not changed.
- The daemon, the protocol, the store.
- A basis line on the form, refresh, plan and jobs pages; a feed of
  any kind (part 3).
- A login from a page (parked, "Not doing").
- A price or the policy written from a page (C168: no annotation until
  its page is designed).
- A browser-driven check, a script seam, a front-end build step.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push. (Binds your work.)
- Every `acq` you run is in a mock world (the mock-session skill):
  `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch; never a
  bare `acq`. Nothing in this step needs a daemon. (Binds your work.)
- A page's word this brief leaves open (a label, a sentence's
  wording within Do 4 to Do 7's facts) is the step's own design, named
  in the report and in the commit that builds it. (Binds the thing
  you build.)
- A choice that would add a route, change a ruling or the verbs'
  contract stops the step and comes to the orchestrating session.
- A sentence of this brief the tree disproves is corrected in the
  report, not silently built around.

## Report

`TEMPLATE.md`. Add, under 3, the six goldens' diffs summarised (lines
added and removed per golden) and each masking rule Do 10 needed; and
ask, under 5, for the basis-line test seen failing with one page's
entry composed (a number the page makes) in place of copied.

## For the owner's review

Page design decided here, the session's (the design names the routes
and the forms and leaves these open):
- The basis line is `<meta name="basis">`, a JSON object keyed by the
  document (`answer`, `item`, `saved_searches`, `saved_search`); a
  page that records no document carries none.
- The list page lists every row (`expand`), as the tabs page does;
  `saved_searches`' bound of ten is not a page's.
- The redirect carries `saved_revision`, the row's revision as opened,
  so the save over it names the revision the owner first saw, and an
  agent's write in between is refused as stale; the URL's revision is
  identity, the row's as read now is the basis line's.
- An answer opened from a row offers two forms, save over it and save
  as new; the design names one.
- `SavedSearchShow` links the list; a run as another account than the
  one served is offered nowhere on a page.
- A write way is spelled `{"post", "fields"}` in a page's JSON and
  rendered as a button.
- The **Membership** select on the search form; `membership` folded
  back only where not `live`.
- The saved routes sit behind the account gate as every page does, so
  a world with no account shows the login refusal there, though `acq
  saved` needs none.
- The words of Do 4, Do 6 and Do 7.

Raised, the owner's word wanted or noted:
- **A reload of a receipt resubmits.** C168 says a page answers the
  receipt, so the POST answers a page and not a redirect; a reload of
  a create's receipt asks the browser to post again, and a second
  create lands under a new id with the same name, which a name opened
  by an agent then meets as `ambiguous_saved_search`. Two ways out,
  neither taken: the design's redirect to the list (loses the receipt
  and its undo), or the save form carrying an id the server mints when
  it renders, posted with `if_revision` `0` (the verbs' create at an
  id lands once and a second post conflicts; the form then holds a
  value its URL does not rebuild).
- **The form page's league list carries no revision.** The design's §2
  says the form page records "the league list under the facts revision
  the document names"; `LeagueListShown` names none (`fetched_at`
  alone), and adding one is a verbs change, out of scope. The tabs,
  characters and jobs documents carry none either; the plan's basis
  carries a clock time, so two reads differ. C16's "every document
  carries its basis" is true at this step of the answer, the item and
  the saved documents alone.
- **The row's revision, not the file's.** The answer page's
  `saved_search` entry is the row's revision, the version of the
  document it rendered; part 3's feed, which says the user file's
  revision moved (the design, §9), compares with the list page's
  `saved_searches` entry, not this one.
- **`view.fields` moves to step 5.** The design's §2 and §7 put it in
  step 3's fold; with one value it would be a field that changes
  nothing.
