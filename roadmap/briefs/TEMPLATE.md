# <step id> — <title>

Every brief has these headings. A worker reads `roadmap/README.md`,
then this brief, then what it names, and nothing else unless the work
turns it up.

## Goal
One paragraph: what is true when the step is done, and why it matters.

## Read
The files, sections and decision ids this step rests on, each with a
few words on what to take from it. The brief's writer checks every
citation, and every test the brief relies on, against the tree
before the brief is committed: phase 0's briefs cited a ruling as
built that was parked, named a test that did not exist, and carried
a count wider than true, and each cost a review finding.

## Input
What is fixed before the work starts: a measurement, a capture, a
ruling, and the failing inputs the step's tests will stage, each
named concretely. A step with no fixed input says so.

## Do
Numbered, concrete, in order. A ruling the step needs and does not have
is the first item, brought to the owner before the code.

## Done when
Checkable sentences, each from the commit alone: its tree or its
message. The report may restate a fact and never holds one alone,
because the reviewer reads the commit and not the report. The gate
green is always one of them, on the step's own tree: a step never
needs what a later step delivers.

## Not in scope
What a worker will be tempted to do and must not, with where it lives
instead.

## Rules
- Every `acq` runs in a mock world: `ACQ_PROVIDER=mock` and
  `ACQ_STORE_DIR` under your scratch. Never the real provider.
- Your own worktree and your own scratch subdirectory, named in the
  brief's launch; never another worker's.
- The gate (`AGENTS.md`, "Quality gate") before the commit; the commit
  gated on each stage's own exit status, never on a filtered grep. Never
  run the drivers while the gate runs.
- One commit per concern; the story in the message; a size stated in a
  message is measured after the last edit.
- A finding is a test named for the fault, or a line in the report;
  never a new document.
- After the commit, launch the review (`roadmap/README.md`, "Roles and
  rhythm"), save its findings verbatim at
  `roadmap/reviews/<step>-<sha>.md` and quote them in your report; do
  not fix them before the owner has read them.

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
   each; the session that reads it answers every item in the ledger
   (taken, filed, or not taken and why).
