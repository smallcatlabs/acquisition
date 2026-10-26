# Codex review: step h2, `45badda4`

The outside review of the h2 commit (`roadmap/briefs/h2-build.md`), run 2026-10-26 by the committing session
(`codex exec --sandbox read-only`, the prompt `roadmap/briefs/REVIEW-PROMPT.md`). The findings verbatim; the owner reads them before any fix is built.

45badda4 — the brief’s done-when does not fully hold; the report and gate are not verified.

- **P2 — `crates/acquisition-plan/src/pricing.rs:355` — shape 6:** The new `list` refusal inherits a remedy that drops the selected account. With Alice and Bob known, `acq price list --account Alice --in tab/xbox/orphan` recommends `acq price status --expand --realm xbox`, which refuses as ambiguous when no account environment override exists. Include the resolved account in the remedy; cover both callers of the shared helper with a two-account case.

- **P2 — `crates/acquisition-cli/src/store_cmd.rs:533` — shape 1:** “N skills” describes distinct skills, but the counter counts occurrences, including GGG’s repeated representation of one skill. The existing `an_item_granted_skill_stays_in_place_and_is_not_an_item` fixture contains one granted Skeletal Warrior skill under equipment and again under `skills`, and expects `granted_skills == 2`. Say “skill entries,” explain that repeated representations count separately, and make the same correction in the MCP description, field documentation and regenerated reference.

- **P2 — `crates/acquisition-cli/src/search_cmd.rs:915` — shape 3:** The explanation that never-fetched coverage ignores the query was added to MCP but omitted from CLI text. A ring query still prints never-fetched MapStash and UniqueStash totals without explaining why those types appear; the changed `tabs_pointer_names_its_league` golden also shows realm-wide totals beside a command restricted to one league. Add a sentence saying coverage includes every location in the selected realm across all leagues because unfetched contents are unknown; preserve the existing counts.

- **P2 — `crates/acquisition-cli/src/price_cmd.rs:297` — shapes 1 and 4:** The residue sentence gives a private-tab remedy to character items, which have no tab to publish. A character item carrying `~price 5 chaos` contributes to this count but cannot follow “until the tab is made public.” Distinguish the two cases and explain that a character item must first be moved into a public stash tab; correct the matching MCP description and field documentation together.

- **P3 — `crates/acquisition-mcp/src/main.rs:1165` — shape 3:** The old word remains in the `price_set` and `price_clear` descriptions despite being replaced in the server instructions. Reading either tool still encounters “Intent only”; the regenerated `MCP-REFERENCE.md:100` and `:79` preserve it. Replace both with the same plain wording used in the instructions and regenerate the reference.

What I did not check: builds, tests, fixture regeneration and gate execution, as instructed; their success and individual exit statuses are **not verified**. The per-word worker verdicts were not supplied. GitHub access failed, so issue #11 itself was unavailable; I used the committed seat reports, debriefs and call indexes. I read all 21 changed goldens, the shop fixture and both references; their changed lines have corresponding source edits.

Nitpicks for unlikely cases: none; the findings concern normal tool descriptions, an existing producer fixture or supported account/query configurations.

Brief: make the old-word check case-insensitive and distinguish unexplained wording from explained labels and preserved JSON names.

Plan: no separate implementation plan was supplied.

Prompt: explicitly distinguish committed test coverage from evidence that the tests passed.
