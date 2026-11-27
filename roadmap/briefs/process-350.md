# process-350 — the pricing, plans, store, search and trade rulings that stay shed their mechanism to the code

Issue #350. The last step of the shedding pass (`Step: process-347`
and `Step: process-349` are the first two; read their closes). The
owner read the first step's commit on 2026-11-27 and said to do the
rest.

## Goal
Twenty-one rulings of `decisions/pricing.md`, `plans.md`, `store.md`,
`search.md` and `trade.md` say what they promise and no longer say how
the code keeps it. Each sentence or phrase that named a knob, a type,
a constant, a count or a procedure of the code stands as a doc comment
under the entry's id at the code that does it. No promise is narrowed,
widened or reworded on the way.

## Inputs
- C192 (a property in the entry, its mechanism the doc comment on the
  code that holds it), C170 (who decides: most of these entries are in
  the owner's classes, and his word is for this pass and for no change
  of meaning), C182 (structure before a rule: nothing is added).
- C79 (the trade site is a governed input) and C35 (annotations are
  the only irreplaceable local state): a sentence either rests on is a
  promise.
- `tools/bothy.py`'s docstring, the entry rule: an entry's shape, and
  `decisions/shape-baseline.tsv`, whose row for an entry that now meets
  both limits is stale and is removed.
- The census's marks: `audits/retire-2026-11/reports/class-a.md` and
  `class-b.md`, column `M` the clauses that reader called mechanism and
  `restated` those that repeat another entry;
  `audits/retire-2026-11/CLASS-BRIEF.md`, section 1, the rule they
  marked by. For C74, C75, C98, C99 and C195 a deep reader also
  marked each clause: `reports/first/pricing.md`, `search-3.md` and
  `trade.md`.
- `git show -s f9732068` and `git show -s 56b1af0c`, the first two
  steps' commits to their area files: the form a commit here takes.
- The last push's CI, read as the template says.

## Decisions
1. **The entries.** Pricing: C64, C66, C67, C71, C74, C75, C78. Plans:
   C43, C44, C111. Store: C28, C108, C124, C193, C58, C59. Search:
   C103, C98, C106. Trade: C99, C195.
2. **A sentence moves only when both class readers marked it M and
   you, reading it against the code, agree.** A clause either reader
   called a promise stays, a phrase only one reader marked stays, and
   so does one you judge a promise. The marks are where to look, not
   what to do.
3. **The test for a promise:** could a user, GGG, a frontend or a later
   session be let down if the code stopped doing this, with the rest
   of the entry still true? Then it stays. A clause that names a
   property someone relies on (atomic, synced, bounded, private,
   durable, once, never) stays even when it also names how: the
   daemon step's one review finding was such a cut. A number that is
   itself promised stays; the name of the constant that holds it
   moves. Anything about the owner's stored data, or about what is
   asked of the trade site, is a promise.
4. **A cut may take a phrase out of a clause** where the clause's
   promise reads whole without it, as the daemon step took two
   appositives from C84's headline. Nothing that stays is otherwise
   reworded, beyond a capital or a stop a cut leaves. Before a cut,
   read the next clause: a clause that defines a noun the next one
   uses ("the knob", "that job", "the bound") stays. An entry's *Why:*
   and *Pinned:* are not touched.
5. **Where a sentence goes.** To a doc comment under the entry's id at
   the code that does it: the function, type or constant, not a test.
   If that comment already says it, the sentence is cut and the commit
   names the place. A cut that only repeats another entry or an
   invariant needs no comment: the citation in its place is its home.
   A sentence placed at the code is no wider than the code beside it.
6. **Where an entry's words and the code disagree, the sentence stays**
   and the difference is listed in the close and added as a comment to
   issue #348, which holds the first two steps' seven. Nothing is
   corrected here.
7. **A clause marked `restated`** is cut where the entry it repeats is
   already cited beside it, or can be cited in its place by id and
   gloss. One that carries its citation and has no neighbour the
   citation fits beside stays.
8. **The commits.** The doc comments first, one commit per crate. Then
   one commit per area file, with `decisions/shape-baseline.tsv` in the
   one that stales a row. Each message lists, per entry, each sentence
   cut, quoted whole, and the file and item that now says it, and
   quotes the owner's word as `f9732068` does.
9. **The count is what it is.** An entry left whole is no failure.

## Find first
- `git grep -n -w -E 'C(64|66|67|71|74|75|78|43|44|111|28|108|124|193|58|59|103|98|106|99|195)'
  -- crates tools`: the doc comments that cite each id today. Read
  these before writing any comment: most cuts of the first two steps
  were already said at the code.
- `python3 -I audits/retire-2026-11/scripts/registry.py clauses C64 C66
  C67 C71 C74 C75 C78 C43 C44 C111 C28 C108 C124 C193 C58 C59 C103 C98
  C106 C99 C195`:
  each entry's clauses, numbered as the readers numbered them. An
  entry edited since `50e28e68` may number differently: match by the
  words.
- `grep -n -E '^\| C(64|66|67|71|74|75|78|43|44|111|28|108|124|193|58|59|103|98|106|99|195) '
  audits/retire-2026-11/reports/class-a.md
  audits/retire-2026-11/reports/class-b.md`: the two readers' marks.
- `grep -n -w -E 'C(64|66|67|71|74|75|78|43|44|111|28|108|124|193|58|59|103|98|106|99|195)'
  decisions/shape-baseline.tsv`: the baseline rows of every entry
  read, cut or not.
- `wc -c decisions/pricing.md decisions/plans.md decisions/store.md
  decisions/search.md decisions/trade.md` and `python3 -I
  tools/bothy.py check`, before and after.

## Done when
What the reviewer judges, from the commits before the close:
- Every sentence an area-file commit removes is quoted in its message
  beside the file and item that says it, and that item's doc comment
  cites the entry's id; a cut under decision 7 names the citation.
- `git diff --word-diff` of each area-file commit shows removals, and
  additions only of a capital, a stop, or an id and gloss under
  decision 7.
- No removed sentence was a promise under decision 3.

What the close shows:
- `python3 -I tools/bothy.py check` passes, and no row of
  `decisions/shape-baseline.tsv` is stale.
- The close names each marked clause that stayed and why, each entry
  left whole, and each difference under decision 6.
- `tools/gate.sh` exits 0 on the step's tree.
- One cold review, saved whole, and its findings answered.

## Not in scope
- Any other area's entries, and `AGENTS.md`'s.
- Retiring an entry, writing a test, or changing code beyond comments.
- Correcting a ruling: listed under decision 6, never fixed here.
- `tools/bothy.py` and `tools/docs-check.sh`.

## Rules
`TEMPLATE.md`. What differs:
- The cold review is given this brief's name and the commits, by the
  cold-review skill.

## Report
`TEMPLATE.md`, and three things more: per entry, the clauses moved and
the marked clauses kept, with why; each area file's size before and
after, by `wc -c`; and, for the pass as a whole, what you would tell
the next session that thinks of running one.
