# 4-full-1-form-landing — The landing rule

## Goal
Each submit, and each refused write or plan, leads back to its page
holding what was sent (`4-full-1-form-design.md` §8 step 5): a request
that leaves a page carries that page as `from`, and what answers it
draws one way back from it. The league list's job gets its way back
(#194's last comment), and the save form's and the plan's refusals keep
their input (#190's rest). Closes #190.

## Inputs
- `4-full-1-form-design.md` §5, §8 step 5, §9 (`from` is the
  session's), §10. Step 4's close, `b7b083d8`.
- #194's last comment, #190's last two.
- C16 (a page is its URL), C47 (malformed input is a structured
  error), C138 (the listener: no new route, mutations by POST), C143
  (a refusal speaks the shared words), C154 (a retirement names what
  holds it), C153 (one golden per wording).
- Code: `web/mod.rs` (the module doc, the routes, `list`,
  `fetch_leagues`, `to_job`, `plan`, `apply`, `plan_again`, `jobs`,
  `places` and `place_fields`, the saved routes, `Web::written`,
  `Web::failed`, `switched`, `opened_from`, `taken`, `see_other`,
  `Paged`); `web/page.rs` (`refused`, `post_form`, `save_forms`,
  `fetch_leagues`, `places`, `jobs`, `PlaceLinks`);
  `acquisition_verbs::refusal`; `tests/web.rs`'s `landed_at_a_job`.
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows) at `b7b083d8`: Linux 45207522545 failure, Windows
  45207522489 success. The failure is the Dependencies job's advisory
  on `url` (RUSTSEC-2026-0592), no build or test: not this step's,
  and brought to the design session to file.

## Decisions
The design session's (C172, who designs), each settled:
1. **`from` is a typed value**: a place of this server and its fields,
   written as its route and query (`tabs?realm=…`, no token). One maker
   makes it from a request, one reader reads it. A `from` naming no
   place, or holding a token or a `from`, is a usage refusal, never
   dropped. *Declined:* the `Referer` header, which a browser or a
   setting trims with no test of the gate seeing it, while the landing
   URL must carry the way back anyway (C16).
2. **A route is a place or a step.** Places: the search page (`.`,
   `search`), `tabs`, `characters`, `saved`, `jobs`, an item's page.
   Steps: `plan`, the clear's ask (`saved/clear` GET) and every POST. A
   place's `from` is itself, its route and its fields less `from`; a
   `from` its URL carries is read only as its own way back. A step
   hands on the `from` it was given, and one reached with none has
   none. So `from` never nests and no way back leads to a step.
3. **No leaving form without it**: every maker of a form that posts
   (`post_form`, the save form, the listing, the league list's fetch,
   Apply) and `refused`, whose offers post, takes the page's `From` as
   a parameter and writes its hidden field. No `method="post"` stands
   in the web module outside those makers.
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
   nothing draw the same way back, and `plan_again` hands it on.
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
- `git grep -n 'to_job(\|see_other(\|\.written(\|\.failed(\|failed_under(' crates/acquisition-cli/src/web/mod.rs`:
  every answer and refusal page of a route.
- The refusals the leaving forms can meet (§5's list):
  `git grep -n -e 'Failure::Usage(' -e 'anyhow::anyhow!(' -e 'listed("' crates/acquisition-cli/src/web/mod.rs`
  (each in a leaving route's body), `git grep -n -e 'kind: "' -e '=> "[a-z_]*",$' crates/acquisition-verbs/src/fault.rs crates/acquisition-verbs/src/saved.rs`,
  `sed -n '/pub enum PlanError/,/^}/p' crates/acquisition-plan/src/lib.rs`
  and `sed -n '/pub enum ErrorKind/,/^}/p' crates/acquisition-protocol/src/protocol.rs`:
  the report lists each a leaving route can return, by route.
- `git grep -n 'landed_at_a_job\|jobs?job=\|class=\\"places\\"\|"leagues"' crates/acquisition-cli/tests/ ':!*/goldens/*'`
  and `git grep -ln 'class="places"\|formaction' crates/acquisition-cli/tests/goldens/`:
  the tests and goldens of the landing and the jobs page's links.
- `git grep -n 'switched(\|opened_from(\|taken(' crates/acquisition-cli/src/web/mod.rs`:
  where a route takes its page fields out first.

## Done when
- From each of: the search page's save form, a saved row's rename, the
  clear's ask, a refusal's offered write, the league list's fetch from
  the search page and a place page, each listing, and Apply after a
  plan asked from a ticked place page, the submit lands, and the page
  it lands on links back to the page as sent; the league list's job
  among them (`tests/web.rs`, one test or one table).
- A refused save (an empty name, a conflict) renders the refusal, the
  save form with its name as sent and its source kept, and the way
  back to the search page as sent; a refused plan (nothing ticked, no
  policy, no session) the refusal and the way back to the place page
  with every tick and open (goldens).
- A test whose table is §5's list, its rows the report's hits: each
  refusal's page holds an offer a page takes, or words naming its
  command as a person's act at a terminal; a hit with no row is named
  in the commit's message with why no page meets it.
- `from` read: a place's own value, a token, an absolute URL, a nested
  `from` and a step each refused as usage; a place's URL with `from`
  draws it and writes none into its own forms (tests).
- `git grep -c 'method=\\"post\\"' crates/acquisition-cli/src/web/`
  counts only the makers' lines, and `formaction` is found nowhere in
  `crates/acquisition-cli/src/`.
- `PlaceLinks` is gone; each test retired carries its `retired …;
  held by …` line (C154).
- Goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each diff read;
  the module doc's landing and place-page paragraphs say what is built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- The jobs page's words (#182's rest), the never-fetched tabs of
  another league (#194's rest), the look (#285).
- A new route, the `Referer` header, a way back in a script.
- The value finder (step 6); any change to a verb or the protocol.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
