68cc621b^..9f1d20d7 — `clock-tests.md`’s done-when is not fully met: the sweep leaves clock-dependent assertions.

- **[P2] [crates/acquisition-store/src/lib.rs:4410](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/lib.rs:4410), shape 3:** The required sweep leaves the unchanged re-ingest test asserting completion within five seconds; the committed account of #131 names elapsed bounds in the daemon and client, without disposing of this store instance. **Failing input:** the synthetic upstream tab containing 10,000 items, re-ingested unchanged while scheduling or CPU contention stretches execution beyond five seconds, fails despite correct ingest results. **Smallest fix:** retain the semantic assertions and timing diagnostic, remove the elapsed-time assertion, and adjust the test’s name and comment; alternatively, explicitly defer this instance with its evidence.

- **[P3] [crates/acquisition-cli/tests/store_answers.rs:150](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/store_answers.rs:150), shapes 1 and 2:** Moving the persisted event timestamps closer to now increases the margin but does not cover CI’s configured 240-second test ceiling. **Failing input:** pause for 133 seconds after seeding; the oldest event becomes 145 seconds old, so the client request `--hours 0.04` correctly excludes it and the twelve-event assertion fails before CI’s timeout. **Smallest fix:** use a fractional window exceeding CI’s ceiling plus fixture age, and update the expected rounded text and exact-command assertion together.

Execution evidence—gate status, test count, and deliberate regression checks—not verified, as requested.

I did not inspect external CI runs or issue #131’s body because this review judges the committed evidence. No production network path changed; the reviewed store initialization and rendering changes revealed no invariant bypass.

Finding 2 is a nitpick under ordinary conditions because it requires an unusually long delay, though it falls directly within this brief’s slow-runner concern. Finding 1 is not.

The brief’s unconditional “no assertion” goal conflicts with its allowance to defer instances; I would state that exception in the goal. The plan and review prompt otherwise posed no obstacle.
