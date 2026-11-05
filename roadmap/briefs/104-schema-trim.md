# 104-schema-trim — the MCP's first turn, measured and trimmed

Issue #104, built at the owner's word of 2026-11-04 (the night run),
`trigger:seat` waived. Written by the orchestrating session against
the tree at the close of #18 (`2bcd2800` and after).

## Goal

An agent pays for `initialize` and the whole of `tools/list` before
its first call. #83 held every description to a bound, but the
schemas around them are the larger part of the first turn: rmcp's
`$schema` header on every tool, a `title` naming each Rust struct, and
every optional argument typed `["string","null"]` (#104: 15.3 KB of
26.4 KB; the league seat printed 36,104 bytes of text for `open`).
When this step is done the first turn is measured before and after
one schema trim, the trim lands (or is declined on the measurement,
with the numbers in the commit), and the whole first turn is held to
a byte bound so it cannot grow back unseen.

## Read

- `AGENTS.md`; `decisions/frontends.md`, C46 (the MCP is a Rust
  adapter) and whatever it says of the first turn (#83's words).
- `roadmap/briefs/p83-surface-trimmed.md`: what #83 held, and why.
- `crates/acquisition-mcp/tests/surface_size.rs`: the per-text bounds
  and `surface()`, which drives `initialize` and `tools/list` through
  the harness.
- `crates/acquisition-mcp/src/main.rs`: `Args<T>`'s `JsonSchema`
  (about line 566), the hand-written `RequestParams` schema (about
  line 646), the `#[tool_handler] impl ServerHandler` (about line
  2185), and `AcqMcp`'s construction of its `tool_router()`.
- rmcp 3.1's `ToolRouter` and `Tool` (in `~/.cargo/registry`): where
  the input schema is generated and whether the router's tools can be
  rewritten once at construction, or `list_tools` overridden.
- `MCP-REFERENCE.md` and `crates/acquisition-mcp/tests/mcp_reference.rs`:
  what the reference prints of a schema.

## Input

The before number is measured by the build, on the tree it starts
from, in the one way the after number is: the bytes of
`serde_json::to_string` of the `initialize` result plus that of the
`tools/list` result, as `surface()` receives them (state the method in
the commit). Record also the `tools/list` share and the bytes of
`$schema`, `title` and `"null"` alternatives it removes.

The last push's CI: `Gate (Linux)` 41960599455 and `Gate (Windows)`
41960599484 on `25e1af3c`, both success.

## Do

1. **Measure before.** As in Input.
2. **One trim, in one place.** Every tool's input schema loses what an
   agent cannot use: the `$schema` header, every `title`, and on an
   argument that is not `required` the `"null"` alternative of its
   type (`["string","null"]` → `"string"`; the same for `anyOf` with a
   `{"type":"null"}` arm). A function of the schema value, applied once
   where the tools are built or listed, so a new tool is trimmed with
   no edit. `description`, `required`, `enum`, `default`, `items`,
   `properties`, `$defs`/`$ref` and every bound stay. Argument
   *reading* does not change: `Args::parse` still accepts an explicit
   `null` for an optional argument, so nothing an agent sent before is
   refused now (pin it).
3. **Measure after**, the same way. If the trim saves under 10% of the
   first turn, decline it: revert Do 2, keep Do 4's bound at the before
   number, and say so in the commit with both numbers.
4. **Hold the whole.** `surface_size.rs` gains a bound on the first
   turn's total bytes (the after number rounded up to the next 1,000,
   named as such), and a check that no tool's schema carries `$schema`
   or a `title`. The constant's doc says how the number was measured.
5. `MCP-REFERENCE.md` regenerated under `ACQ_UPDATE_FIXTURES=1` if it
   prints schemas; read the diff.

## Done when

- The commit message carries the before and after bytes of the first
  turn and of `tools/list`, measured by one stated method.
- No tool in `tools/list` carries `$schema` or `title`; no optional
  argument's type admits `null` in the schema (or the trim is declined
  on the measurement, said with both numbers).
- An explicit `null` for an optional argument is still read as absent,
  pinned at the MCP's existing tests.
- `surface_size.rs` holds the first turn's total bytes to a named bound.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Rewording any description (#83 did, and the bounds hold them).
- `deny_unknown_fields` or any change to what `Args` accepts (#56's,
  the owner's).
- Removing tools or folding parameters.

## Rules

`TEMPLATE.md`, and as #18's brief: the main checkout on `main`, commit
locally, no push; every `acq`/`acq-mcp` in a mock world
(`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR` under your scratch), never a
bare `acq` or a real-provider daemon; C153 and C154; goldens and
references regenerated under `ACQ_UPDATE_FIXTURES=1`, read as a diff.

## Report

`TEMPLATE.md`.
