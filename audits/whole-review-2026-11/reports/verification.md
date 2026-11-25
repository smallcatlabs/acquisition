**State machines and formal verification: what is proven and what is hoped (read-only, HEAD 122c44fa)**

## 1. Model

The project already has one good verification idea: a **pure step function**. C157 (`decisions/daemon.md:27`) says a lifecycle is a value: `step(state, event) -> effects`, with time arriving only as events. It is pinned by a table of event scripts. Only two machines are built that way:
- `Life`, in `daemon/life.rs`. It has 3 states, and `#[deny(clippy::wildcard_enum_match_arm)]` makes the compiler check every state-event pair (`life.rs:161`).
- The client `Door`, in `client/door.rs`. Its `step` ends in a catch-all `(state, event) => panic!` (`door.rs:365`), the opposite convention for the same shape.

The limiter is the "model" the 4-0 design cites. It is sync and every method takes `now: Instant`, but 4-0 marked it "already a value; untouched" (`briefs/4-0-design.md:216`).

The property invariant 1 actually depends on is how the limiter, the gate `State` (`gate.rs:42`), the rails halt and rekey combine. That combination lives only in the async loop `acquire_send` (`choke.rs:463-503`). It is checked by hand-written scenarios. Each scenario's journal is checked afterwards by trace oracles (`assert_pacing_follows_responses` in 72 teardowns, `assert_never_over_the_limit` on scenarios facing the mock).

Every other machine is implicit: a set of fields whose legal combinations nobody wrote down.
- **Job:** `JobInfo.state` + `outcome: Option` + `cancel_requested: bool` + `deferred: Option`, with 7 unguarded named transitions (`entry.rs:24-130`). "One ending" is enforced by privacy.
- **Session:** `access_token`/`refresh_token`/`refresh_flight`/`pending` Options + generations + `keyring: String` (`daemon.rs:549-565`).
- **Rails:** six halt causes as `Option<String>`/`bool`. Their precedence lives in `halted()` (`rails.rs:532-557`); which reset clears which cause is written only in prose.
- **Route closed:** four homes. Limiter `Degraded{until}` (Instant), `retry_holds`, rails `closed` (C190, wall clock, `rails.rs:559`), and daemon `closed_with`.

## 2. Measurements

| What | Value | Command |
|---|---|---|
| Machines in C157 form (pure step + table) | 2 (Door, Life) | `git grep -n 'fn step' crates`; `decisions/daemon.md:27` |
| Gate: lines / unit tests / `debug_assert!` | 452 / 7 / 4 (lines 113, 206, 256, 259) | `wc -l`; `grep -n debug_assert gate.rs` |
| Limiter + policy + choke lines; spec tests | 2,848; 72 | `wc -l ratelimit/*.rs`; `git grep -c '#\[test'` |
| Crates with proptest | search, plan, client, daemon. Daemon's are only header-parse no-panic (`spec.rs:4370`) and `frame.rs`, which is duplicated byte-for-byte in client after line 20 | `git grep -c 'proptest!' -- crates`; `diff` of the two `frame.rs` |
| Store, verbs, cli, mcp, protocol proptests | 0 | same |
| loom / shuttle / kani / miri / cargo-fuzz / stateright / bolero | none. The only "fuzz" is a fixed-seed SplitMix ETA-vs-simulation check (`spec.rs:1111-1260`) | the brief's grep; Cargo.toml dev-deps |
| `unsafe` | 41, all FFI (`world*.rs`) or test env setters, so Miri would add little | `git grep -w unsafe` |
| Std `Mutex` fields in daemon / choke / gate / rails | 14. The lock order is a doc comment (`daemon.rs:1032-1041`); only the no-await-under-guard half is machine-checked (clippy `-D warnings`) | `git grep 'Mutex<'` |
| Weekly mutants, run 45221916564 | 803 mutants: 566 caught, 89 timeout (counted caught), 141 unviable, 7 missed (#320) | `gh run view --log`, summary lines |
| Weekly run outcomes, last 10 | 6 failed, 4 passed | `gh run list --workflow weekly.yml` |
| Audit §5 "provably equivalent" items taken | 0 of 9 sampled: `persist(&mut Entry)`, hand `job_state`, `keep_protocol_body`, `pub selector_tree`, 2 `expect`s, `parse_wait`/`parse_within`, `install_token_floor_hold`, two `frame.rs`, `Vec<Option<f64>>` | grep at the cited sites |

## 3. Where a deep review would pay (ranked)

**H1. Lift admission into a C157 value and model-check it.** Value: highest. Tractability: high.

- **Evidence.** The limiter and the gate `State` are already sync and clock-free. The only async glue is the 40-line loop at `choke.rs:463-536`. #206 (still open) says rekeying onto an already-active policy loses one `active_policies` entry: `remove` + `insert` returns false, and `debug_assert` is a no-op in release. I verified this at `gate.rs:107-114` and `:203-207`. A third send on that policy is then admitted alongside a live one, which is the N6 hazard. #66 lists two more edges ("final admission ignores Degraded"; headerless first contact on a no-probe route). Both are parked under "GGG doesn't present it".
- **Proposal.** An `Admission` value with events `Want(route)`, `Tick(t)`, `Landed(route, headers)`, `Rekey`, `Halt`, `Drop(permit)` and effects `Sleep(d)` / `Grant` / `Refuse`. Then a hand-rolled bounded exhaustive explorer in a test, in the same shape as the door table, over small universes: 2–3 routes, 1–2 policies, windows of 1–3 hits, ≤8 events. It checks against an independent oracle:
  - per-policy sliding-window count ≤ max;
  - ≤2 in flight, ≤1 per policy, HEAD exclusive;
  - no grant after an observed halt;
  - liveness: after a hold expires, a waiter is granted within one tick.
- **Tools.** Proptest-state-machine is the cheaper alternative and fits the existing `generated.rs` helper. Loom and shuttle are the wrong tools: the interleavings are task-level under one lock each, not memory-model races. TLA+ is worth it only as documentation.
- **Payoff.** #206 and #66 settle as a proof or a counterexample. The 4 asserts become typed results. Many of the 89 timeouts in `choke.rs` (#151) should become fast failures. Invariant 1's "Pinned" line gains a property, not just scenarios.
- **Read:** `gate.rs`, `choke.rs:396-540`, `limiter.rs:60-100, 370-500`, #66, #206.
- **Confidence:** high on feasibility; medium on whether it finds a real bug beyond #206.

**H2. Session/refresh as a C157 value.** Value: high (refresh single-flight, C87 bearer rule, C24 dead grant). Tractability: medium.

- **Evidence.** The state is spread across Options and counters plus a string-typed keyring word with equality checks: `keyring != "ok"` (`refresh.rs:306`), `= "saving"` (`:591`), `== "ok"` (`login.rs:337`). The audit's §2 order-dependent pairs are exactly this machine's interleavings (reject vs install; the dead-grant mark gap; restore vs submit). Today they are pinned by scenarios in `tests/refresh.rs` and `bearer.rs`, plus one stress test.
- **Proposal.** An enum for the keyring word and a session `step` over {read bearer, 401, install, login, logout, mark failed}, explored the same way as H1.
- **Read:** `refresh.rs:24-420, 580-650`, `login.rs:280-345`, `daemon.rs:540-580`.
- **Confidence:** medium.

**H3. Make illegal job states unrepresentable, and enumerate crash points.** Value: medium-high. Tractability: high.

- **Proposal.** `enum` Waiting{retries} | Running | Deferred(Outcome) | Ended(Outcome) replaces the four-field product. `persist` takes a `Held`-style token, the same pattern as `CredentialsHeld` in `held.rs`. Then a test injects a failed persist at each call site, restores, and asserts the restored set ⊆ the allowed map (audit §3, §4b.3/5).
- **Payoff.** The doc-only lock precondition at `queue.rs:214` and the unguarded transition methods both go away.
- **Confidence:** high.

**H4. Store state-machine proptests.** Value: medium. Tractability: high. Store has no proptest dev-dependency today.

- **Ingest commutation.** Sequences of listings and fetches over 2–3 tabs, checked against an oracle where the listing owns name and type. #205 was found by reading; this would find its class.
- **Annotation CAS.** Put and delete sequences against a map oracle with monotone revisions (`annotations.rs:1100-1133`, already a clean `match`).
- **Confidence:** high that it is cheap; medium on yield.

**H5. One C157 convention.** Life spells every pair (a no-op where it cannot act); Door panics on any unlisted pair. Pick one. Then add a cheap reachability test: BFS over every event sequence from the initial state up to depth N. For Door it shows no driver-producible event can panic; for Life that no pair is spelled twice.

## 4. Low yield / already covered

- **Search.** It already has 18 property tests: parse∘print round trip (c104), equivalence (invariant_7), routes partitioning, plus the C92 exact sums. The residual is the case-canonicalisation gap (audit §4b.14), which I did not re-verify.
- **Migration ladder.** One idempotent step, v8→v9, under an IMMEDIATE transaction (`store/lib.rs:1473-1497`).
- **Kani / Miri.** The core is `HashMap`/`String`/async-heavy, and `unsafe` is FFI-only.
- **Web form, saved searches, `Holder`.** The URL is the identity (C16); saved searches use the same CAS door as annotations. These are caches, not machines.
- **Mutation testing.** It works and is read: #320 found the 7 misses, all in route closure (C190) built days before. Rotted exclusion lines are now gated (#321). Widening it is mostly runner time: about 85% of a shard is building mutants (#151), and six shards already near the job's ceiling. Whole-store mutation would roughly double that. Prefer H1–H4 first.

## 5. Blind spots

- **Lock order.** The order is a comment. A debug-build lock-level wrapper, or a ranked-guard type, would check it. Nobody has measured whether any nesting violates it today.
- **Timeouts counted as caught.** 89 of 662 viable mutants (13%) time out. These include `classify_api_body` and `Verdict::of`, which hold invariant 3, where a hang is weak evidence that a test checks the property.
- **Two clocks in one admission decision.** `ChokePoint::check` takes the max of the limiter's Instant wait and rails' wall-clock `closed_for` (`choke.rs:409`).
- **The mock is the trace oracle's ground truth.** `assert_never_over_the_limit` trusts the mock's reported hits (`harness.rs:1052`), and its `_exact` parameter is unused. Nobody checks the mock's own window arithmetic against an independent model.
- **ManualClock wake order is the scheduler's** (`4-0-design.md` §1). I could not tell whether C158's park-based clock fully closes this.

## 6. Surprises

- **None of the audit's §5 "safe to take mechanically" refactors was taken**, two weeks later, while its §4 findings were filed (#205–214) and several closed. The mechanical, type-level wins have no owner.
- **The daemon's proptests don't touch the property the project most fears.** They cover a duplicated frame reader and header parsing; nothing generated touches admission.
- **"GGG doesn't present it" (#66) stands in for proof** on the component that protects the project's most valuable asset.
- **The weekly mutation run has failed in 6 of its last 10 runs.** #320 says the 2026-11-23 red went unread for a day.

**Verified myself:**
- The gate rekey hazard and the `debug_assert`s (`gate.rs:100-124, 196-260`).
- The `acquire_send` structure and the purity of the limiter and gate (`choke.rs:463-536`, `gate.rs:209-260`).
- The implicit job and session states (`entry.rs:24-31`, `daemon.rs:549-565`, the keyring string comparisons).
- The mutation numbers, from the run log.

**Recommendation:** yes, include a formal-validation phase. Keep it small and inside the project's own idiom, not new toolchains:
1. **H1** (admission as a C157 value plus a bounded exhaustive explorer against an independent window oracle). Expected payoff: #206 and #66 settled, invariant 1 pinned by a property rather than scenarios, and faster mutation runs.
2. **H3** (job sum type plus crash-point enumeration). Expected payoff: two doc-held invariants become type-held.

Leave TLA+, Kani and loom out.
