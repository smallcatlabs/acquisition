# process-347 — the daemon rulings that stay shed their mechanism to the code

Issue #347. The first area of the shedding pass the registry census
proposed (`audits/retire-2026-11/README.md`, "What is the owner's to
decide", 2), at the owner's word of 2026-11-27: one pass, one area
first. He reads this step's commit to `decisions/daemon.md` before the
other areas are ordered, so that commit is written to be read by him.

## Goal
Sixteen rulings of `decisions/daemon.md` say what they promise and no
longer say how the code keeps it. Each sentence that named a knob, a
type, a constant, a count or a procedure of the code stands as a doc
comment under the entry's id at the code that does it. No promise is
narrowed, widened or reworded on the way.

## Inputs
- C192 (a property in the entry, its mechanism the doc comment on the
  code that holds it), C170 (who decides: these entries are mostly in
  the owner's classes, and he has given his word for this pass and for
  no change of meaning), C182 (structure before a rule: nothing is
  added), C154 (the retirement line, for a test or check only: none
  retires here).
- `tools/bothy.py`'s docstring, the entry rule: an entry's shape, its
  limits and `decisions/shape-baseline.tsv`, whose row for an entry
  that now meets both limits is stale and is removed.
- The census's marks: `audits/retire-2026-11/reports/class-a.md` and
  `class-b.md`, one row per entry, column `M` the clauses that reader
  called mechanism and `restated` those that repeat another entry.
  `audits/retire-2026-11/CLASS-BRIEF.md`, section 1, is the rule the
  readers marked by. For C3, C147, C137 and C148 a deep reader also
  marked each clause: `reports/first/daemon-1.md` to `daemon-3.md`.
- The closes of `Step: process-340`, `process-343` and `process-345`
  for what moving a sentence to the code met.
- The last push's CI, read as the template says.

## Decisions
1. **The entries.** C3, C127, C84, C83, C132, C147, C137, C82, C133,
   C134, C123, C31, C50, C135, C144, C148.
2. **A sentence moves only when both class readers marked it M and
   you, reading it against the code, agree.** A clause either reader
   called a promise stays, and so does one you judge a promise: say
   which in the close. The two readers agreed on about two thirds of
   what either marked, so the marks are where to look, not what to do.
3. **The test for a promise:** could a user, GGG, a frontend or a later
   session be let down if the code stopped doing this, with the rest
   of the entry still true? Then it is a promise and it stays. A
   number that is itself promised (a deadline a user waits out, a
   limit GGG is owed) stays; the name of the constant that holds it
   moves.
4. **Nothing changes meaning.** A sentence that stays is not reworded,
   beyond the joining word a cut leaves dangling. Where cutting a
   mechanism sentence would leave the promise it served unsaid, the
   sentence stays whole. An entry's *Why:* and *Pinned:* are not
   touched, except a *Pinned:* a moved test name makes false.
5. **Where a sentence goes.** To a doc comment under the entry's id at
   the code that does it: the function, type or constant, not a test.
   If that comment already says it, the sentence is cut and the commit
   names the place. A sentence placed at the code is no wider than the
   code beside it: read the code path first, and where the entry's
   words and the code disagree, the sentence stays in the entry and
   the close lists the difference for the orderer. Do not correct a
   ruling here.
6. **A clause marked `restated`** is cut where the entry it repeats is
   cited in its place by id and gloss, and stays otherwise.
7. **The commits.** The doc comments first, one commit per crate. Then
   one commit to `decisions/daemon.md` and `decisions/shape-baseline.tsv`
   alone. Its message lists, per entry, each sentence cut, quoted
   whole, and the file and item that now says it, so the owner can
   read what left without a diff. It quotes his word as the census
   records it ("The owner's answers", 2).
8. **The count is what it is.** An entry you leave whole is no
   failure: say why in the close.

## Find first
- `python3 -I audits/retire-2026-11/scripts/registry.py clauses C3 C127
  C84 C83 C132 C147 C137 C82 C133 C134 C123 C31 C50 C135 C144 C148`:
  each entry's clauses, numbered as the readers numbered them. An
  entry edited since the census (`git log -p 50e28e68.. --
  decisions/daemon.md`) may number differently: match by the words.
- `grep -n -E '^\| C(3|127|84|83|132|147|137|82|133|134|123|31|50|135|144|148) '
  audits/retire-2026-11/reports/class-a.md
  audits/retire-2026-11/reports/class-b.md`: the two readers' marks.
- `python3 -I tools/bothy.py show C<n>` for each: where the code cites
  the id today, which is where its mechanism comment belongs.
- `python3 -I tools/bothy.py pack --path <files>` over the files you
  will comment, in one call.
- `wc -c decisions/daemon.md` and `python3 -I tools/bothy.py check`
  before and after.

## Done when
- Every sentence the `decisions/daemon.md` commit removes is quoted in
  its message beside the file and item that says it, and that item's
  doc comment cites the entry's id.
- No sentence of the sixteen entries changed but by a cut:
  `git diff --word-diff` of that commit shows removals, and additions
  only of joining words or of an id and gloss under decision 6.
- `python3 -I tools/bothy.py check` passes, and no row of
  `decisions/shape-baseline.tsv` is stale.
- The close names each marked clause that stayed and why, each entry
  left whole, and each difference between an entry and the code that
  decision 5 found.
- `tools/gate.sh` exits 0 on the step's tree (a comment moves lines
  that check 16 of `tools/arch-check.sh` reads).
- One cold review, saved whole, and its findings answered.

## Not in scope
- Any other area's entries, and any daemon entry not named.
- Retiring an entry, writing a test, or changing code beyond comments.
- Correcting a ruling whose words the code has outgrown: listed, never
  fixed here.
- `tools/bothy.py` and `tools/docs-check.sh`.

## Rules
`TEMPLATE.md`. What differs:
- The cold review is given this brief's name and the commits, by the
  cold-review skill.

## Report
`TEMPLATE.md`, and three things more: per entry, the clauses moved, the
marked clauses kept and why; `decisions/daemon.md`'s size before and
after, by `wc -c`; and what you would change in this brief before the
next area is written from it.
