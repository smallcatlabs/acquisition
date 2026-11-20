# The page seat after GUI step 6, 2026-11-19

A blind agent at `acq web`'s pages after step 6 of 4-full part 1
closed (`9e367764`): the place pages, the value columns that sort, the
grouped table under a query, an item opening under its row, and the
plan, apply and jobs pages. The last seat at the pages was the cards
run of 2026-11-08 (`audits/web-2026-11/seat/`), before every part of
step 6. The procedure is the seat skill's page door. The owner saw
the now/later split and approved it on 2026-11-19 ("yes, go ahead with
the now fixes as well as comments or issues as appropriate for the
later issues"); the table holds the fates taken.

## The run

| Run | Brief | World | Build | Runner | Report | Door calls |
| --- | --- | --- | --- | --- | --- | --- |
| page-opus, 2026-11-19 | `BRIEF.md` (`4e7475b3`) | `~/seat-worlds/web-step6/page-opus`, `setup --store` over `~/seat-worlds/web-2026-11/pristine/store` (`GERWARIC#7694`, 22,721 items) `--daemon`, `seat` logged in by the maker (idle 14400 s, `setup-login.csv`); two servers, `--account 'GERWARIC#7694'` and `--account seat` | release `acq` `040f8625f920…`, `acqd` `f0e966fd2fc5…` built before the rebase, from `9e367764`'s tree and the brief | an Opus session started in the world (`cwd`), reading the brief and the tour | `REPORT.md`, `calls.csv` | 7 (T2 1, T6 4, T7 2); the pane's actions unlogged, cited by tool in the report |

The runner was started from the owner's own turn in the orchestrating
session ("Start the runner"): the app refused the start from a turn
another session began, and refused a proposal card naming the world's
folder. All nine tasks were attempted and answered. There is no
`DEBRIEF.md`: the debrief questions are headings of the report (the
page door, item 4). The report's counts are the runner's; the
orchestrating session read all seven door calls' rows and outputs and
the servers' stderr (empty), checked faults 1, 4, 8, 13, 14 and 15
against the running servers' HTML and the source, and faults 2, 3 and
7 against step 6's briefs; the hrefs the report quotes for 7 were not
fetched again.

The pane was hidden the whole run (the runner's `read_page` saw a 0×0
viewport), so it read by `get_page_text` and the JavaScript tool and
saw no pixels.

Outside the world: the runner extracted the tour section with one
`awk` over `README.md`, listed `W` once, and loaded its harness's pane
skill; nothing of the tool or the repository. The report's last
section was appended by a shell heredoc and then edited with the file
tool.

## The faults, now and later

The owner's word on the surface, 2026-11-19, now C191, governs the split: "acq
web is helping us bootstrap acq's development. it was never intended
to be a high performance desktop app, and it would be wrong to warp it
into such." A finding that asks the pages for desktop-app speed is
later or not this surface, never now. "Now" is what keeps an agent or a
player from doing the work from the pages' own words, and is small.
Each later fault is a comment on its issue or a new one, each pointing
back at its row here.

| # | Fault (the runner's numbering; 13 to 15 from its other sections) | Checked | Fate |
| --- | --- | --- | --- |
| 1 | The collection's place pages have no league control: Stash tabs and Characters open on Standard, and "93 matching items in these stash tabs, of 103 matches" never says the other ten are on characters (8) and in Allflame (2) | reproduced: the select is built from the stored league list alone (`league_select`, `web/page.rs`), and this store has none and no session to fetch one (`acq leagues --stored`: "No league list on record") | **fixed**, `079b0200`, `dd10c229`, `a879ceb2`: with no league list, the league control lists the leagues the store holds places in, and a page's own league stands in it whatever it lists; the places document's query carries `elsewhere`, and the page says each kind and league holding the rest, a link to its page, a league holding none of its kind included |
| 2 | Sorting is out of reach without the query language: a tab or answer with no compared value has no column head to click, and "in the store's order (no sort asked)" names no way to ask | by design (step 6.4c: a value column's head sorts, a card's number adds a column); the runner found the card's numbers only at T3 | **words fixed**, `b22b8984`: where no sort orders the rows, the cards' line, the table's and an open place say a number on a card orders them. **Later**: the picker of named values is the search form's reshaping (`roadmap/README.md`, "Next", item 1) |
| 3 | A tab's line is too thin to use: no stack size in a currency tab, rarity only as a colour, rows in item-id order rather than the tab's | by design: the always-shown columns are the owner's (the step 6 design's §7), and a place's rows are in the view's order | **later**, #296 (`area:frontends`, `trigger:ruling`): the stack size, rarity in words and a place's own order, the owner's columns |
| 4 | The tick boxes are named by id: the pane names them "rh-cur1", "fake0001", not "Currency", "StashHoarder" | reproduced: `<input type="checkbox" name="tab" value="rh-cur1">` with no label | **fixed**, `9851e7aa`: each tab's and character's tick box is named by the place |
| 5 | Jobs says "done" but not what a fetch brought, names a tab by id ("Tab rh-cur1 in Rehearsal") where the plan said "Tab Currency", and links to nothing | known | **later**, #182 (commented) |
| 6 | Every fetch lands on Jobs, and the league list's job offers no way back to the page it came from | the navigation's links stand on every page; the way back to a league's places shows only on a league's jobs | **later**, #295 (commented) |
| 7 | An item's place link opens the tab at the tab's row, not at the item, and drops the query it was opened under | by design for the query (step 6.5c's brief, item 6, "*Not taken:* the query on it") | **anchor fixed**, `5124b1c5`: a card's place link lands at the card's own row, a removed item's at its place's head. **Later**: the query on it, #194 (commented) |
| 8 | A running job's "started 0s ago" never moved on reopening its address | the jobs page is a snapshot with no script or refresh; reopening an address with its `#` fragment is the pane's trap (a same-address open does not reload) | **later**, #182 (commented); a live page is part 2's wire, not this surface |
| 9 | The card loads item art from the game's image host | by ruling (C140, as the cards run found) | no fault |
| 10 | The collection's Jobs page lists the other account's work | one daemon, every account's jobs (as both earlier runs found) | no fault; the sentence saying so, #182 (commented) |
| 11 | The windowed list's last block loaded only on a wheel scroll, not a scripted one | the pane was hidden (a 0×0 viewport); the owner's word above | no fault of this surface |
| 12 | "This job is not among those listed below." over a refresh whose children are listed below it | the wording is the jobs page's | **later**, #182 (commented) |
| 13 | The tour's `acq jobs [--watch] [--all] \| status <id> \| result <id>` reads, by the tour's own `acq auth status \| check \| logout`, as `acq jobs result`, which is refused ([call 3]) with no offer of `acq result` | reproduced: the tour line, `README.md` | **fixed**, `7a7538c5`: the tour puts each top-level verb on its own line, `acq profile \| leagues` and `acq cancel \| set-priority` split alike, and the tour test refuses two top-level verbs as one line's alternatives |
| 14 | The query box refused the pane's `form_input` ("element is not fillable") | a plain `<textarea id="query-text" name="text">`; the hidden pane's | no fault |
| 15 | "3347 tabs in Standard … facts: 3331 tabs", and "215 tabs not shown" of what | 215 is right: 261 top-level places unfiltered, 46 shown; the two counts are #182's, noted by the cards run | **later**, #182 (commented: the two counts, and "places" for what a fold counts) |

Ten faults (1 to 8, 12, 13). Fixed, in part or whole: 1, 2's words, 4,
7's anchor and 13 (`7a7538c5`, `9851e7aa`, `5124b1c5`, `b22b8984`,
`079b0200`), reviewed cold three times (C176): the round's review found
fault 1 incomplete twice, each answered by one rule (`dd10c229`, a place
page names its own league and says where the matches stand when its
league holds none; `a879ceb2`, the same with no league to offer), and
the third was clean (`roadmap/reviews/web-step6-seat-079b0200.md`,
`-fixes-dd10c229.md`, `-fixes-a879ceb2.md`).
Later: 2's picker to the search form's reshaping; 3 to #296, filed; 5,
8, 12 and 15's wording to #182, 6 to #295 and 7's query to #194, each
commented. No fault: 9, 10, 11, 14.

What the runner said works: an opened tab or item stands under its row
and closing it returns the reader to the row; the plan names exactly
what will be fetched; "Items matching" on the place pages shows the
answer by place ("N of M"); every count it checked against the command
line agreed (103 rings: 93 in Standard's tabs, 8 on characters, 1 and
1 in Allflame; the first five rows of the life order).
