# process-340 — the daemon rulings their tests already hold retire

Issue #340. The first of four batches the registry census proposed
(`audits/retire-2026-11/README.md`, "A proposed order"), ordered at the
owner's word of 2026-11-27.

## Goal
Eleven rulings of `decisions/daemon.md` stand as one line each under
"Retired", and what each said is a comment under its id at the test or
type that holds it. A builder in the daemon then finds a rule and its
reason where the code is, and the area file is shorter by what a test
already says.

## Inputs
- The retired rule: `tools/bothy.py`'s docstring, "The retired rule".
  `decisions/process.md`'s "Retired" section is the form to copy.
- C154 (the retirement line, `retired X; held by Y`, Y a test, a type
  or a check that runs in the gate), C182 (structure before a rule: no
  new entry, row or rule comes of this), C192 (a property in the entry,
  its mechanism at the code), C170 (who decides: a session amends an
  entry outside the owner's classes), C153 (a finding joins a surface
  that holds it: a new test is a row or a case, never a file), C122 (a
  behaviour pinned once, at the lowest crate).
- The census's reads, one block per entry, with each clause's holder by
  file and line as of `50e28e68`:
  `audits/retire-2026-11/reports/first/daemon-1.md`, `daemon-2.md`,
  `daemon-3.md`, and for C6, C27 and C86 a second, blind read under
  `reports/second/`. They are a reader's claims, not proof.
- `TESTING-NOTES.md` for the daemon's test surfaces.
- The last push's CI, read as the template says.

## Decisions
1. **The entries.** As they stand: C152, C5, C165, C166, C86, C160,
   C45, C49. After one test each: C6 (`acqd` refuses to start on a
   queue it cannot read; only the restore's error is tested today),
   C23 (cancelling a parent cancels its children; no test reads a child
   after its parent's cancel), C27 (a closed terminal, SIGHUP, beside
   Ctrl-C).
2. **You read every clause's test yourself before retiring its entry.**
   A retirement claims the holder holds every promise of the entry,
   and where two census readers read the same clause the second found
   some loose that the first called held (the census's "The cost", its
   caution). A clause you
   find loose gets its assertion if that fits a table or harness the
   crate has, with no new fixture and no product change. Otherwise the
   entry stays, and the close says which and why.
3. **Where the words go.** `*Held by:*` names the fewest files, three
   at most, whose tests or types between them hold every promise. The
   comment under the id at each says what that file holds; the reason
   is written whole at the first file named. A mechanism sentence goes
   to the doc comment on the code that does it. Every other citation of
   the id in the tree stays as it is.
4. **An entry's words the code has outgrown are corrected as they
   move, and the commit says each difference.** Known: C5 says one
   priority level is in use, and probes and logins use two more. If a
   difference is a promise the code breaks, stop that entry and file an
   issue; do not write the comment to match the fault.
5. **Single entries.** C152: the second question of the census left
   one clause open at low confidence (a new page reading the journal's
   path from the words); read that clause first. C49 states no promise:
   it retires to the doc comment that already says it. C165: its
   holder catches a new document only through a list kept by hand, so
   the comment stands at that list. C5 is cited by a "Not doing" line
   of `decisions/daemon.md`, which then cites a retired id; that
   resolves, and the line's words are checked against what C5 said.
   C27 has one clause its reader marked to cut: name it and its reason
   in the commit, or keep it in the comment if you judge it a promise.
6. **The commits.** Each new test is its own commit, before the
   retirements. The retirements are one commit, carrying one
   `retired C<n>; held by <Y>` line each. An entry in `AGENTS.md` or
   another area's file that cites a retired id by a gloss keeps it.

## Find first
- `for n in 152 5 165 166 86 160 45 49 6 23 27; do python3 -I
  tools/bothy.py show C$n; done`: each entry whole and every place
  that cites it.
- `git grep -n -w -E 'C(152|5|165|166|86|160|45|49|6|23|27)' -- .
  ':!audits' ':!roadmap/reviews' ':!roadmap/briefs' ':!spikes'`: the
  citations that must still resolve, a `*Pinned:*` or a cross-reference
  in another entry among them.
- `grep -n -A14 '^### C<n> ' audits/retire-2026-11/reports/first/daemon-*.md`:
  the reader's clause-by-clause claim for an entry.
- `python3 -I tools/bothy.py check` before and after: its first line
  counts the decisions and the retired.

## Done when
- `python3 -I tools/bothy.py show C<n>` prints a retired line for each
  of the eleven, or the close message names the entry as left and why.
- Each retired line's `*Held by:*` files carry a comment under the id
  that says the property that file holds, and the first carries the
  entry's reason.
- The retiring commit's message has a `retired C<n>; held by <Y>` line
  for each, and names each sentence it changed or cut as it moved.
- Each of the three tests fails with its product line taken out, shown
  by hand and said in its commit.
- `tools/gate.sh` exits 0 on the step's tree (a comment moves lines
  that check 16 of `tools/arch-check.sh` reads).
- One cold review, saved whole, and its findings answered.

## Not in scope
- Any entry not named above, in any area: the other three batches are
  their own steps.
- Moving mechanism sentences out of entries that stay: the shedding
  pass is its own step.
- The 16 entries the census kept for one open clause each (C3, C156
  among them), and the faults under "What the readers noticed" that
  touch none of the eleven; #338 and #339 are filed.
- A change to `tools/bothy.py` or `tools/docs-check.sh`. If the retired
  form refuses something this batch needs, stop and say so.

## Rules
`TEMPLATE.md`. What differs:
- Run each new test alone first (`cargo nextest run -p <crate> <name>`),
  never the workspace, until the one full gate.
- The cold review is given this brief's name and the commits, by the
  cold-review skill.

## Report
`TEMPLATE.md`, and three things more: for each entry, how many of its
clauses you found held as the census said, loose and fixed, or loose
and left; how many files each retirement touched; and what you would
change in this brief before the next batch is written from it.
