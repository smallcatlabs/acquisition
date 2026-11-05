# 99-web-leftovers — the no-account page and smaller page goldens

Issue #99, built at the owner's word of 2026-11-04 (the night run),
`trigger:gui` waived. Written by the orchestrating session against the
tree at `4b4a1c03`, #56's close. Two of its three items; the third is declined
below, with its reason, for the owner.

## Goal

`acq web` started with no account on disk refuses in the terminal, so
the person who ran it to look at the tool meets a shell line instead
of the page the tool is; and the page goldens are whole pages (the
search pages' 62,800 bytes, the console's 56,257), so a one-line edit
to the search form changes every page that carries it. When this
step is done the no-account refusal is a page, and a form edit
changes one small golden.

Item 2 (a test seam into `picker.js`, its no-fallback rule pinned) is
declined at the session's call, advised by Fable: the rule (a pick
fills the first empty stat row, never a written one) is presentation,
not shared semantics (C16: the page composes nothing; the matches and
their fates are already the server's); a JS engine and a DOM shim
would be a second DOM to maintain for 49 lines, and moving the pick
server-side adds a route and a round trip for one line. The issue's
own trigger stands: a seam is the day's work when a second script rule
needs one. A "Not doing" line and a checklist row go to the owner.

## Read

- `AGENTS.md`; `decisions/frontends.md`: C16 (a renderer, not an app),
  C121 (an account that cannot be picked: `not_logged_in` when none is
  known, with its remedy), C138 (the listener), C139, C122 (a wording
  golden names the facts a reader must find).
- `REVIEW-CHECKLIST.md`, the frontends section.
- `crates/acquisition-cli/src/web/mod.rs`: the module doc's "One
  account, named on every call" (the account resolved once, at the
  start), `web` (about line 148: `cx.account()?` refuses before the
  bind); `page.rs`'s `refusal` renderer.
- `crates/acquisition-cli/tests/web.rs`: `offline_web`, the
  `page_golden` callers (lines 471, 526, 583, 778, 822);
  `tests/harness/mod.rs`'s `page_golden` and `Pages`.
- `crates/acquisition-cli/tests/goldens/web/` and their sizes
  (`ls -l`).

## Input

The golden sizes at `6f2e3337`: `search_pages.txt` 62,800,
`console_realm.txt` 56,257, `moved_world.txt` 8,868,
`refresh_loop.txt` 7,933, `jobs_halted.txt` 606. The build measures
which part repeats (expected: the search form, on every search page)
and names it in the commit before cutting.

## Do

1. **No account is a page.** `acq web` with no account on disk binds,
   prints its URL and serves; every page renders the `not_logged_in`
   refusal in C121's words with its remedy (sign in with `acq auth`).
   The account is still resolved once and then fixed for the server's
   life (the module doc's rule): the session's choice is that a
   server started with none resolves at the first request that finds
   one and holds it from then on, so signing in and reloading serves
   the page, and a later login never moves the server to another
   account. Say this in the module doc. An account named with
   `--account` that is not known stays a terminal refusal (a typo is
   the command's, C121's `unknown_account`). Pin: a case in `web.rs`
   (C153) — no account, the page's refusal with its remedy; then an
   account appears and a reload serves its home page. The refusal's
   words held by a golden (an existing page golden gains the page).
2. **Pin the part a change names.** The repeated part of the page
   goldens (the build names it) is pinned once, in a golden of its own;
   the page goldens hold each page's own facts without it (a page
   carrying it says so in one line). Every `# holds` fact still checked
   (`Pages.facts`). Measure after: the sizes of every web golden, in
   the commit. Success is that a one-line edit to the form changes
   one golden; demonstrate it in the report (make the edit, run under
   `ACQ_UPDATE_FIXTURES=1`, count the changed goldens, revert).
3. `CLI-REFERENCE.md` and the README tour regenerated or edited if
   `acq web`'s help or behaviour as a user meets it changes (the README
   has a line for `acq web`; one sentence for the no-account page).

## Done when

- With no account, `acq web` serves a page refusing as `not_logged_in`
  with its remedy; the server takes the first account it finds and
  keeps it; pinned in `web.rs`.
- The repeated part of the page goldens is one golden; the commit
  carries every web golden's size before and after; the report shows
  one form edit changing one golden.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- `picker.js` and any script seam (declined above).
- Any page's words beyond the no-account page (#97 is the web's
  phrasing).
- The listener's rules (C138).

## Rules

`TEMPLATE.md`. Main checkout on `main`, commit locally, no push; every
`acq`/`acq-mcp` in a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR`
under your scratch), never a bare `acq` or a real-provider daemon; the
web server is started only by the tests' harness or under the mock
world, never on the owner's store; C153, C154; goldens regenerated
under `ACQ_UPDATE_FIXTURES=1`, every changed golden read as a diff
(shape 9); one commit per item naming this brief.

## Report

`TEMPLATE.md`.
