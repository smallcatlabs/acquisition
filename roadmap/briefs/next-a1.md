# next-a1 — one seeded world, one command

Step A1 of `audits/whole-review-2026-11/NEXT.md`, cut from its "A1.
The world" and from `audits/whole-review-2026-11/next/reports/world.md`
(an investigator's reading: where it and the tree differ, the tree
holds, and the report says so). Written by the orchestrating session
on 2026-11-25 against `526bcbfa`. No new ruling.

## Goal

One table describes a store shaped like the owner's, one function
writes it through the store's own ingest, and one command puts it on
disk, so that a test, a seat, the counted tasks of the next step and a
builder checking a change all read the same world with no daemon and
no login. Today each frontend test builds its own small world, two of
them copied by hand between the CLI's tests and the MCP's, and the
copies have drifted.

## Inputs

- C1 (the dependency edges) and C122 (a behaviour pinned once; the
  process tests share one harness): the world is a file under
  `tests/`, included by `#[path]`, and adds no edge.
- C34 (one mutation path per layer): facts through `Store::record` and
  `Index::record_login`, intent through the annotation API.
- C153 (a finding joins the surface that holds it): a variation is a
  row, never a file or a function of its own.
- `NEXT.md` "A1. The world": the contents and the done-when.
- `next/reports/world.md` §1, §2, §4: the write calls, the seeds sorted
  into four base worlds (A, P, S, M), the home.
- `crates/acquisition-client/tests/harness/journey.rs`: the form of a
  facts table two doors read.
- CI at `526bcbfa`: Gate (Linux) run 45739542401, success; Gate
  (Windows) run 45739542198, in progress. If it ends red, report it
  and go on.

## Decisions

1. **A table and one writer.** The world is data: accounts, and for
   each its realms and leagues, the tabs listed, the tabs fetched with
   their items, characters, and its intent (prices, a sync policy, a
   saved search). One function takes the table, a store directory
   (what `ACQ_STORE_DIR` names; it writes under `mock/`) and a time,
   and records it. A variation is a method on the table that drops,
   adds or changes rows (the world with one account; a fetch aged by
   an hour), and returns a table. Rules out a seed function per test
   and a builder with a call per fact.
2. **The owner's shape, fixed first; tests come to it.** Its contents
   are `NEXT.md`'s: an account with two realms and three leagues, a
   never-fetched tab, two prices, a saved search and a policy naming a
   league the store does not hold; a second account with a handful of
   items; a third that only logged in; one tab name and one item note
   holding hostile markup (`<`, `&`, a quote, a `<script>`). The P and
   S worlds become parts of it under the ids their tests already name;
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
   this world orphans its intent; the module doc and the skill's line
   say it is for reads, and applies keep the mock's world. Nothing in
   the daemon changes.
5. **Names.** `crates/acquisition-client/tests/harness/seeded.rs`
   (`world.rs` in the CLI's tests is the store directory's lock,
   another thing), and
   `crates/acquisition-verbs/examples/seeded-world.rs`, whose `main`
   is the one writer over a directory argument and a printed path, as
   `crates/acquisition-search/examples/copy-as-store.rs` is.
6. **A golden changes only by what the wider world adds.** A converted
   test asserts what it asserted. Each regenerated golden is named in
   the report with one line on what entered it; a golden that loses a
   fact is a fault of the conversion.

## Find first

- `git grep -n -E 'fn (seed|world)[a-z_]*\(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`:
  the seeds; sort them against the report's four worlds and say where
  it is wrong.
- `git grep -c 'record_login(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`:
  the count the step lowers; report its sum before and after.
- `git grep -n -E '(price_json|search_json|search_tools|pricing_tools)::|[^_a-z]seed(_at|_more|_bob)?\(' -- crates/acquisition-cli/tests crates/acquisition-mcp/tests`:
  every caller of the P and S seeds, each a test that must pass
  unchanged in what it asserts.
- `diff <(sed -n '/^fn seed/,/^}/p' crates/acquisition-cli/tests/search_json.rs) <(sed -n '/fn seed(/,/^}/p' crates/acquisition-mcp/tests/search_tools.rs)`,
  and the same for the price pair: where the copies drifted.
- `grep -n -E 'examples|Index::load|Store::open' tools/arch-check.sh`:
  what the checks say of an example and of who opens a store.
- `./target/debug/acq --help`, after a build: the verbs that read the
  store and need no daemon.

## Done when

- `git grep -c 'record_login(' -- crates/acquisition-cli/tests/price_json.rs crates/acquisition-cli/tests/search_json.rs crates/acquisition-mcp/tests/pricing_tools.rs crates/acquisition-mcp/tests/search_tools.rs`
  prints nothing: the four files seed through the world.
- One seed of world A reads the one-account variation, and one seed
  of world M reads the world; the last commit's message names both
  and the sum of the second "Find first" command, below 44.
- A test in the CLI's tests writes the world, runs every verb the last
  "Find first" command found over it, and holds that each exits 0 and
  no daemon appeared (`no_daemon_appeared`).
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
- Converting the other seeds. The orchestrator files them as the
  step's tail; `seat_faults.rs` stays whole.
- The call and byte counters, and any task table: the next step.
- A page test over the hostile row, and any golden under
  `goldens/web/`: step B4.
- The plan crate's private worlds, #142's scenario corpus, and one
  convention for where a test's store sits.

## Rules

`TEMPLATE.md`. What differs:

- Commit to `main` as each concern passes its tests; push once, after
  the gate, alone in its command. A refused push is left and named in
  the report.
- Order no cold review, and do not read `roadmap/reviews/next-a1-*`:
  the orchestrator runs the reviews and sends you what to answer.
- A second builder works in a clone outside this tree and touches only
  `crates/acquisition-daemon`. If your gate goes red in a test that
  times a daemon, say so before retrying: two lanes under load is a
  trial.
- No change to `decisions/`, `AGENTS.md` or `roadmap/README.md`.

## Report

`TEMPLATE.md`, and with it: each golden regenerated and what entered
it; each place a test's assertion had to change, and why.
