# Notes to the owner, one per phase

Short and plain: what was learned that surprised, and where the review
is going. The detail is under `findings/`.

## Phase 0, ground truth (2026-11-24)

I read the charge, then sat at the tool with a copy of your store
before reading anyone's conclusions.

What surprised me:

- **It took me six tries to ask which items have 90 life.** Every
  refusal was correct and helpful. Nothing is broken. But nobody had
  counted the trips, and the plan I was handed points at the opposite
  problem (too many defaults).
- **Your job ledger is a second copy of your stash.** `daemon.db` is
  70 MB, 60 MB of it the bodies of finished fetches that the facts
  store already holds.
- **A search costs 0.9 s on your store, twice October's number,**
  because your store has doubled and each search rebuilds everything.
- **You have set 17 prices, once, in September.** The tool's real use
  is refresh and search. I say this only so the review weighs what is
  used.
- The answers themselves are very good. That part I would not touch.

Where it is going: nine Opus investigators are reading one track each
(the first answer, what persists, admission, where the words live, the
process, the GUI's direction, the tests, the search's kernel, the
stored row and the crate edges). Each was asked for what it would
remove and the smallest input that would break the removal, not for a
list. I hold the question of what the method would be if designed for
agents.

## Phases 1 to 5: the reads, the trials, the skeptic, the step back (2026-11-25)

The review is done as far as I will take it. You said it could stay
with finding and understanding, so `PORTFOLIO.md` is ten pages of what
is true and how I know, with a few lines of direction on each and seven
questions that are yours. Nothing is designed and nothing is filed.

What surprised me:

- **A search nobody had written found two faults in the send gate and
  the limiter in under two seconds.** One is #206, open sixteen days,
  guarded by an assertion that is not in the shipped binary. The other
  is pinned as intended by an existing test. I do not know that GGG can
  trigger either. This is the page I would read first.
- **Three of my ten theses died**, and the skeptic was right each time.
  I wanted the job queue kept in memory only, the sittings stopped, and
  a lighter method for the frontends. Each broke on one concrete case.
  The pages say which.
- **The plan I was handed was wrong about scope in an interesting way.**
  It expected leftover defaults. There are those, and there is also a
  refusal where a default used to be. The same lesson was applied two
  opposite ways in one tool.
- **Your server is already fast enough for the GUI you describe.** What
  is slow is that every click loads a page. I measured it badly (another
  agent was compiling), so it is a reason to try, not a result.
- **Facts are not refetchable, only their current state is.** A rename
  at GGG would orphan your history today.
- **The tests are better than I expected.** Three quarters of sixty
  read whole were doing real work. The weight is in how they are built,
  not in how many there are.

What I did not do: re-run every investigator's count (each findings
file says which I did), sample the search and store tests, sit a person
at anything, or file an issue. Thirty-three issues are drafted under
`issues/` and wait for your word.

Where it goes next is yours. If it helps: page 1 is small and protects
the thing you care most about; page 3 is the one decision that gates
the road; page 10 is the question I could not answer.

## After: what to build next (2026-11-25)

You asked what set of changes I would orchestrate, for the agents who
build this and use it, with nothing off the table. It is `NEXT.md`,
with its evidence under `next/`.

What surprised me:

- **The tool gave GGG a choke point, a journal and an oracle, and gave
  its readers none of the three.** Three pages of the portfolio are
  that one absence. The plan is to build the reader's side to the
  sender's standard: one seeded world like yours, a count of what each
  task costs an agent, and the ways on carried in the answer.
- **The agents who build the tool almost never run it.** 886 of 63,555
  shell calls in six weeks. A populated world costs a daemon, a login
  and three applies, so nobody checks a change by using it.
- **We read as much governance as source,** and three quarters of the
  turns are Opus's; Sonnet has done 0.4%. Much of what is planned is a
  move under a rule with a compiler to check it.
- **Every one of my first ideas was narrowed by a reader sent to
  break it.** One of them wanted the MCP to answer in text; the code
  and C53 say no, and I dropped it. One found that the account is
  already resolved in one place and the fault is smaller than I drew
  it.
- **Nothing tests that a page escapes a hostile tab name or note.**
  The markup goldens hold it by the accident of one apostrophe. It is
  the first line of the step that changes what those goldens hold.
- **A second build lane costs a minute and 2.5 GB,** from a
  copy-on-write clone of the cache. I tried it once in scratch.

- **Codex then broke the draft, as it broke the theses.** It showed
  that the rule I wanted for an unnamed league needs your ruling, and
  it named what I had set aside: a rename at GGG orphans your facts'
  history, and before a release is the cheap time to fix that. Both
  are in the plan now.

What I need from you is on one page of `NEXT.md`, "What is the
owner's": seven decisions, of which only the order holds everything.
