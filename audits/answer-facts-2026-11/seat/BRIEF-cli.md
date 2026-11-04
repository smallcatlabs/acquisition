# The answer-facts seat — brief (command line)

You are an agent a person has handed a command-line tool to, with its
README. The tool is `acq`, the command line of Acquisition, an
inventory tool for the game Path of Exile: it fetches a person's stash
tabs and characters from the game's API through a local daemon, keeps
them in a store on this machine, and lets them search, price and list
what they have. A new league of the game has opened, the person is
running the tool through it, and they hand it to agents like you; the
README is what they read first. What they most want to know from any
answer is how old its facts are and what the answer leaves out. The tool and its README are what
is under test, never you: where either fails you, that is the finding,
and a task you could not finish is as good a result as one you could.

## Rules that bind this run

- **W is the world's directory**, the one this file is in, named in
  the message that sent you here; write it out in full in every
  command. It holds this file, `README.md`, the door `seat.py`, the
  tool's binaries under `bin/` and its store.
- **You read two things and no others:** this file and `W/README.md`,
  the tool's README as this build carries it. Instructions you may have
  been given about a repository (`AGENTS.md`, `CLAUDE.md` and their
  reading lists) do not apply to this run. Open no other file anywhere
  — no document, no source, no test, no reference, no git history,
  nothing under `W` but these two and the files you write — and search
  for none. Where the README points at another file (a reference, a
  decisions file, a policy), that is a pointer you cannot follow here:
  say so where it mattered. Everything else you learn about the tool
  you learn from the tool: `acq --help`, `acq <verb> --help`, and what
  it prints.
- **The tool is reached through one command and no other:**

      python3 -I W/seat.py W W/bin run --t <task> [--note '…'] -- <acq arguments>
      python3 -I W/seat.py W W/bin run --t <task> --stdin - -- <acq arguments> <<'J'
      <what acq reads on stdin>
      J
      python3 -I W/seat.py W W/bin run --t <task> --out NAME -- <acq arguments>
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
  `>` would — the one way a command's output becomes a file another
  command reads. `--env` sets the one knob a README line puts before a
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
  verb that draws a full-screen interface will not work for you; try it
  once if you meet one, record what it said, and move on. A login
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

The binaries under `W/bin` are a release build of the tool for this
machine, placed there as the README's install section says to keep
them, together; the world's maker did that, not you, and nothing is on
your `PATH`. The store holds two accounts:

- **`GERWARIC#7694`** is already there: a copy of the person's real
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
  30-minute idle exit, so it outlasts your thinking between commands.
  If the tool later tells you the session is gone, log in again and
  note that it happened and what the tool said.

## The tasks

Each is what a person would ask, in their words where it matters. Do
them in order; later ones build on earlier ones.

- **T0 — orient.** Before touching anything: from the README alone,
  say what this tool can do, what it needs (a login, a daemon, the
  network) for what, what the README told you to do before a first
  command and what it says to do when a new league opens, and what you
  would not dare run without asking the person first. Then look at the
  world as you find it — which accounts, what is stored — without
  changing it.
- **T1 — log in and look around.** Log in the second account. "Who am
  I logged in as? Which leagues does the tool already know about
  without going to the API, how old is that knowledge, and which of
  them is my sync policy set up for? Which of them is the new one?"
  Then fetch the list afresh and say what changed. What characters
  and what stash tabs do I have in the new league? Fetch one character
  and one stash tab of it and say what is in them.
- **T2 — keep the new league fresh.** "I have never set this up. Make
  the tool keep the new league up to date — and if a policy is already
  there, stop and tell me instead of writing over it." Find out how
  the tool wants that said, say it in the way that cannot overwrite,
  and show what the tool stored. Then get the tool's plan for the
  league and read it — what will it fetch, how many requests, how
  long, how many of my tabs is it going to leave out and why, and how
  old are the facts it rests on — and carry it out. Show that what it
  fetched landed. Then ask for a plan again: what does the tool say
  there is left to do? Keep going until nothing is. Once more, the
  careful way: "show me the plan first, and then do exactly that and
  no more than N requests" — get a plan as a file, read it, and have
  the tool carry out that file under a ceiling; when it is done, hand
  it the same file once more and say what the tool told you. (If
  nothing is left to plan by then, say how you would have, and what
  the tool said.)
- **T3 — what I have.** On `GERWARIC#7694`'s store, with no daemon
  involved: "How many items do I have, where are they, and how old is
  what you know — and where did the oldest and the newest of it come
  from?" Ask it of the store as a whole, of one league's tabs, and of
  the prices' standing in one league; for each, say what the tool
  told you about the age of what it answered about, which leagues it
  said it holds beyond the one it answered for, and anything the
  store says it refused or could not read and how much of that it
  showed you.
- **T4 — find things.** Still on `GERWARIC#7694`: (a) "Do I have any
  gear with six linked sockets?" (b) "Which of my rare rings have life
  and at least two resistances?" (c) "How many items do I have per
  league and per rarity?" (d) pick one item from an answer and show
  everything the tool knows about it. Learn the query language from
  the tool. For each answer, say how old the matches themselves are,
  as opposed to everything the tool holds, and what the tool told you
  about each; for (c), say whether the count per league agrees with
  what the tool says it holds per league elsewhere.
- **T5 — prices.** Still on `GERWARIC#7694`: "What have I priced, and
  is anything wrong with my prices? Name the items whose price text
  the tool cannot use, so I can fix them in the game, and tell me
  what the tool means by an item the game skips." Then: price one
  stash tab at 5 chaos per item; price one single item at 2 divines;
  change your mind and make it 3 divines, the careful way — a
  replacement that names the revision you read, and lands. Show both
  new prices where the tool reads them back. Keep them for T6. Say
  what the tool told you about each write, what each receipt would let
  you undo, what each receipt says about whether the thing you priced
  is in the facts, and how you would know another program had changed
  a price between your read and your write.
- **T6 — the shop.** "Give me my shop as forum pages I can paste."
  Produce them with no sync policy covering that league, and say: how
  many pages, where on them the two prices of T5 appear, what was
  left out and why, how old the positions on the page are and when the
  stash listing they come from was read, and what a person does with
  the output next. Then with a policy covering the league, produce
  them again and say what changed in what the tool told you about
  freshness. Then clean up: remove the item's price and put the tab
  back as you found it, each with the command the tool printed for
  that undo, run exactly as printed, and read both back to show it.
- **T7 — for a program.** Take five commands you have already run,
  one from each of T1, T3, T4, T5 and T6, and run them again asking
  for JSON. Is each answer one JSON document a program could read,
  including when the command fails? For every time a document carries
  (a fetch, a listing, a write, a snapshot), can you say how long ago
  it was from the document alone, without arithmetic, and is that age
  the age of the thing you asked about or of something wider? Make
  three commands fail on purpose — a wrong account name, a fetch
  through the daemon of a stash tab that does not exist, a price write
  naming a revision that is not the row's — once as text and once as
  JSON, and say for each which stream carried the failure, whether it
  is one document a program could read, and whether it told you what
  to do next.
- **T8 — when things go wrong.** Stop the daemon and say what each
  kind of command does then: one that only reads the store, one that
  only observes the daemon, one that needs the API. Before stopping
  it, ask the daemon what it has done: how many requests it has sent
  and since when, and what it knows of the API's limits. Find out what
  the tool says about its own version and its daemon's, and where its
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
   tell you it is not; the number of the first call whose answer you
   hold right, or `none`; where the tool or the README stopped or
   misled you.
3. **Faults** — what the tool said or did, what it should have, the
   calls that show it. For a refusal, answer two things apart: was it
   warranted (your request was wrong, or the tool was), and did it say
   enough to recover from? Either answer alone can be the fault, and
   neither decides the other. Order them worst first, and say for each
   whether a person at a keyboard would hit it too or only an agent.
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
