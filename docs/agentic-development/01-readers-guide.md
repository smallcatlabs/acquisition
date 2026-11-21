# 1. Reader's guide

## What this is

A case study in agentic software development, written for an
experienced engineer who has not worked this way. The subject is one
project: Acquisition, a tool for a video game's players that has
existed since 2014 as a C++ desktop app, and that its current
maintainer rewrote in Rust between August and October 2026 using AI
agents as his engineering staff. The series was produced in the same
way, by an AI session orchestrating research agents over the
repository, the session transcripts and the owner's own account, and
`METHOD.md` says how.

The series is nine reports. Read this one first; after it, any
order works, and each report says which others it leans on.

## The thesis

Over four months, the work of building this product moved from
writing code to writing the constitution the code is written under.
The owner wrote essentially no code after July. He wrote, or had
written and ruled on, the invariants, the rules, the briefs, the
reviews and the decisions, and the agents wrote the code and most of
the prose. The product works and its core is sound. The question the
record leaves open is whether that trade pays: by late October the
project's own assessment found that the words had become the mass,
and the owner cut his own process in response.

Three findings that run through every report:

- **Structure beats discipline for workers that forget.** The rules
  that held were the ones turned into dependency edges, lints,
  tests and refusals in the binary. The rules that cost the most were
  the ones held by words.
- **The agent is the user.** The project decided in September that
  the primary customer of its command-line tool is an agent, and
  built an instrument, the seat, to test the tool's words with a
  model that has nothing else. Rulings about the product came
  straight from it.
- **Nothing retires unless someone makes it.** Every fault became a
  test, a checklist row, a review shape. The principle that ceremony
  is retired by evidence was written down in September and, by the
  project's own account, barely applied until the owner forced it in
  October.

## The reports

| # | Report | Read it for |
| --- | --- | --- |
| 2 | The project | what Acquisition is, the constraint that governs it, the architecture, the numbers |
| 3 | History | the eras from 2014 to now, with the commit evidence and the gaps in the record |
| 4 | Turning points | fourteen decisions that changed the course, with who made them and what followed |
| 5 | The operating model | the cast, the lifecycle of a step, every mechanism with its origin, the owner's corrections, the prompting arc |
| 6 | Concepts | the ideas, each with its artefact, its cost, and the project's own counter-argument |
| 7 | Retrospective | what worked, what it cost, what the owner would do differently, in his words where he gave them |
| 8 | Playbook | what to do on Monday, scaled to a web team, and what not to copy |
| 9 | Primary sources | the real documents, annotated: a ruling, a brief, a review, a seat record, a commit message |

## Words you will meet

The project writes in a compressed style of its own, and the reports
translate it wherever it is quoted. The handful of terms that recur
too often to gloss each time:

- **The owner**: Tom, the maintainer, git author GERWARIC. The
  reports use "the owner" because the project does.
- **GGG**: Grinding Gear Games, the game's publisher, whose API the
  tool calls. "The relationship" is the project's standing with GGG:
  its OAuth registration and its rate-limit record, treated as the
  most valuable asset.
- **The daemon** (`acqd`): the one process that ever talks to GGG.
  The CLI (`acq`) and the MCP server (`acq-mcp`) are thin frontends
  that share it. MCP, the Model Context Protocol, is how an AI agent
  calls a tool.
- **A ruling**: a decision the owner made, recorded as a numbered
  bullet (C88, C116) with a *Why* and the test that *pins* it. The
  collection is **the registry**. A **finding** is what building or
  running taught, recorded after the fact.
- **A brief**: the committed document a session reads to do one
  step: goal, what to read, what done means. A **step** is one brief,
  build, gate, commit, review and close; a **slice** is a closed body
  of work with one line in the history file.
- **The gate**: the script of ten checks every commit must pass.
  "The gate decides."
- **A seat**: a run in which a blind agent, given only a brief and
  the tool's own words, drives the released tool and reports where it
  got stuck.
- **A live run**: anything that talks to the real GGG API. Always
  the owner's, from a terminal. Every agent session runs in a **mock
  world**: a built-in stand-in for GGG under an isolated directory.
- **Fable, Opus, Sonnet**: Claude models, in descending capability
  and cost. **Codex**: OpenAI's coding agent, used here as a
  read-only reviewer, driven by OpenAI's frontier models.

## How the evidence is marked

Every number in the series is at commit 10c40dc2 of 2026-10-26
unless a sentence says otherwise, and report 3 gives the commands
that reproduce the main ones. Quotations are exact. Where a claim is
reconstructed rather than observed (report 5 on the period with no
transcripts), it is labelled a hypothesis with the test that would
settle it. Where the series corrects the project's own record (the
share of markdown-only commits, a paraphrase of the assessment), it
says so in place. The owner's answers to `INTERVIEW.md`, where he
gave them, are quoted as his.

## What is not here

The owner's two years of learning C++ with AI chat tools before any
of this, which lived outside the repository. The sessions that ran
in Claude Code's cloud between August and mid-September, whose
transcripts were not reachable from this machine. The second
computer's transcripts. Each is named in `METHOD.md` as a candidate
for a second round.
