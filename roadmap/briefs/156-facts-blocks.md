# 156-facts-blocks — the shop render's and price status's own facts (#156)

Round 4 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§4 "The idea" (the shop's and price status's subjects), "Where it
strains" (the shop's shape), and §10 step S3), written by the
orchestrating session on 2026-11-04 against the tree at `2977ea2c`
(round 2 closed; every line number below from it; round 3, if it
lands first, ages the render's listing header and the report's bases
and shifts lines in `listing.rs` and `shop.rs` without touching what
this step adds: the worker reads the tree and compares fixture
changes against the immediate parent). Its rulings are C118 as
amended and C162 (`f3db3427`); the JSON shape in Do 1 and Do 2 and
the text's words in Do 3 are the session's agent-facing design,
listed at the close for the owner's review. Cut three times: the
first draft (`aa2e6f88`) and the re-cut (`0d287307`) were each
reviewed cold (`roadmap/reviews/156-facts-blocks-brief-<sha>.md`, the
prompts beside: eight findings then six, one nitpick in all); this
cut answers the second review's six, listed at the foot under "Taken
from the reviews" with the first's eight; not reviewed a third time,
the roadmap's rule.

## Goal

When this step is done, a shop render and a price status each say how
old the facts they rest on are, by the store's one derivation (C162)
over their own subject. The render's `freshness` carries, whatever
the policy says, a `facts` block over the posted items' locations:
how many, how many fetched, their oldest and newest fetch with
locations and ages, and the oldest of the lists that name them; its
text prints that line under every policy state, naming the stash
listing the ranks come from and the character listing where a
character is posted. A price status carries a `facts` block over the
report's locations, the containers the facts were read from: how
many, how many fetched, the fetched extremes, and the lists seen; its
decision view prints it, where today the ages are in the JSON and
under `--expand` alone. #156 closes by its own line: "a render's
freshness carries the listing's and the posted items' ages whether or
not a policy is set, on both surfaces, pinned in the render's
goldens."

Why it matters: with no policy the render says nothing of its
positions' age while its ranks come from a listing fifty days old
(#156, MCP-2 call 52 against call 39); `price status`'s decision view
has no age at all, so a reader assumes current (the map §3, Root 1).
The block is the function round 2 built, applied to two more sets.

## Read

- The map §4 ("a shop render's subject is the posted entries'
  places"; "a price status's subject is the league's items", read
  here as the containers the report lists, Input's definition; "The
  shop's shape": a block over posted entries has no never-fetched
  half, so the block is a count and the span, not a whole `Coverage`)
  and §10 S3.
- `decisions/pricing.md`: C72 (the shop names coverage, staleness and
  positions with their remedies, reported never enforced), C74 (the
  render is a derivation; every omission counted with a reason).
  `decisions/plans.md`: C77 (freshness is judged at compile against
  the window). `decisions/search.md`: C118 as amended.
  `decisions/frontends.md`: C162, C53 (the decision view carries what
  changes the next decision: the facts' age decides whether to
  refresh; ages in words beside epochs), C146 (a count says what it
  counts), C149. `AGENTS.md`: C116, C122, C154.
- `crates/acquisition-store/src/coverage.rs`: `FetchSpan` and `span`
  as round 2 built them (`(i64, FetchedLocation)` pairs and the bases
  that name the set; the tie rule; `list_seen` the minimum over the
  bases given, `:252`; `aged`).
- `crates/acquisition-store/src/snapshot.rs:60–70` (`TabSnapshot`:
  `listed_at`, `listed_response`, `fetched_at`; `None` fetched
  directly, never listed) and `:86–95` (`CharacterSnapshot`, the
  same; `league` `None` for a league-less character); `:605` (the
  snapshot admits directly fetched, never-listed locations).
- `crates/acquisition-plan/src/listing.rs`: `Basis` (414–422: a
  listing's `response` and `at`, which for a container is its
  `listed_at`, 1616 and 1637, never its fetch); `Listing` (427–440);
  `ReportHeader` (553–575: `taken_at`, `stash_listing`,
  `character_listing`, their ages set by `status` alone, 852–861,
  and initialized `None` by `resolve`, 1784–1785; flattened into the
  report); `ListingReport` (584–592); `resolve` (1564–1790: every
  snapshot tab, folders among them, 1619, the check at 1602 only
  keeping folders out of the priced-tab count; substashes, characters,
  league-less characters included; `counts.containers` counts the
  folders too, 1642); `status`
  (852–861: the listings dropped at 841–855); `list_view` and
  `show` copy the header and nothing else of the report (921–925,
  954–958), so a report field never reaches them.
- `crates/acquisition-plan/src/shop.rs`: `Freshness` (427–475: its
  states; the stale set's `oldest_stale_seconds` is the covered stale
  items' and stays, `:1181–1187`); `ShopRender` (517–545);
  `Entry` (599–616: `location`, `parent`, `seen_response`,
  `seen_at`, filled at 794–795 from each posted item's listing basis:
  an item's sighting, which shares the ingest response with its
  location's fetch, `store/src/lib.rs:1613`, `:1764`, `:1941`, `:2892`;
  a substash's own fetch, and substashes are blocked from posting,
  `shop.rs:734`); `render` (802) and `freshness` (1101–1196: the
  `not_set` and `unreadable` returns before any age, 1128–1135).
- `crates/acquisition-cli/src/shop_cmd.rs`: `freshness_lines`
  (135–215: nothing when nothing is posted, 138–140; one line per
  policy state; the stale line's "oldest", 202); its inline test
  `c53_shop_render_opens_with_one_line_and_labels_every_page` (528:
  fixed line positions and count, the stale and no-policy wordings).
  `crates/acquisition-cli/src/price_cmd.rs`: `render_status`
  (278–440: the zero-item returns at 285 and 320; the counts line; the
  `basis:` line under `--expand`, 399–424); its inline test
  `c53_price_status_opens_with_one_line_and_ends_with_the_next_action`
  (1234: positions, line count, the empty cases, the expanded
  `basis:`) and `c35_an_empty_league_still_reports_its_rows` (1701:
  the whole default text of an empty league compared at 1717, which
  gains the facts line). `search_cmd.rs:1054` (round 2's rule for the
  one-location form: the extremes' locations equal, never a count).
  `crates/acquisition-plan/src/pricing.rs:991` (the render is given a
  raw resolved report, so its header's two listing ages are `None`
  unless this step ages them, Do 1).
- `crates/acquisition-store/src/coverage.rs:30` and `:98`:
  `FetchedLocation` and `FetchSpan` derive `Serialize` alone, where
  `Freshness` (`shop.rs:426`) and `ListingReport` (`listing.rs:583`)
  derive `Deserialize` and their reference tests read the documents
  back (`unit/tests/shop.rs:926`, `unit/tests/listing.rs:724`): the
  two gain `Deserialize` here, additive.
- `crates/acquisition-store/src/snapshot.rs:243`, `:503` and
  `crates/acquisition-cli/tests/new_league.rs:125`: a league with no
  items can still hold listed, fetched, empty locations, and a
  directly fetched location can exist with no listing at all.
- `crates/acquisition-cli/tests/harness/mod.rs:1068` (the age rule
  recognises a number immediately before ` ago`: each extreme prints
  with its own `ago`) and `:656–683` (the live-world mask, extended to
  `facts:` lines as to `total ` lines).
- The MCP: `crates/acquisition-mcp/src/main.rs` (`price_status`,
  `shop_render`; their `describe` entries and glosses, grep
  `freshness` and `basis`); `crates/acquisition-mcp/tests/mcp_reference.rs:38`,
  `:77` (`the_reference_matches_the_server`: `MCP-REFERENCE.md` is
  regenerated when a tool's prose changes, Do 4's kind).
- Every test and golden that asserts on these texts, each changed by
  this step: `crates/acquisition-plan/unit/tests/shop.rs:670`, `:695`,
  `:893` (the reference `shop-render-schema-5.json`, regenerated for
  an added subtree without a bump, `:888`);
  `crates/acquisition-plan/unit/tests/listing.rs:705` (the reference
  `listing-report-schema-3.json` serializes `resolve()`, not
  `status()`: it gains the unaged `facts` subtree, `:700`);
  `crates/acquisition-cli/tests/shop_json.rs:158`, `:321`;
  `price_json.rs:141`; `seat_faults.rs:142` with
  `goldens/seat_faults/price_status_other_leagues.txt:3`;
  `new_league.rs:44` with `goldens/new_league/launch_morning.txt:166`;
  `journey.rs:45` with `goldens/journey/a_first_session.txt:243`;
  `crates/acquisition-mcp/tests/pricing_tools.rs:463`; and whatever a
  grep by command finds that this list misses (`grep -rn 'price status\|shop render' crates/*/tests crates/*/src --include=*.rs`,
  then each test's goldens), the singular `1 item in` among them.
- `REVIEW-CHECKLIST.md`, "Pricing and intent" and "Frontends".

## Input

- The rulings, above. No new ruling.
- The subjects, defined: the render's is the posted items' locations
  (each tab or character with a posted item, once; never a folder, a
  substash is blocked from posting); the status's is the report's
  non-folder containers: every snapshot tab but a folder (substashes
  among them), every character (a league-less one and a never-listed
  one among them), each once, with its snapshot `fetched_at` (not its
  `Basis.at`, which is its listing time); `counts.containers` keeps
  counting the folders and is not this count. `places` counts the subject, `fetched` those with a
  fetch, the span runs over the fetched, and `list_seen` is the
  oldest of the bases that name the subject (the stash listing where
  a tab is in it, the character listing where a character is), null
  and `never` where neither exists.
- The failing inputs the step's tests stage, in the plan crate's
  fixtures (`unit/tests/shop.rs`'s world, `listing.rs`'s `snapshot()`)
  and the CLI's seeded worlds:
  - a render under `not_set` with posted items in two tabs fetched at
    different times and a stash listing older than both:
    `freshness.facts` carries `places` 2, `fetched` 2, the two
    fetches with their tabs and ages, `list_seen` the listing's; the
    text prints the line naming the stash listing (the fact #156
    wanted);
  - the same under `covers`: the block beside the stale line, the
    stale set's `oldest_stale_seconds` unchanged (the covered stale
    items' oldest, a different set: `shop.rs:1181–1187` and the test
    at `unit/tests/shop.rs:725`), the two oldests told apart in words;
  - posted items in one tab and on one character, the character
    listing older than the stash listing: `list_seen` the character
    listing's, the text naming both listings each with its own age;
  - posted items on a character alone: no stash basis among the set's
    bases, `list_seen` the character listing's, the text naming the
    character listing alone;
  - a render with nothing posted: no block and no line, as today
    (`shop_cmd.rs:138–140`);
  - a status over a league with two tabs fetched at different times,
    one never-fetched tab and a league-less character never fetched:
    `places` 4, `fetched` 2, the extremes the two tabs', `list_seen`
    the older of the two bases; the decision view prints the line;
    `--expand`'s `basis:` line stays;
  - a status over a league with no items but two listed tabs fetched
    empty: `places` 2, `fetched` 2, the span theirs; the zero-item
    return (285, 320) prints the same facts line as the full view; a
    league with no items, no listing and one directly fetched empty
    tab: `places` 1, `fetched` 1, `list_seen` null and `never`; a
    league never listed with nothing on record: `places` 0, no fetch,
    `list_seen` null, the text `facts: nothing listed`; a league with a
    character listing and no stash listing: `list_seen` the character
    listing's;
  - a render with posted items in one tab and on one character, the
    header's two listing ages worded (Do 1): `listing.stash_listing_fetched_at_age`
    and `listing.character_listing_fetched_at_age` present, each the
    text's age;
  - the `resolve()` fixture (`listing.rs:710`): `facts` present
    unaged (epochs, no `_age`); the populated `status()` serialization
    pinned with its words.
- The last push's CI, on `main` at `0d287307`: Gate (Windows)
  `41783762148`, success; Gate (Linux) `41783761945`, in progress at
  this cut, named and the step goes on; the head before, `e79796e1`,
  Windows `41783438036` success and Linux `41783438169` cancelled by
  the next push. A red run after a push is the group's first concern.

## Do

1. The shop render: `Freshness` gains `facts: Option<Facts>`, `Some`
   iff `posted_total > 0`, whatever `policy` says, filled before the
   `not_set` and `unreadable` returns (1128–1135): `places`,
   `fetched`, and the span (`coverage::span` over `(seen_at,
   FetchedLocation { kind, id, name, league })` pairs, one per posted
   location, the name as the page names it) with the bases naming
   the set (the header's `stash_listing` where a tab is among the
   places, `character_listing` where a character is). The stale set
   and `oldest_stale_seconds` are untouched; `facts` is
   `skip_serializing_if = "Option::is_none"`, so absence is omission.
   `Facts` is one struct in the plan crate (`places`, `fetched`,
   `#[serde(flatten)] span: FetchSpan`), `Serialize` and
   `Deserialize`, shared by Do 2; `FetchSpan` and `FetchedLocation`
   gain `Deserialize` in the store, additive. The render's header
   gains its two listing ages here (`stash_listing_fetched_at_age`,
   `character_listing_fetched_at_age`, as `status` sets them, 852–861)
   where round 3 has not landed first; where it has, nothing to do:
   the text of Do 3 prints ages the JSON holds (C53).
2. Price status: `ListingReport` gains `facts: Facts` at its top
   level, built in `resolve` from the snapshot's containers and their
   `fetched_at` (never from `Basis.at`), unaged; `status()` ages it
   beside the header's ages. `list_view` and `show` copy the header
   and not the field, as today, so neither carries `facts`. The
   reference fixture serializes `resolve()`'s unaged `facts`; a test
   pins `status()`'s aged serialization.
3. The text, one line, its form chosen from `facts`'s own fields as
   round 2's total line chooses (`search_cmd.rs:1054`): where the
   extremes name two locations, `(the oldest fetch 53d ago, Dump Tab;
   the newest 50d ago, Allflame / Pat (character))`; where they name
   one, `(fetched 53d ago, Dump Tab)`; where none is fetched, `none
   fetched`; each extreme with its own `ago`. The render
   (`freshness_lines`, first, under every policy state when something
   is posted): `facts: 31 posted items at 3 locations, 2 fetched (the
   oldest fetch …; the newest …); stash listing 50d ago, characters
   listed 49d ago`, each listing named only where its kind is among
   the places. The status (`render_status`, after the counts line):
   `facts: 546 locations, 405 fetched (the oldest fetch …; the newest
   …); stash listing 53d ago, characters listed 53d ago`; the same
   line at the zero-item returns (285, 320) whenever `facts.places >
   0`, since no items is not no locations, and `facts: nothing listed`
   only where `places` is 0 and neither listing exists, the lists'
   ages alone where a listing exists and no location; under `--expand`
   the `basis:` line stays. The stale line's "oldest" becomes `oldest covered stale item
   …` so the two oldests read apart. The harness's live-world mask
   covers a `facts:` line's clause (form and places) as a `total `
   line's.
4. The glosses: `freshness.facts` and `facts` in `describe` and the
   two tools' text, each naming its set ("the posted items'
   locations"; "the report's non-folder containers, the locations the
   facts were read from"), that the fetches are the locations' last
   fetches and `list_seen` the oldest of the lists naming them; no id
   in either (C116). `MCP-REFERENCE.md` regenerated for the prose.
5. The pins: in `unit/tests/shop.rs`, one test per shop input, on the
   `Freshness` value, and a case in `c72_…` holding the stale set's
   oldest and `facts`'s oldest apart; in `unit/tests/listing.rs`, the
   status inputs and the aged serialization; the three inline tests
   of Read (`shop_cmd.rs:528`, `price_cmd.rs:1234`, `price_cmd.rs:1701`)
   moved to the new line positions and wordings, the last keeping its
   row visibility and next action; the two reference fixtures
   regenerated for
   the added subtree; in `shop_json.rs`, `price_json.rs`,
   `seat_faults.rs`, `new_league.rs` and `journey.rs`, the text line
   as a golden fact under `ACQ_UPDATE_FIXTURES=1`, every changed line
   read: a `facts:` line, a `# holds` line, an added JSON subtree, the
   stale line's new word, or the MCP reference's prose for the two
   tools; the MCP's `pricing_tools.rs` asserting `freshness.facts`
   under `not_set`; a normaliser case for the live mask.
6. The gate; the commit, naming this brief, the changed lines by kind,
   and `retired nothing`. #156 closes.

## Done when

- `Facts` exists in the plan crate; `Freshness.facts` is `Some` iff
  `posted_total > 0` under every policy state and absent otherwise;
  `ListingReport.facts` is built in `resolve` from snapshot fetch
  times over the non-folder containers and aged in `status`; `list`
  and `show` carry none; the render's header carries its two listing
  ages; the reference documents round-trip through `Deserialize`.
- The shop inputs and the status inputs of Input are each a test; the
  `c72_…` case holds the two oldests apart; the aged status
  serialization is pinned; the MCP test holds the block under
  `not_set`.
- Every regenerated line is one of Do 5's six kinds; the two reference
  fixtures' diffs are the added subtree.
- `describe`'s and the tools' entries name each block's set, no id in
  the text.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- The report's and `list`'s bases aged, the render's `taken_at` and
  `rendered_at` worded, and every other epoch's words beyond
  `facts`'s own and the render header's two listing ages: round 3.
- The stale set's measure, the positions-before-listing rule, the
  policy's remedies: C72's, unchanged but for the one word.
- `price list` and `price show`: a selection's and one item's views,
  whose own bases they print; no block.
- The store's leagues beside `price status` and `shop render` (#157,
  #19): round 5.

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names; never while another round's worker builds.
- `ACQ_UPDATE_FIXTURES=1` runs once, after Do 4, every changed line
  read before the commit.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the regenerated files by directory, the
changed lines by kind, and the test count before and after.

## Taken from the reviews

The second cold review, of `0d287307`
(`roadmap/reviews/156-facts-blocks-brief-0d287307.md`): six findings,
none a nitpick, each answered in this cut.

1. The render's text prints two listing ages, so their words join the
   render's header here where round 3 has not landed first (C53).
2. No items is not no locations: the facts line prints whenever
   `places > 0`, the zero-item returns included; `nothing listed` only
   for no place and no listing; fetched-empty cases staged.
3. `FetchSpan` and `FetchedLocation` gain `Deserialize`, `Facts` both,
   `Freshness.facts` skipped when none, the round-trips kept.
4. `c35_an_empty_league_still_reports_its_rows` named among the
   holders.
5. The gloss says "the report's non-folder containers"; Read says
   what `resolve` does with folders.
6. Both gates' runs named with their ids and conclusions.

The first cold review, of `aa2e6f88`
(`roadmap/reviews/156-facts-blocks-brief-aa2e6f88.md`): eight
findings, one a nitpick, each answered in the second cut.

1. A container's `Basis.at` is its listing time: the status span runs
   over the snapshot's `fetched_at`, carried through `resolve`.
2. The status subject is defined: the report's locations, every
   container `resolve` lists, folders excluded, substashes,
   league-less and never-listed ones included, counted as `places`
   with `fetched` apart.
3. `list_seen` is the oldest of the bases naming the set; the text
   names each listing that applies with its own age; the neither-listed
   case is null and `never`; mixed, character-only and empty cases
   are staged.
4. The forms are chosen by the extremes' equality as round 2's are,
   `fetched` and `places` are both said, the zero-item returns print
   their own line, the stale line's oldest is "oldest covered stale
   item".
5. Each extreme prints with its own `ago`; the live mask covers
   `facts:` lines with a normaliser case.
6. The six holders the brief missed are named, the two inline
   positional tests among them, and the grep is by command.
7. The MCP reference's prose and an added subtree are regeneration
   kinds; the listing reference holds `resolve()`'s unaged `facts`
   and a test pins `status()`'s; `facts` sits outside `ReportHeader`.
8. C77 is cited from `decisions/plans.md`.
