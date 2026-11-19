`fc0f2f63` — the fix-round done-when does not fully hold: the prior findings are addressed, but toggle handling introduces a regression.

- **P2 — `crates/acquisition-cli/src/web/mod.rs:1813`, shape 1:** Any nonempty `toggle` suppresses item windows, although only a toggle naming a current place causes a redirect. **Failing input:** client request `tabs?realm=pc&league=Standard&open=t1&toggle=gone`, where persisted `t1` contains items and `gone` was removed by a refresh, renders `t1` open without its rows; characters share this cause. **Smallest fix:** skip windows only when the validated toggle will redirect, and test a retired toggle with another populated place open.

Not checked: tests, browser behavior, gate status and documentation build are **not verified**, because execution was forbidden; hosted CI and live GGG behavior were not inspected.

Nitpicks for unlikely cases: none.

Brief: no change suggested. Plan: no separate plan was supplied. Prompt: nothing impeded the review.
