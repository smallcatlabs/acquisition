# Testing notes

How this project checks its own work: what the tests are for, the
shapes of check that read as coverage and are not, and the one contract
surface on the GGG side. The experiments and narratives behind these
are history: the register walk at `7aee76ee`, the file before this cut
at `b6197a82`.

## Purpose

The tests, constraints and abstractions here serve two things:
hardening the current implementation, and pinning behaviour so that
the daemon's internals can be rewritten, in place or by a fresh build
judged by the same suite, without losing the ability to catch
rate-limit violations and performance degradation. When they block or
impede that service we break or change them.

The central tension is that "simple but generic" and "catches real
regressions" pull against each other at the level of a single
assertion. Too tight breaks on every legitimate refactor until someone
loosens it into meaninglessness; too loose is a check that reads as
coverage and cannot fail. Where the right tightness is obvious the
invariant is probably not pinning anything interesting; where it is
genuinely hard is where the contract lives. The method that found the
line was empirical: walk the bug register and ask of each entry what
invariant over observable output would have caught it.

## Looks like coverage, isn't

Each a shape met more than once; the question behind them, *what else
here reads as a check but cannot fail?*, is the most useful one this
project has, and matters more as goal-seeking agents build against the
suite.

- **A stop condition that cannot fail.** "One HEAD per route" was a
  tautology with no restarts.
- **A checker that drops the evidence it exists to find.** Timestamps
  compared as strings.
- **Two clocks in one test.** A journal stamped with real time under a
  virtual-time assertion; the mock on real time and the daemon on a
  manual clock, so the mock's windows never expired and "one job fails
  after three 429s" was the mismatch, not a property. One `Clock` with
  two faces drives limiter, token expiry, journal and the mock; tests
  speak the daemon's clock. What stays on real time is deliberate: the
  mock's `sleep` job and the watchdog's poll.
- **A measurement where the thing cannot be seen.** A `wait_ms`
  measured where the wait was not.
- **A rule over inputs that never carry the case.** The 401 rule ran
  over journals none of which carried a 401; the first that did showed
  the product re-sending the rejected bearer (now C87), and the fix's
  sequential scenarios were blind to the gate until the pin moved to
  the server, where dispatch order is visible.
- **The rails verify behaviour; nothing verified which code.** A soak
  ran a binary predating the fix it was restarted for, every rail
  correct and blind to it. Provenance is its own category (C84).
- **A clean run proves less than it feels like it does.** R8, the
  sleep-frozen expiry clock, was found by reading code and confirmed
  live only because the fix was missing.
- **A fix that is resolved only while a rail is armed.** With the
  tripwire off, the shipped default, the product still re-sent a dead
  grant per flight (now C24).
- **Read the sites before deciding what to do with them.** "Sixteen
  sequence-literal assertions" were seven redundant literals and nine
  counts already the right shape; the plan changed from convert to
  cross-check.
- **Commit the check, then break the code.** A chained `cd` that failed
  silently into a `git checkout` discarded an uncommitted invariant.

## The journal is the contract surface

The send journal records what the product emits: pid, method, route
(the endpoint key with its account, `stash@Alice#1234`), status,
`counted`, every rate header, a timestamp on the daemon's clock, and
`wait_ms` (held from dispatcher-ready to transport dispatch); a non-2xx
adds the response headers, which for a failed HEAD probe is the whole
of the evidence. Every harness daemon journals;
`assert_journal_matches_wire` holds the journal equal to what the test
server received, so the journal's own failure is caught by the recorder
and the recorder's coupling by the journal.

Invariants derived from the register, not from the code. The oracles
are in `crates/acquisition-daemon/src/daemon/tests/harness.rs`, and
every journaling harness daemon ends through one of two teardowns: the
scripted one (`finish_harness`, a scripted server or none) runs the
contract oracle (`assert_wire_contract`) and pacing follows responses
(`assert_pacing_follows_responses`); the mock one (`finish_harness_mock`,
the counting mock) runs the contract oracle and never over the limit
(`assert_never_over_the_limit`), the two pacing oracles being
complements; where a scripted server records, `finish_harness_wire`
adds the wire comparison (`assert_journal_matches_wire`). Two tests trip
the rails by recording a forged probe answered 429, never a forged
send. The rest are scenario pins:

| Invariant | Source | Pins |
| --- | --- | --- |
| journal send methods == wire, in order, header as line 0 | R4 | the oracle itself |
| per `(pid, route)` the first send is `HEAD`, except `oauth-token` and the two routes the daemon never probes, `profile` and `league` | N16/N24 | probe before send |
| `counted == (method != HEAD)` | N24 | the product's accounting |
| a send is held only if the previous landed response on its route was a 429, then for `[Retry-After, Retry-After + RETRY_BUCKET_PAD + BUFFER]`; otherwise `wait_ms == 0` | N19, N33 | safety floor and performance ceiling at once (scripted servers only; against a counting mock the limiter holds before a violation and the last row applies) |
| after a 401 the account's next send is `POST oauth-token` (sequential, over the journal); no HEAD or GET carrying the rejected bearer reaches the server after a 401 was answered (concurrent, at the server) | N34, C87 | refresh on rejection |
| after a laptop sleep past expiry, no 401, a refresh reaches the wire, all sends within 60 virtual s | R8 | wall-clock expiry |
| a restored session with an unexpired access token sends no `POST oauth-token` in ordinary reuse (a 401 or `auth check` refreshes sooner) until within the refresh margin of expiry, and one there | C135 | the access token kept beside the refresh token |
| no send answered 429; every reported window state `hits ≤ max` with no restriction active (`assert_never_over_the_limit`) | N4/N25 | never over the limit; real only against a counting mock (`MockPolicy`), in the mock teardown's tests |

Each has been broken deliberately and seen to fail. A pre-emptive hold
appears as `wait_ms` on exactly the send that was held and as ETA on
every job behind it; a column of zeros with one large value is the
expected shape.

## Three kinds of "cannot"

About half the register's entries have a journal invariant. The rest:

1. **Not on the wire**: reporting, persistence, config, job identity,
   the reason a job was refused. A rewrite may change all of it.
2. **Absence**: a line-based oracle cannot see a send that never
   completes; the answer is the client's timeouts.
3. **Vacuous offline**: *never over the limit* and *same-name policies
   share counters* are real only live or against a mock that keeps
   counters; a scripted server echoes whatever the script names, so
   asserting them there would pin the script. `mockggg.rs` keeps
   counters on the test's `Clock`, so the N6 stress asserts them; the
   scripted-server tests stay kind 3 by nature. Kind 3 is the list of
   things a goal-seeking builder could get wrong without the suite
   noticing; the live runs are the un-gameable half and stay human-run.

## The suite's shape

The suite is organised by contract surface: each surface has one
oracle and one corpus, and a finding becomes a row in that corpus, a
case in an existing test or a line of an existing transcript before it
becomes a file of its own (C153, the stopping rule). The four
surfaces, each as the tree holds it and where the test-shape group
(`roadmap/TEST-SHAPE.md`, #137 to #143) moves it:

- **The GGG side.** The oracle is the journal ("The journal is the
  contract surface" below), the corpus the scripted-server scenarios.
  Today the daemon's harness drives internals beside them; the group
  moves the scenarios to the five-call door of "What a rewrite keeps"
  and the permutation families to tables.
- **Facts and plans.** The oracles are the store's public API, the C35
  annotation tests, the migrations, the C38 envelope and the committed
  JSON schemas. A test naming the store's SQL internals is mechanism
  (C48, the schema is internal).
- **The search.** The oracle is the answer: query text in, answer JSON
  out, over items entered through the store's ingest, with the
  metamorphic properties and the trade oracle over captured searches.
  The per-feature examples move to one case table.
- **The doors.** The oracle is the CLI's golden transcript and the
  MCP's field-for-field comparison over one facts table ("The
  journey"); the protocol's wire fixtures stay as they are. The group
  moves the fixture worlds to one canonical world and the short
  goldens into a few long transcripts.

A behaviour is pinned once, at the lowest crate that holds it, and a
frontend's tests pin what its adapter adds (C122). A surface's wording
is held by one golden; a sentence literal elsewhere asserts only what
no golden holds. A test or a check is retired by a commit line,
`retired X; held by Y`, where Y runs in the gate or in CI at that
commit; a worker that cannot name Y keeps X and files it (C154, the
retirement rule).

Every process test takes one harness,
`crates/acquisition-client/tests/harness/mod.rs`, whose one isolation
list scrubs every `ACQ_*` by prefix; the CLI's and the MCP's
`tests/harness/mod.rs` bind it to their binary. The CLI's wording is
held by goldens under `crates/acquisition-cli/tests/goldens/`, each
command naming the facts a reader must find, checked before the update
switch, so a blind regeneration fails; `tools/docs-check.sh` refuses a
golden no test names. A crate's integration tests are one binary,
`tests/main.rs` of modules; a test that sets the process environment
(one that calls `session(`) is its own binary, because a process-wide
write leaks into every test beside it under `cargo test`.
`tools/arch-check.sh` holds that layout, and the gate runs the suite
under nextest (`.config/nextest.toml`), a process per test, never
retried.

## The journey

One first session, from login to `daemon stop`, is held at both doors
over the mock's real-shaped items (C123): the CLI's
`crates/acquisition-cli/tests/journey.rs` records it as one golden
transcript, `goldens/journey/a_first_session.txt`, and the MCP's
`crates/acquisition-mcp/tests/journey.rs` drives the same steps through
the tools, each field compared. Both read one table of facts,
`crates/acquisition-client/tests/harness/journey.rs`: the requests per
plan, the tabs and items, each query's total, the price, the shop's
page, the journal's GETs per route. The CLI's golden holds the wording;
the table holds the numbers both doors must see, so a change to the
mock's items is one edit there and one regeneration.

Each test starts its own daemon with the journal on its own command,
and after the stop checks the journal by field through the shared
reader (`assert_journal_holds`): the header and one pid across every
send, the first send per route a probe but where the daemon never
probes, `counted` exactly on non-HEAD sends, no 429 and no window over
its maximum, and the GETs per route exactly what the plans named.

Two invariants are out of the journey's reach. The user-agent is no
journal field, and the mock records nothing it receives, so no test
holds it here. A retry through a Cloudflare block cannot happen: the
mock never serves a block, and a rule over inputs that never carry the
case pins nothing.

## What a rewrite keeps

- Component unit tests (the limiter's are its spec) survive with the
  component and vanish with it.
- The journal invariants and the R8 scenario need only a driving
  surface of about five calls: construct a daemon with a base URL, a
  clock and a credential store; submit a job; await it terminal; read
  the journal. `wait_ms` is a requirement on any design.
- Harness tests that reach into internals pin this implementation's
  concurrency mechanics; they die with a rewrite and should. The
  harness leaves `daemon/tests/` the day a fresh build of the daemon
  starts, not before.
