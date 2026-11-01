0ae93980^..82ac4f4c — the brief’s done-when does not fully hold.

- **[P2] crates/acquisition-daemon/src/daemon.rs:1027 — shape 1:** `within_on` loses Tokio timeout’s protection against cooperative-budget starvation. Client input keeping a callback body continuously readable, or unrelated job events continuously available during `wait_job_terminal`, can exhaust the wrapped future’s budget on every poll; `SystemClock`’s sleep then also returns Pending, even after expiry. Poll the timer arm unconstrained while retaining future-first ordering, and pin expiry against a receive loop that exhausts its budget.

- **[P2] tools/arch-check.sh:685 — shape 1:** Check 12 enforces one spelling rather than every daemon Tokio timer. Concrete source input `use tokio::time;` followed by `time::sleep(...).await` contains no `tokio::time::`, so an unseen production timer passes. Check imported timer paths too, or enforce a single import spelling, and add this case to the breakers.

- **Not checked:** execution—builds, tests, gates, breakers, measurement totals and mutant outcomes—is **not verified**, because this review was restricted to reading.

- **Nitpicks:** none; the starvation schedule is uncommon, but losing a deadline is a behavioural regression.

- **Brief:** The blanket “no wall-clock bound other than `HANG`” criterion contradicts the existing 30-second guard at `crates/acquisition-daemon/src/daemon/tests/credentials.rs:22,35`; narrow it to the waits this step changes.
- **Plan:** Specify timeout equivalence and timer enforcement as behavioural properties rather than prescribing implementations that leave these gaps.
- **Prompt:** No change needed.
