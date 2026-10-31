d61997a9 — the fix-round done-when does not fully hold: findings 2 and 3 are fixed and pinned; finding 1 retains a production race.

- **[P2] crates/acquisition-daemon/src/daemon/server.rs:862 — Shape 1: the activity comparison is not atomic with connection admission or the exit swap.** Concrete client-input interleaving on production’s multithreaded runtime: the saved verdict is `(0, old_stamp)`; the comparison reads zero connections; `handle_conn` increments the count and pauses before stamping; the comparison reads `old_stamp`, accepts the verdict, and begins exit with a connection open. Alternatively, the driver can be descheduled after the comparison while a client completes `hello`, then resume and swap. The smallest sufficient fix is an atomic admission/idle-commit handshake that orders connection entry against committing the idle exit, preserving lock-free `hello`, with a pin inside this remaining window. The existing probe correctly stages the judge-answer handoff, and removing the comparison would prevent its expected `Busy` acknowledgment, but it releases the client activity before the comparison and cannot expose this race.

- **Other checks:** The judge-start acknowledgment precedes acquiring `shared`; the connection test observes `Busy` without settling. Splitting that test retains coverage through the quiet-daemon test, the table’s rearm row, and their shared arming implementation. The listing uses `HANG` and still prints its measurement. Every probe field and use is test-only; the builder remains the sole `Daemon` construction. No goldens or references changed.

- **What I did not check:** execution, gate exit statuses, test counts, timings, and the comparison-disabled experiment are **not verified**; no builds, tests, or daemons were run.

- **Nitpicks:** none of the finding above. A completed connect-and-close leaving both values unchanged on the frozen manual clock is a test-clock limitation, not a practical production concern; the production race described above does not require equal timestamps.

- **Brief:** Clarify whether idle commitment must be atomic with connection admission or merely revalidate activity before attempting exit.
- **Plan:** No change suggested.
- **Prompt:** No change suggested.
