# seat-issues-c — the store's facts block, and a refusal's ready commands

Round C of the answer-facts seat's issues (`audits/answer-facts-2026-11/seat/`,
filed 2026-11-04), at the owner's word of 2026-11-04 ("Start round
C"), after rounds A and B (`seat-issues-a.md`, `seat-issues-b.md`)
closed. Written by the orchestrating session against the tree at
`4136c19a`. No new ruling: C162 (coverage and age are one derivation
held in the store, applied by every answer to its subject), C53 (text a
function of JSON, JSON additive; one clock per age), C146 (a count
says what it counts and what to do next), C121 kept (the error kinds
for an account that cannot be picked are untouched: #168's other half,
a kind of its own for a name never seen, waits on the owner). One
commit per item, a test named for each; reviewed cold after the build.

The seat's calls are in the worlds under
`~/seat-worlds/answer-facts-2026-11/`: `cli-opus-1/calls/NNN.txt` (CLI
run 1), `cli-opus/` (CLI run 2), `mcp-opus/calls/NNN.json` and
`NNN.args.json` (MCP run 2); `audits/answer-facts-2026-11/seat/calls-*.csv`
index them. Read the calls each item cites before its rule.

## The items taken, in this order

1. **#172, a store's status and a league's tabs carry no facts block
   of their own** (CLI run 2 calls 5, 36, 37; MCP run 2 calls 27, 30;
   both run 2 runners' first change). `store status` prints counts,
   size and the refused bodies and no time; `tabs` gives per-row ages
   and no league-wide oldest or newest. The group gave the facts block
   (places, fetched, the oldest and newest fetch with their realm and
   location, the listings' ages) to `price status`, `search`,
   `shop render` and the policy read, from the store's one derivation
   (`coverage::of`, `crates/acquisition-store/src/coverage.rs`, about
   line 191; the planner's `Facts`, `crates/acquisition-plan/src/listing.rs`,
   about line 649; the CLI's `facts_line`,
   `crates/acquisition-cli/src/price_cmd.rs`, about line 296). Rule:
   - `store status` (CLI) and `store_status` (MCP) carry the store's
     facts block over every realm and league it holds, the league
     list's age (when `acq leagues` last fetched it, or never) and the
     refused bodies' newest age beside their count; one `facts:` line
     in text after the counts, in the words the other surfaces print.
   - `tabs` and the `tabs` tool carry the league's facts block over its
     tabs (the characters apart, as `tabs`' head now says), one `facts:`
     line under the head.
   - Both through `coverage::of` and the shared formatter (no surface
     derives an age or an extreme for itself, C162); additive JSON keys
     named as the other surfaces name them; `describe` and the tools'
     descriptions name them (mind the 600-byte description bound and
     the describe size tests: measure, never guess, and say a raised
     bound in the commit).

   Pin: where the facts line is pinned now (`seat_faults.rs` and the
   goldens), both surfaces; the oldest and newest told apart by a world
   whose two fetches differ (fixed times, never `now()` twice: round
   B's item 0).
2. **#168's text half, a refusal naming several accounts gives no
   ready command in text** (CLI run 1 call 11: `acq leagues --stored
   --expand` answered "2 accounts are known and one is to be picked:
   GERWARIC#7694, seat; pick one with --account or ACQ_ACCOUNT"; the
   JSON's offers carry the commands). Rule: where the CLI's text
   refusal names several accounts to pick from, it prints one ready
   command per account, the command the person ran with `--account
   <name>` added, each running as printed (C156, a printed command is
   for a POSIX shell, or PowerShell 7.3 or newer on Windows; quote as
   the other printed commands do), from the same source the JSON's
   offers come from, never a second construction; capped as the other
   refusal lists are (`acquisition_store::bound::LISTED`), the rest
   counted with the command that lists them (`acq accounts`). The
   kind stays `ambiguous_account` (C121). The MCP's text
   (`crates/acquisition-mcp/src/main.rs`, about line 193) checked
   against the same rule in its own terms (C143: a tool's request, not
   a CLI command). Pin: the refusal's golden on the CLI, a command run
   as printed; the MCP's refusal test.

## Done when

Each item has its commit, its message naming this brief and the issue
(never `Closes #N`: the orchestrator closes them), and its pin; every
regenerated line is one of the items' words or a `# holds` line; both
references regenerated where help or a tool's text changed;
`tools/gate.sh` green, run bare at the end, and `tools/gate.sh --only
doc` green where a doc comment changed.

## Not in scope

#168's kind (C121's, the owner's); #165 (the protocol); round D (#166);
any other wording the reads turn up (report it).

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch
`/private/tmp/claude-502/-Users-tom-Development-GitHub-smallcatlabs-acquisition/seat-issues-c/`;
`ACQ_UPDATE_FIXTURES=1` once after the holders are moved; no JSON key
renamed or removed; no push; no issue opened, closed or commented.
Each fix's test is seen failing before the fix, said in the report.

## Report

`TEMPLATE.md`'s six sections, then one paragraph per item: the rule as
built, the pin, the failing-before evidence, one line of what a reader
now meets.
