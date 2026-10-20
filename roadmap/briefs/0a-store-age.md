# 0a — Store-age performance

## Goal
A search over a long-lived store costs what a search over a fresh one
costs. Today the live ask doubles (0.47 s to 0.98 s, release) once a
store carries 90k removed items, and every reader takes the store's
write lock just to open it. Three small changes and one new measurement
copy fix the blind spot the M3 ledger has (its copy holds 6 removed
items).

## Read
- `roadmap/STATE-2026-10-19.md`, report 5 ("Performance"), sections 1
  to 3 and 6: the measurements, the file and line references, the
  rulings each change touches.
- `decisions/store.md` C48 (the schema is internal: an index is
  allowed) and C108; `decisions/search.md` C98 (reload is whole; a
  persisted projection waits for a measured failure).
- `search/M3-LEDGER.md` and `search/item-facts/scripts/m3-ask.py`: the
  method and the rows to add to.
- `crates/acquisition-store/src/annotations.rs` around lines 606 to
  640: the read-only open the facts file should copy.

## Input
The M3 copy as the ledger describes it; a second copy you build with
synthetic removed items (the perf report's method: copy the store, add
tens of thousands of removed rows, keep the live set unchanged).

## Do
1. `crates/acquisition-search/src/corpus.rs` near line 735: the per-gone
   `locations.iter().any(...)` becomes one lookup in a `HashSet` of live
   ids built once. Pin it with a test over many removed ids whose answer
   is unchanged.
2. `crates/acquisition-store`: a partial index on the items table over
   realm, league, location kind and location id `WHERE removed_at IS
   NULL`, a facts schema bump and its migration, a migration test, and
   `EXPLAIN QUERY PLAN` on the corpus and snapshot reads in the commit
   message.
3. `crates/acquisition-store/src/lib.rs` near line 802: a reader's open
   reads `user_version` in a deferred transaction and takes IMMEDIATE
   only when it must migrate. Keep the two-opener race test green.
4. Add the removed-heavy copy to the M3 script and the ledger, and run
   both copies before and after each of 1 to 3 (release build, the
   ledger's method). Optionally, the corpus not dropped at exit after the
   answer is flushed (measured at about 4%).

## Done when
- The three changes are in with their tests, and the ledger has
  before-and-after rows on both copies.
- The gate is green.

## Not in scope
- Holding the corpus in `acq-mcp` (phase 4, the owner has ruled for it).
- Lifting `note` or `inventoryId` into columns (C108 refuses it).
- Reading the price snapshot inside the corpus's transaction (a later
  step, report 5 row 7).
- Any persisted projection (C98's park has its own trigger).

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`, with the M3 rows in section 3.
