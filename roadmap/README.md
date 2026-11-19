# The road

What is happening now, what is next, and what only the owner can do.
Each close rewrites this file, and it holds no history: what it said
before is `git log -p -- roadmap/README.md`, and the long record it
carried until 2026-11-09 is `roadmap/README.md` at `73e7e77c`. How a
step runs, who decides, and how work is reviewed and closed is
`decisions/process.md`. The goal is the owner's: a faster, simpler,
better Acquisition that is agent-friendly and agent-forward.

## Now

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
  design's §3 and §5 item 5), is cut into four parts. Its first, 6.5a,
  is closed (`briefs/4-full-1-step6-places.md`): "Stash tabs" and
  "Characters" replace "Refresh", each a page of the search crate's
  places document, a parent closed until opened, the URL naming what is
  open and ticked, an opener landing at its head, a place's name its
  items as a table, and a league with nothing listed offering its
  listing on the page. Next is 6.5b, whose brief is written first.

## Next

In order. A session takes the first item (C180), skipping one that
names a wait not yet met, without a planning session; an item leaves
when its step closes or the owner strikes it.

1. 4-full part 1, step 6.5, ruled in `briefs/4-full-1-step6-design.md`
   §3, in four parts; 6.5a, the places and their pages, is closed.
   Then, a brief each, written first: 6.5b, the grouped table and
   the list's filter as the request's (#244, #269, #194's
   never-fetched view; #294, how the search names a value, is an input); 6.5c, an item under its row and the row's
   ways (#270, #194, #186, #190); 6.5d, the plan and jobs pages from
   their documents (#182, #189). The search form's reshaping (#184, #181, #180, #185,
   #188, #183, #193) is the step after, one design; the look is #285.
2. 4-full parts 2 and 3, each its own design session
   (`briefs/4-full-design-prompt.md`): the subscriber and the wire,
   then the resident and morning 2.
3. The test-shape group's t6, one canonical mock world and the door's
   corpus (#142), designed from the GUI's door; t7 (#143) only if t6
   shows the shape pays.
4. The queued search and trade issues: #283 (the translation builds
   two answers of the sitting of 2026-11-17), #46, #47, and #37 once
   the owner has sat the site.
5. `alpha.2` (milestone "League 3.30 and alpha.2"): the release, where
   the `keyring-macos` job runs for the first time; the tables stand at
   3.30 (the close of `search-276`). The prune verb (#43) rides with
   it, and #281, the export's lost trade-id joins, waits on the next
   pin.
6. PoE2 (milestone "PoE2 launch"): the route flip and the
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
  (#220).

## Starting a session

1. Read this file, then `decisions/process.md`, then the step's brief
   or issue and what it names.
2. `STATE-2026-10-19.md`, `STATE-2026-10-26.md`, `GUI-NOTES.md`,
   `TIME-SEAM.md`, `TEST-SHAPE.md` and
   `LANGUAGE-RETROSPECTIVE-2026-11-09.md` are dated evidence: read when
   a brief points into them, never rewritten. `LEDGER.md` is the
   closed steps to 2026-11-09 and stands as written; a later close is
   its commit, `git log --grep='^Step: '`.
