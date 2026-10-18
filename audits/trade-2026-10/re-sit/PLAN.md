# The re-sit — the plan

Step 6 of `brainstorming-notes/38-the-answer-an-agent-reads.md` (note
38), run by the seat skill (`.claude/skills/seat/SKILL.md`): the trade
seat sat again after steps 1 to 5 changed what an agent meets. This
file is the brief the orchestrating session runs under; it is deleted
at the close and cited at its commit. The record is this directory's
`README.md` (the runs, their call counts, one row per fault with its
fate, the counts step 6 owes) and note 38's step 6 row.

Written 2026-10-18 by the session that led steps 1 to 5 (Fable), which
does not run the seat: a fresh session orchestrates, so that the one
with the builds in its head is not the one reading the runners' reports
for them. The owner's decisions that bind here, verbatim: "keep
simplicity, usability, and elegance as guiding lights" (2026-10-18); of
the trade seat's shape, "I do like the idea of exercising both the mcp
and cli", "i like the idea of a sonnet round after an opus round",
"I'm also ok doing all of this in serial", "Let's use GERWARIC"
(2026-10-17); every step's rulings "I accept all 6. Go ahead."
(2026-10-18, note 38).

## What changed since the trade seat, and what this seat asks

The trade seat's record is `../seat/README.md` (closed `b24fc48b`). Its
runs placed these faults first: the never fetched could be neither
listed nor sized against a question; the MCP's default answer was over
the harness's cut; the terms block cost a re-read in three runs; "would
a buyer see it" was inferred at four calls because no field said a tab
is public; ids and jargon stayed unreadable. Steps 1 to 5 built against
each (`git log c0be5c5a..`; note 38's sequence table has each step's
range). This seat does not say any of that to a runner. It asks the same
person's questions again, and its record answers, per run:

1. **The terms block.** How many runs re-read an answer for its terms
   (a second call for the same answer, or the debrief's "read twice"
   naming it). The trade seat counted three; step 1 moved the total and
   the rows above the terms.
2. **Four lines left as built**, each judged by what readers did —
   skipped, misread, or served — never by the orchestrator's taste:
   the CLI total's caveat that repeats the scope line's never-fetched
   count; the scope line's length (about 190 characters on this store);
   the words "the site shows public listings" (the trade header, the
   search tool's `query`) not naming `tab.public`; a row sorted by a
   total showing each contributing line's text but not its weight.
3. **The person's question split.** The trade seat asked "whether the
   site would have shown me the same things" as one question and every
   runner answered a mix. Here it is two: is this search the site's
   (fidelity, the report's business), and which of these could a buyer
   see (a tab's `public`, a field since step 4). The record says, per
   run, how each half was answered and at what cost in calls.
4. **A task whose honest answer turns on a never-fetched tab.** On this
   store 437 of the 502 unique-stash tabs and 2,489 of the 2,550 map
   tabs were never fetched (`tabs --json`, 2026-10-18). One task asks
   for a unique the person is sure they own and the store copy does not
   hold (the orchestrator picks one, confirms it absent with a search in
   a scratch world, and records which in the handoff and nowhere a
   runner reads). The right answer is that the tool cannot say and why;
   the record says which runs said so, from what, at what cost.
5. **A difference between the two surfaces or the two models** on any
   task is a finding of its own kind, as before.

## Roles, and what each reads

- **The orchestrator** — one fresh session in the repository, reading
  as any session does (`AGENTS.md`) and for this work: this plan, note
  38, `../seat/README.md` and its briefs at `3e84db17`
  (`git show 3e84db17:audits/trade-2026-10/seat/BRIEF-cli.md`,
  `BRIEF-mcp.md`), `TASKS.md` at `157f7078`, the debrief questions at
  `538022d6`, and the seat skill. It writes the briefs, builds the
  worlds, launches the runners one at a time, reproduces what they
  claim, runs the fix round, and closes this record. It never runs a
  task itself, and no brief it writes says what it expects to be found.
- **The runners** — four fresh headless sessions, each started in its
  world by `.claude/skills/seat/launch.sh`, never a subagent (the trade
  seat's probe). Opus 5.5 then Sonnet 5.5, CLI then MCP within each,
  each launched only after the previous debrief is committed. A runner
  reads its brief, its tasks and (CLI) the tour copy, and nothing else;
  it is never continued into a rerun.
- **The external review** (Astra's Codex pass) reads the briefs before
  any run (review 1: an expectation that leaks, a task the build
  refuses, a world the door cannot make) and the fix round after
  (review 2). The owner relays each; the orchestrator fixes findings by
  one rule, pinned by tests.

## Inputs

- **The store:** the pristine copy of the owner's `GERWARIC` facts and
  intent files at `~/seat-worlds/trade-2026-10/pristine/`, made
  2026-10-17 with the daemon stopped (C35); nobody runs in it. Every
  run's world is `seat.py setup --store` from it, under
  `~/seat-worlds/trade-2026-10/re-sit/<run>/`.
- **The binaries:** `cargo build --release -p acquisition-cli -p
  acquisition-mcp` at the commit the briefs are committed at, copied
  into each world's `bin/`, their sha256 in the record. No build while
  a runner runs (C84).
- **The tour:** README's "The tour" as it stands at that commit, copied
  into each CLI world as `TOUR.md`.
- **The searches:** the six URLs of `TASKS.md` at `157f7078`, pasted
  as given. Before the briefs are committed the orchestrator decodes
  each with `--print-request` in a scratch world made from the pristine
  store and records what each translates to and with what remainder,
  in its handoff; a URL the build refuses is reported to the owner,
  never swapped.

## The tasks

`TASKS.md` is the trade seat's at `157f7078` with these changes and no
other:

- The person's opening paragraph ends: "For each one, tell me what of
  mine it finds and where it is. Then two things apart: whether the
  tool ran the same search the site would have, and which of the things
  it found a buyer on the site could actually see right now."
- After T6 (maps), a new task, in the person's words: "I'm certain I
  have a <unique> in my unique stash somewhere. Find it for me, or tell
  me straight why you can't." The unique is one the store copy does not
  hold; the orchestrator's handoff names it.
- T7 (what I'd take) stays last and renumbers to T8.

Everything else — T0 orient, the six searches and their follow-ups —
is unchanged, so the runs read against the trade seat's.

## The debrief

Asked after the report, through the door, each answer citing call
numbers where it can and marked "impression" where it cannot:

1. Where did the tool stop you, and what did you do instead?
2. Which outputs did you have to read twice, and what made them hard?
3. What did you want to ask a person, and when?
4. Which of the tool's own words misled you, and what did you take
   them to mean?
5. Which answer would you retract, or not bet on, and why?
6. What would you change first, and why that first?
7. What did you never find out?
8. What should we have asked you that we did not, and what would you
   add to any answer above?

## Steps, each with its acceptance

1. **The briefs.** `TASKS.md`, `BRIEF-cli.md`, `BRIEF-mcp.md` and
   `DEBRIEF-QUESTIONS.md` in this directory, each from the trade seat's
   at `3e84db17` and `157f7078` with the changes above (the task
   numbers `T0` to `T8`, the headings "for each of T1 to T8", the
   acceptance's "nine tasks"). *Acceptance:* committed; review 1 read
   and its findings fixed before any build or run.
2. **The build and the worlds.** The release binaries once, the sha256
   of each recorded; a world per run from the pristine store, holding
   its brief as `BRIEF.md`, `TASKS.md`, the tour copy (CLI), its door,
   `bin/` and the store; nothing else. *Acceptance:* each door's setup
   reports its world.
3. **The runs,** serial: cli-opus, mcp-opus, cli-sonnet, mcp-sonnet.
   For each: the runner's prompt names the brief's path and repeats
   the prohibitions, nothing of what is expected; its debrief resumes
   the same session; `calls.csv`, `REPORT.md` and `DEBRIEF.md` copied
   here as `calls-<run>.csv`, `REPORT-<run>.md`, `DEBRIEF-<run>.md` and
   committed before the next launch. *Acceptance:* the report cites
   calls by number; the debrief answers all eight.
4. **Reproduction.** Every claimed fault reproduced in a scratch mock
   world before it is believed; the record says how many calls were
   read against each report. A fault that is a ruling's consequence
   goes to the owner. The five counts of "what this seat asks" are
   taken from the reports, debriefs and `calls.csv`, each with the
   calls it rests on.
5. **The fix round.** One commit per fault, each held by a test named
   for the fault (`crates/acquisition-cli/tests/seat_faults.rs` or the
   MCP's `search_tools.rs`); a fault that waits is a park with its
   trigger marked fired; the four left-over lines are patched only
   where the runs showed a reader skip or misread them, and left where
   they served; references regenerated after help changes; the gate
   run bare before each commit, the commit inside an `if` on every
   stage's own status (never a `grep` of a log: BSD `grep -qv` passed a
   red stage on 2026-10-18).
6. **Reruns**, only where the round changed what an agent meets: the
   first run's files renamed `-1`, its world moved aside, the release
   built again, a fresh world and a fresh runner of the model that met
   the fault.
7. **Review 2**, outside; its findings fixed by one rule.
8. **Close.** This directory's `README.md`: the runs with their model,
   session, call counts and files; the five counts, each per run with
   its calls; one row per fault with the runs that met it and its fate;
   the debriefs' answers routed. Note 38's step 6 row filled in one
   sentence with the range. The briefs and this plan `git rm`'d, cited
   at their commits; `seat.py <WORLD> <BIN> teardown` for each world.
   Step 7 — the trade slice's close and note 38's deletion — is the
   leading session's, not this one's; the handoff below is what it
   reads.

## Rules

- Serial throughout; no build while a runner runs; one runner at a
  time.
- The runner is never told what to find and never continued.
- A report's counts are the runner's; the record says so.
- Nothing reaches GGG: the doors set the mock provider, the world's
  store and no-spawn on every command; this seat needs no daemon. The
  trade site is a governed surface (C79): nothing fetches it.
- A cut is the door's: a fault that rests on a `head` row is read
  against the whole output in the world's `calls/` first.
- A brief's expectation leak found at review 1 is fixed before the run.

## Handoff

The orchestrator's last message to the owner, and the README's
closing section: the sha256 of each binary; what each URL translated
to, with its remainder; the unique chosen for the never-fetched task
and the search that showed it absent; per run and model, the calls
logged and read, the faults claimed, reproduced and their fates; the
five counts with the calls they rest on; the debrief answers that
became a finding or a park; and what this plan failed to tell it.
