### C21 — The global burst bound sits at
- class: ggg — GGG/Cloudflare are promised that the cap on concurrent sends covers each send until its body completes, token calls included
- clauses:
  1. P · Y · `crates/acquisition-daemon/src/ratelimit/tests/spec.rs:3500`. For each bodied row of SEAM_TABLE (bearer GET, the token POST, plain GET), three sends whose bodies are held: two reach the transport and the third does not arrive within 50 ms, then arrives once one body is released. `gate.rs:295` holds the cap of 2 and per-policy serialization at the primitive. `spec.rs:2974` holds that a dropped send gives the permit back
  2. M · R · invariant 1, which names OAuth code exchange and refresh as inside the gate. (The refresh POST is also a row in the spec.rs:3500 table.)
  3. M · R · invariant 1 ("The browser-owned authorize navigation is outside this boundary")
  4. P · Y · `crates/acquisition-daemon/src/daemon/tests/dispatch.rs:84`: two jobs held behind an unanswered refresh stay Running while an independent job completes. `dispatch.rs:148` holds the same for two rate waits parked on the clock
- unheld: none
- reason: Prop · `crates/acquisition-daemon/src/gate.rs:11` states the property, the token routes and the absence of a job-task cap. The Why (Cloudflare watches bursts across policies and fronts the token endpoint) appears only as "Cloudflare-facing cap" at gate.rs:35 · moves to `crates/acquisition-daemon/src/ratelimit/tests/spec.rs:3493`
- fate: SH-KO
- confidence: high · I did not check whether a code-exchange (authorization_code) send has a gate test of its own; it is left to invariant 1

### C40 — `quote` is its own protocol request,
- class: edge — frontends and the plan are promised a separate protocol request that projects and never reserves or submits
- clauses:
  1. P · Y · `crates/acquisition-daemon/src/daemon/tests/quote.rs:115-117`: the job list and the send journal are unchanged across a quote, so it is read-only and reserves nothing. `Request::Quote` is its own variant, pinned by `crates/acquisition-protocol/tests/wire.rs:494`. The "never a flag on Submit" half is held only by that wire golden, which a regeneration would accept
  2. P · Y · `quote.rs:68-114`: the work is echoed in order (the basis), there is a per-policy scope with window headroom and an ETA, `observed_seconds_ago` per scope, and `not_covered` names the unlearned route, OAuth refresh, 429 re-sends and the non-sending jobs. The type requires `observed_at` (`acquisition-protocol/src/protocol.rs:354`)
  3. M · Y · a consequence of "non-reserving", held by the same assertion (quote.rs:115-117). It is already stated at the code, `protocol.rs:345` ("applying later may receive a different schedule")
- unheld: none
- reason: Y · `crates/acquisition-protocol/src/protocol.rs:343` (nothing sent, reserved or remembered; an estimate, not a promise) · moves to `crates/acquisition-daemon/src/daemon/tests/quote.rs:9`
- fate: SH-KO
- confidence: medium · the class is a judgement between edge (the protocol is the owner's per AGENTS.md) and design. I did not read `with_quote` in acquisition-plan

### C86 — The handshake has a deadline: a
- class: design — users and agents at every door are promised their own unresponsive verdict, with the holder's pid and the kill line, never absence or a replacement
- clauses:
  1. P · Y · `crates/acquisition-client/tests/contract.rs:342`, the c137_ test. Under a zero deadline against an answering peer, observe returns `Unresponsive` and panics on Absent (372-376). Connect returns `ConnectError::Unresponsive` and panics if the use door opens (386-394). Stop's error downcasts to `Unresponsive` (395-404). Line 364 asserts `HELLO_WITHIN == 5s`. Line 303 asserts "did not answer hello within 5s" in the words
  2. P · Y · `contract.rs:406-413`: the same peer, pid 4242, is still found after the doors ran, so nothing replaced it. The interactive connect returned the error rather than spawning. The holder 424242 from the lock is asserted at 380, 390 and 404. The kill or taskkill line, with the Keychain clause on macOS, is asserted at 312-326
- unheld: none
- reason: Prop · `crates/acquisition-client/src/client.rs:62` states the property whole. The reason (a healthy daemon answers in milliseconds, so a late hello is a wedged daemon) is a separate comment at `client.rs:320` · moves to `crates/acquisition-client/tests/contract.rs:333`
- fate: RM
- confidence: medium · the class could be argued as data (no replacement over a daemon that may be checkpointing). I did not read door.rs's expiry rows (door.rs:685)

### C96 — Place is terms, but the
- class: design — agents writing queries are promised that the realm is the request's scope, resolved and printed back, and that place words are terms
- clauses:
  1. P · Y · `crates/acquisition-search/tests/language.toml:342-349`: `realm:pc` and `-realm:pc` are refused with `realm_is_scope`. `tests/answer.rs:518-519`: over two realms, a load naming none is refused with `realm_needed`
  2. P · Y · `language.toml:327` round-trips the league, tab, character and container terms. `crates/acquisition-search/tests/acceptance.rs:569-580` filters by `character:`, `-has:character` and `tab:`. The worked query filters `league=Standard`
  3. P · Y · `answer.rs:564-567` refuses a request whose scope realm differs (`realm_mismatch`). The answer prints `scope.realm` (`answer.rs:52`, `:530`). `tests/language.rs:345` refuses a tree holding a `realm` field, so nothing can be hoisted from a branch
  4. P · Y · `answer.rs:518-532`: the error ends " — realm pc · realm xbox · realm all", and `all` is a scope that matched 8
  5. P · P · the live half is held: `crates/acquisition-search/tests/membership.rs:498-510` excludes removed items under the default load and includes them under all. The every-league half is loose: no test asks a league-less query over two live leagues holding fetched items in one realm and asserts both are matched. `answer.rs:994` asserts only the scope's league list
  6. P · Y · `answer.rs:51-79` asserts the scope's items, realm, fetched, never_fetched, oldest_fetch and its location, and leagues. `crates/acquisition-store/unit/tests/coverage.rs:440` holds `realms_held`
- unheld: 5 simple · wrong · `crates/acquisition-search/tests/answer.rs`, `worked()` plus `list_tabs`/`fetch_tab` into a second pc league, then `ask(&load(&s, Some("pc")), "")` asserting both leagues' items are matched
- reason: Cite · `crates/acquisition-search/src/corpus.rs:123` and `src/answer.rs:118-130` state fragments. No comment states the Why (a template and a total mean different things in two games) · moves to `crates/acquisition-search/tests/answer.rs:500`
- fate: NT · simple
- confidence: medium · I did not read the MCP H3 note (`acquisition-mcp/src/main.rs:56`), saved searches' realm settling (`acquisition-verbs/src/saved.rs`), or `counts.rs`

### C122 — A behaviour is pinned once, at
- class: check — the entry says how tests are written, laid out, isolated and run
- clauses:
  1. P · P · "pinned once at the lowest crate" is a rule on test writers that no check could fail on. "The process tests share one harness" is loose: `tools/arch-check.sh:653` (check 9) allows a spawn from any file under any crate's `harness/` directory, so a second harness of a crate's own passes. The sharing is by `#[path]` (`crates/acquisition-client/tests/harness/mod.rs:1`), and nothing checks it
  2. P · U · a rule on what a frontend's tests pin; no check over the tree could fail on it
  3. P · P · `crates/acquisition-cli/tests/harness/mod.rs:1232`: `golden` refuses a transcript with no facts, even under `ACQ_UPDATE_FIXTURES`. The other golden writers regenerate blind with no named facts: `cli/tests/harness/mod.rs:1810` (`page_text`), `acquisition-mcp/tests/counted_tasks.rs:1469`, `acquisition-plan/unit/tests/listing.rs:720` and `shop.rs:1042`
  4. P · Y · `tools/arch-check.sh:653-689`, check 9: outside `harness/`, no test names `CARGO_BIN_EXE_` or spawns anything but `kill`/`ps`. The daemon crate is excepted by name and starts `acqd` from an emptied environment
  5. P · Y · `tools/arch-check.sh:593-647`, check 8: at most one plain root per crate's `tests/`, files that set the environment are their own binaries, and `[[test]]` entries match the roots
  6. P · Y · held by the holder itself, as the retired seven are. `tools/gate.sh:301` runs `cargo nextest run`, and `.config/nextest.toml:9` and `:19` set `retries = 0`. No check refuses an edit that raises retries
- unheld: 1 simple · para · `tools/arch-check.sh` check 9, extended to require that each crate's `tests/harness/mod.rs` include the client harness by `#[path]` (the pinned-once half stays U)
- unheld: 3 complex · para · the non-CLI golden writers take no transcript with facts, so naming facts needs a new seam in each writer, or the clause narrows to the CLI's `golden`
- reason: Cite · `tools/arch-check.sh:593` gives the one-binary reason (modules link once; a process-wide write leaks). `.config/nextest.toml:1` gives the no-retry reason. Neither states "one wording was three edits" · moves to `crates/acquisition-client/tests/harness/mod.rs:1`
- fate: NT · complex
- confidence: medium · I did not read `docs-check.sh:394` (goldens named by their test file), the `Transcript` fact-checking code, or `arch-check-breakers.sh`

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C21 | ggg | 2 | 2 | 2/0/0/0/2 | Prop | SH-KO | 0/0/0 | 0/0 | high |
| C40 | edge | 2 | 1 | 3/0/0/0/0 | Y | SH-KO | 0/0/0 | 0/0 | medium |
| C86 | design | 2 | 0 | 2/0/0/0/0 | Prop | RM | 0/0/0 | 0/0 | medium |
| C96 | design | 6 | 0 | 5/1/0/0/0 | Cite | NT | 1/0/0 | 1/0 | medium |
| C122 | check | 6 | 0 | 3/2/0/1/0 | Cite | NT | 1/1/0 | 0/2 | medium |

## What I did not check
- C21: whether a code exchange (authorization_code) send is itself shown to take a gate permit. I left it to invariant 1's holders.
- C40: `acquisition-plan/src/lib.rs` `with_quote` and its unit test at `acquisition-plan/unit/tests/lib.rs:976`. I also did not read the body of the wire golden for Submit.
- C86: `acquisition-client/src/door.rs`'s expiry rows and `acquisition-verbs/src/daemon.rs`'s unresponsive arm. I also did not check the CLI's `daemon status` rendering of the verdict.
- C96: saved-search realm settling in `acquisition-verbs` (documents.rs:4449), MCP's H3 rule, and `counts.rs`'s realm-scoped routes.
- C122: how `Transcript.facts` is counted and checked, the docs-check golden-naming rule, and the breaker scripts. I did not run any check.

## Noticed
- C86, step 2: *Pinned:* names "the `c86_` tests in `acquisition-client/tests/contract.rs`". There is one, at contract.rs:303, and it holds only the report's words. The deadline, the verdict on every door and "nothing replaced" are held by `c137_the_handshake_deadline_at_zero_is_unresponsive_against_a_peer_that_answers` (contract.rs:342), which *Pinned:* does not name.
- C122, step 2: the headline says wording is a golden that "a blind regeneration fails". Only the CLI's `golden` helper enforces named facts. The CLI's `page_text`, the MCP's counted-tasks golden and the plan's listing and shop fixtures regenerate blind.
- C40, fate rule: clause 3 is the inverse of a promise (the quote does not bind the schedule). Had it been read as P and unholdable (U) beside Y clauses, the fate rule would not decide: rule 2 needs every P clause U, rule 3 needs every P clause Y, and NT names only P/N. I read it as M, stated at the code, to keep the fate derivable.
- C21, step 1: clause 1 largely restates invariant 1 ("one send-lifetime global gate"), whose *Pinned:* names the same spec.rs test. Only the "until its body completes" span goes beyond the invariant's words, which is why I kept it P.
