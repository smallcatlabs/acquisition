`roadmap/briefs/poe2-prep.md` at `9df29bc6`: a builder needs the first-contact success criterion clarified before building as drawn.

1. **Decisions 7 / Done when — changes what gets built.** The checklist requires handling failed contacts but never states which results permit the merge. A checklist could record a 404 or a store-refused body, prescribe no retry, and proceed to “Then the merge,” satisfying the listed requirements without establishing usable stash support. **Smallest change:** require successful contacts and accepted store readback before merging; otherwise stop and leave the branch unmerged, with launch-day repairs outside this preparation step.

2. **Decisions 7 — changes what gets built.** Contact coverage is specified by route label, although documentation coverage is specified by URL shape. In `crates/acquisition-daemon/src/daemon/routes.rs`, ordinary-tab and substash URLs both use `stash/poe2`; listing plus ordinary-tab contacts satisfy the checklist while leaving substashes unsampled. **Smallest change:** specify whether each rendered shape needs a contact, or explain which sample suffices for both tab shapes. Also say what happens when the listing supplies no suitable tab or substash.

3. **Find first / Decisions 6 — wording.** The prose sweep misses a current claim that meets its amendment criterion. Running the supplied grep excludes `tools/`, while `tools/tracer-rung.sh:265` says “stashes are PoE1 only”; the following code deliberately gives mock PoE2 runs a character-only policy. **Smallest change:** include `tools/` in the search, amend the comment to describe that chosen mock policy, and explicitly exclude changing the driver’s policy behavior from scope.

4. **Find first / Decisions 5 — wording.** The discovery sequence omits the required workspace build before its daemon-spawning test suite. `AGENTS.md` requires `cargo build --workspace` before anything starts a daemon, while the brief directs the builder to flip the arm and run nextest. **Smallest change:** put `cargo build --workspace` immediately before that discovery run.

5. **Done when, second bullet — wording.** The baseline refusal test is named only in the report, which a commit-only reviewer does not receive. The existing evidence includes `crates/acquisition-daemon/src/daemon/tests/sessions.rs`, which asserts refusal and an empty job table, but the done-when does not require its identifier in the close. **Smallest change:** require the baseline commit and test identifier in the close message alongside the branch’s acceptance test.

6. **Find first, `accepts(` search — wording.** The command does not find “every reader of the table.” Running it misses the daemon’s consumers through `Family::realm_of` in `daemon/routes.rs:46` and `daemon/queue.rs:1400`, and includes the unrelated parser-test name `c91_every_reading_an_error_shows_is_a_text_the_parser_accepts`. **Smallest change:** describe it as finding direct calls and supplement it with a search for `.realm_of(`.

I did not build, run tests, start a daemon, execute the gate, or contact GGG. I ran the three read-only “Find first” searches. I did not query CI or issue 45 externally because this review was confined to the committed tree.

No additional obstacle came from the brief’s form or the review prompt.
