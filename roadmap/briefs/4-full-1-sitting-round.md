# 4-full-1-sitting-round — what the owner's sitting at the pages found, with #317

## Goal

The pages do what the owner's hands expected at his sitting of
2026-11-24 (`audits/web-form/sitting/README.md`, the stops marked
**now**), and the page seat's smaller asks (#317) are gone over with
them, so the pages' words and the form's address are settled once
before the frontends' whole-code review. This round closes the road's
item 1.

## Inputs

- The sitting's record: stops 1, 3, 6, 7, 8 and 10, his words quoted.
- #317, its body and two comments; the column head (fault 9) is #285's
  and out of scope.
- C195 (a site search goes back to the site as it was asked), amended
  at `4cac5642`: a link may carry a league a person names for it alone.
- C46 (shared semantics live in Rust): the page composes no link,
  translates nothing and sorts nothing itself. C138 (the listener) and
  C16 (a renderer, not an app). `decisions/frontends.md` whole, and
  `REVIEW-CHECKLIST.md`, its frontends section.
- The module docs of `crates/acquisition-cli/src/web/mod.rs`,
  `page.rs`, `form.rs`, `fill.rs` and `from.rs`.
- CI at `4cac5642`: `gh run view 45538940745 --json status,conclusion`
  (Gate, Linux) and the same for 45538940726 (Gate, Windows), pending
  when this was written. A red one is this step's first concern.

## Decisions

1. **The form's league is filled at load, from disk** (stop 3). The
   select lists the realm's leagues the store holds, each as
   `Standard — 12 tabs, 3 characters` (the counts the tabs and
   characters pages already read), without a press of "Show this
   realm's leagues". A fresh form, one whose address carries no
   `league` key at all, selects the league holding the most tabs (a tie
   goes to the first in the select's order); a search that named
   "every league" (`league=` empty, which `form::fold` drops) keeps it,
   and so does any league an address names. "every league" stays an
   option, last. With no league on disk the select is as it is now.
   Why: he lost track of the league he searched; the site always shows
   one. A league known to GGG and not on disk keeps reaching the
   select as it does now.
2. **The link has its own league** (stop 10, C195 as amended). A
   select beside the link (there is no copy button; none is added)
   lists the named leagues of decision 1, never "every league", and
   starts at the search's league, or, for a search over every league,
   at the one decision 1 would select. The server writes the link once
   per league through `acquisition_search::trade::link` and the select
   swaps between them; where `trade::link` refuses (a realm the site
   lacks), the answer keeps saying why, as now. Nothing reruns and
   no script composes a link. Changing it changes nothing the search
   asks here.
3. **An answer opens as the table** (stop 6). Cards become the view on
   asking. The query, scope, total and the rest of what stands above
   the rows today go into one `<details>` below the table, closed.
   Columns resize by dragging a head's edge, kept for the page's life
   only, a presentation rule held by the browser-driven check (C187,
   the browser-driven check). The help text that says the default (`acq web --help`) changes
   with it, and the references are regenerated.
4. **Every column sorts by its head** (stop 8): name, base, price,
   place and fetched as the stat columns do, through the language's
   sort. A column whose value the language cannot order yet (price
   across currencies may be one) is not sorted by the page: its head
   says why on hover, and an issue is filed for the value. Never a sort
   in the page's script.
5. **The search survives a visit to another page** (stop 7), carried
   in the address. A page reached from a search carries that search's
   address in its `from` (`web/from.rs`, `onward`), every link onward
   passes it on, and the header's Search link leads to it. Never
   browser storage: C16's page state is the URL, and
   `c16_no_served_script_names_a_storage_cookie_or_history_api` holds
   it.
6. **The words** (stop 1 and #317). "translated here, sent nowhere; a
   URL's realm is the search's" and the count's keys line become hover
   text on what they explain. The rest of #317, each by its own entry:
   - an implicit variant sits beside its explicit line in the finder;
   - an answer under an edited form is marked as the search that ran;
   - the Status control does not show a value the form will not send.
   Two are filed, not built: the saved list's door (#326, an origin
   the saved format does not keep) and the canonical address (#325,
   empty fields that mean something). Another entry whose fix wants
   more than words or one rule is filed with its reason.

## Find first

- `git grep -n -i "league" crates/acquisition-cli/src/web crates/acquisition-cli/tests/web.rs crates/acquisition-verbs/src`:
  the league control, its tests and the verbs' league module.
- `git grep -n "c195_\|link_words\|fn link\|trade::link" crates/acquisition-cli crates/acquisition-search`:
  the link's composer, renderers and tests.
- `git grep -n "view.fields\|RowField::Icon\|cards" crates/acquisition-cli/src/web crates/acquisition-cli/tests`:
  where the default view is chosen (`Folded::request`) and asserted.
- `git ls-files crates/acquisition-cli/tests/goldens | grep -i web`:
  the goldens a changed page moves (C153, one golden per wording).
- `git grep -n "fn .*sort" crates/acquisition-search/src`: which values
  the language orders, for decision 4.

## Done when

- With leagues on disk, the search page's league select lists each
  with its tab and character counts; a fresh form selects the league
  with the most tabs, and an address naming "every league" keeps it;
  tests hold both.
- A site search's answer offers a select of named leagues beside its
  link, starting as decision 2 says; each option's link decodes to the
  same query in that league; a test holds it.
- An answer opens as the table, its details below it; a head's edge
  drags to resize its column, held by the browser-driven check; every column
  either sorts by its head through the language or says on hover why
  not, with that issue filed.
- From a search, a visit to the tabs page and its links onward, the
  header's Search link returns to that search's address; a test holds
  it, and the storage test still passes.
- Each of #317's entries but the column head is built or filed with a
  reason; the stop 1 lines are hover text.
- The references are regenerated, the goldens' diffs read, and the
  gate green on the step's tree.

## Not in scope

- The look (#285): colours, the item drawn as the game draws it,
  #317's column head.
- #323 (a search from an item's stats) and #324 (an item copied as the
  game's text).
- A new value in the language: filed, per decision 4.

## Rules

`TEMPLATE.md`.

## Report

`TEMPLATE.md`.
