8bd924bb — the specified properties do not all hold.

- **P2 — `decisions/process.md:14` — shape 3:** The move drops the standing rule that measurements run alone on the machine. Previously, the roadmap’s Roles section prohibited another build, gate or measurement during a measurement; the replacement prohibits them only during the owner’s sitting. A benchmark alongside a gate is now permitted. Restore the measurement isolation sentence here.

- **P2 — `.claude/skills/cold-review/SKILL.md:14` — shape 1:** The review census mixes separate artifacts and counts saved prompts as reviews. For example, the `4-full-1-cards-` prefix matches three brief reviews and one build review, although the build has had only one review. Filter separately for brief versus build/fix reviews and exclude `-prompt.md` files.

- **P2 — `.claude/skills/cold-review/SKILL.md:67` — shape 3:** The unconditional exemption for document-only rounds contradicts the registry’s exception for cuts or moves across many files. Following this skill would exempt this very commit, although `decisions/process.md:23` requires its review. Refer to that registry rule instead of restating an incomplete exemption.

- **P3 — `decisions/process.md:18` — shape 1:** Two entries describe future safeguards as existing pins: the roadmap cap here and the assistant-trailer refusal on line 19. At this commit, `tools/docs-check.sh` budgets only `AGENTS.md`, and `.githooks/commit-msg` only rewrites trailers; neither rejects the stated failing input. The shared cause is describing the sibling tools step as already built. Mark these clauses **Unbuilt**, name tracking issues as the entry rule requires, and correct the corresponding claims in `AGENTS.md` and session-close.

- **P3 — `roadmap/briefs/governance-tools-1.md:64`:** The first census instructions provide no command, contrary to the new registry rule that a brief supplies the commands its builder runs first. The builder must invent the command that produces the baseline and enumerates oversized pin fields. Supply executable read-only commands for both censuses.

- **Not checked:** Execution evidence, CI results and GitHub issue contents are **not verified**; this review used the committed tree and parent without running builds, tests or checks.

- **Nitpicks:** None depends on an unlikely input; the two P3 findings concern wording and brief completeness.

- **Prompt limitation:** The commit quotes blanket approval of an unquoted proposal, which prevents independently matching every new process clause to the precise proposal the owner approved. Including that proposal’s committed location would make property 3 fully auditable.
