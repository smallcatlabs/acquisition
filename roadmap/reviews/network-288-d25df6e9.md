602a801b^..d25df6e9 — issue #288’s done-when does not fully hold.

- **P2 — `crates/acquisition-verbs/src/jobs.rs:142`, shape 3:** Stall detection handles only `apply`, leaving a deep stash parent waiting indefinitely. Concrete client input: `acq stash <map-tab> --deep`, followed by enough upstream invalid substash answers to close the route permanently while children remain queued. The parent stays `Running`, with no `held` value, so neither this command nor `result --wait` returns the remedy. Apply the descendant-stall rule to every parent kind and pin the direct deep-stash case.

- **P2 — `crates/acquisition-daemon/src/daemon.rs:949`, shape 1:** Permanently closed routes still receive finite start estimates. With a persisted closure whose `until` is null, `waiting_etas` attaches `RouteClosedForGood` beside an ordinary limiter ETA, potentially `0`; the jobs table consequently prints `~0s`, and a learned route’s quote likewise retains a finite ETA. Represent an unavailable start time as `None` throughout job estimates, counts and quotes, while retaining the closure reason.

- **P2 — `crates/acquisition-daemon/src/rails.rs:663`, shape 3:** A successful trial’s relist warning reaches the daemon log but not its caller’s report. Concrete upstream sequence: five invalid answers, cooldown, then a separately submitted job answered 200. Reopening removes the closure and returns `None`, so that successful job carries no warning that earlier facts may precede a server rollback. Carry the reopen fact into the job result, render the relist advice, and pin the successful report.

- **P3 — `crates/acquisition-daemon/src/daemon/dispatch.rs:110`:** A permanent token-route closure causes jobs needing refresh to cycle through start and park instead of remaining parked. Concrete persisted input: `closed_routes["oauth-token"].until = null`, with an account whose access token has expired and whose refresh grant has no dead-grant mark. The picker checks only the API route; refresh encounters the token halt, parks the job, and `run_active` immediately wakes the picker again, repeatedly writing queue transitions. Use one effective-hold rule that includes a needed token refresh in picking, waiting status and idle accounting.

- **P3 — `crates/acquisition-daemon/src/daemon/tests/dispatch.rs:3016`, shape 2:** The close/reopen assertion checks dashboard log messages instead of the journal required by the done-when. After five invalid answers and a successful trial—or an explicit reset—the send journal contains no closure transition event; `announce_trip` writes only the daemon log. Add journal transition records and assert them through the existing journal reader.

- **Not checked:** Execution, including the gate and mock tests, is **not verified** because this review was read-only. The uncommitted change to `decisions/network.md` was excluded from commit evidence.

- **Nitpicks:** None. The token-route case is uncommon, but its repeated queue writes are operationally significant.

- **Brief:** Distinguish the send journal from the daemon log explicitly; the landed tests validate the latter.
- **Plan:** No approved plan was supplied, so the replacement mechanism was judged against the issue’s required properties.
- **Prompt:** Nothing else impeded the review.
