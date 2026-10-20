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
- **Codex, via Astra**, reviews each committed step. A finding is pinned
  as a test and fixed by one rule, never per case; a fault that came back
  joins the review checklist.
- **A step** is: brief, build, gate, commit with the story in its
  message, review, a ledger row here. A step that has not earned its
  close slips rather than ships. The report form is in
  `briefs/TEMPLATE.md`.
- **Every `acq` an agent runs is in a mock world** (`ACQ_PROVIDER=mock`,
  `ACQ_STORE_DIR` to scratch). The real provider is the owner's alone.

## The phases

| Phase | Steps | Who | Needs | Closes by |
| --- | --- | --- | --- | --- |
| 0. The week's fixes | 0a store-age performance · 0b per-realm policy merge · 0c an unreadable MCP request gets a reply · 0d the stale and contradicting lines · 0e dev-build levers | Opus workers, parallel | — | 2026-10-26 |
| 1. The governance reset | 1 design (Fable, the owner rules on the drafts) · then the migration steps the design names: entries, parks to "not doing" lines and GitHub issues, records to one review checklist and a history, checks and skills trimmed, notes deleted | Fable, then Opus | phase 0 reviewed | 2026-11-02 |
| 2. The keystone | 2a the shared function layer (Fable design, Opus build) · 2b the tests: one harness, one layer per behaviour, goldens, one binary per crate, nextest · 2c the journey test and real-shaped mock items · 2d the daemon and limiter file splits · 2e the game-diff tool, a mock rehearsal of a new league, a README line for starting one · 2f the ultra review of daemon, store and network (owner-triggered) and its fixes; the frontends' after 2b | mixed | phase 1 | 2026-11-12 |
| 3. League 3.30 | the owner runs the league under the rails · count the undecided classes and untranslated mods · regenerate against the post-launch export when RePoE catches up · `alpha.2` | owner, Opus | 2e | 2026-11-20 |
| 4. The GUI | 4 design (Fable: C16 amended to the loopback web UI, the listener's security, the spawn policy for a frontend with no terminal, item images in or out) · the corpus held in `acq-mcp` · the subscriber with reconnect · the server · the web views · hardening | Fable, then Opus | 2a, 2b | 2026-12-10 |
| 5. PoE2 | the route flip and first-contact checklist prepared unmerged · PoE2 currency rows · launch day as note 36's step 6: a first contact only if GGG ships the stash endpoint | Opus, owner live | 0b | 2026-12-11 |

Deferred past the ultra review, each a row in the assessment's
architecture report: a typed `JobKind`, the generic reference-table
loader, the `Thing` columns, the lock merge, sharing `frame.rs`.

The other session's GUI analysis lands at `roadmap/GUI-NOTES.md` and is
phase 4's second input.

## Current step

Phase 0 is open; its five briefs are ready. Next after its reviews: the
phase 1 design session.

## Ledger

| Step | Commits | What landed | What it left out |
| --- | --- | --- | --- |
| the assessment | this directory's first commit | `STATE-2026-10-19.md`, `parks-triage.md`, this plan, the phase 0 briefs and the two design briefs | nothing built |

## Housekeeping

A brief stays until its phase closes, then is deleted and its ledger row
cites the commit that held it. This file stays current; nothing else
here is rewritten once written. The assessment is dated evidence and is
never updated. After phase 1 the registry's cross-cutting sentence and
`AGENTS.md`'s row point here as they do now.
