roadmap/briefs/words-extents.md at 36c4dc9b, unchanged in the current file: not buildable exactly as drawn.

1. **Real, words to change:** events’ limit does not prove more exist; `10`, `24 h`, and the expansion promise are not generally supported.
2. **Real and sound:** daemon policies and sends need their lifetime named.
3. **Real, words to change:** the live list includes restored jobs from earlier lifetimes; the ledger’s running state is recorded, not current.
4. **Real and sound:** “last sign-in or token refresh” describes both writers without renaming either JSON key.
5. **Real, words to change:** the count covers game-side skip statements in public tabs, not every item marked `~skip` in game.
6. **Real, words to change:** additive `total` supplies the missing extent, but the truncated sentence also needs the way to list the remainder.
7. **Real and sound:** the receipt and status have different extents; the proposed distinction is correct, but sibling sentences must follow it.
8. **Real and sound:** the default is 3600 seconds; `(C3)` belongs only in internal documentation.
9. **Real and sound:** `items` sums live items at every location of the type.
10. **Real and sound:** `never` is an additive field on the shared type rows.
11. **Real and sound:** measuring the note before `generated_at` preserves the key and can leave the authorization envelope untouched.
12. **Real and sound:** the two compiles pin compile-time freshness; `plan_refresh` already takes the clock and reads `max_age_seconds`.
13. **Real and sound:** sharing the display-name rule is consistent with `Place`, provided the raw name still determines `tab.remove_only`.
14. **Real, words to change:** the proposed fetched-empty sentence is clearer, but it misses the contradictory no-listing branch already pinned.

Findings, most severe first:

- **Item 1 — the proposed sentence invents facts and promises the wrong continuation.** Reading: `crates/acquisition-verbs/src/events.rs:74` sets `limit_reached` on equality, including exactly-full results; `store_cmd.rs:738–745` keeps the event limit under expansion; `events.rs:140–145` makes `left_out` a count of locations. Smallest change: use the document’s event count and hours, say “the window may contain more; a larger `--limit` reads more events,” and distinguish expanding the returned events’ location rows. Pin exact-limit, nondefault-limit/window, and zero-limit cases.

- **Item 3 — “since this daemon started” excludes restored work that the count includes.** Reading: `crates/acquisition-daemon/src/daemon/queue.rs:390–415` explicitly includes restored jobs alongside this lifetime’s jobs. Also, `main.rs:122` feeds recorded-running counts into the same summary used elsewhere (`:2982`, `:3060`). Smallest change: name “jobs this daemon holds, including restored work”; say “recorded running” on disk; give live, ledger, and selected-job summaries their appropriate context rather than globally changing `summary_text`.

- **Item 14 — the brief leaves the actual no-listing contradiction standing.** Reading: `crates/acquisition-cli/src/price_cmd.rs:366–368` says “nothing on record” solely because both listings are absent; `:1421–1427` already pins that sentence beside four fetched locations. Smallest change: include this branch and fixture, reserve “nothing on record” for no locations, and take the fetched count from `r.facts.fetched`. The ordinary fetched-empty sentence at `:372` is ambiguous, rather than literally false.

- **Item 5 — “marked ~skip in game” still names a wider set than the count.** Reading: `crates/acquisition-plan/src/listing.rs:525–527` defines `by_game_statement` over public-tab statements; nonpublic price text is residue. Smallest change: say “N items with a game-side ~skip statement in a public tab,” keeping the nearer-hand-row qualification, and stage both public and nonpublic skip text.

- **Item 7 — changing only the status summary leaves contradictory vocabulary on sibling surfaces.** Reading: `price_cmd.rs:229` still heads the expanded list “rows naming nothing in the facts”; `:1173` quotes that wording from receipts; `pricing.rs:1140` repeats it in the status gloss. Smallest change: explicitly include those three producers and their holders, including `price_json.rs:361/:816`, `price_cmd.rs:1352/:1375/:1869/:1878/:1929`, and `seat_faults.rs:162` with `price_status_other_leagues.txt`.

- **Item 6 — the truncated count still omits how to obtain the omitted rows.** Reading: `crates/acquisition-cli/src/store_cmd.rs:687–693` currently offers only reading one body in full. Smallest change: append the scoped `store refused --limit M` continuation when `returned < total`, retain the body-detail action, and render both counts from `RefusedBodies`.

- **Item 13 — the proposed pin names a text surface and golden that do not exist in that form.** Reading: `seat_faults.rs:1500–1523` explicitly keeps scope ages in JSON and names places on the total line; `crates/acquisition-search/tests/seat_faults.rs:1160` is the unique-page test, not a CLI golden. Smallest change: pin matching names in scope JSON, total JSON, and rendered total/row text using an extended seeded fixture. Preserve named suffixes and the raw-name calculation at `corpus.rs:499`. `place_and_its_age.txt` changes only if its fixture changes; there is no existing `a_unique_tabs_page_with_no_name…` golden to regenerate.

- **Do 1 / item 12 — requiring every pin to start red is incompatible with adding a missing pin for correct existing behavior.** Reading: `lib.rs:2197`, `:2409–2420` already judge freshness using the supplied compile time and window; `unit/tests/lib.rs:350` already stages fresh facts. Smallest change: extend that fixture with the later compile, assert the fresh skip and later stale fetch, and exempt this coverage addition from the red requirement. No cycle input or planner behavior change is needed.

- **Do 3 — the holder inventory misses existing producers and assertions that regeneration cannot repair.** Reading: item 1’s unconditional “`--expand` lists every event” footer is at `store_cmd.rs:879`, while the raw-list `(limit reached)` remains at `:787`; item 3’s literal is at `seat_faults.rs:597`; web goldens hold rails/jobs text in `web/refresh_loop.txt:36/:44` and `web/jobs_halted.txt:10`; item 11’s direct pin is `acquisition-plan/unit/tests/ages.rs:116`, and `new_league/launch_morning.txt:49/:78/:113` holds its text. Smallest change: list these holders, update literals before regeneration, and either include the raw events wording or narrow the done-when grep. Also list item 14’s exact assertions at `price_cmd.rs:1414/:1426/:1867` and `new_league/launch_morning.txt:174`.

What I did not check and why:

- I ran no build, tests, gate, daemon, fixture regeneration, or runtime size measurements, as requested. The MCP bounds read are 600/300/1000 bytes for first-turn descriptions/parameters/instructions, and 7000/8000/6000/4000 for answer/pricing/store/daemon sections.
- I did not verify remote CI or issue closure state.
- I did not review landed rounds as code changes. For item 11, `documents.rs:429/:483` protects wrapper clock semantics and unchanged envelope/quote bytes; its empty notes do not prohibit this correction. The equality fixture should use a nonempty note and an answer time later than generation, while leaving league-note ages at answer time.
- The corrected work fits one worker and one round: stage pins, implement, update holders, regenerate, then gate and commit. No cut is necessary; the simplest execution is one gated build commit.

Nitpicks: only the stale line references and nonexistent-golden citation in isolation; the missing coverage and incorrect surface description are substantive.

The step should not start as drawn; correct the wording, branches, and holders first, then keep it as one round.
