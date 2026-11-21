# 4-full-1-form-editor — The editor, served

## Goal
A search's page is its source's editor, filled, above its answer
(`4-full-1-form-design.md` §1). The form renders from the site's query
JSON it folds into, so a form search, a site search, a saved search and
a refused one each stand under the form filled with what was asked, and
a bound can be tweaked and run again as on the site. The stat fields
become positional, so a later step adds rows by script with no index.

## Inputs
- `4-full-1-form-design.md`: §2 is this step less its script; §8 step 2
  its "seen when"; the foot, what its review changed.
- Step 1's close, `49adf439`: the row keeps its source (C194, the row
  keeps its source); it left `asked_by_the_form` to this step.
- #184, #188, #190, #193 and their comments.
- C139 (the web UI a verb of the CLI crate, the form generated from the
  dump), C46 (semantics in Rust), C16 (a page is its URL), C113 (a site
  search a query's third spelling), C141 and C142 (league and defences
  beside the JSON), C143 (a refusal in the request's words), C47
  (malformed input a structured error), C153 (a finding joins the
  surface that holds it).
- Code: `web/form.rs` (the fold), `web/mod.rs` (`home`, `search`,
  `asked_by_the_form`, `opened_from`, `count`), `web/page.rs`
  (`search_form`, `control`, `query_box`, `trade_box`, `answer`), the
  search crate's `trade.rs` (`decode`, `decode_json`),
  `trade/translate.rs` (`stat_value_keys`), `trade/vocabulary.rs`.
- The captured site searches: `crates/acquisition-search/tests/trade-decode.toml`,
  the `query` of `trade-translate.toml`'s cases, and
  `search/pseudo-stats/data/captures.json`'s `searches.*.returned_query`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`; both green at `49adf439`
  (runs 44916726128, 44916726220).

## Decisions
1. **Positional stat fields.** The names are `group.type`, `group.min`,
   `group.max`, `group.disabled`, `stat.id`, `stat.min`, `stat.max`,
   `stat.weight`, `stat.option`, `stat.disabled`, a fixed set in the
   fold's names. A field's presence, not its value, begins: `group.type`
   a group, `stat.id` a row in the group last begun; a `group.*` field
   belongs to the group last begun, a `stat.*` field to the row last
   begun in it. A `stat.id` before any group, or a row's field before
   any row of its group, is a usage error. A row's on/off box stands
   after its `stat.id` in the page's order, a group's after its type.
2. **The fold drops a value key its group's type does not take**,
   asked of `stat_value_keys`, made public in the search crate.
3. **One normal form**, a function in `form.rs`: nulls, empty strings
   and `disabled: false` dropped, then empty objects and arrays, then a
   stat group with no filter. The fold's envelope is in it.
4. **The fill holds by rendering.** Each control takes its value at its
   path from the normal envelope only where it folds back to it exactly
   (a select's own option, a JSON number written as serde writes it in
   a number box, a string equal to its trim in a text box, `true` in a
   box), and the taking marks the path held. A listing control shows
   its value and holds nothing. Every leaf left unheld, `sort` and any
   other envelope key among them, is listed above the form by its path
   and value, under one sentence saying a search from the form leaves
   it out. A panel holding a value renders open.
5. **Rows.** Each group of `query.stats` renders as a group, each filter
   as a row: its id in the typed box, its texts and fate beside it from
   `vocabulary::entry(id)`, a new lookup beside `vocabulary` with the
   same statuses (an id no table holds says so). A weight box where the
   group's type takes one, an option box where the row holds one. Until
   step 3 the last group is topped up with empty typed rows to
   `STAT_ROWS`, at least one; with no group, one `and` group.
6. **The source fills one editor.** From the request's `QueryInput`: a
   text fills the query box, open (a `details`), over an empty form; a
   site search is decoded and fills the form (`league`, `defences`,
   `query`), the query box holding the canonical text, closed. A
   continuation is read by its token's request. The big form carries
   the view as the query box does: limit, the table, a count's keys,
   and hidden the URL's `view.sort`, `view.desc`, `view.value`, and
   `saved` with `saved_revision`.
7. **One page.** `home` and `search` render through one function: the
   bar and form, the query box, the site box, then the answer or the
   refusal under `id="answer"`; every form that submits to `search`
   names `search#answer`. The save form stays with the answer.
8. **A refused search** keeps its status and JSON: the form filled from
   the envelope the fold or the decode gave; the site box holding its
   text where the decode refused; the query box holding its text and
   view as sent (today's); an empty form where the fold refused.
9. **`asked_by_the_form` asks a site search** as `trade`, its word, with
   its view (the one-page rule: the page fills from it), a JSON string
   compared as its object. Place links then stand under a form search's
   and a site search's answer, carrying the canonical text.

## Find first
- `git grep -n 'query\.stats\[' crates/ tools/`: every name to move.
- `git grep -n 'type=\\"hidden\\"\|\.link(\|see_other(\|hidden_fields' crates/acquisition-cli/src/web/`:
  every maker of a link or a hidden field; each keeps its fields' order.
- `git grep -n 'search_form\|query_box(\|trade_box(\|page::answer(\|asked_by_the_form' crates/acquisition-cli/src/`:
  the callers that change.
- `git grep -ln 'stat-find\|query-box\|trade-text\|search_form\|Run this query' crates/acquisition-cli/tests/`:
  the tests and goldens over the form and the answer page.
- `git grep -c '^query = ' crates/acquisition-search/tests/trade-*.toml`:
  the captured searches the pin reads.

## Done when
- A form search's answer, a refused one, a site search's and a saved
  form search's each stand under the form filled with it, the saved
  one opened through `saved/open` (tests in `tests/web.rs`).
- `git grep 'query\.stats\['` finds no field name in `crates/`; the
  fold's positional reading, its usage errors and its drop by type are
  tests in `form.rs`.
- The partition: for every captured site search above and one envelope
  that fills every control, the form read from the rendered HTML as a
  browser submits it (disabled controls and unticked boxes sending
  nothing, a select its selected option) folds to the normal envelope
  less the paths the page lists, and those paths are exactly the rest.
- The fold accepts exactly the names the every-control fill renders,
  with the untyped view names.
- A two-group search keeps both groups, in order, through every link
  and hidden form of its answer page (a test).
- A refused search: each case of Decision 8 renders as it says, its
  status and JSON unchanged (tests).
- An empty form renders one `and` group with typed rows under the
  positional names; `picker.js` and `count.js` are unchanged but for a
  name, and `tests/browser.rs` passes.
- The goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each diff read;
  the module docs of `form.rs`, `mod.rs` and `page.rs` state the page as
  built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- Templates, adding and removing rows and groups, weights shown by
  type, the keyboard: step 3. The quality box, the switch, the fates,
  the site box in the bar, Clear: step 4. The landing rule: step 5.
- Any change to the translation, the language or the saved row.
- A script that fills, orders or composes anything (C46).

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers stops that
choice and goes to the design session (C172); the rest carries on.

## Report
`TEMPLATE.md`.
