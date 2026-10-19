# The release — open record

Step 4 of `brainstorming-notes/36-the-road-to-the-agent-release.md`,
under the brief `RELEASE-PLAN.md`: the agent surface released from
GitHub as `v1.0.0-beta.1`, a pre-release. Opened 2026-10-18 at
`05e6c70c`; closes by 2026-11-22, or slips (note 36, "The sequence").
In the mold of the earlier records: while the slice is open its
findings and observations land here, and at closure it is cut to the
permanent short record. The scope, the roles and the steps are the
brief's; this file never restates them.

## Before building — for the owner

Rulings first (`CONTEXT.md`, "Working style"). One ruling was open;
the options were put with a recommendation and answered 2026-10-18.

1. **Where the user-facing README lives — ruled: the user's README is
   `README.md`.** The options put: (A) the user's README at
   `README.md`, the charter paragraph to `CONTEXT.md` ("Working style",
   which named it as the README's opening paragraph) and the crate
   index to `AGENTS.md` as a table; (B) a separate user guide, the
   README unchanged — the second document a visitor finds, after a
   front page written for a session that has read `CONTEXT.md`, and an
   archive nobody opens for prose; (C) one README for both with a
   contributor tail — over budget and insider text on the front page.
   Recommended A, with: the tour kept as the fenced block
   `readme_tour.rs` pins and the knob table kept whole, so the checks
   keep holding the form; no decision id anywhere in the README (C116,
   the README being user-facing text), file names as the agents'
   pointers; `AGENTS.md`'s budget raised for the crate table; the
   archive unchanged, the release notes linking to the README's install
   section. The README's shape, in order: what it is; install from the
   archives; log in; your agent (the MCP host's configuration, the tool
   families, pricing among them, and that in real mode the agent uses a
   daemon the person started, which exits after a minute idle); the
   tour; before the first command (the seat's facts a person needs
   before typing: the rate limit is the clock, job verbs write the
   store and store verbs read it offline, `--json` is total, account
   selection, where the data lives); knobs; known gaps; what it does
   not do; reporting and contributing; licence. The opening's register
   was drafted for the owner to judge and is step 3's model. The
   owner, verbatim: "go ahead. this is a good plan."

## Step ledger

| Step | Commits | What landed |
| --- | --- | --- |
| 0 the record, the ruling | this file's first commit | the record opened, the README ruling, the crate index and the charter moved |
| 1 pricing on the MCP | `db30d28a`, `40002b72` | the pricing verbs moved from the CLI binary into `acquisition_plan::pricing`, the CLI calling them; the seven tools as adapters, each the CLI's `--json` document; `tests/pricing_tools.rs`; the park deleted |
| 2 the policies | the commit that adds this row | `SECURITY.md` and `CONTRIBUTING.md`, V7's positions and no others, in the owner's voice; read by the owner before the commit |

## Owner verdicts, verbatim

- Step 2, on the two drafts: "I have enabled private security
  reporting. I will probably edit the drafts somewhat, but that will
  take time and thought. What you have written is a good start. Let's
  use them."

## Step briefs

Each worker reads `AGENTS.md` as every session does, then
`RELEASE-PLAN.md` ("What binds", "Rules the orchestrator keeps", its
step), then this section. A worker builds its step and nothing beside
it; what it finds beyond its brief is reported back as a park with a
trigger, never built. Every `acq` runs in a mock world (the
mock-session skill; `ACQ_STORE_DIR` in scratch). The gate runs bare
before every commit, each stage's status in its own variable and the
commit inside an `if` on them.

### Step 1 — pricing on the MCP

The step's text is `RELEASE-PLAN.md`, "Step 1". Before building, read
`PRICING-SLICE.md`, `decisions/pricing.md`, `decisions/frontends.md`
(C11, C52, C53, C115, C116 and the two parks under "Pricing on the
MCP" and "The read economy"), the MCP crate's module doc
(`crates/acquisition-mcp/src/main.rs`, how `set_sync_policy` is built
under C52 and how a store tool reads with no daemon), and the CLI's
pricing commands (`crates/acquisition-cli/src/`, the `price`, `shop`
and `reference` verbs) to find the `acquisition-plan` functions they
call — the tools call the same functions, never a copy. Seven tools:
`price_status`, `price_list`, `price_show`, `shop_render`,
`reference_currency`, `price_set`, `price_clear`. Acceptance is the
brief's. Commits: one per tool or per coherent group, each message
the story; the park "Pricing on the MCP" deleted in the commit that
builds its last tool; `MCP-REFERENCE.md` regenerated. Report back:
the commits, the gate's output, and anything the brief did not state
that the step met, as a finding with its evidence.

### Step 2 — the policies

The step's text is `RELEASE-PLAN.md`, "Step 2", and note 36's V7
(`brainstorming-notes/36-the-road-to-the-agent-release.md`, "Step 4 —
the release" and "Not in this note"). Read `.githooks/commit-msg` for
the stance the AI-assisted rule generalises, `LICENSE-MIT`'s header for
the owner's name, and `README.md`'s "Licence" paragraph for the voice
the owner already signed. Write `SECURITY.md` and `CONTRIBUTING.md` at
the root, each under one screen, plain words, no decision id (C116),
no position V7 does not name. Do not commit: the owner reads both
before the commit (the brief's acceptance). Report back the two files'
text whole and anything V7 left you to decide, as a question.

## Observations still open

- Step 1's refusal words: the pricing tools' account and argument
  kinds are the MCP's existing ones (`no_accounts`, `account_needed`,
  `account_unknown`, `arguments`), as the search tools' are, not the
  CLI's (`not_logged_in`, `ambiguous_account`, `unknown_account`,
  `usage`); a stale or blind revision conflict has no `kind` on either
  surface, so an agent tells it apart by its text; the refusals name
  the CLI's flags (`--league`, `--if-revision`) and `acq price show`,
  the descriptions saying how a flag maps to an argument. The release
  seat drives both surfaces and says whether any of it costs a call.
- Targets have two forms: an answer holds a target as an object and a
  tool takes the word form; the `target` parameter says how one becomes
  the other. Whether the seat re-reads that is the read economy park's
  evidence.
- `acq-mcp` connects per call (`main.rs`, `connect`), and a real-mode
  daemon exits after `ACQ_IDLE_SHUTDOWN` with no connection and no live
  limiter history (C3): an agent session that pauses longer than a
  minute finds the daemon gone and must ask the person to start one.
  `ACQ_IDLE_SHUTDOWN` on the starting command is the workaround; the
  release seat (step 4) measures whether that is enough.
- The whole-CLI seat's first fact (`audits/rehome-2026-10/seat/REPORT.md`,
  section 4, item 1) still stands in the tour's closing paragraph: the
  account rule is stated for job verbs only. Step 3 fixes it.
