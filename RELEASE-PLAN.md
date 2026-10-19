# The release — the brief

Step 4 of `brainstorming-notes/36-the-road-to-the-agent-release.md`,
closes by 2026-11-22: the agent surface — the CLI, the MCP server and
the daemon on Windows, macOS and Linux — released from GitHub as
`v1.0.0-beta.1` (V2), a pre-release. This file is the brief a fresh
session orchestrates under; it is deleted at the close and cited at
its last commit. Written 2026-10-18 by the session that closed the
trade import (`807f237e`), which does not lead this step: the owner,
2026-10-18, "yes, i agree. write the brief for a new session to
orchestrate".

## What binds

- Note 36, step 4 and V7, and its rulings 1 to 7; the owner's
  narrowing of step 2, verbatim: "I want to table the other
  distribution methods and stick to direct downloads from github so
  keep that stuff from slowing down the critical path."
- The pricing park, fired by the owner 2026-10-13, verbatim: "K19
  build in step 4" (`decisions/frontends.md`, "Pricing on the MCP").
- C74: the shop render is in scope as a derivation, publishing is not;
  `shop_render` on the MCP returns the page, as `acq shop render`
  prints it, and posts nothing.
- C46, C11, C53, C116 and C52 for every tool added; `PRICING-SLICE.md`
  and `decisions/pricing.md` before touching pricing.
- The owner's guiding light, 2026-10-18: "keep simplicity, usability,
  and elegance as guiding lights. Patching edge cases and adding
  mechanisms is likely to be seductive, and may be necessary, but
  should only be done in service to the goal of making the system
  better and easier for you to use."

## Roles

- **The orchestrator**: one fresh session, reading as every session
  does (`AGENTS.md`), then this brief, note 36's step 4, the pricing
  park, `PLATFORMS-SLICE.md` and the whole-CLI seat's record
  (`audits/rehome-2026-10/seat/`). It brings the rulings below, opens
  the record, writes each worker's brief from the step's text here,
  reviews each worker's commits against the gate and the acceptance,
  and stops after each step for the outside review (Astra's Codex
  pass) before the next. It reads the release seat's reports itself,
  or hands the seat to a fresh session as the re-sit did, as its
  context allows.
- **Workers**: Opus sessions, one per step below marked so, each given
  its step's finding and brief and never the fix expected; what one
  proposes beyond its brief is a park, not a commit. Two workers run at
  once only where this brief says, each in its own worktree.
- **The owner**: every ruling, every outward act (below), every
  credential and portal setting.

## Before any build: the ruling to bring

**Where the user-facing README lives.** Today's `README.md` is the
workspace's charter and an agent's first read (`AGENTS.md`), its form
held by `tests/readme_tour.rs` and `tools/docs-check.sh`. A person
arriving from the release page or the reddit post reads `README.md`
first. The options as this session sees them: the user's README
becomes `README.md` and the charter and crate index move to a file
the agent docs name; or a separate user guide the release page and the
archive carry, `README.md` unchanged. The orchestrator reads the two
checks and the archive's contents (`.github/workflows/release.yml`),
then brings the options with a recommendation; the owner decides
before step 3.

Anything else a step meets that the rulings do not state stops the
step and comes to the owner (rulings first, findings after).

## Steps

| Step | Who | Builds | Acceptance |
| --- | --- | --- | --- |
| 0 | orchestrator | the record `RELEASE-SLICE.md` (open), its `AGENTS.md` row; the README ruling | the owner's verdict, verbatim, in the record |
| 1 | worker | pricing on the MCP | below |
| 2 | worker, beside 1 | `SECURITY.md`, `CONTRIBUTING.md` | below; the owner reads both before the commit |
| 3 | worker, after 1 | the user-facing README | below |
| 4 | orchestrator, the seat skill | the release seat and its fix round | below |
| 5 | owner, the orchestrator drafting | the outward acts | below |
| 6 | orchestrator | the close | the session-close skill, "When a slice closes" |

**Step 1 — pricing on the MCP.** Build `price_status`, `price_list`,
`price_show`, `shop_render`, `reference_currency`, `price_set` and
`price_clear` as thin adapters over `acquisition-plan`'s report, views,
render and table, the functions `acq price`, `acq shop render` and
`acq reference currency` call; no second semantics (C46). Each
returns the document the CLI's `--json` prints for the same request
(C11, C53); its refusals are the CLI's words, their kinds the MCP's
own, one vocabulary per surface (amended 2026-10-19). `price_set` and
`price_clear` take C52's create-only form, as `set_sync_policy` does:
a new row is written without a revision, and replacing or clearing a
row names the revision it replaces — the CLI's `--if-revision`, optional
there and required here, so the MCP is the stricter by the park's
words and its tool description says so.
Pricing reads intent and facts through the store: no daemon, no
network. The tools' descriptions are their usage home (C116, no id in
them); `MCP-REFERENCE.md` regenerated with `ACQ_UPDATE_FIXTURES=1`.
The park is deleted in the commit that builds it; the read economy
park beside it stays, its trigger the seat's re-read record.
Acceptance: per tool, a test that its document equals the CLI's
`--json` on a mock store; a blind replace and a blind clear refused, each naming the current
revision;
the no-ids test over the new descriptions; the gate green.

**Step 2 — the policies.** `SECURITY.md` and `CONTRIBUTING.md` at
the root, in the owner's voice, holding V7's positions and no others:
the latest release alone supported; the asset named, the keyring-held
refresh tokens and the GGG relationship; private vulnerability
reporting the channel; `CONTRIBUTING.md` a pointer to `AGENTS.md` and
the gate, never a second authority; the AI-assisted rule as the
`.githooks/commit-msg` stance generalised (the submitter the author
and accountable, assistance disclosed as `Assisted-by:`, the gate
decides); and note 36's sentence on issues: the parks are the graph,
an issue is evidence that may fire a trigger, a bug is a finding. It
touches nothing step 1 touches, so it runs beside it. Acceptance: the
owner's reading, the owner's words kept wherever they change it.

**Step 3 — the user-facing README**, where the ruling put it. Install
from the release's archives (`curl … | tar -x` on macOS and Linux,
the macOS binaries Developer ID signed and not notarized; Windows as
`PLATFORMS-SLICE.md` found it); `acq auth`, a person's act at a
terminal, the real provider the default (C88); an MCP host's
configuration for `acq-mcp` over stdio, the pricing tools among the
listed. Its input is the whole-CLI seat's section 4 (twenty facts the
tour and the help did not say, `audits/rehome-2026-10/seat/REPORT.md`):
each checked on the current release build first, and placed in the
help (its usage home) where it belongs there, in the README only where
a person needs it before the first command. Acceptance: every command
it prints runs as printed in a mock world (`acquisition-cli/tests/readme_tour.rs`
or its successor); the gate green.

**Step 4 — the release seat** (the seat skill): a person new to the
tool, from the README alone, on the release build in mock worlds made
from a copy of the owner's store, as before. The tasks include
installing from a local archive of the release's shape, configuring
an MCP host, and pricing end to end on both surfaces (status, a set,
a replace that must name its revision, the render). The owner's shape
of seat, verbatim (2026-10-17): "I do like the idea of exercising both
the mcp and cli", "i like the idea of a sonnet round after an opus
round", "I'm also ok doing all of this in serial". Briefs reviewed
from outside before any run. The fix round is a step of the slice,
closed before the release (V6): faults of the shapes in the records'
findings tables are fixed, anything new is a park with its trigger,
and the division between what is built now and what waits for the
first users is brought to the owner before building (the owner's triage of
2026-10-18). The record counts the re-reads the read economy park asks
for.

**Step 5 — the outward acts, each the owner's.** The orchestrator
drafts and the owner sends, posts or approves; nothing leaves the
machine otherwise.
- The note to GGG under the registration's framing ("Acquisition 1.0,
  same registration, new capabilities"; invariant 4), before anything
  public: the owner's own words; a draft only if asked.
- Private vulnerability reporting enabled on this repository.
- The C++ issues the Rust design answers: a table of
  `acquisition-qt`'s open issues (eight on 2026-10-18; `gh issue list`
  reads them), each with the Rust behaviour that answers it, its test
  or ruling, and the closing note drafted; the owner closes them.
- The tag `v1.0.0-beta.1`, by the owner: `release.yml` signs the macOS
  binaries on the release environment's reviewer and drafts the
  release; it is published as a pre-release, so `/releases/latest`
  stays empty.
- The reddit post: the owner's.

## Rules the orchestrator keeps

- No live run and nothing reaches GGG from an agent shell: every
  `acq` in a mock world, `ACQ_STORE_DIR` in scratch even for an
  offline flag (C88; the mock-session skill). A live row, if a step
  turns out to need one, is the owner's, from the owner's terminal.
- The gate run bare before every commit, each stage's status in its
  own variable, the commit inside an `if` on them (the session-close
  skill, 1). Build before any smoke run (C82).
- A size or count in a commit message measured after the last edit.
- One commit per fault or feature; a review finding becomes the test
  that holds it, named for the fault, fixed by one rule rather than
  per case.
- Routing by `AGENTS.md`: a ruling to the registry, a park with its
  trigger, an observation to the record, never a second authority.

## Left out

The GUI; Windows signing, notarization, installers and every channel
beside the archives (tabled, step 2); PoE2 (step 6, the launch);
the parks deferred past this release (`v1.0.0-beta.1`; "the alpha"
in the triage's words) by the owner's triage of 2026-10-18, which
wait for the first users' evidence; shop publishing
(`CONTEXT.md`, parked: "no shop publishing").
