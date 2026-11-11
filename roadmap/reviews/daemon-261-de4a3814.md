f8b01d5a^..de4a3814 — the issue’s done-when does not fully hold: parent completion and restore-wide write accounting remain incomplete.

- **P1 — `crates/acquisition-daemon/src/daemon/queue.rs:473` (shape 2):** Restore still ends parents in unreadable trees directly, bypassing the required child tally. **Persisted input:** running parent 11, unreadable child 12, and readable waiting child 14, as in the existing queue fixture. The parent’s held result is cleared and its failure reaches `end` without passing through `maybe_finish_parent`, leaving `Outcome::Failure.children` absent. This preserves the fault the issue explicitly required this step to resolve. Smallest fix: hold the selected failure on parents with children in memory, end the open leaves, and finish those parents through `maybe_finish_parent`; strengthen that fixture to assert the readable children’s tally.

- **P2 — `crates/acquisition-daemon/src/daemon/queue.rs:617` (shape 1):** The marker is removed before restore-triggered parent completions, so `every_write_landed` does not cover the whole restore. **Persisted input and write failure:** a running parent holding success, its last open child named by the fault marker, and a database trigger that permits the restore’s initial writes but rejects the parent’s terminal write. The child’s failure lands, the marker is removed, and only afterward `maybe_finish_parent` attempts the refused write at line 640. Smallest fix: complete the parents before deciding marker removal and fold their write failures into the same fact, using the sticky queue failure or propagated write results.

- **Not checked:** gate results, test counts, runtime behavior, and the manual compiler-rejection proof are **not verified**; execution was prohibited.

- **Nitpicks:** neither finding; both concern duties expressly required by the issue.

- **Brief:** “No behaviour change” conflicts with applying pending-cancel arbitration to the unreadable-tree arm, whose previous behavior differed; clarify that exception.
- **Plan:** deferring the unreadable-parent completion change to the next step conflicts with this issue’s explicit scope.
- **Prompt:** no changes suggested.
