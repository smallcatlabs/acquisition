# What an agent reads before it edits

Written by Fable on 2026-11-26, at `a47b7550`. The owner asked for a
step back from the software to the rules, the process and the
documents around it, taken from the seat of the agent that writes and
uses them: what would cut the reading, the count of rulings and the
instructions, and make the work easier for the next agent.

This page changes nothing in the tree. It says what was found, the
frame I would use, and six changes. Its reader is the session that
makes them, after the owner has answered. Seven readers ran, six Opus
and the harness's guide; their reports are whole in `reports/`. An
Opus skeptic then read the first draft to break it; its answer is
whole in `dissent.md`, and "After the skeptic" says what it changed.
A number is mine where it stands beside its command; otherwise it is a
reader's and its report is named.

Paths are from the repository's root. A *builder* is a session
launched with a brief or an issue to change product code. An *orderer*
is the session that writes the brief, launches the builder, orders the
review and closes the step.

## The short of it

1. **The tree gives a builder two reading paths, and they disagree.**
   `AGENTS.md` says to read the area's rulings file, its section of
   `REVIEW-CHECKLIST.md` and the module doc, and names the pack.
   `roadmap/README.md` says to read the road, `decisions/process.md`
   and the brief, and names no area file. Builders follow the road's
   path.
2. **Reading by area is aimed badly.** On six closed steps, the
   rulings the instructions would deliver summed to 442, and the
   changes cited 48 of them.
3. **What the builds needed and were not sent to was never the
   registry.** It was a neighbouring module's limits, a test harness's
   mechanics, a second render site, an outside tool's semantics.
4. **Builders reach rulings through the code and the brief.** Their
   searches for a ruling's id went into `crates/` about six times as
   often as into `decisions/`. The code cites 167 of the 200 ids.
5. **No document tells a builder to check its diff against the
   reviewer's eleven fault shapes.** Most review findings carry one.
6. **The registry and the checklist have no drain.** Ids C1 to C200
   are all present. 70 of the checklist's 77 rows name a holder and
   are still rows.
7. **The registry should not be purged.** For most entries the reason
   is written at the entry and not at the code.

What this audit did not show: that reading a ruling fails to keep it.
The first draft claimed so and the skeptic was right to stop it.

The token saving in all of this is small. A builder's context at its
first edit has a median of 180k tokens, and the governance documents,
with `AGENTS.md`, are about 15k of it. The case is that there would be
one reading path, said once, that a builder can follow and an orderer
can check.

## The frame: a trigger and a lifetime

Every standing sentence has a trigger, the event that causes it to be
read, and a lifetime, the event that removes it.

| Home | Trigger | Lifetime | What the readers found |
| --- | --- | --- | --- |
| `AGENTS.md` | every session | a cap of 18 cross-cutting rulings, where a new one moves another out | the one zero-sum list |
| A brief | its step | its step | named the rulings that mattered; every review judged against it |
| A skill | its occasion | its procedure | loaded when needed, cold-review by 20 builders and session-close by 14 |
| A doc comment or a test | opening the code, or a failure | the code | where builders meet a ruling's id |
| A registry entry | "before touching an area" | none | 11 of 40 builders read the area file whole; none retired |
| A checklist row | "before touching an area" | none | 9 of 40 builders read part of it; 3 of 138 reviews mention it |
| `decisions/process.md` | every session's start | none | read by 36 of 40 builders; a reader judged 70% of it for another occasion |

The homes with a narrow trigger and a lifetime are the ones nobody has
had to audit. The three at the foot have a trigger as wide as an area
or a session, and nothing that removes a sentence. The changes narrow
those triggers. They move little text and add no ruling.

Three working rules follow.

- **A ruling is a lookup, read where it binds.** It is read when the
  code it binds is opened, when a brief names it, or when a test that
  carries its id fails.
- **When a machine holds a rule, the machine says it.** A hook prints
  its refusal. A paragraph read at every start that describes the hook
  is a copy.
- **One home, and the others point.** The texts found to disagree are
  each a paraphrase of another text's grant.

## What the system does well

Kept, and nothing below weakens it.

- **Faults become structure.** 38 of 45 sampled findings were answered
  by a test, a type or a check, and none by a new ruling.
- **The cold review finds real faults**: 123 serious ones in 138
  reviews in 17 days.
- **The brief is the strongest instruction.** It is scoped to one
  step, and it is what reviews judge against. Its "Find first" section
  is the one existing home for what point 3 says builds lacked.
- **Procedure is loaded at its occasion**, through skills.
- **The close commit is the ledger, and the road holds only state.**

## The evidence

| Measure | Number | What it does not show | Source |
| --- | --- | --- | --- |
| Standing governance text | 226 KB on 2026-10-20, 319 KB now; 120 entries, then 200 | one commit in between cut more than it added | mine: `git ls-tree -r -l <commit> -- decisions AGENTS.md REVIEW-CHECKLIST.md roadmap/README.md .claude/skills TESTING-NOTES.md GLOSSARY.md` |
| What an entry is made of | headline 17%, body 47%, reason 19%, holder list 17% of 127 KB | | mine: `scripts/parts.py` |
| Entries that name a holder | 167 of 200; of the 33 that name none, 16 are process and 7 cross-cutting | a named holder may hold only part: 15 of 30 sampled | mine: `scripts/parts.py`; `reports/registry-sample.md` |
| The reason is also at the code | 4 of 30 sampled | only doc comments that cite the id were read | `reports/registry-sample.md` |
| Prescribed reading on six steps | 51 to 151 KB with briefs and skills; 442 entries delivered, 48 cited | 442 sums six steps, so the 42 always-read entries count six times; a ruling followed without its id cited is missed | `reports/reading-precision.md` |
| What 40 builders read before a first edit | `process.md` 36, the brief 32, the road 29, the area file whole 11, part of the checklist 9, the pack 2 | reading inside a launch prompt or by a subagent | `reports/what-builders-read.md` |
| What launch prompts name | a median of 10 paths; `AGENTS.md`, which is already loaded, in 31 of 40 | | `reports/what-builders-read.md` |
| Where builders search for an id | `crates/` 319, `decisions/` 55, of 400 targets over 373 searches | the two counts do not reconcile | `reports/what-builders-read.md` |
| Serious findings, by what they broke | a ruling or a checklist row 9 to 13, the brief 9, nothing written 11, the method's own tools, tests and documents 16 | whether the builder had read what it broke | `reports/findings-against-rules.md`, n = 45 |
| Reviews that mention a shape | 99 of 138; a ruling's id, 3 | reviewers are handed the shapes, which tilts what they tag | `reports/findings-against-rules.md` |
| Process rulings, by holder | a machine 7, partly 3, discipline 14 | read from the checkers' source, not run | `reports/standing-instructions.md` |
| Instructions stated in two or more places | at least 25; six pairs of texts disagree | the reader stopped at 25 | `reports/standing-instructions.md` |

## The changes

Every one is a proposal. "The owner's part" says why none is taken on
a session's own word.

### 1. One reading path, said once

- **What changes.** The sentence in `AGENTS.md`, "The workspace", and
  "Starting a session" in `roadmap/README.md` become one text with one
  home.
  - A builder reads its brief or issue and what that names, the first
    section of `decisions/process.md` (change 4), and the module doc.
    It runs `python3 -I tools/bothy.py pack --path` on the files it
    will change and the tests beside them, and reads the rulings those
    cite.
  - An orderer reads the road, `decisions/process.md` and the area's
    headlines, opens what bears, and names in the brief's "Inputs"
    each ruling the builder must hold. The template already asks for
    that. The orderer's care goes to "Find first": the commands that
    turn up the neighbouring code.
  - A launch prompt names the brief. The brief names the rest.
- **What goes.** The area file as a builder's reading, the checklist
  section, the road for a builder, and the second path.
- **What it loses.** A headline can hide a qualification that the
  entry states. An earlier skeptic said so of this proposal and asked
  that it be decided as a change to how work is done, with a review.
  On the six steps, a pack over every changed file with its tests hid
  no cited ruling, and a pack over the product files alone hid five.
  That pack was run on today's tree, and a new file has no tests yet,
  so the brief's naming carries the weight.
- **What must be done with it.** A ruling no file cites cannot be
  found by a pack. Nine product ids are cited by nothing under
  `crates/`: C15, C20, C73, C110, C125, C154, C155, C191, C193 (my
  loop over `git grep -l -w`). Each gets a citation in the module doc
  of the code it binds, or already stands in `AGENTS.md`.
- **How to tell.** The sitting of `audits/governance-2026-11/seat/`,
  which issue 220 schedules after 2026-11-30: bytes read before a
  first edit, and its stops. The guard is serious findings per first
  build review (`audits/meta-2026-11/scripts/step/agg.py`), which must
  not rise. The count of rulings broken, 9 to 13 of 45, is too coarse
  to show a change.
- **Undone by** reverting two paragraphs.

### 2. The builder checks its diff against the shapes

- **What changes.** One line in `roadmap/briefs/TEMPLATE.md`, under
  "Rules": before the commit, read your diff against the eleven shapes
  in `REVIEW-PROMPT.md`. The shapes are 1.5 KB and stay where they
  are.
- **Why.** A builder meets that file today only as a name in the
  command that launches the reviewer. The frontends checklist repeats
  three of the shapes; no other area's does.
- **What it loses.** It is a bet. A builder may write to the list, and
  no count here can tell fewer faults from better-hidden ones. The
  reviewer still reads the diff cold.
- **How to tell.** The same guard as change 1.
- **Undone by** deleting the line.

### 3. The checklist takes no new row

- **What changes.** `REVIEW-CHECKLIST.md` leaves the builder's path
  and takes no new row. A fault that comes back gets its test
  strengthened, which the checklist's own header already says. The 7
  rows that name no holder are each put where they bind, as an issue
  or a line in the area's file.
- **What this amends.** C182 gives a row one way out: a commit naming
  what holds it now. This proposes a second, standing as written, for
  rows whose holders nobody has checked. One such retirement was
  itself a review finding, which is why I would not retire 70 at a
  stroke.
- **What it loses.** The review prompt still sends the reviewer to the
  area's rows, so 70 rows stay a reviewer's reading with holders
  unverified. A new shape would go to the review prompt, which has no
  lifetime either.
- **The alternative.** Leave the checklist as it is and only drop it
  from the builder's path, which change 1 does anyway.

### 4. `decisions/process.md` by occasion

An earlier review proposed moving process lore into skills and got no
answer (`reports/prior-step-backs.md`). This is a narrower form.

- **What changes.**
  - The rulings on who decides come first, as the section every
    session reads: C170, C171, C172, C179, C181, C183 and C185, about
    5 KB of 15.
  - The seven rulings a machine holds (C184, C186, C196 to C200) go
    last, under a heading that says to read them when the hook or the
    stage stops you. Their text and ids stay.
  - Each fact keeps one home. The file's own head says the mechanics
    are in the skills and the template. A ruling keeps its property,
    its count and its reason; the skill keeps the commands and points
    at the ruling for the rest. Today `cold-review` restates C176's
    counts and `session-close` restates C178, C179 and C184.
- **What it loses.** A skill read alone no longer carries the count,
  so its reader follows one pointer.
- **Cost.** One documents step. It moves prose across files, so it
  takes a cold review.

### 5. Corrections owed whatever else is decided

Each is two texts that disagree today.

- `AGENTS.md`'s hard rule lists "process, scope" among the owner's
  classes; C170 does not, and says a session writes and amends the
  registry.
- `tools/bothy.py`'s docstring, rules 6 and 9, says a new id and a
  change to an entry's words are the owner's ruling; C170 and C192 say
  otherwise.
- `TEMPLATE.md` lets a concern commit after its targeted tests, with
  one full gate before the last commit; `AGENTS.md` says a commit is
  gated on the gate's exit status. C171 can be read either way.
- `AGENTS.md`, "Procedures", states the cold review's trigger more
  narrowly than C175.
- `GLOSSARY.md` says new text should not use "door"; 214 added lines
  have since.
- `search/README.md` calls itself the only place process lives, and
  the workspace table sends agents to it first.

### 6. The pack serves the path, and less is minted

- **The pack.** A small tool step with an issue. `pack --path` takes
  one file, so eight files of one step print 117 KB, each pack
  carrying the area's headlines again; how much is repetition was not
  measured. It would take several files and print each ruling once. It
  refuses `tools/arch-check.sh` because no area governs it.
- **Intake.** One sentence for C182: a check's reason is the comment
  on the check, and it takes no entry. An earlier review judged 7 of
  the then 15 newest entries to restate a check, and C196 to C200 have
  come since. This adds to what C182 means, so it is the owner's.

## Not proposed, and why

- **Moving entries into doc comments, or numbering only law.** The
  reason lives at the entry, saved reviews cite the ids, and an
  earlier skeptic's objections stand. C192 sheds mechanism as entries
  are amended.
- **Deriving the holder lists.** 156 of 167 could be derived from
  tests and tools that cite the id (`scripts/derive.py`). `bothy
  check` already keeps the lists true, and the change is larger than
  it earns now.
- **A byte budget, a new check, a new ruling.** Refused before.
- **A rule file the harness loads when a matching file is read, or an
  edit hook that injects the pack** (`reports/harness-rules-and-hooks.md`).
  It would make the trigger automatic. Its behaviour in subagents and
  headless runs is not documented, and it is one harness's feature.
  Worth one trial if builders still do not run the pack after change
  1.
- **Cutting skills, the brief or the review.**

## The owner's part

One question comes before the rest, in plain words: **who owns the
rules about how the work is run?** `AGENTS.md` says process rulings
are his. C170, as amended at his word on 2026-11-25, lists his classes
as the invariants, the boundaries, the gate's stages and anything that
touches his store or the live API, and gives a session the registry's
wording.

- *If process is his:* all six changes wait on his word, and so does
  every later change to a skill's ruling.
- *If only the classes C170 lists are his:* a session makes changes 2,
  3 and 5 and the wording of 4, and tells him. Change 1 stays his,
  because it changes what a builder knows before it edits. Change 6's
  sentence stays his, because it changes what a ruling means.
- *Recommended:* the second, with one addition: who decides, and how
  many reviews a thing gets, stay his by name. Those are the two
  places a session's convenience and his interest can part.
- *What he would lose:* a read of each change to how steps run. By two
  earlier audits' counts, 12 to 14 of his 23 ruling issues were about
  the rulebook's own text (`audits/meta-2026-11/README.md`).

**Answered, 2026-11-26:** "I agree with your recommendation." C170
and `AGENTS.md`'s hard rule are amended in the commit that records
this, and now state one list. "Scope" stood in `AGENTS.md` and not in
C170; it is kept as his in both, since nothing here examined it. The
first of the corrections in change 5 is thereby made.

Then change 1, put the same way.

## The baseline, taken before any change

Taken on 2026-11-26 at `b2cd41a8`, at the owner's word, so that the
changes can be read against it. Each number has the command that
makes it again.

| Number | Before | How it is made again |
| --- | --- | --- |
| The sitting at the instructions | per task, in KB, always loaded + required + ambiguity: 22 + 116 + 13, 22 + ~84 + 7, 22 + 63.7 + 2.5, 22 + ~47 + ~4; eleven stops | `seat/README.md`; the run is `audits/governance-2026-11/seat/BRIEF.md`'s with `seat/PROMPT.md` |
| Serious findings per first build review | 84 in 86 reviews, 0.98 each; 38 of 86 clean; 10 with a P1 | `scripts/guard.py 2026-11-10`, after `scripts/findings/reviews.py` |
| Reviews per step | mean 1.59; 51 steps took one, 20 two, 14 three, 1 four | the same |
| What 40 builders read before a first edit | `process.md` 36, the brief 32, the road 29, the area file whole 11, the pack 2; a median of 39.6 KB and 180k tokens of context | `scripts/transcripts/`, `extract.py` then `scope.py`, `analyse.py` and `report.py`; `reports/what-builders-read.md` |
| The owner's ruling issues | 23 since 2026-11-09: 18 in the first week, 3 in the second, 2 in the third | `gh issue list --label trigger:ruling --state all --limit 100 --json number,title,createdAt` |

What the baseline says that the audit did not.

- **The largest reading cost the sitting met is none of the six
  changes.** The road's first "Next" item points into two audit pages,
  58 KB together, and the runner read 47 KB to name the next step,
  against 20.9 KB on 2026-11-10. C180 says a session takes the first
  item of "Next" without a planning session.
- **The owner's queue has already fallen.** Eighteen of his 23 ruling
  issues came in the week of the governance rebuild. His answer on who
  owns process saves about one question a week at today's rate.
- **The sitting's task 1 cannot be compared with the earlier run's.**
  Issue 200 is closed and the fallback is a larger issue. The
  after-run uses the same prompt, so it meets the same issue while
  issue 212 is open.
- **The guard is coarse.** At 0.98 serious findings per first review
  over 86 steps, ten or fifteen later builds can show a halving or a
  doubling and little between.

## After the skeptic

| The draft said | What it met | Now |
| --- | --- | --- |
| Reading a rule did not keep it: the 7 own-area breaches were all in the file the builder was told to read | Nobody matched those findings to what their builders read, and 28 of 40 builders did not read that file. Not reading fits as well | Withdrawn, and said so in the short of it |
| The rule about reading is not the reading | `AGENTS.md` does not say "whole", 9 builders did read their checklist section, and the road's path names no area file. Builders followed one of two paths | Two paths that disagree |
| 442 rulings delivered, 11% engaged | A sum over six steps; "engaged" means cited by id | Kept as a count of citations, with what it misses |
| No document a builder reads points at the shapes | The cold-review skill names the file, and frontends rows repeat three | No document tells a builder to check against them |
| Freezing the checklist costs nothing and claims nothing | 70 rows stay live for the reviewer with holders unchecked, and it is a second way out beside C182's | Stated as an amendment, with its loss and an alternative |
| C192 puts a process ruling's how in its skill | C192 is about doc comments on code, and C176's counts are its property | The counts stay in the ruling; the skill points |
| A builder needs five sentences of process | The readers' own list is twelve | A builder reads the who-decides section |
| Two things here are the owner's | Under `AGENTS.md` every process change is his, and one sentence changes what C182 means | One question first, then the rest by its answer |
| "The rules only grow", "nothing drains" | One step-back's commit deleted more than it added | Narrowed to the registry and the checklist |
| Half of recent entries restate a check | An earlier review's judgment of an earlier fifteen | Attributed |

The skeptic would drop change 4 if one had to go, and would keep
point 3 if only one finding could stand. I have kept change 4 in the
narrower form above, and moved point 3 into what the orderer does.

## What was not checked

- Whether reading the area file lowers review rounds. The two groups
  differ, a median of 1 round against 2, and 17 of the 28 who did not
  read it were on the GUI's form steps. No effect is claimed.
- The class of each finding and the kind of each entry are one
  reader's judgment. n = 45 gives about 14 points either way.
- Most "would fail the gate" verdicts rest on test names. Three tests
  were read.
- Whether the checklist rows' named holders hold them.
- Private memory. It has grown from 35 KB to 57 KB since issue 229
  sorted it. It is the orderer's cost and no builder's, and was not
  re-sorted here.
- The trade tooling, the ledgers, `TESTING-NOTES.md` and the skills'
  own text, beyond their size and who reads them.
- The skeptic was of the author's model family, and the revision has
  had no second read.

## How this was made

I read `decisions/process.md`, the road, the two skills a close uses,
the template, the review prompt and the previous two audits' pages
myself, and counted the registry with the two scripts in `scripts/`.
Six Opus readers then took one question each, read-only, told to name
the command behind every number: the ledger of earlier step-backs, the
precision of the prescribed reading on six closed steps, review
findings against written rules, what build sessions read (from the
session transcripts on the owner's Mac, quoting nothing he typed), a
seeded sample of 30 registry entries, and a map of the standing
instructions outside the registry. The harness's guide answered one
question about rule files and hooks. An Opus skeptic read the draft.
The reports are saved from the transcripts by
`audits/meta-2026-11/scripts/save_report.py`, with the scratch
directory's path shortened. The readers' own scripts are under
`scripts/`, one directory to a reader, with the checkout's path made
relative: they run from the repository's root and write their working
files to `/tmp/reading-2026-11/` or beside themselves, where
`scripts/.gitignore` keeps them out of the tree. They were not run
again after that edit, but for `findings/reviews.py`.
