# 4-full-1, step 6.5a — The places and their pages

## Goal
At `acq web`, "Stash tabs" and "Characters" replace "Refresh". Each
lists one league's places with their facts and a tick. A folder, a
map tab and a unique tab stay closed until opened, the URL names what
is open and what is ticked, and opening one leaves the reader where
they were. A place's name opens the search scoped to it, and a league
with nothing listed offers the listing, never a command to type.

Step 6.5 is four parts, a brief each:
- **6.5b**, the grouped table: a query joins the places (#244,
  #269); the list's filter becomes the request's, with the
  never-fetched view (#194).
- **6.5c**, the row's ways: an item's card under its row (#270,
  #194, #186, #190).
- **6.5d**, the plan and jobs pages from their documents (#182, #189).

## Inputs
- `4-full-1-step6-design.md` §3, §6, and §7's rulings: stash order;
  `tabs` and `characters` join and `refresh` retires; no plan schema
  change (#191).
- #248, #189, #269, #270, #182.
- #284, taken under delegation: the listings' routes. No wait.
- C16 (the URL names the state), C46 (semantics in Rust), C138 (the
  listener), C168 (no page writes a policy), C187 (the browser check).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **The places document** (search crate, a new module, recorded in
   its commit). One maker over one store read gives a league's stash
   tabs as a tree in stash order (a folder holding its tabs, a map
   or unique tab its pages, by `grouped`'s rule, moved here) or its
   characters in the store's order. A place is its store row whole.
   A parent counts the places it holds and those never fetched. A
   place that can hold items carries the request that lists them:
   `id:<its id>` over its realm, with the league for a tab or a
   page, from one function with the `tab` count's bucket. The
   league's counts stand once, as `tabs` counts them. *Not* keys on
   `tabs`: that answer is bounded, and a character is no tab. No CLI
   flag, no MCP tool: a `tab` count gives an agent the request.
2. **The pages.** `GET tabs` and `GET characters`: the realm and
   league selects, the league's counts in one line, and a table of
   head rows in the columns each list has today. Never fetched and
   fetched-and-empty read differently, in the words the cells have
   now (#191). No `pre` and no command line (#182). Heads are plain
   rows, no window: with parents closed (#269) a level is short, a
   tick in a dropped block would be lost, and an anchor lands with
   no script. The boxes under the heads stay, `filter.js` as it is,
   over the rows shown.
3. **The URL names what is open and ticked.** The table and its
   buttons are one GET form. `open` names each open parent, `tab` or
   `character` each ticked place, and the server renders both from
   the URL. A parent's opener is a button sending `toggle`; the
   answer is 303 to the page with `open` changed and no `toggle`, at
   that head's anchor. So ticks travel through every open and close.
   One under a closed parent travels hidden and is counted in words.
   A tick and an open are one league's: the realm and league selects
   are a form of their own and carry neither, since one id can stand
   in two leagues. *Not taken:* links, which drop the ticks.
4. **Ticks to the plan.** "Refresh ticked" sends the form to `plan`
   as "Refresh selected" does, the substashes box with it; a map or
   unique tab's page has no tick, as now. Each page plans its own
   ticks. "Plan under the policy" shows only where the policy names
   the league.
5. **The listing.** "List this league's stash tabs" (`POST tabs`)
   and "List characters" (`POST characters`), each its own form,
   submit the `stashes` and `characters` jobs as `fetch_leagues`
   submits `leagues`, landing on Jobs. Each stands beside "Fetch the
   league list", and where the table would be when none is listed.
6. **A place's name** links its request as a table, spelled by
   `Paged` as every way on; a folder's is words. `Paged` spells
   `Way::Tabs` `tabs`.

## Find first
- `git grep -n -i 'refresh' -- crates/acquisition-cli/src/web crates/acquisition-cli/tests/web.rs crates/acquisition-cli/tests/harness crates/acquisition-cli/tests/goldens/web`:
  what retires; the jobs page's reload and the plan's words stay.
  `grep -n 'refresh loop' README.md`: the tour.
- `git grep -n -i 'refresh page' -- crates`: the docs elsewhere.
- `git grep -n -E 'grouped\(\)|TabGroup' -- crates`: its readers;
  with one, it moves.
- `git grep -n -E 'Of::Tab\(|fn label_of' -- crates/acquisition-search/src/counts.rs`:
  a tab's route.
- `git grep -n -E 'fn tabs_document|pub fn characters\(|Way::Tabs|fn policy_shown|jobs::submit' -- crates`:
  the two reads, the tabs way, the policy, a submit.
- `git grep -n -E 'pub const SCRIPTS|pub fn start_browser|fn a_selection_submits|fn a_tab_bucket_is' -- crates`.

## Done when
- Search crate, over a fixture with a folder, a map tab with pages
  and a page whose tab is gone: the tree, its order and each count.
  Each place's request answers the items the store counts there, a
  tab's with its pages'. The `tab` bucket's route and that tab's
  request are one text (tests).
- Goldens hold a tabs page (a never-fetched tab, a fetched empty
  one, a closed parent with its counts, an open one), a characters
  page, and a league with nothing listed. No place page holds a
  `pre` or `acq ` (test).
- A test follows each navigation entry to its page, and the name of
  a tab, a page and a character to a table of that place's items; a
  folder's name is no link.
- A toggle answers 303 to a URL with `open` changed, no `toggle`,
  the head's anchor and every tick; again, it closes. A tick under
  a closed folder reaches the plan: its selection is the URL's
  ticks. With one id in two leagues, ticked in one, the other's
  page, reached by the selects, ticks nothing (tests).
- From a world with nothing listed, the pages alone list, tick,
  plan and apply (the refresh loop's test). "Plan under the policy"
  shows with a policy and not without (test).
- `refresh` answers no page and no page names it. Each golden and
  test that held it is changed, or retired with its line (C154, a
  retirement names its holder).
- A browser test, on a tabs page several viewports long: after a far
  parent's opener is clicked, its head is in the viewport, its pages
  under it, and a box ticked before is still ticked.
- The module doc and the README's tour say the pages. The close
  closes #248 and #191 and says what is left of #189, #182, #269
  and #270.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- What 6.5b, 6.5c and 6.5d hold; the plan, apply and jobs pages; a
  change to `filter.js`.
- The search form's reshaping (#184, #181, #180, #185, #188, #183,
  #193): the step after. The look (#285).
- A plan schema change, a policy written from a page, a change to
  `tabs` or `characters` on the CLI or the MCP, a field on a `Way`.

## Rules
`TEMPLATE.md`, and:
- **The plan first.** Run "Find first", write one screen to `plan.md`
  in your notes folder (what each command found, the document's
  keys, the functions and tests, anything here the tree disproves),
  end the turn and wait.
- Two commits at least: the document, then the pages.
- The round is reviewed cold (C175, what is reviewed).

## Report
`TEMPLATE.md`.

## Taken from the review

`roadmap/reviews/4-full-1-step6-places-brief-92f0be8e.md`. 1, a tick
across leagues: the selects are their own form, with the two-league
case. 2, the never-fetched link: moved to 6.5b with the filter;
there the page spells the answer's `tabs_pointer`, as `tabs_command`
does, with no field on a `Way`. 3, the navigation and the names:
followed in a test. 4, the forms: said. 5, the find commands:
widened and run. 6, "cut at ten": no number. Not reviewed again: the
build's review is the check.
