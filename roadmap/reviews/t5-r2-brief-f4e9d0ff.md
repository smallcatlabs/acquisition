[roadmap/briefs/t5-r2.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r2.md) at `f4e9d0ff`, unchanged in the working file: **buildable as drawn.**

1. **Answered.** The exchange POST and queued profile’s Waiting status are reachable after leave: `login.rs:229` submits it, `dispatch.rs:69` prevents dispatch, and `server.rs:1077` excludes it from the drain.
2. **Answered.** The blocked-worker test can synchronize its first look while holding `shared`, provided that synchronization precedes worker saturation; its real bound-thread assertion remains.
3. **Answered.** In the signal test, observing registration before releasing `shared` guarantees a parked look (`server.rs:916–929`, `harness.rs:201–210`); the general rule also accepts leave without parking when the first look finds quiet.
4. **Answered.** Complete current inventory in `exit.rs`: definitions `77, 82`; `spawn_drain` calls `99, 138, 232, 262, 420, 550, 595`; `settle()` calls `100, 139, 268, 551, 596, 1393, 1413`. Removing the six moved-scenario calls leaves exactly the allowlist.
5. **Answered.** `account_server` is at `harness.rs:1254`, `delayed_token_server` at `:580`, and `scripted_server` at `:549`; `exit.rs:197` waits for Running, while `:199` supplies the gate-queue delay.

Findings, most severe first:

- **Do 2 — Minor sequencing ambiguity:** the blocked-worker synchronization needs an explicit position before both workers block. The existing order spawns the blockers at [exit.rs:414](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:414), then the drain at `:420`, so inserting a registration wait there can strand it. Smallest clarification: say “register the first look while `shared` is held, before spawning the two blocking tasks.” This is a feasible adaptation within the prescribed synchronization; no later round is needed.

What I did not check: compilation, tests, gates, timing measurements or live CI results, because this was a read-only brief review. No files changed.

Nitpicks: the sequencing clarification above. No other introduced contradiction, deferred-round dependency or citation mismatch found within this review’s scope.

Round 2 may start as drawn: **yes.**
