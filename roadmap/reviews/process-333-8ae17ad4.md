8ae17ad4 — the issue’s done-when does not fully hold: emitted byte counts can be wrong, and retired-only citations are misreported.

- **P2 — tools/bothy.py:877, shape 1:** The footer counts UTF-8 bytes with LF endings, but `print()` uses stdout’s encoding and newline translation. Concrete case: redirecting `pack --area store` under native Windows Python with UTF-8 stdout emits CRLF, making the stated count smaller than the output. The fix must count exactly the bytes emitted, including the footer and newline, and pin that property.
- **P3 — tools/bothy.py:947, shape 1:** Filtering retired entries out of `cited_by()` makes the new answer falsely equate “no active citations” with “no citations.” Concrete repository input: `pack --path tools/committer-check.sh` says it cites no ruling, although it cites retired C186. Keep retired entries omitted from packs, but distinguish retired-only citations from none and name their area or holder.

- **Not checked:** Execution, breaker results, the gate and claimed measurements were not verified; this was a read-only source review. No frontend requests, store data or GGG responses are involved in this change.
- **Nitpicks:** The retired-citation finding is wording-only; its input already exists in the repository.
- **Issue:** No obstacle; the acceptance criteria were concrete.
- **Plan:** None supplied.
- **Prompt:** No obstacle or suggested change.
