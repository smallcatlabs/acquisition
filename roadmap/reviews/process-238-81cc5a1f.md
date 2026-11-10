81cc5a1f — #238’s done-when does not fully hold: the removals landed, but ordinary quote forms still escape the check.

- **P2 — tools/docs-check.sh:119–124 (shape 1):** Case-insensitivity covers only `owner:`, so the matcher’s coverage falls short of “in any form.” Concrete repository input: adding `The owner's word: "Yes, the index."` to an entry escapes every alternative, although its lowercase spelling is refused. Make the whole attribution matcher case-insensitive and add breakers for sentence-initial capitalization, retaining the legitimate-party pass case.

- **Not checked:** Gate and breaker execution not verified, as requested; no build, test, or check was run. Historical “first landed” hashes were not independently traced.
- **Nitpicks:** None; sentence-initial capitalization is an ordinary spelling.
- **Brief:** The issue has no explicit done-when list; I treated quote removal and automatic refusal as its acceptance criteria.
- **Plan:** No separate plan was supplied.
- **Prompt:** Nothing impeded the review.
