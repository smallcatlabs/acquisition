# The page seat after the search form's series — the form, its rows, a site search and its link, the language on a page

Written 2026-11-23, after the ninth and last step of the search form's
series closed (`eab18de8`; `git log --grep='^Step: 4-full-1-form'`) and
before the owner sits at the pages. The procedure is the seat skill's
page door. This run is blind: the runner reads this brief and the
tool's own words and nothing else, from a world outside the
repository, and knows nothing of the earlier runs.

You are an agent a person has handed a tool to. The tool is
Acquisition, an inventory tool for the game Path of Exile: it logs in
to the game's API, fetches the person's stash tabs and characters
through a local daemon, keeps them in a store on this machine, and lets
them look through and search what they have. It has a command line,
`acq`, and pages in a browser, `acq web`, which are what this run is
about. The tool is what is under test, never you: where it fails you,
that is the finding, and a task you could not finish is as good a
result as one you could.

## Rules that bind this run

- **Your world is one directory and you stay in it:**
  `/Users/tom/seat-worlds/web-form/page-opus` (`W` below). You read
  two files in it and no others: this brief (`W/BRIEF.md`) and the
  section of `W/README.md` headed "The tour", from its heading to the
  next `##`. Open no other file anywhere: no source, no document, no
  history, no directory listing outside `W`, and search the web for
  nothing. Everything else you learn about the tool you learn from the
  tool: its pages, and `acq --help` or `acq <verb> --help` through the
  door below.
- **The pages are reached through the browser pane** (your
  `mcp__Claude_Browser__*` tools) at two addresses and no others.
  `W/web-collection.out` holds one line, the address of the pages over
  the person's collection; `W/web-rehearsal.out` holds the address of
  the pages over a second account that can refresh (the world, below).
  Open those two addresses, follow the links the pages themselves
  offer, and fetch nothing else: no other site, no address you compose
  yourself beyond editing a page's own address bar query. One link on
  these pages leaves them, for the game's trade site: read where it
  points and what the page says of it, and never open it.
- **The command line is reached through one door and no other,** for
  the tasks that ask for it and whenever a page leaves you no way on:

      python3 -I W/seat.py W W/bin run --t <task> [--note '…'] -- <acq arguments>
      python3 -I W/seat.py W W/bin run --t <task> --stdin FILE -- <acq arguments>

  (`W` spelt out in full; a file the tool reads is one you wrote under
  `W/drafts/`.) `run` runs one `acq` command and prints its exit code,
  its sizes, its time and its output, stdout and stderr apart; `--t` is
  the task the command is for (`T0` … `T10`); `--note` says why you ran
  it when that is not plain; `--out NAME` also saves the command's
  stdout as `W/work/NAME`. Run nothing else against the tool or its
  store: no `acq` by its path, no `acqd`, no `sqlite3`, no `cargo`, no
  `curl`, nothing that stops or restarts the two page servers or the
  daemon. Every output is read as it is printed, with no pipe or filter
  after the command.
- **The world is a rehearsal, and safe.** Both servers and the door
  point at a mock of the game's API and at a store in `W`; nothing you
  do can reach the real API or the person's real store, and you may
  search, save, clear, tick, plan, refresh and apply freely. The mock hands out
  invented stash tabs and characters to whoever logs in.
- **Your report is `W/REPORT.md`**, written with your own file tool a
  part at a time as you finish each task, never all at the end. Besides
  it, `W/drafts/` and `W/work/` you write nothing, anywhere. Never
  commit, push, fetch, build, install or spawn an agent.
- No number of calls or pages is a target. A right result after many
  tries and a wrong one after few are both worth knowing; say which
  you have. Leave a task when your next move would be a guess you have
  already made, and say where the tool stopped you.

## The pane

Facts about the browser pane, not about the tool: the keyboard's
reload does nothing there, so reload a page by opening its address
again or following a link to it; after submitting a form or clicking
something that changes the page, wait a moment before reading; the
pane's network listing shows only same-origin fetches, so the result
of a form submit is read from the page you land on and the tab's own
note; `read_page` with the interactive filter cuts off silently on a
long page, so prefer `get_page_text` or `find` there; hidden fields and
tooltips are read with the JavaScript tool, not the text reads; a
click by element reference can land beside its link where a click by
coordinate hits. A long list may show only the rows near the screen,
so say whether you scrolled. Say in the report which pane tool showed
you each thing you quote.

## The world

Two accounts are in it, each served by its own page server:

- **`GERWARIC#7694`** is a copy of the person's real collection
  (22,721 items, as it stood in early November 2026). It has no login
  in this world: it is a store with no session, so a refresh of it
  cannot run. Its pages are the address in `W/web-collection.out`.
  This is where the looking and searching is done.
- **`seat`** is logged in at the mock, with the mock's invented tabs
  and characters and a league called Rehearsal, and nothing fetched
  yet. Its pages are the address in `W/web-rehearsal.out`. This is
  where the fetching is done.

Saved searches are the world's, not an account's. Both servers and the
daemon were started by the person who made the world; leave them
running.

## The tasks

Each builds on the ones before. Record every one in the report,
whether or not it went as you expected, with what the page said quoted
from its text, not paraphrased. At each task say also whether, after
leaving a page, you could get back to where you were and found it as
you left it.

- **T0 — Orient.** Read the tour section. Open the collection's pages.
  Before touching any control, write down what the pages offer, what
  each page seems to be for, and what you would do first.
- **T1 — A search from the form.** The person wants every rare ring
  they own in Standard with at least 70 to maximum life. Ask it from
  the search form, the way a player fills in the game's trade site,
  typing no query. Say how you found each control, how you named the
  life line, what the page showed beside its Search button before you
  pressed it, how many rings there are, and what stood on the page
  with the answer.
- **T2 — Change the search on the page.** Keep the rings and the life,
  and also ask that fire and cold resistance together come to at least
  60, as the trade site's weighted sum does. Then take one line out
  again and run it. Say what you added and removed, what each control
  did when you used it, whether the number beside the button followed
  you, and whether anything on the form surprised you.
- **T3 — What the form says of itself.** Without running anything, say
  what the form tells you about how it reads a search: any tick box,
  mark or note that says a field is read differently from the trade
  site or not run at all, and how you would see the fields that are
  hidden. Then empty the form and say how.
- **T4 — A search from the trade site.** The person copied this from
  the trade site and wants it run over what they own:

      {"league":"Standard","query":{"status":{"option":"online"},"stats":[{"type":"and","filters":[{"id":"explicit.stat_3299347043","value":{"min":70}}]}],"filters":{"type_filters":{"filters":{"category":{"option":"accessory.ring"},"rarity":{"option":"rare"}}}}}}

  Find where the pages take it, run it, and say what the page did with
  it: how the answer compares with T1's, what the page says it ran and
  did not run, and what happened to the form. The page should offer a
  way back to the trade site for this search: find it, say where it
  points and what the page says goes with it and is left out of it,
  and do not open it.
- **T5 — Keep a search.** Save one of the searches you made, under a
  name. Find it again, open it, change one thing and run it. Say what
  a saved search holds, how the page showed it when you opened it,
  and whether you could tell a search made from the form from one made
  from the trade site's text.
- **T6 — A tab as it lies, and in an order you name.** Open one of the
  person's Standard stash tabs that holds currency and say what order
  its items stand in and what each line shows. Then order a tab of
  gear by a value you pick by its name, without typing a query: say
  where the pages let you name a value, what they offered when you
  typed, and how the page showed the order you chose.
- **T7 — Abyssal sockets.** Some of the person's items have abyssal
  sockets. Find them three ways if the pages let you: from the form's
  socket controls, from a line the items display, and by typing a
  query in the tool's own language. Say how many each found, whether
  the three agree, and how you learned the language's word.
- **T8 — The language's words.** The pages have a box for a query
  typed in the tool's language. Without the terminal, find what the
  language calls a ring's total life, and one thing it can ask that
  the form cannot. Say where the pages told you, what that page is
  like to read, and whether you came back to your search.
- **T9 — The terminal and the page.** Through the door, ask the
  command line for T4's search and say whether it agrees with the
  page, how you found the command, and whether it gives the same way
  back to the trade site.
- **T10 — As a player.** Name three things a player of this game would
  want to do with these pages that you could not do, or could not find
  how to do.

## The report

`W/REPORT.md`, under these headings in this order: Before touching
anything; The tasks (T0 to T10, each with what you did, what the page
said, what you expected, and which pane tool or door call showed it);
Faults, worst first (one line each, then its detail: what you did,
what happened, what should have, which task); What the tour and the
pages did not say; The tool's own words (words or phrases you did not
understand, quoted); What I would ask of the tool; Where I left the
page for the terminal, and why; What I read or ran outside the world
(there should be nothing); Left out; Which answers I would not bet on;
Which claims rest on my own knowledge of the game rather than on the
tool; Did I change the store in any way I did not undo.

Door calls are cited by their number (`[call N]` in the door's output).
A count in the report is yours, from what the page or the door
printed; say where it came from.

The run is accepted when `W/REPORT.md` holds every heading, every task
attempted or left with the reason, and a last message to the person of
at most 300 words saying what they most need to know.
