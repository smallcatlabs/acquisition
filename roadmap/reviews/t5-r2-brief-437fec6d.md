`roadmap/briefs/t5-r2.md` at `437fec6d`, unchanged in the working file: **not buildable as drawn.**

The eleven findings:

1. **Answered.** The absolute SystemClock override is specified; the pinned Tokio version implements `From<std::time::Instant>`.
2. **Narrower (sound).** Four internal cases remain components; `stop()` represents the bootstrap ingress, and a void `await_leave()` followed by protocol reads respects the chosen boundary. Status still answers after recorded leave.
3. **Answered.** Unused door and builder preparation can precede the timer change; the second commit names all ten drain cases, six probe tests, two multi-thread tests and four credential cases.
4. **Not answered.** The signal test’s post-release registration wait can hang if the driver already left.
5. **Answered.** Coordinated driving replaces the unrestricted loop; credentials’ actual drain futures can be polled Pending in place, then driven after landing completion.
6. **Answered.** `await_job` supplies subscribe-before-read predicate waits for Running and Waiting.
7. **Not answered.** The restore test is a sufficient shared pending-before-release pin, but the code-exchange case still demands unreachable successful-login evidence.
8. **Not answered.** The helpers stay, but done-when 6 still excludes retained component callers and matches the helpers’ definitions.
9. **Answered.** Account, Delayed, Own, Script and None match the fixtures. Held GETs and token POSTs occupy the gate; refresh landings use counted blocking work. The other five cases’ proposed protocol evidence is reachable.
10. **Answered.** ManualClock acceptance is restricted to lifecycle callers.
11. **Answered.** The six/four count, ready-on-first-poll citation and dispatcher teardown account are corrected.

Findings, most severe first:

1. **Do 2 — The code-exchange scenario cannot observe a successfully registered login after stopping.** After the held POST returns, [login.rs:229](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/login.rs:229) submits its profile job, but [dispatch.rs:69](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/dispatch.rs:69) starts nothing after exit begins. A Waiting profile is deliberately excluded from the drain’s login hold at [server.rs:1077](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/server.rs:1077); recorded leave therefore does not imply `!pending` or a newly registered session. **Smallest change:** replace successful-login evidence with the exchange POST and its newly queued Waiting profile, obtained through List/Status after leave. Keep successful session registration with the retained staged-login component.

2. **Do 2 — The blocked-worker test can strand its drain after the lock is released.** [exit.rs:420](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:420) spawns the drain without proving both workers have blocked first. It can register a quiet-look timer while `shared` is held; releasing the holder and joining the blocked tasks does not wake that parking-clock timer, so the final drain await at [exit.rs:446](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:446) can hang. **Smallest change:** prescribe coordinated post-release clock driving for this test too, accepting an immediate leave when no further look is needed; retain its real bound-thread assertion.

3. **Do 2 — The signal test cannot unconditionally wait for a registered sleeper after releasing `shared`.** Its observation at [exit.rs:1127](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:1127) proves admission stopped and the line was logged, which happens before the initial quiet look. If release wins that race, [server.rs:920](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/server.rs:920) finds quiet and leaves without registering any sleep; `sleepers() >= 1` then never arrives. **Smallest change:** synchronize the registered look while the lock remains held, or allow immediate leave alongside the registration wait. This and finding 2 share the missing synchronization around a spawned driver’s first look.

4. **Do 2 / done-when 6 — The retirement requirement contradicts the retained component scope.** Components still use `spawn_drain` at [exit.rs:232](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:232), `:262` and `:595`, and `settle` at `:268` and `:596`; the brief retires uses only in moved scenarios. Its grep also matches the definitions at `:77` and `:82`. **Smallest change:** allowlist those retained component uses and definitions, or explicitly require their replacement and name their holders. The fault scenario and bound-thread test are not the only remaining users.

5. **Do 2’s Read inventory — Three harness fixture citations remain stale.** `account_server` is at [harness.rs:1254](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/harness.rs:1254), `delayed_token_server` at `:580`, and `scripted_server` at `:549`, rather than `:1052`, `:378` and `:351`. Also, `exit.rs:197` waits for Running only; the subsequent gate-queue assumption is the sleep at `:199`. **Smallest change:** correct those references and that state-wait description.

What I did not check: compilation, tests, gates, measured timing or live CI conclusions, because this review authorized reading only. No project code was executed or files changed.

Nitpicks: finding 5, and the helper-definition matches within finding 4. The retained-caller contradiction itself is substantive.

Round 2 may start as drawn: **no.**
