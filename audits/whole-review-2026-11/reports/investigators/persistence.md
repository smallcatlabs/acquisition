# Investigator's report: persistence

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/persistence.md`.

**Track: what persists, and why.** Read-only. I ran no cargo. I ran `acq` only against the copy, through `<scratch>/persist/racq.sh` (the preamble's env line, `search` only), and read the files with `sqlite3 'file:…?mode=ro&immutable=1'`.

## 1. The picture

| Thing (store dir `<root>/ggg/`) | Writer → reader | Lost if deleted | Re-derivable? |
|---|---|---|---|
| `<uuid>.annotations.db` (v3, floor 3, no rung; row revision under compare-and-swap; the file's revision is the sum of row revisions) | frontends via the store crate → plan, search, price | the owner's 17 rows (1 sync policy, 16 buyouts; the sum of revisions is 39, the basis's "39") **V** | **No** |
| `user.db` (same type, v3) | verbs → saved searches | 4 saved searches **V** | **No** |
| `<username>.db` facts (v9, floor 8, one rung `lib.rs:1484-1496`) | daemon `record` → everything | current state: a refetch of 3,606 tabs. **History is lost**: `first_seen`, 50,847 `item_events`, 223 removed items, `seen_response` | current state yes, history **no** (§2 T3) |
| `daemon.db` jobs (v2, rungs 0/1→2 `jobs.rs:246-267`) | daemon → restore, and C45 offline reads | waiting work and old results | work: yes, via `plan.unspent` (C111) |
| `accounts.json` (no version; legacy `keyring_name` shim `index.rs:46-74`) | daemon → frontends, restore | which keyring entries to load | by re-login |
| `rails.json` (unknown fields carried) | daemon, offline reset | block mark, dead grant, closed routes | **no** (safety state, not user state) |
| `fault.json` (`queue.rs:64`) | daemon panic → next restore | one "don't replay" | exists only because rows replay |
| keyring entry `(service, username)` holding JSON (or a legacy bare token, C135) | daemon | the session | only by a human login |
| `<root>/daemon.lock`, real-mode `run/ggg.lock` | daemon → clients (holder pid) | nothing | runtime |
| log dir: `daemon.log`, `sends.jsonl`, `world` marker | daemon → people, diagnose | diagnostics | n/a |

Sizes (**V**, dbstat): facts 99.6 MB (`items` 68 MB, `item_events` 6 MB, `responses` 5.3 MB). In `daemon.db`, `jobs` is 66.3 MB; the 3,498 done `stash` outcomes are 61.4 MB, each the whole body (`kinds.rs:629-637`). Rows span 2026-11-03 to 11-20, with 3 active days. Retention prunes only at restore (`queue.rs:326`), so a daemon that never restarts keeps every body.

The revision and version schemes I count are 22:
- **File and data stamps (8):** facts, annotations and jobs `user_version`; the facts revision (max response id); row revision and file revision sum; the keyring entry's two shapes; accounts.json's legacy field.
- **Per-value versions (3):** sync-policy v3, buyout v1, saved-search v2.
- **Build identities (3):** contract revision, artifact SHA, search build.
- **Table versions (8):** derivation, classes, categories, beast colours, totals, defences, properties, currency and notes, as the basis carries them, though the basis also carries the facts version.

## 2. Theses

**T1. The queue is memory; the ledger keeps terminal results, by pointer.**
- **What persisting buys today:** resume after `daemon stop`, after a version replace, or after a crash; `acq result <id>` across lifetimes; ids never reused. Idle exit already never strands work (C3). C25's own *why* ("a halt that failed its never-sent jobs made the rerun refetch everything") predates C111's spend-by-response, which makes the rerun send only the unspent (`plan/lib.rs:1590-1670`) **V**.
- **What goes (V by `wc`/`grep`):**
  - `restore_jobs` and the fault marker, `queue.rs:252-678` (~420 lines).
  - Most of `store/src/jobs.rs`' 789 production lines, replaced by an append-only results table (**I**, about 150).
  - The jobs v2 ladder.
  - C47's tainted-tree arm and C50's restore arm.
  - The not-replayed no-probe arm, the held mid-fan-out parents, and the transition-level sticky write failure.
  - The re-validation of restored `apply` rows (`kinds.rs:579-585`).
  - The 61 MB of bodies.
  - Tests: about 1,150 lines of `daemon/tests/queue.rs:1828-2975`, `evict.rs` (223), `store/unit/tests/jobs.rs` (390), and the CLI and MCP `persisted_queue.rs` (229).
- **Rulings:** C6 is rewritten (ledger of results). C45 narrows: with no daemon there is no outstanding work, by construction. C25 loses "its successor holds the queue". C134 loses its marker; the abort stays. C133 loses "for C6 to replay". C136's motive (C50's restore rule) goes.
- **Payload as a pointer:** done-job payloads become a summary plus `response` id. The failure that keeps a body (C144, `not_recorded`) keeps it. Nothing reads a stored stash payload but generic result printing:
  - `jobs_cmd.rs:441` pretty-prints it.
  - The MCP's own text says "prefer the store tools" (`mcp/src/server.rs:917`).
  - Only `leagues` reads a field (`run.rs:231`).
- **Why the edge cases go:** every restore arm answers "what if this row replays". No replay, no arms.
- **Smallest inputs that change:**
  1. `acq refresh --apply`, then `acq daemon stop`: today the next use verb resumes about 3,000 children. Under T1, stop reports "N waiting dropped", and re-running the same apply sends exactly the unspent.
  2. A tripwire or block halt with waiting jobs, then idle exit, then `reset-tripwire`: today the successor sends the old queue unprompted. Under T1 nothing sends until a person re-runs.
  3. `acq stash X --json` and `acq result 17`: they return `{response: N, …}`, not the body.
  4. A panic mid-send: no faulted replay arm, and nothing replays.
- **Invariants:**
  - 1 and 2 are untouched.
  - 3 is strengthened: no automatic replay after a block reset.
- **Security:** a gain (point 2 above).
- **Performance:** about 3 SQLite writes per job and 60 MB a week gone.
- **Function:** loses automatic resume across replace or stop.
- **An agent's ease:** gains. `acq jobs` with no daemon is simply empty, and `Interrupted` (13 references **V**) mostly goes.
- **Strongest objection:**
  - At the account rate (`x-rate-limit-account: 15:10:60,30:300:300`, read from a stored outcome **V**), a full refresh of 3,606 tabs is hours long **I**. An upgrade in that window drops the queue, and someone must re-run apply.
  - An agent loop re-running after a crash re-sends the request that broke the daemon, once per spawn, still paced by probe and limiter.
  - The owner's real ledger holds one C50 lost-login failure (**V**, `sqlite3 … where kind='profile'`), so restore paths do fire in real use.

**T2. An account is its uuid everywhere.**
- **Every username key (V):**
  - the facts file name (`index.rs:157`);
  - the keyring entry (`auth.rs:616`) and the `keyring_names` machinery that exists to chase it across renames (26 references in `index.rs`, 25 in `refresh.rs`);
  - the session table rename (`daemon.rs:621`);
  - limiter keys `route@username` (`limiter.rs:92-114`);
  - `rails.json`'s `refresh_failed_by_account` and its closed-route keys (`world.rs`, `RailsState`);
  - `jobs.account`;
  - the journal's `route`;
  - `plan.unspent`'s comparison of `job.account` against the current username, while the plan itself is keyed by `account_uuid` (`plan/lib.rs:1345`).
- **What goes:** `filename_safe` collisions, the reserved-stem escape and `HoldsAccountFacts` (`index.rs:147-225`), the keyring-name chase, the rename-follow in refresh, and store.md's "Not doing" line on reserved world files. `accounts.json` shrinks to uuid → display name, since the keyring cannot enumerate.
- **Smallest breaking input:** a GGG rename. Today:
  - the facts file is orphaned: hours of refetch, and the history in T3 is gone for good;
  - the dead-grant mark and closed routes don't follow;
  - jobs queued under the old name escape `unspent`'s queued check, a possible double send (**I**).

  Under T2 nothing happens.
- **Rulings and invariants:** C50 and C49 are strengthened. C31's Account rules are per GGG identity, so the uuid is the truer key. Invariant 5 is untouched.
- **Costs:** a one-time move of the facts file and keyring entry, or a re-login and refetch, which today is the owner alone. The journal names a uuid, which is less readable.
- **Objection:** the token response names the username before the profile fetch does. But C144 already stages that token until the profile lands, so a save under the uuid is natural.

**T3 (must stay). Facts cannot be re-derived from what persists; keep the one migration rung, and stop telling users facts are refetchable.**
- **Evidence (V):**
  - `responses.envelope` is "the body with item arrays removed" (`schema.sql:16`).
  - `items.json` holds only the latest version.
  - `Store::rebuild` re-extracts 9 item columns from item JSON (`lib.rs:2572-2592`). It rebuilds no tab or character column.
  - `container` is "NULL: recorded before facts v4" (`schema.sql:96-98`), the standing precedent of an ingest fact no rebuild can recover.
  - `first_seen`, `seen_response`, `removed_at` and `item_events` are products of ingest order.
- **Smallest breaking input:** delete the facts file, which is the remedy `FACTS_BELOW_FLOOR_REMEDY` and `HoldsAccountFacts` both print. After the refetch, all 44,411 `first_seen` become the refetch time, `acq store events` and the MCP's `item_events` are empty, and `--membership removed` loses its 223 rows. Search shows `first_seen` (`search/derive.rs:203`).
- **What it removes:** nothing. It costs about 30 lines of rung. The alternative, making facts replayable, means keeping full bodies, which `daemon.db` does by accident at 61 MB for 3 days. Done every refresh, that grows without bound unless item versions are deduplicated, a new mechanism I don't propose.
- **Rulings:** C35's "facts are refetchable" and C124's remedy need the words "history is lost". That is a ruling for the owner: either history is kept state, or it is disposable and the docs say so.

**The smallest set that loses no irreplaceable state (answering the charge):**
- **Persisted things:** intent per uuid, `user.db`, the facts file (for its history), `rails.json` (safety), and the keyring entry. Everything else is cache or runtime.
- **Revision schemes:** the facts `user_version` with one rung, the annotations' `user_version`, row and file revisions, the 3 value versions, the facts revision, and the 3 build identities.
- **What goes:** 4 persisted things (the jobs queue, the bodies in the ledger, `fault.json`, and `accounts.json`'s username role) and 3 schemes (the jobs ladder, the keyring-name shim, and the 10 per-table versions in the basis as equality inputs).
- **Smallest breaking input for each:** T1 items 1-3, a rename (T2), and §6's basis.

## 3. One thing I would defend

The intent file's revision sum in every answer's basis, and C167's separate `user.db` with a revision no basis carries. A reader would merge them, but that split is what keeps a saved search from reloading every held corpus.

## 4. Where the old way is right

- **Daemon lifetime (Q5).** A restart is already nearly a non-event for invariant 2. C20's per-lifetime HEAD reads GGG's state headers before the first send. Only `/profile` and `/account/leagues` send blind, and restore already refuses to replay them (`queue.rs:590-604`).
  - The one place long life matters is the idle verdict's hold on `choke.is_live` (`server.rs:679-689`), at most a 300 s window.
  - The machinery measured (production lines **V**): `client.rs` 2,261, `door.rs` 484, client `artifact.rs` 196, `locator.rs` 106, `endpoint.rs` 136, `life.rs` 228, daemon `artifact.rs` 72, `world.rs` 790. That comes to about 4,270 lines, and it pays for one process shared by many frontends behind one gate (C31, invariant 1), not for a long life. It stays.
  - Here I part from core-runtime H3: persisting the limiter's prediction adds a file whose content the next probe overwrites.
- **Facts migration:** stays, per T3.

**Where I part from the scouts:**
- **design-theses T2:** I agree, but it misses the gain for invariant 3, and that C111 retired C25's *why*. Keep `result` via a pointer ledger instead of dropping it.
- **design-theses T5:** wrong as stated. The bodies are not verbatim, and its proposed test fails on `container` and `first_seen`.
- **design-theses T6:** I agree. Add the keyring entry and the `unspent` username comparison.
- **store-plan-verbs:** its "three migration ladders" are two with rungs. The annotations file's floor equals its version.

## 5. What I did not check

- T7, the protocol read side.
- Whether `acq web` persists anything outside the store; a grep for `fs::write` found none.
- Real refresh duration: "hours" is inferred from the headers.
- The mock world: I ran none.
- Contents of the real `rails.json`: absent from the copy.
- The log dir: `real/logs` is absent.
- Windows `.pid`.

## 6. Small finds

- `Basis::against` omits `properties` (its 14-part array, `search/src/corpus.rs:~217`). The text basis omits it too (`search_cmd.rs:764`). A properties-only change refuses a continuation with an empty difference list. **V**
- The basis JSON has 15 fields, the text 13. Code reads only `==` (`corpus.rs:863`, `answer.rs:1077`, `answer.rs:2528`), `snapshot.at` (age) and `account` (scope fallback). `search/build.rs` INPUTS digest every table's source plus the store source, so 10 fields are implied by `build`. One digest plus a detail view suffices. **V**
- `facts_version` in the basis is a per-build constant, because a file at another version is refused (`lib.rs:1415`). **V**
- `Store::rebuild` skips the extracted tab and character columns (`lib.rs:1660`, `lib.rs:2919` vs `lib.rs:2572`). **V**
- Jobs retention prunes only at restore (`queue.rs:326`), so a long-lived daemon keeps every body. **V**
- C25's *why* is superseded by C111. **V**
- The dead-grant mark is keyed by username (`RailsState.refresh_failed_by_account`); core-runtime noted it as standing. **V**
- The orchestrator's "3 days": the rows span 11-03 to 11-20 (3 active days). **V**
