### C152 — The rails status names the configured
- class: design — a frontend or an agent reading the daemon's status: the journal's path is a field of its own, apart from the words of whether it was written
- clauses:
  1. P · Y · crates/acquisition-daemon/src/rails.rs:2108-2153: off gives `(None, None)`; an opened and an unopenable journal both report `journal_path` as the configured path, apart from `journal`
  2. P · Y · rails.rs:2144: an unopenable journal still carries `journal_path`; rails.rs:2120-2123: an opened one does too
  3. P · Y · rails.rs:2110 (absent when off), :2120 (the path when opened), :2145-2148 (`NOT WRITTEN — could not be opened: …` when not); crates/acquisition-protocol/tests/wire.rs:630 checks that a status without the field still reads
  4. P · Y · rails.rs:2150-2153 asserts the words do not contain the directory; crates/acquisition-verbs/tests/daemon.rs:1279-1282 asserts the same through the diagnosis
  5. M · Y · verbs/tests/daemon.rs:1229-1302: the diagnosis redacts the directory from the field while the shell's knob is unset, so reading the words alone would leak nothing and redact nothing; crates/acquisition-cli/tests/diagnose.rs:176 checks the CLI's text (`rails_text`, crates/acquisition-cli/src/daemon_cmd.rs:137, reads `journal_path`)
- unheld: none
- reason: Y · crates/acquisition-verbs/tests/daemon.rs:1217 (the property, and the reason: a path held only in the failure's words is redacted only when this shell's knob names it) · moves to crates/acquisition-daemon/src/rails.rs:2101
- fate: RN
- confidence: high · I did not read crates/acquisition-verbs/src/diagnose.rs past lines 205-215, nor the other frontends' rendering of the rails status beyond `rails_text`

### C86 — The handshake has a deadline: a
- class: design — a user or an agent at any door: a daemon that does not answer `hello` within 5 s is reported as unresponsive, with the hand that stops it, and is never spawned or replaced over
- clauses:
  1. P · Y · crates/acquisition-client/tests/contract.rs:362-404 (in a `c137_` test): `HELLO_WITHIN == 5s`; at zero, observe, connect and stop each return the unresponsive verdict, never `Absent`; crates/acquisition-client/src/door.rs:685-707: the expiry rows for use, observe and stop
  2. P · Y · door.rs:687-706: each expiry row's effects are exactly `Report(Unresponsive)`, with no spawn and no stop, and door.rs:1175 asserts them equal; contract.rs:406-414: the same peer is still there afterwards; contract.rs:303-326 and :380: the report names the lock's holder pid and the platform's kill line
- unheld: none
- reason: Prop · crates/acquisition-client/src/client.rs:62 states the property; client.rs:320 gives half the *Why:* (a healthy daemon answers in milliseconds); "an unbounded read hung every door" is stated nowhere · moves to crates/acquisition-client/src/client.rs:62
- fate: RM
- confidence: high · I did not read the driver's arming code at client.rs:1844

### C147 — A door and a ledger read
- class: mech — another crate or a long-lived frontend: each door acts on the world it is given, and a spawned child is reaped and reported. Medium; it could also be `edge`
- clauses:
  1. P · Y · crates/acquisition-verbs/tests/daemon.rs:670-743: a frontend over world B, with the environment naming A, spawns, writes and reads facts, intent, the quote, the daemon report and the ledger in B; A is never created and has no daemon
  2. P · Y · the same test: a door that read the environment after construction would act in A, and the asserts at :736-743 fail
  3. M · Y · stated at crates/acquisition-client/src/client.rs:42-50; the world-taking forms are used by the test above (`Client::stop_any_at`, `World::observe_at`)
  4. M · Y · stated at crates/acquisition-verbs/src/frontend.rs:12-18; the test builds a `Frontend` by hand, with no environment reading
  5. P · Y · crates/acquisition-client/tests/contract.rs:874-911: a lost race opens on the winner, `!spawned`; :925-944: no winner gives `SpawnFailed` naming the holder; :998-1037: a failed stop on the replace path gives `NotReplaced::StopFailed`; :1052-1081: the spawned pid leaves the process table, not a zombie (Unix only)
- unheld: none
- reason: Y · crates/acquisition-verbs/src/frontend.rs:12 (the property, and the reason: a long-lived server holds more than one world); the second half of the *Why:* (it outlives the daemons it spawns) is at crates/acquisition-client/src/client.rs:2111 · moves to crates/acquisition-verbs/tests/daemon.rs:664
- fate: RN
- confidence: medium · the reaping test and the race test are `#[cfg(unix)]`, so reaping on Windows is checked by nothing I read; I did not read the client.rs unit tests at :2505-2612

### C137 — A deadline's contract is its behaviour
- class: check — later sessions writing tests: a deadline is pinned by forcing its expiry, and its knob is widened for runners
- clauses:
  1. P · U · a rule on future tests. Every deadline the entry names has a test that forces its expiry, but a new deadline without one fails nothing
  2. P · N · I looked at tools/arch-check.sh check 14 (:981-1009): it refuses reading the wall clock (`store::now`, `SystemTime::now`) in process tests, not an assertion on elapsed time. Nothing else I found refuses one. The tests do assert 120 s ceilings (`HANG`, crates/acquisition-client/tests/harness/mod.rs:361; `Instant::now() + Duration::from_secs(…)` 27 times under crates/*/tests)
  3. M · Y · crates/acquisition-client/tests/harness/mod.rs:183-188: the isolation list sets `ACQ_HELLO_WITHIN`, `ACQ_STARTUP_WITHIN`, `ACQ_ANNOTATION_WAIT` and `ACQ_STORE_WAIT` to 60 and `ACQ_IDLE_SHUTDOWN` to 30; stated at :156-161
  4. P · Y · contract.rs:362-404 (hello at zero is unresponsive against a peer that answers); contract.rs:957-972 (startup at zero fails the spawn); crates/acquisition-store/unit/tests/lib.rs:2546-2568 (store wait at zero, the enforced wait read back, refused as busy); crates/acquisition-daemon/src/daemon/tests/exit.rs:1188-1204 (idle at zero leaves)
  5. M · R · C157 (a lifecycle that waits is a value)
- unheld: 2 complex · para · it needs a new check over test code, and the clause's meaning is unclear: the `HANG` ceilings are wall-clock bounds a slow machine can miss, by design
- reason: Y · crates/acquisition-client/tests/harness/mod.rs:156-161 (the deadlines are widened so a starved runner cannot fail a healthy path; a test that pins one sets its own) · moves to crates/acquisition-client/tests/harness/mod.rs:156
- fate: NT · complex
- confidence: medium · I did not read the annotations.rs and jobs.rs `c137_` tests, the drain bound test (exit.rs:1348) or crates/acquisition-cli/tests/daemon_idle_exit.rs

### C157 — A lifecycle that waits is a
- class: check — later sessions: a waiting lifecycle is written as a pure machine and pinned by event scripts. Medium; it could also be `mech`
- clauses:
  1. M · Y · stated at crates/acquisition-client/src/door.rs:1-12 and crates/acquisition-daemon/src/daemon/life.rs:1-11; door.rs:1171-1181 and exit.rs:1149-1160 drive each machine's step with no clock (purity itself is enforced by nothing)
  2. M · Y · stated at door.rs:4-5 and life.rs:4-5; the tables feed deadline events as rows
  3. M · P · stated at door.rs:5-10 and life.rs:5-11; the drivers are exercised by the real-wiring tests, but nothing pins that the clock lives only in the driver
  4. P · Y · door.rs:1171 (each row's effects equal its expectation) and exit.rs:1149 (effects, state and cause), both synchronous with no clock, for the two machines that exist
  5. P · P · driver tests exist (contract.rs:874-1082; exit.rs:1174 onward, under `HANG`), but nothing pairs each effect kind with one test, and nothing refuses a duration assertion
  6. P · U · a rule on future cross-process tests; no check could fail on it
  7. M · U · the criterion, stated at door.rs:14-27 and life.rs:13-29
  8. M · U · the same, stated at the same lines
- unheld: 3 cut · para · already the module doc of both machines, and nobody outside the drivers relies on it; 5 complex · para · it needs a map of effect kinds to driver tests, or a new check
- reason: Prop · crates/acquisition-client/src/door.rs:1 states the property; the *Why:* (a deadline tested in real time is a bet the Windows gate lost; a row cannot be late) is stated nowhere I read · moves to crates/acquisition-client/src/door.rs:1
- fate: NT · complex
- confidence: medium · I did not enumerate the `Effect` variants against the driver tests, nor read the driver loops (crates/acquisition-client/src/client.rs:1272, crates/acquisition-daemon/src/daemon/server.rs:691)

### C158 — A daemon scenario crosses the daemon's
- class: check — later sessions writing daemon tests: a scenario enters through the protocol, and its time moves only by its own hand or the runtime's park
- clauses:
  1. P · P · the time half: crates/acquisition-daemon/src/daemon/tests/dispatch.rs:3236-3290 asserts that a counted blocking task holds the parking clock until its end is observed, and tools/arch-check.sh check 10 (:695-741) refuses any `spawn_blocking` outside `Daemon::blocking`. The boundary half is loose: nothing refuses a scenario that reaches an internal
  2. P · P · dispatch.rs:2905 (the wire table runs through the door, reading the job events, the journal and its clock), but nothing refuses a scenario that reads daemon state
  3. P · P · moving the runtime's clock is held as in 1; nothing refuses "a poll" in a scenario
  4. P · P · crates/acquisition-daemon/src/daemon/tests/harness.rs:324-355: `test_daemon` and its siblings take a `Boundary` first. The census tools/daemon-tests-census.py says of itself that it is report-only and no gate stage, and calling `Daemon::builder` directly is refused by nothing
- unheld: 1 complex · para · the census would have to become a gate stage or a check (gate stages are the owner's); 2 complex · para · the same; 3 complex · para · what "a poll" means is unclear to a check; 4 complex · para · the same as 1
- reason: Prop · tools/daemon-tests-census.py:2-4 and crates/acquisition-daemon/src/daemon/tests/harness.rs:324 state the property; tools/arch-check.sh:696-703 gives the second half of the *Why:* (the runtime's park is the exact signal); "a test that drives internals dies with a rewrite" is stated nowhere I read · moves to crates/acquisition-daemon/src/daemon/tests/harness.rs:324
- fate: NT · complex
- confidence: medium · I did not read crates/acquisition-daemon/src/daemon/tests/door.rs's module doc or its park hook, nor crates/acquisition-daemon/src/daemon/tests/refresh.rs:470

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C152 | design | 4 | 1 | 5/0/0/0/0 | Y | RN | 0/0/0 | 0/0 | high |
| C86 | design | 2 | 0 | 2/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C147 | mech | 3 | 2 | 5/0/0/0/0 | Y | RN | 0/0/0 | 0/0 | medium |
| C137 | check | 3 | 2 | 2/0/1/1/1 | Y | NT | 0/1/0 | 0/1 | medium |
| C157 | check | 3 | 5 | 3/2/0/3/0 | Prop | NT | 0/1/1 | 0/1 | medium |
| C158 | check | 4 | 0 | 0/4/0/0/0 | Prop | NT | 0/4/0 | 0/4 | medium |

## What I did not check
- On Windows, C147's reaping and spawn race (those tests are Unix only).
- Whether every `Effect` kind of the door and of the lifecycle has a real-wiring driver test (C157 clause 5).
- The rest of the store and annotation `c137_` tests, the drain bound at zero, and the CLI idle-exit test, beyond their names.
- The `c158_the_token_table_through_the_door` test (refresh.rs:470) and the daemon test harness's park hook.
- How a GUI or MCP frontend renders `journal_path`; only the CLI's `rails_text` and the verbs diagnosis.

## Noticed
- C147, step 1: clause 3 says `Client::connect`, `Client::observe`, `Persisted::observe` and `World::observe` take the world. In the code, crates/acquisition-client/src/client.rs:42-46, those bare names are the environment-reading forms. The world-taking ones are `connect_at`, `observe_at`, `stop_any_at`, `Subscription::observe_at` and `World::observe_at`. The entry's text disagrees with the code.
- C86, *Pinned:*: "the `c86_` tests in contract.rs" is one test, :303, which checks only the report's sentence. The verdict on every door and the absence of a spawn are held by `c137_the_handshake_deadline_at_zero_…` (contract.rs:342) and by the door table (door.rs:685).
- C137 clause 2 against the code: the harness's `HANG` and the verbs tests' `Instant::now() + 120s` asserts are wall-clock bounds a slow machine can miss. harness/mod.rs:361 calls them a hang ceiling, "never a measure of the wait". The clause as written does not separate a hang ceiling from a bound.
- C158, *Pinned:*: clause 4's census, tools/daemon-tests-census.py, is report-only and runs in no gate stage, and the *Pinned:* does not name it.
- The fate rule, step 2 versus 4: it does not decide an entry whose promise clauses mix Y and U and have no P or N (C157 would have been one, had clause 5 been held). Rule 2 needs every promise clause U, rule 3 needs every one Y, and rule 4's stated condition is unmet; only its "Otherwise" catches it.
