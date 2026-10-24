# Time-seam notes: the #70 investigation and the design for phase 4-0

Provenance: Fable (`claude-fable-5`), one session with the owner on
2026-10-24, read-only until the #70 step it launched. This file is
phase 4-0's first input, in the manner of `GUI-NOTES.md`: dated
evidence and a design proposal, never a ruling. The rulings it led to
are C137 (the deadline's contract is its expiry behaviour,
`decisions/daemon.md`) and the owner's road reorder of 2026-10-24
(`roadmap/README.md`). It is written now, while 2f's review of the
daemon, store and network is warm, for a design session that runs
after `alpha.2`.

## 1. The census behind #70

The Windows gate's reds of 2026-10-22 to 24 (the runs are named in
issue #70 and its comments) were four different things wearing one
colour, plus two real bugs among them:

- **Production deadlines firing correctly.** The handshake's 5 s
  (`HELLO_WITHIN`, C86) and the annotation wait's 5 s
  (`acquisition-store/src/annotations.rs`). On a starved runner the
  daemon genuinely did not answer within 5 s; the code did what C86
  promises. The test's hidden premise — the machine is healthy enough
  that 5 s means wedged — is what failed.
- **Test-authored waits**: the CLI tests' 10 s daemon-up and refusal
  budgets, the 15 s pipe-read bound.
- **The runner ceiling**: the ci profile's 240 s termination, sized as
  the slowest honest test on the development machine (26.5 s) times
  nine. Observed dilation on the Windows runner was 20 to 46×, so the
  ceiling was under water at observed dilations, independent of luck.
- **Performance cliffs**: `took < 2 s` asserts on operations measured
  under 1 ms on the development machine. None flaked; the margins are
  ~2000×. They are the same kind of bet and are listed here so the
  next reader knows they were seen. So is the quote's own 5 s bound
  (`QuoteNote::Timeout`), which the #70 build left as it was: not in
  the census, same kind of deadline.

The two real bugs: the rails write fault (Windows-only, fast-failing,
fixed in 2f round C) burned four consecutive reds before it was
believed; a genuine search property fault
(`generated_cross_checks::…_2`, a fresh seed's minimal
counterexample) surfaced inside the same red as a c83 flake. The cost
of flakes is not minutes; it is that real findings arrive in the same
channel and are adjudicated by a tired human.

Linux over the same window: zero time flakes. The platforms differ in
dilation (process spawn, Defender, filesystem), not in behaviour.

## 2. Why repricing cannot converge

Every timed wait is an independent bet that scheduling dilation stays
under that wait's margin, and a run passes only if every bet wins.
The suite makes hundreds of such bets per run; the dilation
distribution is heavy-tailed; so fixing the currently loudest bet
(one budget raised, one fixture made non-blocking, one WAL retry)
only promotes the next-tightest margin. The history is exactly that
sequence. C137 ends it by rule: no test asserts a wall-clock bound a
slow machine can miss, and a deadline is pinned by forcing its
expiry.

A second conflation made it worse: "the gate decides" plus "never
retried" assumes every red encodes information about the code. On a
machine the project does not control, a red sometimes encodes
information about the machine, and the gate had no verdict for that —
so adjudication fell to the owner's finger, an unrecorded,
indiscriminate human retry, which is the thing the no-retry rule was
written to prevent.

## 3. What landed now, what was declined, and why

- **Landed (the #70 step)**: C137; `ACQ_HELLO_WITHIN` and
  `ACQ_ANNOTATION_WAIT` widened by the isolation list, each pinned by
  forcing expiry with the knob at zero; the test-authored waits raised
  to hang scale; the Windows gate on the share tier of the generated
  properties, since the Windows gate exists to test what Windows
  changes and the platform-blind property volume stays at full on
  Linux.
- **Held: nextest test groups** (capping the heavy property tests'
  concurrency). The share tier removes most of the self-starvation; a
  week of pushes measures whether anything is left. Evidence before
  ceremony.
- **Declined: a structured adjudication step** (re-running only the
  expiry-family failures serially and gating on that). It is
  defensible — a second measurement with the confound removed, not a
  retry — but it is mechanism, and if the landed changes work, reds
  become rare enough that the plain rule stands. Revisit only if the
  week of pushes fails, and bring this paragraph to that discussion.

## 4. The seam: what phase 4-0 builds

The daemon's GGG path already has the pattern: `choke.wall()` is an
injected clock, and the limiter's ETAs are fuzzed against a
simulation that keeps every send. 4-0 extends that seam to the
lifecycles the census named, so their temporal properties are tested
in logical time — deterministic, instant, exact — and the remaining
cross-process tests assert only that the real wiring eventually
works, bounded by the hang ceiling alone.

Candidate machines, in the order the evidence earned:

1. **The client's connect-and-handshake**: discovery, connect, hello
   under its deadline, the mismatch verdicts (C10, C84, C86), the
   spawn-or-report decision (C88). Today its deadline is real time
   (`tokio::time::timeout` in `client.rs`); under the seam the c86
   expiry pin stops costing five real seconds.
2. **The daemon's lifecycle**: start, listening, idle verdict, drain,
   one exit (C133, C134). The little poll sleeps — `IDLE_POLL`,
   `DRAIN_POLL`, `HALT_POLL`, the accept pauses — become transitions
   of one machine over one clock. This is also where two rows already
   under "Open" live: the lock merge (the keyring save stalling under
   `shared` is a lifecycle-machine question), and possibly the typed
   `JobKind`.
3. **The store's annotation wait**: the two-writers race (C35)
   determinised in-process with two connections; the cross-process
   twin shrinks to a smoke test that the OS file lock exists.
4. **The harness's logical-time runner**: how a test drives a machine
   through injected ticks; one shape shared by the three above.

Explicitly not in 4-0: the GUI's subscriber and reconnect — they are
4-full's, and the point of 4-0 is that they are *born* on the seam,
not retrofitted. The design rule already binds them (C137 and the
template's Rules line).

## 5. What the league run should feed this design

Phase 3 is upstream evidence, which is why 4-0 sits after it. Watch
for and record: real 429 holds and their durations against the
limiter's predictions; any reconnect or re-login the fortnight
forces; the longest honest wall-clock wait a live session meets; and
every place a human judged "slow or stuck" during the league — each
is a state the machines must name.

The league is also the same disease at another calibration: the
tables and pins are constants calibrated against a game snapshot, and
launch is the world moving. The cure rhymes with C137 — "the world
moved" as a first-class, measured state (the undecided classes and
untranslated mods counts of phase 3), never a refusal. The 4-lite
build brief carries the user-facing half: every unknown-game-content
path degrades to display with a badge.

## 6. Open questions for the 4-0 design session

- Does the client's machine share the daemon's clock trait, or does
  each crate own a minimal one? (C1's edges constrain where the trait
  lives; the protocol crate links serde alone.)
- How does logical time cross the process boundary in the few tests
  that must stay cross-process — not at all (they assert no numbers),
  or through a journal the machines emit?
- Which of the "Open" architecture rows does 4-0 absorb (the lock
  merge is earned; the typed `JobKind` and `frame.rs` sharing are
  candidates), and which stay deferred?
- Whether the perf cliffs move to a dev-machine-only lane or stay as
  wide complexity cliffs.
