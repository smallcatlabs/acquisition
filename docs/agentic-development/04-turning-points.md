# 4. Turning points

Fourteen decisions that changed the project's course between July and
October 2026, in date order. Each gives the situation before, the
decision and who made it, the evidence, the consequence visible later
in the tree, and what an engineer from a large organisation would
recognise it as. Quotations are exact and at most 25 words; hashes
resolve in the repository's history. The closing sections collect the
reversals, and the decisions agents made that the owner later
ratified or overruled, because those are where the division of
authority shows most clearly.

Three findings from this track reshape the story the series might
otherwise have told:

- **The rewrite was never decided as a rewrite.** The architecture
  decision record that was supposed to settle "rewrite or evolve"
  still read *Proposed* on the day the Rust workspace became the
  repository. The real decision was a shipping ruling two months in.
- **The working style went back and forth.** The founding rule was
  "decisions get recorded here after spikes teach us, not before". It
  became "rulings first" when an audit found the two rules
  contradicting each other, and thirteen days later part of it moved
  back.
- **The biggest refactor came from a build-hygiene question**, not a
  roadmap: 65 GB of object files, then a reviewer's remark.

## 4.1 The C++ cleanup that made a rewrite thinkable (2026-07-07)

**Before.** The owner had maintained the 2014 C++/Qt app since
October 2023. An earlier plan was to migrate its UI to QML.

**Decision.** An architecture decision record, written with Fable,
deferred the QML migration "in favor of an interior design cleanup
that keeps the Widgets UI" (`17310a90`). Owner and agent together.

**Evidence.** July 2026 is a burst of agent-assisted C++ work: seven
cleanup phases merged as pull requests in one week, an items-pipeline
redesign, and a network redesign whose specification went through six
review rounds and was "frozen" on 2026-07-19 (`63be1089`). Findings
were numbered F1 to F67 in a register.

**Consequence.** The side effect mattered more than the cleanup. The
work left the project's knowledge "outside the code, in citable
documents", above all the network ground truth: numbered claims (N1,
N2, ...) about how GGG's API limits requests. That file was one of
only two documents carried across the root swap in October and is
still cited by claim number from rulings and tests. The rewrite
inherited facts, not code.

**Recognisable as** the refactoring project that, by forcing the team
to write down what the legacy system actually does, quietly makes the
rewrite cheap.

## 4.2 "Rewrite or evolve" is opened and never closed (2026-08-08)

**Before.** An outside contributor proposed a local control plane that
would make the C++ app drivable by scripts and coding agents.

**Decision.** A second decision record, "Rewrite or Evolve the Core
and UI", status *Proposed* (`f9795ed1`), naming three options and a
branch discipline: "Nothing changes on master until the ADR is
accepted." Its drivers list "agent accessibility as a first-class
requirement rather than an afterthought".

**Evidence.** The record still read "Proposed (August 2026)" on the
day of the root swap. "Rust playground" titles survived until
2026-10-13, when "the playground's names" were retired (`25332463`).

**Consequence.** The product grew under a spike's name until a
shipping ruling on 2026-10-11 decided the matter in other words
(section 4.11). The owner's own account, given for this series:
once the record was created he "promptly proceeded to pretty much
ignore it", and he places it in "the era when the project was rapidly
accumulating process cruft and I was along for the ride."

**Recognisable as** the prototype that became production while the
architecture review that was supposed to approve it stayed in draft.

## 4.3 The rate-limiter spike, and a cap on review depth (2026-08-09 to 08-15)

**Before.** A frozen network specification. The question for any
rewrite was whether a rate limiter could be built that GGG's servers
would never punish.

**Decision.** A Rust spike: a limiter whose logic is pure functions
with no network code, wrapped in an async actor, because "users and
agents will want to inspect, reorder, and cancel pending refreshes"
(`ef47e658`). The spike drew review after review; by 2026-08-15 it
carried a machine-checked registry of 124 obligation clauses and
findings F1 to F25. Then the owner capped it, "in-session 2026-08-15,
on the analyst's proportionality analysis" (`fa879660`): remaining
reviews merged into one, under a materiality threshold.

**Evidence.** The spike's crate was never imported into the
playground. Its findings were transcribed into the ground truth, and
the playground's limiter was rewritten from those claims on
2026-08-20, "the spec is the test table ... each row citing
network-ground-truth.md by claim number" (`d7f3e9ff`).

**Consequence.** A pattern that held for the rest of the project:
spike code is disposable, spike knowledge goes into numbered claims.
And the first recorded instance of the owner stopping a review loop,
a move he repeats in October ("nothing is reviewed a third time
unless the owner asks").

**Recognisable as** a design review that had to be given an explicit
exit criterion before it would converge.

## 4.4 The playground is founded on invariants from an outside chat (2026-08-16)

**Before.** A merged spike and a ground-truth file; no product code in
Rust.

**Decision.** The owner's own commit, with no agent trailer, of a
context file whose opening says: "This list of invariants came from
an external chat session." Its working style: "decisions get recorded
here after spikes teach us, not before"; "Owner (Tom) holds the
boundaries ... Agents own internals behind those boundaries." A
working daemon and CLI followed the same morning (`463a5438`).

**Evidence.** The founding file has fourteen one-line decisions, and
most survive today nearly word for word as rulings: lazy spawn and
idle exit, requests as jobs, JSON lines over a socket, `--json` on
every command, Tokio and reqwest. The founding line "The CLI is
itself an API; makes MCP and agent use nearly free" is C11's *Why*
today. Within hours, invariant 1 was rewritten from a mechanism to a
property ("reword invariant 1 to be mechanism-free", `b82b12c7`): the
seed of the lasting rule that an invariant names a property, never a
mechanism.

**Consequence.** The entry file every session loads today descends
from this one page. Three of its lines were later reversed: Tauri
for the GUI, jobs in memory, decisions after code.

**Recognisable as** the one-page architecture vision that turns out to
be the most durable artefact the project ever writes.

## 4.5 The ladder: the daemon halts rather than floods (2026-08-22 to 08-30)

**Before.** A limiter that passed its tables against a mock. The
background incident was not part of the rewrite: in July 2024 the
C++ app had triggered a Cloudflare block with "over a thousand HEAD
requests in one minute" (ground truth N2), and GGG's support had
relayed that the Cloudflare layer's rules are "opaque even to GGG
support". The strategy that followed (N4) is to stay far inside the
boundary and never probe it.

**Decision.** Test against the real API by escalating rungs, each
with rails: a tripwire that halts every send after any 429, 401, 403
or 503, and a per-lifetime send ceiling (`d985dd72`). Designed by the
agent, every run executed by the owner.

**Evidence** (the run ledger, rows 2026-08-22 to 08-30). Rung 7b: the
limiter "held 14.75 s (period + 5 s bucket) before the 16th" send,
and the server's next answer showed the window had cleared. Rung 8, a
34-hour soak, was "stopped, not a pass" at its 200-send ceiling with
three 401s; the fix was "seen fixed live twice" in a re-soak that
passed 45 hours clean. Rung 10, a full-account pull: at send 245 of
332 GGG's origin answered 503. "Tripwire halted; 0 sends after it;
the remaining 82 children failed as 'halted by rails' without
sending." The ladder's verdict: confidence that the daemon "halts
rather than floods" was met on 2026-08-27.

**Consequence.** Each failure became a ruling: rung 10's 82 failed
children, which forced a full refetch, became C25 (a halt leaves jobs
waiting). On 2026-08-30 the owner retired the ladder's paperwork,
"a written hypothesis before any live run", keeping "the part with
teeth: the rails, which are code".

**Recognisable as** a staged production rollout with automatic
circuit breakers, whose postmortems each became a design rule, and
whose launch checklist was retired once the breakers were trusted.

## 4.6 Agents may talk to GGG, and plans become binding (2026-08-31)

**Before.** The founding file had deferred agent traffic, "blocked on
verifying GGG's policy stance". The playground fetched and stored but
had no notion of user intent or of reviewing work before it ran.

**Decision.** One day, one design session, one packet. It opens with
the owner's stance: "Traffic to GGG from any frontend is acceptable,
because the daemon regulates it." The session weighed four outside
notes (two by Fable, two by an OpenAI model through Codex), a Fable
proposal, an external audit and a response, merged into a packet the
owner ruled the same day: "everything accepted; D2a accepted as
amended by the owner".

**Evidence.** The packet's decisions are today's C38: a plan is "a
serializable, immutable authorization envelope", and applying one
"executes exactly the listed actions or a strict subset, never an
action that was not reviewed". The phrase "authorization envelope"
entered through the outside audit, committed by the owner himself.

**Consequence.** This is the product's architecture. Agent traffic is
allowed through the daemon as the single gate (C14); facts, intent,
derivations and effects each have one mutation path (C34); the
refresh loop is plan, quote, apply. Later rulings refine the envelope
rather than replace it.

**Recognisable as** an RFC review with an outside reviewer, ending in
the change-management model that Terraform users know: plan, review,
apply.

## 4.7 The registry is born, and the first prose diet (2026-09-02)

**Before.** After the first end-to-end runs, the context file had
doubled in two days. A note measured the required reading at 246 KB
and 2,806 lines, "every commit added".

**Decision.** Agent-proposed, with the owner's assent: the decisions
become a numbered registry, "C1–C63, one bullet each under an
800-byte limit", with a checker that refuses unknown ids
(`56bec954`), split the same day into cross-cutting rulings always
loaded and area files read on demand.

**Evidence.** The diagnosis applied the store's own rule to the
documents: "every fact has exactly one authoritative source. The
documents broke it."

**Consequence.** Nearly all the governance machinery descends from
this day: rule ids cited by tests, the registry linter, rulings in
module doc comments under their id. The pattern also repeats: the
same diagnosis, the weight is prose, returns on 2026-10-19 and again
on 2026-10-26, and the 800-byte limit is itself demoted to a note
that day.

**Recognisable as** the moment a wiki of design notes becomes a
numbered decision log with a linter, and starts accruing its own
maintenance cost.

## 4.8 "Remove the training wheels": the real API becomes the default (2026-09-07 to 09-12)

**Before.** The mock provider was the default; talking to GGG
required a flag, so every offline read of the owner's own data sat
behind one too.

**Decision.** The owner: "I think we are ready to remove the ACQ_GGG
training wheels." The agent wrote the deliberation and four
questions; C88 landed five days later and was proven live the same
day. Its *Why*: "one gate protects the relationship, not callers kept
away".

**Consequence.** Safety moved from a default to a discipline plus
structure: the binary refuses to start a real-mode daemon without a
terminal, which an agent's shell lacks, and the first hard rule in
the entry file says never to try. The 2026-10-26 assessment
criticised the downstream cost: a ten-line "dance" in the README for
running an agent against real data.

**Recognisable as** flipping a feature flag from opt-in to opt-out
once the safety net is trusted, then writing the runbook that keeps
engineers out of production.

## 4.9 The daemon split: "time to grow up" (2026-09-09 to 09-11)

**Before.** Every frontend binary could become the daemon by
re-executing itself. One crate held daemon, protocol, client and
shared types. On 2026-09-08 a status check typed in a second terminal
spawned a mock daemon on the default socket and blocked a live run.

**Decision.** It began as a build-hygiene question: a note measured
that the build stamp rebuilt the workspace on every git index write
and that the build directory held 600,127 object files, 65 GB. Then
Codex, reviewing, observed that embedding the daemon in every
frontend was "an implementation choice, not a ruling"; the author
agreed it "was the cheapest location answer". Put to the owner as a
plain question, his answer: "omg yes--embedding the daemon sounds
terrible compared to `acqd`. the path we took was an unplanned
shortcut. time to grow up."

**Evidence.** The slice closed in three days with review rounds
numbered to 27. A ruling was amended so that an observing command
never spawns or replaces a daemon, "because an observer that replaced
a daemon once cost a live run".

**Consequence.** Today's crate table and the dependency edges that
arch-check refuses; the "world" (one store, one daemon, one lock);
the handshake deadline.

**Recognisable as** extracting a service from a monolith, triggered
not by a roadmap but by a build-time investigation and a reviewer's
remark.

## 4.10 The agent becomes the user (2026-09-22 to 09-26)

**Before.** The search slice had been designed by two models
proposing blind, one of them OpenAI's through Codex, which is why
every brief became "a self-contained repository file". The open
question was whose use would validate the result.

**Decision.** First, five blind Sonnet subagents answered five real
questions over the owner's store with only the tool's help text, and
all five matched the reference answers. Then the owner redefined the
consumer: "I'm probably never going to write queries manually, so
'using the seat' in practice means sitting down with an agent and
talking to it" (`e0a626c4`). The ruling that followed: the seat's
consumer is "an agent driving the CLI, the owner in the loop; his own
seat is the GUI's."

**Evidence.** The first seat was sat the next day, "ten faults to the
fix session". The whole-CLI seat of 2026-10-13 lists 24 faults across
two runs, each with a fate and most with a commit.

**Consequence.** The project measures its interface by agents who
cannot read the code, and rulings come straight from seat failures:
no id in user-facing text, because "every seat met ids it could not
look up"; "a count says what it counts", because "four seats stopped
on the same six words". The 2026-10-26 assessment calls the seats
"the only instrument that measures the agent-facing surface".

**Recognisable as** a usability lab whose participants are AI agents,
and the product decision that the primary customer of the CLI is a
machine.

## 4.11 The rewrite becomes the product: licence, shipping ruling, root swap (2026-10-11 to 10-12)

**Before.** A Rust playground under `spikes/` on a branch of the
GPL-3.0 C++ repository, with the rewrite-or-evolve record still
proposed.

**Decisions.** Three owner rulings in two days, each prepared by
agents.

1. *Licence.* Before declaring the Rust tree MIT OR Apache-2.0, three
   Opus readers and a lexical scan audited it for anything derived
   from the 64 C++ files whose copyright header names anyone but the
   owner. Result: clean at HEAD, zero shared six-word comment
   shingles, one finding in history (a currency table drafted from
   the C++ source on 2026-09-03 and replaced the next day). The
   owner's verdict: "accepted", with history "left as committed,
   under GPL". The boundary became C110.
2. *Shipping.* "The Rust build ships as Acquisition's agent surface
   by PoE2 1.0: the CLI, the MCP server and the daemon on Windows,
   macOS and Linux."
3. *Repository.* "One repo for Rust and c++" in the morning; reversed
   the same evening once the agent laid out the fork link, the issue
   tracker and the C++ update checker's tag parsing: two
   repositories, "I like acquisition-qt for the legacy".

**Evidence.** The root swap commit (`e7b813b8`): "The workspace moves
from spikes/rust-playground/ to the root by rename, so every file
keeps its history". Gate green with 779 tests.

**Consequence.** Three platforms by 2026-10-15, the alpha on
2026-10-19 ("Yes, make this an alpha release ... no GUI yet and the
feature sets untested in the wild, so not a beta"), and a draft note
to GGG under the existing registration, "Acquisition 1.0, same
registration, new capabilities", for the owner to send in his own
words.

**Recognisable as** a licence clearance of the kind legal runs before
a relicense, followed by a launch decision and a repository
migration, with one decision reversed within hours once its
dependencies were on the table.

## 4.12 "Rulings first, findings after", and the first delegation (2026-10-13)

**Before.** A contradiction read over the governing documents, run by
an Opus subagent, found 35 inconsistencies. The deepest: the working
style said both that decisions are recorded after the code teaches
(the founding line) and that rulings are settled before building
(the practice since 2026-08-31).

**Decision.** The owner, verbatim: "rulings first, findings after".
The same day he delegated agent-facing design to the sessions; the
first commit citing it reads "Done under the owner's delegation of
2026-10-13 for agent-facing surface, and flagged to him".

**Evidence.** The delegation lived outside the tree for a week until
a blind-read trial surfaced it and the entry file gained "Who
decides" for the first time.

**Consequence.** The principle became the second hard rule. It also
produced the cost the 2026-10-26 assessment measured: "a flag change
needs an 800-byte entry with Why, Pinned and glosses before the
code." Thirteen days later the delegated class moved back to
rulings-after (section 4.14).

**Recognisable as** writing down the decision-rights matrix, then
amending it once the approval queue becomes the bottleneck.

## 4.13 The morning after the alpha: seven audits, a roadmap, a governance reset (2026-10-19 to 10-20)

**Before.** The alpha shipped. The owner asked, the same evening, for
"a considered reading of the whole project".

**Decision.** A Fable session orchestrated seven read-only audits
(four Opus, three Sonnet) into a state file and a phased roadmap. The
owner: "I accept all your recommendations exactly as proposed with a
loopback web UI." And on models: "stick to Opus 5.5 as much as
possible, with Fable called in for complex analysis, synthesis, and
design."

**Evidence.** The synthesis: "The core is sound; the weight is in the
shell." 817 of 1,920 Rust-era commits touched only markdown; "P6 says
ceremony is retired by evidence; almost nothing has been"; 106 parked
ideas, "an issue tracker in prose". Three audits converged on one
refactor: refresh, plan and apply logic duplicated between the CLI
and the MCP server "has already drifted". Phase 1 ran the next day:
parks became "twenty 'Not doing' lines, 48 issues, the rest deleted";
the context file was deleted and the entry file became the
registry's root; eight slice records became one checklist and a
one-line history. The roles section fixed the division of labour:
Fable "does not build"; Opus "runs a step from its brief"; Codex
reviews, read-only.

**Consequence.** The shared function layer, `acquisition-verbs`,
built in phase 2a, made "every frontend is a Rust adapter" hold
structurally and became the GUI's foundation. Issues and milestones
replaced parks. The model split is visible in the trailers from this
date.

**Recognisable as** a post-launch operational review, followed by a
re-organisation of process and roles, run by the people who built the
thing, and accepted in one sentence.

## 4.14 "You are in the driver's seat": the renderer, the honesty slice, the blank-slate read (2026-10-24 to 10-26)

**Before.** A review phase had closed with six network rulings, one
making a Cloudflare answer "a stop, never a label". The Windows gate
was red, and an investigation found its deadline failures were "four
different things wearing one colour".

**Decisions, three in three days.**

1. *The GUI's shape.* Reviewing the design brief, the owner ruled
   that the first web UI "is a renderer, not an app, every page
   renders one verb document"; the page "evaluates, sorts, filters
   and composes nothing itself". `acq web` landed the same day.
2. *The honesty slice.* Asked about three groups of issues, the owner
   answered "Let's do this", and on the rules: "you are in the
   driver's seat ... they are yours". C144: "The daemon reports no
   success it did not earn." C147: a frontend reads its environment
   once at construction, not on every call.
3. *The blank-slate assessment and p0.* A Fable session's read: "The
   immune system has no apoptosis. Every fault becomes a named test,
   a checklist row and, if it recurs, a review shape. Nothing
   retires." And: "The owner's authority runs through words, and the
   words are the mass." The honesty slice had "spent about 226 KB of
   briefs, reviews and messages on 7.4k lines of Rust". The owner
   ruled an hour idle default, three milestones, a readiness group
   before the league, and four process changes: reviews stay in the
   tree and the bookkeeping steps go; agent-facing design is "recorded
   in the commit that builds it and listed in the close"; the byte
   budgets "print a note and never fail the gate"; a glossary.

**Consequence.** The first deliberate loosening of the governance
system's own rules. Whether the loosening holds is not yet in the
record.

**Recognisable as** a skip-level review telling a well-run team that
its process is now its largest cost, and the team cutting its own
approval steps in response.

## 4.15 Three reversals

1. **The GUI: Tauri, then a loopback web page.** The founding file
   said "Tauri for GUI (webview frontend)". The 2026-10-19 assessment
   called that ruling thin and recommended a loopback web UI on the
   same function layer the MCP server uses, with Tauri as a later
   wrapper, because a loopback server "is a strict subset of the Tauri
   path: a webview can point at the same server later, and the
   reverse is not true." Agent recommendation, owner ruling.
2. **Decisions after code, then rulings first, then rulings after
   for the delegated class.** The pendulum came back two-thirds of
   the way: boundaries, invariants and anything touching the real
   store or API stay rulings first, the owner's; a verb, a flag, a
   tool or the words of an answer are the session's, recorded after.
3. **Safety ceremony retired once its machinery was proven.** The
   ladder's written hypothesis per run, the frozen-soak checker, the
   mock default: each time the record keeps the mechanism with teeth
   and drops the paperwork.

Smaller reversals: jobs "in-memory for v1" became a persisted queue
two weeks later; one repository became two within a day; the
daemon's 60-second idle exit became an hour; the byte budgets went
from failures to notes.

## 4.16 Three agent-made decisions, and what the owner did with them

1. **Overruled: the daemon embedded in every frontend.** No ruling
   required it; the agents took it as "the cheapest answer to 'where
   is the daemon' for the first two consumers". A reviewer flagged
   it, the author agreed, the owner called it "an unplanned
   shortcut", and the split followed in three days.
2. **Ratified with an amendment: the four-layer model and binding
   plans.** Fable's proposal, sharpened by an outside audit, was
   ruled "everything accepted", except that the owner amended one
   decision: the stable key for an account is the profile uuid, "and
   a login that cannot fetch it fails whole". That amendment's *Why*
   reaches back to the C++ app: "an unstable key is what the C++
   app's legacy buyouts paid for."
3. **Ratified: the loopback web UI and the hour idle default.** Both
   came from Fable-orchestrated assessments; the owner took the first
   "exactly as proposed" and the second as "I approve the 1 hour idle
   default". The assessment had framed a larger option, the MCP
   server starting a real-mode daemon itself, as "a question, not a
   recommendation"; the owner took only the smaller one.

The owner also corrects the agents' words, not only their designs. On
2026-09-13 he turned a requirements list cut to his seven questions
into general properties ("a general model, not one tuned to these
questions"), and on 2026-10-26 he restored his own wording of a
ruling, semicolon included, over a session's trim to fit a byte
limit. The assessment that day quoted the second as evidence that
"the owner's leverage is then spent on wording". Both readings are
true, and report 7 returns to the tension.
