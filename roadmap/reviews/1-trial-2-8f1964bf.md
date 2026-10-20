# The second blind read trial, on the fixed drafts of 8f1964bf

A fresh headless Opus session, 2026-10-20, in the worktree on branch `trial/governance-read` at `d8ebcfc3` (the first run is branch `trial/governance-read-1`): the drafts of 8f1964bf applied, the known artifacts of an un-migrated tree declared in the brief, a pricing task added. The run: 72 turns, 224 s, $2.15, one denied command (a `git ls-files` pipeline). Its report is verbatim below.

# Blind read: report

## 1. Reading order and cost

1. `AGENTS.md`, about 13 KB, read whole (auto-loaded).
2. `TRIAL-BRIEF.md`, about 5 KB, whole.
3. `REVIEW-CHECKLIST.md`, about 14 KB, whole. `HISTORY.md`, about 3 KB, whole.
4. `roadmap/README.md`, about 19 KB, whole.
5. `CLI-REFERENCE.md`, by section: the header, `acq`, and `acq search` (about 8 KB).
6. `decisions/search.md`, about 30 KB, whole.
7. `search/README.md`, about 13 KB, whole; no use for T1.
8. `acquisition-search/src/lib.rs`, the first 120 lines.
9. `git show 453a4429:spikes/rust-playground/search/BUILD-PLAN.md`, headings only.
10. The heads of every ruling in `decisions/*`, read with grep.
11. `decisions/network.md`, about 9 KB, whole.
12. `network-ground-truth.md`, by section: the header, N5 to N14 and N19 (about 15 of roughly 60 KB).
13. `ratelimit.rs`, by section: the module doc, the Retry-After parse, `install_retry_hold`, and the vector tests.
14. `.claude/skills/live-run/SKILL.md`, about 9 KB, whole.
15. `TESTING-NOTES.md`, the journal section only.
16. `decisions/pricing.md`, about 19 KB, whole.
17. `listing.rs`, by section: the module doc around C81, and the counting loop.
18. `tools/docs-check.sh` and `tools/bothy.py`, grep and the docstring only.

## 2. T1: `--fetched-within`

**Read first:** `decisions/search.md`, the `acq search` reference, `acquisition-search/src/lib.rs` and `answer.rs` (where C118 puts the fetch age on each row).

**Rulings that bind it:**
- C91: a query is one tree, and every surface encodes that tree. There are no silent modes.
- C96: place is expressed as terms. Only the realm sits outside the query.
- C101: a missing reach becomes a field, never a general door.
- C118: each row already carries its location's fetch age.
- C46: the CLI and the MCP must share one semantics.
- C53: the answer's text is the decision view and its JSON is the contract.
- C100: one answer shape, and a continuation is tied to its basis.
- C116: no rule ids in help text.

**Needs a ruling:** yes, if it is built as a flag. A flag would filter rows outside the query tree, which C91 and C96 forbid. The alternative is a field, for example `fetched<2h`. It fits C101, but the language has no duration value, so that is new grammar. Either way I would bring the choice to the owner. The ruling would be recorded as C121 (the highest id I found is C120) in `decisions/search.md`, with the owner's words first.

**Tests:**
- `acquisition-search/tests/answer.rs` or `rows.rs`, for the boundary at the age limit.
- `language.rs`, for parsing and canonical printing.
- `acquisition-cli/tests/search_json.rs`, to show every printed route still parses (rule 5).
- `acquisition-mcp/tests/search_tools.rs`, for MCP parity.

**Regenerate:** with `ACQ_UPDATE_FIXTURES=1`, `CLI-REFERENCE.md`, `MCP-REFERENCE.md` and the `--describe` text. Possibly one README tour line.

**Write back:** the story goes in the commit message. A fault that came back goes to the checklist row. Leftovers become an issue labelled `area:search`.

## 2. T2: a 400 s hold after one 429

**Read first:**
- `ratelimit.rs`: `install_retry_hold` holds for Retry-After + 60 s + 1 s; its test tables are the spec (C33).
- The bound itself, [Retry-After, Retry-After + 61 s]: the pacing invariant in `daemon.rs` around line 7221 pins it, and `TESTING-NOTES.md`'s table states it.

**Header meaning:** N5 (the triplet `hits:period:restriction`, and Retry-After on a 429), N11 to N13 (timing buckets) and N19 (Retry-After needs padding) in `docs/design/network-ground-truth.md`. Invariant 2 says headers are the truth. C18 and C26 also apply.

**Hypothesis:** an active restriction in the `-State` header, such as `300` plus the 60 s bucket, explains about 400 s. That would be correct behaviour, not a bug. The invariant in TESTING-NOTES is scripted-only, so it does not cover this case.

**To confirm live:** I could not; it is the owner's job, from a terminal, under the live-run skill: rails on, the journal read first, and one `RUN-LEDGER.md` row. N10 rules out provoking a 429, so the only option is to observe one. A fact learned live becomes a new claim, N48, in the ground truth.

## 2. T3

- **(a)** The fault goes to the existing checklist row "A printed command the build refuses". Per the checklist's own rule, a returning shape strengthens the holding test (`search_json.rs` rule 5, `answer.rs`) and never adds a row. The story goes in the commit message.
- **(b)** "Wait for a user" is a GitHub issue labelled `area:search` and `trigger:user`. If the project declines it outright instead, it is a "Not doing" line in `decisions/search.md`.
- **(c)** A numbered claim in `docs/design/network-ground-truth.md`, with a source tag and a confidence.

Without `AGENTS.md`, I would have guessed the area's "Parked" section, which every decision file still uses.

## 2. T4: residue or conflict

**Meaning:**
- C69 defines the two sides and the relation. The game side reads the note first, then the tab name.
- C81: only a public tab's game side is a statement; otherwise it is residue. C80 decides which tab applies.
- The trade claims T1 and T11.
- The `listing.rs` module doc.

**Does the tab name count?** No. The note `~b/o 5` is read before the tab name. If the tab is not public, both texts are residue, and the relation is manual-only, not conflict. `residue` is a separate count that overlaps `by_relation`, so the user is reading two different counts as one.

**Computation:** `listing.rs` around line 1505. It is pinned by the `c69_`/`c81_`/`c80_` tests, `price_json.rs` and `listing-report-schema-3.json`.

**Checklist rows:** "A join that hides the condition it tests" (public before note), "A rename under an additive schema", "A fall-through past an unreadable row", and "A claim wider than its evidence".

**If the count is right:** answer the user, and record their case on the pricing park whose trigger it fires ("a run takes one count for another", `pricing.md` line 43).

## 3. Could not find

- The issue list and its labels: `gh` was out of bounds for this trial.
- Whether the owner or an agent decides a new CLI flag. AGENTS says the owner holds the "core API surface", but does not say whether that includes the CLI.
- The plan's "rule 5". `lib.rs:9` cites `search/BUILD-PLAN.md` at `453a4429`, but the file is at `spikes/rust-playground/search/BUILD-PLAN.md`.

## 4. Had to guess

- The form of a ruling entry. "The entry rule" is not in the tree.
- How to choose between an issue labelled `trigger:user` and a "Not doing" line for (b).
- C121 as the next id.
- Treating a flag as conflicting with C91 and C96.

## 5. Found wrong or contradicted

- **`AGENTS.md`, the quality gate (`tools/arch-check.sh`, `tools/trade-check.sh`):** neither file exists, and neither ever appears in `git log`. C1's own line in AGENTS says arch-check exists only "once step 1e moves them", which `roadmap/README.md:72` confirms. As written, the gate cannot be run.
- **`REVIEW-CHECKLIST.md`:**
  - Line 54 names `tools/arch-check.sh` and `arch-check-breakers.sh` as what holds the shape; both are absent.
  - Line 71 says the group rule is in `arch-check.sh`. It is in `tools/docs-check.sh:557`, which is also what `acquisition-search/src/lib.rs:83` says.
  - Line 91 names `tools/trade-check.sh`, which is absent; that check is at `docs-check.sh:578`.
- **`REVIEW-CHECKLIST.md:108`** says the brief's writer checks every citation, but `roadmap/README.md:128` names `briefs/1b-registry.md`, which does not exist.

## 6. Never needed

C2, C4, C14, C15, C35, C38, C48, C79, C110 and C120; the workspace table's frontend rows; the "Once per clone" line.

## 7. What you would retract

- The 400 s restriction hypothesis: it rests on N5's triplet plus inference, not on a test I read.
- That `residue` and `by_relation` overlap: I inferred it from one loop.
- "Owner decides flags": my session memory says the owner delegates agent-facing design, which the tree does not say.

## 8. What got in your way

- The gate lists scripts that do not exist, so I could not tell what "green" means today.
- AGENTS describes "Not doing" lines, but no decision file has one yet.
- The rules allowed no `wc` and no `ls`, so the sizes above are estimates.
