# The page seat, second blind run — the cards and the ways on

Written 2026-11-08 at the owner's word ("Run the seat"), before his
morning at the pages, after step 5 of 4-full part 1 (cards) closed.
The procedure is the seat skill's page door. This run is blind: the
runner reads this brief and the tool's own words and nothing else,
from a world outside the repository, and knows nothing of the earlier
runs.

You are an agent a person has handed a tool to. The tool is
Acquisition, an inventory tool for the game Path of Exile: it logs in
to the game's API, fetches the person's stash tabs and characters
through a local daemon, keeps them in a store on this machine, and lets
them search what they have. It has a command line, `acq`, and pages in
a browser, `acq web`, which are what this run is about. The tool is
what is under test, never you: where it fails you, that is the
finding, and a task you could not finish is as good a result as one
you could.

## Rules that bind this run

- **Your world is one directory and you stay in it:**
  `/Users/tom/seat-worlds/web-2026-11/cards-opus` (`W` below). You
  read two files in it and no others: this brief (`W/BRIEF.md`) and
  the section of `W/README.md` headed "The tour", from its heading to
  the next `##`. Open no other file anywhere: no source, no document,
  no history, no directory listing outside `W`, and search the web for
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
  yourself beyond editing a page's own address bar query.
- **The command line is reached through one door and no other,** for
  the tasks that ask for it and whenever a page leaves you no way on:

      python3 -I W/seat.py W W/bin run --t <task> [--note '…'] -- <acq arguments>
      python3 -I W/seat.py W W/bin run --t <task> --stdin - -- <acq arguments> <<'J'
      <what acq reads on stdin>
      J

  (`W` spelt out in full.) `run` runs one `acq` command and prints its
  exit code, its sizes, its time and its output, stdout and stderr
  apart; `--t` is the task the command is for (`T0` … `T8`); `--note`
  says why you ran it when that is not plain; `--out NAME` also saves
  the command's stdout as `W/work/NAME`. Run nothing else against the
  tool or its store: no `acq` by its path, no `acqd`, no `sqlite3`, no
  `cargo`, no `curl`, nothing that stops or restarts the two page
  servers or the daemon. Every output is read as it is printed, with no
  pipe or filter after the command.
- **The world is a rehearsal, and safe.** Both servers and the door
  point at a mock of the game's API and at a store in `W`; nothing you
  do can reach the real API or the person's real store, and you may
  write, save, rename, clear, refresh and apply freely. The mock hands
  out invented stash tabs and characters to whoever logs in.
- **Your report is `W/REPORT.md`**, written a part at a time as you
  finish each task, never all at the end. Besides it, `W/drafts/` and
  `W/work/` you write nothing, anywhere. Never commit, push, fetch,
  build, install or spawn an agent.
- No number of calls or pages is a target. A right result after many
  tries and a wrong one after few are both worth knowing; say which
  you have. Leave a task when your next move would be a guess you have
  already made, and say where the tool stopped you.

## The pane

Facts about the browser pane, not about the tool: the keyboard's
reload does nothing there, so reload a page by opening its address
again or following a link to it; after submitting a form, wait a moment
before reading; the pane's network listing shows only same-origin
fetches, so the result of a form submit is read from the page you land
on and the tab's own note; `read_page` with the interactive filter cuts
off silently on a long page, so prefer `get_page_text` or `find` there;
hidden fields and tooltips are read with the JavaScript tool, not the
text reads; a click by element reference can land beside its link
where a click by coordinate hits. Say in the report which pane tool
showed you each thing you quote.

## The world

Two accounts are in it, each served by its own page server:

- **`GERWARIC#7694`** is a copy of the person's real collection
  (22,721 items, as it stood in early November 2026). It has no login
  in this world: it is a store with no session, so a refresh of it
  cannot run. Its pages are the address in `W/web-collection.out`.
  This is where the searching is done.
- **`seat`** is logged in at the mock, with the mock's invented tabs
  and characters and a league called Rehearsal, and nothing fetched
  yet. Its pages are the address in `W/web-rehearsal.out`. This is
  where the refresh loop is done.

Saved searches are the world's, not an account's: one saved on either
pages or from the door shows on both. Both servers and the daemon were
started by the person who made the world; leave them running.

A friend of the person sent them this trade site search, to run over
their own stash:
`https://www.pathofexile.com/trade/search/Standard/H4sIAAAAAAAA_5XM0QrCMAyF4XfJdZFhB2N7FRlSuwiB2JYtFUfJu5speO9t_pyvwSZB6gZTg1yEcoIJcmJKCOo-0dqlgewFLYW0gIM7seD6DbTYGV-FKZKcjsHVn8fR90PXe_t9Bq548A8ye-hUZ3U_L6410o3xL9SEWd8TZHM0ugAAAA`.
The pages run such a search without touching the site; T4 uses it.

## The tasks

Each builds on the ones before. Record every one in the report,
whether or not it went as you expected, with what the page said quoted
from its text, not paraphrased.

- **T0 — Orient.** Read the tour section. Open the collection's pages.
  Before touching any control, write down what the pages offer, what
  each page seems to be for, and what you would do first.
- **T1 — Rings for life, as cards.** Find the rare rings in the
  collection with at least 70 to maximum life, however the search page
  lets you ask it. Say how the answer shows each item, what one item's
  entry tells you and what it does not, and how many items the answer
  holds. Then find the other way the page can show the same answer and
  say what each way is good for.
- **T2 — The same, typed and sorted.** Ask the same thing by typing a
  query in the page's own query language, if it has one, sorted with
  the most life first, showing as many rows as the page allows. Say how
  you learned the language's words, whether the count agrees with T1's,
  and what the page says about how many rows it shows and why.
- **T3 — Follow the links.** On an answer, follow every link the answer
  itself offers beside its numbers and its words (not the menu, not an
  item): say where each lands, whether what you found matched what the
  link said, and which numbers or words you expected to lead somewhere
  and did not.
- **T4 — A friend's search.** Run the trade site search above from the
  pages. Say what the page tells you about how faithfully it ran it,
  what it could not run, and how many items it found. Save that search
  under a name, open it from the saved searches page, and say whether
  what opens is what you ran.
- **T5 — One item.** From the answer, open one item. Say what its page
  tells you about it, what the answer's entry for the same item told
  you, and what you wanted either to tell you that it did not.
- **T6 — The terminal and the page.** Through the door, list the saved
  searches and run the one you saved. Then ask the door for the same
  rings search with each row carrying everything the item page shows,
  if the command line can; say how you found out whether it can and
  what the output looks like.
- **T7 — The refresh loop.** On the rehearsal account's pages, find out
  what the account holds, get its Rehearsal league's tabs listed
  however the pages or the door let you, choose some to fetch, make the
  plan, apply it, and see what landed. Say what each page said at each
  step and whether at any point you did not know what had happened.
- **T8 — As a player.** Name three things a player of this game would
  want to do with these pages that you could not do, or could not find
  how to do.

## The report

`W/REPORT.md`, under these headings in this order: Before touching
anything; The tasks (T0 to T8, each with what you did, what the page
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
