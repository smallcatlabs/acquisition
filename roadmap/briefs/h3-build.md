# h3 — The foundations a long-lived frontend sits on

Third of three groups from the open-issue inventory of 2026-10-25,
after h1 and h2. Not a roadmap phase step; "A step" holds whole.

## Goal

`acq-mcp` is already a long-lived spawner and the GUI's backend will
be another. Two things under them are held by construction, not by the
API. The client never reaps the daemon it spawns, drops a stop's
error, and reports `SpawnFailed` when it lost a spawn race it should
have waited out (#69). A frontend over a chosen world is not
expressible: `Frontend` captures its world at construction while
`Client::connect`, `Client::observe`, `Persisted::observe` and
`World::observe` re-read the process environment at each call, and the
layer only asserts the two agree (#49). When this step is done, every
door and every ledger read takes the world it acts on (C147), the
spawned child is reaped, a failed stop is reported, and a lost spawn
race looks once more for the winner before it reports failure.

## Read

- `AGENTS.md`: C1 (the edges), C12 (two surfaces), C46 (shared
  semantics; the frontend door grep in `tools/arch-check.sh`).
- `decisions/daemon.md`: C147 (this step's ruling), C3 (lazy spawn,
  idle exit), C10 (single-version handshake), C83 (the world and its
  lock), C86 (the handshake deadline), C112 (the derived endpoint).
- #69 and #49 whole; the 2a review's finding 1 at
  `7e33eae9:roadmap/reviews/2a-0958403f.md`.
- `crates/acquisition-client/src/client.rs`: `connect` (around line
  1102), `observe` (983 and 1256), `spawn_daemon` (1544), the `Child`
  field (1532), the replace path (1178), the `SpawnFailed` sites;
  `crates/acquisition-store/src/world.rs` (`observe`, 292; the
  environment read, 193), `jobs.rs` (`Persisted::observe`, 375,
  `observe_job`, 320); `crates/acquisition-verbs/src/frontend.rs`
  (`Frontend`, `human`, `agent`, the `same_world` assertion).
- Every caller of the four doors, found by grep on 2026-10-26: the
  client's own 13, `acquisition-cli/src/main.rs` (3 direct calls, each
  to be routed through the `Frontend` or to take its world),
  `acquisition-verbs/src/{daemon,frontend,jobs,refresh}.rs`, the
  store's own, and the tests `acquisition-client/tests/contract.rs`
  (18), `harness/mod.rs`, `acquisition-cli/tests/watch_recovery.rs`,
  `acquisition-mcp/tests/plan_loop.rs`; grep again before the edit; and the tests in
  `acquisition-client/tests/` and `acquisition-cli/tests/world.rs`
  that pin the doors.

## Input

- C147 in the registry at this brief's commit.
- Failing inputs the tests stage: two clients spawning at once on one
  world, the loser's child exiting on the lock while the winner is
  between lock and bind; a stop that fails on the replace path; a
  spawned child whose exit is never waited; a `Frontend` built over
  scratch world B while `ACQ_STORE_DIR` names scratch world A, whose
  every read and door must land in B.

## Do

1. **C147, the world as an argument.** `Client::connect`,
   `Client::observe`, `Persisted::observe`, `observe_job` and
   `World::observe` gain a form that takes a `World` (or the store
   directory and provider the world is derived from); the
   environment-reading form becomes a thin caller of it, kept for the
   binaries' `human` and `agent` doors. `Frontend` passes its captured
   world to every door and ledger read, and the `same_world` assertion
   goes, replaced by the two-world test. The arch-check's edges do not
   move: the client and the store do not learn new crates.
2. **#69, the child.** Keep the `Child` with the client that spawned it
   (or in a reaper the client owns) and `wait` it: on success, when the
   daemon exits or the client drops; on replace, after the stop. The
   stop's error on the replace path is reported, not dropped. A child
   that exited on the world lock leads to one more look for a listener
   (bounded by the handshake deadline's knob, C86, forced in the test
   per C137) before `SpawnFailed`.
3. Pin each with the inputs above; fill *Pinned:* on C147; head each
   mechanism's doc comment with the id.
4. One commit per concern; "Closes #49", "Closes #69".

## Done when

- A `Frontend` over world B with the environment naming world A reads
  facts, intent, the ledger and the daemon in B, pinned.
- The spawn race, the dropped stop error and the unreaped child each
  pinned by forcing the condition, no wall-clock bound.
- `tools/arch-check.sh` green with no edge or forbid line added.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- #1 (the keeper), #2 (correlation ids), #56 (the frame): 4-full's and
  the owner's.
- The MCP's or the CLI's doors learning a `--world` flag: a surface
  change nobody asked for; the binaries still build one `Frontend`
  from the environment.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree for the build; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h3/`; commit on `main`,
never push, the `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`
line last; long commands in the background; `cargo build --workspace`
before any daemon and after any test run (C84). No task list: do not
call `update_plan`. A red run, if any, in a scratch worktree under
your scratch with its own target dir, stripping asserts on words the
fix introduces. Where a mechanism needs a seam (a reaper, a clock, a
look for the winner), name it in the commit; a `cfg(test)` hook is
acceptable where C137 needs one.

## Report

`TEMPLATE.md`, as text.
