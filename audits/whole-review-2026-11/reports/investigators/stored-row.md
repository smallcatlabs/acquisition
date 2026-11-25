# Investigator's report: stored-row

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/stored-row.md`.

**Track: the stored row, the shared vocabulary, the crate edges.** Read-only. No cargo. All data reads were on a copy of the copy, at `<scratch>/stored-row/db`, through `<scratch>/stored-row/acqr.sh` (observation verbs only).

## 1. The picture

"Unread" means three unrelated things in this tree, and the brief's question A merges them:

| Noun | What it means | What pays for it | Real data (owner's store) |
|---|---|---|---|
| `MalformedBody` → `refused` table (`store/lib.rs:394,501`) | GGG sent a 2xx body ingest cannot file | C47, C30: evidence instead of a refetch | **4 rows, one day (2026-09-02), one cause**: an id-less `socketedItems[0]` (V `sqlite3 … from refused`) |
| `withheld` | a fetch of a location a listing retired | C54 | **0 of 4,946 responses** (V) |
| Facts/annotations/jobs schema stamps (`SchemaTooNew/TooOld/NotCurrent`) | a file another build wrote | C124 | facts v9, annotations v3, jobs v2; none refused (V) |
| `UnreadableRow` (`annotations.rs:224`) | an intent row whose text is not JSON | C151 | **0 of 21 rows** (16 buyouts, 1 policy, 4 saved searches; all `json_valid`, all at the current version) (V) |
| Per-kind "set / unreadable / none" states above the store | an intent row a newer build wrote (the realistic case) | nothing: it is repetition | 0 occurrences (V, the same query) |
| Search `Unread`/`Undecided` (`derive.rs:436`, `class.rs:628`, `eval.rs:155`) | an item line the game tables don't cover; three-valued logic | the search language's honesty | `undecided` = 0 for class/category/rarity/base and `pseudo.total_res` over pc and over all realms (V `acqr.sh search --realm all --count class,category --json`) |

Only the fifth row is the finding class. The search's `Unread` is a different concept and must stay out of any "typed row" thesis.

**Crates as built** (V, each `Cargo.toml`; lines are `src`):

| Crate | Lines | Load-bearing for |
|---|---|---|
| transport | 236 | invariant 1 (reqwest only here; check 5) |
| daemon | 47,975 | invariants 1, 3, 5 (only it links transport and keyring; nothing links it) |
| protocol | 1,814 | C12 wire. In practice never used without the store: every crate that links it (client, daemon, plan, verbs, cli, mcp) also links the store |
| store | 9,850 | C34, C35, C41. Its "links nothing" is habit for the protocol edge (below) |
| client | 5,242 | the door. Links the store for `world` paths, which AGENTS.md's C1 text omits; `arch-check.sh:482` states it |
| plan | 8,558 | holds two crates. The refresh planner is `lib.rs` plus `game_side`, about 2.7k; pricing, listing, shop and currency are about 5.8k |
| search | 35,008 | links plan only for the effective price |
| verbs | 7,332 | C46 doors |
| cli | 27,953 | includes `web/` at about 12k |
| mcp | 2,905 | thin |

The smallest edge set that keeps invariants 1, 3 and 5 structural is: reqwest ∈ transport only; transport ∈ daemon only; daemon ∈ nobody; keyring ∈ daemon only. The fourth is not checked today (`grep keyring tools/arch-check.sh` finds nothing). No other crate boundary holds invariant 1, 3 or 5. The other forbids hold C34, C41 and C89 (I).

## 2. Theses

### Thesis 1: one typed intent read in the store, with no C1 edge moving

The mechanism already lives in the store. `IntentValue` and `check_value` are there (`annotations.rs:347,441`), and so is `get_as::<K>` (`:1277`). But `get_as` has **zero callers outside its own file** (V `git grep get_as`). Instead, 32 production sites call `check_value`/`from_value` above the store, and each folds the result into its own state type.

- **The change.** Add `read::<K>() -> Option<Stored<K>>` and `list_as::<K>() -> Vec<Stored<K>>`, where `Stored<K> = Set{row, value} | Unreadable(UnreadableRow)`, and `UnreadableRow.fault` carries the `ValueError` (so "newer" is a flag on it, not a type of its own).
- **What goes.** About 10 types and variants: `StoredPolicy`, the first two arms of `PolicySource`, `PolicyState`/`PolicyRead`, `Conflicting::Unreadable`, listing's `Unreadable`, `SavedRow::Unreadable`/`UnreadRow`, `Prior::Unread`, `OpenedRow::Unreadable`, and `PutPolicyError::StoredUnreadable`. That is **129 match lines in 18 files**, by `git grep -n "StoredPolicy::\|PolicyRead\b\|PolicySource::\|PolicyState::\|SavedRow::\|OpenedRow::\|Conflicting::\|AnnotationError::Unreadable\|Prior::Unread\|StoredUnreadable\|UnreadablePolicy" -- crates | grep /src/` (V). Outside the search crate, 372 source lines mention unread(able), and 149 test lines do; 10 tests are named for it.
- **Why the edge cases go.** One condition gets one fault. Today they diverge: `verbs/frontend.rs:532-545` turns a non-JSON policy row into `Fault::Other`, but a newer-version row into `Fault::UnreadablePolicy` (V, read).
- **Smallest changed input.** A `sync-policy` row whose text is not JSON, met by `policy set` naming no revision. Today it is refused as `Fault::Other`; afterwards it gets the same refusal as a version-4 row.
- **What still holds.**
  - C35 and C151: a write naming the row's revision, or a delete, still never parses the value it replaces.
  - C47: `ValueError` remains the structured error.
- **Costs and gains.**
  - Security: unchanged.
  - Performance: unchanged.
  - Function: one wording per condition.
  - An agent's ease: a new kind inherits the handling for nothing.
- **Strongest objection.** Saved searches have a third tier (a source that will not translate, C194), which is the search's and not the store's. So `SavedRow` keeps a field. The thesis removes the *row* tier, not every read fault.

### Thesis 2: the store links the protocol, and the vocabulary has one home

**The one-sentence ruling:** *"The store links the protocol crate, and only it; the protocol still links serde alone."* Without this edge the store cannot share the typed job kind and realm, so each crate re-spells them as strings.

- **Edge and check.**
  - C1 changes from "the store links none of the workspace crates" to "the store links the protocol crate alone".
  - `arch-check.sh:488` drops `acquisition-protocol` from the store's forbid list.
  - `allow acquisition-protocol normal serde serde_json` (`:490`) already guarantees the store gains no new dependency, since it links both already.
  - The forbid's stated reasons, C41 and C89 (no HTTP, no search), are not touched: the protocol carries neither.
- **What goes.**
  - **Job kinds.** The daemon's own comment lists **11 places a new kind must be added** (`daemon/queue.rs`, above `KINDS`): `CLIENT_KINDS`, `MOCK_ONLY_KINDS`, `target_of`, `route_for`, `sends_unrouted`, `NO_PROBE_ROUTES`, `kind_needs_account`, `admit_realm`, the run arms, apply's vocabulary, and `Endpoint::from_job` (V). These become one enum with exhaustive matches. The runtime failure "the store knows no place for a {kind} response" (`kinds.rs:265`) becomes unrepresentable.
  - **Realm word lists.** Five lists become one: `protocol Realm::ALL`, search `REALMS` (`corpus.rs:121`), store `SAVED_REALMS` (`user.rs:48`), and `web/page.rs:941,3650` (V).
  - **Realm scope types.** Two become one: search `Realm{All,One(String)}` and store `RealmScope`.
  - **Note.** The search's copy is not forced by C1. `arch-check.sh:486` does not forbid search→protocol today.
- **What does not go.**
  - `Realm::DEFAULT` (12 uses, V) and `DEFAULT_LEAGUE` (22) are frontend defaults for an unnamed scope, not a missing type.
  - They go only under "scope resolved once" (history-hindsight item 1), a product ruling. I part from that report's "partly C1 forces this": none of those sites is in the store.
- **The two alternatives.**
  - A leaf crate adds one crate and removes no forbid; the protocol can carry the same thing.
  - Putting the vocabulary in the store would make the protocol link SQLite. That breaks the one boundary that matters for C12.
- **Smallest changed input.** A persisted job of a kind this build does not know. Today it is refused at restore by string; afterwards it is refused at decode. C47 (a refusal, never a panic) holds either way.
- **Costs and gains.**
  - Security: none.
  - Performance: none.
  - Function: none. The wire stays the same strings.
  - An agent's ease: large. A new kind becomes one edit, with compiler errors where a hand-kept list used to be.
- **Strongest objection.** C58 says "the store stays string-typed: it records the request's realm and never validates it". A typed realm column means a newer build's fifth realm becomes a row this build cannot read, a *new* unreadable case. The fix is for the store to keep reading the column as text and type it only at its API.

### Thesis 3: `record_in` has two cases, not six

`record_in` (`store/lib.rs:1575-2168`, 589 lines, V) is one transaction. Its simplest true statement is:

> *A body is refused whole, or lands whole. A listing stamps its members and retires, within its scope, what it did not stamp. A fetch replaces its location's contents, or, at a location a listing retired, lands only its envelope.*

How the code departs from that sentence:

- Validation is interleaved with writes. The characters loop errors after inserts and relies on rollback.
- The withheld decision is written twice, once in the character arm and once in the stash arm.
- One retirement rule is written as three SQL blocks, one per scope: realm characters, a (realm, league) listing, and a parent's substashes.

The proposed shape: a pure `validate(endpoint, body) -> Parsed`, which holds every `MalformedBody` and is testable without SQLite, then one listing path and one fetch path. I part from code-craft's "one function per endpoint", which keeps the six cases. No ruling changes and C47 holds.

**Objection.** The listing-owned field guards differ: tabs check `listed_at`, characters check `listed_response` (the scout's finding). Unifying them is a behaviour change to pin first.

## 3. One thing I would defend

The `refused` table. It is 4 rows from one day, and another reader would call it ceremony. But it is the only evidence behind N44 and C57, and it costs one table no basis reads. Without it, every malformed body becomes a refetch against a rate-limited API.

## 4. Where the old way is right

- **The transport and daemon edges.** They are what makes invariant 1 a compiler fact.
- **C151's "named, never dropped, cleared by revision".** The fault is only that it is applied at the JSON tier instead of the kind tier.
- **The store not linking plan or search.**

## 5. What I did not check

- Whether today's ingest would accept the 4 refused bodies (C57 came after them).
- The golden wording under thesis 1.
- `jobs.rs`, `coverage.rs` and `restore_jobs` in depth.
- The MCP's paths.
- Whether a full rebuild from verbatim bodies is possible (another investigator's question). For question 6: if it were, only the facts schema refusals would go. `MalformedBody` stays (it is GGG input). Every intent state stays, because intent cannot be rebuilt (C35); thesis 1 shrinks those, it does not remove them.

## 6. Small finds

- `daemon/kinds.rs:606`: `Realm::from_params(&params).unwrap_or(Realm::DEFAULT)` turns a realm that fails to parse into pc.
- The "omitted realm means pc, league Standard" decode default (`Endpoint::from_job`, `Realm::from_params`) serves no real row. The oldest job in the real queue is from 2026-11-03, and the only realm-less rows are `leagues`/`probe`/`profile` (V). It is a retirement candidate under C154.
- AGENTS.md's C1 text omits the client→store edge.
- Invariant 5 has no edge check: keyring sits in the daemon by habit alone.
- `snapshot.rs:290` reads the policy after the facts transaction ends, and the store defines the planner's `SYNC_POLICY_*` keys (`snapshot.rs:33-38`).
- `withheld` has fired 0 times in the real store; its log line and status text serve a case never seen.
- `get_as` is dead outside its own file.
