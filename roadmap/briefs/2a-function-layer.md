# 2a — The shared function layer (a Fable design, then an Opus build)

## Goal
Every verb's semantics exists once, in Rust, and the CLI and the MCP
are adapters over it everywhere, as C46 says and as pricing and search
already are. Today the refresh and apply orchestration, the plan
compile with its no-policy mapping, the quote, the account resolver,
the store opener and the daemon-status shaping each exist twice, in
two vocabularies, and the two apply answers have drifted (the MCP's
lacks `skipped` and `remaining_requests`). The layer this step builds
is also what phase 4's web UI calls, so its shape is the GUI's API.

## Read
- `roadmap/STATE-2026-10-19.md`: report 1, sections 2a and 6 (rows 1,
  2 and 11, with the file and line references); report 7, section 5,
  pre-work item 1; report 2, section 7, row 3 (the test rule that
  follows).
- `AGENTS.md` C1 (the crate edges and who may link what), C12 (two
  surfaces), C46 (shared semantics), C47 (errors, not panics);
  `decisions/frontends.md` C11, C116; `decisions/plans.md` C111 (the
  apply loop); `crates/acquisition-protocol/src/protocol.rs`
  (`ErrorKind`: the one vocabulary that already exists on the wire).
- The code at the cited lines: `crates/acquisition-cli/src/plan_cmd.rs`
  (`try_quote`, the apply loop, the compile), `src/main.rs` (account
  resolution, `daemon_status` shaping, `parse_children_failure`),
  `crates/acquisition-mcp/src/main.rs` (the same five),
  `crates/acquisition-plan/src/pricing.rs` (the shape that works).

## Input
Phase 1 closed, so the entries this step cites are in their new form.

## Do — the design (Fable)
1. **Where it lives.** The flow needs the daemon (quote, submit, apply)
   and the store, so it cannot sit in `acquisition-plan` without a new
   edge. Two options, the first recommended: (a) a new crate the
   frontends link, holding the verb functions over client, store, plan
   and search, which is what the GUI calls too; (b) the flow in the
   plan crate over a trait the client implements. Either is a C1
   amendment; bring it to the owner with the reasoning.
2. **The vocabulary.** One account-error vocabulary, the protocol's
   `ErrorKind` (`not_logged_in`, `ambiguous_account`, …); the MCP's
   `account_needed` / `account_unknown` / `no_accounts` align to it.
   That is a surface change agents may key on: a ruling, recorded.
3. **The signatures.** Each function takes typed inputs and returns the
   typed document both surfaces print (the CLI renders it; the MCP
   serialises it). List them: compile or explain, quote, apply until
   unspent with the store-change summary, resolve account, open facts
   and intent, daemon status. The daemon's child-failure string becomes
   a typed protocol field (the contract revision moves, which is by
   design).
4. Write the build brief for Opus from this, with the done-when below.

## Do — the build (Opus)
5. Build the layer; delete every duplicate it replaces; the CLI and the
   MCP call it; regenerate both references.
6. Tests: the layer's own, over the mock daemon; the CLI's and the
   MCP's reduced to rendering and adapter shape, under 2b's rule.

## Done when
- The design half: `2a-design.md` and `2a-build.md` committed, the
  rulings the design lists approved by the owner, its review read and
  its findings taken or answered.
- `try_quote`, the apply loop, the compile and no-policy mapping,
  `open_intent`, the store opener, the account resolver and the
  `daemon_status` shaping each exist once.
- The CLI's and the MCP's plan and apply answers are the same document;
  the MCP gains `skipped` and `remaining_requests`.
- One account-error vocabulary on both surfaces, pinned by the existing
  refusal tests.
- The gate is green; the references regenerated.

## Not in scope
- The GUI server (phase 4).
- Holding the corpus (phase 4).
- The daemon file split (2d) and the typed `JobKind` (deferred).

## Rules
`TEMPLATE.md`.

## Report
`TEMPLATE.md`; the design session's report is the build brief.
