# Review of the roadmap commit a987d02f (and 9ec14155)

Codex (gpt-6.1-sol, high reasoning), launched 2026-10-19 from the agent shell with `codex exec --sandbox read-only` and the prompt at `roadmap/briefs/REVIEW-PROMPT.md` extended with six questions about the plan itself. 28 read-only commands, 192,173 tokens. The findings are verbatim below; what was done with each is in the ledger row of the commit that cites this file.

The direction is sound, but the plan needs several execution and evidence fixes before its briefs are ready to run.

Findings, most severe first:

1. **[0b-policy-merge.md:23](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/0b-policy-merge.md:23) — The brief delegates a public intent-write rule to the worker and postpones owner approval until after implementation.**
   Evidence: “The rule is the worker’s to choose … the owner may veto it at review” contradicts CONTEXT’s “rulings first” requirement and TEMPLATE’s requirement to obtain missing rulings before code. Accepting per-realm merging does not settle empty-realm deletion or the replacement flag. C52 also requires the MCP’s create-only/CAS distinction to survive.
   **Change:** Make an owner-approved, recorded merge/delete/replacement rule the first prerequisite, including preservation of C52.

2. **[REVIEW-PROMPT.md:6](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/REVIEW-PROMPT.md:6) — The prompt prioritizes editorial fault shapes over finding ordinary Rust defects.**
   Evidence: correctness appears only after eight shapes dominated by prose, presentation and evidence handling. The project’s own NETWORK-CLEANUP findings include deadline overflow and cancellation stranding refresh waiters—precisely the bugs this list does little to elicit. Requiring “one rule … never a patch per case” also pressures reviewers to generalize faults unnecessarily.
   **Change:** Lead with: “Review the diff and affected callers for behavioral regressions, arithmetic and error-path failures, cancellation, concurrency, persistence and invariant violations; then apply the relevant area checklist and these supplementary shapes. Recommend the smallest sufficient fix.” Permit “not verified” for acceptance conditions requiring execution.

3. **[roadmap/README.md:50](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/README.md:50), at `9ec14155` — The prescribed review command uses an unsupported combination of commit selection and custom review instructions.**
   Evidence: the committed command passes both `--commit` and a prompt to `exec review`; the uncommitted replacement explicitly identifies that incompatibility and instead uses ordinary `codex exec` with the SHA inside the prompt. The required review step therefore lacks a usable committed launcher.
   **Change:** Commit the corrected launcher and its matching SHA-reading instructions. This finding is already addressed in the working tree, but not in either reviewed commit.

4. **[roadmap/README.md:70](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/README.md:70) — The November 20 data release depends on external inputs that the schedule neither names nor accommodates.**
   Evidence: report 6 allows RePoE to lag launch by thirteen days, putting availability as late as November 26. Its checklist also requires new owner-captured trade exports, pin/manifest updates and generator checks; “regenerate against the post-launch export” does not supply those governed inputs. C79 prevents workers from obtaining the trade captures themselves.
   **Change:** Separate the November 13 launch rehearsal/live-use milestone from an `alpha.2` milestone gated on named export pins, owner captures and regeneration checks, with the release date conditional on their arrival.

5. **[roadmap/README.md:67](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/README.md:67) — Parallel phase-0 workers can invalidate the measurements that determine whether 0a and 0e succeed.**
   Evidence: the M3 procedure explicitly requires running alone and records contention adding approximately 10–20 ms. Separate worktrees isolate artifacts, but do not isolate CPU, memory or disk from another worker’s build and gate. Those errors are material beside the cited 1–28 ms budget margin.
   **Change:** Reserve exclusive measurement windows for 0a and 0e, pausing other builds, gates and measurements during them.

6. **[0a-store-age.md:46](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/0a-store-age.md:46) — “Done when” permits completion without achieving the stated performance goal.**
   Evidence: it requires three changes and measurement rows, but no acceptable result. Moreover, replacing the nested lookup does not make cost independent of store age: `store/src/corpus.rs:419` still reads and sorts every removed identifier, and `search/src/corpus.rs:725` still constructs the corresponding map. The proposed work removes a multiplicative cost, not all historical-data cost.
   **Change:** Replace the absolute goal with an agreed, measurable slowdown bound on the removed-heavy fixture, and require that bound for completion.

7. **[0a-store-age.md:23](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/0a-store-age.md:23) — The removed-heavy benchmark is described as an input without actually fixing its construction.**
   Evidence: the brief specifies “tens of thousands” of synthetic rows; report 5 names scratch `w6`, 90,914 rows and temporary scripts, but supplies no reproducible generator or manifest. Row distribution and location relationships affect both the loop and index measurements.
   **Change:** Require one deterministic fixture-construction script and a manifest recording source checksum, live/removed counts and construction parameters before collecting the baseline.

8. **[parks-triage.md:129](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/parks-triage.md:129) — Retention is incorrectly classified as waiting for 3.30 although its search trigger has already fired.**
   Evidence: ST-3’s trigger includes “search reading removed”; shipped search supports `--membership all|removed`, `Corpus::load_of` reads those bodies, and the M3 script exercises that path. This changes the claimed five-fired total and identifies a current design obligation missed by the assessment.
   **Change:** Mark ST-3 fired on that existing evidence and assign an explicit retention decision or deliberate deferral in the roadmap.

9. **[roadmap/README.md:68](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/README.md:68) — Issue migration silently replaces several triage recommendations to build work, without recording a scheduling decision.**
   Evidence: the triage’s first recommendation is fired read-economy work, followed by the keeper bundle; it also says to build the fired stat, other-reading and sort work. None receives an implementation step or an explicit deferral in the phase table. Brief 0c specifically routes read economy into migration and an unspecified “later” ruling. Moving an item into GitHub does not execute its recorded fate.
   **Change:** Give each “build first/early/now” recommendation a scheduled step or an explicit owner-approved deferral at the phase-1 boundary.

10. **[STATE-2026-10-19.md:16](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/STATE-2026-10-19.md:16) — The synthesis turns qualified observations into broader assurance and causal claims.**
    Evidence: report 1 says “almost none” for dead code and identifies sampled/impression-based readings; the synthesis says “no dead code” and “invariants held.” Report 3 counts subjects mentioning governance work; the synthesis says those commits existed to satisfy checkers. “Nothing has been retired” is directly contradicted by TESTING-NOTES’ retirement of the frozen-soak checker and procedure. Likewise, the tracker trigger is called effectively met while CTX-3 remains dormant in the triage.
    **Change:** Rewrite the synthesis’s assurance and governance claims with their original qualifications and distinguish measurements, judgments and newly accepted choices.

11. **[0a-store-age.md:15](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/0a-store-age.md:15) and [0b-policy-merge.md:11](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/0b-policy-merge.md:11) — Two mandatory-reading citations misidentify their rulings.**
    Evidence: C48 lives in CONTEXT, not `decisions/store.md`; C37 rules tab-and-child coverage, rather than planner ownership of the policy schema.
    **Change:** Correct the file target and cite the appropriate planner/schema rulings with accurate glosses.

What I did not check and why:

- I did not build, run binaries, execute checks or reproduce timings, as requested.
- I read both commits, all seven reports, the triage, template and briefs, and cross-checked standing documents and relevant source paths; this was not an exhaustive correctness or security audit of every Rust module.
- I did not independently verify original audit transcripts, discarded measurement scripts, live-store contents, upstream exports or endpoint availability. I used the game-release dates supplied.
- I excluded the uncommitted roadmap, prompt and template edits from the verdict on the commits.

Which findings are nitpicks:

- **11 only.** The others affect authorization, review effectiveness, acceptance, evidence or scheduling.