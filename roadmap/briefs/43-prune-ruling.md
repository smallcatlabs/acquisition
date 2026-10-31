# #43 — `acq store prune`: the drafted ruling

Phase 3's prune verb (`roadmap/README.md`, row 3), drafted ahead of
the 3.30 launch (2026-11-13) at the owner's word of 2026-10-30 ("Yes,
both", the milestone review). A ruling packet, not a build brief: the
owner rules on the questions below, the entry lands in
`decisions/store.md` with his words, and a build brief follows from
`briefs/TEMPLATE.md`. Deleting from his store is his ruling (`AGENTS.md`,
"Rulings first").

## What the store holds and who reads it again

Read at `b85dc2d8`; not run against any store.

- **Removed items** (`items.removed_at` set; a retired location's items
  among them). Read by `acq search --membership all|removed` and by the
  live read's gone list (C108, the search read). They cost every search:
  on the `removed-heavy` copy, 90,890 removed items, every ask is over
  500 ms (`search/M3-LEDGER.md`, its header and rows).
- **Item events** (`item_events`). Read by `acq store events` and the MCP's
  `item_events`, newest first, by hours; `Store::events_after` reads a
  watermark, the highest id, and relies on ids that only grow
  (`acquisition-store/src/lib.rs`, `StoreStatus::last_event`).
- **Responses** (`responses`). Read again only as the latest of
  something: the highest row (the revision, C98 via `corpus.rs`), the
  latest 2xx listing of each realm, league and kind (`stash_basis`,
  `character_basis`, `read_listings`), the latest fetch of each target
  (`read_fetches`, C111, spent by response), the latest league list
  (`league_list_in`), and the v4 migration's re-stamp. Every older row
  is read by nothing but `store status`'s counts.
- **Refused bodies** (`refused`): evidence by C30; four bodies and 135 KB
  at the last measurement.
- **Intent** lives in its own file (C35) and is out of reach of anything
  here. A price on a pruned item becomes an orphan, which the listing
  already shows as a row to look at (`acquisition-plan/src/listing.rs`).

The rules already written for whatever prunes the file
(`acquisition-store/src/corpus.rs`, "Rulings held here"): it never deletes the
highest `responses` row, since an INTEGER PRIMARY KEY would hand that
id out again; a prune that changes what a basis read returns advances
the revision.

Last measured 2026-09-20 over 12.8 days of the owner's copy:
`responses` 1,126 rows and 2.5 MB, `item_events` 22,751 rows and 2.8 MB
(#43). Removed items and events are the only history here GGG cannot
give back; everything else a prune would take is refetchable or
unread.

## What nobody knows yet: the league end

Ground truth says nothing about how a league's end reaches this store.
Two facts decide what prune is for, and the first refresh after
2026-11-13 shows both:

1. **Do item ids survive the migration to Standard?** If they do, an
   ended league's items arrive as `moved`, not `removed`, and the
   league end adds few removed items at all.
2. **What does the ended league's stash listing answer?** If it is
   never fetched again (the policy moves to 3.30), nothing retires its
   tabs, and they stay live in the store under the old league name.
   That is a question about retiring a league, not about pruning
   history, and this draft leaves it out.

## The questions for the owner, each with a recommendation

1. **What prune deletes.** *Recommend:* by age only, the age named on
   the command (no default). Removed items whose `removed_at` is older,
   with the socketed items inside them; item events older; responses
   older, except every row a read above takes as the latest of
   something. Not refused bodies (evidence, tiny), not live items, not
   intent, not tab or character rows (small, and a retired location
   names a removed item's place).
2. **A dry run first.** *Recommend:* the bare verb reports what it
   would delete, by table, with counts and bytes; `--apply` deletes,
   then reclaims the space. A file never shrinks without that last
   step: facts v8 dropped an index and left 1.3 MB of free pages.
3. **Who may run it.** *Recommend:* the CLI only, run by a person. The
   daemon never prunes (C34, no schedules). No MCP tool until an agent
   consumer asks for one, since this verb destroys what nothing can
   refetch.
4. **When it is built.** *Recommend:* in the week after the launch,
   after the first post-launch refresh answers the two facts above and
   gives a fresh measurement. Nothing breaks if it lands late: the
   store only grows and search slows by degrees. If the facts show the
   league end lands as `moved`, the build may wait for the size or
   search-time trigger instead.

## The drafted entry

For `decisions/store.md`, "Store: facts", in the entry rule's form,
should the owner take questions 1 to 3 as recommended; its id is the next free one when it lands:

- **C‹next› — `acq store prune` deletes history by age, never what a read of the current state needs.** It deletes removed items with their socketed items, item events and responses older than the age the user names. It keeps every response a read takes as the latest: the highest row, each listing, each target's fetch and the league list. It never touches live items, locations, refused bodies or intent. Without `--apply` it reports what it would delete. A prune that changes what a read returns advances the revision (C108, a revision never goes back). Only a person runs it; the daemon never prunes (C34, no schedules). *Why:* removed items and events are history GGG cannot give back, and their count slows every search. *Pinned:* nothing yet. Unbuilt (#43).

## Measured at the build, not before

Each from a command the build names, on a `.backup` copy, never the
live file: the table sizes and row counts beside 2026-09-20's; the
removed items' count and age spread; the search read's time on the
copy before and after a prune (`search/M3-LEDGER.md`'s rows).
