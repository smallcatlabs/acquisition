# 56-frame-internals — a frame read that survives a cancel, a write that never panics

Issue #56's internal items, built at the owner's word of 2026-11-04
(the night run), `trigger:gui` waived. Written by the orchestrating
session against the tree at `2a250c54`, #102's close. The issue's other items, a
bound on what `List` and `Quote` answer (paging, a capped echo or a
refusal by size) and `deny_unknown_fields` on the protocol's
`Request`, change the wire and are the owner's: not this step (a
proposal goes to him in the night run's report).

## Goal

`read_frame` keeps a partly read line in a local, so a `select!` that
drops it when another branch wins (the daemon's connection loop, an
event racing a request on a subscribed connection; any client caller
that races `Subscription::next`) loses the bytes already consumed from
the buffer, and the next read starts mid-line: a split request is
read as two malformed frames. And `write_line` `expect`s that every
response serializes, so a response that cannot be serialized panics
the connection's task instead of answering. When this step is done the
reader is cancel-safe by construction and a response that cannot be
serialized is answered as an internal error, the connection kept.

## Read

- `AGENTS.md`; C47 (panics only for broken internal invariants;
  malformed input a structured error with a stable kind), C85 (a frame
  has a bound; an oversize or malformed frame is answered with an
  error, not a closed socket).
- `REVIEW-CHECKLIST.md`, the daemon section.
- `crates/acquisition-daemon/src/frame.rs` and
  `crates/acquisition-client/src/frame.rs`: one reader in two copies,
  byte for byte below the module doc's first paragraph, the tests
  running in both; keep them identical.
- `crates/acquisition-daemon/src/daemon/server.rs`: `handle_conn`'s
  `select!` (about line 118) and `write_line` (about line 1185) with
  its five callers.
- `crates/acquisition-client/src/client.rs`: the two `read_frame`
  callers (`Subscription::next`, about line 1133, and the request
  exchange, about line 2046).
- tokio's docs on cancel safety (`AsyncBufReadExt::fill_buf` is
  cancel-safe; a future holding state in locals is not).

## Input

The failing input, for the pin: a reader whose first poll yields half
a line and then pends, cancelled (dropped) and polled again with the
line's second half and its newline available: today the second read
returns the second half alone. Stage it with an in-memory reader that
pends on demand (no sleep, no timer).

## Do

1. **The reader's state outlives a cancel.** A frame reader value that
   owns the buffered reader and the partial line (and whether it is
   discarding an oversize one), whose `next` (or `read_frame` on it)
   resumes from that state when a previous call was dropped. The
   daemon's connection loop and the client's two callers use it.
   Both copies stay byte-identical below the doc's first paragraph.
   Pin in the shared tests (so both copies run it): the Input's
   cancel-and-resume yields the whole line once, and an oversize line
   cancelled midway still reads as one `Oversize` and the next frame
   intact. The module doc says the reader is cancel-safe and why.
2. **A response that cannot be serialized is answered.** `write_line`
   returns the serialization failure as an error its callers handle by
   writing an `internal` refusal in the protocol's existing shape (no
   new kind), built so that it cannot itself fail to serialize, and
   keeping the connection; no `expect`. Pin at the lowest place that
   holds it (a unit test of the writer over a value whose `Serialize`
   fails, reading back the internal refusal line).
3. Nothing about the size of what is written: a response past the
   frame bound is still written whole (the owner's question).

## Done when

- A cancelled read resumes its line: pinned in the shared frame
  tests, run in both crates; the two copies identical below the first
  paragraph (`diff` of the two files from that line on prints nothing).
- `write_line` has no `expect`; an unserializable response is answered
  as `internal` and the connection kept; pinned.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Paging or capping `List`/`Quote`, refusing an answer by size,
  `deny_unknown_fields` on `Request`: the wire is the owner's.
- The frame bound's value.

## Rules

`TEMPLATE.md`. Main checkout on `main`, commit locally, no push; every
`acq`/`acq-mcp` in a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR`
under your scratch), never a bare `acq` or a real-provider daemon;
C153, C154; one commit per item naming this brief; a forced
interleaving, never a sleep (C137).

## Report

`TEMPLATE.md`.
