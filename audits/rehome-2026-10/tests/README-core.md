# The test read: core

200 rows (`data/read-core.tsv`), one per row of `data/inventory-core.tsv`, every body read: store 83, plan 89, protocol 11, client 17.
boundary 193 · weak 4 · mechanism 3 · cannot-fail 0 · duplicate 0 · harness 0.
Confidence: high 173, medium 27, low 0; every medium row says why in `note`.
`data/cases-core.tsv`: 7 properties at proptest's default 256 cases, knob `PROPTEST_CASES` (the crate's own, undocumented here); 3 rows the inventory calls `proptest` are plain tests; 2 scale loops listed beside them.

## weak

- `crates/acquisition-store/src/annotations.rs:1682` `export_is_a_consistent_snapshot_and_never_overwrites` — no writer runs during the export (`annotations.rs:1690`); an export that copies the database and its WAL file by file, torn under a concurrent write, passes. The no-overwrite and no-partial halves hold.
- `crates/acquisition-store/src/lib.rs:3076` `a_large_tab_ingests_in_one_transaction_quickly` — nothing observes the transaction, and the first ingest's time is printed, never asserted (`lib.rs:3099`); only the unchanged re-ingest is bounded, at 5 s (`lib.rs:3101`). A per-row-commit ingest passes.
- `crates/acquisition-store/src/world.rs:763` `c83_the_private_dir_is_this_users_and_a_root_is_utf8` — the UTF-8 half runs only where the filesystem lets the test make a non-UTF-8 directory (`world.rs:800`); on macOS it does not, so a `World::at` that accepts such a root passes there. The 0700 half holds everywhere.
- `crates/acquisition-plan/src/lib.rs:2267` `a_disagreeing_count_proves_change_and_an_agreeing_one_proves_nothing` — no state in the test has an agreeing count: at the "agreeing count cannot skip a stale fetch" step (`lib.rs:2325`) the stub still says 3 against 1 held. A planner that skips a stale substash whose count agrees passes. This is C36's own sentence, pinned for one of its two halves.

## mechanism (all three; most disposable first)

- `crates/acquisition-store/src/lib.rs:2993` `v7_to_v8_swaps_the_index_and_moves_no_row` — its leading assertion is the list of index names on `items` (`lib.rs:3022`). No read returns an index; "moves no row" and the facts version are the observable part and would survive as a two-line migration test.
- `crates/acquisition-store/src/annotations.rs:1654` `the_first_difference_names_the_path_and_both_sides` — calls the private `first_difference` with its path-prefix argument (`annotations.rs:1657`). The property is real (C66: refused naming the path) and reachable through the public `check_value`; `annotations.rs:1547`, `price.rs:954` and `plan/src/lib.rs:2026` each hold one path that way. Move rather than drop.
- `crates/acquisition-store/src/snapshot.rs:731` `body_strings_read_each_field_by_its_json_type_and_never_descend` — calls the private `body_strings::<2>` (`snapshot.rs:732`). The deep-nesting case is a review finding nothing else holds; `pricing_snapshot` (`snapshot.rs:1441`) reaches the scanner but covers a number and an object only. Move rather than drop.

## Read against the brief's question, not a class

- The inventory's `kind` is wrong for `shop.rs:2100`, `2212`, `2272`: `inventory.py` ends a `proptest!` block only at a `}` in column 0, and these sit in the `real_scale` module after the indented block at `shop.rs:1979`. The group has 7 properties, not 10.
- The private `conn` appears in 23 store tests. Most use it to stage a fault or check a second way; five assert on a column or pragma no public read returns and are medium for that: `lib.rs:2378` (the envelope's `_split`), `2859`, `3834` (`item_events.response_id`), `3926`, `annotations.rs:1898`. `corpus.rs:999` pins the literal facts version 8, so every facts migration edits that test.
- Three plan tests build on private items and stay boundary because no cheaper public rung holds the property: `shop.rs:1989` (the cutter over any entries, through `cut_pages` and a hand-built `Entry`), `1806` (`stash_numbers` against the website's list), `1547` (a cut point computed from `page_fixed_chars`).
- Nine planner tests compare a reason by its `Debug` text (`plan/src/lib.rs:3237`, `3321`); the plan's JSON is the contract, and `2738`, `2886`, `3897` hold that separately.
- Three golden tests rewrite their fixture and pass when `ACQ_UPDATE_FIXTURES` is set (`listing.rs:2136`, `shop.rs:1908`, `wire.rs:40`), by design.
- Count pins on reviewed data: `currency.rs:308` and `486` (42 rows) turn red on every addition to the table.
- Real time in an assertion: `contract.rs:510` (at least 5 s, C86's deadline), `annotations.rs:1824` and `1873` (400 ms of "did not return"), `listing.rs:2503`, `2890`, `shop.rs:2105`, `lib.rs:3101` (cliffs of 2 to 5 s).
- No exact duplicate. The nearest pair: `plan/src/lib.rs:2704` is the tab half of `3800`'s two-facet loop, under a v1 policy and a stale listing.
