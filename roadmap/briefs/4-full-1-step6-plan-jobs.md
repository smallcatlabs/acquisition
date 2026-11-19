# 4-full-1, step 6.5d — The plan and jobs pages from their documents

## Goal
At `acq web`, the plan, apply and jobs pages render their documents
in a player's words, as the place pages do since 6.5a: no `pre`, no
command line, no daemon jargon. A failed job opens to why, and the
plan says when the daemon has refused the account for having no
session. This closes #182's plan and jobs half and #189's fault 12.

## Inputs
- #182 and its comments (the page seats' T6 and T7); #189, fault 12;
  6.5a's close, `git log --grep='^Step: 4-full-1-step6-places'`.
- `4-full-1-step6-design.md` §3, "After an apply".
- C16 (a page holds what its URL names), C46 (semantics in Rust), C53
  (JSON is the contract), C138 (the listener; routes were the owner's
  drafts, #284), C165 (a failed job's kind and way on).
- The documents: `Planned`, `Apply` (verbs `refresh.rs`); `Jobs`,
  `Listed`, `PersistedJobs`, `JobResult` (verbs `jobs.rs`); `JobInfo`
  (protocol `job.rs`); `Quote`, `QuoteScope` (protocol `protocol.rs`);
  `RailsStatus` (protocol `status.rs`).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **No document changes.** Each route's JSON answer stays byte for
   byte: no protocol key, no field, no route name. A page needing one
   stops for the owner.
2. **Renderers read documents, never the CLI's text,** in `web/page.rs`
   beside the place pages; `render_plan_on_a_page`, `jobs_text`,
   `persisted_queue_text` and `rails_text` leave `web/`.
3. **The plan page:** what is refreshed (realm, league, selection or
   policy), the facts' ages (`PlanAges`), `PlanNotes`, the actions as a
   table (what, why), the skipped and uncovered counted, and Apply. The
   quote in words: a scope's `eta_seconds` is when its *last* request
   could go, so the largest known one is said as the time for the
   requests whose timing is known, each scope without one counted as
   not yet known, `not_covered` said as what no estimate includes, and
   a `halted` quote as sending nothing. Scope keys stay in the JSON.
4. **A refused session** (#189, fault 12). `QuoteNote` gains a variant
   for a quote the daemon refused `not_logged_in`, its `Display`
   today's sentence. Only it says "not signed in": no daemon, a timeout
   or another miss says nothing of a session, since none was observed.
   Apply stays; the apply's refusal is the authority.
5. **The apply page** says `Apply::Nothing` and `Spent` from their
   fields, with a link to the plan compiled again.
6. **The jobs page:** `Listed.rows` as a table, a child under its
   parent where both are listed: what (kind in words, `target`), state,
   when, a waiting job's start from `eta_seconds`; counts and the left
   out in words. Of `RailsStatus`, only `halted`, `closed_routes` and
   `refresh_failed`, as prose. No daemon: the ledger's rows, how old,
   and that the next job resumes them. The reload while a job runs
   stays.
7. **A job's ways.** No link is derived from a job's kind or params:
   the nav bar on every page is the way to the places, and an apply
   may span leagues and realms. A failed or cancelled job opens its
   `JobResult` (Q2), read by id through `Frontend::job` and `result`,
   listed or not: its failure said by `FailureKind` in the page's own
   words, the first failed children the same way, offers spelled by
   `Paged`, which now spells `Way::Result`. The daemon's `error` and
   `WayOn.words` can name a command and stay in the JSON alone; a
   failure with no kind says only that it failed.

## Open for the designer (C172, Fable designs)
Each answer amends this brief before the build starts.
- **Q1, where an apply lands.** The design says back at the places,
  but their facts change only once the job lands. Recommended: Jobs,
  the apply's job opened and reloading until done, the nav bar to the
  places. The other: the places page, which then reads the jobs too.
- **Q2, where a job's result shows.** Recommended: under its row,
  `jobs?job=<id>`, as `open` names a parent on the places; no route
  name. The other, a route of its own, is a C138 route, taken under
  delegation as #284 was.

## Find first
- `git grep -n -E 'render_plan_on_a_page|jobs_text|persisted_queue_text|rails_text|fn pre\(' -- crates/acquisition-cli/src`:
  the text the pages read, and its other readers.
- `git grep -n -E 'GET (plan|jobs)|POST apply|"jobs"' -- crates/acquisition-cli/tests`:
  the tests and goldens of these pages, and those comparing JSON.
- `git grep -n -A30 'pub async fn quote' -- crates/acquisition-client/src/client.rs`
  and `git grep -n 'NotLoggedIn' -- crates/acquisition-client/src crates/acquisition-verbs/src`:
  whether the refusal's kind reaches `Planned`. If only a protocol
  change carries it, Decision 4 stops for the owner.
- `git grep -n -E 'CLIENT_KINDS|MOCK_ONLY_KINDS|fn target_of' -- crates/acquisition-protocol/src`:
  every kind, for its words on a row.
- `git grep -n -E 'pub async fn (job|result)\(' -- crates/acquisition-verbs/src/jobs.rs`:
  an opened job's reads.
- `git grep -n -E 'fn an_answer_item_or_place_page_prints_no_command_line_and_no_flag|Way::Result|fn way_on\(' -- crates`.
- `git grep -n -E 'not_logged_in|no session' -- crates/acquisition-cli/tests/web.rs crates/acquisition-cli/tests/harness`:
  a world with no session.

## Done when
- Every route's JSON answer is unchanged: no test comparing a page's
  JSON with the CLI's `--json` is edited.
- `git grep -n -E 'plan_cmd::|jobs_cmd::|daemon_cmd::' -- crates/acquisition-cli/src/web`
  finds nothing.
- The no-command-line test also reads every answered `GET plan`,
  `POST apply` and `GET jobs` page: no `pre`, no `acq `, no flag, and
  outside attribute values none of `rails`, `tripwire`, `prio`, `eta`,
  `web:`, `cli:`, `-limit`.
- Goldens hold a selection's plan, the policy's with skips and
  uncovered tabs, a quote with known and unknown scopes and one all
  unknown, a refused session, apply's nothing and spent, jobs with a
  parent and children, a waiting job, a failed job opened (a token
  refused, its `error` naming a command), the daemon halted with a
  closed route, and the queue with no daemon.
- A test opens a job the list leaves out, by its URL.
- With a daemon that refuses the session, the plan says it; with none
  running, it says nothing of a session; the CLI's text and the JSON
  are unchanged in both (tests).
- The module doc's "words are still the CLI's" bullet is rewritten; a
  retirement has its line (C154, a retirement names its holder). The
  close closes #189 and says what of #182 is left.
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- 6.5b's and 6.5c's pages; the place pages beyond Q1's answer.
- The refusal block every page shares (`failure_words`).
- A page that retries, cancels, signs in, starts a daemon or resets
  its halt: these pages explain in prose. A success payload beyond its
  link; the CLI's and the MCP's text. The look (#285).

## Rules
`TEMPLATE.md`, and:
- **The plan first.** Run "Find first", write one screen to `plan.md`
  in your notes folder, end the turn and wait.
- Two commits at least: the plan and apply pages, then the jobs page.
- The round is reviewed cold (C175, what is reviewed).

## Report
`TEMPLATE.md`.

## Taken from the review
`roadmap/reviews/4-full-1-step6-plan-jobs-brief-82b66b49.md` and
`…-758587f0.md`. Both found faults in a job's links, derived from its
kind and params; the links are gone, the nav bar being the way to the
places (Decision 7). Not reviewed again (C176, a brief's two): the
build's review is the check.
