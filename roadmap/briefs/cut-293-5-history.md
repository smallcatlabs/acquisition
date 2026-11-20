# Cut 5 (#293 item 5), the rest: history out of the code and the current documents

The owner approved the cut and its whole scope after the first crate's
measure: help text included, version ledgers cut whole, and the
non-Rust documents included. `acquisition-plan` is done (`70aca067`).
This brief covers the other nine crates, the tools and the current
documents. Comments and documents only: no behaviour, no protocol key,
no answer and no ruling's property changes.

## The rule

**History lives in git and in the records. Everything else says what
is and why.**

The boundary is by place, never by line:

- **Current**, swept: `crates/`, `tools/`, `.github/`, `AGENTS.md`,
  `README.md`, `GLOSSARY.md`, `REVIEW-CHECKLIST.md`, `TESTING-NOTES.md`,
  `SURFACES.md`, `decisions/` and `.claude/skills/`. The generated
  `CLI-REFERENCE.md` and `MCP-REFERENCE.md` follow their sources.
- **Records**, untouched: everything else. That means `HISTORY.md`,
  the ledgers, `roadmap/` (the road and the working papers), `audits/`,
  `search/` (the slice's evidence), `docs/design/` (the ground truth)
  and `spikes/`.

In a current file, a **pointer** stays and a **story** goes:

- **Pointers stay:**
  - a ruling id (C…), an invariant, a ground-truth claim (N…);
  - an issue that is open work;
  - a cited text whose content the claim rests on (`<path>` at `<commit>`);
  - a date that is part of a name (a fixture file's).
- **Stories go:**
  - a date;
  - a step, rung, round or seat;
  - "found by", "the cold review at", "was X until";
  - an issue cited only as where something came from;
  - a version ledger (a wire schema's `History: v1 …, v2 …`): the constant keeps its current number and what that version is.
- **A dated fact about GGG** in a comment cites its N claim instead. If
  it has none, keep it and list it in the report: a new claim is the
  owner's to add.
- **A clause's reason stays.** Where the story carries the reason, the
  reason is kept in words and the story goes.

Examples from `acquisition-plan` (`59e1c24c`, `70aca067`):

- "(an untagged enum would lose that: review finding 2026-09-02)" →
  "(an untagged enum would lose that)"
- "the release seat met 5 MB of them" → "listed whole, they reached
  5 MB"
- "(the answer-facts seat, CLI fault 10: the status counted them and
  offered no route)" → "(without it the status counted them and
  offered no route)"

## How

Find candidates with the sweep, case-insensitive, and judge each hit
in context:

```
step-|-step|\bstep [0-9]|\brung\b|tracer|PRICING-SLICE|\bround [0-9A-Z]\b|fix round|finding [0-9]|review of|cold review|\bseat\b|20[0-9]{2}-[0-9]{2}-[0-9]{2}|#[0-9]{2,3}\b
```

The sweep finds markers, not narrative: also read the paragraph each
hit sits in. Subagents may do the reading crate by crate. You judge
every edit and make the commit.

A `///` on a clap argument or an MCP parameter is help text: regenerate
the references under `ACQ_UPDATE_FIXTURES=1` and read every diff. A
golden that moves for any reason but help text stops the batch.

## Batches, in series

Each batch is one commit, comments and documents only. Each gets the
full gate and `--only doc`, then one cold review (C175: prose cut
across many files), then its fixes. Land it, and wait for its CI to go
green before the next push.

1. `acquisition-store`, `-verbs`, `-client`, `-protocol`,
   `-transport` (about 350 hits)
2. `acquisition-daemon` (about 300)
3. `acquisition-cli`, `-mcp`, with the reference diffs (about 780)
4. `acquisition-search` (about 660)
5. `tools/`, `.github/`, and the current documents (about 280)

## Stop and ask

- A comment or a document line is the only record of a property: keep
  it and ask.
- A golden moves beyond help text.
- A batch's review finds a reason lost with its story.
- A batch's third review still finds faults (C176).

## Done when

- The sweep over the current files returns only pointers. The report
  lists each remaining hit and why it stays.
- Every batch is landed with its CI green.
- The close is `Step: cut-293-5`, with a comment on #293 and a report
  with the size by crate and in comment lines.
