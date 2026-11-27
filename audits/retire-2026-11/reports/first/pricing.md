### C69 — A listing is two independent resolutions,
- class: design — a user or agent reading `acq price` / the MCP pricing tools is told an item's two sides, their relation and the effective price
- clauses:
  1. P · Y · crates/acquisition-plan/unit/tests/listing.rs:281-318: both sides are carried in full on one listing (the note and the tab name read, plus the manual row), with relation Conflict and a sentence naming both. The effective price is in the c81_ tests (499-608).
  2. M · R · restates C70 (tab coverage)
  3. P · Y · unit/tests/game_side.rs:26-60: every fixture note reads as skip, invalid-if-empty, negotiable for `~b/o`, else exact. game_side.rs:125-150: a ratio is a price in a note and invalid in a tab name. listing.rs:287-317: the note comes first, then the tab name (Source::Note, then Source::TabName).
  4. P · Y · listing.rs:320-341: the dialog residue and the non-price note "50" have no effect, the note text is still shown as written, and the tab name applies. Real scale at listing.rs:1735-1749.
  5. P · Y · listing.rs:300-380 asserts Agree, Conflict, ManualOnly and None and counts all five keys. game_only is exercised in crates/acquisition-cli/src/price_cmd.rs:1988 and :2009 (a CLI unit test over resolve): "relation game_only", 11 items.
  6. P · Y · listing.rs:283-318: the note-parser and currency-table versions, the raw note and tab-name texts, the row revision, the basis and the why sentence. The report's JSON golden and round trip are at listing.rs:~720-731.
- unheld: none
- reason: Prop · crates/acquisition-plan/src/listing.rs:1-33 (property; the Why, fused statements need locks and display states are not modelled, is not stated) · moves to crates/acquisition-plan/unit/tests/listing.rs:275
- fate: RM
- confidence: high · I did not read game_side.rs:172-400 (the currency, amount and proptest cases) or the listing golden's reference file

### C70 — A priced tab covers that tab
- class: design — a user's hand price on a tab, folder, substash or character applies to the items beneath it by a stated specificity
- clauses:
  1. P · Y · listing.rs:752-753 (i-fifty is covered by the folder f1's row, i-sub by m1's row) and listing.rs:1322-1336 (a folder covers its tabs' items, a parent covers its substash's items). Real scale at listing.rs:1705-1713.
  2. P · Y · listing.rs:612-623: an unreadable substash row on s1, under m1 which has a readable row, stops the walk. The test asserts manual_problem is set, which fails if the parent's row were walked first. No fixture has a readable substash row under a priced parent.
  3. P · Y · listing.rs:748-751: i-plain's own row beats the folder's row. listing.rs:806-812: an unreadable own row stops the walk before the tab. The chain order item → substash → tab is asserted at listing.rs:1342-1346. Item over substash is held only by composing those two.
  4. P · Y · listing.rs:766-773 (i-worn takes the row from character/ch1, inherited) and real scale at listing.rs:1789-1801
  5. M · Y · listing.rs:1768-1771: the substash row covers 5 items by inheritance. unit/tests/shop.rs:1262 (real scale) blocks those same 5 as Cell::Substash, so coverage did not make them eligible.
- unheld: none
- reason: Prop · crates/acquisition-plan/src/listing.rs:35-41 (the walk is stated; "the way a policy id does, C37" is the nearest to the Why, one shape for both kinds) · moves to crates/acquisition-plan/unit/tests/listing.rs:734
- fate: RM
- confidence: medium · clauses 2 and 3 hold by composition, not by a readable-override fixture. The real-scale fixture is committed (reference/pricing-snapshot-2026-09-07.json, per real_scale_fixture.rs:1-7), but I did not open it.

### C74 — `shop render` is a derivation, publishing
- class: design — a user running `acq shop render` / the MCP `shop_render` gets pages to paste, with every omitted item counted and given a reason
- clauses:
  1. P · Y · unit/tests/shop.rs:268-317: every item lands in exactly one cell, posted + omitted + blocked + off_page equals items, and each policy row carries its count and its why. The "publishing is out of scope" part restates C120.
  2. M · R · restates C34 (derivations reproducible from declared inputs). It is also structural: render(&ListingReport, &RenderOptions) at crates/acquisition-plan/src/shop.rs:809 takes no store or daemon.
  3. P · Y · shop.rs:487-499 asserts the page spoiler "Shop Post 1 of 1 (6 items)". The c74_pages_are_cut test (shop.rs:521) labels each page n of N. crates/acquisition-cli/tests/price_story.rs:173 reads the page from the spawned binary's stdout.
  4. M · R · restates C120 (no posting)
  5. P · P · the template is an input: the RenderOptions.template field (src/shop.rs:555-566) and the template-wrap and refusal cases of shop.rs:521. Loose half: nothing asserts the template is not stored.
  6. P · Y · shop.rs:297-298: GameLists and GameSkips are omitted and counted. shop.rs:439-449: a kind no row rules is blocked in Cell::UnruledKind. "A prior post is never read" is held by type: neither RenderOptions nor ListingReport's source, PricingSnapshot (crates/acquisition-store/src/snapshot.rs:235-255), has any post input.
  7. P · U · a rule on how the policy table is authored (its rows cite T-claims, never the render's output). No check reads Cell::why's citations (tools/trade-check.sh does not name the shop table).
- unheld: 5 simple · para · crates/acquisition-cli/tests/price_story.rs's spawned-binary story (c74_set_show_render_clear_is_one_story): after a render with a template, assert no file under the store dir holds the template text
- reason: Prop · crates/acquisition-plan/src/shop.rs:1-7 (a pure function that reads no store and sends nothing; the Why, every human surface is a derivation over a machine surface, is absent) · moves to crates/acquisition-plan/unit/tests/shop.rs:264
- fate: NT · simple
- confidence: medium · I did not read shop.rs:521-1120 (the page-cut and proptest bodies) or shop_cmd.rs's template handling

### C75 — Pricing lives in `acquisition-plan` as a
- class: edge — when the crate graph may gain a pricing crate. Clause 4 is about a future boundary, which C170 gives the owner. The rest is a mechanism or a hypothesis.
- clauses:
  1. M · N · stated at crates/acquisition-verbs/src/lib.rs:14-15 ("pricing in the planner (C75)"). No check refuses pricing elsewhere; I looked at bothy show (one code citation) and arch-check is not cited.
  2. M · R · restates P3 (generalise after two consumers)
  3. P · U · a hypothesis for a later session; no code or test can fail on it
  4. P · U · a rule on future work (a split waits for a dependency-graph property). A new crate is a C1 edge change in any case.
- unheld: 1 cut · — · only verbs/lib.rs:14 names it as a placement note; no caller relies on it as a promise, and a move would break imports, not a contract
- reason: Prop · crates/acquisition-verbs/src/lib.rs:14 · moves to crates/acquisition-plan/src/lib.rs:1 (no test holds it)
- fate: SH-KO
- confidence: medium · the class is a judgment between edge and mech (see Noticed). I did not read acquisition-plan/src/lib.rs's module doc.

### C81 — The effective price is the most
- class: design — the user or agent is told one effective price, and what the page posts follows it
- clauses:
  1. P · Y · listing.rs:499-608: a note beats the item row on a tie (the game's), an item row beats the game tab price, the game tab price beats a folder row. A non-public tab's text is residue and the manual side applies (599-608).
  2. M · R · restates C70 (tab coverage)
  3. M · R · restates C69 clause 3 (the note, then the tab name) and C80 (the nameable tab)
  4. P · Y · listing.rs:599-608: c1 is made non-public, so the note reads "5 chaos" but game.reading is None, residue is true and the relation is ManualOnly. listing.rs:350-358: the character note is residue and shown.
  5. P · Y · by type: PricingSnapshot (crates/acquisition-store/src/snapshot.rs:235-255) and resolve's input carry no post, so no side can come from one. Stated at listing.rs:12-13.
  6. P · Y · listing.rs:609-640: an unreadable item row and substash row above the game's level leave the price unresolved, while one below the tab's price lets the game decide. listing.rs:423-494: an unreadable note in a public tab is unresolved, and in a non-public tab it changes nothing.
- unheld: none
- reason: Cite · crates/acquisition-plan/src/listing.rs:12-13, :56-57 (the index-visibility half of the Why) and :92-108; the "a page must not contradict the site" half is at crates/acquisition-plan/src/shop.rs:28 · moves to crates/acquisition-plan/unit/tests/listing.rs:496
- fate: RM
- confidence: high · I did not read acquisition-search/tests/price.rs:220 (the search's read of the effective price, held by C100)

### C80 — The game side's tab for a
- class: design — the game side a user is shown for a nested item is the tab they can name and publish
- clauses:
  1. P · Y · listing.rs:819-848: i-sub's game tab is m1, i-plain's is c1 (not the folder), and priced_tabs counts no folders or substashes
  2. P · Y · listing.rs:821-834: a substash item reads its parent's name and public and reports its own name in substash_name. An orphan substash reads nothing (849-858).
  3. P · Y · listing.rs:836-838: the folder child c1 reads its own name
  4. P · Y · listing.rs:840-845: the folder f1, named "~price 9 chaos", reads None with no tab_name. Its child c1 reads its own 3 chaos, not the folder's 9.
- unheld: none
- reason: Prop · crates/acquisition-plan/src/listing.rs:14-17 (the substash half of the Why is stated; the folder half, "a folder is a grouping", is at :37 under C70) · moves to crates/acquisition-plan/unit/tests/listing.rs:815
- fate: RM
- confidence: high · I did not read the real-scale C80 lines past listing.rs:1787

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C69 | design | 5 | 1 | 5/0/0/0/1 | Prop | RM | 0/0/0 | 0/0 | high |
| C70 | design | 4 | 1 | 5/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C74 | design | 5 | 2 | 3/1/0/1/2 | Prop | NT | 1/0/0 | 0/1 | medium |
| C75 | edge | 2 | 2 | 0/0/1/2/1 | Prop | SH-KO | 0/0/1 | 0/0 | medium |
| C81 | design | 4 | 2 | 4/0/0/0/2 | Cite | RM | 0/0/0 | 0/0 | high |
| C80 | design | 4 | 0 | 4/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |

## What I did not check
- Whether the plan crate's `#[path]` test modules (crates/acquisition-plan/src/lib.rs:2477 and on) include listing.rs, game_side.rs and shop.rs. I took it from the lib.rs `#[path = "../unit/tests/..."]` pattern and did not read each inclusion line.
- The JSON golden files (crates/acquisition-plan/reference/listing-report-schema-3.json and the shop schema) and the real-scale fixture file itself.
- The MCP pricing tools' tests (crates/acquisition-mcp/tests/pricing_tools.rs). The semantics live in the plan crate and are reached through it, per C46.
- game_side.rs past line 200, and the shop proptest (shop.rs:1122).
- Whether `acq shop render` writes anything to the store (C74 clause 5). I found no test either way and did not read shop_cmd.rs's template path.

## Noticed
- C70, step 2: no fixture has a readable substash row under a parent that also has a row (the listing.rs fixture's u1 has no row, and the real-scale parent has none). "A substash row overrides its parent's" is held only through the unreadable-row case at listing.rs:612-623, and "an item row overrides both" only by composing the chain order with the walk.
- C69, clause 4: the entry cites T11 (priced by tab name) for "any other text has no effect, the tab applies", while the tests and the module doc cite T18 for that behaviour (listing.rs:276, :321; src/listing.rs:63). The citation may be the wrong claim.
- C81, clause 3 and C69, clause 3 say the same thing (the note, then the tab name), and C81 clause 2 restates C70. C81's own promises are clauses 1, 4, 5 and 6.
- C74, clause 6: "a prior post is never read" is held only because no input type carries a post; it is also C79's surface rule (the forum is a governed input).
- C75, step 5: the class rule does not decide this entry cleanly. Read as "where one crate's code sits", it is mech (session's, SH-KU). Read as "when a new crate, an edge, may be created", it is edge (owner's, SH-KO). I chose edge because a crate split changes C1's graph, which C170 gives the owner. Its *Pinned:* says "nothing yet", and that is accurate.
- C74, clause 7 (the policy's rows cite trade claims, never the render's own output) has no check. tools/trade-check.sh does not read `Cell::why`'s citations, so the clause is a rule on authors.
