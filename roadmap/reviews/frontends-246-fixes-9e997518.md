02b83466^..9e997518 — The requested behavior and review fixes are partially met; the done-when does not fully hold.

The two P2 findings share a cause: account attribution is reconstructed in adapters rather than carried through the operation’s failures.

- **P2 — `crates/acquisition-mcp/src/server.rs:824`, shapes 3 and 7:** Saved-search creation resolves the account once for attribution and again inside `saved::resolved_realm`, reintroducing inconsistent account selection. For client input `saved_search_set {"name":"Rare","text":"rarity=rare"}`, a concurrent persisted main-account change from Alice to Bob between those reads, followed by failure opening Bob’s facts, returns Bob’s failure attributed to Alice. The CLI repeats this pattern at `crates/acquisition-cli/src/saved_cmd.rs:80`. Resolve once inside the verb and carry that choice through its receipt and every subsequent failure.

- **P2 — `crates/acquisition-cli/src/jobs_cmd.rs:106` and `crates/acquisition-mcp/src/told.rs:152`, shape 3:** Job-submission refusals other than `not_logged_in` still lose main-account attribution. With Alice main and logged in, client command `acq stash missing --league Standard`, where the persisted listing excludes `missing`, returns the admission refusal before `note_main`; the equivalent MCP submission returns a JSON-RPC error that bypasses `marked`. Preserve attribution on every failure after selection, including JSON-RPC error data, and pin this ordinary admission refusal on both adapters.

- **P3 — `crates/acquisition-cli/src/store_cmd.rs:230`, shape 1:** The bare main-account display claims one account exists when none is known. Client command `acq accounts main` against a fresh world prints “the one account known is taken” because the empty result from `main_line` represents both zero and one accounts. Branch on `accounts.total` and give the zero-account case its own wording and login action.

Execution, gate results, test counts, and live/browser behavior: **not verified**, as requested; uncommitted working-tree changes were excluded.

Nitpicks: the P3 empty-world wording; neither P2 finding.

Brief/plan: The issue has no explicit done-when list, so I assessed its requested behavior and the recorded review dispositions.

Prompt: No change needed.
