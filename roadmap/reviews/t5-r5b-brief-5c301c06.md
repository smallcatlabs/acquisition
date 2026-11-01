`roadmap/briefs/t5-r5b.md` at `5c301c06`, unchanged in the current tree, is **not buildable as drawn**.

Findings, most severe first:

1. **Do 1/2 — `Provider::ggg()` disconnects the daemon from the recording bait and configures real GGG destinations.** `provider.rs:61–66` hardcodes the production URLs; `daemon/tests/door.rs:561` currently derives the provider from the fixture’s base, as the old real-named test preserves at `daemon/tests/queue.rs:692–695`. An empty account-server record would no longer prove nothing was sent. **Smallest brief change:** construct a GGG-named provider from the fixture’s base inside `build`, using a provider factory or mode option.

2. **Do 1/4/5 — Reading the live queue file widens the scenario boundary beyond C158.** `decisions/daemon.md:25` permits journal and protocol answers; C45 at `decisions/daemon.md:31` explicitly describes disk observation “never while a daemon answers.” SQLite permits these second-connection reads (`acquisition-store/src/jobs.rs:150,226,450`), but that does not establish authorization under C158. **Smallest brief change:** retain persisted-state assertions in named component companions, or obtain and record an explicit boundary exception before building.

3. **Do 2 — The universal successful `sleep` submit cannot work on either GGG-mode row.** `daemon/queue.rs:1220–1223` refuses mock-only kinds in real mode. **Smallest brief change:** give the id-preservation check a provider-specific accepted submit; for GGG mode, use a live session and a supported kind, then cancel it before dispatch.

4. **Do 5, scenario 517 — `settle` before releasing the POST deadlocks until the hang ceiling.** `daemon/tests/door.rs:217–224` checks send occupancy before waking `settle`; its contract explicitly excludes held sends at `door.rs:639–648`. **Smallest brief change:** use `quiet` after the arrival and cancellation, or release immediately and settle afterward.

5. **Do 5, scenario 586 — Settling does not establish that the successful `whoami` is Done.** After refreshing, `daemon/kinds.rs:621` sleeps for 300 ms on Tokio’s real clock; the park hook can resolve `settle` while that job remains Running (`door.rs:221–225`). **Smallest brief change:** await that job’s terminal answer before the final settled-state assertions, and acknowledge the intentional real-time wait.

6. **Do 4, scenario 1369 — The recipe reads parent 6’s result before the cancellation that produces it, and misidentifies the leagues row.** The old test seeds probe 3, cancelling-running job 4, leagues job 5, and waiting child 7 (`daemon/tests/queue.rs:1398–1409`); parent 6 concludes only after `cancel(7)` (`queue.rs:1432–1449`). Cancelling-running job 4 becomes Cancelled during restore (`daemon/queue.rs:265–267`); child 7 remains cancellable. **Smallest brief change:** read leagues failure from row 5, check probe 3 absent, and place `Cancel { 7 }` before parent 6’s `Result`.

7. **Do 4, scenario 1728 — A failed reprioritization preserves the original priority.** `daemon/queue.rs:696–703` restores `before`; the old assertion expects 3 after attempting 9 (`daemon/tests/queue.rs:1732–1738`). **Smallest brief change:** replace “priority as set” with an explicit expectation of the original priority.

8. **Do 5, scenario 423 — The refusal message is not equal to the quote’s halt text.** Although `auth.rs:307` obtains `halted()`’s words, `daemon/refresh.rs:512` prefixes them with `"token refresh failed: "`; the old assertions use containment (`daemon/tests/rails.rs:447,461`). **Smallest brief change:** require each refusal to contain the quote’s full halt text, preserving the existing prefix.

9. **Do 5, ceiling table — The zero-send row cannot carry “ceiling: 1 of 1.”** `rails.rs:560–561` returns `"ceiling: 0 sends allowed"` for `max_sends: Some(0)`. **Smallest brief change:** make the expected halt words a row field.

10. **Do 2/3/6 — The prescribed replacements hold less than several tests being retired.** The admission table loses 1077’s positive twin—`submitted_by: "cli:1"` admitted as id 1, Done, exactly one GET (`daemon/tests/queue.rs:1104–1116`)—and 724’s explicit zero-second admission and completion (`queue.rs:739–745`). The restored-sleep table also needs 844’s exact equality with the client refusal (`queue.rs:849–859`); 872 additionally checks that its request fits a frame (`queue.rs:950–953`). **Smallest brief change:** name these assertions and their replacement scenarios explicitly before authorizing the C154 retirements.

11. **Done-when 2/3/6 — The textual checks contradict permitted code and match unrelated text.** `halted()` matches the retained scenario name at `daemon/tests/rails.rs:586`; `rails()` can match `reset_rails()`. Done-when 2 excludes the retained helper’s `Arc::get_mut` (`daemon/tests/queue.rs:1170`), while done-when 6 permits only reads despite requiring `JobDb::open` for seeding and retaining component writes (`queue.rs:1548,1600`). **Smallest brief change:** distinguish actual calls from identifiers, allowlist retained helpers/components, and distinguish setup writes from observation reads. The `logged_in(` pattern itself does not match `logged_in_sessions(`; the explicit exception for 1696’s polling `yield_now` is appropriate.

12. **Do 1/2 — The account bait does not literally record every method/path combination.** For `/token`, `daemon/tests/harness.rs:1425–1434` indexes grant fields before recording at `1444–1447`; a bodyless `HEAD /token`, for example, panics before entering `sent()`. `Door::finish` does not await account-server tasks (`door.rs:1008–1023`). **Smallest brief change:** narrow “any send” to well-formed daemon requests, or require the bait to record before parsing the grant.

What I did not check: execution, gate results, CI outcomes, or timing, because this was a read-only brief review. Timing remains report-only.

Nitpicks: finding 12’s overstatement and finding 6’s row-number typo in isolation. The sequencing, expected-value, coverage, and boundary findings are substantive.

**Round 5b may not start as drawn.**
