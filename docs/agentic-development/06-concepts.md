# 6. Concepts and paradigm shifts

The ideas this project arrived at, each stated as a claim, with the
artefact that embodies it, the fault that motivated it, what it cost,
and what a large-organisation engineer would recognise it as. Some
are new. Most are old ideas that changed shape when the workers
became stateless models. The last section is the counter-argument the
project makes against itself, which is the most instructive part.

## 6.1 The repository is the only memory

**Claim.** A session starts from nothing. Whatever the previous
session knew that is not in a file is gone. So the repository must
hold not only the code but the reasons, the rules, the open
questions, the procedures, and the owner's decisions, in a form a
reader with no context can use.

**Artefact.** The entry file, the rulings registry, the briefs, the
review files, the ledgers, and the commit messages, which became
documents (a median of 1.3 KB by October, 2.74 MB in total since
June).

**Origin.** A commit of 2026-08-12 names it: two incidents that day
"shared a cause: the rule ... existed only as one agent's habit, not
as repo text." And the rate-limiter spike's entry file: "The repo is
the only shared channel. Sessions differ, some agents keep private
memory, external reviewers keep none, and neither can read the
other's."

**Cost.** This is the root of the weight problem in section 6.10.
Every stage of work produces a document because the next stage cannot
ask anyone.

**Recognisable as** the written culture of a distributed team taken
to its limit: the "six-pager" and the design doc, but for every unit
of work, and read by someone who has literally never seen the
codebase.

## 6.2 Context is a scarce engineering resource

**Claim.** What a session reads at its start is paid for in tokens
and attention on every task. The always-loaded document must
therefore be small, and size pressure on it is a signal that
something landed in the wrong place.

**Artefact.** The byte budget on the entry file (8,000 bytes in
September, 15,277 by late October), the "read on demand" rule for
the area files and the glossary, the registry linter's "pack"
command that prints what to read before touching a given file, and
the 800-byte limit on a ruling.

**Origin.** On 2026-09-02 a note measured the required reading at 246
KB and 2,806 lines, with the context file doubling in two days
"every commit added". The fix was ids, area files and a budget
enforced by a script.

**Cost.** A hard budget turned into byte-shaving: a session trimmed a
semicolon from the owner's wording to fit, and he spent a commit
restoring it. Since 2026-10-26 the budget prints a note and never
fails.

**Recognisable as** the onboarding problem. Every organisation has a
document a new engineer must read first; few measure it, and none
has to pay for it per task.

## 6.3 Structure over discipline

**Claim.** A rule that can be broken silently is not a rule. It
becomes a dependency edge, a lint, a test, or a refusal in the
binary. The remaining rules, the ones that cannot be mechanised, are
few and are stated as properties.

**Artefact.** The architecture check that reads dependency edges from
cargo metadata and refuses a frontend that opens a store itself; the
daemon that refuses to start in real mode without a terminal; the
clippy lints that deny `unwrap` in the crates where a panic would
loop; the five invariants in the entry file, each "a property, never
a mechanism".

**Origin.** Invariant 1 was rewritten within hours of being written,
from "the daemon's single job queue and token bucket" to a
mechanism-free property. The principle is stated in the entry file:
"A rule that can be broken silently becomes a dependency edge, a lint
or a test." The mock-world rule is the clearest case: it began as a
flag agents had to remember, and became a binary that cannot be made
to send from an agent's shell.

**Cost.** Low. This is the part of the system the 2026-10-26
assessment lists first under "what to protect".

**Recognisable as** "make the right thing the easy thing" and
"shift left", applied to workers who will forget the rule tomorrow.
The difference from a human team is only that the forgetting is
certain.

## 6.4 Who tests the tests: breakers

**Claim.** A checker that has never been seen to fail proves nothing.
Every checker carries a suite of deliberately broken inputs, and the
checker must refuse each one by name.

**Artefact.** `tools/*-breakers.sh`, run in CI as their own job.

**Origin.** The dependency guard went through six review rounds that
"found the same shape each time: a reader whose empty, partial or
renamed answer satisfied every rule". The founding suite's first run
found two of its own cases wrong. The design rule: "a refusal is
evidence only when it names the property, because twice a malformed
breaker printed a refusal that was not the property."

**Cost.** One more suite per checker. The gate script itself has
none yet, and that is an open issue.

**Recognisable as** mutation testing for process checks. Most
organisations have CI; few have tests that the CI would fail if it
should.

## 6.5 Rulings and findings: a decision-rights matrix for models

**Claim.** Two kinds of knowledge need two homes. A *ruling* is the
owner's decision on a boundary, recorded before the code that
depends on it, with a *Why* and the test that *pins* it. A *finding*
is what building or running taught, recorded after, in the test named
for the fault or in the commit message. Agent-facing design in
between (a verb, a flag, the words of an answer) is the session's to
decide, recorded in the commit that builds it.

**Artefact.** `decisions/<area>.md`, 131 rulings with ids; the hard
rule "Rulings first, findings after"; the delegated class from
2026-10-26.

**Origin.** The founding file of 2026-08-16 said the opposite,
"decisions get recorded here after spikes teach us, not before". An
audit on 2026-10-13 found the two rules contradicting each other and
the owner ruled "rulings first, findings after". Thirteen days later,
after an assessment measured the cost ("a flag change needs an
800-byte entry with Why, Pinned and glosses before the code"), the
delegated class moved back.

**Cost.** The approval queue. The owner became the bottleneck for
decisions he did not need to make, and his attention went to wording.

**Recognisable as** the decision-rights matrix (who decides what),
with the twist that the delegates have no judgement that persists
between sessions, so the matrix itself has to be in the file they
read first.

## 6.6 Property, not mechanism; tests pin boundaries

**Claim.** Tests pin behaviour at the two boundaries, the wire and
the protocol, never the mechanism in between. The GGG-side contract
is the send journal, one JSON line per real request, checked at
teardown by oracles "written from the rules rather than from the
code". Wording a user sees is a golden that names the facts a reader
must find, "so a blind regeneration fails". A behaviour is pinned
once, at the lowest crate that holds it.

**Artefact.** The journal, the oracles, the goldens under
`ACQ_UPDATE_FIXTURES=1`, the one test harness, the architecture
check's test-layout section.

**Origin.** The rate-limiter spike's post-mortem of 2026-08-09: "green
was the only gate and green cannot see vacuity, mirror-oracles, or
cross-slice interactions." And the finding that 72 integration test
binaries linked the same crates, consolidated to nine on 2026-10-21.

**Cost.** About 58% of the Rust is test code. The known limit is
that a golden of the binary's own words pins a wrong sentence rather
than catching it; the 2026-10-26 assessment verified five such.

**Recognisable as** contract testing and characterisation testing,
with the extra discipline that the contract surfaces are named and
counted.

## 6.7 The agent is the user: the seat

**Claim.** If the primary consumer of a command-line tool is an
agent, then the tool's own words (help text, error messages, the
shape of an answer) are its entire user interface, and the way to
test them is to give a model nothing else and watch where it stops.

**Artefact.** The seat skill; the seat records with their fate
tables; the rulings born from seats: no id in user-facing text, "a
count says what it counts", a plan spends each request once, the MCP
server's default answer fits every harness's cut.

**Origin.** The owner on 2026-09-25: "I'm probably never going to
write queries manually, so 'using the seat' in practice means sitting
down with an agent and talking to it." The first seat was sat the
next day. Blindness turned out to be the hard property: a subagent
"starts in the repository with its instructions, the memory index
and the git log", so a seat now runs in a world outside the
repository with locked-down permissions, and is never continued
after its first run because "it has learned the tool".

**Cost.** A dollar and a half to three and a half per run at the
release, and a fix round after each. The assessment calls the seats
"the only instrument that measures the agent-facing surface" and
notes the last one predates the latest wording changes.

**Recognisable as** usability testing, with a model as the
participant, and the product decision that the machine is the
customer. For a frontend engineer this is the most transferable idea
in the series: the help text is the UI, and it can be tested the way
a UI is tested.

## 6.8 Evidence before ceremony; ceremony retired by evidence

**Claim.** A procedure becomes a skill only after it has run twice
and repeated a trap. A check or a rule is retired when what it
guarded is proven. A ruling lands before the code, but a mechanism
lands only after a fault.

**Artefact.** The six skills, each citing its two runs; the ladder's
hypothesis-first paperwork retired two days after the limiter was
proven; the four memory notes of gate lore retired when the gate
became one script.

**Origin.** The principles P1 and P6 in the entry file, and the
commit of 2026-08-30 that retired the ladder's "paperwork half" and
kept "the part with teeth: the rails, which are code".

**Cost.** The project's own verdict is that the second half failed:
"P6 says ceremony is retired by evidence; almost nothing has been"
(2026-10-19), and "The immune system has no apoptosis" (2026-10-26).
Section 6.10 takes this up.

**Recognisable as** "you aren't gonna need it" for process, and the
observation that every organisation finds it easier to add a check
than to remove one.

## 6.9 Independence is engineered: second models and blind readers

**Claim.** A review by the author's own model, with the author's own
context, finds what the author would find. Independence has to be
constructed: a different vendor's model, a reader that cannot see
the author's explanation, proposers in separate worktrees with
anonymised copies.

**Artefact.** Codex as the reviewer of every committed step, never
seeing the worker's report; the two blind search proposals of
2026-09-17 ("proposers blind by construction"); the design brief of
2026-08-09 from a chat "with no access to this codebase ... That
independence is the point"; the five blind Sonnet subagents that
answered real questions with only the help text.

**Origin.** The owner's practice from August, and the memory note that
records it as a rule: a second-model study for hard design choices.

**Cost.** A second subscription and a relay step until 2026-10-19,
when sessions began launching the reviewer themselves. And the
reviewer finds faults in the process as well as the code, which is
by design: the review prompt's last question is what it would change
about the brief, the plan and the prompt.

**Recognisable as** blind review and red-teaming, and the ordinary
rule that the reviewer is not the author, applied where the "author"
and the "reviewer" would otherwise be literally the same model.

## 6.10 The counter-argument: the words are the mass

The project is unusual in having assessed itself twice in a week and
written down what it found. The second assessment, by a Fable
session asked for a blank-slate read on 2026-10-26, is the
counter-argument to everything above, and it belongs in this report
in its own words.

- **"The immune system has no apoptosis."** Every fault becomes a
  named test, a checklist row and, if it recurs, a review shape.
  Nothing retires. The review prompt carries nine shapes and about
  fifty checklist rows; the suite is 58% of the Rust; "the gate does
  not feel it; the reader does."
- **"The owner's authority runs through words, and the words are the
  mass."** Rulings come first for every class of change, so a flag
  change needs an 800-byte entry before the code. The owner's
  leverage is then spent on wording. The compressed register "is
  memorable in rulings and commit messages and a tax everywhere
  else"; it "leaks into the product": a tool description of 3,864
  bytes in one paragraph, a tool list of 41 KB per session, and a
  ruling that had to be written because four seats stopped on six
  words.
- **"Mechanisms taken for properties."** A build-order rule that
  appears in five places and could be retired by fixing the feature
  graph; about twenty rulings held by discipline that "the field
  makes look held"; "the gate decides" while the Windows gate had
  been red on every push for a week.
- **The measure.** The honesty slice "spent about 226 KB of briefs,
  reviews and messages on 7.4k lines of Rust". Since the alpha, 43%
  of commits touched only markdown and 72% touched some (the
  assessment said 73%; report 3 corrects the wording).

The owner's response was not to defend the system but to cut it: the
four process changes of the same day removed the bookkeeping commits,
delegated agent-facing design, demoted the budgets to notes, and
added a glossary. Whether that holds is not yet in the record.

What an outsider should take from this is not that the system
failed. The core it protects is sound, the product works, and the
assessment lists what to keep. It is that a process built for
stateless workers has a specific failure mode that human teams do not
have in the same form: because nothing is remembered unless written,
everything gets written, and because every writer is careful, nothing
gets unwritten. The owner's own phrase for the earlier period, given
for this series, is that the project was "rapidly accumulating
process cruft and I was along for the ride." The remedy the project
found is the one it already had as a principle and had not applied:
retire by evidence, and measure the prose as carefully as the code.

## 6.11 Summary table

| Concept | One line | Transfers to a human team? |
| --- | --- | --- |
| the repository is the only memory | if it is not in a file, the next session does not know it | partly: distributed teams already live this way |
| context is a scarce resource | budget the always-loaded document; route everything else to its home | yes, as onboarding cost, but nobody measures it |
| structure over discipline | a rule that can be broken silently becomes a check | yes, and it is already best practice |
| breakers | every checker has inputs it must refuse by name | yes; rare in practice |
| rulings and findings | decide boundaries before the code, record lessons after, delegate the middle | yes, as decision rights; the matrix must be where workers read first |
| tests pin boundaries | the wire and the protocol are the contract; the mechanism is free | yes |
| the agent is the user | the tool's words are the UI; test them with a blind model | yes, for any CLI or API an agent will drive |
| evidence before ceremony | mechanisms after faults; retire what is proven | yes, and it is the one this project found hardest |
| independence is engineered | a second vendor, a blind reader, anonymised proposals | yes, for review of any kind |
| the words are the mass | a written culture for stateless workers accretes; measure it | the warning |
