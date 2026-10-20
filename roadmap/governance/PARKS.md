# The parks: what stays as a line, what is deleted, what becomes an issue

Every park in `CONTEXT.md` and `decisions/*.md` (104 at `c5535d52`;
the triage's 106 less the two phase 0 built, PL-9 and F-4) has one
fate below. The ids are the triage's (`roadmap/parks-triage.md`). The
migration (brief 1b) deletes every "Parked" section; the text of each
park stays whole at the commit before that step's, cited in its
message.

## 1. Not doing — one line per area, at the foot of its file

A "Not doing" line is a boundary: something someone would propose,
declined until a named trigger. No evidence prose; `bothy.py check`
requires each line to state its trigger ("not until"). Twenty lines.

### daemon (6)
- Priority levels beyond one: not until a second level is in use (C5 keeps the field). *(D-1)*
- Packaging beyond the GitHub archives (an installer, Windows signing, notarization, another channel): not until the owner reopens a channel, or a user is stopped by Gatekeeper on a browser-fetched macOS archive or by SmartScreen on a Windows one. *(D-2)*
- A second installation targeting one world: refused under C82; a consumer that needs one is a design event. *(D-3)*
- keyring 4's per-platform stores in place of keyring 3's backends: not until a sandboxed package or a provisioning-profile bundle. *(D-6)*
- A GUI-hosted daemon: refused; the GUI spawns `acqd` as the CLI does, and phase 4 records the spawn policy. *(D-9)*
- musl static Linux builds below the glibc 2.35 floor: not until a user on an older distribution. *(D-12)*

### frontends (1)
- A standalone queue TUI (`dash` as its own binary): not until a queue-monitoring slice opens. *(F-9)*

### network (1)
- A wire-send budget beside the logical bound (C42): not until a consumer needs it. *(N-1)*

### plans (3)
- Dynamic `--deep` fan-out: not until live evidence that two-cycle reconciliation hurts. *(PL-5)*
- Type-level sync-policy filters: not until a policy author needs an exclusion the parent-covers-children rule cannot express. *(PL-6)*
- A daemon-side check against two applies of one plan racing: not until a duplicate fetch is traced to concurrent applies in a journal; until then both send. *(PL-7)*

### pricing (2)
- The legacy import (C73): not until the owner rules the replace. *(PR-1)*
- Third-party price feeds: not until a currency-ratio or suggest-a-price consumer, and then as a governed surface under C79. *(PR-2)*

### search (4)
- A persisted projection or a totals batch: not until the empty-query load passes 500 ms release (`search/M3-LEDGER.md`); a fired trigger opens an experiment, never persistence by default. *(SR-2, SR-3, ST-2)*
- Whole-corpus export of the derived lines: not until a recorded question the language and the JSON answer cannot serve. *(SR-4)*
- A realm word inside a term: refused under C96; two searches are the workaround. *(SR-5)*
- Mod attribution (affix type, tier): not until a recorded question needs an affix count or a tier; C102 names it out of reach. *(SR-10)*

### trade (3)
- The outbound translation and PoE2 on the trade site: not until a consumer needs a search sent, or PoE2 trade is in use. *(T-1)*
- A map tier table for bases naming no tier: not until the owner registers a source (C79). *(T-4)*
- User-authored weighted sums: not until a recorded question a named total and `sum` cannot serve. *(T-8)*

## 2. Deleted outright

Each with its reason. The owner reads this list before anything is
deleted. The text of every row is whole at `c5535d52`.

| Park | Why it goes |
| --- | --- |
| CTX-1 shop/forum publishing (withdrawn) | becomes the refused ruling C120 in `AGENTS.md`; the render-to-stdout sentence is C120's |
| CTX-2 transcription lint | folded into C110's last clause (same trigger) |
| CTX-3 external tracker | resolved by this reset: issues are the backlog |
| D-7 artifact dependencies (`-Z bindeps`) | a toolchain wish with no consumer; rot |
| D-11 hash-only artifact identity | becomes issue 4 (a false match is a user's report) |
| D-15 `subst` and mapped drives | unobservable: no path from a user to the owner |
| N-2 the invalid-request threshold as a claim | unobservable without breaking the rails; not seen in dozens of rows |
| N-3 token expiry during a limiter hold | harmless: one 401, then C87 refreshes |
| PL-1 coverage advice in `refresh --plan` | folded into issue 22's body |
| PL-2 `aging` in the plan | unobservable ("a cycle estimate that is trusted") |
| PL-4 universal plan grammar | C75 holds the direction and its test |
| PR-7 `~c/o` as a value | the census found zero rows |
| PR-11 "what did I last post" | folded into issue 42 (user-scoped intent) |
| PR-12 render cells no sitting has observed | the mechanism blocks and counts them; nothing is parked |
| PR-14, F-11 | folded into issue 9 (the read economy) |
| SR-1 saved queries | folded into issue 42 |
| SR-9 whether a line can still be made | no source exists; C107 leaves it the user's |
| SR-12 what no sitting has asked | becomes issue 33, its leftovers as a checklist |
| ST-6 a listing row keeps no raw JSON | **the owner parked this by name (2026-09-26)**: none observed in 3,427 tabs; proposed for deletion, his call |
| T-2 sat rounds as data | the gate's `trade-sheet.py --verify` holds what it asked |
| T-3 capture tests rebuilt from the extract | an observation the tests already state |
| T-7 the empty default stat group | by design; `translate.rs` says so |
| T-9 a gem-set level requirement | round x showed nothing; no site filter selects by gem |
| D-4, D-14, F-6, F-20 | folded into issue 1 (the keeper) |
| F-12, F-14, F-15, F-16, F-21 | folded into issue 10 (refusals and receipts) |
| F-17, PR-13, PR-15, PR-16, SR-7, SR-21, ST-7 | folded into issue 11 (the words) |

## 3. Issues

Labels to create: `area:daemon`, `area:frontends`, `area:network`,
`area:plans`, `area:pricing`, `area:search`, `area:store`,
`area:trade`; `trigger:user` (a user's report fires it),
`trigger:seat` (a seat run fires it), `trigger:consumer` (a second
consumer fires it), `trigger:gui` (phase 4 decides it), `trigger:live`
(a live run, a sitting or a game event fires it). A roadmap step that
schedules an issue is named in its body, not a label. Evidence (call
numbers, audit paths, measurements) goes in the body: that is what an
issue is for. Forty-eight issues.

| # | Title | Labels | Body (one or two sentences; the evidence) | Step |
| --- | --- | --- | --- | --- |
| 1 | A keeper for frontends that never spawn | daemon, consumer | The foreground `acqd` contract: readiness, signals, exit codes, launchd/systemd. Folds: a host whose job forbids breakaway ends the daemon (Windows sshd; `world_windows` asks to break away); the MCP's `job_result` and `job_status` with no daemon (over `Persisted::read_job`, as the CLI's since 2026-10-13; the old park also named an `auth_status` tool the MCP does not have); the no-daemon remedy's daemon has no keyring and answers not_logged_in. Evidence: the release seat (acq-mcp connects per call, idle exit), the README recipe `e8f21169`, the rerun worker 2026-10-19. | phase 4's first pre-work |
| 2 | Correlation ids and multiplexed requests on one connection | daemon, gui | C85 amended when a GUI shows it needs them. | phase 4 |
| 3 | `spawned_by` in `hello` and `daemon status` | daemon, gui | Two spawners on one machine and a human needing to know which. | phase 4 |
| 4 | Hash-only artifact identity | daemon, user | A false match from file identity, or a second install location (C84). | — |
| 5 | An elevated daemon serves the same user's unelevated clients | daemon, user | Measured 2026-10-15: the pipe's DACL names the user and carries no integrity label; a high mandatory label when `acqd` runs elevated (C112 amended) when the owner rules elevation a boundary or one is met in use. | — |
| 6 | A job verb spawns a daemon that then refuses for want of a login | daemon, user | C3 keeps the spawn and the refusal says so; the release seat's CLI runs called it the fault (cli-opus call 101, cli-sonnet 92–93). Reopen on a user's report. | — |
| 7 | A GGG error handed on in the daemon's words gets a verb-level hint | daemon, user | `GET /stash/Standard/nosuchtab returned 404` (cli-opus call 89): add "acq stashes lists the tabs". Invariant 2 keeps the error as truth. | — |
| 8 | `world.rs` out of the store crate | daemon, consumer | When a frontend must not link SQLite. | — |
| 9 | The read economy: summary by default, filters, bounded detail | frontends, seat | Fired: the rerun's `left_out` still cut at 160 KB (`d79c4ca7`, mcp-opus-2 call 37). Folds a container's `price show` carrying every item (22–174 KB: mcp-sonnet 25, cli-sonnet 71) and the shop document's unbounded `posted`/`freshness` with `page` dropping counts. One bound under C53 at the shared layer. Measured at the release seat: `audits/release-2026-10/seat/README.md`, count 1. | 2g |
| 10 | One refusal and receipt shape across the MCP's tools | frontends, seat | Folds: refusals naming the CLI's flags (`--realm`, `--if-revision`); a clear's receipt carrying `written: null` and not the tombstone's revision, and `price_clear`'s undo text (mcp-opus-2 calls 39–53, debrief 8); `price_show`'s `manual.revision` being a container's when inherited (mcp-opus-2 call 31); `set_sync_policy`'s conflict without its revision, job tools' malformed arguments as rmcp errors (the rerun worker 2026-10-19; 0b's review finding 1); ages as epoch seconds alone. One helper sorts every refusal (today `refusal()` in `acquisition-mcp/src/main.rs`). | 2g |
| 11 | The words nobody understood | frontends, pricing, store, search, seat | Text fixes: `price status`'s `residue` and overlapping counts, and no listing of invalid notes (cli-opus 49, mcp-opus 19, mcp-sonnet 20, cli-opus-2 55, mcp-opus-2 21); the shop page's `StashN` rank with no map to tab names (cli-opus 68, cli-sonnet 62, cli-opus-2 68, mcp-opus-2 38); `store status`'s `granted`, `unlifted`, `withheld`, `intent` (all four debriefs 8); the scope's "newest" fetch age naming no location (cli-opus 35, cli-sonnet 20 and 25, cli-opus-2 34); `never_fetched_by_type` listing totals (mcp-opus-2 call 7); `sum 0 · N lacking` reading as a measured sum; `price list`'s refusal for a container with its own row (`7571a570`). | after 2a |
| 12 | A term's count beside the total reads as a contradiction | frontends, search, seat | Each term is counted alone over every league in scope (cli-opus 26, cli-sonnet 14–15); every re-sit run re-read it once and none spent a call (`audits/trade-2026-10/re-sit/`, count 1). Fire: a run spends a call on the terms block. | — |
| 13 | Queue-management UI | frontends, gui | Drag to reorder, per-job progress. | phase 4 |
| 14 | Results over a subtree as events or a `results` verb | frontends, gui | Today `list` then `result` per child: hundreds for a map tab. | phase 4 |
| 15 | A watch that waits for the daemon to return | frontends, gui | `acq jobs --watch` ends when the daemon goes (an observer never spawns, C10); the GUI's subscriber waits. | phase 4 |
| 16 | Printed commands are POSIX-quoted | frontends, user | `'\''` does not read back in PowerShell; quote for the shell that will run them. | — |
| 17 | The structured-error taxonomy beyond the additive `kind` | frontends, gui | C85's closed `ErrorKind`; the GUI is the validating consumer. | phase 4 |
| 18 | A `store_refused` MCP tool | frontends, user | `acq store refused` has no twin. | — |
| 19 | A sticky default realm and a default league | frontends, gui | Adapters only (C96); persisted, it is user-scoped intent (issue 42). PoE2 1.0 makes realm friction real. | phase 4 |
| 20 | The README's checksum compared by eye | frontends, user | `readme_tour.rs` refuses a download URL that is not an archive. | — |
| 21 | The explicit-selection door (C76) | plans, gui | `--tabs`, `--all`, `--characters` compile to a plan through the one envelope; the GUI's refresh gesture is the first consumer. | phase 4 |
| 22 | A plan says nothing of the tabs the policy leaves unnamed | plans, seat | One count in the summary (`skipped_tabs: []` beside 3,347 tabs of which the policy names 4); read as full coverage (mcp-opus call 37 fault 5; re-sit mcp-opus 34–37, cli-sonnet 20–22). Folds coverage advice for priced locations outside coverage. Deferred past the alpha 2026-10-18. | — |
| 23 | The bulk-exchange item list as a reference table | pricing, user | `reference/exchange-items-2026-09-08.json`, 750 ids in 15 of 22 groups; fire: the owner prices a non-currency item by ratio, or a manual row lands on a stack in a ratio-named public tab. | — |
| 24 | Row-granularity annotation history | pricing, user | Fire: a "since" question the returned prior cannot answer. | — |
| 25 | One change cursor over facts and intent | pricing, consumer | Fire: an agent consumer shows two reads where one would do. | — |
| 26 | Batch pricing by query | pricing, user | Fire: a batch workflow the handles cannot express; likely at a league start. | — |
| 27 | PoE2 currencies and per-realm tags | pricing, live | Rows in the currency table at the PoE2 stash endpoint's first contact. | phase 5 |
| 28 | Currency totals and history | pricing, user | A derivation over facts, when the owner asks. | — |
| 29 | Name to id resolution for price targets | pricing, user | `price set tab:"Maps"`, the rule of C63. | — |
| 30 | Query by example | search, seat | The one-item view prints the item as an editable query. | — |
| 31 | A unique's variant as a derived field | search, user | The Path of Building label (`Pre 3.23.0`, `Current`) under C107; what the fit needs is `spikes/rust-playground/search/legacy/README.md` at `77bca830`; 3.30's balance changes make variants matter. | — |
| 32 | An incremental update of a held corpus | search, gui | Over `item_events`; M5: a resident consumer pays one whole load (492 ms) per committed tab. | phase 4 |
| 33 | What no sitting has asked the site | search, live | A checklist: which number the site compares in a text of three numbers (`stat.text.many`); the three percentile lines and S4; Maven's Invitations; grafts; `scourge-in-sum` and `blighted-base` (`tests/trade-oracle.toml`, under `asks`). Fired twice (rounds v and x). | the next sitting |
| 34 | The twin id the site leaves out of totals | search, live | Reported to GGG 2026-09-26; four `not mimicked` rows move when a sitting shows the site counting it. | — |
| 35 | The answer's words the seats left unruled | search, seat | The list is `spikes/rust-playground/search/OBSERVATIONS.md` at `453a4429`, entries headed "a seat". | — |
| 36 | The scope's `tabs` pointer under two `league=` terms | search, seat | `fixed_league` names the league with the most never-fetched tabs, neither term's. | — |
| 37 | Whether the site can list an item, as a field and a row's mark | search, live | Needs a sitting on what the site lists; the worst fault of six of eight seat runs (cli-opus-2 13–17, mcp-opus-2 30–32; re-sit cli-opus 1, mcp-sonnet 1). GGG's own flags are fields since C117. | 2h |
| 38 | The query's own filtered value as the sort, unnamed | search, seat | Fired at the re-sit (cli-opus fault 4, cli-sonnet fault 2). | 2h |
| 39 | The never fetched beside the total, by the query's category's tab types | search, user | mcp-opus 1 (its worst), mcp-sonnet 2; `tabs --never-fetched` shipped. | — |
| 40 | One explain over every term of a zero answer | search, user | mcp-opus 11, mcp-sonnet 4. | — |
| 41 | Alternatives for a node | search, user | What a query matches with one leaf replaced; the normal evaluator over the replaced query. | — |
| 42 | User-scoped intent: `user.db` and its scope taxonomy | store, gui | Saved queries, names and tags; the shop template; currency ratios; a last-posted basis. Fire: the first user-scoped kind actually written. | phase 4 |
| 43 | Retention: `acq store prune` | store, live | Fired: `acq search --membership removed` reads removed items; 3.30 ends the league. Measured 2026-09-20: `responses` 1,126 rows 2.5 MB, `item_events` 22,751 rows 2.8 MB; 0a's ruling keeps the removed-heavy copy an M3 row. | phase 3 |
| 44 | The store says "folder" once | store, consumer | `Folder` is compared by four consumers each for itself. | — |
| 45 | Realm segment semantics per endpoint; PoE2 on the character endpoints; `inventoryId` | store, live | Claims at the launch-day first contact. | phase 5 |
| 46 | A named stat value, `stat("…")` | trade, seat | Fired 2026-10-17 (`audits/trade-2026-10/seat/`: `reduced` read as a caveat by four runs); the design is note 37 at `<commit>`; the owner asked to queue it. | 2h |
| 47 | An other reading for a bound within one, and for `map_tier` | trade, seat | Fired at the re-sit (mcp-opus 21–22, cli-sonnet 9–11, mcp-sonnet 10–11). | 2h |
| 48 | The price plan and its receipt (C71, C78) | pricing, consumer | Unbuilt direction; fire: a batch a human cannot review row by row (an agent writer, a document of many rows). The mechanism paragraph is note 10 at `<commit>`. | — |

`<commit>` is the commit before the deleting step's, filled in by the
worker.
