# 2b — The tests: the design (Fable)

The roadmap's step 2b in one line: "one harness, one layer per
behaviour, goldens, one binary per crate, nextest". This document is
the design for it, drawn from four read-only surveys of the tree at
`3bc71a86` (the harness copies, the cross-layer duplicates, the prose
literals and the fixture mechanisms, the binaries with a measured
nextest run) and the assessment's test report
(`roadmap/STATE-2026-10-19.md`, "Report 2"). The owner reads this and
rules on section 4 before `2b-build.md` runs. Every count below is as
of `3bc71a86`; a line number is as of that commit.

## 1. The problem, measured against the tree

The suite is sound where it matters most: the daemon's limiter and gate
tests, the journal invariants, the protocol fixtures, the client
contract, the store and plan units, and the search crate's data-driven
tables and metamorphic properties. None of that changes here. The
excess is in how the tests are built and where they sit:

- **Four harnesses and many copies.** The CLI's `tests/harness/mod.rs`
  (378 lines), the MCP's (469), the harness half of the client's
  `tests/contract.rs`, and the copy of that half the 2a build made in
  the verbs crate's `tests/daemon.rs`, which has already drifted (it
  lost the sibling assertion and the daemon's name on panic). Beside
  them, private copies in ten CLI test files and the MCP's
  `plan_loop.rs`: ten `sole_json`, six `http_get`, four `place_link`,
  eight scratch types. The environment scrub lists disagree
  (`ACQ_JOURNAL` is `0`, a path, or removed; `ACQ_IDLE_SHUTDOWN` is 30
  or the 60 s default), and three knobs are scrubbed nowhere
  (`ACQ_JOB_RETENTION_DAYS`, `ACQ_FAILED_JOB_RETENTION_DAYS`,
  `ACQ_MOCK_DEGRADED_HEAD`): a value in the shell reaches every test
  daemon.
- **One behaviour pinned at two or three layers.** Of 152 frontend
  tests read, 14 pin only what a lower crate's test already holds, 12
  pin a lower behaviour that has no test at its own layer, and 27 mix
  the adapter's part with the semantics. The equality check the
  assessment proposed already exists: every MCP search test runs its
  request through `acq --json search` too and compares (about 70
  spawns), and since both frontends call the same four search
  functions it can hardly fail. The tabs listing (`Tabs::of`) has no
  test in the search crate at all; its four pins are frontend tests.
- **Prose literals.** 395 string assertions in the CLI tests and 66 in
  the MCP's, plus 97 in loops. About 137 of the CLI's and 19 of the
  MCP's are sentence-length wording. A rewording is three edits in
  three files, and the 250-character tool description one MCP test
  asserts is already held whole by `MCP-REFERENCE.md`. The five
  fixture checks that exist write the file and pass under
  `ACQ_UPDATE_FIXTURES=1`, and four of them fail with one sentence and
  no diff.
- **72 integration binaries.** Search 38, CLI 20, MCP 9, protocol 2,
  verbs 2, client 1; 87 with the units and examples. The search
  binaries alone are 1.05 GB of executables and 2.48 GB of debug
  bundles per build. Touching the search tests' shared module relinks
  32 binaries in 13.5 s; touching the search library relinks 75 in
  21.8 s. On macOS about nine tenths of a link is debug-info packing.
- **The runner.** `cargo test` runs binaries one after another: 94.5 s
  for the suite's 986 tests. 27 s of that is the client contract's
  nine serial tests, and 20 s of those 27 are two tests each waiting
  out the client's full 10 s stop bound against a scripted peer that
  never exits. A measured `cargo nextest run` over today's files took
  29 to 30 s (54 s with `ACQ_CASES=full`), 986 passed, no leaks, no
  new failures, and it rewrites `acq-mcp` exactly as `cargo test` does,
  so the build-order rule stands unchanged.

## 2. The shape

Five moves, each structural where it can be, in the order the build
takes them.

### 2.1 One harness: a shared file, one isolation list

The process harness becomes one file,
`crates/acquisition-client/tests/harness/mod.rs`, which the client's
`contract.rs` takes as `mod harness;` and the verbs, CLI and MCP tests
include by `#[path]`. It is not a crate: a crate is a C1 boundary with
a ruling, a table row, a contract-revision move (the protocol's digest
covers the root manifest and lock) and a new arch-check rule, and it
could not name a frontend's binary anyway (`CARGO_BIN_EXE_*` exists
only in the owning crate's tests). Every consumer already links the
file's dependencies normally (client, protocol, store, tokio,
serde_json), so no manifest changes; the file sits under `tests/`,
where the structure checks do not look. The crate stays the upgrade
path when the GUI's backend is a fifth consumer (P2).

The core of it is **one isolation list, two appliers**:
`isolation(world, base)` names every knob once: first every `ACQ_*`
the process holds, removed by prefix (so a new knob needs no list kept
in step), then `ACQ_STORE_DIR`, `TMPDIR` and `XDG_RUNTIME_DIR` under
the scratch, `ACQ_LOG_DIR`, `ACQ_NO_KEYRING=1`, `ACQ_JOURNAL=0`,
`ACQ_IDLE_SHUTDOWN=30`, `ACQ_PROVIDER=mock`. `isolate(cmd, base)`
applies it to a child with `Command::env`; `session(tag)` applies it to
the test's own process under the one lock, for the four files that
drive an in-process `Client` or `Frontend`. A daemon is always started
from the explicit list, never by inheriting a mutated environment. A
caller's later `.env(..)` wins, so a test still opts into the real
provider's refusal path, a journal path or `ACQ_NO_SPAWN`.

Shared beside it: `scratch(tag)` from the process's original temp dir
(so a session's `TMPDIR` never nests the next scratch past the socket
cap), `acqd_beside(frontend)` and `acqd_built()` with the message that
names `cargo build --workspace`, one `link_beside_this_test` with the
sibling assertion, `Proc` (killed and reaped on drop, named on panic),
`daemon_command`, `start_daemon` polling `acq daemon status --json`
and `start_daemon_in_process` polling `Client::observe`, one
`sockets_under` per platform, `sole_json`, `text`, `http_get`, one
`approve` that encodes `#`, `login` and `login_in_process`,
`compatible_client`, `wait_all_terminal`. `reqwest` leaves the client's
and verbs' dev-dependencies.

The CLI and MCP keep a thin `tests/harness/mod.rs` each, about thirty
lines, binding their own binary: today's names (`command`, `acq`,
`start_daemon`, `login`, `login_with`) over the shared ones, so the
nine using files change no call sites; the MCP's `Mcp` stdio client,
`run_to_exit` and its no-spawn `acq` stay in its adapter (one consumer
each, and `run_to_exit`'s bounded reaping is a checklist row). What
stays out of the shared file: `contract.rs`'s raw frame client and
scripted peers (its subject, C85), scenario helpers (`world`, `row`,
`ledger`, `stage`), `world.rs`'s own daemon start (it accepts a
non-mock provider and pipes stderr). Appendix C has the API as
signatures.

Behaviour the merge changes, accepted knowingly: every `ACQ_*` is
scrubbed; the journal is off everywhere unless a test names a path
(no test reads the default one); idle shutdown is 30 s everywhere; the
offline tests gain `TMPDIR`/`XDG_RUNTIME_DIR`/`ACQ_LOG_DIR` under their
base; the login encodes `#`; the verbs suite regains the two assertions
its copy dropped.

### 2.2 One layer per behaviour

The rule: a behaviour is pinned by a test at the lowest crate that
holds it, and a frontend's tests pin what the adapter adds. For the
MCP that is: an argument the tool cannot read (`arguments`), the
account selector and its remedy words, `failed`'s sort of JSON-RPC
errors from `isError` results, `fault_words`, `told`'s text equal to
the structured content in order (C115), size budgets, the defaults
(`DEFAULT_LEAGUE`, the realm, `Expect::Create` under C52, the tabs
`Selection`), the door (never spawns in real mode; store tools reach
no daemon), transport depth. For the CLI: clap and usage, streams and
exit codes (C11), text as a function of JSON (C53), flag-to-request
mapping, printed commands that run as printed, no ids in user text
(C116), and its text renderings.

Against that rule the 152 frontend tests in scope sort as **14
deleted, 12 moved down (11 of them then deleted), 27 thinned, 99
kept**: about 1,000 lines of frontend tests go, and nine new lower
tests appear where a behaviour had no pin at its own layer. The named
list is appendix A. The move-downs are the real gain: `Tabs::of`
(counts first, the ten with the most items, `expand`, `Selection`)
gets a test in the search crate; so do `list_seen` under a fixed
league, the exact trade clause's slim shape, a count key with a comma,
the cross table's limit, the unknown count key's refusal, the bare
word read as `name:` first, the keys a touched term carries, and the
request-level trade rules; the store's `league_list`; the verbs
crate's job list and daemon report over a ledger with rows and the
Running and Incompatible arms of `DaemonReport`; the plan crate's
`"sub": null` spend and the pricing verbs' limit, conflict and
unreadable-row refusals (the CLI's pricing tests stay too, two being
checklist holders).

The MCP's per-test equality with `acq --json search` shrinks to one
test over four requests (an answer, a refusal, a continuation, an
offer) beside the key-order test that can see what the equality
cannot. Two adapter gaps get a test each: `show`'s malformed `against`
refused as `tree`, and `item_events`' `hours` with its defaults.
`language.rs`'s fixture self-check stays, named for what it is; the
ignored completion measurement stays.

### 2.3 Goldens with facts

A `golden(file!(), name, &transcript, facts)` in the CLI adapter
compares a normalised transcript against
`crates/acquisition-cli/tests/goldens/<test file stem>/<name>.txt`,
one file per test, holding every `acq` the test ran as
`$ acq <args>`, its stdout, its stderr and a non-zero exit, streams
kept apart. On a mismatch it prints a unified line diff (a hand-rolled
LCS of about forty lines in the shared harness; no new dependency) and
the regeneration command; under `ACQ_UPDATE_FIXTURES=1` it writes the
file and passes, the convention the five existing checks already
follow. The same diff function serves `cli_reference.rs`,
`mcp_reference.rs` and the two plan fixtures, which today fail with a
sentence.

`facts` is required and non-empty: the one or two words the seat
finding was about ("MapStash 3 of 3", "fetched 3d ago"), checked
against the raw text before the update switch, so a blind
regeneration that lost the fact still fails. That is the structural
guard; the second, in words, is one sentence added to
`REVIEW-PROMPT.md`: every changed file under `tests/goldens/` or a
`*-REFERENCE.md` is read as a diff of what a user now reads, and a
change no source change in the commit explains is a finding. A
commit-message rule adds nothing the reviewer does not already check,
and a "golden changed only with a source change" check would never
have fired: all 59 reference regenerations since 2026-09-20 also
changed `crates/`.

Normalisation, named rules in order: line endings to `\n`; the scratch
base (as given, canonicalised, and backslashed on Windows) to
`<scratch>`; `N[smhd] ago` to `<age> ago` (the seeded searches render
ages of about 20,700 days that grow daily); the basis line's store and
build digests to `<store>` and `<build>` (the build digest changes with
every source edit); the continuation token to `acq1.<token>`; the
version to `<version>`. Job ids are not normalised in this step: the
daemon-backed texts (about 17 sites) wait for 2c's journey golden to
prove that rule, and stay literals here.

Text only, CLI only: JSON is the contract and stays asserted by field;
the MCP's wording is its tool text, which `MCP-REFERENCE.md` holds, so
its sentence literals shrink to fact words with no new golden. About
20 to 23 goldens result (search_json about 8, seat_faults about 9,
shop, price and currency about 4, two refusal walks), replacing about
105 literal sites; the test Rust shrinks by 300 to 400 lines and 1.5
to 3 K lines of `.txt` are committed. A golden is worse than the
literal, and is not used, for measured sizes, printed commands that
must run, literals built from computed values, negative asserts,
single-line refusals and the path-heavy world-lock messages. One site
(`seat_faults.rs:1330`) checks a printed pointer by its ending and
then runs a hand-typed command; it becomes a parse-and-run test.
`docs-check.sh` gains one line: every golden file has a `golden(…,
"<name>"` in its stem's test file (no orphans).

### 2.4 One binary per crate, with one exception

Each crate's `tests/*.rs` become one `tests/main.rs` of `mod` lines
(the search crate's `common` compiled once instead of 32 times), except
a file that sets the process environment, which stays its own binary:
the CLI's `watch_recovery.rs`, the MCP's `plan_loop.rs`, the verbs
crate's `daemon.rs` beside `documents.rs`, and the client's
`contract.rs`, already alone. Measured in scratch, merging the CLI
with `watch_recovery.rs` fails six `world` tests and merging the MCP
with `plan_loop.rs` fails nine, both from the leaked environment;
without those files, 124 and 59 pass. The exception is structural: the
binary boundary is what isolates a process-wide write, and it holds
for whoever runs plain `cargo test`. `world.rs`, `readme_tour.rs`,
`daemon_observe.rs` and the two reference checks merge (measured
passing); the references' documented regeneration command changes once
in the seven places that print or name it.

Result: integration binaries 72 to 9 (search 1, CLI 2, MCP 2, verbs 2,
client 1, protocol 1), all binaries 87 to 24. Measured on the scratch
merge: the search relink after a touch of its shared module 13.5 s to
6.7 s and its run 36.7 s to 14.4 s; the CLI's 19 merged files 20.4 s
to 7.6 s; disk about 3.4 GB less per build; `cargo test` end to end
about 94 s to 59 s, then bounded by the contract's 27 s.

`tools/arch-check.sh` holds the layout: under each crate's `tests/`,
a `main.rs` and, beside it, only files that call `session(` (or name
`env::set_var`), each of which `main.rs` must not `mod`; with breaker
cases in `arch-check-breakers.sh` (a second plain file beside
`main.rs`; a `session(` file listed as a `mod`).

### 2.5 nextest in the gate

The gate's test line becomes `cargo nextest run --workspace
--all-targets`: a process per test, so the four environment-setting
files and the contract's lock cost nothing, 29 to 30 s for the suite
against 94 s, no doctests lost (`--all-targets` runs none today). A
`.config/nextest.toml` fixes the policy: `retries = 0` in every
profile (a retry turns a flaky limiter or gate test green, the
opposite of "the gate decides"); `fail-fast = false` (nextest's own
default is on); a slow timeout that kills a hung test only after the
code's longest in-test deadline (60 s): `30s × 4` locally, `60s × 4`
under a `ci` profile sized from the slowest honest full-cases test
(26.5 s on this machine; a four-core runner is slower). `cargo test`
stays supported and documented, which is why 2.4 keeps the exception.

CI installs it with `taiki-e/install-action@v2`, pinned to a version,
the action `release.yml` already uses, and both gate workflows run
`--profile ci`; AGENTS.md's gate block, its build-order paragraph
(`cargo nextest run` rewrites `acq-mcp` as `cargo test` does, measured)
and its once-per-clone line change accordingly. The two contract tests
that wait out the stop bound get a scripted peer that exits after it
answers `stopping`, as a daemon does, unless one of them asserts the
bound itself, in which case that one keeps it and says so; the suite's
wall floor then drops from the 10 s pair to the trade forms' 10.8 s.

## 3. What each crate's tests become

| Crate | Binaries | Tests (about) | Notes |
| --- | --- | --- | --- |
| protocol | 2 → 1 | 12 | `wire.rs` and `line_endings.rs` as mods; the wire fixture's regeneration command rewritten |
| client | 1 | 17 | the harness's home; `contract.rs` keeps its raw frames; two tests stop waiting out the bound |
| verbs | 2 | 20 → 24 | `daemon.rs` gains the ledger-rows jobs and daemon report; `documents.rs` the Running and Incompatible arms; the copied harness half deleted |
| store, plan, search (units) | — | +3 | `league_list`; the `"sub": null` spend; the pricing refusals |
| search | 38 → 1 | 350 → 359 | `tabs.rs` new; `answer.rs`, `counts.rs`, `refusal.rs`, `trade_translate.rs`, `next.rs` gain a test each or a row; `common` once |
| CLI | 20 → 2 | 124 → 122 | 1 deleted, 2 moved down, 8 thinned, 8 thinned in `search_json.rs`; about 22 goldens; ten private harness copies gone |
| MCP | 9 → 2 | 56 → 44 | 13 deleted, 10 moved down, 10 thinned, 2 added; `Seat::search`'s equality one test |
| all | 87 → 24 | 986 → about 975 | `cargo test` about 59 s; nextest about 30 s |

## 4. The rulings this needs

Two, both the owner's, measured against the entry rule (one bullet,
at most 800 bytes, one *Why:*, one *Pinned:*).

**R1, new, cross-cutting (`AGENTS.md`), 783 bytes:**

- **C<next> — A behaviour is pinned once, at the lowest crate that holds it, and the process tests share one harness.** A frontend's tests pin what its adapter adds: parsing, text rendering, refusal shape, streams and exit code, size bounds, the door. Wording is a golden, regenerated under `ACQ_UPDATE_FIXTURES=1` and naming the facts a reader must find, so a blind regeneration fails. Every test process takes the harness's one isolation list. A crate's integration tests are one binary, except a file that sets the process environment; the gate runs them under nextest, a process per test, never retried. *Why:* one wording was three edits in three crates, and 72 binaries linked the same crates. *Pinned:* `tools/arch-check.sh` (the layout), the nextest profile, the one harness.

Its *Pinned* names the two files in words until they exist; the
build's last commit may replace the words with paths.

**R2, the gate (`AGENTS.md`, the owner's process):** the test line of
the quality gate becomes `cargo nextest run --workspace --all-targets`;
the build-order paragraph says that `cargo nextest run` rewrites
`acq-mcp` as `cargo test --all-targets` does; the once-per-clone line
adds `cargo install cargo-nextest --locked`. `cargo test` remains a
supported way to run the suite, which is why the environment-setting
files stay their own binaries.

**The budget.** `AGENTS.md` measures 14,263 bytes under a budget of
14,363. C<next> adds 783 and R2 about 130; the budget rises to what the
file then measures plus a hundred, under the owner's temporary
permission for this phase (given 2026-10-21 during 2a), which he
reassesses himself.

**No ruling needed**, stated so the owner can object: the harness as a
shared file (nothing under `tests/` is a boundary); the goldens' home
and the `REVIEW-PROMPT.md` sentence (a roadmap mechanism file, the
Fable session's); the review checklist's new row for the fault shape
"a behaviour pinned at three layers" (a finding record, written after
the work); the deletions in appendix A (tests, under the delegation,
listed for review; no deletion removes a checklist holder, each row
checked).

## 5. Alternatives weighed and declined

- **A test-kit crate.** A C1 amendment, a table row, a contract
  revision move and a new check rule, for a harness that still could
  not name a frontend's binary. The shared file does the same with no
  boundary touched. Revisit at the GUI's backend.
- **An equality table, MCP search against `acq --json search`.** It
  exists already inside every MCP search test and, on the request
  path, cannot fail: both sides call the same four functions. Kept as
  one small test; the key-order test is the one that can see a fault.
- **Running both runners in the gate.** 94 s more per gate for no
  coverage: `--all-targets` runs no doctests, and nextest builds the
  same artifacts.
- **Merging the environment-setting files too, trusting nextest.** It
  passes under nextest and fails under `cargo test` (measured: 6 and 9
  tests). A binary that passes under one runner only is a trap for the
  next agent.
- **JSON goldens.** JSON is the contract, asserted by field and
  fixtured where versioned; a JSON golden would carry the basis, token
  and epochs and duplicate the field asserts.
- **A diff crate.** `similar` would do, but a forty-line LCS in the
  harness costs no manifest or lock edit; the goldens are under about
  200 lines.
- **A commit-message rule for regenerations, or a source-change
  check.** The first is discipline the reviewer already applies; the
  second never fires on this history.
- **Normalising job ids now.** The daemon-backed texts wait for 2c's
  journey golden, which proves the rule once with a real transcript.
- **The journey test and real-shaped mock items.** Step 2c, by the
  roadmap.
- **Retries under nextest.** Never: a test that fails once is a
  finding.

## 6. Risks and what holds them

- **The shared `#[path]` string.** Relative, and stale if a crate
  moves; a compile error, never a silent one. The builder confirms it
  compiles from a `mod.rs` first.
- **`ORIGINAL_TMP` and the socket cap.** A session points `TMPDIR` into
  its scratch; the shared `scratch()` always derives from the original
  temp dir, and the daemon's stderr is piped and printed on a start
  timeout so a refused socket path is seen, not "did not come up".
- **Windows and `place_link`.** The hard link into `deps/` is re-placed
  after a rebuild; under nextest's processes the first wave after a
  rebuild could race on it. Watched on the first Windows CI run; if it
  shows, the link is placed once by the gate's build step.
- **Idle shutdown at 30 s everywhere.** No test idles that long between
  commands by the reading; the first run confirms it.
- **The offline tests' base as their world.** `tmp/` and `logs/` appear
  inside it; a test that lists the root would see them. The builder
  checks the seed helpers and any root-listing assertion.
- **Goldens regenerated blind.** The required facts fail the test; the
  review prompt reads the diff.
- **A thinned test that loses the adapter's part.** Each thin names
  what it keeps (appendix A); the review reads them against the rule.
- **The slow timeout killing an honest test on a slow runner.** Sized
  at four periods past the slowest measured; a kill prints as such,
  never as a pass.

## 7. The owner's word

Pending: the rulings in section 4.

## Appendix A — the tests, by action (as of `3bc71a86`)

**Delete (14).** MCP `search_tools.rs`: `:306 aq1`, `:348 aq2`,
`:369 aq3`, `:440 aq5`, `:486 a_next_is_sent_back_whole…`, `:528
a_realm_needed_is_offered…`, `:563 no_arguments_over_two_realms…`,
`:621 what_has_left_is_asked…`, `:822 a_count_read_back…`, `:861
a_view_beside_a_request…`, `:1209 a_count_holds_its_request…`, `:1352
a_zero_answers_suggestion…`, `:1729 an_ambiguous_clause…` (with the
dead helpers `seed_poe2` at `:145` and `seed_rings` at `:1671`); CLI
`seat_faults.rs:1555 a_rows_place_says_which_cell…` (subsumed by
`:1570`).

**Move down (12), then delete the frontend test unless noted.**
Search crate: `Tabs::of` from MCP `search_tools.rs:1281`, `:1497` and
CLI `seat_faults.rs:627`, `:762` (new `tests/tabs.rs`; the frontend
tests are thinned, not deleted); `list_seen` under a fixed league from
MCP `:1467` and CLI `seat_faults.rs:876` (`answer.rs`); the exact
trade clause from MCP `:1780` (`trade_translate.rs`); a comma inside a
key from MCP `:592` (`counts.rs`); `view.cross.limit` from MCP `:1248`
(`counts.rs`); the unknown count key's refusal from MCP `:1845`
(`counts.rs` or `refusal.rs`); the bare word read as `name:` first
from MCP `:1387` (a row in `refusal.rs:152`'s table); a touched term's
keys from MCP `:1570` (`answer.rs`); the request-level trade rules
from MCP `:985` (`next.rs`/`answer.rs`; CLI `search_json.rs:2664`,
`:2919` keep their rule-5 halves). Store: `league_list` from MCP
`:1410` (`src/lib.rs` units; CLI `seat_faults.rs:1630` keeps the
text). Verbs: `Frontend::jobs()` and `daemon()` over a ledger with
rows from MCP `persisted_queue.rs:42` and CLI `persisted_queue.rs:38`
(`tests/daemon.rs`, beside `:370`; the CLI test keeps its text);
`DaemonReport`'s Running and Incompatible arms from MCP
`daemon_status.rs:32` and CLI `daemon_observe.rs:42`
(`tests/documents.rs`, beside `:558`; both frontend tests thinned).
Plan: the `"sub": null` spend from CLI `seat_faults.rs:1020`
(`src/lib.rs`, beside `:3593`; the CLI test stays as the cross-door
e2e); the pricing verbs' limit, conflict and unreadable-row refusals
(`src/pricing.rs`, beside its three units; CLI `price_json.rs` stays
whole).

**Thin (27), keeping the adapter's part named.** MCP `search_tools.rs`:
`:409 aq4` (one `show` with `body: true` equal to `acq show --body`),
`:687` (one search refusal and one `show` refusal as `isError` with
`kind` and `readings`), `:775` (the selector mapping: `scope.account`,
`account`, the continuation), `:1281` and `:1497` (size budget,
`expand`, the `type`/`never_fetched` mapping, equality to the CLI),
`:1608` and `:1636` (the description assertion). MCP others:
`daemon_status.rs:32` (never replaced; `wanted`), `persisted_queue.rs:91`
(`wrong_state` as an `isError` result with `next`; one `result`
equality), `pricing_tools.rs:364` (the `expand` mapping). CLI
`seat_faults.rs`: `:233` (one case: exit 1, the fault's fields in
`--json`), `:357`, `:454`, `:627`, `:762`, `:972`, `:1110`, `:1748`
(the text lines, exit codes, `--routes` beside `--next`). CLI
`search_json.rs`: `:191`, `:1203`, `:1486`, `:1632`, `:1819`, `:1980`,
`:2255`, `:2391` (the text and rule 5; the matched-id assertions go,
held by `ranged.rs`, `pseudo.rs`, `counts.rs`, `surface_faults.rs`,
`price.rs`, `rows.rs`, `category.rs`, `percentile.rs`,
`membership.rs`). CLI `persisted_queue.rs:38` (the text).

**Keep (99)** everything else in scope, among them every checklist
holder (`REVIEW-CHECKLIST.md` rows naming frontend tests: all kept).
`Seat::search`'s equality becomes one test over four requests. **New
(2):** MCP `show`'s malformed `against` refused as `tree`; MCP
`item_events`' `hours` and its defaults.

## Appendix B — the golden sites (CLI crate)

| Group | Sites | Goldens |
| --- | --- | --- |
| `search_json.rs`: the answer with `--routes` (`:664`), explain, aside and against (`:1024–1135`), priced rows (`:1233–1254`), row detail (`:1363–1382`), sort lines (`:740–876`, `:1455–1476`, `:1890`), categories (`:1653–1716`), membership (`:2320–2458`), the trade table (`:2703–2718`) | ~45 direct, ~30 in loops | ~8 |
| `seat_faults.rs`: tabs (`:633–849`), the scope line (`:1276`, `:1324`), the pointer (`:1404–1409`, its run kept), place and age (`:1560–1601`), store status (`:1636`), price status (`:151`, `:169`), routes (`:1887–1908`), trade over (`:2110–2119`), the price line (`:944`) | ~28 | ~9 |
| `shop_json.rs` render (`:248–329`), `price_json.rs` list and show (`:244–565`), `reference_currency.rs` (`:59–100`) | ~15 | ~4 |
| refusal walks in `price_json.rs` (`:289–306`, `:443–476`, `:727–858`) | ~12 | ~2 |
| not converted: MCP descriptions (the reference holds them; shrink to fact words), daemon-backed texts (2c), measured sizes, printed commands, computed literals, negative asserts, single refusals, `world.rs` | | 0 |

## Appendix C — the harness API (signatures)

```rust
// crates/acquisition-client/tests/harness/mod.rs
pub struct Scratch(pub PathBuf);                 // removed on drop; declare first
pub fn scratch(tag: &str) -> Scratch;            // <original temp>/acq-<tag>-<pid>-<n>
pub fn original_temp_dir() -> &'static Path;
pub fn scratch_tmp(base: &Path) -> PathBuf;      // base/tmp, created
pub fn isolation(world: &Path, base: &Path) -> Vec<(OsString, Option<OsString>)>;
pub fn isolate(cmd: &mut Command, base: &Path);           // world = base/store
pub fn isolate_at(cmd: &mut Command, world: &Path, base: &Path);
pub fn offline(cmd: &mut Command, base: &Path);           // + ACQ_NO_SPAWN=1
pub fn acqd_beside(frontend: &Path) -> PathBuf;  // names `cargo build --workspace` on absence
pub fn acqd_built() -> PathBuf;
pub fn link_beside_this_test(acqd: &Path);       // the sibling asserted the same file
pub fn sibling(exe: &Path, name: &str) -> PathBuf;
pub struct Proc(pub Child, pub PathBuf);         // killed and reaped on drop; named on panic
impl Proc { pub fn id(&self) -> u32; pub fn wait_exit(&mut self, within: Duration); }
pub fn daemon_command(acqd: &Path, base: &Path) -> Command;   // stderr piped, shown on timeout
pub fn start_daemon(acq: &Path, acqd: &Path, base: &Path) -> (Proc, u64);
pub fn sockets_under(dir: &Path) -> Vec<PathBuf>;
pub fn no_daemon_appeared(base: &Path) -> bool;
pub fn sole_json(out: &Output) -> Value;
pub fn text(out: &Output) -> String;
pub fn http_get(url: &str) -> (u16, HashMap<String, String>, String);
pub fn approve(authorize_url: &str, user: Option<&str>);  // encodes `#`
pub fn login(acq: &Path, base: &Path, user: Option<&str>, env: &[(&str, &str)]);
pub fn same_text(expected: &str, actual: &str, path: &Path, regenerate: &str); // the diff
pub struct Session { pub base: PathBuf /* lock held */ }
pub fn session(tag: &str) -> Session;            // the one lock; isolation applied in-process
impl Session { pub fn point_world_at(&self, root: &Path); }
pub async fn start_daemon_in_process(s: &Session, acqd: &Path) -> Proc;
pub async fn compatible_client() -> Client;
pub async fn login_in_process(client: &mut Client, user: Option<&str>);
pub async fn wait_all_terminal(client: &mut Client, expected: usize);

// crates/acquisition-cli/tests/harness/mod.rs (the adapter), today's names kept:
// acq_exe, command, command_of, command_in, acq, acqd, start_daemon, login, login_with,
// plus Transcript::{new, run} and golden(file, name, &Transcript, facts).
// crates/acquisition-mcp/tests/harness/mod.rs: spawn, spawn_daemon, acq (no spawn),
// run_to_exit, Mcp.
```

## Appendix D — measurements (this machine, 2026-10-21)

| What | Today | Measured or estimated after |
| --- | --- | --- |
| test binaries | 87 (72 integration) | 24 (9) |
| `cargo test --workspace --all-targets` | 94.0–94.5 s | ~59 s (merged; estimated from per-crate runs) |
| `cargo nextest run --workspace --all-targets` | 29–30 s (today's files) | ~30 s; 54 s with `ACQ_CASES=full` |
| search tests, relink after touching `common` | 13.3–13.8 s (32 binaries) | 6.7 s (1) |
| touch of the search library, `--no-run` | 21.8 s (75 relinks) | ~10–12 s (estimated) |
| search test executables + debug bundles | 1.05 + 2.48 GB | 0.2 + 0.2 GB |
| client contract binary | 27.3 s serial | ~7 s once the two stop-bound waits go |
| slowest test, local cases | trade_forms 10.8 s | unchanged |
