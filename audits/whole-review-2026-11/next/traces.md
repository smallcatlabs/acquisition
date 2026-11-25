# What the agents here actually did, counted

The review's process investigator named one thing it could not check:
"tokens actually read by real sessions". The harness keeps a transcript
of every session and subagent on the owner's Mac. These counts are over
those, from 2026-10-12 to 2026-11-25: 255 sessions and 480 subagent
transcripts. The scripts are `scripts/trace.py` and `scripts/trace2.py`
(they read tool calls and result sizes, and print no message text). The
co-change and gate tables are from `git log` and `runs/gate/verdicts.tsv`.

Every number is a heuristic's. A shell command is classed by a regex;
a command's output is credited to the tracked files it names, split
evenly. Read the shares, not the digits.

## Who did the work

| Model | Turns | Output tokens | Cache read |
| --- | --- | --- | --- |
| Opus 5.5 | 102,539 | 75.5 M | 22,221 M |
| Fable 5.1 and 5.2 | 34,457 | 57.1 M | 9,518 M |
| Sonnet 5.5 | 502 | 0.3 M | 45 M |
| Haiku 5.5 | 62 | 0.0 M | 3 M |

The sessions' own cost records sum to about $5,300 for the six weeks.
An Opus turn re-reads 217k tokens of context on average. Cost here is
turns times context, and three quarters of the turns are Opus's.

## What a turn is

63,555 shell calls against 1,196 uses of the file reader and 1,785 of
the file editor and writer. Agents here read and edit through the
shell.

| Shell calls, by kind | Calls | Wall time |
| --- | --- | --- |
| grep, cat, sed, ls, find | 28,995 | 12.0 h |
| git log, show, diff, status | 7,212 | 3.9 h |
| cargo nextest or test | 4,869 | 36.6 h |
| gh | 3,596 | 8.5 h |
| the full gate | 2,024 | 14.6 h |
| codex (the cold reviewer) | 1,410 | 5.4 h |
| the docs, arch and trade checks alone; the docs gate | 2,459 | 8.4 h |
| cargo build | 1,281 | 4.7 h |
| the registry tool | 1,108 | 1.2 h |
| **`acq` or `acq-mcp`, the tool itself** | **886** | 1.2 h |
| waiting (sleep, pgrep) | 476 | 7.2 h |

The tool is run in 1.4% of shell calls. The agents who build it and
are its named users almost never use it while building.

## What is read

Shell output, by the area of the files a command names (about 115 MB):

| Area | Share |
| --- | --- |
| Product source, all crates | 38% |
| Tools, the road, briefs, reviews, rulings, skills, the checklist, audits | 38% |
| Tests and goldens | 18% |
| README and the two references | 3% |

The files named in the most transcripts are not source:
`roadmap/README.md` (378 of 735), `tools/docs-check.sh` (341),
`tools/bothy.py` (330), `roadmap/briefs/TEMPLATE.md` (311),
`AGENTS.md` (295), `tools/gate.sh` (294), `REVIEW-CHECKLIST.md` (290),
`README.md` (289), `CLI-REFERENCE.md` (281), `decisions/daemon.md`
(272). The first source file is `daemon/src/daemon.rs` (205).

## What changes together

Of 549 commits since 2026-10-26 that touch product source:

| Also touches | Share |
| --- | --- |
| the CLI crate's source | 46% |
| the CLI crate's tests | 42% |
| a golden | 30% |
| the daemon's source | 26% |
| the MCP's source | 22% |
| the generated references | 19% |

Of the 252 that touch the CLI's source: 76% touch its tests, 57% a
golden, 35% the MCP's source, 35% a reference, 16% the README. A
commit that touches a golden touches a median of ten files.

## The local gate

229 full runs since 2026-11-10: median 65 s, p90 112 s. 31 were red:
test 10, fmt 9, docs 7, clippy 3, winclippy 3, tree 3, arch 2, diff 2,
bothy 1, artifacts 1, trade 0.

## What I take from it

- The reading path is half governance. This agrees with the review's
  63 to 102 KB, from the other side.
- The CLI crate is where the project changes. It holds three things:
  the terminal's adapter, the words every door needs, and the GUI.
- The cheapest seat there is, a builder running the tool on a world
  while building, is not happening, because a populated world costs a
  daemon, a login and three applies.
- Sonnet has done 0.4% of the turns. Whatever is a move under a rule
  with a compiler and a golden to check it is being done by Opus.

## Not checked

Whether the transcript directory is complete (cloud sessions and the
Windows and Linux guests are not in it). The write-detection in
`trace2.py` is too noisy to cite and is not cited. "Context at first
edit" was computed and dropped: most edits here go through the shell,
so the editor tool's first use is not the first edit.
