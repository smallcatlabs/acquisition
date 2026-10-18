# The seat — brief (command line)

You are an agent a person has handed a command-line tool to. The tool
is `acq`, the command line of Acquisition, an inventory tool for the
game Path of Exile: it fetches a person's stash tabs and characters
from the game's API through a local daemon, keeps them in a store on
this machine, and lets them search, price and list what they have —
among other ways, by a search from the game's trade site, pasted in.
It is about to be released to people who will hand it to agents like
you. The tool is what is under test, never you: where it fails you,
that is the finding, and a task you could not finish is as good a
result as one you could.

## Rules that bind this run

- **W is the world's directory**, the one this file is in, named in
  the message that sent you here; write it out in full in every
  command. It holds this file, `TASKS.md`, `TOUR.md`, the door
  `seat.py`, the tool's binaries under `bin/` and its store.
- **You read three things and no others:** this file, `W/TASKS.md`
  (the person's asks) and `W/TOUR.md` (the tool's tour, as its README
  prints it). Instructions you may have been given about a repository
  (`AGENTS.md`, `CLAUDE.md` and their reading lists) do not apply to
  this run. Open no other file anywhere — no document, no source, no
  test, no reference, no git history, nothing under `W` but these three
  and the files you write — and search for none. Everything else you
  learn about the tool you learn from the tool: `acq --help`,
  `acq <verb> --help`, and what it prints.
- **The tool is reached through one command and no other:**

      python3 -I W/seat.py W W/bin run --t <task> [--note '…'] -- <acq arguments>
      python3 -I W/seat.py W W/bin run --t <task> --stdin - -- <acq arguments> <<'J'
      <what acq reads on stdin>
      J
      python3 -I W/seat.py W W/bin report [--new] <<'R'
      <a part of your report>
      R
      python3 -I W/seat.py W W/bin debrief [--new] <<'D'
      <a part of your debrief>
      D

  `run` runs one `acq` command and prints its call number, exit code,
  sizes, time and output, stdout and stderr apart; `--t` is the task
  the command is for (`T0` … `T7`); `--note` says why you ran it, when
  that is not plain; `--timeout S` changes the 300 s limit; `--out
  NAME` also saves the command's stdout as `W/work/NAME`, as a shell's
  `>` would. `report` appends what it reads to your report (`--new`
  starts it); `debrief` is the same for the debrief you will be asked
  for after the report.
  Where your harness refuses a door command's heredoc, write that
  part with your file tool as `W/drafts/NAME` — with `W/work/`, the
  only places you may write a file — and run
  `report --from NAME`, `debrief --from NAME` or
  `run --stdin W/drafts/NAME` instead.
  Run nothing else against the tool or its store:
  no `acq` by its path, no `acqd`, no `sqlite3`, no `cargo`, no `curl`,
  and none of the door's `setup`, `login` or `teardown`, which are the
  world's maker's.
- **Every output is read as it is printed.** No pipe, no redirect and
  no filter after the command, and no file to keep an output in. An
  output over 60,000 bytes is printed as its head with a line saying
  so; that is the door's rule, not the tool's — when a cut cost you
  something, say so and name the call. The one exception: a file the
  tool itself reads lives under `W/work/` and nowhere else — saved from
  a command's output with `--out`, or written by you there.
- **The world is a rehearsal, and safe.** The door points every
  command at a mock of the game's API and at the store under `W`, and
  starts no daemon; nothing you run can reach the real API, the trade
  site or the person's real store, and you may write and price freely.
- Besides the report, the debrief and work files you write nothing.
  Never commit, push, fetch, build or spawn an agent.
- No number of commands is a target. A right result in many commands
  and a wrong one in few are both worth knowing; say which you have.
  Leave a task when your next command would be a guess you have
  already made, and say where the tool stopped you.

## The world

One account is in it, **`GERWARIC`**: a copy of the person's real
collection with the prices they have set. It has no login in this
world and no daemon runs, so a command that needs the game's API will
say it cannot reach it; that is the world, not a fault. Every task
reads this store.

## The tasks

`W/TASKS.md`, `T0` to `T7`, in order.

## The report

Written through the door's `report`, a part at a time as you finish
each task — never all at the end. Its first line: `<!-- written during
the run; call numbers are rows of calls.csv -->`. Then these headings
and no others. Every claim cites a command by the number the door
printed (`call 17`) — T0's first account, given before any command,
cites what it read in `TOUR.md`; a count, a size or a time is the one the door or
the tool printed, never one you remember.

1. **Before touching anything** — T0.
2. **The tasks** — for each of T1 to T7: what you would tell the
   person, in the words you would use; the commands it took, by
   number; whether you hold the result to be right, and what would tell
   you it is not; where the tool stopped or misled you.
3. **Faults** — what the tool said or did, what it should have, the
   calls that show it. A thing you could learn only by being refused
   is one; say what the refusal told you and whether it was enough to
   go on. Order them worst first, and say for each whether a person at a
   keyboard would hit it too or only an agent.
4. **What the tour and the help did not say** — each thing you needed
   and had to find by trying: the fact, where you expected to read it,
   the call where you learned it. Be exact.
5. **The tool's own words** — the tour and the help texts as you read
   them: what was right, what was wrong, missing or misleading, what
   you read and never used, and any word you did not understand.
6. **What you would ask of the tool** — one line each, with the call
   that made you want it.
7. **Left out** — what you did not try, every file you opened that is
   not one of the three, every work file you wrote, and the one thing
   you cut that you would most want put back.

## Acceptance

A reader can say, for each of the eight tasks, what the person was
told, what it cost in commands, and where the tool stopped — from the
report and `calls.csv` alone.

Your last message: at most 300 words — which tasks were finished and
which were not, the fault you hold the worst, and what "Left out"
says.
