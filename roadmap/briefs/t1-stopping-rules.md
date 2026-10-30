# t1 — the stopping rule and the retirement rule

Issue #137. The first step of the test-shape group (`roadmap/README.md`,
"Pull-forward", item 2; `roadmap/TEST-SHAPE.md`, §5). The rhythm is "A
step". Documents only. This brief stops at Do 1, three rulings, for the
owner's word.

## Goal

The group deletes tests and checks. Before the first deletion, three
rulings stand where every session reads them: when a finding becomes a
file (the stopping rule), how a test or a check is retired (the
retirement rule, P6 made a procedure), and which of the group's rounds
the outside review reads. Then the three places a later worker meets
them carry their lines: `TESTING-NOTES.md`'s shape section, the close
skill, the review prompt. A later worker reads `TESTING-NOTES.md`, not
the 13 KB note.

## Read

- `AGENTS.md`: C122 (a behaviour pinned once, the shared harness), at
  799 bytes against the entry limit of 800, so it takes no amendment;
  "Principles", P6 ("a check or a rule is retired when what it guarded
  is proven").
- `roadmap/TEST-SHAPE.md` §2 (the patterns: a ledger of incidents,
  wording pinned twice), §3 (the reframe by contract surface), §5 (the
  governance applied).
- `tools/bothy.py`'s docstring, the entry rule; `tools/docs-check.sh:55`,
  the `AGENTS.md` budget, 15,277 bytes, a note and never a fault.
- `TESTING-NOTES.md`, "The suite's shape"; `.claude/skills/session-close/SKILL.md`;
  `roadmap/briefs/REVIEW-PROMPT.md`, its nine shapes.
- `roadmap/README.md`, "The owner's rulings for this road": process
  rulings land there with "No registry entry" (#128's item 1).

## Input

- `AGENTS.md` is 15,268 bytes; its budget 15,277. The owner's word of
  2026-10-29 in this session: "I give you permission to raise the
  AGENTS.md budget as needed for the duration of this work."
- The last push's CI: `Gate (Linux)` run 40998406288 and `Gate
  (Windows)` run 40998406312, both on `9d3930d8`, in progress when
  this brief was written.

## Do

1. **The rulings, for the owner's word.** Each with the
   recommendation.

   (a) The stopping rule, a new cross-cutting id, C122's complement
   (C122 has one byte of room). Draft, 557 bytes:

   > - **C<n> — A finding joins the surface that holds it before it becomes a file, and a close retires as it pins.** A finding becomes a row in an existing table, corpus or transcript, or a case in an existing test, before it becomes a test file of its own. A surface's wording is held by one golden; a sentence literal elsewhere asserts only what no golden holds. A step's close lists what it retired beside what it pinned. *Why:* findings pinned as files of their own grew a ledger of incidents, one behaviour pinned at three layers. *Pinned:* nothing yet.

   Recommended: yes, as drafted. It stays discipline (`TEST-SHAPE.md`
   §5, "Structure over discipline"), held at the close and in the
   review prompt; "Pinned: nothing yet" says so.

   (b) The retirement rule, a new cross-cutting id. Draft, 454 bytes:

   > - **C<n+1> — A test or a check is retired by a commit that names what now holds its property.** The message carries one line per retirement, `retired X; held by Y`, where Y is a test, a golden, a check, a type or a dependency edge that runs in the gate or in CI at that commit. A worker that cannot name Y keeps X and files an issue. *Why:* P6 (retire what is proven) had no procedure and no evidence home, so nothing was retired. *Pinned:* nothing yet.

   Recommended: yes, as drafted. "Runs in the gate or in CI" is the
   one word beyond #137's text: it lets t2 move the doc stage to a CI
   job and name the job as Y, and it refuses a Y that runs nowhere.

   (c) The review rule, for the group only, in the README's rulings
   list with no registry entry (process, scoped to #137 to #143):
   a round that changes only tests or documents gets its retirement
   lines and the gate, no outside review; a round that changes
   anything else gets the review as "A step" says. "Anything else"
   names, so no worker has to judge it: non-test code, anything under
   `tools/`, `tools/gate.sh`'s stage list, a workflow,
   `.config/nextest.toml`, and the shared harness's isolation list
   (`acquisition-client/tests/harness/mod.rs`, which keeps every test
   off the owner's store). Recommended: yes, and revisited at the
   first tranche's close, when four steps have shown whether a
   test-only round needed the review.

   And, recorded beside them: the `AGENTS.md` budget raised to fit
   the two entries, at the owner's word above.

2. The rulings in `AGENTS.md`, "Cross-cutting rulings", after C122;
   the budget in `tools/docs-check.sh` raised to the file's size after
   the edit plus the same headroom it had (9 bytes, rounded up to a
   hundred); the README's rulings list carrying all four in his words.
   `python3 -I tools/bothy.py check` green. One docs commit.
3. `TESTING-NOTES.md`, "The suite's shape": the four contract
   surfaces (`TEST-SHAPE.md` §3) as the frame, a sentence each on its
   oracle and its corpus as the tree holds them today and what the
   group moves toward; the two rules named in words. Within its 10.7 KB
   or close to it: the note's §3 is the source, not the text.
4. The close skill, §2: the close lists each `retired X; held by Y`
   from the step's commits beside what it pinned (the stopping rule). The review
   prompt: a tenth shape, "A pin kept where a lower pin or a
   structural check holds it", and, for a commit carrying `retired X;
   held by Y`, a check that Y exists at that commit and holds X's
   property.

## Done when

- `AGENTS.md` carries the two entries as ruled, the next two ids; the README's rulings
  list carries the four rulings in the owner's words; `tools/docs-check.sh`'s
  budget is the raised figure.
- `TESTING-NOTES.md`, "The suite's shape", names the four contract
  surfaces and the two rules.
- The close skill and the review prompt carry the lines of Do 4.
- `tools/gate.sh --docs` exits 0 on the step's tree.

## Not in scope

Any deletion, any gate stage change (t2), any check's breakers (t3).
The first retirement outside `TEST-SHAPE.md`'s lists waits for the
owner's word (README, "Pull-forward", item 2).

## Rules

`TEMPLATE.md`. What differs: documents only, so `tools/gate.sh --docs`
serves (no test reads these four files: grepped, `TESTING-NOTES.md`
appears in four test files' comments only, `REVIEW-PROMPT.md` and the
close skill in none; `AGENTS.md`'s budget is read by docs-check); no
outside review, a round touching only documents. Settled in the
round: Do 2's budget raise touched `tools/docs-check.sh`, which the
review rule of Do 1 (c) sends to review, so the step was reviewed
(`roadmap/reviews/t1-095d5ed1.md`).

## Report

`TEMPLATE.md`.
