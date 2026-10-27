# How this series was produced

Started 2026-10-26 on branch `report/agentic-development`, in a git
worktree beside the main development tree, so the reports never
collide with ongoing work.

## Sources

- **The repository.** Git history from 2014 (3,840 commits at the
  start of this work), `HISTORY.md`, the rulings under `decisions/`,
  the roadmap and its briefs and reviews, the audits, the run ledger,
  the skills, the tools and the gate.
- **GitHub.** Issues, labels, milestones, releases and workflow runs.
- **Session transcripts.** Claude Code keeps a JSONL transcript per
  session. 114 survive, from 2026-08-10 to the present, about one
  gigabyte raw, all from this machine. The July to mid-September 2026
  sessions are missing here. The commit history says where most of
  them went: 513 commits between 2026-08-08 and 2026-09-14 carry a
  `Claude-Session` trailer pointing at claude.ai/code, 80 distinct
  sessions, so that period ran largely in Claude Code's cloud
  sessions, whose transcripts are not on this machine and were not
  reachable from the desktop app's session tools. The owner also
  worked from a second computer (a Windows PC, by the paths in the
  commits) for part of August. A second round could export the cloud
  sessions from claude.ai/code by their ids. The project's
  earlier location on disk (`gerwaric/acquisition`) contributed its
  four surviving sessions; the C++ app's own repository has none here.
  Separately, 25 transcripts of the October "seat" runs survive (a
  blind agent driving the released tool); they were not digested but
  are drawn on for the seat concept and the primary-sources appendix.
  The dialogue (the owner's prompts, the model's replies, subagent briefs
  and commit commands) was extracted to about 9 MB and digested.
- **The owner's memory files.** The per-project memory Claude keeps
  across sessions, which reads as the owner's running list of
  corrections and lessons.
- **The owner.** An interview (`INTERVIEW.md`) for what no artifact
  records: motive, hours, money, feeling.

## Not mined

The 2024 to early 2026 era, when the owner used AI chat tools on the
side to learn C++ while maintaining the original app. Those
conversations live outside this repository and its tooling. The era
is narrated from the owner's account and the C++ commit history only.

## Division of labour

The orchestrating session (Claude Fable 5.1) scoped the series with
the owner, extracted the transcripts and wrote the briefs. Research
ran as subagents: a Sonnet pass over git and GitHub for the history
and the numbers; Opus passes over the briefs, reviews, audits and
rulings for the turning points and the operating model; an Opus pass over the commit history of the
window no transcript covers (2026-04-11 to 2026-09-20, 1,378
commits), reconstructing those sessions as labelled hypotheses; a
Batch API job over the transcript dialogue for the per-session digests the
prompting narrative rests on. Synthesis chapters were written from
the research dossiers, with every claim that matters checked against
the tree before it was written down.

## Cost

The Batch API work ran on Claude Opus 5.5 at batch rates: the 114
digests took 3.5 million input tokens and 650,000 output tokens, the
five era narratives about 1.4 million input and 45,000 output, roughly
twenty dollars in all. The research agents and the writing ran in
Claude Code sessions whose token spend is not itemised here.

## Status

First pass of all nine reports complete on 2026-10-27, one day of
work. Open: the owner's interview answers, to be folded into the
retrospective; and a possible second round over the cloud sessions.
