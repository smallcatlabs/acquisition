# answer-facts-seat-fixes — the seat's fix round

The fix round of the answer-facts seat (`audits/answer-facts-2026-11/seat/`,
run 1: two Opus runners at the release build of `2bed3ba5`, the MCP
door 62 calls and the CLI door 121 calls, their reports `REPORT-mcp-1.md`
and the CLI's copied at the close). Written by the orchestrating
session on 2026-11-04 against the tree at `ff3ed0ac` (line numbers
from it). The seat skill's step 8: one commit per fault, each held by
a test in `crates/acquisition-cli/tests/seat_faults.rs` (or the
crate that holds the behaviour, C122) named for the fault; a fault
that is a ruling's consequence is kept and made visible. Every fault
below was checked by the orchestrating session against the calls it
cites (the worlds' `calls/NNN.json` and `calls/NNN.txt`); the fates
of the faults not taken are the close's. No new ruling; the words
are the session's agent-facing design.

## The faults taken, each one rule

1. **Two locations for one newest time** (MCP faults 4; calls 26, 31,
   33, 34, 35: `scope.newest_fetch_location` stash `Dump Tab` and
   `total.newest_fetch_location` character `gerwarwitch`, both at
   1,789,147,353). The scope's span breaks a tie by the store's order of
   locations; the total's by the order the matches were met. Rule: a
   span's tie is broken by the store's order of locations on every
   block, so where both blocks hold the tied locations they name the
   same one. The search orders the matches' places by the header's
   location order (the index of each location in `CorpusHeader.locations`,
   kept by the corpus by coordinate) before `coverage::span`
   (`crates/acquisition-search/src/answer.rs`, the fold round 2
   added). Pin: a fixture with a tab and a character fetched in one
   second, a query holding both, in both insertion orders: the two
   blocks name one location. The gloss of the two location fields
   says "the first in the store's order on a tie".
2. **A location label without its realm** (CLI fault 4; calls 50, 63,
   98: `Standard / gerwarshot (character)` for a poe2 character under
   `--realm all`, read as pc). Rule: a location named on a span names
   its realm as a row's place does (`place pc · Standard / …`):
   `FetchedLocation` gains `realm` (additive; the store's row has it)
   and the text's clause prints `pc · Standard / Dump Tab` on the
   scope's and the total's lines and on the facts lines of round 4,
   through the one formatter `search_cmd::span_words` (round 6 shared
   it). Pin: a two-realm fixture, the clause naming each realm; the
   goldens that hold the clause regenerated, each changed line the
   realm joining a label.
3. **`never_fetched_leagues` names tabs' leagues alone** (CLI fault 5;
   call 98 against call 51: `["Standard"]` where never-fetched
   characters sit in four other leagues). The field is, by design,
   the leagues holding a never-fetched tab (`coverage.rs`, by_league
   over stash locations), and its name says leagues. Rule: a key's
   gloss and the text that prints it name the set exactly (C146): the
   `describe` entry says "the leagues holding a never-fetched tab;
   characters never fetched are counted by type under
   `never_fetched_by_type`", and the scope's never-fetched text line,
   where it names leagues, says "tabs". No rename (C146). Pin: the
   gloss sentence as a golden fact; the line's words.
4. **The plan's facts line is dropped when nothing is to do** (CLI
   fault 12; call 31 against 28: `facts: stash listed 46s ago` absent
   on the `nothing to do` path). Rule: the facts line prints on every
   text path of a plan, the nothing-to-do path included
   (`plan_cmd.rs`, `render_nothing_to_do` beside `render_header`).
   Pin: a nothing-to-do plan's text holds its facts line, as a golden
   fact in the plan goldens.
5. **A count's heading says four values over five lines** (CLI fault
   13; call 65: `count rarity · 4 values` with five lines, "no rarity"
   among them). Rule: a count's heading counts the lines it prints
   (C146); where a "none" bucket is a line, it is a value. Pin: the
   counts view golden with a none bucket, the heading equal to the
   lines.
6. **The README says 25 tools where the server has 26** (MCP fault 8;
   call 1; `README.md:123`). Rule: the README's tool count is the
   server's, held by a test: `crates/acquisition-mcp/tests/mcp_reference.rs`
   (or the readme tour) reads the number from the README's sentence
   and compares it to the tools list. Pin: that test.
7. **Nothing marks the new league** (MCP fault 3 and CLI fault 6; MCP
   calls 5, 9, 10, CLI calls 14, 16, 96: the stored list and `acq
   leagues` show ids, realms, named and held, no start; the README
   says `acq leagues` names a new league the day it opens). Rule: a
   league entry's own start is a time the document carries (C163):
   `LeagueListShown.leagues[]` and the job's text gain `started_at`
   and `started_at_age` from the entry's `startAt` as GGG gives it
   (null and `never` where an entry has none; the mock's entries gain
   a `startAt` so the fixture holds one: `crates/acquisition-daemon/src/mockggg.rs:325–336`),
   the text says `started 2d ago` beside each league, newest start
   first in the stored read, and the README's new-league line says
   the stored read shows which league started last. Pin: a stored
   list with two starts, the words and the order; the README's tour
   line.
8. **`price status` counts the game's skips and offers no route to
   them** (CLI fault 10 and MCP's T5 note; CLI calls 69, 73, MCP call
   38: the route under the line lists invalid notes alone; the runner
   found the skips by `search 'note:"~skip"'`). Rule: a count a
   frontend prints names what the reader can do next (C146): `price
   list --game-text skip` (and the MCP's `game_text: skip`) lists the
   items whose game side says skip, and the status's skip count
   prints that route beside it where the count is nonzero. Pin: the
   route runs as printed and lists the three; the status text's
   words.
9. **`oldest_stale_seconds_age` reads as a duration's age** (MCP fault
   5; call 51: 4,651,202 beside `53d ago`). No code change: the
   seconds are the oldest stale item's age (now minus its sighting),
   and `53d ago` is that age's words. Rule: the gloss says so in those
   words. Pin: the gloss sample in `surface_size.rs`'s table.

## Not taken, the close's fates (issues or no fault)

MCP 1 (live job records carry no time: `JobInfo` is the protocol's, a
protocol change; joins #7), MCP 2 (a stash job for a tab the listing
does not name is sent), MCP 6 and CLI 3 (a create's undo stale after a
replace), MCP 7 and CLI 9 (`not_logged_in` for a name never seen),
MCP 9 (`shop_render` with `page` drops `freshness`), MCP 10 (no verb
clears a policy: C36's), MCP 11 (no MCP `store refused`: by design,
`describe` says so), MCP 12 (eta null until a route is learned: C40's
honesty), CLI 1 (invalid price text in tab names uncounted), CLI 2 (a
tab price's receipt does not say how many items take it), CLI 7 (a
failed fetch through the daemon offers no way on), CLI 8 (a refused
API verb leaves a daemon running), CLI 11 (text receipts omit
`in_facts`: C53's decision view), CLI 14 (a refusal naming several
accounts gives no ready command in text).

## Done when

- Each of the nine has its commit, its message naming this brief and
  the fault by its run and number, and its pin named for the fault.
- Every regenerated golden line is the realm joining a label, a facts
  line on a nothing-to-do plan, a count heading, a started line, a
  route under the skip count, or a `# holds` line; `MCP-REFERENCE.md`
  and `CLI-REFERENCE.md` regenerated where a tool's or a flag's text
  changed.
- `tools/gate.sh` is green on the round's tree, every stage 0; the
  round ends with the gate run bare (the seat skill's trap).

## Rules

`TEMPLATE.md`. The main tree, no other worker building; scratch the
subdirectory the launch names; `ACQ_UPDATE_FIXTURES=1` once after the
holders are moved, every changed line read; no push; no issue closed
or opened.

## Report

`TEMPLATE.md`, one paragraph per fault: the rule, the commit, the pin.
