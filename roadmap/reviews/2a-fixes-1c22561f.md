f476aa0e^..1c22561f (two commits): the brief’s done-when is not fully verified; execution-dependent criteria are **not verified**.

Findings, most severe first:

1. **P2 — [daemon.rs:2188](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon.rs:2188), shapes 3 and 4:** removing failed-child handles from the daemon’s sentence loses the evidence and remedy on text paths that still ignore `children`. **Failing input:** parent job 4 finishes with failed child 7, then the daemon exits; `acq result 4` reaches `print_outcome` and prints only “1 of 3 child jobs failed, 2 done,” without identifying job 7 or its cause. Ordinary refresh parents have the same problem even while the daemon runs. **Smallest fix:** render bounded failed-child handles from `children.failed_ids` on every text failure path, including ledger results, while preserving the parent’s error.

2. **P3 — [roadmap/README.md:129](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/README.md:129), shape 3:** “Current step” still directs the owner to read the previous findings before the fix round, although this range deletes that review and lands those fixes. **Failing reading:** checkout at `1c22561f` points the next session to a missing review file and an already-completed action. **Smallest fix:** record the fix range and change the next action to reviewing this round, citing the previous review at `7e33eae9`.

What I did not check: builds, tests, gate stages, breaker execution, reference regeneration, or runtime/platform behavior, because execution was prohibited. Their done-when criteria remain **not verified**. I did not re-audit untouched limiter, gate, or provider implementations.

Nitpicks: finding 2 is administrative; finding 1 affects ordinary recovery workflows.

Brief: require failed-parent text results both with a live daemon and from the ledger.

Plan: explicitly assign the current-step update to the worker or launching session.

Prompt: no change suggested.
