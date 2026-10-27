# p0 — The process round: the rhythm made cheaper before the readiness steps

The first step of the readiness group (`roadmap/README.md`, the
phases table, row r), opened by the 2026-10-26 assessment
(`roadmap/STATE-2026-10-26.md`, sections 2A to 2C and R2). It runs
before r1 because every later step then costs less to run and to read.
Half a day of building; the owner's rulings come first.

## Goal

A session runs the gate as one script and reads its table instead of
re-learning the exit-status and build-order lore; a review is saved
once and left where it is; a design decision in the delegated class is
recorded in its commit and promoted to the registry only when it turns
out to be a boundary; the byte budgets note and never fail; and a
fresh reader can look up the thirty working nouns. Each of these
removes agent work and reuses a shape the tree has.

## Read

- `AGENTS.md`: "Hard rules" (Build order, The gate decides), "Quality
  gate", "Where things live", "Principles" (P6, ceremony retired by
  evidence).
- `roadmap/README.md`, "A step", items 3 to 6: what the review
  choreography is today.
- `roadmap/briefs/TEMPLATE.md`, "Rules": the gate and build-order
  bullets this round reduces to one sentence.
- `.claude/skills/session-close/SKILL.md` section 1 and "Why build
  order matters"; `.claude/skills/mock-session/SKILL.md`, "Set up".
- `tools/docs-check.sh`: `budget()` and the `ENTRY_LIMIT` branch, the
  two failures this round turns into notes; `tools/docs-check-breakers.sh`,
  the cases that expect those failures.
- `roadmap/STATE-2026-10-26.md`, sections 2A, 2B, 2C and R2: the
  evidence (five copies of the gate lore; 73% of post-alpha commits
  markdown-only; the review choreography's bookkeeping commits; entries
  edited to the byte; the four meanings of "door").
- The memory notes a session loads (`MEMORY.md`): "Exit status on its
  own line", "Gate the commit on the checks", "grep -qv is no guard",
  "No edits while the gate runs" are the lore the script absorbs.

## Input

The owner's answers to Do 1, recorded in the commit that applies them
(the commit quotes him). Nothing else is fixed: the glossary's entries
come from the documents as they stand.

## Do

1. **The rulings packet**, one message to the owner before any edit,
   four questions with the recommendation under each:
   - (a) Reviews stay under `roadmap/reviews/` for good; a ledger row is
     a commit range and a close date; the save-alone, delete,
     cite-whole and hash-in-ledger steps of "A step" items 3 and 5 go,
     and a phase close deletes nothing. Recommended: yes. What it
     protects today is `tools/docs-check.sh`'s path scan, which already
     excludes `roadmap/reviews/`.
   - (b) Rulings after the fact for the delegated class: an agent-facing
     design decision (a verb, a flag, a tool, a query word, the words
     of an answer) is recorded in the commit that builds it and listed
     in the close; it enters the registry only when the close finds it
     a property another area must know. A boundary, an invariant and
     anything touching the real store or the real API stay rulings
     first, the owner's. Recommended: yes; the owner's reading drops to
     boundaries and findings.
   - (c) `AGENTS.md`'s budget and the 800-byte entry limit print a note
     and never fail the gate; the history-marker refusal and the
     cross-cutting count stay as failures. Recommended: yes; the
     evidence is 28 entries within 50 bytes of the limit and a wording
     the owner had to restore (`ae837d12`).
   - (d) The glossary's home: a "Glossary" section in `AGENTS.md` if
     (c) frees the room, else `GLOSSARY.md` named from "Where things
     live". Recommended: `GLOSSARY.md`, read on demand, named from the
     entry file in one line.
2. **`tools/gate.sh`.** The ten stages of "Quality gate" in order, each
   run bare with its exit status captured into its own variable, a
   per-stage table printed at the end (`stage  rc  seconds`), exit 1 if
   any stage failed, and `cargo build --workspace` as the last stage,
   so the artifact rule (C84, the artifact mismatch) holds by order
   rather than by lore. `--only <stage>` runs one; `--docs` runs the
   five stages that build no Rust (`git diff --check`, docs-check,
   arch-check, trade-check, bothy) for a documents-only change. The
   script never pipes a stage through a filter. CI stays as it is: it
   runs the stages singly for timing.
3. **The lore reduced to one sentence each**, naming the script:
   `AGENTS.md` "Build order" and "The gate decides"; "Quality gate"
   keeps the ten lines as the definition and adds the script's name;
   `session-close` section 1 and "Why build order matters";
   `mock-session` "Set up"; `TEMPLATE.md`'s gate bullet. The memory
   notes are the owner's and are not edited; the close names them for
   him to retire.
4. **Apply (a) to (d) as ruled**: "A step" items 3 to 6; `TEMPLATE.md`
   Rules; `budget()` and the `ENTRY_LIMIT` branch in `docs-check.sh`
   (and the breaker cases that expected a failure now expect the
   note); the glossary: thirty nouns, one plain-English line each
   (door, world, seat, sitting, rail, rung, slice, step, round, tail,
   last, fold, route, home, basis, scope, place, journal, ledger,
   oracle, pin, golden, breaker, brief, close, park, knob, realm,
   facet, mirror), the four meanings of "door" each given its own
   word, and `docs-check.sh`'s stale-identifier scan reading the
   glossary's file.
5. One commit per concern; the rulings commit first, quoting the owner.

## Done when

- `tools/gate.sh` exists, runs green on the round's tree, and its
  table is the commit message's gate line.
- `grep -rn 'filtered grep\|build again after\|build after any test'`
  over `AGENTS.md`, the skills and `TEMPLATE.md` finds one sentence
  per file at most, each naming the script.
- `AGENTS.md` is shorter than 15,235 bytes, measured after the last
  edit and stated in the message.
- "A step" holds whatever (a) and (b) ruled, in the owner's words.
- `tools/docs-check.sh` prints `note` for a budget or an entry over
  its limit and exits 0 for that alone; `tools/docs-check-breakers.sh`
  exits 0.
- The glossary exists with at least thirty entries, and the
  stale-identifier scan reads it.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- Any change under `crates/`: this round touches scripts and documents.
- `briefs/REVIEW-PROMPT.md`'s content and the Codex launch; "Open"
  holds the owner's items on them.
- Retiring any test, check or breaker beyond the two failures named.
- The AGENTS budget's number: a note needs none.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch under your session's scratchpad in a
subdirectory named `p0`; commit on `main`, never push, the
`Co-Authored-By` line for your model last; the gate before each
commit by its own exit statuses until the script exists, then by the
script's table. The rulings packet (Do 1) is a message to the owner,
not a document; wait for his answers before Do 4.

## Report

`TEMPLATE.md`, as text.
