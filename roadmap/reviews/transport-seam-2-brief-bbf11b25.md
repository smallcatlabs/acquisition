`roadmap/briefs/transport-seam-2.md` at `bbf11b25`, unchanged in the working tree, is not fully buildable as drawn.

Findings, most severe first. All six previous findings are answered in substance, including the helper-retirement order, component callers and `finish` checks.

1. **Do 5; done-when 2 — the bind-based inventory misses a test that still performs real provider I/O.** Concrete reading: `crates/acquisition-daemon/src/ratelimit/tests/spec.rs:28` constructs a production transport; `transport_failure_on_an_established_policy_paces_as_if_counted` uses it at `2236` and `2275`, attempting GET and HEAD connections to port 9 at `2259` and `2294`. It binds nothing and calls none of the inventoried fixture helpers, so both completion greps can pass while these exchanges remain real I/O outside the six kept tests. **Smallest change:** explicitly migrate both halves of this test to the refusing transport in Do 5, and extend done-when 2 to check transport selection for non-kept spec exchanges, including those without listeners.

What I did not check: builds, clippy, gates, tests, counters, census, mutants, breakers or current CI, because execution was prohibited. Static reading found no park between admission and the transport’s first poll, or between body completion and permit release. No further static commit-order blocker emerged; the hook’s branch precedence must apply across all registered doors.

Nitpicks: none; the finding concerns the promised I/O boundary and its completion evidence.

The step should not start as drawn; add the no-listener test’s migration first.
