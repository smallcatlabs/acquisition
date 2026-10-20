# The release — closed record

Step 4 of `brainstorming-notes/36-the-road-to-the-agent-release.md`:
the agent surface — the CLI, the MCP server and the daemon on Windows,
macOS and Linux — released from GitHub as `v1.0.0-alpha.1`, a
pre-release (named beta until the close; the verdict below), with pricing on the MCP, the user-facing README and the
two policies built on the way. Opened 2026-10-18 at `05e6c70c`, closed
2026-10-19. This file is the permanent short record; the open record
is whole at `d0ab421e` (its step briefs, the fix round's triage and
every observation as it stood), and the brief `RELEASE-PLAN.md`, the
roles and rules the slice ran under, at the same commit. Nothing here
is a second authority.

No further step of the release is authorized by this record: the
owner's outward acts (below, "Final state") follow the close in the
brief's order, and what the first users teach opens under its own
record.

## Authority

The rulings this slice brought or amended are `decisions/frontends.md`
(C11: a revision conflict is `kind: "conflict"` with its revision on
both surfaces; C116: a refusal speaks the verb's words, no internal
name) and the owner's reading of `SECURITY.md` and `CONTRIBUTING.md`;
C3's lazy spawn was kept with its refusal made visible, and the shop
render is bounded on both surfaces under C74 and C53 (every omission
counted with its reason, the list behind a request). The README ruling is below. The
release's own mechanism is `.github/workflows/release.yml`,
`tools/release-package.sh` and `tools/release-smoke.sh`; the
whole-CLI seat's findings table (`audits/rehome-2026-10/seat/`) and
the records' shapes of fault bound every step.

## Final state

- **The release.** Five archives per tag (`release.yml`'s matrix, the
  names pinned by `readme_tour.rs`), each holding `acq`, `acqd`,
  `acq-mcp`, the licences and the third-party notices, with
  `SHA256SUMS`; the macOS binaries Developer ID signed and not
  notarized, repacked with `COPYFILE_DISABLE=1`; Windows unsigned; the
  smoke on every target runs the extracted `acq-mcp` and refuses an
  AppleDouble entry.
- **Pricing on the MCP** (`crates/acquisition-mcp`, `tests/pricing_tools.rs`):
  seven tools over `acquisition_plan::pricing`, the functions the CLI
  calls, each the document `acq … --json` prints; the writes
  create-only without a revision and stricter than the CLI's.
- **The README**, the user's, at `README.md` in the ruling's order;
  `readme_tour.rs` runs every `sh` block in a mock world, `no_ids.rs`
  sweeps it; `tools/docs-check.sh` holds its form (the tour and the knob table)
  (shipped knobs alone).
- **The policies**: `SECURITY.md`, `CONTRIBUTING.md`, V7's positions
  in the owner's voice; private vulnerability reporting enabled.
- **The seat**: `audits/release-2026-10/seat/README.md`, six runs on
  both surfaces, the fix round and the rerun; every fault's fate is
  its table.
- **The owner's acts, drafted and not sent**: the note to GGG under
  the registration's framing (the owner's own words); the C++ issues
  with a closing note each (`audits/release-2026-10/cpp-issues.md`);
  the tag `v1.0.0-alpha.1` on the close's last commit, the release's
  body `audits/release-2026-10/release-notes.md`, published as a
  pre-release; the reddit post.
- **Not built**, each parked with its trigger: the parks in "Where
  the open observations went", and the brief's "Left out" — the GUI,
  every channel beside the archives, PoE2, shop publishing.

## Before building — the ruling

**Where the user-facing README lives — ruled 2026-10-18: the user's
README is `README.md`.** Three options were put: the user's README at
`README.md`, the charter paragraph to `CONTEXT.md` and the crate index
to `AGENTS.md` as a table (A); a separate user guide, the README
unchanged (B); one README for both with a contributor tail (C).
Recommended A, with the tour kept as the fenced block `readme_tour.rs`
pins and the knob table whole, no decision id in the README (C116),
`AGENTS.md`'s budget raised for the crate table, the archive
unchanged. The owner, verbatim: "go ahead. this is a good plan."

## Step ledger

| Step | Commits | What landed |
| --- | --- | --- |
| the brief | `05e6c70c` | `RELEASE-PLAN.md`, for a fresh orchestrating session |
| 0 · the record, the ruling | `e1fde41d` | the record opened; the README ruling; the crate index to `AGENTS.md`, the charter to `CONTEXT.md` |
| 1 · pricing on the MCP | `db30d28a`, `40002b72`; review 1's fixes `b45a0a23`–`b7a5fa4b`; review 2's `7571a570`, `d7a262fc` | the pricing verbs moved into `acquisition_plan::pricing`; the seven tools; `tests/pricing_tools.rs`; the park deleted; an orphan row listed whole, the receipt's prior the row at the named revision, the write last, the clear opening intent alone, the smoke running `acq-mcp` |
| 2 · the policies | `6b3d1a10` | `SECURITY.md`, `CONTRIBUTING.md`; read by the owner before the commit |
| 3 · the README | `7490c275`, `9105ee3d`; review 3's fixes `21460844`, `c170c84d`, `d34e5606`; review 4's `036d0cb4`, `92f5f312` | six verbs' help say what the whole-CLI seat had to find; the README whole; the recipe's keyringless line; three blanket rules narrowed; the reference's global options |
| 4 · the seat's briefs and doors | `26603d0f`, `2044a994`; after reviews 5 to 7, `81e6e9bb`, `10902dbb`, `e2dbeb8f` | `BRIEF-cli.md`, `BRIEF-mcp.md`, the prompts; `daemon status` reporting its idle exit; the doors' deadlines whole, `doors_test.py` |
| 4 · the runs | `3ebf1329`, `de084c7d`, `c2746323`, `8af722b2` | cli-opus, mcp-opus, cli-sonnet, mcp-sonnet: calls, reports, debriefs in the record |
| 4 · the triage | `561910a3`, `91cdd4fa` | fifteen consolidated faults with a fate each; C11 and C116 amended, C3 kept, six parks; the workers' briefs |
| 4 · the fix round | B `f23b10bb`; C `e8f21169`, `e35b5d06`; A `add1088a`–`a6d66df0`; review 8's `b8820b35`; the record's `081cf561`, `8ce6258d` | the refusals and texts; the README and the archive; the shop bounded, `price list` limited, `kind: "conflict"`, every undo runs as printed; review 8's seven |
| 4 · the rerun | `ead41b14`, `ce5b2d30`, `31ab4bf4`, `9a0b6bca`, `db5126fe`, `be4ba120`, `d79c4ca7`, `2fa60e8f` | cli-opus-2, mcp-opus-2 on the fixed build: the first rerun's four, the remedy's provider and store, every MCP refusal an `isError` result, `left_out` bounded, the README's own-account configuration; no further rerun |
| 5 · the outward acts, drafted | `d07b75af`, `10530b78` | the C++ issues table; the release notes and the owner's acts in order |
| 6 · the close | `a07d5808`, `c6614be0`, then this record and the brief's deletion | the seat's record with its fate table and the re-read count; the briefs and prompts removed; the worlds torn down; this record; the brief deleted |

## Shapes of fault — the review checklist

Eight outside reviews (Astra's Codex pass over each step's commits),
the fix round and the rerun taught these; each came back until it was
named. The earlier records' tables apply whole.

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A claim wider than its evidence.** A sentence written for the common case and read as a rule: "Standard unless you name a league" of every verb; "every command talks to the real GGG"; "the rest read offline"; a seat observation saying the seat confirms every README line; a runner's "every time" | reviews 3 and 4 (`c170c84d`, `036d0cb4`), review 6 (4), the debriefs | each bullet names its verbs; the coverage paragraph says what the door does and assigns the rest; the debrief asks what the runner would retract |
| **A fix staging the nearest failure, not the named one.** A test green on a daemon the recipe never started; a deadline that waited for a byte and then for a line; a write run before the timed wait | review 4 (`92f5f312`), reviews 6 and 7 (`10902dbb`, `e2dbeb8f`) | `the_agent_recipe_logs_in_again_after_the_daemon_stopped` reads running=false first; `doors_test.py` stages a stall, half a line, non-JSON, an exit and a linger, and hung the doors before |
| **A sibling sentence left standing.** The fix made where the fault was met and not where the same words stood: `show`'s refusal naming a command that printed nothing; five daemon tools made refusal documents and two more tools still JSON-RPC errors; an undo promised for a prior the write refuses | review 2 (`7571a570`), the rerun (`be4ba120`), review 8 (`b8820b35`) | the test runs the command as the refusal types it; one helper sorts every tool's refusal (`tool_refusals.rs`); `undo_words` runs the writer's rule |
| **A refusal that says what is wrong and not what to do.** No daemon, said without the line that starts one; a remedy naming no provider or store, so typed as printed it starts a daemon for another world; a next step that dropped its league | the triage (4), the rerun (`db5126fe`, `ce5b2d30`) | `a_job_tool_with_no_daemon_names_the_command_that_starts_one`, `the_no_daemon_remedy_names_the_provider_and_store_the_server_runs_with`; status's next step runs as printed |
| **A document unbounded by default.** The shop render at 5 MB, its `left_out` at 160 KB after the first bound; `price list` whole at 67 KB; a tab's `price show` | the triage (1, 2), the rerun (`d79c4ca7`) | `the_shop_document_lists_what_is_left_off_for_a_reason_and_counts_what_nothing_applies_to`, `the_default_shop_document_lists_a_bound_of_what_is_left_out_and_counts_it_all`, `price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many`; the door's cut, which names the rest |
| **A printed command that does not run as printed.** An undo without its account, a target unquoted; a recipe line answering not_logged_in; a configuration block that, copied, named the stand-in | the triage (6, 9), review 3 (`21460844`), review 8, the rerun (`31ab4bf4`) | `every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known`, `an_undo_quotes_a_target_a_shell_would_split`; `readme_tour.rs` runs every line of every `sh` block |

## What the reviews and the runs taught

A number here was measured, never recalled; each is its source's.

- The seat (`audits/release-2026-10/seat/README.md`): six runs, 473
  calls; every task finished on every run; the first four met the
  same leading faults on both surfaces, the reruns met the fix round's
  fixes as printed and found eight small ones. Four second calls for
  the same answer across the six runs; five or six reported re-reads
  each, laid to five causes, four of them parked below.
- The read economy's cost was size, not calls: a document cut by the
  door on every run of the first four (the shop at 5 to 7.5 MB,
  `price list` at 67 KB); after the bounds the shop's JSON alone, at
  208 KB and 160 KB, its `left_out` then bounded at 50.
- The fix round: fifteen consolidated faults, each with a fate —
  built now, kept as a ruling's consequence made visible, parked with
  a trigger, or no fault — and the owner's triage drew the line
  before building.
- The outside reviews: eight rounds of findings (the table at
  `d0ab421e`), every one fixed by a rule and held by a test or a
  sentence; the doors' reviews (6 and 7) found, before any run, a
  reader that could hang past its deadline.
- Two workers in parallel worktrees shared one scratchpad directory
  and one overwrote the other's file; a brief for a parallel round
  names each worker's own scratch subdirectory.
- A worker's commits carried the model that did the work in
  `Assisted-by:`, where the brief said the session's line; kept.

## Owner verdicts, verbatim

- The README ruling (2026-10-18): "go ahead. this is a good plan."
- Step 2, on the two drafts: "I have enabled private security
  reporting. I will probably edit the drafts somewhat, but that will
  take time and thought. What you have written is a good start. Let's
  use them."
- Review 1's fixes, on the five rules and the brief's amended line,
  all built now: "approved. go ahead."
- The fix round's triage, every proposed fate taken: "yes, go ahead.
  use subagents to save context or tell me when you'd advise a fresh
  session, or do the work yourself if it's related."
- The rerun's round (2026-10-19): no further rerun, by the owner's
  triage of time against gains; the close to a fresh session.
- The release's name, at the close (2026-10-19), asked what would
  have to change to make it an alpha, with no GUI yet and the feature
  sets untested in the wild: "Yes, make this an alpha release."

## Where the open observations went

| Observation | Went to |
| --- | --- |
| A revision conflict without a `kind`; the refusals naming the CLI's flags in an MCP session | `b24e9e05`; the CLI-words park (`decisions/frontends.md`) |
| Whether a target's two forms cost a re-read | counted: no run spent a call on it (the seat's record, count 1); the read economy park re-triggered (`decisions/frontends.md`) |
| `acq-mcp` connects per call and a real-mode daemon exits idle | the README's recipe (`e8f21169`), confirmed by the seat; the keeper park (`decisions/daemon.md`) |
| Two corpus tests failed a gate twice and passed alone | built (`c6614be0`): the test's temp directory is removed before use, a pid coming round again; the store park deleted |
| `PRICING-SLICE.md`'s fixture citations by a renamed file | rewritten as "schema N, at `<commit>`" |
| `list`'s refusal for a container with a row of its own | parked (`decisions/pricing.md`); no run reached it |
| `price_clear`'s tool text names no `if_revision` for the undo | folded into the clear-receipt park (`decisions/frontends.md`) |
| The README's unrecorded guidance (quarantine, Windows `PATH`, glibc, the data paths) | the seat's record, count 2: the recipe and the archive's binaries confirmed; the rest the platforms slice's VM rows, CI's smoke and the owner at the tag |
| No verb names the leagues a store holds | built before: `store status` lists them (`9ea55826`) |
| `cancel` and `set-priority`'s no-daemon refusal reads as a spawn | built: the refusal names what starts one (`f23b10bb`) |
| The README at 95 % with three development-only knob rows; the AppleDouble entries | built (`e8f21169`, `e35b5d06`, `b8820b35`) |
| The README's room for the next addition; the checksum step compared by eye | parked (`decisions/frontends.md`) |
| No verb lists the notes `price status` counts invalid; `residue` and the store's `unlifted`, `withheld`, `granted`, `intent` never understood | parked (`decisions/pricing.md`, `decisions/store.md`) |
| GNU `tar` on AppleDouble entries | dropped: the archives carry none, the smoke refuses them |
| `characters_wire.rs`'s mock login failed one gate under another worker's build | dropped: a gate under a concurrent build is C84's artifact mismatch; alone it passed |
| Two workers sharing one scratchpad | the lesson above |
| The rerun worker's parks: the remedy's daemon answering not_logged_in without a keyring; `posted` and `freshness` unbounded; `set_sync_policy`'s conflict kind; the job tools' malformed arguments; the empty `outputSchema` gone | parked with their triggers (`decisions/frontends.md`, `decisions/pricing.md`) |
| `price_status`'s overlapping counts; the shop page's `StashN` rank; the scope's "newest" age beside the tabs' | parked (`decisions/pricing.md`, `decisions/search.md`): the seat's re-read causes |
| Four always-loaded documents past 90 % of budget | dropped: the check is the signal, and the `AGENTS.md` row shrank at the close |
