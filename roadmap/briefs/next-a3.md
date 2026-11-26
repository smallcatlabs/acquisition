# next-a3 — the gate's fault fixed, its search kept as a test

Step A3 of `audits/whole-review-2026-11/NEXT.md`, cut from its "A3.
The gate's fault, and its search", from
`audits/whole-review-2026-11/findings/admission.md` (its move 1 and
move 3, the gate's half) and from the trial beside it,
`trials/admission-explorer.md` with its `.patch`. Written by the
orchestrating session on 2026-11-25 against `4e0cf51f`. It closes
#206. No new ruling: the step strengthens invariant 1 (every send
passes one rate-limit choke point and one send-lifetime gate) and
changes no boundary.

## Goal

The send gate can no longer grant a second send on a policy that has
one in flight, in a release build as in a debug one, and that is held
by a search over every sequence of the gate's events to a depth, not
by scenarios someone thought of. Today the gate keeps a second record
of which policies are busy beside its live permits, kept in step by
assertions compiled out of the shipped daemon; when a route learns its
policy's name while that policy is already live, the record goes stale
and a waiting send is let out beside a live one. Two sends in flight
on one policy is what the gate exists to prevent (ground truth N6:
same-name policies share counters).

## Inputs

- Invariant 1, and C21 (the burst bound spans each send from dispatch
  until its body completes; one live send per policy, a HEAD
  exclusive), in `decisions/network.md` and the module doc of
  `crates/acquisition-daemon/src/gate.rs`.
- C47 (a panic is for a broken internal invariant only).
- C200 (an excluded mutant's named line must still hold its pattern):
  `.cargo/mutants.toml` names lines of `gate.rs`.
- #206 (`gh issue view 206`): the hazard as first filed.
- `trials/admission-explorer.md`: what the search explores, its
  oracle, the sequence it found, its cost by depth.
- `trials/admission-explorer.patch`: the trial's module. Take its
  gate half as a start; it prints what it finds and passes.
- CI at `4e0cf51f`: read `gh run list --branch main --limit 4` and
  name a red run in the report; it is not this step's to fix.

## Decisions

1. **Derive, never record.** "This policy is busy" is read from the
   live permits, of which there are at most two, and the second record
   is deleted with the three assertions that kept it in step. Rules
   out #206's first proposal, promoting the assertions to a refused
   rekey or an abort: each adds an outcome the choke point must
   handle, for a state the derivation cannot reach.
2. **What a rekey may leave.** Two permits granted under different
   keys can come to name one policy when a route learns its name.
   Those sends are already out and the gate cannot recall them. The
   rule it holds from then: no new permit on that policy until every
   live one naming it is dropped. The module doc says this in a
   sentence, under C21.
3. **The oracle is not the gate.** The search keeps its own model of
   which task holds which key and never reads the gate's state. It
   holds the trial's five properties; the one #206 breaks is that a
   newly granted permit never shares its key with a live one. A rekey
   that joins two live permits is not itself a failure.
4. **A test that fails.** It asserts, and its message is the shortest
   failing sequence. It runs to depth 6 in the gate's own profile; an
   environment variable may deepen it by hand, never shallow it. If
   depth 6 costs more than nextest's slow period there
   (`.config/nextest.toml`), the lever is the search's own (states
   seen before, the symmetry of the keys), never a smaller depth;
   raise it in the report if neither is small.
5. **Its home** follows the limiter's: a file of its own included by
   `#[path]`, as `ratelimit/tests/spec.rs` is, so `gate.rs` stays the
   primitive and its scenario tests.
6. **The gate only.** The trial's limiter search does not land: what
   it finds waits on the owner (a test pins today's behaviour as
   intended).

## Find first

- `grep -n 'active_policies\|debug_assert' crates/acquisition-daemon/src/gate.rs`:
  the record and its assertions.
- `git grep -n 'rekey_policy\|\.occupancy()' -- crates/acquisition-daemon/src`:
  the callers, and whether any reads what the record held.
- `grep -n -B3 -A8 'gate\.rs' .cargo/mutants.toml`, and check 16 in
  `tools/arch-check.sh`: each exclusion whose line moves.
- `git apply --check audits/whole-review-2026-11/trials/admission-explorer.patch`:
  whether the trial still applies.
- `git grep -n -e '#206' -e 'active_policies' -- decisions docs REVIEW-CHECKLIST.md crates`:
  each text that cites the issue or the record.
- With the search applied and the fix not: run it in the dev profile
  and in release, and keep the sequence each prints.

## Done when

- `grep -c 'debug_assert' crates/acquisition-daemon/src/gate.rs`
  prints 1, and `grep -c 'active_policies' crates/acquisition-daemon/src/gate.rs`
  prints 0.
- One test explores every sequence of the trial's six events over
  three tasks and three keys to depth 6 against the oracle of decision
  3, and runs in the gate with no variable set.
- The fix's commit message holds the shortest sequence the search
  printed on the tree without the fix, in the dev profile and in
  release, and the seconds depth 6 took with the fix in the gate's
  profile. The test and the fix are one commit (a test rides with the
  fix it pins), and its message says `Closes #206`.
- `git diff --stat 4e0cf51f -- crates ':!crates/acquisition-daemon'`
  prints nothing, and so does
  `git diff 4e0cf51f -- crates/acquisition-daemon/src/ratelimit/tests`.
  The scenario tests in `gate.rs` change only where one names the
  deleted record.
- `tools/gate.sh` exits 0 on the step's tree, in the lane.

## Not in scope

- The limiter's missing hit after an answer naming another policy, and
  the trial's limiter search: the owner's question.
- `findings/admission.md`'s move 2, one decision function for the
  admission loop, and anything in `choke.rs` or the rails.
- The drafted issues under `audits/whole-review-2026-11/issues/`.
- Driving the choke point from the search. The claim is the gate's
  alone.

## Rules

`TEMPLATE.md`. What differs:

- You build in the lane, the clone you made, and never run cargo, the
  gate or `acq` in the main tree. Before the first edit:
  `git fetch /Users/tom/Development/GitHub/smallcatlabs/acquisition main`
  then `git merge --ff-only FETCH_HEAD`, in the lane.
- Commit in the lane. Never push from it: the orchestrator fetches
  your commits into the main tree and pushes. Your report names each
  commit by its full hash.
- Order no cold review: the orchestrator runs it and sends you what to
  answer.
- No change to `decisions/`, `AGENTS.md` or `roadmap/README.md`. Leave
  the lane in place when you end your turn.

## Report

`TEMPLATE.md`, and with it: what the lane cost this step (the gate's
wall time beside lane 1's work, anything that failed under load).
