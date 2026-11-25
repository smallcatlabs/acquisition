# Investigator preamble — planning the whole-project review (2026-11-24)

You are one of about a dozen read-only investigators. The repository is
the repository (Rust workspace,
~115k source lines, ~130k test lines, ~4,200 commits since July 2026,
built almost entirely by agents under one owner). `AGENTS.md` at the root
is the entry point; read it first.

## What this is for

The owner is about to order a full-stop review of the whole project, run
by a future orchestrating session. **You are not performing that review.**
You are scouting one perspective so the orchestrator knows (a) how the
system really works from your angle and (b) where a deep review would
find *valuable or unexpected* improvement: things missed because everyone
was busy building. Bugs and gaps matter less than patterns: a missing
shape that several places work around, one duty held in many places, a
second semantics, a fact in two homes, a mechanism that could be deleted,
a cost nobody measures, a lesson only visible in hindsight. The owner's
guiding lights are simplicity, usability and elegance, and the goal is "a
faster, simpler, better Acquisition that is agent-friendly and
agent-forward". Ask what would make the thing radically simpler or more
beautiful, not what one more patch would fix.

## Hard rules

- Read-only. No edits, no commits, no `git stash`, no `gh` writes
  (`gh issue list/view`, `gh run list`, `gh pr list` reads are fine).
- **Run no `cargo` command, no `tools/gate.sh`, no test, no `acq`,
  `acqd` or `acq-mcp`.** Builds run one at a time in this tree and other
  investigators share it. `git`, `grep`/`rg`, `wc`, `awk`, `python3 -I`
  one-liners over tracked files, and `python3 -I tools/bothy.py list|pack`
  are fine.
- Never read the owner's real store or anything outside the repository
  except the sibling clone the brief names.
- Do not spawn agents.

## Prior art: do not rediscover it, build on it

- `audits/semantic-model-2026-11/README.md` and `reports/*.md`: state,
  transitions, invariants "held by hope", one fact in two places, dead
  work, refactors (read at `5f3ad0d`, mid-November).
- `audits/assessment-6-5/README.md`: what could be removed; tables of
  every review finding by step, area, file and repeating cause.
- `audits/governance-2026-11/README.md`: the governance audit and what
  was ruled from it (`decisions/process.md`).
- `roadmap/TEST-SHAPE.md`, `TESTING-NOTES.md`, `REVIEW-CHECKLIST.md`,
  `decisions/<area>.md`, `roadmap/README.md` (the road),
  `roadmap/briefs/REVIEW-PROMPT.md` (the standing review prompt and its
  numbered "shapes").

Where one of these covers your slice, read it, then say what has happened
since (fixed, filed as an issue, or still standing: check with
`git log`, `git grep` and `gh issue list --search`) rather than repeating
it. A finding that an audit made and nothing acted on is itself a result.

## Method

- Name the command behind every number. Mark judgment as judgment.
- Cite `path:line`. Re-read the code behind your top three claims
  yourself before reporting them; say which you verified.
- Prefer breadth first, then depth on the two or three places that look
  richest. Sampling is fine: say what you sampled.

## Report: returned as text, 1,500 words at most, in this order

1. **Model**: how this slice actually works, the things an orchestrator
   new to it must know (150–250 words). Not a file listing.
2. **Measurements**: a small table, each with its command.
3. **Where a deep review would pay**: ranked hypotheses. For each: the
   evidence, the question a reviewer should ask, the files to read, the
   rough size of the payoff (lines removed, concepts removed, risk
   retired, tokens or seconds saved), and your confidence.
4. **Low yield / already covered**: where not to look, and why.
5. **Blind spots**: what nobody seems to have looked at, and what you
   could not determine.
6. **Surprises**: anything that contradicted what the documents say.
