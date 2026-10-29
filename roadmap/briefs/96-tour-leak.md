# 96 — the tour's daemon is stopped when its test ends, failure included

Issue #96. The last step of the league group in "Pull-forward"
(`roadmap/README.md`), after #94's close: a test run that fails must not
leave a mock daemon alive for an hour holding its world's lock. The
rhythm is "A step".

## Goal

`readme_tour.rs`'s agent-recipe test starts its daemon through the
README's line `acq auth   # the same without a keyring` (`README.md:128`,
run by the harness's `login_with` with the idle knob empty, so the
daemon keeps C3's default, the hour idle exit) and stops it only at its
own last line, `acq daemon stop --json`. Any failure between the two
leaves that daemon running, reparented to pid 1, for the hour: what #88's
measurement runs found, four daemons under `acq-recipe-<pid>-0/store`.
Every other test's lazily spawned daemon idles out in the isolation
list's 30 s, which is why only this one was seen. When this step is
done the tour's test holds the guard #68 added to the CLI harness
(`stop_on_drop`, which runs `acq daemon stop` when the test ends,
failure included), and the guard checks, by the harness's own
accounting, that no daemon of the test's world is left: no endpoint
bound under the test's scratch runtime directory (C83, a daemon's
socket derives from its world into that directory and nothing else
binds there). A test that ends with one still bound fails, naming it.

## Read

- `AGENTS.md`: C122 (one harness, one isolation list; the process tests
  share it) is a cross-cutting ruling there, not in `decisions/daemon.md`.
- `roadmap/README.md` ("A step", "Pull-forward"); issue #96
  (`gh issue view 96 --json body,comments`; no comments).
- `decisions/daemon.md`: C3 (the hour idle default, the owner's word of
  2026-10-26; its *Pinned* names `readme_tour.rs`, the agent recipe),
  C10 (a mismatched daemon is replaced, never negotiated: `daemon stop`
  stops any daemon on the world, which is what lets the guard stop a
  stale one), C83 (the world, its socket under the runtime directory).
- `decisions/frontends.md`: no ruling governs the tour's test beyond C3's
  pin; the README tour line is the "Where things live" rule of
  `AGENTS.md`.
- `REVIEW-CHECKLIST.md`, "Protocol, client, daemon lifecycle, the
  world": "A test that owns what it does not clean" (guards that kill
  and wait on drop) and "A stop that returned early" (held by
  `provider_default.rs:204`,
  `a_stopped_daemon_is_gone_when_the_stop_returns`, which counts a
  daemon by `sockets_under(&scratch_tmp(base))`: the accounting this
  step reuses). "Frontends and the surface an agent meets": "A fix
  staging the nearest failure, not the named one" (the pin must fail
  the way the measurement runs did, after the daemon is up) and "A
  fixture that cannot see the default" (the pin asserts a daemon came
  up before it forces the failure, or it passes on nothing).
- The code:
  - `crates/acquisition-cli/tests/readme_tour.rs:308`,
    `the_agent_recipe_starts_a_daemon_that_idles_out_after_the_hour`:
    the scratch at 320, the login at 326, the test's own stop at 346.
  - `crates/acquisition-cli/tests/harness/mod.rs:121` to 137, `StopOnDrop`
    and `stop_on_drop`, added in `4f37d782` (#68's fix round, finding 1:
    "the tests that stop their daemon themselves are untouched", which
    is how the tour stayed outside it); its users are the four tests of
    `error_kind.rs` (`grep -rn stop_on_drop crates --include='*.rs'`).
  - `crates/acquisition-client/tests/harness/mod.rs`: `scratch_tmp`
    (138), `sockets_under` (464 Unix, 489 Windows: the live pipes of the
    worlds beside the runtime directory), `no_daemon_appeared` (522).
  - `crates/acquisition-client/src/client.rs:1578`, `stop`: a stop
    returns once the daemon's connection closes, which is when its
    process ends after its endpoint is removed, bounded by
    `STOPPED_WITHIN` (20 s, line 393; the daemon's own exit bound is 15 s
    with a 3 s grace before it ends at once, `daemon.rs:187` to 201).
- `python3 -I tools/bothy.py pack --path` for both test files routes to
  `daemon` and `frontends`; nothing there beyond what is cited above
  bears on a test's teardown.

## Input

Measured on this tree (`f5e270ba`), after `cargo build --workspace`,
the store and every daemon under the test's own scratch (the harness's
`acq-recipe-<pid>-0`), no `acqd` alive before the first run.

- Twenty passing runs, the test alone:
  `cargo nextest run -p acquisition-cli --test main -E
  'test(the_agent_recipe_starts_a_daemon_that_idles_out_after_the_hour)'`
  in a loop; after each, the `acqd` processes whose `ACQ_STORE_DIR` is
  a recipe world counted (`ps -axE`, filtered on `acq-recipe-`). Tally:
  20 passed, 0 failed, 0 daemons alive after any run. A passing run does
  not leak; there is no race in the success path on this machine.
- The failing run, #88's M2 staged: one comment line appended to
  `crates/acquisition-store/src/lib.rs`, the same command once (a `-p`
  run rebuilds `acq` and not `acqd`, so the spawned daemon is another
  contract), the file restored from a scratch copy, the workspace
  rebuilt. Tally: 1 run, failed in `login_with` (`acq auth` printed
  `daemon (pid …) is another contract (…), still, after a respawn`),
  1 daemon alive after it, its parent pid 1, its `ACQ_IDLE_SHUTDOWN`
  empty (the hour), its store `…/T/acq-recipe-<pid>-0/store`: the
  issue's observation exactly. Killed by this session, which started it.
- So the leak is the failure path, not a teardown race: the line that
  starts the daemon the test never stops is `README.md:128`, `acq auth`,
  and "never" is any panic before line 346. The tally above counted
  processes because the harness's accounting, the endpoints under the
  scratch, is gone once `Scratch` removes the directory; the guard runs
  before that removal, which is where this step puts the check.

## Do

1. **The accounting in the guard** (`crates/acquisition-cli/tests/harness/mod.rs`).
   `StopOnDrop::drop` runs `acq daemon stop` as now, then reads
   `sockets_under(&scratch_tmp(base))`. When the test is not already
   panicking, an endpoint left is a failure naming the endpoints and the
   scratch; while it panics, the endpoints are printed beside the
   failure and nothing more (a second panic in a drop aborts the
   process). The doc comment says the check and why it is the harness's
   accounting and not a process listing.
2. **The tour holds the guard** (`readme_tour.rs`). The recipe test takes
   `let _daemon = stop_on_drop(base);` after its scratch and before the
   login; its own `daemon stop --json` and the `stopped: true` assertion
   stay (the recipe's stop is part of what it shows). The login with the
   empty idle knob becomes one function, `recipe_daemon(base)`, so the
   pin starts its daemon exactly as the recipe does.
3. **The pin** (`readme_tour.rs`), named for the property:
   `a_daemon_the_recipe_starts_is_stopped_when_its_test_fails`. Inside
   `std::panic::catch_unwind`: the guard, `recipe_daemon(base)`, an
   assertion that one endpoint is bound (the daemon is up, so the pin
   can see the fault), then a forced panic. After it: the closure
   panicked, and no endpoint is left under the scratch. If one is, the
   pin stops it before failing, so a regression never leaks the hour.
   No `sleep`, no wall-clock bound, no process listing: the stop waits
   for the daemon's connection to close, bounded by the client's
   `STOPPED_WITHIN` (20 s) and returning success past it, so the
   endpoint check after it is what proves the daemon gone; the forced
   panic is the interleaving. Settled in the round (#96's review, the one
   finding; the brief first said the stop returns when the daemon is
   gone).
   Run once against the unfixed guard (the guard's `drop` reverted to a
   no-op in a scratch copy) and its failure reported, the report and not
   the history.
4. **After.** The M2 staging of Input run once more on the fixed tree:
   the test fails as before and no recipe daemon is alive after it.
5. The full gate (`tools/gate.sh`) after `cargo build --workspace`, in
   the background with its output under the session's scratch; one
   commit (1 to 3 are one concern: the guard's check, its first user
   outside `error_kind.rs`, and its pin). Then "Current step" as built,
   review pending, and the Codex review over the range.

## Done when

- `readme_tour.rs`'s recipe test declares `stop_on_drop` after its
  scratch and before `recipe_daemon`.
- `StopOnDrop::drop` checks `sockets_under(&scratch_tmp(base))` after
  its stop, failing a test that is not panicking when an endpoint is
  left.
- `a_daemon_the_recipe_starts_is_stopped_when_its_test_fails` is green,
  asserts a bound endpoint before its forced panic, and contains no
  `sleep`.
- The four `error_kind.rs` tests that hold the guard are green under
  its check.
- The commit message names `README.md:128`, `acq auth`, as the line that
  starts the daemon the test never stopped on failure.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- The other CLI tests that start a daemon lazily (through `login` or a
  use verb) and stop it only on success: their daemons idle out in the
  isolation list's 30 s. Raised in the report with the count, not fixed
  here; the structural form (a `login` that returns the guard, so no
  test can forget it) is a harness change for its own step.
- The frontend leaving a daemon it judged another contract running
  after its respawn (`still, after a respawn`): C10's behaviour, not a
  test's teardown. Raised, not changed.
- A process listing in any test: the issue rules it out.

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `96`;
the tests' own scratch is the harness's, under the temp directory, so
the socket path fits its cap. Kill only the processes you started. Any
`acq` is in a mock world. Nothing sends to GGG. Never push.

## Report

`TEMPLATE.md`.
