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
  over places; "Approved as designed.", 2026-11-11). Its build is that
  file's §5, in series. Values on rows (#38) and the count (#245) are
  closed (`briefs/4-full-1-step6-values.md`,
  `briefs/4-full-1-step6-count.md`). The spike (the design's §5 item
  3) is closed in its two halves. The check's driver is measured
  (`audits/web-2026-11/driver/README.md`): a hand-written one works.
  The window script holds the table in the owner's Brave, flat and
  grouped, at 60 Hz (`audits/web-2026-11/spike/README.md`; his words:
  "All passed. Nothing flickered."). Next is the flat table, step 6.4.
  It waits on its brief and on the route that serves a window of rows
  (#268), with the driver and the check's home (#266, #267) beside it.
  Then the places and the grouped table, step 6.5, where tabs nest
  under their parent (#269); opening something keeps the reader's
  place in both (#270).

## Next

In order. A session takes the first item (C180), skipping one that
names a wait not yet met, without a planning session; an item leaves
when its step closes or the owner strikes it.

1. 4-full part 1, step 6: the dense table, its view ruled
   (`briefs/4-full-1-step6-design.md`) and built in series: the flat
   table (6.4), once its brief is written and #268 ruled; then the
   places and the grouped table (6.5): #244 with the place pages
   (#248, #194, #189, #182), #191, and #186 and #190 riding, with
   #269 and #270. The search form's reshaping (#184, #181, #180, #185,
   #188, #183, #193) is the step after, one design.
2. 4-full parts 2 and 3, each its own design session
   (`briefs/4-full-design-prompt.md`): the subscriber and the wire,
   then the resident and morning 2.
3. The test-shape group's t6, one canonical mock world and the door's
   corpus (#142), designed from the GUI's door; t7 (#143) only if t6
   shows the shape pays.
4. The queued search and trade issues: #46, #47, and #37 once the
   owner has sat the site.
5. `alpha.2` (milestone "League 3.30 and alpha.2"): regenerate the
   tables when their inputs exist (a post-launch `poe1` pin, the
   owner's four site captures), then the release, where the
   `keyring-macos` job runs for the first time. The prune verb (#43)
   rides with it.
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
- At the next live run, `tools/tracer-rung.sh`, edited (#246: it passes
  `--account` where it exported `ACQ_ACCOUNT`) and not run.
- The site sitting #37 waits on.
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
