# The whole-project review, planned (2026-11-24)

The owner asked for a full-stop review of the whole project that reaches
past bugs and gaps, to the improvements nobody saw because everyone was
building. This is the planning read for it, at `122c44fa`. It reviews
nothing and changes no code. It says how the system works as a whole,
what fifteen scouting reads found, where a deep review would pay, and
how to run one. Its reader is the session that orchestrates the review;
that session's charge is `BRIEF.md`.

## How it was made

Twelve Opus scouts each read one perspective under one preamble
(`reports/PREAMBLE.md`): read-only, nothing built or run. Two Sonnet
runs took censuses. One Fable advisor, shown none of the scouts'
reports, proposed design theses for the whole. The planning session read
the process rulings, the road and the three earlier step-backs
(`audits/semantic-model-2026-11/`, `audits/assessment-6-5/`,
`audits/governance-2026-11/`), re-read the code or the record behind the
claims marked ✔ below, and wrote this synthesis.

The reports are as returned, local paths removed. The scouts' one-off
scripts were not kept; each report names the rule behind its numbers.
**Every number here is a scout's, unverified unless marked ✔. The
review re-measures what it leans on.**

| Report | What it read |
| --- | --- |
| `reports/core-runtime.md` | daemon, transport, protocol, client |
| `reports/store-plan-verbs.md` | the stores, the planner and pricing, the verbs crate |
| `reports/search-trade.md` | the search crate, `search/`, the trade tooling |
| `reports/agent-surface.md` | the CLI and the MCP as an agent meets them; the seats |
| `reports/web-gui.md` | `acq web`, its tests, its direction |
| `reports/tests.md` | the test estate as a system |
| `reports/governance-context.md` | rulings, process, skills, and what an agent must read |
| `reports/ci-tools-security.md` | the gate, CI, the tool scripts, release, security posture |
| `reports/history-hindsight.md` | churn, built-then-removed, review rounds, repeating causes |
| `reports/verification.md` | state machines, what is proven and what is hoped |
| `reports/code-craft.md` | comments, idiom, file and function shape |
| `reports/performance.md` | every existing measurement, and the read and write paths |
| `reports/design-theses.md` | the Fable advisor's eleven theses and how to run the review |
| `reports/census-code.md`, `reports/census-docs.md` | raw counts |

## The system in one page

**The product.** A daemon (`acqd`) is the only sender to GGG. A request
is a job in a queue that is mirrored to disk; a dispatcher hands jobs to
one choke point, where a limiter (a prediction the response headers
correct), a send gate (two in flight, one per policy) and the rails
(halts that only refuse) decide whether a send may go. Three SQLite
stores hold what was fetched (facts, named by username), what the user
decided (intent, named by account uuid) and saved searches; four more
files hold the queue, the halts, the accounts and the send journal.
Two pure libraries sit on the stores: the planner (what to refresh,
plus pricing, listing and the shop) and the search (a query language, a
corpus derived from stored item bodies, the answer, and a translation
from trade-site URLs driven by about 2.5 MB of embedded tables). A verbs
crate holds what needs daemon and store together. Three frontends render
the same typed documents: the CLI as text, the MCP as JSON, and
`acq web` as server-rendered HTML inside the CLI crate.

**The scale.** About 120k lines of source (24% comments) and 133k of
tests in ten crates; 200 numbered rulings; 86 open issues; 565 saved
reviews; about 4,200 commits since July. Of the 2,550 since the
workspace took the root in October, about 59% touch no file under
`crates/`.

**The process.** The owner rules on boundaries; sessions decide the
rest. A step is an issue or brief, a build, the gate, a cold review by
another model family, and a close commit. Knowledge of the outside world
is gathered by procedure: live runs against GGG, sittings at the trade
site, seats where a blind agent drives the tool.

**What is strong, and should be imitated rather than reviewed.** The
dependency edges that make invariants 1, 3 and 5 structural. The send
journal with its teardown oracles, and the rate-limit spec tables. The
two sans-IO state machines, `client/src/door.rs` and `daemon/life.rs`
(C157): a pure `step(state, event)` pinned by a table is the project's
best technique and only two machines use it. The search's property
tests and the catalogue's text-template translation. A local gate with
a 65-second median. The web server's security posture. A tracker with
every issue labelled and none older than five weeks.

## Seven things the reports say together

No scout was asked for these; they are where independent reads met.

**1. One question, answered in many places.** The project's own lesson
from #261 ("one ending") recurs at larger scale:

- *May this route send now?* Five holders under separate locks, checked
  in three tiers, with route closure held four ways (core-runtime H1,
  verification H1).
- *Can this build read this stored row?* Seven per-kind state types
  across four crates, because the store's read is untyped and the
  realistic case is found above it (store-plan-verbs H1). This is the
  assessment's largest finding class, 24 findings and 7 of 20 P1s.
- *Which account, realm and league?* Defaults applied at dozens of
  sites and three realm types; about a quarter of P1 and P2 findings in
  every period name one of the three (history 1, code-craft 1).
- *What does this document say?* Each frontend words it again, and
  remedies are composed in many places (agent-surface 5, design-theses
  T1 and T4, history 2).
- *What kind of job is this?* Strings matched in three crates, because
  the dependency edges of C1 leave no home for a shared type
  (code-craft 1).

**2. A convenience becomes a family of states.** Everything built and
then removed (the main account, the closed-for-good route, the query's
own order, `acq pull`) saved a user one decision and cost every reader a
case. What still has that shape: the realm and league defaults, job
priority, the unreadable states, and perhaps the persisted queue and the
lazily spawned daemon (history 2, design-theses T2).

**3. Words outgrow the code.** Module docs restate behaviour under
`# As built` (about 3,200 lines; one grew from 10 KB to 33 KB in two
weeks) and paste rulings verbatim. An agent following the instructions
literally reads 70 to 160 KB before a first edit, perhaps a fifth of it
relevant. The README still opens by saying there is no UI ✔. The
largest finding shape in the assessment was a sentence wider than its
code (governance 2, code-craft 4, design-theses T10).

**4. Guards grow guards, and nothing retires.** No ruling has been
retired among 200. There are 407 retirement lines in commit messages
and one test file ever deleted. The breakers grew 85% and are now CI's
critical path. Per line changed, the tools, hooks and workflows draw
about four times the daemon's review findings. Every byte cap is
written to: 16 of 29 recent briefs sit within 104 bytes of the limit
(governance 1 and 3, tests 1 and 4, ci H2, history 4).

**5. A finding with no route does not land.** The step machine moves
issues. The semantic-model audit's faults were filed; of its
"provably equivalent" refactors, none of nine sampled was taken, and
six of its findings in the daemon stand unfiled. Two performance
findings of 2026-10-19 stand with no issue. The assessment's candidates
6 to 10 have none. An improvement that is not a bug has no trigger
label (verification, core-runtime, performance, history 3).

**6. What is capped is measured; what matters is not.** Briefs, the
road and the MCP's first turn have byte bounds. Nothing measures
latency (no benchmark anywhere; the last hand measurement is five weeks
old and predates the GUI), the size of the owner's store since the
league began, the calls an agent needs for a refresh (flat at 18 to 21
across three fix rounds), or which frontend tests are load-bearing. CI
finishes on about half of pushes. The last CLI or MCP seat was
2026-11-04, 116 commits ago (performance, agent-surface, tests).

**7. The size goes where the direction is unsettled.** `acq web` is
12k lines, grew by more in four days than the #293 cuts removed, and
leads every findings table. C191 ✔ rules that it "is never reshaped
into a high-performance desktop app" and that a GUI laid out like the
trade site or the C++ app "is a separate frontend". The sitting round
of 2026-11-24 built resizable columns and sortable heads into it, and
parts 2 and 3 are about to be designed (web-gui H1, design-theses T11).

## Verified here, and where the reports disagree

✔ C191's text (`decisions/frontends.md:18`). ✔ The send gate holds
active policies as a set; `rekey_policy` onto a policy that is already
active loses an entry, guarded by `debug_assert!` alone
(`crates/acquisition-daemon/src/gate.rs:105-113, 203-207`); #206 and #66
are open. Whether GGG can present the input is not known. ✔
`Corpus::is_current` builds the whole corpus header to compare a
revision (`crates/acquisition-search/src/corpus.rs:856-864`); its cost
was not measured here. ✔ The Windows gate failed on `main` at
`0701b7d6` and `178d51bb`, after #312's fix, and no open issue names
the test.

The advisor would not review the limiter, gate and choke point (the
best-tested code in the tree); two scouts name admission as the best
target for both simplification and proof. The advisor would leave the
corpus load until a GUI performance target is ruled; the performance
scout names three cheap wins. Each is a thesis to try, not a settled
answer. The advisor's sizing of lifecycle and queue durability (10.6k
lines against 5.7k for rate limiting) is its own estimate.

## The review

### Its kind

A cold review reads a diff against a brief and can only add: a fault, a
rule, a pin. It cannot see a premise, an absence, an aggregate or a
cost, and both removals that worked came from tables, not reviews
(design-theses §3). This review reads for what is one thing in two
places, what is the third instance of a shape, what pays for each state
and file and rule, and what could go. It grades nothing P1 to P3 until
its last phase.

### What a finding must carry before it reaches the owner

1. What goes, counted by a named command: concepts, states, lines,
   rulings, tests.
2. The smallest input whose behaviour changes (the semantic-model
   audit's standard for a "trust me" refactor).
3. Each invariant and ruling it touches, and what holds each after.
4. What it costs or gains in security, performance, function and an
   agent's ease, said separately.
5. A dissent written by a reader who tried to break it, of another
   model family where one is installed (C177).
6. Its route: an issue with its labels, a question for the owner, or a
   stated reason it is dropped. No finding ends as a paragraph in a
   record (pattern 5).

### Phases

**0. Ground truth.** Measure what the scouts could not, since they ran
nothing: the read path's split (open, currency check, load, answer,
show) on the measurement copy and, with the owner's word, a copy of his
store as it is now; the MCP's real first-turn bytes; per-test times;
which package's tests kill each mutant of the verbs crate. Re-run the
headline counts the tracks lean on.

**1. Census of nouns.** Every state variant, file on disk, revision
scheme, protocol variant, job kind, knob, tool, check and ruling, each
with what pays for it. This is the table the design reads work from.

**2. Design reads.** One per track D1 to D7 below, each by a reader who
holds the whole track in mind, ending in one page under the standard
above. An advocate lists what goes; a skeptic finds the breaking input.

**3. Estate reads.** Tracks E1 to E9, wider and shallower, ending in
lists with named holders.

**4. Trials.** For at most three theses that reading cannot size: a
patch applied, measured by `git diff --shortstat` and the gate, saved
under this directory and reverted. Nothing lands in `crates/`.

**5. The step back.** The orchestrator's own read of everything above:
what the project believed it needed and did not, and what the process
would be if designed today. Then a portfolio of at most ten one-page
proposals for the owner, ranked, and every other finding routed.

### Tracks

Ranked within each group by expected value, as this read judges it.

| Track | The question | Start with |
| --- | --- | --- |
| **D1. The GUI's direction** | Under C191, what are parts 2 and 3 designed for? The form speaks the trade site's query model and the other doors speak the language: which is the model? What would C++-grade interaction need that URL-as-state and a full re-render cannot give? | web-gui H1 to H3; design-theses T3, T11; performance H3, H5 |
| **D2. One document, three projections** | Should the words, the remedies, the set each count is over, and the list envelope live below the frontends, so a frontend only lays out? Should an offer be a value any door takes back? Should an MCP tool be the CLI's verb path? | agent-surface 2, 4, 5, 6; design-theses T1, T4; history 2 |
| **D3. Admission as one value** | Is "may this route send now" one pure function of state and time, explored exhaustively against an independent oracle, in the form `door.rs` already has? | core-runtime H1; verification H1; #206, #66 |
| **D4. The stored row and the scope** | One typed read at the store boundary; account, realm and league as one value resolved once with no defaults; a home for shared vocabulary under C1; the account as its uuid everywhere. | store-plan-verbs H1 to H3; history 1; code-craft 1; design-theses T6 |
| **D5. What persists, and why** | Seven files, three migration ladders, three revision schemes. Need the queue persist when a plan is re-appliable? Can facts be re-derived instead of migrated? Is the daemon's lifetime load-bearing only because the limiter's prediction is not kept? | design-theses T2, T5, T7; core-runtime H2, H3; store-plan-verbs §1 |
| **D6. The algebra of values** | Is there one value expression (a field, a total, a weighted sum, a named stat) under #316, #46 and the totals table, before #316 is built as a third special case? | search-trade H1 |
| **D7. The process, designed today** | Why do improvements not land? What does review buy per round now? What would the registry be if only law were numbered? What does a step look like when the builder can hold a crate in mind and rewriting is cheap? | governance 1 to 5; history; design-theses T9, §2; pattern 5 |
| **E1. Load-bearing tests** | Which frontend tests kill a mutant no lower test kills? Can page goldens hold structure, seeds become one world, `seat_faults.rs` become tables? | tests 3, 4; design-theses T8; search-trade H2 |
| **E2. Guards as structure** | Which grep in `arch-check.sh` could be an edge, a visibility or a clippy `disallowed-methods` line (#196, #230)? What have the breakers caught? Is the artifact-identity chain proportionate? | ci-tools-security H1, H2, H5 |
| **E3. The reading path** | Halve what an agent reads before a first edit: pack as the one entry, module docs that own and point, a registry that does not restate its checks. A human's way in. | governance 2, 3; code-craft 4; design-theses T10 |
| **E4. The agent's first turn and call count** | The smallest text that lets an agent act; the refresh loop's calls; undo as a restorable state. Then a seat, since none has sat the CLI or MCP since 2026-11-04. | agent-surface 1, 3, 7 |
| **E5. Performance** | The currency check, the double parse, the item card; then one standing measurement. | performance H1 to H4 |
| **E6. Craft reads** | `store/src/lib.rs` (`record_in`), `search/src/answer.rs`, `daemon/kinds.rs` with `queue.rs`; ten table loaders with ten error types. | code-craft 2, 3, 7 |
| **E7. The trade loop's proportion** | About 19k lines of tooling and 1,255 hand-run searches behind 5.5k lines of translation at 64% exact under a bound: what does the next sitting buy? | search-trade H3 |
| **E8. Security, narrowly** | One page: what a local process and a hostile API body can reach (the socket, item text in HTML, the log and `diagnose`), and the Actions pins. | ci-tools-security H6 |
| **E9. One-off tools** | Which scripts under `tools/` still run. | ci-tools-security blind spots |

If the review can only do five: D1, D2, D7, D3, D4.

### Where not to look

Transport, `Secret` and the keyring; the protocol's wire fixtures; the
search's parse, bind and eval core; ingest beyond what is filed; the
local gate's speed; the road file and the tracker; release and
licensing; the language choice; history in comments (cut). The
per-step cold review does its own job and is not this review's
subject, only its yield is (D7).

### What is the owner's

These cannot be settled by a session and should reach him as questions
with the evidence beside them, late and few: what `acq web` is for
(C191 against the sitting rounds); whether the queue persists (C6,
C45); the dependency edge a shared vocabulary would need (C1); facts
named by uuid (his store); what in the registry is law; whether a copy
of his store may be measured.

### Found in passing

Defects, not review subjects, each standing with no issue at this
commit: the browser test `two_stat_groups_one_weighted_…` failing on
the Windows gate after #312 closed ✔; the limiter's `PolicyMismatch`
arm skipping `assume_counted`; cargo-deny running only in CI, where
advisories were half of Linux's failures on `main`;
`rust-toolchain.toml` naming no target, so `winclippy` fails on a
fresh clone. The reports hold the rest.
