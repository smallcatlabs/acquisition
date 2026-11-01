`roadmap/briefs/describe-slices.md` at `9dfa89f5`, unchanged in the working tree: not buildable exactly as drawn; three concrete conflicts need correction.

1. **Answered** — C53’s recorded amendment matches Input and precedes the build.
2. **Narrower (sound)** — language-only patterns preserve keys, counts and tiers under `assert_spoken_as`; MCP entries such as `revision` and `wedged` deliberately remain reachable only by name.
3. **Answered** — the offer now crosses `Offered`, `searched`, `Way` and each surface, separately from readings; the lifting alternative can preserve existing callers and realm offers.
4. **Answered** — omitting unset metadata preserves `all`’s existing serialization and section pin; the new index pin has a separate defect below.
5. **Answered** — exact, case-insensitive category-ID selection and its round-trip pin give the worker enough direction to change validation and retention.
6. **Answered** — the echoed pattern becomes payload, with an explicit MCP preservation case.
7. **Answered** — the examples reader moves to `all` and gains a nonempty witness.
8. **Answered** — the owner selected the default, and the alternative is removed throughout.
9. **Narrower (sound)** — density control and measured acceptance cases replace the universal byte promise; the exceptions are explicit.
10. **Answered** — the web reader is identified and its named-trade behavior preserved.

Findings, most severe first:

- **F1 — Do 1, 4–5 / settlements 1 and 6:** The CLI cannot pass every promised Rust regex intact because clap splits commas inside patterns such as `'~^.{1,20}$'`, whereas the MCP receives one pattern. **Concrete reading:** [search_cmd.rs:156](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:156) declares `value_delimiter = ','`; [brief:318](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:318) promises Rust regex syntax. **Smallest change:** specify a lossless CLI encoding for commas within pattern items, including its delimiter handling, and add that pattern to the parity cases.

- **F2 — Do 4 / settlement 2:** Treating every item starting with `~` as a pattern breaks the existing operator entry named exactly `~` and Do 3’s name round trip. **Concrete reading:** [describe.rs:575](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:575) defines that operator entry; [brief:399](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:399) reserves its prefix without an exception. **Smallest change:** retain bare `~` as the operator’s name, require something after it for the pattern form, and pin both cases.

- **F3 — Do 3 / settlement 7:** The prescribed index-key assertion contradicts Do 1 because the index omits `trade`, so its keys cannot equal all sections plus two metadata keys. **Concrete reading:** [brief:383](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:383), [describe.rs:63](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:63) and [describe.rs:74](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:74): absent `trade` is skipped, but `SECTIONS` includes it. **Smallest change:** assert `(SECTIONS − {"trade"}) ∪ {"trade_rows", "next"}` for the index; retain the existing section equality on `all`.

- **F4 — Do 3 / settlement 2:** “Every name it prints … answers whole” also includes the retained `not_built` constructs, whose existing behavior is refusal. **Concrete reading:** [brief:381](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:381), [bind.rs:138](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/bind.rs:138) and [describe.rs:754](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/describe.rs:754). **Smallest change:** scope that promise and pin to `Named` entries, category IDs and section names, explicitly retaining the `not_built` refusals.

- **F5 — Settlement 6:** The Markdown example contains two literal backslashes, so `~^pseudo\\.total` does not express the intended literal dot. **Concrete reading:** [brief:322](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/describe-slices.md:322). **Smallest change:** write `~^pseudo\.total`. Separate-string matching is otherwise buildable; Do 5 pins name anchors, and an anchored-definition case would complete that settlement’s coverage.

What I did not check and why:

- Builds, tests, gates, runtime behavior, sizes and match counts were **not verified**, as requested.
- Future implementation and fixture diffs do not exist yet; issue bodies and comments were assessed through the brief’s account.

Nitpicks: F4’s overbroad wording and F5’s escaping typo. The anchored-definition pin is a coverage improvement.

The step should start after F1–F3 are resolved in the brief.
