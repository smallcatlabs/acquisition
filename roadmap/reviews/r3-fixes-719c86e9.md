# r3 fix round review — 30c90878^..719c86e9

Codex, read-only, `roadmap/briefs/REVIEW-PROMPT.md`, 2026-10-27. Its final message, verbatim:

30c90878^..719c86e9 — brief not fully met; execution-dependent done-when lines are **not verified**.

- **P2 — [queue.rs:275](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/queue.rs:275), shape 5:** The widened listing range imports unrelated history across lifetimes and can make a previously readable queue exceed the protocol’s 64 MiB frame limit. Concrete input: restored waiting job 1, plus nine recent finished jobs accumulated across lifetimes, each with admissible params just below 8 MiB. `job_list` now includes them all; the client discards the response, breaking ordinary `jobs` and apply progress reads. Smallest fix: include explicitly restored ids alongside this lifetime’s finished jobs, rather than every intervening historical row, and update the descriptions.

- **P3 — [README.md:121](/Users/tom/Development/GitHub/smallcatlabs/acquisition/README.md:121), shapes 1 and 3:** The idle guarantee still exceeds what `is_idle` guarantees, with the same cause repeated in the knob row at line 241. Concrete cases: a queue-write failure permits idle exit over waiting jobs without a rails halt; a halted apply parent holding its result remains `Running` but does not hold the daemon. Smallest fix: qualify both sentences for queue failures and distinguish executing jobs from parents held for halted children.

Not checked: builds, tests, reference regeneration, the gate, actual daemon status, or RSS measurements, because execution was forbidden. Their success is **not verified**; the reviewed commits contain no gate stage exit-status table. The changed reference text matches its source changes and is explained in the commit message.

Nitpicks: the README qualification is minor wording. The frame failure is not a nitpick, although near-limit padding is a stress input.

Brief: reconcile Do 3’s “every finished job within retention” with the narrower listing accepted in the commits.

Plan: add accumulated history across restarts, queue failures, and halted deferred parents to the schedules.

Prompt: no change needed.
