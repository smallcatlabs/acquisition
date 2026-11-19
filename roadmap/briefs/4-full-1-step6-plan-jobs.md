# 4-full-1, step 6.5d — The plan and jobs pages from their documents

## Goal
At `acq web`, the plan, apply and jobs pages render their documents
in a player's words, as the place pages do since 6.5a: no `pre`, no
command line, no daemon jargon. A job links to what it fetched and a
failed one to why, and the plan says when the daemon has refused the
account for having no session. This closes #182's plan and jobs half
and #189's fault 12.

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
   could go, so the page says the largest known one as "all sent within
   about …", counts each scope without one as not yet known, and says a
   `halted` quote sends nothing. Wire estimate and scope keys stay in
   the JSON.
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
7. **A job's ways,** only for a job whose account the server serves
   (`web.serves`); another's row names its account and links nothing.
   Done: `leagues` links the tabs page; `stashes` and `stash` their
   league's tabs; `characters` and `character` the characters page;
   `apply` the tabs of the league its children name, else its realm's
   characters page, its league picked there (character params name
   none). Other kinds (`profile`, `probe`, mock kinds) link nothing.
   Failed or cancelled opens its `JobResult` (Q2): the error, the first
   failed children, offers spelled by `Paged`, which now spells
   `Way::Result`. Its words are `WayOn.words` except where those name a
   command (`NotLoggedIn`, `NotRetried`): there the page's own, by kind.

## Open for the designer (C172, Fable designs)
Each answer amends this brief before the build starts.
- **Q1, where an apply lands.** The design says back at the places,
  but their facts change only once the job lands. Recommended: Jobs,
  the apply's job opened and reloading until done, its link the places.
  The other: the places page, which then reads the jobs too.
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
- `git grep -n -E 'CLIENT_KINDS|MOCK_ONLY_KINDS|fn target_of|fn job\(&self\)' -- crates/acquisition-protocol/src crates/acquisition-plan/src`
  and `git grep -n -E '"(profile|leagues|character)"' -- crates/acquisition-daemon/src/daemon/kinds.rs`:
  every kind and its params, for Decision 7.
- `git grep -n -E 'fn an_answer_item_or_place_page_prints_no_command_line_and_no_flag|Way::Result|fn way_on\(' -- crates`.
- `git grep -n -E 'not_logged_in|no session|fn serves' -- crates/acquisition-cli/tests/web.rs crates/acquisition-cli/tests/harness crates/acquisition-cli/src/web`:
  a world with no session, and a second account.

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
  uncovered tabs, a quote with known and unknown scopes, a refused
  session, apply's nothing and spent, jobs with a parent and children,
  a waiting job, a failed job opened (one `NotLoggedIn`), the daemon
  halted with a closed route, and the queue with no daemon.
- A test follows each linked kind to its page, an apply to its places;
  another account's finished job links nothing.
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
`roadmap/reviews/4-full-1-step6-plan-jobs-brief-82b66b49.md`, all
eight: 1 the quote's last-request wait (D3); 2 an apply's league (D7);
3 another account's job (D7); 4 words naming a command (D7); 5 a
session observed only by a refusal (D4); 6 the jargon list; 7 every
kind; 8 no daemon controls.
