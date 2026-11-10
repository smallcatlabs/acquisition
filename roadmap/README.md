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
  milestone "4-full, the GUI"). Steps 1 to 5 are closed: the saved
  search, its MCP tools, the pages, the answer and item pages, and
  cards. Next is the owner's morning at `acq web` in his own store,
  under the live-run skill. Step 6, the dense table, follows that
  morning, its spike first (the design's §7, step 6).
- **The governance rebuild.** The documents are in: the process
  rulings (`decisions/process.md`), this file cut to state, the brief
  template and the review prompts. The registry's tools follow from
  `briefs/governance-tools-1.md` and `briefs/governance-tools-2.md`,
  then a cold agent sits at the instructions as the acceptance test.

## Next

In order. A session takes the first item it can, without a planning
session; an item leaves when its step closes or the owner strikes it.

1. What holds the invariants: #216 (a second HTTP path), #217 (a token
   on disk), #218 (the mutation run reaches invariants 3 to 5).
2. 4-full part 1, step 6: the dense table, after the owner's morning.
3. 4-full parts 2 and 3, each its own design session
   (`briefs/4-full-design-prompt.md`): the subscriber and the wire,
   then the resident and morning 2.
4. The test-shape group's t6, one canonical mock world and the door's
   corpus (#142), designed from the GUI's door; t7 (#143) only if t6
   shows the shape pays.
5. The queued search and trade issues: #38, #46, #47, and #37 once the
   owner has sat the site.
6. `alpha.2` (milestone "League 3.30 and alpha.2"): regenerate the
   tables when their inputs exist (a post-launch `poe1` pin, the
   owner's four site captures), then the release. The prune verb (#43)
   rides with it.
7. PoE2 (milestone "PoE2 launch"): the route flip and the
   first-contact checklist prepared unmerged; a first contact only if
   GGG ships the stash endpoint.

Issues labelled `trigger:step` are taken by the next step in their
area: `gh issue list --label trigger:step`.

## Waiting on the owner

His queue is the open issues labelled `trigger:ruling`, one decision
each: `gh issue list --label trigger:ruling`. What only he can do:

- His morning at `acq web` in his own store (the live-run skill); step
  6 waits on it.
- The league run under the rails, at his word: count the undecided
  classes and the untranslated mods.
- At the next live login, deny once: whether GGG's authorize page
  echoes `state` on a denial is a fact for
  `docs/design/network-ground-truth.md`.
- At the next live run, `tools/persist-check.sh`, not run since it was
  last edited.
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
