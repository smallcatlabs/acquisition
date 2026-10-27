460b6a25^..a01e748e — the fix-round done-when is not fully met.

- **P2 — [shop_cmd.rs:346](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/shop_cmd.rs:346), shape 6:** Both shop remedies now pass unsupported flags to `price set`. After rendering league Big, filling the placeholders produces `acq price set --account 'Alice#1234' --league Big item/i-b00 exact 3 chaos`, which clap rejects; nondefault realms similarly add unsupported `--realm`. Price writes are league-less and accept neither flag. Print only the account for writes, and pin a remedy with its placeholders filled; the new sweep currently skips these commands.

- **P2 — [price_cmd.rs:250](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/price_cmd.rs:250), shapes 1 and 6:** Suppressing `--league Standard` does not always preserve the resolved scope, leaving prior finding 1 incompletely fixed. With `tab/pc/t1` recorded live in Standard and Big, showing twelve Standard items with explicit `--league Standard` prints audit routes without that flag; both routes then fail because `league_of` refuses multiple leagues. Always include the resolved league in printed read commands, and pin this case.

- **P2 — [price_json.rs:1439](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/price_json.rs:1439), shape 2:** Prior finding 2’s unresolved-item remedy is corrected but remains unpinned: the regression test stages eleven conflicts and no unresolved items. Removing `--expand` solely from the unresolved branch would escape these assertions. Add eleven unresolved items, run their printed status remedy, and assert that it returns all eleven.

Execution, including gate status and test counts: **not verified**, because execution was prohibited.

Nitpicks: none; the second finding requires the less common case of a tab recorded live in multiple leagues.

Brief: distinguish the scope flags supported by reads from the account-only scope of writes.

Plan: no separate implementation plan was supplied.

Prompt: no changes suggested.
