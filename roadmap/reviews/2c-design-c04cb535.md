4d2932fb^..c04cb535 — design and build brief delivered; the build brief’s done-when is not met yet, and execution-dependent criteria are **not verified**.

- **[P2] `crates/acquisition-store/examples/export-items.rs:21` — shapes 1, 3:** The exporter still promises that no rows are written, although its `Store::open` call migrates older databases. Input: the owner’s schema-v8 store described by the amendment is upgraded by this schema-v9 build, even if the requested item is missing. Smallest fix: correct the example’s documentation to disclose migration and direct preservation-sensitive exports to a database copy; the design’s warning does not correct this sibling claim.

- **[P2] `roadmap/briefs/2c-design.md:470` — shapes 3, 6:** The currency search cannot run as printed because `--sum stack` is inside the query, and moving it outside still leaves the required count view absent. Input: `acq search 'class="Stackable Currency" --sum stack'`; `Shape::of` also explicitly refuses a standalone sum. Smallest fix: use `acq search 'class="Stackable Currency"' --count base --sum stack` and give the MCP twin the corresponding count view.

- **[P2] `roadmap/briefs/2c-design.md:467` and `:471` — shape 1:** Two journey facts were not recalculated after the fixture amendment. Input: the specified armour’s six gems become separate stash rows, so `tabs` reports **37 items**, while the ring’s `pseudo.total_life` is **108**: `22 + 78 + 16 × 0.5`, according to `totals-v6.toml`. Smallest fix: correct both facts and distinguish top-level array lengths from stored-item counts in the count-preservation requirement.

- **[P2] `roadmap/briefs/2c-design.md:462` — shapes 1, 3:** The required `(alice): done` progress text belongs to a simple job, whereas an apply reports its children’s tally. Input: the successful two-request apply prints `(alice): 2/2 done`, through `jobs::wait` and `children_line`. Smallest fix: use that terminal tally in appendix B and correct section 2.3’s sibling claim that the terminal progress line is always `done`.

- **[P2] `roadmap/briefs/2c-build.md:236` — shape 2:** The prescribed regression breaker references `before.at`, which neither the existing nor proposed `Status` contains, so it can fail at compilation without testing the named regression. Input: perform the replacement exactly as specified. Smallest fix: substitute a compiling timestamp-based read using the staged ingests’ shared timestamp, and require the test to execute and fail its count assertion.

- **[P2] `roadmap/briefs/2c-build.md:14` — shape 1:** The promise to count one apply’s events “and no other’s” exceeds what an event watermark provides. Input: apply A captures watermark 100, another job commits event 101, and A commits event 102; `events_after(100)` includes both. Smallest fix: narrow the promise to excluding events committed before submission, including earlier events in the same second; retaining exclusive attribution requires associating events with the apply.

- **[P2] `roadmap/briefs/2c-build.md:76` — shape 8:** The byte-for-byte fixture requirement depends on an uncommitted temporary file without a recorded digest or exact selected-id manifest. Input: that path is replaced or cleaned before the build, leaving no committed way to establish which export was intended; additionally, the fourteen listed bodies minus the Divine Orb leave **thirteen**, not twelve. Smallest fix: record the export digest and thirteen selected ids, verify them before reading, and preserve the named input through the build.

- **Not checked:** No builds, tests, gates, mutation runs, daemon sessions, timings, binary sizes, or runtime stability checks were performed, as requested; those criteria remain **not verified**. I did not read the owner’s store or uncommitted scratch exports, so their contents and fixture equality are also not verified. No golden or reference files changed in this range.

- **Nitpicks:** None of the substantive findings above; the twelve-versus-thirteen wording alone would be a nitpick once the selected input is pinned.

- **Brief:** Give the design delivery its own done-when, separate from the future implementation’s acceptance criteria.
- **Plan:** No additional obstacle.
- **Prompt:** The fixed revision and prohibition on execution made the review scope clear.
