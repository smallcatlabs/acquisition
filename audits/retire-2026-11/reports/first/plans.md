### C36 — The sync policy is the first
- class: design — an agent or user writing a sync policy is promised its write semantics (a set merges by realm, `replace` writes the whole) and minimal plans that never trust an agreeing count.
- clauses:
  1. P · Y · crates/acquisition-plan/unit/tests/lib.rs:383-395 (fresh facts compile to no action) and :438-465 (stale facts compile to exactly the listing plus two fetches, nothing extra). "Per-account" and "annotation" restate C35 and C34.
  2. M · R · C34 (the daemon runs no schedules)
  3. P · Y · crates/acquisition-plan/unit/tests/lib.rs:513-525 (a disagreeing count forces a fetch while the tab is fresh) and :537-543 (an agreeing count past the window still fetches as stale)
  4. P · Y · crates/acquisition-plan/unit/tests/lib.rs:1262-1286 (a named realm replaces that realm whole, others stay, a realm with no leagues is removed) and :1377-1380 (a first merge drops an empty realm); crates/acquisition-cli/tests/plan_json.rs:373-386 and crates/acquisition-mcp/tests/plan_loop.rs:294-309 through both frontends
  5. P · Y · crates/acquisition-plan/unit/tests/lib.rs:1299-1301 (`Replace` drops the realm the value leaves out); crates/acquisition-cli/tests/plan_json.rs:388-393 (`--replace`); crates/acquisition-mcp/tests/plan_loop.rs:310-314 (MCP `replace`)
  6. M · R · C52 (no blind replace), also asserted at crates/acquisition-mcp/tests/plan_loop.rs:287-293
- unheld: none
- reason: Cite · crates/acquisition-plan/src/lib.rs:28 states clause 3 and :839 states the write kinds; no comment gives the *Why:* (the C++ tracked set and clean refresh describe this one object) · moves to crates/acquisition-plan/src/lib.rs:28
- fate: RM
- confidence: medium · I did not read lib.rs:100 (the strict parse) or the docs-check-breakers citations. The class could be argued ggg ("minimal requests"), but the planner is not the guard on sends.

### C126 — A league the list lacks is
- class: design — a user or agent who names a league the fetched list lacks gets a note naming the list's age and is never refused.
- clauses:
  1. P · Y · crates/acquisition-cli/tests/new_league.rs:168-180 (a policy naming a typo league is written with exit 0, and the plan is made); crates/acquisition-mcp/tests/journey.rs:222-230
  2. P · Y · crates/acquisition-cli/tests/new_league.rs:171,177 (the note on the receipt and on the plan, held by the launch_morning golden); crates/acquisition-plan/unit/tests/ages.rs:19-24 (one note per unlisted league; a listed one gets none)
  3. P · P · crates/acquisition-store/unit/tests/lib.rs:37-67 (a league that left the latest list is not held) and :74-100 (the list recorded last wins, even if its clock ran back). The "successful" half is loose: no test records a failed leagues response and checks that the list stands. The filter is `status BETWEEN 200 AND 299` at crates/acquisition-store/src/lib.rs:891.
  4. P · Y · crates/acquisition-plan/unit/tests/ages.rs:26-29 ("2d ago", and "never" with no list); crates/acquisition-cli/tests/plan_json.rs:164-169 ("never" on the receipt)
- unheld: 3 simple · wrong · crates/acquisition-store/unit/tests/lib.rs, the C126 rows at :33-118 (record a non-2xx `Endpoint::Leagues` after a good one, then assert `league_list()` is unchanged)
- reason: Y · crates/acquisition-plan/src/lib.rs:707 · moves to crates/acquisition-plan/src/lib.rs:707 (already there)
- fate: NT · simple
- confidence: high · I did not open the launch_morning golden beyond the grep hits.

### C37 — A tab id in the sync
- class: design — a user naming a tab in the policy is promised that its children are covered and when they get planned.
- clauses:
  1. P · Y · crates/acquisition-plan/unit/tests/lib.rs:672-684 (naming m1 plans its substash s1) and :770-781 (naming a folder plans its child). Coverage is type-blind (`covers_tab`, crates/acquisition-plan/src/lib.rs:600).
  2. P · Y · crates/acquisition-plan/unit/tests/lib.rs:647-658 (cycle 1 plans the parent only) and :672-684 (cycle 2 plans the substash). The unique half follows from the type-blind `covers_tab`.
  3. P · Y · crates/acquisition-plan/unit/tests/lib.rs:770-790 (a folder's child is planned in the same cycle; the folder itself is skipped)
  4. P · P · crates/acquisition-plan/unit/tests/lib.rs:685-698 (a 0-count stub is skipped as `EmptyStub`) and :743-751 (a 0 stub over held items is fetched). Only a MapStash parent is tested: the `UniqueStash` arm of `is_empty_substash` (crates/acquisition-plan/src/lib.rs:2344) could be dropped without any test failing.
- unheld: 4 simple · para · crates/acquisition-plan/unit/tests/lib.rs, `a_named_map_tab_covers_its_substashes_the_cycle_after_its_first_fetch` (add a UniqueStash parent with a 0 stub)
- reason: Prop · crates/acquisition-plan/src/lib.rs:139 (states the property; not why an exact id match was wrong) · moves to crates/acquisition-plan/src/lib.rs:139
- fate: NT · simple
- confidence: high · I did not read crates/acquisition-plan/unit/tests/shop.rs or crates/acquisition-verbs/tests/documents.rs beyond their doc comments.

### C40 — `quote` is its own protocol request,
- class: edge — every frontend is promised that a daemon protocol request exists which projects work and never submits or reserves anything (AGENTS.md: the owner holds the protocol).
- clauses:
  1. P · P · crates/acquisition-protocol/tests/wire.rs:269,304-310 (Quote is its own pinned request variant; Submit's shape is pinned at :278); crates/acquisition-daemon/src/daemon/tests/quote.rs:115-117 (the job list and the send journal are unchanged across a quote). The "non-reserving" half is loose: nothing compares limiter state or a second quote's headroom before and after.
  2. P · Y · crates/acquisition-daemon/src/daemon/tests/quote.rs:66-71 (the work echoed as its basis), :75-97 (per-policy scope, per-window headroom, ETA, `observed_seconds_ago`), :99-114 (the unlearned route and `not_covered`: OAuth, 429 re-sends, sleep); crates/acquisition-daemon/src/daemon/tests/quote.rs:335,339-344 (per-scope queue ahead, keyed per account)
  3. P · U · "may receive a different schedule" is a permission; no test could fail on it
- unheld: 1 simple · wrong · crates/acquisition-daemon/src/daemon/tests/quote.rs, `quote_projects_per_scope_reserves_nothing_and_names_what_it_cannot_see` (quote twice and assert the learned scope's windows and ETA are the same)
- reason: Y · crates/acquisition-protocol/src/protocol.rs:344 (nothing sent, reserved or remembered; an apply may differ; an estimate, not a promise) · moves to crates/acquisition-daemon/src/daemon/tests/quote.rs:9
- fate: KO
- confidence: medium · The class is a judgement (edge, not design). I did not read the plan's `with_quote` test past :990.

### C76 — One door: an explicit selection
- class: design — a user is promised that every explicit selection (a flag or a page's ticks) is one plan, reviewed and applied like a policy plan.
- clauses:
  1. P · Y · crates/acquisition-plan/unit/tests/lib.rs:1499-1550 (a selection compiles to a `RefreshPlan` that round-trips and is spendable); crates/acquisition-daemon/src/daemon/tests/queue.rs:384-399 (the `refresh` kind is refused at submit as unknown); crates/acquisition-cli/tests/web.rs:4319-4330 (`refresh --all` says "a selection")
  2. M · R · C44 (a selection plan carries no revision)
  3. P · P · crates/acquisition-plan/unit/tests/lib.rs:1502-1509 holds the first half: a listing that is fresh is still listed first as `Selected`. The half about a never-fetched map tab's pages is loose: no test compiles a deep selection over a never-fetched map tab and then a second one after its fetch. crates/acquisition-cli/tests/web.rs:4334-4342 shows only "4 never fetched" after the first.
- unheld: 3 simple · para · crates/acquisition-plan/unit/tests/lib.rs, `c44_a_selection_plan_is_authorized_by_its_selection` (a deep selection naming an unfetched map tab plans no page; after its fetch, the next plan does)
- reason: Prop · crates/acquisition-plan/src/lib.rs:1943 (the listing is refreshed every run and a newly listed tab takes the next plan; the reason, one door rather than two, is not stated) · moves to crates/acquisition-plan/src/lib.rs:1943
- fate: NT · simple
- confidence: medium · I did not read lib.rs:1590-1668 (the characters and poe2 selection tests) or apply_loop.rs:170.

### C77 — Freshness is judged at compile time
- class: design — a reader of a plan is promised that freshness is judged when the plan is compiled and a later replan's refetch is correct.
- clauses:
  1. P · Y · crates/acquisition-plan/unit/tests/lib.rs:382-395 (at compile time 1100 the tab is skipped as fresh, age 90) and :407-416 (the same snapshot compiled past the window fetches it)
  2. P · Y · crates/acquisition-plan/unit/tests/lib.rs:404-416 (the later compile fetches the tab the first one skipped)
  3. P · Y · crates/acquisition-plan/unit/tests/lib.rs:383-395 (no action for the tab that was fresh at compile)
- unheld: none
- reason: Prop · crates/acquisition-plan/unit/tests/lib.rs:362 (the property is in the test's doc; the reason, that an offline plan has no honest cycle estimate, is not) · moves to crates/acquisition-plan/unit/tests/lib.rs:362
- fate: RM
- confidence: high · Only the plan unit tests cite C77; I did not look for an uncited freshness comment in crates/acquisition-plan/src.

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C36 | design | 4 | 2 | 4/0/0/0/2 | Cite | RM | 0/0/0 | 0/0 | medium |
| C126 | design | 4 | 0 | 3/1/0/0/0 | Y | NT | 1/0/0 | 1/0 | high |
| C37 | design | 4 | 0 | 3/1/0/0/0 | Prop | NT | 1/0/0 | 0/1 | high |
| C40 | edge | 3 | 0 | 1/1/0/1/0 | Y | KO | 1/0/0 | 1/0 | medium |
| C76 | design | 2 | 1 | 1/1/0/0/1 | Prop | NT | 1/0/0 | 0/1 | medium |
| C77 | design | 3 | 0 | 3/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |

## What I did not check
- Whether `Store::record` stores a non-2xx leagues response at all (C126 clause 3). If it never does, the "successful" half is already held by structure.
- The CLI golden files (launch_morning, the journey goldens) beyond the lines grep showed.
- The C36 strict-parse test (lib.rs:100) and the `tools/docs-check-breakers.sh` citations of C77.
- The rest of crates/acquisition-cli/tests/web.rs's refresh loop beyond :4300-4370, and apply_loop.rs:170 (`--characters`).
- The C40 plan enrichment test (crates/acquisition-plan/unit/tests/lib.rs:976) past its first lines.

## Noticed
- C76 clause 1 ("the ad hoc `refresh` kind is gone") is held by a row named for C127 (crates/acquisition-daemon/src/daemon/tests/queue.rs:384-399), so the clause may restate C127. Step 1.
- C126's *Pinned:* names only the CLI and MCP tests. The lowest crates that hold it are crates/acquisition-store/unit/tests/lib.rs:33-118 and crates/acquisition-plan/unit/tests/ages.rs, so C122 (pin at the lowest crate) would point the *Pinned:* there. Step 2.
- For C37 clause 4, the module doc at crates/acquisition-plan/src/lib.rs:146-149 calls the empty-stub skip a guard GGG's listing makes moot ("a guard, not a saving"). That reads as mechanism, so clause 4 may be M and `cut` rather than P. Step 1.
- For C40, the class rule does not clearly decide whether one protocol request's semantics is `edge` or `design`. AGENTS.md says the owner holds "the protocol"; CLASS-BRIEF.md's `edge` speaks of "that a contract exists … and how it may change". I took `edge`; a reader taking `design` would get RM or NT instead of KO. Step 5.
- C77's *Why:* restates C36's reasoning (the C++ cycle model), and C36's *Why:* is at no comment in the code. If both retire, the reason has only one place to go: crates/acquisition-plan/src/lib.rs:28 or crates/acquisition-plan/unit/tests/lib.rs:362. Step 4.
