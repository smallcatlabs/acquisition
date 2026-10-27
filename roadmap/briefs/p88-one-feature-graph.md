# p88 — One feature graph for the plain build and the test build: measure, then retire the build-order rule

Pull-forward item 1 of what remains (`roadmap/README.md`,
"Pull-forward"), issue #88 with its comment of 2026-10-27, and p0's
ledger row ("the rebuild observation, on #88"). The step is a
measurement first, then two small removals of cause, then a check
that holds them; the retirement of a hard rule in `AGENTS.md` and the
amendment of C84 (the runtime identity) are the owner's, built only on
his word (Do 6). About half a day of machine time, most of it the
machine's alone.

## Goal

A test run leaves `target/debug/acqd`, `acq` and `acq-mcp` exactly as
`cargo build --workspace` would write them, so a daemon may start
after a test run without a build between, and the gate's last stage
is a check that this still holds rather than a repair that hides it.
Today the rule "build again after any test run" (`AGENTS.md`, "Build
order"; C84, the artifact mismatch) is stated in the skills and in
every brief's Rules, and the gate ends with a second
`cargo build --workspace` because of it. P6 (`AGENTS.md`,
"Principles"): a check or a rule is retired when what it guarded is
proven. This brief says what to prove, how, and what retires if it
holds; if the measurement disproves it, the brief says what the rule
still guards and nothing retires.

## Read

- #88 and its comment: at four docs-only trees the final build took
  a second and compiled nothing; the measurement starts from an edit,
  not a clean tree. `roadmap/STATE-2026-10-26.md`, section C and R2.2
  (the auditor's trace: "feature unification, nine crates twice,
  163 s"), and the appendix's gate row. Take the claims as
  hypotheses, not findings: the pre-read below contradicts part of
  them.
- `AGENTS.md`: "Build order" (the hard rule under measurement), the
  "Quality gate", P6, C122 (a behaviour pinned once at the lowest
  crate; a crate's integration tests are one binary; the process tests
  share one harness), C1 (the daemon links the protocol crate and the
  store, nothing else; nothing links the daemon).
- `decisions/daemon.md`: C84 (the two identity values: the contract
  revision, a digest of files, and the artifact, a hash of the
  executable; features, profile and target are not inputs of the
  revision), C10 (a mismatched daemon is replaced, never negotiated
  with; an observer never replaces; the MCP never replaces), C82 (a
  frontend spawns only the `acqd` beside itself).
- `crates/acquisition-protocol/build.rs` (the revision's inputs, and
  "features, rustc, profile and target are not inputs") and
  `crates/acquisition-search/build.rs` (the build name, the same
  rule). Neither can differ between the two builds; neither is a
  cause here.
- `tools/gate.sh`: the stage table and the closing comment that
  names the test and doc stages as the artifact rewriters. The doc
  claim is one of the hypotheses. `.config/nextest.toml`: the runner's
  profile, nothing about features.
- The manifests. `Cargo.toml` at the root: the `tempfile` comment
  ("already in every test build through proptest, so declaring it
  adds no feature (C84)") and the dev profile (`incremental = false`,
  the owner's, `77771530`). `crates/acquisition-store/Cargo.toml`: the
  `rusqlite` line, `hooks` compiled in always "so one build writes one
  binary" — the one precedent for the fix this brief draws.
  `crates/acquisition-search/Cargo.toml`: the dev `rusqlite` at the
  store's features for the same reason. `crates/acquisition-plan/Cargo.toml`
  and the search's: `proptest`, the dev-dependency whose `std` feature
  is the one divergence found. `crates/acquisition-daemon/Cargo.toml`:
  no `[[test]]`, no `autotests`, and no `tests/` directory in the crate.
- `crates/acquisition-daemon/src/main.rs`: the door (no arguments,
  the environment alone; no `--version`), and the exit with status 2
  naming the frontend and the environment. Nothing in the tree pins
  that refusal (grep "takes no arguments" under `crates/`: the
  source alone). `crates/acquisition-cli/tests/provider_default.rs`,
  `c88_the_real_provider_is_the_default_and_the_retired_knob_is_refused`
  drives `acqd` for the retired knob from the CLI crate; it stays.
- `crates/acquisition-client/tests/harness/mod.rs`, "the
  executables": `acqd_beside`, `acqd_built` and `link_beside_this_test`,
  each naming `cargo build --workspace` on absence, with the reason "a
  `-p` test run does not build acqd". That parenthesis stays true after
  this step; the rest of their words are checked in Do 5.
- `.claude/skills/session-close/SKILL.md`, "Why build order matters";
  `.claude/skills/mock-session/SKILL.md`, "Set up" (the build line and
  its link); `roadmap/briefs/TEMPLATE.md` (whose only C84 line is the
  example of naming an id, not the rule: #88's count of five places
  includes it wrongly; the places are `AGENTS.md`, the two skills, the
  gate's comment, and the Rules of every brief since h1).
- `REVIEW-CHECKLIST.md`, the rows citing C84 (a version where an
  identity belongs; an identity judged twice): the faults this area
  has had, neither of them this one.
- `tools/arch-check.sh`, the LAYOUT checks (a `[[test]]` entry per
  root under `tests/`, `autotests = false`, C122): the shape the
  daemon's new test target must take.

## Input

The pre-read, taken 2026-10-27 at `71b12c9a` with cargo 1.99.0 and
nextest 0.9.149, from `cargo tree`, no build. The worker repeats it as
the first part of Do 1; what differs is a finding.

- The two resolutions, `cargo tree --workspace -e no-dev,features`
  (the plain build's) and `cargo tree --workspace -e features` (the
  test build's), printed as `{p} {f}` with no prefix, sorted unique,
  differ in 61 lines. Every package in the difference but one exists
  only in the test graph (proptest and what it alone brings: bit-set,
  bit-vec, fnv, quick-error, rand_xorshift, rusty-fork, unarray,
  wait-timeout, tempfile, fastrand, rustix 1.x). The one package present
  in both with different features is `num-traits` 0.2.19: no features
  in the plain graph, `std` in the test graph, enabled by proptest's
  `std` feature. In the plain graph `num-traits` is reached only
  through chrono, from schemars and rmcp, by `acquisition-mcp`. Neither
  `acqd` nor `acq` links it. So the "second feature set" is one
  feature on one crate, and it reaches `acq-mcp` alone: that is why
  the test build "rewrites `acq-mcp` in another form" and touches
  nothing else.
- `crates/acquisition-daemon` has no integration-test target. Cargo
  builds and uplifts a package's binaries for a test run only when the
  package has one (`CARGO_BIN_EXE_*`); without one, a `--workspace`
  test run compiles `acqd` as its own unit-test harness and never
  writes `target/debug/acqd`. This is independent of features, and it
  is the half of the rule the feature fix cannot touch: after an edit
  to a contract input, a test run rebuilds `acq` and `acq-mcp` on the
  new revision and leaves the sibling `acqd` on the old one.
- The `rand` 0.9, `rand_core`, `bitflags`, `rustix` 0.38 and `getrandom`
  sets are equal in both workspace resolutions. A `-p acquisition-daemon`
  resolution differs in more (bitflags gains `std`, tempfile and
  rustix appear): resolver 2 resolves per the selected set, so a `-p`
  build is a third graph. The harness already refuses it; it is out of
  this step's scope and named below.
- The target directory is 13 GB of accumulated forms (cargo collects
  nothing). The measurement is taken warm, at HEAD, never from a clean
  target: the cold build is a different number (`77771530`).

## Do

1. **Measure, before any edit.** Alone on the machine, every command
   as a tracked background task, every output under your scratch.
   Record, per stage, the SHA-256 and the inode (`shasum -a 256`,
   `ls -li`) of `target/debug/acq`, `acqd` and `acq-mcp`, the
   `deps/` files named `acq-`, `acqd-` and `acq_mcp-` with a 16-hex
   suffix written since a marker file (`find -newer`), and the
   fingerprint directories under `target/debug/.fingerprint` written
   since it; time each command; count the `Compiling` lines of each
   build.
   - M0, settle: `cargo build --workspace` at HEAD, then the record.
   - M1, the docs tree (no edit): `cargo nextest run --workspace
     --all-targets`, record; `RUSTDOCFLAGS="-D warnings" cargo doc
     --workspace --no-deps`, record; `cargo build --workspace`, record.
   - M2, a contract edit: a comment line in a store source
     (`crates/acquisition-store/src/lib.rs`), then the nextest line
     *first*, with no build before it, record (which tests fail, and
     with what words: this is what the rule guards today); then
     `cargo build --workspace`, record; then the nextest line again
     and the build again, record. Revert the edit.
   - M3, a daemon-only edit: a comment in
     `crates/acquisition-daemon/src/main.rs`, the nextest line, record
     (`acqd` untouched is the second cause, seen alone). Revert.
   - M4, the gate whole, twice: at the docs tree and at the store-edit
     tree; keep both tables; the `rebuild` row's seconds are the cost
     the retirement weighs against.
   - M5, the resolutions: the two `cargo tree` lines of Input, and
     each binary crate's with `-p` (for the record, not the pin).
   The hypotheses, each with what confirms it:
   - H1, feature unification through a dev-dependency: confirmed if
     `acq-mcp`'s hash changes after the test stage and returns after
     the build at M1 while `acq`'s and `acqd`'s do not, and the test
     stage writes a second `acq_mcp-` form in `deps/`; the pre-read
     predicts exactly this and nothing wider.
   - H2, the missing test target: confirmed if `acqd`'s hash and inode
     are untouched by the nextest line at M2 and M3, and M2's first
     nextest run fails at the daemon's door (a contract mismatch, or
     the harness naming the build step).
   - H3, the profile: refuted if the `acq-` form the test stage writes
     at M1 is the plain build's file (same inode or same hash): cargo
     builds non-test units of a test run under the dev profile.
   - H4, "nine crates twice": confirmed as the unit-test harness, not
     features, if M2's test stage compiles each workspace crate as a
     lib and again as a test, and M2's final build compiles nothing
     below `acq-mcp`. Not removable; named in Not in scope.
   - H5, `--all-targets`: examples and benches are built under the
     test graph into `target/debug/examples`; no artifact under
     `target/debug` changes for them. Confirmed by the record.
   - H6, the doc stage: the gate's comment says it rewrites `acq-mcp`;
     refuted if the hashes after the doc stage equal those after the
     test stage. The comment is corrected either way.
   - H7, a `-p` build is a third graph: M5 shows it; out of scope.
   The commit of Do 1 is the measurement's record: a table of hashes
   per stage per tree, the seconds, the `Compiling` counts, the one
   package the resolutions differ on, and which hypotheses held.
2. **Remove the first cause.** Name `num-traits` with `std` in the
   plain graph where it is linked, `crates/acquisition-mcp/Cargo.toml`,
   one line with the store's `rusqlite` comment as its model: linked
   through rmcp's chrono; the feature is proptest's, unified into every
   test build, and compiled in always so one build writes one binary.
   Verify: the two `cargo tree` lines of Input now agree on every
   package present in both (a diff restricted to those packages is
   empty). If a reviewer or the arch-check refuses a dependency the
   crate never names, the alternative is proptest without its `std`
   feature in the plan and the search, if their property tests still
   compile; try it only then, and say which was built.
3. **Remove the second cause.** Give `crates/acquisition-daemon` one
   integration-test binary in the C122 shape (`autotests = false`, one
   `[[test]]` named `main` at its root under `tests/`), pinning the
   door the source states and nothing pins: `acqd` with any argument
   exits with status 2 and names the frontends and the environment as
   its door, driven through `CARGO_BIN_EXE_acqd` with std alone. The
   daemon links no client, so the client's harness is not available
   to it and the test needs none: no world, no socket, no daemon
   started. Its existence makes cargo build and uplift `acqd` on every
   `--workspace` test run; that is a consequence the commit message
   states, not the test's reason.
4. **The pin.** The gate's last stage, renamed from `rebuild` to
   `artifacts`: record the three hashes, run `cargo build
   --workspace`, and fail if any hash changed or the build printed a
   `Compiling` line; the failure names the artifact that moved and the
   two `cargo tree` lines that diagnose a feature divergence. It costs
   what the stage costs today at a green tree (the no-op build and
   three hashes), and it is a property at the boundary (what the test
   build leaves on disk), not the mechanism (equal feature graphs).
   Breaker, in the same commit: the stage run at the tree before Do 2
   and Do 3 fails on `acq-mcp` (H1) and, after a daemon edit, on `acqd`
   (H2); after them it passes. Until the owner's word (Do 6) the stage
   still performs the build, so nothing is retired by this commit: the
   repair becomes a repair that reports.
5. **The words that follow from the tree.** The gate's closing comment
   (the doc stage per H6; the stage's new name and purpose); the
   harness messages, kept where they stay true (a `-p` run still
   builds no `acqd`); the `--only` name in the gate's usage. Not
   `AGENTS.md`, not `decisions/daemon.md`, not the two skills: those
   wait for Do 6.
6. **The ruling, after the measurement, in the report.** One packet
   for the owner: the measured table, and the recommended text below.
   Built only on his word, in one commit: `AGENTS.md`'s "Build order"
   replaced; C84 extended; the session-close skill's "Why build order
   matters" reduced to the release-form sentence it also carries; the
   mock-session skill's "after any test run" dropped; the Rules
   paragraph for future briefs loses "and after any test run". If the
   measurement shows the two fixes do not leave the three artifacts
   unchanged across the nextest line, the packet says instead what the
   rule still guards, and the hard rule stands.

   Recommended text (the owner edits as he sees fit; 724 bytes, measured):

   `AGENTS.md`, "Build order": **Build order.** Run `cargo build
   --workspace` before anything that starts a daemon. A `--workspace`
   test run writes the same `acqd`, `acq` and `acq-mcp` as that
   build, held by the gate's last stage, so no build is owed after a
   test run; a `-p` run writes no `acqd`. Never run the live drivers,
   a bare `acq` or a second gate while the gate runs.

   C84, one sentence appended before *Why*: One tree writes one
   artifact: the workspace's plain build and its test build resolve
   one feature graph and leave the same executables, so a test run
   never leaves a sibling behind the frontend that would spawn it.
   *Pinned* gains `tools/gate.sh` (the artifacts stage) and the
   daemon crate's integration test.

## Done when

- The commit of Do 1 carries the table: for the docs tree and the
  store-edit tree, the three hashes after each of build, test, doc and
  build again; the seconds per command; the `Compiling` count of each
  final build; the `rebuild` row of both gate tables; the one package
  on which the two resolutions differed, and each hypothesis marked
  held or not from the table.
- `cargo tree --workspace -e no-dev,features` and `cargo tree
  --workspace -e features`, printed as `{p} {f}` with no prefix, agree
  on every package present in both; the message of Do 2's commit shows
  the command and its empty result.
- `crates/acquisition-daemon` has `autotests = false`, one `[[test]]`
  entry, and one integration test that drives `acqd` by
  `CARGO_BIN_EXE_acqd` and asserts the exit status and the words of the
  no-arguments refusal; `tools/arch-check.sh` is green on it.
- At a store-edit tree, after `cargo nextest run --workspace
  --all-targets`, `cargo build --workspace` prints no `Compiling` line
  and the three hashes are unchanged; the message of Do 4's commit
  shows the measurement, and the breaker's failure at the pre-fix tree.
- `tools/gate.sh`'s last stage is named `artifacts`, fails on a
  changed hash or a compile, and its seconds at a green docs tree are
  within the `rebuild` row's of Do 1 (the number in the message).
- `AGENTS.md`, `decisions/daemon.md` and the two skills are untouched
  by the step's commits before the owner's word; the report holds the
  ruling packet with the measured table and the recommended text.
- `tools/gate.sh` green on the step's final tree.

## Not in scope

- The contract revision's inputs: a store comment moves the revision
  and recompiles the protocol crate and everything above it by design
  (C84; the build script's "conservative in two ways"). The 163 s of
  the auditor's row is mostly that and the unit-test harness (H4),
  not features; a narrower digest is a ruling, the owner's, not this
  step's.
- The unit-test harness compile (each crate as a lib and as a test)
  and the dev profile (`opt-level`, `incremental = false`, `77771530`):
  the owner's, measured there.
- The `-p` resolution (H7): the harness refuses a `-p` test run by
  name, and the mock-session skill's build line stays `--workspace`.
- The target directory's size: `cargo clean` at a phase close
  (session-close skill).
- Moving the CLI crate's `acqd` retired-knob test into the daemon
  crate: it is pinned once already; C122 does not ask for a move.
- The handshake, the locator and the artifact comparison
  (`acquisition-client`): nothing here changes what a client compares,
  only what the test build leaves for it to compare.
- CI's stage timing and the Windows gate (r1): CI runs the stages
  singly and does not call the gate script.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `p88`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands as tracked background tasks; `tools/gate.sh`, and while
the hard rule stands (until Do 6 lands) `cargo build --workspace`
before any daemon and after any test run (C84, the artifact mismatch);
a measurement runs alone on the machine, with no gate, build or other
driver beside it, and the before-run is taken at HEAD before the first
edit; an edit made for a measurement is reverted before the next and
never committed; a fix round's done-when is the template's.

## Report

`TEMPLATE.md`, with the ruling packet of Do 6 as its own section
after the measurements.
