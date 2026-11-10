# The instructions sitting (#228): the record

The run of 2026-11-10, as `BRIEF.md` beside this file says it runs.
The prompt is `PROMPT.md`, sent as it stands. The report is
`REPORT.md` (`result.json`'s `result`, whole). The call log is
`calls.csv`.

## The run

- **Tree:** a clone of `main` at `517836f8` (search-179's close) in
  `~/seat-worlds/governance-2026-11/tree`, its `origin` set to GitHub's.
  The clone's status was clean after the run.
- **Runner:** one headless `claude -p --model opus` (Claude Code 2.1.322),
  in the background, under the brief's rails. It ran from 13:06:31 to
  13:10:40 UTC: 42 turns and 248,731 ms, by `result.json`. Its session
  is `d813997e-a0a1-406b-897c-23104344b557`.
- **Calls:** 41 tool calls, counted as `calls.csv` counts them (each
  call id once).
- **CI before the run:** `268425e3` was green on both gates. On
  `5e8ba9d4`, Linux failed (run 42815716666) on
  `generated_cross_checks::…_3`, the case search-179 was fixed for.
  `9afb15c4` and the clone's tree, `517836f8`, were green on both
  gates (Linux 42820370270, Windows 42820370286).

## The checks made before believing the report

- **No memory.** None of the project memory directory's note names
  occurs in the transcript; each was tested with `grep -c`. The only
  `MEMORY.md` text is the prompt's own ground rule and the harness's
  note on the clone's empty memory directory. `AGENTS.md`'s first
  heading occurs in the transcript.
- **Every path read is inside the clone.** All nine `Read` calls are
  under `…/tree/`. The Bash calls ran in the clone on relative or
  in-clone paths, apart from two `wc -c` calls that named
  `~/.claude/CLAUDE.md`. Both were denied (calls 1 and 12). None of
  the 41 calls opened anything under `audits/governance-2026-11/`. The
  report logs its two pointers there (road file lines 30 and 36) and
  says it did not follow them.
- **The denials, three of them** (`permission_denials`):
  - call 1, a `wc -c …; ls …` chain that named `~/.claude/CLAUDE.md`;
  - call 12, a `wc -c` over in-clone paths plus `~/.claude/CLAUDE.md`;
  - call 34, `wc -c tools/bothy.py && python3 -I -c …`, a chain with a
    command outside the rails.

  The runner re-ran calls 1 and 12 within the rails (calls 3 and 13)
  and measured the docstring by reading the file (call 35). No task
  was left unanswered.
- **Byte counts.** 13 of the counts the report's log names were
  recomputed with `wc -c` on the clone, and all 13 match:
  - `AGENTS.md` 19,124 plus `CLAUDE.md` 11 (the report's 19,135);
  - `decisions/process.md` 10,097;
  - `REVIEW-CHECKLIST.md` 27,035;
  - `roadmap/README.md` 4,754;
  - session-close 4,209, cold-review 3,346 and mock-session 2,978;
  - `HISTORY.md` 4,342;
  - `GLOSSARY.md` 7,634;
  - `decisions/daemon.md` 30,327;
  - `pack --area frontends` 23,629;
  - `pack --path crates/acquisition-store/src/world.rs` 15,325.

  The bothy docstring is 6,173 bytes, and the report's "~6 KB" stands.

## What differs from the baseline

The three differences the brief names held:

- this runner had no memory index;
- it ran under the rails;
- it was given the five prompt changes.

One more, met here: the owner's global `~/.claude/CLAUDE.md` was in
the runner's context under `--setting-sources project`. The report
says so and says the file holds nothing about the project. The
baseline, a subagent of the owner's session, had the same file in
context. The harness's system prompt carries Claude Code's default
git rules ("commit only when asked", "branch first"), as the
baseline's did.

## Bytes read, per task

Each cell is always loaded + required by a rule + needed to resolve
an ambiguity, in KB.

| Task | Baseline (73e7e77c) | This sitting (517836f8) |
| --- | --- | --- |
| 1. Fix #200 | 16.7 + 45.6 + 33 | 19.1 + 50.0 + 8.1 |
| 2. The locked-store refusal | 16.7 + 44 + 12 | 19.1 + ~57 + ~14 |
| 3. Start the next step | 16.7 + 59.3 + 8 | 19.1 + 14.9 + 6 |
| 4. A boundary ruling | 16.7 + 37.5 + 2 | 19.1 + ~46 + ~6 |

`AGENTS.md` grew by 2.4 KB, which is the named holders of the
invariants. Task 3 named the next step from 20.9 KB beyond what is
always loaded, against 67.3 KB in the baseline. The done-when's bound
is 25 KB. The commit and push question was answered from one ruling,
C171 ("commits to `main` … pushes … without asking"). Task 2 grew by
13 KB: this time the runner read a second pack, for the store file
that holds the text.

## The baseline's stops

- **Commit and push: three sources, three answers. Gone.** C171
  answers it alone. A new form of the question remains: the harness's
  default against C171 (new stop 10).
- **The next step took the whole road file. Gone.** The road file is
  4,754 bytes. A new form remains: its two "next" lists (new stop 4).
- **No stated path for a small fix. Gone.** C173: "A small fix needs
  no brief, since its issue states the goal and the done-when."
- **A ruling said a stage was retired that the gate still ran.
  Gone.** The road file holds no rulings now, and the runner met no
  stale stage.
- **The gate's stage list in `AGENTS.md` did not match. Gone.**
  `AGENTS.md` lists the thirteen stages in order, and docs-check holds
  the list against the script. The runner raised nothing about it.
- **`pack --path` could not place the workspace manifest. Still
  there.** Call 5 got "no area governs Cargo.toml … Use --area".
- **Whether the retirement rule covers a checklist row. Changed.**
  C182 now covers "a checklist row" and its retirement. What remains
  is whether a crate feature can be the holder: C154 names "a test, a
  golden, a check, a type or a dependency edge".
- **Working nouns with no glossary line. Changed.** Packet, cut, cold
  review, orchestrating session and spike are no longer raised. New
  ones are raised instead: phase, group, sitting as a seat at the
  instructions, and "door" used in `AGENTS.md`'s own crate table
  although the glossary says new text does not use it.
- **No rule for minting an id, and no exact regeneration command.
  Still there, both.** Task 4: "No rule says how ids are assigned."
  Task 1: "`ACQ_UPDATE_FIXTURES=1` is named three times; the command
  is never given."
- **Rules scoped to a closed group. Changed.** No rule is scoped to a
  group any more. The runner could not tell whether three general
  rules applied to it: C185's "a window he names", C177's "where one
  is installed", and C175's "behaviour" for a refusal's wording or a
  feature.

## New stops

1. The step id's format is never defined. The skills write
   `<step id>` and `<step>`.
2. session-close says a close "rewrites `roadmap/README.md`", but
   search-179's close (`517836f8`) did not rewrite it, because the
   road had not moved.
3. No rule says who closes a step's issue.
4. The road file has two "next" lists: "Now"'s queue and "Next".
   C180 says "takes its first item", while the file says "the first
   item it can".
5. Nothing maps a text a user meets (a refusal) to where it lives and
   which golden holds it. It took 14 KB of grepping.
6. `AGENTS.md`'s cross-cutting list is capped at 18 by docs-check, and
   it holds 18. `AGENTS.md` does not say so.
7. Bothy rule 5 says an entry carries "no owner's quote".
   `AGENTS.md` says "the owner's word first". Some entries carry his
   words, and docs-check reads only `owner:`.
8. The crate-feature holder under C154 (above).
9. Whether an issue the owner names is its own step, given that
   `trigger:step` says "taken by the next step in their area".
10. The harness's default ("commit or push only when the user asks",
    "branch first") against C171. C183 ranks the tree over memory,
    not over the harness.
11. The glossary's "step … with one ledger line" and "fold … its
    ledger lines", against C178 ("no file retells it").
12. session-close glosses C153 as "the stopping rule". C153's
    headline is "a close retires as it pins".
13. #200 names "the workspace manifest" for the feature. The gate's
    `features` stage names the pin block in
    `crates/acquisition-cli/Cargo.toml`.
14. Whether a one-line request from the owner, with no issue, owes a
    close.

## Fates

One row per stop. A fix is one rule's change in the commit after this
record, and the close names that commit.

| Stop | Fate |
| --- | --- |
| `pack --path` on a root file | Issue #236. |
| A crate feature as a retirement's holder | Issue #237 (`trigger:ruling`): the words of C154 are the owner's. |
| Minting an id | Fixed: `AGENTS.md`, "Where things live", a ruling's line. |
| The regeneration command | Fixed: `AGENTS.md`, "Where things live", the line for what a user meets. |
| Working nouns: phase, group, sitting | Fixed: `GLOSSARY.md`. |
| "Door" in `AGENTS.md`'s crate table | Fixed: the table says "shared openers". The entries C46 and C122 keep the word, because the glossary governs new text and an entry's words are the owner's. |
| C185, C177 and C175 as the runner read them | No fault. C185's window is one the owner names to the session. C177 says "where one is installed". C175 reviews everything except a change to tests only or to comments and documents only, so a refusal's wording and a feature are reviewed, as the runner concluded. |
| 1. The step id | Fixed: session-close §3. |
| 2. Rewriting the road file | Fixed: session-close §3 says the close rewrites the road file where the step moved what it says. |
| 3. Closing the issue | Fixed: session-close §3. |
| 4. Two "next" lists | Fixed by #228's close, which rewrites the road file with one ordered "Next". |
| 5. Where a text a user meets lives | Fixed: `AGENTS.md`, "Where things live", the same line as the regeneration command. The lock refusal's own literal is issue #239. |
| 6. The cross-cutting cap | Fixed: `AGENTS.md`, "Cross-cutting rulings", its opening sentence. |
| 7. The owner's quotes | `AGENTS.md`'s "the owner's word first" is fixed in "Where things live". The entries and docs-check are issue #238 (`trigger:ruling`). |
| 8. The crate-feature holder | Issue #237. |
| 9. An owner-named issue | Fixed by #228's close: the road file's `trigger:step` line says an issue the owner names is its own step. |
| 10. The harness's default against C171 | Fixed: `AGENTS.md`'s hard rule "Rulings first, findings after" says the tree's grant stands over a harness's default. |
| 11. The glossary's ledger lines | Fixed: `GLOSSARY.md`, step and fold. |
| 12. C153's gloss | Fixed: session-close §2. |
| 13. Which manifest for #200 | No fault in the instructions. Which block carries the feature is a code choice for #200's builder, and the gate's `features` stage judges it. |
| 14. A request with no issue | No fault. C173: "A step is an issue or a brief", so a request that has neither is a commit and owes no close, as the runner guessed. |
