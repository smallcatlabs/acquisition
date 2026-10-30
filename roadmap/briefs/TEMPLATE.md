# <step id> — <title>

Every brief has these headings. A worker reads `roadmap/README.md`,
then this brief, then what it names, and nothing else unless the work
turns it up. A brief's Rules section says `TEMPLATE.md` and lists only
what differs for the step; its Report section says `TEMPLATE.md`. A
copied rule drifts: 2d-oracle's copy differed from this file in five
bullets.

## Goal
One paragraph: what is true when the step is done, and why it matters.

## Read
The files, sections and decision ids this step rests on, each with a
few words on what to take from it. The brief's writer checks every
citation, and every test the brief relies on, against the tree
before the brief is committed: phase 0's briefs cited a ruling as
built that was parked, named a test that did not exist, and carried
a count wider than true, and each cost a review finding. Where a brief
changes the words or shape of a document a frontend prints, it names
every test that asserts on that document, found by grepping the test
trees for the old words: 2a's design named eleven, and three more
broke.

## Input
What is fixed before the work starts: a measurement, a capture, a
ruling, and the failing inputs the step's tests will stage, each
named concretely. A step with no fixed input says so. A step whose
work includes a sweep for a class runs the sweep before the brief is
committed, and its Input lists the instances found, so the reviewer
holds the commits against a fixed list (clock-tests' Do 4 ran after
its brief). A brief that drafts a registry line (a ruling, a "Not
doing" line) runs `python3 -I tools/bothy.py check` on it before the
brief is committed (league-buildables' two Not-doing drafts failed
its form).

The last push's CI: each gate's run on `main` (`gh run list --branch
main --workflow "Gate (Linux)" --limit 1`, the same for `Gate
(Windows)`), its commit, id and conclusion. A red run says whether
this step fixes it as its first concern or files it, and where; a run
still in progress is named as such, and the step goes on.

## Do
Numbered, concrete, in order. A ruling the step needs and does not have
(a boundary, an invariant, anything touching the real store or the real
API) is the first item, brought to the owner before the code. An
agent-facing design decision is the step's own: recorded in the commit
that builds it and listed at the close (`roadmap/README.md`, "A step").

## Done when
Checkable sentences, each from the commit alone: its tree or its
message. The report may restate a fact and never holds one alone,
because the reviewer reads the commit and not the report. The gate
green is always one of them, on the step's own tree: a step never
needs what a later step delivers. A number or a literal in a line is
measured on the tree the brief is written against, or the line says
the build measures it: 2e's brief said fourteen tabs where the empty
league lists seven, and a grep whose literal the fix changed.
A fix round's done-when is its own, not the brief's: each finding of
the review it answers fixed by one rule and pinned, or named as not
taken and why, and the gate green on the round's tree. Execution
evidence (the gate's exit status per stage, the test count, a
measurement) lives in the worker's report, which the orchestrating
session holds against the review; a commit message carries what it
measured, never the gate table.

## Not in scope
What a worker will be tempted to do and must not, with where it lives
instead.

## Rules
`AGENTS.md`'s hard rules apply, and the rhythm is `roadmap/README.md`,
"A step". What a worker adds:
- Your own worktree and your own scratch subdirectory, named in the
  brief's launch; never another worker's. Kill only the processes you
  started.
- Touch nothing in the tree while the gate runs: its `tree` stage fails
  on any edit, and an edit to a source it built fails `artifacts` too
  (daemon-safety's first gate).
- One commit per concern; the story in the message, which names this
  brief's file; a size or a count stated in a message is measured after
  the last edit. A test rides in the commit of the fix it pins, and a
  breaker case in the commit of the check it stages: step 1e ran the
  full gate eight times for that. A brief that wants a test seen
  failing first asks for it in the report, not the history.
- Each concern may commit after its own targeted tests; the one full
  gate runs on the step's tree before the "built" commit, and that
  commit waits on its exit status.
- A draft the brief names is taken verbatim unless the tree disproves
  a sentence of it; each correction is named in the report. A brief
  sentence a fix round corrects in place is marked "Settled in the
  round".
- A JSON pin may add `--json` to a printed command it runs.
- No test asserts a wall-clock bound a slow machine can miss: a
  deadline's behaviour is pinned by forcing its expiry, and a lifecycle
  that waits across processes is designed as a state machine over an
  injected clock before it is built (C137, the deadline's contract is
  its expiry behaviour).
- An id names its rule in words the first time a brief, a report, a
  commit message or a ledger line uses it ("C84, the artifact
  mismatch"): the reader outside the area has no registry open. The
  user-side twin is C116, no id in user-facing text.

## Report
Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What the gate said, as exit statuses per stage.
3. Measurements, before and after, where the brief asked for them.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is wrong,
   and what would have made the job easier. Unstructured, a sentence
   each; the session that reads it answers every item in the close
   commit's message (taken, filed, or not taken and why).
