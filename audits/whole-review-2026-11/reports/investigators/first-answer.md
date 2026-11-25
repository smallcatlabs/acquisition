# Investigator's report: first-answer

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/first-answer.md`.

## The first answer: what it costs to get from nothing to a useful answer

My thesis: scope is resolved three different ways, and the plan path differs between the two doors. Those two things cost more than the size of the help text does.

### 1. The picture

**What the cold sitting cost.** I ran the real-store copy with read verbs only. Every call was logged by `<scratch>/first-answer/r.sh` to `calls.log`, with outputs under `o/`. **V**

| Task | Help read | Calls (refused) | Answer bytes |
|---|---|---|---|
| (a) what I hold, how fresh | `--help` 13,250 | 5 (1): `store status` refused for 3 accounts, with no item counts in the refusal; then `accounts`, `store status`, `leagues --stored`, `tabs` | 4,917 in total. There is still no single view of items and freshness per league: `tabs` silently chose pc Standard |
| (b) best life+res rings | `search --help` 11,284; `--describe fields` 23,042, which the help calls "the place to start" but which has no `pseudo.*` | 2 (1): realm refused | 7,254 as text; 35,383 as `--json` (16,647 compact, because it is pretty-printed) |
| (c) one named tab | none | 2: the default gives 10 of 87 rows, then `--limit 100` | 3,694, then 17,888 |
| (d) priced or not | `price --help` 3,259 | 1, silently Standard pc | 1,913 |
| (e) one item whole | none | 1 (`show f3f98dcc` refused: ids must be whole) | 1,702 |

Altogether: about 51 KB of help, 11 calls, and 2 scope refusals. The orchestrator's `life>=90` result reproduces. The near-names for `life` are `level`, `links` and `pseudo.life_regen`; for `res` they are `resolve` and `resolve.of`. **V**

**The MCP first turn** in the mock world is `initialize` at 1,157 B plus `tools/list` at 41,391 B, so 42,548 B. **V**

`tools/list` breaks down as follows. **V**
- Tool descriptions: 14,194 B.
- Parameter descriptions: 15,290 B, of which the `account` parameter is 5,624 B.
- Schema: 12,027 B.

**The refresh loop to "nothing to do"**, measured with `first-answer/flow.py`. **V**

| Door | Calls | Bytes |
|---|---|---|
| MCP | 23: 9 of them `job_status`/`job_result` polls | 23.0 KB read; 11.7 KB sent, of which the `apply_plan` envelope echo is 9.3 KB |
| CLI | 5: `policy set` plus four `refresh --apply` | about 1.9 KB stdout |

The MCP `leagues` tool refused `{}` because a field `stored` must be `true`. **V**

**How scope resolves: three policies in one tool.** **V**

| Noun | Search | tabs / price / refresh / policy / web / MCP | What pays for it |
|---|---|---|---|
| account | refuse when more than one is known (C121) | same | The owner declined a remembered account (`decisions/frontends.md:40`) |
| realm | refuse when more than one is held (`corpus.rs:964-975`, C96) | silent `Realm::DEFAULT` = pc (`pricing.rs:430`, `run.rs:327-467`, `params.rs:57`, `server.rs:536,635`) | C96: "a template means different things in two games" |
| league | every league | silent `DEFAULT_LEAGUE` = "Standard" (`pricing.rs:297,325,331`; 5 clap defaults in `cli.rs`; `server.rs:176,500,546,645`) | nothing written; `frontends.md:41` declines only a *remembered* league |

**The owner's store**, counted with `sqlite3` read-only. **V**
- Live items: pc Standard 43,433; pc Covens 604; pc Allflame 53; poe2 Standard 98.
- Accounts: 44,188, 10 and 0 items.

So for this owner the silent "Standard" happens to be right. For a challenge-league player it is wrong: a Covens player's `acq price` reads an empty Standard. **I**

The refusals in search guard nothing that `--realm all` does not already handle honestly. `search --realm all 'pseudo.total_life>=90'` answers 586 matches and marks 98 poe2 items undecided with the reason "no totals table for realm poe2". **V**

So the lesson from removing "the main account" was overlearned in `search`, which refuses, and underlearned everywhere else, which quietly reads fixed names.

### 2. Theses

**T1: Scope defaults to "everything held". It is never a fixed name and never a refusal, except for the account when two accounts hold items.**
- **What goes:** `Realm::DEFAULT` as a read default, `DEFAULT_LEAGUE`, `realm_needed`, and the 5 clap `default_value = DEFAULT_LEAGUE` args. That is the roughly 28 sites `git grep -n "Realm::DEFAULT\|DEFAULT_LEAGUE"` prints outside tests, plus the MCP parameter docs that say 'defaults to "Standard"'. **V**
  - Reads span every realm and league held, and the scope line already printed says so.
  - `price` and `tabs` give one line per league held instead of reading Standard.
  - A bare `refresh --apply` compiles the whole stored policy, every realm and league it names.
  - The account refusal stays, and carries each account's item count.
- **Why the edge cases go:** "which realm", "`--realm all` spelled out", "Standard when the target is not held" (`league_of`) and the web picker's default all exist only because a default had to be picked.
- **Smallest input whose behaviour changes:**
  - `acq search 'pseudo.total_life>=90'` on a two-realm store: today a refusal, after it 586 rows and 98 undecided.
  - `acq price` on a store holding only Covens: today "0 items in Standard".
- **Invariants and rulings touched:** none of the five invariants (reads; refresh still goes through the plan and the gate). C96 is reversed only in its default: realm stays scope, never a term. C38 holds: a bare apply is still exactly the compiled set. The "Not doing" lines at `frontends.md:40-41` hold: nothing is remembered.
- **Security:** neutral.
- **Performance:** a query over every realm reads 0.2% more items.
- **Function:** gains: the challenge-league player is served.
- **Agent's ease:** gains: in my sitting, 2 of 11 calls go, and the orchestrator's 5-try path loses one refusal.
- **Strongest objection:** poe2's 98 items show as undecided on every totals query over a 99.8% pc store. A `line(…)` count conflates pc and poe2 templates that are worded the same, which is exactly C96's concern. Mitigation: print undecided-for-want-of-a-table once in the scope line, not per term; rows already carry their realm.

**T2: The MCP plan path becomes the CLI's.**
- **What goes:** `refresh{apply:true}` (compile now, apply, wait, return the result) replaces `refresh_plan` → `apply_plan` → `job_status`×n → `job_result` per round.
  - Lost from the measured run: 9 polling calls and the 9.3 KB echo.
  - 23 calls become about 5, the CLI's own count. **V/I**
- **Why the edge cases go:** stale-envelope replay, `spent`/`remaining_requests` on a second apply, and the polling cadence are all artifacts of a client carrying the plan.
- **Smallest input whose behaviour changes:** the second round after a listing.
- **Invariants and rulings touched:**
  - Invariant 1: unchanged, the daemon still gates.
  - C38: holds, because bare `--apply` already exists on the CLI and compiles the policy, which is the reviewed intent (C52).
  - The reviewed-envelope form stays for anyone who wants to review.
- **Security:** neutral; `max_requests` is kept.
- **Performance:** fewer IPC round trips.
- **Function:** none lost.
- **Agent's ease:** large. The seat ledger's T2 count stayed at 18–21 through every fix round, because fixes polished the words, never the shape.
- **Strongest objection:** an agent stops seeing the plan before it is spent. But the CLI already allows that, and the policy write is where review sits.

**T3: The smallest surface that keeps every function.**

**MCP: 30 tools → 14.**

| Keep | Absorbs |
|---|---|
| `world` | `accounts`, `store_status`, `leagues`, the summary in `daemon_status` |
| `search` | — |
| `describe` | `reference_currency` |
| `show` | — |
| `tabs` | — |
| `price` | `price_status`, `price_list`, `price_show` |
| `price_set` | `price_clear` (value null) |
| `policy` | `sync_policy`, `set_sync_policy` (value optional) |
| `refresh` | `refresh_plan`, `apply_plan`, `submit_job` for listings |
| `jobs` | `list_jobs`, `job_status`, `job_result`, `cancel_job` |
| `saved` | read and set |
| `saved_clear` | — |
| `shop_render` | — |
| `item_events` | — |

`store_refused` goes to the CLI only (evidence for a person).

**CLI: 30 verbs → 21.** Delete:
- `demo`: 0 uses in tests or tools.
- `set-priority`: 0.
- `dash`: overlaps `daemon status`, `jobs` and `diagnose`.
- `submit`.
- `profile`, `characters`, `character`, `stashes`, `stash`.

What breaks:
- README lines 167–180.
- The tests that drive the deleted verbs.
- `tabs`' empty-state offer of `acq stashes` (it becomes `refresh --listing`). **V/I**
- No `tools/*.sh` driver or skill invokes the deleted raw verbs (by grep). **V**
- The live drivers go through `refresh`. **I**

**The new document:** `world` is about 0.8 KB for this owner (**I**: three account lines, four realm/league lines with item counts and oldest/newest fetch, and the exact `--account/--realm/--league` words). It lets go of three verbs and four MCP tools.

**First turn:** 42.5 KB → about 15 KB (**I**). This comes from 14 tools, an `account` parameter written once as a `scope` object, and the `describe` index cut to a 1.5 KB card.

**Strongest objection:** multi-mode tools move bytes from tool count into parameter docs. Also, the owner's rule in `surface_size.rs:22` ("a trim under 10% is declined; a new tool raises the bound") has to go first.

**The query language (question 5).** About 1.5 KB would have got every task right first time:
- class, rarity, base, name, tab, league, character, ilvl, links, priced, id;
- the ten `pseudo.total_*` names by the player's word (life, res, ele res, …);
- `line("text").arg1` for any other mod;
- the operators;
- `--sort`/`--desc`/`--limit`/`--count`;
- `show <whole id>`.

The vocabulary is the trade site's: `pseudo_total_life`, category ids, q20 defences. Players say "life", "res" and "rings".

What the language buys over a read-only SQL view:
- derived semantics that live in no column: class from a table, per-realm totals, three-valued undecided/lacked, line templates parsed from `items.json`, coverage and freshness, continuation bases;
- a view would have to materialize all of that (a cache) or leave each agent to re-derive it from raw JSON (a second semantics, C46).

C48 holds for agents, but its stated reason (staleness) is the weaker one. The real reason is C46. **I**

### 3. One thing I would defend
The scope and basis block on every answer (about 600 B). Another reader would cut it as noise. It is what makes the "everything held" default in T1 honest.

### 4. Where the old way is right
- The refusals themselves: each names the exact way on, and the agent never guessed wrong.
- Text output beats `--json` for an agent: 7.3 KB against 35.4 KB for the same answer. **V**
- The account refusal should stay when two accounts hold items.

### 5. Where I part from the scout and the ledger
- The scout ranks first-turn bytes first. Round trips from scope refusals and from the plan loop cost more, and fix rounds have not moved them.
- Seats are measured in pristine one-realm worlds. The owner's real world (three accounts, two realms) produces refusals no seat counts.
- The scout's "two doors share no naming" leads to renaming. I would delete instead.

### 6. What I did not check
- The MCP `search`'s bytes against the CLI's.
- The web pages.
- Non-Opus agents.
- Timings: one search took about 1.1 s in a debug build while other readers worked. To rerun alone: `time acq search --realm pc 'class:ring pseudo.total_life>=60'`.
- Whether tab ids are unique across leagues, which matters for scope taken from a target.

### 7. Small finds
- `bind.rs:530-556`: `near()` keeps the first 3 suggestions and breaks ties alphabetically, and a whole-word "contains" match ranks below a prefix match. So `life` never offers `pseudo.total_life`, and `res` never offers `pseudo.total_res`.
- `search --help` calls `--describe fields` "the place to start", but that 23 KB answer lists no `pseudo.*` total.
- The `refresh` help says "a first sync takes two rounds". Both doors took three applies: listing, tabs, substashes.
- `--json` output is pretty-printed: 35,383 B against 16,647 B compact.
- The MCP `leagues` tool's `stored` is required and may only be `true`.
- 12 of the 30 tool descriptions sit at 570–598 B, against a 600 B cap.
- The `ambiguous_account` refusal lists 3 accounts with no item counts; two of them hold 0 and 10 items.
- `tabs` with nothing on disk offers `acq stashes`, a raw verb, as its first way on.
