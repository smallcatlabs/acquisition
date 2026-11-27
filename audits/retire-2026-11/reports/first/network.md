### C17 — The rate limiter is a custom
- class: mech — nobody outside the limiter is promised anything here: the header-driven limiter is how invariant 2 is built, and taking no off-the-shelf crate is a choice about the build.
- clauses:
  1. M · R · restates invariant 2 (rate-limit headers are the source of truth); the custom-over-simple build is described in the doc at crates/acquisition-daemon/src/ratelimit.rs:1-8, and crates/acquisition-daemon/src/ratelimit/tests/spec.rs:727 `pacing_table` asserts the wait for each set of headers and history. No rate-limit crate is in any manifest.
- unheld: none
- reason: Prop · crates/acquisition-daemon/src/ratelimit.rs:1 ("The header-driven rate limiter (C17)"; the off-the-shelf reason is not given) · moves to crates/acquisition-daemon/src/ratelimit.rs:1
- fate: RM
- confidence: medium · the fate depends on the order of the fate rules, which is ambiguous for an entry with no P clause (see Noticed)

### C21 — The global burst bound sits at
- class: ggg — GGG and Cloudflare are promised that at most the cap of sends is in flight across all policies, token requests included, from dispatch until the body completes.
- clauses:
  1. P · Y · crates/acquisition-daemon/src/ratelimit/tests/spec.rs:3500-3550: for each body-carrying row of SEAM_TABLE, the third send does not reach the transport until one of the first two bodies is released ("a third actual send bypassed the global cap"). crates/acquisition-daemon/src/gate/tests/explore.rs:1-20 checks, at every event, that at most MAX_ACTUAL_SENDS permits are live.
  2. P · Y · SEAM_TABLE's `oauth-token` POST row (spec.rs:2629-2638) goes through the same cap test at spec.rs:3500. The code exchange and the refresh share the one `post_form` call at crates/acquisition-daemon/src/auth.rs:288. Check 5 and check 15 make every send pass through the choke point.
  3. M · R · invariant 1 (the browser-owned authorize is outside the boundary)
  4. M · P · crates/acquisition-daemon/src/daemon/tests/dispatch.rs:85-133 (an auth wait) and :148 (rate waits): two waiting jobs and one independent job all progress, so any global cap of 3 or more would still pass. No cap constant exists in the daemon.
- unheld: 4 cut · para · gate.rs:19-22 already states it; no caller relies on the absence of a cap itself, only on independent keys progressing, which the dispatch tests hold
- reason: Prop · crates/acquisition-daemon/src/gate.rs:11-27 (states the property; the Cloudflare reason appears only as "Cloudflare-facing cap" at gate.rs:35) · moves to crates/acquisition-daemon/src/ratelimit/tests/spec.rs:3493
- fate: SH-KO
- confidence: high · I did not read explore.rs's oracle body, only its doc

### C33 — The rate limiter's spec is test
- class: check — later sessions are promised that the limiter's spec is the tables the gate runs, with the ground-truth claims as their input.
- clauses:
  1. P · U · a rule on how the spec is kept. The tables exist (spec.rs:611 `pacing_rows`), but no check fails if a table is removed or replaced by prose.
  2. P · U · a rule on sessions. The rows cite claims in their `claims` strings (e.g. spec.rs:629 "N25 … N12 … N13"), but no check in tools/ reads `docs/design/network-ground-truth.md` or resolves the cited claims.
  3. P · Y · crates/acquisition-daemon/src/ratelimit/tests/spec.rs:727-739 `pacing_table`: each row's headers and history must produce its `expect_wait` (assert_wait). It runs under nextest in the gate.
- unheld: none
- reason: Prop · crates/acquisition-daemon/src/ratelimit/tests/spec.rs:1 (states the tables are the spec; the drift reason is not given) · moves to crates/acquisition-daemon/src/ratelimit/tests/spec.rs:1
- fate: NT · none (no fate rule decides a mix of U and Y; see Noticed)
- confidence: medium · I read the pacing table, not the other tables in spec.rs

### C150 — The one client honours the environment's
- class: ggg — a user behind a proxy is promised the tool works, with sends to GGG routed through the proxy while the pacing stays the daemon's. Scope is a plausible second reading.
- clauses:
  1. P · N · I looked in crates/acquisition-transport/src/lib.rs (the tests module at :166 and after), the daemon's tests, and a `git grep` of tools/.github for HTTPS_PROXY, NO_PROXY and `.proxy(`. No test sets a proxy variable.
  2. P · N · the same search; no variable is exercised
  3. M · N · build_client at crates/acquisition-transport/src/lib.rs:159-163 calls no `.proxy(`, but nothing asserts it. Check 5 holds the reqwest features line, not this.
  4. M · R · invariant 1 (one choke point, one gate)
- unheld: 1 simple · para · the transport crate's own tests module (crates/acquisition-transport/src/lib.rs:166, which already drives a local TcpListener): set HTTP_PROXY to a listener, send to another host, and assert the listener receives the request (a process-env test, nextest runs one process per test)
- unheld: 2 simple · para · the same test as rows over HTTP_PROXY, ALL_PROXY and NO_PROXY
- unheld: 3 cut · para · stated at crates/acquisition-transport/src/lib.rs:48-49; nothing outside the builder relies on how the routing is achieved
- reason: Prop · crates/acquisition-transport/src/lib.rs:48 ("routes through the proxy variables as reqwest's default does (C150)", no reason) · moves to crates/acquisition-transport/src/lib.rs:48, or to the new test's doc
- fate: SH-KO
- confidence: medium · I did not verify that reqwest 0.12.28 with default-features=false still honours the proxy variables

### C159 — The routes whose first send carries
- class: check — later sessions are promised that the wire oracle and every copy of the exempt-route set hold the daemon's one set. Which routes skip the probe is C32's and the ground truth's.
- clauses:
  1. M · R · C32 (NO_PROBE_ROUTES declared in routes.rs). The token endpoint's exemption is stated under C21 at crates/acquisition-daemon/src/gate.rs:16 ("no HEAD probes the token endpoint").
  2. P · P · the oracle half is Y: crates/acquisition-daemon/src/daemon/tests/harness.rs:836-847 asserts an exempt route's first send per lifetime is no HEAD and every other route's is one; it runs over every journal at harness.rs:746. The copy half is loose: check 11 (tools/arch-check.sh:845-850) reads only the three named copies, so a fourth copy anywhere else passes.
  3. M · Y · harness.rs:836-847, with probe_exempt at :861-864 reading the daemon's own constants
  4. M · Y · tools/arch-check.sh:845-856 holds the client harness's `matches!` copy and the two `NO_PROBE` copies in tools/*-verify.py equal to NO_PROBE_ROUTES plus TOKEN_ROUTE. tools/arch-check-breakers.sh:410-413 shows each drift refused.
- unheld: 2 complex · para · catching an unnamed copy needs a new tree-wide reader, and what counts as a copy of a route set is unclear (a grep that a respelling passes)
- reason: Prop · tools/arch-check.sh:747-753 and harness.rs:857 (both state the property; the reason about five places and probes on exempt routes passing unseen is not given) · moves to crates/acquisition-daemon/src/daemon/tests/harness.rs:857
- fate: NT · complex
- confidence: medium · the class depends on reading clause 1 as C32's (see Noticed)

### C161 — Every request the daemon sends to
- class: check — later sessions are promised that a door scenario's provider exchanges are in-process, so the park hook sees every wait. The seam and the one client are invariant 1's mechanism.
- clauses:
  1. M · Y · crates/acquisition-daemon/src/ratelimit/tests/spec.rs:2729-2839: the choke point writes the user-agent, the bearer (marked sensitive) and the form onto the prepared request, and the production transport delivers exactly that to a listener. spec.rs:2870-2916: each send is cut at dispatch plus REQUEST_TIMEOUT on the choke point's clock, not 1 ms before, and its permit is released.
  2. P · Y · crates/acquisition-daemon/src/daemon/tests/door.rs:102-132 and :854: every Server variant, `OwnServer`'s type included, is an InProcess transport, and the door always passes it to `.transport(...)`. Check 15 (tools/arch-check.sh:1015-1090) puts the word Production on one workspace line, so a scenario cannot build a real transport except by building around the structure on purpose.
  3. M · R · invariant 1 (check 5 and check 15 are its pins)
  4. M · R · invariant 1 (what its boundary leaves outside)
  5. M · N · stated at spec.rs:30 and spec.rs:2727 ("stays TCP for good"). No check fails if the real-wiring tests became in-process.
  6. M · Y · crates/acquisition-daemon/src/tests/transport.rs:810-815 (held from the first poll to the body's end), :934 and :942 (a silent head or body holds nothing). door.rs:2014-2019 asserts the clock does not move while a request is held.
- unheld: 5 cut · para · a rule on how tests are written, already stated in doc comments; no caller relies on it
- reason: Prop · crates/acquisition-daemon/src/tests/transport.rs:1-3 (states that no provider exchange in a scenario is real I/O; "a real socket was the one wait the park hook could not see" is not stated) · moves to crates/acquisition-daemon/src/tests/transport.rs:1
- fate: RM
- confidence: medium · I did not open the c158 wire-table rows the *Pinned:* names, nor spec.rs:3025 (the production connection close)

## Rows
| id | class | P | M | held Y/P/N/U/R | reason | fate | scope s/c/x | worth own/para | conf |
| C17 | mech | 0 | 1 | 0/0/0/0/1 | Prop | RM | 0/0/0 | 0/0 | medium |
| C21 | ggg | 2 | 2 | 2/1/0/0/1 | Prop | SH-KO | 0/0/1 | 0/0 | high |
| C33 | check | 3 | 0 | 1/0/0/2/0 | Prop | NT | 0/0/0 | 0/0 | medium |
| C150 | ggg | 2 | 2 | 0/0/3/0/1 | Prop | SH-KO | 2/0/1 | 0/2 | medium |
| C159 | check | 1 | 3 | 2/1/0/0/1 | Prop | NT | 0/1/0 | 0/1 | medium |
| C161 | check | 1 | 5 | 3/0/1/0/2 | Prop | RM | 0/0/1 | 0/0 | medium |

## What I did not check
- Whether reqwest 0.12.28 built with `default-features = false, features = ["json", "rustls-tls"]` honours HTTPS_PROXY, HTTP_PROXY, ALL_PROXY and NO_PROXY. C150's clauses 1 and 2 may be false today, not just untested.
- The c158 wire table rows that C161's *Pinned:* names (`c158_the_wire_table_through_the_door` in dispatch.rs), spec.rs's `c161_the_deadline_closes_the_production_transports_connection`, and explore.rs's oracle body (I read its doc only).
- Whether tools/arch-check-breakers.sh runs in CI. I relied on the retirement commit 51f2e4f9, which says it is staged there.
- The other tables in spec.rs beyond `pacing_rows`, and the refresh's call path in auth.rs beyond the single `post_form` site at :288.

## Noticed
- C21, step 2: crates/acquisition-daemon/src/ratelimit.rs:23-24 says "The dispatcher cap remains separate job-scheduling machinery". This disagrees with C21's clause 4 and with gate.rs:21 ("has no global job-task cap"); no cap constant exists in the daemon, so the ratelimit.rs sentence looks stale.
- C17, the fate: with no P clause, fate rule 2 ("every P clause is U") is vacuously true and would give SH-KU before rule 3 is reached. I took rule 3 (RM) as intended; the rules should say how an entry with no promise is decided.
- C33, the fate: its P clauses are U, U and Y. No rule decides this mix: rule 2 needs all U, rule 3 needs all Y, and rule 4 needs a P or N. I wrote NT with no scope; the honest reading may be that its unholdable rule-on-sessions clauses keep it, as KU.
- C159, step 5: the class depends on whether clause 1 ("the routes … are the daemon's declaration and the token endpoint") is a restatement of C32 plus the token exemption under C21's doc, or a GGG-facing promise of its own. Read the second way, the class is ggg and the fate is SH-KO. The token endpoint's exemption is stated in no entry, only at gate.rs:16.
- C161, step 5: clause 1 can be read as a GGG-facing promise (each send's deadline cuts a hung connection and releases its permit), which would make the class ggg and the fate SH-KO. I read it as invariant 1's mechanism. Separately, the transport crate's doc (crates/acquisition-transport/src/lib.rs:32-35) records a property held "by review … and by no check" (no reqwest item in the public API), which no clause of C161 names.
- C150, step 2: *Pinned:* is "nothing yet", and that is accurate; I found no test.
