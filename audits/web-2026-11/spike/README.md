# The window spike: a hand-written script over the dense table

Step 6.3 of the GUI's first slice, to
`roadmap/briefs/4-full-1-step6-spike.md`: whether a hand-written
window script holds the table in the owner's Brave, flat over every pc
item of the measurement copy and grouped under pc Standard's tabs. It
is answered by a rig, built, watched and changed (the owner on #264:
"build, observe, and iterate instead of front loading the design
work").

## Verdict

None yet. This commit is the rig and the bar its runs will be judged
by, written before any judged run. The headless runs are the next
commit, and the owner's sitting in Brave decides.

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
row in the viewport is judged by where it is, whatever index it
declares: one more than half a pixel from its index's place is off. A
slot seen holding two items during the run has changed. Each window
the page asked is kept in the result: its route, offset and count,
when it was asked, its time to the response's end and each step of the
server's part (the store's opening, the held corpus's check, the
answer or the listing, the rendering); each jump keeps when it began,
so a miss shows which windows were its and whose time they took.

The headless runs: `rig-run.py`, three runs of the four cases in turn,
in Chrome headless on a profile directory under the builder's scratch,
against `acq web` built `--release` from the patch, nothing else
running on the machine. Each call to the browser waits 30 seconds at
most and each run 180: a browser that stops answering is a run
recorded as failed, never a wait without end. The owner's: the same pages in Brave, each
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
