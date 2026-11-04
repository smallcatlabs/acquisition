a6c2b121 — the brief’s done-when does not fully hold.

- **P2 — [leagues.rs:86](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/leagues.rs:86), shapes 2 and 7:** An unreadable persisted policy can now prevent `price status` from answering. Concrete input: the sync-policy row contains malformed JSON; `Annotations::get` returns `Unreadable`, which this helper propagates, while `tabs` and stored leagues report `policy: unreadable`. Previously, pricing read only buyout rows. Handle the structured unreadable-row error in one shared policy reader, preserve its revision, and pin this persisted-input case.

- **P2 — [frontend.rs:395](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/src/frontend.rs:395), shape 1:** A failed facts read becomes a successful claim that the store holds nothing. Concrete input: readable intent beside a facts file stamped with a newer schema; `policy show` prints `held: no tabs or characters on disk in any realm`, and MCP returns an indistinguishable empty list. Keep the policy readable, but carry and render holdings as unavailable, with the read failure, instead of confirmed empty.

- **P3 — [documents.rs:1314](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/tests/documents.rs:1314):** The document walk still contains neither `LeaguesHeld` nor `LeagueListShown`, contrary to the explicit done-when requirement. Reading the table shows only the existing policy entry updated; the new documents have separate, stronger tests. Add their cases to the walk, or explicitly accept those existing tests as satisfying this contract.

Execution evidence, including the gate, test results, tour resolution and regeneration count: **not verified**, as requested. No builds, application runs, network calls or writes were performed.

Nitpicks: the third finding is a mechanical contract gap; the dedicated tests already check stronger document properties.

Brief: define how `policy show` represents unavailable facts and allow stronger document tests to satisfy the walk requirement. Plan: no additional obstacle found. Prompt: no change suggested.
