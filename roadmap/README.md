# The road after the alpha

The plan for the work after `v1.0.0-alpha.1`, opened 2026-10-19 from a
seven-audit assessment of the project (`STATE-2026-10-19.md`, dated
evidence, read when a brief points into it). Read this file first for
any step of it. It holds the order, the current step, what each step
left open and one line per step closed. A ruling goes to the registry;
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

1. #83, the MCP surface trimmed (short tool descriptions, the language
   as resources), then a seat with the first row of the seat ledger
   (#84), before `alpha.2`.
2. #88, one feature graph for the plain and the test build: measured,
   and if it holds, C84's build-order rule and the gate's final rebuild
   retired (P6).
3. 2g, the read economy and the refusal bundle (#9, #10).
4. #85, #86, #87, #89: the standing automation and the hygiene from the
   assessment's R5, each a short step on any idle day.

## Open

What closed steps left behind, each with its home. An item leaves this
list when a step's close commit or an issue takes it.

- **Tooling issues carry no area label** (#84, #86, #88, #89): no
  area file holds the tools or the workflows; the owner's, whether a
  label or an area.
- **Deferred past 2f, the daemon's architecture**, each a row of the
  assessment's architecture report: a typed `JobKind`, the generic
  reference-table loader, the `Thing` columns, the lock merge, sharing
  `frame.rs`. (2f's second input to the lock merge, the keyring's save
  under `shared`, was taken by r2: C148.)
- **Unruled**: the stream of a text failure's report (`refresh --apply`
  prints it on stdout, `acq result` on stderr); `--if-revision 0` as
  "no policy stands" is the CLI's meaning alone, where the MCP's
  `if_revision` names a revision and creates when none is given (2e's
  second fix round); whether the environment's proxy variables may
  route the one client (`build_client` in `ratelimit/choke.rs` names
  none).
- **For 3b, the regeneration**: #53, the defence extract's generator
  is not in the tree; the pin is bumped before the generators run
  (C125, the tables name their export); at `poe1@ee2084de` the forms
  table would differ by two trade-stat ids (2e's measurement).
- **The owner's, from r2**: a brief that asks for its tests committed
  failing first (r2's Do 1) meets `TEMPLATE.md`'s rule that a commit
  is gated on the gate's exit status; r2 committed red and said so in
  the message. Whether a failing-tests commit is an exception the
  template names, or the tests ride with their fix.
- **The owner's**: the `AGENTS.md` budget, raised to 15,277 bytes under
  his temporary permission during phase 2, his to reassess; whether a
  *Pinned:* path with a slash becomes a bothy rule (2d writes nineteen
  that way); whether the
  frontends get the review 2f gave the daemon, store and network, which
  the phase table had put after it; the reviews' process suggestions
  from 2f (its reviews, whole at `b1588df3`): for
  `briefs/REVIEW-PROMPT.md`, a round's brief named once for a range,
  client input told apart from persisted or upstream input, and prior
  findings accepted as a fix round's done-when; for a round's plan,
  restart after a failed write and a login's finalisation among the
  recovery scenarios.
- **From r3, the eviction** (`briefs/r3-idle-and-eviction.md` and its
  rounds): `job_list` holds the queue lock for its whole range read, so
  the dispatcher's persists wait on it, a cost that scales with the
  lifetime's work; `ended_elsewhere` answers `no_job` for an open row
  on disk and not in memory, leaving the orphan until the next restore
  drops or replays it; the login wait's broken-queue pin calls
  `wait_job_terminal` directly, since no seam stages a queue break
  between the profile job's eviction and the read through a login; the
  parent scan's O(n²) stands, over live work only now.
- **From r4, the diagnosis** (`briefs/r4-diagnose.md` and its rounds):
  `acq daemon status` waits without bound on a peer that answers
  `hello` and never its status, where the diagnosis bounds its own
  request by the handshake's deadline (C86 untouched; a bound on every
  observe request is the owner's); the placeholders are angle-bracketed
  and vanish outside a code block, so the header says to paste between
  backtick fences, confirmed as the design; a stopped daemon's journal
  and log are found by this shell's knobs, and the daemon writing its
  journal path beside its log marker is more mechanism than today's
  evidence asks (#94 carries the contract form).
- **From r5, the sentences** (`briefs/r5-surface.md` and its round):
  C127's words say the client kinds are "the daemon's kind table lists
  for clients, in the provider modes each row names", and since
  `b2c9dd6e` the protocol's two lists (`job::CLIENT_KINDS`,
  `job::MOCK_ONLY_KINDS`) name the kinds and the modes, read by the
  daemon's door and both frontends' help; the property stands and the
  mechanism words are the owner's to redraw. A selection plan carries
  no `notes` on any surface (the CLI's `--tabs` and `--characters`, the
  page, the MCP's `selection`), where a policy plan notes a league the
  fetched list lacks; a selection-shaped `plan_notes` is a planner
  change for its own step if a consumer asks. The tool descriptions
  state sizes of compact JSON while `acq --json` is pretty-printed,
  about a third larger; a worker measuring with the CLI gets the wrong
  number.
- **From the C135 round** (`briefs/c135-round.md` and its rounds):
  `exit.rs`'s `settle()` waits 100 ms and reads `is_finished`, the
  scheduling assumption the round's review named in the new tests and
  the round fixed there with `polled_once`; the older tests take the
  same helper when it moves into the harness. Since `1bf30803` the
  auth requests and `quote` are refused as stopping during any exit,
  as a submit always was, not only during the restore; a logout
  against a departing daemon is refused and the client reaches the
  next one. The brief's "`quote` never waits" was wrong for the tree
  (`quote` resolves its account against the session table), and the
  build has it wait. A 401 under an unreadable account index followed
  by a restart resends the rejected bearer once (#63).
- **The Windows gate's watch (#70)**: the step landed 2026-10-24
  (`dd9b7859` to `90db890a`); the issue closes when a week of pushes
  needs no re-run, counted from r1's first green Windows run
  (40428110668, `10c40dc2`, 2026-10-27). Held for that evidence: nextest test groups capping
  the heavy properties, and the two trade tests that read no
  `ACQ_CASES`, so the share tier only quiets their neighbours
  (`TIME-SEAM.md`, section 3). The default profile's comment still
  says the longest in-test deadline is 60 s, written before the waits
  rose to 120 s; re-read both profiles' sizing at the close.
- **For 4-full, from 4b**: the results as site-style cards, which need
  rows carrying every field, one maker with `show` (`GUI-NOTES.md`
  section 7; the owner, 2026-10-25: "just for form layout for now");
  item counts beside the league select, if a document that carries
  both on one basis is ever made; a web phrasing for the next actions
  the pages print as CLI command lines beside their buttons, a verb
  change; `acq web` starting with no account on disk and showing the
  refusal as a page, where it refuses in the terminal today; the MCP's
  `search` description naming `view.rows.fields`, which only the web
  UI asks for.
- **From 4b's worker, no step named**: no test seam reaches a page's
  script, so the picker's no-fallback rule is unpinned, and a seam is
  the day's work when a second script rule needs one; the search crate
  still prints flags in text that is no refusal (route commands,
  `describe` entries, the keys not built, readings), outside C143's
  rule as worded; a whole-page golden is about 51 KB and two of them change on
  a one-line form edit.
- **The owner's, from 4b's reviews and worker**: for
  `briefs/REVIEW-PROMPT.md`, telling committed test coverage apart from
  evidence that the tests pass (both of the sitting round's reviews
  asked), and a fix round's acceptance told apart from the build's
  still-pending sitting; for "A step", that the committing session
  launches a worker step's review (settled so in 4b) and polls until
  `codex exec` exits before ending its turn, since an idle session
  does not wake when a background process does.
- **Tooling, no step named**: the document-path check reads no roadmap
  file, whose ledger and briefs tell deletions as story; the trade
  check has no breaker suite; `bothy pack --path README.md` returns
  the search and trade rulings, since the basename matches intros that
  name `search/README.md` (`tools/bothy.py:284-305`); a session's
  must-read carries the memory index beside `AGENTS.md`.
- **An issue ready for a step, none named**: #11, the words nobody
  understood, which waited on 2a.
- **From h1, the honesty group** (outside the phase order, the owner's
  word of 2026-10-25; `briefs/h1-build.md` and its rounds): the
  substash `stash.id` check (C145) was settled by a read of the owner's
  store (N50: 638 of 638 substash bodies carry the requested sub id).
  #63's five hygiene
  rows stay on the issue; #74 is the printed retry command after a
  rename under a failing index; #16 has its first instance fixed.
- **From h2, the words** (`briefs/h2-build.md` and its rounds; C146):
  #11's two remaining parts stay on the issue, a flag listing the
  items behind the residue and invalid-note counts and a location on
  the scope's oldest and newest fetch, each a design; the price
  refusal's `leagues_of` ignores the realm for an item, so an item
  asked for in the wrong realm can still print a remedy that shows
  nothing; `price_json.rs`'s `run_printed` runs `sh` outside the
  harness's isolation list (C122); the seat worlds' stores are gone,
  so the new words were judged on fixtures, and the next seat reads
  them at scale.
- **From h3, the foundations** (`briefs/h3-build.md` and its rounds;
  C147): the provider stays process-wide (`want_provider` reads the
  knob at each judgement; no consumer holds two); the older
  ten-second waits in `acquisition-client/tests/contract.rs` predate
  h3 and still wait on a clock; the other environment-moving unit
  tests in `client.rs` remove what they set instead of restoring it,
  and `Moved` is there for them; the replacement pin spawns a real
  `acqd`, so the client's unit tests need `cargo build --workspace`
  first, or an injectable spawner; the race look costs up to one
  handshake deadline on a child that dies before the lock when a stale
  pid sits in the lock file.

## Starting a session

1. Read this file, then the current step's brief, then what the brief
   names. `STATE-2026-10-19.md` and `GUI-NOTES.md` are dated evidence,
   read when a brief points into them.
2. A session ends with "Current step" true and its step's ledger line
   written; nothing else here changes except at a close.

## Current step

#83, the MCP surface trimmed (pull-forward 2,
`briefs/p83-surface-trimmed.md`), is built and its review pending, the
count glosses waiting on the owner's ruling; the seat (#84) follows its
close, its brief written from `briefs/TEMPLATE.md` by the session that
starts it. Phase 3, the owner's league run, follows at his word; #70
closes on a week of pushes without a re-run. The assessment these rest on is
`STATE-2026-10-26.md`; the issues carry the evidence per step.
`AGENTS.md` governs; the gate is `tools/gate.sh`.

## Ledger

One line per step: its commit range, when it closed, and what it left open,
each item's home under "Open" or an issue. The story is in the commits'
messages.

| Step | Commits | Closed | Left open |
| --- | --- | --- | --- |
| the assessment and phase 0, the week's fixes (0a to 0e) | `a987d02f` to `cca73395`; the briefs and the reviews whole at `cca73395` | 2026-10-20 | the roadmap review's finding 10 remainder: the synthesis's five corrected sentences stand as dated evidence; 0a's bound accepted at 1.34, the remainder the prune verb's (#43) |
| phase 1, the governance reset (1 design, 1b to 1e) | `7470a3b8` to `0704a8f7`; the briefs, the drafts and the reviews whole at `0704a8f7` | 2026-10-20 | the memory index beside `AGENTS.md` (under "Open") |
| 2a the shared function layer | design `05100056`, `a6eda757`; build `5ecbc59a` to `0958403f`; fixes `f476aa0e`, `1c22561f`, `1bc001f2`, `6b661c9d`; closed `3bc71a86` | 2026-10-21 | #49, one source for the world; the stream of a text failure's report, unruled |
| 2b the tests | design `9501f750`, `fa7dce2a`; build `a19dbce5` to `bdb3b646`; fixes `2d751d1f`, `aff30846`, `a2a59898`; closed `8c10cdbe` | 2026-10-21 | #50, the shape of `item_events`' answer |
| 2c the journey test and real-shaped mock items | design `4d2932fb`, `c04cb535`, `1c347053`; build `f4e5cb15` to `f38fa001`; closed `2b275eec` | 2026-10-21 | #51 for 2f; a `log` rule when a second golden needs it; the read-only open, since ruled C124 and closed as #52 (`3795e6ec`) |
| 2d the daemon and limiter file splits | design `dc993ddb`, `ac98c488`; build `e4019380` to `c670f112`; closed `af5badde` | 2026-10-21 | 2f's inputs and the owner's two, under "Open" |
| 2d-oracle the journal oracles over every journaling daemon | `e59ee842`; closed `1f524d15` | 2026-10-21 | the login daemons journal nothing (2f) |
| 2e the game-diff tool, a new-league rehearsal and the README line | design `08c0907a`, `e9b8bc78`, `cb7db91c`, `53ec784f`; build `35e0b03c` to `4e543dd0`; fixes `252e8c9e`, `bb319873`, `fb8371e0`; closed `64a6240c` | 2026-10-21 | #53 for 3b; the two claims for phase 3's live run (the live-run skill); the CLI-only `--if-revision 0`, under "Open"; the six remote-gate fixes after the close, `82199010` to `7b4793fe`, before 2f |
| 2f the review of daemon, store and network, and its fix rounds | the scan and its working notes outside the repository; round A `718f1cde` to `12395d33` (#54); round C `76be73d6` to `2fa31fc6` (#57) and `0c4a0829` to `b1d9cf3c` (#58), the owner's rulings at `76be73d6`, `f86c9b7e` and `1d72cf4f`; round D `26160d48` to `cf2137ef` (#60); the reviews whole at `b1588df3`; the brief written after, `briefs/2f-review.md`; closed by the commit carrying this line | 2026-10-23 | #51, #55, #56, #59 (its first part), #61 to #72; the architecture rows and the keyring under `shared`, and the proxy environment, under "Open" |
| 4a the 4-lite design | the brief re-cut `cbd7caf2`; the owner's rulings `00e440d1`; C141 under delegation `eeb72437`; the build brief `f1deb782`; closed by the commit carrying this line | 2026-10-25 | a selection compiled as a plan (the spend check refuses a plan with no stored policy), in `4b-build.md`'s "Not in scope" with its trigger, the owner's sitting; C141 listed for the owner's review at the close; the AGENTS budget at 15,235 of 15,277 bytes, under "Open" |
| 4b 4-lite, `acq web` | build `f29506bc` (C141) to `fba1b03e`: the icon field `00020cac`, the stat vocabulary `d4e7f2ce`, the catalogue's listing-only sentences `550cd616`, the moved-world pins `843b391c`, the CLI's renderers as text `3323a488`, `acq web` `fba1b03e`; the review `548051f9`; fixes `0b1fc1ff` to `c48b9f73` (the review deleted there, whole at `548051f9`); the fix round's review `a8d12d0a`; its one fix `13e725ff` (that review deleted there, whole at `a8d12d0a`); the owner's sitting, its first stop fixed at `c1ec1411` and its `RUN-LEDGER.md` row; the sitting round (`briefs/4b-sitting-round.md`, rulings `f2239aa6`), merged at `4c58d9e3`: `341bf94c`, `98ad588b`, `1dafc2d4`, `61b58ba1`, `4baf9a70`, `459c79ba`, `20c224b5`, `f55a43af`, `03e2bdc8`, `d1bca992`; the round's review `404e4fcd`; its fix round (rulings `eb9486ac`, C111 and C76), merged at `fadf0806`: `a88e318e`, `6ea0461a`, `9b0cd16b`, `5b4bc5b3`, `828875f8`, `9f561d8c`, `81e87042`, `bc7a715a` (that review deleted there, whole at `404e4fcd`); the fix round's review `25d98cef`; its four fixes `fb181ae0`, `03d37f17`, `7eb5fd1d`, `c38d9604` and the commit carrying this line (that review deleted there, whole at `25d98cef`) | 2026-10-25 | the 4-full items and the worker's items, under "Open"; the picker's no-fallback rule unpinned (no seam reaches a page's script); item counts beside the league select, declined (the league list and the counts are two documents on two bases); C142 and C143 under delegation, listed for the owner's review |
| #70 the Windows gate's deadline flakes | ruling and brief `dd9b7859`; build `04001a27` to `90db890a`; the reorder and the review `54d9f957`, `493ff5d4`; fixes `6697afc3`, `32769f72` (the review deleted there, whole at `493ff5d4`) | the fix-round review found nothing (`reviews/70-fixes-32769f72.md`); the issue closes on a week of pushes without a re-run | the watch and what it holds, under "Open" |
| p0 the process round | `f1a29952` to the commit carrying this line; reviews `reviews/p0-1523a776.md`, `reviews/p0-fixes-16ef9210.md` | 2026-10-26 | the gate script's breaker suite, on #87; the rebuild observation, on #88; the four memory notes, retired |
| r1 the Windows gate decides again | build `c2318752` to `de6ad867`; review `reviews/r1-de6ad867.md` (no findings); the Windows lint `ad216160`; closed by the commit carrying this line | 2026-10-27; Windows green at run 40428110668 | #70's week, counted from that run, under "Open" |
| r2 the credential I/O off the shared lock | design `912206df`, `62eaeea0`; ruling `610286d1`; brief `193c2c21`; tests `81e80d3f` (committed failing); build `e4e2b9b7`, `19c8a759`; review `d2cfdf70` (`reviews/r2-19c8a759.md`); fixes `f354ede2` to `344da98f`; their review `c32c057e` (`reviews/r2-fixes-344da98f.md`); its fix `8393336a`; closed by the commit carrying this line | 2026-10-27 | the save cut at the exit's bound, left to the time seam (4-0); #92 and #55 for the C135 round; a brief's "commit the tests failing" beside the gate-per-commit rule, the owner's, under "Open" |
| r3 the hour idle default and finished trees out of memory | ruling `e258a4c8`; build `61627c82`, `2e848e7d`, `0c04b6e3`; review `reviews/r3-0c04b6e3.md`; fixes `30c90878` to `719c86e9`; their review `reviews/r3-fixes-719c86e9.md`; fixes `5d434a27`, `068044f0`; their review `reviews/r3-fixes-068044f0.md`; the lost pin restored `79033fe5` (no review: no behaviour changed); closed by the commit carrying this line | 2026-10-27 | the list's count bound, on #56; the list's range read under the queue lock, the orphan open row `ended_elsewhere` leaves, and the login wait's missing seam, under "Open"; Do 3's "within retention" not built, on #56 |
| r4 `acq diagnose` | build `abc7e71d` (the document and the redaction), `ecf2c89c` (the verb, its text, the README and the reference); review `12c0d1e6` (`reviews/r4-ecf2c89c.md`); fixes `41bb35d6` (the token rule), `fa76532f` (no empty needle), `5b019b36` (the bounded status), `42548143` (`ACQ_JOURNAL` offline), `b0b6e487` (the printed locations), `62b6e9f4`; their review `bf3c4f25` (`reviews/r4-fixes-62b6e9f4.md`); fixes `c8907c6c`, `4cb10cd8`, `b9303f3a`; their review `a44fa3e0` (`reviews/r4-fixes-b9303f3a.md`), whose numeric finding that commit fixes unreviewed; closed by the commit carrying this line | 2026-10-27 | #94, the daemon report's journal path and the owner's question of a log that never formats a token; `acq daemon status` on a peer that stalls after `hello`, and a fix round's own acceptance line, under "Open" |
| r5 the five surface sentences | build `b2c9dd6e` (the kind list), `a694d991` (the describe sections), `2f4ae288` (the logout words), `aae783b2` (`refresh --characters`), `19592b01` (the MCP's `selection`); review `6f5e0119` (`reviews/r5-19592b01.md`); fixes `6f5e0119`, `5e7cc216`, `43cad6ff`; their review `566b22e9` (`reviews/r5-fixes-43cad6ff.md`), whose one wording finding that commit takes unreviewed; closed by the commit carrying this line | 2026-10-27 | C127's mechanism words and the league note on a selection plan, under "Open" |
| the C135 round, the access token beside the refresh token and the start's restore off the socket's path | brief `9a6a3c63`; build `d21c79c5` (the entry), `b99675f7` (the landings and the restore), `38620e2e` (the registry), `f28e027e`; review `reviews/c135-f28e027e.md`; fixes `b3ab68f0` (a rejected token leaves the entry), `380fccb1`; their review `reviews/c135-fixes-380fccb1.md`; its wording fix `4ab6342a` (unreviewed); the rulings `0065e17d`; build `fb92f468` (the restore as the fifth landing), `b9bada44` (the `auth status` words), `7cb7bd7f`; review `reviews/c135-7cb7bd7f.md`; fixes `1bf30803`, `1e8879aa`, `9f222231`; their review `reviews/c135-fixes-9f222231.md`; its two fixes `355229d0` (unreviewed); closed by the commit carrying this line | 2026-10-27 | the index-unreadable 401 restart, on #63; `exit.rs`'s `settle()` and the refusal during any exit, under "Open" |
| #82 the held corpus in `acq-mcp` (pull-forward 1) | brief `500e4468`; build `551520fd` (the holder), `b439d30e` (the web on it), `bfd3a410` (the MCP on it, M8); review `reviews/p82-bfd3a410.md`; its one doc fix `9a1e8d0b` (unreviewed); closed by the commit carrying this line | 2026-10-27 | #95, the resident size per reload; `show` over the held corpus and a per-account slot, declined until a consumer asks |
| #83 the MCP surface trimmed (pull-forward 2) | brief `1dd3dfdb`; build `81178cd0` (the answer section), `c9d6aec3` (the refusal's spelling), `c250b2c8` (`describe` by name), the commit carrying this line (the cut, the length test, the measurement) | review pending; the count glosses wait on the owner's ruling (the brief's Do 1) | the close's |

## Housekeeping

What "A step" does not say. The dated files, `STATE-2026-10-19.md` and
`GUI-NOTES.md`, are never rewritten; the assessment is corrected only
where an outside review shows a sentence wider than its report, the
correction dated in place. The parks triage the assessment points to,
`parks-triage.md`, is whole at `1f524d15`; step 1b gave every park it
judged one of four fates, an issue, a "Not doing" line, a ruling or a
deletion, listed in `roadmap/governance/PARKS.md` at `0704a8f7`.
