# The assessment at the end of step 6.5: what could be removed, 2026-11-20

Issue #293 asks what design, code or comment craft could be removed to
make the project materially smaller, now, while there are no users and
a key or a verb can simply go. This is the read it sets up, by a cold
session that built none of the code, at `3618728f` (step 6.5 closed at
`9e367764`). It removes nothing and makes nothing a task: each
candidate waits for the owner's word.

The first five, by size saved against risk:

1. **Stored shapes older than the released build.** The steps of the
   facts migration below version 8, the sync policy's versions 1 and
   2, the remedy for an earlier build's `user.db`, and `acq store
   import`. Needs his ruling: it touches his store.
2. **The terminal dashboard**, `acq dash` without `--json`: 811 lines
   and 43 of the CLI's 159 packages. No ruling by class; his say,
   since he is the one person who could be using it.
3. **The query's own order**: the order a search takes from its one
   comparison when no sort is named. Needs his ruling: it is in the
   design he approved, and #294 waits on him.
4. **The quote's estimate over a closed route**: a closed route is
   quoted with no estimate and its note, as an unlearned route is. No
   ruling.
5. **History in comments**: the comment lines that say when and in
   which round a thing was built. No ruling; a cut across many files,
   so it is reviewed (C175, what is reviewed).

The rest are below, with the two that already happened as the worked
examples and the item page, which this read would keep.

## The first read: every finding since the GUI work began

The window is every review under `roadmap/reviews/` first committed
on or after 2026-11-06, the day of the first review of 4-full part 1,
the GUI's first slice. The saved prompts are left out. `findings.py`
beside this file makes the four tables that follow:

```sh
python3 -I audits/assessment-6-5/findings.py
```

A finding is a line of a build review that opens with P1, P2 or P3
and a place. Three build reviews are in an older form and are counted
by their numbered heads, ungraded. A brief's review is counted by its
items, and those are not tabulated by file, since they are against
the brief. The close is the commit carrying the step's `Step:`
trailer; the closes before 2026-11-09 are rows of `roadmap/LEDGER.md`
and show none here. The last column is the close's count of
`retired X; held by Y` lines, or what it says instead.

### By step

| Step | First review | Brief reviews (items) | Build reviews | P1 | P2 | P3 | Ungraded | Reviews with no finding | Close | Retirement lines |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `4-full-1-design` | 11-06 | 1 (14) |  |  |  |  |  |  |  |  |
| `4-full-1-saved` | 11-06 | 6 (36) | 3 | 1 | 6 | 1 |  |  |  |  |
| `4-full-1-mcp` | 11-07 | 2 (6) | 1 |  |  |  |  | 1 |  |  |
| `4-full-1-pages` | 11-07 | 3 (22) | 2 |  | 1 | 2 |  | 1 |  |  |
| `4-full-1-answer` | 11-08 | 3 (39) | 1 |  |  |  | 9 |  |  |  |
| `4-full-1-cards` | 11-08 | 3 (29) | 1 |  |  |  | 6 |  |  |  |
| `204-tls-stack` | 11-09 |  | 1 |  |  |  | 9 |  |  |  |
| `governance-docs` | 11-09 |  | 1 |  | 3 | 2 |  |  |  |  |
| `governance-tools-1` | 11-09 |  | 1 |  |  |  |  | 1 | `7569da68` | says none |
| `governance-tools-2` | 11-09 |  | 1 |  |  |  |  | 1 | `268425e3` | 2 |
| `frontends-243` | 11-10 |  | 1 |  |  |  |  | 1 | `5f1ba7bb` | says none |
| `governance-seat` | 11-10 |  | 1 |  | 1 |  |  |  | `8e04545a` | says none |
| `network-216` | 11-10 |  | 1 |  |  |  |  | 1 | `2960d4bb` | says none |
| `network-218` | 11-10 |  | 1 |  |  | 1 |  |  | `8ea8be80` | says none |
| `process-229` | 11-10 |  | 1 |  | 1 |  |  |  | `7795a78f` | none stated |
| `process-238` | 11-10 |  | 2 |  | 2 |  |  |  | `9241b67a` | in prose |
| `process-240` | 11-10 |  | 3 | 1 | 3 | 2 |  |  | `daa31a51` | says none |
| `process-241` | 11-10 |  | 1 |  |  | 2 |  |  | `10c9ae82` | says none |
| `search-179` | 11-10 |  | 1 |  |  |  |  | 1 | `517836f8` | says none |
| `4-full-1-step6-count` | 11-11 |  | 2 |  | 2 |  |  | 1 | `c0a0493c` | in prose |
| `4-full-1-step6-driver` | 11-11 | 3 (13) | 1 |  |  | 1 |  |  |  |  |
| `4-full-1-step6-spike` | 11-11 | 4 (18) | 3 |  | 8 | 2 |  |  | `7d62c9ed` | says none |
| `4-full-1-step6-values` | 11-11 |  | 3 |  | 2 | 4 |  |  | `5f79bf96` | in prose |
| `daemon-178` | 11-11 |  | 1 |  | 1 | 1 |  |  | `5a3fef0f` | in prose |
| `daemon-198` | 11-11 |  | 1 |  |  |  |  | 1 | `c84b88b1` | says none |
| `daemon-208` | 11-11 |  | 1 |  | 1 |  |  |  | `32e2e2b9` | says none |
| `daemon-209` | 11-11 |  | 4 | 3 | 5 | 2 |  | 1 | `f19db25b` | in prose |
| `daemon-261` | 11-11 |  | 1 | 1 | 1 |  |  |  | `e183558f` | in prose |
| `frontends-203` | 11-11 |  | 1 |  | 1 |  |  |  | `977319db` | says none |
| `frontends-211` | 11-11 |  | 2 |  | 1 | 1 |  | 1 | `bd4cf1e5` | in prose |
| `frontends-246` | 11-11 |  | 5 | 4 | 8 | 5 |  |  | `87f560aa` | in prose |
| `plans-191` | 11-11 |  | 1 |  |  |  |  | 1 | `0f8f02d4` | says none |
| `process-214` | 11-11 |  | 1 |  |  |  |  | 1 | `434621eb` | in prose |
| `process-236` | 11-11 |  | 1 |  |  |  |  | 1 | `bd06abde` | says none |
| `saved-247` | 11-11 |  | 1 |  | 1 |  |  |  | `f07dd3de` | says none |
| `store-200` | 11-11 |  | 1 |  |  | 2 |  |  | `fd666971` | in prose |
| `store-205` | 11-11 |  | 1 |  |  |  |  | 1 | `8c347371` | says none |
| `store-234` | 11-11 |  | 2 |  | 1 | 4 |  |  | `a9ecfc72` | 2 |
| `trade-150` | 11-11 |  | 3 | 1 | 2 |  |  | 1 | `d3d022bc` | in prose |
| `273` | 11-12 |  | 2 | 1 |  | 1 |  |  |  |  |
| `4-full-1-step6-check` | 11-12 | 2 (5) | 1 |  | 2 |  |  |  | `69ff08aa` | in prose |
| `4-full-1-step6-sort` | 11-12 |  | 3 | 3 | 2 |  |  | 1 | `07590d6f` | 1 |
| `4-full-1-step6-table` | 11-12 | 2 (9) | 2 |  | 2 | 2 |  |  | `de5d8247` | 3 |
| `277-tables-and-bodies` | 11-17 | 1 (6) | 1 |  |  |  |  | 1 | `0ad74932` | says none |
| `search-275` | 11-17 |  | 1 |  |  | 1 |  |  | `f01d2ba0` | says none |
| `search-276` | 11-17 |  | 2 | 1 |  | 2 |  |  | `3e449f1d` | says none |
| `sitting-282` | 11-17 |  | 1 |  | 1 | 1 |  |  | `8bee2ab8` | says none |
| `sitting-33` | 11-17 |  | 1 |  | 1 |  |  |  |  |  |
| `store-279` | 11-17 |  | 1 |  | 1 |  |  |  | `a6d574f4` | says none |
| `4-full-1-step6-grouped` | 11-18 | 2 (8) | 3 | 1 | 3 |  |  | 1 | `043726af` | none stated |
| `4-full-1-step6-places` | 11-18 | 1 (3) | 2 |  |  | 4 |  |  | `f73b8a12` | 1 |
| `4-full-1-step6-plan-jobs` | 11-18 | 2 (14) | 2 |  | 3 | 1 |  | 1 | `b205fa1a` | in prose |
| `network-288` | 11-18 |  | 3 |  | 5 | 2 |  | 1 | `39947ef2` | 1 |
| `network-291` | 11-18 |  | 3 |  | 3 | 3 |  | 1 | `6725e97b` | 13 |
| `process-286` | 11-18 |  | 3 | 3 | 5 | 1 |  |  | `0b70defa` | 2 |
| `search-292` | 11-18 |  | 1 |  |  | 1 |  |  | `24773654` | says none |
| `4-full-1-step6-row` | 11-19 | 2 (12) | 2 |  | 1 | 1 |  | 1 | `9e367764` | says none |
| `web-step6-seat` | 11-19 |  | 3 |  | 3 |  |  | 1 |  |  |

58 steps, 37 brief reviews (234 items), 97 build reviews, 155 graded findings (P1 20, P2 83, P3 52), 24 ungraded.

### By area

| Area | Findings | P1 | P2 | P3 |
| --- | --- | --- | --- | --- |
| cli src | 32 | 1 | 20 | 11 |
| tools, hooks, workflows | 23 | 4 | 12 | 7 |
| daemon src | 21 | 4 | 12 | 5 |
| search src | 17 | 5 | 6 | 6 |
| audits (the spike's rig, a seat's record) | 12 | 0 | 9 | 3 |
| verbs src | 10 | 2 | 5 | 3 |
| documents, rulings, skills, messages | 10 | 0 | 5 | 5 |
| cli tests | 8 | 0 | 5 | 3 |
| store src | 8 | 1 | 4 | 3 |
| mcp src | 5 | 2 | 3 | 0 |
| search tests | 4 | 1 | 1 | 2 |
| daemon tests | 2 | 0 | 1 | 1 |
| mcp tests | 1 | 0 | 0 | 1 |
| verbs tests | 1 | 0 | 0 | 1 |
| store tests | 1 | 0 | 0 | 1 |

### By file, three findings or more

| File | Findings | Steps |
| --- | --- | --- |
| `crates/acquisition-cli/src/web/mod.rs` | 12 | `4-full-1-pages`, `4-full-1-step6-count`, `4-full-1-step6-grouped`, `4-full-1-step6-plan-jobs`, `4-full-1-step6-row`, `4-full-1-step6-table`, `web-step6-seat` |
| `crates/acquisition-daemon/src/daemon/queue.rs` | 10 | `daemon-209`, `daemon-261` |
| `audits/web-2026-11/spike/rig-1.patch` | 9 | `4-full-1-step6-spike` |
| `tools/tracer-rung.sh` | 8 | `process-286` |
| `.githooks/pre-push` | 7 | `process-238`, `process-240` |
| `crates/acquisition-cli/src/web/page.rs` | 6 | `4-full-1-step6-places`, `4-full-1-step6-plan-jobs`, `web-step6-seat` |
| `crates/acquisition-search/src/answer.rs` | 5 | `4-full-1-step6-values`, `trade-150` |
| `crates/acquisition-verbs/src/saved.rs` | 4 | `4-full-1-saved`, `saved-247` |
| `crates/acquisition-mcp/src/server.rs` | 4 | `frontends-246` |
| `crates/acquisition-store/src/annotations.rs` | 4 | `store-234` |
| `crates/acquisition-store/src/index.rs` | 3 | `4-full-1-saved` |
| `crates/acquisition-cli/src/search_cmd.rs` | 3 | `4-full-1-step6-values`, `frontends-246` |
| `crates/acquisition-daemon/src/daemon.rs` | 3 | `daemon-208`, `daemon-209`, `network-288` |
| `crates/acquisition-verbs/src/jobs.rs` | 3 | `frontends-246`, `network-288`, `network-291` |
| `crates/acquisition-cli/src/store_cmd.rs` | 3 | `frontends-246`, `store-279` |
| `crates/acquisition-cli/tests/browser.rs` | 3 | `4-full-1-step6-check`, `4-full-1-step6-table` |
| `crates/acquisition-search/src/trade/stats.rs` | 3 | `4-full-1-step6-sort` |
| `crates/acquisition-daemon/src/daemon/quote.rs` | 3 | `network-288`, `network-291` |
| `crates/acquisition-cli/tests/apply_loop.rs` | 3 | `network-288`, `network-291` |

### By shape

| Shape of the review prompt | Findings citing it |
| --- | --- |
| none cited | 41 |
| 1 | 40 |
| 3 | 37 |
| 2 | 28 |
| 7 | 15 |
| 6 | 8 |
| 10 | 6 |
| 4 | 5 |
| 5 | 2 |

### The closes and what they retired

```sh
git log --since=2026-11-06 --grep='^Step: ' --format=%h | wc -l          # 55
git log --since=2026-11-06 --grep='^Step: ' --format=%B | grep -c '^[ -]*retired .*; held by'   # 27
```

Of the 55 closes, 28 say in words that nothing was retired. The 55
carry 27 retirement lines, 13 of them network-291's, which is the one
close that withdrew a property. No test file was deleted in the
window. Between the last commit before 2026-11-06 and `3618728f`:

| Measure, under `crates/` | Before 2026-11-06 | Now | Change |
| --- | --- | --- | --- |
| Source lines (tests apart) | 101,660 | 115,720 | +14,060 |
| Of them comment lines | 23,105 | 26,184 | +3,079 |
| Test lines | 113,556 | 128,561 | +15,005 |
| Test functions | 1,553 | 1,676 | +123 |

The counts are `git grep -c` over `crates/*.rs` at each commit, a path
holding `/tests/` or `/unit/` counted as tests. P6 (retire what is
proven) has a procedure since C154, and in this window it moved pins
between layers; it made nothing smaller but where a ruling was
withdrawn.

### Where findings repeat

The grouping is by hand, from reading each graded finding's failing
input; the script does not make it, and a finding can stand in two
rows.

| Where | Findings | Steps | What repeats |
| --- | --- | --- | --- |
| A stored row or file this build would not have written | 24, 7 of the 20 P1s | `4-full-1-saved`, `4-full-1-pages`, `daemon-209`, `daemon-261`, `frontends-246`, `store-234`, `process-286` | The failing input is a row at another version, a damaged row, or an earlier build's file: a saved search, a queue row, the main account's value, the annotation file, a version 2 policy. Each store and each verb above it handles the case for itself. |
| Which account, among several | 20 | `4-full-1-saved`, `frontends-246`, `saved-247`, `process-286` | A default account read twice, said in one place and not the next, dropped from a printed command. The main account was built and backed out the same day. |
| Words wider than the code | 71 cite shape 1 or shape 3; about 19 of them are against a comment, a module doc or a document | 17 steps for the 19 | A sentence written for the common case, or left standing beside a fix: a module doc that still promises what the commit withdrew. |
| The values a search compares, and their order | 14 | `4-full-1-step6-values`, `273`, `4-full-1-step6-sort`, `search-292` | A value joined by its text in three places; the bound counted before the sort joined; a default order that a rewrite of the same question changes. #294 is open on it. |
| A closed route (C190) | 13 | `network-288`, `network-291` | Five of #288's seven were the closed-for-good state. The quote's estimate over a closure is a finding in four of the six reviews. |
| The live-run drivers | 11 | `process-286`, `frontends-246` | `tools/tracer-rung.sh`'s write and put-back of a policy (since removed), and `tools/persist-check.sh`'s account preflight. |
| The pre-push hook and its CI twin (C186) | 9 | `process-238`, `process-240`, `process-241` | The recovery commands the hook prints, and the history it reads in a shallow clone. |
| The spike's rig | 10 | `4-full-1-step6-spike` | Three rounds on a patch under `audits/` that ships nothing. C176 has since given a spike's brief one review. |
| The league of a place page | 6 | `4-full-1-step6-places`, `4-full-1-step6-grouped`, `web-step6-seat` | Two leagues that differ by case, and a page that shows one league while its control shows another. |
| One stored thing read twice | 5 | `4-full-1-pages`, `frontends-246`, `4-full-1-step6-grouped` | A second read of the same row or corpus that a concurrent writer can move (shape 7). |

`crates/acquisition-cli/src/web/mod.rs` leads the table by file with
12 findings over seven steps. They share no cause: it is the file
every GUI step changed. The briefs drew 234 items in 37 reviews, 36
of them over the six rounds of the saved searches' brief. C176 caps a
brief at two rounds, and each brief reviewed from 2026-11-12 on took
one or two.

## The worked examples: two removals that already happened

**C190's closed-for-good state** (#291), the example #293 names. One
state brought a protocol key, a parent rule, null estimates and #289.
#288 added 2,018 lines under `crates/` (`git diff --shortstat
602a801b^ 39947ef2 -- crates`); the removal `9ca22041` deleted 919
and added 386; its close carries 13 retirement lines, nine of them
"held by nothing" because the property was withdrawn.

**The main account** (#246), which the table surfaced. A default
account among several was ruled, built, reviewed three times and
backed out on 2026-11-11: 17 findings in five reviews, four of them
P1. The back-out `fd7fc5fb` deleted 1,931 lines under `crates/` and
added 116. What replaced it is a refusal: among several accounts, a
job names its account.

Both were a default or a state that saved a user one decision, and
both went when a refusal or a wait that the tool already had was
found to say the same thing. Candidates 3 and 4 below have that
shape.

## The candidates, ranked by size saved against risk

Sizes are rough: counted lines where a command is given, estimates
elsewhere, and said so.

### 1. Stored shapes older than the released build

- **What goes.** The facts store's migration steps below version 8
  (`crates/acquisition-store/src/lib.rs`, the ladder in `Store::open`,
  156 lines of which the step from 8 to 9 stays) and the history of
  them in the module doc. The sync policy's versions 1 and 2, upgraded
  on parse (`crates/acquisition-plan/src/lib.rs`). `HoldsAccountFacts`,
  the detection of an earlier build's `user.db` and its remedy
  (`crates/acquisition-store/src/index.rs`). `acq store import`, which
  replays a snapshot of the retired `acq pull`, with its tolerance for
  items without ids (`crates/acquisition-cli/src/store_cmd.rs`,
  `import` and `strip_idless_items`, about 95 lines).
- **What holds after.** A file or a value below the floor is refused
  with its remedy, as the annotation file already is
  (`SCHEMA_FLOOR`, `c65_a_file_below_the_floor_is_refused_never_migrated`);
  `v8_to_v9_adds_the_live_indexes_and_moves_no_row` stays, and so
  does the jobs ledger's version 1, which is the release's own; facts
  are refetchable (C35, annotations are the only irreplaceable state).
- **Evidence.** `v1.0.0-alpha.1`, the one release, wrote facts 8,
  policy 3 and annotations 3 (`git show v1.0.0-alpha.1:<file> | grep
  VERSION`). Everything older was written by development builds, and
  the annotation file set its floor at 3 for that reason. Findings in
  `4-full-1-saved`'s fix rounds (three) and `process-286` (a P1) are
  faults in serving such files.
- **Size.** About 300 source lines; three store tests, part of one
  plan test and three tests of the earlier `user.db` (about 400 test
  lines); one verb with its reference section; one golden block.
- **Ruling.** Yes. It touches his store, which must stand at facts 9
  first. A stored policy is rewritten only by a write, so his row may
  still be a version 2 value: one `acq policy set` before the cut
  rewrites it. C124's and C65's words name the migration.
- **Risk.** Low once his store is checked. A mock world from an
  earlier build is scratch.

### 2. The terminal dashboard

- **What goes.** `acq dash` as a terminal UI
  (`crates/acquisition-cli/src/dash.rs`, 811 lines) and `ratatui`.
  `acq dash --json`, the snapshot, stays, and so does the
  `Dashboard` request: the daemon's own tests read it in 44 places.
- **What holds after.** Nothing needs holding: the file has one test.
  The same facts are `acq dash --json`, `acq daemon status`, `acq
  jobs` and the jobs page.
- **Evidence.** It is the one surface no agent can drive, in a tool
  whose goal is agent-forward, and the GUI is now the person's
  surface. `cargo tree -p acquisition-cli -e normal --prefix none`
  lists 159 packages, and 116 with `--prune ratatui`.
- **Size.** About 700 lines and 43 packages out of the CLI's build
  and its licence list.
- **Ruling.** Not by class: a verb is the session's (C170). He is
  the only one who could be watching it at a live run, so it is his
  to say.
- **Risk.** Low.

### 3. The query's own order

- **What goes.** The order a search takes when no sort is named and
  its query compares exactly one value with a direction
  (`crates/acquisition-search/src/values.rs`: `Direction`,
  `default_order`, and the rules for a not, a capped `holds`, a range
  and two nots that cancel). The words that explain it in the CLI's
  answer, `describe`, the help and the README tour. The values stay
  as columns, and a head still sorts.
- **What holds after.** Rows stand in the store's order unless a sort
  is named, so two spellings of one question take the same order by
  construction, which is the order half of the search's invariant 7.
  The head's sort is held by
  `a_head_orders_by_its_value_and_flips_and_the_box_keeps_what_was_named`.
- **Evidence.** Fourteen findings over four steps on the values and
  their order; #292 was a default order moved by a padding; #294 is
  open because one question spelled two ways gets two values and so
  loses the order. With no inferred order, #294 shrinks to a second
  column. The trade site, the GUI's reference, orders by nothing the
  query implies.
- **Size.** About 80 source lines, 200 to 300 test lines (the
  direction cases in `tests/rows.rs`, the order half of
  `tests/generated_equivalence.rs`, #292's pin), and about a dozen
  goldens regenerated. The saving is the rule more than the lines.
- **Ruling.** Yes. "Values on rows and the query's own order" is in
  the step 6 design he approved, and #294 waits on his word.
- **Risk.** Medium. A search for `life>=90` no longer opens largest
  first; it is one press of a head, or `sort:`.

### 4. The quote's estimate over a closed route

- **What goes.** The estimate a quote gives for a scope with a closed
  route: the floor in `crates/acquisition-daemon/src/daemon/quote.rs`
  (the C190 block) and the closure arithmetic in
  `ChokePoint::project`. A closed route's scope answers
  `eta_seconds: null` and its note, as a route whose policy is not
  yet learned and one closed by a failed probe already do.
- **What holds after.** C40's own words, an estimate "and the
  prerequisites it cannot see":
  `quote_projects_per_scope_reserves_nothing_and_names_what_it_cannot_see`,
  with one row for a closure. The wait itself is unchanged; a waiting
  job's own estimate is the choke point's, not the quote's.
- **Evidence.** A finding in four of the six reviews of #288 and
  #291: each fix composed the pacing with the closure for one more
  case (a policyless route, a scope whose sample is open, 31 requests
  past a window).
- **Size.** About 30 source lines and two tests of 113 lines; the
  plan page's sentence for an unknown timing gains a case.
- **Ruling.** No. The key stays and is already optional.
- **Risk.** Low. A plan over a closed route says when the route
  reopens and not when its last request could go.

### 5. History in comments

- **What goes.** The comment lines that tell when and in which round
  a thing was built: 255 name a date, 318 an issue and 585 a round, a
  review, a step, a rung or a seat (`git grep -c` over the source's
  comment lines; the three overlap). A line that states the
  mechanism or cites its ruling stays, as `AGENTS.md` asks.
- **What holds after.** `git log` and `git blame` hold the history,
  the registry the ruling, the test the property.
- **Evidence.** 26,184 of 115,720 source lines are comments, 7,128 of
  them module docs. 71 of the 155 graded findings cite shape 1 or
  shape 3, and about 19 are against a comment, a module doc or a
  document: every sentence that retells the code is one more to keep
  true.
- **Size.** Perhaps 800 to 1,500 lines, unmeasured until the cut is
  tried on one crate.
- **Ruling.** No. A cut across many files is reviewed (C175).
- **Risk.** Low for behaviour. The risk is cutting a "why" with a
  "when", so the rule is to cut the date and the round and keep the
  reason.

### 6. The unreadable row, above the store

- **What it is.** The largest pattern in the table (24 findings,
  seven P1), and the one this read could not size. Every kind of
  intent threads its own unreadable state from the store through the
  planner and the verbs to each frontend: `StoredPolicy`,
  `PolicySource` and `PolicyState`, `Conflicting`, `SavedRow`,
  `OpenedRow`, and the queue's tainted trees. `git grep -ci
  unreadable -- crates` counts 227 lines in source and 170 in tests,
  with 22 tests named for it and six goldens.
- **What could go.** The states above the store, leaving C151 where
  it was ruled: the store names the row, never drops it, and clears
  it by revision. A verb that meets one answers the store's one
  refusal with its remedy, and no surface renders a half-read state.
- **Ruling.** Yes (C151's "never a wall", C47, C6), and after a
  design read: C151 came from a damaged row that hid every price.
- **Risk.** Medium to high. It is here because the table says the
  shape is missing; a second round of faults in one place is the
  sign to look for a reframing before the next patch.

### 7. A job's priority

- **What goes.** `priority` on a submit and on a job, the
  `SetPriority` request, `acq set-priority` and the MCP's parameter.
  `git grep -ci priority -- crates` counts 98 lines in source and 87
  in tests.
- **What holds after.** Jobs run in the order submitted.
- **Evidence.** No frontend sets one but by hand; the pages offer
  none.
- **Ruling.** Yes: C4 names it, and it is a protocol key.
- **Risk.** Medium. The GUI's resident (4-full part 3) may want one
  press to go ahead of a long refresh, which is what a priority is
  for. Decide before `alpha.2`, since the key is then a compatibility
  question.

### 8. The persistence driver

- **What goes.** `tools/persist-check.sh` and
  `tools/persist-verify.py`, 523 lines. The road says it has not run
  since it was last edited (2026-11-11), and its one ledger row is of
  2026-08-30. The tracer driver is not a candidate: it ran at the
  3.30 launch on 2026-11-17.
- **What holds after.** Its three steps, a halt, a crash and a
  resume, are held against the mock by the daemon's `queue.rs`,
  `exit.rs` and `rails.rs` tests and by `persisted_queue.rs`; what
  only the live run holds is that GGG's counters agree after the
  second lifetime.
- **Ruling.** Yes: a live run is his. The cheapest answer is to run
  it once at the next live run, and retire it if it teaches nothing.

### 9. The recovery commands the pre-push hook prints

- **What goes.** The printed rebase recipe in `.githooks/pre-push`
  and its breaker cases. The hook still refuses and names each
  commit.
- **Evidence.** Seven findings in two steps, one P1 (the printed
  rebase drops what a merge commit alone introduced).
- **Size.** About 30 lines of a 99-line hook.
- **Ruling.** Yes: C186's words say the hook "prints how to commit
  them again".

### 10. Values that differ only by case

- **What it is.** The search folds case, so two values that fold to
  one ("twins") take an exact term: 128 source lines and 109 test
  lines name them, for leagues, fields and templates. Three findings
  came from two leagues that differ by case, which a reviewer called
  unlikely outside fixtures.
- **Status.** Not weighed: whether templates need it is a question
  for a read of `counts.rs`, and the league case alone is small. The
  session's to decide (C170).

### The item page: keep

The page at `item/<id>` is reached from the cards view and the
reasons table, and since 6.5c a row opens its item in place. Its
handler is about 40 lines and its renderer about 100, because the
card is shared. It is the one page that shows an item's lines with
their templates and slots and a query against it term by term, and
`item/<id>` is what a person and an agent quote to each other. The
saving is small and the loss is real.

### Looked at and left

- `acq demo` and the mock-only kinds a hand `submit` takes (`sleep`,
  `fetch`, `whoami`): four source lines for the verb; the kinds are
  the tests' instruments.
- The saved prompts under `roadmap/reviews/` (105 files, 462 KB, each
  the standing prompt again): they sit on no reading path, this read
  filtered them by name, and `decisions/process.md` already declines
  to fold them until a reader is slowed.
- The quote as a whole (C40): it tells an agent what a plan costs in
  time before it is applied, which a rate limit that can hold 300
  seconds makes worth knowing.
- The persisted queue (C6): `queue.rs` drew ten findings, all in the
  restore of rows it cannot read, which is candidate 6's shape and
  not a reason to stop persisting.

## What this read did not do

It built nothing and ran no test. No candidate's removal was tried,
so every size but a counted file is an estimate. It did not read the
owner's store. Candidates 6 and 10 need a design read before they
can be weighed at all. The grouping of findings by cause is one
reader's.
