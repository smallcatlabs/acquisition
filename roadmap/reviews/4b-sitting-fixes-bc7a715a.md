`a88e318e^..bc7a715a`: the brief’s done-when does not fully hold; execution-dependent conditions are **not verified**.

Findings, most severe first:

1. **P2 — `crates/acquisition-cli/src/web/page.rs:169`, shape 3:** Stat Filters still has no enable control, so the panel-enable fix remains incomplete. Enter a stat and minimum, then collapse the panel: its constraint remains active, with no way to disable it while retaining its values. Add a `query.stats[0].disabled` checkbox, admit its name in the fold, and pin the folded group.

2. **P2 — `crates/acquisition-plan/src/lib.rs:3926`, shapes 1 and 2:** The same-second regression test introduces a wall-clock bound forbidden by the brief and does not reliably stage its named case. A pause spanning two clock seconds between capturing `now` and reading the snapshot fails the assertion; crossing one second passes it without staging a same-second snapshot. Give the test snapshot a fixed `taken_at` equal to the staged fetch time, retaining real response ordering.

3. **P3 — `crates/acquisition-plan/src/lib.rs:1584`, shape 7:** Spend detection reads response identity from history but its timestamp from the current location, inventing epoch zero when that location is absent or revived. Compile a policy plan containing one tab fetch, fetch that tab, retire it through a listing, then apply the original plan: its spent entry carries `fetched_at: 0`, and the explanation reports an ancient fetch. Carry the timestamp belonging to the identified response and use that timestamp for the explanation.

4. **P3 — `crates/acquisition-plan/src/lib.rs:1500`, shape 3:** The public `unspent` documentation still describes the timestamp comparison this range replaces. Its statement that an earlier fetch in the snapshot’s second spends the action directly contradicts the corrected selection behavior. Replace the timestamp explanation with the response-watermark rule and update the adjacent spend-test description.

What I did not check and why:

- Builds, tests, the quality gate, browser execution, and live traffic were not run, as requested; their done-when conditions are **not verified**.
- Visual agreement with the owner’s capture and underlying journal counts are **not verified** from committed evidence.
- History-scan performance was not measured. Static tracing found no new rate-limit choke-point or global-gate bypass.
- Every changed golden was read; its changes correspond to source or test changes in the range.

Nitpicks: finding 4 is documentation cleanup. Finding 3 concerns an uncommon sequence, but reports false evidence; findings 1 and 2 are not nitpicks.

Brief: reconcile “three callers” with the two actual text-list callers, CLI and page; MCP accepts an array.

Plan: stage the same-second regression with fixed timestamps so its evidence matches its name.

Prompt: the commit-range notation was clear and caused no obstruction.
