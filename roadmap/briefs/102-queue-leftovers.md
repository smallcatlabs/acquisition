# 102-queue-leftovers — the job list, an orphan row, the tree walk, the login wait's break

Issue #102, built at the owner's word of 2026-11-04 (the night run),
`trigger:live` waived. Written by the orchestrating session against
the tree at `6f2e3337` and read again at #44's close (`38aeb107`). Daemon internals:
an Opus worker.

## Goal

Four leftovers of r3, the eviction (`briefs/r3-idle-and-eviction.md`).
None sends to GGG. When this step is done the job list no longer holds
the queue table's lock across its whole range read, so the
dispatcher's persists do not wait on a lifetime's worth of rows; a job
whose row is open on disk but not in memory is answered as that, not
as a job that does not exist; a tree's walk in memory is linear in the
jobs it scans; and the login's wait on a broken queue is pinned through
a seam that stages the break where it happens, or parked with the
reason no seam stands without a production hook.

## Read

- `AGENTS.md`; invariant 1 (the choke point: untouched here).
- `decisions/daemon.md`: C6 (the queue persists; memory the source of
  truth for live trees, the table their mirror; a failed queue write
  is sticky), C50 (a login lands only with its profile's uuid), C157,
  C158 (a daemon scenario crosses the frontend boundary only; a test
  needing another boundary says which and stays a component test),
  C165 (a job's times).
- `REVIEW-CHECKLIST.md`, the daemon section.
- `crates/acquisition-daemon/src/daemon/queue.rs`: `job_list` (about
  line 414), `ended_elsewhere` (about line 695) and its three callers
  (`cancel`, `set_priority`, and the one after), `stored_job`.
- `crates/acquisition-daemon/src/daemon/dispatch.rs`: `evict_tree`
  (about line 502), whose `root_of` is computed for every job in
  memory at every conclusion; `queue.rs`'s `cancel`, whose descendant
  loop scans every job per target.
- The store's jobs table reader (`listed_finished_from`,
  `listed_finished`; grep `crates/acquisition-store/src/jobs.rs`).
- `crates/acquisition-daemon/src/daemon/login.rs`: `wait_job_terminal`
  (about line 440); `daemon/tests/queue.rs`'s
  `the_login_wait_answers_a_broken_queue_after_its_job_concluded`
  (about line 2281) and `break_queue`.
- `daemon/tests/door.rs`: how a door scenario holds a transport
  request and parks (C158), for item 4.

## Input

No fixed measurement. The last push's CI: read
`gh run list --branch main --workflow "Gate (Linux)" --limit 1` and
the same for `Gate (Windows)` at the start and name them in the
report; a red run is reported, not fixed here.

## Do

1. **The list reads the table in bounded pieces.** `job_list` reads
   memory first (as now), then the finished rows in pages by id (a
   page size named as a constant, its reason in its doc), taking and
   releasing the table's lock per page, so a persist waits at most one
   page. The answer is unchanged: same rows, same order, a job in both
   memory and table listed from memory. Say in the doc why paging
   after the memory read loses nothing (a job evicted during the pages
   was in memory at the first read). Pin: the list over more rows than
   one page answers every row once, in order (an existing list test
   gains the case, C153). No count bound on the list (#56's, the
   owner's: a request field is a protocol change).
2. **An open row not in memory is answered as what it is.** Today
   `ended_elsewhere` answers `no_job` for a row still open on disk (a
   probe whose delete failed at the restore). Answer it with an
   existing kind that names it (the session's call: `queue_failed`
   fits, since the row is there because a queue write failed), in
   words that say the job is open on disk, not held by this daemon,
   and that the next start drops or replays it. No new `ErrorKind`
   (the protocol is the owner's). Pin: the existing test that stages
   an orphan row, or a case beside the restore's tests.
3. **The tree walk is linear.** `evict_tree` and `cancel`'s descendant
   collection build one parent-to-children map per call (or walk an
   index if one already exists) and collect a tree from its root,
   O(jobs in memory) per call, not O(jobs × depth) or O(jobs ×
   targets). Behaviour unchanged; the existing eviction and cancel
   tests hold it. No new long-lived index in `Shared` unless the
   per-call map cannot serve (say why).
4. **The login wait's break through a seam.** The pin today calls
   `wait_job_terminal` directly. Find whether a door scenario can stage
   the break between the profile job's eviction and the wait's table
   read with what the door already offers (a held transport, the park,
   the queue file the scenario owns). If it can, the pin goes through
   the login request and the component test retires with its
   `retired X; held by Y` line. If it cannot without a new production
   hook, keep the component test, add no hook, and say in the report
   exactly which interleaving no existing seam reaches; the
   orchestrator parks it.

## Done when

- `job_list` takes the table's lock per page, never across the whole
  read; the multi-page case is pinned.
- An open row on disk not in memory is refused with an existing kind
  in words naming it; pinned.
- `evict_tree` and `cancel` collect a tree with one map per call; the
  existing tests pass unchanged.
- Item 4 pinned through the login request, or the report names the
  interleaving and why no seam reaches it.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- A count bound or paging on the wire (#56, the owner's).
- The limiter, the gate, the choke point, any send.
- `read_frame` and `write_line` (#56's internal items, the next step).

## Rules

`TEMPLATE.md`. Main checkout on `main`, commit locally, no push; every
`acq`/`acq-mcp` in a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR`
under your scratch), never a bare `acq` or a real-provider daemon;
C153 and C154; one commit per item, each naming this brief. A wait in
a daemon test is pinned by forcing it (C137, C160), never by a sleep.

## Report

`TEMPLATE.md`.
