# Search crate — recovered semantic model

Read-only pass over `crates/acquisition-search/src` (31.5k lines), the CLI's `printed.rs`/`search_cmd.rs`/`web/form.rs`, the MCP's `main.rs`, `decisions/search.md`, `decisions/trade.md`, `REVIEW-CHECKLIST.md` "Search"/"Trade", `tools/arch-check.sh` §7. Every claim cites `path:line` (paths relative to `crates/acquisition-search/src/` unless stated).

## 1. State

| Value | Lives | Written by | Read by | Truth or mirror |
| --- | --- | --- | --- | --- |
| Query tree `Node` + canonical text | memory, per request | `parse::parse` (`parse.rs:72`), `json::from_json` (`json.rs:113` runs `tree::check`) | `bind`, `print`, every answer (`answer.rs:1241` `QueryOut`) | the tree is truth; text is a print of it (C104) |
| Bound query (`bind::Query`, `Term`s with paths) | memory | `bind::bind` | `eval`, `answer`, `counts` | derived from tree, pure |
| `Group` (whole, selector, selector_tree, together, admitted, templates) | memory | `group::Group::bind` (`group.rs:720`) | eval/answer/counts via Group's methods and `pub selector_tree` (`group.rs:702`) | derived from tree, pure |
| `Item` (header, lines, rows, numbers `Vec<Option<f64>>`, flags, `unread`) | memory, inside `Corpus` | `derive::derive` — pure of body+facts (`derive.rs:1-30`) | eval, show, counts, pseudo | derived; `DERIVATION` version in basis (`corpus.rs:106`) |
| `Held` (item, place, class, category, price) | memory | `Corpus::read` inside the store callback (`corpus.rs:682-699`); price joined after (`corpus.rs:818-826`) | everything | derived snapshot |
| `Basis` | memory, printed in every answer/JSON, carried in `--next` tokens | `Basis::of` (`corpus.rs:516`) | `Asked::answer` (`answer.rs:1046-1056`), `Holder::corpus`, `Request::place` | label of the snapshot; store revision + intent revision are the truth it mirrors |
| Held corpus slot | process memory (`Holder`, `Mutex<Option<Slot>>`, `corpus.rs:875`) | `Holder::corpus` | MCP (`acquisition-mcp/src/main.rs:1616,1831`), `acq web` (`acquisition-cli/src/web/mod.rs:203,643`) | cache, revalidated by `is_current` before every hand-out (`corpus.rs:841`) |
| Reference tables (class, category, colours, totals, defences, properties, trade catalogue/stats/forms) | compiled in, parsed once (`LazyLock`, e.g. `category.rs:402`) | build | binder, eval, translate | truth for this build; versions in basis |
| `Place.public`, `Place.remove_only` | memory, `#[serde(skip)]` (`corpus.rs:336,345`) | `placed`/price join | `eval::texts` for `tab.public`/`tab.remove_only` | derived from store + listing state; never in JSON directly |
| Trade report (`QueryOut.trade`) | memory per answer | `translate` on every read (`answer.rs:85-90`) | answer JSON, CLI, web | derived; versions are the report's, not the basis's (`answer.rs:102-109`) |

No SQLite, file, keyring or env state is owned by this crate (C89). Only `Holder` persists across queries, and only inside one process.

## 2. Transitions

Pipeline, as a function: `text ──parse──▶ Node ──check──▶ bind(Group::bind inside) ──▶ Query` is pure and corpus-free (invariant 3, `parse.rs:5-11`, `bind.rs:1-5`). `Corpus::load_of` is the one store read: header+rows derived in the store's callback, then `price::join`, then `still_at` (`corpus.rs:818-821`); if facts or intent moved, re-read up to `REREADS=3` (`corpus.rs:568-577`) else `basis_moved`. `answer()` is pure over `(Corpus, Request)`: every term over every item (`answer.rs:1977-2008`), then composition, then view (counts or sort+cut), then routes. Explain re-enters `answer` with the forced tree (`answer.rs:272-283`).

Transitions on the held slot (`corpus.rs:905-928`): (realm, membership) match and `is_current` ⇒ reuse; else drop and reload.

Classification:
- **Idempotent**: `canonical(parse(canonical(q))) == canonical(q)` — pinned for generated trees (`tests/language.rs:672-677`) and corpus cases (`tests/language.rs:150-168`). `Holder::corpus` twice with no write. `answer()` twice over one corpus.
- **Commute**: order of terms in the terms block vs matching (each term counted alone, `answer.rs:1983`). Two `Holder` asks for different scopes commute in result (not in cost: slot holds one corpus, `corpus.rs:862`).
- **Order-dependent**: (a) *store write vs Holder check*: check-then-answer; a write after the check is a correctly labelled stale answer by C98 (`corpus.rs:862-870`). (b) *continuation after a refetch*: `--next` carrying a basis ≠ corpus's is refused (`answer.rs:862-874`, `1046-1056`); a route without basis runs anywhere. (c) *rows: sort then cut* — `order.sort_by` then `.skip(after).take(limit)` (`answer.rs:2150-2163`), stable sort so ties keep store order. (d) *vocabulary row vs plural form*: a template whose fold is spelled two ways is a "twin" and routes by `~` instead of `=` (`counts.rs:1107-1110`, `label_of` `counts.rs:632-637`): adding one item with the other spelling changes every bucket's route text.

## 3. Dependencies

| Effect | Precondition | Checked at |
| --- | --- | --- |
| Any evaluation | tree passes `tree::check` (depth ≤ 32, slot rules) | `parse.rs:112`, `json.rs:113`; `tree.rs:567,772` |
| Answer printed with basis B | prices joined at revision B | `still_at` after the join, same thread (`corpus.rs:818-821`) — not TOCTOU-free but re-checked, retried |
| Reused held corpus | `is_current` (store path digest, account, revision, intent) | under the slot lock (`corpus.rs:905-914`); answer computed after release — by design |
| Route printed | the route parses and is not refused | rule 5: `Group::bind` drops a together route `check` refuses (`group.rs:744-750`); CLI test runs every printed command |
| `--next` continues | `request.basis == corpus.basis` (full `PartialEq`) | `answer.rs:1053` |
| Site search run as the site's | translation remainder empty; text sent beside equals the translation | `answer.rs:85-93` |

TOCTOU flagged: **Holder** — `is_current` reads the header with `RealmScope::All` while the corpus was loaded under `RealmScope::One` (`corpus.rs:841-849` vs `590`); correct only while `header.revision` and `account_uuid` are scope-independent (they are store-global today). Verified by reading; held by nothing in this crate.

## 4. Findings

### 4a. Nondeterminism

- **None reaching output from HashMap order** (verified). Every HashMap that becomes a list is sorted on a total key: `counts.rs:882` (items desc, then `spelled`, which includes the tab's id, `counts.rs:742-754`), `counts.rs:976` (cells, spelled vectors), `counts.rs:1191` (`(items, template, realm)` = the map key), `answer.rs:2072` (resolved values, value is the key), `answer.rs:3024` (suggestions, value last). Lookups only: `corpus.gone`, `order`, `price::join` (iterated over a `BTreeSet`, first-wins `or_insert_with`, `price.rs:326-342`).
- **Sort ties fall to store order** (verified): rows' `sort_by` returns `Equal` for None/Incomplete pairs and equal values (`answer.rs:2150-2160`); `sort_by` is stable, cut is after. `tabs.rs:176` ties on `(item_count, id)`, total.
- **`dedup` after non-total sort**: none found; `group.rs:386,601`, `derive.rs:613,893`, `eval.rs:1186` dedup sorted scalars/strings.
- **Floats**: no float arithmetic reaches a sum, cut or print. Sums go through `Exact` (`eval.rs:664-668,683-685`); the only `f64` ops are comparisons (`bind.rs:622-631`), `reduce(f64::max)` for the largest (`eval.rs:1494,1529`), `row.weight * 2.0` at table load, checked whole (`totals.rs:620-630`), and `Exact::of` conversions. `units()` reads the float's decimal print, bounded by `exact::reads` at the deriver (`exact.rs:256-275`).
- **Suspected, low**: `Exact::sum` uses unchecked `i128` `.sum()` (`exact.rs:95`) and `halved` an unchecked multiply (`exact.rs:103`); safe given `reads` (≤1e15 units per number) but held by the deriver's rule, not by the type.

### 4b. Invariants enforced by hope

1. **"One reader of a group's meaning"** (checklist; `group.rs:10-18`) is held by a *token* grep for `Member`/`BMember` (`tools/arch-check.sh:435-446`). `Group.selector_tree` is `pub` (`group.rs:702`) and `answer.rs:2691-2693, 2856-2858` wrap it in `Node::Members` without naming `Member`; a reader that pattern-matched `where_` through a `Node::Members { where_, .. }` binding would also pass the grep. Today nothing does (verified: only `bind.rs:1156-1163`, `translate.rs:2322` match the outer shape). Hope-adjacent.
2. **C98's "checked before every answer"** holds for `Holder` consumers; the CLI loads per process (`search_cmd.rs:419-440,611`). The `is_current` scope mismatch (§3) is unpinned.
3. **`Basis::against` omits `properties`** (`corpus.rs:214-281` lists 14 parts + notes; `Basis.properties` at `corpus.rs:124`). Equality uses full `PartialEq`, so a continuation across a property-table bump is refused with a message that names no differing part — unreachable in practice only because `build` (a digest of sources) moves too. Verified; pinned by nothing.
4. **Canonical text does not normalise name case**: `Node::Test.field`, `Has`, `Is` and `pseudo.<name>` keep the authored case (`parse.rs:703-709,1452-1470`); the binder matches any case (`bind.rs:500`). So `Rarity=RARE` and `rarity=rare` are two canonical texts of one meaning, contra invariant 1's "normalises spelling" (`lib.rs:33-34`). Idempotence holds (parse keeps the case), and the round-trip property generates only lower-case field names (`tests/language.rs:414,598`) so it cannot see this. Verified.
5. **C96/C105/C48 "Pinned: nothing yet"** (C48 raw SQL not a surface): C48 held structurally — the crate links the store's read API only (`tools/arch-check.sh` edges, C89) and nothing is persisted (`corpus.rs:12-14`). Verified by grep: no `rusqlite`/SQL in the crate.
6. **`eval::sorted_by` claims "the first of them where several tie"** (`eval.rs:1489-1511`): correct, but it reads `Read::line(...).added.contains(&most)` with `f64` equality — exact only because both were read from one decimal (`exact.rs:21-24`). Held by `reads`, not a type.

### 4c. One fact in two places

- **The route's basis**: a route carries none (B3); `Before.counted_at` is the caller's copy of the answer's basis (`answer.rs:960-975`). Authoritative: the corpus's basis; disagreement is the point (reported, `answer.rs:1049-1055`). Fine.
- **Trade report versions vs basis**: the catalogue/stat/forms versions ride in `QueryOut.trade`, not in `Basis` (`answer.rs:102-109`); the build digest covers both. Two places by design; a report from an older build beside a newer corpus is told apart only by `build`.
- **`Of::Text` legal spelling vs twins**: a bucket's value is the list's spelling (`counts.rs:566-569`) while `twins` are computed over the corpus's raw spellings (`counts.rs:460-481`); for closed sets `field_twins` is empty (`counts.rs:764-771`) so no drift. Verified.
- **The CLI's `SPOKEN` table in the MCP** (`acquisition-mcp/src/main.rs:1343-1368`) re-spells CLI words found in shared texts — a second speller of shared wording, by ruling C143.

### 4d. Work that never reaches the screen

- **`Total.places`** (`answer.rs` `Total`, computed by `subject`, `answer.rs:1794-1806`) is in JSON but printed by neither the CLI nor the web page (grep: 0 hits in `search_cmd.rs`, `web/page.rs`). JSON-only, not dead.
- **`snapshot_at_age`, `before_snapshot_at_age`** (`answer.rs:1265-1279`): JSON-only (C163), no terminal line.
- **`Table.left_out_needs`** (`counts.rs:901`) — JSON-only; the CLI prints its own count from `values - left_out` (`search_cmd.rs:2174-2177`).
- **Web page** ignores `span`, `incomplete` slot ranges, `computed_left_out`, `rest` of resolved (grep counts 0 in `web/page.rs`).
- Trade `forms`: 2,647 game forms + 14 rows (`tests/trade_forms.rs:33-38`) — the loader checks structure and the walk checks refusals; the oracle (`tests/trade_oracle.rs`) exercises only forms present in the captured items' lines (518 searches, 4,733 items); `trade-translate.toml` names "form" 7 times. **Not every form row has a translation case** (verified by count; the forms' semantic effect is "presence over every form", one code path, so the gap is in evidence not behaviour).
- Dead-ish branch: `Basis::against` for `properties` (4b.3).

## 5. Refactors

**Provably equivalent**
- Fold `Basis::against`'s 14-tuple into an iteration over a derived field list (or add `properties`): equivalence by `tests/next.rs` (refusal across a changed basis names parts) plus the struct's `PartialEq`.
- Replace the `Member` token grep with the type's visibility: make `selector_tree` `pub(crate)` read through a `Group::selector_node()` that returns a `Node` — the compiler holds §7; `tests/generated_equivalence.rs` establishes no answer changes.
- Store `Line.numbers` as `Vec<Option<Exact>>` instead of `Option<f64>`: every consumer already converts via `Exact::of` or compares floats read from one decimal (`exact.rs:21-24`); `tests/derive.rs`, `tests/rows.rs` and `tests/pseudo.rs` pin values, and `whole_numbers` serialisation (`derive.rs:1326`) stays.
- Lower-case field/`has:`/`is:` names at parse: `tests/language.rs` corpus cases pin the canonical texts; a case-only change is visible in `language.toml` diffs.

**Trust me**
- *Lower-casing closed-set values at parse* (`rarity=RARE` → `rare`): breaks on a `text:` phrase or `name=` value where case is data; smallest breaking input: `name="Kaom's Heart"` printed `kaom's heart` — matching unchanged (`bind.rs:21-26` any-case), but the printed canonical and `(?-i)` patterns differ (`bind.rs:27`).
- *Holder keyed by resolved realm* (treat `None` and `Some(One(r))` over a one-realm store as one slot): differs when the store gains a second realm between asks — `None` must then refuse (`corpus.rs:598-611`) while `Some(One)` still answers; smallest input: two asks, a tab list of another realm ingested between them.
- *Dropping `REREADS`* in favour of trusting the store's transaction: differs when the intent file moves between the corpus read and `price::join` (`corpus.rs:818-821`); smallest input: a price write landing between the two reads — today `basis_moved` after 3 tries, after the change a basis labelling prices from another revision.
- *Make `is_current` read with the slot's own scope*: equivalent only while revision/account are store-global; smallest breaking input would be a store whose header revision became per-realm.
