1dc6145a — findings 1 and 2 are fixed by stated rules and pinned; full done-when remains conditional on the gate.

Findings: none.

- The scripted upstream twins require the cause’s words: truncated 200 carries `error decoding response body` beside the URL fragments, truncated 500 preserves status and body-read cause, and non-JSON carries `bad JSON`. These match the locked reqwest and daemon error paths. The rule is stated beside the twins.
- `door.journal_text()` reads the daemon’s own scratch `journal_path` whole, including its header, **before** `finish()` runs its oracles. The projection still reads send records separately. Forbidding `"event":"open"` would therefore fail on the header by inspection.
- Only test modules changed; no production code, golden, or reference changed.

What I did not check: execution evidence—including gate status, test counts, and the reported header experiment—is **not verified**; builds and execution were prohibited. Registry finding 3 is outside this range.

Nitpicks: none.

Brief: no change suggested for this fix round.

Plan: no change suggested.

Prompt: nothing obstructed the review.
