# 4-full-1, step 6.3 — The spike: the check's driver

## Goal
The second question of step 6's spike is answered by measurement:
what drives the browser-driven check, and what it adds to the gate.
The record gains the driver's table and two shapes for the owner, who
picks the driver and the check's home before step 6.4 is briefed.
`main` gains the record's section and nothing else.

## Inputs
- `4-full-1-step6-spike.md`, built first: its copy outside the
  repository, its prototype's case F, its record
  (`audits/web-2026-11/spike/README.md`).
- `4-full-1-design.md` §2 (the check is for a presentation rule
  inside a script: the picker's today, the window's at step 6) and §7
  item 6 (a headless browser from a Rust test with no Node, or a
  CI-only job, measured by what it adds to the gate; two shapes to
  the owner).
- `decisions/frontends.md`, "Not doing": no Node or bundler in the
  gate.
- `tools/arch-check.sh`, check 5: no member reaches an HTTP client but
  through the transport crate, a dev-dependency's reach included.
  `crates/acquisition-cli/Cargo.toml`, the feature pins: a feature a
  dev-dependency asks unifies into every test build (C84, the
  artifact mismatch), which `tools/gate.sh --only features` checks.
- `deny.toml`: the licences allowed.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Where it lives.** The spike's copy outside the repository, each
   candidate tried there and never committed; if the copy is gone, a
   fresh `git worktree add --detach` and the window brief's diff from
   the notes folder.
2. **One check, written once per candidate.** A test of the cli
   crate's integration tests starts `acq web` in a mock world over
   the wrapped copy, loads case F, scrolls to row 15,000, and asserts
   that the rows shown are the ones the server answers for that
   offset and that the element count is bounded.
3. **The candidates.**
   - (a) A CDP client crate as a dev-dependency of the cli crate.
     Read the maintained ones' current docs and take one that
     downloads no browser.
   - (b) Hand-written: the browser launched headless with a debugging
     port, the protocol spoken over a hand-written loopback WebSocket
     client, no new crate. Capped at half the step's time; record how
     far it got.
   Ruled out: WebDriver (a driver binary matched to each browser's
   version) and anything that fetches a browser.
4. **A candidate is admitted by the tree's own checks, unchanged.**
   With the candidate in the copy: `tools/arch-check.sh`,
   `tools/gate.sh --only features` and `cargo deny check` each exit 0.
   One that fails any is recorded with the failing line and measured
   no further; no check is changed to admit it.
5. **What is measured, for each admitted candidate and for the copy
   without one.** Each number is beside its command in the record,
   and said as measured on this Mac:
   - crates in `Cargo.lock` (`grep -c '^name = ' Cargo.lock`);
   - a clean test build: `cargo clean`, `cargo build --workspace`,
     then the seconds of `cargo nextest run --workspace --all-targets
     --no-run`;
   - an incremental one: the same command after touching the check's
     own file;
   - the check alone: the median of 20 runs and the failures in 20;
   - the browsers here it drives (Brave, Chrome), and what a machine
     must have installed.
   The difference of a candidate from the copy without one is what it
   adds. One build at a time, and no gate while one runs.
6. **The check's home is the owner's.** The record gives both shapes
   with the numbers: a gate stage (a browser on every gate machine; a
   missing one fails, never skips) or a CI-only job on hosted
   runners. What a hosted image has installed is read from its
   published list and said as read, not run; a cost in CI is said as
   projected from this Mac's numbers, never as measured. No workflow
   or gate change, in the copy or on `main`.

## Find first
- `grep -n "http_clients=\|http_wire=" tools/arch-check.sh`: the
  crates check 5 names, read before choosing a candidate.
- `git grep -n "feature pins" -- crates/acquisition-cli/Cargo.toml`:
  where a pin would stand if a candidate needs one, for the record.
- `git grep -n "SCRIPTS\|fn no_state" -- crates/acquisition-cli/tests`:
  what a served script joins today, for the record's note to 6.4.
- `git grep -n "browser-driven" -- REVIEW-CHECKLIST.md`: the row the
  check will take over.
- `grep -c '^name = ' Cargo.lock`, before each candidate.

## Done when
- The record's driver section is on `main`: Decision 5's table, each
  candidate's admission under Decision 4, and the two homes of
  Decision 6 with their numbers.
- Two decisions are drafted in it for the owner, each ready to file:
  the driver, and the check's home. The launching session files them.
- The copy is removed and its worktree pruned; the candidates' diffs
  are in the notes folder, named in the record.
- `tools/gate.sh --docs` exits 0 on `main`'s commit.

## Not in scope
- The picker's check, and the window's check as the gate will run it:
  the step that follows the owner's pick.
- A change to a workflow, the gate or a check.
- A dependency on `main`.
- The window script: its numbers are the window brief's.

## Rules
`TEMPLATE.md`. Every `acq` runs in a mock world over the wrapped copy
under scratch, with `ACQ_NO_SPAWN=1`. A browser the builder launches
is headless, on a profile directory under scratch, never the owner's
profile, and opens no window.

## Report
`TEMPLATE.md`.
