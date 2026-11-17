# 277-tables-and-bodies — an answer says when its tables and the store's characters are from different releases (#277)

A design brief, written 2026-11-17 against the tree at `5f02102e`, its
second cut: the first proposed a stored table of version sightings and
a count of places by fetch time, and was withdrawn before a ruling for
the reasons under "Ruled out". Its ruling, C188, and its Not-doing
line are the owner's and are in the registry (`56b37ff1`). Reviewed
cold once (`roadmap/reviews/277-tables-and-bodies-brief-56b37ff1.md`);
its six findings are answered here, listed at the foot. The JSON names
and the sentences are the session's agent-facing design. A release is
a game version's first two numbers: `3.30.0b` and `3.30.0.2.2` are
both 3.30.

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
- The measurement that shaped this cut is #277's comment of
  2026-11-17 (`gh issue view 277 --comments`): the items in places
  not fetched since the patch against the items that show a reworded
  line, on a copy of the owner's store.
- The last push's CI, each gate's own run:
  `gh run list --branch main --workflow "Gate (Linux)" --limit 1` and
  the same for `"Gate (Windows)"`. A red run is this step's first
  concern; the report names each run and its conclusion.

## Decisions

1. **One fact, read and never stored (C188).** The store's search read
   gives, per realm, the newest release a live character states, as
   the version of the most recently fetched character of that release
   with its fetch time. It is read from `characters.json` when the
   corpus is read. No table, no column, no migration. A character with
   no fetch time on record is left out as one stating none is: a
   listing that revives a retired character clears its fetch time and
   keeps its body (`git grep -n 'removed_at IS NOT NULL THEN NULL' -- crates/acquisition-store/src/lib.rs`).
   The entry is C188 in `decisions/store.md`, taken as written.

2. **Two fields on the answer's scope.** `tables_game`, the class
   table's game as written, and `store_games`, a list: the read's
   entry (`realm`, `version`, `fetched_at`, `fetched_at_age`) for each
   realm of the scope that the tables name and that has one. A scope
   of one realm holds at most one entry; the all-realms scope holds
   one per such realm, and a realm the tables do not name (poe2) none.
   One function in the search crate compares each entry with the
   tables by release; every text surface calls it, and none compares
   for itself.
3. **Two sentences, printed under the total beside the never-fetched
   sentence, one for each entry whose release differs, opening `on
   {realm}: ` where the scope holds more than one realm:**
   - the store is ahead of the tables: `these tables read release {r} and a character here was fetched from {s} ({age}): an item of {s} may read undecided, or lack a line the tables do not know, until a newer acq`
   - the store is behind the tables: `these tables read release {r} and no character here was fetched from it, the newest from {s} ({age}): items may show earlier wording the tables do not read; a fetch cures it`
   The MCP's `describe` entry for the two fields carries the same
   words.
4. **No truth value changes.** A line the tables do not read stays
   lacked and a base they do not know stays undecided.
5. **Not built: the items that show a wording a release retired.** It
   can be counted exactly (the two exports name each rewording), but
   the generator must treat a translation's lookup slots as wildcards
   or it miscounts, and the measurement found few such items. The
   Not-doing line is the last in `decisions/search.md`.

6. **Ruled out.** A stored table of sightings with a count of places
   fetched before each release (the first cut): it counts a proxy,
   many times the items affected; in a store with tabs never fetched
   again its sentence stands on every answer for good; and it costs a
   migration. Old wording read as new: a rewording often changes its
   number slots, so no text maps to text. A league's start date as the
   release's: the store holds a
   league list only when one was asked for, and an event league would
   read as a release.
7. **The gaps that stay.** A store with no character fetched says
   nothing. A store whose characters are current says nothing of a tab
   not fetched since the patch; the tab's fetch age is on the answer.

## Find first

- `git grep -n -A8 'struct ScopeOut' -- crates/acquisition-search/src`
  and `git grep -l -i -e 'never_fetched' -e 'never fetched' -- crates`:
  the scope's document, and each surface and golden the sentences
  join, the web's among them.
- `git grep -n -A12 'pub struct CorpusHeader' -- crates/acquisition-store/src/corpus.rs`:
  the read's header, where the new member sits beside the listings.
- `git grep -n -A34 'pub struct Shown' -- crates/acquisition-search/src/show.rs`:
  whether `acq show` carries a scope. If it does it gains the same; if
  not, it is named in the report and left.
- `git grep -n 'fn populated' -- crates/acquisition-verbs/tests/documents.rs`:
  the world the documents walk reads, which states no version today.

## Done when

- The read returns, per realm, the newest release its live characters
  state, the most recently fetched of that release giving the version
  and the time; a character stating none, a removed one, one fetched,
  retired and revived by a listing, and a version whose first two
  numbers do not parse are each left out and fail nothing. Cases in
  the store's existing corpus tests.
- `scope.tables_game` and `scope.store_games` are as decision 2 says,
  with a case of the all-realms scope over two realms at different
  releases. The documents walk's world states a character's version,
  so the walk (C163) meets `fetched_at` and its age.
- Each of the two cases prints its sentence, and a store at the
  tables' release prints neither, held by one golden beside the
  never-fetched one; a web test shows the answer page printing the
  same sentence; the MCP's `describe` carries the fields.
- No fact-schema version moves: `git grep -n 'FACT_SCHEMA_VERSION: i64'`.
- The gate exits 0 on the step's tree, and the round is reviewed cold.

## Not in scope

A mark on a row or an item; `acq store status`; the plan; any change
to what a term answers.

## Rules

`TEMPLATE.md`.

## Report

`TEMPLATE.md`.

## Taken from the review

1, a revived character: left out, with its case. 2, the all-realms
scope: a list, a sentence per realm. 3, the web and the walk: each in
the done-when. 4, the find commands: corrected and run. 5, the
measured figures: left in #277, cited by command. 6, CI: the
template's commands. Not reviewed a second time: the build's review
is the check.
