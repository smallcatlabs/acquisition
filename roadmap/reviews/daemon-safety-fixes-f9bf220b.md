95b7c541^..f9bf220b: the fix round’s done-when does not fully hold; finding 1 remains incomplete, and finding 4 lacks a regression pin.

Findings, most severe first:

1. **crates/acquisition-daemon/src/auth.rs:334, shape 1:** The token-free logging guarantee still admits arbitrary upstream text through raw rate headers and the OAuth-code filter. An upstream 400 with `X-Rate-Limit-Debug: rt-live-w4Qe` logs that token through `response.rate`; a body `{"error":"rt_echoed"}` also logs its value because it satisfies the filter at line 388. These share one cause: syntactic shape is treated as evidence that text contains no credential. Smallest fix: map recognized OAuth errors to fixed diagnostics, render validated rate information without arbitrary header text, and pin both echo cases.

2. **crates/acquisition-cli/tests/daemon_observe.rs:181, shape 2:** The apostrophe fix is correct but its failing filesystem input is not pinned. On a normal checkout path, removing the new escaping leaves this test green because `real_acqd` contains no apostrophe. Smallest fix: copy the daemon into a scratch path containing an apostrophe and make the existing partial-install scenario execute that copy.

What I did not check and why:

- Execution evidence, gate statuses and test counts: **not verified**, as requested.
- External CI and GitHub state were outside the committed evidence.
- No golden or reference files changed. The inspected send paths retain the choke point and global gate.

Nitpicks: finding 2 concerns an uncommon pathname; finding 1’s examples require unusual upstream responses, but contradict the explicit credential guarantee.

Brief: specify recognized OAuth errors and malformed-header cases in the acceptance inputs.

Plan: nothing impeded the review.

Prompt: nothing impeded the review.
