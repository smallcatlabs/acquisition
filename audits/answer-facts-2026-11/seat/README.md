# The answer-facts seat — the record

The seat the answer-facts group waited on (`roadmap/reviews/answer-facts-map.md`;
rounds 1 to 6 closing #155, #156, #157, #22, #164, #105 and #12): a
blind agent at the group's `acq` and `acq-mcp`, asked at every task
how old what it answered from was, and where it came from. Run under
the briefs (`audits/answer-facts-2026-11/seat/BRIEF-cli.md` and
`BRIEF-mcp.md` at `61e32568`, removed at the close as the seat skill's
step 10 says), `PROMPT-cli.md`, `PROMPT-mcp.md` and
`PROMPT-debrief.md`, from the league seat's at `ea15df1d`. Two
sittings per run, the CLI door and the MCP door, each its own Opus
runner; run 1 on 2026-11-04, the fix round
(`roadmap/briefs/answer-facts-seat-fixes.md`, `0a77a06c..b3008e89`,
nine faults) the same day, its cold review
(`roadmap/reviews/answer-facts-seat-fixes-b3008e89.md`: one P2, a
count heading not counting its none line, and two P3) answered in
`f87e645d`; then run 2 the same day on the tree at `f87e645d`; run
2's four word faults fixed in a third round
(`roadmap/briefs/answer-facts-seat-fixes-2.md`, `eecb6ef4..6483b9f6`),
reviewed cold (`roadmap/reviews/answer-facts-seat-fixes-2-6483b9f6.md`:
two P2, the audit view's listing ages still frozen without their clock
and the scope gloss missing its fallback to the realm's oldest, and
four P3) answered in `ded97bad..a805e002`, one finding left unpinned
by its nature (the plan's age read after the awaited quote: a pin
would need a slow quote); not sat again, the league seat's precedent
for a third round. The fates below were decided by the
orchestrating session under the owner's standing word for the seat
and his grant of 2026-11-03 for the group; what touches a ruling is
listed for him at the close.

## The worlds

Made 2026-11-04 under `~/seat-worlds/answer-facts-2026-11/`, outside
the repository and short-pathed for the socket. `pristine/` is `setup
--store` over the league seat's pristine
(`~/seat-worlds/league-2026-11/pristine/store`: one account,
`GERWARIC#7694`, provider `mock`, the readiness seat's facts of
2026-09-02 to 09-11, 22,721 live items, 52 MB), so the owner's store
was not read. `cli-opus/` and `mcp-opus/` are each `setup --store
pristine/store --daemon`, with the brief as `BRIEF.md`, `README.md` as
the build carries it, the doors and `bin/`. The MCP world's daemon was
started by the world's maker, as the README's agent section has a
person do: `seat.py login seat --env ACQ_IDLE_SHUTDOWN=14400` before
the launch (its call set aside as `setup-login-mcp-N.csv`), so the
runner found a daemon up with a four-hour idle exit. The CLI runner
logged `seat` in itself through the door. Run 1's worlds were renamed
`cli-opus-1` and `mcp-opus-1` at the rerun and run 2's made under the
same names from the pristine; all four torn down at the close. **The
pristine stays.**

A door trap met while making the worlds: `setup --store DIR` with a
DIR that did not exist made a bare world (`store: []`) and said "the
world at …" without a word; the two first worlds were torn down and
remade from a pristine made first, and the door now refuses a
`--store` that holds no store (`8e5c077e`).

| Build | Tree | `acq` sha256 | `acqd` | `acq-mcp` | Worlds |
| --- | --- | --- | --- | --- | --- |
| `cargo build --release -p acquisition-cli -p acquisition-mcp -p acquisition-daemon` and the store copier, aarch64-apple-darwin | `2bed3ba5` (round 3's close) | `0e34debc…` | `6d846514…` | `e7a94edc…` | cli-opus-1, mcp-opus-1 (run 1) |
| the same | `f87e645d` (the fix round's review answered) | `20414487…` | `42a487d3…` | `5f963500…` | cli-opus, mcp-opus (run 2) |

The runner's startup context is the owner's global `CLAUDE.md` alone,
sha256 `e9c1329f…`, as at the league seat.

## The runs

Each launched by `.claude/skills/seat/launch.sh` in its world, the
debrief the same session resumed. Counts are the door's (`calls.csv`);
a report's own counts are the runner's, never recomputed here. Cost
is the session's at list price, the debrief's the difference the
resumed session added. Denials are the runner's own harness refusing
a compound door command or a heredoc, each sent again alone or as a
draft.

| Run | Model | Session | Calls | Cut | Turns | Cost | Denials | Files |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus-1 (run 1) | `claude-opus-5-5` | `0ebe6d84` | 121, the door's login among them | 0 | 135 + 2 | $4.54 + $0.17 | 0 | `calls-cli-1.csv`, `REPORT-cli-1.md`, `DEBRIEF-cli-1.md` |
| mcp-opus-1 (run 1) | `claude-opus-5-5` | `f972085b` | 62 | 0 | 89 + 3 | $3.75 + $0.25 | 3 | `calls-mcp-1.csv`, `setup-login-mcp-1.csv`, `REPORT-mcp-1.md`, `DEBRIEF-mcp-1.md` |
| cli-opus (run 2) | `claude-opus-5-5` | `04cae034` | 107, the door's login among them | 0 | 124 + 3 | $4.25 + $0.24 | 2 | `calls-cli-2.csv`, `REPORT-cli-2.md`, `DEBRIEF-cli-2.md` |
| mcp-opus (run 2) | `claude-opus-5-5` | `5395b39f` | 65 | 0 | 64 + 2 | $3.67 + $0.22 | 1 | `calls-mcp-2.csv`, `setup-login-mcp-2.csv`, `REPORT-mcp-2.md`, `DEBRIEF-mcp-2.md` |

cli-opus-1: all nine tasks finished; no file opened outside the two;
T2's ceiling run met an empty plan, the runner making no work for it;
every price cleared and read back; the daemon left stopped. mcp-opus-1:
all nine tasks finished; the harness saved the `open` output (calls 1
and 3) and the runner read it there, named in its "Left out"; the
policy it wrote on `GERWARIC#7694` left in place, no verb clearing
one. cli-opus (run 2): all nine tasks finished; no file opened outside
the two (the harness kept call 43's 33.5 KB `--describe`, unopened);
the plan file's ceiling and second hand-over met an empty plan (calls
32, 33), the runner saying how it would have made work rather than
changing the person's freshness; both prices cleared and read back
(calls 79 to 83), the stale undo recovered through the refusal's
offer; the daemon left stopped (call 107). Its debrief marks the store's
growth, the class and level mismatch and "the new one" as the mock's
or inferences, and names the two answers it gave for a set it was not
the age of (below). mcp-opus (run 2): all nine tasks finished; the
`open` output read from the harness's file, named; the stale undo
recovered through the conflict's offer (calls 55, 57); every count
back to call 31's (call 58); the daemon never went away. Its debrief
retracts "same instructions" for call 4 (a size compared, not the
text), "all 16 jobs done" (the daemon's ledger, not the apply's 7) and
"492 priced by the game" (489, the 3 skips apart), and says it wrote
the irreversible policy (call 52) knowing it from call 12 and flagged
it after rather than before.

## What this seat measured

1. **Calls per task**, T0 to T8: run 1, CLI 8, 13, 23, 10, 11, 19, 9,
   13, 14 (121, with the door's login), MCP 2, 2, 19, 6, 7, 11, 8, 5,
   2 (62); run 2, CLI 7, 13, 12, 9, 11, 21, 9, 11, 13 (107, with the
   login), MCP 2, 3, 21, 5, 8, 11, 8, 5, 2 (65). The MCP's `open`
   37,872 bytes (run 1) and 37,913 (run 2), against 36,104 at the
   league seat.
2. **Calls to the first answer the runner held right**, within the
   task from the call its report names: run 1, CLI T1 1 (call 10), T2
   1 (the guarded write, 24) and 5 (the plan, 28), T3 2 (the counts,
   47) and 5 (the ages, 50), T4 3, 7, 10, 11 for (a) to (d) (58, 62,
   65, 66), T5 3 (the status, 69), 5 (the bad notes, 71) and 17 (the
   read-back, 83), T6 2 (87), T7 1 (95), T8 1 (108); MCP T1 1 (3), T2
   1 (5), T3 1 (24), T4 2 (31), T5 2 (the invalid note, 38) and 6 (the
   writes, 42), T6 1 (48), T7 1 (56), T8 1 (61). Run 2: CLI T1 1 (9),
   T2 2 (23), T3 4 (37), T4 6 (48), T5 2 (55), T6 2 (76), T7 1 (84), T8
   1 (95); MCP T1 2 (4), T2 1 (6), T3 1 (27), T4 2 (33), T5 1 (40), T6
   1 (51), T7 1 (59), T8 1 (3).
3. **The group's question: how old is what the answer rests on, and
   from where.** Both run 2 runners answered it from the printed
   document alone at every task that holds facts: the facts line on
   `price status` with the oldest and newest fetch, each with its
   realm and location (CLI call 42, MCP 30); the matches' span apart
   from the scope's on a search (CLI 48, 86: "the age of the matches
   … separately from the scope's"; MCP 33: "correct by key, not
   verified by value", the two spans reading the same for 187 matches
   as for the realm); each league's start on the stored list, "started
   4d ago" naming the new league (CLI 15, MCP 11); the shop's
   freshness and its "positions seen before the current stash
   listing" (CLI 76, 78); a plan's basis ages at the plan's time (CLI
   27, MCP 14). What neither could read: an age for the store as a
   whole (CLI 36, MCP 27), a league's oldest and newest on `tabs` (CLI
   37, MCP 30), a fetched time on a job's record (CLI 14 to 20, MCP 9
   to 25): the issues below.
4. **The ages the runners quoted for a set they were not the age of**
   (the debrief's tenth question, asked for this seat): run 2's CLI
   gave 57 d as the store's oldest character fetch from ten of 65
   characters (call 40), let a listing's age stand beside positions
   the page says predate it (76), and quoted a route's header age as
   the limiter's state (T2); it answered "what have I priced" for
   Standard as though for every league (42, 57) and took `tabs`'
   20,231 for the league's whole before correcting it (37 against 51).
   The MCP's gave the store's snapshot age for the age of the items
   counted (T3) and could not show the matches' span differed from the
   realm's (33). Each is a number the document carries beside a wider
   one; the third round's faults 1 and 3 are the two whose words
   invited it.
5. **The fix round's nine, met again by neither run 2 runner:** the
   scope's and the total's spans named one location on a tie (MCP 33);
   every location label carried its realm (CLI 42, 48; MCP 30, 33);
   the never-fetched line said "tabs" (CLI 48); the facts line printed
   on the nothing-to-do plan (CLI 30, 32, 33); the count heading
   counted its none line (CLI 51); the README's 26 tools matched the
   server's (MCP 1, 4); each league carried its start (CLI 15, MCP 11);
   `price status` named the skips' route (CLI 42, 56); the stale age's
   gloss read as an age (MCP 31).

## The faults, by the rule each expresses

Run 1's, read against the calls they cite before the fix brief: MCP
calls 1, 9, 26, 51, 60 and CLI calls 28, 31, 50, 51, 65, 98, with both
`calls.csv`. Run 2's, read before their fates: CLI calls 32, 33, 37,
38, 42, 48, 52, 86 and MCP calls 27, 30, 38, 52, 53, 54. **Run 2 met
none of run 1's nine fixed faults again** (measured above).

| Rule | Faults (calls) | Whose | Fate |
| --- | --- | --- | --- |
| A span's tie names one location on every block | the scope's newest a stash and the total's a character at one second (MCP run 1 fault 4; calls 26, 31, 33 to 35) | the tool's | fixed, `0a77a06c`: the matches' places ordered by the store's order before the span; `seat_faults.rs` |
| A location named on a span names its realm | `Standard / gerwarshot (character)` for a poe2 character under `--realm all` (CLI run 1 fault 4; calls 50, 63, 98) | the tool's | fixed, `0c75c2ca`: `pc · Standard / Dump Tab` through the one formatter |
| A key's gloss and its text name the set exactly (C146) | `never_fetched_leagues` names the leagues holding a never-fetched tab, read as every league (CLI run 1 fault 5; call 98 against 51) | the tool's | fixed, `f83e1826`: the gloss and the line say "tabs"; no rename |
| The facts line prints on every text path of a plan | absent on the nothing-to-do path (CLI run 1 fault 12; call 31 against 28) | the tool's | fixed, `dfb5df28` |
| A count's heading counts every printed line (C146) | "4 values" over five lines, the none line uncounted (CLI run 1 fault 13; call 65); the review's P2, the undecided line (the fix round's review) | the tool's | fixed, `ad843dac` and `f87e645d`: every printed line counted |
| The README's count is the server's, held by a test | 25 tools where the server has 26 (MCP run 1 fault 8; call 1) | the README's | fixed, `03032e50`: the sentence read against `tools/list` |
| A time a document carries has its words (C163) | nothing marks the new league: the stored list shows no start (MCP run 1 fault 3; calls 5, 9, 10; CLI run 1 fault 6; calls 14, 16, 96) | the tool's | fixed, `0ed4fc88`: `started_at` and its age from `startAt` (N51), the newest start first; the README's new-league line |
| A count names the list that holds it | `price status` counts the game's skips and offers no route (CLI run 1 fault 10; calls 69, 73) | the tool's | fixed, `7e52710c`: `--game-text skip` and its command |
| A gloss says what its number is | `oldest_stale_seconds_age` read as a duration's age (MCP run 1 fault 5; call 51) | the tool's | fixed by the gloss, `b3008e89`: the oldest stale item's age in seconds and in words |
| A date is read whole before it is believed | the fix round's review: a calendar or a fraction the parser let through | the tool's | fixed, `f87e645d`: `parse_utc` validates |
| A count says what it counts (C146), run 2 | `tabs`' "20231 items" for "3347 tabs in Standard" read as the league's total, the characters' items apart (CLI run 2 fault 2; calls 37, 42, 51, 52) | the tool's | fixed, `eecb6ef4`: "items in tabs" with the characters' command beside it, both surfaces |
| An age names its clock where it is not the answer's (C53), run 2 | an applied plan's "facts: stash listed 53s ago" identical minutes apart, measured at the plan (CLI run 2 fault 4; calls 32, 33) | the tool's | fixed, `17663885` and `ded97bad`: "before the plan" with the plan's own age at the answer's time, on the apply, the plan and the audit view, the header's `(plan N old)` retired into it |
| An age over several things names the set, run 2 | "location list seen 57d ago" read as one listing's age where it is the oldest of the realm's lists (CLI run 2 fault 8; call 86 against 42) | the tool's | fixed, `593879fe`, `02fdef46` and `a805e002`: "location lists seen, the oldest 57d ago" or "no location list seen", both pinned; the gloss naming the two cases as `coverage::span` reads them |
| A count of none says which none and what to do (C146), run 2 | `tabs --limit 0`'s dangling "the 0 with the most items:" (CLI run 2 fault 14; call 52); `store events --summary`'s "0 events in the last 24 h" on a store whose newest event is 53 d old (call 38) | the tool's | fixed, `6483b9f6` and `54651bc2`: no table header at 0; the summary names the newest on record and the way to widen, only where the window is empty and never where a `limit` of 0 cut it, pinned as the newest against an older event |
| A job's record carries its time, and a failed fetch offers a way on | `job_status` and `job_result` with no submitted, started or finished time (MCP run 1 fault 1; run 2 fault 3; calls 8, 14, 19, 20, 23; 9, 16, 17, 22, 25, 62, 63; CLI run 1 fault 7, run 2 faults 5 and 7); a 404 as a sentence with no `kind` and no offer (CLI run 2 calls 91, 92) | the tool's; a protocol change | not fixed here: #165, filed by this seat |
| A job for a tab the stored listing does not name is refused offline | sent and failed on the wire (MCP run 1 fault 2, run 2 fault 2; calls 57, 59, 60; 60, 63, 23) | the tool's | not fixed here: #166, filed by this seat |
| A receipt says how long its undo holds | a create's undo stale after a replace, no receipt offering the way back to before the first write (MCP run 1 fault 6, run 2 fault 5; CLI run 1 fault 3, run 2 fault 11) | the tool's | not fixed here: #167, filed by this seat |
| A kind names the mistake, and a refusal naming several accounts gives a ready command | `not_logged_in` for a name never seen (MCP run 1 fault 7, run 2 fault 8; CLI run 1 fault 9, run 2 fault 12); the two-accounts refusal with no command in text (CLI run 1 fault 14; call 11) | C121's consequence, the fourth seat to meet it | not fixed here: #168, filed by this seat; raised for the owner |
| A page says how old its positions are, whatever its size | `shop_render` with `page` drops `freshness`, `left_out` and `policy` (MCP run 1 fault 9, run 2 fault 10; calls 50; 53, 54) | the tool's | not fixed here: #169, filed by this seat |
| What a tool reads invalid, a count counts | a tab name that reads invalid in no count and under no `--game-text` value (CLI run 1 fault 1, run 2 fault 3; MCP run 2 fault 6; calls 53, 69, 75; 59, 60; 41, 31) | the tool's | not fixed here: #170, filed by this seat |
| A receipt counts what the row decides | a tab price's receipt silent on how many items take it (CLI run 1 fault 2; calls 79, 84, 85) | the tool's | not fixed here: #171, filed by this seat (#48's shape, the league seat's) |
| A store and a league's tabs carry their own facts block | `store status` with no age at all, `tabs` with no league oldest or newest (CLI run 2 fault 1, MCP run 2 fault 9; calls 5, 36; 27, 30); both debriefs' first change on the CLI | the tool's; the subjects the group did not reach | not fixed here: #172, filed by this seat |
| A cross view names the leagues it holds with no items | three buckets where the scope lists eight leagues (MCP run 2 fault 11; call 38) | the tool's | not fixed here: #173, filed by this seat |
| A refusal offers the spelling the reader meant | `-priced` refused with `name:priced`, `"priced"` and `line(template:priced)`, the flag form `-has:priced` found only in `--describe=priced` (CLI run 2 fault 10; calls 66 to 68) | the tool's | not fixed here: #174, filed by this seat |
| The README says what a first write cannot undo, what a knob takes and what the mock does | a first policy irreversible with no word before it (MCP run 1 fault 10, run 2 fault 1; calls 11, 49; 12, 52); `ACQ_STORE_DIR`'s value unsaid (MCP run 2 fault 7; call 3); "never starts one" read as never (MCP run 2 fault 13; call 65) | the README's | fixed, `9ea28af9`: three sentences |
| Not faults | a job verb with no login starting a mock daemon and saying so (CLI run 1 fault 8, run 2 fault 6; calls 118; 104): C3's, kept as the league seat kept it; `eta_seconds: null` until a route is learned (MCP run 1 fault 12, run 2 fault 4; calls 12, 17; 14, 19): C40's honesty, the JSON says why; text receipts without `in_facts` (CLI run 1 fault 11; calls 79, 80, 82): C53's decision view, the help says so; no MCP tool lists refused bodies (MCP run 1 fault 11, run 2 fault 12; calls 29; 27): by design, `describe` says a person runs `store refused`; the mock's class and level differing between its list and its body (CLI run 2 fault 9; calls 16, 18) and its tab `index` 0 (calls 17, 19, 20): the mock's bodies; the store's growth on first open (CLI run 2 fault 13; calls 5, 36): the migration, as the league seat found; the README's "the first plan is the listings" where the runner's own fetches had made them (CLI run 2 call 27): the sentence holds for a league not yet listed | the runner's reading | not taken |

## For the owner

What this seat touched that is his: C3's daemon a refused job verb
starts under the mock, met by a fourth seat on the CLI and kept; C121's
`not_logged_in` for a misspelled account, met by a fourth seat on both
doors and this time filed as #168 (a kind that names the mistake, a
ready command in a refusal naming several accounts), his to close if
C121 bars it; C36's no-clear for a policy, kept, the README now saying
it before the first write; C53's decision view on text receipts,
kept. The agent-facing wording the rounds chose is in each commit's
message, listed for his later read in the group's close.

## What the README and the tools did not say (input, not faults)

That `tabs` counts tab items alone (now said); how to say "has this
line" in the query language (CLI run 2 call 49; `line("…")` is a sort
value); that `search --describe` with no argument is a 33.5 KB index
(CLI 43; #153's shape, a measurement); the default events window and
its newest-event age (CLI 38, now said); what `ACQ_STORE_DIR` takes
(MCP T1, now said); whether `ACQ_NO_KEYRING` must match between host
and daemon (MCP run 2 call 5: nothing complained); the query field for
life and that `count_res` counts chaos (MCP 34, 35); that `view.cross`
defaults to 10 cells (MCP 36); the words `residue`, `contract`,
`rails`, `membership live`, `derivation`, `intent`, `unruled_kind`,
`off_page`, `the game decides` (CLI run 2's "words I did not
understand"). The install, the Windows shells, the rails knobs, `web`,
`store import`, `diagnose`, `submit`, `demo`, `cancel` and the trade
import were read and never needed by either runner.

## The debriefs, routed

| Answer | Went to |
| --- | --- |
| "Which age did you quote for a set it was not the age of?" (both runs' 10): the store's oldest from ten characters, a route's header age as the limiter's, Standard's prices as every league's, the snapshot's age as the items', `tabs`' count as the league's | measured above (4); the third round's faults 1 and 3 where the words invited it; the rest the runner's own |
| "What would you change first?" CLI run 2 (7): an age on `store status` | #172 |
| MCP run 2 (7): a warning before the first `set_sync_policy`, or a clear | the README sentence (`9ea28af9`); C36 kept |
| "Did you knowingly do something irreversible?" MCP run 2 (11): yes, flagged after | the runner's, not the tool's; the brief's safe world |
| The store not as found: revisions past a clear (both 9) | by design (C35: a tombstone takes a revision), as the league seat routed it |
| The policy on `GERWARIC#7694` left in place (both 9) | the task's; the worlds torn down, the pristine untouched |
| "Which numbers did you derive yourself?" (both 6): body armours by base type, Rehearsal "the new one" from its start, empty cells from gaps, `~skip` as not for sale | the runner's own; "started 4d ago" is the tool's word now |
| "What did you never find out?" (both 8): what "location list seen" means | the third round's fault 3 |
| the ceiling run on an empty plan (CLI run 2 2, 3) | the runner's choice, the brief's way ("say how you would have"); the league seat's run 1 chose the other |
