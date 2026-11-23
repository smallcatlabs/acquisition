# 4-full-1-form-landing — The landing rule

## Goal
Each submit, and each refused write or plan, leads back to its page
holding what was sent (`4-full-1-form-design.md` §8 step 5): a request
that leaves a page carries that page as `from`, and what answers it
draws one way back from it. The league list's job gets its way back
(#194), and the save form's and the plan's refusals keep their input
(#190's rest). Closes #190.

## Inputs
- `4-full-1-form-design.md` §5, §8 step 5, §9 (`from` is the
  session's), §10. Step 4's close, `b7b083d8`.
- #194 comment 7056915725; #190 comments 7020795874, 7065908074.
- C16 (a page is its URL), C47 (malformed input is a structured
  error), C138 (the listener: no new route, mutations by POST), C143
  (a refusal speaks the shared words), C154 (a retirement names what
  holds it), C153 (one golden per wording).
- Code: `web/mod.rs` (its doc, the routes, every POST handler, `plan`,
  `jobs`, `places`, `Web::failed`, `Paged`); `web/page.rs` (`refused`,
  `post_form`, `save_forms`, `places`, `jobs`).
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows) at `b7b083d8`: Linux 45207522545 failure, Windows
  45207522489 success: the advisory on `url` (`gh run view 45207522545
  --log-failed`), not this step's, filed by the design session before
  the build.

## Decisions
The design session's (C172, who designs), each settled:
1. **`from` is a typed value**: a place of this server and its fields,
   written as its route and query (`tabs?realm=…`, no token). One maker
   makes it from a request, one reader reads it. A `from` naming no
   place, or holding a token or a `from`, is a usage refusal, never
   dropped. *Declined:* the `Referer` header, which a browser may trim
   unseen by the gate, while the landing URL must carry the way back
   anyway (C16).
2. **A route is a place or a step.** Places: the search page (`.`,
   `search`), `tabs`, `characters`, `saved`, `jobs`, an item's page.
   Steps: `plan`, `saved/open`, the clear's ask and every POST. A
   place's `from` is itself, its route and its fields less `from`; a
   `from` its URL carries is read only as its own way back. A step
   hands on the `from` it was given, and one reached with none has
   none. So `from` never nests and no way back leads to a step.
3. **Nothing reaches a step without it**: every maker of a form that
   posts (`post_form`, the save form, the listing, the league list's
   fetch, Apply), of a link to a step (a saved row's opening and
   Clear, `expand` kept; `plan_again`; `Paged`'s plan and opening) and `refused`, whose offers
   post, takes the page's `From` and writes it. No `method="post"`
   stands in the web module outside those makers.
4. **The place form never leaves.** "Refresh ticked" and "Plan under
   the policy" become fields of its own page, no `formaction`; the
   place page answers such a field `303` to `plan` with the selection
   as sent and the page as sent, less that field, as `from`. A refused
   plan's way back then holds every tick and open, and "Show" carries
   no `from`. No route is added (C138).
5. **Two answers to a leaving request.** *Landed*: `303` to the next
   page (a job, the saved list) carrying `from`, left out where `from`
   is that page. *Refused*: the refusal in the shared words (C143),
   then the form as sent where the route has one a reader fills (the
   save form, by its one maker, its name as sent and its source kept),
   then the way back. `to_job` takes the job and `from` and decides
   nothing else. The plan page, answered, and an apply that sent
   nothing draw the same way back.
6. **The jobs page draws one way back from `from`**, as every place
   does with one its URL carries, by one function; its `realm` and
   `league` fields and `PlaceLinks` retire.
7. **`from` is the page's**: each route takes it out of its fields
   first, as `switched` takes the switch, so no fold, verb or JSON
   answer meets it; a route that forgets refuses it as a field it does
   not take where its fields are checked (`taken`, the fold).

## Find first
- `git grep -n 'method=\\"post\\"\|formaction\|post_form(\|fetch_leagues()' crates/acquisition-cli/src/web/`:
  every form that leaves, and its makers.
- `git grep -n 'to_job(\|see_other(\|\.written(\|\.failed(\|failed_under(\|page::refused(' crates/acquisition-cli/src/web/mod.rs`:
  every answer and refusal page of a route.
- The census, the refusals the leaving requests can meet (the design's
  §5): `git grep -n -e 'Failure::Usage(' -e 'anyhow!(' -e 'listed(' crates/acquisition-cli/src/web/mod.rs`
  (a leaving route's body), `git grep -n -e 'kind: "' -e 'arguments(' crates/acquisition-verbs/src/saved.rs`,
  `sed -n '/pub enum Fault /,/^}/p' crates/acquisition-verbs/src/fault.rs`,
  `sed -n '/pub enum PlanError/,/^}/p' crates/acquisition-plan/src/lib.rs`
  and `sed -n '/pub enum ErrorKind/,/^}/p' crates/acquisition-protocol/src/protocol.rs`:
  each a candidate, traced through the verbs each leaving route calls;
  the report lists each the route can return, by route.
- `git grep -n 'landed_at_a_job\|place_form(\|jobs?job=\|class=\\"places\\"\|c168_\|saved/' crates/acquisition-cli/tests/ ':!*/goldens/*'`
  and `git grep -ln 'class="places"\|formaction\|method="post"\|saved/clear?' crates/acquisition-cli/tests/goldens/`:
  the tests and goldens of every leaving form and link; a caller of
  `place_form` follows the new `303` before reading the plan.
- `git grep -n 'switched(\|opened_from(\|taken(' crates/acquisition-cli/src/web/mod.rs`:
  where a route takes page fields out.

## Done when
- From each of: the search page's save form, a saved row's rename, the
  clear's ask, a refusal's offered write, the league list's fetch from
  the search page and a place page, each listing, and Apply after a
  plan asked from a ticked place page, the submit lands on the page as
  sent or on one that links back to it, a saved list's own `from`
  left out; the league list's job among them (`tests/web.rs`, one
  table). The replan links after an apply that sent nothing and after
  a moved policy, followed through a plan and an apply, keep it.
- A refused save (an empty name, a conflict) renders the refusal, the
  save form with its name as sent and its source kept, and the way
  back to the search page as sent; a refused plan (nothing ticked, no
  policy) the refusal and the way back to the place page with every
  tick and open (goldens). A plan with no session still compiles.
- A test whose table is the census: each refusal's page holds its
  form as sent, or an offer a page takes, or words naming its command
  as a person's act at a terminal. The commit's message lists the
  census by route, and each hit with no row and why no page meets it.
- `from` read: a step, a route no place names, a token, an absolute
  URL and a nested `from` each refused as usage; a place's URL with
  `from` draws it, and its own forms carry the place, never that
  `from` (tests).
- `git grep -c 'method=\\"post\\"' crates/acquisition-cli/src/web/`
  counts only the makers' lines, and `formaction` is found nowhere in
  `crates/acquisition-cli/src/`.
- `PlaceLinks` is gone; each test retired carries its `retired …;
  held by …` line (C154).
- Goldens regenerated under `ACQ_UPDATE_FIXTURES=1`; the module
  doc's landing and place-page paragraphs say what is built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- The jobs page's words (#182), another league's never-fetched tabs
  (#194's rest), the look (#285).
- A new route, a way back in a script.
- The value finder (step 6); a change to a verb, the protocol, or
  what a plan or a quote compiles.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). Read each golden's regenerated diff. A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
