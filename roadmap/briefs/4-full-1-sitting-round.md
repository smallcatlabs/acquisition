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
- CI at `4cac5642`: Gate (Linux) 45538940745 and Gate (Windows)
  45538940726, pending when this was written. Read their conclusions
  first; a red one is this step's first concern.

## Decisions

1. **The form's league is filled at load, from disk** (stop 3). The
   select lists the realm's leagues the store holds, each as
   `Standard — 12 tabs, 3 characters` (the counts the tabs and
   characters pages already read), without a press of "Show this
   realm's leagues". With no league named, the league holding the most
   tabs is selected, never "every league", which stays an option, last.
   Why: he lost track of the league he searched; the site always shows
   one. A league known to GGG and not on disk keeps reaching the
   select as it does now.
2. **The link has its own league** (stop 10, C195 as amended). A
   select beside the link's copy button lists the same leagues and
   starts at the search's. The server writes the link once for each
   league listed and the select swaps between them. Nothing reruns and
   no script composes a link. Changing it changes nothing the search
   asks here.
3. **An answer opens as the table** (stop 6). Cards become the view on
   asking. The query, scope, total and the rest of what stands above
   the rows today go into one `<details>` below the table, closed.
   Columns resize by dragging a head's edge, kept for the page's life
   only. The help text that says the default (`acq web --help`) changes
   with it, and the references are regenerated.
4. **Every column sorts by its head** (stop 8): name, base, price,
   place and fetched as the stat columns do, through the language's
   sort. A column whose value the language cannot order yet (price
   across currencies may be one) is not sorted by the page: its head
   says why on hover, and an issue is filed for the value. Never a sort
   in the page's script.
5. **The search survives a visit to another page** (stop 7). The
   Search link of the header leads back to the last search this page
   ran, as its address. This is a per-viewer convenience (the browser's
   session storage, read in a `try`), and the page that renders is the
   server's rendering of that address, nothing restored by script.
6. **The words** (stop 1 and #317). "translated here, sent nowhere; a
   URL's realm is the search's" and the count's keys line become hover
   text on what they explain. The rest of #317, each by its own entry:
   - the saved list says which door made a search;
   - an implicit variant sits beside its explicit line in the finder;
   - an answer under an edited form is marked as the search that ran;
   - the Status control does not show a value the form will not send;
   - a form answer's address drops its empty fields, by a `303` to the
     canonical address (the landing rule's shape).
   An entry whose fix wants more than words or one rule is filed with
   its reason, not built here.

## Find first

- `git grep -n "every league\|league_select\|Show this realm" crates/acquisition-cli`:
  every caller of the league control and every test asserting on it.
- `git grep -n "c195_\|link_words\|copy" crates/acquisition-cli/src/web crates/acquisition-cli/tests`:
  the link's renderers and their tests.
- `git grep -n "cards\|view=" crates/acquisition-cli/src/web/mod.rs crates/acquisition-cli/tests/web.rs`:
  where the default view is chosen and asserted.
- `git ls-files crates/acquisition-cli/tests/goldens | grep -i web`:
  the goldens a changed page moves (C153, one golden per wording).
- `git grep -n "fn .*sort" crates/acquisition-search/src`: which values
  the language orders, for decision 4.

## Done when

- With leagues on disk, the search page's league select lists each
  with its tab and character counts and selects one league, with no
  press of a button; a test holds it.
- A site search's answer offers a league select beside its link; each
  option's link decodes to the same query in that league; a test holds
  it.
- An answer opens as the table, its details below it; every column
  either sorts by its head through the language or says on hover why
  not, with that issue filed.
- The header's Search link returns to the last search run.
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
