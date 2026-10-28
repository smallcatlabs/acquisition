`3a9a8442^..ec4c8166`: the fix-round done-when does **not** hold; whole-request recovery remains incomplete.

- **[P2] crates/acquisition-cli/src/search_cmd.rs:371 — Shape 3: CLI account recovery drops route-comparison evidence, although the MCP sibling now preserves it.** Given `--request route.json` containing `{"request":{"scope":{"account":"Typo","realm":"pc"}},"count":12}`, the refusal retains only `asked.request`; replay against an account holding one item silently omits `before.count: 12`, losing any `counted_at` too. Preserve the accepted envelope while changing its nested account, and pin CLI replay of a counted route.

- **[P2] crates/acquisition-cli/src/main.rs:1133 — Shapes 2 and 6: continuation offers for another account retain an incompatible basis and are refused again.** With populated Alice and Bob stores, run `acq --json --account Typo search --next <Alice-token>`: the Bob offer changes `scope.account` but retains Alice’s `basis` and row position, so resubmission returns `basis_changed`. Suppress incompatible continuation offers or offer an explicitly labelled restart with `basis` and `rows.after` cleared; replay every emitted offer in the test, which currently checks only `offers[0]`.

- **What I did not check:** execution evidence, including gate statuses and test counts, is **not verified**; no builds, tests, daemons, drivers or live calls were run, as requested.
- **Nitpicks:** neither finding; both affect recovery semantics.
- **Brief:** clarify how account recovery handles continuations whose basis binds them to another store.
- **Plan:** no additional obstacle.
- **Prompt:** no change needed.
