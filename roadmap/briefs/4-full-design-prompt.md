# 4-full's first design: a GUI and an agent at one daemon

*Draft prompt for a design session, to revise with the owner before it
runs. Kept on its own branch until the owner settles it.*

## Why this session exists
The owner wants to use Acquisition himself, every day, through a GUI and
through his agents together, starting before the 3.30 league launches.
That is the fastest way to find out whether the tool is right, and the
people most affected by the result are the owner and the agents who
build and use it. Nobody else uses the project yet. On 2026-11-06 he
moved 4-full's first slice ahead of `alpha.2` for this reason.

This session designs that slice. It builds nothing. It ends in a design
brief, `roadmap/briefs/4-full-design.md`, in the form of
`roadmap/briefs/4-0-design.md`, with the rulings it needs drafted for
the owner.

## Two scenarios, in the order they happen
The design is judged by whether these two mornings work. They are the
acceptance test, and at the end the owner runs them himself as the
validating consumer (P2, real use validates).

**1. Before launch, on Standard.** The owner signs in on Standard and
sits at the GUI looking for specific gear to test builds before 3.30.
An agent beside him, over `acq-mcp`, helps him search: it proposes
queries, narrows them, and explains what it found. Most of this is
reading the stored corpus; refreshes are occasional.

**2. Launch morning.** The owner signs in, refreshes his stash for the
new league in the GUI and watches it land: hundreds of child jobs, rate
limits that hold for minutes, failures with a way on. Meanwhile an
agent prices items beside him, and the GUI sees the agent's price
writes without being told by hand.

Scenario 1 comes first in time, and it asks different things of the
system than scenario 2: it's mostly search, the agent and the person
sharing one view of the stash, and the daemon kept alive while either
is there; scenario 2 is mostly jobs, events and the live loop. Say
whether that means two slices, and in which order, or one.

## Your stake in this
You're a primary stakeholder twice over, and the design should be judged
by whether it serves you:

- **As a user.** You are the agent in both scenarios. What do you need
  from the tool to help a person who is looking at the same stash in a
  GUI? How do you hand him what you found? How do you wait on a job
  that may hold for five minutes without sleeping in your loop? What
  would make you say something wrong to him?
- **As a developer.** Agents like you will build and maintain this.
  What shape keeps a third frontend from becoming a third copy of every
  rule? What can be pinned by tables of events instead of by timing?
  What would you not want to inherit?

When the two seats want different things, say so plainly. Don't average
them.

## How to think
Take your time. Read the system as it is now, not as the GUI notes
describe it: they were written on 2026-10-18, before the time seam, the
transport seam, the paged job list and the job record's times and
failure kinds. Look for the pattern under the issues, not a list of
fixes. The aim is for complexity to collapse into something simpler
where practical: one mechanism in place of several polls, one machine in
place of per-frontend waiting logic, one rule a reader can hold in their
head.

Prefer shapes that make a wrong answer hard to write (AGENTS.md,
"Structure over discipline"). Be suspicious of a design that covers
every issue; say what each choice costs and where it strains. JSON
changes are additive (C53), and the protocol is single-version (C10), so
count the cost of every wire change honestly. It is a valid conclusion
that the protocol needs little or no change for the first slice, if the
evidence says so.

## Read
- `AGENTS.md` (loaded), `GLOSSARY.md`, `roadmap/README.md` (the step
  rhythm, the roles, phase 4's row).
- **The starting sentence.** `roadmap/reviews/time-seam-retrospective.md`:
  Root B ("the door built the wait the frontends lack"), its two shapes
  (the subscription widened to carry auth and lifecycle events, and a
  bounded wait on the request plane), step G1 in §10, §11 question 5,
  and the `IDLE_REJUDGE` row in §3, which names the first long-lived
  subscriber as its trigger.
- **The design as it stood.** `roadmap/GUI-NOTES.md` §3 (how a GUI plugs
  in; re-check its claim that no protocol change is needed), §5 (the
  live results loop), §8 (the foundation moves) and §12 (the build
  sequence). `roadmap/briefs/4a-design.md` and `4b-build.md`, for what
  4-lite built.
- **The rulings**, whole, with their mechanisms in the code: C85, C10,
  C13, C16, C138, C139, C157, C160, C165, C166, C162, C163, and C46,
  C53, C143 where a frontend speaks. The "Not doing" lines in
  `decisions/daemon.md` and `decisions/frontends.md`, especially "a
  GUI-hosted daemon: refused; the GUI spawns `acqd` as the CLI does, and
  phase 4 records the spawn policy".
- **The machines to copy.** `crates/acquisition-client/src/door.rs`,
  `crates/acquisition-daemon/src/daemon/life.rs`, and the client's
  `Subscription` in `crates/acquisition-client/src/client.rs`.
- **What polls today.** `crates/acquisition-verbs/src/jobs.rs`, `acq
  auth`'s wait, the dashboard, and the MCP's job tool texts
  (`MCP-REFERENCE.md`).
- **The web frontend.** `crates/acquisition-cli/src/web/`, and
  `acquisition-cli/tests/web.rs`.
- **The issues.** A starting set, not a boundary; scan every open issue
  and add or drop on your own reading:
  - the subscriber and the daemon's lifecycle: #1 (the keeper), #15
    (reconnect), #14 (results over a subtree), #177 (answers still whole
    after the paged list), #3 (`spawned_by`), #2 (multiplexing);
  - the live loop's data side: #25 (one change cursor over facts and
    intent), #32 (an incremental corpus update), #95 (memory per
    reload);
  - the GUI as the owner meets it: #176 (pages still print the CLI's
    text), #98 (cards), #19 and #42 (a default league; user-scoped
    intent), #13 (a queue UI);
  - possibly settled by C165 already, to check: #17, #7.

## The owner's decisions so far
- 4-full's first slice moves ahead of `alpha.2` (2026-11-06).
- The two scenarios above are the target.
- #142 (t6, one canonical mock world and the scenario corpus through
  both doors) is **not** a prerequisite. The first slice runs on the
  existing harness, under one rule: it adds no private world or seed
  function of its own, only rows in the mock's one world. t6 follows
  the slice, designed from the GUI's door as its third consumer rather
  than guessed before it exists. Say if the design shows this to be
  wrong.

## Questions to answer
1. **The slice.** What exactly must exist for each scenario to work, end
   to end, with the owner at the GUI and an agent at the MCP? What can
   wait? One slice or two, and in which order?
2. **Root B.** Which of its two shapes does the slice need: the widened
   subscription, the bounded wait, or both? What does each frontend
   gain, and what does each cost on the wire, in the daemon and in the
   tests?
3. **The daemon's life with a GUI attached.** Who spawns `acqd` and who
   keeps it alive (#1)? Does an open subscription hold the daemon up,
   and what does an agent connecting per call then meet? What happens
   when the owner rebuilds and the GUI, the CLI and an agent each meet a
   mismatched daemon (C10: the MCP never replaces)? The `IDLE_REJUDGE`
   row.
4. **The subscriber as a machine.** The subscriber with reconnect as a
   third machine under C157 and C160, after `Door` and `Life`: its
   states, events and effects, and the table that pins it. Where does it
   live (C1, C46), so the GUI and any later frontend share it?
5. **What changed, beyond jobs.** Job events don't cover writes that
   never reach the daemon, such as an agent's price set through the MCP.
   How does the GUI learn the store or the intent moved (#25)? What does
   each change cost the GUI's held corpus (#32, #95), and does the GUI
   and an `acq-mcp` each holding a corpus matter?
6. **The agent and the person, together.** In scenario 1, how does the
   agent hand the owner what it found, so it appears in front of him in
   the GUI? Consider what already exists (a query's text, a trade URL as
   a third spelling of a query, C138's per-launch token) before
   proposing anything new.
7. **The rulings.** Draft, in the registry's form, every ruling the
   slice needs: the C85 amendment if any, the spawn policy, the keeper,
   anything else. Mark each as an amendment or a new ruling.
8. **What not to do.** Shapes you considered and rejected, and why.

## Deliverable
`roadmap/briefs/4-full-design.md`, every claim cited by path:line,
commit, issue, ruling id or seat transcript. It ends with:
- the build, in steps ordered by risk, each with what it retires (C154's
  form) and the issues it closes by their own "Closes when";
- "Drafts for the owner": the rulings, each with your recommendation;
- "Raised for review": what a cold reviewer should press on.

The brief is then reviewed cold, per the rhythm in `roadmap/README.md`,
before the owner rules on it.

## Boundaries
- Design only. **Don't change code or rulings, and don't comment on,
  close or relabel issues.** Commit only the brief, and only when it's
  done.
- You may run the mock to see what a surface does today, under the
  mock-session skill. Follow the build-order rule. Never start a
  real-provider daemon or run a bare `acq`.
- No id in text a user would meet (C116), in any wording you propose.
