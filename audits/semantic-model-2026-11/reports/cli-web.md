# CLI frontend `acq` and its loopback web UI — semantic model as built

Read-only recovery from `crates/acquisition-cli/src/**` (≈20.5k lines). Every claim cites `path:line` (paths relative to `crates/acquisition-cli/src/` unless given whole). "verified" = read in the code; "suspected" = inferred from one read without a second confirming site.

## 1. State

| Value | Lives in | Writer | Reader | Truth / mirror |
| --- | --- | --- | --- | --- |
| `Ctx { frontend, account }` — provider, door, store root, the `--account`/`ACQ_ACCOUNT` selector as typed | process memory, built once | `run` (`main.rs:1852-1855`), `web::web` (`web/mod.rs:241-244`) | every verb | mirror of env + argv |
| `SPAWNED` — "this command started the daemon" | process static `AtomicBool` (`main.rs:1757`) | `note_spawn` (`main.rs:73-77`, `web/mod.rs:1551`, `plan_cmd.rs:489`) | `noting_the_spawn` (`main.rs:1763-1785`), only on `NotLoggedIn` | derived from `Client::spawned()` |
| Printed-command shell (`ACQ_SHELL`) | env | user | `shell_quoted`, `pasted_shell_hint` (`main.rs:1810-1816`) | truth (declaration) |
| `now` | `ACQ_NOW` env or wall clock (`acquisition-store/src/lib.rs:1154-1162`) | harness / OS | every age the text prints (`store_cmd::ago`, `*_text(…, now)`) | truth for ages; read many times per command (see 4a) |
| `Progressor { tty, last, last_print }` | memory, per wait (`main.rs:3059-3063`) | `report` | stderr progress line | throttle state (wall `Instant`) |
| Dash `App { selected, expanded, scroll }` | memory (`dash.rs:33-40`) | key handler (`dash.rs:89-123`) | `draw` | UI-only; every datum rendered is the daemon's `Snap` of the last poll (`dash.rs:130-176`) |
| `rails.json` (world) | file | **the CLI itself** on `daemon reset-tripwire` with no daemon (`main.rs:2485-2507`: `RailsState::reset` + `write`, or `remove_file`) under `Lock::acquire` (`main.rs:2459-2468`) | the next daemon | truth (shared with the daemon) |
| Facts store | SQLite via `Account::facts()` | **the CLI** on `store import` (`store_cmd.rs:1116`, `store.record`) and `store rebuild` (`store_cmd.rs:1062`) | every read verb | truth |
| Web `Web { account: OnceLock<Account>, token, host, origin, form, held: Holder }` | server memory (`web/mod.rs:194-204`) | `web()` once; `served()` lazily (`web/mod.rs:440-446`) | every handler | token/host/origin: truth for the launch; `account`: pinned mirror of the index at first resolve; `held`: cache checked against store+intent revision before every answer (`acquisition-search/src/corpus.rs:837-845, 894-915`) |
| Web page state | URL query (`realm, league, text, trade, view.*, membership, next, saved, saved_revision, selected, tab[], character[], deep, id, expand, q`), POST bodies (`plan` envelope `web/mod.rs:1517-1521`; saved `id, if_revision, name, text, realm, membership`), `<meta name="basis">` (`web/page.rs:129-142`), `<meta http-equiv=refresh>` (`web/page.rs:120-123`) | handlers | browser → handlers | URL and basis are the page's identity (C16); the hidden plan envelope is a copy of the plan document |
| Browser-held transient state | DOM: filter boxes (`assets/filter.js`), `#stat-list`, `#stat-N-text` labels, `#stat-said` (`assets/picker.js:30-41`); closure: picker's `asked` generation (`assets/picker.js:11,13`) | scripts | scripts | not in the URL; discarded on reload (documented `web/mod.rs:88-92`) |

## 2. Transitions

CLI commands are one-shot; the state machines are:

**Job wait** (`wait_for_job` → `acquisition_verbs::jobs::wait`, `main.rs:3045-3054`): trigger submit; effect progress lines then `print_result`. `print_result` (`main.rs:3223-3240`) re-reads `client.status(id)` *after* the result to decide the apply-success rendering: same result renders two ways depending on whether status is still retained (order-dependent on daemon retention, verified).

**`jobs --watch`** (`main.rs:2954-3027`): subscribe → attach → pid check → snapshot → loop{event → re-read `status` → print | resync → restart | EOF → restart}. Every print is from a read, never the hint (`main.rs:2992-3010`), matching C85. Commutation: two events for one job (waiting→running, running→done) re-read after both landed print `-> done` twice and never `running` — order-dependent, visible, by design. `--state` narrows snapshots only (`Listed::read(&mut client, view, all)` `main.rs:2979`); event lines ignore `view` and `all` (`main.rs:2989-3011`) — verified.

**Login** (`main.rs:2618-2673`): `AuthStart` → print URL (two JSON documents) → poll `AuthStatus` every 500 ms until `!pending`; `login_error` wins over `login_ok`.

**`daemon stop`** (`main.rs:2548-2604`): read `AuthStatus` for non-keyring sessions → `stop_any_at`. Not idempotent across the pair: a login landing between the two reads prints `sessions_ended` without it (TOCTOU, see 3).

**`daemon reset-tripwire` with no daemon** (`main.rs:2443-2544`): lock → read file → `reset()` → write or remove. Idempotent (second run: `held.reset()` false → `continue` → "no rails halt is persisted"). "reset then daemon start" vs "daemon start then reset": the lock makes the second order a refusal (`main.rs:2460-2468`), verified.

**Web account pinning** (`web/mod.rs:440-446`): `OnceLock::get_or_init`; "resolve then login" vs "login then resolve" pins different accounts for the server's life; documented. Idempotent after the first.

**Web saved writes** (`web/mod.rs:1762-1857`): POST under `Expect::or(if_revision, Create)` / `Exactly`; never `Current` (C168). Reload after 303 re-GETs the list, so a write is not re-applied (idempotent by redirect). "open at r, write elsewhere, save over" → conflict (`c168_a_stale_page…` pins it).

**Web apply** (`web/mod.rs:1536-1564`): POST envelope → `Opened::apply`. "apply twice" → `Spent` page (C111). "edit the hidden field, apply" → whatever `RefreshPlan::from_value` accepts, with the daemon's admission checks; same trust as `--apply=FILE`.

Commuting pairs: `price set` then `saved set` (different files). Order-dependent: `policy set` then `refresh --plan` (plan carries the revision); `saved_open` then `saved_clear` then the opened page's "Save over" (→ `Gone`, new row created, `web/page.rs:450-456`).

## 3. Dependencies (effect → precondition → where checked)

| Effect | Precondition | Checked at |
| --- | --- | --- |
| Any GGG send from the CLI | goes through `connect`/`attach` → verbs door; the CLI never links reqwest (`Cargo.toml`) | `tools/arch-check.sh:412-414` (door grep), edges |
| `process::exit(2)` for a usage error | `--json` typed before a bare `--` | `main.rs:1139-1142` reads **`std::env::args()`**, which panics on non-UTF-8 argv; clap fed `args_os` and refused it as `InvalidUtf8`, so a non-UTF-8 argument reaches `refuse_usage` and panics instead of exiting 2 (C47 breach; verified from std docs, suspected unexercised) |
| `exit 1` | any `Err` from `run`, or `AlreadyReported` | `main.rs:1791-1805`; `AlreadyReported` raised at `main.rs:1928,3259`, `plan_cmd.rs:562,586,591`, `search_cmd.rs:282,730` — scattered, no table |
| `exit 2` vs clap's own code | text mode uses `e.exit_code()` (`main.rs:1159`), JSON hard-codes 2 (`main.rs:1164`); clap returns 1 for `Io`/`Format` kinds | two spellings; suspected divergence |
| `exit 0` from `diagnose` whatever it found | by design | `diagnose_cmd.rs:16-34` |
| `rails.json` write | no daemon holds the world lock | `Lock::acquire` held across read+write (`main.rs:2459-2495`); same process, no TOCTOU |
| Facts write (`store import`/`rebuild`) | account resolved | `store_cmd.rs:18-20`; **no daemon coordination**: a running daemon writing the same SQLite file is not checked (WAL relied on, module doc `store_cmd.rs:1-2`) |
| `sessions_ended` printed by `daemon stop` | the sessions read are the ones stopped | read (`main.rs:2554-2563`) and stop (`main.rs:2567`) are two connections, two instants — TOCTOU across daemon requests |
| Web `apply`/`fetch_leagues` submit as the served account | `served()` | `web/mod.rs:1430,1547`; the request's own account is never read from the browser (good) |
| Web route runs | Host == bound address, Origin absent or own | `guard` (`web/mod.rs:347-362`), before every route |
| Web page holds a corpus answer | corpus current against store revision + intent revision | `Holder::corpus` under its mutex (`corpus.rs:894-915`); the answer is computed after the lock drops, on a corpus that may have moved — accepted (basis is labelled) |
| Web jobs page's rails text | daemon still up after the jobs read | `web/mod.rs:1569` then `1585`: two observes; a daemon gone between prints an empty rails block silently |

## 4. Findings

### 4a. Nondeterminism

1. **`now` is read many times per command** (verified): `answer_text_of`, `answer_head`, `answer_tail` each call `acquisition_store::now()` (`search_cmd.rs:1031,1048,1541`); `tabs` page calls it four times (`web/mod.rs:1197,1209,1234,1265`); `web/page.rs:886` and `:1707` again. Under wall clock a second boundary between calls prints two ages for one document; under `ACQ_NOW` it is pinned. Text only; JSON ages are the verbs' (`aged`, `Listed::of`).
2. **`print_result` renders by retention state** (`main.rs:3229-3238`, verified): the apply-success summary prints only if `client.status(id)` still answers; after a daemon restart the same outcome prints as a JSON payload dump in text mode.
3. **`jobs --watch` prints the re-read, so transitions can be skipped or repeated** (`main.rs:2992-3010`, verified) — correct under C85, but the pid check (`main.rs:2974`) runs once per sequence; a restart mid-stream is detected only by EOF.
4. **`daemon stop` sessions list** (`main.rs:2554-2567`): two requests on two connections (TOCTOU above).
5. **Web `jobs` page** two observes (`web/mod.rs:1569,1585`); **web `home`/`tabs` league marks** join `league_list` and `LeaguesHeld::read` from two store reads (`web/mod.rs:652-682`) with no common basis — "Not doing" acknowledges counts, not the on-disk mark.
6. **Web `served()` race** (`web/mod.rs:440-446`): two first requests racing resolve independently; `get_or_init` keeps one. Documented.
7. **Progress line count depends on wall time** (`PROGRESS_EVERY`, `main.rs:3087-3099`): a captured run prints a different number of lines per run. Accepted UX.
8. `row_shows`' `HashMap`s (`search_cmd.rs:1420-1440`) feed only max/lookup; output order follows `parts` — **deterministic**, verified.
9. **`demo`** prints ANSI cursor escapes unconditionally (`main.rs:3526-3533`) — breaks C53 "escapes only on a tty"; mock-only.

### 4b. Invariants enforced by hope

1. **The door grep misses the CLI's own writes to world and facts** (`tools/arch-check.sh:412`: `Index::load|Store::open|Annotations::open_for|User::open_for|plan_refresh|check_spendable|apply_params_of|\.quote\(|Persisted::observe|Client::connect`). Not in it: `World::observe_at`, `Lock::acquire`, `RailsState::{reset,write}`, `remove_file` on the rails path (`main.rs:2452-2507`); `Store::record` and `Store::rebuild` from a frontend (`store_cmd.rs:1062,1116`); `Client::observe_at`/`stop_any_at` (`main.rs:2421,2555,2567`); `Subscription::observe_at` (`main.rs:2967`). The grep holds "no frontend opens a store or picks an account" (every store open goes through `Account` — verified: `price_cmd.rs:54`, `search_cmd.rs:686-687`, `store_cmd.rs:19`, `shop_cmd.rs:64`, `web/mod.rs:642,1133,1189`); it does not hold "no frontend mutates facts or world state". C34 ("effects through the daemon") is breached by `store import` structurally and no check names it — verified.
2. **Search evaluation happens in the frontend** (`request.corpus(&store,&intent)` + `answer(&corpus,&request)`: `search_cmd.rs:419-423,439-440,611,632`; `web/mod.rs:643-644`; `Tabs::read` `store_cmd.rs:342`, `web/mod.rs:1190`; `show` `search_cmd.rs:672-674`, `web/mod.rs:1134`). C46 permits it (semantics is the search crate's), but the MCP does the same, so "one evaluator" rests on both adapters calling the same function — held by `pricing_tools.rs`-style tests for pricing, by nothing for search's `Request::corpus`+`answer` pairing (suspected).
3. **C16 on scripts is lexical** (`tests/web.rs:2445`); `picker.js` keeps `asked` and writes `#stat-N-text` labels that nothing clears when the reader empties the input (`assets/picker.js:30-41`: no `input` listener on `.stat-id`) — a stale label beside an empty row; no test reaches the script (checklist rows acknowledge).
4. **C2, C4, C15, C48, C79, C120 "Pinned: nothing yet"** — in this area C120 is held only by the absence of any outward client (no reqwest in the CLI's manifest; `shop render` writes stdout only, `shop_cmd.rs:55-135`) — structural via C1's edge, not by a test naming C120.
5. **"Text is a function of JSON" (C53)** is held per verb by goldens, but the structure allows text-only fields (4d) with no lint.
6. **"callers must" comments**: `web/mod.rs:335-337` (borrow across await), `web/mod.rs:1704-1708` (refuse from the row read, not a second read), `search_cmd.rs:607` ("said before the store is read") — discipline only.
7. **`refuse_usage` reads `std::env::args()`** (`main.rs:1139`, also `main.rs:1813`) — C47 held by hope for non-UTF-8 argv (4a/3 above).

### 4c. One fact in two places

1. **Plan ages** (verified): the verbs compute `PlanAges` for every plan (`acquisition-verbs/src/refresh.rs:94-121`); the CLI destructures `Planned { .. }` dropping `ages` (`plan_cmd.rs:334-342`) and re-derives every age itself: `dur(plan.generated_at - l.fetched_at)` (`plan_cmd.rs:805-820, 853-862`), `age::ago(now, generated_at)` (`plan_cmd.rs:823,881`). Two interpreters of the same epochs; the MCP shows the verbs' words, the CLI its own. `acq refresh --plan --json` prints the envelope alone (`plan_cmd.rs:376`) with no `_age` beside `generated_at`/`fetched_at` — C163 unmet on this surface.
2. **`version`**: JSON is `version_document()` (`diagnose.rs:142-158`: `acqd{path,len,mtime_ns}`, `acqd_absent`); text is built separately from `sibling()` (`main.rs:2312-2324`) — a second maker; `mtime_ns` JSON-only.
3. **`auth status` with no daemon**: JSON hand-picks four fields via `json!` (`store_cmd.rs:57-67`, keys alphabetical, `running:false`), text prints keyring words (`store_cmd.rs:71-88`); `acq accounts` prints the verbs' `Accounts` document (`store_cmd.rs:97-99`) — two makers of the account list.
4. **`auth logout --json` `account`** is `cx.account` as typed (`main.rs:1920`), not the account logged out; `null` when implied. Text names no account.
5. **Web `refresh` JSON** is `json!({"tabs": listed, "characters": characters})` (`web/mod.rs:1232`): through `serde_json::Value` the structs' declared order is lost (workspace `serde_json` has no `preserve_order`, `Cargo.toml:26`), so the page's JSON is alphabetical where `acq tabs --json` (`store_cmd.rs:363`, struct) is counts-first (C115). Same for `daemon status --json` (`DaemonReport::document`, `acquisition-verbs/src/daemon.rs:89-119`, `Value`) and every refusal (`refusal::document` builds a `Map`, `refusal.rs:229`) — the checklist's key-order row names the refusal as fixed, but it is still a `Map`.
6. **Dash job counts** (`dash.rs:600-609`) and job order (`dash.rs:616-621`) are derived in the frontend from `Snap.jobs`, while `acq jobs` reads the daemon's `ListCounts`. Two counters; the dashboard's `jobs` is `self.job_list()` whole (`acquisition-daemon/src/daemon/server.rs:459`), so `acq dash --json` is unbounded (checklist "unbounded by default", suspected re: `job_list` bound).
7. **`store events` defaults** differ by mode: text default is the summary, JSON default the event list (`store_cmd.rs:888`), documented as the one divergence.
8. **Web `leagues` "tabs on disk" mark** (4a/5): joined from two documents in the page layer (`web/mod.rs:675-678`) — a page composing.

### 4d. Work that never reaches the screen

- `Planned.ages` dropped by the CLI (`plan_cmd.rs:334-342,450-455`) — computed, never printed on this surface (verified).
- `RefreshAction::FetchCharacter { league }` ignored (`plan_cmd.rs:1329`, `let _ = league`).
- Web `apply`: a `Submitted` apply's job id is dropped (`web/mod.rs:1557` redirects); the `wants_json` arm comes **after** it, so a JSON client never receives the `Apply::Submitted` document (verified by arm order `web/mod.rs:1557-1558`). `fetch_leagues` drops the job id too (`web/mod.rs:1424-1432`).
- Web `search` JSON drops `opened` (saved-from) and `carried` (realm notice) (`web/mod.rs:957`) — text-only facts; `saved`/`saved_revision` reach only the HTML.
- Text-only: `accounts` store size from `fs::metadata` (`store_cmd.rs:111`); `auth status` "other accounts" (`main.rs:1866-1876`); every `next:` command line; `tabs` `leagues:` line in text while `--json` carries `leagues_held` (fine, same doc).
- JSON-only: `daemon stop` `sessions_ended` when the daemon is not ours (`main.rs:2579` vs `2582-2593`); `Job::Absent.next`; offers' `argv`.
- `reset-tripwire --json` reports `cleared.first()` while text loops `cleared` (`main.rs:2518-2543`); `candidates` is one path (`main.rs:2471-2473`), so the loop and the `Vec` are dead generality.
- `Terminal::request` always returns `tool: None` (`main.rs:1465,1468,1473`) — the tuple's second field is the MCP's.
- `Repeat::None` for `Terminal::writing()` makes `Way::Again` unreachable there (`main.rs:1492-1498`).
- `print_persisted_queue` on a `Jobs::Live` returns an empty string (`main.rs:102-104`) — unreachable arm (callers match `Absent` first, `main.rs:2333-2335`).

## 5. Refactors

**PROVABLY EQUIVALENT**
- Replace the CLI's plan-age derivations (`plan_cmd.rs:805-820,853-862,823,881`) with `Planned.ages`: `PlanAges` is built from the same epochs by the same `age::span`/`age::ago`; equality is a pure-function property over `(generated_at, fetched_at, now)`, checkable by one table test; `--plan=FILE` needs `Planned::aged` run on the parsed plan.
- Centralise exit codes: an `enum Exit { Ok, Failed, Usage }` → `i32` at `main.rs:1159,1164,1804`; the type check establishes the three values; `AlreadyReported` keeps its sites.
- `version` text from `version_document()` fields (`main.rs:2312-2324`): the JSON is already the superset; text becomes a projection (golden `seat_faults.rs:3480` pins the provider word).
- Build web `refresh` JSON as a `#[derive(Serialize)] struct { tabs, characters }` instead of `json!` (`web/mod.rs:1232`): key order becomes declared; `tests/web.rs:1682` compares documents by value, unaffected.
- `reset-tripwire`: `candidates` → one `Option<PathBuf>`, `cleared` → `Option<(PathBuf,bool)>`; the loop has exactly one iteration (`main.rs:2471-2473`).

**TRUST ME** (smallest breaking input)
- Filter `jobs --watch` event lines by `--state`: differs when a `--state open` watcher sees `running → done`; today it prints the `done` line, after the change nothing — a reader relying on seeing the completion loses it.
- Swap `apply`'s match arms so JSON gets the `Submitted` document (`web/mod.rs:1557-1558`): a browser form posting with `Accept: application/json` (none today) would get JSON instead of the 303; the page seat's browser never sends it.
- Let `print_result` decide the apply rendering from the result (e.g. `payload["child_jobs"]` present) instead of `client.status(id)`: differs for a `submit apply` whose payload lacks `child_jobs` (a mock-only `sleep` wrapped as apply) — today renders as a dump, after as a summary of "0 requests".
- Read argv as `args_os()` in `refuse_usage`/`pasted_shell_hint` with lossy conversion: differs only for a non-UTF-8 argument, where today the process panics and after it exits 2 with `kind: usage`.
- Make `daemon stop` ask the daemon to report ended sessions in its stop reply instead of a prior `AuthStatus`: differs when a login lands between the two reads (today: omitted from `sessions_ended`).
- Merge `auth_status_absent`'s JSON into the `Accounts` document plus `running:false`: differs on field set (`store` path and `left_out` appear; keys reorder) for any script reading the absent form.
- Clear `#stat-N-text` when its input is emptied (`picker.js`): differs when a reader clears a picked id by hand and reads the label — today the old stat text stays.

**Which printed commands are run by a test** (for (g)): run as printed — price receipts/undos/status next steps (`tests/price_json.rs:1437,1745,2610`; `tests/seat_faults.rs:3542,3697,3728,3753,3800`), saved receipts/refusals (`tests/saved_json.rs:252`), policy undo (`tests/plan_json.rs:405`), logout retry (`tests/error_kind.rs:156`), `--describe=~` example and variable-setting lines through each shell (`tests/error_kind.rs:303,339`), search routes/continuations/offers (`tests/search_json.rs:216,333,2903`), `store characters/events/refused` "all" commands (`tests/store_answers.rs:69,131,279`). **Not run**: README tour lines are checked against `--help` only (`tests/readme_tour.rs:137-181`, `every_tour_line_resolves_under_clap`), contrary to `REVIEW-CHECKLIST.md:113`'s "runs every line"; `no_tabs_text`'s `acq stashes`/`acq refresh --all` (`store_cmd.rs:174-175`), `wait_hint`'s `acq result N --wait` (`plan_cmd.rs:598-603`), `tabs_command` (`printed.rs:175`) beyond parsing, `mismatch_remedy`/`rails_text`/`persisted_queue_text` backticked lines (`main.rs:164-170,2863-2918,105-131`) — static text, never executed (verified absence in the tests listed; suspected no other file runs them).
