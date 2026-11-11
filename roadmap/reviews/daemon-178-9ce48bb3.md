7c6b7b1c^..9ce48bb3 — the issue’s explicit wording, retirement and coverage conditions hold by inspection; two findings remain.

- **P2 — crates/acquisition-daemon/src/daemon/start.rs:395 — shape 4.** Removing the CLI commands also removes the recovery instruction, and no adapter restores it. A second daemon started against an occupied world now reports only why it cannot start; the client’s startup-log excerpt carries that same dead end. This meets local process state, not client, persisted or upstream input. Smallest fix: add frontend-neutral guidance to use the existing daemon or have a person stop it before retrying, and pin that guidance in the existing world-lock refusal test.

- **P3 — crates/acquisition-mcp/tests/harness/mod.rs:340.** The newly checked protocol-error message includes echoed client input, which the rule incorrectly treats entirely as daemon-authored prose. With the existing known-account fixture and daemon, `submit_job` with `{"kind":"profile","account":"--account"}` correctly returns an unknown-account refusal quoting `"--account"`, but the harness panics on that echo before the test can inspect it. Smallest fix: exclude the bounded selector echoed from the request when scanning the sentence, and add this case.

Execution evidence, including the gate’s exit status and test count: **not verified**, as requested. I did not exercise runtime, cancellation or concurrency; I inspected the changed paths and their callers.

The second finding is a nitpick affecting an unusual test input; it does not change production output.

The supplied issue was sufficient as the brief. No separate plan was supplied, and nothing in the prompt obstructed the review.
