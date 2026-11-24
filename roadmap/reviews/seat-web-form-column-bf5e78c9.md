bf5e78c9 — The issue’s static done-when requirements hold; fixture regeneration, browser tests, and gate exit status: not verified.

- **P3 — crates/acquisition-cli/tests/web.rs:1284 — Shape 11.** The new test comment records seat history (“The page seat’s fault” and the former behavior), contrary to the rule keeping history in records. No runtime input is involved. Describe the current preservation requirement instead.
- **P3 — Commit message:34 — Shape 1.** The message claims 11 query boxes lose the finder, but the committed golden diffs show 19 occurrences: 14 in `search_pages.txt` and five elsewhere. No client, persisted, or upstream input is involved; this is an evidence-count error. Record the corrected count and its unit in the next commit message.

- **Not checked:** Runtime and browser behavior, fixture-generation procedure, or gate execution, as execution was prohibited. The changed rendering, affected submission paths, and all seven golden diffs were inspected; no behavioral defect was found.
- **Nitpicks:** Both findings concern words alone and do not affect users’ searches.
- **Brief:** No obstacle.
- **Plan:** No separate plan was supplied or needed.
- **Prompt:** No change suggested.
