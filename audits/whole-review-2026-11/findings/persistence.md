# What persists, and why

The plan's D5. Investigator's report: `reports/investigators/persistence.md`. Portfolio: proposals 6 and 9.

## What was found

| Persisted thing | Irreplaceable? |
| --- | --- |
| Intent, one file per account uuid (the owner's: 1 policy, 16 prices) | Yes |
| `user.db`, the saved searches (4) | Yes |
| The facts file, named by username (99.6 MB) | Its current state is a refetch away. Its history is not: `first_seen`, 50,847 item events, 223 removed items |
| `rails.json`, the halts | Safety state |
| The keyring entry | A person's login |
| `daemon.db`, the job queue and ledger (69.6 MB) | No: a plan re-applies and spends each request once (C111) |
| `fault.json`, `accounts.json`'s username role, the locks, the log | No |

The investigator counts 22 revision and version schemes. Ten of the thirteen identifiers on an answer's basis line are implied by the search build's digest; code reads the basis only for equality, the snapshot's age and the account.

## Checked by the orchestrator

- A done `stash` job's payload is the whole body (`kinds.rs:629-637`), and `db.prune` is called from `restore_jobs` alone (`queue.rs:326`). So the ledger keeps every fetched body until the next daemon start: 60.0 MB in 3,498 rows on the owner's disk.
- `Store::rebuild` (`lib.rs:2572-2594`) re-extracts nine item columns and nothing else.
- Correction to phase 0: the job rows span 2026-11-03 to 11-20 with three active days.
- Not checked: that a full refresh takes hours (inferred from the rate headers), and the mock world.

## Where this parts from the plan

- **The advisor's T5 dies.** Facts cannot be re-derived on a schema mismatch: the stored envelope has the item arrays removed, only the latest version of an item is kept, and the history is a product of ingest order. The one migration rung stays; it is about 30 lines.
- The advisor's T2 (the queue need not persist) holds, and misses two things: it strengthens invariant 3, and C25's stated reason was retired by C111.
- The core-runtime scout wanted the limiter's prediction persisted. A restart is already nearly a non-event for invariant 2, because the first probe of a lifetime reads GGG's own counters; persisting a prediction adds a file the next probe overwrites.
- The machinery that keeps one daemon alive and replaces it safely (about 4,270 production lines) pays for one process behind one gate, not for a long life. It stays.

## Position

**The queue is memory.** A ledger keeps terminal results, a done fetch's result being a summary and its response id. Goes: `restore_jobs` and the fault marker (about 420 lines), most of `store/src/jobs.rs` (789), the jobs ladder, the tainted tree, held mid-fan-out parents, most of `Interrupted`, about 2,000 test lines, 60 MB a week. The smallest inputs that change: after `acq daemon stop` mid-refresh, the next apply sends the unspent instead of the daemon resuming alone; after a block's reset nothing is sent until someone asks. The second is a gain for invariant 3. The loss is real: a refresh of thousands of tabs interrupted by an upgrade must be re-run by someone.

**The account is its uuid everywhere.** A rename at GGG today orphans the facts file and its history.

**Say that history is not refetchable**, or rule that it is disposable.

## Route

Proposals 6 and 9. Issues drafted: `ledger-prunes-only-at-restore`, `facts-are-not-wholly-refetchable`, `c25-why-is-superseded`, `basis-against-omits-properties`, `rebuild-skips-tab-and-character-columns`, `a-rename-may-double-send`.

## After the skeptic

**"The queue is memory" dies** (`dissent/theses-6-10.md`, thesis 6).

- C134's promise that a body which breaks the daemon is sent once rests on a marker that outlives the process and on the row it names. With neither, re-running the apply recreates the probe and the crash: a loop the fault quarantine exists to prevent (`daemon/tests/exit.rs:1656`).
- C111 spends a request by a landed response or a live job. It keeps no record of an attempt that failed.
- A stored response has its item arrays removed and `items` keeps one current row per item, so a done job's payload is not a second copy of the facts. It is the only place that response survives as it was. "Every fetched body kept twice" was too strong.
- Login reads `payload.body.uuid` from its profile job (`login.rs:244`).

What stands as a defect: the ledger prunes only at a daemon's start and holds 60 MB after three days of use. What stands as a question for a later session: what resume across a restart is worth, now that the two inputs that break its removal are written down.

**"The account is its uuid" is narrowed** (thesis 9). The diagnosis stands: `record_login` orphans the old facts file on a rename (`index.rs:300`) and no refetch brings its history back. The deletion list was too wide: `filename_safe` also serves the uuid-named intent files; logout needs the old keyring names until their cleanup has succeeded (`refresh.rs:119`); the first profile job runs before any uuid is known; an old index entry can lack one.
