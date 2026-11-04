`roadmap/briefs/155-coverage-fold.md` at `6935be77`, unchanged in the working tree: the move is feasible without a new dependency edge, but the brief still needs correction before building.

1. **Answered** — both tied extremes select the first location, tested in both input orders.
2. **Narrower, not sound** — the exception names `basis.build`, but its normal consequences also change emitted continuation commands and build-bearing messages.
3. **Narrower, sound** — the resolved realm is now explicit; the fixture tests the store’s filter rather than the search adapter’s realm resolution.
4. **Answered** — direct `by_type` and `aged` tests, the five requested properties, and explicit counts address the original finding.
5. **Narrower, not sound** — `describe.rs` and serialization details are covered, but the prescribed comparison omits the coverage function and adapter inputs.
6. **Answered** — `f3db3427` and C39’s correct file are named.

Findings, most severe first:

1. **Do 2 / Done when 4–5 — The preservation exception remains narrower than the unavoidable build-identity changes.** Goal permits only `basis.build` to differ ([brief:25](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/155-coverage-fold.md:25)), but continuations embed the basis in their request ([answer.rs:2039](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:2039)), encode that request into the printed command ([answer.rs:2974](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:2974)), and report build differences in message text ([corpus.rs:215](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:215)). **Smallest change:** permit the normal build identity change wherever carried, encoded or reported, including cross-build consequences; require everything else identical at fixed inputs and time.

2. **Done when 6 — The explicit source comparison excludes the principal function being moved and the call-site adaptation.** Its ranges cover the types, aging and `by_type` ([brief:255](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/155-coverage-fold.md:255)), but omit `coverage()` ([corpus.rs:1063](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:1063)) and the call that supplies its realm, rows, total and counts ([corpus.rs:856](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:856)). Unchanged goldens and MCP’s sampled gloss/size checks cannot establish complete equivalence. **Smallest change:** include `corpus.rs:1038–1040`, `:1063–1136` and `:853–866` in the comparison, explicitly checking row order, resolved realm, count lookup, item-total expression and both overrides.

3. **Do 3 / Done when 2 — Input still omits several independent properties of the fold.** Today a non-character location lacking `tab_type` uses the empty type name ([corpus.rs:1080](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:1080)); a retained type includes fetched locations’ item counts too; and `items` and `realms_held` pass through independently of the selected locations ([corpus.rs:1111](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/corpus.rs:1111)). Input does not expressly stage these distinctions. **Smallest change:** add cases for the absent type, mixed fetched/unfetched locations sharing a type with nonzero fetched-item counts, and supplied totals/held realms that cannot be reconstructed from the selected rows. All are hand-built value fixtures requiring no SQLite.

What I did not check and why: no builds, tests, gates, applications, stores or live requests were run, and CI was not independently verified, as instructed. Static consumer grep found no omitted typed consumer; the re-exports suffice. The ruling citations match the entered rulings, and the explicitly deferred listing-base expansion is consistent with the unfinished group.

Nitpicks: none of the findings above.

The step should not start as drawn; tighten the brief first.
