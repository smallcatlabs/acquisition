4b5118f8 — all three findings are fixed and pinned by inspection; the fix round’s done-when remains conditional on a green gate.

- **Findings: none.** The upstream HEAD/GET-429 case observes Waiting/retries=1, the re-pick and exactly one registered pacing sleeper before `begin_exit`. It advances to that sleeper’s deadline below `BOUND`, uses `drive_drain`, and asserts no cut, Waiting/retries=1 in memory and through a reopened database, elapsed time below `BOUND`, the turn-away log, two requests and journal equality. The C154 holder exists, runs under the gate’s test selection, and retains the retired scenario’s guarantees.
- All three clean scenarios retain their own parking clock through an `Arc` clone. Their strict `< HELD` assertions reject a bound cut, including the named exchange-count and unfinished-landing faults.
- The guard precedes `AuthStart` and spans the callback through the exchange’s arrival, where send occupancy protects the clock. The hook checks it first; the documentation states the general rule for scenario-owned external waits. No new unguarded window found. Only test code changed; no production path or invariant bypass changed.
- **What I did not check:** execution evidence—gate status, tests, breakers and timings—is **not verified**, as requested for this read-only review.
- **Nitpicks:** none.
- **Brief:** no change requested for this fix round.
- **Plan:** no change requested.
- **Prompt:** no change requested.
