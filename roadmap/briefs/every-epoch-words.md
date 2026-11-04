# every-epoch-words — every time a document carries has its words beside it

Round 3 of the answer-facts group (`roadmap/reviews/answer-facts-map.md`
§4 "The structural pin" and §10 step S4, as revised by its reviews).
Written by the orchestrating session on 2026-11-03 against the tree
at `2977ea2c` (round 2 closed; every line number below from it). Cut
three times: the first draft (`b03b3e78`) and the re-cut (`a3f4f280`)
were each reviewed cold (`roadmap/reviews/every-epoch-words-brief-<sha>.md`,
the prompts beside: eight findings, then five, none a nitpick); this
cut answers the second review's five, listed at the foot under "Taken
from the reviews" with the first's eight; not reviewed a third time,
the roadmap's rule. Its issue is #164. Its ruling is C163
(`decisions/frontends.md`, beside C162), entered on 2026-11-04 on the
owner's word, "yes, as drafted": the first cut had read C53 as
already requiring words beside every epoch a document carries, the
review held that reading an extension of C53's letter, not the
letter, and the orchestrator agreed; the line was drafted here and
ruled. The second review asked two amendments of the line, both the
owner's: a clause for words under a path the document declares where
the time sits in a value that is identity (the search basis's, Do 3),
and the walk clause said of a time that is not null; and that Unbuilt
name #164. He amended it on 2026-11-04 ("Yes, amend it"); the line as
it stands in the registry is the one the design below rests on.

## Goal

When this step is done, every time any document a verb returns
carries has its words beside it, under its own name and `_age` or
under a declared path in the document's wrapper, made at the answer's
time (a plan's measured from its own `generated_at`, as its listings'
are), and a pin in the verbs crate holds the class: an inventory of
`(document, time path, words path)` over populated fixtures, and a
walk over every document's owned fields as a supplement, so the next
bare time fails the gate. Words never enter a value's identity: the
basis, its revision, and every carrier of either keep their shape.
No text line of any surface changes.

Why: the class "a time serialized with no words beside it" is held
today by discipline, each `aged` method naming its fields by hand
(`crates/acquisition-store/src/lib.rs:744–760`); the MCP-2 seat worked
a date out by hand from `first_seen` and retracted it (its debrief);
the sweep and the two reviews found the inventory below, longer each
time, where the map had counted ten; the worker measures its count on
the tree. Structure over discipline, at the cost of one test and one
inventory.

## Read

- C163 as entered (`decisions/frontends.md`, beside C162; quoted in
  Input); C53 (`:11`: "an age the text
  prints stands in words beside its epoch in the JSON"; "every nothing
  says which nothing"), C146 (no rename), C149 (one JSON object), C98
  (the basis is identity: `decisions/search.md`), C38 (the plan's
  envelope is bound: `decisions/plans.md`). `AGENTS.md`: C116, C122,
  C154, C1 (the edges: no lower crate links the process harness).
- `crates/acquisition-store/src/age.rs`: `ago`, `before`, `span`, and
  `SAID` (22), the gloss the MCP prints for `ages`
  (`crates/acquisition-mcp/src/main.rs:737`), whose sentence covers
  every new key without change.
- The identity types, which gain nothing: `Revision`
  (`crates/acquisition-store/src/corpus.rs:119–127`, derives `Eq`),
  the search's `Basis` (`crates/acquisition-search/src/corpus.rs:155–170`,
  derives `Eq`; compared as `counted_at` at `answer.rs:979`,
  `tests/next.rs:281`), and their carriers: an answer's `basis`, a
  continuation's `request.basis` (`answer.rs:2106`), a `before`'s
  `counted_at` (`answer.rs:894`), `show`'s `basis`.
- The inventory's sources, each a maker and the times it serializes
  bare (Input lists the paths): `crates/acquisition-store/src/lib.rs:671–693`
  (`TabRow`: `listed_at`, `removed_at` serialized even when null) and
  `:744–760` (`TabRow::aged`, `CharacterRow::aged`);
  `crates/acquisition-search/src/derive.rs:206–208` (an item's facts:
  `first_seen`, `last_seen`; `removed_at` has its words) and
  `show.rs:230` (`Shown::aged`), `answer.rs:1626` (`Answer::aged`);
  `crates/acquisition-verbs/src/events.rs:161–166` and `:132` (the
  summary's `first`, `last`, `last_age`); `crates/acquisition-verbs/src/jobs.rs:237–266`
  (`PersistedJobs::of`: the rows unaged) and `:575` (one job on disk
  aged), `crates/acquisition-store/src/jobs.rs:393–404`
  (`submitted_at`, `updated_at`, `updated_at_age`: `aged` words the
  second alone); `crates/acquisition-verbs/src/frontend.rs:500–545`
  (`ShownPolicy`: `updated_at_age` alone beside a flattened
  `AnnotationRow`; `PolicyWritten`: its flattened `row`, `written` and
  `prior`, each an `AnnotationRow` with `created_at` and `updated_at`,
  `crates/acquisition-store/src/annotations.rs:189–190`);
  `crates/acquisition-plan/src/listing.rs:163–167` (the design note:
  the ages "absent from the report itself and from `list`, which say
  none"), `:253–257` (`manual.updated_at`), `:418–422` (`basis.at`),
  `:465–470` (`rows.unmatched[]`, each a `ManualSide` with
  `updated_at`), `:553–575` (`ReportHeader`, flattened into the
  report, so its times serialize as `taken_at`,
  `stash_listing.fetched_at`, `character_listing.fetched_at`, their
  words as `taken_at_age`, `stash_listing_fetched_at_age`,
  `character_listing_fetched_at_age`, set by `status` alone),
  `:695–700` (`ShowView`: the header, `listing`, and the container and
  item collections), `:767` (`ShowView::aged`: it ages the snapshot
  and the target's listing, neither header base nor the collections);
  `crates/acquisition-plan/src/price.rs:639` and
  `crates/acquisition-verbs/src/receipt.rs:35` (a price write's
  receipt: `written` and `prior` rows), `crates/acquisition-verbs/src/refusal.rs:338`
  (a policy conflict's `current` row), `crates/acquisition-plan/src/pricing.rs:163`
  (a price conflict's `current`, built with `updated_at_age: None`);
  `crates/acquisition-search/src/answer.rs:1467` (a continuation's
  `request.basis`, the answer's own basis carried whole) and `:891` (a
  `before`'s `counted_at`, a historical basis); `crates/acquisition-protocol/src/job.rs:86`
  (`JobInfo`, a live job's row: no time at all);
  `crates/acquisition-plan/src/shop.rs:517–522`
  and `:911` (`ShopRender`: the header cloned unaged, `rendered_at`);
  `crates/acquisition-plan/src/lib.rs:1249` (`PlanBasis.snapshot_taken_at`),
  `crates/acquisition-verbs/src/refresh.rs:50–100` (`PlanAges`: the
  wrapper's words for the envelope's times; `Planned::aged`).
- The pin's home and its fixtures: `crates/acquisition-verbs/tests/documents.rs`
  (the helpers `scratch`, `login`, `listing`, 36–110; the walk over
  ten documents on an empty world, 1234–1309; `:429` and `:483`, a
  plan's ages beside an envelope that round-trips unchanged; `:1040`).
  The verbs tests' store helpers, the store's ingest API and the
  public makers of the search and the planner can populate every
  document the inventory names; a maker's own test builders are
  examples, not dependencies.
- The assertions this step changes, each named with the fact that
  replaces it: `crates/acquisition-cli/tests/plan_json.rs:104–112`
  (`shown_row` strips `updated_at_age` alone before comparing a policy
  row) and `:182`, `:476` (a receipt's flattened row and its `prior`
  compared against `shown_row`'s result and against an earlier
  `written`: both sides of each comparison are normalized by one
  helper that strips every `_age` key, the words validated at their
  owning boundary, the verbs tests; the conflict documents'
  comparisons the same way);
  `crates/acquisition-plan/unit/tests/listing.rs:1062–1080`
  (`c53_status_and_show_say_the_ages_their_texts_print` asserts the
  report and `list` carry no ages: they carry them now, as every view
  does); the two reference fixtures,
  `crates/acquisition-plan/reference/listing-report-schema-3.json`
  and `shop-render-schema-5.json`, regenerated for added keys without
  a bump as their tests allow (`unit/tests/listing.rs:705`,
  `unit/tests/shop.rs:893`); `crates/acquisition-store/unit/tests/coverage.rs:343`
  (an empty coverage's fetch ages stay `None`: unchanged, the null
  rule below). No golden under any `tests/goldens/` holds these
  documents in JSON form (`shop_json/`, `price_json/` hold CLI text),
  so no transcript golden changes.
- `REVIEW-CHECKLIST.md`, "Frontends and the surface an agent meets"
  (a document's key order lost through a JSON value).

## Input

- **The ruling, C163, as entered:**

  > - **C163 — Every time a document carries has its words beside it.** A time a document serializes (seconds since 1970) has its age in words under the time's own name and `_age`, or under a path the document declares for it where the time sits in a value that is identity (C98, the basis) or a bound envelope (C38, the plan's); the words are made at the answer's time, or from a time of the document's own where its text measures so (C53, output levels); a null time says `never` or carries no words, as its maker says. A pin walks every document a verb returns and fails on a time, not null, with no words. *Why:* C53 words the ages a text prints, and a seat worked a date by hand from a time no text printed. *Pinned:* `acquisition-verbs/tests/documents.rs`; Unbuilt, #164.

  775 bytes as amended; `python3 -I tools/bothy.py check` passes on the
  tree that holds it.
- The sweep at `2977ea2c`: `grep -rn 'pub [a-z_]*: \(Option<\)\?i64' crates/acquisition-{store,verbs,plan,search,protocol}/src | grep -i '_at\b\|_at:\|seen\|first\|last\|login\|written\|rendered\|generated\|observed\|taken\|\bat:\|since\|expires\|until' | grep -v '_age'`,
  59 field declarations; the review's reading of them by document,
  which is this step's inventory (document → time path → words path):
  - `tabs`: `rows[].listed_at` → `rows[].listed_at_age`;
    `rows[].removed_at` → `rows[].removed_at_age`.
  - `search`: `basis.snapshot.at` → `snapshot_at_age` at the answer's
    top level (a declared wrapper path: `basis` and `Revision` are
    identity and gain nothing); a continuation's
    `view.rows.next.request.basis.snapshot.at` → the answer's
    `snapshot_at_age` (the same basis, carried whole: a declared
    mapping to the answer's words); a `before`'s
    `counted_at.snapshot.at` → `before.snapshot_at_age` (a historical
    basis, its own time's words).
  - `show`: `basis.snapshot.at` → `snapshot_at_age` at its top level;
    `item.facts.first_seen` → `first_seen_age`; `last_seen` →
    `last_seen_age`.
  - `store events --summary`: `first` → `first_age`.
  - the job list from the ledger (`persisted.jobs[]`) and one job on
    disk: `submitted_at` → `submitted_at_age`; `updated_at` →
    `updated_at_age` (the list's rows aged as one job is; one job's
    `aged` gains the first). The live list's `rows[]` are `JobInfo`,
    which carries no time: nothing to word.
  - `policy show`: `policy.created_at` → `created_at_age`; a policy
    write's receipt: the flattened row's, `written`'s and `prior`'s
    `created_at` and `updated_at` → their `_age`s; a policy conflict's
    `current` row the same; a price write's receipt (`price set`,
    `price clear`): `written` and `prior` the same; a price conflict's
    `current.updated_at` → `updated_at_age`.
  - the price report and `list`: `taken_at` → `taken_at_age`;
    `stash_listing.fetched_at` → `stash_listing_fetched_at_age`;
    `character_listing.fetched_at` → `character_listing_fetched_at_age`
    (the header is flattened; these are the wrapper paths `status`
    has); each listing's `basis.at` → `basis.at_age` and
    `manual.updated_at` → `manual.updated_at_age`; `status`'s
    `rows.unmatched[].updated_at` → `updated_at_age`; `show`'s two
    header bases (unaged today, `listing.rs:767`) and its container
    and item collections' rows, each as its `ManualSide` or basis is
    worded in `status`.
  - the shop render: `listing.taken_at`, `listing.stash_listing`,
    `listing.character_listing` → the three ages as `status` has them;
    `rendered_at` → `rendered_at_age`.
  - the plan, every mapping a wrapper path into `PlanAges`:
    `plan.generated_at` → `generated_at_age` (exists);
    `plan.basis.stash_listing.fetched_at` →
    `stash_listing_fetched_at_age` and `plan.basis.character_listing.fetched_at`
    → `character_listing_fetched_at_age` (exist, measured `before
    generated_at`); `plan.quote.observed_at` → `quote_observed_at_age`
    (exists); `plan.basis.snapshot_taken_at` → `snapshot_taken_at_age`
    (new, measured `before generated_at`).
  - already worded, in the inventory as such: `fetched_at`,
    `removed_at` on items, the event row's `at` and the listing's
    `last`, `last_login`, `written_at`, `generated_at`, the quote's
    `observed_at`, the listing bases on `status` and `show`,
    `list_fetched_at`, `list_seen`, the span's fetches.
  - never serialized or no document's: `store status`'s `last_event`
    (`#[serde(skip)]`, `lib.rs:888–897`); `snapshot.rs`'s rows; the
    corpus read's `LocationRow` and `CorpusItem`; a job's `params` and
    a refused body, opaque JSON the walk treats as opaque.
  The worker re-runs the sweep on its tree and names any time this
  inventory misses.
- The null rule: a null time's words are `never` where its maker says
  so today (`list_seen`, a never-fetched row's `fetched_at`) and
  absent where its maker says so today (the span's fetch epochs,
  `coverage.rs:136` and its test at `unit/tests/coverage.rs:343`; an
  item's `removed_at`); both tests of Do 2 apply it: a time that is
  not null must have its words, a null time satisfies its convention
  either way, and this step changes neither convention.
- Failing inputs, each a test staged before its maker changes (Do 1):
  the inventory test failing on each of the bare times over a
  populated fixture; the walk failing on a document that carries a
  bare time under a name the inventory does not hold (a fixture
  struct of the test's own); a plan's `snapshot_taken_at_age` as
  `before generated_at`; a job list's rows' words at the answer's
  time; the identity types unchanged (the existing route and
  continuation tests, `tests/next.rs`, pass as they are).
- The last push's CI, on `main` at `f64dfd0c`: both gates success
  (`41778385321`, `41778385239`); round 2's commits are pushed at
  this brief's commit, their runs named at cut time.

## Do

1. C163 in the registry: a worker who finds it missing stops and
   reports.
2. The pin first, red: in `documents.rs`, one test over populated
   fixtures with the inventory above as a table of `(document, time
   path, words path)`, each row asserting the words path holds a word
   of the vocabulary (`span`'s forms with ` ago` or ` before
   generated_at`, or `never`) or, where the time is null, `never` or
   absence as the null rule says; and a second test walking every
   document's owned fields for a key ending `_at` or `_seen`, or named
   `first`, `last`, `at`, `last_login`, `oldest_fetch` or
   `newest_fetch`, whose value is an integer, resolving the
   inventory's declared wrapper paths before requiring a sibling, and
   failing on a time with neither; a null under such a name satisfies
   the null rule and is not failed; opaque JSON (a job's `params`, a
   refused body) is not walked; the walk's exemption list is the
   inventory's wrapper rows and nothing else. The fixtures are built
   centrally in the verbs tests (its store helpers, the ingest API,
   the public makers). Where a maker's own crate needs the same walk,
   the pure JSON helpers live in one file,
   `crates/acquisition-client/tests/harness/ages.rs`, included by
   `#[path]`, never the process harness and never a copy. The
   inventory test is run and seen failing on every bare time.
3. The words, maker by maker, each new key additive under its own
   name and `_age` or its declared wrapper path, made where the
   document's other ages are, never on `Revision` or `Basis`:
   `TabRow::aged` words `listed_at` and `removed_at`; `Shown::aged`
   and `Answer::aged` set `snapshot_at_age` at the top level, a
   `before`'s `snapshot_at_age` beside its `counted_at`, and the
   item's `first_seen_age`, `last_seen_age`; the events summary gains
   `first_age`; `PersistedJob::aged` words `submitted_at` too and the
   ledger list's rows are aged; `ShownPolicy`, `PolicyWritten`, the
   policy conflict, the price receipts and the price conflict word
   every `AnnotationRow` they carry; the price report and `list` are
   aged as `status` is (the design note at `listing.rs:163–167`
   rewritten: every view says its ages), `status`'s unmatched rows
   and `show`'s header bases and collections with them; the shop
   render's header is aged and `rendered_at` worded; `PlanAges` gains
   `snapshot_taken_at_age`.
4. The assertions named in Read moved to their replacing facts; the
   two reference fixtures regenerated for the added keys; no
   transcript golden changes (`git diff --stat -- 'crates/*/tests/goldens/'`
   empty); `CLI-REFERENCE.md` and `MCP-REFERENCE.md` unchanged.
5. The gloss: the `ages` entry unchanged; a per-tool gloss that names
   one of these times says its words stand beside it; no id in any
   (C116).
6. The gate; one commit, its message naming this brief, the keys
   added by document, and `retired nothing`.

## Done when

- C163 is in `decisions/frontends.md` at the step's base commit.
- `documents.rs` holds the inventory test and the walk; the
  inventory's rows are every time path of Input; the walk's exemption
  list equals the inventory's wrapper rows; the fixture-struct case
  proves the walk fails on a bare time.
- Every time path of Input that is not null has its words path
  present on a populated fixture, and a null one satisfies its
  convention; `Revision` and `Basis` are unchanged (`git diff` of
  their definitions empty); `tests/next.rs` unchanged and passing.
- `git diff --stat <base>..<head> -- 'crates/*/tests/goldens/'
  CLI-REFERENCE.md MCP-REFERENCE.md` is empty; the two reference
  fixtures' diffs are added keys only.
- The commit message carries `retired nothing` and the keys by
  document.
- `tools/gate.sh` is green on the step's tree, every stage 0.

## Not in scope

- Any text line of any surface; the subject blocks (rounds 2 and 4);
  the scope of league-taking surfaces (round 5); a rename (C146); a
  plan's envelope (C38); the shop's `freshness` under no policy
  (#156, round 4); the null rule's two conventions (left as they are).

## Rules

`TEMPLATE.md`. What differs for this step:
- The main tree, not a worktree; scratch is the subdirectory the
  launch names.
- `ACQ_UPDATE_FIXTURES=1` runs once, for the two reference fixtures
  alone, after Do 3; every changed line is an added key.
- Every rule here binds the thing this step builds.

## Report

`TEMPLATE.md`. Under 3: the keys added by document, the sweep's
count on the step's tree, and the test count before and after.

## Taken from the reviews

The second cold review, of `a3f4f280`
(`roadmap/reviews/every-epoch-words-brief-a3f4f280.md`): five
findings, none a nitpick, each answered in this cut; on the line it
asked for a wrapper-path clause, a null-safe walk clause and an issue
for Unbuilt, which are the owner's (#164 opened for the last).

1. A continuation's and a historical basis's copies of the snapshot
   time are declared mappings: the continuation's to the answer's own
   words, the `before`'s to its own.
2. The inventory gains the price receipts' and conflicts' rows, the
   policy conflict's row, and `show`'s header bases; the count is the
   worker's measure.
3. Every existing wrapper mapping is enumerated (the plan's four), the
   headers' flattened paths are spelled as they serialize, the ledger
   list is told from the live one, which carries no time.
4. Both tests apply the null rule: words for a time that is not null,
   a null's convention either way.
5. The receipt and conflict comparisons normalize both sides by one
   helper; the words are validated at their owning boundary.

The first cold review, of `b03b3e78`
(`roadmap/reviews/every-epoch-words-brief-b03b3e78.md`): eight
findings, one a nitpick, and a verdict on the head, each answered in
the second cut.

- **The head:** the reading of C53 was an extension; the line is
  drafted, ruled and entered as C163 (2026-11-04, "yes, as drafted").
1. Words never enter identity: `Revision` and `Basis` gain nothing;
   the words sit at the answer's and `show`'s top level under a
   declared wrapper path; the continuation's and `before`'s carriers
   carry none; the route and continuation tests are named.
2. The inventory carries a words path, and the walk resolves declared
   wrapper paths before requiring a sibling.
3. The inventory is by document and path, seventeen times across ten
   documents, the receipts, the report and list headers, the unmatched
   rows, the collections and the singular job's `submitted_at` among
   them.
4. The walk is a supplement over owned fields, opaque JSON opaque, the
   fetch extremes' names included.
5. The null rule keeps both conventions and the coverage tests.
6. No JSON goldens: the affected assertions are named with their
   replacing facts; the reference fixtures regenerate additively.
7. The pin is staged red before the makers change; the helpers live
   in one file under the client harness by `#[path]`.
8. `retired nothing`.
