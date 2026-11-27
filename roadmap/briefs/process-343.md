# process-343 — the search and trade rulings their tests already hold retire

Issue #343, with #341. The second of four batches the registry census
proposed (`audits/retire-2026-11/README.md`, "A proposed order"). The
first is `Step: process-340`; read its close for what a batch meets.

## Goal
Up to fifteen rulings of `decisions/search.md` and `decisions/trade.md`
stand as one line each under "Retired", and what each said is a comment
under its id at the test or type that holds it. The two files' opening
paragraphs say who may change a line as C170 says it, and C101 no
longer says a query word is ruled first.

## Inputs
- The retired rule: `tools/bothy.py`'s docstring. `decisions/daemon.md`'s
  "Retired" section is the form to copy.
- C154 (the retirement line, `retired X; held by Y`), C182 (structure
  before a rule: no new entry, row or rule), C192 (a property in the
  entry, its mechanism at the code), C170 (who decides), C153 (a
  finding joins a surface that holds it: a new test is a row or a case,
  never a file), C122 (pinned once, at the lowest crate; a golden is
  regenerated under `ACQ_UPDATE_FIXTURES=1` and its diff read), C79
  (the trade site is a governed input: nothing here fetches it).
- `search/README.md`, the search crate's working directory, for where
  its corpus, tables and goldens are.
- The census's reads, one block per entry, holders by file and line as
  of `50e28e68`: `audits/retire-2026-11/reports/first/search-1.md` to
  `search-4.md` and `trade.md`; a second, blind read of C91, C95, C96,
  C125 and C142 under `reports/second/`. A reader's claims, not proof.
- The owner's answers of 2026-11-27: the census's "The owner's answers"
  (the small tests are written now; C170 stands over C101's sentence)
  and issue #344 (the Standing paragraphs, taken under delegation).
- The last push's CI, read as the template says.

## Decisions
1. **First, who may change a line.** One commit, before any retirement:
   the "Standing" paragraph of each file says an entry in the owner's
   classes changes by his ruling and a session writes and amends the
   rest, telling him what changed, pointing at C170 and restating no
   more of it; C101 loses its last sentence. The message quotes his
   answer as the census records it and names #344. This closes #341.
2. **The entries.** As they stand: C91, C104, C93, C94, C95, C105, C96,
   C119, C141, and C118. A clause a reader marks `cut` or unholdable
   (C118 and C94 have one each) leaves, its reason said in the commit,
   or stays in the comment if you judge it a promise. After one test
   each: C92 (two occurrences each meeting one condition of a group;
   the worked corpus), C142 (three of the four defence filters, and the
   label naming its field; the translate table), C90, C109, C125 (each
   as its reader's `unheld` line names it).
3. **C113 is left.** Its reader found a promise about which game's
   searches the product reads, which is scope and the owner's.
4. **You read every clause's test yourself before retiring its entry,
   and you split a clause by each code path that can break it.** The
   first batch's review found a second path its readers had not. A
   loose clause gets its assertion if that fits a table or harness the
   crate has, with no new fixture and no product change. Otherwise the
   entry stays, and the close says which and why.
5. **A trade test asserts only what a capture or a numbered claim of
   `docs/design/trade-ground-truth.md` already says.** Where the
   expected value would be a guess about the site, the entry stays.
6. **A promise made for every platform and held on one is not held.**
7. **Where the words go.** `*Held by:*` names the fewest files, three
   at most, whose tests or types hold every promise as the entry states
   it; a file holding only mechanism-level cases keeps its citation and
   is not named. The comment under the id at each says what that file
   holds, and the reason is whole at the first. A mechanism sentence
   goes to the doc comment on the code that does it.
8. **Words the code has outgrown are corrected as they move, and the
   commit says each difference**, a stale clause in a headline among
   them. Known: C95 says there is no grand total and the counts view
   answers a sum; C118 is marked unbuilt and is built; C96's second
   reader found no test of a league-less query over two live leagues
   (write it or leave C96); C94 has one clause open at low confidence
   (a new derived value that sums lines in code), read first. If a
   difference is a promise the code breaks, stop that entry and file
   an issue. A "Not doing" line whose trigger a retirement shows met is
   deleted, said in the commit.
9. **The commits.** The Standing commit; each new test its own commit;
   then the retirements as one commit per file, each carrying one
   `retired C<n>; held by <Y>` line per entry.

## Find first
- `for n in 91 104 93 94 95 105 96 119 141 118 92 142 90 109 125 101;
  do python3 -I tools/bothy.py show C$n; done`: each entry whole and
  every place that cites it.
- `grep -n "owner's ruling" decisions/search.md decisions/trade.md`:
  the sentences decision 1 rewrites.
- `grep -n -A16 '^### C<n> ' audits/retire-2026-11/reports/first/*.md
  audits/retire-2026-11/reports/second/*.md`: the readers' claims.
- `python3 -I tools/bothy.py pack --path <files>` over a retirement's
  holder files, in one call.
- `python3 -I tools/bothy.py check` before and after.

## Done when
- `grep -n "owner's ruling" decisions/search.md decisions/trade.md`
  finds no line saying a line changes only by his ruling, and C101's
  body does not say a change to what a user types is ruled first.
- `python3 -I tools/bothy.py show C<n>` prints a retired line for each
  entry of decision 2, or the close names it as left and why.
- Each retired line's `*Held by:*` files carry a comment under the id
  saying what that file holds, and the first carries the reason.
- Each retiring commit has its `retired C<n>; held by <Y>` lines and
  names each sentence changed or cut as it moved.
- Each new test fails with its product line taken out, shown by hand
  and said in its commit. A golden's diff is read and said.
- `tools/gate.sh` exits 0 on the step's tree.
- One cold review, saved whole, and its findings answered.

## Not in scope
- Any entry not named above: C113, C97, C102 and C114 stay, and the
  other areas are their own steps.
- Moving mechanism out of entries that stay: the shedding pass.
- The query language, a table or a product behaviour: a test that
  needs one changed leaves its entry standing.
- `tools/bothy.py` and `tools/docs-check.sh`.

## Rules
`TEMPLATE.md`. What differs:
- Run each new test alone first (`cargo nextest run -p <crate> <name>`),
  never the workspace, until the one full gate.
- The cold review is given this brief's name and the commits, by the
  cold-review skill.

## Report
`TEMPLATE.md`, and three things more: for each entry, how many clauses
you found held as the census said, loose and fixed, or loose and left;
how many files each retirement touched; and what you would change in
this brief before the next batch is written from it.
