# 5. The operating model

How one owner and a cast of AI agents actually work together in this
repository, the mechanisms that hold the arrangement, where each came
from, and how the way of working changed over four months. This is
the longest report in the series because it is the one Aaron asked
for. Every mechanism is cited to the file or commit that defines it;
anything inferred says so.

## 5.1 The shape in one paragraph

One person owns the project and writes essentially no code. He rules
on boundaries, runs anything that touches the real game API, reads
every outside review before a fix is built, handles every credential,
and says "go" or "no". Claude Code sessions write the code and most
of the prose: of 1,700 commits between mid-August and late October,
about 1,050 carry a Claude Fable trailer and 550 a Claude Opus
trailer. An OpenAI Codex agent, driven by OpenAI's frontier models (one
commit names `gpt-6.1-sol`; the owner names Astra), reviews every
committed step, read-only. Around that division sits a written process: one
auto-loaded entry file under a byte budget, a registry of numbered
rulings, a checklist of faults that came back, briefs from a
template, a quality gate script, checkers with "breaker" suites that
prove the checkers fire, skills for procedures that recurred, and
memory notes that mostly record the owner's corrections. The newest
layer, from the last week of October, is an attempt to remove process
weight rather than add it.

A reader from a large organisation can hold one picture through the
rest of this report: an engineering organisation of one human, with
the roles of tech lead, engineer, reviewer, tester and researcher
filled by models, and the artefacts that a team of humans would carry
in their heads written down because the workers have no memory
between sessions.

## 5.2 The cast

| Role | May | May not | Where it says so |
| --- | --- | --- | --- |
| **The owner** | rule on invariants, boundaries, process, scope, his real store and the real API; run every live run from a terminal; capture the trade site in a browser; triage at each phase boundary; read every review's findings before a fix is built; push; handle every credential | (he is the authority) | `AGENTS.md`; the roadmap's "Roles"; memory notes |
| **A Fable session** (Claude Fable 5, then 5.1) | open and close a phase; design the steps that need a design; write assessments; read workers' reports; keep the roadmap true; orchestrate Opus workers | build (since 2026-10-19); keep building after it has assessed (since 2026-10-26) | the roadmap's "Roles"; two memory notes |
| **An Opus session** (Claude Opus 5.5) | run one step from its brief: build, gate, commit, launch the Codex review of its own commit, report in six fixed sections | push without the owner's word; record a boundary ruling; touch another worker's scratch or processes; start a real-mode daemon | the roadmap; the brief template |
| **An Opus worker in a worktree** | the same, in its own git worktree and scratch subdirectory, in parallel with siblings | share scratch, `git stash` or processes with a sibling | the roadmap; three memory notes about collisions |
| **Codex, the reviewer** | read one commit or range against the brief, the checklist and nine named "shapes of fault"; report findings with severity, failing input and smallest fix; critique the brief, the plan and the prompt | build, run or edit anything; see the worker's own report | `roadmap/briefs/REVIEW-PROMPT.md` |
| **A Sonnet subagent** | basic searches; some audit reading; seat runs | rulings | memory note on model roles |
| **The seat** (a blind agent) | drive the released binaries from a world outside the repository, reading only its brief and the README's tour, through one logging script; write a report and answer a debrief | read the repository; run the tool by path; be a subagent (a subagent inherits instructions, memory and git log, so it is not blind) | `.claude/skills/seat/SKILL.md` |
| **Astra, through Codex** | before 2026-10-19, a second frontier model: OpenAI's Astra (the owner: "somewhat comparable to Fable"), driven through Codex; it authored a competing search proposal and review on its own branches (merged 2026-09-17) and checked ruling packets | | commit messages name it; the owner identified it for this series |

Two facts about the cast that the record forces on the narrative.
First, "Opus builds, Fable designs" is a ruling of 2026-10-19, not a
founding principle. Before it, Fable wrote most of the code: 825
Fable trailers against 119 Opus in the spike era; after it, 102
against 271. Second, the owner's division of labour between models
was reached by use, not by benchmark, and the ruling's wording
("stick to Opus 5.5 as much as possible, with Fable called in for
complex analysis, synthesis, and design") is about cost and fit, not
capability alone.

## 5.3 The lifecycle of a step

The roadmap states the rhythm in six words: *brief, build, gate,
commit, review, close*. In full, with the artefact each stage leaves:

1. **Evidence.** An assessment, a seat, a live run, a review or a
   user report surfaces work. It becomes a GitHub issue labelled with
   its area and its trigger, or a line in a dated state file.
2. **Ordering.** The Fable session proposes where the work goes in
   the roadmap; the owner rules.
3. **A ruling packet**, if the step touches a boundary: numbered
   questions, each with a recommendation, sent to the owner before
   any edit. His answer is quoted verbatim in the commit that records
   the ruling ("I have reviewed the questions and approve all 4
   recommendations").
4. **A design brief**, if the step needs a design, itself reviewed by
   Codex and ruled by the owner.
5. **A build brief** from the template: Goal, Read, Input, Do, Done
   when, Not in scope, Rules, Report.
6. **The session starts** by reading the roadmap, then the brief,
   then what the brief names, "and nothing else unless the work turns
   it up".
7. **The build**, in a mock world, one commit per concern, the story
   in each message.
8. **The gate** before every commit, with its exit-status table
   pasted into the commit message.
9. **The Codex review** of the step's commits, launched by the
   committing session, findings saved verbatim under
   `roadmap/reviews/`.
10. **The worker's report** in six sections, the sixth "Raised for
    review". It is withheld from the reviewer by design: the commit
    is the evidence and the report its author's account.
11. **The owner reads** the findings and the proposed fixes and gives
    his word ("Fix all four findings as proposed, then close p0").
12. **The fix round.** Each finding is pinned as a failing test first
    and fixed by one rule, never per case; a fault that came back
    becomes a checklist row.
13. **The close.** A commit that checks the brief's done-when lines
    against the tree, answers every raised item (taken, filed, or not
    taken and why), lists the design decisions the step made, and
    writes the ledger line. "A step that has not earned its close
    slips rather than ships."
14. **The phase close** folds the ledger lines into one history line.
15. **Memory.** What the session learned about working with the owner
    goes into a memory note outside the repository.

Anyone who has shipped inside a large company will see the familiar
skeleton: ticket, design review, implementation, CI, code review,
fix, done. The differences are in the details. The reviewer is a
different vendor's model and never sees the author's explanation.
The owner's approvals are quoted verbatim into history. The report's
last section exists to let the worker complain about the process.
And every one of these stages produces a document, because the next
session starts from nothing.

## 5.4 The mechanisms

Grouped by what they do. Each entry gives what the mechanism is, when
it was born, what motivated it and what it replaced.

### The documents

**The entry file and its byte budget.** `AGENTS.md` is "the one file
every session loads"; `CLAUDE.md` is one line pointing at it. The
Rust entry file was born in the playground on 2026-08-24 at 1,916
bytes, "so a fresh session lands on the spike docs". A byte budget on
it arrived on 2026-09-02 at 8,000 bytes, with the check's comment
explaining why: growth past the budget "is the signal that a
narrative landed where a ruling, a test or a commit message belongs".
The budget rose to 9,000, then 14,000 when the file absorbed the
registry's root on 2026-10-20, then to 15,277 under a temporary
permission. Since 2026-10-26 an overrun prints a note and never
fails the gate, because "a budget that failed had entries edited to
the byte"; the assessment that day found the owner spending a commit
to restore a semicolon a session had trimmed to fit. The lesson for
an outsider is that the context window is a scarce engineering
resource, that a budget on the always-loaded document is the right
instrument, and that a hard budget turns into byte-shaving at the
margin.

**Rulings with ids, Why and Pinned.** A ruling is one bullet: an id,
a title, the ruling in plain declaratives, one *Why* sentence, and
*Pinned*, naming the test that holds it or saying "nothing yet".
Numbered decisions began in a single `CONTEXT.md` on 2026-08-16; the
registry form with an 800-byte limit per entry dates from
2026-09-02, and the split into eight area files the same day.
"Rulings first, findings after" is a hard rule: a decision is
recorded before the code that depends on it, and tests cite it by
id. The weakness the project names itself: about twenty rulings say
"Pinned: nothing yet" and are "held by discipline, and the field
makes them look held".

**The glossary.** Born 2026-10-26, after the assessment counted about
thirty undefined working nouns and four meanings of "door". The
compressed register, it said, "is memorable in rulings and commit
messages and a tax everywhere else". Each sense of "door" got its own
word.

**The review checklist.** "The shapes of fault that came back until
they were named", one table per area; a shape that comes back again
gets its holding test strengthened, never a second row. Born
2026-10-20, replacing the findings tables of each closed slice
record. Rows read like "A claim wider than its evidence" and "A fix
staging the nearest failure, not the named one".

**Briefs and their template.** Born 2026-10-19 with the roadmap,
replacing per-track brief files and, before those, numbered
brainstorming notes. The template carries each phase's lesson as a
rule with its cause: "phase 0's briefs cited a ruling as built that
was parked, named a test that did not exist, and carried a count
wider than true." The recurrent fault is literals written from
memory; on 2026-10-25 a worker disproved five numbers in two briefs,
and the rule became: run every grep the brief cites and paste the
hits.

### The checks

**The gate.** Ten stages: build, tests under nextest, clippy with
warnings as errors, format check, whitespace check, docs-check,
arch-check, trade-check, the registry linter, strict rustdoc; then a
rebuild, because tests overwrite one binary and never the daemon, so
a daemon started after tests would be a different build. The script
`tools/gate.sh` dates only from 2026-10-26; before it the same list
and its exit-status lore lived in five documents and four memory
notes ("grep -qv is no guard"). The script's own first review caught
it printing green for a stage filter that ran nothing.

**Breakers.** A breaker suite deliberately breaks what a checker
guards and requires the checker to refuse it by name. The founding
suite (2026-09-10) came after the dependency guard had been through
six review rounds that "found the same shape each time: a reader
whose empty, partial or renamed answer satisfied every rule". The
suite's first run found two of its own cases wrong. This is the
project's answer to the question every CI owner eventually faces:
who tests the tests.

**arch-check.** Dependency edges read from cargo metadata (the
protocol crate links serde only; the store links no workspace crate;
nothing but the daemon names the daemon), and a grep that refuses a
frontend which opens a store, picks an account or compiles a plan
itself. Born with the daemon split in September, moved to its own
script on 2026-10-20. The principle behind it is in the entry file:
"A rule that can be broken silently becomes a dependency edge, a lint
or a test."

**docs-check and the registry linter.** Seven checks on the
documents: the budget, the registry's grammar, that cited commits
exist, that a backticked identifier in a control document still
exists in the code ("a parallel description that has rotted"), the
README's form, the hook setting, and that every golden is named by a
test. The registry linter `bothy.py` came from a sibling project of
the owner's and also prints, for any file, what to read before
touching it. The assessment's caveat: "the checkers verify pointers,
not truth".

**nextest and process isolation.** A process per test, no retries:
"a retry turns a flaky limiter or gate test green, the opposite of
'the gate decides'. A test that fails once is a finding." Adopted
2026-10-21, when 72 integration test binaries became about nine.

### The safety rules

**The mock-world rule.** "Never start a real-mode daemon from an
agent shell." The first entry file had the opposite default: nothing
talked to GGG unless a flag was set. On 2026-09-12 ruling C88
flipped it: the real API is the default, the mock is opted into, and
the binary refuses to spawn a real-mode daemon without a terminal,
which an agent's shell does not have. The guard moved from "agents
must remember a flag" to "the binary cannot be made to send from an
agent shell". A slip on 2026-10-17, when an agent ran a bare command
with an offline flag and it read the owner's account index before
refusing, tightened the rule to every command, offline or not.

**The live-run skill and the ledger.** "A live run is the owner's,
from a terminal, under this procedure." First contact with a new
endpoint runs with rails on (a tripwire that halts on the first bad
response, a ceiling of three sends), reads the send journal first,
and records the observed policy as a ledger row and a numbered
ground-truth claim; after a violation, wait six minutes, "never
reset-and-retry". The ledger's rows "stay as written: a later fact is
a new row or a claim, never an edit to an old row's evidence". The
discipline was born in the ladder of ten live runs in late August,
about 1,450 sends with zero rate-limit violations, and its
hypothesis-first paperwork was retired two days after the limiter
was proven, an early instance of the project's rule that ceremony is
retired by evidence.

### The instruments

**The seat.** "A seat asks what the tool's own words are worth to
someone who has nothing else." A headless agent, started in a
directory outside the repository with locked-down permissions, reads
only a brief and the README's tour, drives the release binaries
through one logging script, writes a report, and is then resumed for
a debrief ("which answer would you retract?", "which claims rest on
your own knowledge of the game rather than on the tool?"). The
session then checks every reported fault against the call log before
believing it, fixes each in its own commit held by a test, and reruns
with a fresh agent, never the same one, because "it has learned the
tool". First sat on the search tools in September, then on the whole
CLI on 2026-10-13 (thirteen faults), then on the trade import and the
release (six runs, Opus and Sonnet, a dollar and a half to three and
a half each). Blindness turned out to be the hard property: a probe
in October showed a subagent "starts in the repository with its
instructions, the memory index and the git log", and a later note
found even a headless run in a worktree loads the memory index. The
seat produced product rules directly: no id in user-facing text
("every seat met ids it could not look up"), and "a count says what
it counts", because four seats stopped on six words.

**The Codex loop.** One prompt and one command: Codex reads a commit
in a read-only sandbox against the prompt, the checklist and nine
shapes, and ends with what it would change about the brief, the plan
and the prompt. Outside reviews ran from mid-October with the owner
pasting findings by hand; on 2026-10-19 he asked whether a session
could launch Codex itself, and from then on the committing session
does. The first use, on the roadmap itself, produced eleven findings,
nine taken, and corrected the review mechanism: the prompt had put
the project's own fault shapes before plain defects, Codex pointed
out that this would steer a reviewer away from deadline and
cancellation bugs, and the order was flipped.

**Upward feedback.** Every brief's report and the review prompt end
with the same question: what got in your way, what do you think is
wrong with this way of working? The session that reads the answer
must answer each item in the close commit. The owner called this a
"hard-earned lesson", and the reason given is precise: "an unanswered
channel goes quiet within two rounds." Kept out of the gate and the
tests, which should stay silent or fail.

### Coordination and record

**Worktrees and parallel workers.** Independent steps may run as Opus
workers in parallel, each in its own worktree and scratch
subdirectory; otherwise steps run one after another, because a build
under a live daemon or a concurrent gate produces a binary that is
not the one the code describes. The traps were found by hitting them:
two workers overwrote each other's scratch (2026-10-19), one surveyor
killed another's daemons believing them stray (2026-10-21), and
worktrees share the stash. Each became a rule in the template.

**The memory files.** Claude Code's per-project memory, outside the
repository: an index loaded each session and one note per lesson,
41 of them, dated 2026-10-13 to 2026-10-26, mostly the owner's
corrections quoted verbatim. Some lessons graduate into the tree and
the note is retired. The roadmap lists the open question that the
index is a second always-loaded document outside the budget.

**The commit-msg hook and attribution.** A hook rewrites Claude's
`Co-Authored-By` trailer to `Assisted-by`, so 88% of commits since
April name their model. The contributing policy puts it plainly: the
human who submits a change "is its author, and you answer for every
line of it, however it was written".

**Generated references as goldens.** The CLI and MCP references are
generated from the binaries' own help text and a test fails when the
committed copy drifts. Born 2026-09-07 at the owner's request, so an
agent "doesn't need to run and build the executable just to see how
to use it", replacing a hand-written guide "that agents edited twice
within a week, seeing a command table and completing it". The known
limit: a golden of the binary's words pins a wrong sentence rather
than catching it.

**No id in user-facing text.** Help, refusals and reasons say the
rule in words, never its id, because a seat cannot look an id up and
reads it as a defect. Its internal twin: a brief, report or commit
message names an id's rule in words the first time it uses it.

## 5.5 How it evolved: the entry file in six stages

The entry file is the clearest record of the way of working, because
every session reads it and every change to it is a commit.

| Stage | Date | Bytes | What it told a session |
| --- | --- | --- | --- |
| 0. C++ guidelines | 2026-08-20 | 3,413 | a project map, a build recipe, "run with a throwaway data dir" |
| 1. The spike's landing page | 2026-08-24 | 1,916 | a reading order of four documents; the owner holds the boundaries, agents own internals; nothing talks to GGG without a flag; a four-line gate |
| 2. The router | 2026-09-02 | 3,858 | read conditionally, "only when the work touches it"; three procedures moved into skill files; "every fact has one source, and a copy elsewhere is a parallel description that rots" |
| 3. Before the root swap | 2026-10-11 | 7,133 | the real provider is the default; area decision files; five skills; a thirteen-row routing table; the build-order rule |
| 4. The alpha | 2026-10-19 | 8,374 | the crate table; six skills including the seat; "a harness that does not list these skills still needs them read" |
| 5. The governance reset | 2026-10-20 | 13,774 | the file becomes the registry's root: what Acquisition is, five invariants, hard rules, sixteen cross-cutting rulings in full, three principles, "Where things live"; the context file, slice records and notes deleted the same day |
| 6. Today | 2026-10-26 | 15,231 | as 5, with the gate as one script, the glossary, and agent-facing design recorded after the code rather than before |

Three sentences survived from stage 1 to today nearly verbatim: the
owner holds the boundaries, a live run is a human's, and the gate
decides. One rule inverted: the opt-in flag for the real API became
"the real provider is the default" with a structural refusal. Several
things grew and went: a routing table and a read-before table of
slice records, replaced by a shorter "Where things live" when the
records themselves were deleted. The arc is from a landing page, to a
router that sends a session elsewhere, to a constitution that holds
the rules themselves.

The second record of the evolution is the owner's own prompts, which
survive in the session transcripts from 2026-08-10. That narrative is
drawn from a digest of 114 sessions and is in section 5.7.

## 5.6 The owner's corrections

The memory notes are, in effect, the owner's running feedback to his
workforce. A selection, each with the rule it produced:

| Date | What happened | The rule it became |
| --- | --- | --- |
| 2026-10-13 | an agent asked the owner to decide an agent-facing design | decide and build it yourself; the CLI is "agent-friendly and agent-first, so your opinions about agent-facing features is more important than mine"; bring boundaries and the real store to him |
| 2026-10-14 | an agent asked for a secret | "I do every portal action and handle every credential myself. Never ask me for a secret's value." |
| 2026-10-14 | a deferral justified with an unchecked cost | before writing "rare" or "cheap", read the code path that bounds it; close a small gap rather than document it |
| 2026-10-16 | guessing what a trade-site clause means | "I prefer using sittings to answer questions so that we don't have to guess or over-generalize our design." |
| 2026-10-17 | a bare command read his real account index | every command runs in a mock world, offline flags included |
| 2026-10-17, again 10-21 and 10-25 | commit messages carried predicted byte counts, amended twice | measure in one call, read the number, write the message in the next |
| 2026-10-18 | seat faults inviting one patch each | "keep simplicity, usability, and elegance as guiding lights. Patching edge cases and adding mechanisms is likely to be seductive"; a change must remove agent work or reuse a shape the tool has |
| 2026-10-18 | a seat found more than a round should build | triage "now versus after the alpha" and present it before building |
| 2026-10-19 | six review rounds where every follow-up finding was on the agent's own fixes | three checks before a commit; "think about deeper patterns, even if it confirms we just need to address these findings individually" |
| 2026-10-19 | the owner's "hard-earned lesson" | every brief and review asks what got in the way, and the reader answers |
| 2026-10-21 | the entry-file budget blocked a design | temporary permission to raise it, "not a ruling"; "do not raise it again without asking" |
| 2026-10-22 | — | commit at logical points and always before he looks, "so his input shows up in history as a before and after" |
| 2026-10-26 | a full-context assessing session about to build | "keep this session to brainstorming, analysis, synthesis, and setup, as the context is filling up"; an assessing session writes briefs and stops |

The themes are few and they repeat: measure before you state; one
rule rather than a patch per case; simplicity over mechanism; triage
now against later; ask the source rather than guess; and the
coordination traps of parallel workers. A reader who has run a team
will recognise every one of them as the feedback a careful lead gives
a capable but overconfident new hire. The difference is that here it
had to be written down, because the hire forgets everything between
sessions, and the writing is what made it a system.

## 5.7 Before the transcripts: the way of working, reconstructed

No transcript on this machine covers July to mid-September 2026, when
1,392 commits landed. What follows is reconstructed from those
commits by a research pass that labelled every claim verified or
hypothesis; the hypotheses are marked here too.

**What is verified.** From the first week of July the method was
spec, then implement, then review, as separate sessions, and the
repository wrote that rule down itself: "Do not edit code from a
review-only session", and in the rate-limiter spike's entry file,
"The repo is the only shared channel. Sessions differ, some agents
keep private memory, external reviewers keep none, and neither can
read the other's." Hand-off briefs from 2026-07-20 were written "so a
fresh agent with no prior context can produce a commit-by-commit
plan", and the next day's were "model-agnostic and memory-independent
so a cloud agent or other model can run them cold". A commit of
2026-08-12 explains why every rule became a file: two incidents that
day "shared a cause: the rule ... existed only as one agent's habit,
not as repo text." On 2026-08-09, the day the first Rust file landed,
one design day ran from a charter "prepared in Tom's session with
Claude", through an independent brief from a chat session "with no
access to this codebase ... That independence is the point", a Codex
plan review, four external review rounds, a frozen design, the code,
an audit and a post-mortem whose rule was "A slice is not complete at
green."

From 2026-08-31 the unit of design was the packet: a framing note
"written before the session" holding the owner's stances, paired
proposals by Fable and by an OpenAI model, an audit, a response, and
a final packet of numbered questions the owner ruled in one line
each, harvested into the context file the same evening. The refresh
loop was built the next day in about nine sessions, one per step, the
owner reviewing between them. Two days later a note measured the
required reading at 246 KB and the registry with ids was born.

**The rhythm.** Sundays and Saturdays were the heaviest days, then
Friday; almost nothing happened between midnight and six; the peak
hour was nine in the evening. The research pass read this as the
rhythm of someone with a day job, and the owner confirms it: "Several
hours a day on average, and sometimes a partial or full weekend",
beside a day job, with travel in September and October.

**The strongest hypotheses, each with its test.** First, that most of
the 287 untrailered commits in the window are Codex's, acting as
implementer as well as reviewer: they share a style (one-line
subjects, "reconcile accepted review", a `Validation:` paragraph), the
kickoff files are addressed to "Claude Code, Codex, or other", and
the owner wrote on 2026-09-14 that "I've been using codex for reviews
and this has worked well". Asked for this series, the owner agrees:
"Codex was probably writing the commits. I rarely wrote commits
myself once I started using the agentic development tools." Codex's
own session logs would settle it beyond "probably".
Second, that the owner framed work in conversation and steered with
short verdicts on numbered questions, which the agents then wrote
into the repository before work began: "agreed", "I agree with you on
1", "agree to flip the default"; a note of 2026-09-14 says "Until now
the plan lived in a conversation; a Codex session could not have read
it." The surviving transcripts from 2026-09-21 on can confirm or
refute this, and section 5.8 reports what they show. Third, that the
second computer was a Windows PC: a Codex review of 2026-08-09 cites
a Windows path, a Claude session built and ran the Windows binary on
2026-08-22, and seventeen C++ commits that month are signed with a
key not used today. The owner confirms the Eastern-time stretch was
travel, not a third machine, and that the cloud sessions "might also
be linked to my desktop computer at home".

**What the commits cannot tell.** The prompts themselves, beyond the
short quotes the agents kept; what was rejected; who typed the
untrailered commits; the chat sessions with no codebase that produced
the founding invariants; how reviews were relayed between tools; time
spent reading rather than committing; cost. The 80 cloud session ids
in the trailers are the first place a second round would look.

## 5.8 The prompting arc

What the owner actually typed, and how it changed. This section rests
on digests of the 114 surviving transcripts, one per session, each
written from the transcript with verbatim quotes and timestamps, then
five era narratives written from the digests. Every quotation below
is the owner's, exact, with its date; the session ids are in the
research files. The coverage is uneven: one day in August, then
continuous from 2026-09-21.

### One day in August: "plan, review, build, review, repeat"

The three sessions of 2026-08-10 are the only window into the spike
era, and they show the owner as an active co-designer. His opening
request named what he cared about and no method: "I'm looking not
only for bugs, but gaps, non-idiomatic or poorly design rust
patterns, inconsistencies with documentation, and opportunities to
simply what's been been". The audit found four real bugs and a
flagship property test that asserted in about 3% of cases, behind
green tests. His reaction was a question, not an order: "it seems
like the implementation was a bit shoddy. Have you run across any
clues as to how this might have happened?" And then he took the
process failure on himself: "I will change my process to review each
slice. I had thought they were simple enough to breeze through, but
it looks like I was wrong."

He described his own method that day in one sentence that holds for
the whole record: "You've lived through how I build best with AI,
plan, review, build, review, repeat. That's how I get things done."
He brought in a second model the same night ("I'm having another
frontier model review the repo in it's current state in case either
of us missed something") and acted as courier between them. He asked
for plain English when the jargon thickened ("i don't fully
understand the planning terms being thrown around"). He challenged
the design out of stated motive ("This is somewhat pride in the c++
implementation ... but I'm open to pushback"). And by evening he was
already questioning the weight of what he had built: "i'm dismayed
with how complex this spike has grown due to the way we've been
building it, but i'm committed to the process so we will see where it
lands."

The day's process births are the seeds of everything later: slices
end at the owner's review, not at green; a written review guide; a
findings register; an outside model at milestones; rulings
attributed by name and date; and the pattern of having the outgoing
session write the incoming session's brief ("summarize what's left in
a concise bullet list ... then give me a brief prompt to kick that
work off in a new session").

### Late September to the root swap: shrinking the reading, widening the delegation

The search slice opens with nine-word prompts ("Read and run step 5
of search/BUILD-PLAN.md", 2026-09-21) and the owner ferrying an
outside auditor's findings back and forth, with yields he recorded
per step: 6, 3, 0. He did no code review himself. His questions were
about process: fresh session or not, where findings go "without
creating process cruft", session versus subagent.

Two moves define the era. First, he loosened his own rules
repeatedly: "The goal is not to blindly follow rules. The goal is to
simplify documentation and process to make it easier for an agent to
find only what's needed for its work" (2026-09-22); "You are the
final arbiter, because you are the primary consumer and producer of
documentation" (2026-09-24); "don't necessarily constrain yourself to
the processes we have defined for ourselves if they are not serving
us" (2026-09-27). Second, he named the real user: "In a sense, this
is almost my seat, because the cli cumbersome to run by hand, so in
normal use I will ask an agent to drive it for me" (2026-09-23), and
later, "the MCP is meant as an agentic interface, not mine, so I
would rather let you decide on how to handle the questions"
(2026-10-10).

The documentation tax was the recurring complaint. The mandated
reading reached about 75,000 tokens; "I want to stop now and think
about what we can do to minimize or even reverse this content growth,
because we have about 4 more phases" (2026-09-24). He also protected
the work from the budget when it mattered: "Spending time trimming
text while we are still working on the code feels like wasted
attention. Please ignore the budget until these items are confirmed
resolved" (2026-09-25). By October he said plainly that the agents'
briefs had become "agentic shorthand that I do not understand well"
(2026-10-07).

The shape of an opening prompt changed in this era. It stopped being
"build step N" and became a gate template the model usually drafted
at the end of the previous session: stop for my read; launch nothing
before then; one blind look; the gate at the end. By 2026-10-10 the
delegation was open: "Use subagents at your discretion", "your call",
and "This seems like a long, complex plan. You have it in your head
better than I. How do you want to proceed?" He kept domain rulings,
reviewing the risky operation himself, pushing, and the wording of
the registry.

### The root swap to the alpha: opinion first, then a terse ruling

From 2026-10-12 the pattern settled: the model brings one to three
judgment calls, the owner rules in a word each ("1. confirmed 2. yes
... 3. ok"), then pastes the outside review verbatim with one framing
line, the model proposes a plan, "yes, go ahead". He asked for the
model's opinion before ruling ("What is your perspective on making
this a registry entry or not? I don't want to accumulate entries
unless there's a clear need and future benefit"), and said where he
had none: "i dont have a technical opinion on the plan question.
that's another feature where i'd trust you or another model"
(2026-10-14). He chose models for jobs by name: a Fable subagent for
a design question, an Opus agent to study which property tests had
earned their cost.

His corrections in this era were rarely in his own voice. They
arrived as pasted findings plus a design stance: "look for patterns
behind the patterns"; "Be careful here. This is likely a situation
where the quick fixes will add complexity and fragility"; "Do not
just add edge cases or new if-then-else logic unless there is a
coherent reason" (2026-10-16). When review rounds kept returning he
added a brake: "keep in mind the practical limits of remediation"
(2026-10-17). He was candid about his method: "I've gotten good
results with very basic prompt engineering combined with developing
my own judgement on how to direct my agents" (2026-10-18).

The outside review moved from an occasional relay to a fixed stop
after every commit, proposed by the model and adopted by him. He
delegated orchestration itself ("You are the writer and then the
orchestrator the plan describes; you never run a task yourself, and
no brief you write says what you expect to be found", 2026-10-17)
and, by the trade re-sit, the lead ("You are the person with the most
coherent view of the situation, so you can thoughtfully lead us
through this", 2026-10-18). What he never delegated: credentials
("Never ask me for a secret's value"), pushes and publishing, and the
calls that weigh schedule against design, such as choosing option A
over the model's preferred B on schedule grounds, and alpha over
beta: "since we don't have a gui yet, and the existing feature sets
have not been tested in the wild, i'm reluctant to call this a beta,
but i'd like to know what would have to change first" (2026-10-19).

### The roadmap's first week: the owner becomes the baton

The week after the alpha began with a long framing prompt
("Ruminate deeply") and ended with the owner asleep through an
unattended scan of over a hundred subagents ("I'm asleep: don't wait
for me or ask me anything; record blockers and move on",
2026-10-22). In between, his openers were one sentence, a document
and a stop line: "stop after each step's review is saved; do not
start a fix round" (2026-10-20); "Bring me the drafts and every
deletion list before anything is deleted; build nothing."

His approvals became ritual words that the models quoted back into
commit messages as authority: "launch", "go ahead", "yes to all",
"build it", "merge when green". He asked for simplification before
ruling ("Please restate the six rulings needed in simple sentences"),
and the models learned to end every report with numbered asks. He
cast the agents as stakeholders: "you and all the agents who have
built this system, which really isn't a sloppy mess at all, are
primary stakeholders" (2026-10-20); "you and other agents such as
Opus, Sonnect, and OpenAI models are the primary stakeholder for the
CLI, so you are better suited to understand the right way to address
the problem" (2026-10-21).

He kept review depth and cost as his own dial: "no further review";
a requested extra round; a waived one; and the late judgment that
the cloud ultra-reviews "haven't added much value". He kept every
irreversible act: the repository recreation, signing keys, pushes,
the real-store migration after the harness refused the model. And he
named his own limit without apology: "(i'm smart, but this work is
very technical and i'm unfamiliar with its inner working compared to
you)" (2026-10-23). The week's most consequential rule, the stopping
rule for review, came from his question "are we in the weeds".

### The second week: rulings in one sentence, work in the brief

The last era's openers refuse to anchor the model: "Don't worry
about a solution yet, focus on understanding the nature and scope of
the issue first. We will brainstorm solutions later" (2026-10-24);
"Could the design be simpler, more elegant, or more idiomatic with a
reframing?" (2026-10-25); and for the blank-slate assessment, "I also
don't know which aspects of this project might benefit from
attention, so I won't even mention any so as not to lead you
accidentally in any particular direction" (2026-10-26). He asked the
model to choose the work: "Ignore me. Ignore the league start and
the existing plan. Which issues or kinds of issues would you want to
tackle next as both a developer and user and primary stakeholder of
this repo?"

Once decided, rulings were one templated sentence, and he began
writing ruling text himself and asking for the model's view of it.
His one correction of method in the week was scoping: "please do the
work in series, not parallel. otherwise i like your approach
completely" (2026-10-26). He vetoed a new enforcement check on a
principle of his own: "if we set a clear pattern in the code, i
suspect future agents will follow that pattern, so we may not need
the technical creep of an enforcement check." He protected a
session's context as a resource: "keep this session to
brainstorming, analysis, synthesis, and setup, as the context is
filling up". And he said what the project was for: "It may be a
hobby project, but I want us both to be proud of how professionally
acquisition is managed, and I want us both to enjoy the process of
building as well as of using what we build."

By the last surviving session, 2026-10-27, the whole exchange was
one prompt pointing at a committed brief with a permission boundary
and a proof criterion ("You may push to main once the gate is green;
I want the Windows CI run as the proof"), forty-five minutes of
silence, and "how do you advise starting r2?"

### The arc in five lines

- **August:** co-designer and courier; "plan, review, build, review,
  repeat"; the review loop turned on itself the same day.
- **Late September:** nine-word prompts and relayed audits; the owner
  loosens his own rules and names the agent as the user.
- **Early October:** opinion first, then a one-word ruling; outside
  review after every commit; the model drafts the next prompt.
- **The roadmap's first week:** ritual approvals as authority; the
  owner as the baton between sessions; review depth his only dial.
- **The second week:** framing questions that refuse to lead; the
  model chooses the work; one sentence, a brief, and silence.

What stayed constant across all five is shorter: credentials,
pushes, the real store, schedule against design, and reading every
review before a fix. Those were his on the first day and on the last.

### Caveats

The digests were written by a model from extracted dialogue with
tool output removed, and a few extracts overlap or disagree on
details (the research files mark each case). The quotations are
copied from the transcripts and were not altered, including the
owner's typing as he typed it.

## 5.8 The translation table

For the reader coming from a large organisation, what each piece of
this operating model corresponds to, and what is different.

| Here | There | The difference |
| --- | --- | --- |
| the entry file | the onboarding doc, the team wiki's front page | read in full at the start of every unit of work; held to a budget because reading costs tokens |
| a ruling with Why and Pinned | an architecture decision record | mandatory before dependent code; cited by id from tests; linted |
| a brief | a ticket plus a design doc | the worker reads nothing else; done-when is checkable from the commit alone |
| the Codex review | code review by a peer | a different vendor's model; never sees the author's explanation; critiques the process in every review |
| the gate | CI | runs locally before every commit, with the table pasted into the message; "the gate decides" |
| breakers | tests of the CI itself | required for every checker, in CI |
| the review checklist | postmortem action items | one row per shape of fault that came back; strengthen the test, never add a second row |
| the seat | user acceptance testing, usability testing | the tester is a model with no context, and the tool's words are the only interface |
| the run ledger | the deploy log | one row per contact with the real API; rows are never edited |
| the roadmap's ledger | the sprint board | one line per closed step; "slips rather than ships" |
| memory notes | the lead's one-on-one notes | written by the worker, about the owner's corrections |
| "what got in the way" | the retrospective | asked of every worker and reviewer on every step, answered in writing |
| the mock world | the staging environment | the default for every agent, structurally enforced by the binary |
