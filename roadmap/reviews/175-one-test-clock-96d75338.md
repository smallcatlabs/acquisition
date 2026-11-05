# Review of 175-one-test-clock at 96d75338^..9f27644c

Codex (`codex exec --sandbox read-only`, `roadmap/briefs/REVIEW-PROMPT.md`), its answer verbatim.

96d75338^..9f27644c — the brief’s done-when does not fully hold.

- **P2 — [price_json.rs:1822](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/price_json.rs:1822), shapes 3 and 10:** Three process comparisons still use `but_for_the_clock`, retaining the stripping rule this step retires. For example, documents differing only in `taken_at` or `"89s ago"` versus `"1m ago"` still compare equal. Compare the documents whole at all three sites, remove the helper and `is_age`, and record their retirement.

- **P2 — [arch-check.sh:850](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/arch-check.sh:850), shape 1:** Check 14 misses ordinary aliases, repeating the checklist’s “source reader matches one spelling” fault. Test-source input such as `use acquisition_store as facts; let at = facts::now();` passes its regex while introducing a second clock. Refuse clock-source aliases at their imports or resolve them, and add cases to the existing breakers.

- **P3 — [lib.rs:1153](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/lib.rs:1153), shapes 1 and 3:** Invalid environmental input silently falls back to wall time in executable callers outside the three shipped binaries. With `ACQ_NOW=oops` and an otherwise valid fixture store, `redact-pricing` and `listing-report` reach `pricing_snapshot` without `check_now`; `copy-as-store` likewise calls `now` unchecked. Add startup validation to those callers.

- **Not checked:** Runtime behaviour, staged red runs, and the required two green gates: **not verified**, because this review was read-only and execution was prohibited.
- **Nitpicks:** The third finding concerns an unlikely combination of a mistyped test seam and an owner-run example.
- **Brief:** The global “`unaged` is gone” criterion conflicts with permitting documented in-process exceptions; `documents.rs:2653` retains one, explicitly acknowledged by the commit message.
- **Plan:** Its searches should cover timestamp normalization by behaviour, including `but_for_the_clock`, rather than only the name `unaged`.
- **Prompt:** Nothing impeded the review.
