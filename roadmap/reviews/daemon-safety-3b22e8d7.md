3668c786^..3b22e8d7: the brief’s done-when does not fully hold for #113 and #116; #110’s narrowing change and frozen-index restart pin are present.

Findings, most severe first:

1. **[crates/acquisition-daemon/src/daemon.rs:1045](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon.rs:1045), shape 1:** The new token-free log guarantee excludes typed credentials but still admits raw upstream token-response text. An upstream HTTP 500 body such as `{"error":"temporary","refresh_token":"rt-live"}` passes through `auth.rs:342`, the refresh refusal and the dispatcher into this writer unchanged. Serde diagnostics can also quote malformed upstream values. Smallest fix: exclude raw OAuth response bodies and value-bearing parse diagnostics from logged errors, and extend the log test with those failure inputs.

2. **[crates/acquisition-daemon/src/auth.rs:439](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/auth.rs:439), shape 1:** Public `KeyringEntry::Serialize` leaves a plaintext escape from the claimed type boundary. For a persisted entry restored from the keyring, any daemon module can call `serde_json::to_string(&entry)` and obtain both tokens in a loggable `String`, despite `Secret` lacking `Serialize`. Smallest fix: remove serialization from the exposed entry type and serialize a private representation inside `to_secret`; pin that outside-module serialization is unavailable.

3. **[crates/acquisition-client/src/client.rs:623](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:623), shape 1:** The new refusal claims no daemon remains even when shutdown was only acknowledged. An incompatible child that answers `Stopping` but keeps its connection and endpoint open beyond `STOPPED_WITHIN` still produces `Some(Ok(()))`, because `stop()` discards both timeout and read errors at lines 1622–1627. Smallest fix: require confirmed shutdown before selecting this message, propagate timeout/read failures, and pin an acknowledging peer that lingers.

4. **[crates/acquisition-cli/tests/daemon_observe.rs:187](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/daemon_observe.rs:187):** The generated shell script does not escape apostrophes in the executable’s filesystem path. A checkout under `/tmp/Tom's acquisition/` produces an invalid shell command, so the test fails before exercising the intended identity mismatch. Smallest fix: escape the path as a shell argument or pass it separately to the script.

What I did not check and why:

- Execution evidence, gate statuses and test counts: **not verified**, as requested.
- External GitHub label and CI state were outside the committed evidence.
- No golden or reference files changed. The affected send paths retain the choke point and global gate.

Nitpicks: finding 4 concerns an uncommon checkout pathname; I would call it a nitpick.

Brief/plan: add malformed OAuth responses and acknowledged-but-incomplete shutdown to the acceptance cases.

Prompt: nothing impeded the review.
