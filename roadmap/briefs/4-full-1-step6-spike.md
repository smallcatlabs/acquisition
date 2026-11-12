# 4-full-1, step 6.3 — The spike: the window script

## Goal
One question is answered by measurement before step 6.4 is briefed:
whether a hand-written window script holds the table in the owner's
Brave, flat over every pc item of the measurement copy and grouped
under pc Standard's tabs. `main` gains a record and nothing else. The
check's driver is the next brief, `4-full-1-step6-driver.md`.

## Inputs
- `4-full-1-step6-design.md` §3 (a window is asked within one place),
  §4 (one row height, the script by hand first), §5 item 3.
- `4-full-1-design.md` §2: what a page may not hold.
- C46 (semantics in Rust), C98 (one basis per answer), C16 (the URL
  names the state), C171 (a commit lands before the owner looks).
- `answer.rs`: `Rows.after`, a row offset in the request's own order,
  refused across a changed basis; `RowField::Item`, the one path by
  which a row carries a price, under which an answer holds at most
  `ITEM_ROWS` rows. `acquisition_search::tabs`, `Tabs::read` and
  `grouped`, as `web::tabs` calls them. `assets/count.js`: a fetching
  script's shape (a timer and an `AbortController`, no counter).
- The measurement copy,
  `search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`, SHA-256
  `fbae8786d79c67e3d9f277bae3e8ce60382f55eeabc2dc5bdb5f89965bccfda7`,
  read under the owner's grant of 2026-10-19; wrapped as a store by
  `crates/acquisition-search/examples/copy-as-store.rs`. In a mock
  world over it: `acq search --realm pc --limit 0` says 22,623
  matches, and `acq tabs --realm pc --league Standard` 3,347 tabs.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`; green at `9163283a`.

## Decisions
1. **A copy outside the repository, no branch.** The prototype is
   uncommitted edits in a detached copy of `main` under scratch (`git
   worktree add --detach`), so no route joins `acq web` by this step;
   the route the table will need is drafted for the owner.
2. **Two commits to `main`, the record's**
   (`audits/web-2026-11/spike/README.md`). The first, before the
   owner is asked: the headless numbers, the two scripts as files,
   the copy's Rust as a patch with its base commit, the copy's path.
   The patch passes the gate's `diff` stage (a blank context line may
   lose its space; `git apply` reads it) and is proven to apply to
   its base. The second, after his sitting: Brave's numbers, his
   words, the verdict.
3. **A block.** The server writes each windowed run of rows as one
   element: its height the row count times the row height, and on it
   the count, the height, the most rows one window may ask, and a
   window URL whole but for a row number. The page scrolls as one
   document. Each row carries its index, written by the server.
4. **The script.** With `h` the row height and `v` the rows a
   viewport shows: for each block the viewport meets, `first` is how
   far the viewport's top is past the block's top (zero if it is
   not), divided by `h`, rounded down; the window starts at `first -
   v`, never under zero, and asks `3v` rows, never past the count or
   the block's most. The fragment is placed at its start times `h`.
   A block the viewport does not meet asks nothing. The script holds
   a timer, a controller and a few fragments a block: no counter, no
   order.
5. **A window's answer** is rows rendered by Rust from one answer of
   the request at `after`, its fields `item` as the cards ask, so a
   window is at most `ITEM_ROWS`. In the copy a `rows` route answers
   it, carrying the request and its basis as the request's token
   does, and answers the same rows as JSON where JSON is asked, as
   `search` does; a moved basis is refused and the script reloads the
   page. If
   the server's time for a window is what misses the bar, the record
   says so with the time: a lighter path to a price is 6.4's design.
6. **A row** is one fixed height: name, base, price, location and the
   query's values, overflow clipped.
7. **The cases.** F: the empty query in pc, flat, one block. F1k: a
   query matching about 1,000, flat. G0: pc Standard's tabs as head
   rows in plain DOM, stash order, no block. G1: G0 with its largest
   tab open by a URL field, that tab's rows one block under its head.
8. **The page measures itself.** A measuring script, the copy's
   alone, started by a URL field, runs a steady scroll top to bottom
   in 20 s, then 20 jumps to fixed fractions. A jump's time is until
   every row index the viewport covers is in the DOM at its index
   times `h` (in G0, to the next frame). It records every interval
   between frames, every jump's time, the frames of the steady scroll
   with a covered index missing, the element count at rest and at
   peak, fetches and bytes. It posts the samples whole to a route of
   the copy that writes a file under scratch. Each case three times,
   the window in front, the display's rate recorded.
9. **The bar, named before the first run.** A run is invalid if a
   row is shown off its place, or a jump's rows are not there within
   2 s. A run passes when under 1 interval in 100 is over 1.5 frames
   of the display, none is over 50 ms after the first second, and the
   95th percentile of its jumps is within 100 ms. A case passes when
   its three runs are valid and pass. The script holds if every case
   passes in Brave on the owner's Mac and F's peak element count is
   within a tenth of F1k's. Missing frames and the grouped cases'
   counts are recorded, not judged. On a miss, find the cause first
   (the same run with the fetch answering no rows tells layout from
   insertion), make one change by it and measure again. Only if it
   still misses, try one vendored, pinned virtualiser file under the
   same measure and record both.
10. **Headless first, then the owner.** Launch Chrome headless at
    each case's URL, on a profile directory under scratch, ended when
    the samples arrive: a first read, never the verdict. Commit, then
    start `acq web --no-browser` in the background, report its URL
    with the four cases' links, and end the turn.

## Find first
- `git grep -n -A12 "fn place" -- crates/acquisition-search/src/answer.rs`
  and `git grep -n "basis_changed\|pub after\|ITEM_ROWS" --` the same
  file: a continuation's place, its refusal, the item rows' bound.
- `git grep -n "pub fn token\|pub fn from_token" --` the same file:
  the request's token. `git grep -n "fn mint" -- crates/acquisition-cli/src/web`:
  the listener's, a different thing.
- `git grep -n "Tabs::read" -- crates/acquisition-cli/src/web/mod.rs`.
- `sed -n 1,40p crates/acquisition-search/examples/copy-as-store.rs`.
- `shasum -a 256` of the copy, before the backup and after the last
  run.

## Done when
- The first commit holds Decision 2's files and, per case, each
  headless run's numbers under Decision 8 and the bar's result, with
  the machine, the date and the commands.
- The second holds the same for Brave, the display's rate, the
  owner's words and the verdict first; the window script's size in
  lines and what it holds; the window's route as measured, drafted
  for the owner as one decision, which the launching session files.
  The copy outside the repository is then removed and pruned.
- The measurement copy's hash is the same before and after.
- `tools/gate.sh --docs` exits 0 on each commit.

## Not in scope
- The places document, `matched`, the place pages: step 6.5.
- The table's renderer, its heads and sort links: step 6.4.
- A change to the search or store crates' API or bounds.
- The check's driver: the next brief.
- A framework, Node, a bundler; a workflow or gate change.
- Every place open at once; Firefox and Safari, unless the owner
  opens them.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world over the wrapped copy
under scratch, with `ACQ_NO_SPAWN=1`. A browser the builder launches
is headless, on a profile directory under scratch, never the owner's
profile.

## Report
`TEMPLATE.md`.
