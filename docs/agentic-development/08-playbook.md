# 8. A playbook

What an engineer would do on Monday to work this way, scaled to a
small team or a solo founder building a modern web application, and
an explicit list of what in this project not to copy at that scale.
Everything here is drawn from what the record shows worked, failed,
or cost more than it returned; where the recommendation goes beyond
the evidence it says so.

## 8.1 The one-page version

1. Write a short entry file the agent reads first. Hold it to a
   budget. Put in it: what the product is, the three to five
   properties that must never break, who decides what, how to run
   things safely, and where everything else lives.
2. Make every rule you can into a check. A dependency edge, a lint,
   a test, a refusal in the binary. Keep the rules that cannot be
   mechanised few, and state them as properties.
3. Give every unit of work a brief: goal, what to read, what done
   means, what is out of scope. Have the agent read nothing else.
4. Run a gate before every commit and paste its result into the
   commit message. Never retry a flaky test; a test that fails once
   is a finding.
5. Review with a different model than the one that wrote the code,
   and do not show it the author's explanation.
6. Test the tool's words with a blind agent that has nothing else.
7. Record the owner's decisions verbatim in history, with the reason.
8. Ask every worker, at the end, what got in the way, and answer.
9. Measure the prose. When it grows faster than the code, retire
   something.
10. Keep anything that touches production, money, or a third party's
    terms of service behind a door only a human can open.

## 8.2 Day one: the entry file

The project's entry file went through six stages (report 5, section
5.5). Start at stage 1, not stage 6. The 1,916-byte file of
2026-08-24 had everything needed for a first month:

- a reading order of four documents;
- one sentence on who decides: the owner holds the boundaries, agents
  own internals;
- the safety rule for the dangerous thing (there, the real API; for
  a web app, production data and third-party services);
- a four-line quality gate;
- a recipe for an isolated working environment.

Write it in plain sentences. The project's compressed register cost
it a glossary and a ruling later. Put a byte budget on the file from
the start, but as a note that prints, not a failure: the project
learned that a hard budget produces byte-shaving at the margin.

For a web application, the dangerous thing is usually one of:
production credentials, a payment provider, a customer database, an
email or notification service, or a third-party API with rate
limits or terms. Name it in the entry file on day one and make the
local environment the default, structurally. The project's best
version of this is a binary that refuses to run in real mode without
a human at a terminal.

## 8.3 Week one: the gate and the checks

Set up, in this order:

1. **A gate script** that runs build, tests, lint, format and any
   document checks, prints an exit-status table, and exits non-zero
   if any stage failed. The project's lesson: never gate on a
   filtered grep, and put the exit status on its own line.
2. **A dependency check**, if the stack has layers. For a web
   application: the UI layer imports nothing from the data layer
   directly; shared validation lives in one package both client and
   server import. The project's version refuses a frontend that
   "evaluates for itself", and that rule is worth copying exactly:
   a client that re-implements a server rule is a second semantics.
3. **No retries in CI.** The project's reason is good: a retry turns
   a flaky test green, "the opposite of 'the gate decides'".
4. **A breaker for each checker** you write yourself. A deliberately
   broken input the checker must refuse by name. This is the one
   idea in the series almost nobody does and the project's record
   shows paid for itself immediately.

## 8.4 The unit of work: briefs

Copy the template's headings and nothing else at first: Goal, Read,
Input, Do, Done when, Not in scope, Rules, Report. The rules the
project added later came from faults, and the three worth adopting
on day one are:

- every number in a brief is measured on the tree, not remembered
  (five literals in two briefs were disproved by the worker on
  2026-10-25);
- a done-when line is checkable from the commit alone;
- the report ends with "what got in the way", and the reader answers
  every item.

Have the agent read the brief and what it names "and nothing else
unless the work turns it up". The project's finding is that a fresh
session with a good brief outperforms a long session with
accumulated context, and that blindness is a property to engineer,
not assume.

## 8.5 Review

The project's loop is: commit, launch a reviewer of a different
vendor in a read-only sandbox with a fixed prompt, save its findings
verbatim, read them yourself, say which to fix, pin each as a failing
test before fixing it by one rule rather than per case, and record a
fault that came back as a row in a checklist. Three things to copy
exactly:

- **The reviewer never sees the author's explanation.** The commit is
  the evidence.
- **Plain defects first.** The first review of the project's own
  review prompt found that leading with the project's pet fault
  shapes "would steer a reviewer away from deadline and cancellation
  bugs". Put regressions, error paths, concurrency and persistence
  before anything bespoke.
- **A stopping rule.** The project needed one three times (the spike
  in August, the daemon split, the 2f phase). Decide in advance what
  severity gets fixed now and what becomes an issue, and do not
  review a third time unless someone asks.

## 8.6 Testing the words

If an agent will drive your tool, API or CLI, the help text and the
error messages are the interface. Once a release candidate exists:

1. Write a brief that says only what a user would know.
2. Start a fresh agent in an empty directory outside the repository,
   with the released build on its path and a script that logs every
   command, and give it the brief and the README.
3. Ask it to do five real tasks and report where it got stuck.
4. Resume it once for a debrief: what would you retract, which
   answers rest on your own knowledge rather than the tool's?
5. Check every reported fault against the log before believing it.
6. Fix each in its own commit with a test. Rerun with a new agent.

Budget a few dollars per run. For a web application the same
instrument works on an API, a component library's documentation, or
an onboarding flow: the question is always what the words are worth
to someone who has nothing else.

## 8.7 Decisions and their record

Keep a decision log with ids, but start lighter than the project
did. The three-part entry (the decision, one *Why* sentence, what
test holds it) is the right shape. Cite ids from tests and comments
so a search finds every consequence of a decision. Record the
owner's word verbatim in the commit that acts on it; the project's
history is readable because "omg yes ... time to grow up" is in it
next to the hash.

Do not require an entry before every change. The project tried
"rulings first" for every class of change and moved most of it back
within two weeks. Reserve rulings-first for boundaries: anything
that touches production, money, a third party's terms, data you
cannot refetch, or the public API. Let the agent decide the rest and
list its decisions at the close.

## 8.8 Models and roles

The project's division, reached by use: one model for assessment,
synthesis and design; a cheaper one for building from a brief; the
cheapest for searches and seat runs; a different vendor for review.
Two cautions from the record. The division was ruled on 2026-10-19,
after two months in which the most capable model wrote most of the
code; there is no evidence in the tree that one arrangement produced
better code than the other, only that the later one was cheaper and
kept the owner's attention on design. And parallel workers collided
three times in a week (shared scratch, a shared stash, a shared
process table) before each got its own worktree, scratch directory
and the rule "kill only the processes you started".

## 8.9 What not to copy at a startup's scale

The project is one owner, no team, no customer, a hard external
constraint, and a maintainer who wanted to learn. Several of its
mechanisms are answers to that situation and would be a tax
elsewhere:

- **A ruling before every agent-facing change.** The project itself
  retreated from this. A web team shipping daily should log
  decisions, not gate on them.
- **The compressed register.** Write plainly. The project's own
  assessment found the style "a tax everywhere else" and it leaked
  into user-facing text.
- **Numbered review rounds into the twenties.** The daemon split ran
  about 24 rounds in two days. A startup should set the stopping rule
  first and keep rounds to two or three.
- **Commit messages as documents.** They work here because the next
  session reads them. On a team with humans who read pull requests,
  put the story in the PR and keep the message short.
- **Seven kinds of ledger.** One decision log, one run log for
  anything that touches production, and an issue tracker are enough.
- **A byte-budgeted context file at week one.** Start with a page.
  Add the budget when the page becomes five.

## 8.10 What transfers to a team of humans and agents

The project's record is about one human. Extrapolating, with care:

- The entry file becomes the team's onboarding doc, and every agent
  session is a new hire who read it an hour ago. That is a reason to
  keep it current that human-only teams never had.
- "The repository is the only shared channel" is already how good
  distributed teams work; agents make it mandatory.
- Decision rights have to be written where the agent reads first,
  because the agent cannot learn them by watching.
- The reviewer-of-a-different-vendor rule generalises to: the
  reviewer is never the author's own model with the author's own
  context. On a team that is cheap to arrange.
- The owner's corrections (report 5, section 5.6) are the feedback a
  lead gives a capable new hire. On a team, they belong in the brief
  template, not in one person's memory.
- The cost of prose becomes a team cost. Someone has to own retiring
  checks and documents, or nobody does.

## 8.11 A starter kit, in files

For a repository that starts tomorrow, with a web application in
mind:

```
AGENTS.md            one page: what, invariants, who decides, safety, gate, where things live
CLAUDE.md            @AGENTS.md
decisions/README.md  the entry form; one file per area as areas appear
briefs/TEMPLATE.md   Goal, Read, Input, Do, Done when, Not in scope, Rules, Report
reviews/             the reviewer's findings per commit, kept
tools/gate.sh        build, test, lint, format, checks; exit-status table
tools/arch-check.sh  the import rules that must hold
tools/*-breakers.sh  what each checker must refuse
.githooks/           attribution trailer; enable once per clone
CHECKLIST.md         faults that came back, by area
RUN-LEDGER.md        one row per contact with production or a third party
```

Start with the first four and the gate. Add the rest when a fault
asks for it, which is the principle the project states and, by its
own account, found hardest to keep.
