# Admission: may this route send now?

The plan's D3. Investigator's report: `reports/investigators/admission.md`. Trial: `trials/admission-explorer.md`. Portfolio: proposal 4.

## What was found

The plan held two opposed views: the advisor would not review the best-tested code in the tree; two scouts wanted it rewritten as one pure machine. The investigator read all of it and took a third position, which the orchestrator shares.

Admission already has values at its core. The limiter is synchronous and takes `now` as an argument; the gate's state is synchronous and its own tests poll it with a no-op waker. What is not a value is the composition: three locks, read by a loop that asks the same questions on both sides of an await. On one job's path the halt is asked five times and the limiter three. Only the asks after the gate's permit carry safety; the earlier ones are scheduling.

So "five holders in three tiers" is right as a count and wrong as a diagnosis. There is one safety check and several hints.

## Checked by the orchestrator

- `gate.rs:105-113, 203-207, 250-256`: `active_policies` is a second record of what `active` already holds, kept in step by three `debug_assert!`s. #206 is that record going stale on a rekey.
- `limiter.rs:612-639`: the parse-failure arm calls `assume_counted` for a counted send; the `PolicyMismatch` arm beside it does not.
- `limiter.rs:736-741`: `policy_for` returns nothing for any state but `Policy`, so a route degraded while a job sits in final admission has no pacing at the last check.
- The admission loop itself was read whole (`choke.rs:463-537`).

## Position

Do not rewrite admission as one machine. Three smaller moves get the proof for little of the risk:

1. **Delete the cache.** Derive "this policy is busy" from the at most two live permits. The stale entry cannot exist; three of four `debug_assert!`s go. Smallest input that changes: a third send on a policy waits where today it could be admitted beside a live one.
2. **Make the decision one function** with an exhaustive match over the endpoint's state, called once before the gate and once after. About 100 to 150 lines go, not the 800 to 1,500 a scout estimated; the limiter's arithmetic, the policy parse and the rails stay under any form.
3. **Explore.** Because the gate and the limiter are values, a bounded explorer can drive them as they are, against an oracle that is not the gate's own bookkeeping. This is the charge's question answered: the property that protects the standing with GGG can be held by every sequence up to a bound and not only by scenarios someone thought of.

The gate stays a separate RAII primitive. A permit released by `Drop` cannot be released twice or forgotten; a pure machine would turn release into an event someone must remember to emit.

What would change this toward the rewrite: a counterexample that needs one lock across limiter, gate and rails, or a live journal showing two sends in flight on one policy.

Invariant 1 is strengthened and none is weakened. Every existing test stays as the check on moves 1 and 2.

## Route

Proposal 4 (its law half). Issues drafted: `policy-mismatch-skips-assume-counted`, `closed-with-matches-by-instant`, `check-mixes-two-clocks`, `a-rekeyed-waiter-loses-its-place`. Move 1 belongs on #206 as a comment when the issues are filed.

## After the trial and the skeptic

The trial (`trials/admission-explorer.md`) found #206 at depth 5 in a release build and the mismatch fault at depth 3, each in under two seconds, and nothing else to depth 8. The orchestrator re-ran both.

The skeptic (`dissent/theses-1-5.md`, thesis 4) is right on two points that bear on this track: a bounded search is not a proof of an invariant, and the trial's module prints what it finds and passes. It is a finding, not yet a test.
