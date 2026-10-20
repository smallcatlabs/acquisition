# 1e — The checks split and the skills trimmed

A worker reads `roadmap/README.md`, then this brief, then what it
names, and nothing else unless the work turns it up. Runs after 1d.

## Goal
The gate is the ten lines `AGENTS.md` lists: `docs-check.sh` holds the
documents, `arch-check.sh` the structure, `trade-check.sh` the sitting
tools; each breaker suite proves its own script; the session-close
skill is gate, commit and the roadmap's row; the skills are scanned
for stale identifiers.

## Read
- `roadmap/governance/DESIGN.md`, "The checks after the reset".
- `roadmap/governance/session-close.draft.md`, the file verbatim.
- `tools/docs-check.sh` sections 5, 7, 9; `tools/docs-check-breakers.sh`;
  `.github/workflows/gate-linux.yml`, the steps that run the checks and
  the breakers.
- `.claude/skills/research-track/SKILL.md` and `seat/SKILL.md`: the
  lines that cite a document this phase deleted or renamed.

## Input
No measurement. The failing inputs the breakers stage: every case
`docs-check-breakers.sh` stages today, under its new name; a skill
naming a test or tool that does not exist (refused by the
stale-identifier scan, staged by renaming one in a scratch copy).

## Do
1. `tools/arch-check.sh`: section 5 (the edges) and section 7 (the
   group reader) moved verbatim with their comments; `git mv
   tools/docs-check-breakers.sh tools/arch-check-breakers.sh`, its
   target the new script. `tools/trade-check.sh`: section 9 moved
   verbatim. `docs-check.sh`'s header comment renumbered to what it
   still holds.
2. `AGENTS.md`'s gate list gains the two lines; `gate-linux.yml` runs
   both scripts and the renamed breakers.
3. The session-close skill from its draft. The research-track and seat
   skills: pointers only.
4. The gate; one commit per concern; the review.

## Done when
- The three scripts exist, each exits 0 on the tree, and the breakers
  exit 0.
- `AGENTS.md` lists ten gate lines and stays under 14,000 bytes.
- The workflow runs the three scripts and both breaker suites.
- The gate green.

## Not in scope
- Any new check beyond the stale-identifier scan over the skills.
- The seat and research-track procedures themselves.

## Rules
As 1b's, with the review saved at `roadmap/reviews/1e-<sha>.md`.

## Report
As 1b's; the measurement is the gate's wall time for the three scripts.
