b313662c: both prior findings are fixed and pinned in source; the broader logging guarantee still has gaps, and the gate-dependent done-when is unconfirmed.

Findings, most severe first:

1. **[P2] crates/acquisition-daemon/src/ratelimit/policy.rs:311 — shapes 1, 3:** The shared renderer still prints an over-cap `Retry-After` verbatim, including values that never parsed as numbers. Concrete upstream input: a successful HEAD with valid policy headers and `Retry-After: 18446744073709551616000123456789` writes that entire string through the newly changed HEAD logger; an echoed numeric credential therefore escapes the stated rule against upstream text accepted merely by shape. Smallest fix: make `RetryAfter::OverCap` display fixed words without `raw`, retain the original in diagnostic evidence, and pin the overflow-length header case against the whole log.

2. **[P3] crates/acquisition-daemon/src/daemon.rs:1052 — shape 3:** The logging contract still says accepted policy and rule names are “Kept by ruling,” although this commit explicitly withdraws that exception. Concrete failing reading: a maintainer following `write_log_line`’s contract would consider logging an accepted `rt-policy-p2Wn` permitted. Smallest fix: remove that exception and document the fixed rule labels and omission of policy names.

What I did not check and why:

- Execution evidence, gate statuses and test counts: **not verified**; no builds or tests were run, as requested.
- External CI and issue state were outside the committed evidence.

Nitpicks: finding 2 is a documentation nit. Finding 1 requires an unusual numeric-token echo, but contradicts the explicit construction guarantee.

Brief: distinguish bounded parsed numbers from digit strings retained after parsing fails.

Plan: the formatter sweep should include `RetryAfter::Display` alongside policy and send errors.

Prompt: nothing impeded the review.
