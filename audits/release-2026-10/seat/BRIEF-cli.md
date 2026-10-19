# The release seat — brief (command line)

You are an agent a person has handed a command-line tool to, with its
README. The tool is `acq`, the command line of Acquisition, an
inventory tool for the game Path of Exile: it fetches a person's stash
tabs and characters from the game's API through a local daemon, keeps
them in a store on this machine, and lets them search, price and list
what they have. It is about to be released to people who will hand it
to agents like you, and the README is what they will read first. The
tool and its README are what is under test, never you: where either
fails you, that is the finding, and a task you could not finish is as
good a result as one you could.

## Rules that bind this run

- **W is the world's directory**, the one this file is in, named in
  the message that sent you here; write it out in full in every
  command. It holds this file, `README.md`, the door `seat.py`, the
  tool's binaries under `bin/` and its store.
- **You read two things and no others:** this file and `W/README.md`,
  the tool's README as the release carries it. Instructions you may have been
  given about a repository (`AGENTS.md`, `CLAUDE.md` and their reading
  lists) do not apply to this run. Open no other file anywhere — no
  document, no source, no test, no reference, no git history, nothing
  under `W` but these two and the files you write — and search for
  none. Where the README points at another file (a reference, a
  decisions file, a policy), that is a pointer you cannot follow here:
  say so where it mattered. Everything else you learn about the tool
  you learn from the tool: `acq --help`, `acq <verb> --help`, and what
  it prints.
- **The tool is reached through one command and no other:**

      python3 -I W/seat.py W W/bin run --t <task> [--note '…'] -- <acq arguments>
      python3 -I W/seat.py W W/bin run --t <task> --stdin - -- <acq arguments> <<'J'
      <what acq reads on stdin>
      J
      python3 -I W/seat.py W W/bin run --t <task> --env ACQ_IDLE_SHUTDOWN=<seconds> -- <acq arguments>
      python3 -I W/seat.py W W/bin login [--env ACQ_IDLE_SHUTDOWN=<seconds>] <user>
      python3 -I W/seat.py W W/bin report [--new] <<'R'
      <a part of your report>
      R
      python3 -I W/seat.py W W/bin debrief [--new] <<'D'
      <a part of your debrief>
      D

  `run` runs one `acq` command and prints its call number, exit code,
  sizes, time and output, stdout and stderr apart; `--t` is the task
  the command is for (`T0` … `T8`); `--note` says why you ran it, when
  that is not plain; `--timeout S` changes the 300 s limit; `--out
  NAME` also saves the command's stdout as `W/work/NAME`, as a shell's
  `>` would. `--env` sets the one knob a README line puts before a
  command, the daemon's idle time, on that command alone; it takes no
  other. `login` is a scripted login, described below, and takes the
  same `--env`. `report` appends what it reads to your report (`--new`
  starts it); `debrief` is the same for the debrief you will be asked
  for after the report.
  Where your harness refuses a door command's heredoc, write that
  part with your file tool as `W/drafts/NAME` — with `W/work/`, the
  only places you may write a file — and run
  `report --from NAME`, `debrief --from NAME` or
  `run --stdin W/drafts/NAME` instead.
  Run nothing else against the tool or its store: no `acq` by its
  path, no `acqd`, no `acq-mcp`, no `sqlite3`, no `cargo`, no `curl`,
  no `tar`, and none of the door's `setup` or `teardown`, which are the
  world's maker's.
- **Every output is read as it is printed.** No pipe, no redirect and
  no filter after the command, and no file to keep an output in. An
  output over 60,000 bytes is printed as its head with a line saying
  so; that is the door's rule and not the tool's — when a cut cost you
  something, say so and name the call. The one exception: a file the
  tool itself reads (a plan to apply, a template, a policy passed as
  `@FILE`) lives under `W/work/` and nowhere else — saved from a
  command's output with `--out`, or written by you there when you
  compose it yourself.
- **The world is a rehearsal, and safe.** The door points every
  command at a mock of the game's API and at the store under `W`;
  nothing you run can reach the real API or the person's real store,
  and you may write, price, refresh and cancel freely. The mock hands
  out invented stash tabs and characters to whoever logs in.
- **No terminal, no browser.** You have no interactive terminal: a
  verb that draws a full-screen interface (`acq dash`) will not work
  for you; try it once, record what it said, and move on. A login
  needs a browser, which you have not: the door's `login` does what a
  person does there (below).
- Besides the report, the debrief, a draft under `W/drafts/` and a
  work file under `W/work/` you write nothing. Never commit, push,
  fetch, build or spawn an agent.
- No number of commands is a target. A right result in many commands
  and a wrong one in few are both worth knowing; say which you have.
  Leave a task when your next command would be a guess you have
  already made, and say where the tool stopped you.

## The world

The binaries under `W/bin` are the contents of the release's archive
for this machine, `acquisition-1.0.0-beta.1-aarch64-apple-darwin.tar.gz`,
unpacked there as the README's install section says to unpack it; the
world's maker did that, not you, and nothing is on your `PATH`. The
store holds two accounts:

- **`GERWARIC`** is already there: a copy of the person's real
  collection (about 50 MB) with the prices they have set. It has no
  login in this world — it is a store with no session. The tasks that
  read, search and price use it.
- **A second account, which you log in.** In real use a person logs
  in through a browser (`acq auth`). You have none, so the door does
  it: `seat.py W W/bin login seat` starts `acq auth --no-browser
  --json` and approves it at the mock. That login is the one thing in
  this run that is not the tool as a person meets it; do not report
  its mechanics as a fault. The tasks that talk to the API use this
  account. This world has no keyring: what the README says of a login
  without one holds here. The door starts the world's daemon with a
  30-minute idle exit, so it outlasts your thinking between commands;
  the tool ships with one minute, and the README's recipe for a longer
  one is T2's. If the tool later tells you the session is gone, log in
  again and note that it happened and what the tool said.

## The tasks

Each is what a person would ask, in their words where it matters. Do
them in order; later ones build on earlier ones.

- **T0 — orient.** Before touching anything: from the README alone,
  say what this tool can do, what it needs (a login, a daemon, the
  network) for what, what the README told you to do before a first
  command, and what you would not dare run without asking the person
  first. Then look at the world as you find it — which accounts, what
  is stored — without changing it.
- **T1 — the install.** "I unpacked the archive like the README said.
  Is everything there, and does it work?" From the README's install
  section and what the tool prints: what of the archive's contents
  can you confirm from here; does `acq` find its daemon beside it;
  what does it say of its own version. Say which of the README's install
  steps a person on this machine (a Mac) would still have to do, and
  which you could not check from here.
- **T2 — log in, and a daemon for an agent.** Log in the second
  account. "Who am I logged in as, what leagues and characters do I
  have, and what stash tabs?" Fetch one character and one stash tab
  and say what is in them. Then the README says how a person keeps a
  daemon up for an agent's session: do that, as the README says, for
  this world (no keyring). The result to show: that a new daemon is
  up — not the one your login started — and what it says its idle
  exit is.
- **T3 — what I have.** On `GERWARIC`'s store, with no daemon
  involved: "How many items do I have, where are they, and how old is
  what you know?" Tabs, characters, counts, the age of the data, and
  anything the store says it refused or could not read.
- **T4 — find things.** Still on `GERWARIC`: (a) "Do I have any gear
  with six linked sockets?" (b) "Which of my rare rings have life and
  at least two resistances?" (c) "How many items do I have per league
  and per rarity?" (d) pick one item from an answer and show
  everything the tool knows about it. Learn the query language from
  the tool.
- **T5 — prices.** Still on `GERWARIC`: "What have I priced, and is
  anything wrong with my prices?" Then: price one stash tab at 5 chaos
  per item; price one single item at 2 divines; change your mind and
  make it 3 divines, the careful way — a replacement that names the
  revision you read, and lands. Show both new prices where the tool
  reads them back. Keep them for T6. Say what the tool told you about
  each write, what each receipt would let you undo, and how you would
  know another program had changed a price between your read and your
  write.
- **T6 — the shop.** "Give me my shop as forum pages I can paste."
  Produce them, say how many pages, where on them the two prices of
  T5 appear, what was left out and why, and what a person does with
  the output next. Then clean up: remove the item's price and put the
  tab back as you found it, and read both back to show it.
- **T7 — for a program.** Take five commands you have already run,
  one from each of T2, T3, T4, T5 and T6, and run them again asking
  for JSON. Is each answer one JSON document a program could read,
  including when the command fails? Make three commands fail on
  purpose — a wrong account name, an id that does not exist, a price
  write naming a revision that is not the row's — once as text and
  once as JSON, and say whether each failure told you what to do next.
- **T8 — when things go wrong.** Stop the daemon and say what each
  kind of command does then: one that only reads the store, one that
  only observes the daemon, one that needs the API. Find out what the
  tool says about its own version and its daemon's, and where its
  logs are. Leave the daemon stopped when you finish.

## The report

Written through the door's `report`, a part at a time as you finish
each task — never all at the end. Its first line: `<!-- written during
the run; call numbers are rows of calls.csv -->`. Then these headings
and no others. Every claim cites a command by the number the door
printed (`call 17`); a count, a size or a time is the one the door or
the tool printed, never one you remember. A fact this file gave you —
what the tool is, whose items are here, that the world has no keyring
— is this file's: cite it as `(the brief)`, never as the tool's or the
README's, and count it as nothing they told you. Under any heading,
`none` is an answer when it is true.

1. **Before touching anything** — T0, what this file told you, what
   the README told you and what the tool told you kept apart.
2. **The tasks** — for each of T1 to T8: what you would tell the
   person, in the words you would use; the commands it took, by
   number; whether you hold the result to be right, and what would
   tell you it is not; where the tool or the README stopped or misled
   you.
3. **Faults** — what the tool said or did, what it should have, the
   calls that show it. A refusal is a finding when it did not let you
   recover: say what it told you and whether that was enough to go on,
   and count one that was as no fault. Order them worst first, and say
   for each whether a person at a keyboard would hit it too or only an
   agent.
4. **What the README and the help did not say** — each thing you
   needed and had to find by trying: the fact, where you expected to
   read it (the README's section, or the verb's help), the call where
   you learned it. Be exact. And the other way: what the README said
   that you never needed.
5. **The tool's own words** — the README and the help texts as you
   read them, and the words of every answer, refusal and note the tool
   printed: what was right, what was wrong, missing or misleading, what
   you read and never used, and any word you did not understand.
6. **What you would ask of the tool** — one line each, with the call
   that made you want it.
7. **Left out** — what you did not try, every file you opened that is
   not one of the two, every file you wrote, and the one thing you cut
   that you would most want put back.

## Acceptance

A reader can say, for each of the nine tasks, what the person was
told, what it cost in commands, and where the tool stopped — from the
report and `calls.csv` alone.

Your last message: at most 300 words — which tasks were finished and
which were not, the fault you hold the worst, the three things the
README most needs to say or stop saying, and what "Left out" says.
