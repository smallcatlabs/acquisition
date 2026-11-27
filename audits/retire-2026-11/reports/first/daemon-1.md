### C3 — The daemon spawns lazily and exits
- class: design — a user or agent at a frontend is promised a daemon that starts when work needs it and leaves on its own an hour after it goes quiet, plus the `daemon status`/`daemon stop` verbs and the words of the no-login refusal
- clauses:
  1. P · Y · held through clauses 2 and 5: crates/acquisition-cli/tests/provider_default.rs:245 (a `submit` with no daemon running leaves one socket: "a daemon started") and crates/acquisition-cli/tests/daemon_idle_exit.rs:49-51 (the real `acqd` exits 0 and logs "idle timeout; exiting")
  2. P · Y · crates/acquisition-cli/tests/provider_default.rs:234-249 (`submit --detach` starts a daemon); seat_faults.rs:3629-3638 (`profile` leaves a daemon running); readme_tour.rs:351-353 (running false, then true after a login)
  3. M · R · C10 ("an observation verb never spawns or replaces"); also held at crates/acquisition-cli/tests/daemon_observe.rs:40
  4. M · R · C13 (the MCP never spawns in real mode); crates/acquisition-mcp/tests/ggg_refusal.rs:50
  5. P · Y · crates/acquisition-daemon/src/daemon/tests/idle.rs:46 (a fresh daemon is not idle until the knob elapses; a connection, a waiting job or a submit holds it); idle.rs:181 (default 3600 s; a job's end or a cancel restarts the hour); readme_tour.rs:355-365 (status reports idle_shutdown_seconds 3600)
  6. P · Y · idle.rs:200-204 (a running job two hours on still holds the daemon)
  7. P · P · crates/acquisition-cli/tests/seat_faults.rs:3626 checks that `profile` with no login spawns and that the refusal says "not logged in" and "a daemon was started". Loose half: nothing checks that it says when the daemon will exit ("runs until idle", src/main.rs:73)
  8. P · Y · readme_tour.rs:351,373 and seat_faults.rs:3637-3640 (`daemon status --json` gives running/provider/idle; `daemon stop --json` gives stopped true)
- unheld: 7 simple · para · seat_faults.rs:3633, the existing `a_job_verb_refused_for_want_of_a_login_says_a_daemon_started`: add an assertion for "until idle"
- reason: Prop · crates/acquisition-daemon/src/daemon.rs:7-9 states the lifecycle (gpg-agent model, the hour), and daemon.rs:173 gives a reason for the hour. "No stray processes, no start-up ceremony" is not stated · moves to crates/acquisition-daemon/src/daemon/tests/idle.rs:41
- fate: NT · simple
- confidence: medium · did not read daemon/tests/exit.rs (the lifecycle table), client.rs's spawn path, or the MCP's spawn tests beyond their names

### C5 — The job queue is a priority queue
- class: design — a user of `acq set-priority` (and the protocol's `SetPriority`) is promised that a higher priority runs sooner, ties in submission order
- clauses:
  1. P · Y · crates/acquisition-daemon/src/daemon/tests/dispatch.rs:2233-2261, a row of `c158_the_wire_table_through_the_door` (:2905): two fetches on one key, priority 1 submitted before 9; the priority-9 job gets the server's first answer ("first")
- unheld: none
- reason: Prop · dispatch.rs:2222 states the property; the protocol doc crates/acquisition-protocol/src/job.rs:40 and the queue's daemon.rs:856 state the order. The *Why* ("a reorder is a field write") is stated nowhere · moves to dispatch.rs:2222
- fate: RM
- confidence: high · did not read how the Row runner matches `jobs` to ids (assumed submission order)

### C6 — The job queue persists: a
- class: mech — the daemon's queue and its results survive a restart, and a daemon without its queue does not run (the promise reaches users; see Noticed)
- clauses:
  1. P · Y · crates/acquisition-daemon/src/daemon/tests/queue.rs:1879 (across three lifetimes: finished results answered, running re-queued, priority kept, ids continue); crates/acquisition-cli/tests/persisted_queue.rs:109 (a previous daemon's result served to `result` and `result --wait`)
  2. M · Y · queue.rs:2284 (a job concluded after a failed write stays in memory, which answers for it); stated at daemon.rs:796 and the queue.rs:8 module doc
  3. M · Y · crates/acquisition-daemon/src/daemon/tests/evict.rs:98 (an evicted tree answers status/result/list/cancel/set-priority from the table as memory did); evict.rs:186 (a thousand concluded jobs, none in memory, all listed)
  4. M · Y · evict.rs:61 (a slow child keeps parent and siblings in memory until it concludes)
  5. P · Y · queue.rs:2256-2270 (a refused submit rolls `next_id` back)
  6. P · P · queue.rs:2518 asserts that `restore_jobs` errs with "could not read". Loose half: the daemon refusing to start (crates/acquisition-daemon/src/daemon/start.rs:565 bails) is held by reading only; the door's test helper mirrors start, not `acqd`'s start
  7. P · Y · queue.rs:2205 (an undeletable probe row trips the sticky failure, logged once; the next submit is refused `QueueFailed`); queue.rs:2256 (a failed queue dispatches nothing)
  8. M · R · C83 (one daemon per world)
- unheld: 6 simple · wrong · crates/acquisition-cli/tests/daemon_idle_exit.rs's harness (`harness::daemon_command`, `spawn_drained`): seed an unreadable `daemon.db`, then assert `acqd` exits non-zero with "will not run without its queue"
- reason: Prop · crates/acquisition-daemon/src/daemon/queue.rs:8-35 states the property and the mechanism whole. The *Why* (the one thing a restart lost; memory holds work, not lifetime) is not stated; evict.rs:180 has the memory half · moves to queue.rs:8
- fate: NT · simple
- confidence: medium · did not read crates/acquisition-store/unit/tests/jobs.rs:119, the world-lock test (world.rs:142), the restore tests at queue.rs:2062/2415, or queue.rs:2527 beyond its doc

### C165 — A job's record says when, and
- class: design — an agent is promised the job's times and a closed, branchable failure kind on the protocol's job record and result
- clauses:
  1. P · Y · held through clauses 2 and 3
  2. P · Y · crates/acquisition-daemon/src/daemon/tests/queue.rs:2975 (through the door on a parked clock: `submitted_at` set at submit, `started_at` when it runs, `finished_at` 10 s later; `Result` and the ledger row carry the same; no `_age` from the daemon); crates/acquisition-protocol/tests/wire.rs:537 (each time absent until set; an old record reads without them)
  3. P · Y · wire.rs:865 (`FailureKind::ALL` equals the type's variants, pinned by the `failure_kinds` fixture; an unknown kind does not parse); crates/acquisition-verbs/tests/documents.rs:1148 (way on by kind); frontends: crates/acquisition-cli/tests/seat_faults.rs:518 (kind `not_found`, argv offer, golden) and :490 (times in words); crates/acquisition-mcp/tests/tool_refusals.rs:344-411 (kind, the `submit_job` offer, `_at` and `_age` fields). The frontend half restates C53 and C143
  4. M · R · C53 ("JSON changes are additive")
- unheld: none
- reason: Prop · crates/acquisition-protocol/src/job.rs:112 (times) and job.rs:234 (closed kinds, "never from the failure's sentence") state the property in two places. The agent-facing *Why* is not stated whole · moves to crates/acquisition-protocol/tests/wire.rs:532 (with :861)
- fate: RM
- confidence: medium · did not read the store's jobs.rs:54, daemon/tests/responses.rs:167, or the dispatch rows' `.fails(kind)` runner; did not check `acq web`'s rendering of job times

### C166 — The job list is paged, and
- class: design — an agent reading the job list is promised pages that fit the frame, a state filter, `after`, newest first, and a `next` until the last page
- clauses:
  1. P · Y · crates/acquisition-daemon/src/daemon/tests/list.rs:336 (a page stops at its byte budget, keeping at least one row); list.rs:421 (a row's bound holds its widest row); the budget is `MAX_FRAME_BYTES / 2` (queue.rs:98)
  2. P · Y · list.rs:261-278 (bare `list` is the open jobs; `finished` reads the evicted from the table); crates/acquisition-protocol/tests/wire.rs:912 (default `open`; each word parses)
  3. P · Y · wire.rs:913-921 (default `LIST_LIMIT_DEFAULT`); list.rs:303-324 (0 and MAX+1 refused `BadRequest` with words)
  4. P · Y · list.rs:279-301 (`after` resumes; `all` paged as [8,7,6],[5,4,3],[2,1])
  5. P · Y · list.rs:294,301 (newest first; `next` present until the last page, absent then); wire.rs:940
  6. P · Y · crates/acquisition-daemon/src/daemon/tests/quote.rs:393 (`APPLY_MAX_ACTIONS` quoted; +1 refused, "nothing was quoted")
  7. P · Y · crates/acquisition-daemon/src/daemon/tests/queue.rs:1778 (a line one byte past the bound is replaced by `internal` "too large to send … the connection stays open"; the next frame is intact). It is a component test of `write_line_within`
- unheld: none
- reason: Prop · crates/acquisition-daemon/src/daemon/queue.rs:87 (a page always fits its frame), crates/acquisition-protocol/src/protocol.rs:532 and server.rs:1204 state the property in parts. The *Why* (applies open thousands of jobs; the queue persists; an oversize answer is lost) is stated nowhere · moves to crates/acquisition-daemon/src/daemon/tests/list.rs:208
- fate: RM
- confidence: high · did not read list.rs:375 (no row past a page is built) or door.rs:983; did not confirm every response path goes through `write_line_within`

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C3 | design | 6 | 2 | 5/1/0/0/2 | Prop | NT | 1/0/0 | 0/1 | medium |
| C5 | design | 1 | 0 | 1/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C6 | mech | 4 | 4 | 6/1/0/0/1 | Prop | NT | 1/0/0 | 1/0 | medium |
| C165 | design | 3 | 1 | 3/0/0/0/1 | Prop | RM | 0/0/0 | 0/0 | medium |
| C166 | design | 7 | 0 | 7/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |

## What I did not check
- The read-first items `tools/bothy.py`'s docstring and `git show -s 51f2e4f9`. I read only process.md's C170, C182, C192 and its Retired section.
- C3: daemon/tests/exit.rs (the lifecycle table and the c137 tests), the client's spawn code, the MCP's mock-mode spawn.
- C6: the store's unit jobs.rs:119, cli world.rs:142 (C83, left as R), restore tests queue.rs:2062, 2415 and 2527 beyond their docs.
- C165: store unit jobs.rs:54, daemon responses.rs:167, the dispatch Row runner's check of `.fails(kind)`, the verbs crate's daemon.rs:236/447, and whether `acq web` shows job times and way on, with or without a test.
- C166: list.rs:375, door.rs:983, and whether every response write goes through `write_line_within`.
- No test was run. Every Y assumes the file is compiled into a nextest binary the gate runs; I checked that for dispatch.rs's table (`#[test] c158_the_wire_table_through_the_door`) only.

## Noticed
- C5, step 1 and the text: "though one level is in use" and the Not-doing line at decisions/daemon.md:55 ("Priority levels beyond one: not until a second level is in use (C5 keeps the field)") do not match the code. Probes run at `PROBE_PRIORITY = u8::MAX` and logins at `LOGIN_PRIORITY = u8::MAX - 1` (crates/acquisition-daemon/src/daemon.rs:311,315), and `acq set-priority` takes any u8 (crates/acquisition-cli/src/cli.rs:456-458). Retiring C5 also leaves that Not-doing line citing a retired id.
- C3, text against code: the idle verdict has a clause the entry no longer states. Counted hits inside a policy window, or a pending wait, hold the daemon (server.rs:646-652; idle.rs:117-122 quotes C3 as "stays up to 300 s"). That clause protects the limiter's picture (a respawned daemon would assume the worst). It reads as a `ggg` promise held by C3's tests but stated in no entry. Retiring C3 would leave it with no entry.
- C3, *Pinned:*: the readme_tour agent recipe holds the hour default and `status`/`stop`, not the spawn policy or the refusal's words; those are held in provider_default.rs and seat_faults.rs.
- C6, step 5: the class rule does not decide cleanly. The promise (submitted work and its results survive a restart; a daemon that cannot read its queue does not run) is to users and agents, so it is neither agent-facing `design` nor promise-free `mech`. queue.rs:2527 also ties it to sends (a no-probe job "would replay blind"), a `ggg` reading the entry's text does not make. I chose `mech`; a second reader could fairly say `ggg`.
- C6, *Pinned:*: `a_broken_queue_is_fatal_to_restore` holds the restore's error, not the start's refusal that clause 6 states.
- C165, step 1: the additive clause restates C53 ("JSON changes are additive"), but the code's comment (crates/acquisition-protocol/src/job.rs:117) cites C10 for it.
