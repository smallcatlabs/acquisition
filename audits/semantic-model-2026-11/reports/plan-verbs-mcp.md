# Report: planner, pricing, verbs, MCP (read-only recovery)

Crates read: `crates/acquisition-plan/src/{lib,listing,price,pricing,shop,game_side}.rs`, `crates/acquisition-verbs/src/*.rs`, `crates/acquisition-mcp/src/{main,transport}.rs`, plus the store seams they call (`snapshot.rs`, `annotations.rs`, `lib.rs`). Every `path:line` is in `/home/user/acquisition`.

## 1. State

| Value | Lives | Written by | Read by | Truth or mirror |
| --- | --- | --- | --- | --- |
| Sync policy row (`SYNC_POLICY_*`) | intent SQLite, per-uuid file | `put_sync_policy_over` only (`acquisition-plan/src/lib.rs:1010`), via `Account::set_policy` (`verbs/src/frontend.rs:464`) | compile (`lib.rs:1989`), `check_spendable` (`lib.rs:1612`), `plan_notes`, `UncoveredTabs::of`, shop `freshness` | truth (CAS, revision) |
| Buyout rows | intent SQLite | `price::set_buyout`/`clear_buyout` (`plan/src/price.rs:805,837`) via `pricing::set/clear` (`pricing.rs:1073,1131`) | `listing::resolve` (`listing.rs:1752`) and nothing else | truth |
| Saved searches | `user.db` (world-scoped) | `saved::set/clear` (`verbs/src/saved.rs:337,509`) | `saved::list/show/request_of`; MCP `saved_into` (`mcp/src/main.rs`) | truth; its `revision` is `SUM(revision)` over the file (`store/src/annotations.rs:1311`) |
| `RefreshPlan` envelope | memory / a JSON file the user keeps / the MCP's call argument | `compile`/`plan_selection` (`lib.rs:2179,2005`); `with_quote` makes a new value | `apply` (`verbs/src/refresh.rs:539`), CLI `read_plan` (`cli/src/plan_cmd.rs:306`), MCP `apply_plan` (`main.rs:2111`) | derived from a snapshot; immutable by construction (consuming `with_quote`, strict `from_value`) |
| `basis.snapshot_response`, `basis.policy_revision`, `basis.selection` | in the envelope | compile | `unspent` (by response), `check_spendable` (by revision) | mirror of store state at compile |
| `PlanAges`, `PlanNotes`, `UncoveredTabs`, `quote_note` | memory, beside the envelope (`Planned`, `refresh.rs:34`) | `Opened::plan` (`refresh.rs:503`) | both frontends' JSON | derived at answer time, never in the envelope |
| `Status.last_event` watermark | memory, `#[serde(skip)]` (`store/src/lib.rs:983`) | `Store::status` at submit (`refresh.rs:620`) | `StoreChanges::since` (`refresh.rs:244`) | mirror of `MAX(item_events.id)` at submit |
| `Submitted.before` (whole `Status`) | memory, `#[serde(skip)]` | `apply` | `landed` (CLI only) | mirror; dropped on the MCP (§4d) |
| Held search corpus | MCP process memory, `Holder` (`search/src/corpus.rs:873`) | `Holder::corpus` on miss | `search` tool | cache, revalidated per call against `(response id, at, user_version)` + intent `SUM(revision)` (`corpus.rs:926`) |
| Daemon job list (`JobInfo`) | daemon memory | daemon | `unspent` via `jobs::every` (`refresh.rs:566`), `wait`, `tally` | truth for "queued" spending |
| Effective price, relation, counts | memory per read | `listing::resolve` | `price_*`, `shop_render`, search `price.rs:211` | derivation (C64) |
| Redactor placeholders / token numbering | memory per diagnosis | `Redactor::new/redact` (`verbs/src/diagnose.rs:651,732`) | `diagnose` output | derived |
| `acquisition_store::now()` | process (`ACQ_NOW` or system clock, `store/src/lib.rs:1154`) | — | every verb; planner takes it as a parameter | input |

## 2. Transitions

**Plan lifecycle** (`compile → [quote] → apply → landed`):

| Transition | Trigger | Precondition (checked where) | Effect |
| --- | --- | --- | --- |
| compile | `Opened::compile/plan` | policy row parses (`SyncPolicy::from_value`), covers (realm, league) (`lib.rs:2187`) | a `RefreshPlan` with `basis.policy_revision = row.revision`, `snapshot_response = MAX(responses.id)` (`snapshot.rs:457`) |
| compile selection | `plan_selection` | realm takes tabs (`lib.rs:2013`) | plan with `policy_revision: None`, listing action always first (`lib.rs:2073`), `max_age_seconds: 0` |
| quote | `quote_within` | daemon observed compatible within 5 s (`refresh.rs:346`) | `with_quote` checks provider/account/work tuples/scope sums (`lib.rs:1454`); else `quote_note` |
| spend gate | `apply` | provider, uuid, then a *fresh SQL read* of the policy row (`lib.rs:1612`; `annotations.rs:1193` is a query per call, not a cached handle) | `Fault::Replan`/`Spend` before any connect (pinned `documents.rs:385`) |
| unspent | `apply` | fetch landed with `response_id > basis.snapshot_response` (`lib.rs:1690`) or an open job/apply of the account holds the same `Endpoint` (`lib.rs:1700-1715`) | split into `unspent`/`spent` |
| connect+re-read | `apply` | `unspent` non-empty (`refresh.rs:560`) | `jobs::every(Open)`, snapshot read again, `unspent` recomputed |
| submit | `apply` | after `before = status()` (`refresh.rs:620`) | one `apply` job with exactly `apply_params_of(&unspent, max_requests)` (`lib.rs:1779`) |
| landed | CLI only | job terminal (`wait`, 500 ms poll) | `StoreChanges::since(before)`, `remaining` recompiles with `now()` |

**Intent writes**: policy `Merge`/`Replace` (`lib.rs:1010`); price set/clear under `Expect::{Exactly,Current,Create}` (`pricing.rs:717`); saved set/clear likewise. All land through `Annotations::put/delete` under `BEGIN IMMEDIATE` CAS (`annotations.rs:1059-1101`).

**Commute / idempotent / order-dependent (verified from code):**

- IDEMPOTENT: `apply` of one plan after its children landed → `Apply::Spent` with `applied:false` (every action's `response_id > watermark`): `lib.rs:1690`, pinned `documents.rs:585`. Replaying while the first apply is still open → spent by `Queued` (`lib.rs:1711` reads the open apply's `params.jobs`).
- NOT idempotent (acknowledged in `decisions/plans.md` "Not doing"): two applies of one plan racing between the second `unspent` (`refresh.rs:568`) and `submit` (`refresh.rs:621`) both send.
- ORDER-DEPENDENT: `set_policy` then `apply(plan)` → `PolicyMoved` refusal; `apply` then `set_policy` → the apply runs. Fact drift never refuses (`c44_fact_drift_never_refuses`, `plan/unit/tests/lib.rs:1715`): a tab the listing retired is still sent and "fails its own child honestly".
- ORDER-DEPENDENT: price `set` (create) then `clear` leaves a tombstone; a later create continues the revision sequence (`annotations.rs:1081`), so a create's undo after clear is "create, revision 0" (`price.rs:710`). `clear` then `set` with `Expect::Exactly(old)` → conflict.
- COMMUTE: `price set` on target A and `price set` on target B (different rows; CAS per row). `set_policy` and `price set` commute as writes, but NOT as observed by the MCP corpus: both move `SUM(revision)` → the corpus reloads either way.
- ORDER-DEPENDENT: `Merge` with a realm naming no leagues *removes* that realm (`lib.rs:1046`); `Replace` of the same value stores it minus the empty realm only on a create (`lib.rs:1036-1040`) — Merge-then-Replace and Replace-then-Merge differ in which realms survive.

## 3. Dependencies

| Effect | Precondition | Checked at | TOCTOU |
| --- | --- | --- | --- |
| GGG sends (via the daemon's `apply` job) | policy revision unmoved; action unspent; daemon admits vocabulary | `refresh.rs:543-568` (frontend process) vs. admission in the daemon | yes, by design (C44 "races a concurrent policy write", C111 "two applies racing both send"). A policy edit between `check_spendable` and `submit` spends a revoked plan. |
| `submit` | `client` connected via the door (`frontend.connect`, Agent door never spawns in GGG mode: `frontend.rs:299`) | `refresh.rs:561` | — |
| intent disk write | CAS on `expected_revision` | inside `BEGIN IMMEDIATE` (`annotations.rs:1067`) | none: the read (`at_revision`) and the put are separate statements, but the put re-reads under the write lock (`annotations.rs:1073`) so a moved row is a conflict, never a lost update. `Expect::Current` reads with `get` then puts naming that revision (`frontend.rs:488-512`): same shape, safe. |
| `store_changes` printed | `before` taken at submit | `refresh.rs:620` | counts every event after the watermark, whoever committed it (`refresh.rs:248-266`): a concurrent apply by another frontend, or any ingest, is counted in this apply's `changed:` line. The pinned test (`documents.rs:634`) covers *sequential* same-second applies only. |
| MCP reply line | parses as JSON ≤127 deep, else answered locally | `transport.rs:135` | the writer `select!` (`transport.rs:99`) interleaves rmcp lines and local replies: an answer to a malformed line may overtake an earlier valid request's reply. JSON-RPC permits this; a host assuming in-order replies sees reorder. |
| `acq-mcp` exit code 2 | provider/shell/`ACQ_NOW` knobs valid | `Frontend::from_environment` (`frontend.rs:187-196`) | — |
| `quote` attached | daemon answers within 5 s about exactly this plan | `refresh.rs:361-375` | the quote's `observed_at` is the daemon's; `quote_observed_at_age` computed at a later `now()` |

## 4. Findings

### 4a. Nondeterminism

1. **Verified** — two clocks in one `plan()`: `Opened::plan` calls `acquisition_store::now()` for `generated_at` (`refresh.rs:511`) and again for notes/ages (`refresh.rs:514`). On a second boundary `generated_at_age` reads `1s ago` on a plan just made; `plan_notes`' league note age and the plan's `generated_at` differ. Under `ACQ_NOW` both are fixed, so no test can see it.
2. **Verified** — `StoreChanges::since` counts neighbours. Any event with `id > before.last_event` is attributed to this apply (`refresh.rs:248`), so two frontends applying overlapping windows each report the other's adds/moves. Order A-submit, B-submit, A-land, B-land: A counts B's events, B counts A's that landed after B's status. `events_after` is also capped at 1,000,000 silently (`refresh.rs:250`).
3. **Verified** — `apply`'s "queued" spend depends on the daemon's list at one instant: `jobs::every` (`refresh.rs:566`) then snapshot then submit; a child finishing between the list and the snapshot is caught by the second snapshot read (the doc's claim at `refresh.rs:530-534` holds), but a *new* apply admitted by another frontend after the list is not seen — both send.
4. **Verified** — MCP writer reorder (`transport.rs:99-124`): `select!` between rmcp's output and the local reply queue, so replies to unparsable lines are not ordered against rmcp's replies; the module doc promises only "in the order it came" among local replies.
5. **Verified** — `wait` polls every 500 ms (`verbs/src/jobs.rs:26,92`) and `tally` pages the list newest-first until it passes the parent's id (`jobs.rs:177`): a progress line's child counts are a snapshot across pages, so a child moving between page reads can be counted in two states or none. Progress only; the final result rereads.
6. **Verified** — `wire_estimate`/`check_quote_matches` are pure; the planner reads no clock (grep: no `SystemTime`/`Instant` in plan, verbs or mcp src; `now` is a parameter everywhere in `acquisition-plan`). The only wall-clock door is `acquisition_store::now()` in the verbs and frontends.
7. **Suspected** — `HashMap` use is internal only (`listing.rs:1317,1759`, `shop.rs:650`, `events.rs:295`); outputs are built from `BTreeMap`/sorted vectors. No iteration-order leak found.

### 4b. Invariants enforced by hope

1. **Verified** — C38 "new facts produce a new plan" is *not* enforced anywhere: `check_spendable` gates intent only (`lib.rs:1612-1650`), `unspent` gates by response id, and `c44_fact_drift_never_refuses` pins that a plan whose listing moved is still spendable. The registry words this as intended (C44 amended, C111); the AGENTS.md C38 sentence is wider than the code. A stale plan applied after a listing change sends fetches for retired tab ids and relies on the daemon's 404 handling.
2. **Verified** — the C44 gate's freshness rests on `Annotations::get` being a per-call SQL read (`annotations.rs:1193`). Nothing pins that; a future cached `Annotations` would silently void the gate. The test at `documents.rs:385` comments "re-open to read it", which suggests an author already suspected caching.
3. **Verified** — freshness is a **second interpreter**, by design but unpinned as one: the planner's window/count/experience/league arms (`lib.rs:2271-2495`) and `shop.rs:1158-1240` each compute `now.saturating_sub(seen_at) > window` over the policy's `max_age_seconds`. Shop's `freshness` reads `e.seen_at` per *item entry* where the planner reads the *tab's* `fetched_at`; the two can disagree for an item whose tab was refetched after the item's own row was last seen (a removed-then-restored item). C62's "consumed, not restated" holds for *liveness* (`fetched_at: None` after revival, `CharacterSnapshot::fetched` nulled; `snapshot.rs:91-102`) but the count-disagreement arm (`lib.rs:2444-2449`) and experience arm (`lib.rs:2471-2487`) are planner logic over raw `metadata`/`listed`/`fetched` values. C162 says one derivation; its Pinned is "nothing yet; Unbuilt".
4. **Verified** — "every surface's refusal is one function" is pinned at the verbs layer (`documents.rs:2199`) and the MCP routes `failed`/`refusing`/`priced`/`refused` through `refusal::document`/`of_search` (`main.rs:325-349, 1585-1600, 669-671`). Two constructors escape it: `transport.rs:164` builds `{"error", "kind":"arguments"}` by hand (necessarily: before rmcp), and `main.rs:597-623` emits JSON-RPC `ErrorData` for daemon/door faults — a refusal shape with `data.kind`, not the document. Pinned only by MCP tests, not by `every_refusal_is_one_shape_from_one_function`.
5. **Verified** — `told_as` holds C149 ("one object") by `text.starts_with('{')` after `spoken` rewrote the text (`main.rs:656-663`): a lexical check, and `spoken` is a hand-written JSON tokenizer (`main.rs:1460-1585`) that re-parses with `IgnoredAny` to prove it stayed JSON. Nothing proves it stayed the *same* document minus appended spellings (a string containing `acq auth` inside a `why` key gets " — here, …" appended; `WORDS = ["why","hint"]`, `main.rs:1416`). C53's "no second rendering" is held by a reviewer.
6. **Verified** — `Undo::Set { if_revision: None }` after a clear (`price.rs:706-709`) promises "puts it back (a create…)" but a create lands only where no live row stands; if anything wrote the target between clear and undo the undo conflicts. `every_undo…runs_as_printed` (`cli/tests/price_json.rs`) runs undos immediately, so the gap is unpinned.
7. **Verified** — `policy_undo` offers `PolicySet { if_revision: written.row.revision, value: prior.value }` with `replace: true` (`receipt.rs:200`, `main.rs:500`): correct, but `check_write` on a *prior* policy only checks parse; a prior naming a league the list lacks is put back silently (C126 says that is a note, so fine).
8. **Verified** — `Expect::Current` + `PolicyWrite::Replace` over an unreadable row is refused (`frontend.rs:496-503`) although `put_sync_policy_over` can replace unread rows; the MCP remedy text (`main.rs:265`) says `replace: true` with `if_revision` — the *revision* is what unlocks it, not `replace`. Pinned wording only (`main.rs:2598`).
9. **Verified** — "Pinned: nothing yet" in my area: C71, C73, C75 (unbuilt), C162 (unbuilt). C2/C4/C15/C48 are cross-cutting with nothing pinned; C48's "no cached search service" is held by `Holder` being per-process with a revalidation (`corpus.rs:873-917`), no test names it in this area.
10. **Verified** — `Redactor` is a scanner over prose, not a field redaction: needles are account names/uuids/paths; tokens are found by `TOKEN_KEYS`/`Bearer` marks (`diagnose.rs:514-542`). A token under a key not in the list (e.g. a future `id_token`, or a token inside a URL path) passes. The doc calls it "a belt over both" (`diagnose.rs:19`) — a hope, pinned by unit cases only (`diagnose.rs:871-975`).

### 4c. One fact in two places

1. **Verified** — `Planned.plan.generated_at` (envelope) vs `PlanAges.generated_at_age` (wrapper) and `PlanNotes.character_listing_fetched_at_age` vs `basis.character_listing.fetched_at`: words beside epochs, both derived, one writer. Benign by C163, but made at two `now()` reads (§4a.1).
2. **Verified** — the daemon's open job list vs the plan's `spent` list: `spent` is a snapshot at `refresh.rs:568`; the printed `holding_jobs` may name a job already finished by the time the line is read.
3. **Verified** — tab coverage computed three ways over one rule: `covers_tab(selection, tab)` (`lib.rs:663`), `UncoveredTabs::of` (`refresh.rs:71`), `shop.rs:1222`. All delegate to `Selection::covers_tab` — one interpreter, good.
4. **Verified** — freshness window: planner and shop (§4b.3). Authoritative: the planner (it spends); shop reports.
5. **Verified** — the listing's `effective.kind == "unresolved"` gate appears twice: `listing.rs:1667` (`note_unread && public == Some(true)`) decides; `search/src/price.rs:215` re-tests the same two fields to word the gap. The search *consumes* `listing.effective` (`price.rs:211`) so the price itself has one door (`listing::resolve` → `effective`), but the *reason* has two.
6. **Verified** — `Status.responses` delta vs `events_after` count in `StoreChanges` (`refresh.rs:252`): a response with no item change gives `responses: 1, added: 0`; consistent but two counters.
7. **Verified** — corpus `Basis.snapshot` = highest response id (`store/src/corpus.rs:320`). `Store::rebuild` (`lib.rs:2640`) rewrites item columns with no response row: a held MCP corpus survives an `acq store rebuild` and serves pre-rebuild derived columns until the next response lands. Suspected impact: low (the search derives from the stored body, not the columns), unverified.

### 4d. Work that never reaches the screen

1. **Verified** — `Submitted.before` (a full `Store::status()` with ~15 COUNT queries, `lib.rs:2349`) is computed on every apply (`refresh.rs:620`) and, on the MCP, dropped: `apply_plan` returns `Apply::Submitted` whose `before` is `#[serde(skip)]` and never calls `landed` (`main.rs:2111-2118`). `store_changes` exists only on the CLI.
2. **Verified** — `Apply::Nothing`/`Apply::Spent` carry `applied: false, requests: 0` at every construction (`refresh.rs:553-558, 596-606`): constants serialized as fields.
3. **Verified** — `max_age_seconds: 0` on every selection plan (`lib.rs:2049`): the field is the policy's window; a selection has none. Readers (the plan text) print it.
4. **Verified** — `PlanNotes` for a selection plan is always `default()` (`refresh.rs:535-538`); `Planned.uncovered_tabs` is `None` on selection and file plans — fields that are `skip_serializing_if` so invisible, but computed paths exist for them in `plan()` only.
5. **Verified** — `Way::RefreshApply` is never spelled on the MCP (`main.rs:466`) and the CLI's `apply_plan` ignores `Skipped` on `Submitted` in text mode (prints `requests`/`spent` only, `plan_cmd.rs:508-518`).
6. **Verified** — `SpentBy::Fetched.fetched_at_age` is `None` until `Spent::aged` (`lib.rs:1741`); every production path calls `aged`, so the `None` arm is unreachable in output.
7. **Verified** — `unspent` computes `request = Endpoint::from_job(kind,&params)` twice per action (`lib.rs:1687,1693`); `Listed::read` reads `counts` from the last page only (`jobs.rs:325`), overwriting the open page's counts with the finished page's — same daemon totals, so harmless.
8. **Verified** — `Frontend.journal` is read for every frontend (`frontend.rs:200`) and consumed only by `diagnose` (`diagnose.rs:243`).

## 5. Refactors

**PROVABLY EQUIVALENT**

1. Hoist `let now = acquisition_store::now()` once at the top of `Opened::plan` and `plan_selection` (`refresh.rs:503-539`); pass it to `compile`, `plan_notes`, `aged`. Equivalence: `now` is read-only; under `ACQ_NOW` (every test) the two reads are already equal, so every golden is unchanged by construction.
2. Replace `Apply::{Nothing,Spent}`'s `applied`/`requests` fields with serde constants (`#[serde(serialize_with)]` or a unit marker). Equivalence: JSON golden (`documents.rs:409-420`) plus `c149_every_document…` unchanged byte for byte.
3. Compute `request`/`Endpoint::from_job` once per action in `unspent` (`lib.rs:1685-1693`). Pure function, same inputs.
4. Make `StoreChanges::since` take `events: &[EventRow]` and move the `events_after` read to the caller: a pure fold, pinned by `two_applies_in_one_second…` unchanged.
5. Stop computing `before = status()` on the MCP path: add `Opened::apply_without_landing` or make `before` lazy (`Option<Box<Status>>` filled only when a caller asks to land). Equivalence: `before` is `#[serde(skip)]` and read only by `landed`; the MCP never calls `landed` (grep `main.rs`).
6. `shop.rs:1221` and planner `window_verdict` (`lib.rs:2413`) could share one `fn past_window(now, seen, window) -> Option<age>`; both already use `saturating_sub(..).max(0) > window`. Type-checked equivalence.

**TRUST ME**

1. *Gate fact drift in `check_spendable`* (refuse when `snapshot.stash_listing != basis.stash_listing`): would make C38's sentence true but changes behaviour pinned by `c44_fact_drift_never_refuses`. Smallest breaking input: list at t=1000, compile, a second listing lands at t=1200 with the same tabs, apply — today sends; after, refuses a plan whose actions are all still valid.
2. *Attribute `store_changes` to this apply's response ids* (filter events by `responses.id` of the apply's children, not by watermark): smallest breaking input: two applies A and B both open; B's child lands between A's `before` and A's end — today A prints B's adds; after, A prints only its own, and a user who relied on "what changed since I started" loses B's.
3. *Drop `spoken()` and let `refusal::Surface::words` carry the MCP spelling in the Rust strings*: the breaking input is any answer string under `why`/`hint` that contains a SPOKEN word as data (a tab literally named `acq auth`): today it gets ` — here, …` appended; after, it does not. Also a `describe` entry's `what` loses its name spelling.
4. *Reload the MCP corpus on `items` change rather than response id* (a `PRAGMA data_version` or an items-table revision): breaking input: `acq store rebuild` while `acq-mcp` holds a corpus — today the next `search` is served from the held corpus; after, it reloads. Observable only if a derived column feeds the answer (unverified).
5. *Make `Expect::Current` + `Replace` allowed over an unreadable policy* (`frontend.rs:496`): breaking input: a newer-build policy stored, `acq policy set --replace <v>` with no `--if-revision` — today refused `UnreadablePolicy`; after, it lands (C35 risk the current code deliberately takes).
6. *Narrow `TOKEN_KEYS`-driven redaction to structured fields only* (redact by JSON key, drop the prose scan): breaking input: a log line `Authorization: Bearer abc` inside the daemon log tail (a plain string) — today redacted to `<token 1>`; after, printed.
7. *Replace the `select!` writer with a single ordered channel* (`transport.rs:99`): breaking input: an unparsable line sent between two valid requests; today its `-32700` may appear after the second request's reply; after, strictly in arrival order — a host that matched by id sees no difference, one that counted lines does.
