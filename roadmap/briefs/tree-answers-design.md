# tree-answers-design — The search's answers shaped for a tree

A Fable design, 2026-11-28: the first step of the desktop GUI's road
(`roadmap/briefs/desktop-design.md`, §4). It builds nothing. The first
section is for the owner; the rest is for the build's brief.

## To the owner, in plain words

**The framing that makes this small.** A tree of tabs is not a new
kind of answer. Your items already lie in the tool's memory tab by
tab, and every search result knows which tab it is in. So the tree
needs two small things of the search:

1. **The count for every tab, made in one pass.** The tool has this
   answer already, for `acq web`'s tab pages, and reads your stash
   three times to make it. It becomes one read.
2. **A search that can be told where to look.** "Only these tabs":
   it then reads those tabs and nothing else, so opening a tab under
   a search costs the size of the tab, not of the stash. The command
   line and an agent gain it too.

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
"The rules the rig wrote in Rust". Three rules exist only
because the search answers one place at a time: **count**, **batch**
and most of **hold**. The others are the window's own or speed,
taken later by evidence. What an evaluation and a places document
cost is that record's "What building showed" and "rig-3".

## 2. Decisions

**D1. No new answer shape, and no new count key.** C100 (one answer
shape) stands. A key for an item's own location was weighed and
dropped: `id:` holds on a page's tab too, so a tab's own bucket has
no term that returns exactly its members, and a route may not narrow
its scope (invariant 4 of the surface). The tree's counts are the
places document's.

**D2. The places document is counted in one evaluation, and a held
one with no read of the store.** Asked a query, its `matched`, `own`
and `listed` come from one pass over the corpus that tallies the
query's matches by the location each lies in, rolled up the tree by
one maker (rule 10 of the search's plan); `elsewhere` and the
query's total come from that pass. Today
`git grep -n 'is_true(&self.query\|unaged(' crates/acquisition-search/src/places.rs`
shows three passes, and `Places::tabs` reads `Store::tabs_document`
on every ask. A caller that holds the document asked no query has it
counted by a function of that document and the corpus that takes no
store, refused where their bases differ. So no frontend sums a tree
(C46, semantics in Rust).

**D3. A scope may name places, and such an answer reads only them.**
`scope.places`, a list of places, each its full coordinate (C54:
realm, kind, id, league); absent, the realm as today. The property,
which is also its test:

> An answer scoped to places holds the rows, in the order, of the
> answer whose query puts those places' terms, joined by `or`,
> before the same query. Its scope block, each term's four counts,
> its total and its undecided are of those places alone. No item
> outside them is evaluated.

A place named covers what its terms select (`counts::place_terms`: a
tab named covers its pages). A place the corpus does not hold is
said in the scope block as a realm not held is, and refuses nothing.
The corpus lies place by place (`corpus.rs`, "The order"), so the
read is a range for each place.

**D4. A window over several places asks every row of them.** An
answer's `after` and `limit` cut its one order, which under a sort
interleaves places. So the caller asks the places in view with a
limit of their matches, known from D2, and parts the rows by the
place each carries: a stable order parted is each place's own order.
This is the rig's batch, kept; a window cut place by place inside
one answer is not built.

**D5. Its spelling.** It rides the request, so a continuation,
`--request` and the MCP's request carry it. The command line spells
it with one repeatable flag, because an answer must not print a
route its own build cannot run (rule 5 of the plan); the spelling is
the build's (C170, agent-facing design). The request for a set of
places is made by the search crate from a places document's places,
never by a frontend. Each place's `request` in the document stays
its terms, so no golden of `acq web` changes.

**D6. Nothing is held between asks.** An expansion, a scroll or a
sort under a query evaluates it over the places in view alone, so
the rig's hold of counts and rows is not rebuilt. A resident
frontend holds the corpus and the places document asked no query,
each while its basis is current.

## 3. What the build's brief must hold

One step, two concerns, each its own commit: D2, then D3 to D5.

Find first: `git grep -n 'Places::tabs\|Places::characters\|Asking {' -- crates`
(the document's callers and tests); `git grep -ln 'scope' -- crates/acquisition-mcp/src crates/acquisition-cli/src`
(where a scope is described or read); `git grep -n 'realm_not_held' -- crates`
(the shape D3's absent place copies).

Done when, drafted:

- Over the seeded world (`Step: next-a1`), for each query of a
  table and both documents: a place's `matched` is its own request's
  `total.matched`; a folder's is its places' sum; `own` is the count
  of matches whose location is the place itself; `listed` is as
  `places.rs` states it today. No golden of `acq web` changes.
- The counting of a held document is a public function whose
  signature takes no store, and a test holds its result equal to the
  document asked the query whole.
- D3's property as a test over sets of places, queries and sorts,
  under one realm and under all, with one fixture's term counts
  worked by hand; a tab id held under two leagues and a league
  spelled two ways are in the fixture.
- A measured line, by a release example the build adds: the tab of
  most items, its rows under `pseudo.total_life>=70` with a limit of
  its items, scoped to the tab and asked by its terms; the median of
  twenty `answer` calls each, alone on the machine (C185). The
  scoped is at most a fifth of the other. The spike's store copy is
  deleted, so the example generates a store of about its size
  through the store's ingest.
- The references regenerated; the diff of
  `acquisition-mcp/tests/goldens/counted_tasks.txt` shown.

A registry line for `decisions/search.md`, entered by the build's
first commit if he rules for D3:

> A request's scope may name places, and an answer over them reads
> and counts those places alone; a route never narrows the scope it
> was counted in. *Why:* a place asked as a term costs a read of the
> whole corpus, and an answer remembered to avoid it can be stale.

## 4. Not in scope

- The window, its crate, and its rules: fold, open, flatten, the
  card, the intent's read-only door in the verbs.
- Speed by evidence. The flat list (By Item) under a costly query
  still evaluates over the league for each window.
- The places document on the command line or as an MCP tool, and
  one document for tabs and characters together.
