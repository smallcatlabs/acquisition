9eb3beac^..2afd10b8 — the brief’s done-when does not fully hold: the census misses supported attribute forms, and window-dependent tests lack isolation.

- **P2 — [mockggg.rs:657](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/mockggg.rs:657), shape 1:** The in-process mocks do not reliably “keep” divisor 1; they inherit the environment. With `ACQ_MOCK_WINDOW_DIVISOR=10` exported, `list_matches_the_per_job_walk_on_a_mixed_queue` and `quote_projects_per_scope_reserves_nothing_and_names_what_it_cannot_see` receive shortened windows despite retaining assertions requiring the originals. This is process-configuration input, not client, persisted, or GGG input. Smallest fix: let in-process fixtures explicitly select divisor 1 through a constructor argument, retaining environment parsing for daemon startup.

- **P3 — [daemon-tests-census.py:179](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/daemon-tests-census.py:179), shape 1:** The census silently drops tests with multiline attributes despite promising attributes “with any arguments.” For source input `#[tokio::test(\nflavor = "multi_thread"\n)]\nasync fn uncovered() {}`, it examines the `flavor` line for `fn`, then omits the test entirely, allowing an incomplete census to report zero “neither.” Smallest fix: consume each complete bracket-balanced attribute before locating the function, and pin this spelling.

- Execution evidence—gate statuses, runtime test count, timings, and census output—**not verified**: no builds, tests, or repository scripts were executed, as requested.

- Nitpicks: the census finding is a nitpick for this committed tree, whose test attributes use single-line forms; it matters when those forms change.

- Brief: explicitly require fixed configuration for in-process mocks instead of assuming the environment leaves their divisor unset.
- Plan: no other obstacle.
- Prompt: no change suggested.
