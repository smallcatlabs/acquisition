# Store, protocol, client — the semantic model as built

Paths are relative to `/home/user/acquisition/crates/`. S = `acquisition-store/src`, P = `acquisition-protocol/src`, C = `acquisition-client/src`.

## 1. State

| Value | Lives in | Writer | Readers | Truth / mirror |
| --- | --- | --- | --- | --- |
| `responses` rows (endpoint, params, fetched_at, status, envelope, item_count, withheld) | SQLite `<provider>/<username>.db` (S/schema.sql:10) | `Store::record` only (S/lib.rs:2068) | snapshot bases, corpus listings, `league_list_in`, `Revision` | truth (facts); `status` is always the literal 200 the daemon passes (daemon/kinds.rs:280) |
| `tabs`, `characters` rows incl. `name/type/league/listed_json/listed_response/fetched_at/removed_at` | same file | `record` (listing upsert S/lib.rs:3014; fetch upsert 2040, 1884; retirement 2104/2118/2158) | `tabs_in`, `read_tabs`, `read_locations`, `leagues_held_in`, C164's `stash_listing` | truth; `json` is the fetched body, `listed_json` the listing's entry (two copies by design) |
| `items` rows (location coordinate, derived columns, json, seen_response, removed_at) | same file | `ingest_item` (S/lib.rs:3139/3184), per-location removal (2218), `rebuild` (2654, derived cols only) | corpus `ITEMS_SQL`, pricing `read_items`, `Store::item`, counts | truth for contents; derived cols are a mirror of `json` |
| `item_events` | same file | ingest only; nothing deletes | `events_since/after`, `Status.last_event` | truth (append-only) |
| `refused` | same file | `record` on a `MalformedBody`, own tx (S/lib.rs:1646) | `refused_list/refused` | evidence, no basis reads it |
| `leagues`, `account` tables | same file | `record` upserts (S/lib.rs:1700, 1710) | `leagues`: count only (2357); `account`: `account_identity` (S/snapshot.rs:359) | `leagues` is a union never retired; the list of record is the latest `leagues` response envelope (S/lib.rs:865) |
| `PRAGMA user_version` (facts v9) | file header | `Store::init` migration (S/lib.rs:1584) | every open; `Revision.facts_version` | truth |
| `Revision` = (MAX(responses.id), its fetched_at, user_version) | derived per read (S/corpus.rs:320) | — | search basis (C98) | derived |
| Annotations rows + `meta.account_uuid` | `<uuid>.annotations.db` (S/annotations.rs:109) | `put`/`delete` under IMMEDIATE tx CAS (1061, 1159); `bind` (957) | `get/list/at_revision/cleared`, snapshots | truth, irreplaceable (C35) |
| Annotation file revision = SUM(revision) | derived (S/annotations.rs:1309) | — | C98 basis | derived |
| `user.db` (saved searches) | `<provider>/user.db`, a newtype over `Annotations` (S/user.rs:80) | same doors | `User::list` → `(list, revision)` in one tx (1321) | truth |
| `accounts.json` entries (username, last_login, persisted, uuid?, keyring_names) | file, atomic temp+rename+dir sync (S/index.rs:543) | daemon only, via `index_apply` which needs `CredentialsHeld` (daemon/kinds.rs:182) | `Index::load` in verbs (frontend.rs:236) and daemon start | mirror of the keyring (`persisted`, `keyring_names`) and of /profile (`uuid`) |
| Keyring entries | OS keyring, by username and held names | daemon | daemon | truth for refresh tokens |
| `daemon.db` jobs rows (`user_version` 2) | `<provider>/daemon.db` (S/jobs.rs:40) | daemon `JobDb::upsert/delete/prune` | daemon; frontends via read-only `Persisted` when no daemon answers | mirror of daemon memory (S/jobs.rs:3-6) |
| `rails.json` | `<provider>/rails.json`, temp+rename, no dir sync (S/world.rs:1038) | daemon rails, offline `reset-tripwire` | daemon start | truth for trip/block/dead-grant marks |
| World root (canonical string), id = 12 hex of SHA-256(root) | derived (S/world.rs:289, 328) | — | socket name, log dir, `hello.world` | derived |
| `daemon.lock` (pid text), `run/ggg.lock` | files (S/world.rs:926, 805) | `Lock::acquire` | `Lock::holder` without the lock (916) | truth while held |
| Socket `<runtime>/<id>.sock` / pipe | filesystem (S/world.rs:392) | daemon | client `reach` | derived |
| `CONTRACT_REVISION` | baked `env!` from `build.rs` digest over root manifest, lock, protocol+store manifests and `src/` trees (P/../build.rs:45; digest.rs:68 KINDS) | build | `hello` both ways, `Verdict` (C/client.rs:774) | derived from sources |
| Daemon artifact (FileIdentity + sha256) | `hello` (P/artifact.rs:75) | daemon at start | `Verdict::of` opens the sibling once (C/client.rs:768) | derived |
| `DaemonId` + `Verdict` | client memory, judged once per handshake (C/client.rs:808) | handshake | every report and door decision | derived snapshot |
| Door `Machine` state | client memory (C/door.rs:224) | `step` | driver | — |
| `Client.stalled` | connection memory (C/client.rs:1076) | `request` on expiry (1987) | every later request refused | — |

## 2. Transitions

Ingest (`record`, one IMMEDIATE tx per response, S/lib.rs:1630):

| Trigger | Precondition | Effect |
| --- | --- | --- |
| `stashes` listing | body has `stashes` array, every entry has `id` (1916, 3001) | upsert tabs (listing owns `parent,name,type,idx,listed_json,listed_at`; `fetched_at` cleared if row was retired; `removed_at=NULL`), stamp `listed_response`, retire unstamped top-level/folder-child rows of that (realm,league) (2118), orphaned substashes keep rows but lose `fetched_at` (2146); items at retired locations removed (2218) |
| `stash` fetch | ids on items and children (1966, 1982); body `stash.id` equals the target if present (1970) | if the tab or (for a sub) its parent is retired → withheld: only the response row lands (2007); else lift items, upsert the row (`name,type,json,fetched_at` overwritten; `parent` kept), stubs become substash rows, a parent fetch retires stubs it no longer carries (2158), items not stamped by this response at the location removed |
| `characters` listing | array present, every entry has `id` and `name` (1722–1745) | upsert (listing owns `realm,name,league,class,level,listed_json,listed_at`, revival clears `fetched_at`), stamp, retire unstamped rows of that realm (2104) + their items |
| `character` fetch | `character.id` present (1772); ids in the six arrays (1783) | retired row → withheld (1798); else `name/class/level` overwritten only if never listed (1886), `realm` and `json` overwritten always (1885, 1889), `league` never; items filed at `(request realm, row's listing league, character, id)` |
| `profile`, `leagues` | — | upsert `account` / `leagues`; a response row always lands |
| malformed body | — | whole tx rolled back; body kept in `refused` in its own tx (1646); `responses.id` not consumed |

Item classification per `ingest_item`: new → `added`; known → `moved` if realm/kind/id/(stash)league differs or it was removed; `changed` if json differs (volatile `veiledMods` ignored) or `container` differs (3169–3182).

Annotations: `put(None)` creates (revision 1) or re-creates over a tombstone (tombstone+1); `put(Some(r))` replaces iff live and revision==r; `delete(r)` tombstones iff live and ==r; otherwise `Conflict{current}` or `Unreadable` (S/annotations.rs:1074–1104, 1164, 1457).

Door machine (C/door.rs:286): Reaching → AwaitingHello → {Done, stop → Settling → Reaching} ; Reaching/Nothing → Starting{Spawning→Consulting→Pausing→Reaching | Polling→Looking→AwaitingHello}. Every row pinned in `c157_the_door_is_a_table_of_event_scripts`.

Commutativity (verified from the SQL):
- COMMUTE: two `stash` fetches of different tabs; a `stash` fetch and a `characters` listing; two `put`s on different annotation keys; `rebuild` with anything (no reader of derived cols is revision-labelled except pricing, which is unlabelled).
- IDEMPOTENT: re-ingesting an identical body (no events; `a_large_tab_reingested_unchanged_reports_nothing`) except that `responses.id`, `last_seen`, `seen_response` and the `Revision` move; `ensure_wal`; `Store::init` at the current version; `Index::save`; `Lock::acquire` by the holder (no: a second acquire by the same process fails WouldBlock — flock is per open file description); `RailsState::write` of an unchanged value.
- ORDER-DEPENDENT: listing-then-fetch vs fetch-then-listing of a tab the listing retires: fetch-first lands items then the listing removes them; listing-first withholds the fetch (`a_late_fetch_never_revives_a_location_a_listing_retired`). Listing-that-revives then fetch: `fetched_at` cleared then set; fetch then listing-that-revives: `fetched_at` cleared and the just-landed items remain live while the row says never fetched (S/lib.rs:3018 clears unconditionally on revival; the items were landed by a fetch at a *retired* location only if the fetch was not withheld — it is, so this order cannot land items; verified consistent). `delete` then `put(None)` works; `put(Some(r))` after another writer's `put` conflicts. Door: `Stopped(Gone)` then `Nothing` → spawn; `Nothing` while `stop_failed` set → the error is cleared (C/door.rs:397) — "stop failed" is reported only if the daemon is still there on the next look.

## 3. Dependencies (effects and where their precondition is checked)

| Effect | Precondition | Checked at | TOCTOU |
| --- | --- | --- | --- |
| facts write (`record`) | body shape valid; location liveness | inside the same IMMEDIATE tx (S/lib.rs:1791, 1995) | none within the file; C164's submit-time judgement (daemon/kinds.rs:388) is a different moment than the send: a listing landing between makes the fetch withheld, not wrong |
| `refused` write | the ingest tx rolled back | S/lib.rs:1639 then 1646, two txs | a crash between loses the evidence, never a fact |
| intent write | value typed and canonical, provenance well-formed, revision matches | `check_value` before the tx, CAS inside (S/annotations.rs:1049, 1066) | the planner's `at_revision` read and the `put` are two txs; the CAS makes the race a `Conflict` |
| `accounts.json` publish | `CredentialsHeld` in hand | type-checked (daemon/kinds.rs:182); write temp, fsync, rename, dir fsync (S/index.rs:549) | none: single writer by construction |
| annotation export | `dest` absent | `dest.exists()` then `hard_link` refuses a late appearance (S/annotations.rs:1364, 1386); the rename fallback has a real window (1397) | yes, on filesystems without hard links only |
| `rails.json` write | — | fixed `.partial` name (S/world.rs:1047) | two writers (daemon + offline reset) collide on the partial; no dir sync |
| spawn `acqd` | `may_spawn` (policy, `ACQ_NO_SPAWN`, tty, C88) read once at `Machine::new` (C/door.rs:251); world created first (C/client.rs:1664) | door start | the daemon re-derives the world from `ACQ_STORE_DIR` the door passes (2178): same string, canonicalised twice |
| hello / request frame out | connection in hand; `stalled` unset | C/client.rs:1382, 1969 | — |
| `daemon_stop` frame | the handshake identified a peer (any build) | machine state (C/door.rs:425) | — |
| spawn verdict `Exited` vs the winner look | `holder` read **after** the exit is seen (C/client.rs:1441) | poll time | the winner may not have written its pid yet (`Lock::acquire` locks then writes, S/world.rs:939–959) → `holder=None` → `Ended::Exited` without the look; a daemon is in fact running |
| report of a mismatch | `Verdict` | captured once per handshake (C/client.rs:813) | none (the review's fix) |
| frontend reads the ledger | observe door said Absent | elsewhere (verbs), then `Persisted::read` | a daemon spawned by another process between the two reads a half-restored queue; `written_at` and the rows are two autocommit statements (S/jobs.rs:714–724) |

## 4. Findings

### 4a. Nondeterminism

1. **`Persisted::read` is two snapshots** (verified, S/jobs.rs:714 and 718): `written_at = MAX(updated_at)` and the open rows are separate autocommit reads. Under a writer the user can see `written_at` newer than every row shown, or a row updated after `written_at`. `read_job` is one statement; `stash_listing`, `tabs_document`, `status` and both snapshots use one tx.
2. **Spawn race, late pid** (verified, C/client.rs:1437–1443 vs S/world.rs:939–965): the loser exits on the held lock; the client reads `daemon.lock` only then. If the winner has locked but not yet written its pid, `holder` is None and the door reports "the daemon exited during startup" (`Ended::Exited`, C/door.rs:341) with no look for the winner. Order A (pid written first): `Opened{started_here:false}`. Order B: `SpawnFailed` while a daemon runs; the next command finds it. Pinned only for the holder-present order (`c147_a_spawn_race_is_won_however_late_the_winner_starts`).
3. **`refresh_snapshot`/`pricing_snapshot` pair facts and intent from two files** (verified, S/snapshot.rs:281–291, 328–340): facts under one tx, then `annotations.get/list` after `tx.finish()`. A policy write between gives a snapshot whose `policy.revision` is newer than the facts' instant; the CAS at apply re-checks, so the plan is refused rather than wrong.
4. **`Status.bytes`** is `fs::metadata` outside the read tx and excludes the `-wal` file (verified, S/lib.rs:2355): a store whose facts sit in the WAL prints a smaller size than it holds; `acq store status` prints it (cli/store_cmd.rs:696).
5. **`rails.json` key order** (verified, S/world.rs:1006, 1045): `refresh_failed_by_account` is a `HashMap`; `serde_json::to_string` on a struct emits map entries in iteration order (only `serde_json::Map` is sorted), so the file's key order varies between writes. File-only, no reader orders by it.
6. **Door `run` keeps the last event of a step** (verified, C/client.rs:1333–1344): if an effect list ever carried two observing effects the first would be dropped silently; today every list has at most one (held by the table rows, not by a type).
7. **`stop_until` tie** is deliberate and pinned (`biased` select, C/client.rs:1830).

### 4b. Invariants enforced by hope

1. **C54 is asymmetric between tabs and characters** (verified). A character fetch keeps a listed row's `name/class/level` (S/lib.rs:1886–1888); a tab fetch overwrites `name` and `type` unconditionally (S/lib.rs:2041). `Store::tabs`/`TabRow.name`, `TabSnapshot.name` (S/snapshot.rs:518), the corpus's `LocationRow.name/tab_type` (S/corpus.rs:379) and `TAB_ORDER_KEYS`'s tiebreak (S/lib.rs:1128) all read the column, so a fetch landing after a newer listing rolls the tab's name and type back on every surface until the next listing. Only `metadata` is pinned listing-owned (`a_fetch_never_clobbers_the_listed_metadata` asserts metadata alone). The REVIEW-CHECKLIST row "a late fetch rolled a character's fields back" was fixed for characters only.
2. **A character fetch overwrites `realm`** (verified, S/lib.rs:1885) while the listing's retirement and the live join are realm-scoped (2104; S/corpus.rs:314). Needs one id answered under two realms, which GGG's per-realm ids make unlikely (suspected impact, verified mechanism).
3. **C85's "the rest refuse one" holds for requests only** (verified). `Request` carries `deny_unknown_fields` (P/protocol.rs:503) and is pinned (`c85_a_versioned_request_refuses_a_field_it_does_not_know`); `Response` (625), `JobInfo` (P/job.rs:79), `Outcome` (181), `JobTimes`, `Children`, `SessionStatus`, `ErrorRecord`, `RailsStatus`, `PolicyStatus`, `SendRecord`, `DegradedEndpoint`, `Artifact`/`FileIdentity` are lenient; `Quote*`, `RuleStatus`, `WindowStatus`, `ListCounts`, `ListingBasis` are strict. `flatten` on `Response::Result` and `JobInfo` makes `deny_unknown_fields` impossible there. No test pins either way for the response plane.
4. **C108's prune rule is a comment** (verified, S/corpus.rs:89–99 "Whatever prunes this file keeps two rules"): nothing prunes the facts file today (no `DELETE FROM`/`VACUUM` outside `jobs.rs` and the annotation export), so the rule is vacuous and unheld. `the_revision_moves_with_every_record_and_with_nothing_else` holds the present writers.
5. **The two `frame.rs` copies are held equal by nothing** (verified `diff`): client and daemon copies differ only in the module doc; no check compares them, both read `MAX_FRAME_BYTES` from the protocol.
6. **C147's "environment read once"** is true of the world only: `ACQ_NO_SPAWN`, the tty, `ACQ_HELLO_WITHIN`, `ACQ_STARTUP_WITHIN` and `ACQ_PROVIDER` are read per door/arm/handshake (C/client.rs:1154–1165, 336, 1221, 657, 776). Harmless in-process; the ruling's sentence is wider than the code.
7. **C124 "migration belongs to `acq` and the daemon alone"**: every frontend's facts door is `Store::open` (verbs/frontend.rs:401), the MCP included, so `acq-mcp` migrates an older file too (suspected: the verbs door is the only one I read; the ruling's words are narrower than the code).
8. **Registry "Pinned: nothing yet" entries that are in fact held**: C30 by `an_item_without_an_id_is_malformed_and_poisons_nothing` and the `c145_` tests; C59 by `acceptance_is_per_family_as_documented` (P/realm.rs:238); C60 by `the_basis_and_the_tab_set_are_per_realm`, `realms_keep_the_same_league_and_id_apart`; C48 by `Store.conn` being private and `TAB_GONE`/`TAB_ORDER_KEYS` `pub(crate)`; C28's "the daemon's part is `record` and one read" by review only (the daemon also calls `Store::open` and `checkpoint`, daemon/kinds.rs:326, server.rs:606).
9. **"Every step short of its report asks to observe"** is an `expect` (C/client.rs:1343) and "an event the state cannot receive" a `panic!` (C/door.rs:366): C47-style internal invariants held by the table test, not by types.
10. **`put` doc vs code** (verified, S/annotations.rs:1015–1019 vs 1097–1104): the comment says two writers who read "nothing there" after a clear "both land, in sequence"; the code makes the second `None` create a `Conflict` once the first landed. Code is stricter than its comment.

### 4c. One fact in two places

1. **Account identity: username-named facts file, uuid-named intent file, index entry, `account` table** (verified: S/index.rs:157, S/annotations.rs:178, S/index.rs:15–31, S/lib.rs:1710). The index's `uuid` is `Option` (pre-2026-08-31 entries lack it → `Fault::NoUuid`, verbs/frontend.rs:418). A rename orphans the old facts file (S/index.rs:304). The facts file's `account` table upserts by uuid and never deletes: a username reused by another GGG account leaves two uuids and `account_identity` refuses every snapshot (S/snapshot.rs:375) — the file name is the one place the disagreement shows. `filename_safe` maps every non-ASCII char to `_` (S/index.rs:229): two names differing only there share one facts file (suspected; depends on GGG's name alphabet).
2. **`persisted` / `keyring_names` vs the keyring** (verified, S/index.rs:25, 42): mirrors settled only through `settle_keyring_names`; an entry removed outside Acquisition leaves `persisted: true` until the daemon's restore fails.
3. **`tabs.json` vs `tabs.listed_json`, `characters.json` vs `listed_json`** (by design). After a revival `json` still holds the disowned body: snapshots guard with `fetched_at` (S/snapshot.rs:603, 1046) but `status()`'s `granted_skills`/`unlifted_items` read `c.json` for every live character regardless (S/lib.rs:2386–2393): a revived, not-yet-refetched character contributes a stale count (verified).
4. **`Status.league_list` vs `Status.league_list_latest.fetched_at`** (verified, S/lib.rs:2358–2373): the same predicate twice; agree because read in one tx.
5. **`LeagueList.ids` vs `entries[].id`** (verified, S/lib.rs:896): derived in the same function.
6. **Daemon memory vs `daemon.db`** (by design, S/jobs.rs:3): the table is written at each state change under the daemon's job lock; `Persisted.state == "running"` means "was running when it left" (S/jobs.rs:534).
7. **Two parsers of the wait knobs**: `parse_wait` (S/lib.rs:1261) and `parse_within` (C/client.rs:353) are the same function with the same bound (2_147_483) in two crates.

### 4d. Work that never reaches the screen

- `items.w`, `items.h`: written by ingest and `rebuild`, read by no SELECT (verified by grep; `ItemSnapshot` reads x,y only, S/snapshot.rs:759).
- `responses.item_count` and `item_events.response_id`: written (S/lib.rs:2070, 3146), never selected.
- `responses.status`: always 200 from the daemon (daemon/kinds.rs:280), so every `status BETWEEN 200 AND 299` filter is vacuous and `Refused.status` is constant.
- `leagues` table: upserted per `leagues` response, read only as `count(*)` (S/lib.rs:2357); the list of record is the envelope.
- `TabRow.removed_at`/`removed_at_age` on `Store::tabs` and `StashListing`: always `None` since `tabs_in` filters `TAB_GONE IS NULL` (S/lib.rs:1048, 1063).
- `Store::orphaned_item_annotations` (S/lib.rs:2669): no caller outside the store's own test.
- `Ingest.added/moved/changed/removed/unlifted/granted`: reach only the daemon log line (daemon/kinds.rs:286).
- `UNKNOWN` fallbacks in `Bootstrap::read` for `version` (P/protocol.rs:126): `version` is informational on both sides; nothing branches on it.
- `Timer::Hello` is armed then taken by `SendHello` (C/client.rs:1388); `Consult(Hello)` is never issued, and a consult of an un-armed timer reads as expired (1452) — a fail-closed default no row exercises.

## 5. Refactors

PROVABLY EQUIVALENT
- One `parse_wait` for both crates (client already links the store): identical bodies and constant; the `c137_*` tests on both sides establish it.
- One `has_column` (S/lib.rs:2974, S/jobs.rs:180): same `pragma_table_info` query.
- Drop `LeagueList.ids` and `Status.league_list` in favour of their sources: pure derivations in the same function/tx (S/lib.rs:896, 2358).
- `Persisted::read` under one deferred transaction: identical results whenever no commit lands between the two statements, which is the only case the current code answers consistently.
- Share `frame.rs` as an `include!`d file the way `digest.rs` is shared by two build scripts: the diff is comment-only; the tests "run in both".
- Remove `items.w/h`, `responses.item_count`, `item_events.response_id` columns: no reader exists (grep); but the DDL is a contract input, so the facts version and the contract revision move.

TRUST ME (smallest breaking input)
- Make a tab's `name`/`type` listing-owned as a character's are (guard the fetch upsert with `listed_response IS NULL`). Differs when a listing says `{"id":"t1","name":"New"}` and a later fetch body carries `{"stash":{"id":"t1","name":"Old"}}`: today `acq tabs` prints `Old`, after the change `New`. Also a substash stub named by the parent's fetch and then a fetch of the substash whose body spells the name differently.
- Read `Lock::holder` before polling the child, or retry the look when `holder` is None: differs when the winner holds the lock but has not written its pid at the loser's exit (today `SpawnFailed`, after the change one look).
- Add `deny_unknown_fields` to `Response`: refused by serde where `flatten` is used (`Response::Result`, `JobInfo`); and a daemon one field newer would make a client fail every reply instead of reading past it — differs on `{"resp":"ack","keyring":"cleared","extra":1}`.
- Stamp the revision in the file instead of deriving it from `MAX(responses.id)` (the "Parked" counter): differs for a file restored from a copy whose last response has the same id and second as a newer history (the documented uncovered case).
- Make the verbs' facts door read-only for observation verbs (C124 to the letter): differs on a v8 facts file opened by `acq tabs` — today migrated to v9, after the change refused with `MIGRATE_REMEDY`.
- Collapse the character and stash upserts into one coordinate-keyed path: differs on a substash whose parent is retired (today its stubs are not rewritten, S/lib.rs:1988–1994) unless the withheld arm is carried over exactly.
