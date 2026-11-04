dcd8abcb..e3b5c217 — the fix round’s three findings are addressed and pinned; execution-dependent completion remains unconfirmed.

- Findings: none. The replacement spans preserve negation, the shared league reader follows the ordered syntactic rules, and both count filters preserve their outputs. No goldens or references changed, and the diff stays within the three findings.
- Not verified: builds, tests, runtime behavior and gate status; execution was prohibited.
- Nitpicks: none.
- Brief: no change suggested.
- Plan: no change suggested.
- Prompt: clarify explicitly that double negation still excludes a league from the pointer under the stated syntactic rule.
