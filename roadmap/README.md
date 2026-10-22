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
| 2. The keystone | closed: 2a the shared function layer · 2b the tests · 2c the journey test and real-shaped mock items · 2d the daemon and limiter file splits · 2d-oracle the journal oracles over every journaling daemon. Ahead: 2e the game-diff tool, a mock rehearsal of a new league, a README line for starting one (the assessment's report 6, section 5, is its source) · 2f the ultra review of daemon, store and network (owner-triggered) and its fixes, its inputs under "Open"; the frontends' after it · 2g the read economy and the MCP refusal and receipt bundle (#9, #10), one bound at the shared layer under C53's three output levels · 2h the queued search and trade issues (#38, #46, #47; #37 once the owner has sat the site), which slips before 2e to 2g do | mixed | phase 1 | 2026-11-12 |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · the prune verb (#43) at the league end | owner, Opus | 2e | 2026-11-20 |
| 3b. `alpha.2` | regenerate the tables when the named inputs exist: a post-launch `poe1` pin (RePoE lags launch by 0 to 13 days), the owner's four site captures (C79: nobody else may fetch them), the pins and manifests bumped, every generator's `--check` green · then the release | owner captures, Opus | 3 | when the inputs arrive, 2026-11-26 at the latest |
| 4. The GUI | the keeper bundle (#1) as the first pre-work, the first long-lived frontend being its consumer · 4 design (Fable: C16, the Tauri ruling, amended to the loopback web UI; the listener's security; the spawn policy for a frontend with no terminal; item images in or out; `GUI-NOTES.md` the second input) · the corpus held in `acq-mcp` · the subscriber with reconnect · the server · the web views · hardening | Fable, then Opus | 2a, 2b | 2026-12-10 |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as the assessment's report 6, section 3: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | 2026-12-11 |

## Open

What closed steps left behind, each with its home. An item leaves this
list when a step's close commit or an issue takes it.

- **For 2f, the daemon and the network**: #51, the user-agent
  (invariant 4) has no test; the lock inversion in `exit_process`
  (`crates/acquisition-daemon/src/daemon/server.rs`: `jobs_db` then
  `shared`, where `persist` takes `shared` then `jobs_db`), with the
  lock order as practised written where the locks are declared; the
  "Decisions as recorded" blocks, moved verbatim by 2d and wrong where
  they were wrong, cut to the mechanism each file holds (C18, C19 and
  C32 would then need a code line that names them); the comment on
  `Daemon::log` in `daemon.rs` that says it takes the shared lock,
  which it does not; the daemon crate's second test harness
  (`tests/sessions.rs`'s `test_daemon` and `wait_for_refresh_waiters`,
  the latter in `stress.rs` too) beside `tests/harness.rs`; the client
  harness's probe-first rule, which matches the route name before `@`
  and would miss a realm-suffixed route; the login daemons, which
  journal nothing and so run under no oracle. Deferred past 2f, each a
  row of the assessment's architecture report: a typed `JobKind`, the
  generic reference-table loader, the `Thing` columns, the lock merge,
  sharing `frame.rs`.
- **Unruled**: the stream of a text failure's report (`refresh --apply`
  prints it on stdout, `acq result` on stderr).
- **The owner's**: the `AGENTS.md` budget, raised to 15,277 bytes under
  his temporary permission during phase 2, his to reassess; whether a
  *Pinned:* path with a slash becomes a bothy rule (2d writes nineteen
  that way); the Windows gate, which runs on his push (the guest was
  down for 2d and 2d-oracle).
- **Tooling, no step named**: the document-path check reads no roadmap
  file, whose ledger and briefs tell deletions as story; the trade
  check has no breaker suite; a `log` normaliser rule for
  `<scratch>/logs/<hash>` when a second golden needs it; a session's
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

Step 2e is built, its review pending. The build brief,
`briefs/2e-build.md`, ran on the owner's go in the main tree: six
commits from `35e0b03c` (the rulings C125, the tables name their
export, and C126, a league the list lacks is a note; the game-diff
tool and the drift line; the mock's three leagues; the league named in
every line about its work; the rehearsal; the documents and the
driver). Codex reviews `35e0b03c^` to this step's last build commit;
its findings are saved under `reviews/` and read by the owner before
any fix. The design brief, `briefs/2e-design.md`, is at `08c0907a`
with the owner's word at `e9b8bc78` and its review taken at
`cb7db91c`. The owner reassesses the `AGENTS.md` budget when he
chooses. `AGENTS.md` governs; the gate is its ten lines, under
nextest.

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
| 2e the game-diff tool, a new-league rehearsal and the README line | design `08c0907a`, `e9b8bc78`, `cb7db91c`; build from `35e0b03c`, six commits; review pending | — | named at the close |

## Housekeeping

What "A step" does not say. The dated files, `STATE-2026-10-19.md` and
`GUI-NOTES.md`, are never rewritten; the assessment is corrected only
where an outside review shows a sentence wider than its report, the
correction dated in place. The parks triage the assessment points to,
`parks-triage.md`, is whole at `1f524d15`; step 1b gave every park it
judged one of four fates, an issue, a "Not doing" line, a ruling or a
deletion, listed in `roadmap/governance/PARKS.md` at `0704a8f7`.
