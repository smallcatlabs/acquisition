# GUI notes: the investigation, the synthesis and the plan

Provenance: Fable (`claude-fable-5-1`), one session with the owner on
2026-10-18 and 2026-10-19, read-only in the repo throughout. Three
subagents ran under it, none fetching any GGG surface (C79): a Sonnet
agent on the plumbing (protocol, client, store read API, search crate,
docs check, CI), an Opus agent on the shell and library landscape
(Tauri 2 and 3, egui, Slint, iced, GPUI, Leptos, Dioxus, Svelte,
Solid, TanStack, ts-rs, tauri-specta, testing and packaging), and an
Opus agent on the trade site's anatomy against the catalogue and the
stat table. The C++ app was read in the gerwaric clone at
`/Users/tom/Development/GitHub/gerwaric/acquisition` (its items model,
`src/items_model.h` and `.cpp`; the M1 to M3 results,
`docs/design/m1-m3-result.md` at `63980eb8`; the sort profile,
`docs/design/m3-sort-profile-result.md`). This file is phase 4's second
input (`roadmap/README.md`, "The phases"); it is dated evidence and a
design proposal, never a ruling. Where a number was measured it says
by whom; where it was inferred or is external research it says so.

The owner's rulings on this road already in force (`roadmap/README.md`,
2026-10-19, verbatim): "I accept all your recommendations exactly as
proposed with a loopback web UI", which fixes the GUI's first shape as
a loopback web UI on the shared function layer, Tauri a later wrapper,
and the corpus held in `acq-mcp`. The sections below were written
before that ruling and are kept whole, so the reasoning that led to the
same place is on the record; where a section argues for what is now
ruled, it is the Why.

Contents:

1. What the registry already fixes, and the parks this slice fires
2. The shell: Tauri, a loopback web app, egui
3. How a GUI plugs in
4. The query path: the site's form over the trade translation
5. The live results loop
6. Table performance: QTreeView against a virtualized web table
7. The card and dense-table data gap
8. Foundation work on the crates, in order
9. The ruling packet for the phase 4 design
10. Owner questions the trade-site anatomy raised
11. Risks
12. A build sequence inside phase 4
13. What was read

## 1. What the registry already fixes, and the parks this slice fires

The GUI is not a blank page. These rulings shape it (ids as of
2026-10-19, before the phase 1 governance reset renumbers or moves
anything):

- C12: a frontend consumes exactly two surfaces, the daemon protocol
  and the store's read API. The search crate is a library over the
  second, as the planner is.
- C16: Tauri for the GUI, webview frontend; egui considered and passed
  on. Note 36 says the GUI slice's first question is whether Tauri is
  revisited. The owner has now ruled the loopback web UI first and
  Tauri as a later wrapper; C16's amendment is phase 4's design item.
- C46: shared semantics live in Rust; every frontend has a Rust
  adapter; the webview is presentation, never a second implementation;
  a non-Rust frontend is a design event.
- C85: a subscription connection delivers invalidation hints; the
  subscriber snapshots, re-reads and resyncs. `acq jobs --watch` is the
  reference subscriber (`crates/acquisition-cli/src/main.rs`,
  `watch_jobs`). The daemon commits a tab's store row before it emits
  the job's terminal event, so a subscriber that re-reads on `done`
  sees the new revision; a `running` event precedes the commit.
- C88: the real provider is the default; a real-mode daemon is spawned
  only by a human with a terminal on stderr
  (`crates/acquisition-client/src/client.rs`, `human_present()` is
  `!ggg_mode() || stderr.is_terminal()`; `ConnectOptions::interactive`
  and `::autonomous`). `locator.rs` requires `acqd` beside the current
  executable.
- C98: no search service; a consumer holds the corpus and checks
  `Corpus::is_current` before every answer; nothing is persisted first.
  Neither the CLI nor the MCP holds a corpus today: the MCP's search
  tool calls the loader on every request
  (`crates/acquisition-mcp/src/*.rs`, the `search` handler's
  `.corpus(&store, &intent)`). The GUI is the first consumer the
  mechanism was written for.
- C100: one answer shape; the row is compact; the body comes through
  `show`.
- C104: text on every seat; a client holding a tree (a GUI's rows) may
  send it; every answer carries both.
- C113 and K6: a trade URL, id or query JSON is the third spelling of a
  query. The URL's realm is the scope and its league a term; id and
  JSON fill no scope and take no terms beside them; the site's sort is
  reported, never applied.
- C110 and `deny.toml`: MIT OR Apache-2.0 for the workspace, a licence
  allow-list for dependencies.

Parks whose trigger is this slice, with the triage ids from
`roadmap/parks-triage.md`:

| Park | Where | What the GUI does with it |
| --- | --- | --- |
| D-9 GUI-hosted daemon, refused | daemon | becomes the spawn-policy ruling (section 9, question 3) |
| D-10 `spawned_by` in hello and status | daemon | build if the GUI spawns |
| D-5 correlation ids on one connection | daemon | build only if the server multiplexes many pages over one client connection and proves it needs them |
| F-7 a watch that waits for the daemon to return | frontends | the GUI's subscriber, with reconnect |
| F-10 structured-error taxonomy beyond `kind` | frontends | the GUI as validating consumer (P2) |
| F-18 sticky default realm and league | frontends | the realm control; adapters only (C96) |
| F-2 queue-management UI | frontends | a rendering over the jobs list and events |
| F-5 results over a subtree | frontends | results on the event channel or a `results` verb, if the GUI shows which |
| PL-3 explicit-selection door | plan | the refresh gesture |
| SR-11 incremental update of a held corpus | search | section 8, move 3; "build if measured" |
| SR-1 saved queries, ST-1 `user.db` | search, store | saved searches, later |
| CTX-2 transcription lint | cross-cutting | fires only if C++ columns are ported from C++ files |

Also in force: the persisted-projection park (search), trigger "the
load over 500 ms release", whose ledger floor sits at 467 to 499 ms
(`search/M3-LEDGER.md`); the totals batch park, parked alongside it by
the owner on 2026-10-11; the outbound half of the trade translation (an
encoder from the language to the site's query), parked in
`decisions/trade.md` with the trigger "a consumer who needs a search
sent to the site".

## 2. The shell: Tauri, a loopback web app, egui

### 2.1 External research, dated 2026-10-18

These are the facts the stack subagent found; they rot.

- Tauri 2.12.4 stable (released 2026-10-14); 3.0.0-alpha.5 with an
  optional CEF runtime. IPC: `invoke` serialises JSON and is costly for
  large payloads; `tauri::ipc::Channel<T>` streams ordered messages;
  `ipc::Response` carries raw bytes. Synchronous commands run on the
  main thread, so commands over the corpus must be async. An informal
  figure for IPC on Windows is about 200 ms per 10 MB. WebKitGTK on
  Linux has known graphics issues. `tauri build --no-bundle` exists.
  Notarization is optional in the tool, but Gatekeeper blocks an
  unnotarized downloaded `.app` on macOS 15 and later. `tauri-driver`
  runs on Linux and Windows only; `tauri::test::mock_builder` and
  Vitest with `mockIPC` cover the two halves.
- ts-rs 12.0.1 stable; tauri-specta still at rc.25.
- Svelte 5.57.2, solid-js 1.9.17, TanStack virtual-core 3.19.6, TanStack
  Table headless (MIT). AG Grid Community is MIT but heavy and carries
  its own sort and filter engine.
- Rust-native: egui 0.37 with egui_table 0.10 and egui_kittest (tests
  under `cargo test`); Slint 1.18.1 (royalty-free licence requires a
  badge); iced 0.14 (stale); GPUI (git dependencies, churn); Xilem
  experimental. Leptos 0.8.22 and 0.9-rc and Dioxus 0.7.10 are web
  frameworks in Rust; Dioxus desktop is not Tauri.

### 2.2 Tauri, as C16 stands

Keeps web rendering, which is what C16 actually argued for: item grids,
filters, a 10,000-entry stat picker and rich coloured mod text cost
almost nothing in HTML and CSS. Costs: a bundler and an `.app`, which
forces the Gatekeeper and SmartScreen decisions that the platforms
slice tabled; WebKitGTK on Linux; IPC payload discipline on Windows;
a Node toolchain in the gate; several hundred crates added to the
workspace and the licence list; the spawn-policy amendment (a Finder-
launched app has no terminal). Within Tauri, Svelte over React because
it has no built-in table sort or filter to tempt a second
implementation.

### 2.3 A loopback web app (now the ruled first shape)

A fifth frontend binary (`acq-web`, or an `acq web` verb) with the
MCP's dependency edges: client, protocol, store, plan, search, never
the daemon. It binds a loopback port, serves an embedded static page,
exposes JSON routes to that page, streams invalidation hints over
Server-Sent Events, prints the URL and opens the browser.

Two facts in the tree make it cheap. Tokio, hyper, http and tower are
already in `Cargo.lock` through the client and daemon, so axum adds a
thin layer. The CLI already opens the user's browser for login
(`open_browser` in `crates/acquisition-cli/src/main.rs`, `open` on
macOS and `xdg-open` elsewhere), and the daemon already binds a
loopback listener for the OAuth redirect (`auth.rs`,
`TcpListener::bind("127.0.0.1:0")`).

What it buys:

- Nothing new at release: one more binary in the archives the CLI
  produces, beside `acq` and `acqd`. No bundler, no `.app`, no
  WebKitGTK, no `tauri-driver`. Gatekeeper and SmartScreen stay where
  step 2 of note 36 left them.
- The spawn park closes itself when the user starts it from a
  terminal: stderr is a terminal and the client door spawns `acqd` as
  the CLI does. Reconnect after a daemon restart is the CLI watch's
  case.
- The live loop is the C85 shape with no translation (section 5).
  `EventSource` reconnects by itself, which is most of F-7 at the
  browser hop.
- Many tabs share one corpus and one subscription.
- Loopback HTTP with gzip has none of Tauri's Windows IPC cost and
  streams NDJSON if a view ever needs it.
- Real browser devtools; Vite's dev server proxies to the Rust process;
  page tests are plain HTTP plus an optional headless browser, both in
  the Linux gate.
- It is a strict subset of the Tauri path: Tauri can later point its
  webview at the same server or embed the same assets over the same
  core. The reverse is not true.

What it costs:

1. Security is the one new concern (section 9, question 2). Any page
   the user visits can send requests to a localhost port, and this port
   can drive the daemon, which holds the OAuth session and sends to
   GGG. The standard answer: bind 127.0.0.1 only, random port, a
   per-launch token in the URL that is printed and opened, required on
   every request (a cookie set on first load, `SameSite=Strict`, since
   `EventSource` cannot set headers), and a Host and Origin check that
   defeats DNS rebinding. No CORS. The Jupyter model.
2. The routes must not become a third surface under C12. They are
   private to the page, token-gated, unversioned, and carry the search
   crate's own request and answer shapes unchanged.
3. Lifecycle is looser than a window: closing the tab does not stop the
   process; two launches collide on a port. A lock file beside the
   daemon's socket, a second launch that reopens the browser to the
   running instance, and an idle shutdown after some minutes without an
   SSE client cover it.
4. Browser-shaped limits: no native dialogs, tray, dock icon or global
   hotkey, and the page lives among the user's tabs. Irrelevant for a
   trade-site replica; relevant for an overlay or a clipboard price
   check, which this GUI is not.
5. The front-end toolchain question remains (section 9, question 7).
6. Browser variance (Safari, Chrome, Firefox) where Tauri gives one
   webview per platform. Noise for a form and a list.
7. Item icons, the league beside the query JSON, and rows carrying the
   shown item (sections 4, 7, 9) are unchanged by the shell.

Do not bind beyond loopback to reach a phone on the LAN; that changes
the security story entirely and belongs in a park of its own.

### 2.4 egui, the honest challenger

One language, one small binary, no WebKitGTK, egui_kittest tests inside
the existing gate. It loses on exactly the first deliverable: a visual
replica of a web page with a 10,000-entry autocomplete, nested stat
groups and coloured mod text. A one-day spike of the filter form in
both would settle it better than reading, and is only worth running if
C16 is genuinely reopened toward native. With the loopback ruling it is
closed unless the owner reopens it.

## 3. How a GUI plugs in

- A new frontend crate, `crates/acquisition-web` (name to be fixed by
  the design), with the MCP's edges plus axum and the embedded assets.
  Root `Cargo.toml` lists members explicitly, so the crate is added
  there. `tools/arch-check.sh` §5 reads the graph from cargo metadata
  and needs the new name added to the daemon's forbid list.
- No daemon or protocol change is needed for the first GUI. The
  `Request` and `Response` enums (`crates/acquisition-protocol/src/protocol.rs`:
  Submit, Status, Result, Cancel, SetPriority, List, Subscribe, the
  auth verbs, Quote, DaemonStatus, ResetTripwire, Dashboard; `Event {
  job }` and `ResyncRequired { missed }`; `MAX_FRAME_BYTES` 64 MiB)
  carry everything the subscriber and the queue views need.
- The shared function layer (roadmap phase 2a) is the base the routes
  call. Until it exists the routes are thin wrappers over the same
  calls the MCP makes; the point of 2a is that there is one such layer.
- Tests: handlers are plain functions over a core struct, tested
  against a fixture store by equality with the CLI's JSON document, as
  the MCP's tests are. `DAEMON-SPLIT-SLICE.md` names a must-pass case
  that fails the day a frontend gains a library target; the GUI crate
  is a binary with its handlers in a module, not a library crate.
- Dependency weight: axum and tower-http over the existing tokio and
  hyper; `include_dir` or `rust-embed` for assets; the `open` crate or
  the CLI's `open_browser`. All MIT or Apache-2.0.

## 4. The query path: the site's form over the trade translation

The form is the site's form, so it emits the site's query JSON, which
is already the third spelling a request accepts
(`crates/acquisition-search/src/answer.rs`, `QueryInput { text | tree
| trade }`; `trade.rs`, `Source { Url, Id, Json }`, `decode`,
`decode_json`, `inflate`). The translation labels every clause exact,
ambiguous, unsupported or listing-only, and the catalogue's `says`
sentence is the tooltip. The webview never learns the language.

What the anatomy subagent measured over the shipped tables
(`crates/acquisition-search/src/trade/stats.rs`, 10,303 ids; the
catalogue, 147 rows; the filter dump
`search/trade-query/data/filters-2026-09-12.json`, twelve groups):

| Stat table entries by status | Alone | With a bound |
| --- | --- | --- |
| exact | 12,719 | 8,571 |
| ambiguous | 1,120 | 5,268 |
| unsupported | 229 | 229 |

(The counts exceed 10,303 because an id can carry more than one row.)
So the picker can show an entry's fate before the user picks it, and a
bound changes it, which the form must show as the user types a bound.

Consequences and gaps:

- The translated text appears in an editable box; editing it drops the
  trade input and runs the edit as the user's own query, a rule the
  crate already has. Pasting a trade URL works for free. C119's adds
  and drops counts give every ambiguous control an inline "what is at
  stake" badge.
- The JSON spelling carries no league and takes no terms beside it,
  and there is no encoder. Either the trade input accepts an object
  with realm, league, query and sort, or the GUI composes a real URL,
  which needs the id encoder, the parked outbound half. A replica's
  "open this search on the trade site" button is precisely that park's
  trigger (section 9, question 4).
- The six trade filters (account, collapse, indexed, sale type, fee,
  price) and the status dropdown are listing-only over a private stash.
  Sort is listing-only; the GUI's sort should use the rows view's own
  `sort` (`view.rows.sort`), which exists.
- Weighted-sum groups translate only with one shared weight and no
  per-stat bounds, so most site weighted searches come back unsupported
  whole; the form can warn while the user edits.
- Line order: the derivation's lines are grouped by source
  alphabetically; the site shows them in the body's order. A card that
  mimics the site needs the body's order or a stable rule.

## 5. The live results loop

Simplest form first. A `Searcher` in the Rust process owns the store
handle, the intent file, one `Arc<Corpus>` and nothing per page. Two
invalidation sources, both cheap:

- the daemon subscription, for fast reaction and job progress; re-read
  on `done` events, since the store row is committed before the terminal
  event;
- a one-second poll of `Store::revision()` (one `SELECT id, fetched_at
  FROM responses ORDER BY id DESC LIMIT 1`,
  `crates/acquisition-store/src/corpus.rs`) and the intent revision, as
  the catch-all for CLI-made price writes, imports, and a daemon that is
  not running.

On change: mark dirty, debounce about half a second with a two to
three second ceiling, reload on a blocking thread, swap the Arc, and
send one SSE line carrying the new basis. Each page re-posts its own
request and receives a fresh answer. The server keeps no per-page
state, so every route is a pure function of corpus and request.

Rows: the page keys rows by item id and flags changed ones; scroll and
selection anchor by item id. The request's window is one rows view
whose `limit` grows as the user scrolls, because a continuation
(`after`) is refused across a changed basis by design; one request with
a growing limit re-runs cleanly on every basis change and the order is
deterministic, so the diff is sane. `DEFAULT_LIMIT` is 20
(`answer.rs`); the GUI names its own.

Numbers that decide the shape:

| Cost | Figure | Source |
| --- | --- | --- |
| whole reload, owner's copy, 22,721 items, release | 492 ms (empty query); ledger floor 467 to 499 ms | M3, M5; `search/M3-LEDGER.md` |
| per committed tab, resident consumer | one whole reload | M5 (measured 2026-10-09) |
| held corpus, peak | 214 MB, about 9.5 KB per item | M7 (a process that loads and answers once; steady figure not measured apart) |
| one page of 100 compact rows as JSON | roughly 30 to 60 KB | estimate from the `Row` shape, not measured |
| loopback fetch overhead | about a millisecond | general knowledge, not measured here |
| Tauri IPC on Windows | about 200 ms per 10 MB | informal external figure (section 2.1) |

A page of rows costs about a millisecond of transport, so sending a
page whole per update is fine. The windowed slice-and-delta protocol
(server keeps the ordered ids, diffs by id, pushes a compact delta,
page asks `rows(version, start, len)`) is the next shape if evidence
asks for thousand-row windows; it is more mechanism than the first
build needs. The whole reload per committed tab is exactly SR-11's
trigger: build whole, measure under a hundred-tab refresh in a mock
session, and fire the park on numbers.

## 6. Table performance: QTreeView against a virtualized web table

### 6.1 What the C++ app does and what it paid

QTreeView never materialises rows. It asks the model for `data()` on
the visible cells (about 40 rows by 26 columns) and paints them; the
rest is a scrollbar and a layout vector. The C++ model
(`src/items_model.cpp`, 561 lines) is a two-level tree, By-Tab (bucket
per stash tab or character, at most 576 items for a quad tab) or
By-Item (one flat bucket); 26 columns (Name, four flag columns,
property columns, DPS, pDPS, eDPS, ED, CD, cDPS, Price, Last Update,
ilvl, and others; `src/column.cpp`). Its cost was never the view: the
M3 sort profile found two regex evaluations per side on about ten
million comparisons per refilter at 1m items; M3 fixed it with
precomputed sort keys, sorting only expanded buckets, and bucket-scoped
insert, remove and move operations with persistent-index remapping.
The final M1 to M3 table (`docs/design/m1-m3-result.md` at `63980eb8`,
the S7 gate rows):

| Operation | 100k | 1m | Budget |
| --- | --- | --- | --- |
| unfiltered By-Tab refilter, default collapsed | 23.6 ms | 282.3 ms | 60 / 500 ms |
| broad filter, every bucket expanded | 76.6 ms | 825.9 ms | 150 ms / 1.2 s |
| By-Item flat refilter | 106.6 ms | 1,331.7 ms | 250 ms / 1.5 s |
| clean By-Item reactivation (key hydration) | 46.7 ms | 475.3 ms | 100 / 500 ms |
| expand one 576-item bucket, cold keys | 0.53 ms | 0.59 ms | 10 ms |
| one tab's delta into a visible bucket | 0.87 ms | 0.86 ms | 5 ms |
| By-Item merge after the fix | 4.0 ms | 32.4 ms | 50 ms |
| view overhead per delta batch | about 2.5 ms | about 2.5 ms | — |
| resident sort keys, worst shape | 21.8 MB | 223.9 MB | 300 MB |

Filtering and sorting dominate; the view adds a few milliseconds. The
row-operation protocol that made deltas cheap is the most intricate
code in that app.

### 6.2 The same view in a browser

Unvirtualized is out of the question: 22,000 rows by 26 columns is
about 600,000 cells and over a million DOM nodes, seconds to build,
hundreds of megabytes, poor scrolling.

Virtualized, the browser is at parity on the view layer. About 50 rows
and 1,500 cells live in the DOM whatever the result size; they restyle
and paint in one to three milliseconds, inside a 16 ms frame. Scroll is
a transform and a spacer height. Tree expand and collapse is a
recomputation of a flat visible-row index (microseconds at this scale)
and a re-render of the window. Column resize to contents, which the
C++ app schedules as an O(n) pass on every expand
(`ScheduleResizeTreeColumns`), is cheaper in the browser by measuring
visible rows only. A DOM cell is heavier than a delegate paint, perhaps
an order of magnitude, and that margin is invisible once the window is
bounded. (These browser figures are general knowledge, not measured in
this project.)

Where the rows live decides the design:

- The page holds every row of the answer as a JavaScript array and the
  virtualizer slices locally. At 22,000 items with 26 projected columns
  that is roughly 10 to 25 MB of JavaScript, fine, with instant local
  scroll, grouping and expand. At a million it is half a gigabyte or
  more and dead.
- The server holds the answer and the page asks for a window as it
  scrolls: `data()` across a socket, about a millisecond plus the slice.
  Scales to anything the Rust side holds, and stays correct when the
  basis changes under the user because the page never owns an order it
  must patch.

Recommendation: server-windowed, keyed by item id, with a client cache
of a few pages. Sorting, filtering and grouping are requests to the
search crate (C46), never page work.

The C++ By-Tab tree is two shapes the language has: bucket headers with
counts are a Counts view keyed on the tab; an expanded bucket is a Rows
view scoped to that tab. Collapsed by default, the page fetches headers
once and a bucket's rows on expand, which is the C++ D2 materialisation
rule for free.

Live deltas invert the C++ picture: the C++ app gets a tab's delta into
the view in under a millisecond after weeks of protocol work; the Rust
side today pays the whole reload, then a re-ask, then a keyed re-render
of 50 rows in a few milliseconds. The view is not the bottleneck and
the row-operation protocol is unnecessary: a keyed re-render anchored
by item id does what `changePersistentIndexList` does, in a few lines.

### 6.3 Two honest limits

The scale target is a corpus question, not a view question. The C++
app budgets a million items in about 300 MB of keys plus its item
objects. The Rust held corpus derives everything up front at about 9.5
KB per item (M7), so a million items would be about 9.5 GB. No table
technology changes that; if a million-item stash is a goal, the corpus
derivation or a store-backed non-held mode is the work. At the owner's
22,000 items every design above is comfortable.

Qt gives a header view, selection model, keyboard navigation, inline
editors, tooltips and accessibility for free; a browser gives none for
a table. A headless table library (TanStack Table with TanStack
Virtual, both MIT) supplies headers, column sizing and reordering,
selection state and the expand model while leaving sorting, filtering
and grouping in manual mode so the server stays the one implementation.
AG Grid Community brings its own sort and filter engine, which is the
second implementation C46 forbids. Hand-rolling is one to two thousand
lines of interaction code. That, not rendering speed, is the real cost
of a C++-style table on the web, paid once. Add the library when the
dense view is built, not before.

## 7. The card and dense-table data gap

A `Row` (`answer.rs`) carries id, name, typeline, base, rarity, place,
removed_at, the touched evidence and the sort value, and nothing else.
A site-style card needs frame, flags, properties, requirements,
sockets, every line with its source and flags, note and price; a dense
table needs per-column values. `show` has all of it (`show.rs`, `Shown
{ basis, place, item, class, category, lines, price, against }`), but
`show` is a full streaming read per item, so fifty cards would cost
about two seconds. The `--fields` construct is listed as not built and
"waits for a consumer that asks"; the GUI is that consumer.

The clean form is a rows view option that puts the shown item (or the
requested fields) on each row, made by the one maker `show` already is,
under C100. The GUI adapter must not compose cards from `Held.item`
itself, or there are two makers. The derived `Item` (`derive.rs`) holds
name, typeline, base, rarity, frame, ilvl, stack, reqlevel, note,
flags, properties, requirements, lines (`Line { source, flags,
template, numbers, text, rows }`), sockets, cell and unread, so most of
the 26 C++ columns resolve to fields the derivation has; the DPS family
needs checking against what the pseudo fields compute. The icon URL is
in the stored body only (section 9, question 5).

## 8. Foundation work on the crates, in order

### 8.1 Where the time and the memory go

From M1, M3 and the read loop in `crates/acquisition-search/src/corpus.rs`
(`Corpus::read`: stream rows under one snapshot, `derive(facts,
&body)`, classify, categorise, then `name_sockets` and the price join
over the whole vector):

| Stage of one whole load, 22,721 items, release | Cost | Source |
| --- | --- | --- |
| header and locations | 6 ms | M1 |
| stream every body out of SQLite | 36 ms | M1 |
| parse every body as JSON | about 113 ms | M1 (`parse` row) |
| derive, classify, categorise, join prices | about 340 ms | by subtraction from the 492 ms empty query; not measured apart |
| evaluation of the dearest query | adds about 29 ms over the floor | M3, earlier build |
| stored bodies, total | 29.9 MB | M1 |
| held corpus, peak | 214 MB, 9.5 KB per item, 7 times the body | M7 |

Three things follow. The database is not the cost, so a projection
that only avoids the read saves little. The derivation is the cost, and
it is a pure function of body and facts under C103, which is what makes
it incremental. The 214 MB is a peak including the price join and
SQLite transients; the steady figure of a long-lived holder was never
measured apart (M7 says so).

The store already has what an incremental refresh needs
(`crates/acquisition-store/src/lib.rs` module doc): `items` is the
only place an item lives, one row per GGG id, keyed by full location
coordinate; ingest records `item_events` (added, moved, changed,
removed) per response; the revision advances in the transaction that
changes items, locations or membership (C108).

### 8.2 The moves

1. Measure the four unmeasured numbers: the ask over a held corpus
   (evaluation alone); the steady resident size after the load's
   transients drop; the split of the 340 ms across derive, classify,
   categorise and price join; the byte split of a held item across
   lines, properties, flags and strings. A bench binary, a day,
   measured alone on the machine per the roadmap's rule. Every later
   decision reads from it. New rows go to `search/MEASUREMENTS.md`.
2. The MCP holds its corpus and reuses it while `Corpus::is_current`
   says so (already ruled by the owner, 2026-10-19, and listed in
   phase 4). An agent's ask drops from about 480 ms to the evaluation
   alone between refreshes. Smallest change on the list.
3. Incremental refresh of a held corpus (SR-11). A corpus at revision
   R applies the `item_events` since R: re-derive the changed rows (at
   most one tab, 576 items, about 10 ms by proportion of 340 ms over
   22,721; inferred, not measured), drop the removed, re-run socket
   naming and coverage, re-join prices only when the intent revision
   moved, refresh tables never (they are versioned in the `Basis`).
   The park names the shape: the basis's components, facts, intent and
   tables, each refreshed alone. The pin is equality: a corpus
   refreshed through events equals a whole load at the same revision,
   asserted over a mock session. This gives the GUI its live loop and
   lets the MCP's held corpus survive a refresh. The CLI gains nothing
   here.
4. Rows that carry the item's fields (section 7), the one maker over
   the shown item. Serves the GUI, agents asking the MCP for columns,
   and a CLI table.
5. A memory diet for `Held`, only if move 1 says so. Templates are
   6,148 distinct strings over 22,721 items (M3 census), and sources,
   flags and property names repeat the same way, so interning is the
   obvious move. The serialised shapes stay as they are; an internal
   representation can print the same JSON.
6. A faster derivation before any projection: parsing 30 MB of JSON in
   113 ms and deriving 22,721 items at about 15 microseconds each both
   have in-process headroom (typed or borrowed parsing, fewer
   allocations), likely two to three times (a judgment, not a
   measurement). Shortens the CLI's floor without a cache to keep
   coherent.
7. The persisted projection, last, as the experiment the park
   describes. Its real beneficiary is the CLI, a process per ask. It
   costs an invalidation story keyed on the derivation version
   (`DERIVATION` is 12 and moves often) and either the daemon deriving
   at ingest or a lazy fill. After moves 2 and 3 the resident consumers
   do not need it; after move 6 the CLI may not either.

Nothing above touches C98: a held corpus with incremental refresh is
still a consumer holding its own corpus, checked before every answer.
The one shape to avoid is the CLI asking a resident process for
answers, which would be the service C98 ruled out; keep it out even
once the GUI's server exists.

## 9. The ruling packet for the phase 4 design

Each question with the recommended answer. The owner answers verbatim;
answers land in the registry under the governance of the day.

1. **C16's amendment.** Recommended: "Web rendering, served from a Rust
   frontend binary over loopback; Tauri is the shell to add if a
   windowed app is wanted." Already ruled in substance on 2026-10-19;
   the wording is the design's.
2. **The listener's security, a new ruling.** Recommended: the server
   binds 127.0.0.1 on a random port; mints a per-launch token carried
   in the printed and opened URL and required on every request
   thereafter (a `SameSite=Strict` cookie set on first load); refuses
   any request whose Host or Origin is not its own; serves no CORS; and
   its routes are private to its page, unversioned, carrying the search
   crate's request and answer shapes unchanged, so they are not a third
   surface under C12. Binding beyond loopback is parked with its own
   trigger.
3. **The spawn policy for a frontend without a terminal (D-9, C88).**
   Recommended: a frontend states human presence explicitly through
   `ConnectOptions`, replacing the stderr sniff as the mechanism; the
   loopback server started from a terminal is the CLI's case and spawns
   as the CLI does; a frontend launched with no terminal (a double-
   clicked binary, a future Tauri wrapper) is human presence by its
   window, which the ruling says in words. The `.app` or archive must
   carry the same `acqd` artifact as the CLI's or the two will replace
   each other's daemons (`locator.rs`). D-10's `spawned_by` is built
   with it if two spawners can exist on one machine.
4. **The league beside the query JSON.** Recommended: build the
   encoder (the parked outbound half), since the replica's "open on the
   trade site" button is its trigger, and the GUI composes a real URL
   that carries realm and league. The alternative, a `trade` input
   object with realm, league, query and sort, is smaller but serves
   only the inbound half.
5. **Item images in or out.** The icon URL is in the stored body only;
   loading it means the browser fetches images from GGG's CDN outside
   the daemon. Invariant 1 covers daemon-originated API traffic; C79
   wants a surface registered deliberately. Recommended: text-only
   cards for the first build; a decision and a `SURFACES.md` row before
   pictures.
6. **The table's shape, a ruling for the GUI.** Recommended: the
   GUI's table is server-windowed and keyed by item id; the page holds
   at most a few pages; sorting, filtering and grouping are requests to
   the search crate; the By-Tab tree is Counts keyed on the tab plus
   per-tab Rows.
7. **The front-end toolchain.** Options: Svelte 5 with Vite and
   TypeScript types from ts-rs (Node joins the gate and the build), or
   no build step (ES modules served as written, one pinned vendored
   virtualizer file, types checked by `tsc` alone or not at all).
   Recommended: decide at the first spike; lean to the smaller
   toolchain unless the form's complexity asks for a framework.
8. **Sort.** The site's sort is reported, never applied; the GUI sorts
   through `view.rows.sort`. No price sort across currencies exists
   without a rates source (parked under pricing). Recommended: expose
   the language's sort keys and no price sort until rates exist.
9. **The listing-only trade filters.** Render disabled with the
   catalogue's sentence as the tooltip, or map to stash analogues
   (`has:priced`, `tab.public`), which needs rulings and breaks the
   all-or-nothing trade input. Recommended: disabled for the first
   build.
10. **The sequence across note 36 and the roadmap.** Moves 1 to 3 of
    section 8 help the MCP directly and are small; the roadmap already
    places the held corpus in phase 4 and the keeper bundle as phase
    4's first pre-work. Recommended: moves 1 and 2 before or beside
    phase 2a, since 2a's shared layer is where the held corpus lives;
    move 3 inside phase 4 after measurement; moves 4 to 7 inside phase
    4 or later, each gated on move 1's numbers.

## 10. Owner questions the trade-site anatomy raised

- Weighted-sum groups: warn while editing, or refuse the group whole?
- Fonts and frame art: Fontin is free; FrizQuadrata and GGG's art are
  not. A replica's look stops short of GGG's assets.
- Picker contents: the stat table's 10,303 ids with status, or the
  catalogue's rows alone?
- Autocomplete source for names and bases: the store's `items_names`
  index, or the reference tables?
- Ambiguous labelling: badge per control, or a summary line under the
  form?
- Packaging: a browser-downloaded binary meets Gatekeeper on macOS 15
  and later, the same as `acq` today; a GUI release does not add a new
  decision under the loopback shape, but a future Tauri wrapper does.

## 11. Risks

- WebKitGTK only matters if Tauri returns; the loopback shape avoids
  it.
- The whole reload per committed tab is felt the day the GUI runs
  during a refresh; SR-11 is the answer and is gated on measurement.
- Node in the gate lengthens cold builds and adds a toolchain to
  maintain; the no-build option avoids it at the cost of a framework.
- A loopback listener is an attack surface the project has not had;
  question 2's ruling must land before the server serves anything that
  can submit a job.
- Two makers for the card (adapter reading `Held.item` beside `show`)
  is the fault C46 and C100 exist to prevent; move 4 of section 8
  prevents it.
- The million-item scale is out of reach of the held corpus as
  derived; say so rather than design the table for it.

## 12. A build sequence inside phase 4

1. The design session: questions 1 to 9 ruled; C16 amended; the
   security ruling written; the spawn ruling written; images decided.
2. Move 1 of section 8, measured alone on the machine, rows in
   `search/MEASUREMENTS.md`.
3. The keeper bundle (D-4, F-6, F-20, D-14), per the roadmap, and the
   corpus held in `acq-mcp` (move 2).
4. A timeboxed spike of the `Searcher` loop alone, headless: hold,
   invalidate from the subscription and the revision poll, debounce,
   reload, swap; measured during a mock refresh of many tabs. Decide
   SR-11 on the numbers.
5. The web crate: listener with the security ruling, lifecycle (lock,
   reopen, idle exit), the SSE line, the rows and counts routes as thin
   wrappers over the shared layer, the subscriber with reconnect (F-7).
6. The form from the committed filter definitions, emitting trade JSON;
   the translated text in its box; the fate badges; the encoder if
   question 4 says so.
7. Rows carrying fields (move 4); the card renderer; then the dense
   table with the headless library and the By-Tab mode from Counts and
   per-tab Rows.
8. Queue views over the jobs list and events (F-2, F-5 if proven, F-10
   as the validating consumer, F-18's realm control).
9. Hardening and the owner's own seat as the validating consumer (P2).

## 13. What was read

In this repo, read-only: `README.md`, `CONTEXT.md`, `AGENTS.md`,
`decisions/frontends.md`, `decisions/search.md`, `decisions/trade.md`,
`decisions/daemon.md` and `decisions/store.md` (parks),
`brainstorming-notes/36-the-road-to-the-agent-release.md`,
`SEARCH-SLICE.md`, `TRADE-SLICE.md`, `search/README.md`,
`search/M3-LEDGER.md`, `search/MEASUREMENTS.md`, `SURFACES.md`,
`PLATFORMS-SLICE.md`, `DAEMON-SPLIT-SLICE.md`,
`crates/acquisition-protocol/src/protocol.rs`,
`crates/acquisition-client/src/client.rs` and `locator.rs`,
`crates/acquisition-cli/src/main.rs` (`watch_jobs`, `open_browser`) and
`dash.rs`, `crates/acquisition-search/src/answer.rs`, `corpus.rs`,
`show.rs`, `derive.rs`, `eval.rs`, `trade.rs`, `trade/stats.rs`,
`trade/catalogue.rs`, `crates/acquisition-store/src/corpus.rs` and
`lib.rs`, `crates/acquisition-daemon/src/auth.rs` (the loopback
listener), `crates/acquisition-mcp/src/*.rs` (the search tool),
`search/trade-query/data/filters-2026-09-12.json`,
`stats-2026-09-12.json`, `grammar.json`, the `reference/` tables,
`tools/docs-check.sh`, `deny.toml`, `.github/workflows/gate-linux.yml`,
`rust-toolchain.toml`, root `Cargo.toml` and `Cargo.lock`,
`roadmap/README.md` and `roadmap/parks-triage.md`.

Outside it: the gerwaric clone's `src/items_model.h` and `.cpp`,
`src/search.h`, `src/column.cpp`, `src/ui/mainwindow.cpp`,
`tests/m3_holdpoint_benchmark.cpp`, `docs/design/m1-m3-result.md`,
`docs/design/m3-sort-profile-result.md`, `docs/design/items-pipeline-m3.md`;
the awakened-poe-trade sibling clone (Vue 3 and Tailwind filter rows;
no full item card), by the anatomy subagent.
