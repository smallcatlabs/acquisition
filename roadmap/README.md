# The road after the alpha

This is the plan for the work after `v1.0.0-alpha.1`, opened 2026-10-19
from a seven-audit assessment of the whole project
(`STATE-2026-10-19.md`: the synthesis first, the seven reports after it
as the evidence; `parks-triage.md`: every park with a status and a
fate). Read this file first for any step of it. It is the one file that
changes as steps close: the order, the current step and the ledger live
here; rulings go to the registry as before; a step's working detail is
its brief under `briefs/`.

The goal it serves is the owner's: a faster, simpler, better Acquisition
that is agent-friendly and agent-forward. The assessment's one-paragraph
verdict: the core is sound and the weight is in the shell (prose, tests,
three giant files); the governance framework was tuned for the phase
that ended at the alpha; one refactor, a shared function layer under the
CLI and the MCP, is where simplicity, the tests and the GUI meet.

## The owner's rulings for this road (2026-10-19, verbatim)

- "I accept all your recommendations exactly as proposed with a loopback
  web UI." The recommendations accepted: the governance reset (issues as
  the backlog, AGENTS absorbing CONTEXT, history stripped from entries,
  budgets retired except on the auto-loaded file); a plain-prose style
  rule with a gloss on every id; the corpus held in `acq-mcp`; the GUI's
  first shape a loopback web UI on the shared function layer, Tauri a
  later wrapper; one account-error vocabulary across CLI and MCP; the
  mock serving real-shaped items.
- "If it would help to add real items to the mock, I give permission to
  use items from any of my stores to whichever agent ends up building
  that part of this effort."
- Models: "stick to Opus 5.5 as much as possible, with Fable called in
  for complex analysis, synthesis, and design."
- On the state of the project: "which really isn't a sloppy mess at all".

## Roles and rhythm

- **The owner** rules on boundaries, runs anything live, captures the
  site, and makes the triage call at each phase boundary.
- **A Fable session** opens and closes a phase: it designs (phases 1, 2a
  and 4 have design briefs), reads the workers' reports, and rewrites
  this file. It does not build.
- **An Opus session** runs steps from their briefs. Independent steps
  may go to Opus workers in parallel, each in its own git worktree with
  its own scratch subdirectory; otherwise steps run one after another in
  the main tree, because a build under a live daemon or a concurrent
  gate is C84's artifact mismatch.
- **Codex reviews each committed step**, launched by the session that
  made the commit, read-only, with the project's prompt:
  `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>."` (the `review` subcommand cannot take both a commit and custom instructions, so the prompt names the commit).
  It runs on the owner's Codex account (verified from an agent shell
  2026-10-19). The findings are quoted whole in the step's report, the
  owner reads them and the proposed fixes before any fix is built, a
  finding is pinned as a test and fixed by one rule, never per case,
  and a fault that came back joins the review checklist.
- **A step** is: brief, build, gate, commit with the story in its
  message, review, a ledger row here. A step that has not earned its
  close slips rather than ships. The report form is in
  `briefs/TEMPLATE.md`; its last section is the worker's own, anything
  it wants raised about the brief, the plan or the tools, and the
  session that reads the report answers each item in the ledger row:
  taken, filed as an issue, or not taken and why.
- **Every `acq` an agent runs is in a mock world** (`ACQ_PROVIDER=mock`,
  `ACQ_STORE_DIR` to scratch). The real provider is the owner's alone.

## The phases

| Phase | Steps | Who | Needs | Closes by |
| --- | --- | --- | --- | --- |
| 0. The week's fixes | 0a store-age performance · 0b per-realm policy merge · 0c an unreadable MCP request gets a reply · 0d the stale and contradicting lines · 0e dev-build levers | Opus workers, parallel | — | 2026-10-26 |
| 1. The governance reset | 1 design (Fable, the owner rules on the drafts) · then the migration steps the design names: entries, parks to "not doing" lines and GitHub issues, records to one review checklist and a history, checks and skills trimmed, notes deleted | Fable, then Opus | phase 0 reviewed | 2026-11-02 |
| 2. The keystone | 2a the shared function layer (Fable design, Opus build) · 2b the tests: one harness, one layer per behaviour, goldens, one binary per crate, nextest · 2c the journey test and real-shaped mock items · 2d the daemon and limiter file splits · 2e the game-diff tool, a mock rehearsal of a new league, a README line for starting one · 2f the ultra review of daemon, store and network (owner-triggered) and its fixes; the frontends' after 2b | mixed | phase 1 | 2026-11-12 |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · the prune verb (ST-3, fired) at the league end | owner, Opus | 2e | 2026-11-20 |
| 3b. `alpha.2` | regenerate the tables when the named inputs exist: a post-launch `poe1` pin (RePoE lags launch by 0 to 13 days), the owner's four site captures (C79: nobody else may fetch them), the pins and manifests bumped, every generator's `--check` green · then the release | owner captures, Opus | 3 | when the inputs arrive, 2026-11-26 at the latest |
| 4. The GUI | 4 design (Fable: C16 amended to the loopback web UI, the listener's security, the spawn policy for a frontend with no terminal, item images in or out) · the corpus held in `acq-mcp` · the subscriber with reconnect · the server · the web views · hardening | Fable, then Opus | 2a, 2b | 2026-12-10 |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as note 36's step 6: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | 2026-12-11 |

Deferred past the ultra review, each a row in the assessment's
architecture report: a typed `JobKind`, the generic reference-table
loader, the `Thing` columns, the lock merge, sharing `frame.rs`.

The parks the triage said to build first each have a home above or an
explicit deferral here: F-4 is 0c; PL-9 is 0b; ST-3 is phase 3; the
read economy (F-3 with F-11 and PR-14, one bound under C53) and the
MCP refusal and receipt bundle land after 2a as a step 2g, since the
bound belongs at the shared layer; the queued search and trade parks
(T-5, T-6, SR-17, and SR-16 once the owner has sat the site) are a
step 2h after 2b, which slips before 2a to 2c do; the keeper bundle
(D-4, F-6, F-20, D-14) is phase 4's first pre-work, the first
long-lived frontend being its consumer, and the real-mode MCP gains
with it. The words bundle goes to issues at phase 1.

The other session's GUI analysis lands at `roadmap/GUI-NOTES.md` and is
phase 4's second input.

## Starting a session

1. Read this file, then the current step's brief, then what the brief
   names. The assessment is evidence, read when a brief points into it.
2. Phase 0's five steps run in the main tree one after another, in the
   order 0d, 0b, 0c, 0a, 0e (documents first, the measured steps last),
   or at most two at a time in worktrees of their own: a fresh worktree
   builds from nothing, and on 2026-10-19 `target/` held 86 GB against
   149 GB free. Each worker names its own scratch subdirectory. A
   measurement (0a's M3 rows, 0e's timings) runs alone on the machine:
   no other build, gate or measurement while it runs.
3. After each step: the gate, the commit with the ledger row below
   added in it, the review launched, its findings saved verbatim at
   `roadmap/reviews/<step>-<sha>.md` so they survive a session's
   compaction, and the report to the session that launched you. The
   owner reads the findings before a fix round; the commit that lands
   the fix round deletes the review file and cites it.
4. A session ends with "Current step" below true, and nothing else in
   this file rewritten.

## Current step

Phase 0 is open; its five briefs are ready and 0b has its ruling. Next
after its reviews: the phase 1 design session.

## Ledger

| Step | Commits | What landed | What it left out |
| --- | --- | --- | --- |
| the assessment | `a987d02f`, `9ec14155` | `STATE-2026-10-19.md`, `parks-triage.md`, this plan, the phase 0 briefs and the two design briefs, the review command | nothing built |
| the roadmap's review | `reviews/roadmap-a987d02f.md`; its fixes in the commit citing it | eleven findings, nine taken (0b's rule now rulings-first with C52; the review prompt leads with plain defects and asks for the smallest fix; the launcher corrected; `alpha.2` split from league day and gated on named inputs; measurements run alone; 0a's goal narrowed, a bound and a reproducible fixture added; ST-3 fired, the prune verb scheduled; the build-first parks given homes; five synthesis sentences corrected; two citations fixed) | finding 10's suggestion to re-split measurements, judgments and choices throughout the synthesis: the five sentences were corrected, the rest stands as dated evidence |
| 0d the stale lines | the commit adding this row | the session-close skill's help-string sentence reversed to C116; RELEASE-SLICE's README line says form, not budget; a rulings pointer atop every crate entry doc but search's, which had one, and AGENTS' two sentences narrowed to the entry docs; the ground truth's OBS row and dead `findings.md` pointer; the fused budget comments split; daemon.rs's and lib.rs's opening sentences; note 36's head line | item 7: the test's comment at `seat_faults.rs:2113` describes its own test and always has (checked at `259cad75`, where it landed); the audit's row was wrong, nothing changed |
| 0b per-realm merge | the commit adding this row | C36 amended with the owner's yes: a set replaces the realms it names, a realm named with no leagues goes, `--replace` (MCP `replace: true`) writes the whole; built once in the plan crate's `put_sync_policy`, merged from the row at the named revision in one put; a stored policy this build cannot read is refused for a merge and replaceable whole; tests on the plan crate, the CLI and the MCP; both references regenerated, the README gap and the park deleted | type-level filters and a sticky default realm (out of scope) |

## Housekeeping

A brief stays until its phase closes, then is deleted and its ledger row
cites the commit that held it. This file stays current; nothing else
here is rewritten once written. The assessment is dated evidence,
corrected only where an outside review shows a sentence wider than its
report, with the correction dated in place. After phase 1 the registry's cross-cutting sentence and
`AGENTS.md`'s row point here as they do now.
