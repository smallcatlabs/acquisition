# governance-tools-1 — the registry's entry rule becomes a shape

## Goal
`tools/bothy.py` holds what a registry entry may be by its shape, not
its bytes: a headline that states the rule whole, a handful of
clauses, and a `*Pinned:*` that is short and true. Its `list` is the
index an agent searches, and its `pack --path` leads with what the
file itself cites. The 800-byte limit rewarded compression and passed
the densest entries; a shape has an outlet a byte count lacks, which
is a new id.

## Inputs
- The owner's ruling of 2026-11-09, quoted in the commit that adds
  this brief: three shape rules in place of the byte limit.
- `decisions/process.md`: C182 (a fault becomes structure before a
  rule), C174 (a brief carries commands, never their output).
- `tools/bothy.py`: its docstring is the entry rule; `check`, `list`,
  `show` and `pack` are the commands this step changes.
- `tools/bothy-breakers.sh`: the fixture that proves each refusal.
- `AGENTS.md`, "Invariants": each item now ends with `*Pinned:*`.
- CI: read both gates' last runs on `main` before starting, as
  `TEMPLATE.md` says, and name them in the report.

## Decisions
1. **Three rules, each failing `check`.** The headline (the bold
   sentence after the id) is at most 160 bytes. The headline and the
   body together are at most six clauses. `*Pinned:*` names at most
   three files, and every name it ticks exists.
2. **A clause** is counted on the text before `*Why:*`, with code
   spans and parenthesised text removed, split at `. ` or `; ` and at
   a final `.` or `;`; the non-empty pieces are the clauses.
3. **A baseline that only shrinks.** `decisions/shape-baseline.tsv`
   holds one row per entry that is over a headline or clause limit
   when this step lands: id, headline bytes, clauses. An entry in the
   baseline may not exceed its row. An entry not in it meets the
   limits. A row whose entry now meets both limits fails as stale and
   says to remove the row. Splitting a long entry is the owner's
   ruling and is not this step's.
4. **`*Pinned:*` has no baseline.** This step trims every one that
   names more than three files. Keep, in this order: a structural
   check; the test at the lowest crate that holds the property; a
   test whose name carries the id's prefix. Change no word before
   `*Pinned:*`.
5. **A ticked name exists.** A ticked token with a `/` or a file
   extension is a path, resolved as today's path check resolves one.
   Any other ticked token occurs in a file the same `*Pinned:*` names,
   as written or by its last `::` segment; where no file is named, it
   occurs under `crates/`, `tools/` or `.github/`. `nothing yet`
   passes.
6. **The invariants are checked like entries**: each numbered item
   carries `*Pinned:*`, under rules 4's cap and 5.
7. **`list` prints each headline whole.**
8. **`pack --path <file>`** prints, in order: the entries the file
   cites by id, whole; the headlines of its area's other entries; the
   area's "Not doing". Neither `pack` form repeats `AGENTS.md`, which
   every session has loaded; one line says so. A path that does not
   exist is refused.
9. **The docstring's entry rule** states rules 1, 3 and 5 here in
   place of its byte limit, and adds: an entry's ruling and its
   `*Why:*` change only by the owner's ruling, and its `*Pinned:*` is
   kept true by whoever moves a holder.

## Find first
- The entries over each limit, by the counter decision 2 defines: the
  rows of the baseline.
- The `*Pinned:*` fields naming more than three files, and for each
  holder you would drop, whether its test name or doc comment cites
  the id: `python3 -I tools/bothy.py show C<n>` lists where code
  names it.
- Every reader of bothy's output:
  `git grep -n 'bothy' -- ':!roadmap/reviews' ':!roadmap/briefs' ':!audits'`
- Whether any test or build reads what this step changes:
  `git grep -n -E 'bothy|shape-baseline|decisions/' -- 'crates/*.rs' 'crates/*/build.rs'`

## Done when
- `python3 -I tools/bothy.py check` exits 0 on the tree.
- `tools/bothy-breakers.sh` exits 0 and has a case that fails `check`
  for each of: a headline over 160 bytes outside the baseline; a
  seventh clause; an entry over its baseline row; a stale baseline
  row; a `*Pinned:*` naming four files; a `*Pinned:*` ticking a name
  found nowhere; an invariant with no `*Pinned:*`; `pack --path` on a
  path that does not exist. Each refusal says what is wrong and what
  to do.
- No `*Pinned:*` names more than three files, and the commit that
  trims them changes no text before `*Pinned:*` in any entry; the
  commit message gives the command that shows it.
- `python3 -I tools/bothy.py list` prints no cut headline.
- `pack --path` on a file that cites ids prints those entries first
  and nothing of `AGENTS.md`; the report gives its size before and
  after for three files in three crates.
- The docstring states the shape rules and no byte limit.
- `tools/gate.sh --docs` exits 0 on the step's tree.

## Not in scope
- `tools/docs-check.sh`: its entry-length note goes in
  `governance-tools-2.md`; the two tools keep one reporter per fault.
- The words of any ruling or `*Why:*`, and splitting any entry.
- Anything under `crates/`: where a dropped holder does not cite its
  id, list it in the report and edit no code.
- `tools/arch-check.sh`.

## Rules
`TEMPLATE.md`. For this step: `tools/gate.sh --docs` is the gate,
once "Find first" shows no test or build reads what changes; the
orchestrating session runs the full gate after the sibling step.
End each commit message with
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Report
`TEMPLATE.md`.
