# The road after the alpha

The plan for the work after `v1.0.0-alpha.1`, opened from a
seven-audit assessment of the project (`STATE-2026-10-19.md`). Read
this file first for any step of it. It holds the owner's rulings for
the road, the rhythm of a step, the order, what the closes left with
the owner and the current step; the closed steps are `LEDGER.md`. A
ruling goes to the registry; a step's working detail is its brief
under `briefs/`; the story of a step is its commits' messages. The
goal is the owner's: a faster, simpler, better Acquisition that is
agent-friendly and agent-forward.

## The owner's rulings for this road

Each in his words, with the commit that landed it.

- 2026-10-19: "I accept all your recommendations exactly as proposed
  with a loopback web UI." Landed: the governance reset (`0704a8f7`),
  one account-error vocabulary (`3bc71a86`), the mock serving his real
  items (`2b275eec`), an id named in words (`briefs/TEMPLATE.md`), the
  corpus held in `acq-mcp` (`6750b4ef`), the loopback web UI
  (`c3537ceb`). Ahead: Tauri, a later wrapper.
- 2026-10-19: "If it would help to add real items to the mock, I give
  permission to use items from any of my stores to whichever agent ends
  up building that part of this effort." (`a987d02f`)
- 2026-10-19, models: "stick to Opus 5.5 as much as possible, with
  Fable called in for complex analysis, synthesis, and design."
  (`a987d02f`)
- 2026-10-24: the #70 step first, then 4-lite before the league; 2g
  and 2h after phase 3 and `alpha.2`; the time seam (4-0) before
  4-full (`54d9f957`).
- 2026-10-25: 4-lite is a renderer, not an app; the packet: C16
  amended, C138 to C140 ("Accept as drafted", "acq web, as drafted",
  "In, via the browser") (`00e440d1`).
- 2026-10-26: "I approve the 1 hour idle default" (C3, `b2cb5449`);
  "your proposed milestones make sense"; "if the first milestone goes
  quickly, I would love to be able to pull some of the work you've
  identified forward rather than sticking to our calendar and idling"
  (the readiness group and "Pull-forward", `0c69ae0e`).
- 2026-10-26, p0's packet: "I have reviewed the questions and approve
  all 4 recommendations." Reviews stay under `reviews/` and a phase
  close deletes nothing; a ledger row is a commit range and a close
  date; an agent-facing design decision is recorded in its commit and
  listed at the close, and enters the registry only when the close
  finds it a property another area must know; budgets print notes;
  the glossary is `GLOSSARY.md` (`f1a29952`).
- 2026-10-28, p1's packet (`briefs/p1-roadmap-sweep.md`): "I accept
  all the packet's recommendations, (a) to (q), as proposed." "Open"
  holds an item only until the next close gives it a home; the ledger
  is `LEDGER.md`, a range and a date per row; the tools' issues are
  `area:process`; C150, the proxy variables; a step's tests ride with
  their fix; the `AGENTS.md` budget of 15,277 bytes stands; no
  frontends review before the league; #91 and #11 join "Pull-forward"
  (`64b7f505`).
- 2026-10-28, the road reordered: "Yes: reorder the road as you
  recommend, the league group first starting at #87, and write it
  into the README and the pull-forward list before a step starts."
  The league group is the milestone's buildable issues the owner's
  run would otherwise meet; the automation and the words follow it
  (the commit carrying this line).
- 2026-10-28, #85's Do 1: "#85: (a), proptest properties in the
  daemon's and the client's own tests, a small count in the gate and
  the full count on CI, as the brief proposes." No registry entry: (a)
  adds no edge and no boundary (the commit carrying this line).
- 2026-10-28, #86's Do 1: (a), a `keyring-macos` job in `release.yml`
  that `draft` needs, so the macOS Keychain round trip is earned on
  the commit a release tags. Taken by the orchestrating session under
  the owner's delegation, which reads: "If the decision is clear cut,
  you may take the recommendation. Use fable otherwise to get a
  practical, simple, elegant recommendation." Clear cut because the
  repository is public, so the job costs no minutes; `release.yml`
  already runs a macOS job `draft` needs; and (a) alone meets the
  owner's words on the issue, "so the pass is earned on every platform
  a release names", with nothing to remember. No registry entry: it
  adds no edge and no boundary (the commit carrying this line).
- 2026-10-29, #128's item 1: "yes to 128". Each step reads the last
  push's CI for both gates on `main` before it starts (`gh run list
  --branch main --workflow "Gate (Linux)" --limit 1`, the same for
  `Gate (Windows)`). The brief names a red run and says whether the
  step fixes it as its first concern or files it; a run still in
  progress is named as such and the step goes on. No registry entry:
  it is process, and adds no edge and no boundary (the commit carrying
  this line).
- 2026-10-29, a CI trigger: "Yes, add a CI trigger label." The label
  `trigger:ci` ("A red CI run fires it", #131 the first to carry it)
  joins the trigger list in `AGENTS.md`, "Where things live". No
  registry entry: it is process, and adds no edge and no boundary (the
  commit carrying this line).
- 2026-10-29, #106's Windows clippy: "Make the Windows-target clippy a
  gate stage", on the measurement in `briefs/tools-hygiene.md` (42 s
  cold, 2 s warm, on this Mac with a stub C toolchain). The stage lints
  the `cfg(windows)` code before a push, which only the Windows gate in
  CI saw. No registry entry: it is process, and adds no edge and no
  boundary (the commit carrying this line).

## Roles

- **The owner** rules on boundaries, runs anything live, captures the
  site, and makes the triage call at each phase boundary.
- **A Fable session** opens and closes a phase and designs the steps
  that need a design; it reads the workers' reports and keeps this
  file true. It does not build.
- **An Opus session** runs a step from its brief. Independent steps may
  run as Opus workers in parallel (`briefs/TEMPLATE.md`, "Rules");
  otherwise steps run one after another in the main tree, because a
  build under a live daemon or a concurrent gate is the artifact
  mismatch C84 names. A measurement runs alone on the machine: no
  other build, gate or measurement while it runs.
- **Codex** reviews, read-only, launched by the session that made the
  commit, on the owner's Codex account:
  `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>."`
  (the `review` subcommand cannot take both a commit and custom
  instructions, so the prompt names the commit; a range is written
  `<first>^..<last>`, so git includes the first). The session that
  commits a worker's step launches its review and waits in the same
  turn until `codex exec` exits: an idle session does not wake when a
  background process does.

## A step

Brief, build, gate, commit, review, close. This is the one statement
of the rhythm; `briefs/TEMPLATE.md` and the session-close skill point
here rather than restate it.

1. **The brief** is written from `briefs/TEMPLATE.md`, after reading
   the last push's CI for both gates on `main`, whose runs it names
   (the ruling of 2026-10-29). A step that
   needs a design has a design brief first, reviewed and ruled by the
   owner before the build brief is written. A worker reads this file,
   the brief, and what the brief names, and nothing else unless the
   work turns it up.
2. **The commits** carry the story: what changed, what was measured,
   what was built otherwise than drawn and why. The build's last
   commit sets a "Current step" that says the review is pending.
3. **The review** runs over each committed step and over each fix
   round that changes behaviour, over its commits as one range. The
   findings are saved verbatim at `roadmap/reviews/<step>-<sha>.md`
   (a fix round's at `<step>-fixes-<sha>.md`), from the final answer's
   first line, the one naming the range, to the end (`codex exec`
   prints the answer twice, the copy to save after `tokens used` and
   its count), are committed on their own before the round that
   answers them (corrected 2026-10-29: #115, #91 and the clock-tests
   step did so), and stay there for good. A round that touches only
   comments and documents gets no review, and nothing is reviewed a third time
   unless the owner asks.
4. **The owner reads** the findings and the proposed fixes before any
   fix is built. A finding is pinned as a test and fixed by one rule,
   never per case; a fault that came back joins `REVIEW-CHECKLIST.md`.
5. **The close** is a commit whose message checks the brief's done-when
   line by line against the tree and answers each item the worker
   raised: taken, filed as an issue, or not taken and why. It lists the
   agent-facing design decisions the step made (a verb, a flag, a tool,
   a query word, the words of an answer), each recorded in the commit
   that built it. It writes the step's row in `LEDGER.md`, gives each
   item the step left open one of the homes "Open" names, and sets
   "Current step". A step that has not earned its close slips rather
   than ships.
6. **A phase close** folds the phase's rows in `LEDGER.md` into one and
   adds the `HISTORY.md` line; it deletes nothing.

The report a worker returns is `briefs/TEMPLATE.md`'s; its last section
is the worker's own, anything it wants raised about the brief, the
plan or the tools, and the close answers every item.

## The phases

| Phase | Steps | Who | Needs | Closes by |
| --- | --- | --- | --- | --- |
| 0. The week's fixes | 0a to 0e | Opus workers | — | closed |
| 1. The governance reset | 1 design, 1b to 1e | Fable, then Opus | — | closed |
| 2. The keystone | closed: 2a to 2g (`LEDGER.md`) · ahead: 2h the queued search and trade issues (#38, #46, #47; #37 once the owner has sat the site) | mixed | phase 1 | with 2h |
| r. League readiness | closed: p0, r1 to r5 (`LEDGER.md`) | Opus workers | 2f, h3 | closed |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · the prune verb (#43) at the league end | owner, Opus | 2e | League 3.30 and alpha.2 |
| 3b. `alpha.2` | regenerate the tables when the named inputs exist: a post-launch `poe1` pin (RePoE lags launch by 0 to 13 days), the owner's four site captures (C79: nobody else may fetch them), the pins and manifests bumped, every generator's `--check` green · then the release | owner captures, Opus | 3 | League 3.30 and alpha.2 |
| 4. The GUI | closed: 4a the 4-lite design · 4b 4-lite, `acq web` · ahead, after `alpha.2`: 4-0 the time seam (`TIME-SEAM.md`) · 4-full on it: the keeper bundle (#1), the subscriber with reconnect, the live loop, the results as site-style cards (#98), the dense table, hardening | Fable, then Opus | 2a, 2b | 4-full, the GUI |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as the assessment's report 6, section 3: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | PoE2 launch |

"Closes by" names the GitHub milestone; its due date lives there.

## Pull-forward

The owner's word of 2026-10-26: when a milestone's steps close early,
the next item starts from this list, in order, without a planning
session; a step starts when the one before it closes. Each has an
issue carrying its evidence and its done-when, and takes a brief from
`briefs/TEMPLATE.md` written by the session that starts it. An item
leaves the list when its step closes or the owner strikes it.

1. The words, after the first seat on the league's build: #11, #16,
   #20, #72, #74, #101, #107, in the order that seat gives them.
2. The test-shape group's first tranche, between league sittings
   (`TEST-SHAPE.md`, the owner's word of 2026-10-29): t1 the stopping
   and retirement rules (#137), t2 the gate trims and the real-time
   waits (#138), t3 the breakers as positive controls (#139), t4 the
   compile cost (#140), in that order, each on the shared checkout.
   The orchestrator proceeds on its own judgment (the owner's word of
   2026-10-29): a ruling in his classes comes to him with a
   recommendation, and each close's retirement list, every `retired
   X; held by Y`, comes to him before the next step starts; #137's
   rulings and the first retirement outside `TEST-SHAPE.md`'s lists
   wait for his word. The machine is his during a league sitting, as
   a window he names.
   Its second tranche is phase 4's: t5 the daemon's door inside 4-0
   (#141), t6 the canonical world and the door corpus before 4-full's
   door (#142), t7 the search's case table (#143).

## Open

An item lives here only until the next close, which gives it an issue
in a milestone, a "Not doing" line with its trigger, a place on the
owner's list below, or a reason in the close message.

The owner's list, his to strike:

- Run `tools/tracer-rung.sh` and `tools/persist-check.sh` at the next
  live run; edited at 2g, not run since.
- The frontends' review, as 2f's of the daemon, store and network:
  after the league (p1's packet, (i)).
- Whether GGG's authorize page echoes `state` on a denial redirect
  (`error=access_denied`): the daemon reads a denial only with the
  flow's state, so a denial without it is answered 400 and the login
  waits to its bound (#68). Read at the next live login by denying
  once; a fact for `docs/design/network-ground-truth.md`.
- Whether the local gate runs the class, defence and forms generators'
  `--check` where the `poe1` clone sits at its pin, so a table body and
  its generator cannot drift between pin bumps: 9.1 s a gate on this
  machine, 8.8 of it the forms table's, 0.35 s without it (#89's
  close).
- `.claude/skills/session-close/SKILL.md`, "Why build order matters":
  whether its release-uplift sentences stand now the gate has its
  `artifacts` stage, a retirement for the next process round.

## Starting a session

1. Read this file, then the current step's brief, then what the brief
   names. `STATE-2026-10-19.md`, `STATE-2026-10-26.md`, `GUI-NOTES.md`,
   `TIME-SEAM.md` and `TEST-SHAPE.md` are dated evidence, read when a brief points into
   them and never rewritten; a sentence an outside review shows wider
   than its report is corrected in place, dated.
2. A session ends with "Current step" true; nothing else here changes
   except at a close.

## Current step

The league group (#87, #90, #68, #103, #63, #53, #94, #96) closed on
2026-10-28 (`LEDGER.md`); #85 closed the same day, and #86, #89 and
#125, #115, #91 and the clock-dependent tests (#127, #76, #111 and
#128) on 2026-10-29. Nothing starts from "Pull-forward" until the
league build's first seat: its one item, the words, waits on that
seat. The owner's pre-league work, in order: the daemon paths his run
will hit, the league milestone's buildables (#74, #107, #112), tools
hygiene. The first, daemon-safety (#113, #116, #110), closed on
2026-10-29 (`LEDGER.md`), and the league milestone's buildables
(#74, #107, #112) and tools hygiene (#117, #118, #126, #114, #106,
#135) the same day: the owner's pre-league plan is done. Next, Phase 3,
the owner's league run, at his word; #70 closes on a week of pushes
without a re-run. Between sittings, the test-shape group's first
tranche ("Pull-forward", item 2), the orchestrator proceeding on its
own judgment as that item says. t1 (#137, `briefs/t1-stopping-rules.md`)
waits at Do 1, its three rulings, for the owner's word. `AGENTS.md` governs; the gate is
`tools/gate.sh`.
