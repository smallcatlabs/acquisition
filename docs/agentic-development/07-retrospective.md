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

*From `INTERVIEW.md`. Quoted as his where answered; the questions he
has not yet answered are listed so the reader knows what is missing.*

Said in the transcripts, in the course of the work:

- On his method, 2026-08-10: "You've lived through how I build best
  with AI, plan, review, build, review, repeat. That's how I get
  things done." And the same evening: "i'm dismayed with how complex
  this spike has grown due to the way we've been building it, but
  i'm committed to the process so we will see where it lands."
- On taking a process failure on himself, 2026-08-10: "I will change
  my process to review each slice. I had thought they were simple
  enough to breeze through, but it looks like I was wrong."
- On the documentation tax, 2026-09-24: "I want to stop now and think
  about what we can do to minimize or even reverse this content
  growth, because we have about 4 more phases, and the grown will
  make each one more difficult."
- On his own prompting, 2026-10-18: "I've gotten good results with
  very basic prompt engineering combined with developing my own
  judgement on how to direct my agents."
- On his limits, 2026-10-23: "(i'm smart, but this work is very
  technical and i'm unfamiliar with its inner working compared to
  you)".
- On what the project is for, 2026-10-26: "It may be a hobby project,
  but I want us both to be proud of how professionally acquisition is
  managed, and I want us both to enjoy the process of building as
  well as of using what we build."

Answered in conversation while this series was written:

- On the rewrite-or-evolve record: once created, he "promptly
  proceeded to pretty much ignore it."
- On the period of accumulating process: "I was along for the ride."
- On the second model: Astra, the OpenAI model driving Codex, is
  "somewhat comparable to Fable."

Not yet answered: the 2024 era of learning C++ with AI; the first
moment an agent did something he could not have; why rewrite rather
than maintain; hours a week and money; a typical day; the worst and
best moments; how the assessment's verdict landed; how he arrived at
the model roles; what he does and does not trust the agents with;
the four questions the commit history raised (the cloud sessions,
the untrailered commits, the Eastern-time stretch, the day job); and
the three questions for Aaron.

## 7.6 What the record suggests he would do differently

The owner has not said; the record has. Each of these is something
the project changed itself, late, after measuring:

- Delegate agent-facing design from the start and reserve rulings
  for boundaries. The project did this on 2026-10-13 and then
  un-did it with "rulings first" the same day, and re-did it on
  2026-10-26.
- Set a stopping rule for review before the first round, not after
  the twentieth.
- Write plainly. The compressed register cost a glossary, a ruling,
  and a tax on every reader.
- Budget as a note, not a failure.
- Decide on day one where session records live, and keep them.
- Retire something every week. The principle was there in September.

## 7.7 For Aaron

The honest summary of this project for someone who will try it: the
agents can build a serious system fast and well, and they will do
exactly what the written environment tells them, including the
wrong things, for as long as nobody edits it. The owner's job turned
out to be editing the environment: the invariants, the rules, the
briefs, the stopping rules, and above all what to take out. The
project's best week was the one where he cut his own process. The
playbook in report 8 starts from that week, not from the first one.
