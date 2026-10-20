# 1d — The records to one checklist and a history; the notes and `LIVE-TESTING.md` out

A worker reads `roadmap/README.md`, then this brief, then what it
names, and nothing else unless the work turns it up. Runs after 1c.

## Goal
The eight closed records, the fifteen notes and `LIVE-TESTING.md` are
gone from the working tree; what reviews needed from the records is
`REVIEW-CHECKLIST.md`, what history needs is one line each in
`HISTORY.md`, what live runs need is the live-run skill, and
`TESTING-NOTES.md` is its purpose and its shapes. No path in the tree
points at a deleted file.

## Read
- `roadmap/governance/REVIEW-CHECKLIST.draft.md`,
  `HISTORY.draft.md`, `TESTING-NOTES.draft.md`, `live-run.draft.md`:
  the files, verbatim but for the placeholders (`HISTORY_LT`,
  `HISTORY_TN`, `GOVERNANCE_AT`, `TESTING_NOTES_AT`), each the commit
  before the deleting commit, filled in at the commit.
- `roadmap/governance/DESIGN.md`, "Rulings asked of the owner" item 5
  (the two code texts) and the deletion table.
- The pointers to redirect, found by grep over `crates tools .claude
  .github *.md *.toml docs search` for each deleted file's name and
  for `brainstorming-notes`; `tools/docs-check.sh`'s scan lists
  (sections 2, 2b, 6).
- `.claude/skills/live-run/SKILL.md` as it stands, to see what the
  draft kept.

## Input
No measurement. The failing input: a backticked path to a file this
step deletes, anywhere the stale-identifier scan reads (refused by
`docs-check.sh` once `REVIEW-CHECKLIST.md` and the skills join the
scan, which this step does).

## Do
1. Write `REVIEW-CHECKLIST.md` and `HISTORY.md` from the drafts;
   check every test and mechanism name in the checklist exists (`grep
   -rn` each backticked name over `crates tools .claude`) before the
   commit, and correct any the drafts got wrong, saying so in the
   report.
2. Write `TESTING-NOTES.md` and the live-run skill from the drafts.
3. `git rm` the eight records, `LIVE-TESTING.md` and
   `brainstorming-notes/`; the message cites the commit before this one
   as holding every one of them whole.
4. Redirect every pointer in one scripted pass over the crates, the
   tools, the workflows, `about.toml`, `deny.toml`, `search/`,
   `docs/design/`, `SURFACES.md`, `README.md` and the skills: a
   citation of a deleted file by path becomes `<path>` at `<commit>`;
   a citation of a note becomes `note NN at <commit>`; a pointer to
   `LIVE-TESTING.md`'s rule or rails points at the live-run skill or
   `rails.rs`. Comments only; no logic changes.
5. The two code texts: the `reset-tripwire` help in
   `acquisition-cli/src/main.rs` and the protocol's doc say the rule
   in words ("after a trip, wait 360 s before clearing; a trip is
   evidence to read"), and the references regenerate
   (`ACQ_UPDATE_FIXTURES=1`); `tools/preflight.sh`'s dirty list names
   `.claude/skills/live-run` for `LIVE-TESTING.md`.
6. `tools/docs-check.sh`: the scan lists drop the records,
   `LIVE-TESTING.md` and `CONTEXT.md` and add `REVIEW-CHECKLIST.md`,
   `HISTORY.md` and `roadmap`; the stale-identifier docs add
   `REVIEW-CHECKLIST.md` and `.claude/skills/*/SKILL.md`; section 6
   (the notes) is deleted with its comment.
7. The gate; one commit per concern (the new files; the deletions and
   redirects; the help text); the review.

## Done when
- `ls *.md` shows no record, no `LIVE-TESTING.md`; `brainstorming-notes/`
  absent.
- `grep -rln 'SLICE\.md\|NETWORK-CLEANUP\.md\|LIVE-TESTING\.md\|brainstorming-notes/' crates tools .claude .github *.md *.toml docs search`
  prints only lines in the history form (`at \`<hash>\``).
- `CLI-REFERENCE.md` regenerated and committed.
- The gate green.

## Not in scope
- Trimming the "Decisions as recorded" copies in code (phase 2).
- The checks split, the session-close skill (1e).
- `search/README.md`'s own content beyond its pointers.

## Rules
As 1b's, with the review saved at `roadmap/reviews/1d-<sha>.md`.

## Report
As 1b's; the measurement is root markdown bytes before and after, and
the count of redirected lines.
