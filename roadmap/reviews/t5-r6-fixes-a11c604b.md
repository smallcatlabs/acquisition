`a11c604b^..a11c604b`: the fix-round done-when does not fully hold: completeness is enforced by one rule, but no regression test pins it.

- **[P2] [tools/mutants-compare.py:75](/Users/tom/Development/GitHub/smallcatlabs/acquisition/tools/mutants-compare.py:75), shape 1:** The commit claims the finding is pinned, but adds only implementation checks and no test exercising them. The affected input is persisted cargo-mutants artifacts: a missing shard or an otherwise consistent shard with null or absent `end_time`. Both are now refused by inspection, but removing either guard has no regression test to catch it. Add one table-driven self-test covering complete, missing, extra and unended shards, and run it in the gate or CI.

- **What I did not check:** Execution, gate status, reproduction results and byte-identical comparison output: **not verified**, because execution was prohibited and the run artifacts were unavailable.

- **Nitpicks:** None. The worker’s timeout/upload caveat preserves the refusal through the missing-shard check; the repeated display count and spelling-dependent regexes introduce no defect with the current workflow.

- **Brief:** No additional obstruction for this fix round.
- **Plan:** The requirement to pin the finding needs a committed regression test.
- **Prompt:** No change needed.
