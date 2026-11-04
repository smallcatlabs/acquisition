# What an answer says about itself: a map before the design

Provenance: Fable (`claude-fable-5-1`), one session on 2026-11-03 at
`7ab91cc4`, read-only; a read and a proposal, nothing built, no issue
changed. Committed at `a064b508`, reviewed cold by Codex
(`answer-facts-map-a064b508.md`, thirteen findings), revised by the
rules at the foot ("Taken from the review"), reviewed narrowly at
`d3cbb0c2` (`answer-facts-map-fixes-d3cbb0c2.md`, four findings) and
revised again. What was read:
`AGENTS.md`, `GLOSSARY.md`, `roadmap/README.md`, the rulings C53, C96, C98, C100, C105, C108, C115,
C117, C118, C126, C143, C146, C149, C72, C74, C77 and C45 with their
mechanisms in the code, the honesty briefs (`h1-*`, `h2-*`, `h3-*`),
`briefs/words.md`, `briefs/league-seat-fixes.md`, their `LEDGER.md`
rows, the parks they retired (`decisions/*.md` at `5862a1e2`), the
league seat whole (`audits/league-2026-11/seat/`: the README, the four
reports and debriefs), the readiness and release seats and the trade
re-sit where they touch an age or a count, every open issue's title
and the bodies of #155, #156, #157, #22, #39, #36, #12, #105, #35, #19,
#97, #7, #104, #40, #37, #44, #17, #50, #18, #38 and the closed #11,
and the code and goldens of every surface that answers. Four Opus
readers inventoried the surfaces field by field (search; plans and
policy; pricing and shop; store, jobs and daemon); every claim of
theirs used below was checked against the tree, and one was wrong (the
pricing reader held the league seat's fault 5 unlanded; the words are
at `crates/acquisition-cli/src/price_cmd.rs:380`, landed in
`e8aabdac`). What was run: `cargo build --workspace`, then the mock
over a scratch copy of the league seat's pristine store (22,721 live
items, 3,444 tabs, 65 characters; `~/seat-worlds/league-2026-11/pristine`
copied under this session's scratchpad, the mock-session skill's
environment, a second account `seat` logged in through the mock, the
daemon stopped at the end), each surface read as a user reads it, text
and `--json`, and three tools through `acq-mcp` over stdio. Every
number below names its command or its call.

The brief's questions are answered in §2 to §8; §9 is the checklist
for the owner's own run; §10 is the next steps; §11 is his alone.

## 1. The verdict in one paragraph

The system is closer to right than the issues suggest, and the gaps
are not small and separate: they are one missing idea seen from three
sides. Every answer already carries facts about itself, the words are
one vocabulary (`age.rs`), the JSON is one document on both doors, and
where the words name the set a number is over (the terms block) no
seat has stumbled since, and the never-fetched line's text reads once
though its JSON was read twice (the MCP-2 debrief, call 25). What is
missing is the middle: every surface derives its ages and coverage
from whatever struct it happens to hold (a row's location, the corpus
header, the listing response, the policy window, nothing), so where an
answer gives an age it is a *row's* or the *whole scope's*, where it
holds none it gives none, and never is it the age of its *subject*,
the set the reader asked about. The matches'
age (#155), the posted items' age without a policy (#156), a counts
view with no age, a price status whose text has no age, a store status
with none, and "how old is my data" with no verb to ask are one gap.
Its mirror is a second root: surfaces that take one league say nothing
of what the store holds beyond it (#157, #22, #19). The third root is
not an idea but a sweep: counts and ages whose extent (a lifetime, a
window, a limit, a file, a realm) goes unnamed, which C146 read
precisely already forbids. The collapse is one function from a set of
places to its coverage and ages, applied by every answer to its
subject and, where wider, to its scope, each block named for the set it
is over. It would retire two per-surface derivations and two partial
aging methods, and it changes the scope line's text and two surfaces'
ages, which are contract changes and not retirements; it strains at
the plan, at the shop's shape, at removed items and at the MCP's size,
and §4 says where.

## 2. The map

One row per fact a surface states about itself, by door. "Of what" is
my reading of the code, not the field's name. A cell marked `**` is a
fact of something other than what a reader would take it to be about,
or a fact a reader needs that is missing. "CLI = JSON = MCP" means one
document: the CLI's `--json` is `serde_json::to_string_pretty` of the
same value the MCP's `told` serializes (`search_cmd.rs:315-317`,
`acquisition-mcp/src/main.rs:504-506`); the MCP seat's documents and
mine agree byte for byte in content. Line numbers are at `7ab91cc4`.

The cut every list takes is one number (C53): `accounts`, `tabs`,
`characters`, `store events` and the job lists each carry `left_out`
and `left_out_needs` in the JSON and a "N of M … lists all" line in the
text (`frontend.rs:102-123`, `characters.rs:41-49`, `events.rs:111-113`,
`jobs.rs:199-204`, `store_cmd.rs:127`, `:313-329`); they are not
rows below, since each says what it cut. `store refused` is the one
list that does not (its row in §2.2). Not mapped: `describe`, `auth`'s
receipts, `diagnose`'s log tail beyond §2.5, and the web beyond §2.6.

### 2.1 Search: rows, counts, cross (CLI text; `--json` = MCP `search`)

`acq search` has no `--expand`; its audit view is `--limit`, `--routes`
and `--explain` (`CLI-REFERENCE.md`, the `search` entry).

| fact | field or phrase | of what | ruling | pinned by |
| --- | --- | --- | --- | --- |
| the matches | `total.matched`; text `total 187 matches` | the matches | C100, C115 | `goldens/seat_faults/every_total_says_the_never_fetched.txt`; `acquisition-search/tests/answer.rs:35` |
| a row's age | `rows[].place.fetched_at`, `fetched_at_age`; text `· fetched 56d ago` | that row's own location, a socketed item's its holder's (`corpus.rs:579`, `:441-443`) | C118, C53 | `goldens/seat_faults/place_and_its_age.txt`; `membership.rs` `c53_an_answer_says_each_age…`; `acquisition-mcp/tests/search_tools.rs` `a_rows_place_carries_…` |
| the scope's size | `scope.items`, `fetched`, `never_fetched`, `folders`; text `22623 items · 546 locations fetched` | `**` every live item and every non-folder location in the realm, every league, before any term (`corpus.rs:1069-1119`); a `league=Standard` query still says the realm's (`goldens/seat_faults/scope_line_by_tab_type.txt`) | C96, C118 | `answer.rs:55`; `membership.rs:113` |
| the never fetched by type | `scope.never_fetched_by_type[] {type,tabs,fetched,items}`; text `MapStash 2489 of 2582 … (counted over every location in the realm, in every league, whatever the query asks, …)` | the scope, by GGG's type, `character` a type (`corpus.rs:1077-1106`); the text names its set; `**` the JSON gloss says `items` counts the fetched locations' items (`describe.rs:238`) while the code counts items at every location of the type (`corpus.rs:1086`), equal only while a never-fetched location holds none | C118, C146 | `goldens/seat_faults/scope_line_by_tab_type.txt`; `acquisition-search/tests/seat_faults.rs:1375` |
| the oldest and newest fetch | `scope.oldest_fetch`, `_age`, `_location`; `newest_…`; text `(the oldest fetch 57d ago, Allflame / TheAbsenceOfPatience (character); the newest 53d ago, Standard / Dump Tab)` | `**` one location each, the extremes over every fetched non-folder location in the realm, every league, empty tabs included, never narrowed by the query (`corpus.rs:1073-1076`, `:1120-1125`); identical for 187 matches and for 22,623 items (my runs, the seat's calls MCP-2 32 and 37); the text names the location and not the set | C118's "ages half, which its text does not describe" (`0d8db46c`), C53, C146 | `answer.rs:60-76` `c100_the_worked_example…`; `membership.rs:1038`; 18 goldens hold "the oldest fetch" (the search reader's grep over the four test directories: 20 files, 18 of them goldens) |
| the lists seen | `scope.list_seen`, `_age`; text `· location list seen 57d ago` | the oldest of the location lists the query reads: a fixed league's tab list and the realm's character list, else every list of the realm (`corpus.rs:425-433`, `:1126-1134`; `answer.rs:2088-2093`); the one coverage fact the query narrows | C118 | `acquisition-search/tests/answer.rs:781` `the_scopes_list_age_under_a_query_fixing_one_league_is_that_leagues` |
| the pointer | text `each tab: acq tabs … --league Standard --never-fetched`; `**` no JSON field | the fixed league if it holds a never-fetched tab, nothing if it does not; when no league is fixed, the league with the most (`answer.rs:2896-2938`); #36 | C118 | `goldens/seat_faults/tabs_pointer_names_its_league.txt` |
| the terms | `terms[] {matched,failed,lacked,undecided}`; text `each term counted alone over all live pc items in all leagues, never over the matches` | the scope, each term alone (`answer.rs:1751-1764`); the text names its set; #12's contradiction has its sentence | C93, C115 | 19 goldens hold "each term counted alone" (the same grep) |
| the rows' cut | `view.rows {returned, left_out, next}`; text `rows 1–10 of 187 shown: …` | the matches past the page | C53 | `acquisition-search/tests/next.rs` |
| a count's buckets | `view.counts.tables[].buckets[].count`, `values`, `left_out`; cross `cells`, `left_out` | the matches carrying each value; `left_out` counts values or cells, never items (`counts.rs:850-886`, `:962-997`); `**` the routes line reads "over the scope" beside a count of the matches (`search_cmd.rs:1577`) | C95, C105 | `acquisition-search/tests/counts.rs` |
| a sum | `sum {value, lacking, incomplete}`; text `sum X over every match: …` | every match; lacking counted | C95, C146 | `search_cmd.rs:2235` `c146_a_sum_says_it_covers…` |
| `**` the matches' age | — | missing on every view: no oldest or newest of the matched items, on rows, counts or cross; the only ages on the page are the rows' own (ten of 187) and the scope's (#155; my counts run: the head carries the scope's ages, the table none) | — | — |
| the basis | `basis {store, build, snapshot{response, at}, intent, …}`; text `basis store … snapshot 1126 · facts v9 · prices and policy revision 0 …` | the facts revision, the intent revision and the table versions (`corpus.rs:604-627`); `**` `snapshot.at`, the nearest whole-store freshness fact, has no `_age` and no text, appearing only as UTC inside a `basis_changed` refusal (`corpus.rs:221`) | C98, C115 | every search golden |
| the undecided | `total.undecided`, `undecided_reasons` | scope items the query could not decide, counted apart | C93, C105 | `answer.rs:35` |

### 2.2 `show`, `tabs`, `characters`, `store status`, `store events`, `leagues`, `accounts`

| surface (doors) | fact | of what | ruling | pinned by |
| --- | --- | --- | --- | --- |
| `show` (CLI text; `--json` = MCP `show`) | `place.fetched_at(_age)`; text `place pc · Standard / TheFireworksCompany (equipment) · fetched 56d ago` | the item's own location | C118, C53 | `goldens/seat_faults/place_and_its_age.txt` |
| `show` | `item.facts.first_seen`, `last_seen` | `**` epochs with no words and no text (`derive.rs:206-207`; my run: `"first_seen": 1788362161`); the MCP-2 seat worked a date out by hand and retracted it (its debrief) | C53's letter holds (the text prints no such age); its spirit does not | `derive.rs:20` (presence only) |
| `show` | `basis.intent`; text `prices and policy revision 2` | the intent file's revision | C98 | describe "basis" |
| `tabs` (CLI text, `--expand`; `--json` = MCP `tabs`) | `tabs`, `fetched`, `never_fetched`, `folders`, `items`, `by_type[]`; text `3347 tabs in Standard: 405 fetched, 2926 never fetched, 16 folders (never fetched); 20231 items` | one realm and one league (`tabs.rs:139-196`) | C115, C146 | `goldens/seat_faults/tabs_counts_first.txt` |
| `tabs` | `rows[].fetched_at(_age)`; the `fetched` column | that tab | C118 | `acquisition-search/tests/tabs.rs:87` |
| `tabs` | `rows[].listed_at` | `**` an epoch with no words: `TabRow::aged` sets `fetched_at_age` alone (`acquisition-store/src/lib.rs:747-750`) where `CharacterRow::aged` sets both (`:756-758`); the MCP-2 seat's note confirmed | C53 | none (`listed_at_age`: 0 test files) |
| `tabs` | `listed: "most_items"` | `**` a ranking by item count that includes never-fetched tabs, whose count is unknown (`goldens/seat_faults/tabs_counts_first.txt` lists `-` tabs under "most items") | C53 | the golden |
| `tabs` | `**` when the league's tab list was last seen | missing: only a search answer's `list_seen` carries it; the CLI-2 seat went looking (its T3, "I found it by luck") | — | — |
| `characters` (CLI text, `--expand`; `--json` = MCP) | `characters`, `fetched`, `empty_bodies`, `never_fetched`, `items`; rows `listed_at(_age)`, `fetched_at(_age)`, `item_count`, `fetched_items` | the selection; a row's last *landed* fetch: a refused body rolls back whole (`lib.rs:1356-1380`), so a character whose newest body was refused keeps its older fetch and items, and nothing on the row points at the refusal (the CLI-2 seat's `pilatesinstructor`, its T3) | C53, C146 | `acquisition-verbs/src/characters.rs:118` |
| `store status` (CLI text; `--json` = MCP `store_status`) | `responses`, `leagues`, `league_list`, `characters`, `tabs`, `items`, `items_removed`, `events`, `refused_bodies`, `granted_skills`, `unlifted_items`, `withheld_*` | the whole file, every realm and league (`lib.rs:2049-2082`); `**` no age at all, not the league list's fetch (`LeagueList.fetched_at` exists, `lib.rs:793-797`), not the newest ingest; `**` `leagues` is a never-retired union of every id ever listed in any realm (`lib.rs:1422-1427`, an upsert), printed as `league list 3 leagues`; `responses` and `events` are all-time rows, unglossed | C146 | `goldens/seat_faults/store_status_counts.txt`; `c146_store_status…` |
| `store events` (CLI text, `--summary`, `--expand`; `--json` = MCP `item_events`) | `hours`, `limit`, `returned`, `limit_reached`; text `0 events in the last 24 h` | the request's window, named | C146 | `acquisition-cli/tests/store_answers.rs`; `documents.rs:936` |
| `store events --summary` | `events`, `locations`, `by_kind`; text `N events at M locations in the last 24 h` | `**` the newest events the limit returned, counted as the window's (`events.rs:136-141`, `events: events.returned`); the text says "(limit reached)" when the limit cut and names no count of rows (`store_cmd.rs:809`), and its zero-event return prints "0 events" with no qualification at all (`:792`); the MCP gloss names the rows; #105's third item | C146 | `documents.rs:936` |
| `store events` | `at(_age)` per event | `**` the ingest time of the response that produced the event, not when the change happened in game | — | — |
| `store refused` (CLI text; `--json`) | `limit`, `returned`, rows `fetched_at(_age)`; text `N refused body(ies)`, one body `fetched X ago` | the newest `limit` rows, 50 unasked, each fetch aged (`refused.rs:23-31`); `**` the text prints `returned` as the count and never whether the store holds more than the limit (`store_cmd.rs:664-672`), the JSON `limit` and `returned` and no whole, where `store status`'s `refused_bodies` is the whole and neither names the other | C53, C146 | `acquisition-cli/tests/store_answers.rs:289` |
| `leagues` (a job: CLI table; `--json` and MCP `job_result` the payload) | the list as GGG gives it | `**` no fetch time anywhere (the payload is `{provider, params, body, rate_limit}`); no surface reads the stored list but the web's league select and the plan's note (#157); `store status` says only `never fetched` or `N leagues` | C126 | — |
| `accounts` / `auth status` | `last_login(_age)`; text `last login 19d ago` | `**` the index's `last_login`, which login and the refresh's persist both write (`index.rs:246, 259, 332`; `refresh.rs:104, 204-207`): the last credential write, not the last interactive login | — | `goldens/seat_faults/auth_status_without_a_keyring.txt` (the keyring line alone) |

### 2.3 Plans and policy (CLI text, `--expand`; `--json`; MCP `refresh_plan`, `apply_plan`, `sync_policy`, `set_sync_policy`)

| fact | field or phrase | of what | ruling | pinned by |
| --- | --- | --- | --- | --- |
| the facts line | `basis.stash_listing{response_id, fetched_at}`, `character_listing`; text `facts: stash listed 53d ago; characters listed 53d ago` | `**` the league's newest stash *listing* response and the *realm's* character listing (`snapshot.rs:142-146`, `:414-430`), measured at `generated_at` and worded "ago" (`plan_cmd.rs:773-799`, `:835-837`); the text says neither "listing" nor "realm"; no age of the items is on the line | C77, C53 | `goldens/journey/a_first_session.txt:39`; `new_league/launch_morning.txt:75` |
| an action's age | `actions[].reason {kind: stale, age_seconds}`; text `322 stale (53d–56d)`, `stale for 53d` | that location's own fetch at compile (`lib.rs:2358-2370`); the one per-location age the plan carries | C77 | `lib.rs` unit tests |
| the skips | `skipped_tabs[] {reason}`, `skipped_characters[]`; text `skipped 16 tabs: 16 folders — containers only` | `**` covered tabs only (`lib.rs:2243-2245`); the tabs the policy does not name appear nowhere, no list and no count (#22; no test I found pins their absence: `crates/acquisition-plan/unit/tests/lib.rs:679` is a child named directly, covered as before); the no-league characters of the realm are counted in every league's skip (`snapshot.rs:146-149`) | C37, C61 | `goldens/new_league/launch_morning.txt:88`, `:111` |
| the quote | `quote.scopes[] {requests, queued_ahead, eta_seconds, observed_seconds_ago, notes}`, `not_covered`; text `quote: ETA unavailable until 2 rate-limit policies are learned` / `stash-request-limit 7 requests — ~<eta>s` | the daemon's limiter at `observed_at`, per serial key (`quote.rs:37-133`); `null` means unlearned or degraded, and the text says so; `**` the decision view drops `observed_seconds_ago`, so a "ready" rests on headers of unstated age (`plan_cmd.rs:1676-1730`); no total time on either view | C40 | `daemon/tests/quote.rs:94-113`; `launch_morning.txt:53-54`, `:91` |
| the wire | `wire_sends {min, max, prerequisites}`; text `wire: 3374–10122 sends; plus first-contact probes and token refresh if needed` | the actions times one to three; the "plus" clause is two fixed strings whatever the routes know (`lib.rs:1310-1325`) | C42 | `plan_json.rs:263` |
| the league note | `notes.league {league, list_fetched_at(_age)}`; text `note: the league list was never fetched, so Standard is not read against it` | `**` the latest `leagues` response of any realm, matched by id alone (`lib.rs:741-758`, `LeagueList` has no realm); silent when never fetched on a plan (`:831`), said on the policy receipt (my run) | C126 | `journey.rs:209-212`; `launch_morning.txt:189`, `:198` |
| two clocks, one name | `notes.character_listing_fetched_at_age` (at now, `lib.rs:833`) beside `Planned.character_listing_fetched_at_age` (`before generated_at`, `refresh.rs:68`, `:88`) | `**` one key name under two measures | C53 | `crates/acquisition-plan/unit/tests/ages.rs:43`; `documents.rs:483-494` |
| the apply's changes | `store_changes`; text `changed: no items — 7 responses recorded, nothing added, changed, moved, or removed` | `**` the whole account file's delta since the apply started (`refresh.rs:193-216`): another apply's items count too | — | `a_first_session.txt:67`, `:78` |
| what is left | `remaining_requests`; text `N requests more are planned now (what this apply revealed, such as a map or unique tab's substashes, what it left undone, or what went stale since)` | a fresh compile against the current policy and store at the apply's end (`refresh.rs:495-509`), never a residue of this plan; the parenthetical a fixed list of causes, printed on a spent replay too (`plan_cmd.rs:489-494`) | C111 | `goldens/seat_faults/an_apply_ends_on_what_is_still_planned.txt`, `a_spent_plan_applied_again.txt` |
| nothing left | text `left: nothing — every covered entry is fresh`; `nothing to do in Rehearsal: covered entries are fresh (under 1h) or do not need fetching` | the recompile at zero; `**` "fresh" printed beside skipped folders and stubs (`launch_morning.txt:103`); "under 1h" is the window, not an age; the headline names the league, the set is the covered one | C77 | `the_second_round_leaves_nothing_planned.txt` |
| a selection plan's window | `max_age_seconds: 0` | `**` a field about no window (`lib.rs:1991`) | C76 | — |
| `policy show` | the row, `revision N, updated X ago`; with none, the offered first write | `**` nothing of which leagues or realms the store holds that the policy does not cover, nothing of the league list (#157) | C36 | `plan_json.rs:108-120`; `launch_morning.txt:15-32` |
| the MCP's apply | `{job_id, account, requests, spent, skipped}` | `**` no `store_changes` and no `remaining_requests`: the MCP never waits (`landed` is the CLI's and the web's); what is left is learned by replanning | C13 | `plan_loop.rs:124`, `:161-162` |
| the describe | `describe ["ages"]` glosses "before generated_at"; `**` no entry glosses `skipped`, `logical_requests`, `remaining_requests`, `wire_sends` or `eta_seconds` | — | C146 | — |

### 2.4 Pricing and the shop (CLI text, `--expand`; `--json`; MCP `price_status`, `price_list`, `price_show`, `price_set`, `price_clear`, `shop_render`)

| fact | field or phrase | of what | ruling | pinned by |
| --- | --- | --- | --- | --- |
| `price status` counts | `counts.items`, `by_effective`, `by_relation`, `by_game_statement`, `invalid_notes`, `residue`, `priced_tabs`; text `21311 items in Standard: 492 the game decides, 0 decided by hand, 20819 nothing applies` | the league read, Standard given silently (`main.rs:598`); `items` includes the realm's league-less characters' items in every league, and the text says so in a line of its own when any exist (`snapshot.rs:764-770`, `price_cmd.rs:389`); `**` no door names the other leagues the store holds (#19; `rows: … 2 for other leagues` counts rows and names no league) | C53, C146 | `goldens/seat_faults/price_status_other_leagues.txt`; `price_cmd.rs:1234` |
| `price status` ages | `stash_listing_fetched_at_age`, `character_listing_fetched_at_age`, `taken_at_age` | `**` the listing's age and the realm's character listing's, in JSON always and in text only under `--expand` (`price_cmd.rs:413-424`): the decision view carries no age at all (my run); `taken_at_age` is the answer's own clock, `0s ago`, reading as freshness; no age of the priced items' own fetches anywhere | C53 | `price_cmd.rs:1271` (a unit test, no golden); `crates/acquisition-plan/unit/tests/listing.rs:1066` |
| `price status` words | `skipped (~skip)`; `inherited`; `invalid_notes` "(no effect: a hand row or its tab's name applies, where either does)" | `**` "skipped" overclaims: a nearer hand row beats a tab's `~skip` (C81); `inherited` is the manual side's inheritance, not the items a container row decides; the invalid-notes words landed (`price_cmd.rs:380`, `listing.rs:525`) | C81, C146 | `price_cmd.rs:1288` |
| `price list` | `total`, `listed`, `left_out`, `items_on_record` | the selection before and after the bound; `**` no row carries an age in text, JSON epochs only (`listing.rs:164-168`) | C53 | `price_json.rs:458` |
| `price show` | `manual.updated_at_age` "set 1m ago"; `basis.at_age` under `--expand`; `taken_at_age` | the row's last write (a container's, when inherited); the subject's own basis | C53 | `goldens/price_json/list_show_and_refusals.txt` |
| a receipt | `revision`, `prior`, `undo`, `in_facts`, `note` | the row; `**` `in_facts` is "the address exists in facts in any league, removed included" (`pricing.rs:899-901`, `lib.rs:2180-2186`), wider than status's "name nothing in the facts"; nothing of the items the row decides (#48) | C78, C146 | `pricing_tools.rs:545`, `:1012` |
| `shop render` counts | `counts {items, posted, omitted, blocked, off_page, by_cell}`; text `21311 items in Standard: 31 items to post on 1 page; 492 omitted …` | the league's items by cell | C74 | `goldens/shop_json/shop_render.txt`; `crates/acquisition-plan/unit/tests/shop.rs:266` |
| the ranks | `Stash57`; text `Stash57 is the tab "ACQUISITION-PRICE-TEST"` | `**` the tab's rank as of the *stored* listing (`shop.rs:634-660`), glossed in the present tense (`pricing.rs:1155`) | C146 | goldens |
| the freshness | `freshness {policy, uncovered, stale, oldest_stale_seconds(_age), position_before_listing}`; text `policy: none set, so no refresh covers the page's facts` | `**` under `not_set` the function returns before any age (`shop.rs:1119-1129`): the render holds every posted item's `last_seen` and the listing's `fetched_at` (`shop.rs:517-519`, `:910`) and prints none of it; with a policy set my run said `stale_total: 31, oldest_stale_seconds_age: "53d ago"`, without one `policy: not_set` and every list empty; the text prints no freshness at all when nothing is posted (`shop_cmd.rs:138`); `position_before_listing` compares response ids and cannot see a reindex after the listing (#156) | C72 | `crates/acquisition-plan/unit/tests/shop.rs:695`; `shop_cmd.rs:636`; `shop_json.rs:468` |
| the listing header | `listing.stash_listing.fetched_at`, `taken_at` | `**` epochs without words in the render's JSON; no `_age`, since the render calls `resolve` and never `status()` (`pricing.rs:991`) | C53 | `crates/acquisition-plan/reference/shop-render-schema-5.json:221-238` |

### 2.5 Jobs, daemon status, diagnose (CLI text; `--json`; MCP `list_jobs`, `job_status`, `job_result`, `daemon_status`)

| fact | field or phrase | of what | ruling | pinned by |
| --- | --- | --- | --- | --- |
| a job's wait | `eta_seconds`; text `eta ~Ns`, `rate limited, starting in ~Ns` | the limiter's prediction of a waiting job's start, its route simulated at the count of same-route jobs ahead (`daemon.rs:850-883`); `**` the words say "rate limited" and not that queue depth is in it; raw seconds, not the vocabulary | C4, C40 | — |
| the job counts | `total`, `by_state`; text `8 jobs: 0 running, 0 waiting, 8 done` | `**` this daemon's lifetime, memory plus the ledger rows from the first id it issued (`queue.rs:397-415`), every account, phrased bare | C45, C146 | `store_answers.rs` |
| the ledger read | `persisted {total, waiting, recorded_running, written_at(_age)}`; text `on disk: 0 waiting, 0 recorded running, written 4m ago` | the open rows; `written_at` is the last write of any row, terminal ones included; `**` the text's `summary_text` prints `0 done` for a count never read (`goldens/persisted_queue/the_queue_read_from_the_ledger.txt`); the rows' `submitted_at`, `updated_at` epochs carry no words (`verbs/src/jobs.rs:262`) | C45 | the golden |
| a failed job | `{job_id, account, outcome: "failure", error}` | `**` a sentence: no `kind`, no `next`, no finish time, the body as "a body of 25 bytes" (#7; my run) | C11, C23 | `goldens/apply_loop/a_failed_applys_result_from_the_ledger.txt`; `documents.rs:906-926` |
| `daemon status` | `uptime_seconds`, `idle_shutdown_seconds`, `policies_known`, `rails.sends`, `connections`, `jobs_*` | `**` `policies_known` and `sends` are this lifetime's (the limiter is never persisted; `rails.rs:33`), the text `sends 17` unqualified beside a journal path that spans lifetimes; `policies_known` is rate-limit policies, glossed only in `describe ["daemon"]` and `protocol.rs:603` (the MCP-1 seat read sync policies, its T2, retracted); `**` the idle exit's moment is not carried (the retrospective, §5); `**` `protocol.rs:595` says the knob's default is 60 where `daemon.rs:172` holds 3600 | C3, C152 | `acquisition-mcp/tests/daemon_status.rs` (identity fields alone) |
| `diagnose` | `store status` for every account; the log's tail | the same missing ages; `**` log lines carry `[{uptime}s]` of the writing daemon, not wall time (`daemon.rs:1434`), so lines of two lifetimes read as one | — | `goldens/diagnose/no_daemon.txt` |

### 2.6 The web

The pages render the verbs' documents (C16): the search page prints
`answer_head` word for word (`search_cmd.rs:947-949`), so §2.1 holds
there; the plan page prints the CLI's plan text whole, its `next: acq
refresh --apply …` included (`web/mod.rs:743-751`, through
`render_plan`); the apply drops `remaining_requests` and the
skip counts (`web/mod.rs:766-796`); the league select marks `(tabs on
disk)` and never policy coverage (`goldens/web/refresh_loop.txt:61`);
there is no shop page. Nothing the web adds says anything about
itself the verbs do not.

## 3. The roots

Three. Each in one sentence, then what it settles by the issue's own
"Closes when", what it touches, and the gaps the map found that no
issue names.

### Root 1. Ages and coverage are of a row or of the whole scope, never of the answer's subject

**The idea in a sentence:** every surface derives its self-facts from
whatever struct it holds (a row's location, the corpus header, the
listing response, the policy window, nothing), so an answer says how
old a row is or how old everything is, or, holding neither, nothing at
all (`price status`'s text, `store status`, a counts view), and never
how old *what it answered about* is.

The evidence is in the code's own words. `Coverage` is "the earliest
and the latest of the fetched locations' last fetches, each one
location's; a row's place carries its own" (`corpus.rs:405-407`); it is
computed once per corpus read from the header (`corpus.rs:856-864`) and
cloned into every answer with only `list_seen` narrowed
(`answer.rs:2088-2093`). The commit that named its locations said of
C118 that it "describes the never-fetched half of the scope; this adds
to the ages half, which its text does not describe" (`0d8db46c`). The
matcher already walks every item with its place in hand
(`answer.rs:1751-1782`) and pushes the matches; the matches' oldest and
newest are one fold that loop does not make. The shop holds every
posted item's `last_seen` and prints it only through the policy's
window (`shop.rs:1119-1129`, `:1139-1195`). `price status` has the
listing's age in its struct and prints it only under `--expand`
(`price_cmd.rs:413-424`). `store status` has no timestamp in its
struct at all (`lib.rs:846-895`). A `tabs` head has counts and no list
age; a `characters` row has two ages and a `TabRow` one
(`lib.rs:747-758`).

- **Settles by its own "Closes when":** #155 (the matched items' oldest
  and newest fetch with their places beside the scope's, on rows, counts
  and cross, both surfaces, each a golden fact); #156 (a render's
  freshness carries the listing's and the posted items' ages whether or
  not a policy is set, both surfaces, pinned in the render's goldens):
  the posted set's oldest and newest fetch and its listing's age are
  exactly the subject's coverage.
- **Touches, does not settle:** #39 (the never fetched beside the total
  by the query's category's tab types): the subject's coverage can say
  nothing of what was never fetched, since a never-fetched location
  holds no match; #39 needs a reading from category to tab type, which
  is the trade catalogue's, a different idea. #12 (a term's count
  beside the total) names its own fire, "a run spends a call on the
  terms block": the block now says "never over the matches"
  (`search_cmd.rs:1379`), the re-sit's count was one re-read and no
  call, and the two league-seat runs did not stop on it; whether that
  closes it is the owner's (§11, question 7), and this root does not.
- **Gaps no issue names:** a counts or cross view carries no age of
  the counted items (§2.1); `price status`'s decision view carries no
  age (§2.4); `store status` carries no age and "how old is my data"
  has no verb (the CLI-2 seat, T3); a `tabs` head says not when its list
  was seen (§2.2); `show`'s `first_seen` and `last_seen` are bare
  epochs (§2.2); the plan's "characters listed" is the realm's listing
  under a league's heading (§2.3); the shop's listing header is bare
  epochs (§2.4); the ledger rows' and the events summary's `first`
  epochs are bare.

### Root 2. A surface that takes one league says nothing of what the store holds beyond it

**The idea in a sentence:** `price status`, `shop render`, `tabs` and
the plan each take a league as their scope, Standard by default, and
`policy show` shows the stored row whole (`frontend.rs:501`); no block
says what else the store holds or what the policy leaves out, where a
search answer says both.

The search answer's scope block already does this job: `realms_held`,
`leagues`, `never_fetched_leagues` and the pointer (`corpus.rs:380-404`).
The league-taking surfaces have no scope block: `price status` names
no other league (`goldens/seat_faults/price_status_other_leagues.txt`,
where Allflame's three bad notes appear only when asked by name, the
CLI-2 seat's T5); the plan lists covered tabs and never the league's
others (`lib.rs:2243`, #22); `policy show` prints the row and nothing
of the store (`frontend.rs:501-525`); the stored league list is read
by the plan's note and the web's select and by no verb (#157); `tabs`
takes a league and offers no all-leagues view (the CLI-1 seat's T3).

- **Settles:** #157 (both surfaces read the stored league list without
  a daemon, each league beside whether the policy covers it; the README's
  new-league line names that read); #22 (a count of the league's tabs
  outside the policy, in the plan's summary); the league-seat comment on
  #19 (one line per league with price state, or the leagues the store
  holds named beside the default), though #19's own body is phase 4's
  sticky default and stays open.
- **Touches:** #36 (the pointer's league under two `league=` terms):
  the pointer is the scope block's; its rule stands. C126's realm-blind
  match (`lib.rs:748`): a bug the scope block would meet when it reads
  the list per realm.
- **Gaps no issue names:** `shop render` without `--league` reads
  Standard and says nothing of the rest (the CLI-2 seat's fault 6, filed
  on #19 only); `policy show` with a policy says nothing of the store's
  leagues; `store status` says `league list 3 leagues` for a union
  across realms.

### Root 3. A count or an age is of an extent the sentence does not name

**The idea in a sentence:** a number is taken over a lifetime, a
window, a limit's rows, a file or a realm, and the words beside it say
neither which nor that there is one.

This is C146 read precisely, not a new idea: "a number a frontend
prints names what it counted, where it came from" covers the extent,
and the words step applied it to `never_fetched_by_type` and the terms
block and no further. The cases: the events summary counts the rows the
limit returned and says "in the last 24 h", "(limit reached)" its one
qualification (§2.2); `store refused` prints a count and not whether
more exist (§2.2); `daemon status`'s
`sends` and `policies_known` and `jobs`' `total` are this lifetime's,
bare (§2.5); `store status`'s `leagues` is a never-retired union
(§2.2); `accounts`' "last login" includes refresh writes (§2.2);
`price status`'s "skipped" overclaims (§2.4; its league-less
characters' items the text already names, `price_cmd.rs:389`); the
plan's `changed:` is the whole file's delta and "left: nothing — every covered entry is fresh"
prints beside skipped folders (§2.3); a receipt's `in_facts` is wider
than status's "names nothing in the facts" (§2.4); the ledger text
prints "0 done" never read (§2.5); the never-fetched gloss counts
"the fetched ones" where the code counts every location (§2.1).

- **Settles:** #105's third item (the summary says what it covers or
  covers what it did).
- **Touches:** #7 and #48 are not this root (a document's shape, not
  its extent); #104 (the MCP's first turn) is touched by every gloss
  this sweep adds.
- **Gaps no issue names:** all of the above but #105's.

### What fits no root

- **#7, #48, #50, #18, #17**: the shape of a document (a failure's
  `kind`, a receipt's counts, a bare array, a missing tool, a
  taxonomy), not what a document says of itself.
- **#35, #40, #41, #38**: the answer's words and views, their own
  triggers.
- **#97**: which surface's words a shared refusal speaks; C143's.
- **#104, #95**: size.
- **The quote's `null`**: honest, and the text says why; the league
  seat judged it no fault and I agree.

A reframe that covered #7 and #48 as well would be a warning sign;
these three cover three things each, and the first two are one shape.

## 4. The collapse

### The idea

**Coverage and age are one function of a set of places, and every
answer applies it to its subject and, where its scope is wider, to its
scope, each block named for the set it is over.**

A place is a location with a kind, a type, a league and a fetch time:
what `LocationRow` already is (`acquisition-store/src/corpus.rs:202`).
The function takes a set of places with their observations: each
place's fetch, the listing bases of the set's realm and leagues (a
listing that names nothing has no location row to carry its time, and
a substash's listing time is its parent's fetch, `corpus.rs:35-38`),
and the item count at each place. That is `coverage()`'s signature
already (`corpus.rs:1063-1068`: the header, the realm, the locations,
the item total and the counts by coordinate), and what it computes
over the header (`:1069-1136`) is the function's result: how many
places, how many fetched, the never fetched by type and by league, the
oldest and newest fetch with their locations, when the lists were seen,
aged at `now` by one method (`Coverage::aged`, `:446-455`). Today it
runs once, over the whole realm. The collapse is to run the same
function over the answer's subject:

- a search answer's subject is the matches' places: `total` gains the
  matches' oldest and newest fetch and lists seen, the same shape, one
  fold over the loop at `answer.rs:1751`; the scope's block stays for
  what was and was not searched;
- a counts or cross view's subject is the same set, so the same block
  serves it (no per-bucket ages: a bucket's route gives its rows);
- a shop render's subject is the posted entries' places: their oldest
  and newest fetch and the listing their ranks come from (`list_seen`),
  whatever the policy says; the policy's `stale` and `uncovered` stay
  as what the window adds;
- a price status's subject is the league's items: the block in the
  decision view, where today the JSON alone holds the listing's age;
- a tabs listing's subject is the league's tabs: the block adds the
  list's age and the extremes to the counts it has;
- a store status's subject is the file: the block answers "how old is
  my data" with no new verb, one line per realm or whole;
- a plan's subject is the covered places: its "facts" line is already
  the lists seen and its actions carry their ages, so the plan changes
  least; its wrapper gains the league's block beside the covered set
  (#22) and the leagues the store holds beside the policy's (#157).

What the function makes right is the ages and coverage of a set of
places, and only that. The credential ages (`accounts`' last login),
the events' window, a lifetime's counts and the policy's omissions are
Root 3's sweep and Root 2's scope block; the collapse claims neither.

Each block is named for its set, in the text and in the JSON key:
`scope` is what could have been seen, `total` or `posted` or `league`
what was answered about. The text's rule follows C53: the decision view
prints the subject's ages; the scope's stay in the JSON, where a
program keeps them. That is a removal from the text: the scope line's
ages half, added at the words step for #11 (`0d8db46c`), says the age
of a set the reader did not ask about, and once the subject's stands
beside the total it is provenance that does not change the next
decision.

### Where it lives

In the store crate, beside `age.rs`: a pure function over place rows,
returning the struct the search already serializes as `scope`'s
coverage, with its `aged`, widened by one field: each listing base the
set reads (the realm's characters, each league's tabs) with its own
age, beside the aggregate `list_seen` the struct holds today
(`corpus.rs:426`, the minimum over them at `:1126`). The header
already carries the bases one by one
(`acquisition-store/src/corpus.rs:172`), and `price status`'s two
listing fields map onto them. The reason is C1 and P5 together: every
consumer that answers links the store (the search, the planner's
pricing and shop, the verbs' tabs, characters and status), the fact it
reads (`fetched_at`, `listed_at`) is the store's (C103: place is the
store's), and the lowest rung that holds the knowledge is the store.
The search's `coverage()` becomes a call over the header's rows; its
`tabs::by_type` moves down or is called from below. The daemon links
the store and never calls it; no edge moves.

This is where the idea meets a ruling. C108 says the store gains "what
search needs and nothing more"; C39 says the store exposes "neutral
snapshots, never half a planner". A coverage function is a neutral
read-side derivation with no policy in it, which is what C39 permits;
but it is not "what search needs" alone, and C108 was ruled about the
read. The owner decides whether a derivation every reader shares is
the store's (§11, question 2). If he declines, the function lives in
the search crate, and the planner cannot call it: the search links the
planner, and C1 forbids the reverse edge (`decisions/search.md:9`,
`tools/arch-check.sh:377`). The shop's and price status's blocks would
then be made in the verbs crate, a wrapper holding the planner's
document and the block beside it, the planner unaware of both. That
works and is worse: the function sits one crate above the fact it
reads, and two answers are assembled above their makers.

### What it retires, in C154's form

- retired `acquisition-search/src/corpus.rs`'s `coverage()` and
  `tabs.rs`'s `by_type` counting; held by the store's one function and
  its unit tests over a fixture of places, with the search's
  `c100_the_worked_example…` reading its result, `tabs.rs:154` calling
  it and `acquisition-search/tests/tabs.rs:117` reading the counts.
- retired `TabRow::aged` and `CharacterRow::aged` as two partial
  methods; held by one aging per row and the named paths of the pin
  below.
- retired nothing of the plan's envelope (C38: the envelope is bound;
  the block sits in the wrapper with `notes` and `quote_note`).

Three changes the first draft of this list called retirements are
not: each swaps a fact for one with a different subject, or ends a
property without re-holding it, and C154 has no line for that. They
are contract changes, each held by a regenerated golden that names the
new fact (C122):

- the scope line's ages half leaves the CLI text (18 goldens, the
  search reader's grep): the property "the scope's ages are in the
  text" ends; the subject's ages under `total` and the scope's in the
  JSON goldens hold what replaces it, not what it held.
- the shop's `oldest_stale_seconds` stays: it is the stale covered
  set's oldest, which its test keeps apart from an older uncovered
  item (`shop.rs:1181`, `crates/acquisition-plan/unit/tests/shop.rs:725`),
  a different subject from the posted set's. The posted block stands
  beside it, and one new case in `c72_coverage_staleness_and_positions…`
  holds the two apart: an uncovered posted item older than every stale
  one makes the block's oldest older than the window's.
- `price status`'s `stash_listing_fetched_at_age` and
  `character_listing_fetched_at_age` stay: two bases over two sets,
  the league's tabs and the realm's characters, which one `list_seen`
  cannot carry (`listing.rs:852`); the block carries each base with
  its age beside `list_seen` (the widening under "Where it lives"),
  and the two fields are read from it rather than made by hand.

### The structural pin

The class "an epoch serialized with no words beside it" (`tabs`'
`listed_at`, `show`'s `first_seen` and `last_seen`, the events
summary's `first`, the ledger rows' `submitted_at` and `updated_at`,
the shop's listing header) is held today by discipline: each `aged`
method names its fields by hand. The verbs crate already walks every
document a verb returns (`documents.rs`,
`c149_every_document_a_verb_returns_is_one_json_object`), but over
empty fixtures (`documents.rs:1240`: a store with no rows), so a rule
over keys there meets no row's timestamp, and a suffix rule would miss
`first_seen`, `last_seen`, `first` and `last`, four of the class's own
members. The rule is a list of paths, each read from a fixture that
holds a row, and the list is the class whole, longer than the first
draft's by the five the narrow review found: `tabs`' `listed_at`;
`show`'s `first_seen` and `last_seen`; the search's and `show`'s
`basis.snapshot.at` (`acquisition-search/src/corpus.rs:156`); the
events' `first` and `last`; the queue list's rows' `submitted_at` and
`updated_at` (`verbs/src/jobs.rs:262`), where one job's status already
ages them (`:575`); the policy's `created_at`, bare beside an aged
`updated_at` (`frontend.rs:509-513`, `annotations.rs:189`); the price
list's `basis` and `manual` epochs outside `status` and `show`, which
alone word them (`listing.rs:163-167`); the shop's listing header and
its `rendered_at` (`shop.rs:520`); the plan's `basis.snapshot_taken_at`,
which `PlanAges` does not map (`crates/acquisition-plan/src/lib.rs:1249`,
`refresh.rs:57-70`), its wrapper holding the plan's other ages outside
the envelope (`documents.rs:483`). The walk that holds the shape today
enumerates ten documents (`documents.rs:1240`); the rule extends the
enumeration to every document the list names. That is structure over
discipline at the cost of one test and one list. C53's own condition
is narrower than the rule: "an age the text prints stands in words
beside its epoch in the JSON, made at the answer's own time"
(`decisions/frontends.md:11`), so an epoch the text never prints as an
age owes no words under its letter. "Every epoch an answer carries has
its words" is one step past that letter in its direction, and the
owner reads it as C53's mechanism or as a line of its own (§11,
question 3).

### Where it strains

- **The plan.** Its subject is the covered set and its clock is
  `generated_at`, not `now`; the block would be aged "before
  generated_at" as its listing ages are (`age::before`). The envelope
  cannot change (C38); the block sits in the wrapper. And #22's count,
  the league's tabs outside the policy, the planner's snapshot already
  holds: `RefreshSnapshot.tabs` is the league's live tabs whole
  (`snapshot.rs:140`), and compile keeps the covered ones
  (`lib.rs:2243`); the wrapper counts the rest from the same snapshot
  with the planner's own coverage test, no second read. An hour or
  two, and the plan's text gains a line it did not have.
- **Removed items and the empty set.** A search over `all` or `removed`
  (`search_cmd.rs:92`) holds items whose stored body predates later
  fetches of the location they left, and a row's place age is that
  location's current fetch (`corpus.rs:579`), not the item's last
  sighting. A block folded over the matches' places would report the
  places' fetches for a subject of sightings: this map's central fault
  again, for historical subjects. The block must say which it reports
  (the places' observations, each removed row carrying when it left,
  as today), and a match set with no places has no ages: the block
  says so rather than carrying none.
- **The shop's shape.** A block over posted entries has `never_fetched`
  at zero by construction (a posted item was fetched) and `by_type` of
  little use; one struct serving a set where half its fields cannot
  occur. The JSON would carry zeros a reader might take for a claim.
  The honest form is the fields as `Option`s, absent where the set
  cannot have them, which is one more rule for the describe gloss.
- **Two blocks with ages in one answer.** A reader who today takes the
  scope's age for the matches' will tomorrow see two ages. The words
  must name the sets ("scope … searched", "total 187 matches, fetched
  53d to 57d ago"), and the scope's ages leaving the text is what keeps
  the decision view one age. If the owner keeps the scope's ages in the
  text, the gain shrinks to a label.
- **The MCP's size.** A block of about 200 bytes on every search
  answer and on four more tools; the describe gains one entry and loses
  none. #104's measure moves by that much. Small, and named.
- **Golden churn.** Every search text golden changes once (the total
  line), every price status and shop golden gains a line, and the
  text's age rule already masks the words (`harness/mod.rs:1070`),
  so a regeneration is read as a diff of one line per file. The words
  step did 18 files in one commit; this is of that size, three times.
- **The gloss.** One `coverage` entry in `describe` replaces the
  per-surface sentences the MCP seat read (`scope`,
  `scope.never_fetched_by_type`, the shop's `position_before_listing`,
  the plan's none). The entry must say the one thing every block
  shares and name what a subject's block lacks. That sentence is the
  hardest word in the step.

## 5. The rulings

Neither "C118 as written" nor "a new ruling alone" answers it. The
property is C118's, widened from a row to the answer's subject; the
extent rule is C146's, read precisely; the mechanism is one line that
could stand in either file or in C118's doc comment. Three drafts, in
the registry's form, each under 800 bytes, no id in any sentence a
user would meet.

**C118, amended (`decisions/search.md`):**

> - **C118 — An answer says how old its subject and each row are, and what kind of place it could not see.** A row carries its location's fetch age beside `place` (C53, output levels). The subject, the set asked about, has its places' oldest and newest fetch with their locations and when its lists were seen, named for the set. A wider scope's ages stand apart in the JSON. The scope line says the never fetched by tab type, the largest named and the rest counted, and points at the list that exists. Why a location was never fetched, by policy or not yet, is left out until a seat asks. *Why:* a seat read the scope's age as the matches', and a page hid positions' age. *Pinned:* `acquisition-search/tests/seat_faults.rs`, `acquisition-search/tests/explain.rs`; the subject's Unbuilt, #155, #156.

**C146, read precisely (no change recommended; the alternative is one clause):**

C146 says "a number a frontend prints names what it counted, where it
came from and what the reader can do next". An age is a number and an
extent is where it came from; the events summary's "in the last 24 h"
over ten rows and `sends 17` over one lifetime already fail it. If the
owner wants the extent said in the ruling's own words, the clause is:
"a number taken over a lifetime, a window, a limit, a file or a realm
names that extent". I recommend no change: the sweep is held by
goldens, and a ruling that lists extents will be read as closed.

**The mechanism, one line (`decisions/frontends.md`, beside C146 and C53; or C118's doc comment):**

> - **C162 — Coverage and age are one derivation over a set of places, held once, and every answer applies it to its subject and, where wider, to its scope.** The derivation takes a set of places with their observations (each location's kind, type, league and fetch, the set's listing bases, the items at each) and returns their count, the fetched and never fetched by type and league, the oldest and newest fetch with their locations and when their lists were seen, aged at the answer's time by one method; a surface derives none of these for itself. *Why:* four surfaces derived coverage four ways and each reported the age of what its struct held. *Pinned:* nothing yet.

Where the derivation lives is the owner's (§11, question 2): the
store crate under C108 and C39 read as permitting a neutral
derivation, or the search crate under them read as written. The
ruling's words above name neither crate on purpose.

## 6. The two seats

### The agentic user

What it needs that the system does not give it, each from a seat call:

- **The age of what it was told.** "How old are these facts?" answered
  from the scope for 187 matches (MCP-2 calls 32, 37, its fault 1); a
  page's positions 50 days old under `policy: not_set` (MCP-2 call 52,
  its fault 5); a price status with no age in text (my run); "no
  single verb answers how old and from where … I found it by luck"
  (CLI-2, T3). It would rather not page ten rows to find a floor, nor
  convert an epoch (the MCP-2 debrief retracted a date it worked out).
- **What else is there.** The leagues the store holds beside Standard
  (CLI-2 calls 54, 56; CLI-1 call 43); which league is new (MCP-2 calls
  7 to 9; CLI-2 call 8); whether the policy covers the store (#22).
- **What a number is over.** `never_fetched_by_type` with fetched
  counts in it (MCP-2 call 25); `policies_known` read as sync policies
  (MCP-1 call 5); the events summary's "in the last 24 h" over ten rows.
  It wants the set in the sentence, every time, in that surface's own
  words.
- **A time it can repeat.** The plan's quote says why it has none; the
  daemon says not when it exits (MCP-2 call 63). Not this map's: the
  retrospective's §5 and Root B there.

What it would rather the tool not make it work out: ages from epochs
(done, but for `first_seen`); the arithmetic that reconciles counts
across tools (every seat did it, and it is what "right?" cost them);
which of two ages is the one to quote.

### The agentic developer

What each freshness or coverage field costs it today, measured in
sites:

| cost | where | source |
| --- | --- | --- |
| an `aged` site per struct, by hand | `Coverage::aged`, `Place::aged`, `Shown::aged`, `TabRow::aged`, `CharacterRow::aged`, the verbs' five, the store's `jobs.rs` one | §2's inventory; `grep -rn "fn aged"` over `crates/*/src` |
| a describe gloss per field | `describe.rs:218-282` (the answer's keys), the MCP's own sections (`main.rs:676-737`), `pricing.rs:1108-1159` | the readers' quotes |
| a golden per wording, with the `age` normaliser | 18 goldens hold "the oldest fetch", 19 "each term counted alone", 6 `fetched_at_age`, 0 `listed_at_age` | the search reader's grep |
| two surfaces drifting | `price status` text against its JSON; `TabRow` against `CharacterRow`; the plan's two clocks under one key; the shop's gloss in the present tense | §2 |

What shape would give a new answer-bearing surface its honesty for
free: a value type with one `aged`, one gloss and one golden shape,
so a surface that holds places gets its block by one call and cannot
leave the words out; and the named-paths pin in `documents.rs`, so a
bare epoch on its list fails the gate. Under that shape #155 is a fold
and a line, #156 is a call, and the next surface (the GUI's cards, #98)
inherits both.

### Where they want different things, and where it is one need

They differ. The user wants every surface to say, in its own words,
what set a number is over; the developer wants one wording for one
block, because two wordings are two goldens and a drift. C146 already
sides with the developer ("the same words stand on every surface the
count stands"), and the user pays for it in a sentence that reads the
same under a price status as under a search. I would not average it:
the block's words are one, and the sentence that introduces the block
is the surface's ("the page's items", "the matches", "the league's
tabs").

They are one need in one place: **a number that says what it is of**.
The user wants it in the sentence so it can be quoted onward; the
developer wants it in the type so it cannot be omitted. The block is
both: a struct whose name is its set, serialized under a key that
names the set, glossed once.

## 7. What not to do

Reframes considered and rejected.

- **An age per bucket in a counts or cross view.** Ten buckets with
  two ages each is the audit view standing where the decision view
  belongs (C53). The subject's block serves the table; a bucket's
  route serves its rows.
- **Narrowing the scope block to the query's league.** The scope's job
  is to say what could have been seen, including the never fetched in
  other leagues; narrowing it hides them. `list_seen` is already
  narrowed to the fixed league (`answer.rs:2088-2093`), the one
  exception, and I would not widen the exception.
- **A single freshness number or verdict ("stale", a score).** It is
  judgment (C106) and it hides the set. The block's two ages and its
  lists seen are what a reader needs to decide.
- **A `how-old` verb of its own.** The CLI-2 seat asked for one; the
  store status's block answers it in a shape the reader already knows,
  and a verb is a second maker (the review checklist's shape).
- **A persisted or cached coverage.** C98 and C48; the function is a
  pass over places the answer already walks.
- **Renaming `never_fetched_by_type` or its fields.** C146 bars it; the
  text's "2489 of 2582" and the gloss do the job. An additive `never`
  field per row is cheap and says it as a number; a next-step row, low.
- **Putting the league's block in the plan's envelope.** C38; the
  wrapper is the place, with `notes`.
- **Fixing #7 and #48 under this map.** A failure's `kind` and a
  receipt's counts are a document's shape, their own steps.
- **Computing a term's counts over the matches.** C93 and C115 chose
  the scope as the denominator and the words now say so; the re-sit
  measured no call spent. Leave it.
- **A per-surface sentence for every block.** One gloss, one golden
  shape; the surface's own sentence introduces it (§6).

## 8. The triage frame, tested

The frame: each gap is an answer reporting facts about its scope where
the reader asked about its subject.

Where it holds: Root 1 where an age is given. The matches' age
(#155), the posted items' age under a policy (#156), a tabs head: in
each the answer has a wider set's facts in hand and reports those, and
the frame names the direction exactly. Where Root 1 gives no age at
all (`price status`'s text, `store status`, a counts view, a render
with no policy) the frame does not reach: nothing of any scope is
reported, an omission the block fills and the frame does not name.

Where it is wrong: in two ways. First, Root 2 is its mirror image. For
#157, #22 and #19 the answer reports facts about its *subject* (the
covered tabs, the league read, the row) where the reader asked about a
wider *scope* (the store's leagues, the league's tabs); the frame's
direction is reversed, and a design that only moved facts from scope
to subject would make those worse. Second, Root 3 uses "scope" in a
sense the frame does not: a lifetime, a window, a limit's rows, a
realm. The events summary reports the limit's rows where the reader
asked about the window, which is a *narrower* extent reported as a
wider one; `sends 17` reports a lifetime where the reader asked about
the daemon. These are labelling gaps in both directions.

And it leaves out what no scope explains: a failure's shape (#7), a
receipt's silence (#48), a bare epoch (`first_seen`), a gloss in the
present tense (`Stash57`).

My roots are better by one distinction: they split the direction
(the subject's facts missing; the scope's facts missing; the extent
unnamed), and each direction has a different fix shape (the block on
the subject; the block on the scope of league-taking surfaces; a
wording sweep under a ruling that exists). The frame is right about
the centre and would mislead a design at the edges.

## 9. The owner's store: a checklist for the league run

Questions he can answer from his own `acq` output during the 3.30 run,
each naming the root it confirms or refutes. No agent opens the store;
he reads his terminal.

1. After the new league's first sync, `acq search 'league="<new>"
   <any query>'` (`--realm pc` beside it if his store holds more than
   one realm): does the scope line's oldest fetch name a Standard
   location weeks old while every row says hours or minutes? If he
   quoted the scope's age to himself as the league's, Root 1 holds; if
   he read the rows' ages and never looked at the scope's, the text
   change in §4 costs nothing he uses.
2. `acq search 'league="<new>"' --count tab`: did he want an age
   beside the counts and go back to the rows for it? Root 1, the counts
   view.
3. `acq price status --league <new>`: can he say from the text how old
   its facts are without `--expand`? Did he assume current? Root 1 on
   pricing; the decision view's missing line.
4. `acq shop render --league <new>` with his policy covering it: does
   the stale line's count and "oldest … ago" match what he expects?
   Then `--league Standard`, if Standard is outside his policy and
   holds posted items (with none the render prints no freshness at all,
   `shop_cmd.rs:138`, and the question has no answer): does the render
   say anything of the positions' age? If not, and he pasted, #156 is
   live for him.
5. `acq store status`: could he answer "how old is my data" from it?
   Where did he go instead? Root 1 at the store.
6. The league morning: did `acq leagues` tell him which league was
   new, or did he infer it; did `acq policy show` tell him which of his
   store's leagues the policy does not cover? Root 2, #157.
7. Any `acq price status` or `acq shop render` without `--league` after
   the new league existed: did Standard's answer stand in for the
   league's once? Root 2, #19.
8. After an apply: was the parenthetical "(what this apply revealed, …
   or what went stale since)" ever the reason; did "left: nothing —
   every covered entry is fresh" appear beside skipped folders or
   substashes? Root 3, the plan's words.
9. `acq daemon status` and `acq jobs` after a restart: did `sends N` or
   `N jobs` read as the day's total? Root 3, the lifetime.
10. `acq tabs --league <new>` on the first morning: did he need to know
    when the tab list was last read and look for it in `tabs`? Root 1,
    the tabs head.
11. Did any age in words mislead by its staleness, an MCP answer or a
    saved `--json` quoted later? The ages are made at the answer's time
    (`age.rs:22`); a "no" keeps the vocabulary as it is.
12. Two facts apart, no cause between them: did a plan's `quote` say
    "ready" and the apply then wait; and what `observed_seconds_ago`
    did that plan's `--expand` or `--json` carry? A wait after "ready"
    has other causes (work queued between, a probe, a token refresh, a
    retry: `quote.rs:147`), so the pair records what happened and
    proves nothing. Root 3's one quote row is that the decision view
    drops the age (`plan_cmd.rs:1792`).

## 10. Next steps, ranked, each with its cost, what it retires and whether it needs a ruling

| step | what | cost | retires (C154's form) | ruling |
| --- | --- | --- | --- | --- |
| S1. The subject's block on the search answer | the matches' coverage under `total` on rows, counts and cross, one fold in the loop at `answer.rs:1751`; the text's total line gains the ages and the scope line loses its ages half; the gloss; the goldens regenerated and read | a day | retired nothing; the scope line's ages half leaves the text, a contract change held by the regenerated goldens and the JSON's scope. #155 closes | yes: C118 amended (§5) |
| S2. The one function in the store | `coverage` over place rows beside `age.rs`, with `aged`; the search's `coverage()` and `tabs::by_type` on it; its unit tests over a fixture of places | a day | retired `corpus.rs`'s `coverage()` and `tabs.rs`'s counting; held by the store's function and its tests, read by `c100_…`, `tabs.rs:154` and `tests/tabs.rs:117` | yes: where it lives, under C108 and C39 (§11, question 2); the mechanism line (§5) |
| S3. The shop's and price status's blocks | the posted entries' block whatever the policy; the league's block in `price status`'s decision view; the shop's listing header aged | half a day each | retired nothing: the shop's stale-set age and the two listing-age fields stay as their own facts, the block beside them, one new `c72_…` case holding the two apart. #156 closes | no, after S2; the words are the session's, listed |
| S4. The named-paths pin | one rule in `documents.rs` over a list of paths read from fixtures that hold a row: `tabs`' `listed_at`, `show`'s `first_seen`/`last_seen`, the events' `first` and `last`, the ledger rows, the shop's header, each aged; a plan's in its wrapper | half a day | retired `TabRow::aged` and `CharacterRow::aged` as two partial methods; held by the rule and one aging per row | one step past C53's letter: his reading (§11, question 3) |
| S5. The scope of league-taking surfaces | the store's leagues beside `price status`, `shop render` and `tabs`; the stored league list read by `policy show` and a `leagues --stored` or the same verb offline, each league beside whether the policy covers it; the plan wrapper's count of the league's tabs outside the policy, from the snapshot it holds; C126's match per realm | a day | retired nothing; #157 and #22 close, #19's comment answered | the offline read of the league list is agent-facing design, the session's under the owner's delegation; the README's new-league line is his to re-read |
| S6. The extents sweep | the events summary (#105's third item), `sends` and `policies_known` and `total` said per lifetime, `leagues` as a union or retired to the list's size, "last login" as the last credential write or narrowed, `price status`'s "skipped", `store refused`'s text saying whether more exist, `in_facts`'s width, "0 done", `protocol.rs:595`'s default, the gloss of `never_fetched_by_type.items` | a day | retired nothing | no: C146 read precisely |
| S7. The plan's clocks | one name per clock: the note's `character_listing_fetched_at_age` renamed or measured as the top-level one; the decision quote keeps `observed_seconds_ago` | an hour | nothing | no, but a JSON key moves: C146 bars a rename, so the measure changes, not the name; listed for the owner |
| S8. C77 pinned | a test that a plan's freshness is judged at compile, the clock behind the facts line | an hour | nothing | no |
| S9. #12 closed on the sentence | the terms block says its set; four seats did not stop | minutes | nothing | the owner's |
| S10. `never` per type row | an additive `never: tabs − fetched` on `never_fetched_by_type[]` | an hour | nothing | no |

Not proposed: anything in the daemon; a change to C93's denominator;
a change to the describe index's shape; the GUI's cards.

## 11. For the owner

The decisions only he can make, each as a question with its options.

1. **C118's amendment.** Should C118 be widened from a row to the
   answer's subject, as drafted in §5, so a search answer, a shop page,
   a price status, a tabs listing and a store status each say how old
   what they answered about is, with a wider scope's ages apart and
   named? (a) Yes, as drafted, recommended. (b) Yes for the search alone
   (#155 and nothing else; the rest as surface design under C53). (c)
   Decline; #155 and #156 as filed, two fixes.
2. **Where the one derivation lives.** C108 says the store gains what
   search needs and nothing more; C39 says the store exposes neutral
   snapshots. (a) The store crate, beside the age vocabulary, read as a
   neutral derivation every reader shares, recommended: it sits at the
   fact it reads and every consumer links it. (b) The search crate, the
   shop's and price status's blocks made in a verbs-crate wrapper, the
   planner unaware of the search (C1 bars it the edge). (c) No shared
   function; each surface keeps its own and the blocks are held equal
   by a test.
3. **The mechanism's home.** (a) The one-line ruling of §5 in
   `decisions/frontends.md` beside C146 and C53, recommended. (b) The
   mechanism as C118's doc comment, no registry line. (c) Cross-cutting
   in `AGENTS.md`, at the budget's cost. And the pin of §4, one step
   past C53's letter: read as C53's mechanism, needing no line, or
   named in the line.
4. **The scope line's ages leaving the text.** The words step added
   them for #11 and he read that change. (a) Remove from the CLI text
   once the subject's ages stand under the total, kept in the JSON,
   recommended: one age in the decision view. (b) Keep both in the
   text, each named.
5. **C146.** (a) No change; the extent is "where it came from", read
   precisely, and the sweep is held by goldens, recommended. (b) Add the
   clause of §5.
6. **The league list offline.** #157 asks for a read of the stored
   list on both surfaces. (a) The same verb and tool, offline when
   asked (`--stored`, or the stored list when no daemon runs and a flag
   says so), each league beside whether the policy covers it,
   recommended. (b) A line in `policy show` and `store status` alone.
   (c) Leave as filed.
7. **#12.** (a) Close on the terms block's sentence and four seats'
   evidence, recommended. (b) Keep its trigger.
8. **Order.** S1 and S2 first (the search, where the evidence is and
   the shape exists), then S4 (the pin), then S3 and S5 between
   sittings; S6 to S10 as a words round with the next seat checking
   them. Or S4 first, since it is cheapest and structural.

## Taken from the review

The cold review at `a064b508` (`answer-facts-map-a064b508.md`) found
thirteen; each is answered above by one of five rules, not by itself.

- **A marked cell names what the surface already prints.** The events
  summary's "(limit reached)" and `price status`'s league-less line
  (finding 8); `policy show` shows the row whole, no league (4); the
  tabs pointer gives nothing for a fixed league without a never-fetched
  tab (13); "no seat stumbles" narrowed to the terms block, the
  never-fetched JSON read twice (4); Root 1 and §8 say where an answer
  gives no age at all rather than a wider set's (4).
- **A command in §9 is one the reference prints.** `acq search` takes
  the league as a term, not a flag (5); item 4 needs posted items and
  item 12 records two facts and proves no cause (11).
- **The collapse's contract names its inputs, its retirements re-hold
  their property, its strains are all named.** The function takes
  places with their observations and counts, and claims only the ages
  and coverage of a set (1); the shop's stale-set age and the two
  listing bases stay as their own facts, and the scope line's text is a
  contract change, three lines moved out of C154's form (3); the pin is
  a list of paths over populated fixtures, one step past C53's letter
  (2); the plan's snapshot already holds the uncovered tabs (7);
  removed items and the empty set are a strain (10); the fallback home
  is a verbs-crate wrapper, since C1 bars the planner the search (12).
- **The C118 draft is in the registry's form.** Under 800 bytes, no
  measurement in its Why, the unbuilt half named against #155 and
  #156 (6).
- **The inventory names what it cut.** `store refused` has its row, the
  one list whose text says nothing of more; the lists that do are
  named once in §2's head, and so is what the map does not cover (9).
- **#12** keeps its own fire line and its close is the owner's, not the
  root's (4).

The narrow review at `d3cbb0c2` (`answer-facts-map-fixes-d3cbb0c2.md`)
checked the five claims the round had inferred and found four things:

- **The pin's list is the class whole.** Five bare epochs it missed are
  named (the search basis's snapshot time, the policy's `created_at`,
  the price list's bases, the shop's `rendered_at`, the plan's snapshot
  time), the queue list told from one job's status, the fixture walk
  extended, C53's own condition quoted, and §6's old phrase for the
  pin replaced (finding 1).
- **The result carries each listing base.** The coverage struct holds
  one `list_seen`; the block promised both bases, so the function's
  result is widened by one field and the two price status fields read
  from it (2).
- **The C118 draft is in declaratives.** One idea per sentence, no
  semicolon, no fragment, under the budget (3).
- **Two citations** name the caller and the text's age rule (4).
