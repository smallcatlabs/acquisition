# <step id> — <title>

A brief carries rules and commands, never their output
(`decisions/process.md`, C174, what a brief carries). Its headings are
below, each saying what its section holds. A small fix needs no brief:
its issue states the goal and the done-when, and "Rules" and "Report"
here apply to it unchanged. A brief's own Rules section says
`TEMPLATE.md` and lists only what differs for the step, and its Report
section says `TEMPLATE.md`: a copied rule drifts. A brief over 8,000
bytes is carrying the tree or is two steps.

## Goal
One paragraph: what is true when the step is done, and why it matters.

## Inputs
What is fixed before the work starts, each named concretely: the
rulings the step rests on (each id glossed in a few words), the files
to read with a few words on what to take from each, a measurement
beside the command that produced it, a capture, the issue. A step with
no fixed input says so. A ruling the step needs and does not have is
brought to the owner before the brief is committed, and a drafted
registry line passes `python3 -I tools/bothy.py check` first.

The last push's CI: each gate's run on `main` (`gh run list --branch
main --workflow "Gate (Linux)" --limit 1`, the same for `Gate
(Windows)`), by its id and conclusion. A red run is this step's first
concern or is filed, and the brief says which; a run still in progress
is named, and the step goes on.

## Decisions
The choices that needed thought, each with its reason and what it
rules out. Nothing the builder would decide the same way unprompted.
An agent-facing design choice (a verb, a flag, a tool, a query word,
the words of an answer) is the step's own, recorded in the commit
that builds it.

## Find first
The commands the builder runs before changing anything, each with
what it is for: the tests that assert on a document the step changes,
the fakes that answer a protocol exchange it changes, the callers of
what it moves, the instances of a class it sweeps. The brief gives the
command and never its hits; the builder's report gives the hits. A
claim about a file's history also searches `spikes/rust-playground/`,
the tree before the root swap.

## Done when
Checkable sentences, each from the commit alone, its tree or its
message: the reviewer reads the commit, never the report. The gate
green on the step's own tree is always one of them, and a step never
needs what a later step delivers. A number in a line is one the build
measures, by a command the line names; a step that compares two timed
runs names its tolerance before the first. Execution evidence (the
gate's exit status per stage, a test count) lives in the report, and a
commit message carries what the commit measured, never the gate's
table.

A fix round's done-when is its own: each finding of the review it
answers fixed by one rule and pinned, or named as not taken and why,
and the gate green on the round's tree.

## Not in scope
What the builder will be tempted to do and must not, with where it
lives instead.

## Rules
`AGENTS.md`'s hard rules, `decisions/process.md` and the session-close
skill apply. What a builder adds:
- Your own scratch subdirectory, named at launch. Kill only the
  processes you started.
- Touch nothing in the tree while the gate runs: its `tree` stage
  fails on any edit.
- A commit message names this brief or its issue, and a count in it,
  like a size, is measured after the last edit.
- A test rides in the commit of the fix it pins, and a breaker case in
  the commit of the check it stages.
- Each concern may commit after its own targeted tests; the one full
  gate runs on the step's tree before the last commit.
- When doc comments change, run `tools/gate.sh --only doc` before the
  last commit: CI runs that stage and the full gate does not.
- Rewrite your own unpushed commits with `git commit-tree -S` (signed,
  as every commit here is), then `git update-ref <ref> <new> <old>`,
  which refuses if the branch moved; never a reset, and each rewritten
  commit checked to compile.
- A cut or a move of prose names the committed home of each reason it
  removes.
- A draft the brief gives (a registry line, a wording) is taken as
  written unless the tree disproves a sentence of it; the report names
  each correction.
- An id names its rule in words the first time a brief, a report or a
  commit message uses it ("C84, the artifact mismatch"): the reader
  outside the area has no registry open.

## Report
Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What each "Find first" command found.
3. What the gate said, as exit statuses per stage, and the
   measurements the brief asked for.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is
   wrong, even where you judged it unlikely to matter. A sentence each.
