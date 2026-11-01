# The league seat — the record

Phase 3's first seat: a blind agent at the league build's `acq` and
`acq-mcp` before the owner's live run of League 3.30, the seat the
`words` step (#20, #11, #101, #16, closed 2026-10-31) was waiting on.
Run under the briefs (`audits/league-2026-11/seat/BRIEF-cli.md` and
`audits/league-2026-11/seat/BRIEF-mcp.md` at `ea15df1d`, removed as
the seat skill's step 10 says; the staged removal rode in `93a0b7c0`,
the round-3 review's commit, whose message does not name it),
`PROMPT-cli.md`,
`PROMPT-mcp.md` and `PROMPT-debrief.md`. Two
sittings per run, the CLI door and the MCP door, each its own Opus
runner; run 1 on 2026-11-01, the fix round
(`roadmap/briefs/league-seat-fixes.md`, `847ad3f4..76f1c603`) the same
day, its cold review (`roadmap/reviews/league-seat-fixes-76f1c603.md`,
three findings: a plan-derived command spelling the plan's stale
account name, the covered leagues unbounded, a league name starting
with a hyphen read as an option) answered in `4b02018d..3c566df7` with
fault 1's rule carried to three sibling lines; then run 2 on the same
day, on the tree at `3c566df7`; run 2's three new faults fixed in a
third round, `98a1a741..418885f6`, reviewed cold
(`roadmap/reviews/league-seat-round3-418885f6.md`: one finding, the
two new sentences pinned in more than one place, answered at
`fbb7e433` with one golden each). The fates below
were decided by the
orchestrating session under the owner's standing word for the seat;
what touches a ruling is listed for him at the close.

## The worlds

Made 2026-11-01 under `~/seat-worlds/league-2026-11/`, outside the
repository and short-pathed for the socket. `pristine/` is `setup
--store` over the readiness seat's pristine
(`~/seat-worlds/readiness-2026-10/pristine/store`: one account,
`GERWARIC#7694`, provider `mock`, facts of 2026-09-02 to 09-11, 22,721
live items), so the owner's store was not read. `cli-opus/` and
`mcp-opus/` are each `setup --store pristine/store --daemon`, with
the brief as `BRIEF.md`, `README.md` as the build carries it, the door
and `bin/`. The MCP world's daemon was started by the world's maker, as
the README's agent section has a person do: `seat.py login seat --env
ACQ_IDLE_SHUTDOWN=14400` before the launch (its two calls set aside as
`setup-login.csv`), so the runner found a daemon up with a four-hour
idle exit. The CLI runner logged `seat` in itself through the door. A
first open of the pristine store grows it from 54.4 to 56.3 MB (the
build's migration on open; a second open leaves it), which the CLI
runner reported as unexplained growth. **The pristine stays**; a rerun
is `setup --store pristine/store` into a fresh world.

| Build | Tree | `acq` sha256 | `acqd` | `acq-mcp` | Worlds |
| --- | --- | --- | --- | --- | --- |
| `cargo build --workspace --release`, aarch64-apple-darwin | `43bb0958` | `6be24b9e…` | `aa6b4cf5…` | `da70b645…` | cli-opus, mcp-opus (run 1) |
| the same | `3c566df7` | `b80ff01d…` | `aa6b4cf5…` (unchanged: no daemon code moved) | `d456d2fb…` | cli-opus-2, mcp-opus-2 (run 2) |

The runner's startup context is the owner's global `CLAUDE.md` alone,
sha256 `e9c1329f…`, as at the readiness seat.

## The runs

Each launched by `.claude/skills/seat/launch.sh` in its world, the
debrief the same session resumed. Counts are the door's (`calls.csv`);
a report's own counts are the runner's, never recomputed here. Cost
is the session's at list price.

| Run | Model | Session | Calls | Cut | Turns | Cost | Files |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus (run 1) | `claude-opus-5-5` | `c5f3b5fc` | 111 | 0 | 78 + 2 | $3.53 + $0.16 | `calls-cli-1.csv`, `REPORT-cli-1.md`, `DEBRIEF-cli-1.md` |
| mcp-opus (run 1) | `claude-opus-5-5` | `f9165d79` | 71 | 0 | 89 + 2 | $3.86 + $0.20 | `calls-mcp-1.csv`, `REPORT-mcp-1.md`, `DEBRIEF-mcp-1.md` |
| cli-opus-2 (run 2) | `claude-opus-5-5` | `a6e5c136` | 97 | 0 | 114 + 3 | $3.71 + $0.21 | `calls-cli-2.csv`, `REPORT-cli-2.md`, `DEBRIEF-cli-2.md` |
| mcp-opus-2 (run 2) | `claude-opus-5-5` | `f4cdc239` | 64 | 0 | 83 + 2 | $3.83 + $0.21 | `calls-mcp-2.csv`, `REPORT-mcp-2.md`, `DEBRIEF-mcp-2.md` |

cli-opus: all nine tasks finished, no denial, no file opened outside
the two (the harness saved call 53's 48.8 KB `--describe` to a file of
its own, which the runner did not open); T2's ceiling run had nothing
left to plan on an empty league, so the runner lowered the policy's
age to 60 s, revision-guarded, and restored it (calls 36, 50), which
its debrief names as a change the person did not choose; every price
it wrote was cleared and read back (calls 84 to 88); the daemon left
stopped (call 111). mcp-opus: all nine tasks finished; two door
commands refused by the rails (a shell variable, an `apply_plan`
heredoc, sent again as drafts); the harness saved the `open` output
and the runner read it there, named in its "Left out"; the tab's undo
sent as its receipt printed (call 58), the item's clear built from the
refusal (call 60), every count back to call 41's (call 61); the daemon
never went away, so the no-daemon words were not seen. Its debrief
retracts its reading of `policies_known` as sync policies (rate-limit
policies, call 71).

cli-opus-2: all nine tasks finished; one report heredoc refused by the
rails, sent as a draft; no file opened outside the two (the harness
kept call 39's 49.8 KB `--describe`, unopened); the plan file's ceiling
and second hand-over met an empty plan (calls 32, 33), the runner
making no work for it; every price cleared and read back (calls 74 to
76); the daemon left stopped (call 97). Its debrief marks the store's
growth and "the new one" as inferences. mcp-opus-2: all nine tasks
finished; two heredocs refused, sent as drafts; the `open` output read
from the harness's file, named; the stale undo recovered through the
conflict's offer (calls 54, 55); every count back to call 39's (call
56); the daemon never went away. Its debrief retracts a date it worked
out from an epoch (call 38) and names the MCP-only stumbles (a `line`
key's spelling, no leagues read, no `store refused`, the CLI's words
inside an MCP refusal, call 54).

## What this seat measured

1. **Calls per task**, T0 to T8: run 1, CLI 7, 13, 22, 9, 11, 17, 8,
   11, 12 (111, with the door's login), MCP 2, 3, 18, 7, 10, 16, 7, 6, 2
   (71); run 2, CLI 5, 13, 14, 9, 10, 16, 8, 11, 10 (97), MCP 3, 2, 18,
   7, 8, 13, 6, 5, 2 (64). The MCP's `open` 36,104 bytes (run 1) and
   36,101 (run 2), against 31,472 at the readiness seat.
2. **Calls to the first answer the runner held right**, within the
   task from the call its report names: CLI T0 –, T1 2 (call 9), T2 3
   (the write, call 24) and 6 (the plan, 27), T3 1 (38), T4 4, 8, 10,
   11 for (a) to (d) (56, 60, 62, 63), T5 2 (65) and 12, 15 for the
   writes (75, 78), T6 2 (82), T7 1 (89), T8 1 (100); MCP T0 –, T1 1
   (3), T2 3 (the leagues, 8) and 4 (the policy, 9), T3 1 (24) and 4
   for the ages (27), T4 2, 7, 8, 10 (32, 37, 38, 40), T5 1 (41), T6 1
   (57), T7 1 (64), T8 1 (70). Run 2: CLI T1 1 (call 7), T2 4 (the
   write, 23) and 9 (the apply, 28), T3 1 (34) and 9 (the ages, 42), T4
   3, 9, 10 for (a), (c), (d) (45, 51, 52) and 7 for (b) held open
   (49), T5 2 (54) and 3, 6 (the bad notes, 55, 58), T6 2 (70), T7 1
   (77), T8 1 (88); MCP T1 1 (4), T2 1 (no policy, 6) and 17 (the task
   whole, 22), T3 2 (25), T4 2 (32), T5 1 (39), T6 1 (52), T7 1 (58),
   T8 1 (63).
3. **The league loop.** Both runners wrote the first policy in the
   create-only form from the help or the tool text alone (CLI call 24,
   MCP call 9), planned and applied until nothing was left (CLI calls
   27 to 31, MCP 11 to 23), and on the CLI applied a plan file under a
   ceiling, were refused over the ceiling with the count (call 47), and
   handed the spent file back to "every request of this plan is spent"
   (call 49) and, after the policy moved, to "the plan cites revision 2,
   but revision 3 is stored" (call 52). No plan gave a time: the mock's
   first morning has the route's limit unlearned (`eta_seconds: null`,
   MCP calls 11, 17) and the CLI's quote said "ready" (call 27).
4. **The `words` step's four fixes, met without being named.** #20:
   the install's checksum lines were read at T0 by both and run by
   neither (no `curl`; the CLI runner lists them under "never
   needed"). #11: `--game-text invalid_note` and `residue` named the
   items (CLI calls 66, 68; MCP 42 to 44); the scope's oldest and newest
   fetch places answered "from where" outright (CLI call 42, MCP call
   27); the three keys' sentences were not reported either way. #101:
   `--if-revision 0` / `if_revision: 0` as the create-only form on both
   surfaces (CLI call 24, MCP call 9); a failed job's report on stderr
   (CLI call 96); the `describe` sizes read as compact JSON's (MCP's
   T4). #16 and #146: every printed undo and offer that was run as
   printed ran (CLI calls 85, 95; MCP 58, 64); the one refused (CLI
   call 83) was stale, not misquoted.
5. **Fault 4's cost**, measured on a scratch copy of the pristine on
   the fix round's release build: an item's `price set` 106 ms warm and
   559 ms cold against 5 ms before (the listing is resolved to tell a
   substash); `price clear` 5 ms either way.

## The faults, by the rule each expresses

Run 1's, read against the calls they cite before the fix brief: CLI
calls 13, 16, 24, 26, 27, 28, 40, 43, 44, 48, 49, 52, 57, 58, 65, 69,
76, 78, 83, 84, 98, 99, 109 (twenty-three) and MCP calls 25, 26, 49,
53, 59, 67, 68 (seven); the store's growth reproduced on a scratch
copy. Run 2's, read before their fates: CLI calls 22, 32, 33, 54, 61,
70, 94 (seven) and MCP calls 32, 37, 39, 47, 49, 52, 54, 55, 64
(nine). **Run 2 met none of run 1's ten fixed faults again**: every
printed next step it ran landed (CLI calls 23 to 31, 70 to 76), the
stale undo was recovered through the conflict's words on the CLI (71,
72) and its offer on the MCP (54, 55), the uncovered-league refusal
was not provoked (both named `--league`), and the substash note was
not met (the runners priced tab items).

| Rule | Faults (calls) | Whose | Fate |
| --- | --- | --- | --- |
| A printed command runs as printed (C156) | `policy set`'s, `refresh --plan`'s, `refresh --apply`'s and `tabs`'s next steps name no account, and `policy set`'s no league (CLI 24, 26, 27, 28, 43, 44, 48, 49, 52); the CLI runner's worst, its debrief's first change | the tool's | fixed, `847ad3f4`: every printed next step names `--account`, `--realm` and `--league` through the refusal's `Way`, one spelling with the offers; `seat_faults.rs` `every_printed_next_runs_as_printed_with_a_store_only_account_known` |
| A refusal says what to do (C11) | a plan for a league the policy does not cover names no covered league and offers none (CLI 26) | the tool's | fixed, `bc6bafe8`: the refusal names the covered leagues and offers each plan, both surfaces |
| A conflict offers the write at the revision it names | "re-read and retry" with no offer (CLI 83, MCP 59); over a cleared row "a new price names no revision" against the receipt's `--if-revision 0` (CLI 84, 98, 99; MCP 68); after a replace no printed command removes the row (CLI 76, 78, 83, 84; MCP 51, 53, 59, 60) | the tool's; the MCP's second | fixed, `4c8bc17d`: a conflict offers the same write at the stored revision, or at 0 over a cleared row, both surfaces; C52 kept (the refusal carries the row, and the offer names the revision); raised for the owner at the close |
| A write says what the shop will do with it | a substash item priced silently, blocked at render (CLI 76, 82; MCP 51, 57) | the tool's | fixed, `81c40943`: the receipt's `note` carries the shop's rule words, both surfaces; the cost measured above |
| A count's words claim no more than the count knows | "(no effect: the tab's name applies)" for a tab that reads none (CLI 65, 69); the apply's "what this apply revealed … or left undone" for requests gone stale (CLI 48, 49) | the tool's | fixed, `e8aabdac` and `9fa058eb` |
| A name is redacted where it stands whole | `~/seat-worlds` printed as `~/<account 2>-worlds` (CLI 109) | the tool's | fixed, `39dad372`: a hyphen joins a word |
| A refusal's spelling is one the tool takes back | `--describe sum(` refused after the refusal spelled `sum(` (CLI 57, 58); `realm_needed` names `all` and offers it nowhere (CLI 40, MCP 25) | the tool's | fixed, `9a5949fa` and `1b1f2cab` |
| The README claims no more than the tool does | "exits an hour after its last connection" (MCP 5: 14400 s); every `--json` failure has `kind` and `offers` (CLI 97: a job's outcome record has neither); "A new league" says nothing of the first policy's create-only form (CLI 24, MCP 9) | the README's | fixed, `76f1c603` |
| A failed job's record is a document a program can act on | a 404 on a tab as a sentence, no `kind`, no `next`, the body unseen (CLI 96, 97; MCP 65 to 67); the MCP runner's worst | the tool's | not fixed here: #7, this seat's calls added |
| A receipt counts what the row decides | a tab's `price_set` says nothing of the 50 items whose notes beat it (MCP 49, 54, 56; CLI 80) | the tool's | not fixed here: #48, this seat's calls added |
| What a tool counts, a tool lists | `refused_bodies: 4` with no MCP listing (MCP 24, 30) | the tool's | not fixed here: #18, this seat's call added |
| A document is bounded by default | the MCP's `describe` has no index form and its computed names were guessed (MCP 35, 36); the CLI's bare `--describe` is a 402-line, 48.8 KB index (CLI 53) | the tool's | not fixed here: #153, filed by this seat; `open` 36,104 bytes, a measurement on #104 |
| A kind names the mistake | a typo'd account refused as `not_logged_in` (CLI 95, MCP 64), the third seat to meet it | C121's consequence | kept as designed; raised for the owner at the close again |
| A job verb with no login starts a daemon | a refused `leagues` left a daemon running and said so (CLI 107, 108) | C3's consequence | kept, as the release seat kept it |
| A printed command runs as printed, the first policy's create-only form (run 2) | `policy show` with no policy offered `acq policy set '{…"Standard"…}'` with no account and no `--if-revision 0` (CLI 22); the CLI runner's worst | the tool's | fixed, `98a1a741`: the offered first write is the create, as its account |
| The read-back names what the next write names (run 2) | `price show` on a target with no row printed no revision while `price set --help` says to name what `price show` printed (CLI 61, 62; run 1's 74) | the tool's | fixed, `aadde24d` |
| The create-only refusal says what it refused (run 2) | `policy set --if-revision 0` over a stored policy: "at revision 1 (re-read and retry)" when the refusal was the point (CLI 24) | the tool's | fixed, `418885f6`: "a sync policy stands at revision 1; 0 writes only where none is stored: name 1 to replace it", both surfaces |
| An answer's ages are the answer's (run 2) | a search answer's `scope.oldest_fetch` and `newest_fetch` read the same for 187 matches and for all 22,721 items (MCP 32, 37); the MCP runner's worst | the tool's | not fixed here: #155, filed by this seat |
| A page says how old its positions are (run 2) | `shop_render`'s `freshness` is `policy: not_set` and empty while the listing its ranks use is 50 days old (MCP 52, 39; CLI 70) | the tool's | not fixed here: #156, filed by this seat |
| A fetched list is read without a daemon (run 2) | no tool reads the stored league list, and nothing marks the league the policy does not cover (MCP 7 to 9; CLI 8) | the tool's | not fixed here: #157, filed by this seat |
| A verb says which leagues it does not read (run 2) | `price status` and `shop render` read Standard and name no other league the store holds (CLI 54, 70, 56) | the tool's, by the README's "Standard unless you name a league" | not fixed here: #19, this seat's calls added |
| An MCP answer speaks the MCP's words (run 2) | the conflict's new sentence carries the CLI's read-back with the spelling table's gloss after it (MCP 54) | the tool's; the round's wording | not fixed here: #97, this seat's call added |
| Not faults | the tab `index` 0 in a fetched body against the listing's 1 and 3 (CLI 13, 16): the mock's body; the store's growth (CLI 5, 38): the migration on first open; a plan's missing time on a fresh route (MCP 11, 17): the route's limit is unlearned until its probe, and the JSON says so; `policies_known` (MCP 5): rate-limit policies, as its description says, retracted by the runner; `sync_policy` for an account with none (MCP 6): `{"policy": null}` is the answer; an unknown count key's refusal with no offers (MCP 26): the keys are listed and `realm` is the scope; run 2: a job verb with no login starting a mock daemon as a contradiction of the README's "refuses to start a daemon against GGG" (CLI 94): the sentence is about GGG, and the mock's spawn is C3's, kept above; an empty plan file applied twice answering the same words (CLI 32, 33): a plan with no actions is applied as nothing (C111), and the JSON says `applied: false`; the mock's character level and class differing between its list and its fetch (CLI 10, 15): the mock's bodies; `never_fetched_by_type` holding fetched counts under its name (MCP 25): C146 bars the rename, the entry's sentence is `describe`'s; "Begin with search and no arguments" refused on two realms (MCP 1, 24): the sentence goes on to name `scope.realm`, and the `all` offer now recovers it in one call (25) | the runner's reading | not taken |

## For the owner

What this seat touched that is his: C121's `not_logged_in` for a
misspelled account, met by a third seat (both runs, both doors) and
kept; C52 beside the conflict's offer (`4c8bc17d`: the refusal carries
the row and the offer names the revision, so the agent has read what it
writes over, judged within C52's words); the policy conflict's missing
offer (a wrong nonzero revision on `policy set` names the revision and
offers nothing, left out of the round by its brief); the daemon a
refused job verb starts under the mock (C3, kept as the release seat
kept it). The agent-facing wording the rounds chose is in each
commit's message, listed for his later read in the close.

## What the README and the tools did not say (input, not faults)

That job verbs and store verbs pick a default account differently and
`refresh --apply` picks as a store verb (CLI calls 10 to 19, 28; C51,
kept visible since the whole-CLI seat); that `refresh --plan` without
`--league` plans Standard (CLI 26, now named by the refusal); a plan's
time only as `eta_seconds` per scope in JSON (CLI 46); the computed
names (`pseudo.count_res`) reachable only through a refusal's list
(CLI 58, 59; MCP 35: #153); `tabs` per league with no all-leagues view
and `characters` with no `--league` (CLI 12, 43); what `ACQ_STORE_DIR`
is set to for a host (MCP T1: the door sets it on every call, so the
README's bare configuration was not measured, as at the readiness
seat); whether a re-login must carry the same knobs (MCP T8, the
runner's untested command); that `never_fetched_by_type` lists
characters under a `tabs` count (MCP 27; C146 bars the rename); the
words `unlifted_items`, `withheld_responses`, `contract`, `rails`,
`tripwire`, `derivation` (CLI 90, 100; MCP's "words I did not
understand"). The install, the Windows shells, the rails knobs, `web`,
`dash`, `store import`, `submit`, `demo`, `cancel` and the trade import
were read and never needed by either runner.

## The debriefs, routed

| Answer | Went to |
| --- | --- |
| "Did you change the person's settings without their consent?" (CLI 10): the 60 s policy to make work for the ceiling run | the runner's, not the tool's: the brief's "if nothing is left to plan by then, say how you would have" was the way; noted above |
| The policy named only the new league, the case the README warns drops Standard (MCP 10, CLI 3) | the README's "A new league" line stands (`76f1c603` adds the first policy's form before it); the choice was the person's words, "keep the new league fresh" |
| Every printed next step with its account first (CLI 7) | `847ad3f4` |
| A failed job's `kind` and `next` first (MCP 7) | #7 |
| The store not byte-for-byte as found: revisions count past a clear (both 9) | by design (C35: a tombstone takes a revision), as the release seat routed it |
| "Which numbers did you derive yourself?" (both 6): "life" as the maximum-life line, the (none) rarity as currency and cards, "Rehearsal is the new one" from its category | the runner's own; the tool never labels a league new (the README's "`acq leagues` names it the day it opens" is the day's word) |
