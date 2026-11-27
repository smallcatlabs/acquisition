# The desktop spike: a window over the engine, laid out like the C++ app

The record of `roadmap/briefs/desktop-spike.md` at `9f5594c1`. One
question: does a window over the engine the tool has feel like the C++
app, or does it need a native toolkit? It is answered by a throwaway
window the owner sits at over a copy of his own stash, built, watched
and changed. This file names the rig and the bar before any judged run.
The runs, the sitting and the verdict follow it in later commits.

## The rig

A package of its own, `spikes/desktop-rig`, in a lane (a `cp -cR` clone
of the main tree at `9f5594c1`), never committed as code. `rig-1.patch`,
beside this file, is the lane's whole diff from `9f5594c1`: the package,
and the two lines that add it to the workspace (`Cargo.toml`'s members,
and `Cargo.lock`'s entry for the package itself, the one change the lock
takes: no crate is added, and the rig builds `--offline`). It applies to
its base, from the repository's root:

```sh
git worktree add --detach <dir> 9f5594c1
git -C <dir> apply --check "$PWD/audits/desktop-2026-11/spike/rig-1.patch"   # exit 0
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
  the order, and the window of rows wanted.
- **Every list is an answer.** The tree's places are the search crate's
  places document (`Places::tabs`, `Places::characters`) for the league,
  asked the query; a place's rows are its own request's answer; the flat
  list is one answer of the query over the league, the league's term as
  the answer's count of `league` writes it; an item's card is the answer
  to its id with the item asked. The page sorts, filters, groups and
  composes nothing (C46, semantics in Rust; C16, the page is
  presentation). The rules the rig had to write in Rust are listed below.
- **Reads only.** The start script runs the rig over a fresh clone of
  the store copy under the read environment of
  `audits/whole-review-2026-11/INVESTIGATOR.md`; no daemon starts and
  nothing is sent to GGG. The facts open read-only (`Account::facts_read`)
  and nothing is written through the store; the file the rig writes is
  the sitting's log.
- **A check of its rows**: `desktop-rig --check <league>`, under the read
  environment, opens every
  place of the league under no query and three, and fails unless the
  rows drawn are the rows counted, each at its index, and each place's
  rows are its own request's answer, in its order (200 places a case).
  On the lane's build, Standard: every case holds.
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
rig. Every act goes to `runs/desktop-spike/logs/sitting-<time>.jsonl`.

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
- **Hold**: the places documents and each place's rows, once answered,
  held for one league, query and order at the corpus's basis, and reused
  only while the holder finds that basis current before the act. A place's
  own terms (its request asked with no query) are held per basis and
  league.
- **Batch**: the window's places not answered yet are one request, their
  own terms joined by `or` before the query, and its rows parted by the
  place each lies in. The check above holds the parted rows to each
  place's own answer.
- **Card**: the item's card from the answer to its id with the item
  asked.

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
  place. An answer carries its basis, which the act records.
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
    was made. The next request leaves once the earlier answer is drawn
    into the DOM, so `wait` also carries that answer's transport and DOM
    work, a few milliseconds, counted as the engine's;
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
after a change to the form, then refilters, which its own notes measured
at a median of 422.5 ms at a 100k-item synthetic preset before its sort
was made faster (`docs/design/m3-sort-profile-result.md`, at
`acquisition-qt@da7b3cd4`; no figure at 44k items); it fills the item
panel 100 ms after a selection. It records no figure for opening a tab
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
   and every one within 250 ms; (b) during the steady wheel, at most 1%
   of frames late, a late frame being longer than one and a half times
   the display's interval.

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
- *scroll*: *By Item*, a wheel of 25 px every 1/60 s for 10 s (1,500 px
  a second), then ten jumps of the scroll position to fractions of the
  list (as dragging the thumb), each held until the page is idle.

## What building showed

Exploratory runs while the rig was built, with other sessions' builds
on the machine (load averages 3 to 12), so none is judged. In every
one, the browser's share of an answered act was a few milliseconds
(median 1 to 8 ms; 17 ms for a scroll, its frame), and the transport's
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
