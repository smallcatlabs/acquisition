# 3. The history in eras

What happened, in what order, with the evidence. Everything here is
derived from the git history at commit 10c40dc2 (2026-10-26) unless a
sentence says otherwise; the commands that reproduce each table are
in the research dossier this report rests on, and the key ones are
repeated at the foot. Dates are the author's local dates.

## 3.1 The whole arc in one table

| Era | Dates | Commits | Per active week | Who | What the code was |
| --- | --- | --- | --- | --- | --- |
| 1. The original | 2014-06 to 2016-06 | 428 | 7 | Ilya Zhuravlev and contributors | the C++/Qt desktop app |
| 2. Community tail | 2016-11 to 2019-09 | 93 | 3 | a handful of contributors | the same, patched |
| 3. Dormant | 2020-07 to 2022-12 | 57 | 4 | one unrelated contributor | small fixes |
| 4. Owner's maintenance | 2023-10 to 2026-03 | 891 | 15 | the owner alone | Qt 6, OAuth, a new rate limiter, PoE2, releases |
| 5. First AI-assisted upkeep | 2026-04 to 2026-06 | 20 | 5 | the owner, Claude on one day | Qt and API bumps |
| 6. AI-driven C++ cleanup | 2026-07-07 to 2026-08-08 | 387 | 77 | the owner with Claude sessions | characterisation tests, refactors, a network redesign spec |
| 7. The Rust spikes | 2026-08-09 to 2026-10-11 | 1,287 | 129 | the owner with Claude sessions | a rate-limiter core, then the whole Rust workspace under `spikes/` |
| 8. Root swap to alpha | 2026-10-12 to 2026-10-19 | 298 | 149 | the same | Rust becomes the repository; three platforms; release |
| 9. After the alpha | 2026-10-19 to 2026-10-26 | 379 | 190 | the same | the roadmap: governance reset, readiness, the honesty slice |

Two things to notice before the detail. First, ten months of 2023 to
2026 maintenance by one person, at fifteen commits a week, is already
a respectable solo open-source pace. Second, the four AI months hold
2,351 commits against 1,416 for the twelve years before them, and the
rate climbs within those months, from 77 a week to 190. Commits are
not units of work, and section 3.6 shows that what a commit *is*
changed as well. But the step change is real and it is the setting
for everything else in this series.

## 3.2 Before the owner: 2014 to 2023

Ilya Zhuravlev published Acquisition in June 2014 as a C++/Qt desktop
tool for Path of Exile players. The first month has 28 commits, with
subjects like "Fix linux build" and "Fix critical strike chance
filter". Over two years he and a few contributors (Eric White, xyzz,
Gloorf) built the tool most players still know: stash download, item
search with filters, buyout notes and shop pages.

By mid-2016 the original author's activity ends; the tree receives a
thin tail of fixes to 2019 and then goes quiet. Between 2020 and 2022
a single unrelated contributor lands 57 small commits. There are no
commits at all between 2022-12-31 and 2023-10-26.

## 3.3 The owner's years: 2023 to mid-2026

The owner's first commit is dated 2023-10-26 ("add the ability to
expire test builds"). What follows is 891 commits over two and a half
years, every one his. Classifying the subjects gives the shape of the
work: about 190 fixes, 160 release and version commits, 90 build and
packaging, 80 Qt and dependency, 75 rate-limit and network, 55 OAuth.
In other words, the unglamorous work of keeping a decade-old desktop
app alive: the Qt 6 migration, GGG's move to OAuth (the v0.10 release
of November 2023), a dynamic rate limiter, Path of Exile 2 API
changes, crash reporting, and installers for three platforms.

This is also the era the owner describes as learning C++ with AI
chat tools on the side: "describing problems of pasting code snippets
into a web chat, then copying bits and pieces back", and using the
chats to climb learning curves and to brainstorm ("'what are my
options' was my favorite prompt from this era"). None of that is
visible in the repository: commit messages are one-line subjects,
there are no AI trailers before April 2026, and the conversations
lived in other tools. The series does not mine them, and treats this
era as the owner's own account. It matters to the story because the person who later
directed the agents had spent two years reading this codebase with an
AI explaining it to him, which is a different starting point from
either a C++ expert or a newcomer.

## 3.4 The hinge: April to August 2026

Three dates mark the turn.

- **2025-12-25.** The owner adds a 2.2 KB "Repository Guidelines"
  file named `AGENTS.md` to the C++ repository, the same day he
  replaces the crash reporter. It is a conventional contributor
  guide. The Rust entry file that later replaced it at the root swap
  has a separate lineage, born in the playground on 2026-08-24 at
  1.9 KB and revised 41 times there before it moved.
- **2026-04-11.** Eight commits in one day carry the trailer of
  Claude Opus 4.6: a version bump, a Qt workflow update, missing item
  fields. This is the first trace of Claude Code in the history, and
  then nothing with a trailer for three months. A one-line
  `CLAUDE.md` pointing at `AGENTS.md` lands on 2026-06-14.
- **2026-07-07.** The take-off. From this day the commit rate goes
  from five a week to seventy-seven, and 78% of July's 357 commits
  carry a trailer, mostly Claude Fable 5 with Claude Opus 4.8 beside
  it. The work is still C++: a "design cleanup" plan, characterisation
  tests that pin the app's existing behaviour, layering and threading
  refactors, a set of network redesign specifications, and spikes with
  the QCoro coroutine library. The C++ tree grows from 22,000 to
  54,000 lines in this month, most of it tests and scaffolding.

Report 4 treats the decision that follows as the first turning point,
and report 5 reconstructs it from the commits, since no transcript of
it survives here. What the history shows plainly is this: on
2026-08-09 the first Rust file enters the repository, in
`spikes/rate-limit-core`, a rate-limiter core written against the
network redesign's numbered claims, and a week later
`spikes/rust-playground` begins. The Rust rewrite was never announced
as such in a commit; it started as a spike to test one component and
did not stop. The owner's own account of the trigger: he had been
"toying with the idea in stages", and the agentic rewrite "took off"
when an outside contributor reached out after forking the app to add
agentic hooks, the pull request of early August that report 4
describes.

## 3.5 The Rust spikes: August to October 2026

Sixty-four days, 1,287 commits, all under `spikes/` while the C++ app
remained the repository's nominal content. The project's own ledger,
`HISTORY.md`, records this era as slices, each a closed body of work
with a commit range:

| Slice | Span | Commits | Days |
| --- | --- | --- | --- |
| Network cleanup, packages N0 to N6: strict parsing, the gate, the dispatcher, stress tests | 2026-08-21 to 08-22 | 20 | 0.6 |
| The ladder: ten live runs against the real API, the daemon halts rather than floods | 2026-08-22 to 08-27 | about 80 in the window | 5 |
| Testing notes: the register walk and three experiments | 2026-08-24 | 26 that day | 1 |
| The refresh slice: policy, plan, apply, characters, legible output | 2026-08-31 to 09-02 | 57 | 1.8 |
| The pricing slice: render-first pricing | 2026-09-03 to 09-07 | 103 | 4.3 |
| The daemon split: `acqd`, the protocol and client crates, the world | 2026-09-10 to 09-11 | 85 | 1.5 |
| The item search slice: the crate, the language, the derivation, the answer | 2026-09-19 to 10-10 | 295 | 20.5 |
| The relicense audit: MIT OR Apache-2.0, nothing transcribed | 2026-10-11 | an audit directory | 1 |

Read down the column and a rhythm appears. A slice of one to four
days, closed with a record; a pause; the next. The search slice is
the exception at three weeks, and it is also the first slice with a
committed brief, a research "working directory" with its own README,
and a seat (a blind agent sat at the tool). The lead model changes
on 2026-09-01, when Claude Fable 5.1 replaces Fable 5 in the
trailers, and from September on 97% of commits carry a trailer.

By the end of the era the tree held three things at once: the whole
C++ app (54,000 lines), a Rust workspace of 136,000 lines under
`spikes/`, and 2.75 MB of markdown, more bytes than the C++ source.
On 2026-10-11 the relicense audit cleared the Rust tree of any
transcription from the GPL C++ code, a commit-msg hook started
writing Claude's trailer as `Assisted-by:`, and the tag
`playground-end` marked the last commit of the old layout.

## 3.6 The root swap and the alpha: 2026-10-12 to 10-19

At 07:31 on 2026-10-12, nine and a half hours after `playground-end`,
one commit made the Rust workspace the repository and sent the C++
app to its own repository, `acquisition-qt`. Markdown fell from 133
files to 62 as the spike's notes were cut to what the root needed.

The following week is the densest in the history before the roadmap:
the rehome, a whole-CLI seat and its fix round; the three-platform
slice with first contacts on Windows and Linux (the run ledger has
the rows); the trade import slice, 158 commits in three days, in
which a trade-site URL became a third spelling of a search query;
then the release slice: pricing on the MCP server, the README, the
security and contributing policies, a final seat, and the tag
`v1.0.0-alpha.1` on 2026-10-19 at 18:54. Seventy-one days after the
first Rust file.

The owner's note at the top of the README is worth reading in full.
It calls the release experimental, says the documentation is mainly
for agents, and estimates several months before it competes with the
original app. The alpha had no external user then and has none known
now.

## 3.7 After the alpha: the roadmap week

On the day of the tag a Fable session ran a seven-audit assessment of
the project and wrote `STATE-2026-10-19.md`. The owner accepted its
recommendations whole, including a loopback web UI as the GUI's first
shape, and the project reorganised itself around a roadmap with
phases, briefs and an external reviewer. The week after holds:

- **Phase 0 and phase 1** (closed 2026-10-20): the week's fixes, then
  the governance reset, in which parked ideas became GitHub issues,
  per-slice records became a checklist and a one-line history, the
  notes directory went, and one byte budget replaced seven.
- **Phase 2** (2a to 2f): one account-error vocabulary across CLI and
  MCP; the mock serving the owner's real items; the function layer;
  review rounds under Codex.
- **The GitHub repository recreated** on 2026-10-22 with a clean
  history; issues kept their numbers.
- **4a and 4b** (closed 2026-10-25): `acq web`, the loopback renderer,
  ruled "a renderer, not an app".
- **The second assessment** (2026-10-26), a blank-slate read that
  found the system sound and the prose heavy, and the **honesty
  slice** that closed the same day: four rulings (C144 to C147) under
  which the daemon reports no success it did not earn and a count
  says what it counts.

In these eight days the commit rate reached 190 per active week, the
rulings register grew by 26 to 131, and 35 briefs and 10 review files
came into being. Report 4 takes up what this week decided; report 6
takes up what it cost.

## 3.8 What a commit became

The history is not only faster after July 2026; its unit changed.

| Month | Commits | Median message size | 90th percentile | Share touching only markdown |
| --- | --- | --- | --- | --- |
| 2026-06 | 8 | 90 bytes | 259 bytes | 0% |
| 2026-07 | 357 | 712 bytes | 1.6 KB | 34% |
| 2026-08 | 489 | 542 bytes | 1.4 KB | 48% |
| 2026-09 | 697 | 1.1 KB | 2.7 KB | 52% |
| 2026-10 | 808 | 1.3 KB | 2.8 KB | 36% |

A commit message in the owner's C++ years was a subject line. By
October a median message was a page, the largest 8 KB, and 2.74 MB
of message text had accumulated since June, as much as all the
markdown in the tree. The project's own rules explain why: a finding
is recorded "in the test named for the fault, or the commit message",
and the story of a step is its commits' messages. The commit log is a
primary document, read back by later sessions, not a changelog.

The second column to read is the markdown share. Between a third and
a half of commits in every AI month touched no code at all. The
2026-10-26 assessment put the post-alpha figure at 73%; the history
track found that number counts commits touching *any* markdown. The
markdown-only share after the alpha is 43%. The corrected figure is
still high, and report 6 discusses what it means; the correction is
here because the series promised to check the claims that matter.

## 3.9 Who the agents were

Six model names appear in trailers, and the lead changed three times
in four months.

| Model | Commits | First | Last |
| --- | --- | --- | --- |
| Claude Opus 4.6 | 8 | 2026-04-11 | 2026-04-11 |
| Claude Fable 5 | 520 | 2026-07-07 | 2026-10-24 |
| Claude Opus 4.8 | 73 | 2026-07-11 | 2026-08-15 |
| Claude Opus 5 | 23 | 2026-07-26 | 2026-08-24 |
| Claude Fable 5.1 | 911 | 2026-09-01 | 2026-10-26 |
| Claude Opus 5.5 | 542 | 2026-10-04 | 2026-10-26 |

Codex, OpenAI's coding agent, appears in 108 commit messages from
2026-08-09 on, always as a reviewer, never as an author. The owner's
rule from 2026-10-19 was to "stick to Opus 5.5 as much as possible,
with Fable called in for complex analysis, synthesis, and design",
and the October trailers show the shift: Opus 5.5 carries 542
commits against Fable 5.1's 254 that month. Report 5 describes the
roles; here the point is only that "the AI" in this story is several
models, changed deliberately, with a second vendor watching.

A caution the history track insisted on: a missing trailer proves
nothing. 235 non-merge commits since April carry none, 153 of them in
August, and the cause was not established.

## 3.10 Gaps in the record

- **The transcripts.** Claude Code keeps a transcript per session.
  On the machine this series was written on, they begin on
  2026-08-10 and are continuous only from 2026-09-21. Of the 2,371
  commits since April, 1,378 fall on days with no transcript here.
  The commits themselves say where most of those sessions were: 513
  of them carry a trailer linking a claude.ai/code session, 80
  distinct ones between 2026-08-08 and 2026-09-14, so that stretch
  ran in Claude Code's cloud sessions rather than on this machine.
  The owner confirms they were cloud sessions and that they may also
  be linked to his desktop computer at home; he also worked from a
  second computer for part of August and was travelling for the
  Eastern-time stretch in September and most of October. Report 5 reconstructs the missing window from the
  commits and says so wherever it does.
- **GitHub.** The repository was recreated on 2026-10-22, so issue,
  workflow and release history on GitHub starts that day. Older
  pull requests belong to the C++ app's repository.
- **The 2024 era.** The owner's AI-assisted learning of C++ lived in
  chat tools outside this repository and is told from his account.
- **Planning before the first commit.** A slice's span in git starts
  at its first commit; the design conversation before it is in the
  transcripts where they survive and nowhere otherwise.

## Reproducing the figures

```sh
git rev-list --count HEAD                                     # 3,840
git log --since=2026-07-01 --format=%h | wc -l                # 2,351
git log --reverse --diff-filter=A --format='%h %ad %s' --date=short -- '*.rs' | head -1   # first Rust file
git log --grep='^Co-Authored-By: Claude' --format=%h | wc -l  # 1,398
git log --grep='^Assisted-by:' --format=%h | wc -l            # 678
git log --reverse --format='%h %ad' --date=short -- AGENTS.md | head -1   # 2025-12-25
git log --no-merges --format='%H' --name-only v1.0.0-alpha.1..HEAD        # markdown-only share, counted per commit
```
