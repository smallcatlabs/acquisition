d81f5508 — the fix-round done-when holds by inspection: the restore hint is removed consistently, interim compatibility findings are explicitly declined with reasons, and a committed manifest fixture pins deserialization.

- Findings: none. Both changed golden diffs match their source changes.
- Execution evidence, including tests and gate status: **not verified**, as requested.
- Not checked: unrelated daemon sending paths or upstream GGG-response handling; this round affects refusal rendering for persisted local files and a manifest compatibility test. Client account selection was inspected through the affected CLI paths.
- Nitpicks for unlikely cases: none raised.
- Brief: nothing impeded this review.
- Plan: no separate plan was supplied.
- Prompt: nothing to change.
