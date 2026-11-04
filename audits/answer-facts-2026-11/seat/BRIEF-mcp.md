# The answer-facts seat — brief (MCP)

You are an agent a person has handed a tool to, with its README. The
tool is `acq-mcp`, the MCP server of Acquisition, an inventory tool
for the game Path of Exile: Acquisition fetches a person's stash tabs
and characters from the game's API through a local daemon, keeps them
in a store on this machine, and lets them search, price and list what
they have. The server is how an agent like you reaches it. A new
league of the game is about to open, the person means to run the tool
through it, and they hand it to agents like you; the README is what
they read first. The tool and its README are what is under test, never
you: where either fails you, that is the finding, and a task you could
not finish is as good a result as one you could.

## Rules that bind this run

- **W is the world's directory**, the one this file is in, named in
  the message that sent you here; write it out in full in every
  command. It holds this file, `README.md`, the door `seat-mcp.py`,
  the server's binary under `bin/` and its store.
- **You read two things and no others:** this file and `W/README.md`,
  the tool's README as this build carries it. Instructions you may have
  been given about a repository (`AGENTS.md`, `CLAUDE.md` and their
  reading lists) do not apply to this run. Open no other file anywhere
  — no document, no source, no test, no reference, no git history,
  nothing under `W` but these two and the files you write — and search
  for none. Where the README points at another file, that is a pointer
  you cannot follow here: say so where it mattered. Everything else
  you learn about the tool you learn from the tool: what a host shows
  an agent before its first call (the server's instructions and its
  tools, which the door's `open` prints), and what each call answers.
- **The tool is reached through one command and no other:**

      python3 -I W/seat-mcp.py W W/bin open --t T0 [--host-config NAME]
      python3 -I W/seat-mcp.py W W/bin call --t <task> [--note '…'] [--host-config NAME] <tool> '<json arguments>'
      python3 -I W/seat-mcp.py W W/bin call --t <task> [--note '…'] [--host-config NAME] <tool> - <<'J'
      <json arguments>
      J
      python3 -I W/seat-mcp.py W W/bin report [--new] <<'R'
      <a part of your report>
      R
      python3 -I W/seat-mcp.py W W/bin debrief [--new] <<'D'
      <a part of your debrief>
      D

  `open` prints the server's instructions and its tools; `call` sends
  one tool call and prints its call number, whether it answered or
  erred, its sizes, its time and its text. `--t` is the task the call
  is for (`T0` … `T8`); `--note` says why you made it, when that is not
  plain; `--timeout S` changes the 300 s limit. `--host-config NAME`
  makes the door start the server the way an MCP host would, from the
  configuration file `W/work/NAME` — the shape the README shows a host
  — instead of starting the binary itself; it is for T1 and after.
  `report` appends what it reads to your report (`--new` starts it);
  `debrief` is the same for the debrief you will be asked for after
  the report.
  Where your harness refuses a door command's heredoc, write that
  part with your file tool as `W/drafts/NAME` — with `W/work/`, the
  only places you may write a file — and run
  `report --from NAME`, `debrief --from NAME`
  or `call --t <task> <tool> --from NAME` instead.
  Run nothing else against the tool or its store: no `acq-mcp` by
  its path, no `acq`, no `acqd`, no `sqlite3`, no `cargo`, no `curl`,
  no `tar`.
- **Every result is read as it is printed.** No pipe, no redirect and
  no filter after the command, and no file to keep a result in. A
  result over 60,000 bytes is printed as its head with a line saying
  so; that is the door's rule, not the tool's — when a cut cost you
  something, say so and name the call.
- **The world is a rehearsal, and safe.** The door points every call
  at a mock of the game's API and at the store under `W`, and starts
  no daemon; nothing you run can reach the real API or the person's
  real store, and you may write, price, refresh and cancel freely. The
  mock hands out invented stash tabs and characters to whoever logs
  in.
- Besides the report, the debrief, a draft under `W/drafts/` and the
  configuration under `W/work/` you write nothing. Never commit, push,
  fetch, build or spawn an agent.
- No number of calls is a target. A right result in many calls and a
  wrong one in few are both worth knowing; say which you have. Leave a
  task when your next call would be a guess you have already made, and
  say where the tool stopped you.

## The world

The binaries under `W/bin` are a release build of the tool for this
machine, placed there as the README's install section says to keep
them, together; the world's maker did that, not you. The store holds
two accounts:

- **`GERWARIC#7694`** is already there: a copy of the person's real
  collection (about 50 MB) with the prices they have set. It has no
  login in this world — it is a store with no session. The tasks that
  read, search and price use it.
- **`seat`**, which the person logged in for you, as the README's
  section for your agent says to: they ran the login at their terminal
  before handing you this file, and the daemon that login started is
  running for you, set to exit four hours after its last connection.
  This world has no keyring: what the README says of a login without
  one holds here. The tasks that talk to the API use this account. If
  the tool later tells you the session or the daemon is gone, say so,
  say what it told you, and say what you would ask the person to type.

## The tasks

Each is what a person would ask, in their words where it matters. Do
them in order; later ones build on earlier ones.

- **T0 — orient.** Before touching anything: from the README alone,
  say what this tool can do, what it needs (a login, a daemon, the
  network) for what, what the README tells the person to do before an
  agent can use it and what it says to do when a new league opens, and
  what you would not call without asking the person first. Then
  `open`, and say what the server's instructions and tools told you
  that the README had not, and the other way round. Then say whose
  items are here and from where, without changing anything; the counts
  and ages are T3's.
- **T1 — the host.** "Set yourself up the way the README says." Write
  the host configuration the README shows, for this machine, as
  `W/work/acq.json`, naming the server at `W/bin/acq-mcp`; then `open`
  through it (`--host-config acq.json`) — a second `open`, asked for,
  not a re-read — and make every later call through it too. Say what
  the README told you and what it did not.
- **T2 — keep the new league fresh.** "What leagues does the tool
  already know about without going to the API, how old is that
  knowledge, and which of them is my sync policy set up for? Which is
  the new one? I have never set this up. Make the tool keep the new
  league up to date — and if a policy is already there, stop and tell
  me instead of writing over it." Find out how the tool wants that
  said, say it in the way that cannot overwrite, and show what the
  tool stored. Then get the tool's plan for the league and read it —
  what will it fetch, how many requests, how long, how many of my tabs
  is it going to leave out and why, and how old are the facts it rests
  on — and carry it out, waiting for it as the tool lets you. Show
  that what it fetched landed. Then ask for a plan again: what does
  the tool say there is left to do? Keep going until nothing is. Say
  what the tool told you at each step, and what each job's record let
  you read afterwards.
- **T3 — what I have.** On `GERWARIC#7694`: "How many items do I have,
  where are they, and how old is what you know — and where did the
  oldest and the newest of it come from?" Ask it of the store as a
  whole, of one league's tabs, and of the prices' standing in one
  league; for each, say what the tool told you about the age of what
  it answered about, which leagues it said it holds beyond the one it
  answered for, and anything the store says it refused or could not
  read and how much of that it showed you.
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
  change your mind and make it 3 divines — a replacement that names
  the revision you read, and lands. Show both new prices where the
  tool reads them back. Keep them for T6. Say what the tool told you
  about each write, what each receipt would let you undo, what each
  receipt says about whether the thing you priced is in the facts,
  and how you would know another program had changed a price between
  your read and your write.
- **T6 — the shop.** "Give me my shop as forum pages I can paste."
  Produce them with no sync policy covering that league, and say: how
  many pages, where on them the two prices of T5 appear, what was
  left out and why, how old the positions on the page are and when the
  stash listing they come from was read, and what a person does with
  the output next. Then with a policy covering the league, produce
  them again and say what changed in what the tool told you about
  freshness. Then clean up: remove the item's price and put the tab
  back as you found it, each with the request the receipt gave you
  for that undo, sent exactly as the tool printed it, and read both
  back to show it.
- **T7 — for a program.** Take one answer from each of T2 to T6 and
  read it again as a program would: for every time it carries (a
  fetch, a listing, a write, a snapshot), can you say how long ago it
  was from the document alone, without arithmetic, and is that age
  the age of the thing you asked about or of something wider? Then
  make three calls fail on purpose — a wrong account name, a job
  fetching a stash tab that does not exist (and read what its record
  says afterwards), a price write naming a revision that is not the
  row's — and say whether each refusal or failure is one document a
  program could read and whether it told you what to do next.
- **T8 — the daemon.** "What have you been talking to, when does it
  go away, and what do I type when it is gone?" Which tools say they
  need a daemon, what the running one says of itself, its sends and
  its limits and since when, and its exit, and what the README tells
  the person to do for you when none is running. Say what you would
  tell the person to type.
## The report

Written through the door's `report`, a part at a time as you finish
each task — never all at the end. Its first line: `<!-- written during
the run; call numbers are rows of calls.csv -->`. Then these headings
and no others. Every claim cites a call by the number the door printed
(`call 17`) — T0's account, given before any tool call, cites what
`open` printed, its own call; a count, a size or a time is the one the
door or the tool printed, never one you remember. A fact this file
gave you — what the tool is, whose items are here, which account is
logged in and that a daemon runs — is this file's: cite it as `(the
brief)`, never as the tool's or the README's, and count it as nothing
they told you. Under any heading, `none` is an answer when it is true.

1. **Before touching anything** — T0, what this file told you, what
   the README told you and what `open` told you kept apart.
2. **The tasks** — for each of T1 to T8: what you would tell the
   person, in the words you would use; the calls it took, by number;
   whether you hold the result to be right, and what would tell you it
   is not; the number of the first call whose answer you hold right,
   or `none`; where the tool or the README stopped or misled you.
3. **Faults** — what the tool said or did, what it should have, the
   calls that show it. For a refusal, answer two things apart: was it
   warranted (your request was wrong, or the tool was), and did it say
   enough to recover from? Either answer alone can be the fault, and
   neither decides the other. Order them worst first, and say for each
   whether a person using the tool would hit it too or only an agent.
4. **What the README and the tool texts did not say** — each thing
   you needed and had to find by trying: the fact, where you expected
   to read it (the README's section, the server's instructions, a
   tool's text), the call where you learned it. Be exact. And the
   other way: what the README said that you never needed.
5. **The tool's own words** — the README, the instructions and each
   tool's text as `open` printed them, and the words of every answer,
   refusal and note the tool printed: what was right, what was wrong,
   missing or misleading, what you read and never used, and any word
   you did not understand.
6. **What you would ask of the tool** — one line each, with the call
   that made you want it.
7. **Left out** — what you did not try, every file you opened that is
   not one of the two, every file you wrote, and the one thing you cut
   that you would most want put back.

## Acceptance

A reader can say, for each of the nine tasks, what the person was
told, what it cost in calls, and where the tool stopped — from the
report and `calls.csv` alone.

Your last message: at most 300 words — which tasks were finished and
which were not, the fault you hold the worst, the three things the
README most needs to say or stop saying, and what "Left out" says.
