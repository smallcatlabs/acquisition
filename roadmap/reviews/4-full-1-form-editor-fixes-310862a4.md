310862a4 — the fix-round done-when does not fully hold: recovery loses a text query, and stat holdability remains incomplete.

- **P2 — [page.rs:872](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/page.rs:872), shapes 2 and 3:** Clearing conflicting settings is available only in the structured editor, which cannot retain a text query. **Client input:** `search?realm=pc&text=class:rings&view.count=rarity&view.sort=ilvl`. The query box still carries its sort invisibly; clearing it through the structured form instead counts every item. The new test likewise drops `class:rings` and checks only status 200. **Smallest fix:** share the visible clearing controls with the query box, and assert that recovery preserves the query and its matches.

- **P3 — [fill.rs:158](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/fill.rs:158), shape 1:** Equality with `trim()` does not establish that a text input preserves an ID. **Client-supplied trade JSON:** `{"query":{"stats":[{"type":"and","filters":[{"id":"explicit.stat_329934\n7043"}]}]}}`. This passes the new check, but a browser strips the embedded newline, submitting a different ID without an omission notice; the submission helper preserves it and misses the failure. **Smallest fix:** include browser text-input sanitization in the holdability check, use the whole-stats fallback, and pin this boundary case.

- **P3 — [web.rs:1007](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/web.rs:1007), shape 10:** The count-save sentence now has a standalone wording assertion beside the existing `search_pages` golden. **Failing maintenance case:** rewording the note requires updating both pins. **Smallest fix:** make the note a required fact of the existing recorded count page and remove the standalone assertion.

- **Not checked:** gate results, test counts and browser execution are **not verified**, as requested; remote issue discussions were not supplied.
- **Nitpicks:** the embedded-newline ID is unlikely malformed input; the duplicate wording pin is maintenance-only.
- **Brief:** no additional impediment.
- **Plan:** the recovery pin should compare the query and answer, beyond successful status.
- **Prompt:** no impediment.
