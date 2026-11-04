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
listed at the close for his review. To be reviewed cold before a
worker reads it.

## Goal

When this step is done, every search answer says how old the matches
are: `total` carries the matched items' places counted and their
oldest and newest fetch with locations and ages, computed by the
store's one derivation (C162) over the matches' places, on the rows,
counts and cross views and on both surfaces; the text's total line
says them in the clause the scope line says today, the scope line
keeps its counts and loses that clause, and the scope's ages stay in
the JSON; the `describe` gloss for `total` names the new fields and
their set. #155 closes by its own line: "a search answer carries the
matched items' oldest and newest fetch (and their places) beside the
scope's, on the rows, counts and cross views, on both surfaces, each a
golden fact."

Why it matters: a seat quoted the scope's ages for 187 matches and
named it the fault a person would quote wrong (#155; MCP-2 calls 32
and 37); the matcher already walks every match with its place in
hand; the fold is the function round 1 built, applied to a second
set. The decision view then carries one age, the matches', and the
scope's provenance stays where a program keeps it (C53).

## Read

- `roadmap/reviews/answer-facts-map.md` §4 ("The idea": the search's
  subject is the matches' places; "Where it strains": two blocks with
  ages in one answer, removed items and the empty set, golden churn,
  the gloss) and §10 S1. `roadmap/briefs/155-coverage-fold.md` and its
  reviews (`reviews/155-coverage-fold-brief-*.md`): what round 1 built
  and why the function's inputs are the store's types.
- `decisions/search.md`: C118 as amended (the subject carries its
  places' oldest and newest fetch with their locations, named for the
  set; a wider scope's ages apart in the JSON), C100 (one answer
  shape), C96 (the scope). `decisions/frontends.md`: C162 (every
  answer applies the one derivation to its subject), C53 (the decision
  view; ages in words beside epochs, made at the answer's time), C146
  (a count says what it counts: the clause names its set), C115 (the
  total before the terms block). `AGENTS.md`: C122 (a golden
  regenerated under `ACQ_UPDATE_FIXTURES=1` names the facts a reader
  must find), C154 (a retirement line).
- `crates/acquisition-store/src/coverage.rs`: `FetchedLocation`
  (24–44); `Coverage` (46–106), its six fetch fields (78–95) and
  `list_seen` (101–105); `Coverage::aged` (108–118); `of` (134–219):
  the oldest and newest fold (152–161, the tie rule: the first in
  order both ways) and the fields it fills (205–210); `by_type`
  (221–240). `unit/tests/coverage.rs`: the tie test and the fixture
  builders, the model for the new tests.
- `crates/acquisition-search/src/corpus.rs`: `Place` (291–325:
  `kind`, `id`, `name`, `league`, `fetched_at` at 323, the location's
  own fetch, a socketed item's its holder's); `Corpus` (370–384: no
  location rows are kept, so the fold reads each match's `Place`);
  the call site (764–790). `answer.rs`: `Answer` (1144–1162, `scope`
  then `total`); `ScopeOut` (1275–1284, `coverage` flattened);
  `Total` (1375–1385: `matched`, `undecided`, `undecided_reasons`,
  `reasons_left_out`); the matcher loop (1751–1782, each match
  pushed at 1779 with its index into `corpus.items`); the `list_seen`
  narrowing (2087–2093), untouched.
- `crates/acquisition-cli/src/search_cmd.rs`: `answer_text_of`
  (934–946: the head, the rows, more, the tail; the counts and cross
  views are `ViewOut` variants under the same head, 1258–1278);
  `answer_head` (950–1075): the scope line's clause (994–1022, with
  the comment that says why each age names its location), the
  never-fetched line (1024–1030), the total line (1058–1062), the
  line under every total (1064–1074). `web/page.rs:288` prints the
  head word for word.
- `crates/acquisition-search/src/describe.rs`: the `scope` entry
  (230–232) and the `total` entry (246–248).
- `crates/acquisition-cli/tests/harness/mod.rs`: `fetch_place_rule`
  (656–683, masks the places on a `scope ` line of a live transcript)
  and the `age` rule (324); `normaliser.rs` (its cases hold the scope
  line's literal).
- The twenty files that hold "the oldest fetch" at `fb6c5c02`
  (`grep -rl 'the oldest fetch' crates/*/tests`): goldens under
  `search_json` (8), `seat_faults` (6), `web` (2), `journey` (1),
  `new_league` (1), and `tests/seat_faults.rs`, `tests/normaliser.rs`.
  Every one changes once in this step.
- The tests that pin the answer's shape: `crates/acquisition-search/tests/answer.rs:35–80`
  (the worked example's scope); `seat_faults.rs:1375` (the scope by
  tab type); `membership.rs:1030–1045` (coverage under `all`);
  `crates/acquisition-mcp/tests/search_tools.rs` (names scope fields;
  the MCP's search answer is the CLI's `--json` document, so a field
  on one is on the other: `acquisition-mcp/src/main.rs:504–506`).
- `REVIEW-CHECKLIST.md`, "Search" and "Frontends and the surface an
  agent meets".

## Input

- The ruling and the owner's word, above. No new ruling.
- The twenty files, by grep at `fb6c5c02`, listed in Read.
- The failing inputs the step's tests stage, in the search crate's
  fixtures (`tests/answer.rs`'s world builder) and one MCP test:
  - matches in two locations fetched at different times, the scope
    holding a third location older than both: `total.oldest_fetch`
    and `newest_fetch` are the two matched locations' and
    `scope.oldest_fetch` the third's (the fact the seat wanted);
  - every match in one location: `places` 1, oldest and newest the
    same epoch and location;
  - no match: `places` 0, no span field in the JSON;
  - a location holding three matches: counted once in `places`;
  - a socketed match whose holder sits in a location: the holder's
    place, once with the holder;
  - the counts view and the cross view of the first fixture: the same
    `total` object and the same total line;
  - under `all`, a removed match whose location was retired: its
    `place.fetched_at` is none (`corpus.rs:579` at `6544366c`, now
    `Place`'s own), so it is in `places` and outside the span, as a
    never-fetched location is outside the scope's; the gloss says so;
  - the scope's JSON before and after this step identical on every
    regenerated golden (the reviewer reads every hunk).
- The last push's CI, on `main` at `7ab91cc4`: Gate (Linux)
  `41736216175`, success; Gate (Windows) `41736216174`, success.
  Fourteen commits since are unpushed, round 1's Rust among them; the
  push is the orchestrator's, and a red run after it is the group's
  first concern, not this step's.

## Do

1. In the store's `coverage.rs`: the part of a coverage result that
   any set of fetched places has becomes its own value, `FetchSpan`:
   `oldest_fetch`, `oldest_fetch_age`, `oldest_fetch_location`,
   `newest_fetch`, `newest_fetch_age`, `newest_fetch_location`, each
   field with the serde attribute and doc comment it has today
   (78–95), and `FetchSpan::aged(now)` (the two lines of 113–117).
   `Coverage` embeds it as `#[serde(flatten)] pub span: FetchSpan` in
   the position the six fields hold, so the scope's JSON does not
   change by a byte; `Coverage::aged` calls it. `pub fn span(...)`
   is the fold of 152–161 and 205–210 on its own: it takes the
   fetched places as `(i64, FetchedLocation)` pairs in the caller's
   order and keeps the tie rule (the first in that order, both ways;
   a unit test over two orders as round 1's); `of` builds the pairs
   from its fetched locations and calls it. A unit test holds that
   `of`'s span equals `span` over the same places. Every existing
   test passes unchanged, and the goldens are untouched through this
   item: gate it (`cargo nextest run --workspace`) before item 2.
2. In the search's `Total`: `pub places: usize` and
   `#[serde(flatten)] pub span: FetchSpan`, after `matched` and before
   `undecided`. After the loop, the matches' places: for each match in
   order, its `Place` keyed by (`realm`, `kind`, `id`), the first
   occurrence kept; `places` is their count; `span` is
   `coverage::span` over those with a `fetched_at`, each as
   `(fetched_at, FetchedLocation { kind, id, name, league })`; aged
   where the answer's other ages are made. No match: `places` 0 and
   every span field absent.
3. The text (`answer_head`): the clause moves from the scope line to
   the total line. With two or more places: `total   187 matches (the
   oldest fetch 57d ago, Standard / Dump Tab; the newest 53d ago,
   Allflame / TheAbsenceOfPatience (character))`, the clause's words
   those of 1017–1022 verbatim, the location's form unchanged, then
   ` · 3 undecided` where there are undecided; with one place: `total
   1 match (fetched 57d ago, Standard / Dump Tab)`; with none, no
   clause. The scope line keeps `account · realm · membership · items
   · locations fetched` and loses the clause and its comment (994–
   1022; the comment's reasons move to the total's clause). The
   never-fetched line and the line under every total stay. The
   harness's `fetch_place_rule` masks the clause on a `total ` line as
   it masks it on a `scope ` line, both forms.
4. The gloss: `describe`'s `total` entry says what the new fields are
   of ("`places` the locations the matches sit in, each once, and the
   oldest and newest fetch among those fetched with their locations:
   the matches' own ages, where `scope`'s are every location's"); the
   `scope` entry's sentence on its ages says they are the scope's,
   the matches' under `total`. No id in either (C116).
5. The goldens: `ACQ_UPDATE_FIXTURES=1` once over the workspace; every
   diff read line by line, each hunk one of: the clause leaving a
   `scope ` line, the clause joining a `total ` line, `places` and the
   span joining a `total` object; any other hunk is a fault. One new
   golden under `seat_faults`, named for #155's fault, whose `# holds`
   lines name the total's clause, the scope line without it, and
   `total.oldest_fetch` apart from `scope.oldest_fetch` on the first
   fixture. One MCP test (`search_tools.rs`) that the tool's answer
   carries `total.places` and `total.oldest_fetch`. The pins of
   Input's failing inputs in `tests/answer.rs` (the counts and cross
   views included).
6. The gate; the commit, whose message names this brief, lists the
   hunks' three kinds with the file count, and carries the line:
   `the scope line's ages clause leaves the CLI text: a contract
   change (C122), held by the regenerated goldens, the scope's ages
   kept in every search JSON golden`. Nothing is retired in C154's
   form.

## Done when

- `FetchSpan` and `span` exist in `crates/acquisition-store/src/coverage.rs`,
  `Coverage` embeds the span flattened, and the unit tests hold the
  tie rule over two orders and `of`'s span against `span`.
- `Total` carries `places` and the flattened span; `grep -n 'places'
  crates/acquisition-search/src/answer.rs` finds the field and the
  fold.
- On the first fixture, `total.oldest_fetch` differs from
  `scope.oldest_fetch` in a test the reviewer can read; `places` is 1
  on the one-location fixture, 0 with no match and no span field
  serialized; a location with three matches counts once; the counts
  and cross views carry the same `total`; the MCP test passes.
- The scope's JSON is unchanged on every regenerated golden: the
  reviewer reads each hunk and finds the three kinds alone.
- The total line's clause is a golden fact under `seat_faults` with
  `# holds` lines, and no `scope ` line in any golden holds "the
  oldest fetch".
- `describe`'s two entries say the set of each block's ages, with no
  ruling id in the text.
- The commit message carries the contract-change line of Do 6.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- The listing bases widened by each base with its age (C162's "each
  listing base with when it was seen"): round 4 (S3), its first
  consumer.
- `list_seen` under `total`: the matches have no list of their own;
  the scope's stays where it is, narrowed as today.
- Per-bucket ages on the counts view: a bucket's route gives its rows.
- Any change to `scope`'s JSON, to the never-fetched line, to the line
  under every total, or to any word outside the clause and the two
  gloss entries.
- The shop, price status, tabs and store status (S3, S5), the
  named-paths pin (S4).

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names.
- `ACQ_UPDATE_FIXTURES=1` runs once, after Do 4, and every regenerated
  file is read before the commit; a hunk outside Do 5's three kinds is
  a fault to fix, never to accept.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the count of regenerated files by directory,
the count of hunks by kind, and the test count before and after.
