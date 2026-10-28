# The road after the alpha

The plan for the work after `v1.0.0-alpha.1`, opened 2026-10-19 from a
seven-audit assessment of the project (`STATE-2026-10-19.md`, dated
evidence, read when a brief points into it). Read this file first for
any step of it. It holds the order, the current step and what the
closes left open; the closed steps are `LEDGER.md`. A ruling goes to the registry;
a step's working detail is its brief under `briefs/`; the story of a
step is its commits' messages. The goal is the owner's: a faster,
simpler, better Acquisition that is agent-friendly and agent-forward.

## The owner's rulings for this road (2026-10-19)

- "I accept all your recommendations exactly as proposed with a
  loopback web UI." Landed since: the governance reset (phase 1), one
  account-error vocabulary across CLI and MCP (2a), the mock serving
  the owner's real items (2c), the style rule (`briefs/TEMPLATE.md`:
  an id names its rule in words). Still ahead, phase 4's: the corpus
  held in `acq-mcp`, and the GUI's first shape a loopback web UI on the
  shared function layer, Tauri a later wrapper.
- "If it would help to add real items to the mock, I give permission to
  use items from any of my stores to whichever agent ends up building
  that part of this effort."
- Models: "stick to Opus 5.5 as much as possible, with Fable called in
  for complex analysis, synthesis, and design."
- 2026-10-24, from the #70 investigation (`TIME-SEAM.md`): the #70
  step first; then 4-lite, a read-only loopback trade view, built and
  in users' hands before the league; 2g and 2h slide after phase 3 and
  `alpha.2`; the time seam (4-0) lands before 4-full, which carries
  the GUI's live loop.
- 2026-10-25, reviewing the 4a brief before its session: 4-lite is a
  renderer, not an app — every page renders one verb document; the
  refresh pages join 4-lite over the plan loop; the live loop stays
  4-full's. The packet answered the same day: C16 amended, C138 to
  C140 ("Accept as drafted", "acq web, as drafted", "In, via the
  browser").

- 2026-10-26, from the blank-slate assessment (`STATE-2026-10-26.md`,
  dated evidence): "I approve the 1 hour idle default" (C3 amended,
  built by r3); "your proposed milestones make sense" (three GitHub
  milestones hold every open issue: League 3.30 and alpha.2, PoE2
  launch, 4-full); a readiness group of five steps runs before phase
  3, after one process round; and "if the first milestone goes quickly,
  I would love to be able to pull some of the work you've identified
  forward rather than sticking to our calendar and idling" (the
  "Pull-forward" order below). This session was kept to analysis,
  synthesis and setup at the owner's word: the briefs are drafted here,
  the building is the next sessions'.
- 2026-10-26, p0's packet of four process questions
  (`briefs/p0-process.md`, Do 1): "I have reviewed the questions and
  approve all 4 recommendations." (a) Reviews stay under
  `roadmap/reviews/` for good; a ledger row is a commit range and a
  close date; the save-alone, delete, cite-whole and hash-in-ledger
  steps of "A step" go, and a phase close deletes nothing. (b) An
  agent-facing design decision (a verb, a flag, a tool, a query word,
  the words of an answer) is recorded in the commit that builds it and
  listed in the close; it enters the registry only when the close finds
  it a property another area must know; a boundary, an invariant and
  anything touching the real store or the real API stay rulings first,
  the owner's. (c) `AGENTS.md`'s budget and the 800-byte entry limit
  print a note and never fail the gate; the history-marker refusal and
  the cross-cutting count stay failures. (d) The glossary is
  `GLOSSARY.md`, read on demand, named from the entry file in one line.
- 2026-10-28, p1's packet of seventeen questions
  (`briefs/p1-roadmap-sweep.md`, Do 1): "I accept all the packet's
  recommendations, (a) to (q), as proposed." "Open" holds an item only
  until the next close gives it a home (a); the ledger rows are ranges
  and dates in `LEDGER.md` (b, c); the tools' issues are
  `area:process` (d); the proxy variables route the client (C150, e);
  a step's tests ride with their fix (f); the `AGENTS.md` budget of
  15,277 bytes is the note's threshold, no longer temporary (g); no
  bothy rule for a slash in *Pinned:* (h); no frontends review before
  the league (i); three sentences in `REVIEW-PROMPT.md` (j); a bound on
  every observe request, an issue (k); the seat skill's steps stand
  (l); a seat line in `AGENTS.md` (m); two sentences in `TEMPLATE.md`
  (n) and one in "Roles" (o); `STATE-2026-10-19.md` kept (p); #91 and
  #11 join the pull-forward list (q).

## Roles

- **The owner** rules on boundaries, runs anything live, captures the
  site, and makes the triage call at each phase boundary.
- **A Fable session** opens and closes a phase and designs the steps
  that need a design; it reads the workers' reports and keeps this
  file true. It does not build.
- **An Opus session** runs a step from its brief. Independent steps may
  run as Opus workers in parallel, each in its own git worktree with
  its own scratch subdirectory (a fresh worktree builds from nothing);
  otherwise steps run one after another in the main tree, because a
  build under a live daemon or a concurrent gate is the artifact
  mismatch C84 names. A measurement runs alone on the machine: no
  other build, gate or measurement while it runs.
- **Codex** reviews, read-only, launched by the session that made the
  commit, on the owner's Codex account:
  `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>."`
  (the `review` subcommand cannot take both a commit and custom
  instructions, so the prompt names the commit; a range is written
  `<first>^..<last>`, so git includes the first).
- **Every `acq` an agent runs is in a mock world** (`ACQ_PROVIDER=mock`,
  `ACQ_STORE_DIR` to scratch). The real provider is the owner's alone.

## A step

Brief, build, gate, commit, review, close. This is the one statement
of the rhythm; `briefs/TEMPLATE.md` and the session-close skill point
here rather than restate it.

1. **The brief** is written from `briefs/TEMPLATE.md`. A step that
   needs a design has a design brief first, reviewed and ruled by the
   owner before the build brief is written. A worker reads this file,
   the brief, and what the brief names, and nothing else unless the
   work turns it up.
2. **The commits** carry the story: what changed, what was measured,
   what was built otherwise than drawn and why. The build's last
   commit carries the step's ledger line and a "Current step" that
   says the review is pending.
3. **The review** runs over each committed step and over each fix
   round that changes behaviour, over its commits as one range. The
   findings are saved verbatim at `roadmap/reviews/<step>-<sha>.md`
   (a fix round's at `<step>-fixes-<sha>.md`), ride in the next commit,
   and stay under `roadmap/reviews/` for good (the owner's ruling of
   2026-10-26). A round that touches only comments and documents gets
   no review, and nothing is reviewed a third time unless the owner
   asks.
4. **The owner reads** the findings and the proposed fixes before any
   fix is built. A finding is pinned as a test and fixed by one rule,
   never per case; a fault that came back joins `REVIEW-CHECKLIST.md`.
5. **The close** is a commit whose message checks the brief's done-when
   line by line against the tree and answers each item the worker
   raised: taken, filed as an issue, or not taken and why. It lists the
   agent-facing design decisions the step made (a verb, a flag, a tool,
   a query word, the words of an answer), each recorded in the commit
   that built it; one enters the registry only when the close finds it
   a property another area must know (the owner's ruling of
   2026-10-26). It writes the step's ledger line, a commit range and a
   close date, puts what the step left open under "Open" or in an
   issue, and sets "Current step". A step that has not earned its
   close slips rather than ships.
6. **A phase close** folds the phase's ledger lines into one and adds
   the `HISTORY.md` line; it deletes nothing.

The report a worker returns is `briefs/TEMPLATE.md`'s; its last section
is the worker's own, anything it wants raised about the brief, the
plan or the tools, and the close answers every item.

## The phases

| Phase | Steps | Who | Needs | Closes by |
| --- | --- | --- | --- | --- |
| 0. The week's fixes | 0a to 0e | Opus workers | — | closed 2026-10-20 |
| 1. The governance reset | 1 design, 1b to 1e | Fable, then Opus | — | closed 2026-10-20 |
| 2. The keystone | closed: 2a the shared function layer · 2b the tests · 2c the journey test and real-shaped mock items · 2d the daemon and limiter file splits · 2d-oracle the journal oracles over every journaling daemon · 2e the game-diff tool, a new-league rehearsal and the README line · 2f the review of daemon, store and network: a static scan, the owner's rulings and three fix rounds over four PRs, each round reviewed by Codex. Ahead, slid after phase 3 and `alpha.2` by the owner's word of 2026-10-24: 2g the read economy and the MCP refusal and receipt bundle (#9, #10), one bound at the shared layer under C53's three output levels · 2h the queued search and trade issues (#38, #46, #47; #37 once the owner has sat the site), which slips before 2g does | mixed | phase 1 | 2026-12-05 |
| r. League readiness | one process round first, p0 (`briefs/p0-process.md`): the gate as a script, the review choreography and the delegated class ruled by the owner, budgets as notes, a glossary · then r1 the Windows gate decides again (#77; `briefs/r1-windows-gate.md`) · r2 the credential I/O off the shared lock, a design brief first (#81, then #55; `briefs/r2-keyring-design.md`) · r3 the hour idle default and finished trees out of memory (#80; `briefs/r3-idle-and-eviction.md`) · r4 `acq diagnose` (#79; `briefs/r4-diagnose.md`) · r5 the five surface sentences (#78; `briefs/r5-surface.md`) | Opus workers, the owner's rulings in p0 and r2 | 2f, h3 | before 3.30, about a week; what the second week holds is the pull-forward order |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · the prune verb (#43) at the league end | owner, Opus | 2e | 2026-11-20 |
| 3b. `alpha.2` | regenerate the tables when the named inputs exist: a post-launch `poe1` pin (RePoE lags launch by 0 to 13 days), the owner's four site captures (C79: nobody else may fetch them), the pins and manifests bumped, every generator's `--check` green · then the release | owner captures, Opus | 3 | when the inputs arrive, 2026-11-26 at the latest |
| 4. The GUI | closed: 4a the 4-lite design (C16 amended, C138 to C141) · 4b 4-lite, `acq web`, a loopback renderer over the verbs — the search pages (the form in the site's organization, the query box, the table with its art, the counts, one item) and the refresh pages (the selection as a plan, the plan page, the jobs with the rails), every list a group, no subscriber (C44 and C76 one door, C111 spent by response, C142, C143) · ahead, after `alpha.2`: 4-0 the time seam (`TIME-SEAM.md`) · 4-full on it: the keeper bundle (#1), the corpus held in `acq-mcp`, the subscriber with reconnect, the live loop, the results as site-style cards over rows carrying fields, the dense table, hardening | Fable, then Opus | 2a, 2b | 4-lite closed 2026-10-25, before phase 3; 4-full 2026-12-19 |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as the assessment's report 6, section 3: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | 2026-12-11 |

## Pull-forward

The owner's word of 2026-10-26: when a milestone's steps close early,
the next item starts from this list, in order, without a planning
session; a step starts when the one before it closes. Each has an
issue carrying its evidence and its done-when, and takes a brief from
`briefs/TEMPLATE.md` written by the session that starts it. An item
leaves the list when its step closes or the owner strikes it.

1. 2g, the read economy and the refusal bundle (#9, #10).
2. #85, #86, #87, #89: the standing automation and the hygiene from the
   assessment's R5, each a short step on any idle day.

## Open

An item lives here only until the next close, which gives it an issue
in a milestone, a "Not doing" line with its trigger, a place on the
owner's list below, or a reason in the close message.

The owner's list, his to strike:

- Run `tools/tracer-rung.sh` and `tools/persist-check.sh` at the next
  live run; edited at 2g, not run since.
- The frontends' review, as 2f's of the daemon, store and network:
  after the league (p1's packet, (i)).
- `.claude/skills/session-close/SKILL.md`, "Why build order matters":
  whether its release-uplift sentences stand now the gate has its
  `artifacts` stage, a retirement for the next process round.

## Starting a session

1. Read this file, then the current step's brief, then what the brief
   names. `STATE-2026-10-19.md`, `STATE-2026-10-26.md`, `GUI-NOTES.md`
   and `TIME-SEAM.md` are dated evidence, read when a brief points into
   them and never rewritten; a sentence an outside review shows wider
   than its report is corrected in place, dated.
2. A session ends with "Current step" true and its step's ledger line
   written; nothing else here changes except at a close.

## Current step

2g is closed: #9 and #10 closed 2026-10-27 (rounds 1 to 6 of
`briefs/2g-build.md`); round 7 (the host experiment) and round 8 (the
replay of the three calls the brief selected, the step's measurement)
closed 2026-10-28, the structured copy gone from every MCP emitter.
Next from "Pull-forward": #85, #86, #87, #89, the standing automation
and the hygiene, each a short step on any idle day. Phase 3, the
owner's league run, follows at his word; #70 closes on a week of
pushes without a re-run. The assessment these rest on is
`STATE-2026-10-26.md`; the issues carry the evidence per step.
`AGENTS.md` governs; the gate is `tools/gate.sh`.
