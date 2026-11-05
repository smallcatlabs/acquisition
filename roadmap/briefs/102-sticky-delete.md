# 102-sticky-delete — the restore's failed delete fails the queue

Issue #102's last item, built at the owner's word of 2026-11-05 ("Go
ahead with both"), on his ruling that day (C6 amended in
`decisions/daemon.md` at `ddef4980`: a failed queue write is sticky,
the restore's delete too). Written by the orchestrating session against
the tree at `3efeb5fd`. Daemon internals: an Opus worker.

## Goal

At start, `restore_jobs` drops the previous lifetime's probe rows with
`let _ = db.lock().unwrap().delete(entry.info.id)`
(`crates/acquisition-daemon/src/daemon/queue.rs`, line 346), so a
failed delete is silent. The daemon then runs with a table that has
diverged from memory: an open row it does not hold, which
`ended_elsewhere` (line 1049) can only answer after the fact. C6 now
says that failure is sticky like every other queue write. When this
step is done, a failed delete at the restore trips the same sticky
failure as a failed `persist`: it is logged once, new submits are
refused as `queue_failed`, and the dispatcher stops picking.

## Read

- `AGENTS.md`.
- `decisions/daemon.md`: C6 as amended, C47.
- `REVIEW-CHECKLIST.md`, the daemon section.
- `crates/acquisition-daemon/src/daemon/queue.rs`:
  - the module doc's sticky-failure paragraph (lines 17–30);
  - `persist` (line 205), the one place that trips `queue_failure`
    today;
  - `queue_failed` (line 266);
  - `restore_jobs` (line 296) and its doc;
  - the probe drop (line 346);
  - `ended_elsewhere` (line 1049), whose doc names this case.
- `crates/acquisition-daemon/src/daemon/tests/queue.rs`:
  - `restore_drops_probes_and_finishes_a_parent_whose_children_are_done`
    (line 2076);
  - `a_failed_queue_write_refuses_new_jobs_and_stops_dispatch` (line
    2209);
  - `a_waiting_row_on_disk_and_not_in_memory_is_never_acted_on` (line
    2517), which stages the orphan row.
- How a test makes one table write fail while the rest work: the SQLite
  trigger in `daemon/tests/exit.rs` (line 1718) and in `login.rs`
  (line 303).

## Input

No fixed measurement. The failing input for the pin: a `daemon.db`
holding a probe row, with a `BEFORE DELETE` trigger on `jobs` that
raises an error, restored by a daemon.

The last push's CI on `3efeb5fd` succeeded on both runs: `Gate (Linux)`
42041413459 and `Gate (Windows)` 42041413264.

## Do

1. **One rule for a failed queue write.**
   - The sticky trip moves out of `persist` into one function that
     takes what failed and the error. `persist` and the restore's
     delete both call it.
   - The probe drop matches on the delete's result. On failure it trips
     the sticky failure, and the restore goes on: the row stays on
     disk, as `ended_elsewhere` already answers.
   - The log line names the probe's id and says a probe row could not
     be deleted.
   - The module doc and `restore_jobs`' doc say that a delete at the
     restore is a queue write like any other.
2. **Pin.** Add the case beside the restore's existing tests in
   `daemon/tests/queue.rs` (C153: no new file):
   - a probe row whose delete the trigger refuses;
   - the restore completes;
   - `queue_failed` is set, and the log carries the line once;
   - a submit after it is refused as `queue_failed`.

   Show the pin red against the old `let _ =` for one run, then restore
   the fix from a copy and never commit the old code. Say in the report
   what the red run printed.
3. If `ended_elsewhere`'s words or doc no longer fit (the case is now
   also sticky), adjust them in the same commit; their test is
   `a_waiting_row_on_disk_and_not_in_memory_is_never_acted_on`.

## Done when

- `restore_jobs` has no `let _ =` on a table write.
- One function trips the sticky queue failure for both `persist` and
  the restore's delete.
- The pin is in `daemon/tests/queue.rs`, seen red against the old code
  (said in the commit message).
- C6's *Pinned* names the new test. The worker edits only that clause,
  replacing "Unbuilt for the delete, #102" with the test's name.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Other `let _ =` in the daemon (`endpoint.rs`, `login.rs`,
  `refresh.rs`): they are not queue writes. If one is, raise it; do
  not fix it here.
- The completion residual the module doc already accepts.

## Rules

`TEMPLATE.md`, plus:

- Work in the main checkout on `main`. Commit locally, one commit; do
  not push.
- **This is the owner's machine, with his real store and keyring.
  Never start a real-provider daemon, and never run a bare `acq`.**
  Every `acq` and `acq-mcp` runs under the mock-session skill, with
  `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch.
- C153, C154.
- A forced failure, never a sleep (C137).
- Run the gate in the foreground, bare. Read its exit status alone.

## Report

`TEMPLATE.md`.
