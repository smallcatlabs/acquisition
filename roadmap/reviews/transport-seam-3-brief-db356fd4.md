`roadmap/briefs/transport-seam-3.md` — not buildable as drawn: Do 1 leaves an existing gate test red.

Findings, most severe first:

1. **P2 — Do 1: the new deadlines invalidate an existing sleeper-count assertion.** Concrete reading: [door.rs:1789](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:1789) expects exactly one sleeper while requests remain held. With the proposed deadlines, the two-request row has three sleepers and the held-body row has two. Merging Do 1 and Do 2 does not resolve this. **Smallest brief change:** include this test’s adaptation in Do 1, accounting for pending send deadlines while preserving its hold, pending-settle and no-time-advance assertions.

2. **P2 — Do 2: removing reqwest’s timeouts removes the effective hang bound from existing TCP tests.** Concrete reading: [transport/lib.rs:217](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-transport/src/lib.rs:217) awaits the production transport directly; [spec.rs:2733](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/tests/spec.rs:2733) runs production sends on a parking clock that this test never advances. Neither path has an outer hang ceiling, so the new virtual deadline cannot replace the removed real timeout there. **Smallest brief change:** require hang ceilings around the kept TCP tests’ exchanges and completion waits in Do 2, including the transport crate’s test, without adding elapsed-time assertions.

3. **P3 — Do 1: the statement that an answer cut flows through `finish_send` incorrectly includes HEAD.** Concrete reading: HEAD uses its own probe observation and recording path at [choke.rs:680](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:680) and [choke.rs:686](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:686); `finish_send` calls `note_lost_send` only for counted failures at [choke.rs:823](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/choke.rs:823). **Smallest brief change:** restrict that instruction to POST and both GETs, explicitly retaining HEAD’s uncounted probe-failure and recording path.

4. **P3 — Done-when 7: “no frontend prints a send’s error” is false.** Concrete reading: the daemon wraps transport errors into job failures at [kinds.rs:855](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/kinds.rs:855), and the CLI prints their error text at [main.rs:2845](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:2845). **Smallest brief change:** acknowledge that deadline words reach users; justify unchanged goldens by their existing cases not exercising expiry, and retain the requirement that other failure words and references remain unchanged.

What I did not check and why: compilation, clippy, tests, gates, breakers, runtime cancellation/EOF behaviour, or refreshed CI status. This was the requested static, read-only review; no build or test was run.

Which findings I would call nitpicks: 3 and 4 are wording corrections; 1 and 2 are substantive.

The step should not start as drawn; amend the brief first.
