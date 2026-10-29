# 86 — weekly automation: mutants over the limiter and the gate, Dependabot, a scheduled gate run

Issue #86, with the owner's comment of 2026-10-26 (the macOS Keychain
round trip). The second item of "Pull-forward" (`roadmap/README.md`)
after #85's close at `f0bfa17c`. The rhythm is "A step". This brief
stopped at Do 1, a ruling; it was settled under the owner's
delegation, (a).

## Goal

Three standing checks run without a push, and one platform's pass is
earned: a weekly `cargo-mutants` run over the rate-limit choke point,
the limiter, the policy parse and the send gate (`ratelimit/`,
`ratelimit.rs`, `gate.rs`), which mechanises what `TESTING-NOTES.md`
does by hand for invariants 1 and 2 ("Each has been broken deliberately
and seen to fail"); Dependabot for Cargo and for Actions; a weekly
scheduled run of the Linux gate, so a RustSec advisory or a moved runner
image surfaces without a push. And `keyring_round_trip_survives_a_fresh_entry`,
which passes on macOS by returning early, is run against the Keychain
somewhere a release can see it. Each mutant the first run leaves alive
says a test is missing: it is pinned as a test by one rule, or filed,
or named as equivalent with its reason.

## Read

- Issue #86 (`gh issue view 86 --json body,comments`) and its one
  comment, the owner's: "Either a macOS runner for that one test, or a
  line in the release checklist that the owner runs it with the
  variable set before a tag".
- `roadmap/STATE-2026-10-26.md:65`, R5: the evidence for all four.
- `AGENTS.md`: invariants 1 and 2; C122 (a behaviour pinned once, at
  the lowest crate that holds it); "The gate decides".
- `TESTING-NOTES.md:94-105`: the journal invariants and "Each has been
  broken deliberately and seen to fail"; the mutants run is that
  sentence on a schedule, over the daemon's own tests.
- `REVIEW-CHECKLIST.md:11-21`, "Network: the limiter, the gate,
  OAuth": the shapes a surviving mutant is read against. The checklist
  has no process section.
- `.github/workflows/gate-linux.yml`: the trigger and concurrency
  (12-19; the group is the workflow and the ref, cancel-in-progress);
  the Secret Service step (56-59); the test step (75-82); every step
  after it under `!cancelled()` (89-126); the `breakers` job (137-164),
  whose steps carry no condition, so the first red breaker hides the
  rest. `gate-windows.yml`: its one step after the test step, clippy,
  under `!cancelled()` (70-75). `release.yml`: the macOS legs of
  `build` on `macos-latest` (67-69); `draft` needs `build` and
  `sign-macos` (193-195).
- Readiness step r1 (`briefs/r1-windows-gate.md`, Do 3; commit
  `de6ad867`): did the `!cancelled()` part on both gates' `gate` jobs.
  What is left of the issue's sentence is the `breakers` job.
- `deny.toml`: cargo-deny runs on the Linux workflow only, so the
  scheduled run is what makes advisories surface without a push.
- `crates/acquisition-daemon/src/auth.rs:581-611`,
  `keyring_round_trip_survives_a_fresh_entry`: returns before any
  assertion unless `ACQ_KEYRING_ROUNDTRIP` is set; its doc names the
  Linux and Windows workflows that set it.
- The mutated code: `crates/acquisition-daemon/src/ratelimit.rs`,
  `ratelimit/choke.rs`, `ratelimit/limiter.rs`, `ratelimit/policy.rs`,
  `gate.rs`; the tests that hold it, `ratelimit/tests/spec.rs` and the
  daemon's other unit tests (`cargo nextest run -p acquisition-daemon`,
  305 tests, 3.1 s warm on this machine).
- `rust-toolchain.toml` ("nothing floats"; 1.99.0) and
  `.config/nextest.toml` (never retried): what Dependabot must not
  float and what the mutants run's test tool reads.

## Input

Measured on this brief's tree (`f0bfa17c`), before the work:

- The scope: `cargo mutants --list` over the five files names 388
  mutants (`gate.rs` 31, `ratelimit.rs` 10, `choke.rs` 90, `limiter.rs`
  172, `policy.rs` 85). cargo-mutants 27.1.0, installed with
  `cargo install --locked`.
- The Linux gate's Secret Service step (`apt-get update`, then
  `install gnome-keyring dbus`) over the last seven green push runs
  (40807489931 back to 40728020538, step times from `gh run view
  --json jobs`): 11 to 19 s, of a `gate` job of 462 to 556 s. The
  issue's "3.5 minutes of a 12-minute job" is stale: there is nothing
  worth caching, and the step stays as it is.
- The first mutants run's outcome (time, caught, missed, timeouts,
  unviable) is the build's own measurement (Do 3), run alone on this
  machine.

## Do

1. **The ruling** (the owner's; recorded in its own docs commit,
   quoting him, before any code). Where the macOS Keychain round trip
   is earned. The repository is public, so hosted runners, macOS
   included, cost the account no minutes; what each choice costs is
   wall time and what it blocks.
   - **(a) A `keyring-macos` job in `release.yml` that `draft` needs**
     (recommended). On `macos-latest`, `ACQ_KEYRING_ROUNDTRIP=1`, the
     one test through nextest by name. It runs on every release
     dispatch and every `v*` tag, and a tag's draft is not written
     until it passes: the pass is earned on the commit that ships, on
     the platform the archive names, with no line for anyone to
     remember. Cost: one more macOS job per release (a debug build of
     the daemon's tests; its time is seen only on the first dispatch),
     and a release that waits on the Keychain. It changes what blocks a
     release, which is why it is a ruling.
   - **(b) The same job in the weekly workflow** (Do 4), on schedule
     and dispatch. Earned within a week of any commit and blocking
     nothing; a tag can ship a week's commits it has not seen.
   - **(c) A release-checklist line the owner runs** with the variable
     set before a tag. No release checklist exists in the tree today
     (`release.yml`'s header is the procedure), so this creates one; it
     is the only choice that runs against the owner's own login
     keychain rather than a runner's, and the only one a person can
     forget.
   - Whichever is chosen, the test's doc comment names where macOS
     runs it, beside Linux and Windows.

   Settled: (a). Taken by the orchestrating session under the owner's
   delegation on 2026-10-28, quoted in `roadmap/README.md`, "The
   owner's rulings for this road"; no registry entry.
2. **The `breakers` job** (`gate-linux.yml`): `if: ${{ !cancelled() }}`
   on every breaker after the first, so one red breaker hides none, as
   r1 did for the `gate` jobs; nothing else in the two gates is left of
   the issue's sentence.
3. **The first mutants run, measured.** `cargo mutants` over the five
   files with the daemon package's tests through nextest, run alone on
   this machine after `cargo build --workspace`: wall time, the
   counts caught, missed, timed out and unviable. Each missed mutant
   is read for what it says: a missing assertion pinned as a named
   test in `spec.rs` or beside the code it guards, in its own commit,
   one rule per shape (never one test per mutant where one property
   holds several); a mutant whose behaviour is equivalent named as
   such with the reason; one this step cannot settle filed as an issue
   with the mutant's line. A missed mutant that the workspace's other
   tests catch (`--test-workspace=true` over the survivors alone) is
   named in the report, and its pin still belongs at the lowest crate
   (C122).
4. **The configuration and the weekly workflow.**
   `.cargo/mutants.toml`, read by cargo-mutants by default, holds the
   scope (`examine_globs`), the test tool and the exclusions, each
   exclusion (`exclude_re`) with a comment naming its reason or issue,
   so a local `cargo mutants` and the workflow run the same set.
   `.github/workflows/weekly.yml`, on a weekly `schedule` and
   `workflow_dispatch`: a `mutants` job (the pinned toolchain,
   nextest and cargo-mutants pinned by version, `cargo mutants`, the
   `mutants.out` directory uploaded as an artifact whatever the
   outcome), sharded only if the measured time says one runner cannot
   finish inside its timeout. The job fails on a missed mutant, so a
   week that lets a mutant live is red until it is pinned or listed; a
   timeout is a mutant that hangs the suite, which is the suite catching
   it, and is listed in the artifact without failing the job. Settled in
   the build: the brief first said a timeout fails it too.
5. **The scheduled Linux gate.** `gate-linux.yml` gains a weekly
   `schedule` (an hour apart from the mutants run), and its concurrency
   group gains the event name, so a push to `main` and the scheduled
   run of `main` no longer cancel each other. The workflow is reused,
   not copied: one gate, read in one place.
6. **Dependabot.** `.github/dependabot.yml`: `cargo` at the root and
   `github-actions`, weekly, each ecosystem's minor and patch updates
   in one grouped pull request so a week costs the owner at most one
   PR per ecosystem plus any major. `rust-toolchain.toml` is not an
   ecosystem Dependabot reads, so the pinned toolchain stays a
   deliberate commit; the actions stay pinned by major tag, which is
   what Dependabot bumps.
7. The full gate (`tools/gate.sh`) after `cargo build --workspace`, in
   the background, output under the session's scratch; one commit per
   concern (the ruling, the breakers' condition, each survivor's pin,
   the configuration and workflow, the schedule, Dependabot, the
   keyring job); then "Current step" as built, review pending, and the
   Codex review over the range.

## Done when

- `roadmap/README.md`, "The owner's rulings for this road", holds the
  owner's word on Do 1, quoted, in a commit before any code commit; the
  keyring round trip runs on macOS where he ruled, with the variable
  set, and its doc comment names that place.
- Every breaker step of `gate-linux.yml`'s `breakers` job after the
  first carries `!cancelled()`.
- `.cargo/mutants.toml` scopes cargo-mutants to the five files, and
  `cargo mutants --list` over it names the mutants the first run
  tested less those it lists as excluded; every exclusion in it carries
  its reason or an issue number, and a tie between two instants is
  excluded only when both arms leave the limiter in the same state and
  nothing carries the difference forward. Settled in the round (#86's
  review).
- The commit that adds the configuration states the first run: the
  cargo-mutants version, the wall time on this machine, and the counts
  caught, missed, timed out and unviable; every missed mutant is a
  named test in its fix's commit, an exclusion with its reason, or an
  issue named in the close.
- `.github/workflows/weekly.yml` runs the mutants job on a weekly
  schedule and on dispatch, uploads `mutants.out`, and fails on a
  missed mutant.
- `gate-linux.yml` runs on a weekly schedule, and its concurrency group
  names the event.
- `.github/dependabot.yml` names `cargo` and `github-actions`, weekly,
  grouped.
- What only a push can show (the first scheduled and dispatched runs,
  the macOS job, Dependabot's first PRs) is named in the close with how
  the owner checks it.
- The gate is green on the step's tree, each stage by its own exit
  status.

## Not in scope

- Mutants beyond the five files (the frame reader, the auth flow, the
  store): a later widening, filed if the first run argues for it.
- A cargo-mutants stage in the local gate: the run is minutes to hours,
  the gate is a minute.
- Pinning actions by SHA: Dependabot keeps the major tags current; a
  SHA pin is a separate supply-chain choice.
- Repository settings (Dependabot security alerts, branch protection):
  the owner's, in the GitHub UI.
- Running any workflow on GitHub: that needs a push, which is the
  owner's.

## Rules

`TEMPLATE.md`. The step runs in the main tree on `main`, one session;
scratch under this session's scratchpad in a subdirectory named `86`.
A mutants run is long: it runs in the background, alone, with no other
build or gate beside it. Kill only the processes you started. No `acq`
runs; nothing sends to GGG. Never push.

## Report

`TEMPLATE.md`.
