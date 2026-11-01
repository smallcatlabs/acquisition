`roadmap/briefs/time-residue.md` at `a7904b32`, unchanged in the working tree: **not buildable as drawn; Do 3’s reader still cannot establish zero “neither” from the prescribed edits.**

1. **Answered** — Polling the actual GET future once reaches the gate’s enqueue before its oneshot returns Pending; polling a spawned task’s handle would not.
2. **Answered** — `wait_all_terminal`’s 60-second bound joins the shared ceiling.
3. **Answered** — `start_daemon_with` applies its environment after isolation; the CLI wrapper exposes it to `new_league.rs:48`.
4. **Answered** — Four exit waits, five subscription reads and three retained polling intervals are correctly distinguished.
5. **Narrower, unsound** — One-level helper traversal handles `list.rs`, but misses existing deeper chains; helper documentation is not an accepted sentence source.
6. **Answered** — All eighteen constructions and five completion bounds are included; shared, spec-visible `HANG` resolves the healthy-path class.
7. **Answered** — The performance goal is conditional, the rehearsal’s required wait is retained explicitly, and the stop threshold includes fact failures.
8. **Answered** — Attribute-based counting fixes the original helper/function confusion; the new baseline has a minor scope error below.
9. **Answered** — Gate success is identified as execution evidence, with per-commit records, final verification and a close-message home.

Findings, most severe first:

- **Do 3 / done-when 3 — One-level traversal leaves indirectly constructed tests unclassified after the required constructor edits.** In `crates/acquisition-daemon/src/daemon/tests/`, [credentials.rs:649](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/credentials.rs:649) calls `restoring_daemon`, which calls `restoring_daemon_on` at `:568`, reaching `test_daemon` at `:575`. Likewise, [exit.rs:1227](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/exit.rs:1227) and `:1232` call a helper whose component sentence is at `:1194`, then pass through `exit_daemon` and `exit_daemon_on` before reaching the constructor at `:83`. These tests have no accepted sentence in their own or module documentation. **Smallest change:** replace the one-level limit with cycle-safe traversal of same-file helper chains, resolving forwarded `Boundary` arguments.

- **Do 3 / Input — The baseline excludes an attributed test inside the stated census scope.** The 196 tests across eighteen files omit [daemon/tests/door.rs:1367](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:1367); including every attributed test under `daemon/tests` gives 197 across nineteen files. **Smallest change:** correct the total and explicitly include `door.rs` in the reader’s scope.

- **Do 4 / Read — The ETA masking citation names the wrong harness.** The normalizer is [acquisition-cli/tests/harness/mod.rs:1113](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/harness/mod.rs:1113), not the client harness named at brief line 154. **Smallest change:** name and cite the CLI harness’s `eta_rule`.

- **What I did not check and why:** No builds, tests, project scripts, daemons or measurements were run, as requested. Live issue bodies and CI records were not independently verified. Static reading supports Do 4’s scaling and override paths; its stop rule can be followed, including reverting the experiment and gating the default-window fallback.

- **Nitpicks:** The census baseline total and ETA-mask citation. The helper-traversal gap affects completion.

- **May the step start as drawn?** No; repair Do 3’s helper-chain coverage first.
