# 18-store-refused — the refused bodies have a tool

Issue #18, built at the owner's word of 2026-11-04 (the night run),
`trigger:user` waived. Written by the orchestrating session against
the tree at `5e536edc`.

## Goal

`store_status` counts `refused_bodies` and no MCP tool lists them: two
seats (readiness, call 5; league run 1, calls 24 and 30) met the count
and stopped, since `describe ["store"]` says `acq store refused` "has
no tool here: a person runs it". When this step is done the MCP has a
`store_refused` tool answering what the CLI's `acq store refused`
answers, from the one verbs function both call (C46, shared semantics
in Rust): the list, or one body in full by its id. A count a tool
gives, a tool can list.

## Read

- `AGENTS.md`; `decisions/frontends.md` (C46, C53 and C163 an age
  beside every time, C116 no id in user-facing text, C143 a refusal
  speaks the request's terms, C146 a count says what it counts, C149
  one document).
- `REVIEW-CHECKLIST.md`, the frontends section.
- `crates/acquisition-verbs/src/refused.rs`: `RefusedBodies::read`,
  the list both surfaces will share.
- `crates/acquisition-cli/src/store_cmd.rs` `refused` (about line
  734): the CLI's list and its one-body read, which today calls
  `store.refused(id)` itself and bails "no refused body with id …".
- `crates/acquisition-cli/src/main.rs` `StoreCmd::Refused` (about line
  867): `--limit` defaults to 50.
- `crates/acquisition-store/src/lib.rs` `Refused` (line 480),
  `refused_list`, `refused`.
- `crates/acquisition-mcp/src/main.rs`: `store_status` and
  `item_events` (about line 1752) for the tool shape; the `show` tool
  for how a not-found read is refused; `SPOKEN` (line 1152), whose
  `acq store refused` row says "has no tool here"; the store gloss
  (line 743) naming `acq store refused`.
- `crates/acquisition-mcp/tests/surface_size.rs`: the first turn's
  bounds; `mcp_reference.rs` and `MCP-REFERENCE.md`; `tool_refusals.rs`.

## Input

No fixed measurement. The last push's CI: `Gate (Linux)` run
41960599455 and `Gate (Windows)` run 41960599484 on `25e1af3c`, both
success.

## Do

1. **One verbs function for one body.** Beside `RefusedBodies::read`,
   a function returning one refused row, aged at `now`, with its body,
   or a structured not-found error whose kind and words both frontends
   use (follow the kind `show` uses for an id it cannot find; if none
   fits, say so in the report and use the closest stable kind). The
   CLI's `refused` calls it instead of the store; its text and JSON
   unchanged except the refusal's words if they now come from the
   verbs (a golden regenerates under `ACQ_UPDATE_FIXTURES=1`, read as
   a diff).
2. **The tool.** `store_refused` in the MCP, no daemon, taking
   `account` (as the store tools do), `limit` and `id`. Without `id`:
   `RefusedBodies::read`, default limit ten (the MCP's listing default,
   as `item_events`), the document's `returned`, `limit` and `total`
   saying what was left out. With `id`: that one row with its body.
   Its description says both forms and points at `describe ["store"]`;
   it stays within `surface_size.rs`'s bounds (do not raise a bound;
   if one cannot hold, stop and report).
3. **The sibling words** (shape 3): `SPOKEN`'s `acq store refused`
   row becomes "is `store_refused`"; the store gloss names the tool
   where it names `acq store refused`; grep both crates' texts and the
   README for "store refused" and "no tool here" and fix each that now
   says wrong. `MCP-REFERENCE.md` (and `CLI-REFERENCE.md` if any help
   changed) regenerated under `ACQ_UPDATE_FIXTURES=1`.
4. **Pins.** The verbs function at the verbs crate (a row found, an id
   not found, both read through its public maker); the tool's adapter
   at the MCP's lowest existing surface that holds a store tool (a
   case in an existing test file, C153: the tool lists a seeded refused
   body with its `total`, and an unknown id is refused with its kind).
   No new test file unless no existing one holds a store tool.

## Done when

- `store_refused` is a tool in `MCP-REFERENCE.md`, answering the list
  with `returned`, `limit` and `total`, and one body by `id`.
- The CLI's one-body read and the tool's both go through one verbs
  function; the CLI no longer calls `store.refused` itself.
- No text in the MCP says `acq store refused` has no tool.
- The pins in Do 4 exist and name the behaviour they hold.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Paging the list beyond `limit` (#56's paging is the owner's).
- The first turn's size (#104, the next step): this step adds one tool
  within today's bounds.
- Any change to what the store keeps or how it refuses a body.

## Rules

`TEMPLATE.md`. What differs:
- Work in the main checkout on `main`; no worktree, no branch. Commit
  locally, do not push; the orchestrating session gates, pushes and
  reviews.
- Every `acq` or `acq-mcp` you run is in a mock world (the mock-session
  skill): `ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch. Never
  a bare `acq`, never a real-provider daemon: this is the owner's
  machine with his real store and keyring.
- Tests pin behaviour at boundaries; a finding joins an existing
  surface before it becomes a file (C153); a retirement carries
  "retired X; held by Y" (C154).

## Report

`TEMPLATE.md`.
