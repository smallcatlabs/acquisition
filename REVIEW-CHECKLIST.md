# Review checklist

The shapes of fault that came back until they were named, from every
closed slice's record and the roadmap's reviews, grouped by the area a
change touches. Read the area's rows before reviewing a change there;
a review names the shape it found. Each row says where the shape came
back and what holds it now. A shape that comes back again gets its
holding test strengthened, never a second row. The records these rows
come from are whole at the commit `HISTORY.md` names for each slice.

## Network: the limiter, the gate, OAuth

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A parsed value that overflows.** Strict parsing accepted header values that panicked in deadline arithmetic; a token response's `expires_in` was added to the wall clock unchecked, under `shared` | network cleanup N1; 2f | every field of a GGG header or body that enters arithmetic bounded where it is read (the daemon crate has no `unwrap_used` lint); checked parsing; the `acquisition-daemon/src/ratelimit/tests/spec.rs` tables; `an_out_of_range_expires_in_is_a_token_error` (the refresh and the login) |
| **A send recorded before its body completed.** Clean-2xx classification and send recording ran before body resolution | N1 | classification deferred to body completion; `full_200_with_truncated_body_is_network_after_observing_policy` (`acquisition-daemon/src/ratelimit/tests/spec.rs`) |
| **An owner that leaves without publishing.** An aborted refresh owner stranded its waiters forever | N2 | `RefreshOwnerGuard` publishes on drop |
| **An estimate that bypasses the policy.** `eta_for` skipped the token policy's 60 s bucket | N4 | the limiter's bucket applied to every estimate |
| **A stress that never reaches what it claims.** Concurrent refresh never happened; the N6 stress ran the mock on real time and the daemon on a manual clock | N6; TESTING-NOTES | a held response as the barrier; one `Clock` for mock and daemon (`MockPolicy`, `start_with_clock`); the C87 concurrent half checked at the server (`rejected_access_token_*`) |
| **A rail a hint undid.** A met ceiling was a flag `reset-tripwire` cleared, one send out per reset | platforms | `the_ceiling_counts_every_method_and_halts_at_its_number` (`rails.rs`); `a_ceiling_halt_is_advised_as_the_lifetimes_and_a_trip_as_a_reset` |
| **A rule that pins a property the product lacks.** The 401 rule ran over journals none of which carried a 401; the first that did showed the bearer re-sent | TESTING-NOTES | C87; a breaker on each side of every journal invariant |

## Store and plans

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **Authority read from the finer observation.** A fetch revived what a listing retired; a dropped tab left its items live; a late fetch rolled a character's fields back; removal keyed on a timestamp missed two fetches in one second | five review rounds of the refresh slice | C54; `Location` is the full coordinate; `items.seen_response`; the store snapshot tests, `tests/realm_wire.rs`, `tests/characters_wire.rs` |
| **A refusal that destroys its evidence.** PoE2 first contact refused four of five bodies and dropped them | the refresh slice | C30; the `refused` table |
| **A strict parse loose one level down.** serde ignored extra fields beside a unit variant's `kind`; an untagged enum lost top-level strictness; `tabs: []` compiled to an empty plan | refresh steps 3, (1), (3) | the envelope re-serializes to exactly what was read; per-stamp strict parse; `tests/plan_json.rs`, `tests/realm_wire.rs` |
| **A version where an identity belongs.** The handshake compared the package version, so a pre-realm daemon served a console job; the soak ran a binary predating its fix | the realm review; TESTING-NOTES | C10, C84; `tests/daemon_observe.rs`; `tools/preflight.sh`'s `provenance_matches_journal` |
| **A failure that names no cause.** The only failure text was an unsorted id list, the cause in the daemon log | the legibility step | C53's failure line (target, cause, evidence); the `c53_` tests |
| **A check that reads prose.** The tracer driver grepped tab ids out of `acq tabs`'s text; the third layout change aborted a passing run | the daemon split's step 7; platforms | a script reads the JSON by id; `tools/tracer-verify.py --self-test` |
| **A stop condition that cannot fail.** "One HEAD per route" with no restarts; a matching journal count taken as proof the rail was there | TESTING-NOTES; the tracer's rounds | the daemon reports armed, ceiling, count and halt; the verifier accepts 2xx only, probe-first per account route |
| **An index that cannot skip what the read skips.** The live read scanned removed rows under the one index, and a gone-id lookup was quadratic | phase 0, step 0a | the two live partial indexes (`items_live`, `items_live_at` in `schema.sql`); the removed-heavy copy as a standing M3 row |
| **A realm written one league at a time.** `policy set` replaces each realm it names whole, and a policy naming only the new league dropped Standard: the tracer driver wrote a one-league realm, the Allflame run (`RUN-LEDGER.md`) lost Standard and the owner restored it by hand, and the tour's comment read as "the leagues named, the rest kept" | the Allflame run; roadmap 2e's survey | C36's replace rule; the README's new-league bullet and the tour comment; `tools/tracer-rung.sh` writes the realm back whole; the rehearsal's policy step (`acquisition-cli/tests/new_league.rs`) |
| **A count bounded by a whole-second clock.** An apply's `changed:` line read the events at or after the second it was submitted, so two applies in one second counted each other's events (9 of 20 journeys) | roadmap 2c's design survey | the event watermark, `Status.last_event` and `Store::events_after`; `two_applies_in_one_second_do_not_count_each_others_events` in `acquisition-verbs/tests/documents.rs` |

## Pricing and intent

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A fall-through past an unreadable row.** An unreadable row never reached the effective price and let a game price through, by two doors; the C++ bug | plan 4b and its reviews; search step 9 | `c81_a_note_that_cannot_be_read_leaves_the_price_unresolved`; the override gated on `game.public` |
| **Membership read off a missing coordinate.** A league-less character read as membership | plan 4's review | `a_league_less_characters_items_are_flagged_in_every_leagues_report` |
| **A blind write over what this build cannot read.** | plan 5's review | `price_set_and_clear_refuse_a_row_this_build_cannot_read_unless_the_revision_is_named`; `c66_a_writer_s_rule_refuses_a_new_write_and_never_a_read` |
| **A rename under an additive schema.** A count rename replaced keys under schema 1 | plan 4b's second review | `the_report_json_matches_the_committed_fixture` |
| **A table that fails open; arithmetic unchecked.** | plan 6's review | `c47_corrupt_facts_and_unruled_words_are_cells_never_panics` |
| **A first-open race on the one irreplaceable file.** Two gate reads in two statements; two exports sharing a partial file's name within one tick; WAL on a fresh file taking the write lock inside its own read | plan 7; the daemon split's round 24 | `c35_a_losing_first_opener_waits_for_the_winners_stamp_and_proceeds`, `c35_a_create_attempted_between_the_gates_two_reads_cannot_commit_or_cause_a_refusal`, `simultaneous_exports_to_one_destination_publish_exactly_one`; `ensure_wal` |
| **A value narrower than the game writes.** An amount refusing the decimals the game's own notes carry; the C++ `emit` column as folklore | plan 2's review; the census | `c67_the_amount_is_a_four_place_decimal_or_an_unreduced_lot_pair`; `c68_the_loader_refuses_what_a_reviewer_would` |
| **A join that hides the condition it tests.** `site-join.py` read "Price with Note" before the tab's `public` | plan 7's review | `tools/site-join.py --self-test` |
| **A claim left standing after the code disproved it.** The ground truth still said what parser v2 had disproved | plan 7's review (the closeout blocker) | the commit that disproves a claim amends it; no test |

## Protocol, client, daemon lifecycle, the world

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A reader whose partial answer satisfies every rule.** A guard parsing manifest text; `jq` in process substitutions whose failure bash drops; `cargo tree`'s host closure; names for vertices; matching counts for correspondence | eleven rounds on one check | the graph read twice and compared row by row (`tools/arch-check.sh`, the edges; `tools/arch-check-breakers.sh` stages every case) |
| **A door that fails open on what it could not tell.** An absent tool, a timeout, an error read as absence | four rounds at the doors | absence is exactly `NotFound` and `ConnectionRefused`; every probe bounded; `acquisition-client/tests/contract.rs` |
| **A hint trusted over a re-read.** The watch kept a lagged subscription and trusted the event when its re-read failed | rounds 1 and 2 | `acquisition-cli/tests/watch_recovery.rs` |
| **An identity judged twice.** `report()` looked at the sibling again after the verdict; the sibling was `stat`ed then reopened by path | rounds 14 and 15 | one verdict per handshake; `c84_the_sibling_is_judged_from_the_handle_that_was_identified` |
| **Provenance that reads only the headers.** A headerless journal passed | round 13 | `provenance_matches_journal` requires a header before every send (`tools/preflight-breakers.sh`) |
| **A test that owns what it does not clean.** Raw children without `Drop`; a scratch removed on success only; a test reading the environment; constant assertions | rounds 15 to 24; 42 GB of facts files, 2026-10-26; r2's fix round, a child ended by `exit` | guards that kill and wait on drop; a child process that ends by `exit` writes only under its parent's directory (`TestPath::file_in`; `c148_a_panic_in_a_jobs_landing_marks_the_job`); `Scratch` (process tests); a `tempfile::TempDir` the test holds, or the daemon harness's `TestPath`, never a name built from the pid; `Store::open_memory()` where the file is not the subject; derivations tested with directories the test names |
| **Privilege made loose and tightened after.** A private directory created loose; a migration logged and not atomic; the refusal path opening the log with `create(true)` | rounds 19 to 21 | 0700 in the creating call (`private_dir`); fail-closed, typed, atomic; the `world.rs` tests |
| **An exit that unlinks the successor's socket.** | the daemon split | `BoundSocket` |
| **A stop that returned early.** `daemon stop` returned while the daemon still exited | platforms | `a_stopped_daemon_is_gone_when_the_stop_returns` |
| **A Unix mechanism assumed in product code.** The daemon inherited its caller's stdout pipe on Windows; a file's identity was its inode; paths kept `\\?\`; an fsync through a read handle | platforms | the suite on Windows in CI; `a_lazily_started_daemon_keeps_no_pipe_of_the_command_that_started_it` |
| **Bytes that differ by checkout.** CRLF in `Cargo.lock` and `*.sql` gave the Windows archive another contract revision | platforms | `.gitattributes`; `acquisition-protocol/tests/line_endings.rs` |
| **A process tree the host owns.** A kill-on-close job ended a lazily spawned daemon | platforms | the spawn asks to break away (`world_windows`); a host that forbids it is issue 1 |

## Search

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A meaning read off the syntax.** Which sources a group admits, its selector, its bound, a slot, the zero block: each answered at one level and wrong one level down, so parentheses changed the answer | step 4's four audits, five times; 9c4, four times | `group.rs`, the one reader of a group's meaning (the rule in `tools/arch-check.sh`); the equivalence property; `tests/generated_rows.rs` |
| **One thing counted under two names, two taken for one.** A number read by two alternatives added twice to a sum | 9c4 | `derive::Place`; `eval::Read`; `tests/rows.rs` |
| **Unknown said at the wrong grain.** A whole object unread for one flag that is no boolean; too wide leaves terms open, too narrow reads as absence | steps 4 to 9d-B | `derive::Slot`, `Line::flags_unknown`, `Unread::line`; the completion property |
| **Two statuses for one thing.** A number kept beside the unread that may replace it; a total exact on the item and rounded in the bucket | steps 5, 6, 9d-B | one status per value; `exact::Exact`, units carried to the print |
| **A second maker.** A route, a command, a reason, a resolved list made in two places that then disagreed | steps 4 to 11b; the trade import's step 7 | `answer::command`, `answer::Router`, `eval::why`; the CLI test that runs every printed command; `tools/trade_query.py`'s `asking()` |
| **A printed command the build refuses.** A route starting with `-`; a reading that does not parse; a continuation the key parser could not read back | steps 4 to 10b | the route property, every command an answer prints parses and is not refused: `acquisition-cli/tests/search_json.rs` (rule 5) and `acquisition-search/tests/answer.rs` |
| **Arithmetic on floats.** A sum whose value depended on the order of its occurrences | steps 4b, 5 | `exact.rs`: a number read once, whole units from then on |
| **A cut in the query's order.** Two spellings of one query showed different sixes | step 7 | `eval::why` orders a reason by what it says; `tests/pseudo.rs` asks two spellings |
| **A block past its bound.** Rows appended after the cut, the omission uncounted | step 7 | every list an answer holds is cut by the limit and counts its rest |
| **A number no game displays, read as one.** Scientific notation through a length check; a product past the units rounded in silence | step 7 | `exact::reads` reads decimal syntax alone; `Exact::times` is none where the units cannot hold it |
| **A join that inherits the producer's grain.** A note or slot the other area typed strictly failed that read whole, for queries that never asked the field | step 9's first review, four of five | `ItemSnapshot::note_unread`, `inventory_id_unread`, `body_string`; C81's own gate on the override; `price::number` |
| **A claim the code did not make.** A hand count wrong; "covered" where the generators could not reach the case; a message claiming what a failed step had not shown; a finding's evidence quantified past its rows; in the daemon and the network, a commit message, doc comment or registry citation wider than the code or the reds it showed ("spends no token", "GGG never saw the request", a size passed on unmeasured, a Pinned field naming no test) | every step; roadmap 2f's rounds A, C and D, each caught by the agent that checked the unit or by Codex | every count worked by hand and then run; a cost claimed is a cost asked for (`tests/rows.rs`); a step a tool's text gives is run over a store that refuses it; a claim found wide is corrected in a later commit's message (`12395d33`, `437a7929`, `e768a73a`, `bf9d44ef`, `6ef1c0a4`; all but the first name the corrected commit by its hash before the rebase) |
| **Silent reach.** Right by its rule and wrong for the reader: an example reaching 87 bases with five shown | the first seat | `tests/seat_faults.rs` (`f3_`); the help teaches the closed set |
| **A refusal right in kind, wrong in size.** 82 names inline; one reason printed once per term | the first seat | `bind::near`, `bind::LISTED_INLINE`; `tests/seat_faults.rs` |
| **A patch for a body composed to probe.** A finding on an input no real item has, mended as a case of its own, each case opening its neighbour | 9d-A | the probe measured on the owner's copy before a fix; one rule in place of the cases (`pseudo::mark_of`); `tests/percentile.rs` |

## Trade

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **Evidence read off whatever is on disk.** A script without its clone wrote 16 grouping members out; an extract over a partial restore wrote 1,149 searches for 1,150; a local capture joined to the committed extract took a replaced capture's marks | three outside reviews in a row | every input declared and held to its record before any is read (`tools/sources.py`); raw read by the extract alone (`tools/trade_query.py`); the sitting tools' self-tests in `tools/trade-check.sh` |
| **A reading applied past the shape it asked.** One shape's answer built for every clause that looks like it: `#% chance to X` elided everywhere where the game elides 212 of 372 | 7e and its reviews; C119's other readings | a reading applies where the clause has every property of the shape asked; the forms' identity rule (`one_stat` in `tools/trade-forms-table.py`); no expected disagreement under an exact clause (`tests/trade_oracle.rs`) |
| **A group's statuses computed where it is written, not where it is read.** | step 4's reviews | every group bound alone; every clause that runs owns one term path inside its group |
| **A scope read one level deep.** The league a query fixes read off the top of the tree; a fetch of no rows counted as no fetch | note 38's step 2 review; the re-sit's review | `fixed_league` reads through nested conjunctions; a fetch of no rows is a fetch |

## Frontends and the surface an agent meets

| Shape | Where it came back | Held by |
| --- | --- | --- |
| **A claim wider than its evidence.** A sentence written for the common case and read as a rule ("Standard unless you name a league" of every verb; a runner's "every time") | the release's reviews 3, 4 and 6; the debriefs | each bullet names its verbs; the debrief asks what the runner would retract |
| **A fix staging the nearest failure, not the named one.** A test green on a daemon the recipe never started; a deadline that waited for a byte and then for a line; a restored row tested with a fresh token, so the refresh it would send never came due; a failed write whose reset had nothing to write, so one platform answered Ack | reviews 4, 6 and 7; roadmap 2f's round A review and round C's Windows gate | `the_agent_recipe_starts_a_daemon_that_idles_out_after_the_hour` reads running=false first; `.claude/skills/seat/doors_test.py` stages a stall, half a line, non-JSON, an exit and a linger; `c127_a_row_that_renders_no_url_fails_before_any_refresh` runs with a refresh due; `a_rails_write_that_fails_is_logged_and_its_reset_refused` keeps a dead grant, so the reset must write |
| **A sibling sentence left standing.** The fix made where the fault was met and not where the same words stood | review 2, the rerun, review 8 | the test runs the command as the refusal types it; every surface's refusal is one function's, `acquisition_verbs::refusal::document` (the MCP's `refusal()`, `refused()` and `priced`, the CLI's `error_report` and search failure, the web's failure page), pinned by the verbs crate's `every_refusal_is_one_shape_from_one_function`; every receipt's undo is one function's, `acquisition_verbs::receipt`, and `price_undo` runs the writer's rule through the planner's |
| **A refusal that says what is wrong and not what to do.** No daemon, said without the line that starts one; a remedy naming no provider or store | the triage, the rerun | `a_job_tool_with_no_daemon_names_the_command_that_starts_one`, `the_no_daemon_remedy_names_the_provider_and_store_the_server_runs_with` |
| **A document unbounded by default.** The shop render at 5 MB; `price list` whole at 67 KB | the triage, the rerun | `the_default_shop_document_lists_a_bound_of_what_is_left_out_and_counts_it_all`, `price_list_is_bounded_by_a_limit_and_says_how_many_of_how_many`; issue 9 for the rest |
| **A printed command that does not run as printed.** An undo without its account; a target unquoted; a configuration block that, copied, named the stand-in | the triage, review 3, the rerun | `every_undo_a_price_receipt_prints_runs_as_printed_with_a_second_account_known`, `an_undo_quotes_a_target_a_shell_would_split`; `readme_tour.rs` runs every line of every `sh` block |
| **A request the transport drops silently.** `rmcp`'s stdio reader dropped any line serde_json refused, a deep call among them | phase 0, step 0c and its review | `acq-mcp` reads stdin itself; `transport.rs`'s tests (`a_deep_line_cut_short_is_a_parse_error_not_a_refusal`, `a_call_nested_past_the_limit_is_a_tool_refusal_under_its_own_id`) |
| **A wait with no deadline.** A test waited on a child with no bound, so a transport that never exits would hang the gate | phase 0's fix-round review | the MCP harness's `run_to_exit` |
| **A brief trusting the audit over the tree.** A ruling cited as built that was parked; a test named that did not exist; a count wider than true | phase 0's three raised items | the brief's writer checks every citation and named test against the tree (`roadmap/briefs/TEMPLATE.md`) |
| **A document's producer moved without its readers listed.** The failed ids left the daemon's sentence while the rows were gathered on one CLI path, so the ledger and post-restart paths printed counts with no handle; the next fix read every failed child read as missing retention | 2a's three fix rounds | `failed_children` on the layer's result documents, filled by one sorter for both readers; every CLI text failure through `job_report`; the ledger cases in the verbs crate's `tests/daemon.rs` and the CLI's `apply_loop.rs` |
| **A behaviour pinned at three layers.** One wording asserted in three crates' tests, so a rewording was three edits; every MCP search test re-ran `acq --json search` for an equality that could not fail; `Tabs::of` held by frontend tests alone; the harness copied, its isolation lists disagreeing | step 2b's design survey | C122: the lowest crate holds the behaviour and a frontend's tests its adapter's part; the wording in `crates/acquisition-cli/tests/goldens/` with its facts; one harness, `crates/acquisition-client/tests/harness/mod.rs`; the layout rule in `tools/arch-check.sh` |
| **A fixture that cannot see the default.** `item_events`' 24-hour window under a 1970 fixture, which any default satisfied; then under events 1 and 48 hours old, which a 12-hour default satisfied | step 2b's build review, then its fix round's review (the fix brief chose the hours) | `item_events_reads_back_its_hours_and_defaults_to_a_day` seeds 23 and 25 hours: the edge seen from both sides, and the test fails under a 12-hour and a 48-hour default (checked once each) |
