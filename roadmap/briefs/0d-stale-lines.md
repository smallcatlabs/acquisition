# 0d — The stale and contradicting lines

## Goal
Every sentence the audits found false or contradictory is made true,
before the governance reset rewrites the documents around them. The
checkers verify pointers, not truth; these passed both.

## Read
`roadmap/STATE-2026-10-19.md`, report 3 ("Documentation and
governance"), section 2, rows 6 to 10 and 12; report 1, section 5
("Spot-check"); report 2, section 5 ("Stale doc comment").

## Input
The lines below, each verified by the audit that found it.

## Do
1. `.claude/skills/session-close/SKILL.md`, the sentence "A help string
   is a doc comment the user reads: cite the id there too": it
   contradicts C116 and `no_ids.rs`. Make it say the opposite, citing
   C116.
2. `RELEASE-SLICE.md`: the README is unbudgeted since 2026-10-19
   (`tools/docs-check.sh` line 94); the record's sentence saying the
   check "holds its budget" becomes "holds its form (the tour and the
   knob table)".
3. `AGENTS.md`: "each module doc names its decision file and ids at the
   top" is false for the client, daemon and protocol crates, and
   `acquisition-cli/src/main.rs` has no module doc. Either add the
   one-line pointer to each of the four (preferred: it is one line
   each) or narrow the claim. Mind the 9,000-byte budget.
4. `docs/design/network-ground-truth.md` header: the OBS tag is used
   sixteen times, so "(none yet…)" goes; the pointer to
   `docs/cleanup/findings.md`, a file not in this repository, becomes
   "at `<commit>` in `acquisition-qt`" or is dropped.
5. `tools/docs-check.sh` line 102: two trailing comments fused onto one
   budget line; split them.
6. `crates/acquisition-daemon/src/daemon.rs` line 1 and
   `src/lib.rs`: "single worker" (the dispatcher runs one task per
   scheduling key) and "Unix socket" (a named-pipe arm exists). Make the
   opening sentences true.
7. `crates/acquisition-cli/tests/seat_faults.rs` near line 2114: the
   doc comment is a copy of another test's; write the one this test
   holds.
8. `brainstorming-notes/36-the-road-to-the-agent-release.md` says it
   is cited by `README.md` and ships `1.0.0-beta.1`; one sentence at
   its head says the citation is `CONTEXT.md` and the release was
   `alpha.1`, and that `roadmap/README.md` holds the work after it.

## Done when
- Each line above is true, and `tools/docs-check.sh` and
  `python3 -I tools/bothy.py check` are green with no budget over.
- The gate is green (clippy and rustdoc see the doc comment changes).

## Not in scope
- Rewriting anything else in those files: phase 1 does that by rule.
- Trimming the "Decisions as recorded" copies from module docs (phase 1
  decides their fate).

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`; one line per item saying what it now reads.
