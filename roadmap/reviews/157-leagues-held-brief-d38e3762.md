[157-leagues-held.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/157-leagues-held.md) at `d38e3762` is not yet buildable as drawn; the amendments fit one round and one gated build commit.

The eight findings:

1. **Answered:** existing status fields survive; `league_list_latest` is additive.
2. **Answered:** optional entry realms survive beside `ids`, and matching and deduplication carry `(realm, league)`.
3. **Narrower, sound:** `named` means policy membership, independently of tab coverage; the held fragment carries policy state.
4. **Narrower, incomplete:** bounds and omissions are specified, but whole counts and `policy show`’s expansion path remain missing.
5. **Answered:** the new MCP tool takes the standard account selector; reference discovery and description bounds apply automatically; the CLI can return before `connect`.
6. **Answered:** the observation belongs to `Planned`, uses the policy plan’s snapshot, excludes selection/file plans, and reaches empty-plan text and JSON stderr.
7. **Not fully answered:** the inventory improved, but holders and incorrect citations remain.
8. **Answered:** regeneration is ordered once, in Do 7.

Findings, most severe first:

- **A — Do 1–3, done-when 1–3: the bounded held-league lists promise expansion through surfaces that cannot request it, and neither new document names its whole league count.** Concrete reading: [main.rs:833](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/main.rs:833) declares argument-free `PolicyCmd::Show`; MCP [main.rs:1742](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-mcp/src/main.rs:1742) takes `StoreParams`, whose only field is `account` at `:834–837`. The brief’s shapes at `:169–196` contain omissions but no whole count. Smallest amendment: explicitly add CLI `policy show --expand`, MCP policy-read parameters containing standard `account` and `expand`, and whole league counts beside both lists; stage twelve leagues through those adapters. Specify whether the current league occupies one of the ten slots.

- **B — Do 1 and 3: `LeagueListShown.named` lacks a defined interpretation when the policy is absent/unreadable or an entry has no realm.** Concrete reading: [brief:169](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/157-leagues-held.md:169) gives `LeaguesHeld` policy state and a realm-specific predicate, but `:194–199` gives the stored-list document only `named`, defining cross-realm aggregation for `held` alone. [SyncPolicy::league](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-plan/src/lib.rs:712) requires a realm. Smallest amendment: carry policy availability in this document too, define `named` for realm-less entries—such as membership in any policy realm—and specify unreadable-policy behavior; add those cases to Input.

- **C — Do 7, done-when 3: the holder inventory still omits assertions and a golden affected by the proposed changes.** Concrete reading: [store_answers.rs:288](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/store_answers.rs:288) asserts the complete policy answer equals `{"policy":null}`; [documents.rs:1106](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-verbs/tests/documents.rs:1106) separately asserts that shape outside the named walk. [diagnose_cmd.rs:71](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/diagnose_cmd.rs:71) reuses `status_text`, held by [diagnose.rs:173](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/diagnose.rs:173) and `goldens/diagnose/no_daemon.txt:32`. `LeagueList` constructors also live in `acquisition-store/unit/tests/lib.rs:58,94,111` and `acquisition-plan/unit/tests/ages.rs:10`. Smallest amendment: name these holders and their replacement facts before launch; include the diagnosis golden in the single regeneration.

- **D — Do 5: the retained text count still does not identify its set under the count-naming rule.** Concrete reading: [store/lib.rs:1425](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-store/src/lib.rs:1425) accumulates distinct ids across responses, and [store_cmd.rs:578](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/store_cmd.rs:578) prints that count as `league list N leagues`; the brief adds a latest-fetch parenthesis without explaining the first number. Smallest amendment: label the retained text count as league ids seen across all successful league fetches, while preserving both JSON fields. This needs no broader extents sweep.

- **E — Do 2: the insertion position remains undefined where the shop prints no `facts:` line.** Concrete reading: [shop_cmd.rs:138](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/shop_cmd.rs:138) suppresses freshness lines with nothing posted, and `:253–260` returns early with no items. The brief requires the league line on both paths but says to place it after `facts:`. Smallest amendment: add “after the head where no facts line prints.” The populated-path moves are correct: `facts` stays at index 1, `leagues` takes 2, and `sides`/`coverage` moves to 3.

- **F — Do 7: several citations still identify the wrong code or an unaffected holder.** Concrete reading: MCP `main.rs:359,391,435` are refusal-request mappings; the tools are at `:1500,1817,1753`. Price `render_status` starts at [price_cmd.rs:338](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/price_cmd.rs:338), and `:1701` belongs to an item-show assertion. The store-status `:578–582` citation needs the `store_cmd.rs` filename. Smallest amendment: correct these references and remove the item-show assertion from the “each changed” inventory.

Not checked: builds, tests, regeneration, gate execution, CI conclusions or live behavior, as instructed. Round 3’s future implementation is unavailable; reconcile its changed policy-age assertions and document walk before launch. The dependency order otherwise supports one round without intermediate commits requiring later work to pass.

Nitpicks: E and F only.

The step may not start as drawn; amend the brief first.
