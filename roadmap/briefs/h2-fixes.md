# h2 fixes — the review's five findings and two wording items, one rule each

The fix round of step h2 (`h2-build.md`), over the Codex review saved
verbatim at `roadmap/reviews/h2-45badda4.md` and two items the
coordinating session read off the regenerated goldens. The owner's
word, 2026-10-26: "build the h2 fix round over the five findings and
your two wording items, one rule each, then a Codex pass over the
round". "A step" (`roadmap/README.md`) holds.

## Goal

Every sentence h2 wrote is true for every case it covers, not only the
common one, and says the same thing on every surface it stands. A
printed command runs as printed for a user with two accounts; a count
of entries is not called a count of things; a sentence explaining a
count stands beside the count on the CLI as it does on the MCP; a
remedy fits the case it is given to; the old word is gone from every
surface, found case-insensitively; and a line that repeats an
instruction, or spells out zeros, is cut to its counts.

## Read

- `roadmap/reviews/h2-45badda4.md`, whole.
- `h2-build.md`; C146 (a count says what it counts) in
  `decisions/frontends.md`; C53 (three output levels; JSON names are
  the contract); C121 (an account that cannot be picked is refused in
  the protocol's words on every surface).
- The commit `45badda4` with `git show`; the sites the review names:
  `acquisition-plan/src/pricing.rs:355` (the shared refusal's remedy),
  `acquisition-cli/src/store_cmd.rs:533` (granted skills),
  `acquisition-cli/src/search_cmd.rs:915` (never-fetched coverage),
  `acquisition-cli/src/price_cmd.rs:297` (the residue sentence),
  `acquisition-mcp/src/main.rs:1165` (`price_set`, `price_clear`);
  and for the two wording items `search_cmd.rs` (the scope line,
  around 899) and `price_cmd.rs` (the sides line, around 282).
- The fixture `an_item_granted_skill_stays_in_place_and_is_not_an_item`
  (`granted_skills == 2` for one skill shown twice), and
  `price_json.rs` for the two-account case.
- The goldens and references that carry each changed sentence, found by
  grepping the test trees for the current words before the edit.

## Input

- The review's five concrete inputs, each staged as written: Alice and
  Bob known, `price list --account Alice --in tab/xbox/orphan`; the
  granted-skill fixture; a ring query's never-fetched types under one
  league; a character item carrying `~price 5 chaos`; the two tool
  descriptions.
- The two wording items: every scope line ends "a row's place says
  when its own was"; a sides line with every count zero reads
  "0 priced by hand (0 by hand alone, 0 agree with the game's price,
  0 conflict with it); 0 priced in game".

## Do

1. **The remedy carries the resolved account** (finding 1): the shared
   refusal in `pricing.rs` prints `--account <name>` when the frontend
   resolved one, for both callers; pin with two accounts in
   `price_json.rs`, both `show` and `list`.
2. **A count of entries is called one** (finding 2): "skill entries",
   with one sentence that GGG shows a granted skill once per place it
   appears, on the CLI line, the MCP description and the `Status` doc;
   the existing fixture is the pin, its name updated if it says
   "skills".
3. **The explanation stands beside the count on every surface**
   (finding 3): the CLI's never-fetched line gains the sentence the MCP
   description has (coverage spans every location in the realm across
   leagues, since unfetched contents are unknown), the counts
   unchanged; the `tabs_pointer_names_its_league` golden regenerated
   and read.
4. **A remedy fits its case** (finding 4): the residue sentence names a
   character item apart from a private-tab item, the first to be moved
   into a public tab, on the CLI, the MCP and the field doc; pin with a
   character item carrying a price note.
5. **The old word gone, found case-insensitively** (finding 5):
   `price_set` and `price_clear` say what the instructions say;
   `MCP-REFERENCE.md` regenerated; the grep `rg -i 'intent'` over
   `crates/`, the references and the README returns JSON names and
   their docs alone, and the commit message pastes its hit list.
6. **A line carries its counts, not an instruction** (wording 1): the
   scope line ends after the two ages; the sentence about a row's own
   place goes to the `--expand` view or nowhere.
7. **Zeros collapse** (wording 2): when every count on the sides line
   is zero it reads "nothing priced"; when one side is zero its
   parenthesis is dropped. Pin both forms in the `c53_price_status`
   tests.
8. One commit per rule (seven), each naming this brief and the golden
   lines a reader now sees; the last deletes
   `roadmap/reviews/h2-45badda4.md` and cites it whole at `117c0aa6`.

## Done when

- Seven tests or goldens named for the seven faults, each staging the
  input above, green on the tree.
- The case-insensitive grep's hit list in the commit message.
- The review file deleted and cited.
- The gate green on the round's own tree, every stage's exit status
  shown.

## Not in scope

- The two parts of #11 the build left (a flag listing the residue's
  items; a location on the scope's oldest and newest fetch).
- Any JSON field rename or change to what a count counts.

## Rules

`TEMPLATE.md`, with these differences: alone in the main tree on
`main`, no worktree; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/760d4dea-22c5-4f55-b274-679afa47172f/scratchpad/h2-fixes/`;
every `ACQ_STORE_DIR` under it; commit on `main`, never push, the line
`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` last; long
commands in the background; `cargo build --workspace` before any
daemon and after any test run (C84, the artifact mismatch). No task
list: do not call `update_plan`.

## Report

`TEMPLATE.md`, as text.
