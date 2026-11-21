# The road

What is happening now, what is next, and what only the owner can do.
Each close rewrites this file, and it holds no history: what it said
before is `git log -p -- roadmap/README.md`, and the long record it
carried until 2026-11-09 is `roadmap/README.md` at `73e7e77c`. How a
step runs, who decides, and how work is reviewed and closed is
`decisions/process.md`. The goal is the owner's: a faster, simpler,
better Acquisition that is agent-friendly and agent-forward.

## Now

- **The #293 removals** (`audits/assessment-6-5/README.md`, "The
  candidates, ranked", items 1 to 5): all five cuts are closed
  (`git log --grep='^Step: cut-293'`); the last, cut 5, took history out
  of the code and the current documents (`AGENTS.md`, "Where things
  live", History). #293 stays open.
- **The GUI's first slice, 4-full part 1** (`briefs/4-full-1-design.md`;
  milestone "4-full, the GUI"). Steps 1 to 5 are closed. Step 6's view
  is designed and ruled (`briefs/4-full-1-step6-design.md`, values
  over places; "Approved as designed.", 2026-11-11), and built in
  series as its §5 lays out. Values on rows (#38), the count (#245),
  the spike, the browser-driven check (step 6.4a, C187: every gate
  machine needs a Chromium-family browser) and the windowed table
  (step 6.4b) are closed. So is the table's next part (step 6.4c,
  `briefs/4-full-1-step6-sort.md`): nothing is typed to sort, a value
  column's head orders the rows by it and flips, a number on a card
  adds its value as a column, and the URL names the values
  (`view.value`). Step 6.5, the places and the grouped table (the
  design's §3 and §5 item 5), is closed in four parts: 6.5a, the place
  pages (`briefs/4-full-1-step6-places.md`); 6.5d, the plan, apply and
  jobs pages in a player's words (`briefs/4-full-1-step6-plan-jobs.md`);
  6.5b, a place page under a query and a filter, each open place a
  segment of the page's one window (`briefs/4-full-1-step6-grouped.md`);
  and 6.5c (`briefs/4-full-1-step6-row.md`): a row opens its item as a
  card under it, the table cut at it and the reader kept at its row, the
  card's place a link to that place opened and the query against it one
  link away; a refused query keeps its box, and the save form says what
  a save leaves behind. Next is the search form's reshaping.

## Next

In order. A session takes the first item (C180), skipping one that
names a wait not yet met, without a planning session; an item leaves
when its step closes or the owner strikes it.

1. 4-full part 1, the search form's reshaping (#184, #181, #180,
   #185, #188, #183, #193), one design, its brief first; #190's rest
   (the save form's, the big form's and the plan's refusals) rides with
   it, and so does a sort by values a player names (the page seat after
   step 6, fault 2; `audits/web-step6/seat/README.md`) and #296, a
   place's line, as ruled on it. A saved search's row keeps its source
   (#193, as ruled on it). The look is #285; what an apply brought, said
   on its job, is #182's rest; the never-fetched tabs of a league the
   scope's link does not name are #194's rest.
2. The frontends' whole-code review, once item 1 closes.
3. 4-full parts 2 and 3, each its own design session
   (`briefs/4-full-design-prompt.md`): the subscriber and the wire,
   then the resident and morning 2.
4. The test-shape group's t6, one canonical mock world and the door's
   corpus (#142), designed from the GUI's door; t7 (#143) only if t6
   shows the shape pays.
5. The queued search and trade issues: #283 (the translation builds
   two answers of the sitting of 2026-11-17), #46, #47, and #37 once
   the owner has sat the site.
6. `alpha.2` (milestone "League 3.30 and alpha.2"): the release, its
   `keyring-macos` job dispatched once by hand before it; the tables
   stand at 3.30 (the close of `search-276`). Before it: #256, #222 and
   the facts door (#212), which the prune verb (#43) waits on. #281,
   the export's lost trade-id joins, waits on the next pin.
7. PoE2 (milestone "PoE2 launch"): the route flip and the
   first-contact checklist prepared unmerged; a first contact only if
   GGG ships the stash endpoint.

Issues labelled `trigger:step` are taken by the next step in their
area, and an issue the owner names is its own step whatever its
label: `gh issue list --label trigger:step`.

## Waiting on the owner

His queue is the open issues labelled `trigger:ruling`:
`gh issue list --label trigger:ruling`. What only he can do:

- The league run under the rails, at his word: count the undecided
  classes and the untranslated mods.
- At the next live login, deny once: whether GGG's authorize page
  echoes `state` on a denial is a fact for
  `docs/design/network-ground-truth.md`.
- At the next live run, `tools/persist-check.sh`, not run since it was
  last edited.
- At the next live run, `tools/tracer-rung.sh`, rewritten as the
  evidence run's driver (C189) and run against the mock alone.
- The site sitting #37 waits on; it can also ask what the sitting of
  2026-11-17 left open on #33 (which pair the site compares when an
  item's two differ, a prefixed map Base's refusal captured, what lifted
  e043's ring) and #34's jewel question.
- Whether to tell the RePoE fork that its 3.30 export joins no trade
  id for a string with markup (#281): a post on another project's
  tracker.
- After 2026-11-30, re-measure what the governance rebuild changed
  (#220), and rule the gate's four questions on its record (#257,
  #225).

## Starting a session

1. Read this file, then `decisions/process.md`, then the step's brief
   or issue and what it names.
2. `STATE-2026-10-19.md`, `STATE-2026-10-26.md`, `GUI-NOTES.md`,
   `TIME-SEAM.md`, `TEST-SHAPE.md` and
   `LANGUAGE-RETROSPECTIVE-2026-11-09.md` are dated evidence: read when
   a brief points into them, never rewritten. `LEDGER.md` is the
   closed steps to 2026-11-09 and stands as written; a later close is
   its commit, `git log --grep='^Step: '`.
