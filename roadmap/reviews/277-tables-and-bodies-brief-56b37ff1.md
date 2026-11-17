`roadmap/briefs/277-tables-and-bodies.md` at `56b37ff1`: a builder cannot start without guessing how revived characters and all-realms searches behave.

Findings, most severe first:

1. **Decisions 1 / Done when — changes what gets built.** The read does not define what happens when a live character retains a version but has no fetch time. Under C54, the character-listing upsert in `crates/acquisition-store/src/lib.rs:1762` revives a character by clearing `fetched_at` while retaining its fetched `json`, including `metadata.version`. The proposed entry requires both version and time. Smallest change: explicitly exclude characters whose current `fetched_at` is null, and add a fetch–retire–revive case to the existing corpus tests.

2. **Decision 2 / Not in scope — changes what gets built.** “The scope’s realm” leaves the existing all-realms scope undefined. C96 explicitly supports it, `ScopeOut.realm` accepts `Realm::All`, and different realms can supply different store releases; the class table covers pc, xbox and sony together. A builder must choose between suppressing the warning, selecting one realm or changing the document shape. Smallest change: specify the all-realms representation and warning behavior, with a mixed-release acceptance case; if deliberately excluded, say so in the Goal and Not in scope.

3. **Done when — changes what gets built.** The stated checks can pass while the web answer omits the warning and the document walk never exercises its new timestamp. The terminal and web have separate rendering sites (`search_cmd.rs:1124`, `web/page.rs:1152`), while the C163 walk inspects only documents constructed by `populated()` in `acquisition-verbs/tests/documents.rs`, which currently supplies no character version. Thus a terminal golden and the unchanged walk can pass without establishing the goal across surfaces. Smallest change: require an adapter check that the web renders the shared warning, and populate and inventory `scope.store_game.fetched_at` in the document tests.

4. **Find first — wording.** The discovery commands fail or omit material they promise to find. All three commands placing `-A8` or `-A12` after the pattern fail with “unable to resolve revision”; after fixing that ordering, `pub struct Corpus\b` still finds nothing because the header is `CorpusHeader`. The `Shown` search also finds unrelated types and shows too little context to establish the absence of a scope. Finally, `never_fetched` misses prose-only goldens such as `tests/goldens/web/answer_ways.txt`. Smallest change: put options before patterns, search for `CorpusHeader`, restrict `Shown` to `show.rs` and inspect its complete definition, and include the literal “never fetched” when discovering goldens.

5. **Inputs / Decisions 5–6 — wording.** The brief carries measurement output without the producing commands, contrary to C174. The figures 2,208, 88, 24,000 and 211 of 831 support design choices, but none has a reproducible command beside it; “#277 holds the numbers” supplies neither the command nor fixed input identifiers. Smallest change: leave the measured results in their evidence home and give the brief the producing commands and exact inputs.

6. **Inputs, CI — wording.** The CI command does not guarantee finding both gates or prescribe handling a red result. `gh run list --branch main --limit 3` selects recent runs across workflows, whereas the inherited template requires the latest Linux and Windows gates separately and makes a red run the first concern or a filed issue. Smallest change: use the template’s workflow-specific commands and retain its red-run disposition.

What I did not check and why: I ran no builds, tests, gates, application commands or external queries, as requested. I did not reproduce the owner-store measurements or verify #277’s contents; those inputs are outside the supplied tree.

Form impediment: the template requests CI run ids and conclusions in Inputs while C174 says briefs never carry output, leaving those instructions in tension.

Prompt impediment: none.
