# 1b — The registry: entries under the rule, not doing, issues

A worker reads `roadmap/README.md`, then this brief, then what it
names, and nothing else unless the work turns it up. The owner has
ruled on `roadmap/governance/DESIGN.md` before this brief starts.

## Goal
Every ruling in `CONTEXT.md` and `decisions/*.md` is one bullet under
the entry rule, with no history in it; every "Parked" section is gone,
replaced by a "Not doing" list of twenty lines across the area files
and by 48 GitHub issues; the checks hold the new form. After it an
area file is rulings and boundaries only, and the backlog is on
GitHub.

## Read
- `roadmap/governance/ENTRY-RULE.md`: the rule and ten samples, the
  form every entry takes.
- `roadmap/governance/PARKS.md`: the twenty lines, the deletion list,
  the issue table with labels and bodies.
- `roadmap/governance/AGENTS.draft.md`, "Cross-cutting rulings": the
  fourteen cross-cutting entries and C120 as they will stand; write
  them into `CONTEXT.md` now in that form (step 1c moves them).
- `CONTEXT.md`, "Decisions — the registry" and "Parked": today's
  grammar, which `tools/bothy.py` reads; `tools/docs-check.sh`,
  section 2; `tools/bothy-breakers.sh`.
- The memory note "simplicity over mechanism": a change removes agent
  work and reuses a shape the tool has.

## Input
No measurement. The failing inputs the checks stage: an entry carrying
a date, a seven-hex commit, "owner:", "verbatim", "Amended", "Decided",
"Ruled" or "call N" (each refused by name); a "Not doing" line without
"until" or "refused" (refused by `bothy.py check`).

## Do
1. Rewrite every entry under the rule, area by area, reading the
   current entry whole before writing its replacement: a ruling in
   plain declaratives, one *Why*, one *Pinned* (or `nothing yet`), a
   gloss on every cited id, nothing else. Where an entry's *Pinned* or
   *Details* named a file, check the file exists before carrying the
   name; a *Pinned* that names a test prefix (`c35_`) is checked by
   grep. The ten samples are taken verbatim. A ruling whose build waits
   (C71, C73, C78, C99) says `Unbuilt` and names its issue by number
   once the issues exist. C1 stays pinned to `tools/docs-check.sh`
   until 1e moves the edges. C101 gains one sentence: a change to what
   a user types is recorded as a ruling before the code, under the
   owner's delegation (the search plan's rule 4, read with
   `AGENTS.md`, "Who decides").
2. Add C120 to `CONTEXT.md`'s cross-cutting section in the draft's
   words, and C110's last clause.
3. Replace each file's "Parked" section with "## Not doing" and the
   lines `PARKS.md` section 1 gives it; delete the three cross-cutting
   parks. Delete every other park. The commit message cites the commit
   before this one as holding every park's text.
4. Create the thirteen labels and the 48 issues from `PARKS.md`
   section 3 with a script in your scratch (`gh label create`, `gh
   issue create --title --label --body`), after the owner's go on the
   list; the script skips a title that already exists. Fill `<commit>`
   in the two bodies that carry it with the commit before this step's.
   Put each issue's number into the entry or line that names it.
5. `tools/bothy.py`: the "Parked" parser reads "Not doing" instead, a
   line is `- <item>: not until <trigger>` or `- <item>: refused …`,
   and `check` refuses one that states neither; the docstring says so.
   `tools/bothy-breakers.sh`: the park cases become not-doing cases.
6. `tools/docs-check.sh`, section 2: refuse an entry that carries a
   history marker (the list under Input), naming the entry and the
   marker.
7. The gate; one commit per concern (the rewrite, the parks, the
   checks); the review.

## Done when
- Every entry passes the history scan; `bothy.py check` reports zero
  entries without *Why*.
- `grep -c '^## Parked' CONTEXT.md decisions/*.md` is zero;
  `bothy.py list parks` prints the twenty not-doing lines.
- `gh issue list --limit 100` shows 48 issues, each with one `area:*`
  and one `trigger:*` label at least.
- The gate green.

## Not in scope
- `AGENTS.md` and `CONTEXT.md`'s structure (1c).
- The records, the notes, `LIVE-TESTING.md` (1d).
- Code doc comments (phase 2, as files are touched).

## Rules
- Every `acq` runs in a mock world: `ACQ_PROVIDER=mock` and
  `ACQ_STORE_DIR` under your scratch. Never the real provider.
- The main tree; your own scratch subdirectory, named in the launch.
- The gate (`AGENTS.md`, "Quality gate") before the commit, green on
  this step's own tree with nothing a later step delivers; the commit
  gated on each stage's own exit status, never on a filtered grep.
  Never run the drivers while the gate runs.
- One commit per concern; the story in the message; a size stated in a
  message is measured after the last edit.
- A finding is a test named for the fault, or a line in the report;
  never a new document.
- A deletion names the commit that holds the deleted text.
- After the commit, launch the review (`roadmap/README.md`, "Roles and
  rhythm"), save its findings verbatim at
  `roadmap/reviews/1b-<sha>.md` and quote them in your report; do not
  fix them before the owner has read them.

## Report
Returned to the session that launched you, in this order, each a short
paragraph or a table:
1. What landed: the commits, one line each.
2. What the gate said, as exit statuses per stage.
3. Measurements: entry bytes before and after, as a distribution.
4. What you left out and why.
5. What surprised you, and what you would retract from your own report.
6. Raised for review: anything about the brief, the plan, the tools or
   this way of working that got in your way or that you think is wrong,
   and what would have made the job easier.
