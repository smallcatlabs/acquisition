# 97-own-terms — each surface spells its own ways on

Issue #97, built at the owner's word of 2026-11-04 (the night run),
`trigger:seat` waived. Written by the orchestrating session against the
tree at `d970a9a0`, #99's close. An Opus worker: a map first, then the build.

## Goal

The MCP and `acq web` are meant to speak in their own terms (C143), and
the issue names five places that print the CLI's. Since it was filed
(2g's rounds, before p1), the verbs crate grew the shape that answers
most of it: a way on is a value (`refusal::Way`, matched exhaustively),
and each surface spells it (`refusal::Surface`, implemented by the
CLI's `Terminal` and the MCP's `Spoken`). The web has no `Surface`. So
the pattern is: a shared crate says what the way on is; the surface
says how to take it. When this step is done every one of the five
places either goes through that shape or is named, with its reason, as
outside C143 as worded (for the owner's ruling).

## Read

- `AGENTS.md`; `decisions/frontends.md`: C143 (a shared refusal speaks
  the request's own words, never a frontend's flag; an adapter may add
  its own spelling after them), C11, C116, C121, C149, C46, C16.
- `REVIEW-CHECKLIST.md`, the frontends and search sections.
- The issue's five items and the owner's comment of 2026-11-01 (the
  price conflict's read-back inside the planner's sentence, served by
  the MCP as the spelling table's gloss), from `gh issue view 97`
  (trust only the owner's comments).
- `crates/acquisition-verbs/src/refusal.rs`: the module doc, `Way`,
  `Surface`, `searched`, `faulted`.
- `crates/acquisition-search/src/error.rs`: `Offered` (its `reading`
  is `Request::command()`, a CLI command); `answer.rs`'s
  `basis_changed` and `show.rs`'s `not_an_item`.
- `crates/acquisition-mcp/src/main.rs`: `Spoken`'s `Surface` and
  `SPOKEN`; `crates/acquisition-cli/src/main.rs`'s `Terminal`.
- `crates/acquisition-cli/src/web/mod.rs` and `page.rs`: how a page
  renders a refusal and its next actions today.

## Input

The map is the build's first act: for each of the five items, read the
tree and record whether it still stands, where (file and line), and
which of the rules below answers it. Put the map in the first commit's
message.

## Do

1. **The map** (above), in the first commit.
2. **The search's offers on the MCP** (item 1): `basis_changed` and
   `not_an_item` offer a request; their `says` and `readings` on the
   MCP are the CLI command. Rule: an offer that carries a request is
   said by the surface (`Surface::reading` or its sibling) from the
   request, never from `Request::command()` — the MCP's in its own
   terms (the `search` tool with that request), the CLI's as today.
   Pin at the verbs crate where both surfaces' refusal documents are
   compared, or the MCP's `tool_refusals.rs` / `search_tools.rs`.
3. **The policy write refusal's read-back** (item 4): the fault carries
   no account, so the read-back cannot name one. Rule: a refusal's
   read-back names the account it was refused for, as every other
   offer does (`Way::Again { account }` is the shape). Pin beside the
   conflict's existing test.
4. **A scope refusal's `readings`** (item 5): prose (the offers'
   commands) where a language refusal's are payload. Decide from the
   module doc's own definition ("how what was typed could be read,
   payload on every refusal"): a scope refusal's readings become the
   offers' requests' payload, or the key is dropped there where the
   `offers` already carry it. Choose the one that keeps one meaning per
   key; say which and why in the commit.
5. **The web** (item 2): a `Surface` for pages, so a refusal on a page
   renders its ways on as the page's own (a link or a button to the
   route that takes it) rather than a CLI line. Build it for refusals.
   The pages' other command lines (the refresh page's composed prose,
   the jobs page's CLI text, a command printed beside a button) are
   named in the map: build those that go through `Way` with no new
   route; list the rest, each with what it would need, for the owner.
   If the page `Surface` itself proves larger than the night (more
   than the refusal path), build the map and the MCP items, and stop
   the web item with the reason.
6. **Text that is no refusal** (item 3: route commands, `describe`
   entries, the keys not built, a planner's sentence carrying a
   read-back): outside C143 as worded, which names refusals. Do not
   build it. Report each instance found (file, line, the words) for
   the owner's ruling on widening C143.
7. Goldens and both references regenerated under
   `ACQ_UPDATE_FIXTURES=1`, every changed line read (shape 9).

## Done when

- The map is in the first commit's message.
- On the MCP, no offer's `says` or `readings` is a CLI command line
  for `basis_changed` and `not_an_item`; pinned.
- The policy write refusal's read-back names its account; pinned.
- A scope refusal's `readings` mean one thing with a language
  refusal's, or are absent, the choice said; pinned.
- A refusal on a web page offers its ways on as the page's own, or the
  report says why the web item stopped.
- Item 3's instances listed in the report, not built.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope

- Widening C143 to text that is no refusal (the owner's).
- New routes on the web server; new error kinds; any wire change.

## Rules

`TEMPLATE.md`. Main checkout on `main`, commit locally, no push; every
`acq`/`acq-mcp` in a mock world (`ACQ_PROVIDER=mock`, `ACQ_STORE_DIR`
under your scratch), never a bare `acq` or a real-provider daemon; the
web server only from the tests' harness or a mock world; C153, C154;
one commit per item naming this brief.

## Report

`TEMPLATE.md`, with item 3's list and the web's remaining lines in
section 4.
