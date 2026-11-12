1ab468eb — the brief’s done-when does not fully hold.

- **P2 — `crates/acquisition-search/src/answer.rs:1195` (shape 5):** The sort joins the values after their bound is enforced, allowing thirteen values despite the twelve-value contract. **Failing client input:** twelve distinct valid `rows.values` excluding `ilvl`, plus `sort: "ilvl"`; the same overflow occurs with twelve default columns and a distinct explicit sort. **Smallest fix:** enforce the bound after including the sort; refuse explicit overflow, and reserve space for the sort when cutting default values, counting displaced values. Pin both cases.

- **P3 — `crates/acquisition-cli/src/search_cmd.rs:1544` (shape 2):** The CLI explains the store’s order only when rows remain undisplayed, so ordinary complete answers miss the explanation required by the brief. **Failing client input:** the new golden’s `'"＋# to maximum Life">=90 ilvl>=1'` query, using an ASCII `+`, with `--limit 10` instead of `--limit 1`; its two matches receive no order explanation. **Smallest fix:** emit `order_words` for complete nonempty answers whenever it returns words, and add an untruncated golden.

- **P3 — `crates/acquisition-search/src/answer.rs:1192` (shape 3):** The query’s omitted-value count exists only in JSON: neither renderer displays `values_left_out`, and `describe answer` never documents it. **Failing client input:** a query comparing thirteen distinct valid sums, with no named values; JSON reports one omitted value while CLI and page text give no omission notice. **Smallest fix:** render the count through shared wording, document the key, and pin a default list past twelve.

- **P3 — `crates/acquisition-cli/src/search_cmd.rs:131` (shapes 1, 3):** Help promises ordering whenever a query compares one value, although equality, ranges, negation and conflicting directions deliberately preserve store order; the same broad promise appears in `describe.rs:259` and the README tour. **Failing client input:** `ilvl=80..84` without a sort. **Smallest fix:** qualify all three surfaces with the requirement for one consistent ordering direction, then regenerate the affected references and goldens.

Execution, gate status, test count and `--only doc`: **not verified**, as requested; no build, tests, application or browser was run. Unchanged daemon and persistence internals were outside the affected call paths.

Nitpicks: the omission-description gap is minor for searches exceeding twelve values; the other findings concern explicit contracts or ordinary searches.

Brief: add combined value-bound/sort cases and an untruncated multi-value golden. Plan: no change suggested. Prompt: no obstruction.
