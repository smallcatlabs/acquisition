`roadmap/briefs/156-facts-blocks.md` at `0d287307`, unchanged in the current tree: **not buildable as drawn**.

1. **Answered.** `resolve` receives snapshot `fetched_at` values; building unaged facts there, retaining them through summary, and pinning aged `status()` serialization is reachable and coherent.
2. **Narrower, sound subject.** The explicit non-folder snapshot set includes substashes, league-less and never-listed locations; the gloss still incorrectly equates it with every report container.
3. **Answered.** `list_seen` is the oldest relevant basis, while separately named listings retain their own times; round-order age parity remains a finding below.
4. **Narrower, not sound throughout.** Extreme equality, both counts and the stale label are corrected; zero-item text still suppresses nonempty location evidence.
5. **Answered.** Each extreme has its own `ago`; extending the live mask with a normalizer case addresses changing extreme locations and forms.
6. **Narrower, not complete.** The six added holders are named, but another exact-output status test remains missing.
7. **Answered.** Raw resolve facts, aged status serialization, MCP prose regeneration and additive fixture changes are distinguished; facts outside the header stay off list and show.
8. **Answered.** C77, freshness judged at compile time, now points to `decisions/plans.md`.

Findings, most severe first:

1. **Do 3 / done-when 1 and 3 — Printing separate shop listing ages requires JSON words that this step explicitly leaves to round 3.** Reading: `crates/acquisition-plan/src/pricing.rs:991` supplies a raw resolved report; `shop.rs:910` clones its unaged header; `listing.rs:1784` initializes both listing-age fields to `None`. `FetchSpan` carries only the aggregate `list_seen_age` (`crates/acquisition-store/src/coverage.rs:125`), which cannot word two different listing times. The brief excludes header aging at `roadmap/briefs/156-facts-blocks.md:262`, contradicting C53’s requirement that printed ages have JSON words. **Smallest change:** age the render header’s two listing fields in this step when round 3 has not already supplied that behavior, permit those additive fixture keys, and pin both ages on the mixed-location render.

2. **Do 3 / done-when 2 — Zero items does not mean zero locations or no fetch evidence.** Reading: the branch at `crates/acquisition-cli/src/price_cmd.rs:285` depends solely on `counts.items`; snapshots independently carry locations and fetches (`crates/acquisition-store/src/snapshot.rs:243`, `:503`). The existing rehearsal already has fetched empty tabs (`crates/acquisition-cli/tests/new_league.rs:125`). Showing only listing ages there hides the subject’s fetch span; directly fetched empty locations can also exist without either listing. **Smallest change:** print the common counts-and-span line whenever `facts.places > 0`, including this branch; reserve `facts: nothing listed` for zero places and neither basis, and retain listing ages for empty listed sets. Stage fetched-empty cases with and without a listing basis.

3. **Do 1–2 — The specified embedding requires deserialization support that the store’s span types currently lack.** Reading: `FetchedLocation` and `FetchSpan` derive only `Serialize` (`crates/acquisition-store/src/coverage.rs:30`, `:98`), while their proposed containing documents derive `Deserialize` (`crates/acquisition-plan/src/shop.rs:426`, `listing.rs:583`) and existing reference tests deserialize them back (`unit/tests/shop.rs:926`, `unit/tests/listing.rs:724`). **Smallest change:** explicitly include `Deserialize` for `Facts`, `FetchSpan` and `FetchedLocation`, preserve the round-trip assertions, and specify `skip_serializing_if = "Option::is_none"` for `Freshness.facts` so absence means omission. These are additive changes; no schema bump is required.

4. **Do 5 / Read — An additional whole-text assertion necessarily breaks when the empty status gains its facts line.** Reading: `crates/acquisition-cli/src/price_cmd.rs:1701`, `c35_an_empty_league_still_reports_its_rows`, compares the complete default output at `:1717`. It is separate from the named positional test. **Smallest change:** name this holder in Read and require its expected text to include the new line while preserving its row visibility and next action.

5. **Do 4 — The status gloss describes a larger set than `facts.places` counts.** Reading: `resolve` pushes folder listings too (`crates/acquisition-plan/src/listing.rs:1619`); the check at `:1602` only excludes folders from priced-tab counting. Consequently `counts.containers` includes them (`:1642`), whereas Input excludes them from facts. “Every one the report lists” at `roadmap/briefs/156-facts-blocks.md:224` misstates that distinction. **Smallest change:** say “the report’s non-folder containers” in the gloss and correct Read’s claim that resolve itself excludes folders. Preserve the existing report listings and container count.

6. **Do 6’s required input — The brief supplies no concrete CI runs or conclusions.** Reading: `roadmap/briefs/156-facts-blocks.md:180` says “the runs named at cut time” but names none; `roadmap/briefs/TEMPLATE.md`’s Input requires both gates’ commit, run ID and conclusion. **Smallest change:** replace that placeholder with the two observed run records and the required disposition of any failure or pending run.

What I did not check and why: no builds, tests, gate, regeneration, daemon execution, remote CI queries or live evidence, as instructed. This was static review; no files were changed.

Nitpicks: none of the findings above. The previous citation nitpick is fixed.

**The step may not start as drawn.**
