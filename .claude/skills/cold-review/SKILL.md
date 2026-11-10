---
name: cold-review
description: Order an outside review of a committed round or a brief — pick the standing prompt, launch the reviewer, save its answer whole, count the rounds, and answer the findings by one rule each. Use after committing a round that changes behaviour, a tool, the gate, a workflow or the shared test harness, and for a brief that carries a design.
---

# Cold review

The rulings are `decisions/process.md`: what is reviewed (C175), how
many times (C176), and who reviews with what prompt (C177). This is
how.

## 1. Count first

Count each artifact apart, by the step's whole name, and never count a
saved prompt:

- a brief's reviews:
  `ls roadmap/reviews | grep '^<step>-brief-' | grep -v -- '-prompt\.md$'`
- a build's, with its fix rounds:
  `ls roadmap/reviews | grep '^<step>-' | grep -v -e '-brief-' -e '-prompt\.md$'`

A fourth review of one artifact (a brief, or a build with its fix
rounds) is the owner's word, quoted in the commit that saves it.
After a third round that still finds faults which change what gets
built, stop and bring him what it found: the design is unsettled.

## 2. Launch

The reviewer is Codex, on the owner's account, read-only. The prompt
is a standing file, and you add only what names the artifact:

- A committed round, against its brief:
  `codex exec --sandbox read-only "$(cat roadmap/briefs/REVIEW-PROMPT.md) The commit: <sha>." < /dev/null`
  A range is written `<first>^..<last>`, so git includes the first.
- A round whose brief is an issue: the same, with the issue's own text
  appended after the commit (`gh issue view <n> --json title,body`),
  since the reviewer has no network.
- A brief:
  `codex exec --sandbox read-only "$(cat roadmap/briefs/BRIEF-REVIEW-PROMPT.md) The brief: <path> at <sha>." < /dev/null`

Close stdin with `< /dev/null`: without it the process waits for input
and never reviews. Run it in the background and wait for it to exit; a
minute in, check its output has grown. Add no account of your own to
the prompt: the commit or the brief is the evidence, and your reading
of it would become the reviewer's.

Where Codex is not installed, a fresh subagent of another model family
is next; a reviewer of your own family is the last resort, and the
saved file's first line says so.

## 3. Save

The answer is printed twice; save the copy after `tokens used`, whole,
from its first line to its end, and nothing else:

- a build: `roadmap/reviews/<step>-<sha>.md`
- a fix round: `roadmap/reviews/<step>-fixes-<sha>.md`
- a brief: `roadmap/reviews/<step>-brief-<sha>.md`

Commit the file on its own, before the round that answers it. The
prompt is not saved: it is the standing file at that commit.

## 4. Answer

- Reproduce each finding as a failing test before fixing it, and
  break each fix in turn to see its test fail.
- Look for the one rule beneath several findings, and fix by that
  rule, never case by case.
- A finding is sized by who can meet its case, read from the code
  that bounds it (a real account, or only a mock world). A fix is
  weighed by the mechanism it adds against that risk; a small gap in
  a promise an agent relies on is closed, not caveated; and a finding
  for a case nobody will meet is named as not taken, with the reason,
  in the round's commit message.
- A finding that would change a ruling's words, or that has two
  defensible answers in the owner's classes, goes to his queue
  (`decisions/process.md`, C179) before it is built.
- A clean round ends the reviewing. Whether a fix round needs a review
  of its own is C175's to say, and this skill does not restate it.
