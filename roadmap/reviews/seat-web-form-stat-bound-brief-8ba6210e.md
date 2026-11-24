`roadmap/briefs/seat-web-form-stat-bound.md` at `8ba6210e`: a builder could start; the design fits the rulings, but its verification instructions need correction.

- **Find first / Done when — changes what gets built.** The prescribed test command skips the aggregate assertions needed to measure the changed counts, and Done when names only bound counts although unbounded counts also change. In `tests/trade_translate.rs`, `c113_every_stat_id_of_the_table_translates` executes `if stride > 1 { continue; }` before those assertions; `common::generated::cases(16, 1)` selects that path unless `ACQ_CASES=full`. Reclassifying `other` also removes `stat.text.wording` from unbounded clauses (`translate.rs:1771`), affecting their statuses. The local gate does not set `ACQ_CASES=full`. Smallest change: prescribe `ACQ_CASES=full cargo nextest run -p acquisition-search --test main c113_every_stat_id_of_the_table_translates` and require both unbounded and bounded counts to be measured and updated.

- **Find first, second command — wording.** The command does not find “every reader of a form’s kind.” Running it misses, for example, `f.kind.word()` in `tests/trade_forms.rs:253` and the `Kind::Numberless` validation in `forms.rs:499`. Smallest change: add patterns for `\.kind` and `Kind::`, or narrow the command’s stated purpose.

- **What I did not check and why.** I did not build, run tests, regenerate tables or captures, or execute the gate, as instructed. I did not verify GitHub issues, CI conclusions or the owner’s off-tree instructions. The local recording’s file count, byte count and aggregate hash match the brief; its semantic extraction remains unchecked. I found no concrete way to satisfy every substantive Done-when requirement and still miss the stated goal.

- **Brief form:** No additional obstruction.
- **Review prompt:** No additional obstruction.
