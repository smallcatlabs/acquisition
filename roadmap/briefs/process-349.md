# process-349 — the network, ci and frontends rulings that stay shed their mechanism to the code

Issue #349. The second step of the shedding pass (the first is
`Step: process-347`, the daemon's; read its close). The owner read
that step's commit on 2026-11-27 and said to do the rest.

## Goal
Sixteen rulings of `decisions/network.md`, `decisions/ci.md` and
`decisions/frontends.md` say what they promise and no longer say how
the code keeps it. Each sentence or phrase that named a knob, a type,
a constant, a count or a procedure of the code stands as a doc comment
under the entry's id at the code that does it. No promise is narrowed,
widened or reworded on the way.

## Inputs
- C192 (a property in the entry, its mechanism the doc comment on the
  code that holds it), C170 (who decides: most of these entries are in
  the owner's classes, and his word is for this pass and for no change
  of meaning), C182 (structure before a rule: nothing is added).
- The five invariants of `AGENTS.md`. The network entries are how four
  of them are kept; a sentence an invariant rests on is a promise.
- `tools/bothy.py`'s docstring, the entry rule: an entry's shape, and
  `decisions/shape-baseline.tsv`, whose row for an entry that now meets
  both limits is stale and is removed.
- The census's marks: `audits/retire-2026-11/reports/class-a.md` and
  `class-b.md`, column `M` the clauses that reader called mechanism and
  `restated` those that repeat another entry;
  `audits/retire-2026-11/CLASS-BRIEF.md`, section 1, the rule they
  marked by. For C21, C150, C149, C156 and C187 a deep reader also
  marked each clause: `reports/first/network.md`, `frontends-1.md`
  and `frontends-2.md`.
- `git show -s f9732068`, the daemon's commit to its area file: the
  form a commit here takes.
- The last push's CI, read as the template says.

## Decisions
1. **The entries.** Network: C18, C21, C22, C87, C26, C32, C88, C128,
   C130, C150. CI: C155. Frontends: C16, C149, C139, C156, C187.
2. **A sentence moves only when both class readers marked it M and
   you, reading it against the code, agree.** A clause either reader
   called a promise stays, and so does one you judge a promise. The
   marks are where to look, not what to do.
3. **The test for a promise:** could a user, GGG, a frontend or a later
   session be let down if the code stopped doing this, with the rest
   of the entry still true? Then it stays. A clause that names a
   property someone relies on (atomic, synced, bounded, private,
   durable, once, never) stays even when it also names how: the
   daemon step's one review finding was such a cut. A number that is
   itself promised stays; the name of the constant that holds it
   moves. Anything about what is sent to GGG, when, or how often is a
   promise.
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
   names the place. A sentence placed at the code is no wider than the
   code beside it.
6. **Where an entry's words and the code disagree, the sentence stays**
   and the difference is listed in the close and added as a comment to
   issue #348, which holds the daemon step's four. Nothing is
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
- `git grep -n -w -E 'C(18|21|22|87|26|32|88|128|130|150|155|16|149|139|156|187)'
  -- crates tools .github`: the doc comments that cite each id today.
  Read these before writing any comment: seven of the daemon step's
  eleven cuts were already said at the code.
- `python3 -I audits/retire-2026-11/scripts/registry.py clauses C18 C21
  C22 C87 C26 C32 C88 C128 C130 C150 C155 C16 C149 C139 C156 C187`:
  each entry's clauses, numbered as the readers numbered them. An
  entry edited since `50e28e68` may number differently: match by the
  words.
- `grep -n -E '^\| C(18|21|22|87|26|32|88|128|130|150|155|16|149|139|156|187) '
  audits/retire-2026-11/reports/class-a.md
  audits/retire-2026-11/reports/class-b.md`: the two readers' marks.
- `wc -c decisions/network.md decisions/ci.md decisions/frontends.md`
  and `python3 -I tools/bothy.py check`, before and after.

## Done when
What the reviewer judges, from the commits:
- Every sentence an area-file commit removes is quoted in its message
  beside the file and item that says it, and that item's doc comment
  cites the entry's id.
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
- Any other area's entries, and `AGENTS.md`'s: its six marked entries
  were read and are left.
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
after, by `wc -c`; and what you would change in this brief before the
last step is written from it.
