### C27 — A client that disappears leaves its
- class: design — a user or an agent is promised that the job they submitted runs, and can be read back by its id, whatever happens to the client that submitted it
- clauses:
  1. P · Y · crates/acquisition-cli/tests/seat_faults.rs:578-599, `a_detached_job_is_collected_by_waiting_and_a_cancelled_one_is_no_success`: the submitting `acq` exits, and the 1.5 s sleep still runs to `outcome: success` for a later `result --wait`
  2. P · P · crates/acquisition-cli/tests/provider_default.rs:182-220 sends SIGINT to the starting command's process group and asserts the daemon is still running with the same pid. This is unix only (`#[cfg(unix)]`). A closed terminal (SIGHUP) is never staged, and Windows Ctrl-C has no test. A crash is clause 1.
  3. M · Y · the property this clause serves is held on unix by the same Ctrl-C test. The mechanism is stated at crates/acquisition-client/src/client.rs:2208 (unix `process_group(0)`); the Windows flags `CREATE_NEW_PROCESS_GROUP` and the rest sit in code just above, at client.rs:2197
  4. P · Y · crates/acquisition-client/tests/contract.rs:1336-1383: after a stop and a restart, `Request::Result { id }` still answers the old job, and ids continue. The CLI form (`acq result <id>`) is exercised at seat_faults.rs:597
- unheld: 2 simple · para · crates/acquisition-cli/tests/provider_default.rs, a sibling of `a_lazily_started_daemon_survives_a_ctrl_c_at_the_prompt_that_started_it` that sends HUP to the group (the Windows half would be complex: it needs a console-control-event harness)
- reason: Prop · crates/acquisition-cli/tests/provider_default.rs:176 states that Ctrl-C cancels nothing, but not the Why (the sends are committed either way) · moves to crates/acquisition-client/tests/contract.rs:1332
- confidence: medium · I did not read the daemon's handling of a client connection that drops mid-wait (not detached), and did not check whether the daemon calls setsid

### C91 — A query is one typed tree
- class: design — a human or an agent is promised one composable query language, whose ambiguities are refused and whose understood form is printed back
- clauses:
  1. P · Y · crates/acquisition-search/tests/language.rs:672-681, proptest `c104_a_generated_tree_survives_its_text_and_its_json`: every generated tree round-trips through text, the JSON form and serde. Immutability and portability are the `Node` type's (src/tree.rs:1)
  2. P · Y · the `Node` type nests `All`, `Any`, `Not` and `Holds{min,max}` over any node, `Members` of lines and links included. The generator at language.rs:610-660 builds every one of those across fields, lines and links, and the round trip holds them. "Every surface encodes the one tree" restates C46: frontends parse through this crate.
  3. P · Y · crates/acquisition-search/tests/language.toml:61 (the `composition.refinement` case, canonical `(old) new`), run by language.rs:185 `c104_every_case_of_the_corpus_round_trips_or_refuses_by_kind`. The route built as a refinement is at src/answer.rs:3272
  4. P · Y · language.toml (`mixed_and_or` cases with readings), asserted by language.rs:171-181 (the error kind and the exact readings) and language.rs:226 (every reading parses). The canonical query printed back is asserted at crates/acquisition-search/tests/answer.rs:41-45 (`query.text` and `query.tree`)
- unheld: none
- reason: Prop · crates/acquisition-search/src/tree.rs:1 states the property, not the Why (the C++ app cannot compose, the site composes only among stats, an answer must say how it was understood) · moves to crates/acquisition-search/tests/language.rs:669
- confidence: medium · I did not read the evaluator tests (rows.rs) that hold composition as evaluated, as opposed to as expressible

### C125 — A table drawn from the game's
- class: check — the entry's subject is the drift line and the generators' pin refusal, a gate check that keeps shipped tables from going stale silently
- clauses:
  1. P · P · the "gate refuses a clone past the pin" half is held by tools/game-diff.py:421-423 (self-test: a clone off the pin fails, naming the recovery), run by tools/trade-check.sh:63 in the gate and in CI. The "names its export" half is loose. The header-pin branch at game-diff.py:316-317 (`p != pin[:8]`) is never staged in the self-test, and runs only where a clone exists. crates/acquisition-search/tests/class.rs:105 holds the class table's source only, against a literal typed in the test. "Never typed" is not checkable.
  2. M · P · game-diff.py:418-435 stages off-PINS, a header's game, and the lagging export; a header's pin is unstaged. Stated in the docstring at game-diff.py:30-34
  3. M · Y · game-diff.py:417-418: no clone passes, and the line says so
  4. M · P · the generators' refusal off the pin is staged by `tools/sources.py --self-test` (trade-check.sh:60). "Same commit" and "pin first" are a procedure no check holds, stated at game-diff.py:88 (`PROCEDURE`) and tools/trade-check.sh:38-40
- unheld: 1 simple · wrong · tools/game-diff.py `self_test`, a drift case whose header names another pin than the clone's
- unheld: 2 cut · the drift line's conditions are stated in game-diff.py's docstring (lines 30-34); no frontend or crate reads the flag, only trade-check.sh
- unheld: 4 cut · the procedure is the tool's printed `PROCEDURE` (game-diff.py:88); nothing outside the tool and the sessions that run it relies on the entry's sentence
- reason: Prop · tools/game-diff.py:3 cites C125 for the property ("the tables name their export"), not for the Why (the tables went stale silently at each patch, and a clone pulled under the pin left the gate green) · moves to tools/game-diff.py:30 (the drift line's paragraph)
- confidence: medium · I did not read tools/trade-check-breakers.sh beyond its drift lines, nor the other shipped tables' header tests

### C137 — A deadline's contract is its behaviour
- class: check — how deadline tests are written, plus the harness's isolation list
- clauses:
  1. P · U · a rule on how a test is written ("pins it by forcing expiry"). The existing `c137_` tests follow it; nothing refuses a test that does not
  2. P · U · a rule on future tests. tools/arch-check.sh:982-1009 refuses only wall-clock reads in process-test code, never an `elapsed()` bound
  3. M · Y · crates/acquisition-client/tests/harness/mod.rs:183-188 sets `ACQ_IDLE_SHUTDOWN`, `ACQ_HELLO_WITHIN`, `ACQ_STARTUP_WITHIN`, `ACQ_ANNOTATION_WAIT` and `ACQ_STORE_WAIT` on the isolation list. Stated at harness/mod.rs:156-161
  4. M · Y · crates/acquisition-store/unit/tests/annotations.rs:729 (wait at zero refuses a held writer at once, the wait read back from SQLite); crates/acquisition-store/unit/tests/lib.rs:2546, 2582 and 2617; crates/acquisition-store/unit/tests/jobs.rs:323 and 355; crates/acquisition-client/tests/contract.rs:342 and 957; crates/acquisition-daemon/src/daemon/tests/exit.rs:1188 (I read only the annotation test's body, and the others' names and docs)
  5. M · R · C157
- unheld: none
- reason: Prop · crates/acquisition-client/tests/harness/mod.rs:157-161 states the property and "so a starved runner cannot fail a healthy path". Not stated anywhere: repricing a budget only promotes the next margin · moves to crates/acquisition-client/tests/harness/mod.rs:157
- fate: SH-KU
- confidence: medium · I read one `c137_` test body in full and saw the others only by name and doc

### C142 — A trade envelope may name the
- class: design — an agent or a page is promised one envelope key choosing which defence fields the site's four defence filters land on
- clauses:
  1. P · P · crates/acquisition-search/tests/trade_translate.rs:1024-1071: under `displayed`, the site's Armour filter lands `armour>=330`, exact, and matches differently. Evasion, Energy Shield and Ward are not staged; they go through `DEFENCE_ROWS` (src/trade/translate.rs:1170) unpinned
  2. P · Y · crates/acquisition-search/tests/trade_decode.rs:446-466: `q20`, null and absent all decode to `Defences::Q20`, and other values are refused by name. trade_translate.rs:1044 shows null landing `armour.q20>=330`, ambiguous
  3. P · P · trade_translate.rs:1049 asserts the translated text, and :1056-1062 asserts a note naming `armour` under `displayed`. The clause's label is not asserted, and the q20 note is not checked
  4. M · R · C141 (read beside `league`); trade_decode.rs:456 asserts nothing else lands beside the query (`beside` is empty)
- unheld: 1 simple · wrong · trade_translate.rs `c142_the_defences_reading_lands_the_field_it_names`, its loop widened to the `ev`, `es` and `ward` filters
- unheld: 3 simple · wrong · the same test, asserting the clause's label and its note under both readings
- reason: Prop · crates/acquisition-search/src/trade.rs:48 quotes the entry verbatim, without its Why · moves to crates/acquisition-search/tests/trade_translate.rs:1017
- fate: NT · simple
- confidence: medium · I did not read the web page's C142 citations (crates/acquisition-cli/src/web/form.rs and page.rs), where "label" may be a page label held by a web golden

### C145 — A present key of the wrong
- class: mech — how the store's ingest reads a body. The promise beyond it is to the daemon, which recognises a refusal as such (clause 5)
- clauses:
  1. P · P · crates/acquisition-store/unit/tests/lib.rs:434-525: five cases (`items`, `children`, a nested `socketedItems`, `leagues`, a stash id) are each refused as `MalformedBody`, kept in `refused`, with no item, substash or league retired. The character's own arrays (`equipment`, `inventory`, `jewels`, … at src/lib.rs:464-480) of the wrong type are not staged; they share `present_array` (src/lib.rs:483), so only a seam that bypasses it would escape
  2. P · Y · lib.rs:535 (a tab with no `children` or `items` holds none); lib.rs:548-569 (a character with every array absent retires its items, `removed == 1`)
  3. M · Y · lib.rs:470-491, the four keys in the cases table, with the kept row and nothing retired asserted at :505-524
  4. M · Y · lib.rs:576-596: a gem two levels down with no id is refused at its position, and nothing lands
  5. P · Y · lib.rs:603-626: with a trigger aborting the `refused` insert, the error is still `MalformedBody` with `not_kept`, in exact words
  6. P · Y · lib.rs:491-495: a stash body whose id is `b` under params naming `a` is refused as malformed
- unheld: 1 simple · wrong · crates/acquisition-store/unit/tests/lib.rs, the `cases` table of `c145_a_present_key_of_the_wrong_type_is_malformed_and_retires_nothing`: add a character whose `equipment` is an object
- reason: Y · crates/acquisition-store/src/lib.rs:483 states the property and its reason ("never an empty list that would retire every fact the key's location holds") · moves to crates/acquisition-store/unit/tests/lib.rs:427
- fate: NT · simple
- confidence: high · I did not read the daemon side that recognises `MalformedBody`

### C181 — A milestone's date is advisory, and
- class: rule — the owner's promise to himself about the timeline, which only people keep
- clauses:
  1. P · U · a rule on people; no test or check over the tree could fail on it
  2. P · U · a rule on people
- unheld: none
- reason: None · no file outside the registry cites C181 · moves to nowhere in code; it has no holder
- fate: KO
- confidence: medium · whether this is `rule` or `run` is a judgement call (see Noticed)

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C27 | design | 3 | 1 | 3/1/0/0/0 | Prop | NT | 1/0/0 | 0/1 | medium |
| C91 | design | 4 | 0 | 4/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C125 | check | 1 | 3 | 1/3/0/0/0 | Prop | NT | 1/0/2 | 1/0 | medium |
| C137 | check | 2 | 3 | 2/0/0/2/1 | Prop | SH-KU | 0/0/0 | 0/0 | medium |
| C142 | design | 3 | 1 | 1/2/0/0/1 | Prop | NT | 2/0/0 | 2/0 | medium |
| C145 | mech | 4 | 2 | 5/1/0/0/0 | Y | NT | 1/0/0 | 1/0 | high |
| C181 | rule | 2 | 0 | 0/0/0/2/0 | None | KO | 0/0/0 | 0/0 | medium |

## What I did not check
- C27: I read no test where a waiting (not detached) client is killed mid-job. I did not check Windows behaviour or the daemon's SIGHUP exposure, nor confirm that `tools/persist-check.sh` (which cites C27 at :224) runs in neither the gate nor CI. Its usage line says it runs live or `--mock`, by hand.
- C91: I did not read rows.rs or the evaluator's tests of composition as evaluated, only the type, the generator and the round trip.
- C125: I did not read the generators' own refusals, only `sources.py`'s place in trade-check.sh, and did not read the headers of the shipped tables other than the class table's test.
- C137: I read only annotations.rs:729's body in full; the other `c137_` tests by name and doc comment.
- C142: I did not read crates/acquisition-cli/src/web/ (form.rs:44, 611, 713, 960; page.rs:1017, 1114; fill.rs), where the "label" of clause 3 may live, nor its web goldens.

## Noticed
- C137, the fate: as I first read it (clause 4, "the knob at zero forces the expiry", as P and Y; clauses 1 and 2 as P and U), no rule of "The fate" decided the entry. Rule 2 needs every P clause U, rule 3 every P clause Y, rule 4 a P clause that is P or N. I reread clause 4 as M (a knob value only tests see), which gives SH-KU. A reader who keeps it P meets the gap: an entry whose P clauses are a mix of Y and U.
- C181, step 5: CLASS-BRIEF's `rule` lists who decides, reviews, commits and pushes, and live runs; `run` is "outside what `rule` names". A timeline policy in the owner's own words fits neither cleanly: `rule` gives KO, `run` gives KU.
- C125, step 2: the *Pinned:* names `acquisition-search/tests/class.rs`. It holds only the class table's source, against a pin literal typed in the test, not read from `PINS`, so it does not hold "read from the clone and never typed". The drift line's header-pin branch (game-diff.py:316) is the one fault path its self-test never stages, and in CI, with no clone, nothing checks any header's pin.
- C27, step 2: the *Pinned:* names `acquisition-client/tests/contract.rs` whole. The C27 test there (:1336) holds reattachment across a restart, not a client disappearing. That clause is held in the CLI crate (seat_faults.rs:578), which the entry does not name.
- C142, step 2: clause 1 names four filters; the only test of the translation stages Armour.
