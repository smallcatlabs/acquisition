8ae1d597^..0e3a1dc0 — the brief’s done-when does not fully hold: location grouping fails for supported multi-league input.

- **P2 — [answer.rs:2523](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:2523), shape 1:** Match-dependent default columns break the places page’s assumption that every open place shares one column list. With persisted gear in `t1` and currency in `c1`, client input `tabs?realm=pc&league=Standard&open=t1&open=c1` produces currency count cells while the shared header, taken from `t1` by `heads_rows`, has no `stack` heading. Render headers matching each place’s resolved columns, and pin both tabs open together.

- **P2 — [corpus.rs:1002](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:1002):** The new location rank omits a stash’s league, merging distinct persisted locations during sorting. For tabs `Standard/t1` and `Hardcore/t1`, with items `a` and `c` in Standard at columns 0 and 2 and `b` in Hardcore at column 1, an unsorted search returns `a,b,c`, splitting Standard’s location. Use the existing full `coordinate(...)` key, including the stash league, and extend the ordering test with this case.

- **P3 — [search_cmd.rs:139](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/search_cmd.rs:139), shape 3:** The `--value` help still describes the default list as the values the query compares, omitting the newly automatic `stack`. A client searching `tab:Currency` receives stack counts despite comparing no numeric value. Update this help and regenerate `CLI-REFERENCE.md`; correct the same incomplete description on `Rows.values` in `answer.rs`.

Execution—gate, tests, generated cases and browser behavior—was **not verified**, as requested.

Nitpicks for cases unlikely in practice: none; the shared-id case is an edge case supported by the store’s location model.

Brief: add fixtures for multiple open places with different default columns and same-id tabs across leagues.  
Plan: no other change suggested.  
Prompt: nothing impeded the review.
