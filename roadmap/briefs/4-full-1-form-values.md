# 4-full-1-form-values — The value finder

## Goal
A tab opened with no query is ordered by a value picked by name
(`4-full-1-form-design.md` §8 step 6): "Add a column" finds a stat or
a number field by the site's words, and a pick lands on the page
ordered by its value, its column added. The owner could not fill the
sort box and wanted a stat selector (`audits/web-2026-11/sitting/`,
stops 3, 4, 9); the page seat's fault was a sort by values a player
names.

## Inputs
- `4-full-1-form-design.md` §6's first bullet, §8 step 6, §9 (the
  finder's two asks are one route's field), §10. Step 5's close,
  `6c515b9d`; step 3's brief, Decision 6 (the finder reused).
- C16 (a page is its URL), C46 (semantics in Rust), C138 (no new
  route), C143 (a refusal speaks the shared words), C187 (a script's
  rule has a browser-driven check), C154 (a retirement names its
  holder).
- Code: `web/mod.rs` (`search`, `places`, `view_link`, `view_change`,
  `switched`, `stats`); `web/page.rs` (`ViewLink`, `stat_options`,
  `view_hidden`, `item_heads`); `assets/lists.js`, `assets/count.js`;
  the search crate's `show.rs` (`trade_value`), `values.rs`,
  `trade/vocabulary.rs`, `trade/catalogue.rs`, `answer.rs` (`VALUES`).
- CI (`gh run list --branch main --workflow "Gate (Linux)" --limit 1`,
  and Windows) at `6c515b9d`: Linux 45223710065 success, Windows
  45223710118 in progress.

## Decisions
The design session's (C172, who designs), each settled:
1. **An option is its page's own submit button.** The finder stands
   inside each form that carries the page's view hidden: the search
   form and the query box (`view_hidden`'s two callers, so a page
   whose source is a text picks from the editor holding it) and a
   place page's list form, beside its item heads. A pick submits that
   form as it stands with one field more; the listing knows nothing
   of the page. *Declined:* an option as a link made for the page;
   a typed value box, which the sitting failed at; a script that
   composes a URL (C46).
2. **The field carries the value's text**: `view.by`, its value what
   `view.sort` and `view.value` take. A pick needs no second lookup
   and meets no refusal a head's click does not.
3. **`view.by` is the page's.** `search` and `places` take it out of
   their fields first, as `switched` takes the switch; make the change
   by `view_change` over the rows the search as sent answers with (on
   a place page, the rows its heads are drawn from); and answer `303`
   to the URL a head's link writes (`view_link`; a place page's own
   maker), which holds no `view.by`. A reload changes nothing. Where
   the answer is a count, `view.by` is let go as the carried order is
   (#304) and the `303` is the page less it. Where the list is full
   (`VALUES`), the page lands where the search's own refusal of a
   longer list speaks (C143), never silently unchanged. A place page
   sent `view.by` with no place open refuses it in words: no rows say
   an order there. *Declined:* a view field the fold reads.
4. **One function lists, in the search crate** (C46): the value the
   translation compares for a bounded filter on one entry, a stat's
   or a number field's: what `trade_value` makes from a line's stat
   id, made a public function of an entry, `trade_value` its caller. An entry is listed exactly where it answers a value.
   Names are the site's: a stat's texts, a field's catalogue label.
   The form's number fields first, then the stats in the vocabulary's
   order; the first `PICKED` with a value listed, the rest counted.
5. **One route, one field**: `stats` asked for values (`values=true`)
   answers JSON where asked, else the options, each a `<button
   type="submit" role="option" name="view.by" value="…">` in its `<li>`.
6. **The finder script learns nothing of values**, two rules more: a
   finder asks the route its element names, written by the server
   (`data-finder="stats?q="`; the script appends the words), and a
   finder is heard wherever it stands, its listeners on the document
   as `count.js`'s are.
7. **The control reads "Add a column"**, on the search page and on
   each place page.

## Find first
- `git grep -n 'trade_value\|fn compared\|pub fn vocabulary' crates/acquisition-search/src/`:
  the function made public, its callers, the listing's source.
- `git grep -n 'view_change(\|view_link(\|switched(\|from::taken(\|= opening(' crates/acquisition-cli/src/web/mod.rs`:
  where `search` and `places` take page fields out and make a change.
- `git grep -n 'view_hidden(\|order_way(\|item_heads(\|hidden_fields(view.view_fields' crates/acquisition-cli/src/web/page.rs`:
  each form that carries the view, and where a page says its order.
- `git grep -n 'data-finder\|stats?q=\|addEventListener' -- crates/acquisition-cli/src/web/ crates/acquisition-cli/tests/ ':!*/goldens/*'`:
  the finder's markup, its fixed route, its listeners, the tests that
  drive it.
- `git grep -ln 'data-finder\|view\.sort' crates/acquisition-cli/tests/goldens/web/`:
  the goldens the finder changes.
- `grep -c 'shape = "minmax' crates/acquisition-search/reference/trade-catalogue-v1.toml`:
  the number fields the listing may hold.
- `git grep -n 'c16_no_served_script\|SCRIPTS' crates/acquisition-cli/tests/ ':!*/goldens/*'`:
  the scan the scripts meet.

## Done when
- In `tests/browser.rs`, by keys: a place page with a tab opened and
  no query; focus "Add a column", type a value's name, move with the
  arrows, Enter; the page lands at the URL its head's link would
  write, ordered by that value, its column added and its head marked.
- There too: the same pick from the search form after an unsent edit
  lands with the form filled as it stood, the edit among it; the
  ordered head's own link flips the order.
- With no browser, in the search crate: every value the listing can
  offer, over every stat of the vocabulary and every number field of
  the catalogue, is taken by the search as an order (read as a
  request's `view.sort`, refused by none); a stat with no value is
  not listed (a named case).
- `tests/web.rs`: `view.by` on the search page and a place page
  answers `303` to the head link's URL; beside a count, to the page
  less it; past a full list, the search's refusal; on a place page
  with none open, a usage refusal; `stats?values=true` asked for JSON
  answers the listing. The fold never sees `view.by` (a test).
- `git grep -n 'stats' crates/acquisition-cli/src/web/assets/` finds
  nothing; the scan's breakers still refuse a script naming a field.
- The step 3 browser tests pass unchanged, or each change names why.
- Goldens regenerated under `ACQ_UPDATE_FIXTURES=1`, each diff read:
  a listing fragment and a form and a place page holding the finder
  among them. The web module's doc says the finder as built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- A place's line, its default order and the Stack column (#296, step
  7); the look (#285); a remembered view.
- A widened translation or a value no entry reaches today (§6); a new
  route; a script that composes a value or a URL.
- A change to the language, the vocabulary's existing JSON or what a
  head's link writes.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
