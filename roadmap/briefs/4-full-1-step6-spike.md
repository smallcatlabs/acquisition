# 4-full-1, step 6.3 — The spike: the window script

## Goal
One question is answered before step 6.4 is briefed: whether a
hand-written window script holds the table in the owner's Brave, flat
over every pc item of the measurement copy and grouped under pc
Standard's tabs. It is answered by building a rig, watching it and
changing it, not by a recipe written first: the owner, 2026-11-11,
"build, observe, and iterate instead of front loading the design
work" (#264). `main` gains a record and nothing else.

## Inputs
- `4-full-1-step6-design.md` §3 and §4, §5 item 3: what the table is
  to be. `4-full-1-design.md` §2: what a page may not hold.
- C46 (semantics in Rust), C98 (one basis per answer), C16 (the URL
  names the state), C171 (a commit lands before the owner looks).
- `answer.rs`: `Rows.after`, a row offset, refused across a changed
  basis; `RowField::Item`, the one path by which a row carries a
  price, under which an answer holds at most `ITEM_ROWS` rows.
  `acquisition_search::tabs`, as `web::tabs` reads it.
  `assets/count.js`: a fetching script's shape.
- The measurement copy,
  `search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`, SHA-256
  `fbae8786d79c67e3d9f277bae3e8ce60382f55eeabc2dc5bdb5f89965bccfda7`,
  read under the owner's grant of 2026-10-19; wrapped as a store by
  `crates/acquisition-search/examples/copy-as-store.rs`. In a mock
  world over it: `acq search --realm pc --limit 0` says 22,623
  matches, and `acq tabs --realm pc --league Standard` 3,347 tabs.
- The reviews of this brief's earlier forms (`git ls-files
  'roadmap/reviews/4-full-1-step6-spike-brief-*.md'`): each fault
  they found in a recipe is one the rig must not have.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **What does not move.** The server renders every row (C46). A
   window is asked by an offset and a count, Rust applying the bounds.
   The script keeps no order, no counter and nothing a reload must
   keep. The page scrolls as one document. A moved basis is refused
   and the page reloads. The search and store crates' API and bounds
   stay as they are. Everything else about the prototype is the
   builder's to find and the record's to say.
2. **Where it lives.** Uncommitted edits in a detached copy of `main`
   outside the repository, under scratch (`git worktree add
   --detach`). No route joins `acq web` by this step; the route the
   table will need is drafted for the owner at the end.
3. **The cases.** F: the empty query in pc, flat. F1k: a query
   matching about 1,000, flat. G0: pc Standard's tabs as head rows,
   stash order, none open. G1: G0 with its largest tab open, its rows
   under its head.
4. **The rig, and its commit.** The rig is the prototype and a page
   that measures it. When it works, one commit to `main` adds the
   record, `audits/web-2026-11/spike/README.md`, saying what the rig
   is, read off the working page: the viewport, each case's request,
   the row's height, how the script turns a scroll into a window, the
   scroll and the jumps a run makes. It names the bar the runs will be
   judged by, before any judged run, covering at least: frames that
   come late; the time from a jump to its rows; frames with a row
   missing; a row off its place, which voids a run; and the live
   DOM's size not growing with the row count, the record saying what
   it counts. Two things the reviews settled: the steady scroll is
   at a wheel's speed, and frames with a row missing are judged.
   Beside the record: the copy's whole diff from its base commit,
   assets included, as a patch that passes the gate's `diff` stage
   (a blank context line may lose its space; `git apply` reads it).
   The record names the base commit and the command that proved the
   patch applies to it.
5. **That commit is reviewed cold, as a build** (the cold-review
   skill), against this brief; its findings are answered in the rig.
6. **Then the judged runs, each set its own commit.** Headless
   first: Chrome on a profile directory under scratch. Then the
   owner's: start `acq web --no-browser` in the background, report
   its URL with the four cases' links, and end the turn; his numbers
   and his words go in with the verdict.
7. **A miss is a turn of the loop.** Find its cause, change the rig,
   commit what changed and why with the new patch, and judge again.
   Every implementation that was judged keeps its patch. A vendored,
   pinned virtualiser file is tried only after a miss whose cause was
   found and answered once.

## Find first
- `git grep -n -A12 "fn place" -- crates/acquisition-search/src/answer.rs`
  and `git grep -n "basis_changed\|pub after\|ITEM_ROWS" -- crates/acquisition-search/src/answer.rs`:
  a continuation's place, its refusal, the item rows' bound.
- `git grep -n "pub fn token\|pub fn from_token" -- crates/acquisition-search/src/answer.rs`:
  the request's token.
- `git grep -n "Tabs::read" -- crates/acquisition-cli/src/web/mod.rs`.
- `sed -n 1,40p crates/acquisition-search/examples/copy-as-store.rs`.
- `shasum -a 256 search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`,
  before the backup and after the last run.

## Done when
- The rig's commit holds Decision 4's record and patch, and its
  review is saved and answered.
- A commit holds the headless runs judged by the bar, and a later one
  the owner's runs in Brave, the display's rate, his words and the
  verdict first: whether the hand-written script holds.
- The last commit gives the window script's size in lines (`wc -l`
  of it) and what it holds, and drafts the window's route for the
  owner as one
  decision, filed by the launching session and named in the commit's
  message; the copy outside the repository is removed and pruned.
- The measurement copy's hash is the same before and after.
- `tools/gate.sh --docs` exits 0 on each commit.

## Not in scope
- The production renderer, its heads and sort links: step 6.4. The
  rig renders the least a row needs: name, base, price, location and
  the query's values at one fixed height.
- The places document, `matched`, the place pages: step 6.5.
- The check's driver: `4-full-1-step6-driver.md`.
- A framework, Node, a bundler; a workflow or gate change.
- Firefox and Safari, unless the owner opens them.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world over the wrapped copy
under scratch, with `ACQ_NO_SPAWN=1`. A browser the builder launches
is headless, on a profile directory under scratch, never the owner's
profile.

## Report
`TEMPLATE.md`.
