`roadmap/briefs/next-a1.md` at `ca1101a0`: a builder needs the identity and destination rules clarified before implementing it as drawn.

- **Decisions 2, 4, 6 — changes what gets built.** Preserving existing ids leaves Alice’s identity unresolved when P and S become one account. `price_json.rs` uses `u-price` for annotation access; `search_json.rs:746` asserts `u-search` for the same `Alice#1234`; `Index::record_login` replaces the uuid when the username matches. **Smallest change:** require one shared account UUID, explicitly permit updating identity assertions and annotation openers, and distinguish those from the item and location ids that must survive.

- **Decisions 1, 5; Not in scope — changes what gets built.** The writer’s behavior on an existing destination is unspecified. `Index::record_login` retains unrelated accounts, facts ingest operates within response scopes, and typed intent puts with no revision refuse existing rows; writing into a populated directory can therefore retain extra state or fail after partial writes. The referenced `copy-as-store` example documents an empty destination. **Smallest change:** require an empty destination and refusal before mutation otherwise; put merging, replacement and cleanup outside scope.

- **Done when, third bullet — changes what gets built.** The acceptance test does not establish the account shape promised by Decision 2. It requires the three account names and the realms and leagues to appear, but never requires items under the second account or the third account to remain login-only. A canonical world with an empty second account can meet those checks while converted tests add Bob’s items through variations. **Smallest change:** assert the promised distribution through the CLI: the primary account’s realms and leagues, the second account’s items, and the third account’s absence of fetched facts.

- **Find first, last bullet; Done when, third bullet — changes what gets built.** “Read leaves” does not resolve commands whose flags select between reads and effects. The heading command returns `acq leagues` and `acq refresh`, but omits the information that `leagues --stored` and `refresh --plan` are their offline read forms; it also returns parent headings such as `acq store`. Both modes are documented in `CLI-REFERENCE.md`. **Smallest change:** classify invocations, including their mode flags, from the full help sections and retain that classified set in the acceptance test or commit message.

- **Find first, mock facts — wording.** The UUID grep cannot establish the claim that every mock user receives the same stash. Running it finds only the profile comment and UUID construction; the relevant functions are `mock_stash_list` and `mock_stash`, whose inputs contain no username. **Smallest change:** add a function-context search for those functions alongside the existing profile and index searches.

- **Inputs, CI — wording.** The CI ids and conclusions are recorded without commands, contrary to C174’s requirement that a tree claim stand beside its discovery command. **Smallest change:** add commands that retrieve each named run’s commit, status and conclusion, or use the template’s workflow queries.

What I did not check: compilation, tests, the gate, executable behavior or external CI state, because this was a read-only brief review. I did not consult the earlier review, preserving the requested independent reading.

Form: no additional obstacle.

Prompt: no additional obstacle.
