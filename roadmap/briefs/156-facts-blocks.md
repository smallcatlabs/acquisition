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
listed at the close for the owner's review. Cut twice: the first
draft (`aa2e6f88`) was reviewed cold
(`roadmap/reviews/156-facts-blocks-brief-aa2e6f88.md`, the prompt
beside: eight findings, one nitpick); this cut answers each, listed
at the foot under "Taken from the review". To be reviewed cold again
before a worker reads it.

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
  `character_listing`, their ages set by `status` alone, 852–861;
  flattened into the report); `ListingReport` (584–592); `resolve`
  (1564–1790: every snapshot tab but a folder, 1602, substashes and
  characters included, league-less characters included); `status`
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
  `basis:`). `search_cmd.rs:1054` (round 2's rule for the one-location
  form: the extremes' locations equal, never a count).
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
  locations: every container `resolve` lists (every snapshot tab but
  a folder, substashes among them, every character, a league-less
  character among them, a never-listed one among them), each once,
  with its snapshot `fetched_at` (not its `Basis.at`, which is its
  listing time). `places` counts the subject, `fetched` those with a
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
  - a status over a league never listed, nothing on record:
    `places` 0, no fetch, `list_seen` null and `never`; the text's
    zero-item return (285) prints `facts: nothing listed`; a league
    with a character listing and no stash listing: `list_seen` the
    character listing's;
  - the `resolve()` fixture (`listing.rs:710`): `facts` present
    unaged (epochs, no `_age`); the populated `status()` serialization
    pinned with its words.
- The last push's CI, on `main` at `e79796e1`: the runs named at cut
  time; a red run after a push is the group's first concern.

## Do

1. The shop render: `Freshness` gains `facts: Option<Facts>`, `Some`
   iff `posted_total > 0`, whatever `policy` says, filled before the
   `not_set` and `unreadable` returns (1128–1135): `places`,
   `fetched`, and the span (`coverage::span` over `(seen_at,
   FetchedLocation { kind, id, name, league })` pairs, one per posted
   location, the name as the page names it) with the bases naming
   the set (the header's `stash_listing` where a tab is among the
   places, `character_listing` where a character is). The stale set
   and `oldest_stale_seconds` are untouched. `Facts` is one struct in
   the plan crate (`places`, `fetched`, `#[serde(flatten)] span:
   FetchSpan`), shared by Do 2.
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
   …); stash listing 53d ago, characters listed 53d ago`; at the
   zero-item returns (285, 320) `facts: nothing listed` or the lists'
   ages where a listing exists; under `--expand` the `basis:` line
   stays. The stale line's "oldest" becomes `oldest covered stale item
   …` so the two oldests read apart. The harness's live-world mask
   covers a `facts:` line's clause (form and places) as a `total `
   line's.
4. The glosses: `freshness.facts` and `facts` in `describe` and the
   two tools' text, each naming its set ("the posted items'
   locations"; "the containers the facts were read from, every one
   the report lists"), that the fetches are the locations' last
   fetches and `list_seen` the oldest of the lists naming them; no id
   in either (C116). `MCP-REFERENCE.md` regenerated for the prose.
5. The pins: in `unit/tests/shop.rs`, one test per shop input, on the
   `Freshness` value, and a case in `c72_…` holding the stale set's
   oldest and `facts`'s oldest apart; in `unit/tests/listing.rs`, the
   status inputs and the aged serialization; the two inline tests of
   Read (`shop_cmd.rs:528`, `price_cmd.rs:1234`) moved to the new line
   positions and wordings; the two reference fixtures regenerated for
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
  `posted_total > 0` under every policy state; `ListingReport.facts`
  is built in `resolve` from snapshot fetch times and aged in
  `status`; `list` and `show` carry none.
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

- The render's listing header and the report's bases aged
  (`taken_at_age` and the bases' ages on `listing`, on the report and
  `list`): round 3; any epoch's words beyond `facts`'s own.
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

## Taken from the review

The cold review of `aa2e6f88`
(`roadmap/reviews/156-facts-blocks-brief-aa2e6f88.md`): eight
findings, one a nitpick, each answered in this cut.

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
