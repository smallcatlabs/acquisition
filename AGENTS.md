# Acquisition — agent entry point

You are at the root of `smallcatlabs/acquisition`, branch `main`: a
Cargo workspace implementing Acquisition (`README.md`, the charter and
crate index). The C++ app is its own repository, `acquisition-qt`
(GPL-3.0); its build and Qt guidance does not apply here. History
before the root swap is under `spikes/rust-playground/` (tag
`playground-end`).

## Read before changing anything

Always, in this order:

1. `README.md` — what exists, how to run it, knobs, known gaps.
2. `CONTEXT.md` — invariants, cross-cutting decisions and parks, area
   decision index, working style.
   Owner (Tom) holds the boundaries (invariants, protocol, core API
   surface); agents own internals.

Then, only when the work touches it:

- Before running `acq` or driving `acq-mcp`: `CLI-REFERENCE.md` and
  `MCP-REFERENCE.md` (generated semantics, checked against the binaries),
  or `acq <verb> --help`. README's tour shows only the shape.
- Before anything that talks to the real GGG API: `LIVE-TESTING.md`
  (standing rule, rails), the tail of `RUN-LEDGER.md`, and the live-run
  procedure below. **The real provider is the default (C88): never start
  a real-mode daemon from an agent shell — the CLI refuses without a
  terminal, and the mock-session skill sets `ACQ_PROVIDER=mock`.**
  Live runs are human-run, from a terminal.
- Before touching tests or the harness: `TESTING-NOTES.md` — the send
  journal is the contract surface; tests pin boundaries, never mechanisms.
- Before reading, citing or fetching a surface GGG does not sanction
  (trade site, forums, feeds): `SURFACES.md` (C79); tooling fetches need
  the permission recorded in the surface's row.
- Before touching item search: `search/README.md` — one track per
  subdirectory, with the slice's rules in force.
- Before touching an area, read its decisions and parks,
  `decisions/<area>.md`; the areas and their ids are the index in
  `CONTEXT.md`, "Decisions — the registry", and each module doc names
  its decision file and ids at the top.
  `python3 -I tools/bothy.py pack --path <file>` prints them, with the
  invariants and the cross-cutting rulings, as one document.

Also read the applicable closed slice record below; its findings table
— for search, its shapes of fault — is the review checklist. For the
refresh and daemon split, also read the crates' module docs (refresh:
`src/lib.rs`, "As built").

| Before touching | Read |
| --- | --- |
| network layer | `NETWORK-CLEANUP.md` |
| pricing | `PRICING-SLICE.md` |
| store, planner or plan slice | `REFRESH-SLICE.md` |
| protocol, client, daemon lifecycle or world | `DAEMON-SPLIT-SLICE.md` |
| item search | `SEARCH-SLICE.md` |
| platform arms, packaging or release | `PLATFORMS-SLICE.md` |

Facts about GGG live in `docs/design/network-ground-truth.md` (and the
trade appendix beside it), cited by claim number; new claims are authored
here, and `acquisition-qt` takes copies, never the reverse. A slice's history is its commit
range — `git log` — and nothing restates it.

## Procedures (loaded when needed; one home each)

- Live run: `.claude/skills/live-run/SKILL.md`
- Mock session (isolation, scripted login, rehearsals):
  `.claude/skills/mock-session/SKILL.md`
- Session close (route what you learned; run before your last commit):
  `.claude/skills/session-close/SKILL.md`
- Research track (a subagent under a committed brief, then review and
  commit): `.claude/skills/research-track/SKILL.md`
- Seat (a blind subagent at the tool as its user, under a committed
  brief and through one logging door; the fix round and the rerun):
  `.claude/skills/seat/SKILL.md`
- Site sitting (asking the trade site what a computed value counts: the
  searches, the owner's recorded sitting, the captures read):
  `.claude/skills/site-sitting/SKILL.md`

## Quality gate, kept green by every change

```sh
cargo build --workspace
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
git diff --check
tools/docs-check.sh
python3 -I tools/bothy.py check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

Once per clone: `git config core.hooksPath .githooks`, whose commit-msg
hook writes Claude's trailer as `Assisted-by:`; the docs check notes a
clone without it.

Build first, and again before any smoke or live run (C82). If contract
inputs change after building — including a formatting edit — build
again before testing or running. Never run the drivers while the gate
runs (C84). The artifact explanation is in
[session-close, "Why build order matters"](.claude/skills/session-close/SKILL.md#why-build-order-matters).

## Routing: one authoritative home per fact

Every fact has one source; a duplicate description rots. When a document
reaches its budget, route the information to its home. Registry formats
and park lifecycle are in `CONTEXT.md` ("Decisions", "Parked"): use
`decisions/<area>.md`, or `CONTEXT.md` only if every area must know it.

| Kind of fact | Home |
| --- | --- |
| ruling, invariant or boundary property | the registry; next stable `C<n>`, ruling verbatim, *Why:* and pointers |
| parked scope or question without a ruling | the area's "Parked", with its trigger |
| observation without a ruling | the open slice's "Observations still open"; after closure, an area park with a trigger |
| property pinned by a test | test name or comment cites the decision id; the entry's *Pinned:* names the file |
| review finding | the regression test that holds it, named for the fault and never the round; a fault that came back is a rule of the brief or a shape in the record's checklist; the round's story is the fix's commit message |
| build narrative | commit message |
| live run | one `RUN-LEDGER.md` row; journals in gitignored `runs/`, mock rehearsals in `runs/mock/` |
| fact about GGG | numbered ground-truth claim |
| mechanism | code doc comment under "Decisions as recorded" / "As built", headed by the decision id; until the code exists, a rule that decides an answer lives in the area's labelled contract-detail section the registry cites (`decisions/trade.md`, "Contract detail") |
| verb, flag or knob usage | clap help / MCP tool description (regenerate references with `ACQ_UPDATE_FIXTURES=1`), or knob read-site doc; README gets one tour line or knob row, never a comment block |
| procedure | skill file referenced here, when earned under P6 |
| deliberation | numbered `brainstorming-notes/` note while a ruling, tool, skill or crate doc cites its path; otherwise delete, citing history as "note NN at `<commit>`" |
| owner's verdict | verbatim from the conversation, marked as such |

`tools/docs-check.sh` checks budgets, stale identifiers, registry and note
citations, README form and dependency edges (C1); it reports uncited notes
and refuses paths to deleted ones. `tools/bothy.py check` holds the
registry's own shape; the two do not overlap. Rustdoc checks links in code docs.
Headers carry no status. Struck-through items are deleted. Session notes
are history, never a second authority.
