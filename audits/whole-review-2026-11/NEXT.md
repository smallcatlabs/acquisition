# After the review: what to build next, and how

Written by Fable on 2026-11-25, at `3a47098a`, after reading
`PORTFOLIO.md` beside this file and then the tree. The owner asked for
a set of changes that Fable can orchestrate and other models can
design, plan, build and check, chosen to make the project easier for
the agents who build it and who use it. Nothing was off the table.

The first reader is the session that runs this, with none of today in
mind. The owner is the second. Paths are from the repository's root,
except those that begin `next/`, `findings/`, `trials/` or `issues/`,
which are beside this file. House nouns (a door, a seat, a way on) are
in `GLOSSARY.md`. A number is mine where it stands beside its command,
and an investigator's otherwise.

It was made in four passes. I read the review and the tree, and
counted what agents here have actually done (`next/traces.md`). I sent
five Opus readers each a hypothesis to break (`next/reports/`). I
drafted. Then Codex tried to break the draft (`next/dissent.md`) and a
session holding only the draft said where it could not be acted on.
What each pass changed is the last section.

## The idea

This project gave GGG one choke point, a journal, and an oracle that is
not the code. Every send passes one place. Every send is written down.
The tests compare the journal with tables written from numbered claims
about GGG, never from the code's own output. That side is the tool's
best work.

The tool's readers got none of the three. A reader is an agent at the
CLI or the MCP, a person at a page, or the agent building the thing.

- **No one place where an answer gets its ways on.** A refusal carries
  its offers in the document, and a receipt its undo. An answer does
  not: 152 lines spell `acq <verb> …` by hand
  (`git grep -n '"[^"]*acq [a-z]' -- crates | grep /src/ | grep -v /tests/ | wc -l`),
  so the JSON and the MCP never get those remedies.
- **No journal.** Nothing counts what a reader had to do. The
  plainest question took six tries (`findings/ground-truth.md`), and
  the MCP's refresh has cost 18 to 23 calls through three rounds of
  fixes to its words (`findings/first-answer.md`).
- **No oracle.** Each frontend test builds its own small world, and
  none is shaped like the owner's: three accounts, two realms, a
  challenge league.

The portfolio's pages 2, 4 and 7 are this one absence seen from three
sides. The builders have the same gap. An agent here reads as much
governance as source, and runs the tool it is building in 1.4% of its
shell calls, because a populated world costs a daemon, a login and
three applies.

So: build the reader's side to the standard of the sender's.
Instruments first, since they make everything after them checkable.
Then the shapes that remove what review keeps rediscovering. Then the
frontends' code review and the GUI's next parts, which the road
already holds, over a tree that is smaller and can be measured.

## The steps at a glance

| Step | What | Brief from | Done when | Who | Waits on |
| --- | --- | --- | --- | --- | --- |
| A1 | One seeded world, one command | `next/reports/world.md` | two duplicated worlds read it; a command prints a populated world | Opus, then Sonnet | the order |
| A2 | Tasks counted through both doors | `next/reports/world.md` §3 | one golden of answers and counts, today's numbers in it | Opus | A1 |
| A3 | The gate's fault fixed, its search kept as a test | `trials/admission-explorer.patch`, `findings/admission.md` | the search fails before and passes after; #206 closes | Opus | the order |
| A4 | The README, the drafted issues, the reading rule | `issues/`, `next/traces.md` | each is its own small commit | Sonnet | his word, for two of three |
| B1 | One rule for an unnamed realm or league | `next/reports/scope.md` | 36 default sites become 3 | Fable, Opus, Sonnet | his ruling |
| B2 | An answer carries its ways on | `next/reports/words.md` §3, §6 | 152 hand-spelled lines fall, verb by verb | Fable, Opus | A2 |
| B3 | The shared words out of the CLI's binary | `next/reports/words.md` §1, §2 | 93 imports become a handful; no golden changes | Opus, then Sonnet | A2 |
| B4 | Pages held by what they do | `next/reports/pages.md` | 795 KB of markup goldens under a tenth | Opus, then Sonnet | A1 |
| B5 | Two doors a frontend can still open | `next/reports/law.md` | each is a compile error or a refused request | Opus | the order |
| B6 | A rename keeps its history | `findings/persistence.md`, `issues/` | a renamed account reads its old facts, in a rehearsal | Fable, Opus | his decision |

## The steps

### A. Instruments

**A1. The world.** One function writes a seeded world through the
store's own ingest: an account holding two realms and three leagues, a
never-fetched tab, two prices, a saved search, a policy naming a league
the store does not hold; a second account with a handful of items; a
third that only logged in; a tab named and an item noted with hostile
markup (for B4). A variation is a row of it, not a new function, and
one variation is the world with a single account, since tests that
mean one account must keep meaning it. One command writes it to disk,
and every read verb answers over it with `ACQ_NO_SPAWN=1` and no
daemon.

- Why now: the frontends' tests, a seat, A2, the pages and a builder
  checking a change by running it all need the same thing. The road's
  t6 (#142) was waiting for a third consumer.
- Home, by the investigator's reading: a file beside
  `crates/acquisition-client/tests/harness/journey.rs`, included by
  path as that one is, and an example binary in the verbs crate. No
  new dependency edge. `crates/acquisition-cli/tests/world.rs` exists
  and is another thing, so the file takes another name.
- Done when: the two worlds the MCP's tests copy from the CLI's
  (`pricing_tools.rs:34` from `price_json.rs`, `search_tools.rs:77`
  from `search_json.rs`) are the one world; one command prints a
  populated world's path; the mock-session skill gives that command
  for read work.
  `git grep -c 'record_login(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`
  sums to 44 today and falls with each conversion after.
- Opus builds the world and converts the first seed of each of the
  four base worlds; Sonnet converts the rest under that rule. There
  are 23 seed functions and about eleven variations.
- Not in it: the mock serving this world. The mock gives every user
  the same stash and a login overwrites a seeded uuid, so the seeded
  world is for reads, and applies keep the mock's own.

**A2. The counted tasks.** A table of things a player wants, each
scripted as a reader holding only the help would first try it, through
the CLI and the MCP: what do I hold and how fresh; which items have 90
life; one named tab; is this priced; one item whole; refresh to
nothing to do. Each row holds the answer it must reach, the calls and
refusals it took, and a bound on the bytes read. Polls are counted
apart, since they are wall-clock.

- Why now: it is the reader's journal. It gives "an agent's ease" a
  diff a builder can read, and gives "smaller" a done-when, which is
  the review's own account of why improvements did not land.
- It pins today's numbers first, bad ones included. A later step's
  claim is then this table's diff.
- What it is not. A script measures the script: it says a path got
  shorter, never that new words teach a blind reader. The seat stays
  the evidence for that, sat in A1's world by the smallest model that
  can do the tasks, on a session's judgment. And a row with fewer
  calls and a wrong answer is a failure, which is why the answer is
  in the row.
- Done when: one golden in the manner of the journey's
  (`crates/acquisition-cli/tests/journey.rs`, its facts from
  `crates/acquisition-client/tests/harness/journey.rs`); the seeded
  tasks run over A1's world and the refresh task over the mock's; the
  MCP's test driver gains the call and byte counter it lacks
  (`crates/acquisition-mcp/tests/harness/mod.rs:258`).
- Opus. The rows need judgment: a row that scripts the right call
  first measures nothing.

**A3. The gate's fault, and its search.** The portfolio's page 1, the
gate's half: derive "this policy is busy" from the live permits and
delete the second record; turn the trial's search, which prints what
it finds and passes, into a test that fails.

- Done when: the search fails on the tree before the fix and passes
  after, at depth 6, in the gate;
  `grep -c 'debug_assert' crates/acquisition-daemon/src/gate.rs` falls
  from 4 to 1; #206 closes. The claim is the gate's alone: the search
  never drives the choke point or the rails.
- The limiter's half (a counted send whose answer names another
  policy) is a second step that waits on the owner, since a test pins
  today's behaviour as intended.
- Opus, in the second lane, since it touches only the daemon.

**A4. Three small things.**

- The README stops saying there is no UI. Now.
- The 33 issues drafted under `issues/` are filed in one pass, on the
  owner's word, each one a step here absorbs saying so.
- `AGENTS.md` tells an agent to read its area's rulings file whole and
  also to run `tools/bothy.py pack`. The pack prints the entries a
  file cites whole and the rest as headlines. Reading the pack and
  opening an entry when its headline bears on the change would cut
  the reading before a first edit by a third to a half. It is a real
  trade, since a headline can hide a qualification, so it is the
  owner's to decide, not a tidy-up.

### B. Shapes

**B1. One rule for an unnamed realm or league.** Outside `search`,
`tabs`, `price`, `shop render`, `refresh` and the pages read a fixed
`pc` and a fixed `Standard` when none is named. A player in a
challenge league alone gets an empty Standard from `acq price`. The
proposed rule: named, else the only one held, else a refusal that
lists what is held.

- Why now: a quarter of the serious review findings in every period
  name account, realm or league. The account is already resolved in
  one place (`crates/acquisition-verbs/src/frontend.rs:279`). This is
  the part still open, and it should land before the frontends are
  reviewed so the review is not spent finding it again.
- It needs a ruling. I first read it as enforcing C58 and C96; the
  skeptic is right that neither says this. And one input has no
  answer yet: an account that has just logged in, holds nothing, and
  has a policy naming a league. "Held" must then mean something more
  than the store's facts, and which league a refresh compiles decides
  what is sent.
- So: a design note by Fable first, on what "held" is for a read and
  for a refresh; the owner rules on the note; Opus builds one resolver
  beside `league_of` (`crates/acquisition-plan/src/pricing.rs:320`)
  and its first two callers; Sonnet does the remaining sites.
- Done when:
  `git grep -n 'Realm::DEFAULT\|DEFAULT_LEAGUE' -- crates | grep /src/ | grep -v /tests/ | wc -l`
  goes from 36 to 3 (two at the decode boundary, one another
  concept); the five `default_value = "pc"` lines in `cli.rs` go; A1's
  world with only a challenge league answers that league. A2's table
  says what moved, and where a new refusal made a task longer it says
  that too.
- Not in it: whether a read may span every realm and league (the
  portfolio's question 2), and the daemon's own four `Standard`
  literals, which need the protocol to require a league.

**B2. An answer carries its ways on.** As a refusal carries offers.
The text reads them from the document; `--json` and the MCP get them
because they are there.

- Why now: it is the third instance of a shape the tool has twice. It
  is where the MCP gains remedies it has never had, and where the two
  hand-written copies of a fault's remedy
  (`crates/acquisition-cli/src/terminal.rs:78-150`,
  `crates/acquisition-mcp/src/told.rs:35-110`) become one.
- It is not mechanical. Of 15 hand-spelled remedies sampled, 2 fit a
  `Way` as it stands. The design note, by Fable with Codex as its
  skeptic, must first tell three things apart: a request a reader can
  send as printed; an instruction with a hole the reader fills
  (`price set item/<id> exact <amount> <tag>`); and an act only a
  person may take. Today a way a door cannot send is silently
  dropped, so adding variants without that distinction loses words.
- First slice, which pays alone and waits on nothing:
  `acq --json price status --league Nowhere` and the MCP's
  `price_status` carry the refresh plan as an offer.
- Done when: the 152 lines fall verb by verb; a `Surface` used only
  in tests records every way a text spells, and each sendable one is
  accepted by its verb or tool.
- I read additive `offers` as agent-facing design (C170). The owner
  should say if a document's keys are his.

**B3. The shared words out of the CLI's binary.** The pages take 93
symbols from the CLI's command modules
(`git grep -ohE '(search_cmd|plan_cmd|price_cmd|store_cmd|jobs_cmd|shop_cmd|saved_cmd|daemon_cmd|leagues_cmd|terminal|printed|run)::[A-Za-z_]+' -- crates/acquisition-cli/src/web | sort -u | wc -l`),
80 of them command-free fragments over the search's own types. Those
move into the search crate with `now` passed in. Then `plural` and
`ago`, and the pure renderers of `daemon`, `leagues` and `saved`, to
the verbs crate or lower as the edges allow.

- Why now: 46% of product commits touch the CLI crate, because it is
  the terminal's adapter, the home of every door's words, and the GUI
  at once. Whatever the GUI becomes will need the words somewhere it
  can link.
- Done when: 93 falls to the few that are not words (`Ctx`,
  `connect`, the browser opener); no golden changes
  (`git diff --quiet <before> -- '*/goldens/*'`).
- Not in it: `shop`, whose renderer builds a way on and so waits for
  B2 (the plan crate cannot link the verbs); and `store_cmd`, whose
  words query the store and need fields the document lacks.
- One thing a move does change: the search build's digest covers the
  search, plan and store sources, so a held continuation is asked
  again from its start, as after any commit there.
- Opus moves the first group and states the rule; Sonnet moves the
  rest, checked by the compiler and the goldens.

**B4. Pages held by what they do.** A page's golden becomes a
projection: its status, each form and each button with the request it
would send, each link with where it goes, and its facts as text.

- Why now: an in-place redesign would rewrite the markup while a
  projection stays still. In six weeks the 795 KB of markup goldens
  caught no page fault the gate could act on; 11 of 40 sampled golden
  commits were churn; 14 of 25 review findings about pages are things
  a projection shows.
- First, and worth doing alone: nothing tests that a page escapes a
  hostile tab name or note. `page.rs` builds markup at 601 sites with
  302 hand-placed `esc(` calls, and the markup goldens hold the
  property by the accident of one apostrophe. With A1's hostile row,
  the harness's recording seam (`Pages::page`,
  `crates/acquisition-cli/tests/harness/mod.rs`) refuses any recorded
  page that holds it raw.
- Then the projector. `harness/submitted.rs` reads a form's default
  fields and skips its buttons, so the projector must add each
  submitter's own request. `web/search_form.txt` first: 64 KB, no
  named fact.
- Then one golden at a time. Of 445 `# holds` lines, 156 name a class,
  an attribute or a tag; each becomes a fact, a projection line, a
  test of its own, or goes. Opus writes that table for a golden;
  Sonnet regenerates under it.
- Done when:
  `cat crates/acquisition-cli/tests/goldens/web/*.txt | wc -c` falls
  from 795,121 to under a tenth; the query box's open state and the
  rarity class, which a projection loses, have a test of their own.

**B5. Two doors a frontend can still open.** The census in
`next/reports/law.md` asked of nine values "can a frontend make or
change this without touching the module that defines it?" and found
five where only review would notice. Two are small:

- `Account`'s fields become private, so `Frontend::account` is the
  only way to hold one.
- Every page route sits under one router at the token, so a new route
  cannot forget it.

Not in it, each needing a design of its own: the frontends holding a
reader of facts and not the writer, which is #212 and must say what
`acq store rebuild` becomes; a rule that a GET route cannot write,
which no router can infer and which stays a test over the route
table; a plan's fields made private. The greps in
`tools/arch-check.sh` stay, since `Store::open` is public whatever an
`Account` is.

**B6. A rename keeps its history.** Facts are filed by username. On a
rename at GGG, `record_login` follows the new name and leaves the old
facts file behind (`crates/acquisition-store/src/index.rs:300`). A
refetch brings back current state, never `first_seen`, the item
events or the removed items. The skeptic named this as the plan's
largest omission and I agree: it is the one fault here that is
irreversible, and it is cheapest to fix before a release puts
username-named files on other people's disks.

- It waits on the owner's decision that the history is kept state
  (the portfolio's question 5). My position: it is.
- Then a design note by Fable on the transition, holding the states
  the review's skeptic listed (a first profile job before any uuid, an
  old index entry without one, the keyring's old names until logout
  has cleaned them); a rehearsal of a rename and a restart in a mock
  world; the build by Opus.
- With it, the one inference the review left untested: that a job
  queued under an old username can be sent twice
  (`issues/a-rename-may-double-send.md`).

## Held, and why

- **The MCP answering in text.** My idea, dropped: C53 rules the JSON
  is the text, a client passes on one form only, and an agent would
  lose request objects, ids and continuation tokens.
- **The web as its own crate.** B3 makes it possible. It needs C139
  and C1 amended, and nothing here needs it.
- **The store linking the protocol crate** (the portfolio's page 9).
  Sound; waits on its question 7.
- **The registry's entries moved to where they bind.** Question 6,
  and the road already schedules a re-measurement (#220).
- **The value expression** (#316 with #46). The road's own item, and a
  design, so it can proceed in the lane that builds nothing.
- **4-full parts 2 and 3.** Their design waits on the portfolio's
  question 3 whatever happens here. Part 2 should take A2's "refresh
  to nothing to do" row as its done-when: 5 calls by the CLI, 23 by
  the MCP.

## How it is run

**Lanes.** One lane builds in the main tree. A second builds in a
copy-on-write clone (`next/lane-trial.md`, `next/scripts/lane.sh`: a
minute and about 2.5 GB, removed at its close; the first use confirms
what the trial left untried). A third builds nothing: design notes,
documents, reviews. Two rules keep the lanes apart: their crates do
not overlap, and only one lane at a time changes goldens.

| | Lane 1 | Lane 2 | Lane 3 |
| --- | --- | --- | --- |
| First | A1 | A3 | A4; the notes for B1, B2, B6 |
| Then | A2 | B5 | the owner's rulings on the notes |
| Then | B3 | B1, then B6, as each is ruled | |
| Then | B2, then B4 | | |

**Checks.** C175 and C176 stand as written, and each step is one
step under them: reviewed cold once as a range through its close,
again only when a round found something that changes what is built.
A1, A2 and B4 change the shared test harness and are reviewed for
that. Beside the review, each step owes the evidence its kind allows:

- a step that moves code: no golden changes, by command;
- a step that changes what a reader meets: its golden's diff read
  line by line, and A2's table saying what moved;
- a step in the law list: a table, or a bounded search where the
  machine is small.

**The law list.** The portfolio's page 10 ended without a rule for
which code is law. I have no rule either. I have a list and a test.
The list, from the census: the choke point, the limiter, the policy
parse, the gate, the rails, the transport, the token code, the
daemon's job kinds and its submit checks; the planner's envelope and
its spendable check; pricing's target resolution; the verbs' account
and refresh; the store's ingest, intent, saved-search and index
files; the pages' router and guard. About sixteen modules, 20k of
120k lines. The test is B5's question, put to any value a property
rides on. Where its answer is no, the list is the whole truth; where
it is yes, the list is a hope. Three things no list holds: what each
new page route must remember, the wording rules, and a plan as the
reviewed intent of a person, which is a promise and not a capability.

**Models.**

| Who | Does |
| --- | --- |
| Fable | Orchestrates. Writes the design notes (B1, B2, B6). Reads each golden diff of a step that changes what a reader meets. Reads for the pattern when a second review round finds faults from one cause |
| Opus | Builds what needs judgment, and the first instance of every rule a cheaper model repeats. Reads as today's five did: handed a hypothesis to break |
| Sonnet | Repeats a rule Opus has shown once, where a command says done: seed conversions, moved fragments, default sites, a golden regenerated under its table, filed issues, counts |
| The smallest model that can do the tasks | Sits blind at the tool in A1's world |
| Codex | Cold review (C177, the cold-review skill). The skeptic of each design note. A second builder where a design has two defensible shapes, the two compared by A2's table |

Three quarters of the turns here have been Opus's and 0.4% Sonnet's,
so Sonnet's fitness for these moves is a bet. Its bound: a mechanical
step that fails the gate twice goes to Opus.

**Briefs.** As C174 has them, with two habits. The done-when is a
command and its number. A mechanical brief names the commit in which
Opus did the first instance and says to do the same to the rest.

**Sessions.** A fresh session for each step, from its brief; a
session that designed does not build. Builds are sessions, reads are
subagents. The harness's traps are in the orchestrator's memory
(`orchestrating-steps`).

**When to stop.** A shape that lands and does not move A2's table is
reported as that. A design note its skeptic breaks is narrowed or
dropped. The programme does not grow: a new finding is an issue.

**What it adds, and what each addition lets go** (C182). Four things:

- the world: the 23 seed functions and the logins written inline;
- the projector: 795 KB of markup goldens;
- the gate's search: three `debug_assert!`s and #206;
- the counted tasks: nothing, at first. They earn their place if a
  step's claim is settled by their diff where today it takes a seat
  and a fix round. If, when B is done, they have moved for no step
  and caught nothing, they are retired.

Also retired as each lands (C154's form): the two `fault_words`, the
default constants, the pages' imports from the CLI's command modules.

## What is the owner's

Seven, each one decision. Only the first holds everything.

1. **The order.** This goes ahead of the build of 4-full parts 2 and
   3 and of #316; their designs may proceed beside it. The road's
   "Next" is his. If he would rather the GUI moved first: A1, A2 and
   A3 pay alone in about a week, and B can follow part 2.
2. **B1's rule**, when its note is written.
3. **The history is kept state** (the portfolio's question 5). B6
   waits on it.
4. **The limiter's missing hit** (question 1). My position: count
   it. GGG counted the send whatever policy its answer named.
5. **The reading rule in `AGENTS.md`** (A4).
6. **Filing the 33 drafted issues**, which the review left for his
   word.
7. **A prototype to sit at** (question 3). Not needed here. Once A1
   exists the second lane can build one over a copy of his store,
   and parts 2 and 3 are better designed after he has typed in it.

Not asked now: the portfolio's questions 2, 6 and 7, which block no
step, and 4, which B2 and B3 answer in a smaller form.

## What each pass changed

| I thought | What it met | Now |
| --- | --- | --- |
| One `Scope` value, made once; reads span everything held | The account is already one place; resolution happens inside the read; spanning is against C96 (`next/reports/scope.md`). Then the skeptic: even "the only one held" needs a ruling | B1, behind a note and a ruling |
| Move the words below the frontends; every remedy a `Way`; the MCP answers in text | A remedy the JSON is to carry sits in the document; the fragments move, the remedies are design; the MCP in text is against C53 (`next/reports/words.md`) | B2 and B3; the MCP idea dropped |
| One seeded world for everything | For reads only; 23 seeds, not 34 (`next/reports/world.md`) | A1 |
| Page goldens should hold structure some day | Sooner, and they are the only pin on escaping (`next/reports/pages.md`) | B4 |
| Law is the modules that can construct what the invariants ride on, a short list | A sixth of the code, and false for five values (`next/reports/law.md`) | The list and the test above; B5 |
| A worktree costs a cold cache, so one lane | A clone is warm in a minute (`next/lane-trial.md`) | Two lanes |

The skeptic (`next/dissent.md`) broke B1's claim to need no ruling,
and weakened seven of the other eight steps on points I checked and
took: a task count needs its answer beside it; the single-account
tests must stay single; the classes I had drawn promised less review
than C175 allows, and are now evidence owed beside the review; the
shop's renderer cannot move before B2; a projector must read buttons;
a router cannot know a handler writes; the plan named what it retired
and not what it added. It named the rename as the larger omission,
which is now B6.

The cold reader reproduced every count it tried and found three
citations wrong and four terms undefined; they are corrected.

## What I did not check

- I ran no test and built nothing in the main tree. A step's size is
  an estimate; I would guess twenty build sessions and have timed
  none.
- The transcript counts are heuristics over one machine's sessions.
- If A2's refresh row cannot be made honest over the mock's own
  world, A1 needs the mock to serve a table per user, which is larger
  and touches the daemon.
- B2 could be as large as the portfolio's page 4. If its note cannot
  keep a way with a hole simple, the first slice and the faults still
  land and the rest becomes a question.
- The revised plan has had no second skeptic.

## Starting

1. The owner's word on the order; at the workers' start, his grant of
   `gh` writes and pushes in each worker session, as before.
2. Lane 1, an Opus session: A1. Its brief is cut from this file's A1
   and from `next/reports/world.md`; where they differ, this file
   holds.
3. Lane 2, an Opus session in a clone: A3.
4. Lane 3: the README line by Sonnet; B1's note by Fable, then B6's,
   then B2's.
5. Then A2. Its first golden is the baseline every later step is read
   against.
