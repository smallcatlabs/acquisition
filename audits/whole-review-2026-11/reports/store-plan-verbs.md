**Scout report: persistence and the middle layer (store, plan, verbs)**

## 1. Model

There are three durable stores per world, and the account identity is split across them:
- **Facts**: `<provider>/<filename_safe(username)>.db`, SQLite v9 with floor 8 and one rung (8→9). Its revision is `(MAX(responses.id), fetched_at, user_version)`.
- **Intent**: `<provider>/<uuid>.annotations.db`, v3 with floor 3. Each row carries a revision under compare-and-swap; the file's revision is `SUM(revision)`.
- **`user.db`**: a newtype over the same `Annotations` code (saved searches).

Beside them sit:
- `daemon.db` (jobs v2, rungs 0/1→2; its "revision" is `MAX(updated_at)`).
- `accounts.json` and `rails.json`, both unversioned, held by `serde(default)`.

On top of the three SQLite schemas there are three versioned JSON value kinds (`SyncPolicy` v3, `Buyout` v1, `SavedSearch` v2) via `IntentValue`/`check_value`, plus three committed document schemas (refresh plan, listing report 3, shop render 5). In all: seven files, three migration ladders, three revision schemes, three value versions.

The store's annotation API is **untyped**. `get` and `list` return `AnnotationRow{value: serde_json::Value}`, and "unreadable" at the store means only "not JSON" (`annotations.rs:224,260`). Every reader then re-parses the value by its kind with `check_value::<K>`. A newer build's value is the realistic unreadable case, and it is discovered at that second tier, above the store, by each caller separately.

The planner reads a `RefreshSnapshot` (facts in one transaction, then the policy row read after it). Pricing, listing and shop are a second domain living in the same crate. `acquisition-verbs` holds the account/opener/door layer, refresh, jobs, saved searches, receipts and the one refusal shape.

## 2. Measurements

| Measure | Value | Command |
|---|---|---|
| src / test lines | store 9,850 / 8,791; plan 8,558 / 9,089; verbs 7,332 / 6,676 | `git ls-files … \| xargs cat \| wc -l` |
| Lines matching "unreadable" | 253 in src, 176 in tests | `git grep -ci unreadable` over `crates/*/src`, `tests`, `unit` |
| Per-kind unreadable state types and their match sites | 7 types, about 100 match lines in 15 files (saved.rs 29, pricing.rs 14, leagues.rs 11, web/mod.rs 10, …) | `git grep -n "StoredPolicy::\|PolicyRead::\|PolicySource::\|PolicyState::\|SavedRow::\|OpenedRow::\|Conflicting::\|AnnotationError::Unreadable"` |
| `check_value` calls outside the store | 13 production sites (plan 7, verbs 5, cli 1) | `git grep -n check_value -- 'crates/*/src/**'` |
| Direct `SYNC_POLICY_KEY` reads | 7 production sites in 4 crates (store `snapshot.rs:290`, plan ×3, verbs ×3) | `git grep -n SYNC_POLICY_KEY` |
| `plan/reference/` data no Rust reads | 30,780 lines / about 784 KB (two `site-listings-*` and `exchange-items-*`) | `git grep` for each name: only `trade-ground-truth.md` cites exchange-items |
| Comment share of plan sources | lib 761 of 2,482; listing 482 of 2,015 | `grep -cE '^\s*///?'` |

## 3. Where a deep review would pay

**H1. One typed read at the store boundary (assessment candidate 6). Highest value; confidence high on the shape, medium on the size.**

*Evidence.* "Unreadable" has two or three tiers, and each kind re-folds them into its own enum:
- **JSON tier** (store): `UnreadableRow`.
- **Kind tier** (`check_value`, the newer-version case):
  - `StoredPolicy::Unreadable` folds both tiers (`plan/lib.rs:668-716`).
  - Then `PolicyRead`/`PolicyState` (`verbs/leagues.rs:28-90`), `PolicySource` (`shop.rs:407`, a copy of `StoredPolicy` with a field added, mapped at `pricing.rs:1194`) and `PutPolicyError::StoredUnreadable`.
  - `Conflicting::Unreadable` (`pricing.rs:279`).
  - `SavedRow`/`UnreadRow` plus `Prior::Unread` (`verbs/saved.rs:116,489`).
  - The web's `OpenedRow` (`cli/web/page.rs:544`).
  - The CLI's own re-parse, `value_text` (`price_cmd.rs:1162`).
- **Saved searches add a third tier**: `ReadRow.fault`, a source that will not translate (C194).

**Verified (read): the tiers leak as inconsistent faults.**
- `Account::set_policy` with `Expect::Current` turns a non-JSON row into `Fault::Other` but a newer-version row into `Fault::UnreadablePolicy` (`verbs/frontend.rs:532-545`).
- `refresh_snapshot` propagates the JSON-tier row with `?` (`store/snapshot.rs:290`), so `Opened::snapshot` gives `Fault::Other` (`refresh.rs:551-554`). A newer-version row gives `Fault::Plan` instead (`plan/lib.rs:1933`).
- **Verified (read):** `listing.rs:1193-1280` `index_rows` says "one path" in its comment but runs two near-identical loops, one per tier.
- C151 (`decisions/store.md:23`) rules the store "names" unreadable rows, but the realistic case is named above the store.

*Sketch.* Give the store `get_as::<K: IntentValue>()` and `list_as::<K>()` returning `Option<Read<K>>`, with `enum Read<K> { Value{row, value: K}, Unreadable(UnreadableRow) }`. `UnreadableRow.fault` would carry the `ValueError` text plus a `newer` flag. Each kind's state enum becomes `Option<Read<K>>`, and one generic `state()` gives none / unreadable / set for documents.

*Payoff (judgment).*
- Code: roughly 200–400 source lines and about six types gone, and one fault per condition.
- Concept: the most-repeated fault class in the assessment (24 findings, 7 of the 20 P1s), and every future kind inherits the handling.
- Risk: medium. The goldens carry `unreadable` words, and `user.db`'s source tier must stay a document-level field.

*Question for the reviewer:* should the store own "this build cannot read this row" fully, given it already owns `IntentValue`?

*Read:* `store/src/annotations.rs` (get/list/check_value), `plan/src/lib.rs:640-1010`, `listing.rs:1185-1290`, `pricing.rs:180-290,860-880`, `verbs/src/{leagues,saved,frontend}.rs`.

**H2. One policy reader.** Seven direct reads of the policy row, including the store's own `snapshot.rs:290`, which hard-codes the planner's key in the facts crate. There are also four wrappers: `stored_policy`, `StoredPolicy::of_row`, `PolicyRead::{read, of, of_account}`, `Account::policy`. Folds into H1. Confidence high.

**H3. Name facts by uuid, as intent already is.** Username-named facts cause:
- `filename_safe` collisions (`index.rs:228`, all non-ASCII → `_`);
- the `user.db` collision and its `HoldsAccountFacts` remedy, kept by the owner's word;
- a rename orphaning the facts file;
- `Fault::NoUuid`, left after cut 1 as an unfiled follow-up.

The assessment's "which account, among several" (20 findings) now looks held by `Frontend::account` (C121). The file-naming split stays a standing source of edge cases. It is a ruling and touches the owner's store. Confidence medium.

**H4. Stale semantic-model findings.** About half are fixed; nobody owns the rest. See the table below.

**H5. `plan/reference/` hygiene.** About 784 KB / 30.8k lines of trade-site captures sit in the crate with no reader. They belong under `audits/` or `search/`-style evidence. Small win, zero risk. Confidence high.

**H6. Two crates under one name.** `acquisition-plan` is a refresh planner plus a pricing/listing/shop domain that also holds "verbs" (`pricing.rs` returns anyhow errors with a downcast `Refused`). The refusal shape is assembled by `refusal.rs:240` with `Refused::find` over an anyhow chain, alongside `Fault`.
- *Question:* would `acquisition-pricing`, or moving the pricing verbs into verbs with typed faults, remove the anyhow/downcast seam?
- Confidence medium (judgment).

### Semantic-model findings at HEAD

| Finding | Status |
|---|---|
| Tab fetch overwrites listing-owned `name`/`type` (C54) | **Fixed** (#205; `store/lib.rs:1957-1959` guards on `listed_at`). The character fetch still overwrites `realm` (`lib.rs:1795`), and characters guard on `listed_response` rather than `listed_at`: two predicates. Standing, unfiled. |
| `store import` writes facts from the CLI | **Removed** (cut-293-1) |
| `store rebuild` writes facts from the CLI (C34) | **Standing**, #212 |
| Columns written, never read (`items.w/h`, `responses.item_count`, `item_events.response_id`, constant `status`, `leagues` table, `TabRow.removed_at`) | **Standing**; #214 lists them "outside reach" |
| `Persisted::read` in two statements | **Standing** (`store/jobs.rs:714-724`, verified); no issue found |
| Plan ages computed twice | **Fixed** (#211; `refresh.rs:576`, one `now`) |
| Freshness window in planner vs shop | **Filed**, #213 open |
| `Submitted.before` computed and dropped by the MCP | **Fixed** (`Watermark`, `refresh.rs:254`) |
| `parse_wait` duplicated | **Standing** (`store/lib.rs:1289`, `client.rs:351`); no issue found |
| Duplicate `has_column` | **Gone** with cut 1's ladder |

### Q4: verbs as a layer

The boundary is principled for the account, opener, door, refusal and receipt pieces (C46/C121). By its own doc it is "what needs daemon and store together", plus shared openers. In practice domain verbs live in three crates (pricing in plan, search in search, saved and policy in verbs), and the policy write is split between plan (`put_sync_policy_over`) and verbs (`set_policy`). The MCP is thin (`server.rs` is 991 lines; 22 `acquisition_verbs::` references). The CLI is not: 30.8k source lines, with `price_cmd` at 2,274, `plan_cmd` at 2,432 and `web/` about 11k. Most of that is rendering, but it re-parses rows (`value_text`) and re-maps states (`OpenedRow`).

`tests/documents.rs` has 67 tests. They pin cross-surface document properties: one JSON object (C149), ages beside every time (C163), one refusal shape, receipt key order and undo. About 15 tests and 99 `saved::` references are the saved-search write semantics (C167, C194). Those are better seen as saved's own unit tests than as "documents".

### Q5: plan's core

The real core is `compile` plus `window_verdict`/`tab_verdict`/`character_verdict` (`lib.rs:2122-2478`, about 350 lines). Around it, accreted:
- the policy type and its wire parse;
- stored-policy readers and writes (about 400 lines);
- plan notes;
- the envelope wire `TryFrom` (about 130 lines);
- the spend gate and spent logic (about 270 lines);
- quote matching;
- a 217-line module doc.

The 34k non-Rust lines:
- two generated goldens (`listing-report-schema-3.json`, `shop-render-schema-5.json`, regenerated under `ACQ_UPDATE_FIXTURES`, misnamed "schema");
- the compiled-in `currency-v1.toml`;
- three test fixtures, including a 1 MB one-line hashed snapshot of the owner's account;
- 30.8k unread capture lines.

### Q6: the cuts

Measured with `git diff --shortstat` over `crates/`:

| Cut | What went | Lines |
|---|---|---|
| 1 | Migrations below v8, policy v1/v2, `store import` | +418 / −805 |
| 2 | TUI dashboard | +40 / −811; `Cargo.lock` −1,043 lines, packages 454 → 351 |
| 3 | The query's own order | +300 / −551 |
| 4 | The quote's estimate over a closed route | +228 / −189 (net growth) |
| 5 | History in comments | `59e1c24c^..7ac945a3` −6,861 / +6,030, about −831 net (range may include other work); comment lines 43,413 → 42,642 |

Net is about 2,200 lines of about 245k, under 1%. Lessons:
- The removals that paid were whole features and their dependencies (`dash`) or whole rules (order, migrations).
- Comment trimming cost 5 batches and 5 reviews for −771 lines.
- Cut 1 alone took three review rounds, in the store's door. That is the same zone as H1, which argues for a design read before patching there.
- Next removals in this shape: H1, `store rebuild` (#212), #214's unread columns, H5.

## 4. Low yield / already covered

- Account resolution (C121 holds it now).
- The plan's clock (one `now`).
- `Submitted.before`.
- Migration ladders (cut 1 left only 8→9 and jobs 0/1→2, both release-justified).
- Comment history (cut 5 done).

## 5. Blind spots

- Whether H1's typed door preserves every `unreadable` golden word. I ran nothing.
- What a non-JSON policy row does end to end on `acq refresh` (traced from code only; whether a test pins it was not checked).
- Whether `refresh_snapshot` reading the policy after the facts transaction (semantic-model 4a.3) still matters.
- The corpus `Holder` revalidation against `user.db`.
- I did not read `corpus.rs` or `world.rs` in depth, nor any CLI `web/` logic.

## 6. Surprises

- **The store's "unreadable" is not the C151 case.** C151 is worded as if the store names unreadable rows, but a newer-version value, the case that motivated it, is detected above the store by each kind separately.
- **The store knows the planner's policy key.** `snapshot.rs:290` reads `SYNC_POLICY_*` directly, so planner knowledge sits in the facts crate.
- **A comment that says "one path" over two loops.** `listing.rs:1207` claims one path while the code runs one loop per tier.
- **784 KB of captures with no reader.** The `site-listings` files have sat in a shipped crate's directory since the root swap (`e7b813b8`).

**Verified by re-reading the code:** H1's two fault paths (`frontend.rs:532-545`, `refresh.rs:551`, `snapshot.rs:290`), `index_rows`' two loops, `Persisted::read`'s two statements, and the reference files having no reader (grep).
