# Parks triage (2026-10-19) — 106 parks

Amended 2026-10-19 after the roadmap's outside review: ST-3 is FIRED on its third clause, so six parks have fired, not five.

Status key: FIRED = trigger evidence cited; SOON = LIKELY-SOON (3.30 on 2026-11-13, PoE2 1.0 on 2026-12-11, or the GUI slice); DORM = DORMANT; UNOBS = UNOBSERVABLE; WITHDR = WITHDRAWN-CANDIDATE. Size S hours / M a day or two / L a slice.
Rule applied for FIRED: the trigger's own words are met by a cited artifact. Parks written from a seat finding whose trigger is "the next user/run that ..." are DORM, not FIRED, even when the seat evidence is strong (marked "near").

## CONTEXT.md (3)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| CTX-1 | Shop/forum publishing (POESESSID, auto-post) | the owner asking for it | WITHDR — owner 2026-09-12 "no shop publishing. that will happen much later or not at all"; 2026-09-13 "I do not want acquisition updating forum shops directly" | rewrite as a refused ruling or delete; keep the render-to-stdout line | S |
| CTX-2 | Transcription lint against non-owner C++ files | next Rust table drafted from a C++ one | DORM (would fire with PR-1 legacy import or any GUI port) | keep; design open | M |
| CTX-3 | External tracker (GitHub issues/agent tracker) | first issue a park cannot absorb | DORM (alpha out 2026-10-19; first issue is the event; `audits/release-2026-10/cpp-issues.md` drafts C++ issues) | keep; revisit when first issue lands | S |

## daemon (17)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| D-1 | Priority levels beyond the one | a second priority level in use | DORM | keep (GUI interactive vs agent could force it) | S |
| D-2 | Packaging beyond archives (installer, signing, other channels) | owner reopens a channel, or user stopped by Gatekeeper/SmartScreen | DORM (neither met, per the entry) | keep; the README uses curl so quarantine rarely bites | M-L |
| D-3 | Second installation on one machine | a consumer that needs one | DORM | keep (or delete: no consumer imaginable) | M |
| D-4 | Keeper for frontends that never spawn (cron, real-mode MCP): readiness, signals, exit codes, launchd/systemd | first non-spawning consumer in daily use | DORM, near: release seat confirmed "acq-mcp connects per call and a real-mode daemon exits idle" (RELEASE-SLICE.md row; README recipe e8f21169) | build early for the agent surface; fold in F-6 and F-20 | M |
| D-5 | Correlation ids / multiplexed requests on one connection | evidence a GUI needs them | SOON (GUI) | build in GUI slice if the GUI proves it | M |
| D-6 | keyring-core with per-platform stores (keyring 4) | sandboxed package (Flatpak/MSIX/App Store) or provisioning-profile bundle | DORM (a Tauri package could fire it) | keep | M |
| D-7 | Artifact dependencies (-Z bindeps) for tests | bindeps stabilising | DORM (external; toolchain pinned 1.99.0) | delete; a cosmetic wish, re-add when it stabilises | S |
| D-8 | world.rs out of the store crate | a frontend that must not link SQLite | DORM | keep | M |
| D-9 | GUI-hosted daemon (refused: GUI spawns acqd) | GUI slice recording its spawn policy | SOON (GUI slice itself) | convert to a ruling when the GUI slice records spawn policy | S |
| D-10 | `spawned_by` in hello/daemon status | two spawners on one machine, human needs to know | SOON (CLI/MCP plus GUI spawner) | build with the GUI slice if the GUI spawns | S |
| D-11 | Hash-only artifact identity (C84) | a false match from file identity, or a second install | DORM | keep | S |
| D-12 | Linux glibc 2.35 floor -> musl static | user on an older distribution | DORM | keep | M |
| D-13 | Elevated daemon serves unelevated clients (pipe label) | owner rules elevation a boundary, or elevated daemon met | DORM (measured 2026-10-15) | keep | M |
| D-14 | Host job object forbids breakaway, ends daemon (Windows sshd) | an agent host seen ending the daemon | DORM (Windows sshd is a plausible agent host) | fold into D-4 (keeper) | M |
| D-15 | `subst`/mapped drives under `world::canonical` | a store root or archive on one | UNOBS (no path from a user to the owner) | delete | S |
| D-16 | Job verb spawns only when a login would be restored | a user reports the surprise | DORM, near: cli-opus call 101, cli-sonnet 92-93 called the spawn the fault (agents, not users) | keep; text already says so | S |
| D-17 | GGG error passed through with a verb-level hint (`acq stashes`) | a user's report | DORM, near: cli-opus call 89 | build as paper cut | S |

## frontends (21)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| F-1 | A term's count beside the total reads as contradiction | a run spends a call on the terms block | DORM, near: every re-sit run re-read it, none spent a call (re-sit README count 1); owner "Measured; park" | keep | S |
| F-2 | Queue-management UI (drag, progress bars) | the GUI | SOON (GUI) | build in GUI slice | M |
| F-3 | The read economy as a ruling (summary default, filters, bounded detail) | a run spends a second call on an answer's size, or a bounded document a door still cuts | FIRED — rerun's `left_out` still cut at 160 KB (commit d79c4ca7; RELEASE-SLICE.md row "read economy park re-triggered"; mcp-opus-2 call 37) | build as a slice-sized ruling; absorbs F-11, PR-14 | M-L |
| F-4 | acq-mcp drops a request it cannot parse as JSON (no reply; 123 nots / 127 levels) | a consumer that waits on such a call, or the MCP's next seat | DORM | build early: a hang is the worst agent failure; error reply where id is readable | S-M |
| F-5 | Results over a subtree as events or a `results` verb | a second consumer (GUI or MCP) showing which | SOON (GUI) | build in GUI slice | M |
| F-6 | MCP get_result/job_status/auth_status with no daemon | an MCP consumer losing a detached job result to idle shutdown | DORM, near (RELEASE-SLICE.md row 161, idle exit confirmed) | build with D-4; CLI already does it via `Persisted::read_job` | S-M |
| F-7 | A watch that waits for the daemon to return | the GUI slice | SOON (GUI slice) | build in GUI slice (subscriber) | S |
| F-8 | Printed commands are POSIX-quoted (PowerShell breaks) | a Windows agent running printed commands in PowerShell | DORM | keep; plausible with Windows alpha users | S-M |
| F-9 | Standalone queue TUI (`dash` binary) | the queue-monitoring TUI slice | DORM (no such slice scheduled) | delete or keep as direction; low value | L |
| F-10 | Structured-error taxonomy beyond additive `kind` | GUI slice, or owner asking | SOON (GUI) | build in GUI slice (P2) | M |
| F-11 | Container `price show` carries every item (22-174 KB) | a run re-reads a container's show for its cost | DORM (none of the four did) | fold into F-3 | S |
| F-12 | MCP ages are epoch seconds only | user report, or a run gets a date wrong | DORM, near: mcp-opus worked them by hand; mcp-sonnet trusted one clock over another | build: add age beside stamps | S |
| F-13 | No `store_refused` MCP tool | a user needing the list from an agent | DORM | build as paper cut | S |
| F-14 | Shared refusals name CLI flags/verbs in MCP sessions | a run that fails to recover from one | DORM (every release-seat run recovered) | fold into one refusals bundle with F-21 | S-M |
| F-15 | Clear receipt carries `written: null`, not the tombstone revision; tool text | a run names a wrong revision after a clear | DORM, near: mcp-opus-2 calls 39-53, debrief 8 | refusals/receipt bundle | S |
| F-16 | `price_show` `manual.revision` is a container's when inherited | write refused for an inherited revision | DORM, near (mcp-opus-2 call 31) | refusals/receipt bundle | S |
| F-17 | `never_fetched_by_type` lists totals, not never-fetched | a run reads total as never-fetched | DORM, near (mcp-opus-2 call 7) | rename or add count; paper cut | S |
| F-18 | Sticky default realm / default league | the GUI's realm control | SOON (GUI; also PoE2 1.0 makes realm friction real) | build in GUI slice (adapters only, C96) | M |
| F-19 | README checksum compared by eye | a user reports a mismatch they missed | DORM | keep | S |
| F-20 | No-daemon remedy's daemon has no keyring: not_logged_in | a run follows the remedy and is refused | DORM (rerun worker 2026-10-19) | fold into D-4 | S |
| F-21 | Three MCP refusal shapes (set_sync_policy kind, rmcp error, outputSchema) | run parses revision out of prose, sends bad id, or host needs schema | DORM | refusals bundle (`tool_refusals.rs`, C11) | S |

## network (3)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| N-1 | Wire-send budget over actual sends | a consumer that needs it | DORM | keep | M |
| N-2 | Invalid-request (4xx) threshold as a claim | observed at a first contact | UNOBS (probing it breaks the rails; not seen in dozens of live rows) | delete | S |
| N-3 | Token expiry during a limiter hold | a 401 on a token that expired in a hold, in a journal | DORM (cost is one 401, then C87 refreshes; not seen in RUN-LEDGER) | delete or leave; harmless | S |

## plans (9)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| PL-1 | Coverage advice in `refresh --plan` | `price status`/render report prove the wrong place | UNOBS (needs a proof of a negative) | fold into PL-8 | S |
| PL-2 | `aging` in the plan | a cycle estimate that is trusted | UNOBS (nobody can observe "trusted") | delete | S |
| PL-3 | Explicit-selection door (C76) | a one-off workflow wanting a reviewed plan; seat friction; GUI refresh gesture | SOON (GUI refresh gesture named as natural consumer) | build in GUI slice | M |
| PL-4 | Universal Plan grammar / five-verb surface | a second plan-bearing consumer (C75) | DORM (direction only) | delete (C71 already parked) | S |
| PL-5 | Dynamic `--deep` fan-out | live evidence two-cycle reconciliation hurts | DORM (none so far) | keep | L |
| PL-6 | Type-level sync-policy filters | a policy author needing a type exclusion | DORM | keep | M |
| PL-7 | Two applies of one plan at once (C111) | duplicate fetch traced to concurrent applies | DORM | keep; check at daemon submit when seen | S-M |
| PL-8 | Plan silent on tabs the policy leaves unnamed (3,347 vs 4) | a second reader taking a plan for coverage | DORM, near: mcp-opus call 37 fault 5; re-sit mcp-opus 34-37 and cli-sonnet 20-22 tried the plan for coverage; owner deferred past alpha 2026-10-18 | build (one count in the summary); absorb PL-1 | S |
| PL-9 | Per-realm merge at `policy set` (a set erases the other realm's policy) | a second realm in daily use | SOON — PoE2 1.0 2026-12-11; scheduled in note 36 step 6 | build before 12-11: it is a silent data loss | S-M |

## pricing (16)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| PR-1 | Legacy import build (C73) | the replace ruling (owner: "Rust sits beside c++ now and replaces later", undated) | DORM | keep | L |
| PR-2 | Third-party price feeds | currency ratios or a suggest-a-price consumer | DORM | keep (C79 governs) | L |
| PR-3 | Bulk-exchange item list as a reference table | owner prices a non-currency item by ratio, or a manual row on a ratio-named tab | DORM | keep | M |
| PR-4 | Row-granularity annotation history | a "since" question the returned prior cannot answer | DORM | keep | M |
| PR-5 | One change cursor over facts and intent | an agent consumer shows two reads where one would do | DORM | keep | M |
| PR-6 | Batch pricing by query | a real batch workflow handles cannot express | DORM | keep; likely at a league start | M |
| PR-7 | Current offer (`~c/o`) as a value | a real use; census found zero rows | DORM (near-UNOBS) | delete | S |
| PR-8 | PoE2 currencies and per-realm tags | a PoE2 stash endpoint, or a poe2 price the owner wants | SOON — PoE2 1.0 2026-12-11 (note 36 step 6); README says GGG offers no PoE2 stash tabs yet | build at launch-day first contact | S-M |
| PR-9 | Currency totals and history | owner asks (2026-09-09 "when I asked for it") | DORM | keep | M |
| PR-10 | Name->id resolution for price targets | authoring friction | DORM | build as paper cut if agents hit it | S |
| PR-11 | "What did I last post" basis | a user asks | DORM (depends on CTX-1 render use) | fold into store `user.db` park (ST-1) | M |
| PR-12 | Render policy cells no sitting has observed | the next forum sitting | UNOBS in practice (needs an owner sitting; none scheduled) | keep or delete | M |
| PR-13 | `price list` refusal for a container with own row does not say so | a run/user reaching it | DORM | build now (same refusal document) | S |
| PR-14 | Shop document `posted`/`freshness` unbounded, `page` drops counts | next MCP seat's cut on `shop_render` | DORM (the 160 KB cut itself preceded the park; d79c4ca7) | fold into F-3 | S |
| PR-15 | `price status` counts unexplained (`residue`, overlaps) | a user asks which notes, or a run takes one count for another | DORM, near: 4 release-seat runs re-read, 3 never understood `residue` | build: words + invalid-notes listing | S-M |
| PR-16 | Shop page `StashN` rank has no map to tab names | user asks which Stash is which / pastes under wrong tab | DORM, near: 4 runs matched by counting, 2 retracted | build: rank->tab line beside page | S |

## search (21)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| SR-1 | Saved queries, names, tags (user-scoped) | first consumer asking to save/name/tag a query | SOON (GUI) | build with ST-1 in GUI slice | M |
| SR-2 | Persisted projection | empty-query load over 500 ms | DORM — last M3 rows 472-481 ms (search/M3-LEDGER.md, 2026-10-15); 95% of budget; 3.30 corpus growth could cross it | keep; re-measure after 3.30 fetches | L |
| SR-3 | Totals batch | a corpus-holding consumer asks, or SR-2 fires | DORM (tied to SR-2) | fold into SR-2 | L |
| SR-4 | Whole-corpus export of derived lines | a recorded question the language cannot serve | DORM | keep | M |
| SR-5 | Realm word inside a term | a recorded per-realm question | DORM (PoE2 1.0 may supply) | keep | M |
| SR-6 | Query by example | first seat that asks | DORM | keep | S |
| SR-7 | `sum 0 · N lacking` reads as measured 0 | a seat that misreads it | DORM | build as paper cut | S |
| SR-8 | Unique variant as derived field | owner's answer at the seat | DORM (3.30 balance changes make variants matter) | keep | M |
| SR-9 | Whether a line can still be made | owner procuring a source (C79) | UNOBS (no registered surface says; nobody can procure) | delete | S |
| SR-10 | Mod attribution (affix type, tier) | a recorded question needing affix count or tier | DORM | keep | L |
| SR-11 | Incremental update of held corpus | a real GUI feeling the whole reload | SOON (GUI) | build in GUI slice if measured | L |
| SR-12 | What no sitting has asked the site | the next sitting | FIRED — round v (2026-10-16), round x (2026-10-18); remainder routed at TRADE-SLICE.md | split the leftovers (`stat.text.many`, graft, invitation, `scourge-in-sum`, `blighted-base`) into one-line sitting items, delete this park | S |
| SR-13 | Twin id the site leaves out of totals | a sitting that shows the site counting it | DORM (needs sitting) | keep | S |
| SR-14 | Answer's words the seats left unruled | a seat that trips on one | DORM | keep | S |
| SR-15 | Scope `tabs` pointer under two `league=` terms | a seat that writes such a query | DORM | build (cheap) or keep | S |
| SR-16 | Whether the site can list an item (priced, public) as field/mark | a ruling on what the site lists, or the release's first user asking | DORM, strongest near: asked by both Opus second-round runs and every re-sit run (cli-opus 1, mcp-sonnet 1 worst); owner deferred past alpha 2026-10-18 | build first after alpha; needs a site sitting | M-L |
| SR-17 | Query's own filtered value as the sort, unnamed | a second ask | FIRED — re-sit cli-opus fault 4, cli-sonnet fault 2 (audits/trade-2026-10/re-sit/README.md; "queued") | build now | S |
| SR-18 | Never-fetched beside the total, by category's tab types | an alpha user misled by a total over mostly-never-fetched type | DORM, near: mcp-opus 1 (worst), mcp-sonnet 2 | keep; `tabs --never-fetched` already shipped | S-M |
| SR-19 | One explain over every term of a zero | a second ask after the alpha | DORM (mcp-opus 11, mcp-sonnet 4 first ask) | build as paper cut | S |
| SR-20 | Alternatives for a node | a recorded question `--explain` cannot serve | DORM | keep | L |
| SR-21 | Scope "newest" fetch age names no location | a run takes the newest age for an item's/tab's | DORM, near: 3 release-seat runs read it twice and guessed | build: name the location | S |

## store (7)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| ST-1 | `user.db` and scope taxonomy | first user-scoped kind actually written | SOON (GUI: saved searches, shop template, ratios) | build in GUI slice; absorbs PR-11, SR-1 | M |
| ST-2 | Search-at-scale in store (FTS, lines table) | search persistence park fires | DORM | fold into SR-2 | L |
| ST-3 | Retention of removed items/events/responses (`acq store prune`) | first league end the store lives through, size/time over a number, or search reading removed | FIRED — its third clause, "search reading removed", is met: `acq search --membership removed` reads them (re-read 2026-10-19 at the roadmap's review); 3.30 on 2026-11-13 ends the league too | the prune verb is phase 3's work at the league end, after 0a shows what age costs once the loop is fixed | M |
| ST-4 | Store says "folder" once | a fifth consumer, or one of four getting it wrong | DORM | keep | S |
| ST-5 | Realm segment semantics; PoE2 on character endpoints; inventoryId | observed at a first contact | SOON — PoE2 launch-day first contact 2026-12-11 (note 36 step 6) | fire at launch-day live run (human-run) | S |
| ST-6 | Listing row keeps no raw JSON (non-string name/type) | second reader needing unread grain, or non-string observed | UNOBS (none in 3,427 tabs; owner "park it") | delete | S |
| ST-7 | `store status` `granted`/`unlifted`/`withheld`/`intent` unexplained | a user asks what one counts | DORM, near: all four release-seat debriefs 8 | build: words only | S |

## trade (9)
| id | item | trigger | status | fate | size |
|---|---|---|---|---|---|
| T-1 | Trade translation outbound half and PoE2 | a consumer needing a search sent, or PoE2 on the trade site in use | SOON (PoE2 trade data after 2026-12-11; note 36 step 5 "PoE2 trade-data sitting if the site is up") | build outbound first (PoE1); PoE2 after a sitting | L |
| T-2 | Sat rounds as data in trade-sheet.py | a round whose input cannot be frozen | DORM (gate's `--verify` holds today) | delete | S |
| T-3 | Capture tests rebuild items from the extract | a capture test turning on a key the extract drops | DORM | keep | S |
| T-4 | Map tier table for bases naming no tier | owner registers a source of map tiers | DORM | keep | M |
| T-5 | Named stat value `stat("...")` | seat/question bounding a stat labelled ambiguous | FIRED — 2026-10-17 trade seat, four runs read the `reduced` label as a caveat; owner "mark the trigger as fired ... queue this up" | build (note 37) | M |
| T-6 | "Other reading" (C119) for defence/damage bands and `map_tier` | seat measures a band by hand, or map tier table built | FIRED — re-sit: mcp-opus 21-22, cli-sonnet 9-11, mcp-sonnet 10-11 (README fault row "trigger fired"); deferred past alpha | build; pairs with T-5 | M |
| T-7 | Empty default stat group writes no clause | a second reader looking for it | DORM (by design) | delete, or one report row | S |
| T-8 | User-authored weighted sums | recorded question named total and `sum` cannot serve, or translation meeting a weighted group | DORM | keep | L |
| T-9 | Gem-set level requirement under `req_filters.lvl` | a listed item whose requirement a gem sets | UNOBS (round x asked twice, showed nothing; no site filter selects by gem) | delete | S |

---

# Summary

## Counts (106)
FIRED 5: F-3, SR-12, SR-17, T-5, T-6.
LIKELY-SOON 17: D-5, D-9, D-10, F-2, F-5, F-7, F-10, F-18, PL-3, PL-9, PR-8, SR-1, SR-11, ST-1, ST-3, ST-5, T-1.
UNOBSERVABLE 8: D-15, N-2, PL-1, PL-2, PR-12, SR-9, ST-6, T-9.
WITHDRAWN-CANDIDATE 1: CTX-1.
DORMANT 75, about 20 of them "near": a seat run met the thing and the park waits for a user or a second run (F-1, F-4, F-6, F-12, F-15, F-16, F-17, PL-8, PR-15, PR-16, SR-16, SR-18, SR-21, ST-7, D-4, D-16, D-17).
Total 5 + 17 + 8 + 1 + 75 = 106. Of the 17 LIKELY-SOON, 4 fire at PoE2 1.0 (PL-9, PR-8, ST-5, T-1), 1 at 3.30 (ST-3), 12 at the GUI slice.

The owner's remark "a large number of triggered parks already" is only partly borne out: 5 are FIRED on evidence, and 3 of those (SR-17, T-5, T-6) are already "queued". Another ~20 are "near" (seat-found, trigger needing one more observer).

## Clusters
1. GUI slice (12 parks fire together): D-5, D-9, D-10, F-2, F-5, F-7, F-10, F-18, PL-3, SR-1, SR-11, ST-1, plus D-6 (Tauri packaging) and F-9 as maybe. Decide as one list when the GUI slice opens; note 36 says whether Tauri is revisited is that slice's first question, so D-6/F-9 hang on it.
2. Answers unbounded or too large for the door: F-3 (FIRED), F-11, PR-14 (and F-4's silent drop on the input side). One read-economy ruling (C53) retires three parks.
3. MCP refusals/receipts are the CLI's words or incomplete: F-14, F-15, F-16, F-21, F-12 — one helper (`tool_refusals.rs`) plus tool texts; about a day.
4. Counts nobody understood (words, not logic): PR-15, ST-7, F-1, F-17, SR-7, SR-21, SR-14, PR-16 — eight one-hour fixes to text.
5. Site-as-oracle follow-ups (need a site sitting or trade work): T-5, T-6, SR-16, SR-17, SR-18, SR-19, SR-12, SR-13, T-4.
6. Daemon lifecycle for non-spawning agent hosts: D-4, D-14, F-6, F-20, D-16 — the real-mode MCP story.
7. Windows/Linux platform edges with no observer: D-12, D-13, D-14, D-15, F-8, D-2.
8. Persistence and performance: SR-2, SR-3, ST-2, SR-11, ST-3. SR-2's trigger is at 472-481 of 500 ms.
9. Rot, recommended delete (11): D-7, D-15, N-2, N-3, PL-2, PL-4, PR-7, SR-9, ST-6, T-2, T-9 (and T-7, PL-1 fold). Plus CTX-1 rewritten as a ruling.

## Fires at the game events
- 3.30 (2026-11-13): ST-3 retention (the league the store holds ends); SR-2 re-measure of the load as the corpus grows; PR-6 batch repricing and SR-8 unique variants are plausible but have no trigger evidence yet.
- PoE2 1.0 (2026-12-11): PL-9 per-realm merge, PR-8 PoE2 currencies, ST-5 realm/character endpoint claims (all three scheduled in note 36 step 6, launch-day first contact), T-1 (PoE2 half; outbound half is independent), F-18 sticky realm becomes pressing, SR-5 realm word in a term possible. README currently says GGG offers no PoE2 stash tabs.

## First ten to build for an agent-forward tool
1. F-3 read economy (FIRED; folds F-11, PR-14): the only fired park every agent run met; answers over the 60 KB door cut cost second calls.
2. F-4 unparseable MCP request gets no reply: an agent hangs; a transport fix.
3. D-4 + F-6 + F-20 the keeper bundle: real-mode MCP hosts spawn per call and the daemon idles out, losing detached results; the README recipe is a workaround.
4. PL-9 per-realm merge: a `policy set` erases the other realm's policy; deadline 2026-12-11.
5. T-5 `stat()`: FIRED, queued by the owner; removes an "ambiguous" caveat all four trade-seat runs read.
6. T-6 other reading: FIRED, three re-sit runs measured the armour band by hand.
7. SR-17 sort value naming: FIRED, hours.
8. SR-16 whether the site lists the item: the worst fault of 6 of 8 seat runs; needs a site sitting (owner action); after the alpha per owner.
9. MCP refusals/receipts bundle (F-14, F-15, F-16, F-21, F-12): one helper, kills a repeated seat stumble.
10. Words bundle (PR-15, PR-16, ST-7, SR-21, F-17, SR-7): eight seat-observed confusions, hours each; PR-16 (rank-to-tab) is the one that can mislead a paste.
Runners-up: PR-8 and ST-5 (launch day, human-run), ST-3 (3.30), PL-8 (one count), PR-13 (S, free).
