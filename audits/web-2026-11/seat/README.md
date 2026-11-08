# The page seat, 2026-11

The first seats at `acq web`: an agent at the pages through the desktop
app's browser pane, in a mock world over a copy of the owner's store.
Three runs: a probe, not blind, that proved the mechanics and walked the
saved pages (2026-11-07); and a blind run as the seat skill defines one
(2026-11-08); and a second blind run over the cards after step 5 closed
(2026-11-08, `BRIEF-cards.md` at `88fe3975`, retired with this record).
The procedure they settled is the seat skill's "The page
door" section.

## The runs

| Run | Brief | World | Build | Runner | Report | Door calls |
| --- | --- | --- | --- | --- | --- | --- |
| probe, 2026-11-07 | `BRIEF.md` (`97666261`) | the session's scratch, the measurement copy (`search/item-facts/raw/spike-GERWARIC_7694-2026-09-13.db`, 22,721 items) wrapped by `copy-as-store`, no daemon | debug `acq` at `97666261` | an Opus session started in the repository, not blind | `REPORT-probe.md` | none: the shell, unlogged |
| blind, 2026-11-08 | `BRIEF-blind.md` (`a773e274`) | `~/seat-worlds/web-2026-11/page-opus`, `setup --store` over the answer-facts pristine (`GERWARIC#7694`, 22,721 items) `--daemon`, `seat` logged in by the maker (idle 14400 s); two servers, `--account 'GERWARIC#7694'` and `--account seat` | release `acq` `f11ea336cb45…`, `acqd` `9f3569830aee…` at `a773e274` | an Opus session started in the world (`cwd`), reading the brief and the tour | `REPORT-blind.md`, `calls-blind.csv`, `setup-login-blind.csv` | 14 (T2 4, T4 5, T6 5); the pane's actions unlogged, cited by tool in the report |

The blind run's debrief questions (what it would not bet on, what rests
on its own knowledge of the game, what it changed and did not undo) were
headings of its report, answered there; no second turn was run. The
report's counts (103, 111, 132 matches; 787 lines) are the runner's and
were not recomputed here; the orchestrating session read all fourteen
door calls' rows and the servers' stderr (empty).

Outside the world: nothing. The runner's harness saved one long door
output (`[call 1]`, 33 KB) to a file outside the world and read it back;
the record names it as the skill's trap list says.

## The probe's stops, by fate

Recorded on the roadmap's ledger row for the pages' first sitting round
(`roadmap/LEDGER.md`, 2026-11-07): the wording and layout round
`eff2a9f5` (poe2 in the realm select, accessible names, the empty list's
words, the never-fetched sentence), the owner's ruling "Clear should ask
first" and its page `78dd6699`, and #180 to #188 for the rest.

## The blind run's faults, by fate

| Fault | Fate |
| --- | --- |
| 0. The door did not run at first: `W/bin` missing | the brief's (its maker copied the binaries one level up); fixed partway (`cp` into `W/bin`, the same bytes); the skill's page-door section names the path |
| 1. A fresh account's refresh loop dead-ends: no page lists a league's tabs or characters, none sets a policy | #189 (`area:frontends`, `trigger:seat`), with fault 12 |
| 2. The stat picker hides the language's totals; no page lists the language's words | #183, noted |
| 3. The Uniques pages advice repeats after a fetch that listed no pages | #191 (`area:plans`, `trigger:seat`): the plan does not carry whether the tab was fetched; a schema change, ruled first |
| 4. "Weighted Sum (exact)" offered with no weight box | #184, noted |
| 5. A save drops the sort and row count silently | #186, noted |
| 6. The query box cannot sort or page | fixed, `2fcd2cfc`: the query box carries the form's three view controls |
| 7. Refusal pages are dead ends, often naming command-line words | #190 (`area:frontends`, `trigger:seat`) |
| 8. What a fetch brought is not shown; the Jobs page's words | #182, noted |
| 9. No receipt or undo after save, rename, clear on the page | by ruling ("Redirect to the list", 2026-11-07; the clear asks first since `78dd6699`, which the runner met) |
| 10. `acq saved list`'s "next:" hint runs without `--account` and is refused in a two-account store | no fault: the refusal offered both full commands; the hint is the list's, not a store's |
| 11. "no characters known in Rehearsal (run `acq characters` …)" after they ran, all in Standard | fixed, `2fcd2cfc`: the line names the count and the leagues they are in |
| 12. The collection's Refresh page builds a plan with Apply while it has no session | #189, with fault 1 |
| 13. The item page's why line names the account by uuid | fixed, `2fcd2cfc`: the why line names the account as the scope line does |

Not faults, from the task sections: the collection's Jobs page shows the
`seat` account's jobs (one daemon, by design); the "Fetch the league
list" button's result shows only on the Refresh page's league picker
(T6 step 2; #182's kind); a click by `ref` landing beside its link (the
pane); the picker showing nothing on its first keystrokes (the runner
did not count it, nor does this record).

## The second blind run (the cards), 2026-11-08

After step 5 (cards) closed and before the owner's morning, at his
"Run the seat": `BRIEF-cards.md` (`88fe3975`), world
`~/seat-worlds/web-2026-11/cards-opus` over the same pristine
(`setup --store … --daemon`, `seat` logged in by the maker, the
binaries in the world's own `bin/` this time), release `acq`
`d403fcdeb2c5…` and `acqd` `a83d37ea826f…` at `247ab353`, two page
servers, an Opus session started in the world. Report `REPORT-cards.md`
(917 lines), `calls-cards.csv` (12 door calls: T2 1, T6 6, T7 5),
`setup-login-cards.csv`. All nine tasks finished. The orchestrating
session read the twelve calls' rows and the servers' stderr (empty),
and reproduced fault 10 against the running collection server before
sorting it (a hand-typed `view.limit=500` on cards returns 200 rows
and says "500 asked"; the runner's 500 rows were the table, which the
cap does not bound).

Outside the world: the runner decoded the brief's trade URL with one
`python3 -I -c` over the string (base64, gzip, JSON), read its
harness's saved copy of one long door output, and loaded its harness's
pane skill; nothing of the tool or the repository.

### The second blind run's faults, by fate

| Fault | Fate |
| --- | --- |
| 1. A saved trade search forgets it was partial | #193 (`area:frontends`, `area:store`, `trigger:seat`): the row keeps its source, a ruling first; until then the save form says the translation alone is saved (`f4ea403c`) |
| 2. The stat finder has no totals | #183, noted again |
| 3. The pages cannot list a league's tabs or plan without a policy | #189 |
| 4. No page says what a fetch brought | #182, noted |
| 5. A refused typed query lands on a page with no form | #190, noted |
| 6. The "more" page loses the presentation link and has no way back | fixed, `f4ea403c`: the continuation keeps the presentation link and gains a first-rows link |
| 7. The item page is a dead end | #194 (`area:frontends`, `trigger:seat`) |
| 8. "each tab of Standard" opens the whole list, not the never-fetched tabs | #194 |
| 9. "Fetch the league list" lands on Jobs with no leagues | #182, noted (a job submit lands on Jobs by the pages' shape) |
| 10. The Show menu stops at 200 while the address takes more | no fault as stated: the menu's choices are the page's, the 200-row cap is the item rows' and is said where it applies; the table has no cap (reproduced) |
| 11. The rehearsal pages never name their account | fixed, `f4ea403c`: the served account in every page's head |
| 12. The pages fetch item art from the game's image host | by ruling (C140, the art is the browser's); the trade box's "sent nowhere" is the search's |
| 13. The answer's default order is unexplained | fixed, `f4ea403c`: the rows line names the order where no sort is asked |
| 14. CLI rows under `item` are hard to read at column 0 | raised at step 5's close (the roadmap's ledger row), the owner's call |
| 15. The refresh table indents top-level tabs as if in the folder above | fixed, `f4ea403c` |
| 16. Two tab counts on the Refresh page; 61d against 58d | #182 holds the two counts; the ages are the scope's (the lists in scope differ when a league is a term), a wording for a later round |

Not faults, from the task sections: the terminal's `--saved` needing
`--account` in a two-account store (the refusal offers both commands,
as the first run found); the mock's tabs empty (the mock's design,
confirmed by `acq result`); "2 clauses not run" counting a group and
its one filter (the catalogue's count, each row a clause).

From the round, two more things the run asked for and the pages
already render: the "N counts carry no link" sentence said plainly, and
the query box's "Results as" control (`f4ea403c`).

## What the three runs taught the skill

In the skill's page-door section: a started session, from a desktop
turn, in the world as its `cwd`; one page server per account; the pane
logs nothing, so the door's calls are the run's only call log; the
runner writes its report with its own file tool; the pane's traps.
