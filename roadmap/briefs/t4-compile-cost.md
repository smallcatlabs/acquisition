# t4 — the compile cost

Issue #140. The test-shape group's last step of its first tranche
(`roadmap/README.md`, "Pull-forward", item 2; `roadmap/TEST-SHAPE.md`
§1), after t2's close at `d6a6a039` and t3's closing undone. The rhythm
is "A step". Every change here is a measurement first, and a change
the numbers do not pay for is declined by them.

## Goal

After one store edit the gate's `test` stage is 86 s and `build` 28 s
(t2's after-table, `8b02f38b`), and most of it is compile, not run. Two
mechanical causes are named and unmeasured: tests inside a library's
`src/` recompile with every library build and every crate above it,
and the protocol crate's contract revision hashes the store's whole
`src/`, tests included, so a store test edit recompiles everything
that links the protocol. This step measures each cause, removes the
ones the numbers pay for, and brings the one ruling with its numbers.

## Read

- `AGENTS.md`: C1 (the workspace's edges), C47 (no `unwrap` in the
  store's and the plan's production code: tests moved to `tests/`
  keep their own lints), C84 (the artifact mismatch, and the runtime
  identity the contract revision feeds), C122 (a crate's integration
  tests one binary, `tests/main.rs`; a file that sets the process
  environment its own binary), C154 (the retirement rule).
- `TESTING-NOTES.md`, "The suite's shape".
- `Cargo.toml:33-50`, `[profile.dev]`: `incremental = false`, set for
  disk (a fresh worktree 15 GB, 9.3 GB of it `incremental`; the main
  checkout at 124 GB on 2026-10-25), "the gate builds whole anyway".
- `crates/acquisition-protocol/build.rs:44-57` and `digest.rs`: the
  revision's inputs, `crates/acquisition-store/src` among them, whole.
- `roadmap/briefs/t2-gate-trims.md` and `8b02f38b`'s message: how the
  gate was measured (the full gate warm, run twice, the second taken;
  then `touch crates/acquisition-store/src/lib.rs` and the full gate).

## Input

Read at `d6a6a039`. Counts of lines are `awk` from a `#[cfg(test)]`
followed by `mod` to the file's end; the build re-reads them.

| Crate | Inline test modules (lines) | `src/` lines |
| --- | --- | --- |
| store | `annotations.rs` 1353, `snapshot.rs` 916, `corpus.rs` 568, `world.rs` 529, `index.rs` 266, `jobs.rs` 199, `age.rs` 111, `bound.rs` 47, `lib.rs` 43 | 15,030 |
| plan | `listing.rs` 1652, `shop.rs` 1516, `pricing.rs` 896, `price.rs` 831, `game_side.rs` 405, `currency.rs` 208, `lib.rs` 81 | 16,428 |
| daemon | 22 files under `src/**/tests/`, 18,541 lines; 13 `cfg(test)` sites | 34,551 |

The three one-test environment binaries (a test that calls
`session(`): `crates/acquisition-cli/tests/daemon_stalled.rs`,
`crates/acquisition-cli/tests/watch_recovery.rs`,
`crates/acquisition-mcp/tests/plan_loop.rs`; `TEST-SHAPE.md` §1
measured 37 CPU-s of compile for their 3 tests.

The last push's CI: the build reads both gates' last runs on `main`
before its first commit and names them in it. The owner has not
pushed since `9d3930d8`; the commits after it are local.

What each time is: "wall" is seconds on the clock for the command;
"CPU-s" is cargo's `--timings` unit time summed. Every number names
which (t3 closed undone on runner time read as wall time).

## Do

1. **Measure before**, the machine alone in the owner's window: the
   one-store-touch gate as t2 measured it, and `cargo build --workspace
   --all-targets --timings` after the same touch, saving the HTML
   report's per-unit times for the store, the plan, the protocol and
   the daemon libraries with and without their test units.
2. **Incremental.** The one-store-touch gate with
   `CARGO_INCREMENTAL=1`, and `du -sh target` after it against the
   same before. Adopt only if the gate is faster by more than the
   noise between two runs of the same command; a change to
   `[profile.dev]` that grows the target directory is the owner's
   (his disk), brought with both numbers. Otherwise declined in a
   commit message with the numbers, no code change.
3. **The digest.** `digest.rs` skips what cannot change the contract:
   a file under a `tests/` directory inside `src/`, and, if the
   measurement of Do 1 says the store's inline modules matter, a
   `#[cfg(test)] mod` body (a sibling file `foo/tests.rs` is the shape
   that makes this a path rule, not a parse). The revision moves once;
   the commit says so. A store test edit then recompiles no crate above
   the store's test unit: measured by touching a test file.
4. **Tests out of `src/`**, the store and the plan, only if Do 1's
   numbers show the libraries' compile carries them: tests on the
   public API move to the crate's `tests/` as modules of its one
   binary (C122); a test that reaches a private item or forges rows by
   SQL stays inline, moved to a sibling `tests.rs` so Do 3's path rule
   covers it. Moving a test changes no assertion: the commit says the
   test count before and after is equal. No retirement here; a test
   found redundant on the way is a `retired X; held by Y` line only if
   it is on `TEST-SHAPE.md`'s lists, otherwise reported.
5. **The three binaries: the ruling, brought with numbers.** Measure
   their compile (CPU-s) and the gate's `test` stage with and without
   them folded into their crate's `tests/main.rs`. They are their own
   binaries because a process-wide environment write leaks into every
   test beside it under `cargo test`; under nextest each test is its
   own process. Folding them drops the `cargo test` fallback's
   isolation (C122's exception). Stop and report the numbers and a
   recommendation; build nothing for this item until the owner rules.
6. **Measure after**, as Do 1, on the step's tree; the full gate's exit
   status gates the "built" commit, which carries both tables.

## Done when

- Do 1's table and Do 6's, the same commands, are in the "built"
  commit, each number marked wall or CPU-s.
- Incremental adopted with the owner's word on the disk, or declined
  with its numbers in a commit.
- A store test edit recompiles no crate above the store: the touched
  test file and the units cargo rebuilt are named in Do 3's commit.
- Every moved test is counted, the count equal before and after.
- The gate's `test` stage after one store touch is faster by a stated
  number, or each change is declined by one.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

The daemon's inline tests: its harness leaves `src/` with the door
collapse in 4-0 (t5, #141; `TESTING-NOTES.md`, "What a rewrite
keeps"); measure them in Do 1, move none. The search's tests (t7).
The trade pair's 6 s (t2's report: computation at opt-level 1) unless
Do 1 shows it is compile.

## Rules

`TEMPLATE.md`. What differs:
- The group's review rule: Do 3 changes non-test code (the build
  script), so the build is reviewed.
- Measurements and builds run only in a window the owner names, and a
  measurement runs alone.
- The report goes to the notes file the launch names, in full; the
  note relayed back is 400 characters.
- Before a remedy, name its measured cause (t2: the trade pair's time
  was not the parse the brief supposed).
