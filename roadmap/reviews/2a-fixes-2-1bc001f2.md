1bc001f2: the targeted repair is present; the brief’s full done-when remains **not verified**, including execution-dependent criteria.

Findings, most severe first:

1. **P2 — [jobs.rs:294](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/jobs.rs:294), shape 1:** child-read failures become the unsupported claim “no result retained for this job.” **Failing input:** parent 4 and failed child 7 are retained, but the daemon stops after returning the parent and before answering the child request; the document reports missing retention instead of the connection failure. The disk reader shares this cause at [jobs.rs:390](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/jobs.rs:390), swallowing decode and database errors. **Smallest fix:** reserve that sentence for confirmed absence; otherwise retain the child’s handle and report that its result could not be read, with the underlying cause.

What I did not check: builds, tests, gate stages, breaker execution, reference regeneration, or runtime/platform behavior, because execution was prohibited. Those criteria are **not verified**. I did not re-audit untouched limiter, gate, or provider implementations.

Nitpicks: none; finding 1 affects uncommon error paths but gives agents a false factual answer.

Brief: add fix-round acceptance cases for live, restarted, and stopped-daemon parent results, including unavailable child reads.

Plan: update “Current step” when closing this round; it still describes this repair as forthcoming.

Prompt: no change suggested.
