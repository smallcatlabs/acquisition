# 4-full-1, step 6.2 — The count

## Goal
The search form shows, beside its Search button, how many items the
form as it stands would match, asked of the server as the form changes
and never drawn into the rows. The owner watched the old app's table
for this and asked for the number (#245, "yes"). The page stays the
answer its URL names; the count is transient state a reload discards.
Closes #245.

## Inputs
- `4-full-1-step6-design.md` §4 ("The count: one read-only route
  answering the form's match count, one script writing it beside the
  button"), §5 item 2, §7 (the route `count`, ruled with the design).
- Issue #245 and its ruling: the count alone, one read-only route,
  folded by the server as a search is (C46, semantics in Rust), under
  the no-state scan the picker passes; live results stay unruled.
- `4-full-1-design.md` §2: transient state only; no storage, cookie,
  `pushState`, rows outliving a navigation, a sequence number of a
  page's own or a value computed from a document; the no-state scan
  and what it cannot hold (behaviour: a review-checklist row).
- C16 (the URL names the state), C138 (routes behind the token and the
  Host guard), C98 (one basis per answer).
- `web/mod.rs`: the router, `search` (fold, `Asked::read`,
  `web.answer`), `stats` and `document` (a JSON reply), `failed`.
  `form.rs`: `fold`. `assets/picker.js`: the one fetching script's
  shape. `page.rs`: the form's controls and its Search button.
  `tests/harness/mod.rs`: `SCRIPTS`, `no_state`. `tests/web.rs`: the
  scan (`c16_no_served_script_names_…`), the page goldens.
- CI on `main` at `2079120d`, the brief's parent, on GitHub's hosted
  runners while the owner's are offline (`SELF_HOSTED_LINUX` and
  `SELF_HOSTED_WINDOWS` false): the builder reads its three runs
  before the build's first push.

## Decisions
1. **What the route answers.** `GET count?<the form's fields>` folds
   the fields as `search` does and answers the answer document itself,
   JSON, with the view replaced by rows at limit 0 and `sort: none`:
   no rows walked and no order derived for a number, the `total` and
   `basis` as a search gives them.
   *Other answer:* a document of its own (`{matched, undecided}`).
   Pick: the answer; no second document to keep true, and its JSON is
   the CLI's `acq --json search … --limit 0`, so it is pinned by
   equality. The view the form names (a sort, a count key, fields)
   is not read: the count is of the query's matches, and a view the
   Search would refuse is the Search's to say. Every form that submits
   to `search` gets the count of its own fields: the fields form and the
   two query boxes (the language, a trade site search), all folded as
   `search` folds them, so one route serves the three.
2. **A refused form.** The route answers the refusal as `search`'s
   JSON does (its status and `{error, kind}`); the script writes
   a dash and sets the refusal's own words as the output's title.
   Nothing composes the words but the server.
3. **A moved basis.** The count is asked with no basis, so it is the
   store's count now, the one Search would answer; it is never
   compared with the page's answer and never refused for a move.
4. **When the script asks.** A form carries `data-count`; the script
   attaches to each such form, on its `input` and `change` events,
   300 ms after the last; the fields are
   `new URLSearchParams(new FormData(form))`, what Search would send.
   The picker writes a stat's id into its row by assignment, which
   fires no event, so it dispatches a bubbling `input` on that row
   after writing; neither script names the other.
   A new ask aborts the one in flight (`AbortController`); an aborted
   reply writes nothing. The script keeps a timer and a controller,
   nothing else: no counter, no version (§2).
5. **Numbers, never words.** Beside each such form's button the
   server writes the label "Matches:" and an `<output>`, then an
   "undecided" part with its own `<output>`, hidden. The script writes
   `total.matched` into the first and `total.undecided.count` into the
   second, showing that part only where it is not zero. Before any
   change and while an ask is in flight: a dash. A failed fetch leaves
   the dash. No word is made in the script; every word is in a golden.
6. **No rows.** The script never touches the answer's rows, its URL
   or the history; the Search button is still how a count becomes the
   page's answer.

## Find first
- `git grep -n "route(&at(" -- crates/acquisition-cli/src/web/mod.rs`:
  the routes, and where `count` joins them behind the guard.
- `git grep -n "SCRIPTS\|picker.js" -- crates/acquisition-cli`: every
  list a served script must join (harness, router, page head).
- `git grep -n "fn wants_json\|fn document\|fn failed" -- crates/acquisition-cli/src/web/mod.rs`:
  how a JSON reply and a refusal are made.
- `git grep -n "search-btn" -- crates/acquisition-cli`: the button and
  every golden holding it.
- `git grep -n "browser-driven\|picker" -- REVIEW-CHECKLIST.md`: the
  row a script's behaviour is held by until the browser check exists.
- `git grep -n "limit: Some(0)\|filter(|_| limit > 0)" -- crates/acquisition-search/src/answer.rs`:
  that a rows view at limit 0 is accepted (explain's forced counts ask
  one) and takes no row and derives no order; if refused, stop.
- `git grep -n "slot.value = " -- crates/acquisition-cli/src/web/assets/picker.js`:
  the picker's write that must dispatch `input`.
- `git grep -n 'action=\\"search\\"' -- crates/acquisition-cli/src/web/page.rs`:
  every form that submits to `search`.

## Done when
- `count?<fields>` for the ring search answers JSON equal to `acq
  --json search` of the same request at `--limit 0`, its `rows`
  empty; a form whose view names a count key or a sort answers the
  same total.
- A refused form's `count` answers the same status and `kind` as
  `search?…` with `Accept: application/json` (a test).
- The form page and an answer page carry, on each form that submits
  to `search`, `data-count` and the labelled outputs beside its button,
  and `assets/count.js` in their head (goldens);
  `harness::SCRIPTS` lists it and the scan reads it, naming no state
  API.
- `count.js` names no `pushState`, storage or cookie; it reads only
  its form and writes only its outputs, numbers alone, and `picker.js`
  dispatches `input` after a pick (the review checks both: a
  `REVIEW-CHECKLIST.md` row for a script's behaviour, which the
  browser-driven check will take).
- `tools/gate.sh` exits 0 on the step's tree, and `--only doc`.

## Not in scope
- Live results, the rows redrawn as the form changes: unruled (#245).
- A count beside a saved search or on any page without the form.
- The browser-driven check: the spike's (step 3).
- The flat table, the sort box's removal (step 4).

## Rules
`TEMPLATE.md`. Every `acq` and `acq web` in a mock world (the
mock-session skill); nothing is opened in a browser by the builder.

## Report
`TEMPLATE.md`.
