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
| 3 the README | `7490c275`, `9105ee3d` | six verbs' help say what the whole-CLI seat had to find (`CLI-REFERENCE.md` regenerated); the README whole in the ruling's order, 14243 bytes: install by archive name, log in, the agent's host configuration and daemon, the tour, before the first command, knobs, gaps, what it does not do, reporting, licence. The seat's twenty facts: six to help, five to the README, nine nowhere because the build says them. `readme_tour.rs` reads every ```sh block and pins the archive names to `release.yml`'s matrix; `no_ids.rs` sweeps the README |

## Findings

One row per review round; the finding, then the test that holds it
now. The outside review is Astra's Codex pass over each step's commits.

| Round | Commits | Findings | Held by |
| --- | --- | --- | --- |
| review 1, steps 1 and 2 (2026-10-19) | `b45a0a23`, `0b91e7c7`, `4c169939`, `3adaeefd`, `73cc6ecf`, `b7a5fa4b` | five: a row on a target the facts do not hold can be written and then read nowhere but as an address under `status` (1); the receipt's `prior` is read before the store's compare-and-swap, so a create after a racing clear names the cleared row as what it replaced (2); `set` reads the facts for `in_facts` after the commit and a failed read discards the receipt of a write that landed (3); the MCP's `price_clear` opens the facts it does not need, so a facts database the CLI's clear ignores refuses the MCP's (4); the archive smoke checks `acq-mcp` is executable and never runs it (5); and two in the documents: the brief's acceptance says the tools' refusals carry the CLI's `kind` while the built kinds are the MCP's; `price_set`'s undo clause assumes a prior | (1) `rows.unmatched` lists each row in the manual side's shape, listing schema 3, `a_row_on_a_target_the_facts_do_not_hold_is_listed_whole_and_show_says_where` (`price_json.rs`); (2) `a_create_after_a_racing_clear_reports_no_prior` (`price.rs`); (3) `a_set_whose_facts_cannot_be_read_refuses_before_the_write` (`pricing.rs`); (4) `a_clear_lands_alike_on_both_surfaces_without_the_facts` (`pricing_tools.rs`); (5) the smoke's step 5, which fails on an `acq-mcp` that never answers or never exits; the brief's line amended at `e041cd46`, the description at `b7a5fa4b` |
| review 2, the fixes (2026-10-19) | `7571a570`, `d7a262fc` | two: `show`'s refusal named `acq price status`, which counts the orphan row and reads pc, so the command it pointed at printed nothing (1); `price_set`'s replace undo promised the prior's value back without exception, where a named revision replaces a value this build cannot read (2) | (1) the refusal names `acq price status --expand`, `--realm` from the target's address, and the test runs the command as the refusal types it (`price_json.rs`, the orphan test); (2) the description's clause, reference regenerated |
| review 3, step 3 (2026-10-19) | `21460844`, `c170c84d`, `d34e5606` | four: the agent's daemon recipe gave `auth check` for every case, which answers not_logged_in when no keyring kept the login (1); "Standard unless you name it" was said of every verb taking --league, where `price show`, a saved plan and `store characters` read otherwise (2); "every command talks to the real GGG" and "jobs write, the rest read" overstated the provider and what writes (3); the CLI reference stranded a global option's description under the previous option in the long help format, an older defect (4) | (1) `the_agent_recipe_logs_in_again_after_the_daemon_stopped` (`readme_tour.rs`), the printed line run in the keyringless mock world after a stop; (2), (3) the bullets name their verbs, no test holds prose; (4) the generator drops the heading with its block and the test asserts each global's text once (`cli_reference.rs`) |
| review 4, review 3's fixes (2026-10-19) | `036d0cb4`, `92f5f312` | two: the daemon bullet still called every other verb offline and `store import` an intent write, where `auth`, `jobs`, `status`, `cancel` and `daemon` use the daemon and import replays facts (1); the recipe test's `auth check` had started a daemon the recipe's login reused, so the idle knob went unread (2) | (1) the bullet, one sentence each for the daemon verbs, the offline reads, the intent writes and the import; (2) the test stops that daemon and reads running=false before the recipe's line |
| review 5, step 4's briefs (2026-10-19) | the commits that follow `26603d0f` | five, before any run: the CLI's T2 could pass by reusing the login's daemon, the door's 30-minute idle hiding whether the recipe took (1); a stalled MCP server or login would hang the door and leave no call row (2); both pricing tasks cleaned up before the shop, so the shop never showed the new prices, and the CLI's replace could land blind (3); the login was never a numbered call, and a host configuration's contents were not on record (4); the record said install and host configuration ran "as printed" where the door unpacks and reads them (5); and two smaller: mandated repeats counted as re-reads, a refusal preclassified as a fault; the brief's "alpha" against `v1.0.0-beta.1` | (1) `daemon status` reports `idle_shutdown_seconds`, the recipe test asserts it on the daemon the recipe started, T2 asks for a new daemon and its idle exit, the door's default disclosed in the brief; (2) a deadline on every MCP call and on the login, the kill logged as a call; (3) both briefs set, replace naming the revision, read back, render, then clean up and read back; (4) the login is a numbered call with its knob, the configuration's text in the call's file; (5) the record's coverage paragraph says what the door does and assigns the rest; the counting rule excludes mandated repeats, the refusal sentence asks whether it let the runner recover; the brief's "Left out" names the release |
| review 6, the briefs' fixes (2026-10-19) | the commit that follows `81e6e9bb` | four: the doors' deadline waited for a byte and then for a line, so half a line hung them, and a non-JSON answer or an EOF left no row (1); the host configuration appended to the answer's text inflated `text_bytes` and broke the call's JSON file (2); the refusal sentence swung to "recovered means no fault" (3); the old platform observation still said the seat confirms every line, and the coverage paragraph called the plan path unchanged since the whole-CLI seat where C111 landed after (4) | (1), (2) the MCP door's transport rebuilt: a reader thread, one deadline for the whole exchange, every failure one logged row, the configuration kept beside the row as its own file; the login the same; `doors_test.py` drives both against fakes that stall, half-answer, answer non-JSON, exit or linger, and hung the old doors past its watchdog; the skill runs it before a seat (step 4) | (3) two questions apart, warranted and recoverable; (4) both statements corrected, C111 cited at `188b0ae3` |
| review 7, the doors (2026-10-19) | the commit that follows `10902dbb` | two, both the MCP door's: the request's write ran before the timed wait, so a server that initialized and then stopped reading blocked a request larger than the pipe past the deadline (1); a JSON answer that was not the MCP response asked for raised out of the door with no row (2); and one the test found on the way: a large request went whole into the CSV row, past a CSV reader's field limit | (1) the write runs from a thread the deadline abandons, the row saying the server did not read; (2) the answer is decoded and shape-checked inside the guarded operation, an unexpected shape one error row; the arguments kept whole beside the row and cut in it; `doors_test.py` stages all three, and hung the previous door on the first |

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

### Step 3 — the user-facing README

The step's text is `RELEASE-PLAN.md`, "Step 3"; the ruling and the
README's shape are this file's "Before building", 1; the voice is the
README's present opening, which the owner judged. The owner's words
on this file, verbatim (2026-10-18): "It is the first things humans
will read, and it will be used by agents as well. It will frame how
this project is percieved by uses as well as future development
sessions. Carefully consider what should be included, or referenced,
or left out. Carefully think about how to write concisely and without
mannered speach."

Read first: `README.md` as it stands (keep its opening; rewrite the
rest in that register), `tools/docs-check.sh` (the README form it
holds: the tour one line per verb, every `ACQ_*` a crate reads in the
knob table, the 15000-byte budget) and
`crates/acquisition-cli/tests/readme_tour.rs` (every `acq …` line of
the first ```sh block against clap's help, twenty lines at least),
`crates/acquisition-cli/tests/no_ids.rs` (no decision id in the
README); `.github/workflows/release.yml` and `tools/release-package.sh`
(the archive names and contents, five targets, macOS signed and not
notarized, Windows unsigned); `PLATFORMS-SLICE.md` (what Windows
needs of a person: its findings and the Windows rows); the seat's
twenty facts, `audits/rehome-2026-10/seat/REPORT.md` section 4;
`CLI-REFERENCE.md` and `MCP-REFERENCE.md` for every verb and tool you
name; `decisions/daemon.md` C3 and `decisions/frontends.md` C13 for
what a person must do for an agent in real mode.

Build: the README whole, in the ruling's order — what it is; install
from the release's archives (`curl … | tar -x` on macOS and Linux,
the archive by name for each target, the macOS note on a signed,
un-notarized binary's first start; Windows as the platforms slice
found it); log in (`acq auth`, at a terminal, the real provider the
default); your agent (one MCP host's configuration for `acq-mcp` over
stdio, with the tool families, pricing among them, and that in real
mode the agent uses a daemon the person started, which exits after a
minute idle unless the knob says otherwise); the tour; before the first
command; knobs; known gaps; what it does not do; reporting and
contributing (`SECURITY.md`, `CONTRIBUTING.md`); licence. "Development
starts at `AGENTS.md`." stays, one line. The tour's closing paragraph
states the account rule for every verb, not job verbs alone (the open
observation below).

The seat's twenty facts: check each on the current build first (a
mock world: `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` in scratch — the
mock-session skill), and place each where it belongs: in the clap help
of the verb it is about when it is a verb's fact (regenerate
`CLI-REFERENCE.md` with `ACQ_UPDATE_FIXTURES=1`), in the README only
where a person needs it before the first command, nowhere when the
build already says it. Report the twenty as a table: fact, where it
went, or why nowhere.

Acceptance: every command the README prints runs as printed in a
mock world — `readme_tour.rs` or its successor pins the tour, and the
install and MCP lines are pinned by a test or named as not runnable
(a download); the gate green; the budget kept. One commit for the
help changes with the reference, one for the README. Nothing beyond:
what you find is a park in your report.

### Step 4 — the release seat

The step's text is `RELEASE-PLAN.md`, "Step 4"; the procedure is the
seat skill (`.claude/skills/seat/SKILL.md`); the owner's shape of
seat is the brief's quotation. The record directory is
`audits/release-2026-10/seat/`: `BRIEF-cli.md`, `BRIEF-mcp.md`, the
launch prompts `PROMPT-cli.md`, `PROMPT-mcp.md` and the debrief's
`PROMPT-debrief.md`, committed before any run and reviewed from
outside first; after each run its `calls-<run>.csv`,
`REPORT-<run>.md` and `DEBRIEF-<run>.md`; at the close its README with
the fate table, the briefs removed and cited at their commit.

The worlds, under `~/seat-worlds/release-2026-10/`, short-pathed for
the socket: `pristine/`, made 2026-10-19 by `setup --facts
GERWARIC_7694.db` with no daemon running (the owner's files read once
through sqlite's online backup, unchanged since 2026-10-15; the lock
file's pid was dead); then one world per run, `setup --store
pristine/store --archive <the macOS archive>`, the CLI's with
`--daemon`. Each holds its brief as `BRIEF.md`, `README.md` as the
release's commit carries it, its door, `bin/` unpacked from the
archive as a person would unpack it (the door records the archive's
sha256 and each binary's), and the store. The archive is
`tools/release-package.sh` over `cargo build --release --target
aarch64-apple-darwin` at the briefs' commit, with placeholder
third-party notices (`cargo about` is not installed here; CI makes
the real file); `tools/release-smoke.sh` passed on it.

The door changes made for this seat, in the skill: `setup --archive`
unpacks a release archive into the world's `bin/`; `run --env` and
`login --env` pass the one knob a README line puts before a command,
`ACQ_IDLE_SHUTDOWN`, and refuse any other; the MCP door's `open` and
`call` take `--host-config NAME`, starting the server as a host would
from `W/work/NAME` in the shape the README shows, refusing a command
that is not the world's own `acq-mcp`; the login is a numbered call
with its knob, and a stalled server or login is killed at a deadline
and logged as a call. What this covers, exactly: the archive's
binaries run as unpacked (the unpacking is the door's, in Python, not
a person's `tar`); the agent's recipe as the README prints it, its
effect read off `daemon status`, which since this step reports the
idle exit the daemon was started with; and the configuration's shape
as the door reads it, not as Claude Code or Claude Desktop would
load it. Not covered here, and assigned: the Windows and Linux
install lines and the platform data paths (the door sets the store
directory) to the platforms slice's VM rows and the smoke on every
target in CI; the macOS quarantine line to the owner at the tag;
loading the configuration in a real host to the owner's own first
session with the release. The seat does not drive policy, plan and
apply: the whole-CLI seat drove it, and what changed after — C111's
replay rule and admission, `188b0ae3` — is held by its own tests, not
by a seat; the MCP's runs have no daemon, so the fetch tools' refusals
are what it sees of them.

The runs, serial, each launched by `launch.sh` after the previous
debrief is in the record, never continued into another: `cli-opus`,
`mcp-opus`, `cli-sonnet`, `mcp-sonnet` (Opus 5.5 then Sonnet 5.5, the
owner's rounds). A runner reads its brief and the README and nothing
else. What the record counts, per run: the re-reads the read economy
park asks for (a second call for the same answer and a debrief's
"read twice", counted apart, laid to a cause only where the runner
gives one, and a repeat the brief itself asks for — the MCP's second
`open` through the host configuration — never counted); the README's
lines of unrecorded guidance a run could reach (the agent recipe, the
macOS install's archive), confirmed, contradicted or never reached;
pricing end to end on each surface — the two prices set, the replace
naming its revision, both read back on the shop's pages, then the
cleanup read back — at what cost in calls; and a difference between
the surfaces or the models on any task, a finding of its own kind.
The orchestrator reads the calls before believing a fault and says
how many it read.

The runs, as they land. Each launched at `e2dbeb8f` on the archive
`1dd7906b…` (its binaries hashed in the world's record), the runner's
startup context the owner's global file at `e9c1329f…`, as the
re-sit's. Cost is the session's at list price; a report's own counts
are the runner's.

| Run | Model | Session | Calls | Cut | Turns | Cost | Read by the orchestrator |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus | `claude-opus-5-5` | 4592effd-56eb | 107 | 2 (`shop render --expand` 3.5 MB, `--json` 7.4 MB) | 76 + 3 | $3.55 | 22 calls: the seven leading faults hold as stated; the recipe ran as the README prints it (call 24, the new daemon at 3600 s in call 25) and the README's gap is "stop the running one first" (call 22 logged the runner out) |
| mcp-opus | `claude-opus-5-5` | 11891c2e-7e05 | 51 | 2 (`price_list` 67 KB, `shop_render` 5.1 MB) | 66 + 3 | $3.46 | 9 calls: the leading faults hold as stated — the render whole at 5,057,320 bytes (call 32); a write to an id the facts do not hold landing with `in_facts` false (call 47, the ruling's behaviour, as the whole-CLI seat recorded it); the job tools' refusals as JSON-RPC errors (calls 40 to 42), the one naming `ACQ_NO_SPAWN` being the door's world speaking, since the door sets it, so the fix round reads the real-mode text; the instructions' first step refused on this two-realm store (call 5, `realm_needed`) |
| cli-sonnet | `claude-sonnet-5-5` | becbb2ab-c5a8 | 97 | 2 (`price show --json` 174 KB, `shop render --json` 7.5 MB) | 112 + 2 | $1.50 | 6 calls: the leading faults hold and converge with cli-opus's — the shop's JSON at 7,478,732 bytes (call 72), a revision conflict and a failed job without a kind (calls 81, 83), the daemon stop logging the runner out (call 16); new in call 78, a stale revision named on a cleared row answers "annotation no longer exists", hiding the clear and the revision it still counts |

cli-opus: one door command denied by the rails (a `report` heredoc),
sent again as a draft; the harness saved call 41's output to a file
of its own, which the runner did not open; all nine tasks finished;
the daemon left stopped. Its debrief retracts the cause it gave for
the 7.4 MB shop JSON (inferred from the size, the head alone read),
"every time" on the undo fault (more than one account known), and
three inferences of its own.

mcp-opus: two door commands denied by the rails — a one-line date
script outside the door, and a `call` heredoc, sent again as a draft;
the harness saved call 1's output (`open`, 42 KB) to a file of its
own, which the runner read, named in its "Left out" as the one file
outside the two; all eight tasks finished, every price it wrote
removed again (call 51 against call 19). Its debrief marks its T2
dates as hand arithmetic, its left-out counts as resting on the cut
part of call 32, and two remarks on the CLI as untested.

cli-sonnet: one door command denied by the rails (a `report`
heredoc), sent again as a draft; no file opened outside the two; all
nine tasks finished, no daemon left running (call 97); the prices it
wrote read back as before (calls 65 to 67).

The fix round after the four runs: faults of the shapes in the
records' findings tables fixed, one commit per fault held by a test
in `seat_faults.rs` or its surface's file; anything new a park with
its trigger; the division between what is built now and what waits
for the first users brought to the owner before building (the triage
of 2026-10-18). A rerun only where the round changed what an agent
meets.

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
- Two store tests failed one gate run in the fix worker's worktree
  and passed on every rerun (`corpus.rs`:
  `the_revision_moves_with_every_record_and_with_nothing_else`,
  `a_fetch_landing_mid_read_is_the_next_reads_and_never_this_ones`).
  The worker's reading: their `tmp()` names a directory by pid and
  counter and never clears it, so a leftover from an earlier run with
  the same pid seeds extra facts. Trigger: either fails a gate again;
  the fix is to remove the directory before use, as `pricing_tools.rs`'s
  `scratch` does.
- `PRICING-SLICE.md` (plan 4b, review 2) cites
  `reference/listing-report-schema-2.json`, renamed `-schema-3.json` at
  `b45a0a23`; the docs check does not scan closed records for reference
  paths. Trigger: this slice's close, when that citation is rewritten
  as "schema 2, at `<commit>`".
- `list`'s refusal for a container with a row of its own does not say
  so, where `show`'s now does. Trigger: the release seat shows an agent
  reaching `price_list --in` or `--covered-by` on an orphaned container.
- `price_clear`'s receipt text says `price_set` with the prior's value
  puts it back, which is right (a create over the tombstone) but names
  no `if_revision` where `price_set`'s undo clause now does in both
  cases. Trigger: the release seat stumbles on undoing a clear.
- Step 3's README states guidance no run record holds: a browser
  download on macOS is quarantined and the un-notarized binary refused
  until `xattr -dr com.apple.quarantine` clears it; Windows wants the
  directory on `PATH` and no symlink of `acq`; Linux needs glibc 2.35
  (the Ubuntu 22.04 runner); the data path per platform as the
  `directories` identity derives it. The release seat reaches the
  macOS archive's binaries and the recipe alone (the step 4 section's
  coverage paragraph); the Windows and Linux lines and the data paths
  are the platforms slice's VM rows and CI's smoke, the quarantine
  line the owner's at the tag.
- No verb names the leagues a store holds: `search --count league`
  does, and `price status` counts rows "for other leagues" without
  naming them. Trigger: the release seat stumbles on a league it did
  not name.
- `cancel` and `set-priority`'s refusal with no daemon says it spawns
  on demand for job commands, which reads as if they would start one.
  Trigger: the seat retries a cancel expecting a spawn.
- The README is at 95% of its budget, three of its knob rows
  development-only (`ACQ_KEYRING_ROUNDTRIP`, `ACQ_CASES`,
  `ACQ_CODESIGN_IDENTITY`), which the docs check's knob scan requires
  rows for. Trigger: step 4's fix round needs README room; the scan
  then exempts test-only knobs, or the rows route to their read-site
  docs.
- A `.tar.gz` made on macOS carries AppleDouble `._` entries beside
  each file: `tar` merges them on macOS and never lists them, Python's
  `tarfile` extracts them as files (found making the seat's world,
  2026-10-19). The release's two macOS archives carry them; harmless
  to a macOS user, untidy, and `COPYFILE_DISABLE=1` on the packager's
  `tar` removes them. For the fix round, one commit; the smoke cannot
  see them.
- `AGENTS.md`, `CONTEXT.md`, `README.md` and `decisions/search.md`
  are each past 90% of budget. Trigger: this slice's close.
