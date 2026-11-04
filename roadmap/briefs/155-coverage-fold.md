# 155-coverage-fold — the one coverage derivation in the store, with no answer changing

Round 1 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`,
§4 "The collapse" and §10 step S2), written by the orchestrating
session on 2026-11-03 from the map and a re-read of the tree at
`31d8a9ff` (every line number below from it). Its two rulings (Do 1)
are the owner's and are entered in the registry ahead of the code
(C118 as amended, C162), under his words of 2026-11-03: "I agree with
your approach, and I encourage you to orchestrate with autonomy." To
be reviewed cold before a worker reads it.

## Goal

When this step is done, the coverage of a set of places is computed
in one place, `acquisition_store::coverage`, a pure function beside
the age vocabulary (`age.rs`) and the read economy's bound
(`bound.rs`), and the search crate's `corpus.rs` and `tabs.rs` call it
where each had its own; and nothing any user or program meets has
changed: every golden under every crate's `tests/goldens/` is byte for
byte what it was, both references are unchanged, and every JSON
answer is the same document in the same field order. The step moves a
derivation one crate down and proves the move by the absence of any
other change.

Why it matters: the map found that every surface derives its ages and
coverage from whatever struct it holds, and that the search's
`coverage()` is already the one function the collapse needs, run once
over the whole realm. Every round after this one applies that function
to a new subject (the matches, #155; the posted items, #156; a league's
tabs; the store). Those rounds change words and goldens and are judged
by reading. This one changes none, so it is judged by a diff that is
empty where it must be, which is the strongest check the group gets:
if the move is not exact, a golden says so before any subject is built.

## Read

- `roadmap/reviews/answer-facts-map.md` §4 ("The idea": the function's
  inputs are places with their observations and counts; "Where it
  lives": the store, and the precedent of `age.rs` and `bound.rs`;
  "What it retires, in C154's form": the first bullet is this step's)
  and §11 question 2 (the owner's reading of C108 and C39 for the
  function's home).
- `decisions/frontends.md`: C162 (as entered: coverage and age are one
  derivation over a set of places, held once in the store; this step
  builds the function and moves the one caller onto it, and leaves
  "every answer applies it to its subject" to the rounds after), C53
  (three output levels; ages in words beside epochs), C146 (a count
  says what it counts: no word changes here). `decisions/search.md`:
  C118 (as amended: the subject's ages are Unbuilt until the next
  round; this step changes no sentence of the scope line), C96 (the
  realm is the scope, never a term), C100 (one answer shape), C103
  (one deriver per fact: place and every ingest fact come from the
  store's read, which is why the function may sit there), C108 (the
  corpus read: what it forbids is deriving what a body says, which this
  function never does). `decisions/store.md`: C39 (the store exposes
  neutral snapshots, never half a planner: no policy enters this
  function). `AGENTS.md`: C1 (the store links no workspace crate; the
  search links the store: the function's types flow up, never down),
  C122 (a golden regenerated under `ACQ_UPDATE_FIXTURES=1` names the
  facts a reader must find: this step regenerates none), C154 (a
  retirement is a commit line, `retired X; held by Y`).
- `crates/acquisition-search/src/corpus.rs`: the module doc's bullet
  "Coverage is stated from the header" (74–83); `FetchedLocation`
  (354–373) and `Coverage` (376–444) with every doc comment and serde
  attribute, which move verbatim; `Coverage::aged` (446–455); the one
  call site inside `Corpus::read` (853–866), whose two overrides
  (`removed`, `leagues: sat_in`) are the search's membership semantics
  and stay; `coordinate` (506) and `items_at`, the per-location item
  counts the search builds from the items it read; `is_folder` (1038);
  `coverage()` itself (1063–1136), which this step moves.
- `crates/acquisition-search/src/tabs.rs`: the module doc's sentence on
  the counter by type (20–21); `TabType` (126–134, `#[serde(rename =
  "type")]`); the call in `Tabs::of` (154–157); `by_type` (238–260).
- `crates/acquisition-search/src/answer.rs:2087–2093`: the `list_seen`
  narrowing to the fixed league, which is the answer's and stays.
- `crates/acquisition-store/src/corpus.rs`: `RealmScope` (131–137),
  `CorpusHeader` (160–183: `realms`, `listings`, `locations`),
  `ListingSeen` (185–197), `LocationRow` (199–220); the module doc's
  bullet on the header (33–40: an empty listing has no location row to
  carry its time, a substash's listing time is its parent's fetch).
- `crates/acquisition-store/src/bound.rs` whole (34 lines): the model
  for a shared helper in the store, its module doc's first two
  paragraphs (why it lives there) and its `#[path]` unit test;
  `crates/acquisition-store/src/age.rs:1–20` and `:66`.
- The tests that read the result and must pass unchanged:
  `crates/acquisition-search/tests/answer.rs:35–80` (the worked
  example's `oldest_fetch`, `newest_fetch`, `list_seen`, the locations
  named, #11); `crates/acquisition-search/tests/tabs.rs:117–135`
  (`by_type` counts); `crates/acquisition-search/tests/membership.rs:
  105–116` and `:1030–1045` (coverage under `live` and `all`);
  `crates/acquisition-search/tests/seat_faults.rs` (grep
  `never_fetched_by_type`); `crates/acquisition-mcp/tests/search_tools.rs`
  and `surface_size.rs` (they name scope fields); the 18 goldens that
  hold "the oldest fetch" (`grep -rl 'the oldest fetch' crates/*/tests
  | grep goldens`, 18 files at `31d8a9ff`).
- The two consumers outside the search crate that name the moved types
  by the search's path, which the re-export keeps:
  `crates/acquisition-cli/src/search_cmd.rs:740`
  (`acquisition_search::tabs::TabType`) and `:1002`
  (`acquisition_search::corpus::FetchedLocation`). No other crate names
  `Coverage`, `FetchedLocation`, `TabType` or `by_type` (the grep over
  `crates/*/src` and `crates/*/tests` outside the search crate).
- `REVIEW-CHECKLIST.md`, "Store and plans" and "Search".
- `tools/arch-check.sh:372–412`: the edges; the store links nothing
  (381), the search links the store (379). This step adds no edge.

## Input

- The rulings: C118 as amended and C162, entered at the commit this
  brief names in its head; `python3 -I tools/bothy.py check` passes on
  that tree and C162 is 800 bytes.
- The callers, by grep at `31d8a9ff`: `coverage(` has one call
  (`corpus.rs:859`); `by_type(` has two (`corpus.rs:1072`,
  `tabs.rs:154`).
- The goldens: 18 files under `tests/goldens/` hold "the oldest fetch"
  (the grep above); every golden under `crates/*/tests/goldens/` is in
  this step's unchanged set, not these 18 alone.
- The failing inputs the store's unit tests stage, each a fixture of
  `LocationRow`s and `ListingSeen`s built by hand, no SQLite: an empty
  set (every count 0, no oldest or newest, `list_seen` none); a set of
  one never-fetched tab (fetched 0, never fetched 1, its type listed
  with `tabs` 1 and `fetched` 0); a folder among tabs (counted under
  `folders`, in no type, never a fetch); two tabs fetched at one time
  (the oldest is the first in the slice's order and the newest the
  last, both ways, as `coverage()` reads them today at 1073–1076); a
  character (type `character`, its league from the row); a substash
  whose parent is a tab (a location of its own, its own fetch); two
  realms' listings under `RealmScope::One` (only the scope's realm's
  bases counted into `list_seen`) and under `All` (every base); a
  location with no entry in the item counts (0 items at it, never a
  panic); the never-fetched types ordered most never fetched first,
  ties by name, and the never-fetched leagues most first, ties by name
  (1085–1098).
- The last push's CI, on `main` at `7ab91cc4`: Gate (Linux)
  `41736216175`, success; Gate (Windows) `41736216174`, success. The
  six commits since are documents and run no Rust.

## Do

1. The rulings are in the registry ahead of the code (C118 as amended,
   C162). A worker who finds either missing stops and reports; the
   step does not enter rulings.
2. `crates/acquisition-store/src/coverage.rs`, a new module registered
   in `lib.rs` beside `age` and `bound`. Its module doc is headed by
   C162 in words ("C162 — coverage and age are one derivation over a
   set of places, held once here") and says, as `bound.rs`'s does, why
   it lives in the store (every maker links the store; the facts it
   folds are the store's own ingest facts, C103) and what it does not
   do (it reads no body, C108; no policy enters it, C39). It holds,
   moved verbatim from the search crate with every doc comment and
   serde attribute and in the same field order: `FetchedLocation`,
   `TabType`, `Coverage`, `Coverage::aged`, `by_type`, and the function
   itself as `pub fn of(...)`. The function's inputs are the store's
   own types and plain numbers, so no search type crosses down: the
   realm scope as `RealmScope`; the location rows each paired with the
   live item count at it, as a slice of `(&LocationRow, i64)` or an
   iterator of it (the search builds the pairs from its `items_at`,
   so `coordinate` and `Coordinate` stay in the search); the listing
   bases as `&[ListingSeen]`; the realms held as the header gives them;
   and the item total. Its result is the `Coverage` the search's call
   site fills today, `removed` none and `leagues` the locations' (the
   call site's two overrides stay the search's). `is_folder` moves with
   it or is duplicated as the store's own one-line test on `tab_type`,
   whichever keeps the search's other callers of `is_folder` untouched.
3. The unit tests, at `crates/acquisition-store/unit/tests/coverage.rs`
   under a `#[path]` as `bound.rs:34` has it: one test per failing
   input in Input, each asserting on the `Coverage` value, never on a
   serialization.
4. The search crate: `corpus.rs` re-exports the moved types under
   their old paths (`pub use acquisition_store::coverage::{Coverage,
   FetchedLocation};`, and `tabs.rs` `pub use
   acquisition_store::coverage::{by_type, TabType};`) so the two CLI
   lines in Read and every test path stay as they are; `Corpus::read`
   calls `acquisition_store::coverage::of` with the pairs built from
   `items_at`, its two overrides unchanged; `tabs.rs`'s `Tabs::of`
   calls the store's `by_type`; `coverage()`, `by_type` and the moved
   types are deleted from the search crate. The module doc bullets
   that described them (`corpus.rs:74–83`, `tabs.rs:20–21`) say where
   the derivation now lives and keep every sentence that states a
   property.
5. The gate on the step's tree, then the commit whose message carries
   the retirement lines (C154): `retired acquisition-search/src/corpus.rs's
   coverage(); held by acquisition_store::coverage::of and
   unit/tests/coverage.rs, read by tests/answer.rs (the worked example)
   and tests/membership.rs` and `retired acquisition-search/src/tabs.rs's
   by_type; held by acquisition_store::coverage::by_type and its unit
   test, read by tests/tabs.rs:117`.

## Done when

- `decisions/frontends.md` holds C162 and `decisions/search.md` holds
  C118 as amended at the step's base commit, both as the registry's
  own entries, unchanged by the step.
- `crates/acquisition-store/src/coverage.rs` exists, its module doc's
  first line names C162, and `crates/acquisition-store/unit/tests/coverage.rs`
  holds one test per failing input listed in Input, each named for the
  property it holds.
- `grep -rn 'fn coverage(\|fn by_type' crates/acquisition-search/src`
  finds nothing; `grep -rn 'coverage::of\|coverage::by_type'
  crates/acquisition-search/src` finds the two callers.
- `git diff --stat <base>..<head> -- 'crates/*/tests/goldens/'
  CLI-REFERENCE.md MCP-REFERENCE.md` is empty over the step's range.
- No file under `crates/acquisition-cli/`, `crates/acquisition-mcp/`,
  `crates/acquisition-verbs/` or `crates/acquisition-plan/` changes in
  the range.
- `tools/arch-check.sh` reports the same edges as before (the store
  links nothing new; nothing new links the store).
- The commit message carries the two retirement lines of Do 5.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- The subject's block on the search answer, the fold in the matcher
  loop, the total line's words and the scope line's ages leaving the
  text: the next round (#155, the map's S1).
- The widening of the result by each listing base with its own age
  (C162's "each listing base with when it was seen"): the shop and
  price status round (the map's S3), where the first consumer of it
  is. This step keeps `list_seen` as the minimum it is today.
- The `list_seen` narrowing in `answer.rs:2087–2093`: the answer's,
  untouched.
- Any word a user meets, any `describe` gloss, any golden: none
  changes; a worker who finds one must change has found a fault in
  the move, not a task.
- The named-paths pin (the map's S4) and the scope of league-taking
  surfaces (S5).

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree: no other step runs beside it, and the
  gate holds one artifact (C84, the artifact mismatch). Scratch is the
  subdirectory the launch names.
- Every rule here binds the thing this step builds; none binds the
  session's own work beyond `TEMPLATE.md`'s.
- A step that regenerates a golden has failed its own Done when: no
  `ACQ_UPDATE_FIXTURES=1` run in this step.

## Report

`TEMPLATE.md`. Under 3 (measurements): the byte count of
`coverage.rs` and of what the search crate lost, and the test count
before and after.
