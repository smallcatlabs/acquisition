# 4-full-1-form-rides — What rides on the form

## Goal
The form says only what a player needs from it, as the site's does:
"assume 20% quality", ticked; what is not run hidden until asked for;
a fate only where a control is not the site's; a site search pasted
in the bar; Clear beside Search (`4-full-1-form-design.md` §4 less
the sockets, §2's bar and Clear; §8 step 4). Closes #180, #181, #188.

## Inputs
- `4-full-1-form-design.md`: §2 "One page" and "A site search is one
  box and one button", §4, §8 step 4, §9's choices 1, 3, 4, §10.
  Step 3's close, `12eb180c`, its "Raised" and "Left open".
- #180, #181 (its comment: the group types not run), #188.
- C142 (the defences' reading beside the query), C46 (semantics in
  Rust), C187 (a script's rule has a browser-driven check), C138 (no
  new route), C16 (a page is its URL), C153 (one golden per wording).
- Code: `web/page.rs` (`search_form`, `fate_span`, `control`,
  `stat_row`, `stat_group`, `stat_pane`, `stat_options`, `trade_box`),
  `web/form.rs` (the fold, `normal`), `web/fill.rs`, `web/mod.rs`
  (`home`, `opened_from`, `count`), `assets/count.js`, `page.css`,
  `tests/harness/mod.rs` (`FORM_PANES`), `tests/harness/submitted.rs`.
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows): in progress at `12eb180c`, runs 45191688393,
  45191688490.

## Decisions
1. **The quality box** stands where the defences' box stands, worded
   "assume 20% quality", ticked by default: a hidden
   `defences=displayed`, then the box sending `defences=q20`. The
   normal form drops `defences: "q20"`, the site's own reading, as it
   drops a `disabled` of false, so the partition holds. The armour
   panel opens where the box is unticked. The CLI is untouched.
2. **A fate is said where it is not exact** (`fate_span`, a select's
   option, a group type's option, `stat_row` and `stat_options`): an
   exact one renders nothing, mark or title. A stat's two statuses are
   said where either is not exact; its group word (`explicit`) stays,
   as it tells two stats of one text apart.
3. **The switch** is a box at the page's foot, after the answer, tied
   to the search form by `form=` (the form takes an id), named
   `all_fields`, value `true`, worded "Show the fields this search does
   not run", with one line beside it: "A field with no mark runs as
   the trade site's; a mark says it is read differently here, or not
   run." `opened_from`'s partition takes it out with `saved`, so
   `search` and `count` never fold it; `home` and `search` render it
   ticked where the URL names it. No link, other form or Clear carries
   it (§10, no remembered switch).
4. **What the switch hides** is marked by the server, `data-unrun`:
   a control, a select's option, a group type's option and its "Add
   Stat Group" control, whose catalogue status is neither `exact` nor
   `ambiguous` (what `Status::runs` runs) and that holds no value
   (the fill's at its path; an option, selected). A panel every control of which is
   marked is marked. One rule in `page.css` hides `[data-unrun]` while
   the switch is unticked (`:has(…:checked)`), as step 3's type rules.
5. **The site box in the bar**: its text and button after the item
   search, owned by an empty `<form id="trade-form">` before the search
   form by `form=`, as `realm-form` is, with its own hidden realm and
   its count. `count.js` finds a form's outputs and hears its controls
   through `form.elements` and an event's `target.form`, never by
   descent, so neither form's count asks for the other's fields.
6. **Clear** is a link beside Search to `.` with the scope alone:
   `realm`, and `membership` where it is not `live`; `home` reads
   `membership` as `search` reads it.
7. **The golden's cut** (`FORM_PANES`) starts at the search bar's
   opening line, so the bar, the site box and Clear are in
   `web/search_form`; the switch stands in each page's golden.
8. **Open, for the design session**: whether the switch hides a stat
   the finder lists that runs neither alone nor bounded. Until it
   answers, the finder lists every stat, each with its fate.

## Find first
- `git grep -n 'read these as the item stands\|"defences"\|name=\\"defences' crates/acquisition-cli/src/web/ crates/acquisition-cli/tests/ ':!*/goldens/*'`:
  the quality box and its readers.
- `git grep -n 'fate_span(\|status_words(\|status.replace' crates/acquisition-cli/src/web/`:
  every place a fate is said.
- `git grep -n 'fate-\|(exact)' crates/acquisition-cli/tests/ ':!*/goldens/*'`:
  tests that hold a fate's mark.
- `git grep -n 'trade_box\|trade-text\|realm-form\|form=' crates/acquisition-cli/src/web/`:
  the site box and the `form=` controls.
- `git grep -n '"saved_revision"\|opened_from(' crates/acquisition-cli/src/web/`:
  the readers of page fields beside the fold.
- `git grep -n 'FORM_PANES\|panes_golden' crates/acquisition-cli/tests/`:
  the cut and its pages.
- `git grep -n 'querySelector\|addEventListener' crates/acquisition-cli/src/web/assets/count.js`:
  what the count finds by descent.

## Done when
- `web/search_form` holds the five, empty: the ticked quality box, the
  site box in the bar, Clear, no `fate-exact` and no `(exact)`; a page
  golden holds the switch and its line, unticked. Filled: a page
  golden holds a form of `defences: "displayed"` with the box
  unticked, the switch ticked by its URL, a non-exact fate with its
  mark, a site search's text in the bar's box where its decode refused
  it, and Clear's link carrying a removed membership.
- `form.rs` tests: the box ticked folds to no `defences`, unticked to
  `"displayed"`; the partition pin holds with the box and the switch
  (each fill case of the pin unchanged).
- `search` and `count` take `all_fields=true` beside the form's fields
  and answer as without it (a test).
- In `tests/browser.rs`, one test: unticked, a control not run and
  empty and a not-run type in a type select and in "Add Stat Group"
  are not displayed, a not-run control holding a value is; ticked,
  all are, with no request sent. Typing a site search in the bar's box
  moves its count and not the form's, and the reverse; running it
  fills the form.
- Clear, followed from a filled page, renders the empty form of that
  realm and membership (a test).
- Goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each diff read;
  the web module's docs state what is built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- Abyssal sockets (§4's last bullet): step 8. The landing rule: step 5.
- The CLI's `--defences` and C142's words (§4: C142 stands).
- Moving the query box, the look (#285), a remembered switch.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
