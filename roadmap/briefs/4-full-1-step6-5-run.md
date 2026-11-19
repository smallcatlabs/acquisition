# 4-full-1, step 6.5 — the run of its last three parts

A brief for one orchestrating session on Opus. It writes no code, no
brief and no design. It starts sessions, orders their work, reads
each report against its done-when, and stops where this file says.

## Goal
6.5d, 6.5b and 6.5c closed on `main`, in that order of building, each
from a brief written and reviewed first. Then stop: the assessment of
what can be removed (#293) is the owner's to start.

## Inputs
- `roadmap/README.md`, Now and the first Next item: the three parts
  and the issues each carries.
- `roadmap/briefs/4-full-1-step6-design.md`: the ruled view (§2, §3,
  §4, §6, §7). `4-full-1-step6-places.md` and its close
  (`git log --grep='^Step: 4-full-1-step6-places'`): the form a part's
  brief and close take.
- `decisions/process.md` whole; the `cold-review` and `session-close`
  skills. `TEMPLATE.md` for every brief of the run.

## Who does what
- **6.5d, the plan and jobs pages from their documents** (#182, what
  is left of #189): Opus throughout. One session writes the brief
  from the two documents and 6.5a's pattern and has it reviewed cold;
  a fresh session builds it.
- **6.5b, the grouped table** (#244, #269, #194's never-fetched
  view): the designer writes the brief; a fresh Opus session builds.
- **6.5c, an item under its row and the row's ways** (#270, #194,
  #186, #190): the designer answers one question; an Opus session
  writes the brief from that answer, and a fresh one builds.

## The designer
One session on Fable, kept for the whole run because it holds the
design. Opus cannot start a session on a dearer model, so: start a
fresh session parked (read nothing, touch nothing, end the turn),
then `set_session_model` on it with the Fable id from the model
picker; the app asks the owner, once. Each ask is one message that
carries all it needs. Never wake it for status. Its asks, and no
others:
1. **6.5b's brief.** It settles the cut (one build or two), a window
   inside a group, how the URL names an opened place under a query,
   and #294: how a value is identified once it is a column.
2. **6.5c's question,** answered as a decision the 6.5c brief cites:
   the window script needs one row height (§4), and an item opens as
   its card under its row (§3).
3. **The read of 6.5b's plan,** and of any plan that adds a key to a
   protocol or search document.
4. **A look at the shape** when the second stop rule fires.

## Order
One build or gate at a time on the machine: gates share the per-user
real-mode lock, and the three parts change the same web files and
the `refresh_loop` golden. Briefs are documents and overlap a build.
1. Together: 6.5d's brief, then its build; and the designer's asks 1
   and 2.
2. When 6.5d is closed and 6.5b's brief is on `main`: 6.5b's build.
   While it runs: 6.5c's brief.
3. 6.5c's build. Then the report, and stop.

## Rules for every session of the run
Put these in each start prompt, by pointing here.
- Its own worktree. Never build, gate, write or run in the main
  checkout, and never touch its `target/` or its local `main`.
- Never stop, signal or talk to an `acqd` it did not start. Mock
  worlds only (the `mock-session` skill); a bare `acq` opens the
  owner's store.
- `export ACQ_CLONES=/Users/tom/Development/GitHub/smallcatlabs` in a
  shell that runs the gate or a tool.
- The plan first: the brief's "Find first", one screen of plan in
  its notes folder and printed whole, then it waits. You read a plan
  against its brief's done-when and these rules. You do not redesign
  it: where a plan disproves its brief, the brief's author answers.
- Commits on its branch, one concern each; `git fetch`, `git rebase
  origin/main`, a full `tools/gate.sh` at exit 0, then `git push
  origin HEAD:main` alone in its command. No merge commit, no
  `git stash`; `gh` writes one call per command.
- The gate cannot exit 0 while a real-mode `acqd` of the owner's is
  up (`git grep -n 'fn c83_c31_' -- crates/acquisition-cli/tests/world.rs`).
  A run whose one failure is that test is evidence for a local
  commit, said as what it is; the session then ends its turn "ready
  to gate and push" and you release it when the lock is free.
- When a session ends its turn waiting on a gate or a review, start
  one background wait here (`while pgrep -f 'bash tools/gate.sh' >
  /dev/null; do sleep 30; done`, or `'codex exec'`), so its exit wakes
  you; then check the session picked its result up. Once, a session
  was not woken and sat idle for 100 minutes.

## Stop rules
1. **A red gate is read, never rerun to green.** Two failures are
   known and are reported with their output: the lock test above,
   and the browser test `opening_a_far_parent_keeps_its_head_in_view_and_every_tick`,
   which timed out once under the full gate's load. If it fails
   again, file it (`trigger:ci`) before any rerun.
2. **A second review with findings in one place stops the patching.**
   Send the designer both reviews and ask what shape is missing. On
   2026-11-18 five of seven findings sat on one state, and the state
   was removed whole (#291).
3. **A new protocol key, a change to what a query answers, or a
   ruling's words stop for the owner:** a `trigger:ruling` issue, one
   decision each, and a line to him at once.
4. **A third review that still finds a fault which changes what is
   built stops for the owner** (C176).
5. **Nothing here touches the owner's store or the real API.**

## Reading a report
A note is a session's claim. Read its whole report from the notes
folder, then check `origin/main` yourself (`git fetch`, `git log`,
`gh issue view`) before saying a part is closed. Read no diff: the
cold review reads the code, you read the reviews and the close.

## Spending
The owner's allowance is finite. Keep your own context small: a
session's report, never its transcript. Do not wake an idle session
to ask how it is. A candidate for removal met in passing is one
comment on #293, never a task.

## Done when
- Each part is closed by a commit carrying `Step:`, its issues closed
  or said to be left, with CI's two gates named and green on its
  head.
- The road's Now and Next say 6.5 is closed and what is next.
- Your last message to the owner: each part's close, what stopped
  and why, the issues filed, and the candidates added to #293.

## Not in scope
The search form's reshaping, the look (#285), the assessment, #290,
`alpha.2`, and any work the road does not hang on 6.5.

## Report
To the owner, at each part's close in three lines, and at once when
a stop rule fires.
