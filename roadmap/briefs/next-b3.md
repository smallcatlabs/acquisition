# next-b3 — The shared words out of the CLI's binary

## Goal
The pages take their words from crates another frontend can link,
and no longer from the CLI's command modules. Step B3 of
`audits/whole-review-2026-11/NEXT.md`. It matters now because the
desktop GUI (`roadmap/briefs/desktop-design.md`) is a frontend of
its own under every answer the owner may give, and it cannot link a
binary crate. Nothing a reader meets changes: this step moves code.

## Inputs
- `NEXT.md`, B3, and `audits/whole-review-2026-11/next/reports/words.md`
  §1 and §2: which files make their text purely, and what `web/`
  takes from each.
- C1 (the crate edges, held by `tools/arch-check.sh`), C46 (shared
  semantics in Rust), C53 (text is a function of the JSON), C122 (a
  behaviour is pinned once, at the lowest crate that holds it), C139
  (`acq web` is a verb of the CLI crate), C143 (a shared text speaks
  no frontend's flag), C154 (a retired test names what holds its
  property).
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **A word moves to the lowest crate that holds everything it
   reads.** A fragment over the search's own types goes to the search
   crate, beside the printing it has. A fragment over a verb's
   document goes to the verbs crate. `plural` and `ago` go where
   their callers' lowest crate can reach them, once.
2. **A move changes nothing but the address.** No wording, no order
   of arguments, no behaviour. The two changes a move may make: a
   function that read the clock takes `now` from its caller, and an
   item becomes `pub` where it must. The CLI's own callers use the
   moved item from its new home; no re-export stands in the command
   module for the pages' sake.
3. **What stays, by rule, each named in the report with its reason.**
   A name that is no word (`Ctx`, `connect`, the browser's opener).
   A word that reads the store, a client, the terminal or the tty
   from inside itself: it stays until its document carries what it
   reads, which is another step's. The fault's words
   (`fault_words`, `failure_words_by`) and the shop's renderer, which
   wait on `NEXT.md`'s B2.
4. **No new crate and no new edge.** C1 and C139 stand as written. A
   move that would need an edge C1 lacks stops the step for that
   item, which is reported and stays.
5. **Tests move with what they pin** (C122), a unit test beside the
   function it tests. A test retired by a move takes its line (C154).
6. **One group, one commit**, each building alone: the search's
   fragments first, then `plural` and `ago`, then the pure renderers
   of `daemon`, `leagues` and `saved`. The first commit's message
   states the rule the later ones follow.

## Find first
- `git grep -ohE '(search_cmd|plan_cmd|price_cmd|store_cmd|jobs_cmd|shop_cmd|saved_cmd|daemon_cmd|leagues_cmd|terminal|printed|run)::[A-Za-z_]+' -- crates/acquisition-cli/src/web | sort -u`:
  the names the pages take today; its count is the step's before.
- For each name, its definition and what its body reads:
  `git grep -n -E 'fn <name>\b' -- crates/acquisition-cli/src`.
- `git grep -n -E 'SystemTime::now|Instant::now|is_terminal|Terminal::' -- crates/acquisition-cli/src/search_cmd.rs crates/acquisition-cli/src/store_cmd.rs crates/acquisition-cli/src/saved_cmd.rs crates/acquisition-cli/src/daemon_cmd.rs crates/acquisition-cli/src/leagues_cmd.rs crates/acquisition-cli/src/terminal.rs`:
  the clock and the terminal inside the words.
- `sed -n 470,560p tools/arch-check.sh`: the edges and the door grep
  a moved function must still pass.
- `cargo tree -p acquisition-search --depth 1` and the same for
  `acquisition-verbs`: what each home may name.
- `python3 -I tools/bothy.py pack --path crates/acquisition-cli/src/search_cmd.rs crates/acquisition-search/src/print.rs crates/acquisition-verbs/src/lib.rs`.

## Done when
- The first "Find first" command, run on the last commit, prints
  only names decision 3 keeps; the close lists them with the count
  before and after.
- `git diff --quiet <base> -- '*/goldens/*' CLI-REFERENCE.md MCP-REFERENCE.md`
  exits 0 on the last commit, `<base>` being the step's first parent.
- `git diff --quiet <base> -- tools/arch-check.sh` exits 0.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- Any change of wording, and any offer or way on an answer (B2).
- `store_cmd`'s words that read the store; the plan and price
  modules, from which the pages take nothing.
- The web as a crate of its own; the desktop frontend.
- A golden regenerated: a golden that changes is a fault of the
  move, found and undone, never accepted.

## Rules
`TEMPLATE.md`. One more: before any build or gate, `pgrep -f
desktop-rig`; while it prints a process the owner is sitting at the
prototype and nothing builds (C185, no build while the owner sits).

## Report
`TEMPLATE.md`.
