d58ddd29^..27efaeed — the brief’s done-when does not fully hold: the docs checker’s breaker was left incompatible with the new stage list.

- **P2 — `tools/docs-check-breakers.sh:226`, shape 3:** The expected diagnostic omits the newly added `doc` stage, making this breaker fail and turn Linux CI red. Its staged swap now produces `build test clippy winclippy doc diff fmt`, while the assertion requires `build test clippy winclippy diff fmt`. Add `doc` to the expected diagnostic.

- **P3 — `tools/trade-check-breakers.sh:81`:** Reading the clone path and pin as space-separated fields corrupts both when the path contains whitespace. With local configuration input `ACQ_CLONES='/tmp/game clones'`, `poe1` becomes `/tmp/game` and `pin` includes `clones/poe1`; the class-table case is wrongly skipped and the off-pin assertion fails. Read the path and pin separately, preserving each whole value.

- **Not verified:** Execution-dependent done-when checks, Keychain dispatch results, and GitHub closure/comment actions; this review was read-only and ran no builds, tests, or workflows.

- **Nitpicks:** Finding 2 is the limited configuration edge case.

- **Brief/plan:** The range ends before the required close commit, leaving its dispatch run citation and closure record outside the review.

- **Prompt:** No changes suggested.
