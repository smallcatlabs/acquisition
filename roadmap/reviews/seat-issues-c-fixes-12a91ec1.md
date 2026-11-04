aeab6f0c..12a91ec1 — the fix-round done-when does not fully hold: prior findings 1, 2 and 4 remain incomplete.

Findings, most severe first:

- **P2 — `crates/acquisition-cli/src/main.rs:1209` — shapes 2 and 6: request-based account remedies can still print commands that fail.** With Alice and Bob known, client input `acq search --request=request.json`, where the file contains `{"scope":{"account":"Missing"},"query":{"text":""}}`, prints commands selecting Alice or Bob while preserving `scope.account: "Missing"` in the file; both then fail with `account_mismatch`. The new test omits `scope.account`, hiding this case. Smallest fix: render the shared refusal’s rebound request, including its continuation reset, instead of merely replacing the command-line account; pin a request whose scope names an unknown account.

The following two store findings share one overlooked sibling reader: `leagues_held`.

- **P2 — `crates/acquisition-store/src/lib.rs:2226` — shapes 3 and 7: league holdings retain the old definition of live tabs.** With valid persisted input containing retired parent `m1`, retained substash `s1`, and live tab `k1`, `tabs.tabs` and status correctly report one tab, but the same tabs JSON’s `leagues_held` reports two. Other leagues’ orphan counts also reach CLI text. Smallest fix: apply the shared parent-aware predicate to this live-tab count and extend the retained-substash pin to league holdings. The documented planning/pricing snapshot distinction does not justify this count, whose contract explicitly says live tabs.

- **P2 — `crates/acquisition-store/src/corpus.rs:513` — shape 3: the tabs document still performs a facts read after its transaction ends.** CLI and MCP callers subsequently invoke `Held::with`, which queries `store.leagues_held` independently. A valid upstream listing committed then can produce `tabs: 2` beside `leagues_held` counting three tabs for that league. Smallest fix: include the holdings’ facts read in the store-owned snapshot and assemble the wrapper from those captured rows; pin a listing committed between these components.

What I did not check: execution, gate/doc-gate results, test counts, failing-before claims and shell execution are **not verified**, per the read-only instruction.

Nitpicks: none of the findings above.

Brief: no further obstruction.

Plan: no separate plan was supplied.

Prompt: the C54 replacement reads raw SQL, not the planner snapshot; describe that accurately—the separate existing planner orphan test still exercises snapshot retention.
