# 4-full-1, step 6.5a — The places and their pages

## Goal
At `acq web`, "Stash tabs" and "Characters" replace "Refresh". Each
lists one league's places with their facts and a tick. A folder, a
map tab and a unique tab stay closed until opened, the URL names what
is open and what is ticked, and opening one leaves the reader where
they were. A place's name opens the search scoped to it, and a league
with nothing listed offers the listing, never a command to type.

Step 6.5 is four parts, a brief each:
- **6.5b, the grouped table**: a query joins the places (`matched`),
  and a place opened lists its rows under its head (#269, #244).
- **6.5c, the row's ways**: an item's card under its row, its place a
  link, the reader's place kept (#270, #194); #186; #190.
- **6.5d, the plan and jobs pages from their documents** (#182, and
  #189's missing session).

## Inputs
- `4-full-1-step6-design.md` §3, §4's last item, §6, and §7's
  rulings: stash order; `tabs` and `characters` join and `refresh`
  retires; no plan schema change (#191).
- #248, #189, #269, #270, #194's second point, #182.
- #284, taken under delegation: the listings' routes. No wait.
- C16 (the URL names the state), C46 (semantics in Rust), C138 (the
  listener), C168 (no page writes a policy), C187 (the browser check).
- `tabs.rs` (`grouped`), the store's `TabRow` and `CharacterRow`,
  `web/mod.rs` (`tabs`, `plan`, `Paged`).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **The places document** (search crate, a new module, recorded in
   its commit). One maker over one store read gives a league's stash
   tabs as a tree in stash order (a folder holding its tabs, a map
   or unique tab its pages, by `grouped`'s rule, moved here) or its
   characters in the store's order. A place is its store row whole. A parent counts the places
   it holds and those never fetched. A place that can hold items
   carries the request that lists them: `id:<its id>` over its
   realm, with the league for a tab or a page, from one function
   with the `tab` count's bucket. The league's counts stand once, as
   `tabs` counts them. *Not* keys on `tabs`: that answer is cut at
   ten, and a character is no tab. No CLI flag and no MCP tool: a
   `tab` count gives an agent a place's request.
2. **The selection is the request's**: words a place's name holds, a
   type, never fetched. A parent is listed where it passes or holds
   a place that does, and counts those. The boxes under the heads
   and `filter.js` retire (the design's §4).
3. **The pages.** `GET tabs` and `GET characters`: the realm and
   league selects, the league's counts in one line, and a table of
   head rows in the columns each list has today. Never fetched and
   fetched-and-empty read differently, in the words the cells have
   now (#191). No `pre` and no command line (#182). Heads are plain
   rows, no window: with parents closed (#269) a level is short, a
   tick in a dropped block would be lost, and an anchor lands with
   no script.
4. **One form, and the URL names it.** The page is one GET form.
   `open` names each open parent, `tab` or `character` each ticked
   place, and the server renders both from the URL. A parent's
   opener is a button of the form sending `toggle`; the answer is
   303 to the page with `open` changed and no `toggle`, at that
   head's anchor. So a reload is the page, and ticks travel through
   every open, close and "Show". A tick the page does not show
   (under a closed parent, outside the selection) travels hidden and
   is counted in words; one for a place the listing lacks is
   dropped. *Not taken:* links (they drop the ticks) and `<details>`
   (the URL names nothing).
5. **Ticks to the plan.** "Refresh ticked" sends the form to `plan`
   as "Refresh selected" does, the substashes box with it; a map or
   unique tab's page has no tick, as now. Each page plans its own
   ticks. "Plan under the policy" shows only where the policy names
   the league.
6. **The listing.** "List this league's stash tabs" (`POST tabs`)
   and "List characters" (`POST characters`) submit the `stashes`
   and `characters` jobs as `fetch_leagues` submits `leagues`,
   landing on Jobs. Each stands beside "Fetch the league list", and
   where the table would be when nothing is listed.
7. **A place's name** links its request as a table, spelled by
   `Paged` as every way on; a folder's is words.
8. **The never fetched.** The scope line's link names the tabs page
   cut to them, as the terminal's command does. `Paged` spells the
   tabs way `tabs`.

## Find first
- `git grep -n -E '/refresh|action=.{1,2}refresh|link\("refresh"|Refresh selected|Plan under the policy|col-filter|filter\.js|each tab of' -- crates/acquisition-cli README.md`:
  every page, golden, test and list naming what retires.
- `git grep -n -E 'grouped\(\)|TabGroup' -- crates`: the grouped
  listing's readers; with one, it moves and retires.
- `git grep -n -E 'Of::Tab\(|fn label_of' -- crates/acquisition-search/src/counts.rs`:
  where a tab's route is made.
- `git grep -n -E 'fn tabs_document|pub fn characters' -- crates/acquisition-store/src`:
  the two reads.
- `git grep -n -E 'Way::Tabs|fn tabs_command|fn policy_shown|jobs::submit' -- crates`:
  the tabs way, the policy's read, a job's submit.
- `git grep -n -E 'fn the_refresh_loop|fn a_selection_submits|SCRIPTS|fn start_browser' -- crates/acquisition-cli/tests`:
  what to change.
- `git grep -n -i 'refresh page' -- crates README.md REVIEW-CHECKLIST.md`:
  the docs naming the page.

## Done when
- Search crate, over a fixture with a folder, a map tab with pages
  and a page whose tab is gone: the tree, its order and each count.
  Each place's request answers the items the store counts there, a
  tab's with its pages'. The `tab` bucket's route and that tab's
  request are one text. A parent is listed for a passing page (tests).
- Goldens hold a tabs page (a never-fetched tab, a fetched empty
  one, a closed parent with its counts, an open one), a characters
  page, and a league with nothing listed. No place page holds a
  `pre` or `acq ` (test).
- A toggle answers 303 to a URL with `open` changed, no `toggle`,
  the head's anchor and every tick; the same again closes it. A tick
  under a closed folder reaches the plan: its selection is the URL's
  ticks (tests).
- From a world with nothing listed, the pages alone list the tabs
  and characters, tick, plan and apply (the refresh loop's test).
  "Plan under the policy" shows with a policy and not without (test).
- The scope's link opens a page listing the never fetched alone.
- `refresh` answers no page; no page names `filter.js`. Each golden
  and test that held them is changed, or retired with its line
  (C154, a retirement names its holder).
- A browser test, on a tabs page several viewports long: after a far
  parent's opener is clicked, its head is in the viewport, its pages
  under it, and a box ticked before is still ticked.
- The module doc says the pages. The close closes #248 and #191 and
  says what is left of #189, #182, #194, #269 and #270.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- `matched`, a grouping, rows under a head: 6.5b. An item under its
  row, #186, #190: 6.5c. The plan, apply and jobs pages: 6.5d.
- The search form's reshaping (#184, #181, #180, #185, #188, #183,
  #193): the step after. The look (#285).
- A plan schema change, a policy written from a page, a change to
  `tabs` or `characters` on the CLI or the MCP.

## Rules
`TEMPLATE.md`, and:
- **The plan first.** Run "Find first", write one screen to `plan.md`
  in your notes folder (what each command found, the document's
  keys, the functions and tests, anything here the tree disproves),
  end the turn and wait for the answer.
- Two commits at least: the document, then the pages.
- The round is reviewed cold (C175, what is reviewed).

## Report
`TEMPLATE.md`.
