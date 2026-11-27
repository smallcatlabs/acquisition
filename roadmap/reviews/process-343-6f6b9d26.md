0f00397f^..6f6b9d26 — the brief’s done-when does not fully hold; two prose defects remain.

- **P3 — `crates/acquisition-search/src/pseudo.rs:323`, shape 1.** The new rule forbids derived fields from summing lines, but `DefencePct` reads local defence lines and sums their flat and increased modifiers in `rolled()` at line 1502. Concrete persisted input is an item body carrying local defence modifiers. The same overstatement appears in `src/totals.rs:3`; both should constrain the definition of **named totals**, while permitting a derived calculation’s intermediate sums.
- **P3 — `crates/acquisition-search/tests/language.rs:6`, shape 11.** “Reviewers wrote text” moves review history into a code comment. The concrete offending reading is C104’s newly moved reason. Keep the present rationale about exchanging and refining text; leave who used it in the records.

- Execution results—gate, tests and mutation demonstrations—**not verified**; this was a read-only review. Saving this review and answering it fall outside the supplied range.
- **Nitpicks:** both findings concern words alone; neither introduces a runtime regression.
- **Brief:** distinguish a named total’s definition from intermediate arithmetic when directing the C94 move.
- **Plan:** no additional issue found.
- **Prompt:** no change needed.
