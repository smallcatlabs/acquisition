# every-epoch-words — every epoch a document carries has its words beside it

Round 3 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§4 "The structural pin" and §10 step S4, as revised by its two
reviews: `answer-facts-map-a064b508.md` finding 2,
`answer-facts-map-fixes-d3cbb0c2.md` finding 1). Written by the
orchestrating session on 2026-11-03 against the tree at `21a8a28c`
(round 2 built; every line number below from it; round 2's fix
round touches `tests/answer.rs` and `describe.rs:247`, neither cited
here). No new ruling: the step builds a reading of C53 the
orchestrator took as clear-cut under the owner's delegation and lists
at the close (the map's §11 question 3): C53 says "an age the text
prints stands in words beside its epoch in the JSON" and, of the MCP,
"the JSON is the text the reader gets", so on that surface every
epoch a document carries is an age the text prints, and owes its
words under C53's own letter. The owner may ask for a line of its own
instead; the mechanism is the same. To be reviewed cold before a
worker reads it.

## Goal

When this step is done, every epoch any document a verb returns
carries has its words beside it under its own name and `_age`, made
at the answer's time (a plan's measured from its own `generated_at`,
as its listings' are), and a pin in the verbs crate holds the class
two ways: a list of named paths, each read from a fixture that holds
a row, and a walk over every document for a key shaped like an epoch
with no sibling words, so the next bare epoch fails the gate. The ten
bare epochs the map and its reviews found gain their words: `tabs`'
rows' `listed_at`; `show`'s `first_seen` and `last_seen`; the search's
and `show`'s `basis.snapshot.at`; the events summary's `first`; the
queue list's rows' `submitted_at` and `updated_at`; the policy's
`created_at`; the price report's and list's `basis.at` and
`manual.updated_at`; the shop render's listing header (`taken_at`,
`stash_listing.fetched_at`) and its `rendered_at`; the plan's
`basis.snapshot_taken_at`. No text changes anywhere: the words join
the JSON, which is the MCP's text and the CLI's `--json`.

Why: the class "an epoch serialized with no words beside it" is held
today by discipline, each `aged` method naming its fields by hand
(`TabRow::aged` sets one of its two epochs, `CharacterRow::aged` both:
`crates/acquisition-store/src/lib.rs:744–760`); the MCP-2 seat worked
a date out by hand from `first_seen` and retracted it (its debrief).
Structure over discipline, at the cost of one test and one list.

## Read

- `decisions/frontends.md:11`, C53 (the two sentences quoted above;
  "every nothing says which nothing"), C146 (no rename: a new key is
  additive, `_age` beside the epoch's own name), C149 (one JSON
  object). `AGENTS.md`: C116, C122, C154. `decisions/plans.md`: C38
  (the plan's envelope is bound: a plan's words sit in its wrapper).
- `crates/acquisition-store/src/age.rs`: `ago`, `before`, and `SAID`
  (22), the gloss the MCP prints for `ages`
  (`crates/acquisition-mcp/src/main.rs:737`), whose sentence covers
  every new key without change.
- The sweep, its command and its reading (Input).
- Each maker and each bare epoch:
  `crates/acquisition-store/src/lib.rs:744–760` (`TabRow::aged`,
  `CharacterRow::aged`; the tab's `listed_at` at 679);
  `crates/acquisition-search/src/derive.rs:206–207` (`first_seen`,
  `last_seen` on an item's facts) and `show.rs:230` (`Shown::aged`);
  `crates/acquisition-store/src/corpus.rs:120–127` (`Revision`, its
  `at`, serialized as `basis.snapshot` by a search answer and by
  `show`), `crates/acquisition-search/src/answer.rs:1626`
  (`Answer::aged`); `crates/acquisition-verbs/src/events.rs:161–166`
  (the summary's `first`, `last`, `last_age`) and `:132` (where
  `last_age` is made); `crates/acquisition-verbs/src/jobs.rs:237–266`
  (`PersistedJobs::of`: the rows, unaged) and `:575` (one job on disk,
  aged), `crates/acquisition-store/src/jobs.rs:393–399` (`submitted_at`,
  `updated_at`, `updated_at_age`); `crates/acquisition-verbs/src/frontend.rs:500–525`
  (`ShownPolicy`: `updated_at_age` alone beside a flattened
  `AnnotationRow`, whose `created_at` is at
  `crates/acquisition-store/src/annotations.rs:189`);
  `crates/acquisition-plan/src/listing.rs:163–167` (the design note:
  the ages "absent from the report itself and from `list`, which say
  none"), `:253–257` (`manual.updated_at`, its age), `:418–422`
  (`basis.at`, its age), `:553–575` (`ReportHeader`: `taken_at`,
  `stash_listing`, their ages "on `status`, whose text says it"),
  `:767` (`ShowView::aged`); `crates/acquisition-plan/src/shop.rs:517–522`
  (`ShopRender`: `listing: ReportHeader`, `rendered_at`), `:802`
  (`render`), `:911` (`rendered_at: opts.now`);
  `crates/acquisition-plan/src/lib.rs:1249` (`PlanBasis.snapshot_taken_at`),
  `crates/acquisition-verbs/src/refresh.rs:57–100` (`PlanAges`,
  `Planned::aged`: the listings measured `before generated_at`).
- The pin's home: `crates/acquisition-verbs/tests/documents.rs:1234–1309`
  (`c149_every_document_a_verb_returns_is_one_json_object`: ten
  documents over an empty world), `:1040`
  (`c53_the_events_the_refused_bodies_and_the_policy_shown_say_their_ages`),
  `:429` (`c53_a_plans_ages_stand_beside_an_envelope_that_round_trips_unchanged`),
  the helpers `scratch`, `login`, `listing` (36–110); for the
  documents the verbs tests cannot populate alone (a search answer,
  `show`, `tabs`, the price report and list, the shop render), the
  makers' own tests: `crates/acquisition-search/tests/answer.rs`'s
  world builder, `tests/tabs.rs`, `crates/acquisition-plan/unit/tests/listing.rs`
  and `shop.rs`'s fixtures. A pin at the lowest crate that holds the
  behaviour (C122): the verbs crate links every maker, so one walk
  there covers every document; a maker's own test holds its words.
- Every golden that holds one of the ten documents in JSON form, by
  grep of each epoch's key name over `crates/*/tests/goldens/`
  (`listed_at`, `first_seen`, `"at":`, `"first":`, `submitted_at`,
  `created_at`, `rendered_at`, `snapshot_taken_at`, `taken_at`,
  `updated_at`), and the fixtures `crates/acquisition-plan/reference/shop-render-schema-5.json`
  and `listing-report-schema-3.json` (an added key is additive; a
  changed shape a bump: `listing.rs:150–167`); `crates/acquisition-cli/tests/plan_json.rs`,
  `shop_json.rs`, `documents.rs:483` (a plan's ages outside its
  envelope).
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets"
  (a document's key order lost through a JSON value: build no document
  through a `serde_json::Value`).

## Input

- The sweep of the class at `21a8a28c`: `grep -rn 'pub [a-z_]*: \(Option<\)\?i64' crates/acquisition-{store,verbs,plan,search,protocol}/src | grep -i '_at\b\|_at:\|seen\|first\|last\|login\|written\|rendered\|generated\|observed\|taken\|\bat:\|since\|expires\|until' | grep -v '_age'`,
  59 fields: the ten above reach a document without words; the rest
  carry words already (`fetched_at`, `removed_at`, the event row's
  `at`, the listing's `last`, `last_login`, `written_at`,
  `generated_at`, `observed_at`, `taken_at` on `status` and `show`,
  `list_fetched_at`, `list_seen`), or are never serialized (`store
  status`'s `last_event`, `#[serde(skip)]`, `lib.rs:888–897`), or are
  store-internal rows no document carries (`snapshot.rs`'s, the
  corpus read's `LocationRow` and `CorpusItem`). The worker re-runs
  the sweep on its tree and names any instance this reading misses.
- Failing inputs, each a test that fails before its maker changes:
  for each of the ten, the path list over a fixture whose document
  carries the epoch (a populated row), failing on the missing sibling;
  the walk failing on a document that carries a bare epoch under a
  name the list does not hold (a fixture struct of the test's own,
  serialized); a plan whose `snapshot_taken_at` is measured from
  `generated_at` (`before`) as its listings are; a job list whose
  rows' words are made at the answer's time; an empty document of each
  kind, where a `None` epoch's words are `never` and a struct with no
  row carries no words.
- The last push's CI, on `main` at `f64dfd0c`: Gate (Linux)
  `41778385321`, success; Gate (Windows) `41778385239`, success. Round
  2's commits are unpushed; the push is the orchestrator's.

## Do

1. The words, maker by maker, each new key under the epoch's own name
   and `_age`, made where the document's other ages are, additive
   (`skip_serializing_if` none, as the row's siblings are):
   `TabRow::aged` sets `listed_at_age` as `CharacterRow::aged` does;
   the item's facts gain `first_seen_age` and `last_seen_age`, set by
   `Shown::aged`; `Revision` gains `at_age`, set by `Answer::aged` and
   `Shown::aged`, absent in every other carrier of a `Revision`; the
   events summary gains `first_age` beside `last_age`; the queue
   list's rows are aged as one job on disk is; `ShownPolicy` gains
   `created_at_age` beside `updated_at_age`; the price report and
   `list` age `basis.at` and `manual.updated_at` as `status` and
   `show` do, the design note at `listing.rs:163–167` rewritten to say
   every view does; the shop render's `listing` header is aged as
   `status`'s is, and `rendered_at` gains `rendered_at_age`; `PlanAges`
   gains `snapshot_taken_at_age`, measured `before generated_at`.
2. The pin, in `documents.rs`: one test over populated fixtures with a
   list of `(document, path)` pairs for every epoch a document carries
   (the ten, and every one that already has its words), each asserting
   the sibling `<name>_age` exists and is a word of the vocabulary
   (`age::span`'s forms with ` ago`, ` before generated_at`, or
   `never`); a second test walking every document's JSON for a key
   whose name ends in `_at` or `_seen`, or is `first`, `last`, `at` or
   `last_login`, whose value is an integer or null, and failing where
   no sibling `<name>_age` stands, with one named exemption list that
   is empty and must stay so; a fixture struct of the test's own with
   a bare epoch proves the walk fails. A document the verbs crate
   cannot populate is built by its maker's own fixture in that maker's
   tests and walked there by the same two functions, exported from one
   place (the verbs crate's test support, or `acquisition-client/tests/harness`
   if the verbs tests use it), never copied.
3. The goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each changed
   line read: only a new `_age` line, or a schema fixture gaining a
   key. No CLI text line changes.
4. The gloss: the `ages` entry unchanged; a per-tool gloss that names
   one of the ten epochs says its words stand beside it; no id in any
   (C116).
5. The gate; the commit with the lines: `retired TabRow::aged and
   CharacterRow::aged as two partial agings; held by one aging per
   row and the named-paths pin in documents.rs`, and the count of
   keys added by document.

## Done when

- Each of the ten epochs has its `_age` sibling in the document that
  carries it, made by its maker; `grep -rn 'listed_at_age\|first_seen_age\|last_seen_age\|at_age\|first_age\|submitted_at_age\|created_at_age\|rendered_at_age\|snapshot_taken_at_age' crates/*/src`
  finds each maker.
- `documents.rs` holds the path-list test and the walk, the walk's
  exemption list empty, and the fixture-struct case proving the walk;
  the makers' own tests walk the documents the verbs crate cannot
  build.
- Every regenerated line is a new `_age` line or a schema key; no
  line under any crate's `tests/goldens/` that is CLI text changes;
  `CLI-REFERENCE.md` unchanged.
- The commit message carries the retirement line of Do 5.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- Any text line of any surface; the subject blocks (rounds 2 and 4);
  the scope of league-taking surfaces (round 5); any rename (C146);
  the listing bases widened (round 4); a plan's envelope (C38).
- The shop's `freshness` ages under no policy (#156, round 4): this
  step ages the render's listing header and `rendered_at` alone.

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names.
- `ACQ_UPDATE_FIXTURES=1` runs once, after Do 1 and Do 2, and every
  regenerated line is read before the commit.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the keys added by document, the sweep's
count on the step's tree, and the test count before and after.
