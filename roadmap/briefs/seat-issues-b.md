# seat-issues-b — pricing's four seat issues

Round B of the answer-facts seat's issues (`audits/answer-facts-2026-11/seat/`,
filed 2026-11-04), at the owner's word of 2026-11-04 ("Start round
B"), after round A (`seat-issues-a.md`) closed. Written by the
orchestrating session against the tree at `1830e33a`. No new ruling:
C146 (a count says what it counts and what the reader can do next),
C78 (a single write returns the prior so a hand can undo it), C53 (text
a function of JSON; JSON additive). One commit per item, a test named
for each; reviewed cold after the build.

The seat's calls are in the worlds under
`~/seat-worlds/answer-facts-2026-11/`: `cli-opus-1/calls/NNN.txt` (CLI
run 1), `cli-opus/` (CLI run 2), `mcp-opus-1/calls/NNN.json` and
`NNN.args.json` (MCP run 1), `mcp-opus/` (MCP run 2);
`audits/answer-facts-2026-11/seat/calls-*.csv` index them. Read the
calls each item cites before its rule.

## The items taken, in this order

0. **A pricing golden that reads two clocks** (round A's report:
   failed once in a bare gate, passed on the rerun).
   `cli10_the_skip_count_names_the_list_that_holds_them`
   (`crates/acquisition-cli/tests/seat_faults.rs`, about line 1892)
   records the refetch at its own `acquisition_store::now()`, after the
   seed's; where the two land in different seconds the facts line takes
   its two-time form ("the oldest … the newest") and the golden fails.
   Rule: a test's recorded fetches that the golden reads share one
   fixed time, the seed's. Look for the same shape in the other tests
   that refetch after `price_json::seed` (a `now()` beside a seed whose
   golden prints the facts line) and fix every one by the same rule;
   name them in the commit. No production change.
1. **#169, `shop_render` with `page` drops freshness, left_out and
   policy** (MCP run 1 call 50 against 51; MCP run 2 calls 53, 54). The
   MCP's page branch (`crates/acquisition-mcp/src/main.rs`, about line
   1998) answers `pricing::shop_page(&rendered, n)` alone. Rule: a
   page's answer is the page's own fields with the render's context
   beside them, every key the whole render carries but `pages` (Settled
   in the round: the review of round B, finding 4, found "the render's
   document with `pages` narrowed to the one asked" carried the page's
   text twice), so `freshness` (its `facts`),
   `left_out`, `counts` and `policy` stand as for the whole render; the
   CLI's `--page` (`crates/acquisition-cli/src/shop_cmd.rs`) checked
   against the same rule and fixed if it drops any of them in text or
   JSON. The tool's description says what a page's answer carries.
   Pin: the render's goldens, a page asked on both surfaces.
2. **#171, a tab price's receipt does not say how many items take
   it** (CLI run 1 call 79 against 84, 85). `price set tab/…` answers
   "5 chaos (revision 1), was unset"; only `price show` says "81 items
   — 31 by hand only, 50 conflict". Rule: a write on a container row
   (a tab, a folder, a character) carries in its receipt the counts
   `price show` prints for that target's items after the write, from
   the same function `price show` uses (never a second count), naming
   the items the row covers, those that now take it and those the game
   decides; an item row's receipt is unchanged. Additive JSON beside
   `in_facts` (`SetReceipt`, `crates/acquisition-plan/src/pricing.rs`,
   about line 813); one text line on the CLI; the MCP tool's
   description. A clear of a container row says the same after the
   clear. Pin: a receipt golden for a tab row over a tab holding a
   conflict, on both surfaces.
3. **#170, `price status` does not count invalid price text in tab
   names** (CLI run 1 calls 53, 69, 74, 75; CLI run 2 calls 59, 60;
   MCP run 2 calls 31, 41). A public tab named `~b/o 5000/2 chaos`
   reads invalid (`price show` says so, call 75) and is in no count
   and under no `--game-text` value; the status counts invalid notes
   (`Counts::invalid_notes`, `GameText::InvalidNote`,
   `crates/acquisition-plan/src/listing.rs`, about lines 554 and 763).
   Rule: a tab name that starts like a price and does not read as one
   is counted as notes are: a count of such tabs and of the items in
   them, beside `invalid_notes`, and a `--game-text` value
   (`invalid_tab_name`) whose route lists those items, grouped by
   their tab as `price list` groups; the status prints the count with
   its route on the same line, as the skip count does since round 3
   of the seat; `describe` and the tool's description name the new
   count and value. No key renamed; `invalid_notes` keeps its meaning.
   Pin: `price status` over a tab with an invalid name, the route run
   as printed listing its items, on both surfaces.
4. **#167, a create's undo goes stale after a replace** (MCP run 1
   calls 43, 45, 52; MCP run 2 calls 55, 57; CLI run 1 calls 82, 91;
   CLI run 2 the calls the seat README cites). After set then replace,
   the create's receipt offers `price clear --if-revision 1`, refused
   once the replace moved the row to revision 2; the replace's own
   undo puts the prior price back, not "no price". The store keeps no
   row history (#24), so a receipt cannot know what stood before the
   prior. Rule: a write that leaves a row standing whose undo is not
   itself a clear offers, beside its undo, the clear of the row at its
   new revision ("clears it: no price by hand"); a create's receipt is
   unchanged (its undo is the clear). In the receipt's shape
   (`crates/acquisition-verbs/src/receipt.rs`: `undo` or `no_undo`,
   never both, never neither, kept), the clear is a second offer under
   its own key, spelled per surface as `undo` is. Pin: the replace's
   receipt on both surfaces, its clear run as printed leaving no row.

## Done when

Each item has its commit, its message naming this brief and the issue
or the round A report (never `Closes #N`: the orchestrator closes
them), and its pin; every regenerated line is one of the items' words
or a `# holds` line; both references regenerated where help or a
tool's text changed; `tools/gate.sh` green, run bare at the end, and
`tools/gate.sh --only doc` green where a doc comment changed.

## Not in scope

#48 (the price plan and its receipt's counts beyond #171's); #24 (row
history); rounds C and D; any other wording the reads turn up (report
it).

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/seat-issues-b/`;
`ACQ_UPDATE_FIXTURES=1` once after the holders are moved; no JSON key
renamed or removed; no push; no issue opened, closed or commented.
Each fix's test is seen failing before the fix, said in the report.

## Report

`TEMPLATE.md`'s six sections, then one paragraph per item: the rule as
built, the pin, the failing-before evidence, one line of what a reader
now meets.
