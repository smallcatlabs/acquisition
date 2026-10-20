# 0c — An unreadable MCP request gets a reply

## Goal
`acq-mcp` answers every request it receives. Today a request it cannot
read as JSON (a query tree of 123 `not`s or 62 groups, a message past
127 JSON levels) is dropped before any tool sees it, and the server
answers the next call instead; the caller waits. A hang is the worst
failure an agent can meet, and this one is a transport fix.

## Read
- `decisions/frontends.md`, "Parked": the entry beginning "A request
  `acq-mcp` cannot read as JSON gets no reply", for the depths measured
  and where they were measured.
- `crates/acquisition-mcp/src/main.rs`: the server's transport and
  error handling; the `rmcp` crate's stdio transport, to see where the
  parse happens and what it does with a failure.
- `crates/acquisition-mcp/tests/harness/mod.rs` and
  `tool_refusals.rs`: how a test speaks JSON-RPC to the spawned server
  and how refusals are shaped (every refusal is an `isError` result with
  a `kind`).
- C53 (bounded answers) for the reply's size.

## Input
The depths the park names, as the test's staged requests.

## Do
1. Find where the unreadable request is lost (the transport's parse,
   the deserialisation of the tool's arguments, or a depth limit in
   `serde_json`), and make each path answer: a JSON-RPC parse error for
   a message that is not JSON, and an `isError` result with a `kind`
   and the limit named for a message that is JSON but exceeds a depth
   or shape the tool refuses.
2. A test that sends each staged request and asserts a reply arrives
   for it, in order, and that the server still answers the next call.
3. The tool description says the limit, if one stays.

## Done when
- Every staged request receives its own reply; the server survives.
- The gate is green; the park is deleted in the commit that builds it.

## Not in scope
- Raising the depth limits themselves.
- The read economy on the answer side (phase 1's issue migration carries
  that park; a ruling under C53 retires it later).

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`; say where the request was being lost.
