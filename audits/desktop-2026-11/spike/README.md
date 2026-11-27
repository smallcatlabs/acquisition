# The desktop spike: a window over the engine, laid out like the C++ app

The record of `roadmap/briefs/desktop-spike.md` at `9f5594c1`. One
question: does a window over the engine the tool has feel like the C++
app, or does it need a native toolkit? It is answered by a throwaway
window the owner sits at over a copy of his own stash, built, watched
and changed. This file names the rig and the bar before any judged run.
The runs, the sitting and the verdict follow it in later commits.
`rig-1.patch` is the rig the build's review read, `rig-2.patch` the rig
as its answers left it (below, "The review's answers"), and
`rig-3.patch` the rig after the first judged runs' miss (below, "rig-3").

## The rig

A package of its own, `spikes/desktop-rig`, in a lane (a `cp -cR` clone
of the main tree at `9f5594c1`), never committed as code. Each
`rig-<n>.patch` beside this file is the lane's whole diff from
`9f5594c1`, the newest the rig as it stands: the package,
and the two lines that add it to the workspace (`Cargo.toml`'s members,
and `Cargo.lock`'s entry for the package itself, the one change the lock
takes: no crate is added, and the rig builds `--offline`). It applies to
its base, from the repository's root:

```sh
git worktree add --detach <dir> 9f5594c1
git -C <dir> apply --check "$PWD/audits/desktop-2026-11/spike/rig-3.patch"   # exit 0
```

Applied there, each file it makes is the lane's byte for byte (`cmp`
of every file the patch names). A blank context line in it has lost its
one space, so the patch passes the gate's whitespace check; `git apply`
reads it so.

What it is, read off the working window:

- **A loopback server** (`desktop-rig`, Rust, the axum the lock holds)
  that opens the account once through the verbs' door
  (`Frontend::human`, `Account::open`) and holds the corpus in the
  search crate's `Holder`, which checks it is current before every
  answer (C98, one basis per answer). It binds 127.0.0.1, refuses a
  request whose Host is not its own, and serves its page under a content
  policy that admits this server alone, item art included: the window
  loads nothing from off the machine.
- **One hand-written page** (HTML, CSS and a script, no framework, no
  bundler, no Node) that draws what the server answers and keeps no
  state the server cannot rebuild from what the page sends: the form's
  text, the league, the view, which places the person opened or closed,
  the order, the item opened, and the window of rows wanted. All but the
  window are written into the page's URL as they change, so a reload
  asks the same (C16, the page's identity is its URL); the scroll
  position is not.
- **Every list is an answer.** The tree's places are the search crate's
  places document (`Places::tabs`, `Places::characters`) for the league;
  under a query each place's matches are the query's matches counted by
  the place they lie in (Rule count); a place's rows are its own request's
  answer; the flat
  list is one answer of the query over the league, the league's term as
  the answer's count of `league` writes it; an item's card is the answer
  to its id with the item asked. The page sorts, filters, groups and
  composes nothing (C46, semantics in Rust; C16, the page is
  presentation). The rules the rig had to write in Rust are listed below.
- **Reads only.** The start script runs the rig over a fresh clone of
  the store copy under the read environment of
  `audits/whole-review-2026-11/INVESTIGATOR.md`; no daemon starts and
  nothing is sent to GGG. The facts and the intent each open through the
  store's read-only door (`Account::facts_read`;
  `Annotations::open_read_only_for`, by the account's uuid, since the
  verbs' door opens the intent for writing), never made nor migrated
  (C124, a reading tool opens the store read-only). The file the rig
  writes is the sitting's log. A session over a fresh clone left each of
  its 23 files as `shasum -a 256` found them before.
- **A check of its rows and counts**: `desktop-rig --check <league>`,
  under the read environment, opens every place of the league under no
  query and three, and fails unless the rows drawn are the rows counted,
  each at its index; each place's rows are its own request's answer, in
  its order (200 places a case); and under a query every place's
  matches, its own matches and whether it is listed are the places
  document's asked that query (all 3,462 of Standard's). On the lane's
  build of rig-3, Standard: every case holds.
- **The driver and the bar's judge**: `drive.py` plays the four
  scenarios below into headless Chrome as trusted input through its
  debugging protocol, on a profile under the builder's scratch;
  `judge.py` applies the bar to a run's lines or to a sitting's log.
  Both use Python's standard library alone.

**The start command**, from the repository's root, once the rig is
built into `runs/desktop-spike/rig/`:

```sh
runs/desktop-spike/start.sh
```

It clones `runs/desktop-spike/store-2026-11-24` with `cp -cR` into
`runs/desktop-spike/sittings/<time>/`, starts the rig there, and opens
the window as a Chrome app window (no tabs, no address bar) at the C++
app's size, on a profile of its own under `runs/desktop-spike/profile`
with Chrome's background networking off. Closing the window stops the
rig, a second after the window, so the page's last acts land. Every
act goes to `runs/desktop-spike/logs/sitting-<time>.jsonl` as soon as
its paint is timed; one the server does not take is kept and sent again,
and the status bar says how many wait.

## What the C++ app does, which the rig follows

Read from `../acquisition-qt` at `acquisition-qt@da7b3cd4`: the user
guide (`docs/user/searching.md`, `getting-started.md`), the design note
`docs/design/folder-tree.md`, and the main window's description in
`src/ui/mainwindow.ui` and `src/ui/mainwindow.cpp`, read for what the
window does. No code, stylesheet or asset is taken (C110, nothing
derives from the C++ tree).

1. The window opens at 1280 × 720 (`mainwindow.ui`).
2. Its body is three panes side by side: the search form, the item tree,
   the item panel; the form and the tree share the width about 2 : 5,
   and the panel keeps its own width (`mainwindow.cpp`, the splitter's
   stretch factors; the form scrolls and has a 150 px minimum).
3. Above the body, the search's tab, captioned with its matches
   (`Rings [42]`); below it, a status bar (`searching.md`).
4. Under the tree, a "View:" selector: *By Tab*, the items grouped under
   the stash tab or character they came from, or *By Item*, one flat list
   under one *All Items* header (`searching.md`; `mainwindow.ui`).
5. The form's top rows are Tab (a part of the tab's or character's name)
   and Name (a part of the item's name) and Rarity; then grouped ranges
   and check boxes (`searching.md`, "Filters"). A number field that holds
   no number is drawn red and does not filter; a comma before exactly
   three digits is no number.
6. A search updates as you type: a change starts a 350 ms timer and the
   list refilters when it fires (`mainwindow.cpp`,
   `SEARCH_UPDATE_DELAY_MS`).
7. In *By Tab* the roots are stash folders, tabs outside a folder, and
   characters; a folder holds its tabs; a tab or a character holds its
   items (`folder-tree.md`, "Goal").
8. Folders start open, tabs closed. With a filter active, a tab with a
   match shows open and a folder shows only where one of its tabs
   matches. A tab opened or closed under a filter is forgotten when the
   filter is cleared; the tabs opened without one are open again
   (`searching.md`, "Item list").
9. Selecting an item fills the item panel after a 100 ms timer, in place
   (`mainwindow.cpp`, `CURRENT_ITEM_UPDATE_DELAY_MS`): its name, a
   *Tooltip* tab in the game's style, a *Text* tab, *Hide*, and where it
   lies (`mainwindow.ui`).
10. A column head sorts by its column (`searching.md`).
11. No act loads a page: the app is one native window.

## Departures

- **One search.** No row of search tabs and no `+`: the one tab is the
  caption.
- **The league is chosen in the window**, from the realm's leagues with
  their items, the largest first (Standard, 43,433 of the 44,090 pc
  items). The C++ app picks one at login.
- **The form is a part of the C++ app's**: Tab, Name, Rarity; ranges for
  quality, level, item level, map tier, stack, sockets, links, required
  level, total life and total elemental resistance; corrupted, fractured
  and priced. Its Type, offense, defense and mod rows are left out. A
  Query box takes the search's own language, which the C++ app has not.
- **The columns** are Name (name and base), Price (the amount), Q,
  Stack, Lvl, ilvl, Links, Sock, R.Lvl, Life and Res: the values the
  search answers as numbers. A head click orders by the column, largest
  first, again reverses; the Name head returns to the store's order (the
  search sorts by numbers, not by names).
- **No debounce.** A change asks at once; while one answer is on its way
  the next waits, and a later change of the same kind overtakes it (the
  timing below).
- **No writes and nothing they need**: no buyout controls, no check
  boxes, no refresh, no context menu, no extended selection.
- **No splitters, no event log, no rate-limit button.** The panes keep
  the C++ proportions, the item panel 340 px.
- **The item panel has no art** (the window loads nothing off the
  machine) and no minimap: where the item lies is a line of text, its tab
  and its cell. No upload and no copy for Path of Building.
- **Keys**: Up and Down select the next item and open it; the C++ tree's
  Left and Right are not built.
- **No Back.** A change replaces the URL rather than adding to the
  history; the C++ app has no Back, and a design that wants one decides
  which acts are a new entry.
- **A head shows its count**, the matches under a query, its items
  otherwise.

## The rules the rig wrote in Rust

Each is the engine's to have, or to rule out, in a design. Each is in
`src/rules.rs` under its name.

- **Fold**: the form's fields as one query in the search's language.
  `acq web` folds its own form, not this one.
- **Open**: which places stand open (claim 8). The places document says
  what each place holds and matches, not what is open.
- **Flatten**: the open tree as one list of rows by index, and a window
  of it. The places document is a tree; a scrolled list needs rows.
- **Hold**: the league's places documents asked with no query, with each
  place's own terms (its request's text there), held per basis and
  league; each place's matches and its rows, once answered, held for one
  league, query and order at the basis; each reused only while the
  holder finds that basis current before the act.
- **Count**: under a query, each place's matches from one counts answer
  of the query over the realm, keyed `tab` and `character` (a tab's
  bucket names it by id, a character's by name, which the game keeps
  unique); a tab of pages, whose pages' items the `tab` count files
  under the tab, has its rows answered (Rule batch) and each page and the
  tab itself counted from them; a place holding others counts theirs
  beside its own; a place is listed where it holds a match. The check
  holds every count to the places document's.
- **Batch**: the window's places not answered yet are one request, their
  own terms joined by `or` before the query, and its rows parted by the
  place each lies in. The check above holds the parted rows to each
  place's own answer.
- **Card**: the item's card from the answer to its id with the item
  asked.
- **Read door**: the intent opened read-only by the account's uuid; the
  verbs' door has a read-only opener for the facts and none for the
  intent.

## How an act is timed

The page logs every keystroke, click, expansion, scroll, sort and
change of view or league as one act, and the start script's log holds
them all. An act's time runs from its input to the paint that shows
the result it asked for:

- **The input** is the event's own time stamp (`timeStamp`, the clock
  of `performance.now()`); for a scroll, the first scroll event of a
  frame. Each act also records when its handler finished (`input`).
- **The paint**: the page changes the DOM, then in the next frame posts
  a task from inside the frame callback; that task runs once the frame's
  rendering is done, and its time is the paint. A change made inside a
  frame callback (a scroll drawn from rows already held) posts from that
  callback, so it is timed in its own frame. The frame reaches the screen
  up to one display interval later, which no share counts.
- **The result it asked for**: a keystroke, a sort, a view or a league,
  the list answered for it; an expansion, the tree with the place opened
  or closed; a click, the item's card; a scroll, the rows at the scrolled
  place. An answer carries its whole basis (C98's: the store and account,
  the snapshot, the intent revision, the build and every table's
  version), which the act records; a scroll drawn from rows already held
  records the basis those rows were answered at.
- **One answer painted before the next is asked.** Each kind of answer
  (the view, the card) has one request in flight, and the next is sent
  only once the answer before it has been painted, so no answer is
  replaced in the DOM before its frame: an act logged as answered was
  drawn. Each act records its send and its paint (`t_send`,
  `t_paint`).
- **Overtaken**: an act waiting to be sent when a later act of its kind
  comes (another keystroke, another scroll, another click) is logged as
  overtaken, with no time: what it asked for was never drawn. A scroll
  whose answer comes when the viewport has moved off its place is
  overtaken too. An expansion is never overtaken; each one's change rides
  in the next request. A query the search refuses is logged as refused,
  timed to the paint of the refusal's words.
- **The shares**, which sum to the whole:
  - *engine* = the wait for an earlier answer still on its way (`wait`),
    plus the server's wait for the engine (`queue`), plus the engine's
    time from taking it to the serialized body (`engine`), both read
    from the response's `Server-Timing` header, written after the body
    was made. The next request leaves once the earlier answer is
    painted, so `wait` also carries that answer's transport and its
    browser work, a few milliseconds, counted as the engine's;
  - *transport* = from the request's send to its body's last byte, less
    the server's `queue` and `engine`;
  - *browser* = from the input to the request's send, less `wait`,
    plus from the body's last byte to the paint (parsing, the DOM, the
    frame).
  An act answered from rows already held has no request: its whole time
  is the browser's.
- The server's parts of the engine's share stand beside each act:
  the currency check, each places document, the flattening, the window's
  answer, and how many answers it took.

## The bar

The C++ app's own waits are the reference: it shows nothing for 350 ms
after a change to the form, then refilters and re-sorts. Its own notes
measured that whole step (`MainWindow::OnItemsRefreshed`: the filter,
35.2 ms of it, then the sort of every tab's items, 386.8 ms) at a median
of 422.5 ms at a 100k-item synthetic preset, before its sort was made
faster (`docs/design/m3-sort-profile-result.md`, "Live full-refilter
split", at `acquisition-qt@da7b3cd4`; no figure at 44k items). It fills
the item panel 100 ms after a selection. It records no figure for opening a tab
or scrolling; its own design notes take the 60 Hz frame, 16.7 ms, as the
budget for any stall (`docs/design/m1-m2-result.md`).

A run is the four scenarios in turn, in headless Chrome at 1280 × 720,
the rig built `--release`, and passes when it meets every line:

1. **A keystroke to its refiltered list.** (a) The last keystroke of
   each burst (the last before half a second without one): 9 in 10
   within 350 ms, sooner than the C++ app starts to refilter, and every
   one within 700 ms. (b) Every answered keystroke within 700 ms.
2. **A click to the opened item**: 9 in 10 within 100 ms, the C++ app's
   own timer, and every one within 200 ms.
3. **A place opened or closed to its rows**: 9 in 10 within 100 ms and
   every one within 200 ms.
4. **A scroll through the largest list**, *By Item* over Standard,
   43,434 rows: (a) 9 in 10 scrolls drawn at their place within 100 ms
   and every one within 250 ms; (b) during the steady wheel alone, at
   most 1% of frames late, a late frame being longer than one and a half
   times the display's interval, which is the median of the frames
   sampled for a second and a half at rest before the wheel. Headless
   Chrome paces its frames by its own clock; this line is judged for a
   real display only at the sitting.

A run whose rows `--check` fails is void. The bar holds when three
headless runs taken alone on the machine pass every line. A run taken
while a build, a gate or another measurement runs is exploratory and is
repeated alone before it is judged (C185, a measurement runs alone).
A miss the rig can answer is a turn of the loop: its cause found from
the split, the rig changed, the new patch committed beside this one,
and the runs judged again. A miss only a product crate can answer ends
the loop, and the verdict says so.

**The scenarios** (`drive.py`):

- *keystrokes*: typed into the form and cleared with Backspace: Name
  `tabula` (a key every 150 ms) and `chaos orb` (every 60 ms), Tab
  `dump` (150 ms), and in the Query box `pseudo.total_life>=70` and
  `rarity=unique ilvl>=80` (100 ms). In the Query box most prefixes are
  refused by the search; they are logged as refused, not judged.
- *expansions*: with no query, the closed heads in view opened one at a
  time, six, then each closed; twice.
- *clicks*: the query `rarity=unique` typed, then ten items in view
  clicked, 600 ms apart.
- *scroll*: *By Item*, a second and a half at rest, a wheel of 25 px
  every 1/60 s for 10 s (1,500 px a second), then ten jumps of the
  scroll position to fractions of the list (as dragging the thumb), each
  held until the page is idle.

## What building showed

Exploratory runs while the rig was built, with other sessions' builds
on the machine (load averages 3 to 12), so none is judged. In every
one, the browser's share of an answered act was a few milliseconds
(median 1 to 8 ms; 17 ms for a scroll while its paint was timed a frame
late, 3.6 ms since it is timed in its own frame), and the transport's
about 1 ms; the time was the engine's.

- The first rig answered each place in a window by its own request: a
  keystroke under a query that opens many tabs took 33 answers, 500 ms
  of engine. A place's request puts the query's terms before the place's,
  so each answer evaluates the query over the whole corpus, about 12 ms
  each; opening all 3,298 of Standard's places one by one took 41 s.
- The rig now holds what it answered at the basis (Rule hold): an
  expansion or a scroll asks the engine for nothing new but the places it
  has not answered.
- Joining the places' whole requests by `or` cost about as much (the
  query is evaluated once a place). Joining their own terms before the
  query (Rule batch) took a window's rows to 11 to 90 ms.
- Left in the engine's share of a keystroke, about 200 ms: the places
  document for the tabs, about 100 ms (it reads the store's tab listing
  again on each query, 60 ms, and evaluates the query), the one for the
  characters, about 40 ms (it evaluates the query again), and the
  holder's currency check, 10 ms, taken on every act.
- One answer of every match builds its rows at about 14 µs each: the
  43,433 rows of an empty query over Standard, 630 ms.
- A keystroke typed while an answer runs waits for it (`wait`): the
  engine cannot be stopped once it is asked.

## The review's answers

The build's one review, `roadmap/reviews/desktop-spike-98dd155f.md`,
read `rig-1.patch`. Each finding is answered by one rule in
`rig-2.patch`, and each was checked on the working window, headless,
over a fresh clone:

1. *An act logged as answered whose result was never painted* (two
   clicks at once, the second card replacing the first before its
   frame). Rule: a channel sends its next request only once the answer
   before it is painted. Checked: two clicks dispatched together, the
   second sent 0.2 ms after the first's paint, each answered.
2. *The intent opened by a writing door.* Rule: both files through the
   store's read-only door. Checked: a session over a fresh clone, its 23
   files' hashes the same after as before.
3. *Acts lost when the window closes before the next flush; a failed
   write silent.* Rule: each act is sent once its paint is timed; one
   not taken is kept, sent again and counted in the status bar; the
   server answers a failed write with its reason; the page's last acts
   go by beacon as it hides, and the start script waits a second. Checked:
   ten acts, ten logged before a reload; twelve in the log after it.
4. *Late frames judged over the wheel, the jumps and the pauses
   together.* Rule: the display's interval from frames at rest before
   the wheel, lateness counted over the steady wheel's frames alone.
5. *The indent refused by the content policy.* Rule: no inline style
   attribute; the indent is set through the style object, which the
   policy allows. Checked: heads at 4 and 20 px, a page's items at 36.
6. *An act's basis cut to its snapshot, and none for a scroll from held
   rows.* Rule: every answer carries the whole basis and every answered
   act records it, a scroll from held rows the basis they were answered
   at. Checked: a held scroll's record carries all fourteen keys.
7. *The page's state not in its URL.* Rule: what the page asks is written
   into its URL as it changes and read back at load. Checked: a reload
   gives the same caption, the same opened card and the same Name. Back
   is not built (a departure above).
8. *The C++ figure called a refilter.* The bar now names the measured
   step and its two parts.

The review was the brief's one (decision 6); this round is not reviewed
again.

## Judged runs

Each set is three headless runs of the four scenarios, taken in turn by
the start script's way (the rig installed under `runs/desktop-spike/rig`,
over a fresh `cp -cR` clone of the copy for each run), in headless Chrome
158 at 1280 × 633 CSS pixels inside a 1280 × 720 window, at a pixel ratio
of 1. Before and after each run no `cargo`, `rustc`, `cargo-nextest` or
gate process was running on the machine (each looked for by name, its
lane by its working directory); the load average, from the owner's other
applications, stood between 1.9 and 3.3. Each run's whole log is under
`headless/`, one line per scenario, its acts and, for the scroll, its frames;
`judge.py` prints the tables and the bar's lines from it.

### rig-2: the bar misses on one line

`desktop-rig --check Standard` held before the runs. Per run, the
answered acts' medians (ms) and the engine's, transport's and browser's
medians:

| Run | key | engine · transport · browser | click | expand | scroll |
| --- | --- | --- | --- | --- | --- |
| 1 | 234 | 230 · 1.3 · 2.3 | 38 | 34 | 3.4 |
| 2 | 233 | 230 · 1.1 · 2.3 | 46 | 34 | 3.5 |
| 3 | 248 | 245 · 1.3 · 2.3 | 40 | 34 | 2.9 |

The bar, line by line:

| Line | Run 1 | Run 2 | Run 3 |
| --- | --- | --- | --- |
| 1a. last keystroke of a burst: 9 in 10 within 350 ms, all within 700 | 11 of 12, worst 470 | 11 of 12, worst 466 | **10 of 12, worst 522: misses** |
| 1b. every answered keystroke within 700 ms | 41 of 41, worst 470 | 42 of 42, worst 466 | 41 of 41, worst 522 |
| 2. click: 9 in 10 within 100 ms, all within 200 | 9 of 10, worst 111 | 9 of 10, worst 110 | 9 of 10, worst 111 |
| 3. expansion: 9 in 10 within 100 ms, all within 200 | 20 of 20, worst 57 | 20 of 20, worst 55 | 20 of 20, worst 58 |
| 4a. scroll: 9 in 10 within 100 ms, all within 250 | 201 of 207, worst 102 | 204 of 209, worst 102 | 204 of 209, worst 102 |
| 4b. late frames in the steady wheel, at most 1% | 3 of 599 | 2 of 601 | 1 of 601 |

The bar does not hold: run 3 misses line 1a. **The cause, from the
split**: the last keystroke of `pseudo.total_life>=70` took 466 to 522
ms in every run, and the last of `tabula` 374 ms in run 3; in each, the
browser's share was about 2 ms and the transport's about 1 ms. The rest
was the engine's: 186 to 201 ms waiting for the answer to the keystroke
before it (`wait`), then 277 to 312 ms of its own, of which the places
document for the tabs took 130 to 150 ms (60 of them reading the store's
tab listing again), the one for the characters 84 to 95 ms (the query
evaluated a second time), and the window's rows 52 to 54 ms (a third
evaluation, over the window's places alone). A keystroke under a query
evaluated it over the corpus three times. The rig can answer that
(decision 8): rig-3 below.

### rig-3: the query evaluated once a keystroke

What changed, and why: a keystroke under a query asked the places
documents the query, so the query was evaluated twice there and the tab
listing read again, then a third time for the window. rig-3 holds the
places documents asked with no query (Rule hold) and counts each place's
matches from one counts answer (Rule count). It keeps the places
document's meaning: under each of the check's three queries all 3,462 of
Standard's places count and list as the places document does, and the
rows drawn are the rows counted. On a headless try (not judged), the last
keystroke of `pseudo.total_life>=70` spent 129 ms counting and 26 ms on
the window's rows, where rig-2 spent 214 to 245 ms on the two documents
and 52 to 54 on the window.

## The store copy

`shasum -a 256` of each file under `runs/desktop-spike/store-2026-11-24`
before the first run (the copy is never opened; every process reads a
clone):

```
ee43ab6b28e5ad997562dc457df69fb11004dd9fb2fd6df70b00f856b41a4a32  ./ggg/2f9076e7-7d68-4756-8b0f-aac4d910ac65.annotations.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/2f9076e7-7d68-4756-8b0f-aac4d910ac65.annotations.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/2f9076e7-7d68-4756-8b0f-aac4d910ac65.annotations.db-wal
07ac91c6322ac048f97578a8e9f270fe062c8e75c72887ef12110a27a60e7d9b  ./ggg/895be20e-0b9a-46c3-8b46-bd71cb289407.annotations.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/895be20e-0b9a-46c3-8b46-bd71cb289407.annotations.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/895be20e-0b9a-46c3-8b46-bd71cb289407.annotations.db-wal
e0b48396139104cf54f32fe7bdc244a4bf01dd240da483e0b1e575d67f44786d  ./ggg/GERWARIC_7694.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/GERWARIC_7694.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/GERWARIC_7694.db-wal
4f38bac655f5176c1f025a0a9e1b2dc95eb3a3bf233dc1956659d1363d849953  ./ggg/_exiled_6914.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/_exiled_6914.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/_exiled_6914.db-wal
e1521b3961fc15b2c0fb7dc4e180d3ed90b7aee5f282b21bff5efb069312147e  ./ggg/_vagabond_6960.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/_vagabond_6960.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/_vagabond_6960.db-wal
be11c8de74664ac74a0442e954c85e8224c0b4936be4aa9a400769c3a1ad05e5  ./ggg/accounts.json
9dcb1d511ae25ae1f24c8140f5e7c942c7c0869aac9bcac362d4235ae1996349  ./ggg/cac319d8-e65f-4afa-92be-1de85d620033.annotations.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/cac319d8-e65f-4afa-92be-1de85d620033.annotations.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/cac319d8-e65f-4afa-92be-1de85d620033.annotations.db-wal
aa9412eab21b48759927b3079e9a274a806ca34a86e8da80dec8dc066ac4870b  ./ggg/daemon.db
fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb  ./ggg/daemon.db-shm
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./ggg/daemon.db-wal
9d594bc15911b84433f1c675ff03a57a2ac3758975dbe2bc977a4b22e4b2465d  ./ggg/user.db
```
