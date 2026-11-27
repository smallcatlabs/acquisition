# poe2-prep — The PoE2 stash route, prepared on a branch

## Goal
On the day GGG documents a stash endpoint for Path of Exile 2, the
owner merges one branch and runs one first contact. Until then `main`
refuses a PoE2 stash request as it does today, since an unobserved
URL shape is never sent (C59). The road's item 6 and
`roadmap/STATE-2026-10-19.md` (its checklist row "Prepare unmerged")
ask for this before the launch, about 2026-12-10.

## Inputs
- C59 (which realms a route accepts is declared once per route; an
  unobserved shape is never sent), C58 (realm is a coordinate above
  league), C189 (only a live run takes the rails and a ledger row).
  Invariants 1 and 3 of `AGENTS.md`.
- `crates/acquisition-protocol/src/realm.rs`: `Family::accepts`, the
  one place that knows, and the module doc that says what a new realm
  for a family costs.
- The live-run skill (`.claude/skills/live-run/SKILL.md`): "The
  standing rule: first contact" and "A first contact on a new
  endpoint". `RUN-LEDGER.md`: a row's columns.
- `docs/design/network-ground-truth.md`, N41: what the first contact
  on the PoE2 character endpoints recorded, the form the new claims
  take.
- Issue 45: three facts a first contact is to record.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **A branch, `prep/poe2-stash`, never merged by a session.** The
   merge changes what is sent to GGG, so it is the owner's, after GGG
   documents the endpoint. `main`'s tip does not move for the code.
2. **Built in a lane.** A `cp -cR` clone of the whole main tree under
   scratch (`Step: next-a3`'s close describes one), where the branch
   is cut, built, gated and pushed from; the lane is removed at the
   report. The main tree stays on `main`, untouched.
3. **One arm flips: stashes take `poe2`.** The account-leagues family
   stays refused; the checklist says to read GGG's page for it the
   same day and what one more arm costs.
4. **The tests that pin the refusal are found by breaking it.** Flip
   the arm, run the suite, and each test that fails is one the branch
   edits to pin the acceptance, or keeps where it pins the leagues
   family. A refusal no test held is reported.
5. **What stays true is said where it is said now.** The module doc,
   C59's entry and any help or refusal text that says stashes are
   PoE1 only are amended on the branch to what holds after the flip.
6. **The checklist is one file on the branch,
   `roadmap/poe2-first-contact.md`**, naming only what differs from
   the skill: the page of GGG's to read first and what in it would
   change the arm; the rebase of the branch onto `main` and its gate;
   the first contact's command under the rails with its ceiling; what
   to read in the journal; what a 403, a 404 or a body the store
   refuses means and what is done then (never a retry); the claims to
   write, issue 45's three among them; the ledger row, as a template
   with the columns filled but for the result; then the merge. What
   the mock answers for a PoE2 stash is a PoE1 body, and the checklist
   says so: the body's shape is unobserved.

## Find first
- `git grep -n -i 'poe1 only\|PoE1-only' -- crates decisions docs .claude README.md CLI-REFERENCE.md MCP-REFERENCE.md`:
  every sentence the flip makes false.
- `git grep -n 'accepts(' -- crates`: every reader of the table.
- With the arm flipped, `cargo nextest run --workspace --all-targets`:
  the tests of decision 4.
- `sed -n 26,70p .claude/skills/live-run/SKILL.md`.

## Done when
- `origin/prep/poe2-stash` exists, cut from `main`, and `git diff
  main...origin/prep/poe2-stash --stat` names the realm table, the
  tests of decision 4, the texts of decision 5 and the checklist, and
  nothing else.
- On the branch, a mock-world test sends a stash request under
  `poe2` and the journal shows its URL; on `main` the same request is
  still refused before a job id exists.
- `tools/gate.sh` exits 0 in the lane on the branch's tree.
- The branch's range is reviewed cold once, and its review saved on
  `main` by the launching session.

## Not in scope
- Merging, and any live send.
- PoE2's currencies (issue 27), its item vocabulary, a clone of the
  PoE2 export, the per-realm merge at `policy set`.
- The account-leagues family under `poe2`.

## Rules
`TEMPLATE.md`. The branch is pushed, `main` is not.

## Report
`TEMPLATE.md`.
