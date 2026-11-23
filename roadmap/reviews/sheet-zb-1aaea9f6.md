1aaea9f6477a61d2944ff26024328e65223c7158 — The issue’s requested outcome is not complete: this commit prepares searches; the socket word, translation and page control remain unimplemented.

- **P2 — [tools/trade-sheet.py:2629](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/trade-sheet.py:2629), shapes 1 and 2:** zb02, zb06 and zb09 treat equal listing totals as proof of filter semantics or equivalence, although the site caps totals at 10,000 (`docs/design/trade-ground-truth.md:214`). Concrete upstream input: a correctly implemented minimum count matches 15,000 belts for `a:1` and 12,000 for `a:2`; both responses report 10,000, so zb02 incorrectly says the filter is boolean or ignored. Likewise, zb09 can miss excluded belts while reporting the same capped total as zb06. The shared cause is treating capped upstream totals as exhaustive counts. Mark capped comparisons inconclusive and add discriminating searches over known socket counts or counterexamples; update the generated CSV descriptions too.
- **Not checked:** execution checks, including gate, tests and link verification, are **not verified**, as requested. No browser searches or live requests were made.
- **Nitpicks:** none.
- **Brief:** the supplied issue describes the complete feature, while this commit explicitly prepares a sitting; give this round a scoped research acceptance criterion.
- **Plan:** no separate plan was supplied.
- **Prompt:** no other obstacle.
