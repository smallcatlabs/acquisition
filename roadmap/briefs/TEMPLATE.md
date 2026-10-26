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
named concretely. A step with no fixed input says so.

## Do
Numbered, concrete, in order. A ruling the step needs and does not have
is the first item, brought to the owner before the code; a ruling
drafted for it is measured against the entry rule's 800 bytes before
it reaches the build brief (2e's first draft was 860).

## Done when
Checkable sentences, each from the commit alone: its tree or its
message. The report may restate a fact and never holds one alone,
because the reviewer reads the commit and not the report. The gate
green is always one of them, on the step's own tree: a step never
needs what a later step delivers. A number or a literal in a line is
measured on the tree the brief is written against, or the line says
the build measures it: 2e's brief said fourteen tabs where the empty
league lists seven, and a grep whose literal the fix changed.

## Not in scope
What a worker will be tempted to do and must not, with where it lives
instead.

## Rules
- The rhythm (the commits, the review, the owner's reading, the close)
  is `roadmap/README.md`, "A step", stated once there.
- Every `acq` runs in a mock world: `ACQ_PROVIDER=mock` and
  `ACQ_STORE_DIR` under your scratch. Never the real provider.
- Your own worktree and your own scratch subdirectory, named in the
  brief's launch; never another worker's. Kill only the processes you
  started.
- `tools/gate.sh` before the commit, the commit gated on its exit
  status, never on a filtered grep. Never run the drivers while the
  gate runs.
- One commit per concern; the story in the message, which names this
  brief's file; a size or a count stated in a message is measured after
  the last edit. A breaker case rides in the commit of the check it
  stages, never in a commit of its own: step 1e ran the full gate eight
  times for that.
- A finding is a test named for the fault, or a line in the report;
  never a new document. A review's finding is not fixed before the
  owner has read it.
- A draft the brief names is taken verbatim unless the tree disproves
  a sentence of it; each correction is named in the report.
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
