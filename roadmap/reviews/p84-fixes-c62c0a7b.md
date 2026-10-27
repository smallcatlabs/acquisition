67b30646..c62c0a7b (acdda4dd, c62c0a7b): the fix round’s done-when holds by inspection, subject to execution verification.

- Findings: none. The structural exclusions preserve payloads, the round-trip test exercises the reported corruption, the comparison helper requires exact payload equality, and the flag mappings and `not_an_item` sweep address the saved review.
- Gate exit statuses, test count and runtime behavior: not verified; execution was prohibited.
- I did not check external issue comments or the worker’s report; this review judges the committed range and affected callers alone.
- Nitpicks: none.
- Brief: none applies; the saved review and orchestrator’s ruling supplied the acceptance criteria.
- Plan: no change suggested.
- Prompt: no change suggested.
