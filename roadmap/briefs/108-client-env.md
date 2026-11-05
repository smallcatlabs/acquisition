# 108-client-env — the client's unit tests restore the environment and need no built acqd

Issue #108, built at the owner's word of 2026-11-04 (the night run),
`trigger:gui` waived. Written by the orchestrating session against the
tree at `5e536edc` and checked again after #104's close.

## Goal

Two leftovers of h3 in `crates/acquisition-client/src/client.rs`'s
unit tests. Three tests set or remove `ACQ_*` variables by hand and
remove what they set at the end instead of restoring it, so a test
that panics midway, or a caller whose environment had the variable,
leaves the process's environment changed for the next test; `Moved`
exists to restore. And `c147_a_replacement_spawned_after_the_budget_ran_out_gets_its_own`
spawns the real `acqd` the workspace build wrote, so
`cargo nextest run -p acquisition-client` on a fresh tree fails with
"run `cargo build --workspace` first". When this step is done every
environment-moving test restores through `Moved`, and the property the
replacement pin holds is held by pins that need no built `acqd`.

## Read

- `AGENTS.md`, C122 (a behaviour pinned once, at the lowest crate that
  holds it), C153, C154 (`retired X; held by Y`).
- `decisions/daemon.md`: C147 (a child that exited on the world lock
  gets one more look for the winner before the spawn is called failed;
  *Pinned:* the `c147_` tests), C137 (a deadline's contract is its
  expiry), C157 (a lifecycle that waits is a value: the door as a
  machine pinned by event scripts, its driver by real-wiring tests).
- `REVIEW-CHECKLIST.md`, the daemon/client section.
- `crates/acquisition-client/src/door.rs`: the machine; `Event::Spawned`
  arms `Timer::Startup` (about line 319); the table's `lost`,
  `replaced` and `respawned` helpers (about lines 587–660) and the rows
  built from them.
- `crates/acquisition-client/src/client.rs`: the driver's
  `Effect::Arm(timer) => Armed::after(duration(timer))` (about line
  1487); `ENV` and `Moved` (about line 2729); the c147 test (about
  line 2625); the hand-set tests `c84_a_daemon_of_another_contract_…`
  (about line 2876), `an_absent_daemon_names_why_it_was_not_started`
  (about line 3022) and the test that sets `ACQ_NO_SPAWN` (about lines
  3053–3075).

## Input

The sweep, run on the tree this brief was written against:
`grep -n "std::env::set_var\|std::env::remove_var" crates/acquisition-client/src/client.rs`
outside `Moved`'s own body lists lines 2884, 2885, 2995, 3009, 3027,
3028, 3053, 3074, 3075 (three tests). Run it again first; the measure
of done is that it prints only `Moved`'s lines.

## Do

1. **Every environment-moving test restores through `Moved`.** The
   three tests take `Moved` under `ENV` and set or remove through it;
   no hand `set_var`/`remove_var` remains outside `Moved`. Their
   assertions unchanged.
2. **The replacement pin without a built `acqd`.** Name what the c147
   test holds that nothing else does. The design's split (C157) says
   where each part lives: that a replacement's spawn arms a fresh
   startup deadline after the first was spent is the machine's (a row
   of the door's table: a lost child, the winner stopped, the respawn,
   `Arm(Timer::Startup)` and a `Pending` consult, not a spent-deadline
   report); that an `Arm` replaces whatever end the timer had is the
   driver's. Choose one:
   (a) the row exists or is added to the table; the driver's arm is
   pinned without a process (a unit test of the driver's timer state:
   a timer armed after a spent one is pending, not expired); the c147
   test retires with `retired c147_a_replacement_…; held by <row> and
   <driver test>`, and C147's *Pinned:* still reads true (other `c147_`
   tests remain; check);
   (b) if (a) leaves part of what it held unheld, keep the test and
   give it a stand-in instead of the real `acqd`, through a seam that
   exists only under `cfg(test)` and changes no production path (C82:
   what a door spawns is the sibling `acqd`), and say why (a) did not
   suffice.
   Prefer (a): it removes a process from a unit test and pins each
   part at its layer. Record the choice and its reason in the commit.
3. `cargo nextest run -p acquisition-client` passes on a tree where
   `target/debug/acqd` is absent (move it aside, run, put it back;
   say so in the report).

## Done when

- The Input's grep prints only `Moved`'s own lines.
- No unit test in `acquisition-client/src` spawns `acqd` or asserts it
  was built; the client's unit tests pass without it (report).
- The replacement's fresh deadline is held by a named pin at the
  machine and one at the driver, or (b)'s stand-in, the choice and its
  reason in the commit; any retirement carries its `retired X; held by
  Y` line.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- The door machine's behaviour, `open_door`'s production paths, the
  locator (C82). No production change unless (b) needs a `cfg(test)`
  seam.
- The process tests under `acquisition-client/tests/`, which place the
  built `acqd` by the harness on purpose.

## Rules

`TEMPLATE.md`, and as #18's brief: the main checkout on `main`, commit
locally, no push; every `acq`/`acq-mcp` in a mock world
(`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch), never a bare
`acq` or a real-provider daemon; C153 and C154.

## Report

`TEMPLATE.md`.
