# Review of 56-list-paging fix round 1 at d9be0d31^..42051d1f

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

d9be0d31^..42051d1f — the fix-round done-when does not fully hold: P2 is fixed and pinned; P1’s implementation is fixed, but its resource bound lacks an effective pin.

- **[P2] crates/acquisition-daemon/src/daemon/queue.rs:793 — shapes 1, 2:** `list_rows_built` counts returned rows rather than materialized rows, so `c166_no_row_past_a_page_is_built` cannot detect the original memory failure. Concrete client/persisted input: the test’s `finished`, `limit: 1` request over three persisted jobs still reports one build if eager table reads return all three and discard two. Eager memory cloning likewise remains invisible. Count actual memory clones and table-row materializations, including discarded candidates, and retain the existing small-budget assertions.

- **Not checked:** Gate status, test counts and execution-dependent behavior are **not verified**; only source and Git records were read, as requested. No builds, tests, daemons, API calls, worker report or external CI conclusions were consulted.

- **Nitpicks:** None; the ineffective pin concerns the original P1 failure.

- **Brief:** No additional change suggested.
- **Plan:** No separate fix-round plan was supplied; the commit messages identified both fixes.
- **Prompt:** No change suggested.
