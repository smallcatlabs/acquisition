# process-345 — the pricing, plans, store, frontends and network rulings their tests already hold retire

Issue #345. The last batch of the registry's retirements
(`audits/retire-2026-11/README.md`, "A proposed order"), its third and
fourth taken together. The first two are `Step: process-340` and
`Step: process-343`; read their closes for what a batch meets.

## Goal
Up to twenty rulings of `decisions/pricing.md`, `plans.md`, `store.md`,
`frontends.md` and `network.md` stand as one line each under "Retired",
and what each said is a comment under its id at the test or type that
holds it.

## Inputs
- The retired rule: `tools/bothy.py`'s docstring. `decisions/search.md`'s
  "Retired" section is the form to copy.
- C154 (the retirement line, `retired X; held by Y`), C182 (structure
  before a rule: no new entry, row or rule), C192 (a property in the
  entry, its mechanism at the code), C170 (who decides: a session
  amends an entry outside the owner's classes), C153 (a new test is a
  row or a case of a surface that exists, never a file), C122 (pinned
  once, at the lowest crate; a golden is regenerated under
  `ACQ_UPDATE_FIXTURES=1` and its diff read), C47 (the store and plan
  crates deny `unwrap` and `expect` in production code).
- The census's reads, one block per entry, holders by file and line as
  of `50e28e68`: `audits/retire-2026-11/reports/first/pricing.md`,
  `plans.md`, `store.md`, `frontends-2.md` and `network.md`; a second,
  blind read of C56, C69 and C145 under `reports/second/`. A reader's
  claims, not proof.
- The owner's answer of 2026-11-27 (the census's "The owner's
  answers", 1): the small tests are written now.
- The last push's CI, read as the template says.

## Decisions
1. **The entries.** As they stand: C69, C70, C81, C80 (pricing); C36,
   C77 (plans); C54, C56, C62, C188, C57 (store); C163 (frontends);
   C17, C161 (network). A clause a reader marks `cut` or unholdable
   (C57, C62, C188 and C161 have one each) leaves, its reason said in
   the commit, or stays in the comment if you judge it a promise.
   After one test each, as its reader's `unheld` line names it: C126
   (a failed leagues response does not replace the stored list), C37,
   C76 (plans); C29, C145 (the wrong-typed arrays its table leaves
   out), C60 (the realm is the request's, never the entry's field).
2. **C121 is left.** Its words and the code disagree (#339), and it
   retires with that issue.
3. **An entry that turns out to hold a promise in the owner's classes
   is left**, and the close says so: what the daemon sends to GGG, a
   boundary between crates or processes, his data, scope. The census
   classed these twenty a session's; the network two are the nearest.
4. **You read every clause's test yourself before retiring its entry,
   and you split a clause by each code path that can break it.** A
   Before writing a test for a clause a reader calls loose, search
   the crate's other test files for one that holds it: the second
   batch found one neither reader cited. A clause still loose gets
   its assertion as a row of a table harness the crate has, with no
   new fixture and no product change. Otherwise the entry stays, and
   the close says which and why.
5. **A promise made for every platform and held on one is not held.**
6. **A rule about what a later author writes, which no test can fail
   on, does not retire silently.** It stands as a doc comment where
   that author will work, and the close lists each such sentence, so
   the orderer can weigh it.
7. **Where the words go.** `*Held by:*` names the fewest files, three
   at most, whose tests or types hold every promise as the entry states
   it; a file holding only mechanism-level cases keeps its citation and
   is not named, a fourth holder is named in the first file's comment,
   and no test is written only so that three files cover an entry. The
   comment under the id at each says what that file holds, and the
   reason is whole at the first, said as it stands now: no date, step,
   seat or review. A sentence placed at a holder is no wider than the
   code beside it. A mechanism sentence goes to the doc comment on the
   code that does it, and a source file that quotes a retired entry
   word for word points at the holder instead.
8. **Words the code has outgrown are corrected as they move, and the
   commit says each difference**, a stale clause in a headline among
   them. Known: C69 cites one trade claim where its tests and module
   doc cite another; C163 is marked unbuilt and is built, and its
   holder catches a new field only through a list kept by hand, so its
   comment stands at that list; C17 states no promise of its own
   (invariant 2 says it) and retires to the doc comment that already
   does; C188's third clause is a fact about GGG with no numbered
   claim, which stays in its comment and is named in the close, no
   claim added; `AGENTS.md` cites C161, and those citations stay. If a
   difference is a promise the code breaks, stop that entry and file an
   issue. A "Not doing" line whose trigger a retirement shows met is
   deleted, said in the commit.
9. **The commits.** Each new test its own commit; then the retirements
   as one commit per area file, each carrying one
   `retired C<n>; held by <Y>` line per entry.

## Find first
- `for n in 69 70 81 80 36 77 54 56 62 188 57 163 17 161 126 37 76 29
  145 60; do python3 -I tools/bothy.py show C$n; done`: each entry
  whole and every place that cites it.
- `grep -n -A16 '^### C<n> ' audits/retire-2026-11/reports/first/*.md
  audits/retire-2026-11/reports/second/*.md`: the readers' claims.
- `python3 -I tools/bothy.py pack --path <files>` over a retirement's
  holder files, in one call.
- `python3 -I tools/bothy.py check` before and after.

## Done when
- `python3 -I tools/bothy.py show C<n>` prints a retired line for each
  entry of decision 1, or the close names it as left and why.
- Each retired line's `*Held by:*` files carry a comment under the id
  saying what that file holds, and the first carries the reason.
- Each retiring commit has its `retired C<n>; held by <Y>` lines and
  names each sentence changed or cut as it moved.
- Each new test fails with its product line taken out, shown by hand
  and said in its commit. A golden's diff is read and said.
- The close lists each sentence of decision 6.
- `tools/gate.sh` exits 0 on the step's tree.
- One cold review, saved whole, and its findings answered.

## Not in scope
- Any entry not named above, C121 among them.
- Moving mechanism out of entries that stay: the shedding pass.
- A product behaviour, a table or the protocol: a test that needs one
  changed leaves its entry standing.
- `tools/bothy.py` and `tools/docs-check.sh`.

## Rules
`TEMPLATE.md`. What differs:
- Run each new test alone first (`cargo nextest run -p <crate> <name>`),
  never the workspace, until the one full gate.
- The cold review is given this brief's name and the commits, by the
  cold-review skill.

## Report
`TEMPLATE.md`, and two things more: for each entry, how many clauses
you found held as the census said, loose and fixed, or loose and left;
and how many files each retirement touched.
