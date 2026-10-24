# 70 — the Windows gate's deadline flakes

## Goal

The Windows gate stops failing healthy code on a starved runner. Today
most first runs fail on a wall-clock deadline — the handshake's 5 s,
the annotation wait's 5 s, a test's own 10 s process wait, or the CI
profile's 240 s termination under full property cases — so most merges
cost a re-run and a real Windows failure hides among the flakes
(issue #70 and its comments name the runs). When this step is done,
every timed test either forces its expiry deterministically or asserts
no number a slow machine can miss, the Windows gate runs the share
tier of the generated properties, and C137 (recorded before this
build) governs every future timed test. #70 closes only after a week
of pushes without a re-run; the commits say "part of #70".

## Read

- Issue #70 and its three comments: the flake census, run ids, which
  deadline each run hit.
- `AGENTS.md`: C122 (one harness, one isolation list, never retried),
  C116 (no id in user-facing text), the quality gate.
- `decisions/daemon.md`: C86 (the handshake deadline — its expiry
  behaviour is the contract), C137 (this step's ruling, committed with
  this brief), C133 (one exit; its 15 s bound is not this step's).
- `crates/acquisition-client/src/client.rs`: `HELLO_WITHIN` (declared
  near line 328; used in the refusal's words near 376 and in the
  `hello` timeout near 1339).
- `crates/acquisition-daemon/src/daemon/start.rs`,
  `idle_shutdown_from_env` (near line 8): the shape every knob copies.
- `crates/acquisition-store/src/annotations.rs`: `BUSY_TIMEOUT`
  (near line 154; the refusal's words near 494; `busy_timeout` calls
  near 590 and 685).
- `crates/acquisition-client/tests/harness/mod.rs`: `isolation()`
  (near line 151), the one isolation list every test process takes.
- `crates/acquisition-client/tests/contract.rs` (near lines 288–320):
  the wedge is already forced deterministically — a peer that accepts
  and never answers; the model for every expiry pin.
- `crates/acquisition-cli/tests/world.rs` (the wait helpers near lines
  60–140) and `crates/acquisition-cli/tests/provider_default.rs` (the
  no-pipe test near line 125): the test-authored waits this step
  widens.
- `.config/nextest.toml` (both profiles) and
  `.github/workflows/gate-windows.yml`, `gate-linux.yml`.
- `README.md`, the knob table near line 243 and the knob rule near 259.

## Input

Fixed before the work: the flake census in #70 (runs 39881432189,
39928762395, 39941875102, 39898854294: which test, which deadline,
the 20–46 s dilation measured); the owner's ruling of 2026-10-24
recorded as C137 with this brief; the owner's ruling that the Windows
gate runs the share tier. The failing inputs the new tests stage: a
knob set to zero seconds forces each deadline's expiry instantly.

## Do

1. `ACQ_HELLO_WITHIN=<secs>` in `client.rs`, the
   `idle_shutdown_from_env` shape: the default stays 5 s; the
   refusal's words print the effective seconds, not the constant.
   Pin: a `c137_` test in `contract.rs` sets the knob to zero against
   a healthy daemon-shaped peer and gets the unresponsive verdict at
   once — the expiry forced, no real waiting.
2. `ACQ_ANNOTATION_WAIT=<secs>` in `annotations.rs`, same shape:
   default 5 s; the busy refusal prints the effective seconds. Pin: a
   `c137_` test beside the `c35_` tests sets the knob to zero under a
   held writer and gets the busy refusal at once.
3. The isolation list (`isolation()`) sets both knobs to 60, so every
   test process inherits the allowance; the two `c137_` pins and any
   test that asserts the default's words set their own value after
   `isolate`.
4. Test-authored process waits rise to hang scale: every
   `Duration::from_secs(10)` deadline in the CLI tests' wait helpers
   (`world.rs` daemon-up and refusal waits, `wait_stopped`, and their
   kin found by grepping the CLI tests for `from_secs(10)`) becomes
   120 s. The no-pipe test in `provider_default.rs`: set
   `ACQ_IDLE_SHUTDOWN=600` on its command and widen its bound from
   15 s to 60 s — the discrimination stands because a held pipe now
   reads as the 600 s idle exit, which the default profile's 120 s
   termination kills as a hang; update the test's comment to say so.
5. `gate-windows.yml`: remove `ACQ_CASES: full` so Windows runs the
   share tier; rewrite the comment block: the Windows gate tests what
   Windows changes (named pipes, Credential Manager, spawn and handle
   semantics, the `cfg(windows)` code) and the platform-blind property
   volume stays at full on Linux. `ACQ_KEYRING_ROUNDTRIP=1` stays.
   `gate-linux.yml` and `.config/nextest.toml` do not change.
6. `README.md`: one knob row each for `ACQ_HELLO_WITHIN` and
   `ACQ_ANNOTATION_WAIT`, in the table's own form, each naming where
   it is read.
7. The gate, then commits per concern (the knobs and their pins; the
   test waits; the workflow; each message "part of #70", naming this
   brief).

## Done when

- The gate is green on this tree, each stage by its own exit status.
- C137 stands in `decisions/daemon.md` under 800 bytes (committed with
  this brief, before the build).
- `ACQ_HELLO_WITHIN` and `ACQ_ANNOTATION_WAIT` exist with defaults
  5 s, printed effective in refusals, each pinned by a `c137_` test
  that forces expiry with a zero knob.
- `isolation()` sets both knobs to 60.
- No CLI test wait helper holds a deadline under 60 s; the no-pipe
  test's comment states the new discrimination.
- `gate-windows.yml` sets no `ACQ_CASES`; its comment states the
  share-tier rationale; `gate-linux.yml` still sets `full`.
- The README knob table has both rows.
- Every commit message names this brief and says "part of #70", never
  "closes".

## Not in scope

- nextest test groups or thread caps: held until a week of pushes
  measures whether this step alone suffices (the roadmap's "Open").
- Any retry or serial re-run in the gate: declined; C122 stands; the
  rationale is `roadmap/TIME-SEAM.md`.
- The injected-clock retrofit of the client, daemon lifecycle and
  store: phase 4-0, designed after alpha.2 (`roadmap/TIME-SEAM.md`).
- The CI profile's 240 s termination and `STOPPED_WITHIN`: neither
  observed at fault once the share tier lands; leave both.
- `.github/workflows/release.yml`: untouched.

## Rules

`TEMPLATE.md`, and: this step builds in the main tree — no worktree —
because the session that launched it holds the roadmap files and
touches nothing this brief names; kill only processes you started.

## Report

`TEMPLATE.md`.
