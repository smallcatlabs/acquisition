`roadmap/briefs/4-0-door.md` at `71b89153`, unchanged in the tracked tree, is **buildable as drawn**, with one wording nitpick.

1. **Answered** — Goal and Read now build the recorded owner ruling; the criterion’s application to daemon polls belongs to step 2’s brief.
2. **Answered** — `LookFailed` preserves the third outcome, its transport-error propagation and the driver’s original typed error; Do 2 requires its row.
3. **Answered** — the two post-poll scripts answer separate consultations with expiry or pending; neither delivers an unsolicited startup deadline.
4. **Answered** — the initial child pid enters construction without `Spawned` or fresh arming; the driver retains the supplied deadline, starts with `Connect`, and preserves both seam tests.
5. **Answered** — harness lines 180–182 contain exactly the three cited knob settings.

The ruling is **consistent with the brief and registry**: the named composite-effect exceptions retain internal stop/look arbitration, while the machine receives their outcomes; C157’s pure lifecycle boundary remains intact. Hello expiry remains a machine verdict, and startup consultation preserves child-exit handling before expiry and the spawn-turn check.

Findings, most severe first:

1. **Do 1’s criterion — “their pins cost no real time” overstates the retained stop test’s independence from real time.** After the signalled endpoint lookup, the test removes the endpoint and awaits completion at [client.rs:2298](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:2298), through the real polling sleep at [client.rs:1686](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-client/src/client.rs:1686). **Smallest change:** describe these pins as forcing expiry and interleavings without relying on elapsed time, in Goal and the dated design paragraph. The explicit exceptions already make the build scope unambiguous.

No revised row contradicts Do 1 or Do 3; the new child, look-error, startup-script and harness citations match the tree.

What I did not check and why: no implementation, builds, tests, gate, executable probes, live traffic or current CI queries; this was the requested static confirming review.

Nitpicks: finding 1 only.

**Step 1 may start as drawn.**
