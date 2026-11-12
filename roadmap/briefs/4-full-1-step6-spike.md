# 4-full-1, step 6.3 — The spike: the window script and the check's driver

## Goal
Two questions are answered by measurement before step 6.4 is briefed:
whether a hand-written window script holds the table in the owner's
Brave, flat at 22,623 rows and grouped under 3,347 head rows; and what
drives the browser-driven check, at what cost to the gate. `main`
gains a record and nothing else; the prototype is a local branch.

## Inputs
- `4-full-1-step6-design.md` §3 (a window is asked within one place),
  §4 (one row height, the script by hand first, one day in Brave),
  §5 item 3.
- `4-full-1-design.md` §2 (what a page may not hold; the check is for
  a presentation rule inside a script) and §7 item 6 (the driver: a
  headless browser from a Rust test with no Node, or a CI-only job,
  measured by what it adds to the gate; two shapes to the owner).
- `decisions/frontends.md`, "Not doing": no Node or bundler in the
  gate.
- C46 (semantics in Rust), C98 (one basis per answer), C138 (a new
  route is the owner's), C16 (the URL names the state).
- `answer.rs`, `Rows.after`: a row offset, refused across a changed
  basis. `web/mod.rs`: the routes, the guard, `search?next=<token>`.
  `assets/count.js`: a fetching script's shape (a timer and an
  `AbortController`, no counter).
- The measurement copy,
  `search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`, SHA-256
  `fbae8786d79c67e3d9f277bae3e8ce60382f55eeabc2dc5bdb5f89965bccfda7`,
  read under the owner's grant of 2026-10-19; wrapped as a store by
  `crates/acquisition-search/examples/copy-as-store.rs`
  (`search/MEASUREMENTS.md`, M3's method): 22,623 live pc items of its
  22,721; pc Standard lists 3,347 tabs.
- CI on `main` at `9163283a`, hosted runners: Gate (Linux)
  43148807954, Gate (Windows) 43148808004, Committer 43148808005, all
  green.

## Decisions
1. **Where it lives.** A branch `spike/step6-table` off `main`, local,
   never pushed or merged. `main` gains one commit: the record,
   `audits/web-2026-11/spike/README.md`, with the window script and
   the measuring script as files beside it. The Rust stays on the
   branch; step 6.4 builds its own.
2. **The window's shape.** The server writes on the table its row
   count, its row height and a window URL whole but for a row number.
   The script reads the scroll, appends the number, and places the
   fragment it receives: rows rendered by Rust. A spacer holds the
   count times the height. On the branch a `rows` route answers the
   fragment, carrying the request and its basis as `next`'s token does
   and taking the offset as `after`; a moved basis is refused and the
   script reloads the page. The script holds a timer, a controller and
   a few fragments: no counter, no order, no arithmetic but offset by
   height. A route is the owner's, so the record ends with the route
   as measured, drafted for him.
3. **A row** is one fixed height: name, base, price, location and the
   query's values, overflow clipped. Enough to measure, not 6.4's
   renderer.
4. **The cases.** F: the empty query in pc, flat. F1k: a query
   matching about 1,000, flat. G0: pc Standard's 3,347 places as head
   rows in plain DOM, stash order, none open. G1: G0 with its largest
   place open by a URL field, its rows a block windowed by the same
   script.
5. **The page measures itself.** A measuring script, the branch's
   alone, runs a steady scroll top to bottom in 20 s, then 20 jumps to
   fixed fractions, and records: the intervals between animation
   frames (median, p95, max, the share over 1.5 frames of the
   display), a jump's time to its rows (median, max), the element
   count (at rest, peak), fetches and bytes, the heap where the
   browser gives it. It posts them to a route of the branch that
   writes a file under scratch. Each case three times, the window in
   front, the display's rate recorded.
6. **The bar, named before the first run.** The hand-written script
   holds if in Brave on the owner's Mac, in every case: under 1
   interval in 100 is over 1.5 frames and none is over 50 ms after the
   first second; a jump shows its rows within 100 ms at p95; and F's
   peak element count is within a tenth of F1k's. On a miss, find the
   cause first (the same run with the fetch answering no rows tells
   layout from insertion), make one change by it and measure again.
   Only if it still misses, try one vendored, pinned virtualiser file
   under the same measure and record both.
7. **The driver.** One check, written once per candidate: load F,
   scroll to row 15,000, assert the rows shown are the server's for
   that offset and the element count is bounded.
   - (a) A CDP client crate as a dev-dependency of the cli crate.
     Read the maintained ones' current docs; take one whose licence
     `deny.toml` allows and that downloads no browser.
   - (b) Hand-written: the browser launched headless with a debugging
     port, the protocol spoken over a hand-written loopback WebSocket
     client, no new crate. Capped at half the spike's time; record
     how far it got.
   For each: crates added to `Cargo.lock`, `cargo deny check`, the
   check's time (median of 20) and failures in 20, the browsers here
   it drives (Brave, Chrome), what a machine must have installed.
   Ruled out: WebDriver (a driver binary matched to each browser's
   version) and anything that fetches a browser.
8. **The check's home is the owner's.** The record gives both shapes
   with the numbers: a gate stage (a browser on every gate machine; a
   missing one fails, never skips) or a CI-only job on hosted runners.
   What a hosted image has installed is read from its published list
   and said as read, not run. No workflow or gate change, on either
   branch.
9. **Headless first, then the owner.** Run the measuring page through
   the driver, headless, for a first read: labelled headless, never
   the verdict. Then start `acq web` in the background, report its
   URL with the four cases' links, and end the turn. The owner opens
   them in Brave, runs each and scrolls by hand; his words go in the
   record beside the numbers.

## Find first
- `git grep -n "after" -- crates/acquisition-search/src/answer.rs`:
  how a continuation is read and refused across a basis.
- `git grep -n "fn token\|\"next\"" -- crates/acquisition-cli/src/web`:
  how the page's token is made and read.
- `sed -n 1,40p crates/acquisition-search/examples/copy-as-store.rs`:
  how the copy becomes a store.
- `shasum -a 256` of the copy, before the backup and after the last
  run.
- `grep -c '^name = ' Cargo.lock`, before each driver candidate.
- `git grep -n "SCRIPTS" -- crates/acquisition-cli/tests`: what a
  served script joins, for the record's note to 6.4.

## Done when
- The record is on `main`, its verdict first: per case, for Brave and
  for the headless run, Decision 5's numbers and the bar's result; the
  machine, the display's rate, the date, the commands; the owner's
  words.
- It gives the window script's size in lines and what it holds, the
  file beside it, and the route as measured.
- It gives the driver's table (Decision 7) and the two homes
  (Decision 8).
- Three drafts for the owner, one `trigger:ruling` issue each: the
  window's route, the driver, the check's home.
- The copy's hash is the same before and after.
- `tools/gate.sh --docs` exits 0 on `main`'s commit.

## Not in scope
- The table's renderer, its heads and sort links: step 6.4.
- The picker's check; a change to a workflow or the gate.
- A framework, Node or a bundler.
- Every place open at once; the flat table answers that.
- Firefox and Safari, unless the owner opens them: then recorded.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world over the wrapped copy
under scratch, with `ACQ_NO_SPAWN=1`. A browser the builder launches
is headless, on a profile directory under scratch, never the owner's
profile, and opens no window.

## Report
`TEMPLATE.md`.
