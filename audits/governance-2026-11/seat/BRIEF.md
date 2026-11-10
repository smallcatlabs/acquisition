# The instructions sitting (#228): how it runs

#228 is the step: its goal, its four tasks, the baseline's bytes and
stops, and its done-when. This file is how the sitting runs, so that it
is the same sitting as the baseline's and is blind. The session that
takes #228 reads this file and `PROMPT.md`, and never sends the runner
anything else.

## The prompt

`PROMPT.md` is sent to the runner as it stands. It is
`BASELINE-PROMPT.md` (the baseline's own words, recovered from its
transcript) with five changes, and no others (`diff` the two):

1. The repository is "the repository in your working directory", not
   the owner's checkout's path.
2. A new ground rule: open nothing under `audits/governance-2026-11/`,
   which holds this record and the baseline's report; a pointer there
   is logged, not followed.
3. Task 1 no longer names `roadmap/README.md` "A step" and the
   template as where to look for whether a brief is needed: the
   rebuild removed "A step", and the old pointer would test the prompt.
4. Task 1's list of what else to write drops "Current step", a section
   the rebuild removed, for the same reason.
5. Task 3 starts "only from AGENTS.md", not from the old pointer's
   quoted words ("The road ahead …"), which `AGENTS.md` no longer has.

## The world

A clone outside the repository, never a worktree or a subagent: both
start with this project's memory index in context (the seat skill,
step 6). Measured on `268425e3`, 2026-11-10: a headless runner started
in a clone under the scratchpad had `AGENTS.md` in its context and no
memory index (its transcript names no memory file; the only
`MEMORY.md` text is the harness's note on the clone's own empty memory
directory), and `gh` answered once the clone's `origin` was GitHub's.

```sh
REPO=$(git rev-parse --show-toplevel)    # run in the checkout
W=~/seat-worlds/governance-2026-11
git clone --quiet "$REPO" "$W/tree"
git -C "$W/tree" remote set-url origin https://github.com/smallcatlabs/acquisition.git
git -C "$W/tree" log --oneline -1    # the tree under test; the row's build
```

The clone is of the committed `main`: commit first, and say in the
record which commit it was.

## The run

One headless Opus runner, in the background, with rails that allow the
prompt's commands and nothing else. Measured with the same rails on
`268425e3`: `wc`, `bothy.py pack` and `gh issue view` ran; `touch` and
`git commit` were denied; the clone's status stayed clean.

```sh
cd "$W/tree" && claude -p --model opus --permission-mode dontAsk --strict-mcp-config --setting-sources project \
  --allowedTools "Read(/$W/tree/**)" "Bash(git log:*)" "Bash(git show:*)" "Bash(git grep:*)" "Bash(grep:*)" \
    "Bash(wc:*)" "Bash(ls:*)" "Bash(cat:*)" "Bash(head:*)" "Bash(tail:*)" \
    "Bash(python3 -I tools/bothy.py pack:*)" "Bash(python3 -I tools/bothy.py show:*)" "Bash(python3 -I tools/bothy.py list:*)" \
    "Bash(gh issue view:*)" "Bash(gh issue list:*)" \
  --disallowedTools Agent Workflow Skill NotebookEdit WebFetch WebSearch Write Edit \
  --output-format json "$(cat "$REPO/audits/governance-2026-11/seat/PROMPT.md")" \
  < /dev/null > "$W/result.json"
```

The rails are not a fence: `cat` and `grep` can read outside the clone.
The transcript is the check, below.

## After the run

The transcript is the call log. Its session id is `result.json`'s
`session_id`; the transcript repeats a call, so the log keeps each
call id once:

```sh
SID=$(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["session_id"])' "$W/result.json")
python3 -I -c '
import csv,json,sys
w=csv.writer(sys.stdout); w.writerow(["n","tool","input"]); seen=set()
for line in open(sys.argv[1]):
    m=json.loads(line).get("message") or {}
    for c in m.get("content") or []:
        if isinstance(c,dict) and c.get("type")=="tool_use" and c["id"] not in seen:
            seen.add(c["id"]); i=c.get("input",{}); w.writerow([len(seen),c["name"],i.get("command") or i.get("file_path") or json.dumps(i)])
' ~/.claude/projects/*/"$SID".jsonl > "$W/calls.csv"
```

Before believing the report, check from the transcript and `calls.csv`:

- no memory note's name appears in the transcript, and `AGENTS.md`'s
  first heading does;
- every path read is inside `$W/tree`, and none is under
  `audits/governance-2026-11/`;
- `result.json`'s `permission_denials`, each named in the record;
- the report's byte counts, recomputed with `wc -c` on the clone for
  each file its log names (seat skill, step 7: say how many you
  recomputed).

The record, under this directory: `REPORT.md` (`result.json`'s
`result`, whole), `calls.csv`, and `README.md`: the run, its tree, its
call count, the conditions that differ from the baseline's (below), the
three byte columns per task beside the baseline's, each baseline stop
marked gone, still there or changed with the sentence that settles it,
the new stops, and one row per stop with its fate (a commit, an issue,
or why it is no fault), as #228's done-when asks.

## What differs from the baseline, by design

- The baseline ran as a subagent in the repository with the memory
  index in context, told to ignore it; this runner has none.
- The baseline ran in the session's own permission mode; this runner
  runs under the rails above.
- The five prompt changes above.

The baseline's report did not cite its calls by number, and the record
compares bytes and stops, not calls.

## The ledger row

One row of `SEAT-LEDGER.md` when the record is committed, the
columns read for a sitting that runs no tool: the date; the build is
the clone's commit; the door is "the instructions, read-only", the
model, and the world `~/seat-worlds/governance-2026-11/tree`; the
calls, as `calls.csv` counts them; the per-task column holds the three
byte columns per task beside the baseline's (no earlier row on this
door); the stops are the rails' denials and any task left unanswered;
the faults by fate as `README.md`'s table holds them that day.
