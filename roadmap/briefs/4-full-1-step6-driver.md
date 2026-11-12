# 4-full-1, step 6.3 — The spike: the check's driver

## Goal
The second question of step 6's spike is answered by measurement:
what drives the browser-driven check, and what it adds to the gate.
The record gains the driver's section and, where a driver worked, two
decisions drafted for the owner: the driver and the check's home.
`main` gains that section and nothing else.

## Inputs
- `4-full-1-step6-spike.md`, built first: its record
  (`audits/web-2026-11/spike/README.md`), which names the copy
  outside the repository and holds the prototype's patch with its
  base commit; its case F; its headless launch.
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
- C122 (a crate's integration tests are one binary): the check is a
  module of the cli crate's `tests/main.rs`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **Where it lives.** A second copy outside the repository: a fresh
   `git worktree add --detach` at the patch's base commit, the patch
   applied, over its own wrap of the measurement copy. The window's
   copy and its running `acq web` stay as they are for the owner's
   sitting. Nothing is committed there.
2. **The check, written once per candidate.** A test starts `acq web
   --no-browser` in a mock world over the wrapped copy, loads case F
   in a viewport of a fixed size, and scrolls to row 15,000. It
   passes when: the rows the viewport covers are, id for id and in
   order, the rows the prototype's `rows` route answers as JSON at
   that offset;
   and the rows in the DOM number at most what the prototype's window
   asks (three viewports' worth), the same count at row 0 and at row
   15,000, whatever the matches total.
3. **The candidates, the lightest first.**
   - (c) No protocol: the browser launched headless at a URL, as the
     window brief's headless run is, and a script in the page reports
     the row ids and counts to the test. Its cost is what the page
     must carry for it: say what, and whether `main` would serve it.
   - (a) A CDP client crate as a dev-dependency of the cli crate.
     Read the maintained ones' current docs and take one that
     downloads no browser.
   - (b) Hand-written: the browser launched headless with a debugging
     port, the protocol spoken over a hand-written loopback WebSocket
     client, no new crate. Abandoned if its client passes 400 lines
     before the check passes; record how far it got.
   Ruled out: WebDriver (a driver binary matched to each browser's
   version) and anything that fetches a browser.
4. **A candidate is admitted by the tree's own checks, unchanged.**
   With the candidate in the copy: `tools/arch-check.sh`,
   `tools/gate.sh --only features` and `cargo deny check` each exit 0.
   One that fails any is recorded with the failing line and measured
   no further; no check is changed to admit it.
5. **What is measured.** The control is the copy with the prototype
   and no check. Each number stands beside its command in the record,
   said as measured on this Mac:
   - crates in `Cargo.lock` (`grep -c '^name = ' Cargo.lock`);
   - a clean test build: `cargo clean`, `cargo build --workspace`,
     then the seconds of `cargo nextest run --workspace --all-targets
     --no-run`;
   - an incremental one: the same command after touching
     `crates/acquisition-cli/tests/web.rs`, in the control and in
     each candidate alike;
   - the check alone, which the control has none of: the median of
     20 runs and the failures in 20;
   - the browsers here it drives (Brave, Chrome), and what a machine
     must have installed.
   A candidate's difference from the control is what it adds. One
   build at a time, and no gate while one runs.
6. **What may be proposed.** A driver is proposed only if its check
   passed 20 of 20 here. If none did, the section says inconclusive,
   names each blocker with its evidence, and drafts the next question
   in place of both decisions.
7. **The check's home is the owner's.** With a working driver, the
   section gives both shapes with its numbers: a gate stage (a
   browser on every gate machine; a missing one fails, never skips)
   or a CI-only job on hosted runners. What a hosted image has
   installed is read from its published list and said as read, not
   run; a cost in CI is said as projected from this Mac's numbers,
   never as measured. No workflow or gate change, in the copy or on
   `main`.

## Find first
- `grep -n "http_clients=\|http_wire=" tools/arch-check.sh`: the
  crates check 5 names, read before choosing a candidate.
- `git grep -n "feature pins" -- crates/acquisition-cli/Cargo.toml`:
  where a pin would stand if a candidate needs one, for the record.
- `git grep -n "pub const SCRIPTS\|pub fn no_state" -- crates/acquisition-cli/tests`:
  what a served script joins today.
- `git grep -n "browser-driven" -- REVIEW-CHECKLIST.md`: two rows,
  page state the scan cannot see and a script rule no test reaches;
  the section says which properties of each a check could take over.
- `grep -c '^name = ' Cargo.lock`, before each candidate.

## Done when
- The record's driver section is on `main`: each candidate's
  admission under Decision 4, Decision 5's table, and either the two
  drafted decisions with both homes' numbers or Decision 6's
  inconclusive outcome. The launching session files what is drafted.
- Each candidate's diff is a patch beside the record, proven to apply
  over the prototype's; this brief's copy is removed and pruned.
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
