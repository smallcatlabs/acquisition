# next-b3 — The pages' words out of the CLI's binary

## Goal
The pages take their words from crates another frontend can link,
and no longer from the CLI's command modules. Step B3 of
`audits/whole-review-2026-11/NEXT.md`, cut to what a consumer takes
today. It matters now because the desktop GUI
(`roadmap/briefs/desktop-design.md`) is a frontend of its own under
every answer the owner may give, and it cannot link a binary crate.
Nothing a reader meets changes: this step moves code.

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
- `acquisition_store::age`: the age vocabulary the store already
  holds, `ago` among it.
- CI on `main`: `gh run list --branch main --workflow "Gate (Linux)"
  --limit 1`, the same for `Gate (Windows)`.

## Decisions
1. **What moves is what the pages take, with what that calls.** Each
   item `crates/acquisition-cli/src/web` names from a command module,
   and each helper it reaches in any command module, however many
   calls down. A renderer the pages do not take stays: the frontend
   that needs it moves it.
2. **A word moves to the lowest crate that holds everything it
   reads.** Over the search's own types, the search crate, beside the
   printing it has. Over a verb's document, the verbs crate. Where
   the lower crate already holds the function (`ago`), the CLI's copy
   goes and its callers use the one there.
3. **A move changes nothing but the address.** No wording, no order
   of arguments, no behaviour. The two changes a move may make: a
   function that read the clock takes `now` from its caller, and an
   item becomes `pub` where it must. The CLI's own callers use the
   moved item from its new home; no re-export stands in a command
   module for the pages' sake.
4. **What stays, by rule, each named in the report with its reason.**
   - A name that is no word: `Ctx`, `connect`, the browser's opener.
   - A word that spells a frontend's flag or a printed command (C143):
     it waits on `NEXT.md`'s B2, with the fault's words and the shop's
     renderer. Its command-free fragments move.
   - A word that reads the store, a client, the terminal or the tty
     from inside itself: it stays until its document carries what it
     reads.
5. **No new crate and no new edge.** C1 and C139 stand as written. A
   move that would need an edge C1 lacks stops for that item, which
   is reported and stays.
6. **Tests move with what they pin** (C122), a unit test beside the
   function it tests. A test retired by a move takes its line (C154).
7. **Commits follow the calls, each building alone**: a helper lands
   before or with its callers, `plural` first. The first commit's
   message states the rule the later ones follow.

## Find first
- `git grep -n -w -E 'search_cmd|plan_cmd|price_cmd|store_cmd|jobs_cmd|shop_cmd|saved_cmd|daemon_cmd|leagues_cmd|terminal|printed|run' -- crates/acquisition-cli/src/web`:
  every line of the pages that names a command module, grouped
  imports included. Its lines are the step's before; the report
  lists each item taken.
- For each item, `git grep -n -w '<name>' -- crates/acquisition-cli/src`:
  its declaration, whatever its kind; then read its body and what it
  calls, to the leaves.
- `git grep -n -E 'SystemTime::now|Instant::now|store::now|[^_a-z]now\(\)|is_terminal|Terminal::' -- crates/acquisition-cli/src ':!crates/acquisition-cli/src/web'`:
  the clock and the terminal inside the words.
- `git grep -n -E -- '--[a-z]|acq [a-z]' -- crates/acquisition-cli/src/search_cmd.rs crates/acquisition-cli/src/store_cmd.rs crates/acquisition-cli/src/saved_cmd.rs crates/acquisition-cli/src/price_cmd.rs crates/acquisition-cli/src/terminal.rs`:
  the frontend's spelling inside the words (decision 4).
- `sed -n 470,560p tools/arch-check.sh`: the edges and the door grep
  a moved function must still pass.
- `cargo tree -p acquisition-search --depth 1` and the same for
  `acquisition-verbs`: what each home may name.

## Done when
- The first "Find first" command, run on the last commit, prints
  only lines that name what decision 4 keeps. The close gives each
  kept name with its reason, and the count of lines before and
  after.
- Each moved item is defined in a library crate and in no command
  module: for each, `git grep -n -w '<name>' -- crates/acquisition-cli/src`
  finds uses and no declaration.
- `git diff --quiet <base> -- '*/goldens/*' CLI-REFERENCE.md MCP-REFERENCE.md`
  exits 0 on the last commit, `<base>` being the step's first parent.
- `git diff --quiet <base> -- tools/arch-check.sh` exits 0.
- `tools/gate.sh` exits 0 on the step's tree.

## Not in scope
- Any change of wording, and any offer or way on an answer (B2).
- The renderers of `daemon`, `leagues` and `saved` that the pages do
  not take, and the plan and price modules but for a helper the
  pages reach.
- The web as a crate of its own; the desktop frontend.
- A golden regenerated: a golden that changes is a fault of the
  move, found and undone, never accepted.

## Rules
`TEMPLATE.md`. One more: before any build or gate, `pgrep -f
desktop-rig`; while it prints a process the owner is sitting at the
prototype and nothing builds (C185, no build while the owner sits).

## Report
`TEMPLATE.md`.
