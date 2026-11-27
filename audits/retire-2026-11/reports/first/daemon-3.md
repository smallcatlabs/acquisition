### C160 — Every tokio timer in the daemon
- class: check — tells later sessions writing door scenarios that the park hook sees every timer, so a stuck wait fails at the park instead of running to `HANG`
- clauses:
  1. P · Y · `tools/arch-check.sh:892-974` (check 12) refuses any tokio timer outside `impl Clock for SystemClock` and tail test modules, under every spelling, and `tools/arch-check-breakers.sh:432-437` stages the refusals. `crates/acquisition-daemon/src/daemon/tests/door.rs:1715-1760` asserts that a wait on a halted daemon's job fails with "woken by nothing the hook accounts for", names the job, and shows one stuck park and no clock advance
  2. M · Y · `door.rs:1916` (the transport's hold keeps the clock still and leaves `settle` pending until each release); `dispatch.rs:198-210` (a `hold_clock` keeps both rate-wait sleepers parked); `dispatch.rs:3333` (the hook's count is 0 at rest and 1 under a held send); the blocking count feeds the `IdleHook` branch at `door.rs:385-410`. The mechanism is also stated in the doc at `door.rs:265-275`
  3. P · Y · `door.rs:1752` (the verdict's words name what the scenario awaited); `door.rs:1773` (a wait on a door opened beside another gets the verdict too, and a beside-door on its own clock is refused); `dispatch.rs:198-210` (a scenario hold moves no deadline)
  4. P · Y · `door.rs:2069-2118`: a wait that declares the transport's holds as its own fails with "nothing but the requests the scenario holds: 1 held", and the clock does not move
  5. P · Y · the same test, `door.rs:2111`: the verdict's words blame the scenario's held request (the "false declaration" wording in `or_stuck_declaring`, `door.rs:1598-1640`)
  6. M · R · C161 (send deadline at the choke point)
- unheld: none
- reason: Prop · `crates/acquisition-daemon/src/daemon/tests/door.rs:37` states the property; the first half of the *Why:* (a tokio timer parks beside the clock unseen) is only in `tools/arch-check.sh:892` · moves to `crates/acquisition-daemon/src/daemon/tests/door.rs:37`
- fate: RM
- confidence: high · I did not read the bodies of `c160_the_transports_hold_holds_the_clock_through_the_door` past its rows, or the breaker script's cases past their names

### C23 — Work that needs many requests is
- class: design — tells frontends a multi-request job is a parent with children, and that its failure carries a typed tally they read instead of prose
- clauses:
  1. P · Y · `crates/acquisition-daemon/src/daemon/tests/parents.rs:22-95`: an apply becomes exactly its tuples as children, in order, under the parent's account
  2. P · P · "Finishes when its last descendant does" is held: `daemon/tests/queue.rs:2160-2185` (the held parent ends only after child 7 is cancelled, its tally counting that cancel) and `parents.rs:56` (`children.done == 2` at the parent's end). "Cancels its descendants when cancelled" is held by nothing: `parents.rs:99/124` cancel a parent and check only the parent, and no test checks a child's state after its parent's cancel. The mechanism half ("gives up its dispatcher task and scheduling key") is stated at `daemon/dispatch.rs:334` and has no test of its own
  3. P · Y · `crates/acquisition-verbs/tests/daemon.rs:404-440`: a parent whose child failed carries `children` as a field whose counts sum to the total, with one failed id. `acquisition-protocol/tests/wire.rs:470-488` pins the field's shape
- unheld: 2 simple · wrong · `daemon/tests/parents.rs` component tests (`test_daemon`, `submit_child`): cancel the parent and assert each waiting or running child ends `Cancelled`
- reason: Cite · `crates/acquisition-protocol/src/job.rs:186-189` and `:324`, `crates/acquisition-daemon/src/daemon/dispatch.rs:10`, `:407`; no file states the *Why:* (children get the queue, priorities and ETAs free) · moves to `crates/acquisition-daemon/src/daemon/dispatch.rs:10`
- fate: NT · simple
- confidence: medium · I searched for a cascade-cancel test by name and by `cancel(` and `Request::Cancel` in tests, not in every test body

### C27 — A client that disappears leaves its
- class: design — tells a user or agent that leaving, Ctrl-C or a crash cancels none of their jobs, which can be picked up again by id
- clauses:
  1. P · Y · `crates/acquisition-cli/tests/seat_faults.rs:578-600`: after a `--detach` submit the `acq` process exits; the job is still running (`wrong_state`) and later succeeds
  2. P · P · Ctrl-C is held on Unix: `crates/acquisition-cli/tests/provider_default.rs:184-219` sends SIGINT to the starting command's group and asserts the same daemon pid still answers. A closed terminal (SIGHUP) and Windows are not tested; the test is `#[cfg(unix)]`
  3. M · P · Unix is held by the same test (a daemon without its own group dies of the SIGINT). The Windows `CREATE_NEW_PROCESS_GROUP` has no test. Both are stated at `crates/acquisition-client/src/client.rs:2172-2175` and `:2208`
  4. P · Y · `crates/acquisition-client/tests/contract.rs:1337-1380`: after a stop and restart, `Request::Result { id }` still answers the old job, and ids continue
- unheld: 2 simple · wrong · `provider_default.rs:184`, the same test with `-s HUP` beside `INT` (the Windows half would be complex: a Windows console-signal harness)
- unheld: 3 cut · no caller names the process group; the spawn comments at `client.rs:2172` and `:2208` carry it
- reason: Cite · `crates/acquisition-client/src/client.rs:2208`, `crates/acquisition-cli/tests/provider_default.rs:176`; no file states "the sends are committed either way" · moves to `crates/acquisition-client/src/client.rs:2208`
- fate: NT · simple
- confidence: medium · I did not read `tools/persist-check.sh`'s mock path (it is a manual script, not a gate stage), and I did not check whether the hosted Windows gate runs any Ctrl-C test

### C45 — With no daemon to ask, an
- class: design — tells a user or agent at any frontend that, with no daemon, the observer verbs answer `running: false` from the queue on disk, never "nothing outstanding", and spawn nothing
- clauses:
  1. P · Y · `crates/acquisition-verbs/tests/daemon.rs:261-318`: with no daemon, `jobs` lists the open rows [1,2,3] (waiting, recorded running, a child) with `written_at`, and leaves out the done and cancelled rows; `daemon` counts waiting 2 and recorded running 1
  2. P · Y · the same test: a seeded ledger lists its rows, so an empty answer fails, and a world with no ledger is `persisted: null`, not an empty queue (`:265`)
  3. P · Y · The shared function is held at `verbs/tests/daemon.rs:208-251` (status, result and the unknown-job refusal from the ledger; nothing spawned). Through the binaries: `crates/acquisition-cli/tests/persisted_queue.rs:40-100` (`jobs`, `status`, `result` and `daemon status`; no socket; ledger untouched) and `crates/acquisition-mcp/tests/persisted_queue.rs:40-90` (`job_result` equals `acq result --json`, then `job_status`, a `wrong_state` refusal, no daemon). "Never while a daemon answers" is held by the live tests that assert `running: true` (`provider_default.rs:171`). The read-only `Persisted` view is mechanism, stated at `crates/acquisition-store/src/jobs.rs:24`
- unheld: none
- reason: Prop · `crates/acquisition-verbs/src/jobs.rs:1-3` and `crates/acquisition-store/src/jobs.rs:15-18,24` state the property; the reason (a halted daemon idles out with its queue on disk) is not stated · moves to `crates/acquisition-verbs/tests/daemon.rs:254`
- fate: RM
- confidence: high · I did not read the MCP `list_jobs` or `daemon_status` assertions in `acquisition-mcp/tests/daemon_status.rs` past line 110; they call the same verbs function

### C49 — Only the session layer interprets accounts.
- class: mech — how the daemon's internals treat an account, with no promise outside the daemon (the filing and limiter promises it touches are C144's, C50's and invariant 2's)
- clauses:
  1. M · N · No check or test fails on an `if account == …` outside the session layer. I looked at `tools/arch-check.sh` and at the `C49` cites (`daemon.rs:60`, `:650`); it is stated at `crates/acquisition-daemon/src/daemon.rs:60`
  2. M · Y · `crates/acquisition-daemon/src/daemon/tests/sessions.rs:1268-1300` files the body under the job's account path, never the session's. `crates/acquisition-daemon/src/ratelimit/tests/spec.rs:3881-3930` asserts that an Ip-scoped policy is one state across two accounts' keys (the account is an opaque key part, and scope comes from the headers)
- unheld: 1 cut · nothing outside the daemon relies on where the interpretation lives; the doc at `daemon.rs:60` carries it
- reason: Prop · `crates/acquisition-daemon/src/daemon.rs:60` states the property for the limiter only; the *Why:* (a second interpreter of what an account means) is not stated · moves to `crates/acquisition-daemon/src/daemon.rs:60`
- fate: RM
- confidence: medium · I did not search the daemon for an account comparison outside the session layer that would already breach it

### C51 — The daemon keeps no default account;
- class: design — tells every frontend and agent that a job names its one account, resolved exactly and never guessed, and is refused when ambiguous
- clauses:
  1. P · Y · `crates/acquisition-client/tests/contract.rs:1510-1530`: after two logins, a submit with no selector is `AmbiguousAccount`, not the last login. `crates/acquisition-daemon/src/daemon/tests/sessions.rs:1078` asserts the same
  2. P · P · Statelessness is held only as far as clause 1: no test names an account and then submits with none among several to show the daemon did not remember the choice (`sessions.rs:1053-1095` asks `None` before `Some("other")`, never after)
  3. P · Y · A type: `account` is a field of `Request::Submit` and `JobInfo` (`crates/acquisition-protocol/src/job.rs:99`), pinned in the wire sample at `acquisition-protocol/tests/wire.rs:278`
  4. P · P · "Resolves only with exactly one session" is held at `sessions.rs:1057-1060` and `:1078`. "Refuses with the list" is not: the assertion checks only "several accounts", though the message at `daemon.rs:678` carries the names
  5. P · Y · `crates/acquisition-store/unit/tests/index.rs:19-20` (`"Ali"` is `NotFound`: never by prefix); `crates/acquisition-verbs/tests/documents.rs:270-300`: with no daemon, `job_account` among several is refused as `ambiguous_account` with `accounts: [USER, "Bob#9"]` before any submit
  6. P · Y · A type: `JobInfo.account: Option<String>` (`protocol/src/job.rs:99`); children inherit it (`parents.rs:86`)
  7. P · U · "cross-account work is a frontend loop" is a rule on future frontends that no test can fail on
- unheld: 2 simple · wrong · `sessions.rs:1053` `submit_resolves_the_account_against_the_live_session`: after `Some("other")` with two live sessions, assert `resolve_account(None)` is still ambiguous
- unheld: 4 simple · para · the same test, `:1078`: assert the message names both live accounts
- reason: Prop · `crates/acquisition-daemon/src/daemon.rs:72-80` and `:569` state the property; the reason (a default held in the daemon is state every client must guess) is not stated · moves to `crates/acquisition-daemon/src/daemon/tests/sessions.rs:1051`
- fate: NT · simple
- confidence: medium · I did not read `acquisition-cli/tests/seat_faults.rs:279` or the CLI's own `--account` resolution in `run.rs:61`

### C148 — No credential-store or account-index I/O runs
- class: mech — how the daemon locks around keyring and index I/O; the promise beyond it (`hello` and status answered during a keychain prompt) is held in one crate
- clauses:
  1. P · Y · `crates/acquisition-daemon/src/daemon/tests/credentials.rs:131-175` (a socket `hello` is greeted while a refresh's save is held; Unix only); `:680` (the same while the restore reads); `:914` (an index read under `credentials`, and an unrelated request answered)
  2. M · Y · A type: `CredentialsHeld` is made only by `hold_credentials`, and every index write and keyring persist needs one (`crates/acquisition-daemon/src/daemon/held.rs:1-9`). Stated at `daemon.rs:1044-1052`
  3. P · Y · `credentials.rs:680` (the socket serves before the restore), `:758` (a submit waits and is never refused), `:803` (the dispatcher picks nothing, the daemon is not idle, and the drain waits until the release)
  4. P · P · The bearer half is held: `credentials.rs:184-228` (a permit holder's HEAD carries the installed token while the mark reads `saving`). "A waiter on a refresh served at the install" is loose: `refresh_held_in_its_save`'s future is awaited only after the release (`:168`, `:224`), so a refresh that answered at the save would pass
  5. P · Y · `credentials.rs:304-308`: a second `AuthStart` while a login is being recorded is `Refused`, "a login is being recorded; try again in a moment"
  6. P · P · `credentials.rs:332-395` holds the drain waiting for a held logout clear. No test holds the drain over a refresh or login save; the bound is held only generally (`exit.rs:1348`)
- unheld: 4 simple · para · the `saving_daemon` and `refresh_held_in_its_save` harness in `credentials.rs`: assert the refresh future resolves before `held.release`
- unheld: 6 simple · para · a row beside `credentials.rs:332` with `hold_the_next_save` on a refresh, `begin_exit`, and the drain polled pending until the release
- reason: Prop · `crates/acquisition-daemon/src/daemon.rs:1044` states the property; the reason (a keychain prompt made every client report the daemon unresponsive) is only in test docs (`credentials.rs:124-128`, `:670-677`) · moves to `crates/acquisition-daemon/src/daemon.rs:1044`
- fate: NT · simple
- confidence: medium · I did not read the bodies of `credentials.rs:680-912` past their doc comments, or `start.rs:589`

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C160 | check | 4 | 2 | 5/0/0/0/1 | Prop | RM | 0/0/0 | 0/0 | high |
| C23 | design | 3 | 0 | 2/1/0/0/0 | Cite | NT | 1/0/0 | 1/0 | medium |
| C27 | design | 3 | 1 | 2/2/0/0/0 | Cite | NT | 1/0/1 | 1/0 | medium |
| C45 | design | 3 | 0 | 3/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C49 | mech | 0 | 2 | 1/0/1/0/0 | Prop | RM | 0/0/1 | 0/0 | medium |
| C51 | design | 7 | 0 | 4/2/0/1/0 | Prop | NT | 2/0/0 | 1/1 | medium |
| C148 | mech | 5 | 1 | 4/2/0/0/0 | Prop | NT | 2/0/0 | 0/2 | medium |

## What I did not check
- Whether the hosted Windows gate runs any test of C27's Ctrl-C or of C148's `hello`; both tests I read are `#[cfg(unix)]`.
- For C23, the full bodies of `queue.rs` and `dispatch.rs` tests: my search for a cascade cancel used names, `cancel(`, `Request::Cancel` and "descendant".
- For C49, whether the daemon already compares accounts anywhere outside the session layer.
- For C51, `seat_faults.rs:279` and the CLI's resolution in `run.rs`; for C45, the MCP `list_jobs` and `daemon_status` assertions past `daemon_status.rs:110`.
- For C160, the rows of the transport-hold table past its first two, and the breakers' bodies.

## Noticed
- C49, the fate step: with no P clause, fate rules 2 ("every P clause is U") and 3 ("every P clause is Y") are both true of nothing, and rule 2 comes first, so taken literally it gives SH-KU. I applied rule 3 (RM) because the entry promises nothing, but the rule does not decide an all-M entry.
- C23, the *Pinned:* step: the two tests it names (`a_cancelled_parent_cannot_gain_children`, `cancelled_children_do_not_make_a_parent_successful`) hold the tally and the race, not "cancels its descendants when cancelled". That clause has no test, though `queue.rs:1224-1226` implements it.
- C27, the clauses step: "on every OS" and "a closed terminal" go beyond the one `#[cfg(unix)]` SIGINT test its *Pinned:* names. `tools/persist-check.sh:224` cites C27 but is a manual script, not a gate stage.
- C148, the clauses step: clause 6 says "a save in flight", but its one drain test holds a logout's clear, not a save.
- C160, the class step: clause 1's first half is a rule on production code (every timer through `Clock`), but its only reader is the test harness. I classed the entry `check`; a reader taking the headline alone might call it `mech`.
