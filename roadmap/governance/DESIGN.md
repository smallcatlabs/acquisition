# The governance reset — the design, for the owner's ruling

Phase 1's design, from `roadmap/briefs/1-governance-reset.md`, written
2026-10-20. Nothing is built or deleted by this document; the owner
rules on the drafts beside it and on the lists below, then the
migration briefs (`roadmap/briefs/1b-registry.md` to
`1e-checks-and-skills.md`) run one after another in the main tree.
This directory is the phase's working detail and is deleted at the
phase close, its files cited by commit.

The drafts, each the file it becomes:

| Draft | Becomes | Bytes |
| --- | --- | --- |
| `AGENTS.draft.md` | `AGENTS.md`, the one auto-loaded file | 12,693 of a 14,000 budget |
| `ENTRY-RULE.md` | `tools/bothy.py`'s docstring (the rule) and `docs-check.sh`'s history scan; the ten samples show the rule applied | — |
| `PARKS.md` | twenty "Not doing" lines at the foot of the area files; the deletion list; 48 GitHub issues | — |
| `REVIEW-CHECKLIST.draft.md` | `REVIEW-CHECKLIST.md` | 17,635 |
| `HISTORY.draft.md` | `HISTORY.md` | 2,662 |
| `TESTING-NOTES.draft.md` | `TESTING-NOTES.md` | 7,271 |
| `live-run.draft.md` | `.claude/skills/live-run/SKILL.md`, with `LIVE-TESTING.md` folded in | — |
| `session-close.draft.md` | `.claude/skills/session-close/SKILL.md` | — |

Sizes are `wc -c` at the time of writing. Two drafts are larger than
the brief guessed: the checklist carries every record's shapes (53 rows
against "about 12 KB") and the testing notes keep the journal table
(7.3 KB against "about 5 KB"); each is read by its section, and the
migration step may trim the "where it came back" column if the owner
wants the checklist nearer 12 KB.

## The shape after the reset

A session loads `AGENTS.md` (orientation, five invariants, six hard
rules, the crate table, fourteen cross-cutting rulings and one new,
three principle lines, where things live, the gate). Before touching
an area it reads `decisions/<area>.md` (rulings under the entry rule,
then "Not doing"), the area's section of `REVIEW-CHECKLIST.md`, and
the module doc. The backlog is GitHub issues. It writes back to code, a
test, a regenerated reference, and at most one entry or one issue. The
checks: `docs-check.sh` (one budget, the registry's form and
citations, stale identifiers, the README's form), `arch-check.sh` (the
dependency edges and the group-reader rule), `trade-check.sh` (the
sitting tools' self-tests), `bothy.py check` (the registry's grammar,
the not-doing lines' triggers, paths, module docs).

## Rulings asked of the owner

1. **`AGENTS.md`** as drafted. Two choices inside it: the area index
   is dropped rather than generated (the crate table already names
   each crate's rulings file, and `bothy list --area` prints the ids),
   and the three cross-cutting parks go: shop publishing becomes the
   refused ruling **C120**, the transcription lint folds into C110's
   last clause, the external-tracker park is resolved by this reset.
2. **The entry rule** (`ENTRY-RULE.md`) and the ten samples. Choices:
   *Details:* and *Evidence:* markers go (the first is what `bothy
   show` prints, the second is history); the 800-byte limit stays and
   a history marker in an entry is refused; a ruling whose build waits
   says `Unbuilt` and names its issue.
3. **The deletion list** (`PARKS.md`, section 2): 23 parks deleted
   outright or folded, one of them (ST-6) parked by the owner's own
   word on 2026-09-26 and proposed for deletion anyway. And the files:

   | Deleted | Its text survives at | Replaced by |
   | --- | --- | --- |
   | `CONTEXT.md` | the commit before step 1c's | `AGENTS.md` |
   | the eight records: `NETWORK-CLEANUP.md`, `REFRESH-SLICE.md`, `PRICING-SLICE.md`, `DAEMON-SPLIT-SLICE.md`, `SEARCH-SLICE.md`, `TRADE-SLICE.md`, `PLATFORMS-SLICE.md`, `RELEASE-SLICE.md` | the commit before step 1d's; each record's row in `HISTORY.md` names the commit | `REVIEW-CHECKLIST.md`, `HISTORY.md` |
   | `LIVE-TESTING.md` | the same | the live-run skill; the ladder's `HISTORY.md` row |
   | `SEARCH-SLICE.md`'s "The plan's rules" (rules 1 to 10, cited by number from 19 code files) | moved, not deleted | the search crate's `lib.rs` doc, verbatim; rule 4's owner clause (a change to what a user types comes to the owner first) as a sentence of C101 |
   | all fifteen `brainstorming-notes/` | the same, cited as "note NN at `<commit>`" | nothing: every ruling they held is in the registry; note 36's launch-day step is the roadmap's phase 5 row; note 37 (the stat value) and note 10's mechanism paragraph are cited from issues 46 and 48 |
   | every "Parked" section | the commit before step 1b's | "Not doing" lines and issues |
   | `tools/docs-check-breakers.sh` | renamed `tools/arch-check-breakers.sh` | — |
   | `roadmap/governance/` | at the phase close | — |

4. **The issue list** (`PARKS.md`, section 3): 48 issues, 13 labels.
   Creating them is an outward act on the public repository through
   `gh` on the owner's account; the worker runs it from a script after
   the owner's go on this list, and the script skips a title that
   exists so it can be rerun. Bundles follow the roadmap's: the keeper
   (issue 1), the read economy (9) and the refusals (10) for step 2g,
   the words (11) after 2a, the four queued search and trade parks (37,
   38, 46, 47) for 2h.
5. **The two code texts the migration touches**, both user-facing or
   driver-facing and not design: the `reset-tripwire` help string
   ("see LIVE-TESTING.md") says the rule in words instead (and the
   references regenerate), and `tools/preflight.sh`'s dirty-file list
   names `AGENTS.md` and the live-run skill in place of `CONTEXT.md`
   and `LIVE-TESTING.md`. Beside them, about fifty doc-comment lines in
   the crates and tools cite a deleted record, note or `CONTEXT.md` by
   path; step 1d rewrites each to the history form (`<path>` at
   `<commit>`) with one scripted pass, comments only, so no pointer
   dangles. Trimming the "Decisions as recorded" copies stays phase 2's.
6. **The gate's shape**: three check scripts in place of one, each one
   concern, listed in `AGENTS.md`.

## Departures from the brief, each with its reason

- *"The index generated by `bothy.py` instead of checked"*: dropped
  instead. A generated table inside a hand-written auto-loaded file is
  a mechanism; the crate table is already the index by crate.
- *"Note 36 keeps only its launch-day step or moves to an issue"*:
  deleted whole. The launch-day step is phase 5's row in
  `roadmap/README.md`, which already schedules it; a second copy would
  be a second home.
- *"A cheap scan of the skills against test-enforced rules"*: the
  mechanical half only. The stale-identifier scan runs over the skills
  (a skill naming a test, a tool or a knob that no longer exists is the
  rot that recurs); a sentence that contradicts a test's rule is not
  mechanical to find, so no check pretends to (P4). The contradiction
  the assessment found (the help-string sentence) was fixed at 0d.
- *"`REVIEW-CHECKLIST.md`, about 12 KB"*: 17.6 KB as drafted, every
  record's shapes kept; the owner may cut.
- *"The entry limit kept with history forbidden, or raised to 1,000"*:
  kept at 800; the samples land between 386 and 727 bytes.
- The dependency-edge check and the group-reader rule move together
  into `arch-check.sh`, since both are structure (P5), not documents.

## The checks after the reset, by script

`tools/docs-check.sh`: (1) one budget, `AGENTS.md` at 14,000; (2) the
registry: entries one bullet each under 800 bytes with no history
marker, at most 15 cross-cutting, every cited `C<n>` real over the
code, the tools, the skills, `README.md`, `REVIEW-CHECKLIST.md`,
`TESTING-NOTES.md`, `SURFACES.md`, `RUN-LEDGER.md`, `search/` and
`roadmap/`, the uncited reported; (2b) every cited commit in this
history, and every `` `<path>` at `<hash>` `` citation resolving at
that commit (the second read trial found nine of twelve naming a path
as it is now, not as it was there); (3) stale identifiers over `AGENTS.md`, `README.md`,
`REVIEW-CHECKLIST.md` and `.claude/skills/*/SKILL.md`; (4) the README's
form; (8) the hooks note. Retired: the budgets on `CONTEXT.md`,
`LIVE-TESTING.md`, the records and the area files; the note check
(the directory is gone; a history citation is a commit, which 2b
holds).

`tools/arch-check.sh`: the dependency edges (today's check 5, verbatim)
and the group-reader rule (check 7); `tools/arch-check-breakers.sh`
stages the cases (today's `docs-check-breakers.sh`, renamed). The gate
workflow runs it where it runs `docs-check.sh` today.

`tools/trade-check.sh`: today's check 9, the six self-tests.

`tools/bothy.py`: the root is `AGENTS.md` (orientation, invariants, the
three principle lines each ending with the ids it folds, the
cross-cutting entries); no index check; a "Not doing" line must state
its trigger (the word "until", or "refused"); paths and module docs as
today; the docstring carries the entry rule and the not-doing rule.
`tools/bothy-breakers.sh` loses its index cases and gains a
not-doing case.

## The migration, four briefs, one after another

| Step | Brief | Touches | Closes when |
| --- | --- | --- | --- |
| 1b | `briefs/1b-registry.md` | every entry rewritten under the rule, C120 added; every "Parked" section replaced by "Not doing"; the labels and 48 issues created; `bothy.py`'s park grammar and `docs-check.sh`'s history scan | `bothy.py check` and `docs-check.sh` green on the new form |
| 1c | `briefs/1c-agents.md` | `AGENTS.md` from the draft; `CONTEXT.md` deleted; `bothy.py`'s root; the budgets; `CONTRIBUTING.md`'s "Issues"; every `CONTEXT.md` pointer redirected | the one budget holds; `pack` prints from `AGENTS.md` |
| 1d | `briefs/1d-records.md` | `REVIEW-CHECKLIST.md`, `HISTORY.md`; the records, `LIVE-TESTING.md` and the notes deleted; the live-run skill and `TESTING-NOTES.md` from their drafts; every pointer redirected; the help string and `preflight.sh` | no path points at a deleted file; the references regenerated |
| 1e | `briefs/1e-checks-and-skills.md` | `arch-check.sh`, `trade-check.sh` and the breakers split out; the skills scan; `session-close` from its draft; the gate list in `AGENTS.md` and the workflow | the gate as `AGENTS.md` lists it, green |

Each brief carries the roadmap's review step. The phase 1 row for
`roadmap/README.md`, once the owner rules: "1 design (closed
2026-10-20; `roadmap/governance/`) · 1b the registry: entries, not
doing, issues · 1c `AGENTS.md` absorbs `CONTEXT.md` · 1d the records
to one checklist and a history, the notes and `LIVE-TESTING.md` out ·
1e the checks split and the skills trimmed".

## The fix round (2026-10-20, the owner: "yes to all")

Codex's seven findings (`roadmap/reviews/1-7470a3b8.md` at `cd51f43b`) and the blind read's four gaps (`1-trial-7470a3b8.md` at the same commit) are applied in the drafts and briefs: C1 pinned to the docs check until 1e; the skills' own files in the stale-identifier haystack; the limiter's truncated-body test cited; 1c's grep scoped to its files; the realm-word line "refused"; the entry rule applied to the draft's own entries; the `tool_refusals.rs` sentence corrected; the five trigger values named in `AGENTS.md`; the route property's files named; the search plan's rules given a home; each brief's gate held to its own tree.

## Raised for review

- The brief named the entry sample's result as "most entries would drop
  to 300–450 B" (the report's estimate); measured, the ten samples land
  at 386 to 727. The rulings with the most mechanism (C36, C53) stay
  near 700 because their mechanism is their ruling; the limit does not
  bite them, and the number was the report's, not the tree's.
- The release record names `tool_refusals.rs` as the helper that sorts
  every MCP refusal; that file is the MCP test that pins the refusal
  shape, and the sorting function is `refusal()` in the MCP's
  `main.rs` (0b's review filed the one-helper rule to 2g). The
  checklist names both. The same record's `one_stat` lives
  in `tools/trade-forms-table.py`, not the crate; the frontends park on
  the MCP's no-daemon answers names a `get_result` and an `auth_status`
  tool, and the MCP has `job_result` and no auth tool at all.
- Four placeholders stay for the deleting step to fill, since they name
  the commit before it: `HISTORY_LT`, `HISTORY_TN` and `GOVERNANCE_AT`
  in the history draft, `TESTING_NOTES_AT` in the testing-notes draft,
  and `<commit>` in two issue bodies.
