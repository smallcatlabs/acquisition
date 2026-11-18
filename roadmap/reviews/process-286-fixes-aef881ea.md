aef881ea — the fix-round done-when does not hold: restoration regresses for supported persisted policies, and the fixes lack committed regression pins.

- **P1 — tools/tracer-rung.sh:344 — shape 7:** The predicted post-write policy retains the persisted version, whereas the actual merge upgrades it to v3, so cleanup rejects its own write and leaves the temporary selection stored. Concrete persisted input: `{"version":2,"realms":{"pc":{"leagues":{"Standard":{"tabs":"all","max_age_seconds":3600}}}}}` followed by a run with `--tabs t1`; `RUN_POLICY` remains version 2, the stored value becomes version 3, and line 168 refuses restoration without any concurrent writer. Construct the prediction using the writer’s version-conversion semantics, or refuse older versions before mutation, and pin this case.

- **P2 — tools/tracer-rung.sh:153, tools/tracer-rung.sh:530:** None of the four fixes has a committed regression test, despite the fix-round requirement that each finding be pinned. The interrupted write, failed restoration, account-qualified recovery command and failed apply are described as scratch reproductions only; the ordinary mock rehearsal does not stage those failures. Retain those cases in a repeatable mock regression check that fails when each fix is broken.

- **P3 — tools/tracer-rung.sh:179 — shape 1:** A failed restoration still claims the stored policy “is still this run’s selection” even when another client caused the failure. Concrete client input: another client writes a policy after cleanup’s read but before its CAS; restoration correctly refuses, but the sentence is false and the printed command targets an obsolete revision. Say restoration was refused and direct the user to inspect the current policy; retain the revision guard.

Execution—gate, verifier self-test, both mock variants and terminal behavior—**not verified**; this was a source-only review as requested.

Nitpicks: the P3 wording finding, because it requires a narrow concurrent-write window. The persisted-policy regression is not a nitpick.

Brief: no changes suggested. Plan: none supplied separately. Prompt: no changes suggested.
