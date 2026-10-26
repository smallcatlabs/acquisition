# h2 — The words nobody understood

The second of three groups picked from the open-issue inventory of
2026-10-25, after h1 (`h1-build.md`). Not a roadmap phase step; the
rhythm of "A step" (`roadmap/README.md`) holds whole.

## Goal

Issue #11 lists the words four seats (two CLI, two MCP, two models
each) stopped on and never resolved: `residue`, `granted`, `unlifted`,
`withheld`, `intent`, a `StashN` rank with no map to a tab name, a
scope's "newest" fetch age naming no location,
`never_fetched_by_type` listing totals, `sum 0 · N lacking` reading as
a measured sum, and `price list`'s refusal for a container with its
own row. Some have moved since the seats sat (the residue sentence and
the place's fetch age were reworded in later slices). When this step is
done, every word the issue names either says what it counts in the
sentence that carries it, on every surface it stands, or is shown
still standing with its seat call re-read and the reason the tree
already answers it; and one rule, C146, governs the next such word.

## Read

- `AGENTS.md`: C46 (shared semantics in Rust, every frontend an
  adapter), C116 (no id in user-facing text), C122 (wording is a
  golden regenerated under `ACQ_UPDATE_FIXTURES=1`, naming the facts a
  reader must find).
- `decisions/frontends.md`: C146 (this step's ruling), C53 (three
  output levels: text decides, `--expand` audits, JSON is the
  contract: a JSON field's name is a contract and does not change
  here; its description and the text beside it do), C115 (the answer
  leads with what bears on the question), C143 (a shared refusal
  speaks the request's words).
- `decisions/pricing.md`, `decisions/search.md`, `decisions/store.md`
  for the rulings behind each word: a withheld fetch (C29), the
  residue and the sides of a price reading, `lacking` in a sum.
- Issue #11 whole (`gh issue view 11`), and each seat call it cites,
  in `audits/release-2026-10/seat/` (the `REPORT-*.md` and
  `DEBRIEF-*.md` files; a call number is a row of that run's
  `calls.csv`, and the debriefs' "What I never found out" lists are
  the evidence). `7571a570` for the `price list` refusal.
- The printers: `crates/acquisition-cli/src/price_cmd.rs` (the residue
  sentence around line 287, the game reading around 685),
  `store_cmd.rs` (withheld, drift, granted around lines 501 to 517),
  `search_cmd.rs` (`never_fetched_text` at 653, the place's age at 708,
  `summed_text` at 1500), `acquisition-plan/src/shop.rs`
  (`Stash{n}` around 719), `acquisition-mcp/src/main.rs` (every tool
  description that names one of the words), `MCP-REFERENCE.md` and
  `CLI-REFERENCE.md` (regenerated, never edited by hand), and the
  README's knob and tour lines that carry any of them.
- The tests that assert on the words, found by grepping the test trees
  for `residue|unlifted|withheld|granted:|lacking|never fetched` on
  2026-10-26: `acquisition-cli/tests/apply_loop.rs`,
  `acquisition-mcp/tests/search_tools.rs`, and in
  `acquisition-search/tests/`: `tabs.rs`, `properties.rs`, `price.rs`,
  `ranged.rs`, `common/mod.rs`, `sockets.rs`, `class.rs`, `counts.rs`,
  `acceptance.rs`, `next.rs`. Grep again before the edit, and add the
  goldens under `tests/goldens/` the same grep finds.

## Input

- The ruling C146 in the registry at this brief's commit.
- The seat citations in #11, fixed evidence: a word is "still standing"
  when the sentence the seat read is still what the tree prints, or
  when its replacement still names a mechanism rather than a count.
- No measurement.

## Do

1. For each word the issue names, find the seat call, read what the
   seat saw, and read what the tree prints now. Write a one-line verdict
   per word in your report: standing, moved (where, by which commit),
   or answered. Do this before any edit, and commit nothing for it.
2. For each standing word, change the sentence under C146: the count
   says what it counts, where it came from, and what a reader can do
   next, in words a reader with no registry can parse; the same words
   on every surface the count stands (CLI text, the MCP tool's
   description, the JSON field's doc comment where there is one, the
   references regenerated). A JSON field name does not change.
   `StashN` carries the tab's name beside the rank in the shop's text
   and its JSON (the page's own `linkItem` syntax is GGG's and does
   not change). `never_fetched_by_type` is listed by the query's
   category's tab types or says why it lists all. A sum with lacking
   rows says the sum covers the rows that have the value, and how many
   do not. `price list`'s refusal for a container with its own row
   says what to run instead (`7571a570` did this for `show`; check
   `list`).
3. Regenerate the goldens and the references under
   `ACQ_UPDATE_FIXTURES=1`; read every changed line as the diff of what
   a user now reads; a change no edit explains is a fault.
4. Fill *Pinned:* on C146 with the golden tests that hold the words.
5. One commit per surface or per word family, as compiles; each names
   this brief; the last says "Closes #11" if every word is answered,
   else "part of #11" with the standing list.

## Done when

- The report carries a verdict line per word, each citing the seat call
  and the current printer.
- Every standing word's sentence changed on every surface it stands,
  and the grep for the old word across `crates/`, the references and
  the README returns only JSON field names and their doc comments.
- The goldens and references regenerated and read; the commit messages
  name each changed golden and what the reader now sees.
- C146 carries its *Pinned:* line.
- The gate green on the step's own tree, every stage's exit status
  shown.

## Not in scope

- #9 (the read economy) and #10 (one refusal shape): 2g's, a design.
- #12 (a term's count beside the total): its own trigger (a run that
  spends a call on the terms block).
- Any JSON field rename; any change to what a count counts.
- The web UI's pages: they render the verbs' documents and inherit the
  words; check that their goldens regenerate cleanly and say so.

## Rules

`TEMPLATE.md`, with these differences:

- This step runs alone in the main tree on `main`, no worktree; your
  scratch is
  `/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h2/`; every `ACQ_STORE_DIR` is under it.
- Commit on `main`; never push. End each message with the line
  `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`; the hook
  rewrites it as the project's trailer.
- Long commands in the background; `cargo build --workspace` before any
  daemon and after any test run (C84). No task list: do not call
  `update_plan`. A red run, if any, in a scratch worktree under your
  scratch, stripping asserts on words the fix introduces.

## Report

`TEMPLATE.md`, as text.
