# 4-full-1, step 6.3 — The spike: the window script

## Goal
One question is answered by measurement before step 6.4 is briefed:
whether a hand-written window script holds the table in the owner's
Brave, flat over every pc item of the measurement copy and grouped
under pc Standard's tabs. `main` gains a record and nothing else. The
check's driver is the next brief, `4-full-1-step6-driver.md`, which
reads this one's prototype.

## Inputs
- `4-full-1-step6-design.md` §3 (a window is asked within one place),
  §4 (one row height, the script by hand first, one day in Brave),
  §5 item 3.
- `4-full-1-design.md` §2: what a page may not hold.
- C46 (semantics in Rust), C98 (one basis per answer), C16 (the URL
  names the state), C138 (the web's listener, token and guard).
- `answer.rs`, `Rows.after`: a row offset in the request's own order,
  refused across a changed basis. `acquisition_search::tabs`,
  `Tabs::read` and `grouped`, as `web::tabs` calls them: the listing
  G0 and G1 read. `assets/count.js`: a fetching script's shape (a
  timer and an `AbortController`, no counter).
- The measurement copy,
  `search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`, SHA-256
  `fbae8786d79c67e3d9f277bae3e8ce60382f55eeabc2dc5bdb5f89965bccfda7`,
  read under the owner's grant of 2026-10-19; wrapped as a store by
  `crates/acquisition-search/examples/copy-as-store.rs`
  (`search/MEASUREMENTS.md`, M3's method). In a mock world over it:
  `acq search --realm pc --limit 0` says 22,623 matches, and `acq tabs
  --realm pc --league Standard` says 3,347 tabs.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`; green at `9163283a`.

## Decisions
1. **Where it lives: a copy outside the repository, and no branch.**
   The prototype is edits in a detached copy of `main` under scratch
   (`git worktree add --detach`), never committed, as the cards
   spike's scratch program was. So no route joins `acq web` by this
   step: the two the prototype serves exist only in the copy, and the
   route the table will need is drafted in the record for the owner.
   `main` gains one commit: the record,
   `audits/web-2026-11/spike/README.md`, with the window script and
   the measuring script as files beside it. The copy's Rust diff goes
   to the builder's notes folder, named in the record.
2. **A block.** The server writes each windowed run of rows as one
   element: its height the row count times the row height, and on it
   the count, the height and a window URL whole but for a row number.
   The page scrolls as one document; there is no inner scroller.
3. **The script.** On a scroll, for each block the viewport meets: the
   offset is how far the viewport's top is past the block's top (zero
   if it is not), divided by the row height. It appends that number
   to the block's URL, asks for the rows the viewport shows and a page
   each side, and places the fragment at offset times height inside
   the block. A block the viewport does not meet asks nothing. It
   holds a timer, a controller and a few fragments a block: no
   counter, no order, no arithmetic but this.
4. **A window's answer** is rows rendered by Rust (C46). In the copy
   a `rows` route answers it, carrying the request and its basis as
   the request's token does and the offset as `after`; a moved basis
   is refused and the script reloads the page.
5. **A row** is one fixed height: name, base, price, location and the
   query's values, overflow clipped. Enough to measure, not 6.4's
   renderer.
6. **The cases.** F: the empty query in pc, flat, one block. F1k: a
   query matching about 1,000, flat. G0: pc Standard's tabs as head
   rows in plain DOM, stash order, no block. G1: G0 with its largest
   tab open by a URL field, that tab's rows one block under its head.
7. **The page measures itself.** A measuring script, the copy's
   alone, runs a steady scroll top to bottom in 20 s, then 20 jumps to
   fixed fractions. It records every interval between animation
   frames and, per jump, the time to the rows shown (in G0, which has
   none, to the next frame), the element count at rest and at peak,
   fetches and bytes, the heap where the browser gives it. It posts
   the samples whole to a route of the copy that writes a file under
   scratch. Each case three times, the window in front, the display's
   rate recorded.
8. **The bar, named before the first run.** A run passes when under
   1 interval in 100 is over 1.5 frames of the display, none is over
   50 ms after the first second, and the 95th percentile of its
   jumps is within 100 ms. A case passes when all three runs do. The
   script holds if every case passes in Brave on the owner's Mac and
   F's peak element count is within a tenth of F1k's. G0's and G1's
   counts are recorded, not judged: whether 3,347 heads may stay
   plain DOM is read from their frames. On a miss, find the cause
   first (the same run with the fetch answering no rows tells layout
   from insertion), make one change by it and measure again. Only if
   it still misses, try one vendored, pinned virtualiser file under
   the same measure and record both.
9. **Headless first, then the owner.** Each case's URL takes a field
   that starts the run on load. Launch Chrome headless at it, on a
   profile directory under scratch, and end it when the samples
   arrive: a first read, labelled headless, never the verdict. Then
   start `acq web` in the background, report its URL with the four
   cases' links, and end the turn. The owner opens them in Brave,
   runs each and scrolls by hand; his words go in the record beside
   the numbers.

## Find first
- `git grep -n "after" -- crates/acquisition-search/src/answer.rs`:
  how a continuation is read and refused across a basis.
- `git grep -n "pub fn token\|pub fn from_token" -- crates/acquisition-search/src/answer.rs`:
  the request's token. `git grep -n "fn mint" -- crates/acquisition-cli/src/web`:
  the listener's, a different thing.
- `git grep -n "Tabs::read" -- crates/acquisition-cli/src/web/mod.rs`:
  the listing G0 reads.
- `sed -n 1,40p crates/acquisition-search/examples/copy-as-store.rs`:
  how the copy becomes a store.
- `shasum -a 256` of the copy, before the backup and after the last
  run.

## Done when
- The record is on `main`, its verdict first: per case, for Brave and
  for the headless run, each run's numbers under Decision 7 and the
  bar's result; the machine, the display's rate, the date, the
  commands; the owner's words.
- It gives the window script's size in lines and what it holds, the
  file beside it, and the window's route as measured, drafted for the
  owner as one decision. The launching session files it.
- The copy's hash is the same before and after; the worktree is
  removed, or named in the report for the driver's brief.
- `tools/gate.sh --docs` exits 0 on `main`'s commit.

## Not in scope
- The places document, `matched`, the place pages: step 6.5. G0 and
  G1 read the tabs listing the tree has.
- The table's renderer, its heads and sort links: step 6.4.
- The check's driver: the next brief.
- A framework, Node, a bundler; a change to a workflow or the gate.
- Every place open at once; the flat table answers that.
- Firefox and Safari, unless the owner opens them: then recorded.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world over the wrapped copy
under scratch, with `ACQ_NO_SPAWN=1`. A browser the builder launches
is headless, on a profile directory under scratch, never the owner's
profile, and opens no window.

## Report
`TEMPLATE.md`.
