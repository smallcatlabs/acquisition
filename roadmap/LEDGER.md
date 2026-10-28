# Roadmap ledger

Every closed step of the road in `README.md`, one row each, in order,
read from its newest end. A row is the step, its commit range (or its
rounds' ranges), its close date and the homes of what it left open:
an issue, a "Not doing" line, the owner's list under the README's
"Open", or the close message. The story is in the commits' messages;
the reviews stay under `reviews/`. A step's close writes its row; a
phase close folds its rows into one and adds the `HISTORY.md` line.

| Step | Commits | Closed | Left open |
| --- | --- | --- | --- |
| phase 0, the assessment and the week's fixes (0a to 0e) | `a987d02f..cca73395` | 2026-10-20 | #43, 0a's bound remainder; the synthesis's five corrected sentences stand in `STATE-2026-10-19.md` |
| phase 1, the governance reset (1 design, 1b to 1e) | `7470a3b8..0704a8f7` | 2026-10-20 | none in the tree; every park judged by 1b has its fate in `roadmap/governance/PARKS.md` at `0704a8f7` (the triage `roadmap/parks-triage.md` whole at `1f524d15`) |
| 2a the shared function layer | `05100056..3bc71a86` | 2026-10-21 | #49; #101, a text failure's stream |
| 2b the tests | `9501f750..8c10cdbe` | 2026-10-21 | #50 |
| 2c the journey test and real-shaped mock items | `4d2932fb..2b275eec` | 2026-10-21 | #51; the read-only open, ruled C124 and closed as #52 |
| 2d the daemon and limiter file splits | `dc993ddb..af5badde` | 2026-10-21 | its inputs taken by 2f; #100, the architecture rows |
| 2d-oracle the journal oracles over every journaling daemon | `e59ee842..1f524d15` | 2026-10-21 | #64, the login daemons' journal |
| 2e the game-diff tool, a new-league rehearsal and the README line | `08c0907a..64a6240c`; the remote-gate fixes `82199010..7b4793fe` | 2026-10-21 | #53; #101, `--if-revision 0`; the two claims for phase 3, in the live-run skill |
| 2f the review of daemon, store and network, and its fix rounds | `718f1cde..2aac17d7`; the reviews whole at `b1588df3` | 2026-10-23 | #51, #55, #56, #59, #61 to #72; #100; C150, the proxy variables |
| #70 the Windows gate's deadline flakes | `dd9b7859..32769f72` | 2026-10-24, the step; the issue on its week | #70, the watch |
| 4a the 4-lite design | `cbd7caf2..18351a2a` | 2026-10-25 | a selection compiled as a plan, in `briefs/4b-build.md`'s "Not in scope" |
| 4b 4-lite, `acq web` | `f29506bc..c3537ceb` | 2026-10-25 | #97, #98, #99; the league select's counts, "Not doing" in `decisions/frontends.md` |
| the honesty slice, h1 to h3 (C144 to C147) | `efc6e745..b3fc1ace` | 2026-10-26 | #11, #16, #63 in part; #74; #107; #108 |
| the temp-directory rounds (`reviews/tempdir-b350125b.md`, `reviews/tempdir-fixes-893677a3.md`) | `b350125b..39b4d9e1` | 2026-10-26 | the racing `private_dir` test's hang without a second thread, a nitpick, told in `39b4d9e1` |
| the readiness group, p0 and r1 to r5 (C3, C148), opened by the assessment `3249c5ad..69e37e48` | `f1a29952..c5a736db` | 2026-10-27 | #56; #87; #94; #101; #102; #103; #92 and #55, closed by the C135 round; the save cut at the exit's bound, to the time seam (`TIME-SEAM.md`); a selection plan's `notes`, "Not doing" in `decisions/plans.md` |
| the C135 round, the access token kept and the restore off the socket's path | `9a6a3c63..ffc3db83` | 2026-10-27 | #63, the 401 restart; #64, `settle()` |
| #82 the held corpus in `acq-mcp` | `500e4468..6750b4ef` | 2026-10-27 | #95; `show` over the held corpus and a per-account slot, declined in `6750b4ef` |
| #83 the MCP surface trimmed | `1dd3dfdb..c2095319` | 2026-10-27 | #104 |
| #84 the seat, the first row of `SEAT-LEDGER.md` | `799c39d9..71b12c9a` | 2026-10-27 | #9, #10, #11, #18; #97 |
| #88 one feature graph for the plain and the test build | `87dcb51a..e7653f87`; the build-order rule's second half retired at `a6a3b698` | 2026-10-27 | #96 |
| 2g the read economy and the refusal and receipt bundle (#9, #10) | `8e6e1853..5f62d548`, rounds 1 to 8 | 2026-10-28 | #11, #18, #56; #97; #104; #105; the live drivers, on the owner's list |
| #87 the gate refuses: a set fixture switch, a tree its stages changed, a test spawn past the isolation list | `241d5c5d..71a4f084`, one fix round, then the close commit after it | 2026-10-28 | #64, the daemon crate's tests; #70, the Windows tree check; the goldens' reads and the rest in the close message |
| #90 a job kind the store cannot place fails its job | `27ebe98c..03bd377d`, no fix round, then the close commit after it | 2026-10-28 | #109, the age-field flake the gate met; #65, the kind vocabulary; C144's sentence, for the owner in the close message |
| #68 the login callback: a task per connection under a read bound, a denial in the browser's words, a wrong-state request ignored, one browser bound with its knob, the CLI waiting on the daemon's flow | `942d2ee7..f95540f6`, one fix round, then the close commit after it | 2026-10-28 | the denial redirect's `state`, on the owner's list; the C137 knob list, for the owner in the close message; the keyring save's bound, C148's standing concern |
