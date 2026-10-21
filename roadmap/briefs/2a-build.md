# 2a — The shared function layer: the build (Opus)

The second half of step 2a. The design is `2a-design.md`; this brief is
its report, and the rulings it needs are listed there (section 4) and
land in the registry before the code, as commit 1 below.

## Goal
Every verb's semantics exists once, in Rust, in a new crate
`acquisition-verbs` that the CLI and the MCP call and the GUI will
call: the plan compile, the quote, the apply loop with its two
documents, the account resolver, the openers, the daemon and queue
reports, the job verbs, the policy and events reads. Each frontend
parses its arguments, calls the layer, and renders or serialises the
document it gets back; neither computes an answer. One fault type with
one sorter per frontend gives every failure its kind on both surfaces,
in the protocol's words where the protocol has them.

## Read
- `roadmap/README.md`, then `2a-design.md` whole: sections 2.2 and 2.3
  are the signatures and the fault type to build (a drawing: the tree
  decides the details, and the report names each change); section 3
  lists what each frontend loses; section 1 is the table of
  duplicates, every row of which this build closes.
- `AGENTS.md` C1 (the edges), C12 (two surfaces), C34 (the daemon is
  blind to intent), C35 (intent is irreplaceable), C46 (every frontend
  an adapter), C47 (no panics on input); `decisions/frontends.md` C11
  (the failure document), C13 (the MCP never spawns in real mode), C52
  (no blind clobber), C53 (JSON is additive), C116 (no id in user
  text); `decisions/plans.md` C44 and C111 (the spend gates);
  `decisions/daemon.md` C3 (a job verb with no login still spawns),
  C10 (the doors), C45 (the ledger answers), C85 (the closed kinds).
- `crates/acquisition-plan/src/pricing.rs`, the module doc and
  `Expect`: the shape the layer copies.
- `crates/acquisition-cli/src/plan_cmd.rs` (`refresh_plan`,
  `refresh_apply`, `try_quote_within`, `check_plan_applies`,
  `StoreChanges`, `spent_phrase`, `open_intent`, and the renderer tests
  under its `#[cfg(test)]`, which call `plan_refresh`),
  `crates/acquisition-cli/src/main.rs` (`error_report`, `client_of`,
  `noting_the_spawn`, `persisted_as_info`, `print_persisted_*`,
  `DaemonCmd::Status`, `submit`, `wait_for_job`, `tally_children`,
  `progress_line`, `parse_children_failure`, `report_apply`,
  `print_result`), `crates/acquisition-cli/src/store_cmd.rs`
  (`resolve`, `open`, `auth_status_absent`, the accounts listing,
  `summarize_events`), the `Store::open` sites in `search_cmd.rs`,
  `price_cmd.rs` and `shop_cmd.rs`, `crates/acquisition-mcp/src/main.rs`
  (lines 145 to 700, the helpers; `refresh_plan`, `apply_plan`, the job
  tools, `daemon_status`).
- `crates/acquisition-client/src/client.rs` (`ConnectOptions`,
  `ConnectError`, `Observed`, `DaemonError`, `Unresponsive`,
  `Client::quote`) and `crates/acquisition-client/tests/contract.rs`
  (`session`, `acqd_for_tests`, `place_link`, `start_daemon`, `login`:
  the in-process mock harness the layer's tests copy).
- `crates/acquisition-daemon/src/daemon.rs`, `maybe_finish_parent`:
  the children summary to type; and every `Outcome::Failure {` in the
  daemon and the CLI (52 sites), each a constructor or a pattern.
- `crates/acquisition-protocol/src/job.rs` (`Outcome`) and
  `crates/acquisition-protocol/tests/wire.rs` (the fixture that pins
  the wire: one sample per response variant, read its rules before
  adding one).
- `tools/arch-check.sh` (the edge table's comment and the intent grep,
  the model for the new grep), `tools/arch-check-breakers.sh` (its
  save and cleanup lists name the files it stages) and
  `tools/docs-check.sh` (the `AGENTS.md` budget).
- The tests named under Input: read each before the code it pins
  changes.

## Input
Fixed before the work starts:
- The owner's rulings on `2a-design.md` section 4, given 2026-10-21,
  verbatim: "All rulings are approved. Proceed." Recorded as commit 1
  below with those words in its message. His permission of 2026-10-21
  to raise the `AGENTS.md` budget for this phase, quoted there too.
  The design's two open questions closed with it: the crate is
  `acquisition-verbs`, and `job_status` with no daemon answers the
  ledger's row with `next`.
- The documents the tests pin today and this build changes, each named
  so the change is deliberate and not drift:
  - `crates/acquisition-mcp/tests/tool_refusals.rs`
    `a_tool_refusal_is_a_result_marked_is_error_with_its_kind` and
    `crates/acquisition-mcp/tests/search_tools.rs`
    `the_requests_account_selects_the_store`,
    `a_missing_store_names_the_directory_it_looked_in`: `account_unknown`,
    `account_needed`, `no_accounts` and `readings` become
    `not_logged_in`, `ambiguous_account` and `accounts`.
  - `crates/acquisition-cli/tests/seat_faults.rs`
    `an_account_that_cannot_be_picked_says_which_failure_and_lists_the_accounts`:
    `unknown_account` becomes `not_logged_in` with `accounts`.
  - `crates/acquisition-mcp/tests/plan_loop.rs`
    `the_mcp_tools_carry_the_plan_slice_and_its_gates`: `apply_plan`'s
    documents gain `skipped` and `remaining_requests` and keep
    `applied`, `requests`, `job_id` and `spent`.
  - `crates/acquisition-mcp/tests/daemon_status.rs`
    `a_job_tool_with_no_daemon_names_the_command_that_starts_one` and
    `the_no_daemon_remedy_names_the_provider_and_store_the_server_runs_with`:
    both ask `job_status` for job 1 in a world with no ledger. A job the
    ledger does not hold with no daemon becomes `unknown_job` with
    `next`, so the first test keeps a missing-job case under that kind,
    seeds a ledger row for the answered case (`running: false`, the row,
    `next`), and the second moves its remedy assertion to `cancel_job`,
    which still refuses `no_daemon` with `next`, as `submit_job` does.
  - `crates/acquisition-mcp/tests/persisted_queue.rs`
    `c45_the_tools_read_the_queue_on_disk_when_no_daemon_answers`: its
    equality stands (no log exists there); a new test stages a log
    file and reads `log` on both surfaces.
  - `crates/acquisition-cli/tests/apply_loop.rs`
    `a_failed_apply_counts_its_failed_requests_from_the_daemons_own_words`
    and the unit test `a_parent_failure_line_is_read_back_whole` in the
    CLI's `main.rs`: the count is read from `children`, and the unit
    test over the sentence is deleted with the parser.
  - `crates/acquisition-cli/tests/plan_json.rs`
    `json_stdout_is_the_contract_surface`: stands (`policy show --json`
    stays the row or null; the plan stays the bare envelope).
- Failing inputs the layer's tests stage, each a test named for it: no
  index file; an index with two accounts and no selector; a selector
  matching none; a bare name shared by two discriminators; an entry
  without a uuid asked for intent; a blind policy write over a stored
  row this build cannot read; a plan whose policy revision moved; a
  plan applied twice; a plan with every request held by a queued job
  (`remaining_requests` null); a daemon absent for the quote (the one
  note), for `job_status` (the ledger's row with `next`; a job the
  ledger does not hold, `unknown_job` with `next`) and for `job_result`
  of a finished job (its outcome from disk); a parent whose child
  failed (the `children` field read back).

## Do
1. **The rulings** (commit 1, documents only, no review): C1 amended
   and C46's Pinned in `AGENTS.md` with the workspace table's new row;
   the new frontends ruling added and C23 and C45 amended in their area
   files, each verbatim from `2a-design.md` section 4 unless the tree
   disproves a word; the `AGENTS.md` budget in `tools/docs-check.sh`
   raised to what the file then measures plus a hundred bytes, the
   owner's permission quoted in the message. `python3 -I tools/bothy.py
   check` and `tools/docs-check.sh` green. Between this commit and the
   last, the registry names tests that still assert the old words, and
   the Fable session's "Current step" in `roadmap/README.md` says the
   build is under way (the worker does not edit that file).
2. **The protocol** (commit 2): `Children` in `job.rs`,
   `Outcome::Failure { error, children: Option<Children> }` with the
   field defaulting and skipped when absent, and `Outcome::failure(message)`
   for the constructors; every `Outcome::Failure {` constructor in the
   daemon and the CLI calls it and every pattern gains `..`; the daemon
   fills `children` on every parent failure in `maybe_finish_parent`:
   the two arms it builds and the deferred failure a restart mid fan-out
   installs, whose message is kept (the daemon's
   `restore_drops_probes_and_finishes_a_parent_whose_children_are_done`
   then reads the field too); `wire.rs` gains an `Outcome` sample for a
   failure carrying `children`, inside the fixture's own rules. Build,
   then test: the contract revision has moved.
3. **The crate** (commit 3): `crates/acquisition-verbs` as design
   sections 2.2 and 2.3 give it, a workspace member, linking client,
   protocol, store, plan, search, tokio, serde, serde_json and anyhow,
   with reqwest as a dev-dependency for the login; the entry doc
   naming `decisions/frontends.md` first as the other crates do; the
   `unwrap_used` and `expect_used` lints denied as the plan crate denies
   them (C47). Modules by subject, the worker's split (`fault.rs` apart
   at least). Every public function's doc comment names the ruling it
   serves by id with a gloss. Nothing prints. The default league is one
   constant the frontends and the layer share (`pricing::DEFAULT_LEAGUE`,
   re-exported or moved to the plan crate's root; the worker's call,
   said in the message). The realm's refusal sentence is one function
   both frontends call. The arch check's edge-table comment names the
   crate; no forbid entry is added (design 2.1).
4. **The CLI** (commit 4): every duplicate in design section 3 deleted
   and replaced by a call, the `Store::open` sites in `search_cmd.rs`,
   `price_cmd.rs` and `shop_cmd.rs` included (their verbs are
   untouched); `error_report` becomes one match over `Fault`, each
   variant with its remedy in the CLI's words, plus the two existing
   branches for pricing's `Refused` and the daemon's kind; the statics
   `SELECTOR` and `ACCOUNT` go; `SPAWNED` stays, set from the client
   the layer's `connect` returns; `report_apply` reads `children`; the
   renderer tests in `plan_cmd.rs` build their plan through the layer's
   `compile`. The text renderers are untouched except where their input
   type changed. `--json` on `refresh --apply` prints one document:
   `Landed` after the wait (text mode prints the "applying" line from
   `Submitted` first, as today).
5. **The MCP** (commit 5): the helpers in design section 3 deleted;
   `failed` becomes one match over `Fault` deciding JSON-RPC error
   against `isError` result, each refusal with its remedy in the MCP's
   words (the `account` argument, `scope.account` on search), plus the
   existing pricing and search branches; `job_status` and `job_result`
   answer from the ledger; `job_result` carries `job_id` and `account`;
   `apply_plan` returns the `Apply` document; `refresh_plan` returns
   `Planned`; `daemon_status` returns `DaemonReport`; `accounts` rows
   gain `store`. Tool descriptions say the new words where they named
   the old.
6. **The door grep** (in commit 5, when the last door has left the
   frontends): in `tools/arch-check.sh`, over `crates/acquisition-cli/src`
   and `crates/acquisition-mcp/src`, the nine names design section 4
   item 3 lists, worded as the intent grep is and citing C46; one
   breaker case per name in `tools/arch-check-breakers.sh`, its save
   and cleanup lists extended for the staged file, riding in the same
   commit.
7. **The tests** (with commits 3 to 5): the layer's own under
   `crates/acquisition-verbs/tests/`, one file for the pure documents
   (the fault kinds and fields, the ledger mapping, the apply arms over
   a fixture store, the quote note's sentences; values built directly,
   no environment) and one daemon-backed file over the mock, using the
   contract tests' harness copied (`session` and its lock,
   `acqd_for_tests`, `place_link`, `start_daemon`, `login`; about 150
   lines; not shared: step 2b makes the one harness); the frontends'
   tests under Input updated to the new words and shapes, nothing else
   in them changed. Do not prune the frontends' suites beyond what this
   build breaks: 2b owns the reduction.
8. **The references** (commit 5 or its own): `ACQ_UPDATE_FIXTURES=1`
   over `cli_reference.rs` and `mcp_reference.rs`; the README's tool
   lines checked (`readme_tour.rs` runs them).
9. **The gate**, bare, each stage's status its own; `tools/arch-check-breakers.sh`
   run once for the new cases; then the review as `roadmap/README.md`
   says, over the range `<commit 2>^..<last>`.

## Done when
- `crates/acquisition-verbs` exists, is a workspace member, links
  client, protocol, store, plan, search, tokio, serde, serde_json and
  anyhow, and `tools/arch-check.sh` reports the edges and the door grep
  green.
- `try_quote`, the apply loop, the compile with its no-policy case,
  `open_intent`, the store opener, the account resolver, the
  `daemon_status` shaping, the persisted-job mapping and the children
  parser each exist once, in the verbs crate: `grep -rn` over
  `crates/acquisition-cli/src` and `crates/acquisition-mcp/src` finds
  none of `Index::load`, `Store::open`, `Annotations::open_for`,
  `plan_refresh`, `check_spendable`, `apply_params_of`, `.quote(`,
  `Persisted::observe`, `Client::connect`, `parse_children_failure`,
  `persisted_as_info`.
- The CLI's and the MCP's plan answers are the one `Planned` document
  (the CLI's stdout its `plan` member, its note on stderr); their apply
  answers are the one `Apply` document, the CLI's `--json` printing
  `Landed` alone after the wait; the MCP's apply documents carry
  `skipped` and, all spent, `remaining_requests`.
- One account vocabulary: the strings `account_needed`,
  `account_unknown`, `no_accounts` and `unknown_account` occur nowhere
  under `crates/`; the tests named under Input assert `not_logged_in`,
  `ambiguous_account` and `accounts`.
- `Outcome::Failure` carries `children`; the daemon fills it; the wire
  fixture holds a failure sample with it; the CLI reads it.
- The MCP's `job_status` and `job_result` with no daemon answer from
  the ledger, pinned in `crates/acquisition-mcp/tests/persisted_queue.rs`.
- The layer's tests stage every failing input under Input, each test
  named for its fault.
- The gate is green on the step's own tree; both references
  regenerated; the ledger row written.

## Not in scope
- The GUI server and the held corpus (phase 4).
- Pricing's and the search's verbs: they stay in their crates; only
  their openers go through `Account`. Their refusal kinds (`conflict`,
  `not_found`, the search's) stay sorted by their existing helpers
  until step 2g folds every refusal into one shape (issue #10). A
  policy revision conflict stays `refused` for the same reason (filed
  from 0b).
- The test reduction, the one harness, goldens, one binary per crate
  (2b). Copy the contract harness; do not generalise it.
- The daemon file split (2d) and a typed `JobKind` (deferred).
- `dash`, `jobs --watch`, `auth status`, `daemon stop` and
  `reset-tripwire`: they observe through the client as today and are
  otherwise untouched.
- A new MCP tool or argument, a new CLI verb or flag; any change to the
  CLI's `accounts --json` or `policy show --json` shape (C53).
- Any change to the plan envelope, the search answers, the pricing
  documents or the daemon's limiter, gate or providers.

## Rules
`TEMPLATE.md`. In addition: build before every test run (the contract
revision moves in commit 2), and never with a daemon of this world
running.

## Report
`TEMPLATE.md`. Under 5, say which of design section 2.2's signatures
you changed while building and why; the design is a drawing, the tree
is the authority.
