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

Invariants derived from the register, not from the code. The first
four run over every harness journal; the rest are scenario pins:

| Invariant | Source | Pins |
| --- | --- | --- |
| journal send methods == wire, in order, header as line 0 | R4 | the oracle itself |
| per `(pid, route ≠ oauth-token)` the first send is `HEAD` | N16/N24 | probe before send |
| `counted == (method != HEAD)` | N24 | the product's accounting |
| a send is held only if the previous landed response on its route was a 429, then for `[Retry-After, Retry-After + RETRY_BUCKET_PAD + BUFFER]`; otherwise `wait_ms == 0` | N19, N33 | safety floor and performance ceiling at once (scripted servers only; against a counting mock the limiter holds before a violation and the last row applies) |
| after a 401 the account's next send is `POST oauth-token` (sequential, over the journal); no HEAD or GET carrying the rejected bearer reaches the server after a 401 was answered (concurrent, at the server) | N34, C87 | refresh on rejection |
| after a laptop sleep past expiry, no 401, a refresh reaches the wire, all sends within 60 virtual s | R8 | wall-clock expiry |
| no send answered 429; every reported window state `hits ≤ max` with no restriction active (`assert_never_over_the_limit`) | N4/N25 | never over the limit; real only against a counting mock (`MockPolicy`), in the N6 stress |

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

## What a rewrite keeps

- Component unit tests (the limiter's are its spec) survive with the
  component and vanish with it.
- The journal invariants and the R8 scenario need only a driving
  surface of about five calls: construct a daemon with a base URL, a
  clock and a credential store; submit a job; await it terminal; read
  the journal. `wait_ms` is a requirement on any design.
- Harness tests that reach into internals pin this implementation's
  concurrency mechanics; they die with a rewrite and should. The
  harness leaves `daemon.rs` the day a fresh build of the daemon
  starts, not before.
