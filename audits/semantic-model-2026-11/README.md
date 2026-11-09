# The semantic-model audit, 2026-11

A read of the tree at `5f3ad0d` asking what the code holds as state,
which transitions commute, what each effect depends on, and where the
registry's claims and the code part: nondeterminism, invariants held by
nothing structural, one fact in two places, work no surface prints,
and the refactors the read suggests, split into those a type or a test
table proves and those that rest on a judgment. Six area reads under
one brief (`BRIEF.md`), each a fresh agent session over its crates with
the registry and `REVIEW-CHECKLIST.md` beside them, their reports under
`reports/`; the synthesis below is the orchestrating session's, which
re-read the twelve highest-impact citations itself (marked ✔).
No tree change was made.


## 1. State: what changes what a user sees

Grouped by home. "Truth" means the one authoritative copy; "mirror" means a copy something else must keep in step.

| Home | Value | Writer | Truth or mirror |
| --- | --- | --- | --- |
| **GGG headers (remote)** | rate-limit state per policy | GGG | truth (invariant 2) |
| **Daemon memory** | `Limiter.policies[key].rules.state.hits` + `history` + `retry_hold_until`; `endpoints[route]` ∈ Unknown/Policy/Policyless/Degraded; `retry_holds[route]`; `violations` | `observe_impl`, `assume_counted`, probes | mirror of headers (prediction), per lifetime; `violations` read by tests only |
| | `SendGate` permits (≤2 in flight, one per policy, HEAD exclusive) | gate | truth |
| | `Shared.jobs: HashMap<JobId, Entry>` (state, outcome, deferred, cancel_requested, times, retries, priority) | submit/process/finish/cancel/restore/evict | truth for live trees (C6) |
| | `Shared.evicted` counts; `active_jobs`; `Exit{begun,deadline}`; `Connections` word; `last_activity`; `restore` watch; `closed_with[route]`; errors ring | daemon | truth, per lifetime; `evicted` never reconciled with the table |
| | `AuthSession{access_token, expires, refresh_token, username, uuid, generations, refresh_flight}` | login, refresh, 401 rejection, restore | truth for this lifetime |
| | `Rails.State{sends, tripped, dormant_trip, blocked, unreachable, ceiling_tripped, stopping, refresh_failed[username]}` | `Rails::record`, reset, mark/clear | mixed: `tripped/blocked/refresh_failed` mirrored to disk; the rest per lifetime |
| | `Life{state,cause}` machine | driver | truth; never on the wire |
| | Mock `PolicyBook.live` windows | mock | truth for the mock only; reset at every restart (so the restore's "probe reads the previous lifetime's hits" is vacuous on the mock) |
| **`<world>/<provider>/daemon.db`** | `jobs` rows | `Daemon::persist` only | mirror while in memory; truth once evicted or after restart; read directly by frontends as `Persisted` when no daemon answers (C45) |
| **`<world>/<provider>/rails.json`** | tripped, blocked{at,cause}, refresh_failed_by_account | daemon rails; CLI offline `reset-tripwire` | truth across lifetimes; one file field `tripped` lands in two memory fields chosen by `ACQ_TRIPWIRE` ✔ |
| **OS keyring** | `{refresh_token, access_token?, access_expires_at?}` per account | daemon only | truth for refresh tokens (invariant 5); mirror of the session at save time |
| **`accounts.json`** | username, discriminator, uuid?, last_login, persisted, keyring_names | daemon under `CredentialsHeld` (type-checked), atomic+fsync | mirror of the keyring and of `/profile` |
| **`<provider>/<username>.db` (facts)** | `responses`, `tabs`, `characters`, `items`, `item_events`, `refused`, `leagues`, `account`; `user_version` 9 | `Store::record` (daemon); **also the CLI's `store import` and `store rebuild`** ✔ | truth for facts; derived item columns are a mirror of `items.json` |
| | `Revision` = (MAX(responses.id), its fetched_at, user_version) | derived per read | the search basis (C98) |
| **`<provider>/<uuid>.annotations.db`** | prices, sync policy, provenance; file revision = SUM(row revisions) | `put/delete` under IMMEDIATE-tx CAS | truth, irreplaceable (C35) |
| **`<provider>/user.db`** | saved searches (own revision) | same door | truth (C167) |
| **Journal `sends.jsonl`** | one line per send attempt after its body completes | `record_completed` | derived record; written after the response |
| **Process memory, frontends** | `Holder` corpus slot (MCP, `acq web`) | `Holder::corpus` | cache revalidated against store+intent revision before each hand-out |
| | web `Web{account: OnceLock, token, host, origin, held}` | first request | `account` pinned at first resolve for the server's life |
| | CLI `SPAWNED`, `Progressor`, dash `App{selected,expanded,scroll}` | CLI | UI state only |
| **Env** | `ACQ_NOW`, `ACQ_IDLE_SHUTDOWN`, `ACQ_SHELL`, `ACQ_PROVIDER`, `ACQ_STORE_DIR`, `ACQ_TRIPWIRE`, wait knobs | user/harness | inputs; several read more than once per process (§4a) |
| **Build** | `CONTRACT_REVISION` (digest of protocol+store sources, manifests, lock), daemon artifact hash, search table versions | build | derived from sources; the handshake's identity (C84) |
| **URL / page** | `acq web` query string, hidden plan envelope, `<meta basis>` | handlers | the page's identity (C16); DOM-held picker/filter state is transient and outside it |

## 2. Transitions

### The machines, as built

- **Job**: `∅ → waiting → running → {done, failed, cancelled}`, with three loops back to `waiting` (429 requeue while `retries < 2`, halt-park, exit-park) and a `deferred` sub-state for a parent whose children are open. Persisted words are five (`waiting|running|done|failed|cancelled`); `requeued`/`faulted` are paths, not states. Restore maps `running → waiting | failed(interrupted) | cancelled | held` by a fixed branch order (probe → fault marker → lost login → running cases → waiting as-is).
- **Send** (`acquire_send`): loop { halted? wanted? limiter wait? → serial key → permit → re-check all } → bearer read **after** admission → one deadline for head and body → observe headers → rekey gate → classify (block first) → record (journal, rails, ring). HEAD probes take an exclusive permit and establish or degrade a route.
- **Limiter endpoint**: Unknown →(probe Full / declared GET) Policy →(failed probe) Degraded{until} →(expiry) Unknown; same-name shape change clears history; different name is `PolicyMismatch`, state unchanged.
- **Refresh**: single flight per session; fast path if >5 s left; owner POSTs outside `shared`, installs under `shared`, saves to keyring under `credentials`; `RefreshOwnerGuard` publishes on drop.
- **Lifecycle (`Life`)**: `Serving →Deadline(Idle)→ Judge →Busy→ Arm | →Idle→ Begin → Draining →Quiet→ Done | →Deadline→ Cut`; stop and signal enter Draining directly.
- **Door (client)**: `Reaching → AwaitingHello → Done`, or `Nothing → Starting{Spawning → Consulting → Pausing → Reaching | Polling → Looking → AwaitingHello}`; pinned as a table.
- **Ingest** (`record`, one IMMEDIATE tx): listing upserts rows it owns, stamps `listed_response`, retires unstamped rows and their items; a fetch of a retired location is withheld (response row only); a live fetch lifts items, upserts the row, removes items the response did not stamp.
- **Plan**: compile (policy revision + snapshot response id) → optional quote → apply: spend gate (fresh policy read) → `unspent` by response id and open jobs → connect → re-read → submit one `apply` job with exactly the unspent actions.
- **Search**: `text → parse → check → bind(Group::bind) → Query` is pure and store-free; `Corpus::load_of` is the one read (facts tx, then price join, then `still_at`, up to 3 re-reads); `answer(corpus, request)` is pure; routes and continuations carry the basis.

### Commutes / idempotent / order-dependent

| Class | Pairs (verified in code) |
| --- | --- |
| **Commute** | observe(policy A) × observe(policy B); cancel(a) × cancel(b) unrelated; finish(child) × finish(sibling) (one tally under the lock); `reset_tripwire` × `mark_refresh_failed` (disjoint fields, one lock); stash fetch(t1) × stash fetch(t2); fetch × characters listing; `put` on different annotation keys; price set × saved set (two files); two `Status`/`List` reads. |
| **Idempotent** | `rejected(token)`; `mark_refresh_failed`; `declare_policyless`; `Exit::arm`; `set_priority(id,p)`; `maybe_finish_parent`; re-ingesting an identical body (no events; but `responses.id`, `seen_response` and the `Revision` move, so search's basis moves on a no-op ingest); `ensure_wal`; `Index::save`; re-apply after children landed → `Spent`; `canonical(parse(canonical(q)))`; `Holder::corpus` with no write; web saved write via 303 redirect; offline `reset-tripwire`. |
| **Order-dependent** | **listing then fetch vs fetch then listing** for a retired tab: listing-first withholds, fetch-first lands items the listing then removes. **cancel(parent) then submit_child** (refused) vs the reverse (swept). **cancel(child) before/after finish(child)** takes `shared`: Cancelled vs `already done`. **reject_access_token vs install_tokens**: reject-then-install stands; install-then-reject(new) empties the session again. **set_policy then apply** → `PolicyMoved`; apply then set_policy → runs. **price create, clear, create**: revisions continue past the tombstone; undo after clear is a create at revision 0. **install_retry_hold then observe_impl establishing** the route: the hold lands in `retry_holds` only, so status shows no `retry_after_secs` while the route is held; the reverse sets both. **restore then submit** is forced for the queue but not for sessions: a job submitted during the credentials restore is admitted and parked. **evict then List** lists the tree once from the table; List then evict would lose it (today's order is the safe one). Rows: **sort then cut**, stable, ties fall to store order. `daemon stop` reads `AuthStatus` then stops: a login landing between is omitted from `sessions_ended`. Web `served()`: resolve-then-login vs login-then-resolve pins different accounts for the server's life. |

## 3. Dependencies: what must hold before each effect

| Effect | Precondition | Checked where | Gap |
| --- | --- | --- | --- |
| **HTTP send to GGG** | not halted, not exiting, wanted, limiter wait 0, permit held, bearer present and origin is the API origin | `acquire_send`/`acquire_head`, same task; origin before any wait | A halt landing after admission from the *other* in-flight send goes out (cap 2); documented, unpinned as a two-send scenario. The exit check runs outside the lock that admits the send (by design, C133). |
| **Bearer attached** | token read after admission | same task | none |
| **Journal line, rails count** | a `record_completed` call | under the permit, after the body | A never-sent request (`Secret::authorize` failing on a token that is not a valid header value) is journaled, counted toward the ceiling and paced as lost. |
| **Job row write (`persist`)** | caller holds `shared` | **a doc comment only** (`queue.rs:205`) | no type enforces it; every caller complies today |
| **`Submitted{id}` reply** | exit not begun | `refuse_while_stopping` in the server; `submit_marked` checks only `queue_failed` | a stop between the two admits and persists a job nothing picks this lifetime; the client is told `Submitted` ✔(structure) |
| **Intent write** | value typed, revision matches | CAS inside the IMMEDIATE tx | none in-file; planner's read and `put` are two txs, which the CAS turns into `Conflict` |
| **Plan spend** | policy unmoved, action unspent, daemon admits | frontend (`refresh.rs:543-568`) vs daemon admission | by design (C44/C111); a policy edit between `check_spendable` and `submit` spends a revoked plan; two applies racing both send ("Not doing") |
| **`accounts.json` publish** | `CredentialsHeld` | type-checked | none |
| **`rails.json` write** | — | fixed `.partial` name, no dir fsync | daemon and offline reset collide on the partial name (the world lock excludes it in practice) |
| **Keyring rewrite on 401** | generations unchanged, keyring `ok` | under `shared`, save under `credentials` | skipped when the keyring word is `saving`: the rejected bearer stays on disk for the next lifetime |
| **Dead-grant mark** | rejected grant and flight current | `shared` taken then released before `mark_refresh_failed` | a re-login landing in the gap marks the fresh session dead (microseconds; `complete_login` holds `credentials`, not `shared`) |
| **Spawn verdict** | child exited on the lock → look for the winner | `holder` read after the exit | the winner locks, *then* writes its pid: a loser exiting first reads `holder = None` → `SpawnFailed` while a daemon runs |
| **Frontend reads the ledger** | observe door said Absent | elsewhere | `Persisted::read` is two autocommit statements: `written_at` and rows from two instants |
| **Idle exit** | no connections, no live jobs, no landing, no pending login, timed, limiter cold | `judge` on a blocking thread; `claim` on the driver | `timed` is stamped at connect/close/submit/restore only, never at job finish: a job that outlives its submitter ends with the hour already spent and the daemon leaves within 5 s |
| **Facts write from a frontend** | account resolved | `store_cmd.rs` | **no daemon coordination**; C34 ("effects through the daemon") is breached structurally by `store import`/`rebuild` and no check names it ✔ |
| **Web route runs** | Host == bound address, Origin own or absent | `guard` before every route | none |
| **`exit 2` usage refusal** | argv is UTF-8 | `std::env::args()` ✔ | a non-UTF-8 argument panics instead of exiting 2 (C47) |

## 4. Findings

### 4a. Nondeterminism in awaits, retries and event order

1. **Event emission after the lock** (daemon). `finish`, `cancel`, `submit_marked` emit after releasing `shared`: two jobs persisting A,B can emit B,A. Safe under C85 (events are hints) and a watcher re-reads; any test asserting event order would be flaky. `cancel` emits children in `HashMap` order.
2. **Block vs the concurrent admitted send**. Send B on the wire when A's response sets the block: two journal lines after one "SENDS STOPPED"; if B is also a block the mark keeps A's cause.
3. **Gate invariants under `debug_assert!` only** (`gate.rs:113,206,256,259`) ✔. If a no-probe route (`/account/leagues`) learns a policy name already active under a probed route, release builds silently double-occupy the policy: exactly the N6 hazard. Reachable only if GGG shares the policy name across those routes; the ground truth decides.
4. **`PolicyMismatch` arm skips `assume_counted`** while the parse-error arm runs it: at 299/300 a renamed policy's response lets the next send go at once.
5. **Two wall faces**. Job rows stamp `choke.wall()`; the facts and C164's age stamp `acquisition_store::now()` (`ACQ_NOW` or system). Equal in production, divergent under a manual test clock: a C158 scenario comparing `submitted_at` with `fetched_at` will lie.
6. **`now()` read several times per command** in the CLI (search text: 3 reads; `tabs` page: 4; `Opened::plan`: 2, so `generated_at_age` can print `1s ago` on a fresh plan). Pinned only because `ACQ_NOW` freezes every test.
7. **`StoreChanges::since` counts neighbours**: an apply's `changed:` line counts every event after its watermark, another frontend's apply included; `events_after` is silently capped at 1,000,000.
8. **Queued-spend is a one-instant read** of the daemon list; an apply admitted by another frontend after it also sends.
9. **MCP writer `select!`** interleaves rmcp lines and local parse-error replies: a `-32700` may overtake an earlier valid reply (visible only to a host counting lines).
10. **`print_result` renders by retention**: the same apply outcome prints a summary while `status(id)` still answers and a JSON dump after a restart.
11. **`Persisted::read`**: `written_at` and rows from two statements (see §3). **`refresh_snapshot`** pairs facts (one tx) with intent read after it; the apply CAS makes the race a refusal.
12. **Web `jobs` page** observes the daemon twice (jobs, then rails): a daemon gone between prints an empty rails block silently. **Web `home`/`tabs`** join the league list and the on-disk mark from two store reads with no common basis.
13. **Search: none reaches output.** Every `HashMap` that becomes a list is sorted on a total key; sorts are stable and cut after; no float arithmetic reaches a sum or a print (`Exact`). One TOCTOU by design: `Holder::is_current` reads the header under `RealmScope::All` while the corpus was loaded under `One`; correct only while revision and account are store-global.

### 4b. Invariants enforced by hope

Invariants 1, 3 and 5 are **held structurally**: one client in the transport crate, the dependency edges in `arch-check`; `Verdict::of` and `classify_response` are total functions with block first; `Secret` has no `Serialize`/`Deref`/`Debug` leak and no token path to disk, journal or params was found. The transport doc itself names three properties held "by review": single caller of `Production::new`, no reqwest item in its public API, no client timeout. A grep over `pub` items for `reqwest::` would close two.

What is held by a comment, a convention or nothing:

| # | Claim | Held by | Where |
| --- | --- | --- | --- |
| 1 | **C54, tabs**: a fetch owns contents only | **nothing for `name`/`type`**: the tab fetch upsert overwrites `name = excluded.name, type = excluded.type` unconditionally, while the character upsert guards them with `listed_response IS NULL` ✔. Every surface reading the column (tabs list, snapshot, corpus `LocationRow`, the tab order tiebreak) rolls a tab's name back when a fetch lands after a newer listing. Only `metadata` is pinned listing-owned. The checklist row "a late fetch rolled a character's fields back" was fixed for characters alone. | `store/src/lib.rs:2040-2043` vs `1884-1888` |
| 2 | **C38** "new facts produce a new plan" | nowhere: `check_spendable` gates intent only, `unspent` by response id; `c44_fact_drift_never_refuses` pins the opposite. A plan compiled before a listing change sends fetches for retired tab ids (now caught at C164's door, with the request spent on the refusal path). The AGENTS.md sentence is wider than the code; C44/C111 word it as built. | `plan/src/lib.rs:1612-1715` |
| 3 | **C6** memory-is-truth residual | `persist` results ignored on requeue, park, deferred, finish and lost-login ✔; each trips the sticky flag, but memory `waiting retries=1` vs disk `running retries=0` replays with the retry uncounted. The doc names completions only. | `daemon/dispatch.rs:271,309,351,477,544` |
| 4 | **C47** structured error on bad input | an unknown job state word in `daemon.db` is `filter_map`ped away with no log ✔; a terminal row whose outcome no longer deserializes answers `no job` while `Status` for the same id answers the row. Non-UTF-8 argv panics in `refuse_usage` ✔. | `daemon.rs:385`, `queue.rs:540`, `cli/main.rs:1139` |
| 5 | "`persist` is called with `shared` held" | a doc sentence; no guard parameter ✔ | `queue.rs:205` |
| 6 | **C34** effects through the daemon | `store import`/`rebuild` write facts from the CLI ✔; the door grep in `arch-check` (`Index::load|Store::open|…`) does not name `Store::record`, `Store::rebuild`, `World::observe_at`, `Lock::acquire`, `RailsState::write`, `Client::observe_at`, `stop_any_at`, `Subscription::observe_at` | `tools/arch-check.sh:412` |
| 7 | **C166** every answer fits its frame | `Dashboard` is unpaged (`job_list()` whole): past the frame bound `acq dash`/web fails `internal` where `List` pages fine | `daemon/server.rs:459` |
| 8 | **C3** "an hour with no connections and no queued work" | read by the code as "an hour after the last connection/submit, once no work remains"; `idle_verdict_a_live_job_holds_the_daemon_past_the_hour` pins the hold, not the hour after | `server.rs:107,250`, `queue.rs:957` |
| 9 | **C49** accounts interpreted only in the session layer | the dead-grant mark is keyed by `username` in rails ✔; a rename leaves a mark under the old name forever and clears under the new | `rails.rs:719`, `refresh.rs:414,565` |
| 10 | **C85** "the rest refuse an unknown field" | true of `Request` (pinned); the whole response plane (`Response`, `JobInfo`, `Outcome`, `SessionStatus`, `RailsStatus`, …) is lenient, partly forced by `flatten`; no test pins either way | `protocol/src/protocol.rs:625`, `job.rs:79` |
| 11 | **C108** "nothing that prunes may take the revision back" | a comment; nothing prunes the facts file today, so vacuous and unheld | `store/src/corpus.rs:89-99` |
| 12 | **C129** block recognition | `BlockShape::of` is substring matching (`"cloudflare"`, `"nginx"`, `"ray id"`, `"error 1015"`) over a lowercased body of unbounded size, narrowed only by the absence of a policy header or `www-authenticate`; a GGG 4xx JSON echoing "Cloudflare" on a header-stripped route sets a **persisted** mark | `rails.rs:273` |
| 13 | "one reader of a group's meaning" (search) | a token grep for `Member`/`BMember`; `Group.selector_tree` is `pub` and wrapped in `Node::Members` at two sites, so a reader matching through that binding passes the grep | `arch-check.sh:435-446`, `search/src/group.rs:702` |
| 14 | C104 canonical text "normalises spelling" | field/`has`/`is`/pseudo names keep authored case; `Rarity=RARE` and `rarity=rare` are two canonical texts of one meaning; the round-trip property generates lower-case only | `search/src/parse.rs:703,1452` |
| 15 | Gate occupancy invariants | `debug_assert!` ×4 (4a.3) | `gate.rs` |
| 16 | C147 "environment read once" | true of the world only; `ACQ_NO_SPAWN`, tty, the wait knobs and `ACQ_PROVIDER` are read per door/arm/handshake | `client/src/client.rs:336,657,776,1154-1165` |
| 17 | C44's "fresh read" of the policy | rests on `Annotations::get` being a per-call SQL query; a cached handle would void the gate silently | `annotations.rs:1193` |
| 18 | C53 "no second rendering" on the MCP | `spoken()` is a hand-written JSON tokenizer re-spelling CLI words inside shared texts; `told_as` holds C149 by `starts_with('{')` | `mcp/src/main.rs:1343-1585` |
| 19 | `Redactor` (diagnose) | a prose scanner: tokens found under `TOKEN_KEYS`/`Bearer` marks; an `id_token` key or a token in a URL path passes | `verbs/src/diagnose.rs:514-542` |
| 20 | Registry "Pinned: nothing yet" that **is** held | C30, C59, C60, C48 (store: private `conn`), invariant-1 edges; C5 by `queue_order`'s key; C7 by `write_log_line`'s doc. Not held: C2, C4, C15, C22, C49 (see 9), C71/C73/C75/C162 (unbuilt), C150 (reqwest's default under `default-features = false`, no test sets a proxy). |
| 21 | README tour "runs every line" (checklist) | `every_tour_line_resolves_under_clap` resolves each line against `--help`; recipes are run by named tests, the tour lines themselves are not executed | `cli/tests/readme_tour.rs:137` |
| 22 | `picker.js` first-empty-row rule; stale `#stat-N-text` label when an input is emptied | reviewer's read only (acknowledged) | `cli/src/web/assets/picker.js:30-41` |

### 4c. One fact stored in two places, drifting

| Fact | Copies | Authoritative | How they disagree |
| --- | --- | --- | --- |
| A tab's name and type | `tabs.name/type` (fetch-owned today) vs `tabs.listed_json` (listing-owned) | the listing (C54) | a fetch after a newer listing rolls the column back; every reader reads the column (4b.1) |
| Account identity | username-named facts file; uuid-named intent file; index entry (`uuid` optional); facts `account` table (upsert by uuid, never deleted); `filename_safe` maps every non-ASCII char to `_` | the uuid (C50) | a rename orphans the old facts file; a reused username leaves two uuids and `account_identity` refuses every snapshot; two names differing only in non-ASCII share a file |
| Keyring presence | index `persisted`/`keyring_names` vs the keyring | the keyring | an entry removed outside Acquisition reads `persisted: true` until a restore fails |
| Halt | `rails.json.tripped` vs memory `tripped` **or** `dormant_trip` by `ACQ_TRIPWIRE` ✔; the CLI's offline reset reads the file | the file | a user with the tripwire off sees `halted: None` while the file says tripped; two interpreters of "is the world halted" |
| Retry hold | `retry_holds[route].until` vs `PolicyState.retry_hold_until` | the max (pacing) | status shows no `retry_after_secs` for a route held before it was established |
| Server count vs local history | `hits` (headers) vs `history` (local) | headers (invariant 2) | by design; the `PolicyMismatch` arm (4a.4) is the one place the local copy is not corrected |
| Policy state key | `endpoints[k] = Policy(key)` vs `policy_state_key(policy, account)` | — | a policy whose rule scopes change (all-`Account` → mixed) changes key with the same name, passes the name-only mismatch check, and orphans a `PolicyState` that keeps the daemon `is_live` |
| Send counters | `Rails.sends` (every `record`, HEAD and never-sent included), choke ring (100), journal lines, `Limiter.violations` | the journal | agree modulo bounds; the status word "sends" and the doc's "real sends" differ |
| Finished job counts | `Shared.evicted` vs the table | the table | agree only because eviction is the one exit and refuses after a queue failure |
| Job state word | `JobState` serde+`Display` vs `job_state()` parser | the protocol | a renamed variant compiles in both and reads `None` (4b.4) |
| Route closure | limiter `Degraded{until}` vs `closed_with[route] = (until, kind)` | the limiter | a re-degrade without `record_closure` falls back to `Upstream` |
| Idle knob | read at `life()` start and per `DaemonStatus` | env | same parser; drift only if the env changes under a running daemon |
| Plan ages | verbs' `PlanAges` vs the CLI's own `dur(generated_at - fetched_at)` ✔ (the CLI destructures `Planned { .. }` and drops `ages`) | the verbs | MCP shows the verbs' words, the CLI its own; `refresh --plan --json` prints the envelope with no `_age` fields (C163 unmet there) |
| Freshness window | planner arms over `tabs.fetched_at` vs shop over the item's `seen_at` (`shop.rs:1158-1240`) | the planner (it spends) | an item whose tab was refetched after its row was last seen reads stale in the shop and fresh in the plan |
| The "unresolved price" reason | `listing.rs:1667` decides; `search/src/price.rs:215` re-tests `note_unread && public` to word the gap | the listing | the price has one door, the reason two |
| Wait-knob parser | `parse_wait` (store) and `parse_within` (client) | — | same body, same bound, two crates |
| `frame.rs` | daemon and client copies | — | differ in the module doc only; held equal by nothing |
| `version` | JSON `version_document()` vs text from `sibling()` | — | `mtime_ns` JSON-only; two makers |
| Account list with no daemon | `auth status` hand-picked `json!` vs `acq accounts`' document | the verbs' document | field set and key order differ |
| Key order | documents built through `serde_json::Value`/`Map` (web `refresh` JSON, `DaemonReport::document`, `refusal::document`) sort keys alphabetically; structs keep declared order | the struct | the checklist names the refusal as fixed, but it is still a `Map` |
| Basis parts | `Basis::against` lists 14 parts, omits `properties`; equality uses full `PartialEq` | — | a continuation refused across a property-table bump names no differing part |
| `Status.league_list` vs `league_list_latest`; `LeagueList.ids` vs `entries[].id` | same tx / same function | — | cannot drift; pure duplication |

### 4d. Work that never reaches the screen

- **Computed and dropped**: `Submitted.before` (a full `Store::status()`, ~15 COUNTs) on every apply, consumed only by the CLI's `landed`; the MCP's `apply_plan` never calls it ✔ (`verbs/src/refresh.rs:663`, `mcp/src/main.rs:2111`). `Planned.ages` on the CLI (4c). `Limiter.violations`. `retry_holds[].secs`. `Leave`'s `Option<InFlight>`. `restore_jobs`' six counters and `Ingest.{added,moved,…}` reach one log line each. `Endpoint::from_job` computed twice per action.
- **Columns written, never selected**: `items.w`, `items.h`, `responses.item_count`, `item_events.response_id`. `responses.status` is always the literal 200 the daemon passes, so every `status BETWEEN 200 AND 299` filter is vacuous. `leagues` table read only as `count(*)`. `TabRow.removed_at` always `None` (the query filters gone tabs).
- **Dead branches / constant arms**: `Entry::row`'s `submitted_at.unwrap_or(now)`; `kind_needs_account`'s `"fetch" => is_real()` (fetch is refused in real mode at the door); `execute_inner`'s `other =>` and `apply`'s second `validate_apply` (reachable only from another build's row); `SendError::OffOrigin` on POST; `Clock::sleep`'s default body (overridden by every clock); `print_persisted_queue` on `Jobs::Live`; `Terminal::request` always `tool: None`; `Way::Again` unreachable for `Terminal::writing()`; `reset-tripwire`'s `candidates` loop has exactly one iteration; `Apply::{Nothing,Spent}` carry constant `applied: false, requests: 0`; `max_age_seconds: 0` on every selection plan; `SpentBy::Fetched.fetched_at_age: None` unreachable.
- **Emitted nowhere**: `set_priority` emits no event; `cancel` of a Running job emits nothing until it lands. `Life::state()/cause()` are `allow(dead_code)` outside tests.
- **One surface only**: web `apply` redirects before the `wants_json` arm, so a JSON client never receives `Apply::Submitted`; the job id is dropped by `apply` and `fetch_leagues`. Web `search` JSON drops `opened`/`carried`. `Total.places`, `snapshot_at_age`, `Table.left_out_needs` are JSON-only; the web page ignores `span`, `incomplete`, `computed_left_out`, `rest`. `PolicyStatus.retry_after_secs`/`history_len` print only in the TUI dashboard. `Way::RefreshApply` is never spelled on the MCP; CLI text ignores `Submitted.skipped`. `Store::orphaned_item_annotations` has no caller outside its test.

## 5. Refactors

### Provably equivalent

Each rests on a type check, an existing test table, or a pure-function identity; the holder is named.

| Refactor | What establishes equivalence |
| --- | --- |
| `persist(&self, &mut Entry)` → takes `&mut Shared` (or a `Held`-style token as `held.rs` does for credentials) | the type check; every caller already holds the lock |
| Derive `job_state()` from `JobState`'s serde/`FromStr` beside `Display` | protocol `wire.rs` fixtures + one round-trip test |
| Per-state counters maintained at transitions instead of `counts`' full scan; `children_open` counter on the parent instead of the per-finish child scan | `c166_a_page_is_one_state_newest_first…`, `parents.rs`, `evict.rs` |
| Destructure `sent` once in `head` and remove the three `expect("landed responses have a policy parse result")` | the `Option` is `Some` exactly when `completed` is `Ok` |
| `SendReport.ok` derived from `(status, error)` at the journal writer | pure function; journal goldens hold the wording |
| `keep_protocol_body` flag → two methods | two literal call sites, type-checked |
| One `now` per `Opened::plan`/`plan_selection`, passed down | under `ACQ_NOW` both reads are already equal, so every golden is unchanged |
| Replace the CLI's plan-age derivations with `Planned.ages` | same epochs through the same `age::span/ago`; one table test over `(generated_at, fetched_at, now)` |
| Serde constants for `Apply::{Nothing,Spent}`; compute `Endpoint::from_job` once; `StoreChanges::since` over `&[EventRow]` | `documents.rs` goldens; pure folds |
| Skip `Store::status()` on the MCP apply path (make `before` lazy) | `before` is `#[serde(skip)]` and read only by `landed` |
| Share `past_window(now, seen, window)` between planner and shop | identical arithmetic, type-checked (the *input* difference, 4c, stays and is a separate "trust me") |
| One `parse_wait`; one `has_column`; `frame.rs` as one `include!`d file as `digest.rs` already is | identical bodies; the `c137_` tests on both sides |
| `Persisted::read` under one deferred transaction | identical whenever no commit lands between the statements, which is the only case the current code answers consistently |
| Drop `LeagueList.ids`, `Status.league_list` for their sources | same function / same tx |
| Remove `items.w/h`, `responses.item_count`, `item_events.response_id` | no reader (grep); note the DDL is a contract input, so the facts version and contract revision move |
| Centralise exit codes in an `enum Exit { Ok, Failed, Usage }` | the type check over the three sites |
| `version` text as a projection of `version_document()`; web `refresh` JSON as a `#[derive(Serialize)]` struct | JSON is already the superset; `tests/web.rs` compares by value |
| Fold `Basis::against`'s 14-tuple into an iteration over the field list (adding `properties`) | `tests/next.rs` + the struct's `PartialEq` |
| `selector_tree` `pub(crate)` behind `Group::selector_node()` | the compiler replaces the `Member` grep; `tests/generated_equivalence.rs` holds answers |
| `Line.numbers: Vec<Option<Exact>>` instead of `Option<f64>` | every consumer already converts via `Exact::of` or compares floats read from one decimal; `tests/derive.rs`, `rows.rs`, `pseudo.rs` |
| Lower-case field/`has`/`is` names at parse | `language.toml` corpus cases pin the canonical texts; a case-only change is a visible diff |
| Fold `install_token_floor_hold` into `install_retry_hold` | the C19/C131 spec tables, with the 1 s `BUFFER` difference kept as a parameter |
| `reset-tripwire`'s `candidates`/`cleared` → `Option` | the loop has one iteration |

### Trust me, with the smallest input that breaks each

| Refactor | Smallest breaking input (old → new) |
| --- | --- |
| **Make a tab's `name`/`type` listing-owned** (guard the fetch upsert with `listed_response IS NULL`, as characters are) | listing `{"id":"t1","name":"New"}`, then a fetch body `{"stash":{"id":"t1","name":"Old"}}`: `acq tabs` prints `Old` → `New`. Also a substash stub named by its parent's fetch, then a fetch of the substash spelling the name differently. |
| **Gate fact drift in `check_spendable`** (C38 to the letter) | listing at t=1000, compile, an identical listing at t=1200, apply: sends → refuses a still-valid plan (`c44_fact_drift_never_refuses` goes red). |
| **Run `assume_counted` on `PolicyMismatch`** | `stash-request-limit` at 29/30, then a full-header response naming `stash-request-limit-v2`: next GET at once → held one window. New is the C19-consistent one; it is a spec-table row. |
| **Promote the gate's `debug_assert!`s to hard errors** | `league`'s first GET under `league@A` live while `character-list@A`'s policy is live and the leagues response names that same policy: silent double occupancy → panic or refused rekey. N39's policy name decides whether the input exists. |
| **Drop `retry_holds`, keep `PolicyState.retry_hold_until`** | a blocked HEAD 429 (`cf-mitigated`, no policy headers) on an Unknown route, then `reset-tripwire` inside `Retry-After`: today held; after, no `PolicyState` exists to hold it and the next probe goes out inside Cloudflare's window. |
| **Key the dead-grant mark by uuid** | a pre-uuid session restored from an index row without `uuid` meets a 400 on refresh: marked by name → no key to mark under. |
| **Journal line before the token POST** (the owner's parked item) | a refresh whose transport fails after the server counted it: one line with `error` → two lines or an open line; the oracle `counted == (method != HEAD)` and "first send per route is HEAD" need the open-line shape. |
| **Check `exit.begun()` inside `submit_marked` under `shared`** | `daemon stop` racing a submit between the server's check and the persist: `Submitted{id}` parked for the next daemon → `refused: the daemon is stopping`. |
| **Stamp activity at job finish** (C3 as worded) | one connection submits a 2 h `sleep` and disconnects: daemon leaves ≤5 s after it ends → 1 h after. |
| **Fail restore on an unknown state word** | a row with `state='paused'`: silent restore and `no job` → the daemon refuses to start. |
| **Emit an event from `set_priority`** | `acq jobs --watch` beside a priority change: no frame → one `event` frame; a watch golden with an exact frame count changes. |
| **Page the Dashboard's jobs** | 1,001 finished jobs this lifetime: whole list (or `internal` past the frame) → 1,000. |
| **Collapse `has_live_jobs(halted)` to `Running => true`** | a halted daemon with one deferred parent whose children all wait: idles out → held forever. |
| **Attribute `store_changes` to this apply's response ids** | applies A and B open; B's child lands inside A's window: A prints B's adds → only its own. |
| **Reload the MCP corpus on an items change, not the response id** | `acq store rebuild` while `acq-mcp` holds a corpus: the next `search` is served from the held corpus → reloaded (observable only if a derived column feeds the answer). |
| **Allow `Expect::Current` + `Replace` over an unreadable policy** | `acq policy set --replace <v>` with no `--if-revision` over a newer-build row: `UnreadablePolicy` → lands (C35 risk). |
| **Redact by JSON key only, drop the prose scan** | a daemon-log tail line `Authorization: Bearer abc`: `<token 1>` → printed. |
| **Single ordered writer in the MCP transport** | an unparsable line between two valid requests: its `-32700` may follow the second reply → strict arrival order. |
| **Drop `spoken()` for Rust-side spelling** | a `why` string carrying `acq auth` as data (a tab literally so named): gets ` — here, …` appended → not. |
| **Add `deny_unknown_fields` to `Response`** | refused by serde where `flatten` is used; otherwise `{"resp":"ack","keyring":"cleared","extra":1}` from a daemon one field newer: read past → every reply fails. |
| **Retry the winner look when `holder` is None** | the winner holds the lock but has not written its pid at the loser's exit: `SpawnFailed` → one look, `Opened{started_here:false}`. |
| **Stamp the revision in the file instead of `MAX(responses.id)`** | a file restored from a copy whose last response has the same id and second as a newer history (the documented uncovered case). |
| **Observation verbs open facts read-only** (C124 to the letter) | a v8 facts file opened by `acq tabs`: migrated to v9 → refused with the migrate remedy. |
| **Collapse the character and stash upserts into one coordinate-keyed path** | a substash whose parent is retired: its stubs are not rewritten today; the withheld arm must be carried over exactly. |
| **Lower-case closed-set values at parse** (`rarity=RARE` → `rare`) | `name="Kaom's Heart"`: matching unchanged (any-case binder), but the printed canonical and `(?-i)` patterns differ. |
| **Holder keyed by resolved realm** (`None` ≡ `Some(One(r))` over a one-realm store) | two asks with another realm's tab list ingested between: `None` must refuse, `Some(One)` still answers. |
| **Drop `REREADS`, trust the store's transaction** | a price write landing between the corpus read and `price::join`: `basis_moved` after 3 tries → a basis labelling prices from another revision. |
| **Filter `jobs --watch` lines by `--state`** | `--state open` watching `running → done`: the `done` line prints → nothing. |
| **Decide `print_result`'s apply rendering from the payload, not `status(id)`** | a mock `sleep` wrapped as apply whose payload lacks `child_jobs`: dump → "0 requests" summary. |
| **`refuse_usage` over `args_os()` lossy** | one non-UTF-8 argument: panic → exit 2 `kind: usage`. |
| **`daemon stop` reports ended sessions from its own reply** | a login landing between the two reads: omitted from `sessions_ended` → included. |
| **Clear `#stat-N-text` when its input empties** (`picker.js`) | a reader clears a picked id by hand: old stat text stays → cleared. |

## What to look at first

Three findings change what a user sees today and are held by nothing: the tab fetch rolling back a listing's `name`/`type` (4b.1), the Dashboard outgrowing the frame where `List` pages (4b.7), and the idle hour that a long job spends before it ends (4b.8). Two are latent and dangerous in the way the project most fears: the gate's occupancy invariants living in `debug_assert!` (4a.3), and `BlockShape::of` setting a persisted block mark from body text (4b.12). Everything in §5's left column is safe to take mechanically.
