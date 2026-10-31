c0594d77 — the brief’s done-when does not fully hold: one existing rule loses reach.

- **P2 — roadmap/briefs/TEMPLATE.md:71, shape 1:** The session-close skill is narrower than the deleted “touch nothing in the tree while the gate runs” rule. A worker can create or edit an untracked, nonignored file during `tools/gate.sh --docs` without violating the skill’s “edit nothing tracked”; that gate selection has no `tree` stage either. The commit’s claimed duplicate therefore does not preserve the original prohibition. Restore the original sentence in the template.

- **Not verified:** gate execution and the report’s sentence-classification table; execution was prohibited and the report was withheld. Unrelated Rust input paths were outside this prose-only diff. The owner’s recorded rulings, template pointers, headings, report sections, and committed byte counts otherwise check out.

- **Nitpicks:** none.

- **Brief, plan, prompt:** no changes suggested.
