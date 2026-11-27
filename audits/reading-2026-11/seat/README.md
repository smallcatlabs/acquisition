# The instructions sitting, before the changes: the record

The run of 2026-11-26, as `audits/governance-2026-11/seat/BRIEF.md`
says a sitting at the instructions runs. It is the before-run for the
changes `audits/reading-2026-11/README.md` proposes: nothing of those
changes was made when it ran. The prompt is `PROMPT.md` beside this
file, sent as it stands. The report is `REPORT.md` (`result.json`'s
`result`, whole). The call log is `calls.csv`.

## The run

- **Tree:** a clone of `main` at `b2cd41a8` in
  `~/seat-worlds/reading-2026-11/before/tree`, its `origin` set to
  GitHub's. The clone's status was clean after the run.
- **Runner:** one headless `claude -p --model opus` (Claude Code
  2.1.331), in the background, under the brief's rails, flag for flag.
  45 turns and 238,350 ms, by `result.json`. Its session is
  `2ab9028f-6233-454e-bedb-60f1da939498`.
- **Calls:** 44, as `calls.csv` counts them: 29 Bash and 15 Read.
- **The prompt** is the earlier sitting's with one change: the ground
  rule that forbids `audits/governance-2026-11/` also forbids
  `audits/reading-2026-11/`, which holds the audit this run is the
  baseline for (`diff` the two files).

## The checks made before believing the report

- **No memory.** Each note name in the project's memory directory was
  tested against the transcript with `grep -c`. One occurs,
  `orchestrating-steps`, three times: as line 399 of
  `audits/whole-review-2026-11/NEXT.md`, which the runner read, and in
  its own report of that line as a stale pointer. The `MEMORY.md` text
  is the prompt's ground rule and the harness's own note.
  `AGENTS.md`'s first heading occurs.
- **Every path read is inside the clone.** All 15 `Read` calls are
  under `…/before/tree/`. No call names either forbidden directory.
  The report logs one pointer there (`GLOSSARY.md:133`) and says it
  did not follow it.
- **The denials, three of them** (`permission_denials`):
  - call 1, a `wc -c` chain that named `~/.claude/CLAUDE.md`;
  - call 13, `wc -c tools/bothy.py; python3 -I -c …`, a command
    outside the rails;
  - call 42, a `grep` and `sed -n` chain over `tools/gate.sh`, `sed`
    being outside the rails.

  The runner read the docstring from the file (call 14). No task was
  left unanswered.
- **Byte counts.** 14 of the counts the report names were recomputed
  with `wc -c` on the clone, and all 14 match: `AGENTS.md` 20,942 and
  `CLAUDE.md` 11; `decisions/process.md` 15,463; `roadmap/README.md`
  5,187; session-close 4,655 and cold-review 4,243; `GLOSSARY.md`
  8,136; `decisions/store.md` 15,586, `decisions/frontends.md` 20,145
  and `decisions/daemon.md` 30,753; `roadmap/briefs/TEMPLATE.md`
  5,274; `audits/whole-review-2026-11/NEXT.md` 27,064;
  `audits/meta-2026-11/README.md` 31,447; `tools/bothy.py` 42,224;
  `pack --path crates/acquisition-store/src/lib.rs` 28,739.

## Bytes read, per task

Each cell is always loaded + required by a rule + needed to resolve an
ambiguity, in KB.

| Task | 2026-11-10 (`517836f8`) | This sitting (`b2cd41a8`) |
| --- | --- | --- |
| 1. A small fix | 19.1 + 50.0 + 8.1 (issue 200) | 22 + 116 + 13 (issue 212) |
| 2. The locked-store refusal | 19.1 + ~57 + ~14 | 22 + ~84 + 7 |
| 3. Start the next step | 19.1 + 14.9 + 6 | 22 + 63.7 + 2.5 |
| 4. A boundary ruling | 19.1 + ~46 + ~6 | 22 + ~47 + ~4 |

Three things to know before reading the table as a trend.

- **Task 1 is a different task.** Issue 200 is closed. The prompt's
  fallback gave the runner issue 212, which touches two areas and
  owes a brief by its own comment. Of its 116 KB, 75 were read and 41
  were owed and not read (two area files and the template).
- **Task 2's 84 KB is from cold.** The runner counted 22 KB new on
  top of task 1 and gave the cold figure beside it.
- **Task 3 is a like-for-like regression.** The runner could state
  the next step after about 47 KB read, against 20.9 KB on
  2026-11-10. The road's first "Next" item points into
  `audits/whole-review-2026-11/NEXT.md` and
  `audits/meta-2026-11/README.md`, 58 KB together.

The always-loaded column grew from 19.1 to 22 KB: `AGENTS.md` is
20,942 bytes, up 1,818.

## What the runner stopped on

Fates are not assigned here. This is the before-run, and the after-run
is read against it. Each stop is marked by whether the audit's six
changes already propose an answer.

| Stop | The report's words | In the six changes? |
| --- | --- | --- |
| The road's "Next" is a pointer | "63.7 KB to resolve one line" | No. The largest reading cost the run met |
| The owner's queue is not his queue | The road calls the `trigger:ruling` label his queue, the label is empty, and the road lists four decisions filed nowhere | No |
| Two reading rules in one sentence | "28.7 KB of pack plus 35.7 KB of area files owed", with no precedence | Change 1 |
| Who may amend an entry | `tools/bothy.py` rules 6 and 9 against C170 and C192 | Change 5 |
| No test for "a small fix" | Issue 212's comment owes a brief; C173 says a small fix needs none | No |
| No step id for a spoken request, or for an issue with two areas | session-close §3 | No |
| A programme file against the rulings | `NEXT.md` names Sonnet as a builder and a grant of pushes; C172 and C171 say otherwise | No |
| `bothy list` does not print the next free id | the runner sorted by hand | No |
| Nouns with no glossary line | lane, spike, series, programme; "door" in C124 and C193 | Change 5, for "door" |
| Whether a wording change is a behaviour change | C175 | No |
| Which of three refusals "the store is locked" means | nothing maps a user's phrase to a refusal | No; met on 2026-11-10 as well |

The runner found the owner's classes stated alike in `AGENTS.md` and
C170, which the commit under test had just made true.

## What it would keep

`bothy pack` and `show`, a golden's header with its `# holds` lines,
and C170's list of classes beside C171's grant to commit.

## What differs from the earlier sitting

- The tree, by sixteen days of work.
- Task 1's issue.
- One ground rule of the prompt.
- Claude Code 2.1.331 against 2.1.322, and the Opus model of the day.

The owner's global `~/.claude/CLAUDE.md` was in the runner's context,
as before. The runner could not measure it and estimates the always
loaded column at 22 KB with it.
