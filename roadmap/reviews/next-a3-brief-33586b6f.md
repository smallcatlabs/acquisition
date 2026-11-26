`roadmap/briefs/next-a3.md` at `33586b6f`: the implementation is clear enough to start, but the measurement and acceptance instructions need correction.

- **Decisions 4 / Report — changes what gets built.** The brief requests timings beside lane 1’s work without separating them from the isolated measurements required by C185. The Report asks for “the gate’s wall time beside lane 1’s work,” while decision 4 uses runtime to decide whether search optimization is needed. Require an idle measurement window for the depth-6 timing and treat concurrent-load observations separately.

- **Done when / Rules — changes what gets built.** The fixed-baseline scope checks can reject changes legitimately inherited by the required fast-forward from `main`. For example, A1’s committed brief requires `crates/acquisition-client/tests/harness/seeded.rs`; fetching that work would make the diff against `4e0cf51f` nonempty although A3 changed only the daemon. Record the post-fast-forward base commit and compare A3’s changes against that base.

- **Inputs, CI — wording.** The command cannot establish the CI state of the named commit or guarantee that it finds both gates. `gh run list --branch main --limit 4` selects recent runs across workflows, including the separate `Committer` workflow, and does not constrain the SHA. Query each gate workflow for the named commit, report its actual conclusion, and name the existing issue or filing obligation for any excluded failure.

- **Inputs / Find first, mutation exclusions — wording.** The claim that `.cargo/mutants.toml` names lines of `gate.rs` is false at this commit. The supplied grep returns only `examine_globs`; the exclusions name `limiter.rs`, `choke.rs`, and `kinds.rs`. Make this a conditional inspection of exclusions rather than an assertion that gate exclusions need relocation.

- **Decision 1 — wording.** “Live permits, of which there are at most two” carries a tree count without its discovery command. The declaration is `MAX_ACTUAL_SENDS = 2` in `gate.rs`; the supplied searches do not show it. Add `git grep -n MAX_ACTUAL_SENDS -- crates/acquisition-daemon/src/gate.rs`.

What I did not check: builds, tests, timing, or the gate, as instructed; live issue #206 and GitHub CI state, which are external to the committed tree. The literal read-only “Find first” commands ran, including the successful patch applicability check. I found no invariant or dependency-edge contradiction in the proposed fix, and no additional scope exclusion needed.

Form/prompt: mutable external inputs prevent a wholly commit-contained review; otherwise neither impeded the review.
