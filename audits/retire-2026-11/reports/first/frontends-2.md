### C121 — An account that cannot be picked
- class: design — a user or agent gets one protocol word per account mistake (`unknown_account`, `not_logged_in`, `ambiguous_account`) with `accounts`, the same from the store path and the daemon. Every frontend inherits this from the verbs crate's `Frontend::account`, and the arch-check door grep (arch-check.sh:555) refuses `Index::load` in the cli and mcp sources, so it is not `cross`.
- clauses:
  1. P · Y · crates/acquisition-verbs/tests/documents.rs:117: each fault's wire kind and `accounts`. :193: no index is `not_logged_in` with `[]`. :205: none named among several is `ambiguous_account` with both. :238: an unknown selector is `unknown_account` with the names known. :250: a bare name shared by two is ambiguous. The frontends pass it through: cli tests/seat_faults.rs:260 (`ambiguous_account` and `accounts` under `--json`) and mcp tests/tool_refusals.rs:129-145 (`unknown_account` with `accounts` on 8 tools).
  2. P · Y · crates/acquisition-daemon/src/daemon/tests/sessions.rs:1104: submit, auth check and quote all judge a never-held name `unknown_account` and a held name with no session `not_logged_in`; logout of a typo is `unknown_account`; with no index the answer is `not_logged_in`. mcp tool_refusals.rs:240 checks the same through `submit_job`.
- unheld: none
- reason: Prop · crates/acquisition-daemon/src/daemon/tests/sessions.rs:1096 (states the daemon half and "a typo, not a login to make"). No comment says a program branching on `not_logged_in` would try a login. · moves to crates/acquisition-verbs/tests/documents.rs:190
- fate: RM
- confidence: high · I did not read cli seat_faults.rs past :310 or daemon/tests/credentials.rs:906.

### C143 — A shared text speaks the request's
- class: design — the words of a refusal, offer, route, continuation, `describe` entry or note are the request's own, so they read correctly on any frontend.
- clauses:
  1. P · Y · crates/acquisition-mcp/tests/harness/mod.rs:390 `c143_a_text_speaks_no_frontend`: every text an MCP test meets through `call`/`result`/`expect_rpc_error` (:312, :322, :340, :357), daemon messages included, panics on an `acq …` command or a `--flag`. The search crate also checks at crates/acquisition-search/tests/explain.rs:856 (no `--` in the view refusals). It covers only the texts some MCP test meets.
  2. P · Y · harness/mod.rs:473 `spoken_otherwise`, asserted by `assert_spoken_as` (pricing_tools.rs:307, :427, :529, :765): offers are the same offers with the same `says`, each request spelled by its surface. tool_refusals.rs:809 confirms an offer's `request` is treated as payload.
  3. P · Y · harness/mod.rs:479-483: a server string must be the CLI's whole, or the CLI's followed by ` — here, ` and the adapter's spelling. Nothing else passes.
  4. P · Y · crates/acquisition-search/tests/explain.rs:812: the view refusals name `sort`, `count`, `sum`, `explain` and `fields`, with no flag. tool_refusals.rs:849: every generated string (`note`, `no_undo`, `says`) is read. `describe` passes through the same harness `call`.
  5. P · P · harness/mod.rs:371 `PERSONS` lets `acq auth`, `acq daemon stop` and the other person's acts stand, and tool_refusals.rs:836 accepts one. Nothing asserts the "marked as a person's" half: the words "a person starts one" (verbs/src/frontend.rs:377) appear only as a test input (documents.rs:118, :2323).
- unheld: 5 simple · para · in the MCP harness `frontend_word` (harness/mod.rs:435), accept a `PERSONS` command only where its prose says it is a person's, or assert on `Fault::NoDaemon`'s made `next` in verbs/tests/documents.rs
- reason: Prop · crates/acquisition-mcp/tests/harness/mod.rs:380 (and mcp/src/main.rs:87) state the property. No comment says a sentence worded for one frontend reads as a defect on every other. · moves to crates/acquisition-mcp/tests/harness/mod.rs:380
- fate: NT · simple
- confidence: high · I did not read cli tests/web.rs:3168 (the page side), cli/src/printed.rs:208 or mcp/tests/spoken.rs.

### C146 — A count says what it counts
- class: design — every printed count names what it counts, where it came from and what to do next, in the same words on every surface.
- clauses:
  1. P · P · counts are pinned one by one: cli/src/store_cmd.rs:1215 (each `store status` line), search_cmd.rs:3302, daemon_cmd.rs:296, price_cmd.rs:1672. A new bare count fails nothing.
  2. P · P · the same per-count tests (store_cmd.rs:1263-1271 asserts whole sentences with origin and next step). There is no sweep over the numbers a frontend prints.
  3. P · P · mcp/tests/mcp_reference.rs:134 refuses "intent" outside backticks across the instructions, tools and all of `describe`. `residue`, `unlifted`, `withheld` and an unnamed rank are held only where store_cmd.rs:1264-1265 asserts their sentences, and on the CLI alone.
  4. P · P · mcp/tests/surface_size.rs:94 samples one gloss per count for listed tools in the `describe` section each description names. plan unit/tests/pricing.rs:989 holds two glosses' extents. Nothing compares a count's text with its field doc across surfaces.
  5. M · R · C53 (the JSON field name is the contract). store_cmd.rs:1292 checks that the fields exist.
- unheld: 1 complex · wrong · whether a sentence "says what it counts" is not machine-checkable; the clause's meaning would need a form
- unheld: 2 complex · wrong · same: origin and next step per printed number need a declared form per count
- unheld: 3 simple · wrong · extend mcp/tests/mcp_reference.rs `the_old_word_intent_stands_only_as_a_json_name` to the other mechanism words
- unheld: 4 simple · para · add rows to mcp/tests/surface_size.rs `every_count_a_description_left_is_glossed_where_it_points` (a row per count)
- reason: Prop · crates/acquisition-cli/src/store_cmd.rs:729 states the property; the *Why:* (seats stopped on the same words) is at no comment · moves to crates/acquisition-cli/src/store_cmd.rs:1211
- fate: NT · complex
- confidence: medium · I read four of the ~40 citing sites; I did not read cli tests/seat_faults.rs:2166/2271/2526, store_answers.rs or plan unit/tests/shop.rs:459 (the *Pinned:* `c74_` test).

### C162 — Coverage and age are one derivation
- class: cross — every answer's facts block (search, plan listing and shop, store status, tabs) must take the store's one derivation. A new answer in any crate could compute its own oldest and newest fetch, and no test or check would fail.
- clauses:
  1. P · P · the derivation is held at store unit/tests/coverage.rs:64-424, and existing answers are shown to carry it (search tests/answer.rs:1227 `c162_total_carries_the_matches_fetch_span_on_every_view`, plan unit/tests/listing.rs:1159, shop.rs:839, cli seat_faults.rs:2667). "Applied by every answer" is not swept.
  2. M · Y · store unit/tests/coverage.rs:64 (empty set), :93 (never fetched by type and league), :111 (folders apart), :397 (scope span), :424 (serialized aged): the inputs and outputs the clause lists.
  3. P · N · I looked in tools/arch-check.sh, tools/docs-check.sh and `git grep` C162/coverage over crates and tools. No check refuses a surface's own derivation.
  4. M · R · C39 (neutral snapshots, no policy) and C1 (the store links no workspace crate). Where the derivation sits is stated at store/src/coverage.rs:12-20.
- unheld: 1 complex · wrong · a check would have to recognise a coverage derivation outside the store; a grep is a respelling away
- unheld: 3 complex · wrong · same seam as 1
- reason: Prop · crates/acquisition-store/src/coverage.rs:1 (the property and why it lives in the store; not that four surfaces derived it four ways) · moves to crates/acquisition-store/src/coverage.rs:1
- fate: SH-KO
- confidence: medium · I did not read plan unit/tests/listing.rs or shop.rs bodies, or search answer.rs past :1181.

### C163 — Every time a document carries has
- class: design — an agent reading any verb's document finds every epoch's age in words beside it.
- clauses:
  1. P · Y · crates/acquisition-verbs/tests/documents.rs:4031 `c163_no_document_carries_a_time_without_its_words`: walks every document `populated()` (:3025) builds through the makers each frontend calls, and fails on a time-named integer without words. :3986 checks the inventory.
  2. P · Y · documents.rs:2974: words under the time's name plus `_age`, in the vocabulary (`said`, :2855). :2992: a declared path for wrapper rows (`INVENTORY`, :3533) and no other exemption.
  3. M · R · C53 (an age made at the answer's own time). The walk checks the vocabulary, not the instant; documents.rs:604 holds the `generated_at` case.
  4. P · Y · documents.rs:4012: a null time is absent or `never`. :4057-4060: the walk never fails a null.
  5. M · Y · the pin is documents.rs:4031, stated at its doc :4026. "Every document a verb returns" is the hand list in `populated()`, so a new verb's document escapes until it is added.
- unheld: none
- reason: Prop · crates/acquisition-verbs/tests/documents.rs:4026 (and :3533) state the property; the *Why:* (a seat worked a date by hand from a time no text printed) is at no comment · moves to crates/acquisition-verbs/tests/documents.rs:2851
- fate: RM
- confidence: high · I did not read `INVENTORY` whole or `populated()` past :3051.

### C156 — A printed command is quoted for
- class: design — a printed command runs as printed in the shell a user pastes it into (the user-facing knob `ACQ_SHELL`), and a program gets the unquoted words.
- clauses:
  1. P · Y · crates/acquisition-search/tests/answer.rs:1119: each word of `shell-words.json` quoted per shell, and `shell_quoted` follows the environment's shell. cli tests/error_kind.rs:241: each word reaches `acq` whole through each real shell on the machine.
  2. P · Y · search tests/answer.rs:1125-1129: the four declared words are read, `bash` and `PWSH` are not.
  3. P · Y · cli tests/provider_default.rs:39 and mcp tests/provider_default.rs:18: `ACQ_SHELL=bash` is refused with "names no shell", in prose and JSON, before any verb.
  4. P · Y · search tests/answer.rs:1121-1124 (`Shell::detected` over windows × MSYSTEM).
  5. M · Y · answer.rs:1122 (Windows with MSYSTEM is POSIX). Stated at search/src/answer.rs:4005-4007.
  6. P · Y · cli tests/harness/mod.rs:282 `offered_line`: `argv` is the unquoted words, `acq` first, and the line is exactly them quoted. Used at plan_json.rs:449 and :488 and in price_json. Only offers and undos that go through this helper are checked.
  7. P · Y · cli tests/error_kind.rs:110: a cmd.exe-style paste is refused naming `set ACQ_SHELL=cmd` under pwsh, declared or on Windows; cli/src/terminal.rs:723 unit-tests the same.
  8. M · Y · cli tests/plan_json.rs:452 (the policy JSON is one quoted word) and :457-489 (it round-trips beyond ASCII through each shell, 5.1 on the Windows gate). Stated at cli/src/plan_cmd.rs:97.
- unheld: none
- reason: Prop · crates/acquisition-search/src/answer.rs:3942-4007 states the property. The *Why:* is in fragments: the pipe half at cli/src/plan_cmd.rs:97; no comment says detection cannot tell 5.1 from 7. · moves to crates/acquisition-search/tests/answer.rs:1109
- fate: RM
- confidence: medium · I did not read the daemon's twin `shell_word` (daemon/src/daemon/login.rs:782) or its test (daemon/tests/login.rs:1734). The Windows-only branches run only on the Windows CI gate, which I did not see.

### C187 — A script's presentation rule is held
- class: check — the entry says what a test harness does: a headless-browser check of `acq web`'s scripts, run in the gate.
- clauses:
  1. P · Y · crates/acquisition-cli/tests/browser.rs (e.g. :547 `the_count_beside_the_button_follows_its_own_form`) runs under nextest in the gate's `test` stage. Nothing in .config/nextest.toml, tools/gate.sh or .github/workflows filters it out. "Every script rule" is not sweepable; the module doc :5-22 lists the rules held.
  2. M · Y · browser.rs:32-40 and the loopback WebSocket client over `std::net::TcpStream`; harness/mod.rs:1955 (`Browser.endpoint`). Stated at browser.rs:1-4.
  3. M · P · tools/arch-check.sh:232 (check 5) refuses any member's edge, dev edges included, that reaches reqwest. A WebSocket crate that does not reach reqwest would pass. Stated at browser.rs:4 ("no crate added").
  4. P · Y · browser.rs:2418: with no browser found, `locate_browser` errs naming `ACQ_TEST_BROWSER` and each path; harness/mod.rs:1963 `start_browser` panics on that error, so the test fails and is not skipped.
- unheld: 3 cut · nothing outside the cli's tests relies on which client speaks the protocol; browser.rs:4 already says it, and its HTTP half is C1's (arch-check check 5)
- reason: Prop · crates/acquisition-cli/tests/browser.rs:1 (the property; not that a rule inside a script breaks silently before a push, nor the C1 reason) · moves to crates/acquisition-cli/tests/browser.rs:1
- fate: RM
- confidence: medium · I did not read the four named test bodies (:547, :750, :1124, :1357), only the module doc and the locator test.

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C121 | design | 2 | 0 | 2/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | high |
| C143 | design | 5 | 0 | 4/1/0/0/0 | Prop | NT | 1/0/0 | 0/1 | high |
| C146 | design | 4 | 1 | 0/4/0/0/1 | Prop | NT | 2/2/0 | 3/1 | medium |
| C162 | cross | 2 | 2 | 1/1/1/0/1 | Prop | SH-KO | 0/2/0 | 2/0 | medium |
| C163 | design | 3 | 2 | 4/0/0/0/1 | Prop | RM | 0/0/0 | 0/0 | high |
| C156 | design | 6 | 2 | 8/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C187 | check | 2 | 2 | 3/1/0/0/0 | Prop | RM | 0/0/1 | 0/0 | medium |

## What I did not check
- C143: the `acq web` page side (cli tests/web.rs:3168) and cli/src/printed.rs:208.
- C146: most of the ~40 per-count sites, including seat_faults.rs:2166/2271/2526, store_answers.rs:208/272/363 and the *Pinned:* `c74_` test in plan unit/tests/shop.rs:459.
- C156: the daemon's twin quoting function and its test. The Windows CI run.
- C162: the plan listing and shop test bodies.
- C163: whether `populated()` covers every verb's document (I read only its head) and the full `INVENTORY`.
- C187: the four named browser tests' bodies.
- I ran no test. All Y verdicts are from reading assertions.

## Noticed
- C163, step 2: the entry says "Unbuilt, #164", but the walk and the inventory exist and run (verbs/tests/documents.rs:3986, :4031), and commit 2bed3ba5 says "round 3 closed, every time a document carries has its words (#164)". The Unbuilt marker is stale. Its clause 5 ("every document a verb returns") is held only as far as the hand list in `populated()` goes.
- C187, step 1: clause 1 says "a stage of the gate", but the gate's stages (`build` … `tree`) have no browser stage. The check runs inside the `test` stage.
- C162, step 5: the class rule does not decide cleanly. "Held once in the store; a surface derives none for itself" reads as `edge` (which crate may own a computation), which comes before `cross` in the order. I chose `cross` because it is not a link or call boundary. Either is an owner's class, so the fate stays SH-KO.
- C121, step 2: clause 1 promises `accounts` with each refusal, but the daemon's refusals carry kind and message only (mcp tool_refusals.rs:240-258). The daemon is "alike" in kind and words, not in the `accounts` field.
- C146, step 3: the rules do not cleanly decide between `P` (pinned per count) and `U` (a rule on every future count) for clauses 1 and 2. I chose P/complex.
