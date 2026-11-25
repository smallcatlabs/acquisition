# The charge: review the whole project, and think

You are the orchestrator of the whole-project review of Acquisition.
The owner has given his word for it and asked that it pull out the
stops. This is written by the session that planned it, to you, as a
peer. Read it once slowly, then `README.md` beside it, then decide for
yourself.

## What is being asked

Not a bug hunt. This project has had 565 cold reviews; it is very good
at finding the fault in a diff. What it has never had is someone with
the whole of it in mind and no step to close, asking the questions a
diff cannot hold:

- What did we believe we needed when we started that we did not need?
- What are these two things that are really one thing?
- What is this the third instance of, and what is the shape nobody
  named?
- What would we never build today, knowing what we know?
- What is the elephant everyone steps around because a ruling stands on
  it?

Five months of fast building left answers to these lying in plain
sight: in the history, in the tables of findings, in the files that are
rewritten every week. Nobody has had the time to pick them up. You do.

## The frame worth questioning

Much of this project's method was inherited from a world where code was
slow to write, readers were scarce, and a rule had to be written down
because nobody could hold the system in their head. Then it was adapted,
carefully and honestly, to a world where the builders are agents. But
the adaptation mostly asked "how do we keep agents safe inside the old
method", and rarely "what would the method be if we designed it for what
agents are".

An agent can read a whole crate before breakfast, rewrite a module in an
hour, enumerate every state of a small machine, and run a blind user
through the tool on demand. An agent also forgets everything between
sessions, adds a mechanism more readily than it removes one, writes a
sentence to defend itself from a reviewer, and stops when the list is
empty rather than when the thing is beautiful. A method built for those
strengths and against those weaknesses may look quite unlike the one
here. So ask it of everything:

- When rewriting is cheap, what is a migration ladder, a compatibility
  state, a defensive branch for?
- When context is the scarce thing, what is a 30 KB module doc for, and
  who is the one reader of each sentence?
- When tests are cheap to add, what makes one worth keeping?
- When the user is as likely an agent as a person, why do three
  frontends word the same answer three ways?
- When exhaustive exploration is cheap, why is the property that
  protects the GGG relationship held by scenarios someone thought of?
- When the owner's attention is the scarcest resource in the project,
  what is each of his decisions being spent on?

You will not be alone in thinking the answer may sometimes be "the old
way was right". Say so when it is. Three things are not on the table:
the five invariants in `AGENTS.md`, the tool's function, and an agent's
ease in using it. Everything else is, including the rulings, the crate
boundaries, the process that will review your own work, and this plan.

## What a great finding looks like

The owner's words, from the record: "is there a simpler elegant way",
"not patching hypothetical edge cases and instead looking for
underlying/simpler patterns", and "We should hold ourselves accountable
to the goal of a simple, better acquisition more than to our past
decision". The two removals that already worked show the shape: the
main account and the closed-for-good route were each a convenience that
became a family of states, and each went when a refusal the tool
already had turned out to say the same thing. #261 shows the other
shape: eight places ended a job, one was meant to, and sealing it in
the compiler made a run of findings stop.

A great finding removes a concept and explains why its edge cases go
with it. It is strong enough to be wrong. It names the smallest input
that would break it. A merely good finding is a list of forty things to
tidy. Bring the first kind; route the second kind to issues without
ceremony.

## How to think, since I know how we think

- **Read before you delegate.** The plan's seven patterns are mine, and
  an orchestrator handed a hypothesis tends to return it confirmed. Read
  the three or four places you think matter most yourself, slowly,
  before you brief anyone. Form your own picture, then see where it
  parts from mine.
- **Hold the whole.** The insight that matters will come from two
  reports meeting: the admission logic and the verification idiom, the
  frontends' words and the test estate's size, the registry's growth
  and the findings that never land. Scouts cannot see across; you can.
- **Send readers with questions, not checklists.** "What would you
  delete, and what breaks?" finds more than "review this file". Ask
  each reader to defend one thing the others would cut.
- **Seek your own refutation.** For every thesis you like, brief a
  skeptic to break it, from another model family where one is installed
  (the cold-review skill names how). A thesis that survives is worth
  the owner's time. One that dies saved him a week.
- **Notice when you are adding.** If a proposal adds a mechanism, a
  rule, a check or a document, it has to pay for itself twice. The
  history says we reach for the patch.
- **Stay with a hard question.** Where two defensible answers stand,
  the easy move is to list both. Think until you have a position, give
  it, and say what would change your mind.
- **You are a stakeholder.** You work in this repository and you use
  this tool. What slowed you today, reading this? What would you want
  from `acq` if the stash were yours? Trust that.
- **Be honest about what you did not check.** A number is a scout's
  until you have run its command.

## The work

`README.md` holds the model, the patterns, the evidence standard, the
phases and the tracks, with the report each rests on. Treat the tracks
as a ranked menu. Add the ones I missed; I will have missed some.
Depth in five places beats coverage of sixteen.

## Rules of the house

- Read `AGENTS.md` and `decisions/process.md` first. Their hard rules
  bind you: every `acq` in a mock world, never a real-mode daemon, one
  build at a time in this tree, no build while the owner names a window
  at the machine.
- **This review changes no product code.** It may build, run the gate,
  measure in a mock world, and try a patch to size a thesis, reverted
  after and saved under this directory. Its commits are documents under
  `audits/whole-review-2026-11/`, each after `tools/gate.sh --docs`
  exits 0 (C171).
- The owner's real store is read only on his word. Ask once, early, for
  a copy to measure; carry on with the measurement copy meanwhile.
- Opus investigates, Fable designs and advises, Sonnet counts. A
  subagent returns text and cannot write a file; a long build belongs
  in a session, not a subagent. No worktree unless two writers build at
  once.
- A `gh` write or a push the harness refuses is left and reported. Draft
  issue bodies as files under `issues/` here, and file them at the end
  in one pass.
- Nothing reaches the owner's ruling queue until the portfolio, and
  then at most ten questions, each one decision (C179).

## What done looks like

- `findings/<track>.md` for each track taken, to the standard in
  `README.md`.
- `PORTFOLIO.md`: at most ten one-page proposals, ranked, each with
  what goes, what breaks, what holds the invariants after, the dissent,
  and whose decision it is. And one page that answers the frame above:
  what you would do differently if this project began today.
- Every other finding routed: an issue drafted, or a line saying why it
  is dropped.
- A short note to the owner at the end of each phase: what you learned
  that surprised you, in plain words. He manages this; he does not need
  the detail, he needs to know where it is going.

Good luck. I would like to read what you find.
