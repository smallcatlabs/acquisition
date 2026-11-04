`answer-facts-map.md` at `d3cbb0c2`, unchanged in the working tree: sections 4 and 5 are not yet fit to put before the owner.

The five claims:

1. **Holds.** `Selection::covers_tab(id, parent)` is public (`crates/acquisition-plan/src/lib.rs:657`); `RefreshSnapshot` reads the league’s tabs without filtering by policy, type or fetch status (`crates/acquisition-store/src/snapshot.rs:503`), including folders, stored substashes and never-fetched tabs. No second read is needed.
2. **Holds.** `place.fetched_at` comes from the current joined location (`crates/acquisition-search/src/corpus.rs:579`); “left” comes separately from `removed_at`, falling back to location/parent retirement (`crates/acquisition-store/src/corpus.rs:297`). Neither is the item’s `last_seen`. The historical-subject strain is real.
3. **Holds with a correction.** The draft is 797 bytes, retains C118’s existing requirements through its text and C53 citation, and has the required markers, gloss and issue attribution; its compressed antecedent and sentence fragments need repair.
4. **Holds with a correction.** Universal epoch wording extends C53’s conditional requirement, but the proposed path list is incomplete; singular job status already words `updated_at`, while the queue list does not.
5. **Holds.** The shop fixture already separates the uncovered item’s 4,900-second sighting age from the covered stale set’s 3,800 seconds (`crates/acquisition-plan/unit/tests/shop.rs:725`), so it can support the proposed additional assertion. `crates/acquisition-plan/src/listing.rs:858` independently ages both listing bases.

Findings, most severe first:

1. **§4, “The structural pin”, and the foot: the named paths do not cover the universal property claimed.** Missing examples include search/show’s `basis.snapshot.at` (`crates/acquisition-search/src/corpus.rs:156`, `crates/acquisition-store/src/corpus.rs:125`), `policy.created_at` (`crates/acquisition-verbs/src/frontend.rs:511`, `crates/acquisition-store/src/annotations.rs:189`), price-list listing bases and manual timestamps (`crates/acquisition-plan/src/listing.rs:163`), and shop’s top-level `rendered_at` (`crates/acquisition-plan/src/shop.rs:520`). Plan’s `basis.snapshot_taken_at` also lacks a mapping in `PlanAges` (`crates/acquisition-plan/src/lib.rs:1249`, `crates/acquisition-verbs/src/refresh.rs:57`). The cited object-shape test enumerates ten documents, omitting several proposed targets (`crates/acquisition-verbs/tests/documents.rs:1240`). Conversely, singular job status already ages `updated_at` (`crates/acquisition-verbs/src/jobs.rs:575`).
   
   **Smallest change:** complete the path inventory, distinguish queue-list paths from singular status, and explicitly extend the fixture enumeration. Correct §6’s dependent `_at`/`_age` description at map:604. Quote C53’s actual condition—words beside epochs for ages printed in text, calculated at answer time—rather than independently requiring `_age` for answer-time epochs. The foot currently overstates the resolution of first-review finding 2.

2. **§4, “Where it lives” and the retained listing bases: the proposed result cannot yet express everything the block promises.** Map:392 specifies the existing scope-coverage struct, but `Coverage` retains only one `list_seen` (`crates/acquisition-search/src/corpus.rs:426`), computed as the minimum listing time (`:1126`); map:447 then promises that the block names both independently aged bases. Keeping the existing JSON fields is sound, but their route through “the one aging” remains unspecified.
   
   **Smallest change:** state that the shared result retains both named listing observations and their individual ages alongside the aggregate `list_seen`, with the existing header fields mapped to them. Alternatively, keep their aging separate and withdraw that part of the consolidation claim. The foot’s answer to first-review finding 3 is therefore only partial.

3. **§5, C118: compression breaks the promised standalone declarative form.** “Its subject” grammatically follows “A row”; “a wider scope’s apart” leaves its fact implicit; and the *Why* contains sentence fragments. The registry requires plain declaratives (`tools/bothy.py:25`).
   
   **Smallest change:** change `; its subject` to `. The subject`; end that sentence after `named for the set`; write `A wider scope's ages stand apart in JSON.` and begin `The scope line` as another sentence. Use `*Why:* readers mistook scope age for match age and lacked position age.` and end with `subject ages Unbuilt: #155, #156.` This complete combination is **799 bytes**, preserves the existing requirements, and makes the unbuilt property explicit. The foot should claim registry form only after this repair.

4. **§4’s supporting citations need two small corrections.** `tabs.rs:38` is the `Tabs` declaration, not the count-checking test or aggregation caller; the latter is `crates/acquisition-search/src/tabs.rs:154`, with count assertions in `crates/acquisition-search/tests/tabs.rs:117`. Likewise, `crates/acquisition-cli/tests/harness/mod.rs:306` describes the JSON clock normalizer; the text-golden age rule is at `:1070`.
   
   **Smallest change:** cite the actual caller/test for the retirement holder and the text age rule for golden normalization.

What I did not check and why: I did not repeat the cell-by-cell audit outside these sections, verify issue bodies or seat measurements, or reproduce observations. Those are outside this narrow round. I ran no builds, tests, applications or daemons, opened no user store, and changed nothing.

Nitpicks: finding 4. Findings 1 and 2 affect the proposed contract; finding 3 affects the expressly required ruling form.

Sections 4 and 5 should not go to the owner as they stand.
