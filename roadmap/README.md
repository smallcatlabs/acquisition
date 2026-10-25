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
   (a fix round's at `<step>-fixes-<sha>.md`) and committed alone, so
   they survive a session's compaction; that commit needs
   `git diff --check` alone, not the gate. A round that touches only
   comments and documents gets no review, and nothing is reviewed a
   third time unless the owner asks.
4. **The owner reads** the findings and the proposed fixes before any
   fix is built. A finding is pinned as a test and fixed by one rule,
   never per case; a fault that came back joins `REVIEW-CHECKLIST.md`.
   The commit that lands the fix round deletes the review file and
   cites it whole at the commit that held it.
5. **The close** is a commit whose message checks the brief's done-when
   line by line against the tree and answers each item the worker
   raised: taken, filed as an issue, or not taken and why. It writes
   the step's ledger line, puts what the step left open under "Open"
   or in an issue, and sets "Current step". A step that has not
   earned its close slips rather than ships.
6. **A phase close** folds the phase's ledger lines into one, adds the
   `HISTORY.md` line, and deletes the phase's briefs and review files,
   cited whole at the commit before.

The report a worker returns is `briefs/TEMPLATE.md`'s; its last section
is the worker's own, anything it wants raised about the brief, the
plan or the tools, and the close answers every item.

## The phases

| Phase | Steps | Who | Needs | Closes by |
| --- | --- | --- | --- | --- |
| 0. The week's fixes | 0a to 0e | Opus workers | — | closed 2026-10-20 |
| 1. The governance reset | 1 design, 1b to 1e | Fable, then Opus | — | closed 2026-10-20 |
| 2. The keystone | closed: 2a the shared function layer · 2b the tests · 2c the journey test and real-shaped mock items · 2d the daemon and limiter file splits · 2d-oracle the journal oracles over every journaling daemon · 2e the game-diff tool, a new-league rehearsal and the README line · 2f the review of daemon, store and network: a static scan, the owner's rulings and three fix rounds over four PRs, each round reviewed by Codex. Ahead, slid after phase 3 and `alpha.2` by the owner's word of 2026-10-24: 2g the read economy and the MCP refusal and receipt bundle (#9, #10), one bound at the shared layer under C53's three output levels · 2h the queued search and trade issues (#38, #46, #47; #37 once the owner has sat the site), which slips before 2g does | mixed | phase 1 | 2026-12-05 |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · the prune verb (#43) at the league end | owner, Opus | 2e | 2026-11-20 |
| 3b. `alpha.2` | regenerate the tables when the named inputs exist: a post-launch `poe1` pin (RePoE lags launch by 0 to 13 days), the owner's four site captures (C79: nobody else may fetch them), the pins and manifests bumped, every generator's `--check` green · then the release | owner captures, Opus | 3 | when the inputs arrive, 2026-11-26 at the latest |
| 4. The GUI | closed: 4a the 4-lite design (C16 amended, C138 to C141, `briefs/4b-build.md`) · ahead: 4b 4-lite, `acq web`, a loopback renderer over the verbs — the search pages (the form, the table, the counts, one item) and the refresh pages (the selection, the plan, the jobs), no subscriber — before the league · after `alpha.2`: 4-0 the time seam (`TIME-SEAM.md`) · 4-full on it: the keeper bundle (#1), the corpus held in `acq-mcp`, the subscriber with reconnect, the live loop, the dense table and the card, hardening | Fable, then Opus | 2a, 2b | 4-lite before phase 3; 4-full 2026-12-19 |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as the assessment's report 6, section 3: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | 2026-12-11 |

## Open

What closed steps left behind, each with its home. An item leaves this
list when a step's close commit or an issue takes it.

- **Deferred past 2f, the daemon's architecture**, each a row of the
  assessment's architecture report: a typed `JobKind`, the generic
  reference-table loader, the `Thing` columns, the lock merge, sharing
  `frame.rs`. The lock merge has a second input from 2f: the keyring's
  save runs under `shared` (`install_tokens_locked` in
  `daemon/refresh.rs`, the login's in `daemon/login.rs`), so a keyring
  that stalls holds every task that waits on `shared`; C133's hard end
  bounds the exit, not the stall.
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
- **The Windows gate's watch (#70)**: the step landed 2026-10-24
  (`dd9b7859` to `90db890a`); the issue closes when a week of pushes
  needs no re-run. Held for that evidence: nextest test groups capping
  the heavy properties, and the two trade tests that read no
  `ACQ_CASES`, so the share tier only quiets their neighbours
  (`TIME-SEAM.md`, section 3). The default profile's comment still
  says the longest in-test deadline is 60 s, written before the waits
  rose to 120 s; re-read both profiles' sizing at the close.
- **Tooling, no step named**: the document-path check reads no roadmap
  file, whose ledger and briefs tell deletions as story; the trade
  check has no breaker suite; `bothy pack --path README.md` returns
  the search and trade rulings, since the basename matches intros that
  name `search/README.md` (`tools/bothy.py:284-305`); a session's
  must-read carries the memory index beside `AGENTS.md`.
- **An issue ready for a step, none named**: #11, the words nobody
  understood, which waited on 2a.

## Starting a session

1. Read this file, then the current step's brief, then what the brief
   names. `STATE-2026-10-19.md` and `GUI-NOTES.md` are dated evidence,
   read when a brief points into them.
2. A session ends with "Current step" true and its step's ledger line
   written; nothing else here changes except at a close.

## Current step

4b, the 4-lite build (`briefs/4b-build.md`), is built and reviewed:
`acq web` in the CLI crate, a loopback renderer over the verbs, with
the search pages and the refresh pages; the review's six findings and
the fix round's one finding fixed, each by one rule (the ledger), and
no third review by the owner's word. What remains is the owner's
sitting at the pages in his own store (the brief's Do 10, his, from a
terminal); the close answers what it finds. The #70 step's fix-round review is pending too,
and its issue closes when a week of pushes needs no re-run (under
"Open"). 2g and 2h follow phase 3 and `alpha.2`; then 4-0, the time
seam (`TIME-SEAM.md`); then 4-full. `AGENTS.md` governs; the gate is
its ten lines, under nextest.

## Ledger

One line per step: its commits, when it closed, and what it left open,
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
| 4b 4-lite, `acq web` | build `f29506bc` (C141) to `fba1b03e`: the icon field `00020cac`, the stat vocabulary `d4e7f2ce`, the catalogue's listing-only sentences `550cd616`, the moved-world pins `843b391c`, the CLI's renderers as text `3323a488`, `acq web` `fba1b03e`; the review `548051f9`; fixes `0b1fc1ff` to `c48b9f73` (the review deleted there, whole at `548051f9`); the fix round's review `a8d12d0a`; its one fix the commit carrying this line (that review deleted there, whole at `a8d12d0a`; no third review, the owner's word) | the owner's sitting (Do 10) ahead | the sitting's stops, answered at the close; the picker's no-fallback rule unpinned (no seam reaches a page's script) |
| #70 the Windows gate's deadline flakes | ruling and brief `dd9b7859`; build `04001a27` to `90db890a`; the reorder and the review `54d9f957`, `493ff5d4`; fixes `6697afc3`, `32769f72` (the review deleted there, whole at `493ff5d4`) | fix-round review pending; the issue closes on a week of pushes without a re-run | the watch and what it holds, under "Open" |

## Housekeeping

What "A step" does not say. The dated files, `STATE-2026-10-19.md` and
`GUI-NOTES.md`, are never rewritten; the assessment is corrected only
where an outside review shows a sentence wider than its report, the
correction dated in place. The parks triage the assessment points to,
`parks-triage.md`, is whole at `1f524d15`; step 1b gave every park it
judged one of four fates, an issue, a "Not doing" line, a ruling or a
deletion, listed in `roadmap/governance/PARKS.md` at `0704a8f7`.
