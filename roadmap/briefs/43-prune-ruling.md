# #43 — `acq store prune`: the drafted ruling

Phase 3's prune verb (`roadmap/README.md`, row 3), drafted ahead of
the 3.30 launch (2026-11-13) at the owner's word of 2026-10-30 ("Yes,
both", the milestone review), and revised by one rule after its review
(`roadmap/reviews/43-prune-ruling-042b3cc1.md`) at his "ok, go ahead". A
ruling packet, not a build brief. It is brought to the owner when the
build's trigger fires (below), not before: he rules on the questions,
the entry lands in `decisions/store.md` with his words, and a build
brief follows from `briefs/TEMPLATE.md`. Deleting from his store is his
ruling (`AGENTS.md`, "Rulings first").

## The rule

**Prune deletes a row only when no kept row and no later ingest
depends on it.** Each table's exceptions below follow from it; the
review's first three findings were each a row the first draft deleted
while something still read it.

## What the store holds and who reads it again

Read at `b85dc2d8` and `20c672b2`; not run against any store. Functions
are in `crates/acquisition-store/src/lib.rs` unless named otherwise.

- **Removed items** (`items.removed_at` set; a retired location's items
  among them). Read by:
  - the search read, `--membership all|removed` and the live read's
    gone list (C108, the search read; `corpus.rs`). They cost every
    search: on the `removed-heavy` copy, 90,890 removed items, every
    ask is over 500 ms (`search/M3-LEDGER.md`, its header and rows);
  - `ingest_item`, which reads a removed row when its id is seen again:
    the item comes back `moved` with its `first_seen`. A pruned item
    seen again is `added`, its history gone;
  - a socketed item's `socketed_in`, which names its parent. A gem can
    stay live at a tab not fetched again while its parent was fetched
    elsewhere without it and later removed; the search names sockets
    from that link (`acquisition-search/src/corpus.rs`);
  - `Store::item` and `Store::holds`, which answer for removed rows.
    After a prune they answer "not found", and `acq price set` notes a
    target the facts do not know rather than refusing it (`Store::holds`'s
    doc comment);
  - `Store::rebuild`, `Store::status` (counts), and the item join in
    `Store::events_since` and `Store::events_after`, which names an
    event's item: an event kept after its item is pruned loses the name.
    With one age for both, only a clock set back keeps such an event.
- **Item events** (`item_events`). Read by `acq store events` and the
  MCP's `item_events`, by hours. `Store::events_after` reads after a
  watermark, the highest id when a refresh began
  (`acquisition-verbs/src/refresh.rs`, `StoreChanges::since`). The id is
  a plain INTEGER PRIMARY KEY, so a prune that deletes the highest row
  hands its id out again and an apply in flight misses the events
  after it.
- **Responses** (`responses`). Read again only as the latest of
  something: the highest row (the revision, C98 (no search service) via
  `corpus.rs`), the latest 2xx listing of each realm, league and kind
  (`stash_basis`, `character_basis`, `read_listings`), the latest fetch
  of each target (`read_fetches`, C111, spent by response), the latest
  league list (`league_list_in`), and the v4 migration's re-stamp. An
  older row is read by `store status`'s counts alone.
- **Locations** (`tabs`, `characters`), retired ones included: a removed
  item's place, and the address reads (`container_rows`). Prune keeps
  them.
- **Refused bodies** (`refused`): evidence by C30; four bodies and 135 KB
  at the last measurement.
- **Intent** lives in its own file (C35) and is out of reach of anything
  here. A price on a removed item is already an orphan
  (`Store::orphaned_item_annotations`), and stays one after a prune.

The rules already written for whatever prunes the file
(`acquisition-store/src/corpus.rs`, "Rulings held here"): it never deletes the
highest `responses` row; a prune that changes what a basis read returns
advances the revision. A prune of removed items is meant to change the
gone list, `--membership removed` and the file's size.

Last measured 2026-09-20 over 12.8 days of the owner's copy:
`responses` 1,126 rows and 2.5 MB, `item_events` 22,751 rows and 2.8 MB
(#43). Removed items and events are the only history here GGG cannot
give back. Removed items slow the search; events and old responses only
grow the file.

## What nobody knows yet: the league end

Ground truth says nothing about how a league's end reaches this store,
and one refresh after 2026-11-13 need not show it. What the store
records depends on what the policy covers and in what order:

1. **Whether item ids survive the migration to Standard.** Even if they
   do, a refresh that fetches an emptied old-league tab before the
   Standard tab holding its items records `removed` first and `moved`
   after; a policy without the Standard tabs records only `removed`.
   A character's items change league through the character's listing,
   with no event at all.
2. **What the ended league's stash listing answers.** If it is never
   fetched again (the policy moves to 3.30), nothing retires its tabs,
   and they stay live under the old league name. That is a question
   about retiring a league, not about pruning history, and this packet
   leaves it out.

The observations that answer them, from the owner's first refreshes
after launch: whether Standard is under his policy; the event counts by
kind for those refreshes (`acq store events`); the removed items' count
before and after; and one migrated item's id looked up in both leagues.

## The questions for the owner, each with a recommendation

1. **What prune deletes.** *Recommend:* by age only, the age named on
   the command (no default), under the rule:
   - a removed item whose `removed_at` is older, judged item by item,
     never by its parent's age; a removed item stays while a kept item
     names it in `socketed_in`;
   - an item event older, except the highest;
   - a response older, except every row a read above takes as the
     latest of something, the highest among them.

   Not refused bodies (evidence, tiny), live items, locations or intent.
2. **A pruned item seen again.** *Recommend:* it is recorded as
   `added`, with a new `first_seen`, and the entry says so. Keeping its
   history would need a record of every pruned id, which is the row
   prune was asked to delete.
3. **A dry run first.** *Recommend:* the bare verb reports what it
   would delete, by table, with counts and bytes; `--apply` deletes,
   then reclaims the space. A file never shrinks without that last
   step: facts v8 dropped an index and left 1.3 MB of free pages.
4. **Who may run it.** *Recommend:* the CLI only, run by a person. The
   daemon never prunes (C34, no schedules). No MCP tool until an agent
   consumer asks for one, since this verb destroys what nothing can
   refetch.
5. **When it is built.** *Recommend:* when its trigger fires, not on a
   date: the post-launch observations above show removed items piling
   up, or the search read's empty-query ask on the owner's copy passes
   500 ms release. Until then the store only grows and search slows by
   degrees.

## The drafted entry

For `decisions/store.md`, "Store: facts", in the entry rule's form,
should the owner take questions 1 to 4 as recommended; its id is the
next free one when it lands:

- **C‹next› — `acq store prune` deletes a row of history only when no kept row and no later ingest depends on it.** It deletes removed items, item events and responses older than the age the user names. It keeps a removed item while a kept item is socketed in it, the highest event, the highest response and every response a read takes as the latest. A pruned item seen again is recorded as added. It never touches live items, locations, refused bodies or intent. Without `--apply` it reports what it would delete. A prune that changes what a read returns advances the revision (C108, a revision never goes back). Only a person runs it (C34, no schedules). *Why:* removed items are history GGG cannot give back, and their count slows every search. *Pinned:* nothing yet. Unbuilt (#43).

## Measured at the build, not before

Each from a command the build names, on a `.backup` copy, never the
live file: the table sizes and row counts beside 2026-09-20's; the
removed items' count and age spread; the search read's time on the
copy before and after a prune (`search/M3-LEDGER.md`'s rows).
