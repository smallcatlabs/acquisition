# answer-facts-seat-fixes-2 — the seat's third round, run 2's words

The third round of the answer-facts seat (`audits/answer-facts-2026-11/seat/`,
run 2 on 2026-11-04: two Opus runners at the release build of
`f87e645d`, the CLI door 107 calls and the MCP door 65 calls; their
reports copied at the close as `REPORT-cli-2.md` and `REPORT-mcp-2.md`,
until then in the worlds `~/seat-worlds/answer-facts-2026-11/cli-opus/REPORT.md`
and `mcp-opus/REPORT.md`, the calls' outputs under each world's
`calls/`). Written by the orchestrating session against the tree at
`8e5c077e`. None of run 1's nine fixed faults recurred. Four of run
2's are this group's words and are taken here, each checked against
its calls; the rest are the close's fates. No new ruling; C146 (a
count says what it counts and what the reader can do next) and C53
(one clock per age, named where it is not the answer's). One commit
per fault, a `seat_faults.rs` test named for each; reviewed cold,
not sat again (the league seat's precedent for a third round).

## The faults taken

1. **`tabs`' item count reads as the league's total** (CLI run 2
   fault 2; calls 37, 42, 51, 52: `3347 tabs in Standard: … 20231
   items` against `price status`'s 21311, the characters' items the
   difference; both runs' runners worked it out by arithmetic). Rule:
   the head names its set: `… 20231 items in tabs (characters apart:
   \`acq store characters --league Standard\`)`, the command as the
   refusals print one (C156); the MCP's `tabs` gloss the same words.
   Pin: the tabs golden's head line.
2. **An applied plan's facts line is frozen and says `ago`** (CLI run
   2 fault 4; calls 32, 33: `facts: stash listed 53s ago` printed
   identically by two applies minutes apart; the plan file's listing
   ages are measured at the plan's `generated_at`). Rule: an age a
   text prints is at the answer's time, or names the clock it is
   measured from (C53; round 6 made the plan note say `before the
   plan`): the apply's facts line prints the listings' ages as the
   plan's wrapper measures them, `stash listed 53s before the plan`,
   with the plan's own age beside it, `plan made 2m ago`, both at the
   answer's time. Pin: an apply of a saved plan a fixed time later,
   the line's two ages exact.
3. **`location list seen 57d ago` reads as one list's age** (CLI run
   2 fault 8; call 86 against 42: the scope's `list_seen` is the
   oldest of the realm's listings, a character listing 57 days old,
   where the stash listing is 53 days old). Rule: a count or an age
   over several things names the set: the never-fetched line says
   `location lists seen, the oldest 57d ago` and the `list_seen` gloss
   the same ("the oldest of the lists the scope reads: the league's
   tab list and the realm's character list"). Pin: the scope line's
   golden fact; the gloss sample.
4. **Two lines that say nothing of what the reader can do next** (CLI
   run 2 fault 14; call 52: `tabs --limit 0` prints the dangling
   header `the 0 with the most items:`; call 38: `store events
   --summary` prints `0 events in the last 24 h` on a store whose
   newest event is 53 days old, with no way on). Rule: a count of none
   says which none and what to do (C53, C146): `tabs --limit 0` prints
   no table header (the counts stand alone, the selection's words say
   `0 of N shown`); the events summary with no event in its window
   prints `0 events in the last 24 h; the newest on record 53d ago
   (\`--hours N\` widens the window)`, the newest from the store's
   last event, `none on record` where there is none. Pin: both as
   golden facts.

Not taken (the close's fates): run 2 CLI 1 and MCP 9 (no age for the
store as a whole; `tabs` no league oldest/newest: an issue, the
subjects the group did not reach), CLI 3 and MCP 6 (#170), CLI 5 and
MCP 2/3 (#165, #166), CLI 6 (a refusal starts a daemon: C3's, honest),
CLI 7 (job payloads carry no fetched time: with #165), CLI 9 (the
mock's data), CLI 10 (a search refusal's offers for `-priced`: an
issue, area:search), CLI 11 and MCP 5 (#167), CLI 12 and MCP 8 (#168),
CLI 13 (the store grows on first open: the migration, the README
says), CLI 14's `index: 0` (the mock), MCP 1, 7 and 13 (README
sentences, the close's docs commit), MCP 4 and 12 (no fault), MCP 10
(#169), MCP 11 (an issue: a cross view's leagues with no items).

## Done when

Each of the four has its commit, its message naming this brief and
the fault by its run and number, and its pin; every regenerated line
is one of the four's words or a `# holds` line; both references
regenerated where help or a tool's text changed; `tools/gate.sh`
green, run bare at the end.

## Rules

`TEMPLATE.md`; the main tree, no other worker building; scratch the
subdirectory the launch names; `ACQ_UPDATE_FIXTURES=1` once after
the holders are moved; no push; no issue opened or closed.

## Report

`TEMPLATE.md`, one paragraph per fault.
