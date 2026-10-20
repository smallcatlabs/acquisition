# 1c — `AGENTS.md` absorbs `CONTEXT.md`

A worker reads `roadmap/README.md`, then this brief, then what it
names, and nothing else unless the work turns it up. Runs after 1b.

## Goal
One auto-loaded file, `AGENTS.md`, under 14,000 bytes, holds what every
session must know; `CONTEXT.md` is gone; the tools read the registry
from `AGENTS.md`; nothing points at `CONTEXT.md` any more.

## Read
- `roadmap/governance/AGENTS.draft.md`: the file, verbatim but for the
  gate list, which keeps today's eight lines until 1e adds the two
  scripts.
- `roadmap/governance/DESIGN.md`, "The checks after the reset", the
  `bothy.py` paragraph.
- `tools/bothy.py` (`_read_context`, `check`, `cmd_pack`),
  `tools/bothy-breakers.sh`, `tools/docs-check.sh` sections 1 and 2.
- The pointers to redirect: `grep -rn 'CONTEXT\.md' crates tools
  .claude SURFACES.md README.md CONTRIBUTING.md docs search audits
  roadmap` (about fifty lines; the audits' and roadmap's stay as
  history, the rest change).

## Input
No measurement. The failing inputs the checks stage: `AGENTS.md` over
14,000 bytes (refused); a cross-cutting count over 15 (refused); the
draft itself at 12,693 bytes passes.

## Do
1. Write `AGENTS.md` from the draft. The cross-cutting entries are the
   ones 1b wrote into `CONTEXT.md` (identical to the draft's).
2. `git rm CONTEXT.md`; the message cites the commit before this one.
3. `tools/bothy.py`: the root file is `AGENTS.md`; the orientation
   section is "What Acquisition is"; the principles are the three
   bullets, each read for the `P<n>` ids it ends with (so a citation
   of P3 resolves to the bullet carrying it); the index table and its
   checks are deleted, areas being the files under `decisions/`; the
   docstring carries the registry grammar (from `ENTRY-RULE.md`) and
   the not-doing rule. `pack` prints from `AGENTS.md`.
   `tools/bothy-breakers.sh`: the index cases go; the fixture's root
   is `AGENTS.md`.
4. `tools/docs-check.sh`: one budget, `AGENTS.md 14000`; every other
   `budget` line deleted; the registry grep reads `AGENTS.md
   decisions/*.md`; the stale-identifier docs are `AGENTS.md
   README.md` for now (1d adds the checklist and the skills).
5. Redirect every pointer: code doc comments that say "`CONTEXT.md`
   for the cross-cutting ones" say "`AGENTS.md`"; `tools/preflight.sh`'s
   dirty list names `AGENTS.md` for `CONTEXT.md`; `SURFACES.md`,
   `README.md` (one line, "Parked" sections no longer exist: say
   "Not doing"), `CONTRIBUTING.md`'s "Issues" section (issues are
   the backlog, labelled `area:*` and `trigger:*`; what the project
   declines is a "Not doing" line; a bug report is welcome as an
   issue), the research-track and session-close skills (the
   session-close rewrite itself is 1e; here only the pointer).
   `schema.sql`'s two comments and `tracer-rung.sh`'s three cite
   dated rulings: name the ruling's id instead.
6. The gate; one commit per concern; the review.

## Done when
- `AGENTS.md` under 14,000 bytes; `CONTEXT.md` absent; `grep -rl
  'CONTEXT\.md' crates tools .claude *.md docs search` prints nothing.
- `python3 -I tools/bothy.py pack --area store` prints the orientation,
  invariants, principles and cross-cutting rulings from `AGENTS.md`.
- The gate green.

## Not in scope
- The records, the notes, `LIVE-TESTING.md`, `TESTING-NOTES.md` (1d).
- Splitting the checks, the skills' text (1e).

## Rules
As 1b's, with the review saved at `roadmap/reviews/1c-<sha>.md`.

## Report
As 1b's; the measurement is `AGENTS.md`'s bytes and the sum of what a
session loads (`AGENTS.md` plus the memory index), before and after.
