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
  their fix; the `AGENTS.md` budget of 15,277 bytes stands (raised 2026-10-29, below); no
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
- 2026-10-29, #137's three rulings (`briefs/t1-stopping-rules.md`,
  Do 1): "Accept all three." The stopping rule is C153 and the
  retirement rule C154, cross-cutting in `AGENTS.md`. The review rule
  binds the test-shape group (#137 to #143), no registry entry: a
  round that changes only tests or documents gets its retirement lines
  and the gate; a round that changes non-test code, anything under
  `tools/`, the gate's stage list, a workflow, `.config/nextest.toml`
  or the shared harness's isolation list gets the review as "A step"
  says; revisited at the first tranche's close. The `AGENTS.md` budget
  raised for the group's duration: "I give you permission to raise the
  AGENTS.md budget as needed for the duration of this work"; taken as
  16,300 bytes and, under the same word, the cross-cutting count from
  16 to 18 (the commit carrying this line).
- 2026-10-29, #138's gate (`briefs/t2-gate-trims.md`, Do 1): "Accept
  as recommended." The doc stage leaves the full gate's list for a
  parallel job in `gate-linux.yml`, `--only doc` kept; `fixtures`
  retired with the unset kept; `artifacts` retired, held by
  `features` and the binaries' feature pins, with the Build order
  rule's words in `AGENTS.md` following it; `--all-features` kept,
  since it costs nothing and is the flag wanted the day a crate
  declares a feature. No registry entry: it is process (the commit
  carrying this line).
- 2026-10-29, the group's pace: "i accept your proposed change." A
  close whose retirements are all on `TEST-SHAPE.md`'s lists, each
  with a named holder, proceeds to the next step; its retirement list
  is in the close for the owner's later read. A retirement outside the
  lists, or one without a holder, waits for his word (the commit
  carrying this line).
- 2026-10-30, #140's incremental build (`briefs/t4-compile-cost.md`,
  Do 2): "Yes, adopt." `[profile.dev]` sets `incremental = true`, with
  occasional manual cleanups (`cargo clean` after each compiler
  upgrade) and no automated sweep in the gate. The full gate after one
  store touch 62.90 and 62.89 s wall on, against 145.85 and 167.91 s
  off; the target directory 27G to 34G, 5.0G of it
  `target/debug/incremental`. CI builds without it: `Swatinem/rust-cache`
  sets `CARGO_INCREMENTAL=0` (the commit carrying this line).
- 2026-10-30, #140's three environment binaries (Do 5): "don't fold."
  `daemon_stalled`, `watch_recovery` and `plan_loop` stay their own
  binaries, keeping the `cargo test` fallback's isolation. Their
  compile 43.5 CPU-s; touch and the workspace's nextest run 139.74 and
  140.57 s wall unfolded, 138.15 and 135.49 folded, 1391 tests either
  way (the commit carrying this line).
- 2026-10-30, the group's review rule at the first tranche's close:
  "I agree to keep the rule." Unchanged for t5 to t7. Each reviewed
  build found a real fault (t1, t2, t4); the unreviewed rounds (t2's
  last comment fix, the closes) let through none found later (the
  commit carrying this line).
- 2026-10-30, the words ahead of the league seat: "let's pull these
  forward." #20, #11, #101 and #16 leave "Pull-forward" item 1's wait
  on the league build's first seat and run now as step `words`, in that
  order; the seat then checks the fixes, and what it finds new is new
  rows (the commit carrying this line).
- 2026-10-30, #101's `--if-revision 0`, delegated: "I'd prefer an
  agentic ruling", since it bears on agents more than on humans. Fable
  recommends, the `words` step takes and builds it, and the report
  lists it for his later read; a registry bullet, if one is needed,
  comes to him first (`briefs/words.md`, Do 1; the commit carrying this
  line).
- 2026-10-31, the road after `words`: "let's do this. i want to
  address 146 immediately, and after that I want to start work on the
  time seam." #146 (a declared shell for printed commands, `argv`
  beside them in JSON) runs now, C156 amended first; then 4-0, the
  time seam, beside the league rather than after `alpha.2`. 4-0's
  machines are local lifecycles that need no live evidence
  (`TIME-SEAM.md` §5, corrected that day); what the league finds
  "slow or stuck" joins them as added states. 4-full stays after
  `alpha.2` (the commit carrying this line).
- 2026-10-31, #146's Do 1 (`briefs/146-declared-shell.md`): "Accept
  as drafted." C156 amended: `ACQ_SHELL` declares `posix`, `pwsh`,
  `powershell5` or `cmd` over the detection that stays the default,
  an unknown value refused; a JSON field holding one printed command
  stands beside its words or its request; the refusal names the
  declaration. The variable is the one knob, no `--shell`; the daemon
  quotes for the `ACQ_SHELL` it was started with (the commit carrying
  this line).
- 2026-10-31, 4-0's design (`briefs/4-0-design.md` §7): "Accept as
  drafted." C157 (a lifecycle that waits is a value: time enters as a
  named deadline's event and leaves as a request to arm one, the
  socket, the process, the file system and the clock in a driver; a
  machine pinned by event scripts, a driver by real-wiring tests
  asserting no duration); C137's last sentence amended to name it;
  `decisions/store.md` gains "Not doing", the injected store clock
  refused (SQLite is its clock, the knob its seam); the store's fixed
  waits under one knob, `ACQ_STORE_WAIT`, beside `ACQ_ANNOTATION_WAIT`
  (#148); the machines on #147, then t5 (#141) after a spike; step 1
  during sittings, step 2 between them, the spike and t5 at his word
  (the commit carrying this line).
- 2026-10-31, t5's design (`briefs/t5-design.md` §5): "Accept as
  drafted with your recommendations. The reasoning makes sense."
  C158 (a daemon scenario crosses the daemon's frontend boundary
  only, and its time moves by its own hand or when nothing it started
  can run; a test that needs another boundary says which and stays a
  component test); `decisions/daemon.md` gains two "Not doing" lines,
  a request timeout's expiry virtualised and a classification field
  in `Outcome`, each until its trigger; the client's harness keeps
  its exempt-route copy under a check. Landed: the ruling commit
  after this design's third cut (`ef7e4b3f`).
- 2026-11-01, describe's default shape (`briefs/describe-slices.md`,
  the one question brought to him): "Yes, the index." Bare `describe`
  is the index on both surfaces and in both formats (the MCP's
  `describe {}`, the CLI's `--describe` in text and under `--json`),
  the whole kept under `all`, one word away; C53 amended to carry it,
  the additive clause read as protecting shapes a program can use
  (the commit carrying this line).
- 2026-11-01, the time-seam retrospective's questions
  (`reviews/time-seam-retrospective.md` §11) and the stuck-verdict
  brief's ruling (`briefs/151-stuck-verdict.md`, Do 1): "yes to all."
  C160 entered (every wait in the daemon sleeps through its one
  `Clock`, and a door scenario that can never proceed fails at the park
  where nothing it started can run), the request-timeout "Not doing"
  line amended to say it is structural; the criterion that draws a
  machine's edge folded into C157 before the phase close deletes the
  design brief; #70 closed on the class evidence with the VM caveat;
  #147 closed with the re-judge floor's trigger named (4-full's first
  long-lived subscriber); a `Boundary` value at the component
  constructors meets #152's "names its boundary"; and the grant: the
  orchestrating session re-cuts the stuck-verdict brief under cold
  reviews, launches an Opus worker on the cut that reviews clean,
  commits on main, answers the build's reviews by one rule each, every
  narrowing dated and every agent-facing choice listed for his later
  read, and pushes at the step's close (the commit carrying this line).
- 2026-11-01, the road after the retrospective: "Let's use opus 5.5
  for these unless you see a strong argument for the need for Fable or
  for extending this session. Let's do this work in series, not
  parallel, so keep things simple." The residue step (Opus), then the
  transport seam's design brief (Fable's, by the roles: it carries a
  ruling on invariant 1's structural form, the choke point's private
  client), its build (Opus), then the login flow as a machine under
  C157 (Opus); the choke point's admission loop as a machine parked
  under a trigger (`decisions/daemon.md`, "Not doing"). One step at a
  time in the daemon (the commit carrying this line).
- 2026-11-01, the transport seam's design (`briefs/transport-seam-design.md`
  §7, cut six times under five cold reviews): "Accept as drafted."
  C161 entered (every request the daemon sends to the provider crosses
  one transport seam behind the choke point, which prepares it, owns
  its deadline on the daemon's clock and classifies the answer; a door
  scenario's transport is in-process), the "Not doing" line on a
  request timeout's expiry removed with it, C160 amended twice, each
  before the step that depends on it; the connect timeout as (a), one
  deadline on the clock over the whole exchange; the timing his, steps
  1 and 3 before Phase 3 starts with the rung after, or both after the
  run (the commit carrying this line).
- 2026-11-01, #160 (check 13 held `reqwest` to one file by spelling, and
  each review round found another spelling): "(b) Own crate." The one
  production transport and its private client move to a crate of their
  own, `acquisition-transport`, which alone links an HTTP client and
  which only the daemon links; check 5's edges hold it and check 13
  retires, held by them. C1 amended to name the crate and its edges, a
  row in `AGENTS.md`'s crate table, and the file's budget raised to
  16,600 bytes ("Raise to 16,600"). Said with the packet and taken: the
  production transport's constructor is then that crate's public API,
  so "only the choke point builds it" is held by review, not by the
  module tree (`baf625c9`). His words in the orchestrating session the
  same day: "The crate, by the graph." The round's build found the
  crate's public API could re-export reqwest unseen by the graph; held
  by review beside `Production`'s construction, a `REVIEW-CHECKLIST.md`
  row, the orchestrating session's call, listed at step 1's close.

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
| 4. The GUI | closed: 4a the 4-lite design · 4b 4-lite, `acq web` · ahead: 4-0 the time seam (`TIME-SEAM.md`), beside the league (the ruling of 2026-10-31) · after `alpha.2`, 4-full on it: the keeper bundle (#1), the subscriber with reconnect, the live loop, the results as site-style cards (#98), the dense table, hardening | Fable, then Opus | 2a, 2b | 4-full, the GUI |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as the assessment's report 6, section 3: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | PoE2 launch |

"Closes by" names the GitHub milestone; its due date lives there.

## Pull-forward

The owner's word of 2026-10-26: when a milestone's steps close early,
the next item starts from this list, in order, without a planning
session; a step starts when the one before it closes. Each has an
issue carrying its evidence and its done-when, and takes a brief from
`briefs/TEMPLATE.md` written by the session that starts it. An item
leaves the list when its step closes or the owner strikes it.

1. 4-0, the time seam, after #146 (the ruling of 2026-10-31): a design session
   with Fable over `TIME-SEAM.md` §4 and §6 and `TEST-SHAPE.md` §4,
   ruled by the owner, then the build; t5 (#141) rides inside it. The
   machine is the owner's during a league sitting. The design is
   `briefs/4-0-design.md`, ruled 2026-10-31; the build is #147 (its
   three steps), then #141, then #148.
2. The test-shape group's first tranche, between league sittings
   (`TEST-SHAPE.md`, the owner's word of 2026-10-29): t1 the stopping
   and retirement rules (#137), t2 the gate trims and the real-time
   waits (#138), t4 the compile cost (#140), in that order, each on
   the shared checkout; t3, the breakers as positive controls (#139),
   closed undone at the owner's word of 2026-10-30 ("I agree"): the
   breakers job adds no wall time to a push, and the suites are the
   checks' own tests, so the rule that each fixed check gains a
   breaker case stands.
   The orchestrator proceeds on its own judgment (the owner's word of
   2026-10-29): a ruling in his classes comes to him with a
   recommendation; a close whose retirements, every `retired X; held
   by Y`, are all on `TEST-SHAPE.md`'s lists, each with a named holder,
   proceeds to the next step, its retirement list in the close for his
   later read (the group's pace, 2026-10-29); a retirement outside the
   lists, or one without a holder, waits for his word, as #137's
   rulings did. The machine is his during a league sitting, as
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
#128) on 2026-10-29. The owner's pre-league work, in order: the daemon paths his run
will hit, the league milestone's buildables (#74, #107, #112), tools
hygiene. The first, daemon-safety (#113, #116, #110), closed on
2026-10-29 (`LEDGER.md`), and the league milestone's buildables
(#74, #107, #112) and tools hygiene (#117, #118, #126, #114, #106,
#135) the same day: the owner's pre-league plan is done. Next, Phase 3,
the owner's league run, at his word; #70 closes on a week of pushes
without a re-run. Between sittings, the test-shape group's first
tranche ("Pull-forward", item 2), the orchestrator proceeding on its
own judgment as that item says. t1 (#137) closed on 2026-10-29
(`LEDGER.md`), and t2 (#138) the same day; t3 (#139) closed undone
on 2026-10-30, and t4 (#140, the compile cost) the same day
(`LEDGER.md`): the first tranche is done. Its second, t5 to t7, is
phase 4's. The group's review rule, revisited at this close, is kept
(the rulings list). On 2026-10-30 the owner's review of the open
issues closed #64, #136, #133 and #6 and took two groups ahead of the
time seam ("Closures, then A and B"): map-before-seam (#65, #134) and
stop-means-gone (#130, #144), both closed the same day (`LEDGER.md`).
The same day the owner pulled the words forward, ahead of the league
seat: step `words` (#20, #11, #101, #16) closed on 2026-10-31
(`LEDGER.md`), with C156 (a printed command is for a POSIX shell, or
PowerShell 7.3 or newer on Windows outside Git Bash). Next, Phase 3,
the owner's league run, at his word; its first seat checks the words.
Before the seat, at the owner's word of 2026-10-31: #146, then 4-0,
the time seam. #146 closed on 2026-10-31 (`LEDGER.md`), with C156
amended: `ACQ_SHELL` declares `posix`, `pwsh`, `powershell5` or `cmd`
over detection. 4-0's design closed on 2026-10-31
(`briefs/4-0-design.md`, the rulings list), its rulings entered and
its steps on #147, #148 and #141. Step 1, the client's door as a
machine (`briefs/4-0-door.md`, cut four times under cold reviews, the
narrowing recorded as the owner's ruling in the design's §4), closed
on 2026-10-31 (`LEDGER.md`), its review clean. Step 2, the daemon's lifecycle as a machine
(`briefs/4-0-life.md`, cut five times under cold reviews, its narrowing
recorded in the design's §4 under the owner's grant of autonomy),
closed on 2026-10-31 (`LEDGER.md`): its build reviewed, two fix
rounds, the second read by the orchestrating session. Step 3, the
parking-clock spike (`briefs/4-0-spike.md`), closed the same day on
its branch (`LEDGER.md`): the clock works, the design's door count
does not, and the runtime's park hook is the exact signal. t5's design
brief (`briefs/t5-design.md`, cut three times under cold reviews)
was ruled on 2026-10-31 ("Accept as drafted with your
recommendations"), C158 entered. Round 1, the clock and the door
(`briefs/t5-r1.md`), closed on 2026-10-31 (`LEDGER.md`): its build
reviewed, one fix round reviewed clean. Round 2, `Life` on the clock
(`briefs/t5-r2.md`, cut three times under cold reviews), closed the
same day (`LEDGER.md`): its build reviewed, one fix round reviewed
clean. Round 3, the families as tables (`briefs/t5-r3.md`, cut three
times under cold reviews), closed the same day (`LEDGER.md`): 34 rows
for 15 tests, its build reviewed, one fix round reviewed clean. Round
4, the login and the sessions as scenarios (`briefs/t5-r4.md`, cut
three times under cold reviews; the design's §4 narrowed by a dated
line, the other files' private reads to round 5), closed the same day
(`LEDGER.md`): 32 scenarios for 32 retired tests, the door's `quiet`
and `browser`, its build reviewed, one fix round reviewed clean. Round
5 split into 5a, 5b and 5c on 2026-10-31 (the design's §4, from two
inventories), and round 0's baseline dispatched in CI the same day
(run 41265798977 on the tag `t5-round0-baseline`). Round 5a, the
refresh and the bearer as scenarios (`briefs/t5-r5a.md`, cut three
times under cold reviews), closed on 2026-10-31 (`LEDGER.md`): the
token table and 20 scenarios for 29 retired tests, the door's restore,
its build reviewed, one documentation finding fixed. Round 5b, the
queue and the rails as scenarios (`briefs/t5-r5b.md`, cut four times
under cold reviews), closed the same day (`LEDGER.md`): the admission
table of 58 rows, the restored rows, the queue file and the rails, 37
tests retired, its build reviewed, one fix round reviewed clean. Round
0's baseline completed the same day (run 41265798977: 18 missed, twelve
the listed exclusions at drifted lines, six in `policy.rs`). Round
5c, the sweep (`briefs/t5-r5c.md`, cut three times under cold reviews,
C159 entered and narrowed to the owner's ruling), closed on 2026-11-01
(`LEDGER.md`): the door serves a world, the last door-able tests
through it, `ManualClock` retired, every ceiling `HANG`, the
exempt-route set under one source, its oracle and check 11, its build
reviewed, one fix round reviewed clean. Round 6, the close
(`briefs/t5-r6.md`, cut three times under cold reviews), closed on
2026-11-01 (`LEDGER.md`): the after-run against the baseline, 18
missed in both, the twelve listed exclusions moved to their lines, the
six `policy.rs` survivors pinned, none introduced, the census 378 to
416 in rows; its build reviewed, two fix rounds, the second read by
the orchestrating session. #141 closed under C158's wording: t5 is
done, its `HISTORY.md` row written. Next #148; and Phase 3, the
owner's league run, at his word, its first seat checking the words.
`AGENTS.md` governs; the gate is `tools/gate.sh`.

Phase 3's first seat, the league seat, closed on 2026-11-01
(`LEDGER.md`, `audits/league-2026-11/seat/README.md`): two doors, two
runs each by a blind Opus runner on the league build, the `words`
step's four fixes met without being named; run 1's ten faults fixed by
one rule each and reviewed cold, the review's three findings answered;
run 2 met none of them and found three siblings, fixed in a third
round; four issues filed (#153, #155, #156, #157). What it leaves for
the owner's live run: the league loop as the README prints it ran as
printed on both doors; a plan on an unlearned route gives no time until
its probe; what is his to rule is listed in the record's "For the
owner". #148 closed on 2026-11-01 in the two weeks before the league:
the store's five busy timeouts and `ensure_wal`'s retry under
`ACQ_STORE_WAIT`, each expiry pinned at zero (`8de554bf`, its cold
review's three findings answered in `41d09423`, the fix review clean;
the annotation file's switch kept under its own knob, the orchestrating
session's call, listed for the owner). Next, Phase 3, the owner's
league run, at his word. Before it, #153 and #154 (describe's index
and slices) closed on 2026-11-01 as one step (`briefs/describe-slices.md`,
`LEDGER.md`): the index a document of the search crate read at two
densities on both surfaces and in both formats (C53 amended, the
owner's word), `~pattern` a slice over names and definitions whole at
ten or fewer and lines otherwise; the build `eb9f29b9..ea07ebfb`
reviewed cold, its two findings answered in `4e53584d`, the fix review
clean; the derived groups the owner drew wait on #159 for a seat that
types a group name. Next, Phase 3, the owner's league run, at his
word.

The time seam read whole on 2026-11-01 (`reviews/time-seam-retrospective.md`,
`4eb55bc7`): the seam right as built, two ideas named, six questions for
the owner, answered "yes to all" (the rulings list). The step it
recommends first, the stuck verdict (`briefs/151-stuck-verdict.md`, #151:
the daemon's waits on one clock under check 12, a door wait failing at
the exact park), cut three times under cold reviews (first cut
`e1a0dcb8`); C160 entered ahead of its code, its words narrowed at the first review and qualified at the second under the grant (the hook's four accountings named, a `beside` door keeping the ceiling, under 800 bytes), for the owner's re-read. Its build landed
on 2026-11-01 as `0ae93980..82ac4f4c` (the measurement, the timers with
no hold owed, the verdict and the three classes, check 12), reviewed
cold (two P2), its fix round `68cb9448` reviewed (one P2), its tail
`68b9feb1` read by the orchestrating session; closed on 2026-11-01
(`LEDGER.md`), the weekly mutants run dispatched on the tag
`151-stuck-verdict-after`, whose compare against `t5-round6-after` is
#151's close, read on 2026-11-02 (run 41360883538, the compare at
`e812a0d7`): of the 48 moved mutants 36 are caught, the four of class
(b) are removals, and eleven still time out (the stress mutant as
expected, and ten under which four door scenarios hang, each a mutant
that breaks a send's completion or a probe's setup, so the hook holds
for a send that will never land: `a_head_429_that_fails_setup_holds_the_next_probe`
under four, `a_blocked_probe_is_no_probe_failure` under three,
`a_cloudflare_blocks_mark_on_disk_keeps_no_page_or_header` under two,
`observe_landed` and `sleep_until` hanging component tests); the
timeouts 67 against the baseline's 62 and t5's 107, the shards 87 to
118 minutes against the baseline's 66 to 106. #151 stays open on that
remainder. (Corrected the same day by the transport seam's design, its
third review: the three scenarios called door waits are component tests
that drive their own clocks or poll, so no verdict races them, and the
design promises no holder for any of the eleven; what remains on #151
is an inventory the seam build's close reads by its compare, nothing
promised.) The two "missed" were the listed `head` exclusion
at a line the stuck-verdict step moved, now at 611 (the commit
carrying this line). The residue step (`briefs/time-residue.md`, cut three
times under cold reviews) closed on 2026-11-01 (`LEDGER.md`): the
spec on the crate's one clock, the client's one ceiling, the boundary
as a value with a census at zero "neither" (#152 closed), the mock's
window divisor for a user with the harness leaving it unset on the
measurement that found the suite throughput-bound (the retrospective
corrected in place). The transport seam's design
(`briefs/transport-seam-design.md`, a Fable design cut six times under
five cold reviews) was ruled on 2026-11-01 ("Accept as drafted", the
rulings list): C161 entered ahead of its code. Next, in series at his
word: the seam's build briefs from the design's §6, steps 1 to 3, each
Opus-built and reviewed cold, in the window he names beside the
league; then the login flow as a machine; and Phase 3, the owner's
league run, at his word. A fresh orchestrating session starts from
this paragraph, the design brief and #151. Step 1, the seam with no
behaviour change (`briefs/transport-seam-1.md`, cut twice under cold
reviews), landed on 2026-11-01 as five commits from `6ca5b9a3`, this
line's commit the last: `http` named where its types are, the seam and
the one client behind it, the two-run table, check 13 and its breakers,
the words; reviewed cold, its fix round `3365183b..7c1c9626` reviewed in
turn, whose finding went to the owner (#160, "(b) Own crate", C1
amended at `baf625c9`). Its last round (`briefs/transport-seam-1b.md`),
the transport crate with check 5's edges and check 13 retired, landed
on 2026-11-01 as two commits from `24052df0`; reviewed cold, its fix
round `22da60ae` (the edges by package id) reviewed clean. Step 1
closed on 2026-11-01 (`LEDGER.md`). Step 2, the in-process transports
and the fixtures (`briefs/transport-seam-2.md`, cut four times under
cold reviews), landed on 2026-11-02 as seven commits from `05fcb1d9`,
this line's commit the last: the test transports and the counters
before, the mock's answer, the door onto the transports, `beside` on
the hook, the component fixtures and the spec, the occupancy
exclusions and the hold's boundary rows, the counters after and the
words. Reviewed cold (three P3: the raw form cut text by a byte
length, two summaries wider than their evidence), its fix round
`b79e36f8` read by the orchestrating session and not reviewed a third
time. Step 2 closed on 2026-11-02 (`LEDGER.md`). Step 3, the
deadline on the clock (`briefs/transport-seam-3.md`, cut three times
under cold reviews, after C160's second amendment), landed on
2026-11-02 as four commits from `a91658a5`, this line's commit the
last: the deadline on the choke point's clock over the four sends, the
client without timeouts, the expiry as wire-table rows, the words.
Reviewed cold, no findings. Step 3 closed on 2026-11-02 (`LEDGER.md`).
The rung, the owner's live run from his terminal, passed on
2026-11-03 at `1f31a394` (`RUN-LEDGER.md`, the two rows of that date:
a first attempt stopped by the driver's own check, a leftover queue
and an expired refresh token, recovered by hand; then the pass, 25
sends through the seam's production transport). Its rehearsal and its
first attempt changed the driver (`64a8d669`, C135's lifetime with no
token POST; `1f31a394`, a run refused while an earlier queue waits on
disk). The seam closed on 2026-11-03 (`LEDGER.md`): the mutants
after-run's seven survivors fixed in `d831658c`, #151's thirteen open
timeouts each still timing out, and #151 open. The owner restated
#151's close condition the same day ("Accept it"): the sixteen named in
`briefs/151-component-waits.md` caught in a Weekly after-run. That step
(the brief cut three times under cold reviews; C160's third amendment
at `514fdeb9`) closed on 2026-11-03 (`LEDGER.md`): 16 of 16 caught
locally, its build reviewed, one fix round. The Weekly after-run
`41652559158` on the tag `151-component-waits-after` showed each of the
sixteen caught, and #151 closed on 2026-11-03. The answer-facts group
opened the same day under the owner's grant of autonomy ("I agree
with your approach, and I encourage you to orchestrate with
autonomy"): the map `reviews/answer-facts-map.md` (reviewed cold
twice, its eight recommendations accepted as a set), its rulings
C118 amended and C162 entered at `f3db3427`, and its rounds in the
map's §10 order: the coverage fold in the store (S2,
`briefs/155-coverage-fold.md`, cut twice under cold reviews), then the
subject's block on the search (S1, #155), the named-paths pin (S4),
the shop's and price status's blocks (S3, #156), the scope of
league-taking surfaces (S5, #157, #22), the words round (S6 to S10),
and a seat. Current: round 1, its brief under its second cold review.
Round 1 landed on 2026-11-03 as one commit, this line's (the store's
`coverage` module and its unit tests, the search calling it through
re-exports, no golden changed); its review is pending.
