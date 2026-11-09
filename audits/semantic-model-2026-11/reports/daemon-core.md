# Daemon core: recovered semantic model

Paths are under `crates/acquisition-daemon/src/` unless given; `jobs.rs` is `crates/acquisition-store/src/jobs.rs`. "Verified" = read in the code at the cited lines; "suspected" = inferred.

## 1. State

| Value | Lives | Written by | Read by | Truth / mirror |
| --- | --- | --- | --- | --- |
| `Shared.jobs: HashMap<JobId, Entry>` (state, outcome, deferred, cancel_requested, times, retries, priority) | memory, `daemon.rs:859` | `submit_marked` q:942-988, `process` d:227-247, 263-323, `conclude` d:338, `finish` d:460, `cancel` q:997, `set_priority` q:1092, `restore_jobs` q:357-488, `evict_tree` d:503 (removes) | every request handler (server.rs:300-365), dispatcher, idle verdict, drain | **truth** for live trees (C6) |
| `jobs` table in `daemon.db` | SQLite, jobs.rs:40 | `Daemon::persist` q:210 only (plus `delete` q:366, `prune` q:317) | `restore_jobs`, `stored_job/stored_result` q:538-563, list pages q:722-760, 813; `Persisted` (frontends with no daemon, jobs.rs:707) | **mirror** while in memory; **truth** once evicted or after restart |
| `Shared.next_id` / `first_id` / `restored: Vec<JobId>` | memory `daemon.rs:860-867` | restore q:359, 482-484; submit q:958 | list pages q:695-719 | derived from `sqlite_sequence` (jobs.rs:495) |
| `Shared.evicted` (done/failed/cancelled counts) | memory q:102 | `evict_tree` d:527 | `counts` q:125 → `ListCounts` on the wire | derived; never reconciled with the table |
| `queue_failure: Mutex<Option<String>>` | memory `daemon.rs:1191` | `trip_queue_failure` q:233 (sticky) | submit, pick, evict, judge, refusals | truth (process-lifetime) |
| `active_jobs: HashMap<JobId, String>` | memory `daemon.rs:888` | `pick_runnable` d:123, `run_active` d:130 | pick (busy keys), `InFlight::of` s:1083 | truth |
| `Exit {begun, deadline}` | atomics `daemon.rs:222` | `Exit::arm` (first cause) | `pick_runnable`, `process` loop, `refuse_while_stopping`, driver | truth |
| `Connections` word (open/generation/claim) | atomic `daemon.rs:1017` | `handle_conn` admit/close; `Driver::begin` claim | idle verdict, `refuse_while_stopping`, status docs | truth |
| `last_activity` | atomic `daemon.rs:1115` | `handle_conn` s:107,250; `submit_marked` q:957; `land_restore` st:637 | idle verdict, `Driver::duration` | truth |
| `restore: watch<bool>` | `daemon.rs:1180` | `begin_restore`/`land_restore` | pick, `restored()`, `keyring_summary` | truth |
| `closed_with: HashMap<route,(Instant,FailureKind)>` | `daemon.rs:1198` | `record_closure` | `closure_kind` → a failed job's `kind` | derived beside the limiter's own closure |
| `Shared.errors` ring (50) | `daemon.rs:882` | `note_error` | `Dashboard.errors` | derived from log |
| `fault.json` marker | file, provider dir q:55 | panic hook st:275 | `restore_jobs` q:331 (then removed q:501) | truth across lifetimes |
| `Life {state, cause}` + `Driver` timers | memory, life.rs:122, s:792 | the driver loop s:708 | nothing on the wire; log lines | truth |
| knob `ACQ_IDLE_SHUTDOWN` | env | — | `life(knob)` once st:582; `DaemonStatus` per request s:442; start log st:557 | two readers of one env |
| mock policy windows | memory, mockggg.rs:172 `PolicyBook.live` | mock requests | mock answers | truth for the mock only; lost at every daemon restart |

## 2. Transitions

Job state machine (words persisted: `waiting|running|done|failed|cancelled`, jobs.rs:44; `requeued`, `faulted` are not states but paths):

| Transition | Trigger | Precondition (where checked) | Effect |
| --- | --- | --- | --- |
| ∅ → waiting | `Submit` (client) or `submit`/`submit_child` (daemon) | door q:1605-1633, realm, size, submitter, C164 (`admit_listed` k:347), `queue_failed()` none q:938, parent live & not cancelling q:949-955 — all but the last outside `shared` | id issued, row inserted (persist fail → id rolled back q:983), event, `work` notified |
| waiting → running | `process` after pacing | still waiting under `shared` d:232; persist succeeds d:237 (fail → reverted, start unset) | `started_at` first-set, event |
| running → waiting (requeue) | `Exec::RateLimited` d:257 | `retries < 2 && !cancel_requested` d:41 | retries+1, persist (result ignored d:271), event |
| running → waiting (parked) | `Exec::Parked` (halt) d:299 | `!cancel_requested` | persist ignored d:309, no retry spent |
| running → running+deferred | `conclude` when any child exists d:341 | state Running | `deferred = outcome` or `Cancelled` if cancel asked; persist ignored d:351 |
| deferred → terminal | `maybe_finish_parent` d:364 | `deferred.is_some()` and no non-terminal child | tally built, `deferred.take()` (not persisted), then `finish` |
| waiting/running → done/failed/cancelled | `finish` d:460 | none; `cancel_requested` overrides outcome d:466 | `finished_at`, persist, `evict_tree`, event, parent re-judged |
| waiting → cancelled; running → cancel_requested; deferred-running → cancelled | `cancel` q:997 over `tree(id)` | not terminal; a job not in memory refused via `ended_elsewhere` q:1077 | persist per job, events for terminal ones, parents re-judged |
| waiting priority change | `set_priority` q:1092 | Waiting | persist; **no event** |
| restore: running → waiting / failed(interrupted) / cancelled / held(deferred) | `restore_jobs` q:360-477 | branch order: probe → fault/closed-route → lost login → running cases → waiting as-is | see §4b for what is ignored |
| memory → table only | `evict_tree` d:503 | root terminal, every job of the tree terminal, `queue_failed` none | removed from memory, `evicted` counted |

Lifecycle (`Life::step`, life.rs:157): `Serving --Deadline(Idle)--> Judge --Busy--> Arm(Idle) | --Idle--> Begin --ExitArmed{won}--> Draining`; `StopRequested{won}` / `Signal{won}` → Draining (signal adds `StopAdmitting`); `Draining --Quiet--> Done+Leave`; `--Deadline(Drain)--> Cut+Leave`; every other pair → nothing. Verified against life.rs:158-183.

Classification:
- **Commute**: `cancel(a)` vs `cancel(b)` for unrelated jobs; `set_priority` vs any read; two `Status`/`List` reads; `finish(child)` vs `finish(sibling)` (the parent's tally is built once, under the lock, d:365-403; whoever is last finishes it).
- **Idempotent**: `set_priority(id, p)` twice; `restore_jobs` branch decisions (a lost login failed twice yields the same row); `Exit::arm` (swap, d:268); `Connections::claim`; `maybe_finish_parent` (second call finds `deferred` taken, d:370). `cancel` twice is *not* idempotent on the wire (second answers `wrong_state`, q:1012) but is in effect.
- **Order-dependent**: `cancel(parent)` then `submit_child` → child refused (q:950); `submit_child` then `cancel(parent)` → child swept (q:1018). `cancel(child)` before `finish(child)` takes `shared` → Cancelled; after → `already done` and the outcome stands (d:466 vs q:1012). `restore then submit` is forced by `run_with_log` (restore_jobs st:566 precedes `serve` st:586); `submit then restore` cannot occur for the queue, but *can* for the session restore: a sleep/mock-fetch submitted during `begin_restore` is admitted and parked until `restore_landed` (d:75). `evict` vs `List`: memory read then table read, so a tree evicted between is listed once from the table (q:785-808); the reverse order would lose it.

## 3. Dependencies

| Effect | Must be true | Checked where | TOCTOU |
| --- | --- | --- | --- |
| HTTP send (GET/HEAD) | job Running & persisted; not halted; not exiting; route known/probed; token valid; not cancelled | d:148-247 (same task, `shared` taken and released per step), choke point re-checks under permit (k:941 `wanted`) | the exit/halt checks at d:189-199 are outside the lock that admits the send; the choke point's own admission closes the gap for halts, not for the exit (a send admitted an instant before `begun` lands anyway, by design C133) |
| row write (`persist`) | caller holds `shared` | convention only: q:205 "Called with the Shared lock held" | `restore_jobs` q:366 (`delete`) and q:407/421/473 are under `shared`; `conclude` d:351 too; **no type enforces it** |
| probe delete at restore | — | q:366 | — |
| socket reply | serialized line ≤ frame bound | `write_line_within` s:1225-1236 | — |
| `Submitted{id}` reply | exit not begun | `refuse_while_stopping` twice (s:268, s:533/535) but `submit_marked` q:938 checks only `queue_failed` | **yes**: an exit that begins between s:535 and q:987 admits and persists a job the dispatcher will never pick this lifetime (d:69); the client is told `Submitted`, the job waits for the next daemon. Verified window; suspected never observed. |
| submit refused as `queue_failed` | flag set | q:938 outside `shared`; the trip happens inside (`persist`) | **yes**: two submits racing a first failed write; the second may be inserted after the flag trips (persist succeeds, flag already set). It then waits unpicked. Small window, verified structure. |
| process exit (`leave`) | drain quiet or bound passed | driver s:729-733 | the hard bound thread (d:274-285) runs on `std::time::Instant`, independent of the clock the drain uses — deliberate, documented d:213-217 |
| fault marker write | panic inside a job's task | `RUNNING_JOB.try_with` st:273 | a panic in hyper's connection task writes none (d:34-37, documented) |
| idle exit | open==0, no live jobs, no landing, no pending login, timed, limiter cold | `judge` s:660 on a blocking thread; `claim` s:856 on the driver | `claim` closes the connection race; a *submit on no connection* is impossible, so the other clauses need no claim (s:845-852) — holds as long as every work source is a connection or a counted landing |
| `Dashboard`/`List` counts | — | `counts` under one `shared` take q:718 | the page's rows are chosen later (q:770) from a second lock take (q:791): counts and rows describe two instants |

## 4. Findings

### 4a. Nondeterminism in awaits, retries, event order

1. **Event order vs table order** (verified). `emit` runs after `shared` is released in `finish` d:483, `cancel` q:1050, `submit_marked` q:992. Two jobs finishing on two tasks can persist A,B and emit B,A. A `jobs --watch` subscriber sees B's `done` before A's. C85 makes events hints, so no user-visible wrongness; a test asserting event order would be flaky.
2. **`cancel` emits children in HashMap order** (verified). `Shared::tree` q:898-912 builds `children` from `jobs.values()`; emits (q:1050) and persists (q:1035, `updated_at` equal to the second) follow that order. Two runs of the same cancel give two event orders.
3. **Pacing loop re-sorts the queue per slice** (verified). d:210-214 calls `queue_order()` (sort of every waiting job) once per ≤1 s slice per active key; with 50 000 children this is O(n log n) per second per key under `shared`. Not a correctness race; a scheduling cost that scales with the queue.
4. **Idle `timed` clause ignores job completion** (verified). `mark_activity` is called only at connection admit/close, submit and the restore's landing (grep: s:107, s:250, q:957, st:637). A job that runs for 70 min after its submitter disconnected ends with `timed` already true; the daemon leaves at the next re-judge (≤ `IDLE_REJUDGE`, 5 s). C3's "exits after an hour with no connections and no queued work" is read by the code as "an hour after the last connection/submit, once no work remains". Whether that is the intended reading is the owner's; the test `idle_verdict_a_live_job_holds_the_daemon_past_the_hour` pins only the hold, not the hour after it.
5. **Driver `select!` is biased** (verified, s:987-994): a cause beats a judged verdict beats a look beats the drain deadline beats the idle deadline. A `Busy` verdict landing beside a stop is dropped by the machine (`_ => vec![]`), correct; an `Idle` verdict landing one tick after a connection admitted is caught by `claim` (s:856). No observable race found.
6. **`Judged` after exit begun** (verified): `Turn::Judged` with `idle: true` in `Draining` → `Event::Idle` → `_ => vec![]`. Harmless.
7. **Two wall faces** (verified). Job rows are stamped from `choke.wall()` (`unix_now`, daemon.rs:1538); the facts a job records and C164's age are stamped from `acquisition_store::now()` (k:280, k:400, login.rs:329), which reads `ACQ_NOW` or `SystemTime::now`. In production both are the system wall; under a manual test clock they diverge, so a scenario's `submitted_at` and the store's `fetched_at` live on two clocks. Not a user-visible defect; a trap for C158 scenarios that compare the two.

### 4b. Invariants enforced by hope

1. **"`persist` is called with `shared` held"** (q:205-207) — a doc sentence; `persist(&self, &mut Entry)` takes no guard. Every current caller complies (verified: q:366, 407, 421, 473, 982, 1035, 1110; d:237, 271, 309, 351, 477, 544). Nothing stops a new caller. A `&mut Shared`-taking signature or a `Held`-style token (as `held.rs` does for credentials) would make it a type.
2. **Ignored `persist` results** (verified: d:271 requeue, d:309 parked, d:351 deferred, d:477 finish, d:544 start_and_finish, q:421 lost login, q:473 restore re-queue). Each trips the sticky flag, so divergence stops growing, but memory and disk disagree for that job: e.g. memory `waiting retries=1`, disk `running retries=0` → the next lifetime replays with the retry uncounted. The C6 doc names only "completions" as the accepted residual (q:23-27); the requeue, park and deferred cases are the same residual, unnamed and unpinned (`a_job_concluded_after_a_failed_queue_write_stays_in_memory` pins the completion case only).
3. **An unknown state word drops the row silently** (verified). `Entry::from_row` q:385-386 and `listed_info` q:432 return `None` on a word outside the five; `restore_jobs` q:335 `filter_map`s them away with no log line; `stored_job` answers `Ok(None)` → `no job N` (s:306). A `daemon.db` written by a build with a sixth word (the schema gate jobs.rs:247 checks `user_version`, not vocabulary) loses jobs without a word. C47 wants a structured error; nothing pins this.
4. **An outcome that no longer deserializes becomes "no job"** (verified). `stored_result` q:540-542: `outcome(row.outcome)` is `from_value(..).ok()`; a terminal row whose JSON the current `Outcome` refuses yields `Ok(None)` → `no_job`, while `Status` for the same id (via `listed_info`) answers the row. Two requests disagree about one id.
5. **"Probes outrank everything"** (`PROBE_PRIORITY`, daemon.rs:307) — a client's `priority` is an unbounded `u8` admitted as sent (s:263→q:876; no check in `admit_client`); 255 ties a probe and wins by lower id in `queue_order` (q:922). Only ordering within one serial key is affected (probes have their own key, r:118), so the practical effect is nil, but the comment is held by nothing.
6. **C5, C7, C49** are "Pinned: nothing yet" and remain so: C5 (priority field) is held only by `queue_order`'s sort key; C7 (params public, no secret) by the hook's `RUNNING_JOB`-free `write_log_line` doc (daemon.rs:1494-1523) and review; C49 by nothing: `tools/arch-check.sh` names none of C5/C7/C49 and no `account ==`/`!=` comparison exists in the daemon's production sources (verified by grep), so C49 holds today by absence, not by a check.
7. **Dashboard is unpaged** (verified, s:459 `job_list()`). C166's premise ("an unpaged list reaches the bound with no request to blame") still describes `Dashboard`: `job_list` reads every row of this lifetime and the restore's; past `MAX_FRAME_BYTES` the answer is `TOO_LARGE` (s:1231), so `acq dash`/web fails with `internal` for a queue that `List` pages fine. `c166_an_answer_past_the_frame_bound_is_answered_internal` pins the replacement, not the dashboard's reach.
8. **Lock order** (daemon.rs:1087-1097) is a comment; `clippy::await_holding_lock` holds the no-await-under-guard half; the order itself is unchecked. `persist`'s `match self.jobs_db.lock()…` q:212-216 holds `jobs_db` through `trip_queue_failure` (→ `queue_failure` → `log`), which is the documented order, by luck of the match-scrutinee rule the same doc warns about.
9. **Mock vs real** (verified): the mock's windows live in `PolicyBook.live` (mockggg.rs:172), per process; a daemon restart resets them. The restore's premise "a probe reads the previous lifetime's hits before anything sends" (q:454-457) is vacuous on the mock, so the `requeued` branch (q:469-472) is never exercised against a limiter that remembers. The mock's `/profile` HEAD 403 and `/account/leagues` counted HEAD match N38/N39 (mockggg.rs:875-890). The mock answers 404 for unlisted tabs; C164 therefore blocks every 404 the mock could produce, so the `not_found` kind reaches a user only live.

### 4c. One fact in two places

1. **State word**: `JobState`'s serde `snake_case` + `Display` (protocol job.rs:9-37) and `job_state()` (daemon.rs:415) — two interpreters of one vocabulary. Authoritative: the protocol's. Drift: a renamed variant compiles in both and the table reads `None` (→ 4b.3).
2. **Finished counts**: `Shared.evicted` (memory counter) vs the table's finished rows. Authoritative: the table. They agree only because eviction is the one exit from memory and `evict_tree` refuses after a queue failure (d:504). A row pruned by retention at the *next* start is not subtracted — but `evicted` restarts at zero then, so no drift within a lifetime.
3. **Idle knob**: read at `life()` start (st:582) and again per `DaemonStatus` (s:442). Same env, same parser; drift only if the parser changes between (none).
4. **Route closure kind**: the limiter's `Degraded{until}` and `closed_with[route] == (until, kind)` (daemon.rs:1803-1820). Authoritative: the limiter. A closure re-degraded (k:906 after k:789) with a new `until` and no `record_closure` in between falls back to `Upstream` — by design, but a second writer.
5. **`deferred` taken in memory before `finish` persists** (d:403 → d:477): a crash between leaves disk with `deferred` set; restore finishes the parent (q:433-436) — converges, so acceptable.
6. **Restore's account check for closed routes** vs dispatch's: `keyed_route_for` is run twice per restored row (q:383, q:424); same function, no drift.

### 4d. Work that never reaches the screen

- `Entry::row`'s `submitted_at.unwrap_or(now)` (daemon.rs:376): `submitted_at` is always `Some` (q:972, q:401) — dead fallback.
- `Leave`'s `Option<InFlight>` (s:779): `run` discards it (`|daemon, _cut|`, st:583); only the log line in `Effect::Cut` (s:927) carries the cut. The test recorder is its sole reader.
- `Life::state()`/`cause()` are `allow(dead_code)` outside tests (life.rs:139-148).
- `set_priority` persists and notifies the dispatcher but emits no `event` (q:1092-1118, s:342-348); a watcher never learns of it. `cancel` of a Running job emits nothing either (q:1037 guards on `Cancelled`).
- `Exec::Parked(String)` and `Refusal.halted/cancelled/failure/unmatched` never reach the wire (documented, daemon.rs:501-518).
- `kind_needs_account`'s `"fetch" => is_real()` (daemon.rs:1893): `fetch` is refused in real mode at the door (q:1621), so the `true` branch is reachable only for a daemon-internal `fetch`, which nothing submits.
- `execute_inner`'s `other =>` arm (k:916) and `apply`'s second `validate_apply` (k:647): reachable only from a row another build wrote.
- `Shared::list` + `job_list` (q:580) exist for `Dashboard` alone.
- `InFlight.held` is logged in `describe` (s:1148) but `Cut` is the only reader; a healthy exit never prints it.
- `restore_jobs`'s six counters (q:352) reach one log line.
- `blocking_in_flight` (daemon.rs:1121): read by no daemon code (documented).

## 5. Refactors

**PROVABLY EQUIVALENT**
- Replace `job_state()` (daemon.rs:415) with `serde_json::from_value::<JobState>(Value::String(word))` or a `FromStr` derived beside `Display`: the protocol's `wire.rs` fixtures and `JobState`'s serde attribute establish the five words; a round-trip unit test (`to_string` → parse) is a pure-function property.
- Give `persist` a `&mut Shared` (or a `MutexGuard`) parameter: a type check establishes every caller holds the lock; behaviour unchanged since every caller already does.
- Compute `Shared::counts` from a per-state counter maintained in `evict_tree`/transitions instead of a full scan: equivalence is a fold over the same `jobs` map; `c166_a_page_is_one_state_newest_first_and_the_counts_are_the_whole_lists` is the table.
- Replace `Children`'s per-finish full scan (`maybe_finish_parent` d:373-377, `conclude` d:341) by a `children_open: usize` on the parent `Entry`: the existing `parents.rs` and `evict.rs` tests establish the tallies; equivalence rests on "every child has exactly one parent id and finishes once", which `finish` already guarantees by state check.

**TRUST ME** (smallest breaking input)
- *Page the Dashboard's jobs (or make `Dashboard.jobs` the first `List` page)*: breaks a user who reads a dashboard with more jobs than one page — input: 1 001 finished jobs this lifetime, `acq dash` shows 1 000 (new) vs `TOO_LARGE` only above the frame (old).
- *Stamp activity at job finish* (fix 4a.4): input: one connection submits a 2 h `sleep` and disconnects; old: daemon leaves ≤ 5 s after the sleep ends; new: leaves 1 h after.
- *Fail restore on an unknown state word instead of dropping the row*: input: a `daemon.db` row with `state='paused'`; old: silent restore, `no job`; new: the daemon refuses to start (C6's "cannot read the table").
- *Check `exit.begun()` inside `submit_marked` under `shared`* (close 3's TOCTOU): input: `daemon stop` racing a submit between s:535 and q:987; old: `Submitted{id}` with the job parked for the next daemon; new: `refused: the daemon is stopping`.
- *Emit an event from `set_priority`*: input: `acq jobs --watch` beside `acq priority <id> 9`; old: no frame; new: one `event` frame with the new priority — a watch golden with an exact frame count changes.
- *Collapse `has_live_jobs(halted)`'s `Running => deferred.is_none() || !halted`* to `Running => !halted || deferred.is_none()` is identical; collapsing to `Running => true` differs on input: halted daemon, one deferred parent whose children all wait → old: idle (leaves); new: held forever (never idles under a halt).
- *"Not doing"* for this area (`decisions/daemon.md:55,66,67`): a second priority level, the admission loop as a machine, a sticky prune — each is a judgment the owner has already declined; the prune one is the only one where the code and the registry agree exactly (q:317-320).
