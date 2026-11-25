# Trial A: a bounded explorer over the gate and the limiter

The charge asks: when exhaustive exploration is cheap, why is the
property that protects the standing with GGG held by scenarios someone
thought of? This trial tried the cheap exploration, with no rewrite.

An Opus agent wrote one `#[cfg(test)]` module of 435 lines at the foot
of `crates/acquisition-daemon/src/gate.rs`, ran it, saved it as
`admission-explorer.patch` beside this file, and restored the tree. The
orchestrator applied the patch again, ran both explorers, and restored
the tree again; the numbers below are from that second run.
`git diff --quiet -- crates` succeeds. Nothing landed.

## What it explores

**The gate.** Every sequence of events up to a depth, over three tasks
and three keys: acquire a permit under a key, acquire the HEAD permit,
rekey a route to a policy, release, cancel, and release-then-cancel a
future the gate granted and nobody polled. After each event every
waiter is polled with a no-op waker, as the gate's own tests do. The
oracle keeps its own model of which task holds which key and never
reads the gate's bookkeeping. It checks: at most two permits live; a
newly granted permit never shares its key with a live one; a HEAD is
exclusive; a waiting HEAD is granted once nothing is live; no eligible
waiter is held while a slot is free.

**The limiter.** Every sequence of a probe, a send that lands (with
good headers, with none, with another policy's name, or lost in
transport) and a tick of the clock, beside a counting server of one
policy at two hits per ten seconds. The oracle: after a landing, if the
server's window is full, the limiter must ask for a wait.

## What it found

`EXPLORE_DEPTH=6 cargo test --release -p acquisition-daemon --lib
explore_gate_trial -- --nocapture`:

| Depth | Sequences | Time |
| --- | --- | --- |
| 3 | 1,368 | 2.8 ms |
| 5 | 124,524 | 127 ms |
| 6 | 1,171,260 | 1.26 s |

At depth 5, in the release build:

```
[Acquire(0, a), Acquire(1, a), Acquire(2, b), Rekey(a → b), Release(0)]
(b) a new permit was granted on a key that already has a live one
```

Task 1 waits under the route's key. The route learns its policy is
`b`, on which task 2 is already live. Task 0 releases, and task 1 is
granted on `b` beside task 2. That is two sends in flight on one
policy, which the gate exists to prevent. It is #206, found by
enumeration, and shorter than the issue's own account: no third acquire
is needed, because the waiter that was rekeyed is the third send.

In a debug build the first `debug_assert!` panics at depth 3 and hides
the rest. A shipped daemon is a release build.

The agent ran the gate to depth 8 (102,798,996 sequences, 145 s) and
nothing else fired: the two-permit bound, the HEAD's exclusion and the
cancel-at-grant path all held.

`EXPLORE_DEPTH=6 cargo test --release -p acquisition-daemon --lib
explore_limiter_trial -- --nocapture`, 43,880 sequences in 89 ms:

```
[Probe, Send(Ok), Send(Mismatch)]
server window full
```

After a counted send whose answer names another policy, the server has
counted two of two and the limiter asks for no wait. Every failing
sequence holds a mismatched landing; with that event left out the agent
found nothing to depth 9 (10,001,912 sequences). The daemon does not
make up for it elsewhere: `kinds.rs:58` turns the error into a failed
job and the next job on that policy goes out at once. An existing test,
`mismatched_steady_state_observation_preserves_policy_and_topology` in
`spec.rs`, asserts the prediction is unchanged after such a landing, so
it pins the missing hit as intended.

Whether GGG can present either input is not known. A route's answer
naming a policy that another route already holds is how same-name
policies share counters (ground truth N6), so the first is not
far-fetched.

## What it did not reach

- A 429 landing: `observe_landed` is private to the rate-limit module.
  An explorer inside `spec.rs` would reach it.
- The composition. It drove two values side by side, never the choke
  point, the rails, a degraded route at final admission, or the real
  order of observe then rekey.
- GGG's real numbers. The model is two hits in ten seconds, one route,
  one policy.
- Depth beyond 8 on the gate. It has no state deduplication, and each
  level costs nine times the last.

## What it would cost to keep

About 300 to 350 lines once tidied, and about five seconds in the gate
(depth 6 on the gate, depth 7 on the limiter). It cannot pass until
#206 and the mismatch arm are fixed.

## What it shows

Two faults in the code that guards the project's most valuable asset,
one known for sixteen days and guarded by an assertion that is compiled
out of the shipped binary, one known only as a comment on another
issue, were each found in under two seconds by a search nobody had
written. Neither needed the rewrite two scouts proposed. Outside those
two, a hundred million sequences found nothing, which is the first
evidence of that kind this code has had.
