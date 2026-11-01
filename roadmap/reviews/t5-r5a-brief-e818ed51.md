[t5-r5a.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r5a.md) at e818ed51, unchanged in the working tree: not buildable as drawn.

- **Do 1–2; done-when 1 and 4 — `log_text()` widens the scenario boundary without a ruling.** `decisions/daemon.md:25` permits the journal and protocol answers; `daemon/tests/dispatch.rs:668–675` explicitly keeps log-file redaction as a component test. A second asker justifies sharing an accessor, but does not make its evidence cross the frontend boundary. Smallest change: keep T3’s log assertion in a component case, retain its other assertions in the token row, and adjust the component exceptions and required door methods.

- **Do 2–4 and 6 — several proposed holders hold less than the retired tests.** The shared cause is treating evidence of an input or a successful answer as evidence of the resulting token state. T7 explicitly checks that `rt-old` remains after failure (`daemon/tests/refresh.rs:438–442`); its proposed row checks only refusal, saves, mark and journal. Recording the first POST’s body cannot establish that postcondition. T11 checks both an empty credential store and the clear of `test-user` (`refresh.rs:616–620`), omitted from its recipe. T21/T24 check the actual restored access token (`refresh.rs:1215–1218`, `1333`), whereas a positional profile script accepts any bearer (`harness.rs:559–562`). Smallest change: explicitly retain those properties—T7 through a subsequent check carrying `rt-old`, T11 through its store and clear record, and T21/T24 through bearer checks at the fixture—or name retained component holders and permit them in done-when 1.

- **Do 3 — the combined landing recipes conflate different logout sequences.** T18 renames to `B#2`, then `C#3`, and logs out `C#3` (`daemon/tests/refresh.rs:1017`, `1038`); the brief instead requests `NewName#2`, which that scenario never creates. T17’s first logout must return `ErrorKind::Internal` with the printed retry, followed by a successful retry (`refresh.rs:963–992`), rather than the grouped “logout’s Ack.” Smallest change: specify each row’s final account name and distinguish T17’s first error from its retry’s acknowledgement.

- **Do 4 — T24 needs four scripted responses, while the shared recipe specifies three.** Its prescribed jobs send `[GET, GET, POST, GET]`; the shared script says GET, POST, GET. `scripted_server` asserts each response’s expected method immediately (`daemon/tests/harness.rs:557–561`), so the second job would fail that assertion. Smallest change: give T24 its own four-response sequence.

- **Done-when 7 — the stated comparison can never contain only test files.** `git diff --name-only 9736bdcf e818ed51` already contains `roadmap/briefs/t5-design.md` and `roadmap/briefs/t5-r5a.md`, exactly as the brief commit’s diff shows. Smallest change: check the implementation commits’ code diff separately, explicitly excluding the brief, review and close documents; retain the independent golden/reference comparison.

- **Do 1’s Read citations — some locators do not match the fixed tree.** `DoorBuilder::world` is at `daemon/tests/door.rs:380`, not 343; `TOKEN_HEADERS` is at `daemon/tests/refresh.rs:287`, not 283. Smallest change: correct those locators.

What I did not check: builds, tests, gate exits, timings, CI results or mutant artifacts; execution was prohibited. The timing requirement remains report-only.

Nitpicks: the locator corrections and the readily resolved T24 script inconsistency.

Round 5a may not start as drawn.
