# 155-subject-block — the matches' fetch span on the search answer (#155)

Round 2 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§4 "The collapse" and §10 step S1), written by the orchestrating
session on 2026-11-03 after round 1 (`briefs/155-coverage-fold.md`)
landed at `fb6c5c02` (every line number below from it). Its rulings
are C118 as amended and C162, entered at `f3db3427`; the owner's word
on the text is the map's §11 question 4, (a): the scope line's ages
leave the CLI text once the subject's stand under the total ("I agree
with your approach", 2026-11-03). The JSON shape in Do 1 and Do 2 and
the text's words in Do 3 are the session's agent-facing design,
listed at the close for his review. Cut three times: the first draft
(`263ab9db`) and the re-cut (`b8b768d1`) were each reviewed cold
(`roadmap/reviews/155-subject-block-brief-<sha>.md`, the prompts
beside: eight findings, then six, none a nitpick, none structural);
this cut answers the second review's six, listed at the foot under
"Taken from the reviews" with the first's eight. Not reviewed a third
time: the roadmap's rule.

## Goal

When this step is done, every search answer says how old the matches
are: `total` carries the matched items' places counted, their oldest
and newest fetch with locations and ages, and when their lists were
seen, computed by the store's one derivation (C162) over the matches'
places and their listing bases, on the rows, counts and cross views
and on both surfaces; the text's total line says the fetches in the
clause the scope line says today, the scope line keeps its counts and
loses that clause, and the scope's ages stay in the JSON; the
`describe` gloss for `total` names the new fields and their set. #155
closes by its own line: "a search answer carries the matched items'
oldest and newest fetch (and their places) beside the scope's, on the
rows, counts and cross views, on both surfaces, each a golden fact."

Why it matters: a seat quoted the scope's ages for 187 matches and
named it the fault a person would quote wrong (#155; MCP-2 calls 32
and 37); the matcher already walks every match with its place in
hand; the fold is the function round 1 built, applied to a second
set. The decision view then carries one age, the matches', and the
scope's provenance stays where a program keeps it (C53).

## Read

- `roadmap/reviews/answer-facts-map.md` §4 ("The idea": the search's
  subject is the matches' places, with their listing bases among the
  observations; "Where it strains": two blocks with ages in one
  answer, removed items and the empty set, golden churn, the gloss)
  and §10 S1. `roadmap/briefs/155-coverage-fold.md` and its reviews
  (`reviews/155-coverage-fold-brief-*.md`, `reviews/155-coverage-fold-fb6c5c02.md`).
- `decisions/search.md:35`: C118 as amended (the subject, the set
  asked about, has its places' oldest and newest fetch with their
  locations *and when its lists were seen*, named for the set; a wider
  scope's ages apart in the JSON), C100 (one answer shape), C96 (the
  scope). `decisions/frontends.md`: C162 (every answer applies the one
  derivation to its subject; a set's listing bases are among its
  observations), C53 (the decision view; ages in words beside epochs,
  made at the answer's time; every nothing says which nothing), C146
  (a count says what it counts), C115 (the total before the terms
  block). `AGENTS.md`: C116 (no id in user-facing text), C122 (a golden
  regenerated under `ACQ_UPDATE_FIXTURES=1` names the facts a reader
  must find), C154.
- `crates/acquisition-store/src/coverage.rs`: `FetchedLocation`
  (24–44); `Coverage` (46–106), its eight contiguous fields from
  `oldest_fetch` to `list_seen_age` (78–105: `oldest_fetch`,
  `newest_fetch` and `list_seen` carry no `skip_serializing_if`, the
  five words-and-location fields do); `Coverage::aged` (108–118);
  `of` (134–219): the oldest and newest fold (152–161, the tie rule:
  the first in order both ways), the fields it fills (205–217, the
  `list_seen` minimum over the in-realm listings at 211–217);
  `by_type` (221–240). `unit/tests/coverage.rs`: the tie test, the
  fixture builders, and the test that constructs `Coverage` with its
  fields directly (68).
- `crates/acquisition-store/src/lib.rs:1842` (a listing's retirement
  keeps an ordinary tab's fetch) and `:1869` (a retired substash's
  freshness is cleared); `crates/acquisition-store/src/corpus.rs:383`
  (the `all` read returns the retained fetch).
- `crates/acquisition-search/src/corpus.rs`: `Place` (291–325:
  `kind`, `id`, `name`, `league`, `fetched_at` at 323: the location's
  own fetch, a socketed item's its holder's; a removed item's place is
  its location as it stands, 490); `Coordinate` and `coordinate`
  (413–417: realm, kind, id, and the league for a stash alone, since a
  tab id recurs across leagues and a character's does not); `Corpus`
  (370–384: no location rows kept, so the fold reads each match's
  `Place`); `Corpus.listings` (the in-realm listing bases as
  `(league, fetched_at)`, a character listing's league `None`); the
  call site (764–790). `answer.rs`: `Answer` (1144–1162); `ScopeOut`
  (1275–1284, `coverage` flattened); `Total` (1375–1385); the matcher
  loop (1751–1782, each match pushed at 1779 with its index into
  `corpus.items`); `Total` built before sorting, paging and the view
  (1885), so neither a view nor `--limit` can change it; the
  `list_seen` narrowing (2087–2093), untouched.
- `crates/acquisition-cli/src/search_cmd.rs`: `answer_text_of`
  (934–946; the counts and cross views are `ViewOut` variants under
  the same head, 1258–1278); `answer_head` (950–1075): the scope
  line's clause (994–1022, reading the six fields by name, with the
  comment on why each age names its location), the never-fetched line
  (1024–1030), the total line (1058–1062), the line under every total
  (1064–1074). `web/page.rs:288` prints the head word for word.
- `crates/acquisition-search/src/describe.rs`: the `scope` entry
  (230–232), the `total` entry (246–248), and `one_line` (909: an
  entry's index synopsis is its text up to the first `: `, so the
  `total` gloss keeps its `the matches: ` head or the index golden
  changes too).
- `crates/acquisition-cli/tests/harness/mod.rs`: `fetch_place_rule`
  (656–683: on a `scope ` line it finds `fetch <age> ago, ` and
  `newest <age> ago, `, neither of which matches the one-place form
  `fetched <age> ago, `), `run_fed` (562–566: a `--json` command is
  never recorded in a golden, so a JSON fact is pinned by field in a
  test, never in a transcript), and `# holds` (601).
- Every test and golden that asserts on the scope line's clause, each
  found by grep ("the oldest fetch", "newest", "fetch <age>") and
  each changed by this step: the goldens under `search_json` (8),
  `seat_faults` (6), `web` (2), `journey` (1), `new_league` (1), among
  them `search_json/membership_all.txt:31` (its `# holds` lines name
  the clause) and `search_json/describe_at_a_terminal.txt:220` (the
  index synopsis of `total`); `tests/search_json.rs:2140`
  (`step_10_membership_all_at_a_terminal`, its zero-match command
  requires the clause on the scope line) and `:646`
  (`c53_search_json_is_the_answer_whole_and_the_text_is_a_function_of_it`,
  a contiguous literal of the total line); `tests/seat_faults.rs:1493`
  (`a_rows_place_says_how_long_ago_its_location_was_fetched`, the
  clause itself) and `:1375`; `tests/normaliser.rs` (the rule's
  cases); `crates/acquisition-mcp/tests/surface_size.rs:89` (samples
  the `scope` gloss) and `search_tools.rs:679` (the describe's size
  bounds, measured after the gloss edit, no bound raised without a
  measured reason). The worker greps again on its tree and names any
  the list misses.
- The tests that pin the answer's shape: `crates/acquisition-search/tests/answer.rs:35–80`
  (the worked example's scope); `membership.rs:447` (a tab id shared
  across leagues, deliberately) and `:1030–1045`;
  `crates/acquisition-mcp/tests/search_tools.rs` (the MCP's search
  answer is the CLI's `--json` document: `acquisition-mcp/src/main.rs:504–506`).
- `REVIEW-CHECKLIST.md`, "Search" and "Frontends and the surface an
  agent meets".

## Input

- The rulings and the owner's word, above. No new ruling: C118's
  "when its lists were seen" is built here, not narrowed.
- The files listed in Read, by grep at `fb6c5c02`.
- The failing inputs the step's tests stage, in the search crate's
  fixtures (`tests/answer.rs`'s world builder, `membership.rs`'s) and
  one MCP test:
  - matches in two locations fetched at different times, the scope
    holding a third location older than both: `total`'s oldest and
    newest are the two matched locations' with their ages and
    locations, `scope`'s oldest the third's (the fact the seat
    wanted); the same `total` on the rows view with `--limit 1`, the
    counts view and the cross view;
  - every match in one location: `places` 1, oldest and newest the
    same epoch and location;
  - no match: `places` 0, no span in the JSON;
  - a location holding three matches: counted once;
  - a socketed match whose holder sits in a location: the holder's
    place, once with the holder;
  - two matched tabs with one id in two leagues (the fixture of
    `membership.rs:447`): two places under `live`, and under `all` and
    `removed` a character matched in two of its historical leagues:
    one place;
  - under `all`, a removed match in a retired tab: its place's fetch
    retained (`lib.rs:1842`), so inside the span as its location's
    last fetch; and a removed match in a retired substash: its fetch
    cleared (`:1869`), so in `places` and outside the fetch extremes;
    a set of such matches alone, their tab listing known: `places` 2,
    the span present with null fetch epochs, no fetch words or
    locations, `list_seen` the listing's, and no clause in the text;
  - a search over every realm (`--realm all`) with matches in one
    realm and an older tab listing of the same league in another
    realm: `total.list_seen` the matched realm's listing, `scope`'s
    the older one's;
  - the matches' lists: a query fixing one league whose matches are
    stash items of that league and one character item: `total`'s
    `list_seen` the older of that league's tab listing and the
    realm's character listing, where `scope`'s is its own narrowing;
    matches in one league alone: that league's tab listing;
  - the scope's serialized shape: the eight span keys in their order
    between `folders` and the end, pinned once in the store.
- The last push's CI, on `main` at `7ab91cc4`: Gate (Linux)
  `41736216175`, success; Gate (Windows) `41736216174`, success.
  Seventeen commits since are unpushed, round 1's Rust among them;
  the push is the orchestrator's, and a red run after it is the
  group's first concern, not this step's.

## Do

1. In the store's `coverage.rs`: the part of a coverage result that
   any set of places with their observations has becomes its own
   value, `FetchSpan`: the eight fields of 78–105 (`oldest_fetch`,
   `oldest_fetch_age`, `oldest_fetch_location`, `newest_fetch`,
   `newest_fetch_age`, `newest_fetch_location`, `list_seen`,
   `list_seen_age`), each with the serde attribute and doc comment it
   has today, and `FetchSpan::aged(now)` (the three lines of 113–117).
   `Coverage` embeds it as `#[serde(flatten)] pub span: FetchSpan` in
   the position the eight fields hold, so the scope's JSON does not
   change by a byte; `Coverage::aged` calls it. `pub fn span(...)` is
   the fold of 152–161, 205–210 and 211–217 on its own: it takes the
   fetched places as `(i64, FetchedLocation)` pairs in the caller's
   order and the listing bases that name the set as `&[ListingSeen]`
   (the caller chooses them: the set's observations are the caller's
   to name, C162), keeps the tie rule (the first in that order both
   ways; a unit test over two orders as round 1's) and takes
   `list_seen` as the minimum over the bases given; `of` builds the
   pairs from its fetched locations, passes the in-realm bases it
   filters today, and calls it. Every constructor and access of the
   eight fields moves to `.span.`, mechanically, logic and output
   unchanged: the store's unit tests throughout
   `unit/tests/coverage.rs` (68 and after), the CLI's scope clause
   (`search_cmd.rs:994–1022`) and its never-fetched line (`:1029`),
   and the answer's narrowing (`answer.rs:2092`, which becomes
   `coverage.span.list_seen = Some(seen)`, its logic untouched).
   `Corpus.listings` (388, filled at 792) keeps the `ListingSeen`
   values whole in place of `(league, fetched_at)` pairs, and
   `list_seen_in` reads the same two fields from them and returns
   what it returns today. Unit tests: the tie rule over two orders,
   `of`'s span equal to `span` over the same places and bases, and
   the serialized text of `Coverage` as fixed expectations, for a
   populated coverage and for an empty one, each before and after
   `aged` (the key order, the three nulls, the five omitted keys,
   `list_seen_age` `never` on the empty aged one), compared as text
   and never as a sorted value. Checkpoint before item 2: `cargo
   nextest run --workspace` green and no golden changed (a check, not
   the gate; the one gate runs before the built commit).
2. In the search's `Total`: `pub places: usize` and `#[serde(flatten,
   skip_serializing_if = "Option::is_none")] pub span:
   Option<FetchSpan>`, after `matched` and before `undecided`. After
   the matcher loop and before the view (1885): the matches' places,
   for each match in order its `Place` keyed by `coordinate(realm,
   kind, id, league)` as the corpus keys a location (413–417: the
   league for a stash, none for a character), the first occurrence
   kept; `places` is their count. The span is `Some` whenever there is
   a match: `coverage::span` over the places that have a `fetched_at`,
   as `(fetched_at, FetchedLocation { kind, id, name, league })`, with
   the bases from `corpus.listings` (the `ListingSeen` values, Do 1)
   that name a place in the set, chosen by realm, kind and league: a
   stash place's realm-and-league tab listing, and the realm's
   character listing where a character place of that realm is in the
   set; aged where the answer's other ages are made. With no fetched
   place the fetch epochs are null and their words and locations
   absent, and `list_seen` is still the bases' minimum: the lists
   were seen whether or not a fetch landed, and C118 asks for them.
   With no match: `None`, absent from the JSON, `places` 0.
3. The text (`answer_head`): the clause moves from the scope line to
   the total line, its form chosen from `total`'s own fields, so the
   text stays a function of the JSON (C53). Where the oldest and the
   newest fetch name two locations: `total   187 matches (the oldest
   fetch 57d ago, Standard / Dump Tab; the newest 53d ago, Allflame /
   TheAbsenceOfPatience (character))`, the clause's words those of
   1017–1022 verbatim and the location's form unchanged; where they
   name one location (`oldest_fetch_location` equal to
   `newest_fetch_location`, which one fetched place gives, and which
   two places fetched at one time also give, since both extremes name
   the first): `total   1 match (fetched 57d ago, Standard / Dump
   Tab)`; where the span has no fetch, or there is no match: no
   clause. The undecided count follows the clause: `total   187
   matches (the oldest fetch …) · 3 undecided`. The scope line keeps
   `account · realm · membership · items · locations fetched` and
   loses the clause and its comment (the comment's reasons move with
   the clause). The never-fetched line and the line under every total
   stay. The harness's `fetch_place_rule` masks both forms on a
   `total ` line (`fetched <age> ago, ` as well as `fetch <age> ago, `
   and `newest <age> ago, `), and `tests/normaliser.rs` gains cases:
   a two-place total with a character as the newest and a trailing
   undecided count, and a one-place total.
4. The gloss: `describe`'s `total` entry keeps its head `the matches:
   ` (the index synopsis, 909) and its `matched` and `undecided`
   sentence, and adds the new fields by name: `places` the locations
   the matches sit in, each once; `oldest_fetch` and `newest_fetch`
   with their `_age` and `_location` the earliest and latest of those
   locations' last fetches, the locations' fetches and not a removed
   item's last sighting; `list_seen` the oldest of the lists that
   name those locations; where no matched location was fetched,
   `oldest_fetch` and `newest_fetch` null, their `_age` and
   `_location` absent and `list_seen` with its `_age` kept, and the
   block absent where nothing matched, `places` 0. *Settled in the
   round:* the brief first said "the fetch fields null", wider than
   Do 2's null-versus-absent shape (the build review at `21a8a28c`,
   its second finding). The
   `scope` entry's sentence on its ages says they are
   over every location in scope, the matches' under `total`. No id in
   either (C116); `surface_size.rs:89`'s sample and
   `search_tools.rs:679`'s bounds checked after the edit.
5. The assertions named in Read are moved before regeneration, each
   to the fact that replaces it: `search_json.rs:2140`'s zero-match
   command (`membership_all.txt:29–31`, `id:r2`, total 0) holds
   `total   0 matches` and the clause's absence from its scope line,
   and a matching command of the same test holds the clause on its
   total line; `search_json.rs:646`'s literal is the total line in
   its new form; `seat_faults.rs:1480–1493`'s `league=Standard` search
   has one matched place, so its assertion becomes the one-location
   form, the Allflame newest it names today staying in the scope's
   JSON alone; `tests/normaliser.rs`'s existing case is updated beside
   the two new ones. Then `ACQ_UPDATE_FIXTURES=1` once
   over the workspace, and every changed line read, each one of: the
   clause leaving a `scope ` line; the clause joining a `total ` line;
   a `# holds` line moved with it; the `total` gloss and its index
   synopsis; a hunk may hold several. A changed line of any other kind
   is a fault to fix, never to accept. One new golden under
   `seat_faults`, named for #155's fault, whose `# holds` lines name
   the total's clause and the scope line's counts without it.
6. The pins, by field and never by transcript (`run_fed`, 562–566):
   in `tests/answer.rs`, the complete expected span on the first
   fixture (both epochs, both locations, both ages at a fixed `now`,
   `list_seen`), equal across the rows view with `--limit 1`, the
   counts view and the cross view, and `scope`'s oldest apart from it;
   each other failing input of Input as its own test; in the MCP's
   `search_tools.rs`, one check that the complete `total` block
   survives the tool's serialization (every span key present on the
   first fixture, none on the empty one).
7. The gate; the commit, whose message names this brief, counts the
   regenerated files by directory and the changed lines by kind, and
   carries the line: `the scope line's ages clause leaves the CLI
   text: a contract change (C122), held by the regenerated goldens,
   the scope's ages kept in every search JSON document`. Nothing is
   retired in C154's form.

## Done when

- `FetchSpan` and `span` exist in `crates/acquisition-store/src/coverage.rs`,
  `Coverage` embeds the span flattened at its position, and the unit
  tests hold the tie rule over two orders, `of`'s span against
  `span`, and `Coverage`'s serialized text populated and empty,
  before and after aging.
- `Total` carries `places` and the optional flattened span, present
  whenever there is a match; `grep -n 'places'
  crates/acquisition-search/src/answer.rs` finds the field and the
  fold, the fold keys by `coordinate`, and `Corpus.listings` holds
  `ListingSeen` values.
- `tests/answer.rs` holds the complete span on the first fixture
  across the three views and a limited request, `scope`'s oldest
  apart from it, and one test per other failing input of Input,
  the across-league, retired, cleared-fetch and all-realms cases
  included; the MCP check holds the complete block.
- Every regenerated line is one of Do 5's kinds, read by the reviewer
  over the diff; the new `seat_faults` golden holds the total's clause
  and the scope line without it; no `scope ` line in any golden holds
  "the oldest fetch".
- `describe`'s two entries say the set of each block's ages, the
  `total` entry's head unchanged, no ruling id in the text; the
  describe index golden's `total` line unchanged.
- The commit message carries the contract-change line of Do 7.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- The listing bases widened by each base with its own age (C162's
  "each listing base with when it was seen"): round 4 (S3), its first
  consumer. This step's `list_seen` is the minimum, as the scope's.
- Per-bucket ages on the counts view: a bucket's route gives its rows;
  the counts and cross views share the matches' subject.
- Any change to `scope`'s JSON, to the never-fetched line, to the line
  under every total, or to any word outside the clause, the moved
  assertions and the two gloss entries.
- The shop, price status, tabs and store status (S3, S5), the
  named-paths pin (S4).

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names.
- `ACQ_UPDATE_FIXTURES=1` runs once, after Do 5's assertions are
  moved, and every regenerated line is read before the commit; a line
  outside Do 5's kinds is a fault to fix, never to accept.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the count of regenerated files by directory,
the count of changed lines by kind, the describe's measured size
before and after, and the test count before and after.

## Taken from the reviews

The second cold review, of `b8b768d1`
(`roadmap/reviews/155-subject-block-brief-b8b768d1.md`): six
findings, no nitpick, each answered in this cut.

1. The span conditioned on a fetch discarded the subject's known
   listing time: the span is present whenever there is a match, its
   fetch epochs null without a fetch, `list_seen` kept; the clause
   alone is conditional on a fetch; a cleared-fetch fixture with a
   known listing is staged.
2. `Corpus.listings` had dropped the realm: it keeps the `ListingSeen`
   values, the bases are chosen by realm, kind and league, and an
   all-realms case is staged.
3. The text's forms are chosen from `total`'s own fields: two
   locations or one, by the extremes' locations, which keeps the text
   a function of the JSON.
4. Every access of the eight fields migrates in Do 1, the narrowing
   and the never-fetched line named, their logic unchanged.
5. The zero-match golden holds `total   0 matches` and the clause's
   absence, a matching command holds the clause; the one-place
   `league=Standard` assertion takes the one-location form; the
   existing normaliser case is updated.
6. The serialization pin is the serialized text, populated and empty,
   before and after aging, never a key order alone.

The first cold review, of `263ab9db`
(`roadmap/reviews/155-subject-block-brief-263ab9db.md`): eight
findings, no nitpick, each answered in the second cut.

1. C118 as entered requires the subject's "when its lists were seen";
   the first cut excluded it. The span is the eight contiguous fields
   including `list_seen`, built here; the ruling is not narrowed.
2. The fold keys by the corpus's own `coordinate` (the league for a
   stash alone), and the across-league fixture is staged under every
   membership.
3. `oldest_fetch`, `newest_fetch` and `list_seen` serialize `null`
   when absent, so "every span field absent" could not hold with the
   attributes preserved: `Coverage` keeps them, and `Total`'s span is
   an `Option`, absent whole when no matched location was fetched.
4. Do 1 moves the direct readers of the eight fields (the store's unit
   test, the CLI's clause) in the same item, behaviour unchanged, and
   its checkpoint is a check, the gate reserved for the built commit.
5. A retired tab keeps its fetch and a retired substash loses it: both
   staged, the "places without a span" case given its text (no
   clause) and its gloss (the locations' fetches, not an item's last
   sighting).
6. The assertions that would fail before regeneration are named with
   the fact that replaces each; the harness rule's two literal forms
   and the normaliser's new cases are named.
7. "Every hunk one of three kinds" became "every changed line one of
   five kinds", holds lines and the gloss among them; the `total`
   gloss keeps its head so the index synopsis stands.
8. JSON facts are pinned by field in tests, never in transcripts: the
   scope's key order once in the store, the complete span at the
   search boundary across views and a limited request, the whole
   block through the MCP; the CLI golden holds the clause and the
   scope's counts.
