# p1 — The roadmap sweep: the README made true, short and governed

A documents-only step between 2g's close and the next pull-forward
item, opened by a read of `roadmap/README.md` against the tree, the
issues and the milestones on 2026-10-28. It runs before the next step
because every later worker reads the README first, and today a worker
reads 42 KB to find the 8 KB it needs, four of its claims are stale,
and its "Open" list holds about seventy observations that `AGENTS.md`'s
"Where things live" sends to issues. The owner's answers (Do 1) come
first; the rest is one session's work on `main`.

## Goal

A worker who starts a step reads a README of about 12 KB that is true
on the day it is read: the standing rulings, the roles, the rhythm,
the phases as they stand, the pull-forward list, the current step. The
ledger is its own file beside the other two ledgers, every row a
commit range, a close date and the homes of what the step left. What
a step left open lives in an issue, a "Not doing" line or the owner's
list, and nowhere else. The brief template and the roles state no rule
`AGENTS.md` already states.

## Read

- `roadmap/README.md` whole, once, as the thing under repair.
- `AGENTS.md`, "Hard rules" (Rulings first, findings after), "Where
  things live" (work not done is an issue; what is declined is a "Not
  doing" line), "Principles" (P6, a check or rule is retired when what
  it guarded is proven).
- `roadmap/briefs/p0-process.md`, Do 1 (a) and (b), and the owner's
  answers quoted in `roadmap/README.md`, "The owner's rulings for this
  road", 2026-10-26: a ledger row is a commit range and a close date;
  a phase close deletes nothing; an agent-facing design decision is
  recorded in its commit and listed at the close.
- `roadmap/briefs/TEMPLATE.md`, "Rules": the bullets this step reduces
  to a pointer at `AGENTS.md`. Its own first paragraph says why: a
  copied rule drifts.
- `HISTORY.md`: the row form a phase close adds; the honesty slice's
  row, which has no ledger twin.
- `GLOSSARY.md`, "ledger": a table of dated rows read from its newest
  end. `RUN-LEDGER.md` and `SEAT-LEDGER.md` are the two the roadmap's
  joins.
- `tools/docs-check.sh`, the comment above `SKIP_ROADMAP` and check 2b:
  the roadmap is read for cited ids and cited commits, so every hash
  this step writes must exist and every `<path>` at `<hash>` citation
  must name the path as it was.
- `roadmap/STATE-2026-10-26.md`, section 4, R4: the age rule for
  "Open" that the assessment proposed and nobody adopted.
- The three milestones (`gh api repos/{owner}/{repo}/milestones`): the
  due dates the phase table duplicates.

## Input

The routing table below is the step's fixed input: every observation
in "Open" as of `5f62d548`, with its fate. The owner may strike or move
a line before the work starts; the worker applies the table as it
stands after his reading and names in the report any line the tree
disproved. Fates: **issue** (a new one, lettered below), **on #N** (a
comment on an existing issue, in the issue's own words, citing the
brief and the commit the observation came from), **not doing** (a
line at the foot of the named area file, with its trigger), **packet**
(a question in Do 1, by letter), **drop** (nothing to build; the
reason rides in the close message), **mine** (the orchestrating
session's, outside the tree).

### New issues

| Letter | Title | Labels | Milestone |
| --- | --- | --- | --- |
| A | The MCP and the web still speak in CLI commands: the request-offer `says` and `readings` for `basis_changed` and `not_an_item`, the pages' next actions printed as command lines, the search crate's flags in text that is no refusal, the policy write refusal's read-back with no account, a scope refusal's `readings` as prose where a language refusal's are payload | area:frontends, area:search, trigger:seat | 4-full |
| B | Search results as site-style cards: rows carrying every field, one maker with `show` (`GUI-NOTES.md` section 7) | area:frontends, trigger:gui | 4-full |
| C | `acq web` leftovers from 4b: the no-account refusal as a page, a test seam into a page's script (the picker's no-fallback rule unpinned), the MCP `search` description naming `view.rows.fields`, whole-page goldens of 51 KB that change on a one-line form edit | area:frontends, trigger:gui | 4-full |
| D | The daemon's architecture rows deferred past 2f: a typed `JobKind` (the root of #90), the generic reference-table loader, the `Thing` columns, the lock merge, sharing `frame.rs` | area:daemon, area:store, trigger:consumer | 4-full |
| E | Where the CLI and the MCP diverge without a ruling: the stream of a text failure's report (stdout under `refresh --apply`, stderr under `acq result`); `--if-revision 0` as "no policy stands" on the CLI alone; tool descriptions stating compact-JSON sizes where `acq --json` is pretty-printed and about a third larger | area:frontends, trigger:user | League 3.30 and alpha.2 |
| F | The job list under the queue lock: `job_list` holds the lock for its whole range read so the dispatcher's persists wait on it; `ended_elsewhere` answers `no_job` for an open row on disk and not in memory until the next restore; the parent scan is O(n²) over live work; no seam stages a queue break between the profile job's eviction and a login's read | area:daemon, trigger:live | 4-full |
| G | A bound on every observe request: `acq daemon status` waits without bound on a peer that answers `hello` and never its status, where `acq diagnose` bounds its own request by the handshake's deadline (C86, the daemon answers `hello` in milliseconds) | area:daemon, trigger:user | League 3.30 and alpha.2 |
| H | The schemas are the larger part of the MCP's first turn: 15.3 KB of 26.4 KB, rmcp's `$schema` header and every optional argument typed `["string","null"]`; dropping `$schema` and `title` from `list_tools` is one override | area:frontends, trigger:seat | 4-full |
| J | The read economy's leftovers from 2g: a cut `tabs` list in the CLI text sorts pages away from their parents so the indent misleads at eleven tabs and up; `SHOWN` (6) and `LISTED_INLINE` (12) in the search crate are numbers outside the bound; a human's `store events` summary now covers the ten newest events where 200 did | area:frontends, area:search, trigger:seat | 4-full |
| M | Tools hygiene: the document-path check reads no roadmap file; the trade check has no breaker suite; `bothy pack --path README.md` returns the search and trade rulings on a basename match | the tooling label Do 1 (d) settles | 4-full |
| N | The price refusal's remedy ignores the realm: `leagues_of` reads leagues for an item without its realm, so an item asked for in the wrong realm prints a remedy that shows nothing | area:pricing, trigger:seat | League 3.30 and alpha.2 |
| O | The client's unit tests: the environment-moving tests in `client.rs` remove what they set instead of restoring it (`Moved` is there for them); the replacement pin spawns a real `acqd`, so they need a built workspace or an injectable spawner | area:daemon, trigger:gui | 4-full |

### The routing table

| Open paragraph | Observation | Fate |
| --- | --- | --- |
| Tooling issues carry no area label | #84 and #88 | drop: closed |
| | #86 and #89 unlabelled; no area holds the tools | packet (d) |
| Deferred past 2f, the daemon's architecture | the five rows | issue D |
| Unruled | the text failure report's stream | issue E |
| | `--if-revision 0` CLI-only | issue E |
| | the environment's proxy variables routing the one client (`build_client` names none) | packet (e): it is the path to the real API |
| For 3b, the regeneration | #53 generator; the pin bumped first (C125); the two trade-stat ids at `poe1@ee2084de` | on #53 |
| The owner's, from r2 | a brief that commits its tests failing beside the gate-per-commit rule | packet (f) |
| The owner's | the `AGENTS.md` budget under the temporary raise | packet (g) |
| | a *Pinned:* path with a slash as a bothy rule | packet (h) |
| | the frontends' review, as 2f's of the daemon, store and network | packet (i) |
| | `REVIEW-PROMPT.md`: a round's brief named once for a range; client input told apart from persisted or upstream input | packet (j) |
| | `REVIEW-PROMPT.md`: prior findings as a fix round's done-when | drop: in the prompt since `10f24fd9` |
| | a round's plan: restart after a failed write and a login's finalisation among the recovery scenarios | on #63 |
| From r3, the eviction | the four observations | issue F |
| From r4, the diagnosis | `acq daemon status` unbounded on a stalled peer | packet (k), then issue G on yes |
| | the angle-bracketed placeholders and the paste-between-fences header | drop: confirmed as the design |
| | a stopped daemon's journal and log found by this shell's knobs | drop: #94 carries the contract form |
| From r5, the sentences | C127's words redrawn at `ea0b0255` | drop: done |
| | a selection plan carries no `notes`; a selection-shaped `plan_notes` is a planner change if a consumer asks | not doing, `decisions/plans.md`, trigger: a consumer |
| | tool descriptions state compact sizes where `acq --json` is pretty | issue E |
| From the C135 round | `settle()`'s 100 ms and `polled_once`; the older tests take the helper when it moves into the harness | on #64 |
| | the auth requests and `quote` refused as stopping during any exit; `quote` waits on the session table | drop: the behaviour, told in `1bf30803` and the brief's correction |
| | a 401 under an unreadable index followed by a restart resends the bearer once | drop: on #63 already |
| From #83, the surface | the first turn 26.4 KB, the schemas 15.3 KB, the `$schema` and `title` override | issue H |
| | `characters` keeps its two glosses | drop: decided under the bound |
| | the own sections' entries read alone, pinned for `status counts` | drop: pinned and checked |
| The owner's, from the seat (#84) | a typo'd account as `not_logged_in` | drop: ruled 2026-10-27 at `ea0b0255` |
| | the seat skill's steps 10 and 11, process wording an agent changed | packet (l) |
| | `AGENTS.md`'s "Where things live" has no line for a seat | packet (m) |
| | a scope refusal's `readings` prose, a language refusal's payload | issue A |
| | the `not_an_item` sweep shape not seen to fail first | drop: a lapse of one round, the pin stands; the template's done-when already asks that a pin be red before its fix |
| From 2g rounds 1, 2, 3 and 6 | the counts tables' default followed the rows' | drop: decided |
| | the cut `tabs` list's indent | issue J |
| | `SHOWN` and `LISTED_INLINE` | issue J |
| | the events default of ten | issue J |
| | the live drivers under `tools/` edited and not run | on #67: the owner's next live run is their first check |
| | `refresh --plan --json` prints the envelope alone; the ages on the MCP's answer only | drop: decided, the stdout stays pipeable |
| | ages differing by a second between processes | drop: handled in the one test |
| | the two printed-command runners (`search_json.rs` runs `sh`) | on #87, which already names the runner and corrects h2's file name |
| From 2g rounds 4 and 5 | the refusal function's task-local set in a hand-written `call_tool` | drop: internals, agents' own; a reviewer who prefers the plumbing opens an issue |
| | the search's request-offer `says` and `readings` as CLI commands on the MCP | issue A |
| | the policy write refusal's read-back carries no account | issue A |
| | the web phrasing from 4b: command prose, the jobs page's CLI text, no page renders an offer | issue A |
| | `TEMPLATE.md`: the marker "Settled in the round"; `--json` added to a printed command a JSON pin runs | packet (n) |
| | `serde_json` values lose key order | drop: the checklist row at `f8ffb1ec` |
| The Windows gate's watch (#70) | the week from run 40428110668; nextest groups; the two trade tests reading no `ACQ_CASES`; the default profile's 60 s comment | on #70, whole; the watch leaves the README |
| For 4-full, from 4b | the results as cards | issue B |
| | item counts beside the league select | not doing, `decisions/frontends.md`, trigger: a document carrying both on one basis |
| | a web phrasing for the next actions | issue A |
| | `acq web` with no account on disk | issue C |
| | the MCP's `search` description naming `view.rows.fields` | issue C |
| From 4b's worker, no step named | no seam reaches a page's script | issue C |
| | the search crate's flags in text that is no refusal | issue A |
| | a whole-page golden of 51 KB | issue C |
| The owner's, from 4b's reviews and worker | committed coverage apart from evidence the tests pass; a fix round's acceptance apart from the sitting | drop: in the prompt since `10f24fd9` ("not verified" once, in one line) |
| | the committing session launches a worker step's review and polls until `codex exec` exits | packet (o): one sentence in "Roles" |
| Tooling, no step named | the three tool observations | issue M |
| | a session's must-read carries the memory index beside `AGENTS.md` | mine: the index consolidated, its state line retired for the ledger |
| An issue ready for a step | #11 | the pull-forward list, after #91 |
| From h1, the honesty group | the C145 check settled by N50; #63's rows; #74; #16's first instance | drop: each on its issue or in the ground truth already |
| From h2, the words | #11's two parts | drop: on #11 already |
| | `leagues_of` ignores the realm | issue N |
| | `price_json.rs`'s `run_printed` outside the isolation list | drop: #87 corrected the file name; the runner is `search_json.rs`'s, on #87 |
| | the seat worlds' stores gone | drop: #84 sat since and kept its pristine store |
| From h3, the foundations | the provider process-wide | drop: the design |
| | `contract.rs`'s ten-second clock waits | on #70, the deadline family `TIME-SEAM.md` section 3 holds |
| | the environment-moving unit tests; the pin that spawns a real `acqd` | issue O |
| | the race look's cost on a child that dies before the lock | drop: told in h3's commits |

## Do

1. **The owner's packet**, one message before any edit, each question
   with its recommendation. His answers are quoted in the first commit.
   - (a) The "Open" rule, in the README's words: an item lives under
     "Open" only until the next close, which gives it an issue in a
     milestone, a "Not doing" line with its trigger, a place on the
     owner's list, or a reason in the close message; the owner's list
     is the one subsection that stays. Recommended: yes. It is
     `AGENTS.md`'s "Where things live" applied to this file, and the
     assessment's R4.
   - (b) The ledger-row ruling of 2026-10-26 applied to every row,
     those written before it and the nine written after it that list
     each commit (the 2g row is 3,810 bytes): a row is the step, its
     commit range, its close date and the homes of what it left. The
     commits are `git log <first>^..<last>`; the reviews are named by
     sha under `roadmap/reviews/`. Recommended: yes.
   - (c) The ledger moves to `roadmap/LEDGER.md`, read from its newest
     end like `RUN-LEDGER.md` and `SEAT-LEDGER.md`; the README names
     it in one line. Recommended: yes; a worker then reads no row.
   - (d) A home for the tooling issues (#86, #89, and issue M): an
     `area:tools` label whose rulings live nowhere, or an
     `area:process` label whose home is `roadmap/README.md`'s "A step"
     and `tools/`. Recommended: `area:tools`, with `REVIEW-CHECKLIST.md`
     gaining no section until a tool fault comes back.
   - (e) Whether the environment's proxy variables may route the one
     client (`build_client` in `ratelimit/choke.rs` names none, so
     reqwest's default applies). Recommended: a ruling that names the
     current behaviour as deliberate, either way; it is the path to
     the real API and not an agent's to settle.
   - (f) A brief that asks for its tests committed failing first (r2's
     Do 1) against `TEMPLATE.md`'s gate-per-commit rule. Recommended:
     the tests ride with their fix, and a brief that wants the red
     seen says so in the report, not the history.
   - (g) The `AGENTS.md` budget: 15,241 of 15,277 under the temporary
     raise, a note since p0 (c). Recommended: the number stands as the
     note's threshold and the word "temporary" is retired.
   - (h) A *Pinned:* path with a slash as a bothy rule (2d wrote
     nineteen). Recommended: no rule; the docs check already proves
     every path named exists.
   - (i) Whether the frontends get the review 2f gave the daemon, store
     and network. Recommended: not before the league; the seats are
     the frontends' review, and #84 sat one.
   - (j) `REVIEW-PROMPT.md`: name a round's brief once for a range, and
     tell client input apart from persisted or upstream input.
     Recommended: both, one sentence each; the prompt is his.
   - (k) A bound on every observe request (C86 today bounds `hello`
     alone). Recommended: yes, as issue G in the league milestone.
   - (l) The seat skill's steps 10 and 11, wording an agent changed
     without his word. Recommended: stands as written.
   - (m) A line for the seat in `AGENTS.md`'s "Where things live".
     Recommended: one line, "A seat: one `SEAT-LEDGER.md` row", within
     the budget.
   - (n) `TEMPLATE.md`: the marker "Settled in the round" for a brief
     sentence a fix round corrects in place; `--json` on a printed
     command a JSON pin runs. Recommended: both, each one sentence.
   - (o) "Roles": the committing session launches a worker step's
     review and waits in the turn until `codex exec` exits.
     Recommended: one sentence.
   - (p) `STATE-2026-10-19.md`, 94 KB, pointed into by closed phase 2
     briefs alone. Recommended: keep; the dated-file rule stands and no
     reader pays for it.
2. **Route "Open"** by the table: every `gh` write one call at a time
   (an issue, then its milestone, then each comment), the new issues
   lettered in the table given their numbers in the report. The
   "Open" section becomes its rule and the owner's list: what (a) to
   (p) left with him, each item one line.
3. **The ledger** as (b) and (c) ruled: every row rewritten to its
   range, its close date and its left-open homes; the readiness phase
   (p0, r1 to r5) folded into one row and given its `HISTORY.md` line;
   the honesty slice's row written from `HISTORY.md`'s; the tempdir
   rounds (`b350125b`, `893677a3`, `39b4d9e1`, reviewed at
   `reviews/tempdir-b350125b.md` and `reviews/tempdir-fixes-893677a3.md`)
   given their row. A hash a row drops is not lost: the review files
   and `git log` hold it.
4. **The README made true.** The phase table: 2g closed in row 2 and
   the row's "ahead" reduced to 2h; row r closed; the corpus struck
   from row 4 and from the first rulings bullet (#82 closed it); the
   "Closes by" column replaced by the milestone's name, so the date
   lives once, on GitHub. "Pull-forward": 2g and #88 leave; the list
   reads #85, #86, #87, #89, #91, #11. "The owner's rulings for this
   road": the standing rulings alone (the goal, the 4-full shape, the
   models, the pull-forward permission, the p0 answers), each landed
   ruling cited by the commit that landed it and its narration cut.
   "Housekeeping" folds into the ledger's phase 1 row. "Current step"
   says p1 is the step and names the next.
5. **No rule stated twice.** `TEMPLATE.md`'s "Rules" opens with
   "`AGENTS.md`'s hard rules apply" and keeps only what a worker adds:
   its own worktree and scratch, its own processes, a draft taken
   verbatim, no wall-clock bound, an id in words. "Roles" drops the
   mock-world bullet (the hard rule) and the models line if the
   rulings section keeps it, and gains (o)'s sentence if ruled.
6. **Measure**: the README's bytes before and after, `wc -c` after
   the last edit; the count of issues opened and comments written.
7. One commit per concern: the owner's answers first; the routing
   (issues and comments carry no commit, the README's "Open" cut
   does); the ledger; the table and rulings; the template and roles.
   The gate's `--docs` form before each, since nothing here is read by
   a test (the crates name roadmap briefs in comments alone) and the
   docs check reads the roadmap for ids and hashes.

## Done when

- `roadmap/README.md` is under 13,000 bytes, measured after the last
  edit and stated in the message, and holds "The owner's rulings for
  this road", "Roles", "A step", "The phases", "Pull-forward", "Open"
  (its rule and the owner's list alone), "Starting a session" and
  "Current step".
- `roadmap/LEDGER.md` exists; every row's second column is one commit
  range or the ranges of its rounds, with no gloss per commit; the
  readiness phase is one row; `HISTORY.md` has its line.
- Every observation in the routing table has its fate applied: `gh
  issue view` shows each new issue with its labels and milestone, each
  "on #N" is a comment, each "not doing" is a line at the foot of its
  area file with a trigger, and the close message names each "drop"
  with its reason.
- The phase table names no date; `grep -c '2026-1[12]-' roadmap/README.md`
  is 0.
- `grep -n 'ACQ_PROVIDER=mock' roadmap/README.md roadmap/briefs/TEMPLATE.md`
  finds nothing: the hard rule is `AGENTS.md`'s.
- `tools/gate.sh --docs` green on the step's tree; every hash the
  README and the ledger cite is a commit (the docs check's 2b).

## Not in scope

- Any file under `crates/` or `tools/`: this step moves words.
- `STATE-2026-10-19.md`, `STATE-2026-10-26.md`, `GUI-NOTES.md`,
  `TIME-SEAM.md`: dated evidence, never rewritten.
- Deleting a brief or a review: a phase close deletes nothing (the
  owner's ruling of 2026-10-26).
- The registry: the five entries over 800 bytes and C73 uncited are
  the docs check's notes, for a hygiene step of their own.
- The memory index: the orchestrating session's, outside the tree.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `p1`; commit on `main`, never push, the
`Co-Authored-By` line for your model last; `tools/gate.sh --docs`
before each commit; every `gh` write one call at a time, no batch; the
packet (Do 1) is a message to the owner, and nothing is edited before
his answers.

## Report

`TEMPLATE.md`, as text, with the new issues' numbers against their
letters.
