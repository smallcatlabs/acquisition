# r3 — The hour idle default, and finished trees out of memory

Readiness step 3 (`roadmap/README.md`, row r), after r2's design (it
does not wait for r2's build). Closes #80; builds C3 as the owner
ruled it on 2026-10-26 ("I approve the 1 hour idle default"). About a
day.

## Goal

A daemon idles out after an hour, not a minute, so an agent's session
and a person's sitting meet the daemon they started a while ago and the
README's "Your agent" section needs no `ACQ_IDLE_SHUTDOWN=3600`
recipe; and a daemon that lives an hour through many applies holds in
memory only the trees that are live, the mirror on disk answering for
the rest, so its memory is bounded by its work and not by its
lifetime. A long refresh is untouched by either: live jobs hold the
daemon however long the limiter makes them wait (`is_idle`), and disk
retention stays at 7 and 30 days.

## Read

- #80 (the evidence: finished jobs never leave `Shared.jobs`,
  `Entry.params` duplicated, the parent tally computed by scanning
  children); `roadmap/STATE-2026-10-26.md` R1.3.
- `decisions/daemon.md`: C3 (amended: an hour, `Unbuilt: issue #80`),
  C6 (memory the source of truth, the table its mirror: this step's
  amendment), C23 (a parent's tally), C45 (an observer reads the ledger
  on disk), C133 (the exit's drain), C7 (params are public).
- `crates/acquisition-daemon/src/daemon.rs`: `IDLE_SHUTDOWN` (around
  line 179), `Entry` (the two params fields), `Shared.jobs`;
  `daemon/server.rs`: `is_idle` (589), `has_live_jobs` (625), the
  `Request::Status`, `Request::Result` and the jobs-listing arms
  (around 260); `daemon/dispatch.rs`: `maybe_finish_parent` (347),
  `conclude`, `finish`; `daemon/queue.rs`: `persist`, `restore_jobs`
  (125, the prune at start), `describe`; `daemon/start.rs`:
  `idle_shutdown_from_env` (10), `retention_from_env` (78).
- `crates/acquisition-store/src/jobs.rs`: `JobDb`, `prune` (252), the
  read-only `Persisted` view and `observe_job` (what an observer reads
  today; the daemon's own fallback reads the same rows).
- Tests: `daemon/tests/idle.rs` (`idle_verdict_*`, on a manual clock),
  `parents.rs`, `queue.rs`, `persisted_queue.rs` (the `c45_` tests),
  the CLI's `apply_loop.rs` and `jobs` goldens; `acq jobs --all`'s help
  ("every finished job the daemon still holds") in
  `crates/acquisition-cli/src/main.rs`.
- `README.md`: the knob table's `ACQ_IDLE_SHUTDOWN` row (default 60)
  and "Your agent" (the recipe this step removes); `readme_tour.rs`
  runs the tour.

## Input

- The C6 amendment, ruled by the owner before Do 3 (Do 1).
- The schedules the tests stage: a parent with one slow child, an
  eviction attempted, the children still in memory; a tree concluded
  and persisted, evicted, `status` and `result` answered identically
  from the mirror; a thousand single jobs concluded, none in memory;
  a live job under a manual clock advanced past an hour, the daemon
  still up.

## Do

1. **The C6 amendment**, brought to the owner first: memory is the
   runtime source of truth for live trees; a tree whose parent has
   concluded and whose every row is persisted may leave memory, and
   the table answers for it. Under 800 bytes, the owner's words kept.
2. **The hour.** `IDLE_SHUTDOWN` to 3600 s; the knob's README row;
   "Your agent" loses the `ACQ_IDLE_SHUTDOWN=3600` lines and keeps
   `acq auth check` as the way a person starts a daemon for an agent
   (C13); every help text or tool description that says "a minute"
   regenerated (`CLI-REFERENCE.md`, `MCP-REFERENCE.md`); the
   no-daemon remedy's printed command (`Frontend::start_command`) loses
   its `ACQ_IDLE_SHUTDOWN=3600` once the default is the hour, with its
   test. The `idle_verdict_*` tests advance a manual clock past the
   knob, so they move with it; the harness's isolation list sets its
   own value and is unchanged.
3. **Eviction.** One function, `evict_tree`, called after a parent
   concludes (and after a childless job concludes) once its persist
   landed: the tree leaves `Shared.jobs`. `Status`, `Result` and the
   jobs listing for an id not in memory read the mirror (`JobDb`), the
   same rows `Persisted` reads, and answer in the same documents; the
   ten most recent finished jobs the listing shows come from the
   mirror; `--all` means every finished job within retention. Children
   stay until their parent concludes, since the tally scans them.
4. **One params copy.** `Entry.params` goes; `info.params` is the one.
5. **Pins**, each named for its schedule (Input), none on a clock.
6. **A measurement** in the message: the daemon's resident size after a
   thousand single mock jobs, before and after.

## Done when

- `acq daemon status` on a fresh mock daemon reports
  `idle_shutdown_seconds: 3600`; the knob row says 3600; "Your agent"
  names no `ACQ_IDLE_SHUTDOWN`; `readme_tour` green.
- The four pins green; `c45_` and `parents.rs` green unchanged.
- `acq jobs --all`'s help says what it now lists.
- The references regenerated, every changed line explained in the
  message (review shape 9).
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- C135 (#55) and r2's lock change.
- The parent scan's O(n²) and the outranked check, unless eviction
  removes them for free.
- The MCP spawning in real mode (a C13 question the owner has not
  taken up).
- Disk retention and `prune`'s schedule.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `r3`, every `ACQ_STORE_DIR` under it; commit on
`main`, never push; the `Co-Authored-By` line for your model last;
long commands in the background; `tools/gate.sh` if p0 has landed,
else the gate bare by each stage's own exit status, and `cargo build
--workspace` before any daemon and after any test run (C84).

## Report

`TEMPLATE.md`, as text.
