# 4-full-1-form-rows — Rows and groups

## Goal
A search of two stat groups, one weighted, is made, edited and run
from the page with no id typed (`4-full-1-form-design.md` §8, step 3;
#184). A stat row has one maker, in Rust, and the page's script only
copies and removes what the server rendered.

## Inputs
- `4-full-1-form-design.md`: §2's "Rows and groups as the site's", §8
  step 3, §10. Step 2's close, `44a4063c`.
- #184 with its comments.
- C46 (semantics in Rust), C16 (a page is its URL), C187 (a script's
  rule has a browser-driven check), C138 (the listener: no new route),
  C154 (a retirement names what holds it).
- Code: `web/page.rs` (`stat_groups`), `web/form.rs` (the fold),
  `web/fill.rs` (the partition pin), `web/mod.rs` (`stats`; `rows` for
  its HTML answer), the scripts under `assets/`, `tests/browser.rs`;
  the search crate's `stat_value_keys`, `group_value_keys`.
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows): success at `06320e4b`, runs 44930037421, 44930037634.

## Decisions
1. **One row maker.** A function in `page.rs` renders a stat row from
   its vocabulary entry (or an id no table holds) and its values: the
   id a hidden `stat.id`, then the on box, the texts, min, max, weight,
   an option box where the row holds one, the fate, a remove button.
   The filled form and the finder's listing both call it. No row is
   typed: `STAT_ROWS` goes, and an empty form is one `and` group with
   no row.
2. **The listing carries each row.** `stats` answers JSON where JSON is
   asked, as now, and otherwise an HTML fragment, as `rows` does: each
   entry an option whose control holds a `<template>` of its row, and
   the left-out count in the server's words. The fate sentence is then
   written once.
3. **Two verbs, delegated, in one script** that replaces `picker.js`,
   one click listener on the stat pane. An add control copies the
   template it holds, or the one its attribute names, into the item
   list of its nearest list ancestor; a remove control removes its
   nearest item. Then one bubbling `change` is dispatched from the item
   list that changed, which outlives a removal, and after an add the
   copy's element marked for focus takes it (a row's min, a group's
   finder; a data attribute, never `autofocus`). The script names no
   field and writes no word and no value a form sends.
4. **Groups.** One `<template>` per group type, the type selected by
   the server. After the last group, "Add Stat Group": a `details` menu
   of add controls, one a type, which an add closes. Each group ends
   with its own finder, so a pick lands in its own group. A group has
   a remove control. The fold drops a group with no row, so a search's
   page shows none: no fault.
5. **What a type shows is CSS.** Every row renders a weight box and
   every group its bound. Each type's `<option>` lists the positional
   fields it takes, written from the two key lists. `page.css` hides a
   field its group's checked option does not list
   (`:has(option:checked)`), one rule a field, naming no type. The fill
   gives a box a value only where the type takes it. Try the
   selector in the gate's browser first; if it does not follow a
   changed select, stop and report.
6. **The finder** is a box no form sends and a listbox, with no id. It
   asks `stats`, puts the answer's fragment in its list and marks the
   first option. Arrows move the mark, an attribute in the DOM; Enter
   clicks the marked option and never submits. One function empties a
   finder, its box and its list, and retires its asks in flight, so no
   late reply fills it again: Escape calls it, and the click listener
   does after the verb, for a click on an option. It knows nothing of
   what an option's click does.
7. **The count follows the form's fields.** `count.js` asks when the
   form's data differs from the data it last asked, on any `input` or
   `change` within the form; its filter on the target's name goes.
8. **An on box is ticked for on**, by one maker for every `disabled`
   field of the form: a hidden `disabled=true` then the box's
   `disabled=false`, the fold reading the last sent (§4's quality
   box). A URL then carries both for each row that is on: accepted.

## Find first
- `git grep -n 'picker\|stat-find\|stat-list\|stat-said\|stat-id\|STAT_ROWS' -- crates/ '*.md' ':!roadmap' ':!audits' ':!spikes'`:
  every name of the picker and the typed rows.
- `git grep -ln 'name="stat\.\|name="group\.\|disabled' crates/acquisition-cli/tests/goldens/web/`:
  the goldens the row and the on box change.
- `git grep -n 'disabled' crates/acquisition-cli/src/web/`: Decision
  8's boxes and their readers.
- `grep -n 'template' crates/acquisition-cli/tests/harness/submitted.rs`:
  whether the form's reader skips a template.
- `grep -n 'stat\.\|group\.\|query\.\|view\.' crates/acquisition-cli/src/web/assets/*.js`:
  a field a script names.
- `git grep -n 'target.name\|dispatchEvent' crates/acquisition-cli/src/web/assets/`:
  the announcement Decision 7 replaces.

## Done when
- In `tests/browser.rs`, one test, by keys and pointer: on an empty
  form a stat is picked with arrows and Enter, the page not submitted;
  its min is typed where focus stands; "Add Stat Group" adds a
  weighted sum, whose weight boxes and bound show while the `and`
  group's do not; two stats are picked into it, each given a positive
  weight, and the group a min; the first group's type is changed to a
  weighted sum and back, its weight boxes showing then hiding; Search
  answers the server's total for the same fields, the form refilled
  with both groups, their rows, weights and bound. Then a row and a
  group are removed. The count follows each step with no other input.
- There too: a `stats` ask held in flight (the debugging protocol's
  `Fetch` domain), Escape pressed, the ask released: the list stays
  empty.
- That test's commit carries `retired
  a_pick_fills_the_first_empty_stat_row_and_the_count_follows; held by`
  the new test's name. `the_count_beside_the_button_follows_its_own_form`
  still passes, a keystroke in a finder asking no count.
- With no browser: the row an option of `stats` carries is, byte for
  byte, the row the form renders from an envelope holding that id
  alone, and read in a group as a browser submits it folds to that
  envelope (tests beside the partition pin).
- The partition pin holds over its inputs with the on boxes, a
  template's content sent by no form; the fold accepts exactly the
  names the every-control fill renders.
- Each positional field some group type does not take has its hide
  rule in `page.css`, computed from the two key lists (a test); the
  stylesheet and the scripts name no group type.
- The scan (`c16_no_served_script_…`, `harness::SCRIPTS`) lists the new
  script and refuses a served script naming a field of the form
  (`stat.`, `group.`, `query.`, `view.`), with its breaker.
- `git grep 'picker\.js\|STAT_ROWS\|stat-find\|stat-said\|stat-id' crates/`
  finds nothing.
- `stats` asked for JSON answers as before.
- Goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each diff read:
  an empty form, a filled two-group form and one listing fragment are
  among them. The web module's docs and C187's "Pinned" names state
  what is built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- The switch, the fates' words, the quality box, the site box in the
  bar, Clear: step 4. The value finder: step 6, which reuses Decision
  6's finder.
- An option list on a stat row, a new route, a vendored library, a
  build step, a script that fills, orders or composes (§10, C46).
- The look (#285). A change to the translation, the language, the
  vocabulary document's JSON or the saved row.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
