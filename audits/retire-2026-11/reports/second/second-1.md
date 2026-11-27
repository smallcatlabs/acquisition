### C6 — The job queue persists: a `jobs`
- class: mech — the daemon's clients are promised that jobs, their results and their ids survive a restart, and that a queue the daemon cannot write stops new work. (One medium-confidence note: `queue.rs:2527` ties the sticky failure to "a send the queue cannot see must not happen", which leans toward ggg.)
- clauses:
  1. P · Y · `acquisition-daemon/src/daemon/tests/queue.rs:1879` `the_queue_and_results_survive_a_daemon_restart`: across three lifetimes, a finished job's result is still answered, a running job is re-queued, a waiting job keeps its priority and ids continue.
  2. M · Y · `queue.rs:2284`: a job that concluded after a failed queue write stays in memory, and memory answers for it. Also stated at `daemon.rs:796`.
  3. P · Y · `tests/evict.rs:98`: an evicted tree's status and result are answered from the table exactly as memory answered them. `queue.rs:2284` covers the "only when every row is persisted" half.
  4. M · Y · `tests/evict.rs:61-80`: a parent with one slow child keeps all three jobs in memory, and the whole tree leaves once the slow child concludes.
  5. P · Y · `queue.rs:2266`: a refused submit rolls back `next_id`. `queue.rs:1872` checks that ids continue across a restart.
  6. P · P · `queue.rs:2518` `a_broken_queue_is_fatal_to_restore` checks only that `restore_jobs` returns Err. Nothing exercises the actual refusal to start: neither the `JobDb::open` bail at `start.rs:481` nor the start's bail at `start.rs:565`. I found no process test with an unreadable `daemon.db` (searched for "repair or remove" and "without its queue").
  7. P · Y · `queue.rs:2205` `c6_a_probe_row_the_restore_cannot_delete_fails_the_queue`: after the restore's delete fails, `queue_failed()` is set, the failure is logged once, and the next submit is refused with `QueueFailed`. `queue.rs:2256` covers the runtime half.
  8. M · R · restates C83 (the world lock). Held at `acquisition-cli/tests/world.rs:146`.
- unheld: 6 simple · wrong · `acquisition-cli/tests/world.rs`, `refused_daemon` and `daemon_command`: start acqd on a world whose `daemon.db` is not SQLite, then assert a non-zero exit and that stderr names the file (otherwise ids can be reused against the table's history)
- reason: Prop · `crates/acquisition-daemon/src/daemon/queue.rs:8` (states the property at length; the Why — a restart lost the queue, and memory holds work, not lifetime — is absent) · moves to `crates/acquisition-daemon/src/daemon/queue.rs:8`
- fate: NT · simple
- confidence: medium · I read about 15 of the 71 citations. I did not read `persisted_queue.rs` or `store/unit/tests/jobs.rs`.

### C56 — Every item row records the array
- class: design — an agent at search is promised the `container` field (`acquisition-search/src/bind.rs:457`), and `store status` is promised a drift count for unknown item arrays that never fails a sync.
- clauses:
  1. P · Y · `acquisition-store/unit/tests/lib.rs:1685-1764`: the containers of equipment, guardian, skills, a stash's `items` and wickerman are recorded. Swapping two items between arrays is `(moved, changed, removed) = (0, 2, 0)`. Wickerman items are lifted, with nothing unlifted.
  2. P · Y · `lib.rs:1891-1953`: the `pets` array counts `unlifted = 2` and makes no row. The `record` call succeeds (never a failure), the counts appear in `status().unlifted_items` and in the envelope's `_unlifted`, and a later fetch without the array clears them. The `store status` line is held at `acquisition-cli/src/store_cmd.rs:1227-1265`.
- unheld: none
- reason: Prop · `crates/acquisition-store/src/lib.rs:194` (property and mechanism are there, but not the guardian slot-name reason; that reason is in the doc of the test at `unit/tests/lib.rs:1677`, which does not cite C56) · moves to `crates/acquisition-store/unit/tests/lib.rs:1677`
- fate: RM
- confidence: medium · The class is a close call between design and mech. I did not read the `store status` JSON test.

### C69 — A listing is two independent resolutions,
- class: design — the agent or user asking about a price is promised a listing's answer shape: both sides, their relation and the effective price.
- clauses:
  1. P · Y · `acquisition-plan/unit/tests/listing.rs:281-300`: manual and game sides each resolve, `relation` is Conflict and `why` names both sides. The effective price is held by the `c81_` test at `listing.rs:499`.
  2. M · R · restates C70 (tab coverage).
  3. P · Y · `listing.rs:290-314`: a note applies and then the tab name (`Source::Note`, then `Source::TabName`). `unit/tests/game_side.rs:26,125,156` cover exact, negotiable, a ratio in a note, and skip.
  4. P · Y · `listing.rs:322-341`: an invalid note and a non-price note have no effect, the text is shown verbatim, and the tab's price applies.
  5. P · Y · `listing.rs:300,316,359,365` cover conflict, agree, manual_only and none. game_only is covered only through its sentence at `listing.rs:648`.
  6. P · Y · `listing.rs:283-309,318`: the parser and currency versions, raw texts, revision and basis. The "causes" (`effective.from`) are checked at `listing.rs:508-560`.
- unheld: none
- reason: Cite · `crates/acquisition-plan/src/listing.rs:10` (the id only), `game_side.rs:9` (game side only), `listing.rs:311` (the relation). The Why (fused statements need locks) is stated nowhere. · moves to `crates/acquisition-plan/src/listing.rs:10`
- fate: RM
- confidence: medium · I did not read the `c81_` tests or `pricing_tools.rs:885`.

### C95 — The counts view has two shapes
- class: design — an agent at search is promised the counts views (`count`, `cross`), one sum, and refusals for any other aggregation.
- clauses:
  1. P · Y · `acquisition-search/tests/counts.rs:272`: three keys give three tables and no rows. `:682` covers the crossed table. `:449` covers the sum.
  2. P · Y · held by the refusals in clause 5: every other combination is refused with kind `view`.
  3. P · Y · `counts.rs:272-296`: independent facets. `counts.rs:820-824`: a cross with one key and a cross with `line` are refused ("two keys", "two fields").
  4. P · Y · `counts.rs:458-503`: lacking is `{value: 0, lacking: 1}`; unread gives `incomplete: true, unread: 1`; a sum of nothing is 0.
  5. P · Y · `counts.rs:827-829`: a raw line projection is refused, offering `sum(...)`. `acquisition-search/tests/explain.rs:826-836`: `Shape::of` refuses count with sort, count with cross, and a sum standing alone.
- unheld: none
- reason: Prop · `crates/acquisition-search/src/counts.rs:11` (every rule, plus "never an implicit choice of its largest occurrence"; "counts and totals are asked for" is absent) · moves to `crates/acquisition-search/src/counts.rs:11`
- fate: RM
- confidence: medium · Of the other citations I read only the refusal paths. I did not check whether the CLI's own count-with-sort row (`search_json.rs:940`) is a duplicate pin.

### C143 — A shared text speaks the request's
- class: cross — every frontend is promised shared words free of any one frontend's flags. MCP's harness and the web goldens each hold their own frontend; a new frontend (the GUI) could put its spelling inside a shared text and no existing test would fail.
- clauses:
  1. P · Y · `acquisition-mcp/tests/harness/mod.rs:390`: every text an MCP test meets, through `call`, `result` or `expect_rpc_error`, panics if it holds `--flag` or `acq …`, except a person's act. `acquisition-cli/tests/web.rs:3179` does the same for every answer, item and place page in the web goldens. `acquisition-search/tests/explain.rs:856` checks for no `--`.
  2. P · Y · `harness/mod.rs:473-518` (`spoken_otherwise`, `offered_otherwise`): an MCP offer says the same as the CLI's. Called from `pricing_tools.rs:307,427,443,529`.
  3. P · Y · `harness/mod.rs:479-483`: an MCP string may differ from the CLI's only by a trailing ` — here, ` spelling.
  4. P · Y · `explain.rs:812-857`: refusals of count with sort, count with cross, a lone sum, explain and fields name the request's fields, with no `--`.
  5. P · Y · `harness/mod.rs:371` admits only the `PERSONS` commands. `web.rs:5242` asserts "Signing in again is a person".
- unheld: none
- reason: Prop · `crates/acquisition-mcp/tests/harness/mod.rs:380` (property stated; the reason, that a sentence worded for one frontend reads as a defect on every other, is absent) · moves to `crates/acquisition-mcp/tests/harness/mod.rs:380`
- fate: KO
- confidence: medium · I did not read `spoken.rs` beyond its first test, `tool_refusals.rs:804`, or the web refusal census.

### C149 — Every answer and every refusal is
- class: cross — agents at every frontend are promised one JSON object with its counts. MCP's `told` door holds MCP alone; the CLI has no door, so a new CLI verb or a new frontend could answer a bare array unseen.
- clauses:
  1. P · P · Structural for MCP: `acquisition-mcp/src/told.rs:480` refuses any answer not starting with `{`, and `told.rs:485` is the only `CallToolResult::success`. The CLI and verbs are held only for listed answers: `acquisition-cli/tests/store_answers.rs:407-455` and `acquisition-verbs/tests/documents.rs:1660-1777`, each `is_object()`. Loose half: any CLI `--json` answer not on those lists, and refusals, which none of the three tests checks.
  2. P · Y · `told.rs:480`, the one door, exercised by `acquisition-mcp/tests/search_tools.rs:1429-1469` (each `doc.is_object()`).
  3. P · P · Count keys are asserted as numbers for listed answers only (`store_answers.rs:420-445`, `search_tools.rs:1438-1453`). Exact counts beside capped rows are checked only for characters (`acquisition-verbs/src/characters.rs:103-121`). That the text prints those same counts is C53's (restatement), and no test compares them.
  4. M · N · a history note ("taken at the alpha"). No code relies on it.
- unheld:
  1 complex · wrong · needs a product change (a CLI JSON door like MCP's `told`) or a census harness over every verb's `--json`
  3 simple · wrong · `acquisition-cli/tests/store_answers.rs`, the `c149_` table: assert that each listed answer's text footer prints the counts its JSON carries
  4 cut · — · history; git log holds it, and nothing reads it
- reason: Cite · `crates/acquisition-mcp/src/told.rs:477` (property only); `crates/acquisition-verbs/src/characters.rs:100` (the Why, for characters only) · moves to `crates/acquisition-verbs/tests/documents.rs:1652`
- fate: SH-KO
- confidence: medium · I did not read the refusal shapes or the web frontend's JSON.

### C180 — `roadmap/README.md` says what is happening
- class: run — every session is promised a state file that says now, next and what waits on the owner.
- clauses:
  1. P · P · Nothing checks that the three sections exist (they do today: `roadmap/README.md:11,37,78`). "Each close rewrites it" is a rule on people.
  2. P · P · Only the byte cap is a proxy: `tools/docs-check.sh:50` fails over 6000 bytes. A short history line passes.
  3. P · U · "Next" is in order, and a session takes the first item: a rule on people.
  4. P · Y · `tools/docs-check.sh:47-55` fails at 6000 bytes, in the gate's `docs` stage. `tools/docs-check-breakers.sh:106-108` pads the file and expects "over its cap", run in CI (`.github/workflows/gate-linux.yml:215`).
- unheld:
  1 simple · para · `tools/docs-check.sh`, check 1: require the `## Now`, `## Next` and `## Waiting on the owner` headings, plus a breaker row in `docs-check-breakers.sh`
  2 complex · para · what counts as "history" has no mechanical definition
- reason: Prop · `tools/docs-check.sh:7` (property; line 45 adds that growth is history, but not the "buried its one current sentence" reason) · moves to `tools/docs-check.sh:44`
- fate: NT · complex
- confidence: high · I did not read `roadmap/README.md` beyond its headings.

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| C6 | mech | 5 | 3 | 6/1/0/0/1 | Prop | NT | 1/0/0 | 1/0 | medium |
| C56 | design | 2 | 0 | 2/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C69 | design | 5 | 1 | 5/0/0/0/1 | Cite | RM | 0/0/0 | 0/0 | medium |
| C95 | design | 5 | 0 | 5/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C143 | cross | 5 | 0 | 5/0/0/0/0 | Prop | KO | 0/0/0 | 0/0 | medium |
| C149 | cross | 3 | 1 | 1/2/1/0/0 | Cite | SH-KO | 1/1/1 | 2/0 | medium |
| C180 | run | 4 | 0 | 1/2/0/1/0 | Prop | NT | 1/1/0 | 0/2 | high |

## What I did not check
- C6: most of its 71 citations, `persisted_queue.rs`, `acquisition-store/unit/tests/jobs.rs`, and whether every state change, not just submit, start, finish and priority, is written.
- C69: the `c81_` tests, beyond the `from` asserts, and the pricing MCP and CLI tests that cite it.
- C95: the CLI's `search_json.rs:940` refusal rows, which may pin count-with-sort a second time against C122.
- C143: the web refusal census (`web.rs:7863`) and `spoken.rs` beyond its first test. Clause 1's MCP half covers only the texts MCP tests happen to meet.
- C149: whether any frontend's refusals are checked to be one object, and the web's JSON answers.
- No test was run. Every "runs in the gate" rests on file location (nextest `--all-targets`, `docs-check.sh` in the gate, `docs-check-breakers.sh` in `gate-linux.yml`).

## Noticed
- C95, step 2: the entry says "there is no grand total", but `counts.rs:461` asserts a view-level `counts.sum` of `{name: "stack", value: 55, …}` over every match beside the tables, which reads as a grand total. The code's doc (`counts.rs:15`) says "no standalone grand total". The entry's wording is wider than the code's.
- C149, step 1: clause 4 ("taken at the alpha") is history inside a registry entry, and `docs-check.sh`'s "no history" check passes it.
- C149, step 2: its *Pinned:* names three listed-answer tests, but the holder that actually covers every MCP answer is the `told` door (`told.rs:480`), which the *Pinned:* does not name. No test stages that door's refusal.
- C56, step 4: the reason is in the doc of `unit/tests/lib.rs:1677`, which does not cite C56, so `show` reports one citation and the reason looks absent.
- C6, step 2: its *Pinned:* names `a_broken_queue_is_fatal_to_restore` for "refuses to start", but that test stops at `restore_jobs` returning Err. No test reaches the start's refusal (`start.rs:481`, `start.rs:565`).
- C6 and C149, step 5: `cross` is defined as "a new frontend or crate could break it". C149 is broken far more easily by a new verb in an existing frontend (the CLI has no door). The rule decides it, but only through the new-frontend reading.
- C180, the fate rule: it is a process entry whose clauses mix U (a rule on people) with P (held only by a proxy). Fate step 4 sends it to NT · complex because "no history" has no mechanical form, though the entry is otherwise a cap plus a people rule. The fate steps have no case for a session's entry whose held parts are Y and whose remaining P clauses are rules on people mixed with proxies.
