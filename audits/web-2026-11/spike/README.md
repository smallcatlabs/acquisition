# The window spike: a hand-written script over the dense table

Step 6.3 of the GUI's first slice, to
`roadmap/briefs/4-full-1-step6-spike.md`: whether a hand-written
window script holds the table in the owner's Brave, flat over every pc
item of the measurement copy and grouped under pc Standard's tabs. It
is answered by a rig, built, watched and changed (the owner on #264:
"build, observe, and iterate instead of front loading the design
work").

## Verdict

Yes: the hand-written script holds the table in the owner's Brave,
flat and grouped. At his sitting on 2026-11-12, every case passed
every part of the bar on its run, with the same numbers headless
Chrome had given at rig-2 and rig-3. In his words: "All passed.
Nothing flickered." The script is 93 lines, with no crate and no
framework. Holding the listing for its basis was the one change the
grouped cases needed (rig-2). The script itself never changed.

What he asked of the next steps is how the table reads:
- tabs nested under their folder or parent tab and collapsed, for the
  grouped table (step 6.5, #269);
- opening something keeping his place (an item's card in step 6.4, a
  place in step 6.5, #270);
- a look nearer the trade site's, the production renderer's (step 6.4).

The route that serves a window of an answer's rows is drafted for him
as one decision, named `rows` there (#268).

## The rig

The prototype and a page that measures it, never committed as code:
uncommitted edits in a detached copy of `main` at `f3e663db`, outside
the repository. `rig-1.patch`, beside this file, is the copy's whole
diff from that commit, new files and assets included: three routes and
a stylesheet block in `acq web`, the window script, the measure, and
the headless runner. A blank context line in it has lost its space, so
that the patch passes the gate's whitespace check; `git apply` reads
it so. It applies to its base, and the tree it makes is the copy, from
the repository's root:

```sh
patch="$PWD/audits/web-2026-11/spike/rig-1.patch"
git worktree add --detach <dir> f3e663db
git -C <dir> apply --check "$patch"            # exit 0
git -C <dir> apply "$patch"
diff -r -q -x target -x .git <dir> <the copy>  # no output
```

Everything below was read off the working page.

**The viewport.** Headless: Chrome 157, 1440 × 813 CSS pixels at a
pixel ratio of 2 (a 1440 × 900 window). In Brave, whatever the owner's
window is; the page records it with each run.

**A row.** 24 CSS pixels high, from one source: the server writes the
height on the table and the script sets the stylesheet's row height
from it. An item row is its name (in its rarity's colour, a link to
its card), its base, its price (the price's text, from the row's
item), its location, then the query's values, each cut to one line.
A head row is the tab's name (a link that opens it, or closes it when
open), its type, its items and how long ago it was fetched.

**The cases**, each a URL under the server's token, in the mock world
over the wrapped copy:

| Case | Page | What it asks | Rows |
| --- | --- | --- | --- |
| F | `table?realm=pc&text=` | every pc item, the store's order | 22,623 |
| F1k | `table?realm=pc&text=category="accessory.ring" ilvl>=1` | the rings, by item level, largest first: the one value the query compares is the order and a column | 1,133 |
| G0 | `table?realm=pc&text=&group=tabs&league=Standard` | pc Standard's tabs as head rows, in the order `acq tabs --realm pc --league Standard --expand` lists them | 3,347 |
| G1 | G0 and `&open=d8132eff92` | G0 with ULTIMATUMS (Remove-only) open: its rows under its head, asked as `id:d8132eff92 league="Standard"` | 348 heads, 499 rows, 2,999 heads |

F1k is the category nearest 1,000 in `acq search --realm pc --count
category --limit 100` (rings, 1,133; the next, helmets, 745), with
`ilvl>=1` so the table carries a value column and the query's own
order. G1's tab is the largest in `acq search --realm pc --count tab
--routes` (499 items), opened by the route that count gives it; it is
the listing's row 347.

**How a scroll becomes a window.** The server renders a table as
segments, each a run of rows with its route and its count: F and F1k
one segment of rows, G0 one of heads, G1 the heads to the open tab,
its rows, and the heads after it, the last carrying the offset of its
first row in the listing. The script:

1. sizes each segment to its count × 24 px, so the page's height is
   whole before a row loads and the page scrolls as one document;
2. on a scroll, once per frame, reads each segment's top and works out
   the blocks of 50 rows in view, and those within one viewport above
   and below;
3. asks the blocks in view, `<route>&offset=<first row>&count=<50 or
   fewer>`, and the blocks around them once no block in view is still
   on its way;
4. places each block at its first row × 24 px in its segment;
5. drops the blocks that have left that range, aborting their asks.

It keeps the blocks it shows and the asks in flight: no order, no
counter, nothing a reload must keep. Rust applies the bounds: the rows
route continues the page's request at its basis, starting after the
offset and limited to the count, and an answer that carries its items
holds at most 200 rows whatever the count. A window across a moved
basis is refused by the search's own refusal, the route answers 409,
and the script loads the page again; the heads route checks its
token's basis against the held corpus's before it reads the listing,
and caps its count at the same 200. The window script is 93 lines
(`wc -l` of `assets/window.js` in the copy).

**A run.** The measure, loaded only with `measure` in the URL, drives
the page's scroll in each animation frame and looks at what that frame
shows:

1. 90 still frames: the display's frame interval is their median.
2. A move to the steady scroll's start (the top; in G1, one and a half
   viewports above the open tab's rows), held until the viewport is
   whole and half a second more.
3. The steady scroll: 1,500 pixels a second (62 rows) for 10 seconds,
   a fast wheel's speed. In G1 it crosses both edges of the open tab's
   rows.
4. Ten jumps, each held until its viewport is whole (or 3 seconds),
   then half a second. F, F1k and G0: to these fractions of the page,
   0.5, 0.1, 0.9, 0.25, 1, 0.75, 0.37, 0.61, 0.83, 0. G1: the open
   tab's first rows, its middle, its last rows, its top edge and its
   bottom edge each at mid-viewport, the page's middle, end and start,
   and its rows' quarter and three quarters.
5. Every row seen is checked against the rows route's JSON at the same
   basis, asked in chunks of 128, never the script's 50.

What a frame shows: each segment's slots in the viewport. A slot is
whole when the row of its index is there, at its index × 24 px. Every
row drawn in the viewport, in any segment, is judged by where it is
drawn (its rendered box), whatever its layout or its declared index
say: one more than half a pixel from its index's place is off. A slot
seen holding two items during the run has changed. Each window the
page asks from the measure's start is recorded as it is asked (the
measure wraps the page's `fetch`, which the window script calls by
name, and passes each call through unchanged): its route, offset and
count, when it was asked, its state at the run's end (pending, done,
aborted or failed), its time to its body's end, and each step of the
server's part (the store's opening, the held corpus's check, the
answer or the listing, the rendering). Each jump keeps when it began,
so a miss shows which windows were its and whose time they took. The
measure observes the page as drawn and the requests as asked, never
what the page declares or what a browser summary keeps.

The headless runs: `rig-run.py`, three runs of the four cases in turn,
in Chrome headless on a profile directory under the builder's scratch,
against `acq web` built `--release` from the patch, nothing else
running on the machine. Each call to the browser has an absolute
deadline, 30 seconds or the run's end (180 seconds from its
navigation), whichever is first, and each read waits only what is left
of it; so does finding the browser's page (its port file and its page
list): a browser that stops answering, or answers a byte at a time, is
a run recorded as failed, never a wait without end. `rig-probes.py`,
beside the runner in the patch, makes each fault the reviews found and
exits 1 unless the measure and the runner judge it as they must: a
call and the page list answered a byte at a time end at their
deadline; rows drawn off their place, by a declared index or by a
transform, void the run; a window unanswered at the run's end is in
the result, pending. The owner's: the same pages in Brave, each
case's link with `measure=1` running itself, and without it for his
own wheel.

## The bar

A run passes when it meets every line:

1. **Frames that come late.** In the steady scroll, at most 1% of
   frames late, a late frame being one whose interval is more than one
   and a half times the display's (a frame dropped).
2. **Frames during the jumps.** None longer than 100 ms.
3. **From a jump to its rows.** The time from the frame that set the
   scroll to the first frame whose viewport is whole: 9 of the 10
   jumps within 100 ms, and every one within 250 ms.
4. **Frames with a row missing.** At most 1% of the steady scroll's
   frames.
5. **A row off its place** voids the run, as does a slot that changed
   or a row that is not the answer's row at its index.
6. **The live DOM.** At most 5,000 elements
   (`document.getElementsByTagName('*').length`, sampled every 30th
   frame and at each jump's first whole frame) and at most 250 of the
   table's rows, the same bound at 1,133 rows as at 22,623.

A case holds when its three headless runs pass, and the script holds
when every case does, headless and in Brave. A miss is a turn of the
loop: its cause found, the rig changed, its new patch committed beside
this one with what changed and why, and the runs judged again.

## What building it showed

Unjudged trials while it was built, on a machine another session's
C++ build was loading, so their times are not measurements:

- The rows were right from the first trial: in every case, every row
  seen was the answer's at its index, and the live DOM held at most 200
  rows and about 1,000 elements.
- The steady scroll in G1 began with a jump to its start, so its first
  frames were missing rows that belonged to the jump. The measure now
  holds the start until it is whole.
- A jump asked up to four blocks at once, and they waited on each other
  at the server. The script now asks the blocks in view first.
- Asked one at a time, a block of rows took the server about 15 ms (the
  held corpus's check about 9, the answer 4 to 6), and a block of heads
  about 51 ms: the heads route reads the whole listing of 3,347 tabs
  for each window (41 ms), since the store's read and the search's
  interface stay as they are in this step. G0's jumps were the slowest
  of the four cases for it.

## The headless runs: rig-1

Judged by the bar above, as named before them: **F and F1k hold, G0
and G1 miss.** Three runs of each case, 2026-11-11, in turn, with
`rig-1.patch` as committed at `d7d109c3` (its runner then found the
browser's page with no deadline; the page, the script and the measure
were the ones in the patch now). The results, whole, one line a run:
`headless-1.jsonl`.

The machine, by `uptime` and `ps` before and after: load 2.33 and
1.98 on 10 cores, no compiler running; Spotlight's indexer
(`spotlightknowledged`) used one core throughout, and a cold review
ran alongside. Chrome 157 headless, 1440 × 813 CSS pixels at a pixel
ratio of 2, 60 Hz. The server: `acq web` built `--release` from the
patch, the copy's diff equal to the patch (`cmp`). The measurement
copy's SHA-256 was the brief's before the backup and after the runs.

| Case | Runs passed | Jumps, median / 9th of 10 / slowest | Late frames, steady | Steady frames with a row missing | Most elements / rows live | Rows checked a run | Void |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F, 22,623 rows | 3 of 3 | 67 / 83 / 83 ms | 0 of 1,803 | 0 | 780 / 150 | 965 | 0 |
| F1k, 1,133 rows | 3 of 3 | 67 / 67 / 83 ms | 0 of 1,803 | 0 | 931 / 150 | 824 | 0 |
| G0, 3,347 heads | 0 of 3 | 116 / 117 / 117 ms | 0 of 1,803 | 0 | 1,030 / 200 | 930 | 0 |
| G1, 348 + 499 + 2,999 | 1 of 3 | 100 / 100 / 100.1 ms | 0 of 1,803 | 0 | 1,033 / 200 | 755 | 0 |

Every miss is on one line, 9 of 10 jumps whole within 100 ms. G0 missed
it in every run, with jumps of 83 to 117 ms. G1 missed it in two runs
of three by 0.1 ms: those jumps were whole at the sixth frame after
the jump, 100.1 ms at 60 Hz, against a line drawn at 100 ms, so a jump
whole at its sixth frame passes or fails on the frame clock's jitter.
It is a miss as the bar names it, and the bar stands as written. No
frame in any jump was longer than 16.8 ms, and every jump was whole
within 117 ms, under the 250 ms line.

Why, from the windows each missed jump asked (`windows.each` against
the jump's `at`): every one waited on a window of tab heads. A heads
window took 74 ms from its ask to its body's end (median), 44 of them
the listing read: the heads route reads the whole listing of 3,347
tabs for each window, since the store's read stays as it is in this
step. A window of rows took 4 to 9 ms to answer. G0's misses each
waited on two heads windows; G1's on one, beside a rows window where
the jump met the open tab's edge. Each window's server time also
carried the held corpus's check, 22 ms median under a jump's
concurrency (about 9 alone): up to four windows queue on the held
corpus's lock, each checking currency in turn.

## Rig-2: the listing held for its basis

One turn of the loop, for the miss above. `rig-2.patch`, beside the
first, is the copy's whole diff from `f3e663db` again; it applies and
makes the copy, by the commands above with its name. What changed, and
only that: the web module holds the league's tab listing for the basis
it was read at, as the server already holds the corpus. The page reads
it at its answer's basis; a window of heads checks its token's basis
against the held corpus's, as before, and is then a slice of the
listing held at that basis, read again only when the basis has moved.
The lock is held through a read, so the windows one jump asks read it
once. The search and store crates are unchanged, and so are the page,
the window script (93 lines), the measure and the bar.

What it bought, asked one at a time from the server: the listing's
part of a heads window went from 41 ms (the whole listing of 3,347 tabs
read for each window) to 0.01 ms; what is left of the window's server
time is the held corpus's check, about 8 ms.

**For step 6.5.** The places listing as the tree reads it today,
`Tabs::read` over every tab of a league, costs 41 ms on this machine
for pc Standard's 3,347 tabs, and 44 ms under a jump's concurrency. A
places document asked a window at a time cannot read it per window
and hold the jump line; held for its basis, a window of it is a slice.

## The headless runs: rig-2

Judged by the same bar: **all four cases hold, every run.** Three runs
of each case, 2026-11-11, in turn, with `rig-2.patch` as committed at
`93e3f59e`; the results whole in `headless-2.jsonl`. A first attempt
failed before any run, on a stale port file, and is not a run (that
commit says so). The machine, by `uptime` and `ps` before and after:
load 2.50 and 3.06 on 10 cores, no compiler running, Spotlight's
indexer on one core. The same browser, viewport and build profile as
rig-1's runs; the copy's diff equal to the patch (`cmp`). The
measurement copy's SHA-256 is the brief's after these runs too.

| Case | Runs passed | Jumps, median / 9th of 10 / slowest | Late frames, steady | Steady frames with a row missing | Most elements / rows live | Rows checked a run | Void |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F, 22,623 rows | 3 of 3 | 67 / 83 / 83 ms | 0 of 1,803 | 0 | 780 / 150 | 965 | 0 |
| F1k, 1,133 rows | 3 of 3 | 67 / 67 / 83 ms | 0 of 1,803 | 0 | 931 / 150 | 824 | 0 |
| G0, 3,347 heads | 3 of 3 | 67 / 67 / 67 ms | 0 of 1,803 | 0 | 1,030 / 200 | 930 | 0 |
| G1, 348 + 499 + 2,999 | 3 of 3 | 67 / 67 / 67 ms | 0 of 1,803 | 0 | 1,033 / 200 | 755 | 0 |

No frame in any jump was longer than 16.8 ms. Every window asked was
answered (none pending, aborted or failed at a run's end). A heads
window now took 25 to 27 ms from its ask to its body's end (median),
against 74 at rig-1, its listing part 0.01 ms; most of a window's
server time in every case is now the held corpus's check (22 ms median
under a jump's concurrency, about 8 alone), which each of a jump's
windows makes in turn behind the corpus's lock. That is the next cost
a window carries, in the search crate's holder, and none of this
step's to change.

## Rig-3: each run's result kept by the server

So that the owner's sitting costs him nothing but opening the links.
`rig-3.patch`, beside the others, is the copy's whole diff from
`f3e663db` again, and applies and makes the copy as they do. What
changed, and only that: when a run ends, the measure sends its result
to the server, a POST on `result` under the page's token like every
other route, with the case the link names (`case=`) and the window as
the page measures it (inner and outer size, screen, pixel ratio). The
server appends one line to the file `ACQ_RIG_RESULTS` names: the time,
the case, the browser's user agent as its request says it, the window,
and the result whole. A server started without that variable keeps
nothing and says so. The printed result stays, headed "kept by the
server" or why not. The runner's links name their case. The window
script (93 lines), the page, the bar and what the measure judges are
unchanged.

One headless run of each case, 2026-11-12, to show the judged numbers
are rig-2's, not a judgment of their own (`headless-3.jsonl`): all
four pass, jumps 33 to 83 ms, no late frame, no row missing, off or
wrong, the live DOM at most 1,033 elements and 200 rows, as at rig-2.
Each run's result was kept by the server: four lines, each with its
case, the headless browser's user agent and its window (1440 × 813
inside, 1440 × 900 outside, pixel ratio 2). Load 2.54 before and 2.50
after on 10 cores, no compiler; Spotlight's indexer on one core.

## The owner's sitting

Four runs, one per case, 06:40 to 06:42 on 2026-11-12, in Brave by
the owner's word to the builder: their user agent says Chrome 157,
which Brave and Chrome both say, so the file cannot tell them apart.
They ran against `rig-3.patch`'s server as committed (the copy's
diff still equals it, by `cmp`). Each result was kept by the server,
as rig-3 made it. The lines are whole in `sitting.jsonl` beside this
file. The window, as
the page measured it: 1049 × 824 inside, on a 1440 × 900 screen at a
pixel ratio of 2. **The display's rate, measured by the run's still
frames: 60 Hz** (16.7 ms a frame). No build of this step's ran during
the sitting. At 06:36, four minutes before it, load was 1.88 on 10
cores with no compiler running.

| Case | Passed | Jumps, median / 9th of 10 / slowest | Late frames, steady | Steady frames with a row missing | Most elements / rows live | Rows checked | Void |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F, 22,623 rows | yes | 75 / 83 / 84 ms | 0 of 600 | 0 | 780 / 150 | 968 | 0 |
| F1k, 1,133 rows | yes | 67 / 83 / 83 ms | 0 of 600 | 0 | 931 / 150 | 828 | 0 |
| G0, 3,347 heads | yes | 75 / 83 / 83 ms | 0 of 600 | 0 | 1,030 / 200 | 931 | 0 |
| G1, 348 + 499 + 2,999 | yes | 83 / 83 / 83 ms | 0 of 600 | 0 | 1,033 / 200 | 755 | 0 |

No frame in any jump was longer than 17.8 ms. Every window asked was
answered (41, 39, 43 and 47 a run). The median time from ask to body
was 34 to 41 ms, against about 26 headless. The measure's own look
cost under 1 ms a frame.

The file has a fifth line: a second run of F from the Claude desktop
app's own browser pane, not from Brave. Its user agent names the app
and Chrome 156; its viewport is 568 × 724, on a 1470 × 956 screen. Its
result came 14 s after the Brave run of F, and a run takes at least
20 s, so the two ran side by side. It is recorded and not judged: its
frames came about a second apart (the longest 1,017 ms, its jumps
whole in about 2,017 ms), whole seconds, which is what a browser does
to a page that is not in front, so it measured the pane's pacing and
not the script. The Brave run of F passed every line of the bar with
that run going on beside it.

What the owner said after the runs, exact as he gave it to the
builder:

> Done. All passed. Nothing flickered. The styling and format of the
> pages is still spartan compared to the trade site. Aso, map
> sub-stashes were not displayed hierarchically, they were listed with
> the same visual style as top level stashes--this causes visual
> clutter due to how many sub-stashes every map stash contains. There
> are pages and pages and pages of map substashes that I have to
> scroll past. Map stashes should be collapsible. Also, when I clicked
> on a map substash to expand the maps the page jumps back to the top
> so I have to scroll back down to find the tab I expended.

What each point is, and whose step:

- The look. The rig renders the least a row needs. The row's look is
  the production renderer's, step 6.4.
- The sub-tabs. In pc Standard, 3,086 of the 3,347 heads have a
  parent: 2,535 map sub-tabs, 474 unique sub-tabs and 77 tabs in
  folders. Only 261 are top level. The rig lists the heads flat in the
  listing's order. The store holds each tab's parent, so a tree is a
  rendering of facts already held. The grouped table is step 6.5's
  (#269).
- The jump to the top. A head's link is a plain navigation to the same
  table with `open=` and no place to land, so the new page loads at
  its top. Opening something leaves the reader where they were: an
  item's card in step 6.4, a place in step 6.5 (#270). The measure
  never clicks a head: G1 opens its tab by its URL and scrolls there
  itself, so the bar could not see this.

## The window script

`wc -l crates/acquisition-cli/src/web/assets/window.js`: **93 lines**,
unchanged since rig-1, in `rig-3.patch`. What it holds:
- It finds the table and its row height, and sizes each segment the
  server named by its count times that height.
- `fill` runs once a frame after a scroll or a resize. For each
  segment it works out:
  - the blocks of 50 rows in view;
  - the blocks within one viewport around them.

  It drops the blocks outside that span, aborting any still asked. It
  asks the blocks in view first, and the ones around them once none in
  view is still on its way.
- `ask` makes a block's element at its offset and fetches the
  segment's route with the offset and the count. It puts in the
  server's rows as they come. On a 409 (a moved basis) it loads the
  page again.

It keeps nothing a reload must keep: no order, no count, no row it
made itself.

The rig's `window` route, which answers a window of an answer's rows,
is drafted for the owner as one decision, under the name `rows` (#268).
The rig's second route, `heads`, served the grouped table's head rows;
those belong to the place pages the owner ruled (`tabs`,
`characters`), and how they serve a window is step 6.5's brief to say,
with this spike's finding that the tab listing is held for its basis
(rig-2).

The copy outside the repository is removed and pruned, and its server
stopped. The measurement copy's SHA-256 is the brief's after the
sitting too.
