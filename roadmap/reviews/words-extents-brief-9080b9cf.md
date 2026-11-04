roadmap/briefs/words-extents.md at 9080b9cf is substantially corrected, but not buildable exactly as drawn.

The nine findings:

1. **Events — narrower, still unsound:** counts, hours, equality and zero-limit cases are answered; the replacement expansion footer promises the wrong operation.
2. **Jobs — answered:** restored work and recorded running are named; a context-aware `summary_text` can serve live, ledger and selected-job reads without three sentence implementations or duplicate wording pins.
3. **Zero-item status — answered in substance:** the no-listing contradiction is included and fetched counts come from `facts`; the branch coverage still needs specifying.
4. **Skip count — answered:** the proposed words accurately name public tabs’ game-side statements and preserve the nearer-hand-row qualification.
5. **“Names nothing” — answered:** all three producers, the gloss and their existing assertions are included.
6. **Refused continuation — narrower, still unsound:** the continuation exists, but drops the account whose rows it promises to list.
7. **Remove-only names — answered:** the four surfaces are correctly identified; the shared display rule preserves named suffixes, while the raw parent-or-tab name still determines `tab.remove_only`.
8. **C77 starting red — answered:** extending the fresh-facts fixture with a later compile pins existing correct behaviour without requiring a production change.
9. **Missed holders — answered for the first review’s inventory:** those holders are now named; additional item-10 holders remain missing.

Findings, most severe first:

- **Item 1 — the new footer still promises an operation the flag does not perform.** Reading: `crates/acquisition-cli/src/store_cmd.rs:739` selects the raw event list when `--expand` is supplied alone; `:744` expands locations only under `--summary --expand`. Input’s “`--expand` lists every location” at `roadmap/briefs/words-extents.md:189` is therefore false when followed from the default text command. Smallest change: say “`--summary --expand` lists every location touched by the returned events,” preserve the request’s account, hours and event limit, and pin that continuation from the default invocation.

- **Item 6 — the proposed continuation can fail to select the store whose total it quotes.** Reading: `crates/acquisition-cli/src/store_cmd.rs:17` opens the selected account’s facts; `crates/acquisition-verbs/tests/documents.rs:203` pins ambiguity when two accounts exist without a selector. Input’s `acq store refused --limit M` omits that selection. Smallest change: carry the resolved, shell-quoted `--account` in the continuation and stage two accounts while executing the printed command. Keep the body-detail action after it.

- **Do 1 / item 14 — the staged input does not pin the new branch boundaries or the unfetched no-listing form.** Reading: the brief stages only “a zero-item status over fetched empty locations” at `:269`; `crates/acquisition-cli/src/price_cmd.rs:1414` and `:1426` cover fetched locations with and without listings, while `:1438` filters out the opening sentence and checks only `facts:`. Smallest change: explicitly stage and assert the opening sentence beside `facts:` for no locations/no listings, locations/no listings with zero and positive fetched counts, and a listing present with zero and positive fetched counts. State the guards as mutually exclusive: neither listing plus zero places; neither listing plus positive places; otherwise a listing exists.

- **Do 3 / item 10 — the holder sweep still omits assertions that fixture regeneration cannot repair.** Reading: `crates/acquisition-search/tests/tabs.rs:133` compares complete `by_type` JSON rows; `crates/acquisition-store/unit/tests/coverage.rs:98`, `:165`, `:278` and `:296` construct `TabType` expectations, and `:432` holds serialized coverage containing a type row. All must gain `never`. Smallest change: name these holders, include them in the pre-regeneration edits, and pin both zero and positive `never` values on the tabs and search surfaces. For item 13, extend `a_unique_tabs_page_with_no_name_is_named_by_its_id` and a seeded CLI transcript: extending `a_rows_place_says_how_long_ago_its_location_was_fetched` changes `place_and_its_age.txt`; a separate case needs its named golden. The existing C117 field test should retain its raw-name assertions.

What I did not check and why:

- No build, test, gate, daemon, regeneration or runtime size measurement was run, as requested. The MCP limits read are 600/300/1,000 bytes for descriptions/parameters/instructions and 7,000/8,000/6,000/4,000 for answer/pricing/store/daemon sections; the post-edit measurements remain the worker’s.
- I did not verify remote CI or issue state, or review landed rounds as code changes.
- No proposed JSON key is renamed or removed. Item 11 can change the note’s measure while preserving the envelope; its equality fixture should use a populated note and an answer time later than generation. Item 8’s ruling citation remains internal documentation.

Nitpicks: none of the four findings above.

The step should not start exactly as drawn; make these brief edits, then retain one worker, the stated order, and one gated build commit.
