# Investigator's report: one world and a table of counted tasks

As returned by an Opus investigator on 2026-11-25, read-only, no cargo and no `acq`. It was asked what it would take to have one owner-shaped world built by one function with no daemon, and a table of player tasks counted in calls and bytes through both doors. Its claims are its own. "Inferred" marks a conclusion it drew and did not read in the code.

## Where the background it was given is wrong or needs a caveat

- **Seed counts.** 23 named seed or world functions in the CLI and MCP tests, not 34 (the list is below). There are 44 `record_login(` calls (`git grep -c 'record_login(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`), 14 of them in `seat_faults.rs`, so a lot of seeding is written inline in the tests. `git grep 'record('` finds 256 lines. Only 10 of them are literally `store.record(`; most go through local `record` closures. Issue #142 itself says "18 seed functions".
- **Seats already measure calls and bytes, in Python.**
  - `.claude/skills/seat/seat.py:64` writes `task, stdout_bytes, stderr_bytes` per call.
  - `seat-mcp.py:32` writes `text_bytes, result_bytes`.
  - `seat.py setup --facts` copies one of the owner's real facts files into a seat world (`seat.py:139-142`, through the `copy-as-store` example in `crates/acquisition-search/examples/`). So a seat can already sit on real facts, but only one account.
- **The mock-session skill sets five variables, not seven.** It exports five and unsets four (`SKILL.md:21-25`).
- **The first MCP turn.** `surface_size.rs:21-26` records 42,089 B against a bound of 43,000. The audit says 42,548 B.

## 1. Building a world without a daemon

**What exists.** The store's write side is public and needs no daemon:

- `Store::record(endpoint, params, status, body, at)` at `acquisition-store/src/lib.rs:1527`.
- `Index::record_login` at `index.rs:310`.
- `Annotations::open_for(..).put::<K>` at `annotations.rs:757,1059`.
- Saved searches live in the store too (`user.rs:16,44`).

The tests already build worlds this way, for example `seat_faults.rs:24-62`.

**Could one function do it?** Yes: one store-level function could write the whole owner-shaped world. Three things push back:

- **The typed intent values are in the plan crate.** `SyncPolicy` is at `acquisition-plan/src/lib.rs:483`, `put_sync_policy` at `:931`, and `Buyout` at `price.rs:521`. The client cannot link the plan crate (C1), and `client/tests/harness/mod.rs` is included by the client's own tests. So the builder must be a separate file that only the CLI, MCP and verbs tests include, or it must write untyped values through a local `IntentValue`, as `seat_faults.rs:2861` does. That second route skips the plan's validation.
- **The mock's bodies are private daemon code.** These are `MOCK_TABS` (`mockggg.rs:270`), `MOCK_LEAGUES` (`:304`), the stash, list and character functions (`:506-640`), and the profile uuid hash (`:229-242`). Only `mock-items.json` is data. Another crate can read it with `include_str!` without adding a dependency edge; the precedent is `acquisition-cli/src/web/fill.rs:217`. The tree and the leagues are Rust constants, so they would have to move into the JSON for one table to feed both the mock and the builder.
- **Seeded accounts and later mock logins collide (inferred, the hardest part).**
  - The mock serves the same stash to every username; only the profile uuid varies (`mockggg.rs:229`, `:589-640`). In an owner-shaped world, a refresh of account B (10 items) or C (0) against the mock would ingest the mock's 37 items over them.
  - A seeded account has no token, because the isolation sets `ACQ_NO_KEYRING=1` (`client/tests/harness/mod.rs:181`). Any daemon verb for it needs a mock login first.
  - That login calls `record_login`. When it finds the account by username it overwrites the uuid (`index.rs:337-343`). The annotation file is keyed by uuid, so any seeded uuid that is not the mock's hash orphans the seeded prices, policy and saved searches.

  So a seeded multi-account world is good for reads (search, tabs, price, show, an offline `refresh --plan`) but not for apply. Apply needs a mock that serves each user its own table.
- **The daemon side.** Nothing new is needed from the daemon for login state as long as the world is read-only. The account index writes are already in the store.

## 2. The seed functions

| World | Functions (file:line) | Shape |
|---|---|---|
| A: Alice, pc Standard, small | `plan_json.rs:28`, `price_story.rs:93`, `store_answers.rs:25` (a closure), `diagnose.rs:35` (profile, plus logs and journal), `seat_faults.rs:3805` (index and an empty file), MCP `tool_refusals.rs:22` (plus a policy), `ggg_refusal.rs:31` (provider `ggg`, profile only) | One account, one league |
| P: priced | `price_json.rs:53/60` (`seed_at`, aged), `saved_json.rs:94` (calls `price_json`), `shop_json.rs:36` (plus a policy), MCP `pricing_tools.rs:34` (says itself it copies `price_json`) | Folder, public tab, map substash, a character, buyouts |
| S: search | `search_json.rs:31`, `:102` (a poe2 second realm), `:1857` (categories), MCP `search_tools.rs:77` (the same ids as `search_json`), `:128` (a second account, Bob) | A never-fetched tab, an unread implicit |
| M: multi-scope | `seat_faults.rs:19/24` (two leagues), `:2872` (plus Rehearsal and xbox), `web.rs:38` (a league list with Settlers not on disk, xbox Hardcore), MCP `spoken.rs:32` (pc plus poe2, a tab gone from the listing), `tool_refusals.rs:513` (two accounts, Bob with none, xbox Allflame) | |

Two worlds are duplicated across crates only because no shared file exists: P (CLI and MCP) and S (CLI and MCP). They also put the store in two places, `base/mock` and `base/store/mock`.

The variant rows the one world would need:

- a second account that is empty
- a second realm
- a league that is listed but not held
- a policy league that is not held
- a tab gone from the listing
- a never-fetched tab and a folder
- an unread implicit array
- aged fetches (`seed_at`)
- provider `ggg`
- leftover logs and journal (`diagnose`)
- an index entry with no facts

About 4 base worlds, plus roughly 11 variations.

## 3. The journey as a task table

- **What it scripts.** The CLI journey (`acquisition-cli/tests/journey.rs:44+`) starts a daemon with the journal on, logs in as the mock, then:
  - writes a policy, then plans and applies three times;
  - runs tabs, three searches, show, price set, shop and price clear;
  - stops the daemon.
- **Each door's check.** The CLI is held by a 15,221 B golden whose `# holds` facts come from the table. The MCP twin (`acquisition-mcp/tests/journey.rs:24+`) compares fields to the same table.
- **The table.** `client/tests/harness/journey.rs` holds only constants: the policy, `PLAN_REQUESTS [2,9,7]`, the tab and item counts, the query strings with their match counts, the price, and the GETs per route. It uses the standard library only.
- **The MCP driver exists.** `Mcp` is at `mcp/tests/harness/mod.rs:162-370`. It has no byte counter; `rpc` drops the line length (`:258-275`), so adding one is small.

What is missing for a task-table golden:

- **The task rows themselves.** Each must script the naive first call, or no refusal ever appears; that is judgment.
- **A per-task tally** in both drivers.
- **Deterministic numbers.** MCP polls are wall-clock (`finish_job`, `:84-96`, sleeps 100 ms), so the "23 calls" figure cannot be a golden fact unless polls are counted apart.
- **Bytes held as bounds, not exact values (inferred).** Exact byte counts move with every rewording, which recreates the churn the audit measured. Bounds are what `surface_size.rs:15-26` already does.
- **The world itself.** "Refresh to nothing to do" needs a logged-in account that matches the mock's bodies. "Is this priced" and "90+ life" need the seeded world. One table therefore needs both the mock and the seed to produce one world.

## 4. Where it would live

The smallest home is a standard-library-plus-store file, `acquisition-client/tests/harness/world.rs`, included by `#[path]` the way `journey.rs` is (`cli/tests/harness/mod.rs:14`, `mcp/tests/harness/mod.rs:15`). Its consumers:

- **Frontend tests.** The CLI, MCP and verbs tests include it; they already link store and plan.
- **A dev binary.** An example, `acquisition-verbs/examples/world.rs`, would `#[path]`-include it, following the `copy-as-store` example. A `src/bin` in the CLI would fail the door grep (`tools/arch-check.sh:555-557` refuses `Index::load|Store::open|Annotations::open_for` in the CLI's and MCP's `src`).
- **The store and the search tests** can include it, since they link the store.

No new dependency edges are needed. Making the mock serve the same table needs the bodies as data, meaning JSON read with `include_str!`. A shared Rust crate would be a new daemon edge, which C1 forbids without a ruling.

## 5. Smallest first step, and what it would not do

**First step.** Write `world.rs` holding world P∪S: Alice in pc and poe2, Standard and Rehearsal, a never-fetched tab, two buyouts, a saved search, and a policy naming "Typo"; plus Bob with no facts. Expose it through one `seed(base)` function and a facts table of constants. Move the two cross-crate duplicates onto it first: `pricing_tools` to `price_json`, and `search_tools` to `search_json`. Then add the example binary so a seat or an ad-hoc session gets a populated world in one command, with no daemon and no login.

**What it would not do yet:**

- Make the mock serve the seeded world per user.
- Put byte counts into goldens.
- Dissolve `seat_faults.rs`.

**Mechanical work** (a clear rule, a cheaper model can do it):

- Converting seeds that are world A plus a variation into `seed()` plus a variant call.
- Unifying the two store-path conventions.
- Adding the byte and call counter to `Mcp`.

**Needs judgment:**

- The world's contents, which ids each test asserts.
- The variant rows.
- The uuid rule: use the mock's hash, or accept that the seeded world is read-only.
- The intent-typing choice.
- Task rows that script a naive agent.
- Whether the mock's tree moves into JSON.
