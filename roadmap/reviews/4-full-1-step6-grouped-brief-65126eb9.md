`roadmap/briefs/4-full-1-step6-grouped.md` at `65126eb9`: a builder needs the following design gaps resolved before starting.

1. **Decisions 2 and 5; Done when — changes what gets built.** One corpus does not establish that the separately read places share its basis. `Places::tabs` uses `Store::tabs_document`, characters use another store read, and `Corpus` retains neither complete listing; a write between these reads can combine old item matches with new place facts while every static acceptance fixture passes. This is the first thing a builder must guess. **Smallest change:** specify a bounded read/recheck procedure, following `Corpus::read`’s existing pattern, and require a test with a write between the reads.

2. **Decision 4; Done when — changes what gets built.** “Places where it holds places, else its items” makes a parent’s direct items unreachable whenever it also has children. Store ingest accepts both `stash.items` and `stash.children` (`crates/acquisition-store/src/lib.rs`, around the two `shift_remove` calls); a parent containing one matching item and an empty child therefore has a positive `matched` but opens to no matching item rows. The inherited fixture uses an empty parent item array, so the stated checks can pass. **Smallest change:** define a direct-items segment alongside child places, with a request excluding the children, and add this mixed-parent case.

3. **Decisions 3 and 7; Not in scope — changes what gets built.** `asked_by_the_form` accepts scopes that the proposed place-page fields cannot preserve. It carries `membership=removed` or `all`, and search accepts `realm=all`; place-page requests currently use live membership and `realm_of` accepts a concrete protocol realm. Consequently, the proposed links can silently change membership or lead to a refusal. **Smallest change:** restrict grouping links to one concrete realm and live membership, test that eligibility, and explicitly exclude broader scopes—or specify their representation.

4. **Decision 7 — changes what gets built.** “The league the query names” supplies no selection rule when the query names several leagues. For `league=Standard or league=Hardcore`, `league_terms` returns both as named, whereas the existing `fixed_league` deliberately returns none; choosing either helper changes the destination. **Smallest change:** state a criterion, such as using the uniquely fixed league and otherwise the default, and test a disjunction and a nested conjunction.

5. **Done when, third bullet — wording.** Requiring “every field kept” for every followed link contradicts Decision 3’s fold link, which deliberately resets the page to `fetched=never` alone. A value head also deliberately changes view fields. **Smallest change:** require each link’s specified transition, explicitly distinguishing preserved, replaced and cleared fields.

6. **Find first — wording.** The commands miss affected answer goldens and include unrelated filter constants. Running the fourth command finds `console_realm.txt`, `refresh_loop.txt` and `web.rs`, but misses `answer_ways.txt`, whose scope links Decision 7 changes; the first command also returns trade catalogue and translation constants through `FILTER`. **Smallest change:** add a search for answer pages and destination links, for example `git grep -l -E 'GET (search|tabs|characters)|/tabs\?|/characters\?' -- crates/acquisition-cli/tests`, and narrow the filter constant alternative to `const FILTER:`.

Not checked: builds, tests, browser behavior, performance, hosted CI or current GitHub issue contents; this review used committed evidence and read-only discovery commands.

Form: most “Find first” commands omit their purpose, making their intended coverage harder to assess.

Prompt: nothing impeded the review.
