# 0e — Dev-build levers, measured

## Goal
The build-and-gate loop every agent pays is faster, by profile settings
alone, with the numbers in the commit message. Debug runs the corpus
load 5.6 times slower than release; 84 test binaries each link SQLite
and the search crate and each gets a dSYM; `target/` is 86 GB.

## Read
- `roadmap/STATE-2026-10-19.md`, report 5, section 4; report 2,
  section 6 (the daemon hashes its own 27 MB debug executable at every
  start with unoptimised `sha2`).
- `Cargo.toml` `[profile.dev]` and its comment (the September
  split-debuginfo incident, note 16).
- The gate's timing taken 2026-10-19: 937 tests, 235 s wall, 84
  binaries; the three slowest `contract.rs` 36 s, `trade_forms.rs` 32 s,
  `trade_oracle.rs` 30 s.

## Input
A clean baseline: `cargo build --workspace --all-targets --timings`
and a timed `cargo test --workspace --all-targets` on the current
profile, recorded before any change. Every measurement runs alone: no
other worker's build, gate or measurement on the machine while it
runs.

## Do
1. Measure the baseline (build time, test wall, `target/` size).
2. Try, one at a time, each with the same measurement:
   `[profile.dev.package."*"] opt-level = 2` (dependencies only);
   `sha2` at `opt-level = 3` alone, if the first is rejected;
   `[profile.dev.package."*"] debug = false` or `"line-tables-only"`.
3. Keep what pays: a change that costs more than a few seconds on a
   warm incremental build without a larger gain on the gate is not
   kept.
4. Report the `target/` size and what a `cargo clean` would recover;
   do not clean it yourself.
5. Evaluate `cargo nextest` for the gate from the timings alone (do
   not install tools on the owner's machine): the expected gain, and
   whether the CI workflows should adopt it.

## Done when
- A table of before and after per lever is in the report and in the
  commit message of the profile change.
- The gate is green on the kept profile; `acq` and `acqd` still build
  as one form each (session-close, "Why build order matters").

## Not in scope
- One integration-test binary per crate (phase 2b: it moves test
  files).
- Release-profile changes (LTO, the allocator): a later measurement
  under the M3 method.

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`; the table is section 3.
