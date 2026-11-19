# 4-full-1, step 6.5d — The plan and jobs pages from their documents

## Goal
At `acq web`, the plan, apply and jobs pages render their documents
in a player's words, as the place pages do since 6.5a: no `pre`, no
command line, no daemon jargon (rails, prio, eta, `web:<pid>`). A job
links to what it fetched and a failed one to why, and the plan says
where it is made when the account has no session. This closes #182's
plan and jobs half and what is left of #189 (fault 12).

## Inputs
- #182 and its two comments (the blind page seats, T6 and T7); #189,
  fault 12; 6.5a's close, `git log --grep='^Step: 4-full-1-step6-places'`,
  which says what each left.
- `4-full-1-step6-design.md` §3, "After an apply": the reader is back
  at the places, whose facts say what the fetch brought.
- C16 (a page holds only what its URL names), C38 (plans are binding),
  C46 (semantics in Rust), C53 (JSON is the contract), C138 (the
  listener; its routes were the owner's drafts, #284), C165 (a failed
  job's kind and way on), C143 (a way on is spelled per surface).
- The documents: `Planned` and `Apply` (verbs `refresh.rs`), `Jobs`,
  `Listed`, `PersistedJobs`, `JobResult` (verbs `jobs.rs`), `JobInfo`
  (protocol `job.rs`), `RailsStatus` (protocol `status.rs`).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`; at writing, runs
  44455846955 and 44455847035 were in progress.

## Decisions
1. **No document changes.** Each route's `application/json` answer
   stays byte for byte; no protocol key, no field on a verb's document,
   no route name. A page needing one stops for the owner.
2. **Renderers read documents, never the CLI's text.** The pages live
   in `web/page.rs` beside the place pages and read the types above.
   `render_plan_on_a_page`, `jobs_text`, `persisted_queue_text` and
   `rails_text` leave `web/`; one whose last reader went retires.
3. **The plan page:** what is refreshed (realm, league, the selection
   or the policy), the facts' ages from `PlanAges`, `PlanNotes` in
   words, the actions as a table (what is fetched, why), the skipped
   and the uncovered counted in words, the quote as one line saying
   when the work can start (ready, a wait, unknown until GGG answers,
   or halted, from `Quote.halted` and `Quote.scopes`), and Apply. The
   wire estimate and scope keys stay in the JSON. `<meta name="basis">`
   as now.
4. **No session, said on the plan** (#189, fault 12). The quote's
   refusal keeps its kind: `QuoteNote` gains a variant for the
   daemon's `not_logged_in`, whose `Display` is today's sentence, so
   `quote_note` is unchanged. The page then says the account is not
   signed in and Apply will be refused until it is. Apply stays: the
   quote is an observation, the apply's refusal is the authority.
5. **The apply page** says `Apply::Nothing` and `Apply::Spent` from
   their fields (requests, spent, the latest's age, what a plan now
   would fetch), with a link to the plan compiled again.
6. **The jobs page:** a table of `Listed.rows`, a child indented under
   its parent where both are listed: what (the kind in words and
   `JobInfo::target`), its state, when (C165's latest time), and a
   waiting job's start in words from `eta_seconds`. Counts and what is
   left out in words, no flag. Of `RailsStatus`, only what stops sends
   (`halted`, `closed_routes`, `refresh_failed`), in words with no
   command line; the rest is the terminal's. No daemon: the ledger's
   rows in the same table under one line of how old and what resumes
   them. The reload while a job runs or waits stays.
7. **A job's ways.** A finished job links to the page that shows what
   it fetched: `leagues` the tabs page; `stashes` and `stash` its
   league's tabs; `characters` and `character` the characters page;
   `apply` its plan's league's places. A failed or cancelled job opens
   its `JobResult` (see the open question): the error, the first
   failed children, `WayOn.words` and its offers spelled by `Paged`,
   which spells `Way::Result` and leaves `Way::Submit` unspelled.

## Open for the designer (C172, Fable designs)
Each answer amends this brief before the build starts.
- **Q1, where an apply lands.** The design says back at the places;
  the facts there only change once the job lands. Recommended: Jobs
  with the apply's job opened, reloading until done, its link the
  places. The other reading, the places page itself, makes that page
  read the jobs too.
- **Q2, where a job's result shows.** Recommended: opened under its
  row, `jobs?job=<id>`, the URL naming it as `open` does on the
  places; no route name. The other, a route of its own, is a C138
  route, taken under delegation as #284 was.

## Find first
- `git grep -n -E 'render_plan_on_a_page|jobs_text|persisted_queue_text|rails_text|fn pre\(' -- crates/acquisition-cli/src`:
  the text the pages read, and its other readers.
- `git grep -n -E 'GET (plan|jobs)|POST apply|"jobs"' -- crates/acquisition-cli/tests`
  and `ls crates/acquisition-cli/tests/goldens/web`: the tests and
  goldens that hold these pages, and those comparing their JSON.
- `git grep -n -A30 'pub async fn quote' -- crates/acquisition-client/src/client.rs`
  and `git grep -n 'NotLoggedIn' -- crates/acquisition-client/src crates/acquisition-verbs/src`:
  whether the quote refusal's kind reaches `Planned`. If only a
  protocol change carries it, Decision 4 stops for the owner.
- `git grep -n -E 'fn target_of|"(stashes|stash|characters|character|leagues|apply)" =>' -- crates/acquisition-protocol/src crates/acquisition-daemon/src/daemon/kinds.rs`:
  each kind's params, for Decision 7's links.
- `git grep -n -E 'fn an_answer_item_or_place_page_prints_no_command_line_and_no_flag|Way::Result' -- crates`.
- `git grep -n -E 'not_logged_in|no session' -- crates/acquisition-cli/tests/web.rs crates/acquisition-cli/tests/harness`:
  a world whose account has no session.

## Done when
- Every route's JSON answer is unchanged: no test comparing a page's
  JSON with the CLI's `--json` is edited.
- `git grep -n -E 'plan_cmd::|jobs_cmd::|daemon_cmd::' -- crates/acquisition-cli/src/web`
  finds nothing.
- The no-command-line test also reads every answered `GET plan`,
  `POST apply` and `GET jobs` page: no `pre`, no `acq `, no flag.
- Goldens hold a selection's plan, the policy's with skips and
  uncovered tabs, a plan with no session, apply's nothing and spent,
  a jobs page with a parent and its children, a waiting job, a failed
  job opened with its way on, the daemon halted with a closed route,
  and the queue with no daemon. `jobs_halted` names no command.
- A test follows each finished kind's link to its page, and an
  apply's to its league's places, whose facts say what landed.
- `QuoteNote`'s new variant: the CLI's text and the JSON for no
  session are unchanged (test).
- The module doc's "words are still the CLI's" bullet is rewritten;
  a retired function or test has its line (C154, a retirement names its holder). The close closes
  #189 and says what of #182 is left (the refusal block's `pre`).
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- 6.5b's and 6.5c's pages; the place pages beyond Q1's answer.
- The refusal block shared by every page, its words `failure_words`.
- A page that retries, cancels or signs in; a job's success payload
  rendered beyond its link; the CLI's or the MCP's text.
- The look (#285).

## Rules
`TEMPLATE.md`, and:
- **The plan first.** Run "Find first", write one screen to `plan.md`
  in your notes folder, end the turn and wait.
- Two commits at least: the plan and apply pages, then the jobs page.
- The round is reviewed cold (C175, what is reviewed).

## Report
`TEMPLATE.md`.
