# 156-facts-blocks — the shop render's and price status's own facts (#156)

Round 4 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§4 "The idea" (the shop's and price status's subjects), "Where it
strains" (the shop's shape), and §10 step S3), written by the
orchestrating session on 2026-11-03 against the tree at `2977ea2c`
(round 2 closed; every line number below from it; round 3, if it
lands first, ages the render's listing header and shifts lines in
`listing.rs` and `shop.rs` without touching what this step adds: the
worker reads the tree). Its rulings are C118 as amended and C162
(`f3db3427`); the JSON shape in Do 1 and Do 2 and the text's words in
Do 3 are the session's agent-facing design, listed at the close for
the owner's review. To be reviewed cold before a worker reads it.

## Goal

When this step is done, a shop render and a price status each say how
old the facts they rest on are, by the store's one derivation (C162)
over their own subject: the render's `freshness` carries, whatever the
policy says, a `facts` block over the posted items' locations (how
many, their oldest and newest fetch with locations and ages, and the
stash listing their ranks come from, `list_seen`), and its text prints
that line under every policy state; a price status carries a `facts`
block over the league's listed containers and its decision view
prints it, where today the ages are in the JSON and under `--expand`
alone. #156 closes by its own line: "a render's freshness carries the
listing's and the posted items' ages whether or not a policy is set,
on both surfaces, pinned in the render's goldens."

Why it matters: with no policy the render says nothing of its
positions' age while its ranks come from a listing fifty days old
(#156, MCP-2 call 52 against call 39); `price status`'s decision view
has no age at all, so a reader assumes current (the map §3, Root 1).
Both already hold the facts (`Entry.seen_at`, `ReportHeader`'s bases);
the block is the function round 2 built, applied to two more sets.

## Read

- The map §4 ("a shop render's subject is the posted entries' places";
  "a price status's subject is the league's items"; "The shop's shape":
  a block over posted entries has no never-fetched half, so the block
  is the span and a count, not a whole `Coverage`) and §10 S3.
- `decisions/pricing.md`: C72 (the shop names coverage, staleness and
  positions with their remedies, reported never enforced), C74 (the
  render is a derivation; every omission counted with a reason), C77
  (freshness is judged at compile against the window). `decisions/search.md`:
  C118 as amended. `decisions/frontends.md`: C162, C53 (the decision
  view carries what changes the next decision; ages in words beside
  epochs), C146 (a count says what it counts), C149. `AGENTS.md`:
  C116, C122, C154.
- `crates/acquisition-store/src/coverage.rs`: `FetchSpan` and `span`
  as round 2 built them (`(i64, FetchedLocation)` pairs and the bases
  that name the set; the tie rule; `aged`).
- `crates/acquisition-plan/src/shop.rs`: `Freshness` (427–475: its
  states `not_set`, `unreadable`, `league_not_covered`, `covers`; the
  stale set's `oldest_stale_seconds` is the covered stale items' and
  stays, `:1181–1187`); `ShopRender` (517–545: `listing: ReportHeader`,
  `posted`, `posted_total`, `freshness`); `Entry` (599–616: `location`,
  `parent`, `seen_response`, `seen_at`, filled at 794–795 from each
  listing's `basis`); `render` (802) and `freshness` (1101–1196: the
  `not_set` and `unreadable` returns before any age, 1128–1135; the
  positions before the listing, 1119–1126, which stay).
- `crates/acquisition-plan/src/listing.rs`: `Basis` (414–422: a
  listing's `response` and `at`, the fact's response time); `Listing`
  (427–440: `subject`, `chain`, the container a subject sits in);
  `ReportHeader` (553–575: `taken_at`, `stash_listing`,
  `character_listing`, their ages set by `status` alone, 848–861);
  `ListingReport` (584–592: `header`, `counts`, `rows`, `listings`);
  `status` (852–861).
- `crates/acquisition-cli/src/shop_cmd.rs`: `freshness_lines`
  (135–215: returns nothing when nothing is posted, 138–140; one line
  per policy state; the stale line's `ago`; the positions line).
  `crates/acquisition-cli/src/price_cmd.rs`: `render_status` (278–440:
  the decision view's lines; the `basis:` line under `--expand` alone,
  399–424).
- The MCP: `crates/acquisition-mcp/src/main.rs` (`price_status`,
  `shop_render` return the same documents; their `describe` entries
  and glosses, grep `freshness` and `basis`).
- The tests and goldens that pin these documents, each changed by this
  step: `crates/acquisition-plan/unit/tests/shop.rs:670`
  (`c53_the_oldest_stale_age_stands_in_words_beside_its_seconds`),
  `:695` (`c72_coverage_staleness_and_positions_are_reported_never_enforced`),
  `:893` (`the_render_json_matches_the_committed_fixture`, the
  reference `shop-render-schema-5.json`, regenerated for added keys
  without a bump); `crates/acquisition-plan/unit/tests/listing.rs:705`
  (`listing-report-schema-3.json`, the same); `crates/acquisition-cli/tests/shop_json.rs:158`
  and `:321`, `price_json.rs:141`; `crates/acquisition-mcp/tests/pricing_tools.rs:463`;
  the goldens `shop_json/shop_render.txt`, `price_json/*.txt`, and
  every golden holding a `price status` or `shop render` text
  (`grep -rl 'policy: none set\|items in Standard' crates/*/tests/goldens`).
- `REVIEW-CHECKLIST.md`, "Pricing and intent" and "Frontends".

## Input

- The rulings, above. No new ruling.
- The failing inputs the step's tests stage, in the plan crate's
  fixtures (`unit/tests/shop.rs`'s world, `listing.rs`'s `snapshot()`)
  and the CLI's seeded worlds:
  - a render under `not_set` with posted items in two tabs fetched at
    different times and a stash listing older than both:
    `freshness.facts` carries `places` 2, the two fetches with their
    tabs and ages, `list_seen` the listing's; the text prints the line
    (the fact #156 wanted);
  - the same under `covers`: the block beside the stale line, the
    stale set's `oldest_stale_seconds` unchanged (the covered stale
    items' oldest, a different set: `shop.rs:1181–1187` and the test
    at `unit/tests/shop.rs:725`);
  - a render with nothing posted: no block and no line, as today
    (`shop_cmd.rs:138–140`);
  - posted items on a character: its place a character, named as a
    row's place is;
  - a price status over a league with containers fetched at two times
    and one never fetched: `facts` carries `places` (the listed
    containers), the fetched extremes, `list_seen` the older of the
    two bases; the decision view prints the line; `--expand`'s
    `basis:` line stays;
  - a price status over a league never listed: `facts` with `places`
    0, no fetch, `list_seen` null and `never`; the text says so.
- The last push's CI, on `main` at `f64dfd0c`: both gates success;
  later pushes' runs named at cut time.

## Do

1. The shop render: `Freshness` gains `facts`, present whenever
   `posted_total > 0` and absent otherwise, whatever `policy` says:
   `places` (the posted items' locations, each once, keyed by
   `location`) and the span over those with a `seen_at`
   (`coverage::span` over `(seen_at, FetchedLocation { kind, id,
   name, league })` pairs built from each posted entry's location, the
   name as the page names it), with the bases naming the set: the
   header's `stash_listing` where a tab is among the places and
   `character_listing` where a character is. `freshness`'s `not_set`
   and `unreadable` returns (1128–1135) fill `facts` first. The stale
   set and `oldest_stale_seconds` are untouched.
2. Price status: `ListingReport` gains `facts` on the status document
   (not on `list`, not on `show`): `places` the league's listed
   containers (every `Listing` whose subject is a container), the span
   over those with a basis `at`, the bases the header's two. Made in
   `status()` beside the header's ages.
3. The text: `freshness_lines` prints, first, under every policy state
   when something is posted: `facts: 31 items at 3 locations, fetched
   53d to 50d ago (oldest Dump Tab; newest Allflame / Pat (character));
   stash listing 50d ago`, the two-location, one-location and no-fetch
   forms chosen from `facts`'s own fields as round 2's total line
   chooses (one location: `fetched 53d ago, Dump Tab`; none: `no fetch
   recorded`); `render_status` prints the same line shape in the
   decision view after the counts line: `facts: 546 locations fetched
   57d to 53d ago (oldest …; newest …); stash listing 53d ago,
   characters listed 53d ago`, and under `--expand` the `basis:` line
   stays. The never-listed status says `facts: nothing listed`.
4. The glosses: `freshness.facts` and `status.facts` in `describe` and
   the MCP's tool text, each naming its set ("the posted items'
   locations"; "the league's listed containers") and that the fetches
   are the locations' last fetches; no id in either (C116).
5. The pins: in `unit/tests/shop.rs`, one test per shop input above,
   asserting on the `Freshness` value, and a case in `c72_…` that the
   stale set's oldest and `facts`'s oldest differ when an uncovered
   item is older; in `unit/tests/listing.rs`, the status inputs; the
   two reference fixtures regenerated for the added keys; in
   `shop_json.rs` and `price_json.rs`, the text line as a golden fact
   under `ACQ_UPDATE_FIXTURES=1`, every changed line read: a new
   `facts:` line, a `# holds` line, or an added JSON key; the MCP's
   `pricing_tools.rs` asserting `freshness.facts` under `not_set`.
6. The gate; the commit, naming this brief, the changed lines by kind,
   and `retired nothing`. #156 closes.

## Done when

- `Freshness.facts` exists, `Some` iff `posted_total > 0`, under every
  policy state; `ListingReport.facts` exists on `status` alone.
- The shop inputs and the status inputs of Input are each a test; the
  `c72_…` case holds the two oldests apart; the MCP test holds the
  block under `not_set`.
- Every regenerated line is a `facts:` line, a `# holds` line, or an
  added JSON key; the two reference fixtures' diffs are added keys.
- `describe`'s two entries name each block's set, no id in the text.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- The render's listing header aged (`taken_at_age` and the bases' on
  `listing`): round 3; any epoch's words beyond `facts`'s own.
- The stale set's measure, the positions-before-listing rule, the
  policy's remedies: C72's, unchanged.
- `price list` and `price show`: their subjects are a selection and
  one item, whose own bases they print; no block.
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
