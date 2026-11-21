fff1881b — issue #304’s implementation done-when holds by inspection; execution-dependent requirements are not verified.

- **P3 — [page.rs:878](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/page.rs:878), shape 1:** “nothing carried hidden can make a page’s search refused” overstates the count-specific rule. Client input `search?view.sort=no%20such%20value` produces a refusal page that carries the invalid sort hidden; resubmitting either editor without a count refuses again, as the committed golden shows. Smallest fix: qualify the sentence to say that a count cannot be refused because of the carried sort, direction, or columns.
- Did not execute builds, tests, browser checks, fixture regeneration, or the gate, as instructed. No behavioral or invariant defect found by reading the changed paths and affected callers.
- Nitpicks: the P3 finding above is documentation-only.
- Brief: no obstruction. Plan: none supplied. Prompt: no change suggested.
