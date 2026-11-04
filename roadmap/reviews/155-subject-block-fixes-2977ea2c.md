`2977ea2c` — the fix round’s source-level done-when holds.

- Findings: none. The persisted-input fixture now stages removed matches in Settlers and Standard, asserts both leagues and exact matched ids under `all` and `removed`, then asserts one place.
- The gloss now distinguishes null fetch epochs from absent age/location keys and retained listing fields. The added gloss sample and existing serialization assertions pin the correction; Do 4 accurately records it as “Settled in the round.”
- The third fix uses one rule for both live fetch-clause forms, pinned by three normalizer cases. Equal-second fetches can legitimately select the single-location form across multiple matched places. Seeded goldens retain both forms as facts, so this mask does not remove their only coverage; clause presence and the surrounding counts remain checked.
- All four regenerated lines follow that rule. Nothing outside the three faults changed, and no test or check was retired.
- Builds, tests, gate results and claimed measurements: **not verified**, as execution was prohibited.
- Nitpicks: none.
- Brief: the Do 4 correction resolves its conflicting wording.
- Plan: no obstacle.
- Prompt: no obstacle.
