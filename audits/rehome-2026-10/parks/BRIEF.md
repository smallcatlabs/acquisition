# Brief: the parks read

One of the three reads of the rehome step
(`brainstorming-notes/36-the-road-to-the-agent-release.md`, "Step 1",
"Beside it"). The question, from the note: every park with a proposed
fate — stays, fires in a step of the note, deleted as withdrawn — in
one table.

A park is scope deferred with the trigger that reopens it (`CONTEXT.md`,
"Parked", has the lifecycle). The owner ruled on 2026-10-11 that the
Rust build ships (quoted in `README.md`), so every trigger worded "a
shipping decision" now has its event; note 36's sequence table names
the parks each step fires.

## Rules that bind you

- Read-only outside `audits/rehome-2026-10/parks/`. Never edit, create
  or delete any other file. Never commit, push, fetch, stash, checkout
  or spawn agents. Never run `cargo`, `acq`, `acqd`, `acq-mcp` or
  anything under `tools/` (the quality gate may be running; a second
  cargo corrupts its run). Read-only git is allowed (`git log`, `git
  show`, `git grep`).
- No network.
- You propose a fate; the owner decides it. `withdrawn` is proposed
  only with evidence that the thing parked is no longer wanted or no
  longer possible — an owner's quote, a ruling that supersedes it, or
  code that already does it — never because a park is old or its
  trigger looks unlikely.

## Inputs

1. `brainstorming-notes/36-the-road-to-the-agent-release.md`, whole,
   first: the steps and what each fires.
2. `CONTEXT.md`, "Parked", and the "Parked" section of each of the
   seven `decisions/*.md`. `decisions/daemon.md` carries a lot — one
   bullet holding many items separated by semicolons, each with its own
   trigger: every item is its own row. `decisions/pricing.md` says
   three decisions are "parked in place" among its rulings: those are
   rows too.
3. `README.md`, "Known gaps": a gap that names a park is context for
   that park's row; a gap with no park is listed in `gaps.tsv` (below).
4. The slice records' open-observation sections
   (`REFRESH-SLICE.md` and `PRICING-SLICE.md`, "Observations still
   open"; `SEARCH-SLICE.md`, the same; `DAEMON-SPLIT-SLICE.md`, "Where
   the open observations went"): `AGENTS.md`, "Routing", says an
   observation after a slice's closure is an area park with a trigger,
   so an observation still sitting in a closed record with no park is a
   row in `unparked.tsv` (below).
5. The code and `git log`, only to settle whether a trigger has already
   fired or the parked thing is already built.

One park's deletion is already in the working tree and is not a row:
the toolchain pin (`decisions/daemon.md`, fired at `cfd3dc9c`).

## Outputs

`parks.tsv`, tab-separated, one row per park, header:

`id  where  item  trigger  fate  step  evidence  note  verdict`

- `id`: `K1`, `K2`, … in file order (`CONTEXT.md` first, then the area
  files alphabetically).
- `where`: `path:line`.
- `item`: the parked thing, at most 120 characters, in the park's own
  words where they fit.
- `trigger`: verbatim from the park, at most 200 characters.
- `fate`, one of: `stays`; `fires` (a step of note 36 fires it);
  `fired` (its trigger has already happened and the entry is still
  there); `built` (the code already does it); `withdrawn`;
  `retune` (it stays, but its trigger names something that no longer
  exists or can no longer happen as worded — say the new wording in
  `note`).
- `step`: note 36's step number for `fires`, else empty.
- `evidence`: `path:line` or a commit hash that supports the fate;
  required for every fate but `stays`.
- `note`: one clause, only where the fate needs it.
- `verdict`: leave empty; it is the owner's.

`gaps.tsv`: `where  gap  park` — each README known gap, and the id of
the park that carries it or `none`.

`unparked.tsv`: `where  observation  proposed_home` — observations in a
closed record with no park.

No tabs or newlines inside a cell.

`README.md`, written last from the three files: a headline block of at
most eight lines (the count per fate; the parks note 36 says a step
fires that you could not find, and the parks you found firing that note
36 does not name), then one line per row whose fate is not `stays`.
Guide: under 10 KB.

## Acceptance

The count of rows equals the count of parked items you can enumerate
from the inputs (state both numbers and how the lot was split), and
every non-`stays` row's evidence opens on what it claims.

## Report back (at most 300 words)

Counts per fate; where note 36 and the parks disagree; what you left
out and why; the one cut you would most want reversed.
