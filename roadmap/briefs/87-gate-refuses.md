# 87 — the gate refuses a set fixture switch and a tree its stages changed

## Goal

Today every golden reads its switch as `var_os("ACQ_UPDATE_FIXTURES")
.is_some()`, so the variable at `0` or empty regenerates too, and the
harness removes `ACQ_*` from the children it spawns, never from the test
process that compares: a gate run in a shell with the variable exported
rewrites every golden, both references and the wire fixtures, and goes
green. When this step is done the gate refuses that shell at a stage of
its own naming the variable, and runs its other stages without it; a
gate whose stages changed the working tree fails at a stage of its own
naming the files; and every test that spawns a workspace binary does it
through the harness's one isolation list (C122, one harness and one
isolation list for every test process), held by an `arch-check.sh` rule
that refuses the next bypass. The issue is #87; the commits say
"closes #87" only on the last.

## Read

- Issue #87 and its two comments: the two refusals; p0's ask for a
  breaker suite for the gate in `tools/docs-check-breakers.sh`'s shape,
  the unknown stage among its cases; 2g's note that the two
  printed-command runners merge into the harness.
- `AGENTS.md`: C122, the quality gate, "The gate decides" (a commit is
  gated on `tools/gate.sh`'s exit status).
- `tools/gate.sh`: the header (lines 1 to 29; `usage` prints lines 4 to
  9), `names` and `DOCS` (lines 34 and 35), `run_stage`, the selection.
- `tools/arch-check.sh`: §8, the test layout (the last check, its
  python reader), and the header's list of checks.
- `tools/arch-check-breakers.sh` and `tools/docs-check-breakers.sh`:
  the breaker shape (copies, byte-for-byte restore, a trap, owned
  paths refused at start, a refusal counts only when it names the
  property, `tool` edits the checker by exact-once substitution).
- `crates/acquisition-client/tests/harness/mod.rs`: the module doc,
  `isolation` (line 160), `isolate`, `isolate_at`, `offline`,
  `daemon_command` (line 365).
- `crates/acquisition-cli/tests/harness/mod.rs`: `acq_exe`, `command`,
  `offline_command`, `acqd`, `shell_words`; the four
  `ACQ_UPDATE_FIXTURES` reads (lines 739 and 1036; and
  `crates/acquisition-cli/tests/cli_reference.rs` line 124,
  `crates/acquisition-mcp/tests/mcp_reference.rs` line 85,
  `crates/acquisition-protocol/tests/wire.rs` line 36,
  `crates/acquisition-plan/src/listing.rs` line 2375,
  `crates/acquisition-plan/src/shop.rs` line 2130).
- `crates/acquisition-mcp/tests/harness/mod.rs`: `exe`, `spawn`.
- The spawns outside a harness, measured on this tree by grepping
  `crates/*/tests` for `CARGO_BIN_EXE_` and `Command::new(`:
  - `crates/acquisition-cli/tests/search_json.rs` `run_printed`
    (line 196) and the route loop (line 670): `sh -c` with a
    hand-written environment, no isolation;
  - `crates/acquisition-cli/tests/seat_faults.rs` `run_as_printed`
    (line 2076): `sh -c`, isolated by hand;
  - `crates/acquisition-mcp/tests/provider_default.rs` (line 15):
    `acq-mcp` with no isolation;
  - `crates/acquisition-cli/tests/cli_reference.rs` `help` (line 18)
    and `crates/acquisition-cli/tests/readme_tour.rs` `help` (line 32):
    `acq --help` with `ACQ_ACCOUNT` removed alone;
  - `crates/acquisition-cli/tests/world.rs` `daemon_command` (line 50)
    and `crates/acquisition-cli/tests/provider_default.rs` (line 62):
    `acqd` isolated by hand;
  - `crates/acquisition-cli/tests/watch_recovery.rs` (line 107),
    `crates/acquisition-cli/tests/daemon_observe.rs` (lines 156 and
    308), `crates/acquisition-mcp/tests/daemon_status.rs` (line 27):
    the binary's path named, no spawn;
  - `crates/acquisition-daemon/tests/main.rs` (line 39): `acqd` from an
    emptied environment (`env_clear`); the daemon crate cannot link the
    harness, which links the client (C1, the edges; `arch-check.sh`
    refuses a daemon dev-dependency on the client).
  The issue names the first and the third; the rest are the same rule's.
- `.github/workflows/gate-linux.yml`: the test step and the `breakers`
  job.

## Input

No measurement. The failing inputs the breakers stage: the variable at
`0`, at empty and at `1`; a stage that appends to a tracked file,
creates an untracked one, deletes a tracked one, and rewrites a file
already dirty before the gate; a test file under `tests/` naming
`CARGO_BIN_EXE_`, one calling `Command::new` on a path, and one calling
`Command::new("sh")`.

## Do

1. **The fixture switch.** `tools/gate.sh` records whether
   `ACQ_UPDATE_FIXTURES` is set and its value, and unsets it before any
   stage, so no stage of any selection rewrites a golden. A first stage,
   `fixtures`, fails when it was set, naming the variable and its value
   and saying to unset it and regenerate by the command a golden's
   failure names. The goldens' `is_some()` reads stay: the regeneration
   command is theirs, and the gate is the boundary.
2. **The tree.** The issue says "fails when `git status --porcelain` is
   not empty after the tests"; read literally, it fails every gate a
   session runs before committing, since the gate is run on uncommitted
   work. The property is that the gate's stages leave the working tree
   as they found it: a last stage, `tree`, compares a snapshot taken
   before the first stage with one taken at the end, tracked and
   untracked files that `.gitignore` does not hide, content and not
   status alone (a golden already edited and rewritten again moves no
   status line), and fails naming each changed path. The snapshot is a
   tree object written through a copy of the index (`GIT_INDEX_FILE`,
   `git add -A`, `git write-tree`), so the real index is untouched and
   `git diff --name-status` names the paths. A snapshot that cannot be
   taken fails the stage. Another hand editing the tree while the gate
   runs reads the same; the failure says so.
3. **The gate's breakers**, `tools/gate-breakers.sh`, in the shape of
   `tools/docs-check-breakers.sh`: an unknown stage refused; the
   variable at `0`, empty and `1` refused by `fixtures`, unset passed;
   a stage run with the variable set sees it unset; a stage that
   dirties a clean file, creates an untracked file, deletes a tracked
   file, or rewrites an already-dirty file refused by `tree`; a tree
   dirty before the gate and left as found passed. A case edits the
   gate's stage commands (the `tool` substitution of
   `arch-check-breakers.sh`) and runs a selection that builds no Rust.
   The CI `breakers` job runs it.
4. **CI.** `gate-linux.yml` gains a step after the tests that fails when
   `git status --porcelain` is not empty: there the checkout is clean,
   so the literal check is the property.
5. **The spawns.** The harness gains the runners the stray sites need,
   each applying the isolation list: a printed command run through `sh`
   as a person pastes it (`printed`, the world at `base/store`) and its
   offline twin over a world seeded by hand (`offline_printed`, the
   `offline_command` shape); `daemon_command_at`, the daemon under a
   named world; the MCP adapter's `command`, `acq-mcp` isolated and not
   spawned. Each site in "Read" moves to them; a path named and not
   spawned becomes `acq_exe()` or `exe()`.
6. **The rule.** `tools/arch-check.sh` gains check 9: under a crate's
   `tests/`, outside its `harness/` directory, no file names
   `CARGO_BIN_EXE_`, and every `Command::new(` takes a string literal
   that is not `sh` (a system tool: `kill`, `ps`). The daemon crate's
   `tests/` is excepted by name, with the reason (it cannot link the
   harness; it starts `acqd` from an emptied environment); its in-crate
   tests are #64's. `tools/arch-check-breakers.sh` stages a refused
   case for each clause and the tree as it stands passing. *Settled in
   the round (`4edbb709`):* a literal other than `sh` let a path or a
   binary's name through, so `Command::new(` names a tool on an
   allowlist, `kill` and `ps`, and a breaker stages each of the two.
7. The gate, then one commit per concern: the fixture stage with its
   breakers and the CI job line; the tree stage with its cases and the
   CI step; the spawns with the rule and its cases. Each message names
   this brief.

## Done when

- `tools/gate.sh` in a shell with `ACQ_UPDATE_FIXTURES=0` exported fails
  at its `fixtures` stage, which names the variable; its other stages
  run with it unset.
- A gate whose stages changed the working tree fails at its `tree`
  stage, naming each path; a tree dirty before the gate and left as
  found passes it.
- `tools/gate-breakers.sh` stages both refusals and the unknown stage,
  and runs in the CI `breakers` job.
- `gate-linux.yml` fails when the tests leave `git status --porcelain`
  not empty.
- No file under a crate's `tests/` outside `harness/` names
  `CARGO_BIN_EXE_` or spawns a binary by a path or through `sh`, the
  daemon crate's excepted with its reason; `tools/arch-check.sh` refuses
  each, and `tools/arch-check-breakers.sh` stages each refusal.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The goldens' `is_some()` reads made `== "1"`: seven reads in four
  crates, where the gate is the one boundary the issue names. Raised,
  not built.
- The daemon's in-crate tests: #64.
- `AGENTS.md`'s "Quality gate" sentence: the file stands at 15,275
  bytes of its 15,277 budget (p1's packet); `tools/gate.sh`'s header
  states the stages.
- The Windows workflow's tree check: the Linux one holds the property
  the tests share; Windows-only writes are #70's ground.

## Rules

`TEMPLATE.md`. The step runs in the main tree, one session; kill only
the processes you started. The breaker suites edit the tree and restore
it; run them with no uncommitted work of another session in the files
they copy.

## Report

`TEMPLATE.md`.
