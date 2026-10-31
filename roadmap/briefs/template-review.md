# template-review — the brief template keeps its rules and loses its ledger

## Goal
`roadmap/briefs/TEMPLATE.md` is read by everyone who writes a brief and
every worker who runs one: 57 of the 67 briefs cite it, and the README
points to it five times. It was created on 2026-10-19 (`a987d02f`) at
1,794 bytes and has grown to 8,390 bytes in 27 commits. It grew by
appending lessons, each a rule told together with the incident that
taught it, and nothing has retired a line since. When this step is
done:
- each rule is stated once, in the fewest words that hold it, and
  cites the incident behind it in a few words at most;
- a line that a check now holds is retired, naming the check;
- nothing repeats the README's "A step";
- a brief writer and a worker can each find their part quickly.

The owner asked for this review on 2026-10-30: "yes, let's do the
review here or with an Opus worker".

## Read
- `roadmap/briefs/TEMPLATE.md`, the whole file, and its history
  (`git log --follow -p -- roadmap/briefs/TEMPLATE.md`). The commit
  that added a line says why.
- `roadmap/README.md`:
  - "The owner's rulings for this road": a template line that records
    one of his rulings stays as a rule. It may be shortened, but not
    cut.
  - "A step", for overlap.
  - The five places that point to the template, so each pointer still
    lands on the part it names. "Rules" (README:151) is one of them.
- `AGENTS.md`, "Hard rules" and C153 and C154 (a finding joins the
  surface that holds it; a retirement names what now holds the
  property), for overlap and for the retirement form.
- `tools/gate.sh`, to see which template lines a stage now holds. The
  `tree` stage is one candidate; check each before claiming it.
- `REVIEW-CHECKLIST.md`, the home for a fault that came back. A lesson
  about a review-time fault may belong there rather than here; say
  which.

## Input
Measured at `f26bb2c8`: 152 lines, 8,390 bytes, eight headings, 12
top-level bullets under Rules. A rough regex count finds about 55
references to past steps or issues. The build re-counts this with a
command it names. CI at `5c31632a`: `Gate (Linux)` run 41147851380
success, and `Gate (Windows)` run 41147851392 cancelled by the next
push. `f26bb2c8`'s runs (41148435112, 41148435110) were in progress
when this brief was written.

## Do
1. Classify every sentence of the template in a table in your report:
   - rule, kept;
   - rule, shortened;
   - story, cut to a citation;
   - held by a check, retired;
   - duplicate of the README or `AGENTS.md`, cut;
   - moved (to `REVIEW-CHECKLIST.md` or elsewhere);
   - owner's ruling, kept.

   Use a script to list the sentences, so the table is complete.
2. Rewrite the template from that table, in one commit. Keep the eight
   headings and the six-section Report, which every brief and the
   README name. You may reorder the guidance inside a heading, and you
   may mark which lines bind the brief's writer and which bind the
   worker. Each retired line gets a `retired X; held by Y` line in the
   commit message (C154). A line you cannot place stays as it is and
   goes in the report.
3. A citation keeps what a reader needs to find the incident (the step
   name or an issue number, at most a clause). The incident's story
   lives in git and the step's commits.
4. Anything you move to `REVIEW-CHECKLIST.md` or the README is in the
   same commit, and the README's pointers still land on the part they
   name.
5. Run `tools/gate.sh --docs`. The step changes no Rust, so the docs
   stages are the gate that applies; say so in the report.

## Done when
- Every sentence of the template at `f26bb2c8` has a fate in the
  report's table, and the commit message carries each retirement line.
- No rule was lost: each "rule" row's rule is in the new template or in
  the home the table names.
- The owner's rulings that the template recorded are still in it.
- Every README pointer to the template lands on what it names.
- The new size is in the commit message, measured after the last edit.
- `tools/gate.sh --docs` exits 0.

## Not in scope
- New rules or lessons. A gap you see goes in the report.
- Changing "A step" or the README's rulings.
- Rewriting existing briefs to the new template.
- Whether the template should hold lessons at all, as a design
  question. If the work suggests an answer, give it in the report.

## Rules
`TEMPLATE.md`, the version at `f26bb2c8`, which is the one you are
changing. What differs:
- You work in the main checkout, with scratch only under
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/70b56735-1662-47f4-9f40-9a9bbfd8aaf9/scratchpad/tpl/`.
  No build is needed. Commit; never push.
- The orchestrating session reviews the step (a larger prose edit,
  the owner's practice since 2026-10-30).

## Report
`TEMPLATE.md`'s six sections. The classification table goes under
section 3.
