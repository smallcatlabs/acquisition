c399ada8 — #228’s done-when does not yet hold: two recorded stops remain deferred.

- **P2 — audits/governance-2026-11/seat/README.md:186,191 (shapes 1, 3):** Two stops are marked fixed by a future close, but their conflicting instructions remain at this commit. For “Start the next step,” `roadmap/README.md:34` and `:50` still offer different queues; for “Fix issue #200,” `:70` still says `trigger:step` issues wait for the next step in their area. Both share one cause: promised road changes are counted as fixes before landing. Apply the two promised road edits, or file these unresolved stops as issues and correct their fates.

- **Not checked:** Execution and gate results: not verified, as requested. External issue contents and state were not checked; this review used committed evidence. No runtime code changed, so client, persisted, and upstream input handling was outside the diff.

- **Nitpicks:** None.

- **Brief:** The acceptance criteria were clear.
- **Plan:** Separating the road fixes into a later close leaves this commit short of the acceptance criteria.
- **Prompt:** Nothing impeded the review.
