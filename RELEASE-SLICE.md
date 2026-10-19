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
| 2 the policies | the commit that adds the two files | `SECURITY.md` and `CONTRIBUTING.md`, V7's positions and no others, in the owner's voice; read by the owner before the commit |

## Findings

One row per review round; the finding, then the test that holds it
now. The outside review is Astra's Codex pass over each step's commits.

| Round | Commits | Findings | Held by |
| --- | --- | --- | --- |
| review 1, steps 1 and 2 (2026-10-19) | the fix commits, one per fault | five: a row on a target the facts do not hold can be written and then read nowhere but as an address under `status` (1); the receipt's `prior` is read before the store's compare-and-swap, so a create after a racing clear names the cleared row as what it replaced (2); `set` reads the facts for `in_facts` after the commit and a failed read discards the receipt of a write that landed (3); the MCP's `price_clear` opens the facts it does not need, so a facts database the CLI's clear ignores refuses the MCP's (4); the archive smoke checks `acq-mcp` is executable and never runs it (5); and two in the documents: the brief's acceptance says the tools' refusals carry the CLI's `kind` while the built kinds are the MCP's; `price_set`'s undo clause assumes a prior | the rules the owner approved, in the worker's brief below; the tests named there |

## Owner verdicts, verbatim

- Step 2, on the two drafts: "I have enabled private security
  reporting. I will probably edit the drafts somewhat, but that will
  take time and thought. What you have written is a good start. Let's
  use them."
- Review 1's fixes, on the five rules and the brief's amended line,
  all built now: "approved. go ahead."

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

### Review 1 — the fixes

Five faults, each fixed by one rule and held by a test named for the
fault, one commit per fault, in this order. The rule is the ruling;
the mechanism is yours. Every test fails on the code before its fix.

1. **Orphan rows.** A `buyout` row on a target the facts do not hold is
   intent to look at (C35). Rule: `status` lists each such row whole,
   in the shape the listing's manual side already has (`ManualSide`:
   value, `from` the target itself, `revision`, `updated_at`,
   `written_via`, `actor`), in place of the bare address; the CLI's
   text prints the value and revision on the row's line; `show`'s
   refusal for a target with a row of its own says so and names
   `acq price status` as where it is listed. No new verb; `show`
   stays one target's listing, a subject of the facts. The write
   test in `tests/pricing_tools.rs` then reads the row's `written_via`
   through the surface, never through `Annotations`.
2. **The receipt's prior.** Rule: what a write replaced is the row at
   the revision the write named; a create names none and replaces
   none, so its receipt's `prior` is null. In `set_buyout` and
   `clear_buyout`, the plan crate; the store is untouched. The test
   stages the race as the store's lock tests do (one connection holds
   `BEGIN IMMEDIATE` across the other's write): a row at revision 1,
   a clear landing while a create is between its read and its put,
   and the create's receipt reports no prior.
3. **The write is last.** Rule: `set` reads everything it reports,
   the facts included, before the write, and nothing after it; a facts
   read that fails refuses before anything changes. The test makes
   the facts unreadable (drop the table through a second connection,
   or whatever is least mechanism) and shows the row unchanged and the
   refusal naming the facts.
4. **Clear needs no facts.** Rule: the MCP opens for a verb what the
   CLI opens for it. The account-refusal mapping (`no_accounts`,
   `account_unknown`, `account_needed`, `store`) moves into an intent
   opener `open_search` builds on; `price_clear` uses the intent one.
   The test removes the facts database after seeding and shows a
   revisioned clear landing on both surfaces alike.
5. **The smoke runs `acq-mcp`.** In `tools/release-smoke.sh`, after
   the mock login with the daemon up and before the stop: pipe an
   `initialize`, the `notifications/initialized` notification and a
   `tools/call` of `daemon_status` into the extracted `acq-mcp`, one
   JSON-RPC message per line (`tests/harness/mod.rs` is the model),
   and check the daemon it reports is running, compatible, mock, and
   runs the extracted `acqd`. Package locally with
   `tools/release-package.sh` and run the smoke on the archive; the
   smoke is CI's on every platform (`release.yml`).

Then one more commit: `price_set`'s description says the undo after a
create is `price_clear` with the written revision, and after a replace
`price_set` with the prior's value and `if_revision` the written
revision; `MCP-REFERENCE.md` regenerated with `ACQ_UPDATE_FIXTURES=1`;
the no-ids test still green.

Not yours: the brief's acceptance line (amended by the orchestrator);
a `kind` for a revision conflict (the seat's evidence); anything the
fixes make you notice, which is a park in your report.

## Observations still open

- Step 1's refusal words: the kind vocabulary is ruled (the brief's
  amended acceptance, 2026-10-19: the MCP's own); a stale or blind
  revision conflict has no `kind` on either surface, so an agent tells
  it apart by its text; the refusals name the CLI's flags (`--league`,
  `--if-revision`) and `acq price show`, the descriptions saying how a
  flag maps to an argument. The release seat drives both surfaces and
  says whether any of it costs a call.
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
