# r4 — `acq diagnose`: one redacted document for a bug report

Readiness step 4 (`roadmap/README.md`, row r). Closes #79. About a
day. A verb, so the delegated class: its shape is recorded in the
commit under the owner's delegation and listed at the close (p0's
ruling (b), if taken; else a ruling first).

## Goal

A user with a problem runs one command and pastes its output, and the
owner reads in it everything he would otherwise ask for: the build and
its contract, the platform, the daemon's state or the ledger's picture
without one, the store's counts, the rails, the send journal's tail
and the log's tail, with usernames and paths redacted. The first
stranger's bug report is actionable, and the owner's own
`RUN-LEDGER.md` rows can come from the same command.

## Read

- #79; `roadmap/STATE-2026-10-26.md` R1.4 and section E of the Fable
  synthesis it summarises (diagnostics for a bug report).
- `decisions/frontends.md`: C11 (`--json` on every command), C53
  (three output levels: text one screen, JSON the contract), C116 (no
  id in user-facing text), C146 (a count says what it counts); C12
  (two surfaces: this verb composes documents the verbs already
  produce and opens no third door).
- `crates/acquisition-verbs/src/daemon.rs` (`DaemonReport`, the four
  observed states, `PersistedSummary`), `frontend.rs` (`Frontend`, the
  observe door: never spawn), `jobs.rs`; `acquisition-protocol`'s
  `VERSION_WITH_CONTRACT` and `status.rs`.
- `crates/acquisition-cli/src/main.rs`: the `version`, `daemon status`
  and `store status` arms (what they print and from where);
  `store_cmd.rs` for the store counts.
- The journal: `TESTING-NOTES.md`, "The journal is the contract
  surface" (one JSON line per send, never a token or body);
  `crates/acquisition-daemon/src/rails.rs`, the journal path and its
  bound; `README.md` "Knobs" (`ACQ_JOURNAL`, `ACQ_LOG_DIR`) and "Before
  the first command" (where the log lives per platform);
  `crates/acquisition-store/src/world.rs` for the log directory.
- `README.md`, "Reporting and contributing" (the section this step
  rewrites); `SECURITY.md` (what must never appear).
- Tests: `crates/acquisition-cli/tests/no_ids.rs`, `seat_faults.rs`
  (the golden pattern with named facts), `daemon_observe.rs` (the
  observe door's four states).

## Input

- A mock world with one login and one applied plan (the journey's
  fixture), so every section has content.
- The redaction's failing inputs: a username, a store path and a
  planted token-shaped string written into the log file by the test,
  and the account's username in the journal's route keys
  (`stash@Alice#1234`); each must be absent from the output, the
  journal's route keys replaced by a stable placeholder that still
  distinguishes accounts.

## Do

1. **The document**, in the verbs crate (`diagnose.rs`): a struct
   composed of existing documents, each a field: `version` (the
   package version, the contract, the platform triple), `daemon`
   (`DaemonReport`, through the observe door, never a spawn), `store`
   (the store-status counts), `rails` (from the daemon report when it
   runs, else from the world's persisted rails state), `journal` (the
   last N lines, N default 50, parsed as JSON so a torn last line is
   reported, not pasted), `log` (the last N lines, N default 100), and
   `redactions` (how many of each kind were made). One function,
   `redact`, applied to every text field and every line: usernames
   from the index, the store root and the home directory, and any
   token-shaped string (the shapes the journal already refuses to
   write, kept as a belt), each replaced by a placeholder that
   distinguishes instances.
2. **The verb**, `acq diagnose [--lines N] [--json]`: text per C53,
   one screen for the first five sections and the two tails last; JSON
   the whole document; exit 0 whatever it found, since a diagnosis
   that fails is no diagnosis, with every section that could not be
   read saying why (C146).
3. **The README**, "Reporting and contributing": "paste the output of
   `acq diagnose` into the issue"; the tour gains the verb's line;
   `CLI-REFERENCE.md` regenerated.
4. **Pins**: the redaction test over the inputs above; the no-daemon
   case (the ledger's picture, the last daemon's log); the golden with
   its named facts; `no_ids`; the tour.

## Done when

- `acq diagnose` in a mock world prints every section; `--json` is
  total; the redaction test is green and its message says which
  strings it planted.
- With no daemon, the document says so in the daemon section and
  still prints the rest.
- The README's reporting section and tour name the verb; the
  reference regenerated, every changed line explained.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- An MCP `diagnose` tool (a consumer's ask; an issue if one asks).
- Uploading, mailing or posting the document anywhere.
- Reading the keyring or printing anything from it beyond the
  `keyring` health word the daemon already reports.
- The log's rotation or the journal's bound.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `r4`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands in the background; `tools/gate.sh` if p0 has landed,
else the gate bare by each stage's own exit status, and `cargo build
--workspace` before any daemon and after any test run (C84).

## Report

`TEMPLATE.md`, as text.
