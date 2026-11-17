32f55034 — fix-round done-when does not fully hold: the matching regression is pinned, but the provenance correction leaves sibling claims standing.

- **P3 — `crates/acquisition-search/src/totals.rs:76`, shapes 1 and 3:** The loader documentation still says every changes row has captures asking for it; `tools/totals-table.py:17` repeats the same claim. Concrete reading: the committed `increased_rarity` carried row is described as captured upstream evidence although its evidence explicitly says “no capture asking.” Smallest fix: qualify both passages with the carried-by-stat-identity exception already added to the generated header and `source`.

- **Not verified:** Gate status, `tools/game-diff.py --check`, test results and measured counts; execution was prohibited. I read the diff, affected translation and generator paths, and changed golden, without accessing the owner’s store or live services.

- **Nitpicks:** The sole finding is documentation and provenance only; I found no further behavioural defect in the reviewed paths.

- **Brief/plan:** No change suggested for this fix round; the pending owner fetch and sitting are outside its done-when.
- **Prompt:** No change suggested.
