# 4-full-1-form-source — The saved search's row keeps its source

## Goal
A saved search keeps what was asked: a text in the language, or a site
search as given (a URL, an id, or the site's query JSON, the form's
fields among them). The text it runs is derived at every read, so an
opened site search carries its translation's report, and a later step
can refill the form from the row (step 2). Every surface saves a
source, and a row written before this step still reads and runs.

## Inputs
- `4-full-1-form-design.md`: §3 is this step; §8 step 1 its "seen
  when"; §9 holds C194 as entered.
- #193 and its comments: the owner's ruling, "Source is the row";
  no released build holds the row, so its shape may change.
- C194 (the row keeps its source, its text derived at every read),
  C167 (user intent in `user.db`, a saved search its first kind),
  C66 (intent typed at the write: a version gate, a strict parse, an
  exact round trip at the current version, `check_write` for what a
  new write may not say), C151 (an unreadable row is named, never
  dropped), C113 (a site search is a query's third spelling), C52 (no
  blind replace by an agent), C143 (a refusal in the request's words),
  C46 (semantics in Rust, every frontend an adapter).
- Code: the store's `user.rs`, the verbs' `saved.rs`, the search's
  `QueryInput` (`read`, `trade_word`), `acq search`'s `--trade`, the
  MCP's `saved_search_set` and `saved_into`, `web/mod.rs`'s saved
  routes and `web/page.rs`'s `save_forms`.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`. Both green at `33f9e9e0`
  when this brief was written (runs 44912135713, 44912135610).

## Decisions
1. **The kind at version 2**: `{version, name, scope, query}`, `query`
   one of `{"text": T}` or `{"trade": S}`, a serde enum whose one key
   is its variant, so two keys or none do not parse. `trade` is a
   non-empty string or an object; the store reads no further (C1: it
   links no search). The kind's `parse` reads a version 1 value,
   `{version: 1, name, text, scope}`, as `query.text`, version kept 1;
   nothing writes it back. `check_write` refuses a version below 2 as
   a new write, so every write lands at 2.
2. **One source on the verbs' `Set`**: `text` becomes `query:
   Option<Source>`, `Source` `Text(String)` or `Trade(Value)`; a
   write naming none keeps the row's. A text is stored canonical, as
   today. A trade string that parses as a JSON object is stored as
   that object; any other string is stored trimmed. Every way that
   spells a `Set` (an undo, a re-aimed write, a recovery offer) spells
   its source on each surface.
3. **The door reads the source** with `QueryInput::read` in the row's
   realm, refused in the language's words (C143), in two moments: what
   the write names (its source, its realm) is read before any file is
   opened; the merged value, the source and the realm a write keeps
   from the prior row among them, is read after that row is read and
   before the write lands. A URL's realm is the row's: a write naming a
   URL that names a realm and naming no realm takes the URL's, never one
   settled from the facts; one naming another realm is the request's
   conflict, as at `acq search`.
4. **The list's row** keeps `text`, now the text the source reads as
   in this build; adds `query`, the source as stored; adds `fidelity`
   for a site search. `request` is the row's scope and source as a
   request (`query.trade` for a site search, never a text beside it).
   A row whose source this build cannot read or translate keeps its
   name, `query` and `request`, has no `text`, and carries `fault`,
   the language's refusal: listed, never dropped (C151's rule), and
   refused when run by `request_of` with the fault and the list.
5. **The surfaces**: `acq saved set --trade URL|ID|JSON` and
   `--trade-file FILE|-`, as `acq search` reads them, each conflicting
   with the positional query; MCP `saved_search_set` takes `trade`, a
   string or an object, beside `text`, never both; `search` with
   `saved` sets the row's request query whole. The page's save form
   carries the source of the request the page answered (the handler's
   `Request`, a continuation's included, passed to `page::answer` and
   `save_forms`; the answer document gains nothing): `text`, the
   answer's canonical text, or `trade`, its `trade_word`, the form's
   envelope as JSON text. `POST saved` takes `trade`; `saved/open`
   redirects to `search?text=` or `search?trade=`, the row's scope
   beside it as the page reads it. The form is not filled (step 2).
6. **The words**: the CLI's list and show say a site search's
   fidelity beside its derived text; the save form's note that a save
   keeps the translation alone goes, since it no longer does.

## Find first
- `git grep -n "SavedSearch\b" crates/`: each maker and reader of the
  kind.
- `git grep -n "SavedSearchSet\|saved::Set\b\|Set {" crates/ | grep -iv "header\|policy\|price"`:
  each spelling of a write, per surface.
- `git grep -n "VERSION: i64\|NewerSaved\|version 2" crates/` and
  `git grep -n '"version": 2' crates/`: the kinds and raw rows that
  stage a saved search newer than this build. Each stays a newer row
  after the bump (version 3, in the version 2 shape), its goldens
  regenerated.
- `git grep -ln "saved" crates/*/tests crates/*/unit`: the tests and
  goldens over the saved documents and pages.
- `git grep -n "asked_by_the_form\|saved_open\|save_forms" crates/acquisition-cli/src/web/`:
  the page's opening and saving.

## Done when
- `SavedSearch` is at version 2 with `query`; a version 1 row staged
  raw lists with its derived text and runs on the CLI, the MCP and a
  page, and its stored value is unchanged after each read (a test).
- A site search saved by `acq saved set --trade`, by MCP
  `saved_search_set` with `trade`, and by the page's save form under a
  site search's answer and under a form search's answer, each lists
  with `query.trade`, `text` and `fidelity`, and opens (`acq search
  --saved`, MCP `search` with `saved`, `saved/open`) to an answer
  carrying its translation's report.
- A JSON string saved as `trade` is stored as its object (a test).
- The source survives: a site search whose JSON holds a field the
  translation does not run, saved on each surface, is stored equal to
  what was sent (after the one normalization of Decision 2), and
  opened and saved again from the page is stored equal again. The
  retention is pinned once in the verbs; each adapter's test pins that
  it hands the source on whole.
- A source that does not read is refused before `user.db` exists (a
  test); a stored one this build cannot translate is listed with its
  `fault` and refused when run (a test).
- A URL naming a realm saved with no realm lands in the URL's realm.
- The undo of a write over a site search puts the site search back.
- The MCP's first turn is measured by `surface_size.rs` and within its
  bound, or the bound is raised to the next 1,000 above the measure
  with the measure in the commit message.
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the goldens regenerated
  under `ACQ_UPDATE_FIXTURES=1`, each diff read.
- C194's *Pinned* names the tests that hold it, and the module docs of
  `user.rs` and `saved.rs` state the row as built.
- `tools/gate.sh` exits 0 on the close's tree.

## Not in scope
- Filling the form from a source, the positional fold, the one page:
  step 2. Rows and groups: step 3.
- A link back to the site: step 9 (#303).
- Rewriting version 1 rows in place, or a migration: refused by C66.
- Any change to the translation or the language.

## Rules
`TEMPLATE.md`. Every `acq` is in a mock world (the mock-session
skill). A choice of shape with two defensible answers stops that
choice and goes to the design session (C172); the rest carries on.

## Report
`TEMPLATE.md`.
