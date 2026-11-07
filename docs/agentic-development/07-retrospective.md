# 7. Retrospective

What worked, what it cost, what failed, and what the owner would do
differently. The first three sections are from the record. The
fourth is the owner's, from `INTERVIEW.md`, and is quoted as his
where he answered; where he has not yet, the section says so rather
than guessing.

## 7.1 What worked, with the evidence

- **The core held.** Every HTTP path traced by a reader runs inside
  one client and one gate; the ladder's ten live runs and every live
  run since show zero rate-limit violations in the ledger; the one
  server-side 503 halted the daemon without a single send after it.
  The project's most valuable asset was never put at risk by an
  agent, which was the point of most of the process.
- **Structure over discipline paid immediately and kept paying.**
  The dependency check, the frontend-door grep, the real-mode
  refusal without a terminal, the no-retry test runner. The
  2026-10-26 assessment lists these first under what to protect.
- **Breakers caught their own authors.** The founding suite found two
  of its own cases wrong, and the dependency guard's suite found two
  faults that "had failed unseen since the search crate linked the
  planner".
- **The seat produced product rules nobody would have written at a
  desk.** No id in user-facing text; a count says what it counts; a
  plan spends each request once; the default answer fits every
  harness's cut. Each came from watching a blind agent stop.
- **Independence found what self-review could not.** The first
  outside review corrected the review mechanism itself; a reviewer's
  remark started the daemon split; the Codex review of the gate
  script found it printing green for a filter that ran nothing.
- **The pace.** From zero Rust to a three-platform release in 71
  days, with 165,000 lines of which 58% are tests. The owner's C++
  years ran at 15 commits an active week; the Rust weeks ran at 129,
  then 190.
- **The system could take criticism.** Two assessments in a week,
  both sharp, both acted on within a day. The assessment itself
  called this rare.

## 7.2 What it cost

- **Words.** 2.74 MB of commit messages since June, as much as all
  the markdown in the tree. The honesty slice: about 226 KB of
  briefs, reviews and messages for 7,400 lines of Rust. Between a
  third and a half of commits in every AI month touched no code.
- **The owner's attention.** Rulings first for every class of change
  put the owner in the approval path for a flag's wording. His
  leverage went to semicolons. He fixed this on 2026-10-26 by
  delegating the agent-facing class and demoting the budgets, after
  the assessment measured it.
- **Review loops without a stopping rule.** The rate-limiter spike
  needed a materiality threshold imposed from outside; the daemon
  split ran about 24 review rounds in two days; the release's six
  rounds found every follow-up fault in the agent's own fixes. Each
  time the owner had to stop the loop himself.
- **Money.** Seat runs cost a dollar and a half to three and a half
  each at the release. The 114 session digests this series rests on
  cost about fourteen dollars at batch rates (3.5 million input
  tokens, 650,000 output). The project's own token spend is not
  in the record; the interview asks.
- **Disk.** 65 GB of object files found in September; a main
  checkout at 124 GB by 2026-10-25, with a `cargo clean` written into
  the phase close.
- **A red gate for a week.** The Windows gate failed every push from
  the alpha until the readiness step fixed two test defects, while
  "the gate decides" stood in the entry file. The assessment named
  it; r1 closed it.

## 7.3 What failed, plainly

Listed because the series promised the failures in the body, and
because each one produced a rule that is now in the brief template
or the memory files.

- **Two parallel workers overwrote each other's scratch** (2026-10-19).
  Rule: a parallel brief names each worker's own scratch directory.
- **One surveyor killed another's daemons** believing them stray
  (2026-10-21). Rule: kill only the processes you started.
- **A bare command with an offline flag read the owner's real account
  index** before refusing (2026-10-17). Rule: every command runs in a
  mock world, offline flags included.
- **Commit messages carried predicted byte counts and had to be
  amended twice** (2026-10-17, again 10-21 and 10-25). Rule: measure
  in one call, read the number, write the message in the next.
- **Five numbers in two briefs were wrong**, written from memory and
  disproved by the worker (2026-10-25). Rule: run every grep the
  brief cites and paste the hits.
- **A "blind" trial had loaded the memory index** and its record said
  "no memory" (2026-10-20). Rule: a blind runner gets a world outside
  the repository.
- **The entry-file budget blocked a design** and had to be raised
  under temporary permission (2026-10-21), then was demoted to a
  note (2026-10-26).
- **A session trimmed the owner's wording to fit a byte limit**, and
  he spent a commit restoring the semicolon (2026-10-26).
- **A hand-written fixture drifted from the real API's shapes**, and
  the owner had to grant store reads to fix it (2026-10-21).
- **The generated references pinned wrong sentences**: five verified
  on 2026-10-26, including a job kind the daemon refuses by name.
- **The transcripts of the first two months are not on this
  machine.** The method note explains why; a second round could
  recover them. The lesson for anyone starting: decide on day one
  where the sessions' records live, because the commit history alone
  cannot say what the owner asked.

## 7.4 Patterns under the failures

Three shapes account for most of the list, and the owner named all
three before the series did.

- **Claims wider than evidence.** Predicted counts, remembered
  literals, "rare" without reading the bound. The owner's rule:
  measure before the message; verify before deferring. The review
  prompt's first shape is now "a claim wider than its evidence".
- **Fixes that stage the nearest failure.** A test that passes for
  the wrong reason, a patch per case instead of one rule. The
  owner's rule, in the memory note of 2026-10-19: "think about deeper
  patterns ... even if it confirms we just need to address these
  findings individually."
- **Shared state between parallel workers.** Scratch, stash,
  processes. The owner's rule, by experience: one worker, one
  worktree, one scratch, its own processes.

A fourth is the project's and not the agents': process that
accretes. The founding principle said ceremony is retired by
evidence; the record shows one retired checker in two months until
the owner forced the issue. His own phrase for the period, given for
this series: the project was "rapidly accumulating process cruft and
I was along for the ride."

## 7.5 The owner's account

*From `INTERVIEW.md`, his first-generation answers of 2026-10-27,
which he may still revise; quoted as his. The question numbers are
the interview's.*

**Before the agents.** The 2024 era was web chat: "describing
problems of pasting code snippets into a web chat, then copying bits
and pieces back. It was a long time before I felt comfortable
dabbling with a CLI coding agent." What the chats gave him was speed
up learning curves and a way to brainstorm: "'what are my options'
was my favorite prompt from this era." The rewrite had three
motives, "the fun of learning a new language, pushing my AI skills,
and picking a popular modern tech stack", and he had been "toying
with the idea in stages", including rewriting the C++ UI in QML by
hand. The fully agentic rewrite "took off" when an outside
contributor reached out after forking the app to add agentic hooks:
the pull request report 4 names as the trigger of the
rewrite-or-evolve record.

**Time and money.** "I've let this take a lot of my time. Several
hours a day on average, and sometimes a partial or full weekend,
with breaks for socialization or travel." The project ran beside a
day job, which confirms what the commit rhythm suggested, and he was
travelling for the Eastern-time stretch in September and for most
of October. The money: a $200 Claude subscription; a $200 Codex
subscription cut to $20 once Fable 5.1 and Opus 5.5 arrived and the
OpenAI models were "relegated to external review", which "has been
enough".

**A working day.** He asks an Opus-class model to summarise the
state and present options; complex analysis or synthesis gets a
Fable session, "planning and building and investigation" gets Opus.
On what he reads: "I don't know how I decide what to pay attention
to in the agents' outputs. I skim quickly, but stop to read about key
milestones and decisions. Yet, I'm not sure how I pick those items
out. It's somewhat gut instinct and keyword recognition."

**Worst and best.** The worst moments: "when I'm halfway through a
plan and new problems seem to be coming up faster than they are
being fixed. When I look into things I often find a bunch of cruft,
so I know the agent is reading so much and trying to keep track of so
much that it's entering the dumb zone." The best: "The first time I
pushed Fable to think deeply and holistically about Acquisition's
design as a key stakeholder, asking the model what it wanted to see
as a developer and also as a user of the CLI and MCP. The output was
eye-opening."

**On "the words are the mass."** "Heh. This repo has been a learning
curve. One of the meta patterns is that I'd add a layer of
governance, policy, or procedure. That layer would be appropriate at
first, but over time cruft would accumulate and eventually become
untenable, so I'd call in an advanced model to help me think deeply
to understand and reframe the situation." He adds a second cause:
"I did not start using external mechanisms like GitHub issues and
milestones until several thousand commits into the project." His
verdict on the result: "I see it as a sign of learning the hard way.
An existing organization and team or experienced developer should
have things in place to avoid this. However, agentic development is
evolving so rapidly, I still want to spend time close to the models
as much as possible, so I'm willing to learn these lessons."

**The model roles.** Not a benchmark: "Fable 5.1 and Opus 5.5 just
seemed to produce results and communicate with me on my wavelength
in a way that Codex didn't." On the untrailered commits of August:
"Codex was probably writing the commits. I rarely wrote commits
myself once I started using the agentic development tools."

**Trust.** What he does not trust: "I don't trust agents when they
say they are done." Two reasons: "External reviews find material
issues almost every time", and "If I follow up with 'is there
anything else you'd like to do before we wrap up this session', the
answer is almost always yes, and it's almost always something
significant." What he now trusts them with: "I trust agents to
brainstorm, design, plan, code, deploy, test, and maintain. I trust
the most advanced models to think more deeply and more abstractly
than I myself can."

**On the missing transcripts.** The August to September sessions were
cloud sessions; whether they can still be retrieved he could not
check from a plane, and they "might also be linked to my desktop
computer at home."

## 7.6 What he would tell Aaron, and his 2024 self

Three things to copy, in his words:

- "Let the agents be smarter than you. Fable especially you can push
  to think deeply to find hidden patterns and cross-cutting insights
  that a basic 'build this thing' approach will miss." He notes that
  those sessions he has had to prompt by hand, and that he has not
  yet asked a model to write such prompts, so he does not claim it
  cannot.
- On the agent's questions: "I often find when the agent wants to ask
  me a question, I don't like the choices it gives me or I don't
  understand the question well enough to give an answer that adds
  value, so I ignore the question and write my own prompt asking for
  more information or to investigate or reframe the problem."
- "Push the whole system, but let it grow organically and learn to
  develop a gut sense for when you need to intervene and call in a
  Fable-class model for an intervention to figure out how to address
  whatever cruft or dead end you are wrestling with."

On a team of ten humans and agents: "I'm honestly not sure about
this at all. However, I suspect that governance by prompting and
markdown needs to be upgraded to an external system like bothy,
beads, or GitHub issues, projects and milestones." (Bothy is his own
sibling project, a registry linter; beads is a third-party issue
tool for agents.)

To his 2024 self: "Go agentic. Be more ambitious. Become a systems
engineering manager. Spend time 'close to the model' so that you can
develop your own intuition for how to work."

What the record suggested before he answered, and which his answers
confirm or sharpen: delegate agent-facing design early and reserve
rulings for boundaries; set a stopping rule for review before the
first round; write plainly; budget as a note, not a failure; put
issues in an issue tracker from the first week; and retire something
every week. His own addition is the one the record could not have
produced: the cycle of adding a layer, watching it accrue cruft, and
calling in the most capable model to reframe, which he now treats as
the method rather than a failure of it.

## 7.7 For Aaron

The honest summary of this project for someone who will try it: the
agents can build a serious system fast and well, and they will do
exactly what the written environment tells them, including the
wrong things, for as long as nobody edits it. The owner's job turned
out to be editing the environment: the invariants, the rules, the
briefs, the stopping rules, and above all what to take out. The
project's best week was the one where he cut his own process. The
playbook in report 8 starts from that week, not from the first one.
