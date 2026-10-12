# Brief: the contradiction read

One of the three reads of the rehome step
(`brainstorming-notes/36-the-road-to-the-agent-release.md`, "Step 1",
"Beside it"). The question, from the note: the always-loaded set, the
area files and the closed records read against each other, and the
narrative a closed record still carries as "next".

The repository changed shape on 2026-10-11/12: the Rust workspace moved
from `spikes/rust-playground/` on a branch of the C++ repository to the
root of its own repository, the C++ app became `acquisition-qt`, the
code was declared shipping (owner's ruling, quoted in `README.md`), and
the licence was declared (C110). Documents written before that may
still speak from the old shape.

## Rules that bind you

- Read-only outside `audits/rehome-2026-10/contradictions/`. Never edit,
  create or delete any other file. Never commit, push, fetch, stash,
  checkout or spawn agents. Never run `cargo`, `acq`, `acqd`, `acq-mcp`
  or anything under `tools/` (the quality gate may be running; a second
  cargo corrupts its run). Read-only git is allowed (`git log`, `git
  show`, `git grep`).
- No network.
- A file past about 40 KB is read by section (`grep -n '^##'`, then
  `sed -n` by range), never skipped.
- You propose; the owner decides. Never soften a finding because the
  fix looks large, and never report a difference in wording as a
  contradiction when both passages decide the same thing.

## Inputs

Read all of these, in this order:

1. `AGENTS.md`, `README.md`, `CONTEXT.md` (the always-loaded set).
2. `decisions/*.md` (the seven area files: rulings, parks, contract
   detail).
3. The closed records: `NETWORK-CLEANUP.md`, `REFRESH-SLICE.md`,
   `PRICING-SLICE.md`, `DAEMON-SPLIT-SLICE.md`, `SEARCH-SLICE.md`.
4. `TESTING-NOTES.md`, `LIVE-TESTING.md`, `SURFACES.md`,
   `search/README.md`, `.claude/skills/*/SKILL.md`.
5. `brainstorming-notes/36-the-road-to-the-agent-release.md` (the
   current plan; where an older document disagrees with it on what is
   next, the older document is the stale one).

Not inputs: `RUN-LEDGER.md` and `search/*LEDGER.md` (append-only
history), `audits/relicense-2026-10/` (a dated audit, history),
`CLI-REFERENCE.md` and `MCP-REFERENCE.md` (generated), the other
brainstorming notes, `docs/design/` (GGG facts, their own authority).
Crate module docs are read only to settle which of two documents is
right.

## What a finding is

One row per finding in `findings.tsv`, tab-separated, with this header:

`id  kind  confidence  a  b  a_says  b_says  proposed_fix  verdict`

- `id`: `X1`, `X2`, … in the order found.
- `kind`, one of:
  - `contradiction` — two passages decide the same question differently.
  - `stale-next` — a document describes as next, open or undecided
    something since done, ruled or abandoned.
  - `stale-shape` — a passage speaks from the old shape (a path under
    `spikes/rust-playground/`, "master-side", "the spike", "the
    playground" where it now misdescribes, "whether this ships").
    Report these only where a reader would be misled or sent to a place
    that does not exist; a name kept on purpose (the workflow file, the
    tag `playground-end`) is not a finding.
  - `two-homes` — one fact stated with authority in two places
    (`AGENTS.md`, "Routing": one authoritative home per fact), whether
    or not the two agree today.
  - `dangling` — a pointer to a file, section, heading, skill or
    decision id that does not exist.
- `confidence`: high, medium or low.
- `a`, `b`: `path:line` of each passage (`b` empty for a single-passage
  kind).
- `a_says`, `b_says`: the shortest verbatim quote that carries the
  point, at most 200 characters each.
- `proposed_fix`: one clause — which passage changes and to what, or
  which is deleted in favour of which home.
- `verdict`: leave empty; it is the owner's.

No tabs or newlines inside a cell.

## Outputs

- `findings.tsv` as above.
- `README.md`, written last from the TSV: a headline block of at most
  eight lines (counts per kind, and the three findings you would fix
  first, by id), then one short paragraph per `contradiction` row
  saying which side you believe is right and why, citing `path:line`.
  Guide: under 10 KB.

## Acceptance

Every row's two `path:line` cells open on the quoted text at HEAD, and
no row survives the question "would a session acting on the wrong one
of these do something different?" with the answer "no".

## Report back (at most 300 words)

Counts per kind; the three rows you are least sure of; what you left
out and why; the one cut you would most want reversed.
