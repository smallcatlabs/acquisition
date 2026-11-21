2fea4663 — the fix round’s done-when is not fully met.

- **P3 — `README.md:226`, shapes 1 and 3:** The realm-default correction leaves the README’s sibling sentence unchanged: it still says every new search naming no realm takes the realm from facts. **Client input:** saving a valid Xbox trade URL without `--realm` while facts hold only PC saves under Xbox, contradicting this sentence. Update it to state the same precedence as the corrected help: the URL’s realm, then the realm inferred from facts.

Not checked: execution, gate status, test counts and MCP size — **not verified**, per the read-only instruction. Unchanged daemon transport internals were outside this review.

Nitpicks: the sole finding is wording-only.

Brief: no obstacle.
Plan: no change suggested.
Prompt: no change suggested.
