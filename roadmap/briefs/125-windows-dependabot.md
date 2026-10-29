# 125 — the Windows gate red on `main`, and the first Dependabot PRs

Issue #125, with the owner's word of 2026-10-29: "Yes: after #89, a
step for the Windows gate on main and the six Dependabot PRs, before
#115. I merge." Next in "Pull-forward" (`roadmap/README.md`) after
#89's close at `a8c5c5be`. The rhythm is "A step". No ruling is needed:
nothing here changes what blocks a release, raises the toolchain pin
or drops a platform from the gate.

## Goal

`Gate (Windows)` is green on `main` again, by fixes whose cause is
named, so that a red check on a pull request says something about the
pull request; and each of Dependabot's six first PRs (#119 to #124) has
a fate the owner can act on in the GitHub UI, read against that
baseline: safe to merge as is, needs a fix that lands on `main` first,
or closed with its reason. The weekly Dependabot run then brings bumps
whose checks mean what they say.

## Read

- `AGENTS.md`: "The gate decides"; C122 (a behaviour pinned once, the
  one harness); invariant 1 (nothing here touches a GGG path).
- `roadmap/briefs/86-weekly-automation.md`, Do 5: what
  `.github/dependabot.yml` was written to do (cargo minors and patches
  grouped, each major alone, the actions grouped).
- `.github/workflows/gate-windows.yml`: the build, the suite under the
  `ci` profile, then clippy under `!cancelled()`; the only lint of the
  `cfg(windows)` code. `gate-linux.yml`: cargo-deny in a container
  (its "toolchain not installed" line is the container installing the
  pin, not a failure).
- `deny.toml:54`: `multiple-versions = "allow"`, so a major that leaves
  the old version in the graph is not refused.
- `crates/acquisition-verbs/src/diagnose.rs`, `Redactor::new`: a path
  is a needle only when `is_absolute()`, and on Windows `/srv/acq/store`
  has no drive, so is relative; its unit tests build such paths.
- `crates/acquisition-cli/tests/harness/mod.rs`: `Transcript::run`
  checks each fact against the raw output, while the golden is written
  under the `scratch` rule, whose second half writes a path under
  `<store>` or `<logs>` with forward slashes. A fact with a forward
  slash under a placeholder holds on Unix alone.
- `crates/acquisition-verbs/tests/daemon.rs`: the existing idiom for a
  printed path, `.replace('\\', "/")` before comparing (1135, 1197).
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world": "A Unix mechanism assumed in product code", held by the suite
  on Windows in CI. The checklist has no process section.
- Issue #76: the journey golden folding two ages compared in one
  second, the same failure seen once in this streak.
- Issue #72: `lru` 0.12.5 under `ratatui` 0.29, closing when `ratatui`
  takes an `lru` at or past 0.16.3.

## Input

The CI evidence, read with `gh run list --branch main --workflow "Gate
(Windows)"` and `gh run view <id> --log-failed` on 2026-10-29:

- The last green push is `b0296d81` (run 40503726268, 2026-10-27). The
  first red is `f305248a` (run 40582878225), a push of 93 commits, red
  on every push since through `9adad710` (run 40863794738), fourteen
  red and one cancelled; the issue's eight are the last eight. `Gate (Linux)` is
  green on `9adad710`.
- The steady failures, the same on every red run: four tests, each a
  Unix path spelling asserted on a Windows run, and two clippy lints,
  each a test-only item whose only caller is `#[cfg(unix)]`:
  - `diagnose::tests::a_path_under_another_takes_the_nearer_placeholder`
    and `an_account_is_replaced_as_a_whole_word_in_each_spelling`
    (`acquisition-verbs`, unit): left `"/home/al/s/x and /home/al/y"`,
    the paths not replaced, being relative on Windows. Since
    `abc7e71d` (#79, the diagnosis).
  - `a_diagnosis_with_no_daemon_reads_every_section_and_holds_nothing_planted`
    (`acquisition-verbs::daemon`): left `[    5s] store:
    <store>\mock\<account 1>.db opened`, right the same with `/`.
  - `diagnose::a_diagnosis_with_no_daemon_prints_every_section_and_nothing_planted`
    (`acquisition-cli::main`): the fact `"== journal: the last 3 lines
    of <logs>/mock/sends.jsonl (the world's)"` against a printed
    `<logs>\mock\sends.jsonl`.
  - clippy, `dead_code`: `keyring_words`,
    `acquisition-daemon/src/daemon/tests/credentials.rs:598`, called
    only by the `#[cfg(unix)]` C148 test; since `fb92f468`, in the same
    push.
  - clippy, `unused_imports`: `use std::process::Command`,
    `acquisition-cli/tests/provider_default.rs:12`, used only by the
    `#[cfg(unix)]` Ctrl-C test; since `5e05767c` (#87), first red at
    `17de2934` (run 40723023202).
  The product is right in each: Windows prints its own separator, and
  the redaction replaced every planted path and account.
- Two one-off failures, each on one run: the journey golden's
  `stash and characters listed` fold (run 40728020451, `9a91ca53`),
  #76's; and `seat_faults::tabs_says_its_counts_first_and_counts_a_large_league`
  (run 40807489901, `c308cf0d`, 27.4 s on the runner), whose golden's
  `fetched` column is one space narrower: the `age` rule replaces the
  number, not the column's padding, which the widest raw age set.
- The six PRs (`gh pr checks`, then `--log-failed` on each red job):
  | PR | Bump | Linux | Windows |
  | --- | --- | --- | --- |
  | #119 | actions: checkout 6→7, upload-artifact 4→7, download-artifact 4→8 | green (40863955579, 40863971066) | main's four and two (40863955445, 40863971182) |
  | #120 | cargo group, 6 updates (rmcp 3.1.4→3.7.0, rmcp-macros →3.8.0, …) | red: `rmcp::model::ServerInfo` deprecated, three uses in `acquisition-mcp/src/main.rs` under `-D warnings` (40864059623, 40864070049) | the same, beside main's (40864059472, 40864070353) |
  | #121 | sha2 0.10→0.11 | green (40864078452, 40864089715) | main's alone (40864078439, 40864089617) |
  | #122 | base64 0.22→0.23 | green (40864087594, 40864098016) | main's alone (40864087853, 40864097993) |
  | #123 | getrandom 0.3→0.4 | green (40864097819, 40864110327) | main's alone (40864097874, 40864110506) |
  | #124 | ratatui 0.29→0.30 | green (40864116606, 40864127963) | main's alone (40864116525, 40864127984) |
  #119 was rebased by Dependabot at 13:55 onto `a8c5c5be`; its new
  runs were pending when this was read.
- What the bumps add, measured on the PRs' diffs: #121 six lock
  entries (sha2 0.11 and its digest stack), #122 and #123 none (base64
  0.23.1 and getrandom 0.4.3 are already locked), #124 78 entries and 6
  removed, `lru` 0.12.5 → 0.18.5; `cargo tree -p acquisition-cli -e
  normal,build --prefix none | sort -u | wc -l` 177 on `main`, 203 with
  #124 (termwiz is in the lock, not the build). `ServerConfig` is not
  in rmcp 3.1.4 (only `ServerInfo`, `model.rs:1085`); in 3.7.0 both
  name `InitializeResult`, `ServerInfo` deprecated (`model.rs:1341`,
  `1368`).
- A Windows-target clippy runs on this Mac with a stub C compiler for
  the build scripts (`CC_`/`AR_x86_64_pc_windows_msvc` pointing at a
  script that writes empty outputs; clippy links nothing): it reports
  `keyring_words`, as CI does, and `--keep-going` lints every crate.

## Do

1. **The diagnosis tests spell a path as the platform does**, one rule
   in each place a test asserted a Unix spelling: the redactor's unit
   tests build absolute paths for the platform (a drive on Windows);
   the verbs test reads its log lines' paths with forward slashes, the
   crate's idiom; the CLI harness reads a `Transcript` fact also under
   the `scratch` rule's path half, so a fact is written as the golden
   writes it. The product does not change.
2. **No test-only item outlives its only caller's `cfg`**:
   `keyring_words` is `#[cfg(unix)]`, and `provider_default.rs`
   imports `Command` inside its Unix test. The Windows-target clippy
   with `--keep-going` (Input) is clean after.
3. **The cargo group lands on `main`**: #120's `Cargo.lock` as
   Dependabot wrote it, and `ServerInfo` → `ServerConfig` in
   `acquisition-mcp`, the same type. One commit; the gate green on it.
4. **A checklist row**: Windows red from a test's Unix assumption came
   back (`b1d9cf3c`, r1, and here twice), so it joins
   `REVIEW-CHECKLIST.md` beside its product-code twin.
5. **The tabs golden's padding** is filed as an issue with its run,
   beside #76, not fixed here: it is not a cause of the steady red.
6. **`.github/dependabot.yml`'s majors, weighed**: grouped, ignored
   until asked, or left as is, with the reason, in the close.
7. **The fates**, in the close message and the report as a table, with
   what only the push shows and how the owner checks it.

## Done when

- The four tests and two lints of Input pass on the step's tree, read
  on this Mac for the tests and through the Windows-target clippy for
  the lints; Windows itself is seen only on the push (not verifiable
  here, named in the close).
- No test in the diff asserts a path with a literal `/` where the
  product prints the platform's separator, and no product code changed
  for the red.
- `Cargo.lock` holds #120's six updates and `acquisition-mcp` names
  `ServerConfig`; `MCP-REFERENCE.md` unchanged.
- `REVIEW-CHECKLIST.md` holds the row; an issue holds the tabs golden.
- Each PR has a fate with its evidence, and the majors' handling a
  reason.
- `tools/gate.sh` green on the step's tree.

## Not in scope

- #76, the journey fold: its own issue.
- #70, the Windows deadline flakes: closes on its own week.
- Merging, closing or commenting on any PR, and `@dependabot`
  commands: the owner's.
- A Windows stage in the local gate, or a check that a step's start
  reads the last push's CI: process, raised for review.
- The toolchain pin.

## Rules

`TEMPLATE.md`. Scratch under the session's scratchpad, `125/`.

## Report

`TEMPLATE.md`.
