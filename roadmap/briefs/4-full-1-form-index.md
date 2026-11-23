# 4-full-1-form-index — The language's index on a page

## Goal
The language's words are read on a page, as they are at a terminal
(`4-full-1-form-design.md` §6's last bullet, §8 step 8, its second
part; #183's rest): `GET describe` renders the search crate's
`describe` document, the index and any entry, block or pattern by
name, linked from the query box and from every way on to the language
a page meets. The seats met the words only at a terminal (#183's
comments). Closes #183.

## Inputs
- #183 and its comments; the design's §5 (the landing rule), §6, §8
  and §9 (the owner approved `describe`, GET, joining the routes; C138,
  the listener, names no route, so its words do not change).
- C16 (a page is its URL), C53 (the index and the whole, two
  densities), C122 (pin once, lowest crate; a page's golden names the
  facts), C143 (shared words, a frontend's flag after them), C153 (one
  golden per wording), C154 (a retirement names its holder).
- Code: the search crate's `describe.rs` (`Describe`, `Named`,
  `describe`, `index`, `INDEX_NEXT`, `nothing_matched`); the CLI's
  `search_cmd.rs` (`describe_list`, `describe_text`,
  `DESCRIBE_SPELLED`, `flagged`); the verbs crate's `refusal.rs`
  (`Way::Describe`, `searched`); `web/mod.rs` (its doc, the routes,
  `stats`, `plan`, `Paged` and its `request`, `wants_json`,
  `Web::failed`), `web/from.rs` whole, `web/page.rs` (`query_box`,
  `refused`, `back`); `tests/web.rs` and `tests/goldens/web/`.
- CI at `3507969f` (`gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, and Windows): Linux 45287594287 and Windows 45287594268
  in progress when this was written; a red one is this step's first
  concern.

## Decisions
The design session's (C172, who designs), settled with the owner:
1. **The route.** `GET describe` renders `describe(names)`: nothing
   named the index, entries and blocks by name, `all`, a pattern.
   Asked for `application/json` it answers the document itself. A name
   the search crate refuses is the page's refusal in its words, as any
   usage refusal is rendered.
2. **One field, `names`**, read by `describe_list`, which both doors
   call; a second `names` adds a list, as a second `--describe` does.
   No second reader of a list of names.
3. **One renderer**, in `page.rs`, from `Describe` destructured whole,
   so a field the search crate adds does not compile until the page
   renders it. An entry shows its name, kind, what it is, its values
   and its examples. In the index each entry's name links to that entry
   and each block's head to that block; an entry's page links back to
   the index. The words are the search crate's; the blocks' heads and
   their order have one home, a table in `search_cmd.rs` that
   `describe_text` and the page both read, its words naming no flag:
   the terminal adds its own after, and its golden does not move. A
   view's entry is named by the request's word; the terminal alone
   spells a flag. What `next` tells a reader to do, the page spells as
   its links and its names box. Examples are text, not links.
4. **A step**, as `plan` is: it takes `from` first, draws on every page
   of it the one way back it was given, and its own links (an entry, a
   block, the index, the names box) hand that `from` on. It joins the
   routes a `from` may not name. One tab; an unsent edit in the query
   box relies on the browser's Back, accepted.
5. **Linked by `from::onward`**: beside the query box's summary, a link
   to the index in words that say what it is; and `Way::Describe {
   names }` in `Paged::request`, a link to those names, spelled in a
   page's JSON answer as its other ways are. Its one producer is
   `describe`'s refusal of a pattern that matches nothing, which offers
   the index; no search refusal gains one.
6. **No script**: links and one GET form, the names box. No finder, no
   third ask of `stats`.
7. **Pins, at the page**: the index, one entry and one block in a
   golden; an unknown name's refusal; the JSON equal to the search
   crate's document; the query box's link in the form's golden; a
   pattern matching nothing refused with its way to the index a link;
   the way back carried from the index to an entry and back. `all` is
   no golden.

## Find first
- `sed -n '/^fn describe_text/,/^}/p' crates/acquisition-cli/src/search_cmd.rs`,
  read whole: every head (a `block` call's, and each one `push_str`
  prints) and their order; then `git grep -l -F -e 'inside line( … )' -e 'categories — the trade' -e 'limits, as the search states them' -- crates`:
  the goldens holding them.
- `git grep -n -e 'describe_list' -e 'acquisition_search::describe(' -e 'describe(&' -- crates/acquisition-cli/src crates/acquisition-mcp/src`:
  every reader of a list of names and every caller of `describe`.
- `git grep -n -e 'Way::Describe' -e 'describing(' -e 'describe: Some' -- crates`:
  every place a `describe` way is made or matched; the report lists
  each producer. `sed -n '/^    fn request(/,/^    }/p' crates/acquisition-cli/src/web/mod.rs`:
  `Paged`'s `_ =>`, where it is left out.
- `git grep -n -e 'const STEPS' -e 'a place or a step' -e '"saved/open"' -- crates/acquisition-cli`:
  every list of the steps a `from` may not name; `git grep -n -e 'never to a step' -e 'saved%2Fopen' -- crates/acquisition-cli`:
  the tests that refuse one.
- `git grep -n -i -e 'no page takes' -e 'offered nowhere' -e 'a job, .describe.' -- crates/acquisition-cli`:
  the docs and tests that hold `describe` as a way no page takes.
- `git grep -l 'query-details' -- crates/acquisition-cli/tests/goldens`:
  the goldens the query box's link moves.

## Done when
- `GET describe` (bare, `names=fields`, `names=computed`, `names=all`,
  `names=~resist`, two `names` fields) answers as `acq search
  --describe` with the same lists; its JSON equals `acq search
  --describe … --json`'s document (`tests/web.rs`).
- `describe_list` is the one reader, called by the CLI and the page;
  a way to `["fields", "~a,b"]` round-trips through the page's link
  (`tests/web.rs`).
- The page's renderer destructures `Describe` with no `..`. The heads
  a renderer prints over a block, and their order, are defined once and
  read by `describe_text` and the page; `git grep -F` of each head of
  more than one word (`inside line( … )`) finds its literal there alone,
  outside goldens.
- A golden `web/describe.txt` holds the index (each entry and head a
  link, the names box), one entry whole, one block, and an unknown
  name's refusal with its readings; its `# holds` lines name those
  facts.
- From the index with `from`, an entry's and a block's links, the
  index link and the names box carry it, and each page draws it as its
  way back (`tests/web.rs`); a `from` naming `describe` is refused as a
  step (`from.rs`'s tests).
- `search_form.txt` holds the query box's link to the index.
- `Paged::request` spells `Way::Describe`: `describe?names=~<a
  pattern matching nothing>` draws its offer of the index as a link
  carrying `from`, and its JSON spells it as a page's other ways
  (`tests/web.rs`). Each test or doc that held it as no page's way is
  changed, or retired by a `retired X; held by Y` line.
- The web module's doc says `describe` is a step and a way a page takes;
  `README.md`'s `acq web` line names the index.
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the goldens regenerated
  under `ACQ_UPDATE_FIXTURES=1`, each diff read.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- Runnable examples; a finder of the language's words; a link from the
  form's marks to the catalogue's rows; the look (#285).
- The MCP's and the CLI's own describe, but for the heads' one home.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers goes to the
design session (C172, who designs); the rest carries on.

## Report
`TEMPLATE.md`.
