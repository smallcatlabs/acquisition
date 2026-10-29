# 94 — the rails status carries the journal's path as its own field

Issue #94. The first of the league group in "Pull-forward" after #53's
close (`roadmap/README.md`): the diagnosis a user pastes during the
league must not carry a directory it was never told to hide. The rhythm
is "A step".

## Goal

When the daemon cannot open its send journal, its rails status reports
`journal` as `NOT WRITTEN — journal <path> could not be opened: <error>`
(`crates/acquisition-daemon/src/rails.rs`, `Rails::with_config_and_clock`
and `Rails::status`), so the configured path lives only inside error
prose. `acq diagnose` redacts the directories it knows, and from the
shell whose `ACQ_JOURNAL` named that journal it knows this one too; from
any other shell it does not, and the directory is pasted whole (r4's
fix-round review, finding 1). When this step is done the rails status
carries the configured path as a field of its own, `journal_path`,
present whenever a journal is configured, written or not; `journal`'s
failure words name no path; the diagnosis builds its journal needle
from the field, so a daemon under one shell's unwritable `ACQ_JOURNAL`,
diagnosed from a shell whose knob differs, leaves neither spelling of
that directory anywhere in the output, JSON or text. The wider question
the issue's last paragraph raises (a daemon log written through one
function that never formats a token) is its own issue, filed at the
close, and not this step.

## Read

- `AGENTS.md`; `roadmap/README.md` ("A step", "Pull-forward").
- Issue #94 (`gh issue view 94 --json body,comments`; no comments).
- `roadmap/briefs/r4-diagnose.md` (the verb, its sections, the
  redaction by one function) and `roadmap/reviews/r4-fixes-b9303f3a.md`,
  finding 1: the failing case (an unwritable `ACQ_JOURNAL` outside the
  home, store and log directories, diagnosed with the knob unset) and
  the fix it names, the configured path reported apart from the failure
  text.
- `decisions/daemon.md`: C12 (two surfaces, in `AGENTS.md`): a field on
  the daemon's status document is a protocol change, ruled first; C85
  (one request in flight, its `c85_` tests in
  `acquisition-protocol/tests/wire.rs`, which pin every response's wire
  shape against the fixtures under `tests/fixtures/wire/`); C10 (the
  single-version handshake) and C84 (the runtime identity: the contract
  revision is a digest of the protocol sources); C83 (the log and
  journal are bounded diagnostics outside the world).
- `decisions/network.md`: C129 (a Cloudflare answer is a stop: the
  block mark, the `halted` cause the same status carries) and C25 (a
  rails halt leaves jobs waiting); the rails' rulings live there because
  `rails.rs` routes to `network` (`python3 -I tools/bothy.py pack --path
  crates/acquisition-daemon/src/rails.rs`). No ruling governs the
  journal's report; `TESTING-NOTES.md`, "The journal is the contract
  surface", governs its lines, which this step does not touch.
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world" and "Frontends and the surface an agent meets": "A document's
  producer moved without its readers listed" (every reader of
  `rails.journal` is listed below); "A fix staging the nearest failure,
  not the named one" (the pin is the review's case, the knob differing,
  not the shared knob the r4 fix pinned); "A fixture that cannot see the
  default" (the planted directory must lie outside every needle the
  diagnosis already holds, or the pin passes without the fix).
- The code:
  - `crates/acquisition-protocol/src/status.rs:15`, `RailsStatus`:
    `#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq,
    Eq)]` with no `#[serde(deny_unknown_fields)]` (the two structs below
    it that carry one are `WindowStatus` and `RuleStatus`, which a
    serialized plan embeds); `journal: Option<String>`.
  - `crates/acquisition-daemon/src/rails.rs`: `RailsConfig::from_env`
    (`ACQ_JOURNAL`: empty or `0` is `None`, a path is that path, unset is
    the world's default); `with_config_and_clock`, where `journal_error`
    is formatted with the path; `startup_warnings`, which carries
    `journal_error` into the daemon log as `RAILS CONFIG: …`
    (`daemon/start.rs:591`); `Rails::status`, the `journal` match; the
    unit test `unopenable_journal_is_reported_not_silent`.
  - `crates/acquisition-daemon/src/daemon/start.rs:581`, the startup
    `rails:` log line printing `rails.journal`.
  - `crates/acquisition-verbs/src/diagnose.rs`: `JOURNAL_NOT_WRITTEN`
    and the running arm that reports the journal unread "in its words"
    (lines 209 to 228); the `printed` list and the path needles (248 to
    363), including the block that adds this shell's own
    `JournalKnob::At` directory (351 to 360), whose comment names this
    gap as raised for the owner. `bothy pack --path` routes no area for
    this file (it names none near its top; the verbs crate's rulings are
    `decisions/frontends.md`).
  - `crates/acquisition-cli/src/main.rs:2332`, `rails_text`, the one
    renderer of the rails line (`daemon status`, `acq diagnose`'s daemon
    section through `daemon_text`, and the web UI's jobs page at
    `crates/acquisition-cli/src/web/mod.rs:818`);
    `crates/acquisition-cli/src/diagnose_cmd.rs`, the diagnosis text.
- The readers of the wire's rails, found by grep (`grep -rn
  "RailsStatus {" crates --include='*.rs'`; `grep -rln tripwire_enabled
  crates`): the two struct literals outside the daemon,
  `acquisition-protocol/tests/wire.rs:180` (`rails()`) and
  `acquisition-verbs/tests/documents.rs:812`; the two wire fixtures
  that carry a `RailsStatus`,
  `crates/acquisition-protocol/tests/fixtures/wire/response/daemon_status.json`
  and `dashboard.json`; `acquisition-cli/tests/world.rs:787`, which
  asserts `rails.journal` equals the opened default path; the MCP's
  `daemon_status` text (`acquisition-mcp/src/main.rs:699`, "the rails
  state with the journal path"), which stays true.
- The tests that assert on the journal's words (`grep -rn "NOT WRITTEN"
  crates --include='*.rs'` prints 7 lines; one, `daemon.rs:1160`, is
  the rails state's own `RAILS STATE NOT WRITTEN` and unrelated):
  `rails.rs:1689` and
  `acquisition-verbs/tests/daemon.rs`,
  `a_journal_the_daemon_could_not_open_is_unread_in_its_words_and_its_directory_replaced`
  (lines 1140 to 1199), which starts the daemon and the diagnosing
  frontend under the same `ACQ_JOURNAL`. The goldens that print the
  rails line (`grep -rln journal crates/*/tests/goldens`):
  `web/jobs_halted.txt` and `web/refresh_loop.txt` (`journal off`, which
  this step leaves as it is) and `diagnose/no_daemon.txt` (no daemon, no
  rails line).

## Input

Measured on this tree (`48989db0`):

- The claim the ruling rests on, that an older client reading a newer
  daemon is unaffected, holds twice. `RailsStatus` carries no
  `deny_unknown_fields`, so serde skips a field it does not know; and a
  field added to `status.rs` moves the contract revision (C84, a digest
  of the protocol sources), so an older client never reads a newer
  daemon's status at all: an observer reports the daemon `Incompatible`
  by its identity alone (`acquisition-verbs/src/daemon.rs:56`, "no
  vitals"), and a use verb replaces it (C10). A newer client reading an
  older document gets `None` for the missing `Option` field. The wire
  pin (Do 3) parses the old fixture shape to say so.
- The failing input, the review's case: the daemon started with
  `ACQ_JOURNAL=<base>/private-runs/sends.jsonl`, where
  `<base>/private-runs` is a regular file, so no platform can create the
  directory (the existing verbs test's staging); the diagnosing frontend
  with `ACQ_JOURNAL` unset. `<base>` is the harness session's own
  directory, outside the home, the store root and the world's log
  directory; the pin asserts that precondition, so it cannot pass on a
  needle the diagnosis already holds.
- "Neither spelling" is the directory as configured and its
  canonical form (`/var/…` and `/private/var/…` on macOS), as the
  existing pins check.

## Do

1. **The ruling, recorded** (`decisions/daemon.md`, under "Daemon,
   jobs, protocol"), in its own documents-only commit before any code
   that depends on it. The owner, 2026-10-28, relayed by the
   orchestrating session: "I agree to both." Entered as a new entry, the
   registry's next id (one past C151, the unreadable annotation row),
   rather than an amendment of C85, whose subject is the
   connection: the rails status names the configured journal by its
   path in a field of its own, present whenever a journal is
   configured; `journal` is the path when written, the words of why not
   when not, absent when off; the words carry no path; a frontend that
   prints or redacts the journal's location reads the field. The commit
   quotes him; the entry carries no history (`tools/bothy.py`, the entry
   rule).
2. **The field and the words, at the daemon.**
   `RailsStatus.journal_path: Option<String>`, documented on the struct.
   `Rails` keeps the failure's reason without the path
   (`could not be opened: <error>`); `status` fills `journal_path` from
   `RailsConfig::journal_path` whatever happened, and `journal` as
   `NOT WRITTEN — could not be opened: <error>` on failure. The startup
   warning keeps naming the path (`journal <path> could not be opened:
   …`): the log is the local reader's, and the diagnosis redacts its
   tail from the same field. The pin, in `rails.rs`'s tests, replaces
   `unopenable_journal_is_reported_not_silent`: off, both `None`;
   opened, the field and `journal` the path; unopenable, the field the
   path, `journal` starting `NOT WRITTEN` and not containing it, one
   startup warning naming it.
3. **The wire.** `wire.rs`'s `rails()` gains the field; the two
   fixtures regenerate under `ACQ_UPDATE_FIXTURES=1` on a test run, and
   the commit message says what changed in each (one key, its value).
   A `c152_` test there parses a daemon status whose rails lack the
   field (`journal_path` `None`) and one whose rails carry a key this
   build does not know (read, the key ignored): the claim in "Input"
   held by a test, not prose. `documents.rs:812` gains the field.
   Rides in Do 2's commit: the protocol crate's change does not compile
   without its readers.
4. **The redaction, at the verbs crate.** The running arm takes the
   journal's location from `rails.journal_path`: written, it is the
   tail's path; not written, the section is unread in words that name
   the path (`the daemon running reports <path>: NOT WRITTEN — …`),
   which the redaction turns into `<journal>/sends.jsonl`; the path's
   directory joins the `printed` needles whatever was written. The
   `JournalKnob::At` block stays beside the field: every journal this
   diagnosis was told about is a needle, because the log is appended
   across lifetimes and may name a journal an earlier daemon of this
   shell was configured with while the one running reports another or
   none; both the daemon and the frontend read a relative `ACQ_JOURNAL`
   as absolute against their working directory, so the path is one the
   redactor takes. Settled in the round (#94's review, findings 1 and
   2; the build had removed the block). Its comment's raised gap is
   closed. The pin is the existing verbs
   test turned to the review's case: the daemon under the unwritable
   journal, the frontend's `ACQ_JOURNAL` unset, the precondition that
   the directory lies under no home, store or log needle asserted
   first; then the document's `daemon.rails.journal_path` reads
   `<journal>/sends.jsonl`, the journal section is unread in the
   daemon's words, and neither spelling appears anywhere in the
   document (the log tail, which carries the startup warning, among
   it). Renamed `c152_…`. The test is run once against the unfixed
   verbs code and its failure reported (the report, not the history).
5. **The rendering, at the CLI.** `rails_text` prints a journal that was
   not written with its path first (`journal <path> NOT WRITTEN — could
   not be opened: …`) and an opened one as before; `journal off` is
   unchanged, so the two web goldens stand. The pin, a `c152_` test in
   `acquisition-cli/tests/diagnose.rs`: a daemon spawned through the
   harness under the unwritable journal, `acq diagnose` run with
   `ACQ_JOURNAL` unset, its text carrying `<journal>/sends.jsonl` on the
   rails line and neither spelling anywhere, exit 0. The words of the
   rails line and the unread journal are the step's agent-facing design,
   recorded in the commits that build them.
6. The full gate (`tools/gate.sh`) after `cargo build --workspace`, in
   the background with its output under the session's scratch; one
   commit per concern (2 with 3, the field on the wire with its
   readers; 4; 5), each message naming this brief. Then "Current step"
   as built, review pending, and the Codex review over the range.

## Done when

- `decisions/daemon.md` holds the new entry in the entry rule's form, committed
  before the code commits, its message quoting the owner.
- `RailsStatus` has `journal_path`; both wire fixtures carry it; the
  `c152_` wire test is green.
- `grep -n 'journal {} could not be opened' crates/acquisition-daemon/src/rails.rs`
  finds only the startup warning's formatting, and `Rails::status`
  builds `journal`'s failure words without the path (the daemon's
  `c152_` test).
- The verbs `c152_` tests are green: with the diagnosing knob unset
  and the daemon's relative, and with an earlier daemon's journal in
  the log tail under this shell's knob. Settled in the round: the build
  line said the knob's needle block was gone.
- The CLI `c152_` test is green: `acq diagnose`'s text, from a shell
  whose knob differs from the daemon's, prints neither spelling of the
  directory.
- The regenerated fixtures' diffs are one added key each, named in the
  commit message.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The daemon log through one function that never formats a token: the
  issue's last paragraph, the owner's ruling (2); its own issue
  (`area:daemon`, `trigger:user`), filed at the close.
- The journal's lines, its bound, or its rotation (`TESTING-NOTES.md`).
- r4's finding 2, the numeric exemption: closed in r4's round or not,
  not this step's.
- Changing `journal`'s meaning when written (still the path): a frontend
  that prints it today keeps working; the duplication is named in the
  report.
- `bothy pack`'s routing of `diagnose.rs`: raised, not fixed here.

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `94`,
every `ACQ_STORE_DIR` under it; kill only the processes you started.
Any `acq` is in a mock world. Nothing sends to GGG. Never push.

## Report

`TEMPLATE.md`.
