# The governance audit of 2026-11-09

The record of an audit of how this project is governed (its rules, its
process, its documents), and of what was proposed and ruled from it.
The rulings it led to are `decisions/process.md`, C170 to C185; this
directory is the evidence they rest on.

**The reports are whole at `8e04545a`.** The rebuild's close (#229)
deleted them and kept this record and the sitting's (`seat/`); its
`HISTORY.md` line cites that commit.

## The question

The owner, three weeks after the governance reset of 2026-10-20
(`0704a8f7`): "I think it's time for us to take a step back from the
code and look at the governance system around it. I suspect it has
accumulated some cruft, and what served this project three weeks ago
may not be what serves it today."

## How it was done

Opus investigators, read-only, each given one slice and told to name
the command behind every number and to mark judgment as judgment. A
Fable session briefed them, re-ran the headline numbers, and wrote the
synthesis. The files were their reports as returned, with one contact
address and three local paths removed; all but `seat/` are read at
`8e04545a`.

| File | What it read |
| --- | --- |
| `1-registry-and-checkers.md` | the rulings and the tools that lint them |
| `2-step-machine.md` | briefs, cold reviews, the ledger, the road file |
| `3-ledgers-and-homes.md` | every other home a fact has, against "one home per fact" |
| `4-history-and-issues.md` | what git history and the issue tracker show |
| `6-cold-start-walk-baseline.md` | a fresh agent following the instructions on four tasks: the baseline for #228 |
| `7-invariant-holders.md` | what would fail if each of the five invariants broke |
| `8-owner-read-harvest.md` | every promise that the owner would read something later |
| `9-queue-drafts.md` | the drafts of the owner's queue, #221 to #225 |
| `seat/` | #228's sitting: how it runs, its prompt, and the baseline's prompt |

Two reports are not here: 5, on the agent-side apparatus, and 10, a
sort of an agent's private memory against the tree. Both describe that
memory file by file, and it is private. What they concluded is in C183
(where a rule lives) and #229's close; the twelve memory files that
close deleted are `audits/governance-2026-11/memory/` at `9122bd0d`.

## What it found

Measured on the tree at `73e7e77c`, over the commits since 2026-10-19:

- 1,603 commits, of which 793 touched only governance documents, 685
  only `roadmap/`, and 673 touched `crates/`
  (`git log --since=2026-10-19 --no-merges --format='@%h' --name-only`,
  each commit classed by its paths; report 4 counts 1,670 over a
  slightly different window).
- `roadmap/README.md`, read first for every step: 6,018 bytes at its
  first commit, 59,270 at `73e7e77c`, 25,972 of them a "Current step"
  whose last sentence was the only current one (`wc -c`; an `awk`
  split on its headings).
- The registry: 120 entries at the reset, 169 now, 25 over the
  800-byte limit, none ever retired (`tools/docs-check.sh`).
- `roadmap/` held 4,147,337 tracked bytes against 5,442,579 of
  production source (`git ls-files -z <path> | xargs -0 cat | wc -c`).
- Cold reviews of builds earned their keep: 97 saved reviews carried 6
  findings marked P1 and 112 marked P2 (report 2's count of the
  marks). Reviews of briefs paid mostly in their first two rounds.

Four causes: files that mixed law, state and log, so appending buried
the rest; records written where they could be derived, each a claim
that could go wrong; nothing in the governance ever retiring; and no
one home for who may decide what.

## What was proposed and what the owner ruled

| Proposed | Ruled, and where it landed |
| --- | --- |
| The road file holds state only | "yes, drop the history." C180; `8bd924bb` |
| Process rulings as an area of the registry | "yes, add a process ruling area." `decisions/process.md` |
| A standing grant to commit and push | "yes, the push grant and commit to main on green are ok." C171 |
| The close commit is the ledger | C178; `roadmap/LEDGER.md` takes no new row |
| Briefs carry rules and commands | C174; `roadmap/briefs/TEMPLATE.md` |
| Two reviews of a brief at most | Changed by the owner: "I think we should allow at least three". C176 |
| An entry's shape in place of its bytes | Accepted; built by #226 |
| A commit hook that refuses, not a rule to remember | C184; built by #226 |
| Name what holds each invariant | `AGENTS.md`, "Invariants"; the gaps are #216 to #219 |
| One queue for decisions that wait on the owner | C179; #221 to #225 |
| Who plans and who designs | Redrafted at his word, 2026-11-09. C172, C185; `70a6d65c` |

The documents were reviewed cold by Codex
(`roadmap/reviews/governance-docs-8bd924bb.md`) and answered in
`793555eb`.

## What the audit got wrong, and corrected

- Report 3 calls the 105 saved review prompts "verbatim copies" of the
  standing prompt. Twenty are; 85 were written for one review each
  (`grep -l 'You are the outside reviewer of one committed step' roadmap/reviews/*-prompt.md | wc -l`).
- The session told the owner that no code cites an invariant by
  number. Its pattern used `\b`, which `git grep -E` on macOS does not
  support; 98 lines do. Report 7 opens by saying so.
- The session told the owner that reviewer independence had held. By
  the first lines of all 283 saved reviews it held through 2026-11-07;
  the nine reviews saved on 2026-11-08 and 11-09 were run by a model
  of the author's family. C177 answers it.
- Report 2 counts 17 of 37 briefs reviewed three times or more; by the
  review files' names it is 15 of 31. Report 1 says 15 of 25 long
  entries would fit with a short `Pinned:`; with it held to 120 bytes
  it is 8 of 26.
- The investigators' briefs dated the reset 2026-10-19 and counted 19
  cross-cutting rulings; it is 2026-10-20 and 18, as report 1 notes.
