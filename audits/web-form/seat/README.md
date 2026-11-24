# The page seat after the search form's series, 2026-11-23

A blind agent at `acq web`'s pages after the ninth and last step of the
search form's series closed (`eab18de8`): the form and its rows, a
site search and its link back, a saved search, a tab in its own order
and by a named value, abyssal sockets, the language's index. The last
seat at the pages was the one after GUI step 6 (`audits/web-step6/seat/`,
2026-11-19). The procedure is the seat skill's page door. Two runs: the
first, its fix round, and a rerun on the fixed pages. The owner saw each
run's split and took it ("Yes, as split", twice, 2026-11-23); the tables
hold the fates taken.

## The runs

| Run | Brief | World | Build | Runner | Report | Door calls |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 2026-11-23 | `BRIEF.md` (`280f361d`) | `~/seat-worlds/web-form/page-opus` (set aside as `page-opus-1`), `setup --store` over `~/seat-worlds/web-2026-11/pristine/store` (`GERWARIC#7694`, 22,721 items) `--daemon`, `seat` logged in by the maker (idle 14400 s, `setup-login-1.csv`); two servers, `--account 'GERWARIC#7694'` and `--account seat` | release `acq` `5df33a95daac…`, `acqd` `eac175ca54cd…` at `eab18de8` | an Opus session started in the world (`cwd`), reading the brief and the tour | `REPORT-1.md`, `calls-1.csv` | 3 (T9); the pane's actions unlogged, cited by tool in the report |
| 2, the rerun, 2026-11-23 | the same brief | a fresh world at the same path over the same pristine, `seat` logged in by the maker (`setup-login-2.csv`); two servers | release `acq` `d608b9ea4e05…` at `f2e754d0`, `acqd` unchanged | a fresh Opus session started in the world, on the owner's "Start the rerun" (the app refused the start from a turn he had not typed) | `REPORT-2.md`, `calls-2.csv` | 3 (T9); the pane's actions unlogged |

Both runs attempted and answered all eleven tasks. There is no
`DEBRIEF.md`: the debrief questions are headings of each report (the
page door, item 4). Each report's counts are its runner's. The
orchestrating session read each run's three door calls and outputs and
both servers' stderr (empty), and checked the faults marked reproduced
below against the running servers. The pane was visible in both runs
(each cites screenshots). No rerun followed the second run's one fix:
it is narrow, and was checked on the fixed build's pages over the
world's store (one box on the empty page and on a form answer, in the
search form; one on a tab opened by its name, in the query box, and a
pick there keeps the tab).

Outside the world: nothing of the tool or the repository, by each
report. Each runner printed the tour with one `awk` over `README.md`,
listed `W` once, and decoded the trade link's id in the page with the
browser's own stream, fetching nothing; neither opened the link. Each
appended one part of its report by a shell heredoc where the brief asks
for the file tool.

## What held, by both runs

Rare rings in Standard with 70 life are 101 from the form, from the
site's JSON pasted in and from the terminal, and the three agree. The
link back to the trade site is on the page and in the terminal's
output, the same in both, and decodes in the browser to the query as
asked. A saved search opens filled, is edited and run. A tab reads in
its own order; a table is ordered by a value picked by name. The
language's index is reached from the query box and read.

## The first run's faults

| # | Fault (the runner's numbering) | Checked | Fate |
| --- | --- | --- | --- |
| 1 | A weighted sum of the form's own pseudo totals is dropped whole, the answer partial | reproduced: the group is unsupported with a bound and presence without | **later**, #316 (`trigger:ruling`): the trade area's "Not doing" line on user-authored weighted sums waits on a recorded question, and this is one |
| 2 | The displayed line "Has 1 Abyssal Socket" is not found by the stat with a bound | reproduced, and worse than reported: `min` 1 found 0 as implicit and 5 as explicit of 71 | **fixed**, #315: sat (round ze), then `0683bb1e`, `fd5cb638`, `fec7c9af`: a digit where a text has a `#` is the form's number, for every text; five forms moved. The plurals the table's reader still misses are #318 |
| 3 | The count beside Search rose when a minimum was added, with no word that a group had stopped running | reproduced | **fixed**, #313, `2d42824b`: the count says how many clauses would not run |
| 4 | No implicit variant offered for the life and resistance lines | not a fault as stated: the implicit lines are listed, lower than the runner read | the finder's order is **later**, #317 |
| 5 | A site search loses its status once edited in the form | by design | **by ruling**: a search edited in the form leaves the listing's fields out and says so (the form's design, §2 and §7) |
| 6 | The saved list cannot tell a form search from a pasted one | by design in what is stored (C194) | **later**, #317: the words |
| 7 | A stash tab's table first shows only its header | not reproduced: the server sends a tab's first 50 rows; the runner's screenshot followed a scroll to the head; not looked at in a browser here | **no fault shown** |
| 8 | "Show" is 20 from an empty form and 10 elsewhere | reproduced | **fixed**, #314, `7092ea1d`: one maker, the default first and sent empty |
| 9 | A column picked by its site name is headed by the language's word | as built | **later**, #317, and the look (#285) |
| 10 | Clicks by reference land beside links | the pane's | **no fault** |

## The rerun's faults

| # | Fault | Checked | Fate |
| --- | --- | --- | --- |
| 1 | "Add a column" beside the form's Search sends the empty form on an answer made from the query box: a tab of 37 became 22,623 matches | reproduced | **fixed**, #319, `bf5e78c9`: one box on a search page, in the editor that holds the search shown |
| 2 | A weighted sum over the pseudo totals is not run | the first run's 1 | **later**, #316 |
| 3 | The live count says something other than what the group does; a dash after a Remove until the next keystroke | the mark of #313 shows; the dash not reproduced | **later**, noted on #312, whose test asserts that count |
| 4 | The answer under an edited form is not marked stale | by design: the count is the form's, the answer the search's that ran | **later**, #317 (commented) |
| 5 | The Search button moves as the count's text grows | a consequence of #313's longer text | **later**, #285 (commented) |
| 6 | A tab's stack column is off-screen in a narrow pane | as built | **later**, #285 (commented) |
| 7 | A saved search does not say what it came from or holds | the first run's 6 | **later**, #317 |
| 8 | The Status select shows a value the form will not send | as ruled, the banner says so | **by ruling**; the two reading against each other is #317 (commented) |
| 9 | Clear also resets the league and the switch | by design: the league is a term of the search, and the switch is not remembered | **no fault** |
| 10 | A form answer's address is thousands of characters long | as a GET form sends every field | **later**, #317 (commented) |
| 11 | The collection's Jobs page lists the other account's job | one daemon, as the first page seat's record says | **no fault** |

What the fix round was for, as the rerun met it: abyssal sockets came
to 73 by the Sockets box, by the displayed line and by the language;
the count beside Search said "not the site's search: 3 of its clauses
not run"; and it named no difference in the rows shown by door.

Each report's "what the tour and the pages did not say" is input for
the pages' words, not a list of faults: what "Weighted Sum v2" is, what
a blank weight means, what "alone exact · with a bound ambiguous"
means, that the socket box "A" is the abyssal count, that three lines
of one text in an And group ask for all three, and that the terminal
asks for `--account` where a page does not. As a player, both runners
could not price an item from the pages, copy an item's text, or compare
an item with what a character wears.

The brief is retired with this record: `audits/web-form/seat/BRIEF.md`
at `280f361d`. Both worlds' stores are torn down; the pristine stays.
