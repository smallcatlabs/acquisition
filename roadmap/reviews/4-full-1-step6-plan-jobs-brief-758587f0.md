`roadmap/briefs/4-full-1-step6-plan-jobs.md` at `758587f0`: a builder cannot start as written; Q1/Q2 await answers, and the specifications below need correction.

1. **Decisions 3 — changes what gets built.** “All sent within about …” overstates a quote containing unknown scopes or excluded work. `QuoteScope::eta_seconds` can be `None`, and `Quote::not_covered` names work outside the estimates; C40 requires those limits to remain visible. Smallest change: qualify the maximum as covering only requests with known timing, explain excluded work, and require mixed-known/unknown and entirely-unknown cases.

2. **Decisions 7 / Done when — changes what gets built.** Rewording `WayOn.words` alone cannot satisfy the prohibition on command lines because the error itself can contain one. `daemon/refresh.rs` produces `no refresh token for … — run `acq auth``, and `daemon/kinds.rs::refused` preserves that message in `Outcome::Failure.error`, which Decision 7 requires displaying. Smallest change: extend the page’s wording rule to command-bearing error fields, preserve serialized errors, and pin this actual failure case.

3. **Decisions 7 — changes what gets built.** An apply need not have the single realm and league assumed by its destination rule. `daemon/kinds.rs::validate_apply` validates each child independently and permits, for example, stash requests for both Standard and Hardcore; `apply_params_of` supplies no common parent destination. The builder must guess which league wins. Smallest change: specify links for each distinct destination, or an explicit ambiguous-destination fallback, and test mixed leagues and realms.

4. **Decisions 7 / Q2 / Done when — changes what gets built.** The recommended result opening does not say how to display a selected job absent from `Listed.rows`. `Listed::read(..., false)` retains only the newest bounded sets; an apply parent can disappear behind its own children. All named kind-link tests can pass with small queues while the recommended apply landing fails on a larger one. Smallest change: require resolving the selected job independently through `Frontend::job` and `result`, with the account guard, and test an apply whose children push its parent outside the default list.

5. **Find first, job-kind commands — wording.** The commands do not deliver the promised “every kind and its params.” Running them prints the vocabulary declarations and helper signatures, while the daemon search covers only `profile`, `leagues` and `character`; it misses `validate_apply` and the other kinds’ admission rules. Smallest change: add searches for `validate_apply`, `admit_realm`, `admit_segments` and the remaining kinds, with enough context to read their definitions.

6. **Taken from the review — wording.** “All eight” and the numbered mapping carry a tree inventory without its discovery command, contrary to C174. The referenced review does contain eight findings, but the brief supplies their count and recap directly. Smallest change: retain the review’s path and remove the inventory.

- **What I did not check and why:** I did not build, run tests, execute the gate, or start the application, as instructed. I ran every read-only “Find first” command against the clean checkout at the named commit. I did not retrieve live issues, comments or CI results; conclusions depending on those remain unverified. I found no additional necessary scope exclusion beyond those already stated.

- **Brief form:** Q1/Q2 explicitly block building, so the eventual landing and result presentation remain conditional.
- **Prompt:** No obstruction.
