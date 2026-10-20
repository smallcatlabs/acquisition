# 0b — Per-realm merge at `policy set`

## Goal
Writing a sync policy for one realm no longer erases the other realm's.
This is the README's first known gap and a data-loss gotcha whose
trigger, a second realm in daily use, has a date: PoE2 1.0 on
2026-12-11. The park is in `decisions/plans.md` ("Parked", the per-realm
merge entry).

## Read
- `decisions/plans.md`: the park's own text and the type-level filters
  park beside it (the planner owns the policy schema); C37 (a tab id
  covers its children) for what a policy entry means; C111.
- `decisions/frontends.md` C52: an agent never clobbers intent it has
  not read; the MCP's `set_sync_policy` has no blind-replace form. The
  merge changes what a write replaces, never whether it names its
  revision.
- `README.md`, "Known gaps", first bullet, and the tour's `policy set`
  line.
- `crates/acquisition-plan/src/lib.rs`, the policy document and its
  revision; `crates/acquisition-cli/src/plan_cmd.rs` and
  `crates/acquisition-mcp/src/main.rs`, the `policy set` /
  `set_sync_policy` paths.
- `CLI-REFERENCE.md` and `MCP-REFERENCE.md` headers: how the references
  regenerate.

## Input
The ruling below, accepted by the owner before any code (rulings first,
`CONTEXT.md`, "Working style"). Proposed: **a set names whole realms;
each realm the document names replaces that realm's policy alone, and
realms it does not name stay; a realm is removed by naming it with no
leagues; whole-policy replacement stays reachable by one explicit flag
on the CLI and one explicit field on the MCP, and on the MCP every
write still names the revision it replaces (C52).** Recorded as an
amendment to the policy's entry in `decisions/plans.md` in the step's
first commit, with the owner's words.

## Do
1. Record the ruling (above) in the registry; stop and ask if the owner
   has not yet said yes to it.
2. State the rule in the help text and the tool description.
3. Build it once in the plan crate's policy write, so the CLI and the
   MCP share it; keep `--if-revision` semantics as they are (C11: a
   conflict is `kind: "conflict"` with the revision; C52 on the MCP).
4. Tests: a pc policy survives a poe2 set; an empty realm removes it;
   the revision still advances once per write; the MCP still refuses a
   replace that names no revision.
5. Regenerate both references; remove the README gap; delete the park
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
