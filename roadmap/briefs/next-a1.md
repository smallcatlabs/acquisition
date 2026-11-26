# next-a1 — one seeded world, one command

Step A1 of `audits/whole-review-2026-11/NEXT.md`, cut from its "A1.
The world" and from `next/reports/world.md` beside it (a reading:
where it and the tree differ, the tree holds). Written by the orchestrating session
on 2026-11-25 against `526bcbfa`; re-cut once, answering its cold
review (`roadmap/reviews/next-a1-brief-4e0cf51f.md`). No new ruling.

## Goal

One table describes a store shaped like the owner's, one function
writes it through the store's own ingest, and one command puts it on
disk, so that a test, a seat, the next step's counted tasks and a
builder checking a change read one world with no daemon and no login.
Today each frontend test builds its own, and the two copied between
the CLI's tests and the MCP's have drifted.

## Inputs

- C1 (the dependency edges) and C122 (the process tests share one
  harness): the world is a file under `tests/`, included by `#[path]`,
  and adds no edge.
- C34 (one mutation path per layer): facts through `Store::record` and
  `Index::record_login`, intent through the annotation API.
- C167 (a saved search is the world's, in `user.db`, not an
  account's): written through `User::open_for` and its typed put.
- `next/reports/world.md` §1, §2, §4: the write calls, the seeds sorted
  into four base worlds (A, P, S, M), the home.
- `crates/acquisition-client/tests/harness/journey.rs`: the form of a
  facts table two doors read.
- CI at `526bcbfa`: Gate (Linux) 45739542401, success; Gate (Windows)
  45739542198, in progress.

## Decisions

1. **A table and one writer.** The world is data: its saved searches,
   and accounts, each with its realms and leagues, the tabs listed,
   the tabs fetched with their items, characters, and its intent
   (prices, a sync policy). One function takes the table, a store directory
   (what `ACQ_STORE_DIR` names; it writes under `mock/`) and a time,
   and records it. A variation is a method on the table that drops,
   adds or changes rows (one account alone; a fetch aged by an hour)
   and returns a table. Rules out a seed function per test.
2. **The owner's shape, fixed first; tests come to it.** Its contents
   are `NEXT.md`'s: an account with two realms and three leagues, a
   never-fetched tab, two prices and a policy naming a league the
   store does not hold; a second account with a handful of items; a
   third that only logged in; a saved search; one tab name and one
   item note holding hostile markup (`<`, `&`, a quote, a `<script>`).
   The P and S worlds, with the variations their own files seed (a
   second realm, the categories, a second account), become parts of
   it under the ids their tests already name;
   where the MCP's copy has drifted from the CLI's, the world holds
   what both need. A count the world decides (a tab's items, a query's
   matches) is a constant beside the table, read by both doors, never
   a literal repeated in two crates.
3. **Typed intent.** Prices and the policy go through the plan crate's
   own typed puts, so the world cannot hold a value the planner
   refuses. The file is therefore included only where the plan crate
   is linked (the CLI's and the MCP's tests, the verbs crate's
   example), never from `acquisition-client/tests/harness/mod.rs`.
   Rules out a local untyped value.
4. **For reads.** Its uuids are its own. The mock gives every user one
   stash and a login overwrites a seeded uuid, so a mock login over
   this world orphans its intent; the module doc and the skill say it
   is for reads. Nothing in the daemon changes.
5. **Names.** `crates/acquisition-client/tests/harness/seeded.rs`
   (the CLI's `tests/world.rs` is another thing), and
   `crates/acquisition-verbs/examples/seeded-world.rs`, whose `main`
   is the writer over a directory argument and a printed path, as
   `crates/acquisition-search/examples/copy-as-store.rs` is.
6. **A golden changes only by what the wider world adds.** A converted
   test asserts what it asserted, and a golden that loses a fact is a
   fault of the conversion.

## Find first

- `git grep -n -E 'fn [a-z_]*seed[a-z_]*\(' -- 'crates/acquisition-cli/tests/*.rs' 'crates/acquisition-mcp/tests/*.rs'`:
  the candidates. Sort them against the report's four worlds, say
  where it is wrong, and name the one A seed and the one M seed you
  convert.
- `git grep -c 'record_login(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`:
  the count the step lowers; report its sum before and after.
- For each function the first command finds in `price_json.rs`,
  `search_json.rs`, `pricing_tools.rs` and `search_tools.rs`:
  `git grep -n -w '<name>' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`,
  its callers, each a test that must go on asserting what it asserts.
- `git grep -n -W -E 'fn seed[a-z_]*\(' -- <file>` for each of those
  four files: read the CLI's writers beside the MCP's and list where
  the copies drifted.
- `grep -n -A4 '^doors=' tools/arch-check.sh`: who may open a store,
  and that an example is outside it.
- `grep -n -i 'uuid' crates/acquisition-daemon/src/mockggg.rs` and
  `git grep -n -W 'pub fn record_login' -- crates/acquisition-store/src/index.rs`:
  decision 4's two facts; say if either is wrong.
- `grep '^## ' CLI-REFERENCE.md`: every leaf command. A read is a leaf
  that answers from the store with `ACQ_NO_SPAWN=1` set; list them,
  and the leaves left out with the reason.

## Done when

- `git grep -n -E 'fn [a-z_]*seed[a-z_]*\(' -- crates/acquisition-cli/tests/price_json.rs crates/acquisition-cli/tests/search_json.rs crates/acquisition-mcp/tests/pricing_tools.rs crates/acquisition-mcp/tests/search_tools.rs`
  prints nothing: the four files' seeds are the world or a variation.
- One seed of world A reads the one-account variation, and one seed
  of world M reads the world; the last commit's message names both
  and the sum of the second "Find first" command, below 44.
- One test in the CLI's tests writes the world and, through the read
  leaves of the last "Find first" command, each given its account,
  realm and league by name, holds: each exits 0 and no daemon
  appeared (`no_daemon_appeared`); the three accounts, the realms and
  leagues, the never-fetched tab, both prices, the saved search and
  the policy's league are each answered; the hostile tab name and
  note come back as written.
- `cargo run -q -p acquisition-verbs --example seeded-world -- <dir>`
  prints the store directory it wrote, and the mock-session skill
  gives that command under a heading for read work, with
  `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` and `ACQ_NO_SPAWN=1`.
- `git diff --stat 526bcbfa -- 'crates/*/src' crates/acquisition-cli/tests/goldens/web`
  prints nothing.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- The mock serving this world, or its bodies moved into data: a later
  step, if the counted tasks need it.
- Converting the other seeds, and a test in the four files that
  records its own world inline: the step's tail, which the
  orchestrator files. `seat_faults.rs` stays whole.
- What a verb does when no account, realm or league is named: B1.
- The call and byte counters and the task table: the next step.
- A page test over the hostile row, and any golden under
  `goldens/web/`: step B4.
- The plan crate's private worlds and #142's scenario corpus.

## Rules

`TEMPLATE.md`. What differs:

- Commit to `main`; push once, after the gate, alone in its command.
  A refused push is left and named in the report.
- Order no cold review and read nothing under `roadmap/reviews/`: the
  orchestrator runs the reviews and sends you what to answer.
- A second builder works in a clone outside this tree, on
  `crates/acquisition-daemon` alone. If your gate goes red in a test
  that times a daemon, say so before retrying.
- No change to `decisions/`, `AGENTS.md` or the road.

## Report

`TEMPLATE.md`, and with it: each golden regenerated and what entered
it; each place a test's assertion had to change, and why.
