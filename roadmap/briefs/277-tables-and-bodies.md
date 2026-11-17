# 277-tables-and-bodies — an answer says when its tables and the store's characters are from different releases (#277)

A design brief, written 2026-11-17 against the tree at `5f02102e`, its
second cut: the first proposed a stored table of version sightings and
a count of places by fetch time, and was withdrawn before a ruling for
the reasons under "Ruled out". This cut needs one ruling, C188 below,
and one Not-doing line, both the owner's before anything is built. The
JSON names and the sentences are the session's agent-facing design. A
release is a game version's first two numbers: `3.30.0b` and
`3.30.0.2.2` are both 3.30.

## Goal

A search answer says, in its document and its text, when the release
its reference tables were made from is not the release the store's
characters were last fetched from, and what cures it: a newer `acq`,
or a fetch. On 3.30's launch day the search read a reworded line as
lacked and said nothing; this is the sentence that day lacked.

## Inputs

- C108 (the store's search read): the store derives nothing a body
  says. C53 (three output levels): text is a function of the JSON.
  C146 (a count says what it counts): the same words on the text, the
  field's doc and the MCP's `describe`. C163 (words beside a time):
  every time carries its age.
- GGG states its game version in one place, a character response's
  `metadata.version`, which the store keeps in `characters.json` and
  reads nowhere: `git grep -n 'metadata' -- crates/acquisition-store/src`.
  The store already reads that column this way for its status:
  `git grep -n "json_extract(c.json" -- crates/acquisition-store/src`.
- The tables' game is the class table's header, read by
  `ClassTable::game` and called by a test alone:
  `git grep -n '\.game()' -- crates`. The gate's drift line holds the
  other export-fed headers to the same pin.
- The measurement that shaped this cut, on a copy of the owner's store
  of 2026-11-17 against the two pinned exports: 2,208 live pc items
  sat in places not fetched since the patch, and 88 of them showed a
  wording 3.30 reworded in place (#277 holds the numbers).
- The last push's CI: `gh run list --branch main --limit 3`; the
  builder names each run and its conclusion in the report.

## Decisions

1. **One fact, read and never stored (C188).** The store's search read
   gives, per realm, the newest release a live character states, as
   the version of the most recently fetched character of that release
   with its fetch time. It is read from `characters.json` when the
   corpus is read. No table, no column, no migration. The drafted
   entry, for `decisions/store.md`:

   > - **C188 — The store reads the game version its characters state and gives the newest per realm, storing nothing for it.** A release is a version's first two numbers. GGG states its game only on a character response. The read returns, per realm, the newest release a live character states, as its most recently fetched character's version and fetch time. A character stating none is left out, and no item body is read (C108, the store's search read). *Why:* a reworded line read as lacked after a patch, and only a character's response says which game the store's bodies came from. *Pinned:* nothing yet; Unbuilt, #277.

2. **Two fields on the answer's scope.** `tables_game`, the class
   table's game as written, and `store_game`, the read's entry for the
   scope's realm (`version`, `fetched_at`, `fetched_at_age`), absent
   where no character of the realm states one or the tables name no
   such realm. One function in the search crate compares the two by
   release; every text surface calls it, and none compares for itself.
3. **Two sentences, printed under the total beside the never-fetched
   sentence, each only when its case holds:**
   - the store is ahead of the tables: `these tables read release {r} and a character here was fetched from {s} ({age}): an item of {s} may read undecided, or lack a line the tables do not know, until a newer acq`
   - the store is behind the tables: `these tables read release {r} and no character here was fetched from it, the newest from {s} ({age}): items may show earlier wording the tables do not read; a fetch cures it`
   The MCP's `describe` entry for the two fields carries the same
   words.
4. **No truth value changes.** A line the tables do not read stays
   lacked and a base they do not know stays undecided.
5. **Not built: the items that show a wording a release retired.** It
   can be counted exactly (the two exports name each rewording), but
   the generator must treat a translation's lookup slots as wildcards
   or it miscounts, and the measurement found 88 items in a store of
   24,000. The drafted Not-doing line, for `decisions/search.md`:

   > - Items that show a wording a release retired, counted or read as undecided: not until `live`, a second league's regeneration that leaves more than a hundred such items in a store; a fetch cures each, and every answer says its places' fetch age.

6. **Ruled out.** A stored table of sightings with a count of places
   fetched before each release (the first cut): it counts a proxy,
   2,208 items flagged for 88 affected; in a store with tabs never
   fetched again its sentence stands on every answer for good; and it
   costs a migration. Old wording read as new: 211 of the 831 entries
   reworded in place changed their number slots, so no text maps to
   text. A league's start date as the release's: the store holds a
   league list only when one was asked for, and an event league would
   read as a release.
7. **The gaps that stay.** A store with no character fetched says
   nothing. A store whose characters are current says nothing of a tab
   not fetched since the patch; the tab's fetch age is on the answer.

## Find first

- `git grep -n 'struct ScopeOut' -A8 -- crates/acquisition-search/src`
  and `git grep -ln 'never_fetched' -- crates`: the scope's document,
  and each surface and golden the sentences join.
- `git grep -n 'pub struct Corpus\b' -A12 -- crates/acquisition-store/src/corpus.rs`:
  the read's header, where the new member sits beside the listings.
- `git grep -n 'struct Shown' -A12 -- crates/acquisition-search/src`:
  whether `acq show` carries a scope. If it does it gains the same; if
  not, it is named in the report and left.

## Done when

- The read returns, per realm, the newest release its live characters
  state, the most recently fetched of that release giving the version
  and the time; a character stating none, a removed one and a version
  whose first two numbers do not parse are each left out and fail
  nothing. Cases in the store's existing corpus tests.
- `scope.tables_game` and `scope.store_game` are as decision 2 says;
  the documents walk (C163) passes; each of the two cases prints its
  sentence, and a store at the tables' release prints neither, held by
  one golden beside the never-fetched one; the MCP's `describe`
  carries the fields.
- C188 and the Not-doing line are in the registry as ruled, and
  `python3 -I tools/bothy.py check` passes.
- No fact-schema version moves: `git grep -n 'FACT_SCHEMA_VERSION: i64'`.
- The gate exits 0 on the step's tree, and the round is reviewed cold.

## Not in scope

A mark on a row or an item; `acq store status`; the plan; any change
to what a term answers.

## Rules

`TEMPLATE.md`.

## Report

`TEMPLATE.md`.
