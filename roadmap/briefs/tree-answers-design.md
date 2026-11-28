# tree-answers-design — The search's answers shaped for a tree

A Fable design, 2026-11-28: the first step of the desktop GUI's road
(`roadmap/briefs/desktop-design.md`, §4). It builds nothing. The first
section is for the owner; the rest is for the build's brief.

## To the owner, in plain words

**The framing that makes this small.** A tree of tabs is not a new
kind of answer. Your items already lie in the tool's memory tab by
tab, and every search result already knows which tab it is in. So the
tree needs two small things of the search, and no third answer shape:

1. **A count by place.** One table: every tab, page of a map or
   unique tab, and character that holds a match, and how many. The
   command line and an agent get it as `--count place`.
2. **A search that can be told where to look.** "Only these tabs":
   it then reads those tabs and nothing else, so opening a tab under
   a search costs the size of the tab, not of the stash.

**What is yours to rule** is the second. Today a search always reads
everything you own, and "in this tab" is one more condition of the
query (C96, place is terms). The options weighed:

| | Buys | Costs |
| --- | --- | --- |
| **A search told where to look** (recommended) | Opening, scrolling and sorting a tab under a search stay fast however large the stash; nothing is remembered between clicks | Two ways to say "in this tab": a condition, and a place to look. The counts of such an answer are of those tabs alone |
| The tool remembers the last search between clicks, as the prototype did | One way to say "in this tab" | What the project ruled out (C98, nothing held between asks): a remembered result is one that can be shown stale |
| A new "tree" answer that carries counts and rows at once | One call for a keystroke | A third answer shape on every door; and the rows wanted depend on which tabs stand open, which is the window's rule, so it would still take two calls |

## 1. What the tree needs, from the spike

`audits/desktop-2026-11/spike/README.md`, "What a design needs" and
"The rules the rig wrote in Rust". Three of its rules exist only
because the search answers one place at a time: **count**, **batch**
and most of **hold**. Fold, open, flatten and the card are the
window's own; an answer that can be stopped, cheaper rows and a
cheaper currency check are speed, taken later by evidence.

Measured there on 44,090 items: one evaluation of a query over the
corpus is about 12 ms for a cheap query and 129 ms for
`pseudo.total_life>=70`; a places document asked a query re-reads
the tab listing, 60 ms.

## 2. Decisions

**D1. No new answer shape.** C100 (one answer shape) stands. The
tree is a counts view and a rows view, each as it is.

**D2. A count key `place`: an item's own location.** A bucket is the
location's coordinate (C54: realm, kind, id, league): a tab, a page
apart from its tab, a character by its id. Its label carries the
kind, the id, the name and, for a page, its tab's id. Its route is
the query with the place's terms, as a `tab` bucket's is
(`counts::place_terms`): a route keeps its denominator (invariant 4
of the surface), so a route never narrows the scope. `tab` and
`character` stay as they count today.

**D3. The places document counts from that one tally.** Asked a
query, its `matched`, `own` and `listed` are the `place` count's
tally rolled up the tree (a tab's with its pages', a folder's with
its tabs'), by one maker (rule 10 of the search's plan), from one
evaluation. `git grep -n 'is_true\|unaged(' crates/acquisition-search/src/places.rs`
shows today's several. A caller that holds the document asked no
query, at a basis still current, has it counted under a query with
no read of the store. So no frontend sums a tree (C46, semantics in
Rust), and the window's count rule is never written.

**D4. A scope may name places, and such an answer reads only them.**
`scope.places`, a list of places; absent, the realm as today. The
property, which is also its test:

> An answer scoped to places holds the rows, in the order, of the
> answer whose query puts those places' terms, joined by `or`,
> before the same query. Its scope block, each term's four counts,
> its total and its undecided are of those places alone. No item
> outside them is evaluated.

So a place named covers what its terms select (a tab named covers
its pages). Rows stand in the answer's one order and each carries
its place, so a caller parts them by place: a stable order parted is
each place's own order. A place the corpus does not hold is said in
the scope block as a realm not held is, and refuses nothing. The
corpus already lies place by place (`corpus.rs`, "The order"), so
the read is a range for each place.

It rides the request, so a continuation, `--print-request`,
`--request` and the MCP's request carry it. The command line spells
it with one repeatable flag, because an answer must not print a
route its own build cannot run (rule 5 of the plan); the spelling is
the build's (C170, agent-facing design). The request for a set of
places is made by the search crate from a places document's places,
never assembled by a frontend. Each place's `request` in the places
document stays its terms, so no page of `acq web` and no golden of
it changes.

**D5. Nothing is held between asks.** With D4, an expansion, a
scroll or a sort under a query evaluates the query over the window's
places alone, so the rig's hold of counts and rows is not rebuilt.
What a resident frontend holds is what it holds today: the corpus,
and beside it the places document asked no query, each while its
basis is current.

**What a keystroke becomes:** one evaluation over the realm (the
counts by place), then rows over the places in view. The rig's was
two evaluations and the rows of every tab of pages.

## 3. What the build's brief must hold

One step, two concerns, each its own commit: the count (D2, D3),
then the scope (D4). Its done-when, drafted:

- Over the seeded world (`Step: next-a1`), for each query of a
  table, every place's `matched`, `own` and `listed` in the places
  document equal its own request's answer; the `place` table's
  buckets sum to the total (C105, a count accounts for every match).
- D4's property as a test over sets of places, queries and sorts,
  with one fixture's term counts worked by hand.
- A measured line: on the spike's store copy, read as the spike read
  it, one tab's rows under `pseudo.total_life>=70` scoped to the tab
  against the same asked by terms, the tolerance named before the
  first run (C185, a measurement runs alone).
- `CLI-REFERENCE.md` and `--describe counts` regenerated; the diff of
  `acquisition-mcp/tests/goldens/counted_tasks.txt` shown.

A registry line, drafted for `decisions/search.md`, entered by the
build's first commit if he rules for D4:

> A request's scope may name places, and an answer over them reads
> and counts those places alone; a route never narrows the scope it
> was counted in. *Why:* a place asked as a term costs a read of the
> whole corpus, and an answer remembered to avoid that read is one
> that can be stale.

## 4. Not in scope

- The window, its crate, and its rules: fold, open, flatten, the
  card, the intent's read-only door in the verbs.
- Speed by evidence: an answer that can be stopped, row building,
  the currency check. The flat list (By Item) under a costly query
  still evaluates over the league for each window; a sitting says
  whether a person feels it.
- One places document for tabs and characters together: the rig
  read two.
- New work on `acq web`'s pages (parked).

## 5. Not checked

- Whether a places document can be counted with no store read
  without changing a byte of it: `acq web`'s goldens say, at the
  build.
- A league's tab id held under two leagues, and a second spelling of
  a league (`counts::place_terms`, its doc): a place in a scope is
  the full coordinate, and the build's fixture must hold both cases.
