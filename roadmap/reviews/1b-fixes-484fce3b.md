Model gpt-6.1-sol (from codex's stderr), 88,573 tokens used; 2026-10-20.
Command: `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: 47bbef81^..484fce3b."`

47bbef81^..484fce3b — the brief’s done-when is **not fully verified**; inspection confirms no “Parked” headings and twenty replacement bullets, but the fixes leave three gaps.

Findings, most severe first:

1. **CONTRIBUTING.md:32 — shape 3.** The README’s backlog correction leaves the contributor-facing sibling directing readers to deleted “Parked” sections and describing issues as mere evidence. A contributor following “Issues” receives the old workflow. Smallest fix: point this paragraph to GitHub issues and distinguish the remaining “Not doing” boundaries.

2. **tools/docs-check.sh:156 — shape 2.** The all-letter hash fix recognizes only one spelling and length range. An entry containing `at abcdefa`, or ``at `abcdefabc` ``, still escapes the history scan: neither alternative matches. Smallest fix: recognize explicit commit contexts with optional backticks across the existing 7–40-character range, and stage both inputs.

3. **tools/bothy.py:296 — shape 2.** The empty-section check strips spaces and selected punctuation but leaves tabs, allowing a visually empty reason or pin. An otherwise valid entry ending `*Why:*<TAB>*Pinned:* nothing yet.` passes this check. Smallest fix: treat whitespace consistently when testing emptiness and add a tab-only breaker.

What I did not check: I ran no builds, tests, breakers, checker scripts, binaries, or network calls. The green gate, actual checker outputs, and GitHub’s 48 issues and labels are **not verified**. No runtime implementation changed; inspection found no new sender or gate bypass.

Nitpicks: findings 2 and 3 concern uncommon malformed registry inputs and are robustness nitpicks.

Brief: its done-when combines inspectable content with execution results and live GitHub state, so this review cannot establish it completely.

Plan: explicitly identify which backlog instructions govern during the staged migration, while older instructions remain in place.

Prompt: the inclusive range and read-only restriction were clear; I would change nothing.
