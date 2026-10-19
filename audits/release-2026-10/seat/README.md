# The release seat — the record

Step 4 of the release (`RELEASE-SLICE.md`; the brief `RELEASE-PLAN.md`
is whole at `d0ab421e`), run under the briefs `BRIEF-cli.md` and
`BRIEF-mcp.md`, the launch prompts `PROMPT-cli.md` and `PROMPT-mcp.md`
and the debrief's `PROMPT-debrief.md`, committed at `26603d0f` and
revised after reviews 5 to 7 at `81e6e9bb` and `10902dbb`, each
removed at the close. Closed 2026-10-19. A person new to the tool, from
the README alone, on the release build unpacked from an archive of the
release's shape, in mock worlds made from a copy of the owner's store:
nine tasks on the CLI (install, log in and a daemon for an agent, what
I have, find things, prices, the shop, for a program, when things go
wrong) and eight on the MCP (the host's configuration in place of the
install and the login; the daemon as "I have not started anything for
you"). The whole-CLI seat's record is `../../rehome-2026-10/seat/`,
the trade seat's and the re-sit's `../../trade-2026-10/`.

## The worlds

Made 2026-10-19 under `~/seat-worlds/release-2026-10/`, outside the
repository and short-pathed for the socket. `pristine/` is `setup
--facts GERWARIC_7694.db` with no daemon running (the owner's files
read once through sqlite's online backup, unchanged since 2026-10-15),
made with `bin-setup/`'s `acq` (`2c743c24…`) and `copy-as-store`
(`6fbe348d…`). Each run's world is `setup --store pristine/store
--archive <the macOS archive>`, the CLI's with `--daemon`, and holds
its brief as `BRIEF.md`, `README.md` as the release's commit carried
it, its door, `bin/` unpacked from the archive by the door (the
archive's sha256 and each binary's in `world.json`) and the store.
The archive is `tools/release-package.sh` over `cargo build --release
--target aarch64-apple-darwin`, with placeholder third-party notices;
`tools/release-smoke.sh` passed on each.

| Archive | At | sha256 | `acq` | `acq-mcp` | `acqd` | Worlds |
| --- | --- | --- | --- | --- | --- | --- |
| `acquisition-1.0.0-beta.1-aarch64-apple-darwin.tar.gz` | `e2dbeb8f` | `1dd7906b3f04b3e9d6efd6561691b920ba63a3f8322d4a6f31f5a19e764ff26d` | `c027b225…` | `7f390c3a…` | `45610ff4…` | cli-opus, mcp-opus, cli-sonnet, mcp-sonnet |
| the same name | `b8820b35` | `3a95fa98f029bbb84fa2787a8f6bf2290fa9d68eb2d6304de576ab87bab3480f` | `08bbe6e2…` | `83f2934a…` | `4fa27e01…` | cli-opus-2, mcp-opus-2 |

A runner's startup context is the owner's global `CLAUDE.md` alone,
sha256 `e9c1329f…` at every launch, as the re-sit's; it names nothing
of Acquisition.

## The doors' cut

Both doors print a result whole up to 60,000 bytes and past that its
first 3,000 characters, saying so, and mark the row `head` in
`calls.csv`. The cut is the door's, never the tool's: a fault that
rests on a `head` row was read against the whole output in the world's
`calls/` before it was the tool's. The MCP door starts one `acq-mcp`
per call as a host would, from the configuration the runner wrote
under `work/`, and kills a stalled server or login at a deadline,
logging the kill as a call.

## The runs

Each launched by `.claude/skills/seat/launch.sh` in its world, the
debrief the same session resumed; the reruns on the second archive
after the fix round, in fresh worlds, never continued from the first.
Counts are the door's (`calls.csv`); a report's own counts are the
runner's, never recomputed here. Cost is the session's at list price.

| Run | Model | Session | Calls | Cut | Turns | Cost | Files |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus | `claude-opus-5-5` | `4592effd` | 107 | 2 (`shop render --expand` 3.5 MB, `--json` 7.4 MB) | 76 + 3 | $3.55 | `calls-cli-opus.csv`, `REPORT-cli-opus.md`, `DEBRIEF-cli-opus.md` |
| mcp-opus | `claude-opus-5-5` | `11891c2e` | 51 | 2 (`price_list` 67 KB, `shop_render` 5.1 MB) | 66 + 3 | $3.46 | `calls-mcp-opus.csv`, `REPORT-mcp-opus.md`, `DEBRIEF-mcp-opus.md` |
| cli-sonnet | `claude-sonnet-5-5` | `becbb2ab` | 97 | 2 (`price show --json` 174 KB, `shop render --json` 7.5 MB) | 112 + 2 | $1.50 | `calls-cli-sonnet.csv`, `REPORT-cli-sonnet.md`, `DEBRIEF-cli-sonnet.md` |
| mcp-sonnet | `claude-sonnet-5-5` | `b8abe4b8` | 55 | 2 (`price_list` 67 KB, `shop_render` 5.1 MB) | 71 + 2 | $1.48 | `calls-mcp-sonnet.csv`, `REPORT-mcp-sonnet.md`, `DEBRIEF-mcp-sonnet.md` |
| cli-opus-2 | `claude-opus-5-5` | `cbf8834a` | 107 | 2 (`shop render --json` 208 KB, twice) | 77 + 2 | $3.24 | `calls-cli-opus-2.csv`, `REPORT-cli-opus-2.md`, `DEBRIEF-cli-opus-2.md` |
| mcp-opus-2 | `claude-opus-5-5` | `dd922bd9` | 56 | 1 (`shop_render` 160 KB) | 74 + 2 | $3.21 | `calls-mcp-opus-2.csv`, `REPORT-mcp-opus-2.md`, `DEBRIEF-mcp-opus-2.md` |

cli-opus: one door command denied by the rails (a `report` heredoc),
sent again as a draft; the harness saved call 41's output to a file of
its own, which the runner did not open; all nine tasks finished; the
daemon left stopped. Its debrief retracts the cause it gave for the
7.4 MB shop JSON (inferred from the size, the head alone read), "every
time" on the undo fault (more than one account known), and three
inferences of its own.

mcp-opus: two door commands denied by the rails (a date script outside
the door, and a `call` heredoc, sent again as a draft); the harness
saved call 1's output (`open`, 42 KB) to a file of its own, which the
runner read and named in its "Left out"; all eight tasks finished,
every price it wrote removed again (call 51 against call 19). Its
debrief marks its T2 dates as hand arithmetic, its left-out counts as
resting on the cut part of call 32, and two remarks on the CLI as
untested.

cli-sonnet: one door command denied by the rails (a `report` heredoc),
sent again as a draft; no file opened outside the two; all nine tasks
finished, no daemon left running (call 97); the prices it wrote read
back as before (calls 65 to 67).

mcp-sonnet: one door command denied by the rails (a `report` heredoc),
sent again as a draft; the harness saved call 1's output to a file of
its own, which the runner read and named in its "Left out"; all eight
tasks finished, every price it set cleared and read back (calls 35 to
39, 53, 55).

cli-opus-2: no denial, no file outside the two, all nine tasks
finished, every price cleared and read back, the daemon left stopped.
Its debrief marks the shop JSON's contents as unseen (the cut) and the
mock's invented character (calls 89 to 90) as the mock's.

mcp-opus-2: two door commands denied by the rails (a `date` loop
outside the door; an `apply_plan` heredoc it did not send again as a
draft, so T6's apply is untried); the harness's saved copy of `open`
read, named in its "Left out"; every price cleared and read back. Its
debrief retracts four T5 inferences made from positions and
arithmetic, the left-out list unseen behind the cut.

## What this seat counted

1. **The re-reads the read economy park asked for.** Two counts, kept
   apart. *A second call for the same answer:* every `calls.csv` was
   read whole by a script that paired each repeated command with its
   earlier twin and compared the outputs in the world's `calls/`, ages
   and clocks aside; a repeat the brief asks for is excluded (the MCP's
   second `open` through the host configuration; the read-back after a
   write and after the cleanup), as is a check the daemon task asks
   for (`daemon status` after a stop or a verb that might start one).
   What is left: cli-opus 1 (`tabs --type DelveStash`, calls 57 and
   95); mcp-opus 0; cli-sonnet 0; mcp-sonnet 0; cli-opus-2 2 (`version`,
   calls 6 and 104; `store status --account`, calls 27 and 95);
   mcp-opus-2 1 (`store_status`, calls 3 and 5). Four across 473
   calls. *A reported "read twice"* (each debrief's answer 2): five or
   six per run, laid by the runners to five causes, each named by more
   than one run — `price_status`'s overlapping counts and its
   `residue` (mcp-opus, mcp-sonnet, cli-opus-2, mcp-opus-2); the shop
   page's `StashN`, a rank and not a tab's name, matched to a tab by
   counting links (cli-opus, cli-sonnet, cli-opus-2, mcp-opus-2); the
   search scope's "newest 4d ago" beside tabs fetched 41 days ago
   (cli-opus, cli-sonnet, cli-opus-2); the tool texts' and the help's
   length (mcp-opus, mcp-opus-2, cli-opus-2); the terms block (cli-sonnet,
   mcp-sonnet; the park in `decisions/frontends.md`). Of the first
   four runs' causes the fix round answered one, `price list`'s
   columns (`a6d66df0`); the rest are parked, below.
2. **The README's lines of unrecorded guidance.** The agent's recipe:
   confirmed as the README prints it, before the fix round with "stop
   the running one first" as the gap (cli-opus calls 22 to 25,
   cli-sonnet 15 to 18) and after it as printed (cli-opus-2 calls 19
   to 23, the new daemon at 3600 s). The macOS archive's binaries: ran
   as the door unpacked them, every run; the install line itself, the
   Windows and Linux lines and the data paths were never reached here
   and are the platforms slice's VM rows and CI's smoke; the quarantine
   line is the owner's at the tag.
3. **Pricing end to end, at what cost.** Every run set two prices,
   replaced one naming its revision, rendered the shop with them in
   place and cleaned up with a read-back. The pricing task's calls:
   cli-opus 19, cli-sonnet 25, cli-opus-2 22; mcp-opus 12, mcp-sonnet
   13, mcp-opus-2 16. The shop task's: 10, 6, 7 and 6, 7, 6. The
   reruns' replace landed first time on both surfaces with the kind
   and the revision the fix round gave (cli-opus-2 calls 61 to 66,
   mcp-opus-2 calls 30 to 36).
4. **Differences.** Between the surfaces: the undo lines and the
   account are the CLI's faults alone (the MCP names the account in
   the call); the no-daemon refusals the MCP's alone (the CLI's job
   verbs start one, C3). Between the models: the Sonnet runs finished
   every task at under half the cost (the table) and met every fault
   the Opus runs met; what they met alone is the cleared row's words
   (cli-sonnet call 78, mcp-sonnet call 54, fixed at `b24e9e05`) and
   `agree: 0` (no fault); both MCP Opus runs read the cut shop's
   counts off the visible policy cells by arithmetic and retracted it
   in their debriefs, where mcp-sonnet reported the cut as lost
   (its debrief 1).

## The faults

One row per fault, merged across the runs and the triage of
2026-10-19 (`RELEASE-SLICE.md` at `d0ab421e`, "The fix round's
triage", every fate as proposed and ruled by the owner: "yes, go
ahead"); "met by" cites each run. Every row that became a commit was
reproduced in a scratch copy of the pristine store on the release
build or read against the calls it cites before it was believed; a
parked row rests on the runner's calls as cited.

| Fault | Met by | Fate |
| --- | --- | --- |
| The shop render whole is 5 MB on the MCP, 7.5 MB as `--json`: `left_out` lists every item nothing applies to; `page` drops the omissions and freshness | all four; the reruns at 208 KB and 160 KB | `add1088a` (what is left off for a reason listed, what nothing applies to counted, listed under `expand`), `d79c4ca7` (`left_out` bounded at 50, the blocked first, `left_out_total`); `posted` and `freshness` still unbounded: parked (`decisions/pricing.md`) |
| `price list` has no limit (49 to 67 KB cut); a tab's `price show` carries every item (22 to 174 KB) | mcp-opus, mcp-sonnet, cli-sonnet | `27cfd3c7` (`--limit`, `total` and `listed`); the container's show parked (`decisions/frontends.md`) |
| A revision conflict, a cleared row and a missing target refuse without a `kind`, the revision in prose, the store's words | all four | `b24e9e05`: `kind: "conflict"` with `revision`, the verb's words (C11, C116 amended at `91cdd4fa`) |
| The job tools' no-daemon refusals are JSON-RPC errors that say what is wrong, not what to do | mcp-opus, mcp-sonnet | `f23b10bb` (the README's line named), `b8820b35` (`NoDaemon`, one refusal document), `db5126fe` (the remedy names the provider and store the server runs with: mcp-opus-2 calls 45, 47), `be4ba120` (every tool's refusal a result marked `isError` with a kind: mcp-opus-2 calls 46, 50) |
| The instructions' first step, a bare `search`, refused on a store of two realms | mcp-opus, mcp-sonnet | `f23b10bb` |
| The CLI's undo lines: no `--account`, so refused once a second account is known; none after a first write; a clear's names no revision; the target unquoted; the account unnamed with one known | cli-opus, cli-sonnet; review 8 | `5e2daaa0`, `b8820b35`: every printed undo runs as printed (`seat_faults.rs`) |
| A cleared row takes a revision, said nowhere | mcp-opus, mcp-sonnet, cli-sonnet | `b2b64bbd` |
| A job verb with no login starts a daemon, idle thirty minutes under the door, and says "not logged in" alone | cli-opus, cli-sonnet | kept, C3: the refusal says a daemon started and when it exits (`f23b10bb`); the spawn-only-when-a-login-would-restore alternative parked (`decisions/daemon.md`) |
| The README's recipe says "while none is running" and not how to stop one, nor that stopping logs you out without a keyring | cli-opus, cli-sonnet, mcp-sonnet | `e8f21169` |
| The README names eleven tools and "four more", shows no `env`, points at files the archive lacks, names `SHA256SUMS` with no step, the paths unnamed, the shop page's destination unsaid; its sample configuration the stand-in's | all four; mcp-opus-2 | `e8f21169`, `31ab4bf4` |
| `auth --no-browser --json` prints two JSON documents | cli-opus | kept: the URL must print before the login completes; the help says so (`f23b10bb`) |
| A 404 on a stash id is handed on raw | cli-opus | parked (`decisions/daemon.md`): invariant 2 |
| Job verbs print raw JSON; `version`'s line hides the provider; `price list` has no column headers; `item_events` oldest first; `characters` with two counts unexplained; `store_status` "leagues: 0" beside eight named | one run each | job verbs kept (C53); the rest `f23b10bb` and `a6d66df0`; the characters text at `b8820b35` |
| The MCP's ages are epoch seconds; no tool lists refused bodies; the refusals name CLI flags | the MCP runs | parked, each with a trigger (`decisions/frontends.md`); every run recovered through the offers |
| `price_status` reports `agree: 0` beside 489 game-priced items | mcp-sonnet | no fault: the data (checked on a scratch copy) |
| A named `--limit` printed a group over ten as a header alone | cli-opus-2 calls 51, 53, 54 | `ce5b2d30` |
| `price status --league`'s next step dropped the league | cli-opus-2 call 48 | `ce5b2d30` |
| `acq daemon --help` calls its verbs debugging only, against the README's recipe | cli-opus-2 call 92 | `ce5b2d30` |
| The clear's receipt and the help use "revision" in two senses | cli-opus-2 calls 45, 69, 84 | `ce5b2d30` |
| The staleness line's oldest against the coverage line | cli-opus-2 call 68 | no fault: the oldest is among the covered stale items |
| A clear's receipt carries no revision; `manual.revision` on an inherited row invites a wrong `if_revision`; `never_fetched_by_type` lists every tab | mcp-opus-2 calls 39, 31, 7 | parked, each with a trigger (`decisions/frontends.md`) |
| Review 8, on the fix round: the signing repack's bare `tar`; the no-daemon refusals still errors; `--limit 0`'s words; the undo's account and quoting; a retired currency's prior promised back; the characters text | the outside review | `b8820b35` |

Not the tool's: the rails refused a heredoc to five of the six runs
(drafts took them, as the door provides); each MCP harness saved the
`open` call's output to a file of its own, which the Opus runners read
and named; the mock invents a character for any name, so an API
not-found was never seen (cli-opus-2 calls 89, 90).

## Calls read

By the orchestrator, against each report, before a fault was believed:
cli-opus 22, mcp-opus 9, cli-sonnet 6, mcp-sonnet 8, cli-opus-2 9,
mcp-opus-2 7 (`RELEASE-SLICE.md` at `d0ab421e` names them per run).
Every `calls.csv` was read whole for its counts, and every repeated
call's two outputs were compared by the script the first count above
describes; the four `daemon status` outputs of cli-opus's last task
were read by hand (calls 101 to 107).

## The debriefs, routed

| Answer | Went to |
| --- | --- |
| "What did the world's setup do that a person's wouldn't?" — nothing printed says which knobs are in effect (cli-opus 9) | `daemon status` reports the idle exit it was started with (`2044a994`) and `version` the provider (`f23b10bb`); the rest dropped, the door's world being the door's |
| "What did you do that a careless agent would get wrong?" — on a tab whose notes beat its row, `price_show`'s `why` alone says so (mcp-opus 9) | no fault: the relation rule (C69); `price status` counts the conflicts |
| "Would you hand this tool to a person, and what would you tell them first?" — the daemon and the login lost on stop (cli-sonnet 9) | the README's recipe (`e8f21169`) |
| "Did you trust each number?" (mcp-sonnet 9) | the debrief's question 5 asks it; dropped |
| "Did you change the store in any way you did not undo?" — a cleared row keeps its revision and the intent basis moved (cli-opus-2 9, mcp-opus-2 9) | by design (C35: a tombstone takes a revision); the question added to the seat skill's debrief, proposed unasked by both reruns |
| "Did your writes appear where a buyer would see them?" (mcp-opus-2 9) | by design (C74: the render posts nothing); dropped |
| The tool's own text was the best documentation; the README should point to it (mcp-sonnet 9) | the README's tour opens with `acq <verb> --help`; dropped |
| `price_status`'s `residue`, `store status`'s `granted`, `unlifted`, `withheld`, `intent`, never understood (cli-opus 8, mcp-opus 8, mcp-sonnet 8, cli-opus-2 8, mcp-opus-2 8) | parked (`decisions/pricing.md`, `decisions/store.md`) |
| The invalid notes and the refused bodies, never listed (every run's 8) | parked (`decisions/pricing.md`; `decisions/frontends.md`) |
