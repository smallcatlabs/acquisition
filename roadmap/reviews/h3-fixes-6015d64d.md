# Codex review: the h3 fix round, `48b62478^..6015d64d`

The outside review of the four fix-round commits (`roadmap/briefs/h3-fixes.md`), run 2026-10-26 by the committing session
(`codex exec --sandbox read-only`, the prompt `roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner reads them before any fix is built.

`48b62478^..6015d64d`: the brief’s done-when does **not** hold; execution-dependent criteria are **not verified**.

1. **P2 — [client.rs:1319](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1319), shapes 1 and 3:** both rewritten lifecycle pins still depend on the fixed startup budget, contrary to C137 and the claim that a starved runner cannot fail them. **Failing schedule:** let the parent exhaust 100 absent-socket polls before the spawned child gets CPU to refuse the lock or bind; the race test fails before entering its widened look, and the reap test fails before reaching its reaper assertion. The reap test also retains the harness’s 60-second handshake deadline. **Smallest fix:** expose startup expiry through a harness-controlled clock or budget, remove the remaining success-path timing dependencies from these pins, and test expiry separately.

2. **P3 — [client.rs:1278](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1278):** a newly spawned replacement inherits the startup polls already spent waiting for the losing child. **Failing schedule:** spend 99 polls, observe the loser’s exit, find an incompatible winner, then successfully stop it through an interactive door; spawning its replacement increments `polled` to 100 and immediately returns `SpawnFailed`, giving that replacement no opportunity to bind. **Smallest fix:** reset the startup budget whenever a new child is spawned, and pin this replacement schedule.

3. **P2 — [client.rs:1940](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1940), shape 1:** the new test’s environment-safety claim is false because the sibling stop test reads the environment without taking `ENV`. **Failing schedule:** under supported parallel `cargo test`, `a_stop_the_daemon_did_not_acknowledge_is_not_a_stop` creates its socket beneath the new `TMPDIR`; this test’s cleanup at line 1995 removes that directory before the sibling connects, producing `NotFound`. The concurrent environment reads also invalidate the unsafe mutation’s stated justification. **Smallest fix:** have the sibling test hold `ENV` throughout, and restore this test’s modified variables before deleting its scratch directory.

What I did not check: builds, tests, red runs, gate stages, or runtime/platform behavior, because execution was prohibited. The commits record red runs and stage exit statuses; their results remain **not verified**. Four fault-named tests and the review’s deletion/citation are present. I traced connection cancellation and child ownership through the look and reaper; I found no additional defect in those paths. I did not re-audit the unchanged GGG sender, choke point, or global gate. No goldens or reference files changed.

Nitpicks: none. Finding 2 requires an uncommon schedule, but it stops a reachable daemon and then reports failure without waiting for its replacement.

Brief: the whole-file lifecycle criterion also remains unmet by older waits such as [contract.rs:241](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/tests/contract.rs:241); align that criterion with the explicitly narrower scope claimed in `6a30b189`.

Plan: no change suggested.

Prompt: no change suggested.
