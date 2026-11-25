# Investigator's report: admission

As returned by the Opus investigator under `INVESTIGATOR.md`, local paths shortened. Its claims are its own; what the orchestrator re-ran is marked in `findings/admission.md`.

**Track: admission ("may this route send now?"), HEAD 122c44fa. Read-only; ran no cargo and no acq.**

## 1. The picture

Admission already has a value at its core. `Limiter` is sync and lock-free inside, takes `now` as an argument, and is already in sans-IO form (`limiter.rs:217` `wait_for(&self, endpoint, now)`). `gate::State` is sync too; its unit tests poll it with `Waker::noop` (`gate.rs:271-285`). **V.** What is not a value is the composition: three mutexes, read by a loop that asks the same questions again on both sides of an await.

| Holder | Lock | Clock | What pays for it | Asked at (a job's path) |
|---|---|---|---|---|
| Limiter windows, `retry_holds`, endpoint state (Unknown/Policy/Policyless/Degraded) | `ChokePoint.limiter` | Instant | invariant 2, C18/C19, N20 | `pick` dispatch.rs:108; `process` :164; dispatcher loop :222; choke pre-gate :476/:515; post-gate :492/:529 |
| Gate `active`, `active_policies`, waiters | `gate.state` (nested under the limiter at `observe`, choke.rs:842-858) | none | invariant 1, C21, N6, P-B | choke.rs:482/:520 |
| Rails halts (block, trip, unreachable, ceiling, stopping) | `rails.state` | — | invariant 3, C25, C129, C133 | dispatch.rs:85, :156, :199; choke :470/:483 (HEAD :509/:521) |
| Rails C190 closure | `rails.state` | **wall** | C190 | inside `check`, choke.rs:409 |
| `closed_with` (the failure kind of a Degraded route) | daemon `Mutex` | Instant (matched by equality on `until`) | C165 | daemon.rs:1141, :1762 |
| `wanted` (cancel) | daemon `shared` | — | C25 | choke :473/:487 |
| Bearer staleness | session, read after admission | — | C87 | choke.rs:1112, :631 |

All **V**. On a job's path, the halt is asked 5 times, `check` 3 times, and both repeat every ≤1 s slice in two loops. Only the post-permit asks in `acquire_send` and `acquire_head` carry safety. Every earlier ask is scheduling: it keeps keys and permits free. **I**, from reading which checks gate the `carry`.

So the scouts' "five holders in three tiers" is right as a count. It is wrong as a diagnosis: the tiers are one safety check plus scheduling hints, not three authorities.

## 2. Theses

**T1. `active_policies` is a cache of `active`, and #206 is a cache-coherence bug. Delete the set.**
- **What goes:** the field, 3 of the 4 `debug_assert!`s (`gate.rs:113, 206, 256`; `git grep -n debug_assert gate.rs`), and the rekey's set juggling (~6 lines). `grant_waiters` derives "policy busy" from `active.values()`, which holds at most 2 entries.
- **Why the edge cases go:** there is no second fact left to disagree. Rekeying onto a live policy leaves two `active` entries under P, and P stays busy until both drop.
- **Smallest input whose behaviour changes:** `league@A`'s pre-discovery GET is live; a `P@A` send is live; the league answer names P; the P permit drops. Today a third P send is admitted beside the live league send. After the change it waits.
- **Invariants and rulings:** strengthens invariant 1 and N6. The 7 gate unit tests are unchanged.
- **Security:** better. **Performance:** none. **Function:** none on GGG's present policies (#66: league has its own policy). **An agent's ease:** one fact instead of two.
- **Objection:** at the instant of the rekey, two sends on P are already on the wire, so this bounds the hazard rather than removing it. True, but no design can recall a send.

**T2. Do not rewrite admission as one machine. Make the decision one sync function with an exhaustive match, and leave the gate, the transport and the shell alone.**

`fn admit(&self, route, key, wanted) -> Result<(), Hold|Refuse>` reads the halt, the endpoint state (matched on every variant), the limiter wait and the closure. It is called once before the gate and once after. `acquire_send` and `acquire_head` (choke.rs:463-537, 75 lines **V**) differ only in the gate kind and the mapping check, so they collapse into one loop of ~35 lines **I**.

The exhaustive match is the gain. Today `wait_for` maps any non-Policy state to "no policy wait" (`policy_for`, limiter.rs:736-741 **V**). That is #66's "final admission ignores Degraded": a Degraded route has no policy pacing at the last check, and nothing in the type makes anyone decide.

Things the one-value form would not remove:
- **C87's ordering.** The bearer is read under the permit and a rejection is published before release. That is session state, a shell step after Send.
- **The halt poll.** It is a Wait(d) clamp either way. The "Not doing" row (`decisions/daemon.md:66`) already prices it.
- **The halt race.** Send B passes its post-gate halt check, then A's block lands, then B carries. One lock orders admit against record, but B still leaves after the block became known. The race is microseconds and was ruled acceptable in C129's comment **I**.

**Count**, to my estimate (**I**): about 100–150 lines go (the double loop, the set, perhaps `closed_with`), not the core-runtime scout's 800–1,500. The limiter's arithmetic and ETA simulation (1,072 lines), the policy parse (599), `head` and `finish_send` classification, and the rails' journal, persistence and closure ladder all stay under any form. No test retires: the c161 and permit-spans-body tests pin the shell, which stays.

- **Smallest input whose behaviour changes:** a route Degraded while a job sits in final admission. All `degrade` callers are probe paths on Unknown routes (`git grep '\.degrade('`: kinds.rs:789, :838, :906; queue.rs:415; choke.rs:614 **V**), so this is barely reachable.
- **Invariants:** invariant 1 is unchanged and gains a typed decision.
- **Security:** slightly better. **Performance:** none. **Function:** none. **Ease:** one place to read the question.
- **Objection:** this is a refactor of the riskiest code for small structural gain. I answer that by keeping it to one function's extraction, with every existing test unchanged as the check.

**T3. The proof does not need the rewrite. A bounded explorer can run over today's sync values.**

Because `Limiter` and `gate::State` are already values, an exhaustive explorer can drive them directly. The gate futures can be polled with the noop waker, as its tests already do, with a halts flag and a counting-server oracle beside them.

**Sizing, honestly (I):**
- Alphabet: Want(task, route), Land(task, {ok k, 429, headerless, mismatch, block, 401}), Tick, Cancel, Release, Reset. With 3 tasks and 3 routes that is ~25–30 events.
- `Limiter` is not `Hash`, so without state dedup the reach is depth 5–6: 30^6 ≈ 7×10^8 sequences, too slow. Depth 5 is about 2.4×10^7, roughly a minute.
- An abstraction to ticks plus a `Hash` impl buys depth 10+.
- #206's counterexample is depth 5, and the PolicyMismatch one is depth 3, so both are within reach.

**What it would repeat, and what it would add:**
- "Never over the limit" needs an independent server count. That oracle exists: `assert_never_over_the_limit` over the counting mock (harness.rs:1052) **V**. It runs in only 5 call sites **V**.
- The window algebra is `pacing_table` and the fuzzed ETA check (spec.rs:611-1260); the weekly mutants run 566 caught (verification scout).
- The explorer adds only the cross-holder interleavings no scenario chose: rekey collisions, cancel at grant, Degraded at final admission, a halt between grant and carry.
- "Never send while halted" is a 2-line oracle. "Never retry through a block" is a property of `kinds.rs`'s retry loop, outside this machine.

**What it lets go:**
- The `trigger:live` parking of #66's gate edges: they get settled as proof or counterexample.
- The 7 hand-ordered gate interleaving tests in `gate.rs`, subsumed by the explorer with a few rows kept as documentation.

**Objection:** a model with max hits of 1–2 is not GGG's 30:300. The composition faults found so far, though, are structural, not numeric.

## 3. One thing I would defend

**The gate stays a separate, HTTP-blind RAII primitive.** The scouts would fold it into one `Admission` value.
- Its span is a send's lifetime, body included. The limiter's is an instant.
- Rust ownership already proves "released exactly once, including on cancel or unwind" (`Reservation::drop`, gate.rs:176). A pure machine turns release into an event someone must emit, with ids that can be released twice. The surviving `debug_assert` on unique ids is exactly that hazard.
- It has 5 commits in its history (`git log -- gate.rs`) **V**, and it is the one piece that is load-bearing for non-job senders: token POSTs across accounts, which the dispatcher's per-key rule never sees.

## 4. Where the old way is right

- The limiter as a value with `now` passed in, specified by tables (C33). The 4-0 design's "already a value; untouched" was correct.
- The journal oracle against a counting mock: that is the independent-oracle form of the property, already in place.
- The dispatcher's one-task-per-key is scheduling (priority and FIFO, C21), not a second authority. Keep it.

## 5. Soft spots: would the one-value form fix them?

- **#206:** the one-value form would only move it. Deleting the set (T1) makes it unrepresentable.
- **The `PolicyMismatch` arm skipping `assume_counted`** (limiter.rs:632-639 vs :622-628 **V**): the one-value form would only move it. Restructuring `observe_impl` so that "counted and not reconciled ⇒ `assume_counted`" runs once after the match closes the class for any future arm.
- **"Route closure four ways"** (`Degraded`, `retry_holds`, rails `closed`, `closed_with`): two behaviours, held vs failed, on two clocks, with one of them persisted. Unifying them moves rather than removes. Only `closed_with` is pure mirror: a `kind` carried in `Degraded` would delete it.

## 6. Position

Make only the decision pure, delete the two denormalizations, and run the explorer over today's values. Do not rewrite as one machine.

What would change my mind toward the rewrite: the explorer finds a counterexample that needs atomicity across limiter, gate and rails, which no local fix closes. Or a live journal shows two sends in flight on one policy, or a send admitted after a halt landed.

## 7. Where I part from the scouts

- **Core-runtime's 800–1,500 lines:** it counts code that no form removes.
- **"Two tiers serialize independently":** they share one key function (`serial_key`). The gate rule matters for refreshes and rekeys, not for jobs.
- **Verification's H1:** right that the property deserves exploration, wrong that it needs a C157 lift first.
- **The planning advisor's "don't review it":** wrong. #206 and the mismatch arm are real, small and closable.

## 8. What I did not check

- I ran no cargo and no explorer, so the state-space figures are arithmetic, not measurements.
- I did not read `queue.rs` beyond its degrade site, or `mockggg`'s window arithmetic (the oracle's own correctness).
- I did not check the weekly mutation timeouts (#151).
- I did not read whether `league`'s policy name is shared (N39).

## 9. Small finds

- After a waiter is rekeyed while it waits, `mapping_is_current` (choke.rs:491) drops the good permit it is granted and re-queues it at the back. Harmless, but it loses FIFO position **I**.
- `check` takes the max of a monotonic wait and a wall-clock closure (choke.rs:409), so a wall step shifts the closure **V**.
- `closed_with` matches by `Instant` equality on `until` (daemon.rs:1762) **V**. Carry the kind in `Degraded` instead.
- `head` builds its reason with `{other:?}` Debug text: "unexpected endpoint state after probe: {other:?}" (choke.rs:808) **V**.
- The `PolicyMismatch` asymmetry is still filed only as a comment on #66, with no `spec.rs` row **V**.
