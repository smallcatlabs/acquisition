# The page seat, blind run — `acq web` driven through the browser pane

Written 2026-11-08 at the owner's word ("Let's do both of those"),
after the probe run beside this file proved the mechanics. This run is
blind: the runner reads this brief and the tool's own words and nothing
else, from a world outside the repository.

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
  `/Users/tom/seat-worlds/web-2026-11/page-opus` (`W` below). You
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

Facts about the browser pane, not about the tool, learned in the probe
run: the keyboard's reload does nothing there, so reload a page by
opening its address again or following a link to it; after submitting
a form, wait a moment before reading; the pane's network listing shows
only same-origin fetches, so the result of a form submit is read from
the page you land on and the tab's own note; `read_page` with the
interactive filter cuts off silently on a long page, so prefer
`get_page_text` or `find` there; hidden fields are read with the
JavaScript tool, not the text reads. Say in the report which pane tool
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

## The tasks

Each builds on the ones before. Record every one in the report,
whether or not it went as you expected, with what the page said quoted
from its text, not paraphrased.

- **T0 — Orient.** Read the tour section. Open the collection's pages.
  Before touching any control, write down what the pages offer, what
  each page seems to be for, and what you would do first.
- **T1 — Rings for life.** Find the rare rings in the collection with at
  least 70 to maximum life, however the search page lets you ask it.
  Say what the page's controls gave you on the way, what you tried that
  did not work, and how many items the answer holds.
- **T2 — The same, typed.** Ask the same thing by typing a query in the
  page's own query language, if it has one, sorted with the most life
  first. Say how you learned the language's words, whether the count
  agrees with T1's, and what the answer says about the sort.
- **T3 — Keep it.** Save T2's search under a name you choose. Find it on
  the saved searches page, open it, and say whether what opens is what
  you saved. Rename it. Save a second search and clear that one. Say
  what each step told you had happened, and what you would have wanted
  told.
- **T4 — The terminal and the page.** Through the door, list the saved
  searches and run the one you saved on the page. Save one from the
  door and find it on the page. Say what the two sides agree and
  disagree on.
- **T5 — One item.** From any answer, open one item. Say what its page
  tells you about it and what you wanted it to tell you that it did
  not.
- **T6 — The refresh loop.** On the rehearsal account's pages, find out
  what the account holds, choose some tabs and characters of the
  Rehearsal league to fetch, make the plan, apply it, and then see what
  landed. Say what each page said at each step, and whether at any
  point you did not know what had happened or what would happen next.
- **T7 — The jobs.** Find where the pages show what the daemon is doing
  or did, during or after T6. Say what it tells a person and what it
  does not.
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
