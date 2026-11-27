# desktop-spike — A window the owner sits at

## Goal
One question is answered before a desktop GUI is designed: does a
window over the engine the tool has feel like the C++ app, or does it
need a native toolkit? It is answered by a throwaway window the owner
sits at over a copy of his own stash (the owner, 2026-11-11: "build,
observe, and iterate"). This brief fixes what must not move and what
evidence is owed; the mechanism is the builder's. `main` gains a
record and nothing else.

## Inputs
- C191 (`acq web` is never reshaped; a desktop GUI is a separate
  frontend), C16 (web rendering over loopback, the page presentation),
  C46 (semantics live in Rust), C12 (a frontend's two surfaces), C1
  (the crate edges), C98 (one basis per answer), C110 (nothing derives
  from the C++ code), C185 (a measurement runs alone).
- The copy of the owner's store: `runs/desktop-spike/store-2026-11-24`
  in the main tree, gitignored, read under his word of 2026-11-24
  (`audits/whole-review-2026-11/INVESTIGATOR.md`, which gives the
  environment a read takes), under which `acq --account
  'GERWARIC#7694' search --realm pc --limit 0` says 44,090 matches.
- `audits/whole-review-2026-11/trials/gui-in-place.md`: what a scripted
  swap cost on that copy. `../findings/gui.md` beside it, "After the
  skeptic": Back, and the basis a swap must carry.
- The C++ app, `../acquisition-qt` beside the main tree: its layout
  and behaviour, read as a description. `git -C ../acquisition-qt
  rev-parse --short HEAD` names what was read.
- `audits/web-2026-11/spike/README.md`: an earlier spike's bar.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`. A red run is reported
  and the step goes on.

## Decisions
1. **What does not move.**
   - The rig is its own binary. No file of `acq web`
     (`crates/acquisition-cli/src/web/`) changes; it may be read.
   - Rust evaluates everything: the window sorts, filters, groups and
     composes nothing, and every list it shows is an answer the search
     or the store's read API gave (C46). A rule the window needs and
     Rust lacks is written in the rig's Rust and listed for the design.
   - Reads only. Every process the rig starts takes the read
     environment of `INVESTIGATOR.md` over a clone (`cp -cR`) of the
     store copy; nothing opens the copy itself or the owner's store,
     no daemon starts, nothing is sent to GGG.
   - The window loads nothing from off the machine, item art
     included: the owner was told the spike sends nothing to GGG.
   - Nothing is taken from the C++ tree: no code, stylesheet or asset.
     What it does is written as a claim, cited `acquisition-qt@<hash>`.
   - The rig is its own package in the lane. The first rig builds
     `--offline` with no dependency the lockfile lacks, no Node and no
     bundler; one that needs such a dependency or a vendored file
     stops and says which and why first.
2. **Where it lives.** A lane: a `cp -cR` clone of the whole main tree,
   `target/` included, under the builder's scratch (`Step: next-a3`'s
   close describes one). The rig is uncommitted there; the record is
   committed on the lane's `main` and brought home by the launcher.
   The main tree is touched only under gitignored
   `runs/desktop-spike/`, which gets the built rig and one start
   script: the owner starts it from the repository's root by one
   command that needs no agent and no lane.
3. **What the owner must be able to do.** What his hands know from the
   C++ app, on his 44,090 items: a tree whose roots are his stash tabs
   and characters, with their items beneath; a list that refilters as
   he types; an item opened in place; none with a page load. The
   layout follows the C++ app's, and the record says each departure.
4. **Each act is timed where it happens, and its time is split.** The
   window logs every keystroke, click, expansion and scroll of the
   sitting to a file under `runs/desktop-spike/`. An act's time runs
   from the input to the paint that shows the result it asked for, at
   that result's basis; an input overtaken before its answer is
   logged as overtaken, never as answered. The time is split into the
   engine's share (Rust answering), the transport's and the
   browser's, waits and queues included, and the shares sum to the
   whole. A miss in the engine's share is an engine fix under any
   toolkit; only a miss in the browser's argues for a native one.
5. **The rig's commit names the bar before any judged run.** One
   commit adds `audits/desktop-2026-11/spike/README.md`: what the rig
   is, read off the working window; the start command; the C++ app's
   behaviours it follows, as claims; each departure; and the bar,
   covering at least a keystroke to the refiltered list, a click to
   the opened item, a root opened to its rows, and a scroll through
   the largest list, with the C++ app's own waits as the reference.
   Beside it, the rig's whole diff from its base commit as a patch,
   the base named, with the command that proved it applies.
6. **That commit is reviewed cold once, as a build** (the cold-review
   skill), against this brief: that the timing measures what it says
   and no rule of decision 1 is broken.
7. **Then the judged runs, each set its own commit.** Headless first,
   with no build or gate running on the machine (a process's lane is
   told by `lsof -a -p <pid> -d cwd`); a run taken under load is
   exploratory and is repeated alone before it is judged. Then the
   builder reports the start command and what to try, and ends its
   turn. The owner's log and his words go in with the verdict.
8. **A miss the rig can answer is a turn of the loop.** Find its cause
   from the split, change the rig, its own Rust included, commit what
   changed and why with its patch, and judge again. A miss only a
   product crate can answer ends the loop, and the verdict says so.

## Find first
- `git log -1 --grep='^Step: next-a3' --format=%B`: a lane.
- `sed -n 36,60p audits/whole-review-2026-11/INVESTIGATOR.md`: the
  read environment.
- `git grep -n "pub fn\|pub use" -- crates/acquisition-verbs/src/frontend.rs
  crates/acquisition-search/src/{lib,places,corpus,answer,show,tabs}.rs`:
  a frontend's doors, and the tree of places the search answers.
- `ls ../acquisition-qt/docs/user ../acquisition-qt/docs/design`.
- `./target/debug/acq store status`, under the read environment over
  a clone of the copy. If it is refused, stop: a fresh copy is the
  owner's to make.

## Done when
- The rig's commit holds decision 5's record and patch, and its
  review is saved and answered.
- A commit holds the headless runs judged by the bar, each act's
  split beside it.
- After the sitting, a last commit leads with the verdict in plain
  words: a window over this engine feels like the C++ app, or needs a
  native toolkit, or cannot be told until a named engine limit is
  fixed; what in the log and in the owner's words says so; and what
  would change it. It lists what a design needs that the spike found
  (what the engine lacked or was slow at, each rule the rig wrote in
  Rust) and what it did not show, Windows first.
- The record holds `shasum -a 256` of each file of the store copy
  before the first run and after the last, the same.
- `tools/gate.sh --docs` exits 0 on each commit's tree on `main`.

## Not in scope
- Any change to a product crate, `acq web`, a golden or the gate. A
  fix the engine needs is listed, never made.
- The desktop GUI's design, the launcher's after the sitting.
- Writes of any kind: prices, saved searches, the policy, a refresh.
- Windows, Linux, a packaged app, a live feed.

## Rules
`TEMPLATE.md`, but for the full gate, which this step does not run:
nothing of the rig reaches `main`. A browser the builder launches is
headless, on a profile directory under scratch, never the owner's
profile. The builder pushes nothing. The lane is removed at the
launcher's word.

## Report
`TEMPLATE.md`.
