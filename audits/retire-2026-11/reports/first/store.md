### C29 — Bodies are stored as received except
- class: mech — how the store keeps a body (an envelope plus lifted item rows); C48 makes the schema internal, so only the store's own read API sees the split
- clauses:
  1. P · P · `crates/acquisition-store/unit/tests/lib.rs:246-248` asserts the stash envelope has no `items`, carries `_split.items == 2`, and keeps `metadata.colour` verbatim. `lib.rs:764` asserts the character json has `_split` and no item. The "only place to look" half is held. The "as received" half is loose: nothing asserts that the envelope plus the rows give back the response, so dropping some other non-item key would pass. I read it as P because a later session designing a read (search, C108) relies on `items` being the one home.
  2. M · Y · `lib.rs:236` (two stash items become two rows); `lib.rs:1703-1708` (equipment, guardian and skills items become rows with their container); `lib.rs:2085-2137` (one tab id under two realms is two locations, and the move event names both full coordinates). Stated at `src/lib.rs:143-146`.
  3. M · R · restates C54 clause 3. The withheld detail is also held at `lib.rs:2283-2300`.
- unheld: 1 simple · para · add an envelope-plus-rows reassembly assertion to `stash_items_are_lifted_and_the_envelope_keeps_the_count` (`unit/tests/lib.rs:225`)
- reason: Prop · `crates/acquisition-store/src/lib.rs:143` (states the lifted arrays and the envelope; not the Why about duplicated bodies and query scans) · moves to `crates/acquisition-store/unit/tests/lib.rs:224`
- fate: NT · simple
- confidence: medium · I did not read `rebuild_reextracts_columns` or the ingest code behind clause 1

### C145 — A present key of the wrong
- class: mech — the store's ingest refuses a wrong-typed seam rather than retire facts; the facts can be refetched and nothing outside the store sees the seam
- clauses:
  1. P · P · `unit/tests/lib.rs:471-529`: a case table covering `items`, `children`, a deep `socketedItems`, `leagues` and a stash id mismatch. Each is refused as `MalformedBody`, kept in `refused`, and retires nothing. Loose half, "every seam": the top-level character arrays (`inventory`, `equipment`, `jewels`, `rucksack`, `guardian`, `wickerman`, `skills`) present as non-arrays, and the `stashes` and `characters` listings present as non-arrays, have no case. The code refuses them (`src/lib.rs:466-496`, `1632`, `1826`), but no test fails if one went back to an empty list.
  2. P · Y · `lib.rs:548-569`: a character fetch with every item array absent retires its item (`removed == 1`). `lib.rs:535-539` (a folder with no keys) is weaker.
  3. M · Y · `lib.rs:498-529`: refused, `kept == row`, body kept verbatim, response and item counts unchanged, the substash and the league list stand. It names keys and restates clause 1 and C30.
  4. M · Y · `lib.rs:576-596`: an id-less gem two levels down is refused at `items[0].socketedItems[0]` and nothing lands.
  5. P · Y · `lib.rs:603-621`: a trigger makes the `refused` insert fail; the error is still `MalformedBody`, with `not_kept == true` and the exact words.
  6. P · Y · `lib.rs:492-495`, a case in the table: a stash body with id `b` under a request for `a` is refused.
- unheld: 1 simple · wrong · add rows to the case table in `c145_a_present_key_of_the_wrong_type_is_malformed_and_retires_nothing` (`unit/tests/lib.rs:471`) for an `equipment` non-array and a `stashes` non-array
- reason: Y · `crates/acquisition-store/src/lib.rs:483-486` and test doc `unit/tests/lib.rs:427-432` (the property, and that an empty list would retire every item at the location) · moves to `crates/acquisition-store/unit/tests/lib.rs:427`
- fate: NT · simple
- confidence: high · I did not read every `present_array` caller

### C54 — Authority is the coarser observation's.
- class: mech — the store's liveness and ownership rule, which the planner and search consume through the store's read API; it lives in one crate every frontend inherits
- clauses:
  1. P · Y · held by the sum of clauses 2-7, in the tests below.
  2. P · Y · Membership and liveness: `lib.rs:1994-2004`, an empty listing retires the character and its item. Address and league: `lib.rs:1583-1599`, a late fetch under the old name does not roll back the listing's name or level, and the next block says the listing owns league. `corpus.rs:251-279`, an item's league is the listing's, not its stamp.
  3. P · Y · `lib.rs:1996-2004` and `2061-2072`: a late fetch after retirement gives `items == 0` and `withheld == Some(1)`, the row stays retired, and the item stays removed.
  4. P · Y · `lib.rs:2085-2137` (same tab id under pc and xbox is two locations; the move event names `stash/pc/Standard/t1` and `stash/xbox/Standard/t1`). `corpus.rs:288` is also named for it, but I did not read its body.
  5. P · Y · `lib.rs:2176-2187` (the parent's fetch that drops `s1` retires it and its item) and `2219-2228` (a retired parent's late fetch revives nothing).
  6. M · Y · `lib.rs:2034-2036` and `2075-2077`: a revived row has `fetched_at == None`.
  7. M · Y · `lib.rs:2236-2257`: three responses stamped with the same `at` each get their own removal event, by `response_id`.
- unheld: none
- reason: Prop · `crates/acquisition-store/src/lib.rs:208-214` (the property; the Why, one fault in five places, is not there) · moves to `crates/acquisition-store/unit/tests/lib.rs:1977`
- fate: RM
- confidence: medium · I did not read the body of `corpus.rs:288` (the league part of the full coordinate)

### C56 — Every item row records the array
- class: mech — the store records each item's array as an ingest fact and counts unknown arrays; search reads `container` through the store's read
- clauses:
  1. P · Y · `unit/tests/lib.rs:1703-1708` (containers `equipment`, `guardian`, `skills`); `1722` (swapping identical helms between arrays is two `changed` events); `1759-1764` (`wickerman` is lifted, nothing unlifted); `1736-1739` (a stash item's container is `items`).
  2. P · Y · `lib.rs:1909-1921`: an unknown `pets` array is counted (`unlifted == 2`), not lifted, and not a failure; `_unlifted` is in the json. `crates/acquisition-cli/src/store_cmd.rs:1227-1265`, an in-file unit test, asserts the `drift:` line that `store status` renders.
- unheld: none
- reason: Prop · `crates/acquisition-store/src/lib.rs:194-206` (the property). The Why sits uncited in test doc `unit/tests/lib.rs:1677-1679` ("a guardian's helm and the character's own are distinguishable only by that") and under C55 at `src/lib.rs:173-180`. · moves to `crates/acquisition-store/unit/tests/lib.rs:1677`
- fate: RM
- confidence: high · I did not read the `store_cmd.rs` test's setup lines

### C57 — A PoE2 item-granted skill is a
- class: mech — how the store's ingest treats an id-less granted gem
- clauses:
  1. P · Y · `unit/tests/lib.rs:1801-1818`: `granted == 2`, and no row matches the granted skill or its support.
  2. M · Y · `lib.rs:1822` (the host's json keeps the subtree); `1841-1843` (the envelope keeps `skills[0]`, with `_split` and `_granted` counts); `1848` (status `granted_skills == 2`, `refused_bodies == 0`).
  3. P · Y · `lib.rs:1859-1883`: an id-less gem at the top of `equipment` and an id-less rune socketed in it are each refused, at their positions.
  4. M · N · "holds until GGG changes how granted skills are reported" is a condition on the ruling, not on code. No test could fail on it, and the doc at `src/lib.rs:215-228` does not state it.
- unheld: 4 cut · nothing relies on it: no caller or test reads it, and a change in GGG's shape would surface through clause 3's refusals anyway
- reason: Prop · `crates/acquisition-store/src/lib.rs:215-228` (the property, the shape and the discriminator; not the Why: `Item.id` is optional, and a synthetic id would double the entry) · moves to `crates/acquisition-store/unit/tests/lib.rs:1770`
- fate: NT · cut
- confidence: high · none

### C60 — The store's realm is the request's
- class: mech — the store stamps realm from the request and scopes removal by realm
- clauses:
  1. P · P · `unit/tests/lib.rs:1484-1497`: a sony fetch stamps `realm == "sony"`, and xbox and poe2 listings stamp their rows. Loose half, "never from the entry's `realm` field": no store test puts a `realm` on a listing entry that differs from the request's. The mock (`crates/acquisition-daemon/src/mockggg.rs:505-519`) always agrees with the request. Code that read the entry's field and fell back to the request would pass every test.
  2. P · Y · `lib.rs:1467-1475` (an empty xbox listing retires only the xbox tab) and `1500-1527` (a poe2 character listing leaves pc's character standing).
- unheld: 1 simple · wrong · add a disagreeing `"realm": "pc"` to the poe2 entry in `realms_keep_the_same_league_and_id_apart` (`unit/tests/lib.rs:1448`, the block at 1501-1527)
- reason: Y · `crates/acquisition-store/src/lib.rs:186-193` (stamped from the params; the docs' field contradicts the wire; a realm-R listing retires only realm-R rows), with `src/lib.rs:2017-2020` (under-retires, never over-retires) · moves to `crates/acquisition-store/unit/tests/lib.rs:1443`
- fate: NT · simple
- confidence: high · I did not read the stash-listing removal SQL

### C62 — A character changed when a listing
- class: mech — the planner's freshness rule for a character
- clauses:
  1. P · Y · `crates/acquisition-plan/unit/tests/lib.rs:2466-2479`: a newer listing with different experience gives `ListedExperienceDisagrees { listed: 150, held: 100 }`. `2489-2497`: a listing in the same second is no proof. `2521-2550`: a league move forces a fetch in the new league.
  2. M · N · "sibling of a tab's count arm" is a description. No test, but it is stated at `crates/acquisition-plan/src/lib.rs:189-195` (`ListedCountDisagrees` beside `ListedExperienceDisagrees`).
  3. M · Y · one `Selection` type serves both facets (`plan/src/lib.rs:183-186`), so a separate type per facet would be a visible edit. "Liveness consumed, not restated" is stated in the same doc. The test `policy_v3_covers_per_facet...` (`plan/unit/tests/lib.rs:2299`) exercises both facets.
- unheld: 2 cut · nothing relies on it: it is wording about the code, already stated at `plan/src/lib.rs:189`
- reason: Prop · `crates/acquisition-plan/src/lib.rs:183-195` and `2387-2391` ("proves play since"; "the cheapest evidence" is not said) · moves to `crates/acquisition-plan/unit/tests/lib.rs:2443`
- fate: RM
- confidence: medium · I did not read the end of the league-move test past line 2550

### C63 — `acq characters` and the MCP's `characters`
- class: design — what a verb and an MCP tool print, so a human can write a policy by id
- clauses:
  1. P · P · `crates/acquisition-cli/tests/store_answers.rs:119` asserts `rows[1]["id"] == "c01"` in `store characters --json`. The MCP's `characters` tool returns the same `acquisition_verbs::Characters` document (`crates/acquisition-mcp/src/server.rs:198-215`), so the JSON half is held at the lowest crate. Loose half: the text table's id column (`crates/acquisition-cli/src/store_cmd.rs:668-682`) is asserted nowhere. The text test (`store_answers.rs:89-103`) counts rows and checks the footer, but not the id. `crates/acquisition-mcp/tests/search_tools.rs:1437-1447` checks counts only.
  2. P · N · I looked in `crates/acquisition-cli/tests/plan_json.rs` (the C36 `policy set` test at 356), `crates/acquisition-plan/unit/tests/lib.rs` and the MCP tests. No test sets a policy naming a character by name and reads back the stored value unchanged.
- unheld: 1 simple · para · add an `id` assertion on the text rows in `store_characters_lists_ten_beside_its_counts_and_prints_the_command_for_all` (`crates/acquisition-cli/tests/store_answers.rs:69`)
- unheld: 2 simple · para · add a `policy set` with a character name, then read the stored policy back, to the C36 test in `crates/acquisition-cli/tests/plan_json.rs:356`
- reason: Prop · `crates/acquisition-store/src/lib.rs:207` (four words, in a crate that does not print) · moves to `crates/acquisition-cli/tests/store_answers.rs:64`
- fate: NT · simple
- confidence: medium · I did not read the `policy set` code path or the C36 test body

### C188 — The store reads the game version
- class: mech — the store's search read gives each realm's newest stated release; search renders it
- clauses:
  1. P · Y · `crates/acquisition-store/unit/tests/corpus.rs:625-629`: `games` is `[pc 3.30.0c@25, sony 3.29.0@10]` under All, and sony alone under One. "Storing nothing" is not tested; it is stated at `src/corpus.rs:67`.
  2. P · Y · `corpus.rs:652-658`: `release` of `3.30.0b` and of `3.30.0.2.2` is `(3, 30)`, of `0.4` is `(0, 4)`, and of malformed versions is None. Search uses `acquisition_store::release`, so another crate relies on it.
  3. M · N · "GGG states its game only on a character response" is a fact about GGG. No test could fail on it; it is stated at `src/corpus.rs:64`. It is not a numbered claim in `docs/design/network-ground-truth.md`.
  4. P · Y · `corpus.rs:590-593` and `615-628`: an older release fetched later does not displace the newest, and the version and time given are the most recent fetch of that release. `631-647`: a retired-then-revived character is left out.
  5. P · Y · `corpus.rs:601-608`: characters with no version, a numeric version, or an unparseable one are left out and fail nothing. "No item body is read" is an M half, stated at `src/corpus.rs:71`; the SQL at `src/corpus.rs:407-412` reads `characters` alone. Search's `crates/acquisition-search/tests/answer.rs:446-451` asserts the scope's `store_games`.
- unheld: 3 cut · nothing relies on it: it is a GGG fact, stated at `src/corpus.rs:64`, whose home is the ground truth
- reason: Prop · `crates/acquisition-store/src/corpus.rs:62-71` (the property, and "GGG's one statement of its game"; not the reworded-line Why) · moves to `crates/acquisition-store/unit/tests/corpus.rs:567`
- fate: RM
- confidence: high · I did not read past `answer.rs:457`

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C29 | mech | 1 | 2 | 1/1/0/0/1 | Prop | NT | 1/0/0 | 0/1 | medium |
| C145 | mech | 4 | 2 | 5/1/0/0/0 | Y | NT | 1/0/0 | 1/0 | high |
| C54 | mech | 5 | 2 | 7/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C56 | mech | 2 | 0 | 2/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C57 | mech | 2 | 2 | 3/0/1/0/0 | Prop | NT | 0/0/1 | 0/0 | high |
| C60 | mech | 2 | 0 | 1/1/0/0/0 | Y | NT | 1/0/0 | 1/0 | high |
| C62 | mech | 1 | 2 | 2/0/1/0/0 | Prop | RM | 0/0/1 | 0/0 | medium |
| C63 | design | 2 | 0 | 0/1/1/0/0 | Prop | NT | 2/0/0 | 0/2 | medium |
| C188 | mech | 4 | 1 | 4/0/1/0/0 | Prop | RM | 0/0/1 | 0/0 | high |

## What I did not check
- The body of `corpus.rs:288` (`only_live_items_at_live_locations_by_full_coordinate_in_the_scope_asked`), which I took as the league part of C54 clause 4 by its doc alone.
- The `policy set` code path, and whether a typed name is stored verbatim today (C63 clause 2). I found no test either way, but I did not read the code.
- `rebuild_reextracts_columns` (`unit/tests/lib.rs:851`), and whether `rebuild` depends on the envelope being exactly as received (C29 clause 1's worth).
- Whether CI runs anything beyond the gate's nextest for these crates; I assumed every test named runs under nextest in the gate.

## Noticed
- C29, the fate rule: an entry whose clauses are all M (the reading I first had for C29) satisfies rule 2 ("every P clause is U") vacuously before rule 3. Read in order, the rules would give SH-KU to an entry that is fully held. I gave C29 a P clause (its headline), so it did not bite here, but the fate rules do not decide a zero-P entry.
- C60, the *Pinned:* line: `realms_keep_the_same_league_and_id_apart` holds the realm scoping, but not the "never from the entry's `realm` field" that the headline promises. No listing entry in the test carries a `realm` field at all.
- C145, the *Pinned:* line: "the `c145_` tests" cover four of the eleven-plus seams `array_missing` names (`src/lib.rs:466-481`). The listings' own arrays (`stashes`, `characters`) are refused by a separate path (`src/lib.rs:1632`, `1826`) that no test reaches.
- C63: the only code citation is a bare line in the store's module doc (`crates/acquisition-store/src/lib.rs:207`). The printing lives in `acquisition-cli` and `acquisition-verbs`, which do not cite the id. The entry says "Pinned: nothing yet", yet `crates/acquisition-cli/tests/store_answers.rs:119` holds the JSON half.
- C56: the reason sentence exists in test doc `unit/tests/lib.rs:1677-1679`, but that doc does not cite C56, and `show` does not find it.
- C188 clause 3 is a claim about GGG that the registry rule says belongs in `docs/design/network-ground-truth.md`, where no numbered claim states it.
