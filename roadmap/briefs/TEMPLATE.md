# <step id> — <title>

Every brief has these headings. Goal to Not in scope tell the brief's
writer what each holds; Rules bind the worker, and their last list the
brief's writer. A brief's Rules section says `TEMPLATE.md` and lists only
what differs for the step; its Report section says `TEMPLATE.md`. A
copied rule drifts (2d-oracle).

## Goal
One paragraph: what is true when the step is done, and why it matters.

## Read
The files, sections and decision ids this step rests on, each with a
few words on what to take from it. Before the brief is committed, its
writer checks against the tree every citation, every test it relies
on, and each claim it relays from an issue (phase 0; map-before-seam
on #65). A brief that changes the words or shape of a document a
frontend prints names every test that asserts on it (2a), and one that
changes a protocol exchange names the test fakes that answer it
(stop-means-gone), each found by grepping.

## Input
What is fixed before the work starts: a measurement, a capture, a
ruling, and the failing inputs the step's tests will stage, each
named concretely. A step with no fixed input says so. A step that
sweeps for a class runs the sweep before the brief is committed, lists
the instances found, and names its measure of the class as a command
(clock-tests; map-before-seam). A drafted registry line (a ruling, a
"Not doing" line) passes `python3 -I tools/bothy.py check` before the
brief is committed (league-buildables). A claim about a file's history
also searches `spikes/rust-playground/`, the tree before the root swap
(#135).

The last push's CI (the ruling of 2026-10-29): each gate's run on
`main` (`gh run list --branch main --workflow "Gate (Linux)" --limit
1`, the same for `Gate (Windows)`), its commit, id and conclusion. A
red run is this step's first concern or is filed, and the brief says
which and where; a run still in progress is named, and the step goes
on.

## Do
Numbered, concrete, in order. A ruling the step needs and does not have
is the first item, brought to the owner before the code; an
agent-facing design decision is the step's own (`AGENTS.md`, "Hard
rules").

## Done when
Checkable sentences, each from the commit alone, its tree or its
message: the reviewer reads the commit, not the report, which may
restate a fact and never holds one alone. The gate green is always one
of them, on the step's own tree: a step never needs what a later step
delivers. A number or a literal in a line is measured on the tree the
brief is written against, or the line says the build measures it (2e).
A measurement brief names the production edit and the test edit apart
(t4), and a step comparing gate-length runs interleaves them A/B or
names its tolerance before the first (t4: 22 s between two runs).

A fix round's done-when is its own: each finding of the review it
answers fixed by one rule and pinned, or named as not taken and why,
and the gate green on the round's tree. Execution evidence (the gate's
exit status per stage, the test count, a measurement) lives in the
worker's report, which the orchestrating session holds against the
review; a commit message carries what it measured, never the gate
table.

## Not in scope
What a worker will be tempted to do and must not, with where it lives
instead.

## Rules
`AGENTS.md`'s hard rules and its session-close skill apply, and the
rhythm is `roadmap/README.md`, "A step". What a worker adds:
- Your own worktree and your own scratch subdirectory, named in the
  brief's launch; never another worker's. Kill only the processes you
  started.
- Touch nothing in the tree while the gate runs: its `tree` stage fails
  on any edit, and an edit to a source it built fails `artifacts` too.
- A commit message names this brief's file; a count in it, like a
  size, is measured after the last edit.
- A test rides in the commit of the fix it pins, and a breaker case in
  the commit of the check it stages (1e).
- Each concern may commit after its own targeted tests; the one full
  gate runs on the step's tree before the "built" commit.
- Rewrite your own unpushed commits with `git commit-tree -S` (signed,
  as every commit here is), then `git update-ref <ref> <new> <old>`, which refuses if the branch moved;
  never a reset, the working tree untouched, and each rewritten commit
  checked to compile (t4).
- A cut or a move of prose names the committed home of each reason it
  removes, and what to do where the tree has moved since the reason
  was written (map-before-seam).
- Paused tokio time (`start_paused`) needs tokio's `test-util`, which
  splits the feature graph the `features` stage compares; a forced
  interleaving uses injected futures instead (stop-means-gone).
- A draft the brief names is taken verbatim unless the tree disproves
  a sentence of it; each correction is named in the report. A brief
  sentence a fix round corrects in place is marked "Settled in the
  round".
- A JSON pin may add `--json` to a printed command it runs.
- No test asserts a wall-clock bound a slow machine can miss: a
  deadline is pinned by forcing its expiry, and a lifecycle that waits
  across processes is designed as a state machine over an injected
  clock before it is built (C137, the deadline's contract is its
  expiry behaviour).
- An id names its rule in words the first time a brief, a report, a
  commit message or a ledger line uses it ("C84, the artifact
  mismatch"): the reader outside the area has no registry open.

What the brief's writer adds:
- A brief that wants a test seen failing first asks for it in the
  report, not the history.
- A brief says whether each rule binds the session's own work or the
  thing it builds (tools hygiene).

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
   each.
