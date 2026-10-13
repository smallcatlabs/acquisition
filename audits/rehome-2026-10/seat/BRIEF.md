# The whole-CLI seat — brief (note 36, step 1; 2026-10-13)

You are an agent a person has handed a command-line tool to. The tool
is `acq`, the command line of Acquisition, an inventory tool for the
game Path of Exile: it logs in to the game's API, fetches the person's
stash tabs and characters through a local daemon, keeps them in a
store on this machine, and lets them search, price and list what they
have. It is about to be released to people who will hand it to agents
like you. The tool is what is under test, never you: where it fails
you, that is the finding, and a task you could not finish is as good a
result as one you could.

The report you write is the input for the release's README: what an
agent needed that the tool's own words did not say.

## Rules that bind this run

- **You read two things and no others:** this file, and the section
  of `README.md` at the repository root headed "The tour" — that
  section alone, from its heading to the next `##`. The repository's
  entry points (`AGENTS.md`, `CLAUDE.md`) list documents to read before
  any work: for this run they do not apply. Open no other file here —
  no document, no source, no test, no reference, no git history — and
  search for none. Everything else you learn about the tool you learn
  from the tool: `acq --help`, `acq <verb> --help`, and what it prints.
- **The tool is reached through one command and no other,** run from
  the repository root:

      python3 -I audits/rehome-2026-10/seat/seat.py run --t <task> [--note '…'] -- <acq arguments>
      python3 -I audits/rehome-2026-10/seat/seat.py run --t <task> --stdin - -- <acq arguments> <<'J'
      <what acq reads on stdin>
      J
      python3 -I audits/rehome-2026-10/seat/seat.py login <user>

  `run` runs one `acq` command and prints its exit code, its sizes, its
  time and its output; `--t` is the task the command is for (`T0` …
  `T9`); `--note` says why you ran it, when that is not plain;
  `--timeout S` changes the 300 s limit. `login` is a scripted login,
  described below. Run nothing else against the tool or its store: no
  `acq` by its path, no `acqd`, no `sqlite3`, no `cargo`, no `curl`.
- **Every output is read as it is printed.** No pipe, no redirect and
  no filter after the command, and no file to keep an output in. An
  output over 60,000 bytes is printed as its head with a line saying
  so; that is the door's rule and not the tool's. The one exception:
  a file the tool itself asks for or writes (a plan to apply, a
  template, a policy passed as `@FILE`) may be written under
  `runs/seat-2026-10-13/work/` and nowhere else — when a verb needs its
  own earlier output back as a file, say so in the report, since that
  is a thing the door made harder than a shell would.
- **The world is a rehearsal, and safe.** The door points every
  command at a mock of the game's API and at a store under `runs/`;
  nothing you run can reach the real API or the person's real store,
  and you may write, price, refresh and cancel freely. The mock hands
  out invented stash tabs and characters to whoever logs in.
- **No terminal.** You have no interactive terminal: a verb that draws
  a full-screen interface (`acq dash`) will not work for you. Try it
  once, record what it said, and move on.
- You write one file, `audits/rehome-2026-10/seat/REPORT.md`, and
  nothing else besides the work files above. Never commit, push,
  fetch, build or spawn an agent.
- No number of commands is a target. A right result in many commands
  and a wrong one in few are both worth knowing; say which you have.
  Leave a task when your next command would be a guess you have
  already made, and say where the tool stopped you.

## The world

Two accounts will be in it:

- **`GERWARIC`** is already there: a copy of the person's real
  collection (about 54 MB) with the prices they have set. It has no
  login in this world — it is a store with no session. The tasks that
  read, search and price use it.
- **A second account, which you log in.** In real use a person logs in
  through a browser (`acq auth`). You have none, so the door does it:
  `seat.py login seat` starts `acq auth --no-browser --json` and
  approves it at the mock. That login is the one thing in this run
  that is not the tool as a person meets it; do not report its
  mechanics as a fault. The tasks that talk to the API use this
  account. If the tool later tells you the session is gone, log in
  again and note that it happened and what the tool said.

## The tasks

Each is what a person would ask, in their words where it matters. Do
them in order; later ones build on earlier ones.

- **T0 — orient.** Before touching anything: from the tour and the
  help alone, say what this tool can do, which commands need the
  daemon or a login and which do not, and what you would not dare run
  without asking the person first. Then look at the world as you find
  it — which accounts, what is stored — without changing it.
- **T1 — log in and look around.** Log in the second account. "Who am
  I logged in as, what leagues and characters do I have, and what
  stash tabs?" Fetch one character and one stash tab and say what is
  in them.
- **T2 — a first sync.** "I want everything in one league of mine kept
  up to date." Find out how the tool wants that said, say it, get the
  tool's plan for the work and read it — what will it fetch, how many
  requests, how long — and then carry it out. Show that the items
  landed. Then ask for a plan again: what does the tool say there is
  left to do?
- **T3 — the queue.** Submit several pieces of work at once so that
  some of them wait. "What is running, what is waiting, and when will
  it finish?" Cancel one that is waiting, change the priority of
  another, read one finished job's result, and say what the tool told
  you at each step. Start something without waiting for it and come
  back for its result.
- **T4 — what I have.** On `GERWARIC`'s store, with no daemon
  involved: "How many items do I have, where are they, and how old is
  what you know?" Tabs, characters, counts, the age of the data, and
  anything the store says it refused or could not read.
- **T5 — find things.** Still on `GERWARIC`: (a) "Do I have any gear
  with six linked sockets?" (b) "Which of my rare rings have life and
  at least two resistances?" (c) "How many items do I have per league
  and per rarity?" (d) pick one item from an answer and show
  everything the tool knows about it. Learn the query language from
  the tool.
- **T6 — prices.** Still on `GERWARIC`: "What have I priced, and is
  anything wrong with my prices?" Then: price one stash tab at 5 chaos
  per item; price one single item at 2 divines; change your mind about
  the item and remove its price; and put the tab back as you found it.
  Say what the tool told you about each write and how you would know
  another program had changed the prices between your read and your
  write.
- **T7 — the shop.** "Give me my shop as forum pages I can paste."
  Produce them, say how many pages, what was left out and why, and
  what a person does with the output next.
- **T8 — for a program.** Take five commands you have already run,
  one from each of T1, T2, T4, T5 and T6, and run them again asking
  for JSON. Is each answer one JSON document a program could read,
  including when the command fails? Make three commands fail on
  purpose — a wrong account name, an id that does not exist, a
  malformed query or policy — once as text and once as JSON, and say
  whether each failure told you what to do next.
- **T9 — when things go wrong.** Stop the daemon and say what each
  kind of command does then: one that only reads the store, one that
  only observes the daemon, one that needs the API. Find out what the
  tool says about its own version and its daemon's, and where its
  logs are. Leave the daemon stopped when you finish.

## The report — `audits/rehome-2026-10/seat/REPORT.md`

Its first line: `<!-- hand-written during the run of BRIEF.md; call
numbers are rows of calls.csv -->`. Then these headings and no others.
Every claim cites a command by the number the door printed (`call
17`); a count, a size or a time is the one the door or the tool
printed, never one you remember. Write it as you go, not at the end.

1. **Before touching anything** — T0.
2. **The tasks** — for each of T1 to T9: what you would tell the
   person; the commands it took, by number; whether you hold the
   result to be right, and what would tell you it is not; where the
   tool stopped or misled you.
3. **Faults** — what the tool said or did, what it should have, the
   calls that show it. A thing you had to learn by being refused is
   one. Order them worst first, and say for each whether a person at a
   keyboard would hit it too or only an agent.
4. **What the tour and the help did not say** — each thing you needed
   and had to find by trying: the fact, where you expected to read it,
   the call where you learned it. This section is the README's input;
   be exact.
5. **The tool's own words** — the tour and the help texts as you read
   them: what was right, what was wrong, missing or misleading, what
   you read and never used, and any word you did not understand (the
   tool cites rulings as `C` and a number; say whether those helped or
   got in the way).
6. **What you would ask of the tool** — one line each, with the call
   that made you want it.
7. **Left out** — what you did not try, every file you opened that is
   not this brief or the tour, every work file you wrote, and the one
   thing you cut that you would most want put back.

## Acceptance

A reader can say, for each of the ten tasks, what the person was told,
what it cost in commands, and where the tool stopped — from the report
and `calls.csv` alone.

Your last message: at most 300 words — which tasks were finished and
which were not, the fault you hold the worst, the three things the
README most needs to say, and what "Left out" says.
