# 4-full-1, step 6.1 — Values on rows

## Goal
A rows answer carries named values on every row, filled from the
query when none is named, and a query comparing one value is ordered
by it, best first, saying so. An agent gets a table's columns for a
few bytes a row, and #38's reader gets the order the filter meant
unasked. The page inherits the order with no code. Closes #38.

## Inputs
- `4-full-1-step6-design.md` §2 (this step), §5 item 1, §7 (ruled
  2026-11-11, "Approved as designed.", the default order included).
- Issues #38 (the ask, the re-sit's evidence) and #244 (the ruling).
- C92 (one occurrence; a sort takes the largest), C100 (one answer
  shape; a continuation names its order), C98 (a basis names the build),
  C101 (a missing reach is a field to add), C46 (semantics in Rust).
- `answer.rs`: `Rows`, `Shape::of`, `BoundView::Rows`, `RowsOut`,
  `Row`, `Sorted`, the order and the continuation in `answer`.
  `tree.rs`: `Node`, `Member`, `ValueRef`, `nodes`. `bind.rs`:
  `bind_sort`. `group.rs`: `selector` (the favoured selector).
  `eval.rs`: `scalar`, `sorted_by`, `SHOWN`. CLI `search_cmd.rs`
  (`--sort`, the row's sort part, `order_words`), `printed.rs`
  (`shape_flags`); MCP `params.rs` (the `view` text).
- CI on `main` at `cb6c2f26`: Gate (Linux) 43127725200 and Gate
  (Windows) 43127725206 in progress; the step goes on.

## Decisions
1. **Request.** `Rows.values: Vec<String>`, value texts as `sort`
   takes one, each canonical by `print::print_value`; a text named
   twice is kept once. A sort the list lacks joins it at the end. At
   most twelve values, a constant of its own (six, `SHOWN`, is the
   evidence's bound, and a trade search filters on more): more named
   is a `View` error naming the bound; a default list keeps its first
   twelve and counts the rest as `values_left_out`. Where the size
   tests cannot hold twelve, the builder reports what fits.
2. **Row.** `values: Vec<Sorted>`, positional to the answer's list,
   each entry exactly today's `Sorted`. The row's `sort` key retires:
   the sorting value is the entry whose text is `RowsOut.sort`, so a
   number is said once. *Other answer:* keep `sort` beside `values`
   (no reader breaks, the sort's number twice). Pick: retire; the
   goldens are the list of readers.
3. **The answer says the list.** `RowsOut` gains `values` (the list
   used, absent where empty), `values_left_out`, `values_from_query`
   and `sort_from_query` (each true where the query filled it, absent
   otherwise). A default sort fills `sort` and `desc` as if asked.
4. **What the query compares**: one walk over `query.root` in
   `tree::nodes` order:
   - `field op number`, op of `= > >= < <=`, where `bind_sort` binds
     the field (a number field): `Field`;
   - `Compare` (a computed value, a `sum`): its `ValueRef`;
   - `line( … )`: each slot comparison inside it is `line(S).<slot>`,
     S the group's `selector` (its slot tests favoured away), so
     `"T" source=explicit arg1>=30` gives
     `line("T" source=explicit).arg1`, the occurrence C92 binds.
     *Other answer:* the item's `sum(line(S).<slot>)`. Pick: the
     projection; the comparison was on one occurrence, and a sum
     filter already names its sum;
   - `linked( … )` gives none: no value text reaches a link group
     (C101); `undecided( … )` none: it asks readability, compares
     nothing; `has:`, `is:`, text tests and constants none;
   - every branch of an `or` or a `holds` contributes; under a `not`
     (an item's or a group's) a value contributes its column and no
     direction: a flipped direction is a rule to learn, and a click
     sorts it.
5. **Best first.** `>` `>=`: largest first (`desc`); `<` `<=`:
   smallest first; `=`: no direction. A value met with two directions
   (a range), with `=` or under a `not` has none. The default sort is
   taken when the query's deduplicated values number exactly one and
   it has one direction; several values, or one without a direction, give columns
   and the store's order. Named `values` never change the default
   order, only the columns.
6. **Saying it and declining it.** The CLI's order words name the
   value and that it came from the query; the store's order says why
   (none asked, several compared, or declined). `sort: "none"`, any
   case, read before the value parser, asks the store's order;
   `--desc` beside it is refused. Every surface already carries one
   sort text; the builder confirms no field or computed name is
   `none`.
7. **CLI and MCP.** `--value <VALUE>`, repeatable, one value each (a
   template may hold a comma, so no list). A row prints each value as
   its sort part prints today, the sorting one marked as now.
   `shape_flags` prints `--value` for each and `--sort none`. MCP:
   `view.rows.values`, its text in the `view` description; `describe
   answer` names the new keys.
8. **Other views.** `values` beside a count, a crossed table or
   explain is refused as `sort` is (`Shape::of`); a default fills a
   rows view only. `fields: [item]` is orthogonal. A continuation
   carries the resolved list and the resolved sort explicitly (C100),
   so its answer says them as asked, not from the query; accepted
   over re-deriving them. A pre-build token is refused by its basis's
   build (C98).
9. **What changes.** Every rows answer whose query compares a value
   gains `values`; one that compares one value with a direction
   reorders; every row loses `sort` to `values`. The page's answers
   reorder and its order words change through `order_words`, with no
   page code: that is #38's "and the page". Goldens move under
   `ACQ_UPDATE_FIXTURES=1`, each diff read before the commit.

## Find first
- `git grep -n "rows\.sort\|row\.sort\|\.sort\.as_\|Sorted" -- crates`:
  the sort's readers.
- `git grep -n "Shape::Rows(Rows" -- crates`: every rows view's maker.
- `git grep -ln "no sort asked\|sorts by\|\"sort\"" -- crates/*/tests
  README.md`: the tests and goldens holding the order or the key.
- `git grep -n "fn selector\|fn sorted_by\|const SHOWN" -- crates/acquisition-search/src`:
  the makers reused.
- `git grep -n "bytes" -- crates/acquisition-mcp/tests/surface_size.rs`:
  the size bounds values grow toward.
- `git grep -n "Op::Eq\|is_numeric\|fn known_field" -- crates/acquisition-search/src/bind.rs`:
  which fields bind as numbers, and that none is `none`.

## Done when
- One table test in the search crate pins decision 4 and 5 row by row:
  a field, a computed value, a sum, a line's slot under its selector,
  `linked`, `undecided`, `or`, `not`, a range, `=`, a duplicate.
- #38's query, `sum(line("#% increased Rarity of Items found"
  source=explicit).arg1)>=30` with no sort, orders by that sum largest
  first, and the CLI text, its JSON and the MCP answer name it as from
  the query (goldens).
- A query comparing several values gives their columns and the
  store's order, said so (a golden): most page searches are this.
- `--value` twice gives each row both values in order; a sort outside
  them joins at the end; thirteen named is refused.
- `--sort none` gives the store's order; `--desc` beside it, and
  `values` beside a count or explain, are refused (refusal goldens).
- A continuation of a defaulted answer returns the next rows of the
  same order (`next.rs`).
- `CLI-REFERENCE.md`, `MCP-REFERENCE.md` regenerated; the commit
  message names this brief and "Closes #38".
- `tools/gate.sh` exits 0 on the step's tree; `--only doc` too.

## Not in scope
- Page code: columns, sortable heads, a card's numbers as links and
  their value text, the form's sort box (step 4); the count (#245,
  step 2); places (step 5).
- The columns always there (name, base, price, ages): the table's.
- Values on a link group: a field to add (C101), an issue if asked.
- Related terms from a trade group: its own issue.

## Rules
`TEMPLATE.md`. Every `acq` and `acq-mcp` in a mock world (the
mock-session skill).

## Report
`TEMPLATE.md`.
