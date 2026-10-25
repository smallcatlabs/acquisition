# 4b — 4-lite: `acq web`, a renderer over the verbs

## Goal

`acq web` serves a loopback web UI that is a renderer, not an app:
every page renders one document a verb already produces, as the CLI
renders it as text (C53, text is a function of the JSON), and the page
evaluates, sorts, filters and composes nothing itself (C16 as amended;
C46, shared semantics in Rust). Two page groups. The search pages,
shaped like the trade site's search — the form generated from the
committed filter definitions, the results table, the counts, one item
— read the store and never open a door. The refresh pages — the tabs
and characters with a selection, the plan with its quote, the jobs —
walk the loop the CLI has (C38, a plan is a binding authorization
envelope). It is in users' hands for league 3.30: the build beats
phase 3's window or slips whole. The live loop (SSE, the subscriber,
reconnect) is 4-full's, after the time seam.

## Read

- `roadmap/README.md`, then `roadmap/briefs/4a-design.md` (the design
  this brief closes; its Input lists the facts checked against the
  tree), then this file.
- The rulings this build pins, each *Pinned: nothing yet* until it
  does: C16 (the GUI's shape), C138 (the listener), C139 (`acq web`,
  the human door, `web` the channel), C140 (item art is the
  browser's), all in `decisions/frontends.md`; C141 (the league beside
  the trade JSON) in `decisions/trade.md`. Also in force: C11 (`--json`
  on every command), C116 (no id in user-facing text, which a page
  is), C98 (the consumer holds the corpus and checks currency before
  every answer), C100 (one answer shape, with `next`), C113 and C119
  (the translation and what an ambiguous clause says), C44 and C111
  (apply refuses a moved policy and spends each request once), C65
  (intent writes carry their channel), C88 (the human door spawns
  when stderr is a terminal), C122 (pinned once, at the lowest crate;
  wording is a golden), C137 (a deadline is pinned by forcing its
  expiry).
- `SURFACES.md`, the `web.poecdn.com` row: the browser fetches the
  art, nothing else does.
- The verbs the pages render, through `crates/acquisition-verbs/src/`:
  `frontend.rs` (`Frontend::human(via)`, `Account`, `Opened`),
  `refresh.rs` (`Opened::plan`, `Opened::apply`, `Planned`, `Apply`,
  `Landed`), `jobs.rs` (`Frontend::jobs`, `submit`, `wait`). The
  search crate's `answer.rs` (`Request`, `Asked::read`, `Rows { limit,
  sort, desc, after }`, `Answer`, `Row`), `show.rs` (`Shown`),
  `trade.rs` (`decode_json`, `Decoded`), `trade/catalogue.rs` (the
  rows whose `says` is a control's tooltip), `trade/stats.rs` (the
  stat vocabulary the picker lists), `corpus.rs` (`Corpus::is_current`).
- The CLI as the pattern: `crates/acquisition-cli/src/main.rs`
  (`open_browser`; the `auth` verb's `--no-browser` and its two-document
  `--json`; the ad-hoc `refresh` submit with `{realm, league, all, tabs,
  deep}`; the `character` and `stash` submits), `search_cmd.rs` and
  `plan_cmd.rs` (what a text renderer of these documents already
  states once and leaves out, C53); the MCP's `search` tool
  (`crates/acquisition-mcp/src/main.rs`) as the thin-wrapper shape
  every route copies.
- `search/trade-query/data/filters-2026-09-12.json`: twelve groups,
  each `{id, title, filters: [{id, text, option? | minMax?}]}`; the
  form is generated from it, a field's name being its JSON path.
- The tests: `crates/acquisition-cli/tests/` (`main.rs` the one binary
  the web module joins, C122; `harness/`, the isolation list;
  `goldens/`; `cli_reference.rs` and `readme_tour.rs`, which
  regenerate and run the user-facing documents; `no_ids.rs`;
  `new_league.rs`, 2e's rehearsal; `apply_loop.rs` and `plan_json.rs`,
  C38's pins), `crates/acquisition-mcp/tests/pricing_tools.rs` (the
  equality-with-the-CLI shape), `acquisition-search/tests/trade_decode.rs`
  (where C141's pin lands).
- `tools/arch-check.sh`: the frontend door grep (no frontend opens a
  store, picks an account or compiles a plan itself) now reads the web
  module too; the edges do not change (C139).
- `deny.toml`: the licence allow-list axum must pass.

## Input

Fixed: the five rulings above, taken in the owner's words on
2026-10-25. The filter dump of 2026-09-12 and the stat table the search
ships. The fixture store the CLI's tests already use, with the
real-shaped mock items of 2c. The failing inputs the tests stage: a
request without the token; a request whose Host is not the server's;
a GET on a mutating route; a posted envelope whose policy revision has
moved (C44); the same envelope posted twice (C111); a form submission
naming a league the store does not hold; an item whose class the
table lacks and a line no template translates (2e's rehearsal, for
the moved-world line).

## Do

1. **C141 in the search crate first**: `decode_json` reads `league`
   beside `query` and `sort` as a URL's league, a term; the realm
   stays in the scope; any other key is still reported unsupported by
   name. Pin it in `trade_decode.rs` beside C113's pins, and add the
   envelope spelling to `--describe trade`'s words.
2. **The rows carry the icon when asked.** C140 needs the icon URL on
   a row, and a compact row carries none; `show` has it. Add the
   smallest slice of "a caller may name fields" (C100): a rows view
   asked for `fields: ["icon"]` puts the body's icon URL on each row,
   made where `show` makes it, so there is one maker. Pin it in the
   search crate's `rows.rs`. Nothing else of the card gap
   (`GUI-NOTES.md` section 7) is built.
3. **The stat vocabulary for the picker**: the search crate lists its
   stat entries — id, text, group, and each entry's status alone and
   with a bound (exact, ambiguous, unsupported) — from the table it
   ships, as one function the page's picker route serves. The page
   never reads the captured JSON.
4. **`acq web` in the CLI crate** (C139): a `web` module holding the
   server, its routes and its embedded assets (one HTML template set,
   one stylesheet, one small script for the stat picker, served as
   written, no build step). The verb binds 127.0.0.1 on a random port,
   mints the token, prints the URL (`--no-browser` prints it alone;
   `--json` prints `{"url": …}` as `auth` does, C11), opens the browser
   through `open_browser`, and runs until Ctrl-C. No lock, no reopen,
   no idle exit. The listener is C138 whole: the token is a path
   segment every link and form action carries; Host and Origin are
   checked against the bound address; no CORS headers are ever sent;
   mutation is POST only. The door is `Frontend::human("web")`.
5. **The corpus**: the server holds one corpus behind the account it
   serves and checks `is_current` before every answer (C98 as
   written); a stale one is reloaded and swapped. No per-request load.
6. **The search pages**, each a render of an existing document:
   - the form, generated at build time from the filter dump: a
     control per filter, its name the JSON path (`query.filters.
     type_filters.filters.category.option`, `query.stats[0].filters[0].
     id`, `…value.min`); a realm select and a league select, the
     league list being the counts-by-league answer over the corpus;
     the stat rows a fixed handful with the picker; the six trade
     filters and the status dropdown rendered disabled with the
     catalogue's `says` sentence as the title (listing-only); each
     control's fate from the catalogue shown beside it;
   - the fold: the submitted fields folded into the envelope by their
     names, empty fields dropped, the league from the select (C141),
     the realm into `scope.realm`, the view's `limit` and `sort` from
     the form; the whole handed to `Asked::read` as the MCP does;
   - the answer page: the translation's faithfulness and every
     clause's status first, as the CLI prints them (C113, C119), the
     total, the rows as a table (icon per C140 with `alt` text, name,
     base, rarity, place, the touched lines) and a "more" link
     carrying `next`; then the terms block and the basis;
   - one item: a row's name links to the `show` document rendered as
     a page, text and the icon, the lines in the order `show` gives.
7. **The refresh pages**:
   - the tabs page: the `tabs` document for the chosen league and the
     characters listing, a checkbox per tab and per character, no
     default selection; two buttons;
   - "Refresh selected": the ad-hoc `refresh` kind with the ticked
     tabs (`{realm, league, all: false, tabs, deep}`) and a
     `character` job per ticked character, submitted through the
     verbs' `submit`; this keeps the ad-hoc kind alive, which the CLI
     help marks to retire once nothing submits it — the page is now
     its second submitter, and the help's parenthesis is corrected to
     say so;
   - "Plan under the policy": `Opened::plan` rendered as `--plan`
     renders it, quote and notes included, with the envelope in a
     hidden field and an "Apply" button; apply posts the envelope
     whole to `Opened::apply`, as `--apply=FILE`; a moved policy is
     the refusal C44 words, a replay sends nothing (C111) and the page
     says so;
   - the jobs page after either: the `jobs` document, a meta-refresh
     tag of a few seconds while any job is non-terminal, none once all
     are. The jobs page observes and never spawns (`Frontend::jobs`).
8. **The user-facing documents**: `acq web` in the README tour and its
   generated help in `CLI-REFERENCE.md` (regenerated under
   `ACQ_UPDATE_FIXTURES=1`); the mock-session skill gains the one line
   that opens the UI in a mock world.
9. **The moved-world review**, run with 2e's rehearsal machinery over
   the pages: a new item class, an untranslated mod and the league
   mechanic's items each render with a badge, never a refusal or an
   empty page. The badge renders a state the answer already carries
   (undecided with its reason, unknown flags, an unread body); its
   pins live in the search crate, and the page's test asserts only
   that the badge renders that state. A path that refuses is fixed in
   the search crate, never in the page.
10. **The owner sits at it** in his own store, from a terminal, under
    the live-run skill, as the validating consumer (P2): the search
    pages first, the refresh pages once he has read the plan page.
    What stopped him is the report's sixth section.

## Done when

- `acq web` exists in the CLI crate; `tools/arch-check.sh` is green
  with no new edge and no new forbid line; the Linux gate's cargo-deny
  step passes with axum added (`deny.toml`, the licence allow-list).
- C138 is pinned in the CLI's test binary: without the token refused,
  a foreign Host refused, a GET on a mutating route refused, with the
  token answered, and no response carries a CORS header; each
  through the shared harness in a mock world, the URL read from the
  verb's stdout, no wait but the hang ceiling (C137).
- The fold is pinned by equality: a form submission's answer equals
  `acq search --trade <json> --json` for the same envelope over the
  fixture store, and a ticked selection's submit equals the CLI's
  `refresh --tabs` params.
- Every page is a wording golden under `ACQ_UPDATE_FIXTURES=1`, each
  naming the facts a reader must find (a count, a row's name, a clause
  status, a quote line, a badge), and `no_ids.rs` reads the goldens.
- The apply loop runs in a mock world through the pages: plan, apply,
  the jobs page to terminal, a second post of the same envelope
  sending nothing, a moved policy refused.
- C141 is pinned in `trade_decode.rs`; the icon field in the search
  crate's `rows.rs`; the stat listing by one test in the search crate.
- The moved-world review passes as Do 9 states, its pins in the
  search crate.
- No test asserts a wall-clock bound a slow machine can miss (C137).
- The README tour line runs; `CLI-REFERENCE.md` and the mock-session
  skill carry the verb.
- The gate is green on the step's own tree.
- The owner's sitting (Do 10) has happened and its stops are in the
  report, each answered at the close: taken, filed or declined by name.

## Not in scope

- SSE, the subscriber, reconnect, live invalidation, a `Searcher`
  loop, a revision push: 4-full, after 4-0 (`roadmap/TIME-SEAM.md`).
  The jobs page's meta-refresh is the whole of 4-lite's liveness.
- The dense table, the By-Tab tree, a virtualizer, a headless table
  library, rows carrying every field, the card as `GUI-NOTES.md`
  section 7 draws it: 4-full. Only the icon field is built (Do 2).
- The keeper bundle (#1), the spawn policy for a windowless launch,
  Tauri, the held corpus in `acq-mcp`: 4-full.
- Price writes and policy edits from a page: a design of their own,
  after the refresh pages have been used.
- Compiling a selection as a plan: the spend check reads the stored
  policy row and refuses a plan with none, so a transient plan is a
  planner change; the selection goes through the ad-hoc kind (Do 7)
  and the plan page through the stored policy. Reopen if the owner's
  sitting asks for a quote before a selected refresh.
- A build step, a framework, Node in the gate, TypeScript: the pages
  are served as written.
- Binding beyond loopback (the Not-doing line under C138), posting
  anywhere (C120), the outbound encoder (`decisions/trade.md`, parked).
- A spike: none is expected; the form is generated from a 17 KB data
  file and every page renders a document that exists.

## Rules

`TEMPLATE.md`, and:

- A page renders a document and nothing else: no sorting, filtering,
  grouping or composing in a template or a script. Where a page wants
  something no document carries, the verb or the search crate grows
  first, in its own commit, pinned there (C122).
- Every page is user-facing text under C116: no id, no store name.
- The server never opens a store, picks an account or compiles a plan
  itself; it calls the verbs (the arch-check door grep).
- A long-lived frontend resolves its account once, at its start, and
  names it on every call, reads and submissions alike: a submit never
  leaves the account to a fresh resolution by the daemon (corrected
  after the review of `fba1b03e`, its first finding).
- Nothing the build runs touches the real provider; the owner's
  sitting (Do 10) is his, from his terminal.

## Report

`TEMPLATE.md`.
