# 0b — Per-realm merge at `policy set`

## Goal
Writing a sync policy for one realm no longer erases the other realm's.
This is the README's first known gap and a data-loss gotcha whose
trigger, a second realm in daily use, has a date: PoE2 1.0 on
2026-12-11. The park is in `decisions/plans.md` ("Parked", the per-realm
merge entry).

## Read
- `decisions/plans.md`: C37 (the planner owns the policy schema), C111,
  and the park's own text.
- `README.md`, "Known gaps", first bullet, and the tour's `policy set`
  line.
- `crates/acquisition-plan/src/lib.rs`, the policy document and its
  revision; `crates/acquisition-cli/src/plan_cmd.rs` and
  `crates/acquisition-mcp/src/main.rs`, the `policy set` /
  `set_sync_policy` paths.
- `CLI-REFERENCE.md` and `MCP-REFERENCE.md` headers: how the references
  regenerate.

## Input
None fixed. The rule is the worker's to choose and state; the owner
may veto it at review.

## Do
1. State the merge rule in the help text and the tool description: a
   set carries whole realms, each realm named replaces that realm
   alone, realms not named stay; a realm is removed by naming it empty.
   Whole replacement stays reachable by one explicit flag or argument
   if a test needs it.
2. Build it once in the plan crate's policy write, so the CLI and the
   MCP share it; keep `--if-revision` semantics as they are (C11: a
   conflict is `kind: "conflict"` with the revision).
3. Tests: a pc policy survives a poe2 set; an empty realm removes it;
   the revision still advances once per write.
4. Regenerate both references; remove the README gap; delete the park
   in the same commit (`CONTEXT.md`, "Parked": deleted when built).

## Done when
- A poe2-only set leaves the pc policy in place, on both surfaces, by
  test.
- The README gap is gone, the references regenerated, the park deleted.
- The gate is green.

## Not in scope
- Type-level policy filters and coverage advice (their own parks).
- A sticky default realm (phase 4).

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`; name the merge rule you chose in one sentence.
