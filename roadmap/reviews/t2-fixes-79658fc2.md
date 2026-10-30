45cb2946^..79658fc2 — the fix round’s done-when is substantially met: executable verification is restored and the timing claim is narrowed, but one sibling claim remains incorrect.

1. **[P3] `tools/gate.sh:33` — shapes 1, 3.** The restored header says `features` holds “the same property” as executable verification, repeating the equivalence this round rejects. Concrete build-configuration input: `CARGO_PROFILE_TEST_OPT_LEVEL=0` changes test-built executables against this tree’s dev opt-level of 1 while leaving feature graphs equal; neither client, persisted nor upstream input is involved. Smallest fix: say `features` checks feature-graph equality, while `artifacts` checks executable equality, including differences caused by profiles.

Execution—gate and breaker exits, test counts, timings, and CI—is **not verified**, because this review was read-only and ran no checks.

Finding 1 is a nitpick: the restored artifact check protects the functional boundary despite its inaccurate explanation. No other defects found in the changed code and affected callers.

Brief: its artifact-retirement premise proved insufficient; retaining the check is a justified correction. Plan: no further change suggested. Prompt: no obstruction.
