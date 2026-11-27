# poe2-prep — The PoE2 stash route, prepared on a branch

## Goal
On the day GGG documents the stash endpoints for Path of Exile 2, the
owner takes one branch, runs its first contacts from it, and merges
it. Until then `main` refuses a PoE2 stash request as it does today,
since an unobserved URL shape is never sent (C59). The road's item 6
and `roadmap/STATE-2026-10-19.md` (its checklist row "Prepare
unmerged") ask for this before the launch, about 2026-12-10.

## Inputs
- C59 (which realms a route accepts is declared once per route; an
  unobserved shape is never sent), C58 (realm is a coordinate above
  league), C189 (only a live run takes the rails and a ledger row).
  Invariants 1 and 3 of `AGENTS.md`.
- `crates/acquisition-protocol/src/realm.rs`: `Family::accepts`, the
  one place that knows, and the module doc on what a new realm for a
  family costs. `crates/acquisition-daemon/src/daemon/routes.rs`,
  `route_for`: the URL shapes the stash family renders and their
  route labels. `crates/acquisition-daemon/src/mockggg.rs`: what the
  mock answers a stash request in a realm that is not `pc`.
- The live-run skill (`.claude/skills/live-run/SKILL.md`): "The
  standing rule: first contact" and "A first contact on a new
  endpoint". `RUN-LEDGER.md`: a row's columns.
- `docs/design/network-ground-truth.md`, N41: what the first contact
  on the PoE2 character endpoints recorded, the form new claims take.
- Issue 45's three facts, to be recorded at a first contact: what the
  realm segment of each endpoint's path means, and whether `pc` is
  meant when it is left out; whether PoE2 is served on the character
  endpoints only; what `inventoryId` holds.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **A branch, `prep/poe2-stash`, never merged by a session.** The
   merge changes what is sent to GGG, so it is the owner's. `main`'s
   tip does not move for the code.
2. **Built in a lane.** A `cp -cR` clone of the whole main tree under
   scratch (`Step: next-a3`'s close describes one), where the branch
   is cut, built, gated and pushed from; the lane is removed at the
   report. The main tree stays on `main`, untouched.
3. **One arm flips: stashes take `poe2`.** The account-leagues family
   stays refused.
4. **The mock answers `poe2` stashes as it answers `xbox`'s.** No
   fixture is written for a body nobody has observed. The branch's
   test pins what that is: the request admitted, its URL in the
   journal, the mock's answer and what the store holds after.
5. **The tests that pin the refusal are found by breaking it.** Flip
   the arm, run the suite, and each test that fails is one the branch
   edits to pin the acceptance, or keeps where it pins the leagues
   family. A refusal no test held is reported.
6. **A sentence is amended where the flip makes it false.** That is a
   present claim that stashes, or a plan's tabs, cannot be `poe2`. A
   sentence about the trade site, the account-leagues family or a
   dated observation stays.
7. **The checklist is one file on the branch,
   `roadmap/poe2-first-contact.md`**, naming only what differs from
   the skill, in the order the day runs:
   - GGG's page, and the rule for reading it: every URL shape
     `route_for` renders for the family must be documented. Where
     only some are, stop: the arm is too wide, and the file says what
     narrowing it would take.
   - The branch rebased onto `main`, its gate, and the build the
     first contacts run from, which is the branch's.
   - One first contact for each route label the family has under
     `poe2`, the listing first, each with its command under the rails
     and its ceiling as the skill sets it.
   - What to read in the journal after each, and what a 403, a 404 or
     a body the store refuses means and what is done then, never a
     retry.
   - The claims to write, issue 45's three among them.
   - The ledger row as a template: each column a labelled blank for
     what the run observes, the ceilings it was run under beside
     them.
   - Then the merge, and what `main` must show after it.

## Find first
- `git grep -n -i -E 'poe1.only|character work only|poe2' -- crates/*/src decisions docs/design .claude README.md CLI-REFERENCE.md MCP-REFERENCE.md`:
  the sentences decision 6 sorts; the report lists each with its
  verdict.
- `git grep -n 'accepts(' -- crates`: every reader of the table.
- `git grep -n 'Realm::' -- crates/acquisition-daemon/src/mockggg.rs`:
  the mock's own realm cases.
- With the arm flipped, `cargo nextest run --workspace --all-targets`:
  the tests of decision 5.

## Done when
- The branch's tip touches the realm table, the tests of decisions 4
  and 5, the sentences of decision 6 and the checklist, and nothing
  else (`git diff --stat <base> <tip>`, both hashes in the report and
  in the close).
- On the branch a test holds decision 4. On `main` the same request
  is still refused before a job id exists, by a test the report
  names.
- The checklist holds each bullet of decision 7, and a reader with
  the skill and that file alone could run the day.
- `tools/gate.sh` exits 0 in the lane on the branch's tree.

## Not in scope
- Merging, and any live send.
- PoE2's currencies (issue 27), its item vocabulary, a clone of the
  PoE2 export, the per-realm merge at `policy set`.
- The account-leagues family under `poe2`; a PoE2 stash fixture.

## Rules
`TEMPLATE.md`. The branch is pushed, `main` is not. The branch's
range is reviewed cold once by the launching session, which saves the
review on `main`.

## Report
`TEMPLATE.md`.
