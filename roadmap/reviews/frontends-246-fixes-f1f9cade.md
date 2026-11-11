02b83466^..f1f9cade — The requested behavior and prior review fixes are only partially met; the done-when does not fully hold.

- **P1 — `crates/acquisition-mcp/src/server.rs:191`, shapes 3 and 7:** `tabs` still resolves the account separately for facts and policy, leaving another instance of the earlier account-selection race. For client input `tabs {}`, a concurrent change of persisted main-account intent from Alice to Bob after line 173 yields Alice’s tabs with Bob’s policy and `main_account` naming Bob. Resolve one `Account` and reuse it for facts, policy, and attribution; pin the consistent result.

- **P2 — `crates/acquisition-cli/src/run.rs:147`, shape 3:** `auth logout` bypasses main-account selection even though the updated account help promises that default. With persisted main account Alice and only Bob’s session live, client command `acq auth logout` sends no selector and logs Bob out, clearing his credentials. Use `cx.daemon_selector()?`, as `auth check` already does, and pin which account loses its session.

- **P2 — `crates/acquisition-cli/src/search_cmd.rs:636` and `crates/acquisition-verbs/src/saved.rs:999`, shapes 2 and 3:** Post-selection failures still lose main-account attribution; the shared cause is attaching metadata to successful results and selected error cases instead of carrying it through every subsequent failure. With Alice main and facts spanning two realms, client command `acq search --json rarity=rare` returns `realm_needed` without `main_account` or the stderr attribution line. Likewise, `saved_search_set {"name":"Rare","text":"rarity=rare"}` loses attribution if Alice’s persisted facts file cannot open. Pass the selected-account metadata through these error paths and pin both refusals. This remains a gap even with the broader CLI JSON-field work explicitly deferred.

- **P2 — `crates/acquisition-cli/src/store_cmd.rs:218`, shape 3:** The main-account JSON read turns an unavailable persisted file into an apparently unset default. If `user.db` has an unsupported schema or cannot open, `Frontend::accounts` records `main_unavailable`, but client command `acq accounts main --json` discards that field and successfully prints `{"main":null}`. Preserve the unavailable status in this document or return the structured read failure; pin that distinction from an absent main account.

Execution, gate results, test counts, and live/browser behavior: **not verified**, because the review was restricted to reading.

Nitpicks: none of the findings above; the concurrency case requires an overlapping write but produces a materially incorrect answer.

Brief/plan: The supplied issue has no explicit done-when list, so I assessed its requested behavior, the committed ruling, and the recorded review dispositions.

Prompt: No change needed.
