# 2c — The journey test and real-shaped mock items (build)

The Opus build of `2c-design.md`. A worker reads `roadmap/README.md`,
then this brief, then what it names, and nothing else unless the work
turns it up.

## Goal

When this step is done, the mock serves five real-shaped items of the
project's own making beside its Fake Items, so a user, a seat and a
test reach search and price through fetching; one journey from login
to `daemon stop` is held by a CLI golden transcript and an MCP twin
that read one table of facts, with the daemon's journal checked by
field; the `changed:` line counts one apply's events and no other's;
and the daemon-backed texts that 2b left as literals are goldens where
a rule can hold them. Why: the keystone phase ends with a tool whose
whole arc is pinned once, in words a reviewer reads as a diff.

## Read

- `AGENTS.md`: the hard rules (the mock-world rule; every `acq` you run
  has `ACQ_PROVIDER=mock` and `ACQ_STORE_DIR` under your scratch), C34,
  C46, C122, the gate (nextest is its runner).
- `roadmap/briefs/2c-design.md`, whole: sections 2 and 4 are the
  drawing; appendices A, B, C and E are the lists this brief refers
  to. Where a drawn signature cannot be built as drawn, build what the
  tree allows and name the change in your report under 5.
- `roadmap/briefs/2b-design.md` section 2.3 and `TESTING-NOTES.md`
  "The suite's shape": how a golden, its facts and the normaliser work.
- `decisions/daemon.md` (the entry form; C3 lazy spawn, C6 the
  ledger), `decisions/store.md` (C34, C35), `decisions/frontends.md`
  (C53, C81, C116).
- `crates/acquisition-daemon/src/mockggg.rs`: `MOCK_TABS` `:220-236`,
  `mock_items` `:250-258` and its callers `:398`, `:462`, the tab list
  `:435-456`, the characters `:378-433`.
- `search/DIGEST.md:22-37`, `:67` (S2, S3, S9, S10, S16, S17, S50: the
  real shapes); `docs/design/network-ground-truth.md:829-848` (N43, the
  frame words); `docs/design/trade-ground-truth.md:580-583` (T13).
- `crates/acquisition-search/tests/acceptance.rs:16-175`: the content
  the fixture's items are adapted from; `crates/acquisition-search/src/derive.rs:507-600`
  (what the search reads); `crates/acquisition-store/src/lib.rs:2054-2101`
  (what the store refuses).
- `crates/acquisition-verbs/src/refresh.rs:128-160`, `:464-506`
  (`StoreChanges::since`, `Submitted`); `crates/acquisition-store/src/lib.rs:659-`
  (`Status`), `:1885-1900` (`events_since`); `schema.sql:126-135`.
- `crates/acquisition-client/tests/harness/mod.rs` (the isolation
  list, `start_daemon` `:390`, `daemon_command` `:341`, `login` `:548`,
  `approve` `:533`); `crates/acquisition-cli/tests/harness/mod.rs`
  (`Transcript` `:109-231`, the rules `:213-345`, `golden` `:350-382`,
  `login_as_mock` `:95`); `crates/acquisition-mcp/tests/harness/mod.rs`
  (`Mcp`, `spawn_daemon` `:50`, `sibling` `:61`).
- `crates/acquisition-cli/tests/apply_loop.rs:43-245` (the arc the
  journey walks; the failed apply at `:177-245`),
  `realm_wire.rs:54-69` and `characters_wire.rs:64-78` (the journal
  reader to share), `readme_tour.rs:53-136` (`tour_lines`, `parse`),
  `crates/acquisition-mcp/tests/plan_loop.rs:49-61` (`finish_job`).
- `CLI-REFERENCE.md` and `MCP-REFERENCE.md` for every verb and tool the
  journey runs: `auth`, `policy set`, `refresh`, `tabs`, `search`
  (`--sort`, `--sum`), `show`, `price set`, `price clear`,
  `shop render`, `daemon status`, `daemon stop`; `set_sync_policy`,
  `refresh_plan`, `apply_plan`, `job_status`, `job_result`, `tabs`,
  `search`, `price_set`, `show`, `shop_render`, `price_clear`,
  `daemon_status`.
- The tests that assert the texts this step changes, found by grep
  before the edit: `Fake Item`, `-item0`, `Stash1`, `frame rare`
  across `crates/*/tests` (the survey found none pinning item content;
  confirm); `apply job`, `quoted job` in `crates/*/tests` and the
  references; the nine sites of appendix E.

## Input

- The owner's word on the design and on R1, quoted in `2c-design.md`
  section 7; his words go verbatim into commit 1's message.
- The measurements of `2c-design.md` appendix D: the `changed:` line
  over-counts in 9 of 20 runs; job ids identical in 20 of 20; the `job`
  rule matches nothing in the 25 goldens.
- The failing inputs the step's tests stage: two ingests recorded at
  the same `at` for the watermark; a fixture item missing its `id`,
  one with a lowercase frame word, one whose mod line is a string, a
  substash whose count disagrees with `MOCK_TABS`, each refused by the
  daemon's fixture test; for the normaliser, the texts
  `applying 9 requests as job 7`, `job 7 done: 9 requests, 9 done (jobs 8–16)`,
  `$ acq result 7`, `in jobs 7, 12`, `daemon stopped (pid 58345)`, two
  progress lines for one job, and the negatives `26 jobs: 0 running`,
  `1 job: 0 running`, `child jobs failed`, `Alice#1234`, `u001`,
  `dump-item3`; for the layout, nothing new.

## Do

1. **The ruling and the notes, documents only.** Record R1 in
   `decisions/daemon.md` under "Daemon, jobs, protocol" as the next
   free id (the design writes `C<next>`; `tools/docs-check.sh` tells
   you the number by refusing an unknown one), the text verbatim from
   `2c-design.md` section 4, its *Pinned* naming the paths items 3 and
   5 will create; one sentence after the bullet, in the file's
   register, that the owner's grant on his own stores is the fixture's
   upgrade path. Correct `TESTING-NOTES.md:87`: the first send per
   `(pid, route)` is a HEAD except `oauth-token` and the routes the
   daemon never probes, `profile` and `league` (`daemon.rs:1601-1606`).
   The owner's words in the message.
2. **The watermark.** `Status` gains `last_event: i64`, the greatest
   `item_events.id` (0 for none); `Store::events_after(id, limit)`
   returns the events with `e.id > ?1`, oldest first, the shape
   `events_since` returns; `events_since` is unchanged (the MCP's
   `item_events` reads it). `StoreChanges::since(store, before)` counts
   `events_after(before.last_event, …)`; `Submitted.started` goes, and
   every reader of it. Confirm by reading `lib.rs` that nothing deletes
   from `item_events` and say so in the message (the rowid is then
   monotonic). Pin it once, in `crates/acquisition-verbs/tests/documents.rs`
   beside the `Landed` cases: two ingests recorded at one `at`, the
   second apply's `StoreChanges` counting the second ingest's events
   alone; the test named for the fault. The `changed:` line's text does
   not change.
3. **The mock's items.** `crates/acquisition-daemon/src/mock-items.json`,
   compiled in with `include_str!` and parsed once (a `LazyLock`; a
   parse failure is a broken invariant, C47 allows the panic there with
   a message in words). Its shape: an object from key to an array of
   item bodies, keys the tab ids `dump`, `cur1`, `m001`, `old1` and
   `StashHoarder-inv`; the bodies of appendix A, every field it lists,
   adapted from `acceptance.rs` and shaped to S3/S50 (object mod
   lines), N43 (the frame words), S10 (`Map (Tier 16)` as the base),
   S16 (sockets). `mock_items(tab, n)` returns the fixture's array when
   the key is held, else today's Fake Items; a held key's count must
   equal `n`, held by the test below, so `MOCK_TABS` stays the
   listing's truth and no test's count moves. The helm (`:394-396`)
   becomes `frameType` 3, `frameTypeId` `Unique`; the character
   inventory items (`:398`) carry `inventoryId` `MainInventory`; the
   guardian's `normal` becomes `Normal`; the tab `old1` is listed with
   `metadata.public: true`. A unit test in the daemon crate, named for
   the property: every fixture item (socketed ones included) has an
   `id` unique across the fixture, a frame word from N43's set, and mod
   lines that are objects; each held key's count equals its `MOCK_TABS`
   entry; staged failures (the Input's four) each refused. Grep the
   tests for `Fake Item`, `-item0`, `Stash1`, `frame rare` first and
   name in the message what you found. Drive one mock world by hand
   (your scratch) and confirm `acq search 'rarity=rare'` answers and
   `show` prints `unread  nothing` for the ring before you commit.
4. **The harness.** (a) The journal reader into
   `acquisition-client/tests/harness/mod.rs`: `sends(base) -> Vec<Send>`
   with method, route, status, `counted`, `wait_ms` and the window
   states as fields, plus `journal_header(base)`; `realm_wire.rs` and
   `characters_wire.rs` use it and lose their copies. (b) A way to
   start an owned daemon with the journal on: `start_daemon_with(acq,
   acqd, base, env)` or the env on `daemon_command`, as the tree
   allows; the isolation list is unchanged. (c) `finish_job` from
   `plan_loop.rs:49-61` into the MCP adapter; `plan_loop.rs` calls it.
   (d) `acquisition-client/tests/harness/journey.rs`, std-only: the
   policy JSON, the three queries, the price, the ring's id, and the
   numbers of appendix B as named constants with a doc comment each;
   included by `#[path]` from both frontends' adapters. (e) The three
   rules in the CLI adapter, in the order of appendix C, each one
   function in the style of `age_rule`: `job` (`<jN>` by first
   appearance across the transcript, command lines included; en-dash
   ranges and comma lists; the prefixes `job `, `jobs `, `acq result `,
   `acq status `, `acq cancel `, `acq priority `), `pid`, `progress`
   (of stderr lines beginning `job <id>`, keep each job's last). (f) A
   module of `#[test]` functions in the CLI adapter (it compiles into
   `tests/main`): one case per rule from the Input's texts, the
   negatives, and that normalising each committed golden under
   `tests/goldens/` leaves it unchanged. (g) Rename `apply job {i}:`
   (`daemon.rs:4323-4339`) and `quoted job {i}` (`acquisition-plan/src/lib.rs:1212`)
   to `action {i}`; grep the tests and the references for the old words
   first; a reference that changes is regenerated and named.
5. **The journey.** (a) The CLI: `crates/acquisition-cli/tests/journey.rs`,
   a module of `tests/main.rs`, one test: an owned daemon with the
   journal on, the mock login outside the transcript, then appendix B's
   commands 1 to 16 and 18 in a `Transcript` with their facts from the
   table, `golden(file!(), "a_first_session", …)`; between 16 and 18,
   `daemon status --json` asserted by field (running, mock, compatible,
   the owned pid); then the journal checks of `2c-design.md` 2.2 by
   field, through the shared reader; then the tour check: every verb
   path the transcript ran appears in a `README.md` tour line
   (`readme_tour::parse`, made `pub(crate)`). Run the test ten times
   (`cargo nextest run -p acquisition-cli -E 'test(journey)'` in a
   loop) before committing the golden; a differing run is a finding in
   your report, never a new rule. (b) The MCP: `crates/acquisition-mcp/tests/journey.rs`,
   a module of its `tests/main.rs`: the sibling `acq` login, the tools
   of appendix B in order, each field compared to the table, the
   journal checks, the stop; its doc comment names `price_clear`'s
   `if_revision` difference. (c) `TESTING-NOTES.md` gains "The
   journey" after "The suite's shape": what the two doors hold, the
   journal checks and the two invariants out of the journey's reach
   (the user-agent, a retry through a block), each with the reason.
6. **The conversions.** Appendix E's nine golden sites become
   transcripts with facts (one golden per site, under
   `tests/goldens/<stem>/`; the failed apply and its `result` one
   transcript); the nine literal sites each gain a comment of one
   sentence saying why they stay (appendix E's reasons). No assertion
   is loosened; a fact names the value the literal asserted.
7. **The records.** The ruling's *Pinned* confirmed against the paths
   that exist; `REVIEW-CHECKLIST.md`, "Store and plans", a row: a count
   bounded by a whole-second clock (two applies in one second counted
   each other's events), held by the verbs test of item 2; `HISTORY.md`
   one line for the step if the file takes one per slice.

Commits: one per item, in order, each message naming this brief and
telling the story; the full gate bare before each (`cargo build
--workspace` first, the nextest line, then the rest), each stage's own
exit status on its own line, the commit inside an `if` on the ten
statuses; items 1 and 7 are documents only (`tools/docs-check.sh`,
`python3 -I tools/bothy.py check`, `git diff --check`). Then the review
as `roadmap/README.md` "Roles and rhythm" says, over `<commit 2>^..<commit 7>`,
saved verbatim at `roadmap/reviews/2c-<commit 7 sha>.md`, committed
alone, no finding fixed.

## Done when

- `decisions/daemon.md` holds the ruling with the owner's id, within
  800 bytes, one *Why:*, one *Pinned:* naming files that exist;
  `TESTING-NOTES.md`'s probe row names `profile` and `league`.
- `grep -rn "started" crates/acquisition-verbs/src/refresh.rs` finds no
  `Submitted.started`; `Status` has `last_event`; `events_after` exists
  and `events_since` is unchanged; the two-ingests test exists in
  `crates/acquisition-verbs/tests/documents.rs` and fails when
  `events_after` is replaced by `events_since(before.at)` (checked once
  by hand, the edit reverted, said in the message).
- `crates/acquisition-daemon/src/mock-items.json` holds five keys and
  the items of appendix A; the daemon's fixture test exists with its
  four staged refusals; in a mock world `acq search 'rarity=rare'`
  returns the ring and `show` prints `unread  nothing` for it; no test
  outside the daemon changed for a count.
- `grep -rn "fn sends" crates/*/tests` finds one, in the client
  harness; `finish_job` is in the MCP adapter; `journey.rs` beside the
  harness is included by both adapters; the CLI adapter has nine rules
  and a test module whose idempotence case covers every file under
  `tests/goldens/`; `grep -rn "apply job\|quoted job" crates/` finds
  nothing.
- `tests/goldens/journey/a_first_session.txt` exists with a `# holds`
  on every command; the CLI journey test runs ten times green in a
  loop; the MCP twin exists in its `tests/main.rs`; both check the
  journal through the shared reader; the tour check exists.
- The nine conversions each have a golden; the nine literals each a
  comment; `tools/docs-check.sh`'s orphan check passes.
- The gate green at every commit on the step's own tree; the suite
  still 24 binaries; the measured stage time stated in the report.

## Not in scope

- The owner's store as a fixture source; a path knob for the fixture;
  a receive log in the mock; a user-agent test (raised to the owner in
  the design; his call, outside this brief unless he adds it).
- Recording `daemon status`, the `jobs` table or `jobs --watch` in a
  golden; a `cli:<pid>` rule.
- Subsuming `apply_loop.rs` or `plan_loop.rs`; moving either binary.
- Any change to the search's tables, the limiter, the mock's rate
  policies, the protocol, or `AGENTS.md`.
- Converting a site appendix E lists as a literal; loosening an
  assertion; a golden without facts; a retry anywhere.
- 2d's file splits.

## Rules

- Every `acq` runs in a mock world: `ACQ_PROVIDER=mock` and
  `ACQ_STORE_DIR` under your scratch. Never the real provider.
- The main tree and your own scratch subdirectory, named in the
  launch; kill only the processes you started (a daemon you did not
  start is another session's; `lsof` on its socket says whose).
- The gate (`AGENTS.md`, "Quality gate") before the commit; the commit
  gated on each stage's own exit status, never on a filtered grep. Never
  run the drivers while the gate runs.
- One commit per concern; the story in the message, which names this
  brief's file; a size stated in a message is measured after the last
  edit. A breaker case rides in the commit of the check it stages.
- A finding is a test named for the fault, or a line in the report;
  never a new document.
- A draft the brief names is taken verbatim unless the tree disproves
  a sentence of it; each correction is named in the report.
- After the last commit, launch the review (`roadmap/README.md`,
  "Roles and rhythm"), save its findings verbatim at
  `roadmap/reviews/2c-<sha>.md` and quote them in your report; do not
  fix them before the owner has read them.

## Report

Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What the gate said, as exit statuses per stage; the ten journey
   runs' result.
3. Measurements: the test stage's wall time before and after; the CLI
   and MCP journey tests' own times; the golden's size; the fixture's
   size in the binary.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report;
   every shape built otherwise than drawn; every test outside the
   lists above that changed.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is wrong,
   and what would have made the job easier; and two judgements: does
   the journey golden read as a user's session, and did the table of
   facts keep the two doors one journey?
