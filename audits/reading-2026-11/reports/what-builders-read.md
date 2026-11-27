# What build sessions read before and after their first product-code edit

All scripts and intermediate files are in `<scratch>/readers/transcripts/`: `extract.py`, `common.py`, `scope.py`, `analyse.py`, `report.py`, `more.py`, `fp.py`, `weekly.py`, `compare.py`, and `reviews.py` (a copy of the audit script, changed to write `rows.json` to scratch). I quote or paraphrase no human message.

## Scope (`scope.py`, `analyse.py`)
- **Input:** 721 transcript files (top-level and subagent) across the main project directory and 15 worktree directories, each with activity since 2026-10-20.
- **What counts as a crates edit:** an Edit or Write whose path contains `/crates/`, or a Bash edit whose target is under `crates/`. Bash edits include `python3 - <<` heredocs, `apply_patch`, `sed -i`, `perl -pi` and `cat >`. Targets are taken from the path literal or the argument. My first matcher wrongly caught heredocs that edited briefs merely mentioning `crates/` paths; I checked samples and fixed it.
- **Funnel:**
  - 58 files had a crates edit and activity since 11-10.
  - 54 remained after removing forks (copies sharing the same first-edit tool id; the largest was kept).
  - 8 subagent transcripts were set aside.
  - 6 sessions were excluded as owner-driven or orchestrating: 6 or more user-turn messages, or 4 or more `start_session`/Agent launches (0d073efe, a4d7711c, 3f8cbdd8, f595a236, 88804c8a, 35cb9de3).
  - Reading-only sessions drop out by definition, since they have no crates edit.
- **Result: 40 builder sessions.** Of these, 39 began from a launch prompt of over 500 characters, almost all of them hand-offs from another agent. ee5eec19's prompt may have been typed by the owner (judgment).

## Before the first edit (`report.py`)
"Read" here means a Read call, `cat`, `sed -n`, `head` or `tail`; a grep is counted separately. n = 40.

| Document | Sessions that read it | Of those, read whole |
|---|---|---|
| `decisions/process.md` | 36 (90%) | 31 |
| the brief | 32 (80%) | 28 |
| `roadmap/README.md` | 29 (72%) | 22 |
| `REVIEW-CHECKLIST.md` | 9 (22%), plus grep-only in others | **0** |
| `decisions/frontends.md` | 6 | 4 |
| a saved review file | 6 | 5 |
| `decisions/search.md` | 5 | 4 |
| `search/README.md` | 4 | 3 |
| skill files read as files (cold-review 3, site-sitting 3, seat/session-close/mock-session 1 each) | ≤3 each | |
| `decisions/network.md` | 2 | 2 |
| `CLI-REFERENCE.md` (also grepped in 4 more) | 2 | 0 |
| `decisions/trade.md`, `decisions/store.md`, `MCP-REFERENCE.md` | 1 each | |
| `decisions/daemon.md`, `plans.md`, `pricing.md`, `ci.md`, `TESTING-NOTES.md`, `GLOSSARY.md`, `AGENTS.md` | 0 | |

- **Governance bytes read before the first edit** (bytes of the tool results): median 39.6 KB, range 0 to 80.8 KB. One session (24b15c11) read none. The brief is the largest share (median 15.5 KB), then `process.md` (median 9.2 KB).
  - When one `cat` read several files, I split its bytes evenly between them, so per-document bytes are approximate.
- **Tool calls before the first edit:** median 34.5, range 10 to 85.
- **Tokens:** where the transcript carries usage (all 40), the context at the first edit has a median of 180k tokens (106k to 277k). Summed input per turn, cache reads included, has a median of 4.3M (0.8M to 15.5M).
- **`bothy.py pack`** ran before the first edit in 2 of 40 sessions:
  - 4034876f ran 4 calls, `--path crates/acquisition-cli/src/…` and `--path crates/acquisition-mcp/src/…`;
  - 9d13138b ran `--area network`.
- **`bothy.py show`** ran before the first edit in 0 sessions.
- **Skill tool calls before the first edit:** mock-session 1, cold-review 1. Over whole sessions: cold-review 14, session-close 6, mock-session 1.
- **Area file and pack**, where the area comes from the crates edited (judgment mapping: cli/mcp/verbs to frontends, search to search and trade, and so on):

| | Sessions |
|---|---|
| read the area file whole AND ran pack | 1 |
| area file whole only | 10 |
| pack only | 1 |
| neither | 28 |

  A further 6 sessions read the area file only by range or grep. Of the 17 sessions since 11-21, 2 read their area file whole.
- **Launch prompts** (`fp.py`):
  - 34 of 40 name file paths and say to read them. The median is 10 paths among those that name any, 9.5 over all 40.
  - Governance files named:
    - in 31 prompts: `AGENTS.md` and `process.md`;
    - in 29: the brief;
    - in 25: the road;
    - in 13: `REVIEW-CHECKLIST`;
    - area files: `search` 10, `frontends` 9, `store` 4, `trade` 4, `network` 1, `plans` 1.

## After the first edit (`report.py`, `more.py`)
- **Documents first read after the edit** (sessions):

| Document | Sessions |
|---|---|
| a saved review file | 31 |
| `cold-review/SKILL.md` | 20 |
| `session-close/SKILL.md` | 14 |
| `CLI-REFERENCE.md` | 8 |
| the brief | 7 |
| `REVIEW-CHECKLIST.md` | 6 |
| `AGENTS.md` | 5 |
| `mock-session/SKILL.md` | 5 |
| area files (search 3, frontends 3, trade 2, store 2, pricing 1, daemon 1) | ≤3 each |
| `TESTING-NOTES.md` | 2 |
| `GLOSSARY.md` | 2 |

- **`bothy show`:** 3 calls in 2 builders, C190 and a `$c` loop.
- **`bothy pack`:** 4 calls after the first edit.
- **Grep for a C-id:** 373 calls in 33 of 40 builders. Top ids: C190 (61), C187 (49), C194 (23), C195 (17), C47 (16). By target (all builder Bash greps): `crates/` 319, `decisions/` 55, `git log` 10, other 16.

### What came just before each on-demand call
The trigger is the tool result just before the call. Classes are from a heuristic classifier and are approximate.

| Call | Count | Triggers |
|---|---|---|
| C-id grep | 373 | id seen in the previous result 107, code search 91, after an edit 48, review 31, code read 29, app card 16, failing test 14, wait or monitor 13, gate failure 4, other or chain 18, skill 1, hook refusal 1 |
| area-file read after the edit | 35 calls | app card 11, code search 8, id seen 8, review 4, gate failure 2, skill 1, code read 1 |
| review-file read | 140 | review output 113 |
| brief re-read | 161 | review 40, app card 27, wait 26, other 22, id seen 18, chain 12, code search 11, other classes 5 |
| `REVIEW-CHECKLIST` read | 11 | code search 6, app card 3, other 1, id seen 1 |
| reference docs (CLI 26, MCP 10) | 36 | code search 16, failing test 6, after edit 4, id seen 4, code read 3, review 2, other 1 |

In plain terms: rulings are pulled by an id found in code or a result, by a search, by a review, or at plan time. A failing test or a failing gate stage almost never sends a builder to the registry: 5 such reads, 18 such greps.

## pack and show calls per week, all sessions (`weekly.py`)
Deduplicated by tool id. Weeks start on Monday, so the first week includes 10-19.

| Week | pack | show | All tool calls |
|---|---|---|---|
| 10-19 | 44 | 6 | 10,478 |
| 10-26 | 44 | 13 | 15,797 |
| 11-02 | 8 | 0 | 10,142 |
| 11-09 | 43 | 24 | 6,161 |
| 11-16 | 2 | 15 | 6,955 |
| 11-23 | 23 | 11 | 7,059 |

Since 11-10, 58 sessions called pack or show: 3 builders, 24 subagents, 31 others (orchestrators, audits, tool work).

## Never or almost never read by a builder, at any time in the session
- **Area files:**
  - `decisions/daemon.md` 0 of 40 (1 read after the edit);
  - `plans.md` 0;
  - `ci.md` 0;
  - `pricing.md` 1;
  - `network.md` 2;
  - `store.md` 2;
  - `trade.md` 3;
  - `search.md` 5;
  - `frontends.md` 6.
- **Other documents:** `TESTING-NOTES.md` 1, `GLOSSARY.md` 1, `MCP-REFERENCE.md` 1, `AGENTS.md` read explicitly 1 (it is auto-loaded), `seat/SKILL.md` 2.
- **`REVIEW-CHECKLIST.md`:** never read whole by any builder, always by range or grep (14 sessions).

## Comparison: area file read whole vs not (`compare.py`)
Each session was mapped to the step whose review files it named most often. Rounds count build plus fixes reviews, using the copied `reviews.py`. 39 of 40 sessions mapped.

| Group | n | Median rounds | Steps with a P1 in the first review |
|---|---|---|---|
| area file read whole | 11 | 1 | 1 |
| not read whole | 28 | 2 | 4 |

- **Confounded.** 17 of the 28 "not read whole" sessions are GUI form/step6 steps (`4-full-1-*`), with long briefs, against 2 of the 11. The whole-readers are mostly search, network and seat steps.
- Leaving out the `4-full-1` steps, the medians are 1 (n=9) and 2 (n=11): still apart, but small samples.
- A step can have more than one builder, and the P-severity regex is the old audit's.
- I would not claim an effect. Judgment.

## Three observations I did not expect
1. **Builders skip the area files their own prompts name.** Prompts named an area file 29 times and it was read before the edit 10 times (frontends 2 of 9, search 5 of 10). `process.md` was named 31 times and read 28 times. Builders take the process path, not the rulings path.
2. **pack is close to unused by builders.** 2 of 40 ran it before the first edit, and only 1 prompt asked for it. pack calls come mainly from tool-work and audit sessions. The whole week of 11-16 saw 2 calls.
3. **Rulings are found by grepping code for ids, not through the registry.** 319 C-id greps went into `crates/` against 55 into `decisions/`, and 3 `show` calls. Builders discover a ruling where the code cites it, which leaves an uncited ruling invisible to them. That last point is judgment.

## What the transcripts cannot show
- **Launch prompts:** 34 of 40 carry hand-written background (lengths up to 7.5 KB). Rulings stated inside a prompt are content the builder received without reading a file.
- **Auto-loaded context:** `AGENTS.md`, `CLAUDE.md` and memory are loaded automatically, and the content of skills loaded through the Skill tool is not a file read.
- **Reads I could not detect:**
  - reads inside Python (`open().read()`), `git show rev:path`, and paths relative to a `cd crates`;
  - Bash edits whose target I could not parse, which may make some first edits too late.
- **Other readers:**
  - the reading done by the 8 subagent builders I set aside, which I did not tabulate;
  - the Codex reviewers' reading;
  - what compaction summaries carried forward.
- **Whether a read was used:** a read in the transcript does not show attention to it.
- **Trigger classes** come from keyword heuristics, and I did not check them by hand beyond about 50 samples.

## Not checked
- Week-level counts within builder sessions only.
- Brief-review rounds.
- The 6 excluded sessions' reading.
- Whether a pack output's suggested reads were followed.
