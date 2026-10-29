95997a95: both prior findings are fixed in source, with the behavioral fix pinned; full fix-round completion remains unconfirmed.

Findings, most severe first:

1. **[P3] crates/acquisition-daemon/src/daemon.rs:1061 — shape 1:** The rewritten contract overstates retention by saying raw upstream headers and bodies remain in the journal and job payloads. Concrete failing reading: someone investigating a malformed token response expects its raw body in that evidence, but `SendReport` has no body field and token parsing failures return sanitized words without the body. Smallest fix: distinguish retained header snapshots from selected API payloads, and explicitly exclude token response bodies.

What I did not check and why:

- Execution evidence, gate statuses and test counts: **not verified**; no builds or tests were run, as requested.
- External CI and issue state were outside the committed evidence.

Nitpicks: finding 1 is a documentation nit; it introduces no runtime leak.

Brief: no change needed for this fix round.

Plan: no change needed; the shared formatter fixes the affected callers together.

Prompt: nothing impeded the review.
