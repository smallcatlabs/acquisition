# Review of `dc993ddb` (2d design), Codex, 2026-10-21

Launched by the Fable session with `roadmap/briefs/REVIEW-PROMPT.md`; the final block verbatim.

dc993ddb — documents-only design delivered; no design “done when” is defined, and the build brief’s criteria are unmet; execution-dependent checks are **not verified**.

Findings, most severe first:

- **[P1] roadmap/briefs/2d-build.md:281 — Shape 1:** The checker can accept behavioural changes while claiming statement preservation. Changing `"token-request-limit"` to `"token- request-limit"` in `bucket_for_policy` becomes invisible after whitespace removal, although OAuth pacing changes; swapping the `RateLimited` and `Network` returns in `classify_response` also preserves the statement multiset. Smallest sufficient fix: compare ordered token streams per moved item, preserving literal contents, and add these negative cases.

- **[P1] roadmap/briefs/2d-build.md:282 — Shape 2:** The checker rejects structural changes required by the prescribed split. Moving methods from the existing `impl Daemon` into multiple children necessarily adds `implDaemon{` and closing-brace units, which this checker counts; removing inline test-module wrappers similarly changes brace counts. Smallest fix: explicitly account for permitted module and impl wrappers, then validate the checker against an actual proposed split rather than dedenting alone.

- **[P1] roadmap/briefs/2d-design.md:414 — Shape 6:** The prescribed replacement pins do not resolve through the existing checker. For example, `daemon/tests/idle.rs` will live under `crates/acquisition-daemon/src/`, but `tools/bothy.py:269` searches repository and crate roots without adding `src/`; the same problem affects `ratelimit/spec.rs` and the other proposed paths. Smallest fix: consistently use paths such as `acquisition-daemon/src/daemon/tests/idle.rs` throughout both briefs.

- **[P2] roadmap/briefs/2d-build.md:157 — Shape 1:** The whole-directory file-size criterion conflicts with the requirement to leave other production files byte-identical. `mockggg.rs` already has 1,236 lines and `rails.rs` 1,083, including their inline tests, and both are explicitly outside scope. Smallest fix: limit the size criterion to files produced by this split.

- **[P2] roadmap/briefs/2d-design.md:343 — Shapes 1 and 3:** The prescribed replacement coverage sentence overstates what the scenario helpers check. The rejected-bearer helpers call two oracles and never `assert_journal_matches_wire`, whereas this sentence says “the three oracles” run through ten more scenarios; section 1 correctly says those scenarios run “one or two.” Smallest fix: retain that qualification in the replacement sentence and its build instruction.

What I did not check and why: I ran no builds, tests, checker scripts, daemons, or Windows commands, as requested; their results and the commit’s reported checks are **not verified**. No Rust code, goldens, or generated references changed, so this commit introduces no runtime regression directly.

Nitpicks: none; these affect the proposed verification or completion criteria.

Brief: distinguish the design’s acceptance criteria from the future build’s criteria. Plan: keep the move separate from the already identified behavioural fixes. Prompt: the prohibition on execution was clear and did not hinder static review.
tokens used
103,789
dc993ddb — documents-only design delivered; no design “done when” is defined, and the build brief’s criteria are unmet; execution-dependent checks are **not verified**.

Findings, most severe first:

- **[P1] roadmap/briefs/2d-build.md:281 — Shape 1:** The checker can accept behavioural changes while claiming statement preservation. Changing `"token-request-limit"` to `"token- request-limit"` in `bucket_for_policy` becomes invisible after whitespace removal, although OAuth pacing changes; swapping the `RateLimited` and `Network` returns in `classify_response` also preserves the statement multiset. Smallest sufficient fix: compare ordered token streams per moved item, preserving literal contents, and add these negative cases.

- **[P1] roadmap/briefs/2d-build.md:282 — Shape 2:** The checker rejects structural changes required by the prescribed split. Moving methods from the existing `impl Daemon` into multiple children necessarily adds `implDaemon{` and closing-brace units, which this checker counts; removing inline test-module wrappers similarly changes brace counts. Smallest fix: explicitly account for permitted module and impl wrappers, then validate the checker against an actual proposed split rather than dedenting alone.

- **[P1] roadmap/briefs/2d-design.md:414 — Shape 6:** The prescribed replacement pins do not resolve through the existing checker. For example, `daemon/tests/idle.rs` will live under `crates/acquisition-daemon/src/`, but `tools/bothy.py:269` searches repository and crate roots without adding `src/`; the same problem affects `ratelimit/spec.rs` and the other proposed paths. Smallest fix: consistently use paths such as `acquisition-daemon/src/daemon/tests/idle.rs` throughout both briefs.

- **[P2] roadmap/briefs/2d-build.md:157 — Shape 1:** The whole-directory file-size criterion conflicts with the requirement to leave other production files byte-identical. `mockggg.rs` already has 1,236 lines and `rails.rs` 1,083, including their inline tests, and both are explicitly outside scope. Smallest fix: limit the size criterion to files produced by this split.

- **[P2] roadmap/briefs/2d-design.md:343 — Shapes 1 and 3:** The prescribed replacement coverage sentence overstates what the scenario helpers check. The rejected-bearer helpers call two oracles and never `assert_journal_matches_wire`, whereas this sentence says “the three oracles” run through ten more scenarios; section 1 correctly says those scenarios run “one or two.” Smallest fix: retain that qualification in the replacement sentence and its build instruction.

What I did not check and why: I ran no builds, tests, checker scripts, daemons, or Windows commands, as requested; their results and the commit’s reported checks are **not verified**. No Rust code, goldens, or generated references changed, so this commit introduces no runtime regression directly.

Nitpicks: none; these affect the proposed verification or completion criteria.

Brief: distinguish the design’s acceptance criteria from the future build’s criteria. Plan: keep the move separate from the already identified behavioural fixes. Prompt: the prohibition on execution was clear and did not hinder static review.
