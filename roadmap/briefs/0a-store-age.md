# 0a — Store-age performance

## Goal
A store's age stops multiplying the cost of a live ask. Today the ask
doubles (0.47 s to 0.98 s, release) once a store carries 90k removed
items: a loop over every removed id scans every live location, and an
index cannot skip removed rows. Reading and sorting the removed ids
stays (a linear cost the store's `gone` header carries by design); the
multiplicative cost goes. Every reader also takes the store's write
lock just to open it. Three small changes and one reproducible
measurement copy fix the blind spot the M3 ledger has (its copy holds
6 removed items).

## Read
- `roadmap/STATE-2026-10-19.md`, report 5 ("Performance"), sections 1
  to 3 and 6: the measurements, the file and line references, the
  rulings each change touches.
- `CONTEXT.md` C48 (the schema is internal: an index is allowed);
  `decisions/store.md` C108; `decisions/search.md` C98 (reload is
  whole; a persisted projection waits for a measured failure).
- `search/M3-LEDGER.md` and `search/item-facts/scripts/m3-ask.py`: the
  method and the rows to add to.
- `crates/acquisition-store/src/annotations.rs` around lines 606 to
  640: the read-only open the facts file should copy.

## Input
The M3 copy as the ledger describes it; a second copy built by a
deterministic script you add beside the M3 script (the perf report's
method: copy the store, add removed rows, keep the live set unchanged)
with a manifest recording the source copy's checksum, the live and
removed counts, and the construction parameters (how many, spread over
which locations and leagues). The baseline is taken only after the
script and manifest exist, and every measurement runs alone: no other
build, gate or measurement on the machine while it runs (the M3
method's own rule; contention adds 10 to 20 ms against a budget margin
of 1 to 28 ms).

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
  before-and-after rows on both copies, taken alone.
- On the same release build, the removed-heavy copy's empty ask is
  within 1.2 times the base copy's; if it is not, the remainder is
  profiled and named in the report and the bound is renegotiated with
  the owner rather than the step closed.
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
