# The trade import's seat — the record

Step 8 of `search/TRADE-PLAN.md`, closed 2026-10-17. It ran under its
plan, `PLAN.md` at `18a3d7b4`; the briefs `BRIEF-cli.md` and
`BRIEF-mcp.md` at `3e84db17`, `TASKS.md` at `157f7078` and
`DEBRIEF-QUESTIONS.md` at `538022d6`, each removed at the close. Two
rounds of four runs — Opus and Sonnet, CLI and MCP — each runner a
headless session in a world outside the repository, through one door
(the seat skill); review 2, from outside, reads this record next.

## The worlds

Made 2026-10-17 at `538022d6`, outside the repository under
`~/seat-worlds/trade-2026-10/`. The store is the owner's `GERWARIC`
facts file and its intent file, copied once with the daemon stopped
into `pristine/`, which nobody runs in; every run's world is copied
from it (`seat.py setup --store`). A world holds its brief (as
`BRIEF.md`), `TASKS.md`, the tour (CLI: README's "The tour", as
`TOUR.md`), its door, its binary under `bin/` and the store.

| Binary | sha256 |
| --- | --- |
| `acq` (release) | `eb258f2e849dbe64c80bf68502f8a87f9298dd0a1ad83ebf8c628d8541122041` |
| `acq-mcp` (release) | `02cbf760dff336bcebf1db7381a8434860f38c89675760c51a70f170d1ae7b79` |
| `copy-as-store` (release; made `pristine/`) | `6878430c6d35a5fc290f0f16b34a747099b5017a820a8a97ad57d2ebe3ed8012` |

The second round's, built at `80c1de6d` after the fix round, in
`bin-2/`; the copier the same:

| Binary | sha256 |
| --- | --- |
| `acq` (release) | `a37ae7d78a60c5e2cb0cbd413621f4bfda517968ecd3efd8af17903a572a2031` |
| `acq-mcp` (release) | `1e313e6c63436373df061ecf8a4aeaacda940960b2cc85bf6c578f64309fe47b` |

## The door's cut

Both doors print a result whole up to 60,000 bytes and past that its
first 3,000 characters, saying so, and mark the row `head` in
`calls.csv`. The cut is the door's, never the tool's: a fault that rests
on a `head` row is read against the whole output in the world's
`calls/` before it is the tool's.

## The runs

Each launched by `.claude/skills/seat/launch.sh` in its world, the
debrief the same session resumed. Counts are the door's (`calls.csv`);
a report's own counts are the runner's, never recomputed here. Cost is
the session's at list price.

| Run | Model | Session | Calls | Cut | Turns | Cost | Files |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cli-opus-1 | `claude-opus-5-5` | `b12c3dbb` | 34 | none | 46 + 2 | $2.27 | `calls-cli-opus-1.csv`, `REPORT-cli-opus-1.md`, `DEBRIEF-cli-opus-1.md` |
| mcp-opus-1 | `claude-opus-5-5` | `493d51e7` | 39 | call 9 (`tabs`, 745,672 bytes) | 57 + 2 | $3.60 | `calls-mcp-opus-1.csv`, `REPORT-mcp-opus-1.md`, `DEBRIEF-mcp-opus-1.md` |
| cli-sonnet-1 | `claude-sonnet-5-5` | `82f13d76` | 23 | none | 25 + 2 | $0.68 | `calls-cli-sonnet-1.csv`, `REPORT-cli-sonnet-1.md`, `DEBRIEF-cli-sonnet-1.md` |
| mcp-sonnet-0 | `claude-sonnet-5-5` | `c7a92eaa` | 12 | none | 20 | $0.43 | `calls-mcp-sonnet-0.csv`, `REPORT-mcp-sonnet-0.md` (stopped at T2) |
| mcp-sonnet-1 | `claude-sonnet-5-5` | `a2b95231` | 23 | none | 37 + 2 | $0.92 | `calls-mcp-sonnet-1.csv`, `REPORT-mcp-sonnet-1.md`, `DEBRIEF-mcp-sonnet-1.md` |
| cli-opus-2 | `claude-opus-5-5` | `05176cfc` | 49 | none | 61 + 2 | $3.13 | `calls-cli-opus-2.csv`, `REPORT-cli-opus-2.md`, `DEBRIEF-cli-opus-2.md` |
| mcp-opus-2 | `claude-opus-5-5` | `762398e2` | 43 | call 10 (`tabs`, 745,672 bytes) | 58 + 4 | $4.18 | `calls-mcp-opus-2.csv`, `REPORT-mcp-opus-2.md`, `DEBRIEF-mcp-opus-2.md` |
| cli-sonnet-2 | `claude-sonnet-5-5` | `d604b4cd` | 26 | none | 32 + 2 | $0.67 | `calls-cli-sonnet-2.csv`, `REPORT-cli-sonnet-2.md`, `DEBRIEF-cli-sonnet-2.md` |
| mcp-sonnet-2-0 | `claude-sonnet-5-5` | `62f33cf5` | 6 | none | 12 | $0.18 | `calls-mcp-sonnet-2-0.csv`, `REPORT-mcp-sonnet-2-0.md` (stopped at T1) |
| mcp-sonnet-2 | `claude-sonnet-5-5` | `447777cf` | 24 | call 6 (`tabs`, 745,672 bytes) | 42 + 3 | $0.97 | `calls-mcp-sonnet-2.csv`, `REPORT-mcp-sonnet-2.md`, `DEBRIEF-mcp-sonnet-2.md` |

cli-opus-1: one command denied by the rails (a shell line not starting
with the door), run again through it; one file read outside W — the
harness's own spill of calls 9 and 10's joined output, named in the
report's "Left out" and the debrief's answer 7.

mcp-opus-1: one door call denied by the rails — a heredoc whose JSON
body held escaped quotes and a `#`, refused by the harness's permission
check before the door ran — sent again inline (call 26). The harness
moved call 10's 31 KB result to a file the runner did not open, so call
10 went unread; the door's cut of call 9 is the one `head` row.

cli-sonnet-1: no denial; one file of its harness's (call 7's 39,716 bytes)
saved and not opened. Its world's door wrote `calls.csv` with LF line
ends; nothing else in the world differs from cli-opus's.

mcp-sonnet-0: stopped at T2 on the rails, not the tool. The harness
refused three `report` heredocs: it reads a heredoc's body as shell,
and a line with `?` before an unclosed apostrophe ("Right? … each
clause's matched count") fails its parse, bisected to that sentence in
a probe world. The runner stopped and asked, as the brief binds it. The
doors' `report` and `debrief` gained `--from NAME`, read from
`W/drafts/`, which the rails let the runner's file tool write, and both
briefs a sentence naming it; the three finished runs stand, the change
a way round a refusal they did not need. The run's world is
`mcp-sonnet-0/`; the run again is `mcp-sonnet`, fresh.

mcp-sonnet-1: no draft needed; the rails refused one chained command,
run again as its two parts; the harness clipped call 6, asked again as
call 7 with fewer rows.

cli-opus-2: one shell line denied by the rails, run again through the
door; two files of its harness's (calls 9 and 10, the two `--describe`
outputs) read outside W, named in its "Left out".

mcp-opus-2: one `call` heredoc refused by the harness, sent inline; the
harness let `date -u -r` through three times (a timestamp to a date),
which touched neither the tool nor its store — the rails deny what is
not allowed, and the harness counts `date` as safe.

cli-sonnet-2: no denial; before call 18 it ran the door once with a
wrong argument and a `2>/dev/null` redirect, which the brief forbids,
reached no tool and logged nothing — disclosed in its "Left out".

mcp-sonnet-2-0: stopped at T1 on the rails again: the harness refused
a `call` heredoc carrying the ring's URL, and the runner stopped and
asked, as its brief binds it; the Opus runners had sent such a call
inline. The draft path covered `report` and `debrief` alone. The MCP
door's `call` takes `--from NAME` too, and both briefs name every door
command's way round a refused heredoc (the CLI's `run --stdin` already
read a file); the run again is a fresh world.

mcp-sonnet-2: five report parts went in as drafts (`report --from`) when
the harness refused their heredocs; one door call denied, sent again.

## The faults, the first round

One row per fault, merged across runs; "met by" cites each run's own
fault number or debrief answer. Every one was reproduced against the
calls it cites and, where it is the tool's, in a scratch copy of the
pristine store on the release build.

| Fault | Met by | Fate |
| --- | --- | --- |
| `fidelity: exact` read as "the site would show the same items" | mcp-opus-1 1 (its worst); cli-sonnet-1 debrief 4 | `35b43898`: the `over` line, the MCP's `query` words |
| A clause with no stash meaning named by its key alone | cli-sonnet-1 3 | `35b43898`: each reported clause with its words |
| The clause notes cite the builder's records | all four: cli-opus-1 5, mcp-opus-1 5, cli-sonnet-1 debrief 5, mcp-sonnet-1 5 | `efa8be17`: `says` on every noted row, the rule a test |
| A count of N says nothing of the never fetched; the MCP never says what `never_fetched` counts | cli-sonnet-1 4; mcp-opus-1 4; mcp-sonnet-1 3, 5, debrief 5 | `fcb7e577` |
| A term's counts, `resolved` and `scope.items` read as the answer's | cli-opus-1 8; cli-sonnet-1 5; mcp-sonnet-1 1, 2 | `65779f1b` (by design, C93; the words) |
| `sorts by 78` read as the filter's 70 | cli-opus-1 2; cli-sonnet-1 1 | `f737a2bb` |
| An item's cell only in the body | cli-opus-1 3; cli-sonnet-1 2; mcp-sonnet-1 9 | `c60f607f` (the frontends park's position; derivation 12) |
| A unique tab's page printed blank | cli-opus-1 4; mcp-opus-1 3 | `a3ad3273` |
| `map_blighted` exact; Blight-ravaged maps outside it | cli-opus-1 1 (its worst); mcp-opus-1 T6 | `88a9cc61`: ambiguous (the owner's ruling); the next sitting |
| A long term path runs into its term | cli-opus-1 9 | `896a864b` |
| `price show` refuses a bare id | cli-opus-1 6 | `8cd57809`: both readings offered |
| `store status`: `leagues 0`, tabs of every realm | cli-opus-1 7; mcp-opus-1 4 (its sums) | `64bcafd4` |
| `match` beside `matched` in `against` | mcp-opus-1 8 | `580d1a72` |
| `tabs` 745,672 bytes, no limit; a count's requests; `--limit` in JSON | mcp-opus-1 2, 6; mcp-sonnet-1 4, 8 | the frontends park, fired, queued for the release (`80c1de6d`) |
| `--describe trade` 39,716 bytes | cli-opus-1 5; cli-sonnet-1 6 | `efa8be17`: 32,157 |
| The tour is a developer's README | cli-opus-1 11 | no fault of the tool: the release's user-facing README (note 36, step 4) |
| `--count tab` names no tab type | cli-opus-1 10 | no fault: `tab.type` is a count key (V10), `--cross tab,tab.type` |
| A gem-set `Level (gem)` read as the item's `reqlevel` | mcp-opus-1 7 | not shown (the runner: "possible, not shown"); a question, TRADE-SLICE's open observations |
| A zero offers explain, not "what is closest" | mcp-sonnet-1 6 | no fault: the offers ran as printed and found the term |
| Tab names are the owner's price notes | mcp-sonnet-1 7 | no fault: GGG's names, the owner's |
| The stat-value park's trigger | the seat's rarity URL, all four read its label | fired, queued (`7b231644`), the owner |
| `scourge-in-sum` | all four met the label at T5 | read as a caveat, as written; the sitting's |
| `blighted-base` | no seat (the site's form has no tiered Base) | the sitting's (`41d3ce82`) |

Not the tool's, the seat's own: the rails refused heredocs the harness
could not parse (mcp-opus-1 one call; mcp-sonnet-0 stopped at T2),
fixed in the doors (`e459bed7`); the launch script lost a relative
prompt (`5eea0ac6`); `setup --facts` (`875a6729`).

## The faults, the second round

The four reran on the round's release (`bin-2/`). What the round had
fixed: both Opus runs and the Sonnet MCP told the store's items from
the site's, named the Blight-ravaged maps and gave cells; the Sonnet
CLI still took the term counts for the total's ("had to infer why",
cli-sonnet-2 fault 2), the Sonnet MCP read the words as said ("stated
… easy to misread", mcp-sonnet-2 fault 5).

| Fault | Met by | Fate |
| --- | --- | --- |
| `typeline~"^Blighted "` matches non-maps | mcp-opus-2 6 | `a7bb473b`: `map_blighted=true`/`=false` rows, maps alone |
| A nameless page shown by a bare id, its tab apart | cli-opus-2 4; mcp-opus-2 4 | `1eecf3e0`: `page <id>`, the MCP's words |
| Whether the site can list an item | cli-opus-2 1; mcp-opus-2 1 (each its worst) | parked, `decisions/search.md` (`9d7ba727`) |
| The never-fetched cannot be listed (`--view locations`) | cli-sonnet-2 1; mcp-sonnet-2; mcp-opus-2 2 | parked, fired, queued for the release |
| An item's age on a row | cli-opus-2 2; mcp-sonnet-2 4 | parked |
| Sort by the filter's own value | cli-opus-2 5; cli-sonnet-2 3 | parked |
| Sizes: answers, `tabs`, tokens, `--describe` over a harness's limit | mcp-opus-2 2, 3; mcp-sonnet-2 1, 2, 3; cli-opus-2 8, 9; cli-sonnet-2 4 | the frontends park, fired, queued |
| A price reason citing `(C81)`; the help's ids | mcp-opus-2 1; cli-opus-2 7 | parked for the owner's ruling (`decisions/frontends.md`) |
| A map search not warned the map tab is mostly unfetched | cli-opus-2 3 | the coverage list's park |
| The direction an ambiguous clause may differ | mcp-sonnet-2 6 | no fault as stated: each `says` names where it may part from the site; the direction is the sitting's |
| `kind` as a count key refused with no near name | mcp-sonnet-2 7 | no fault: the refusal lists every key |
| A search refusal under text exits 1 | cli-opus-2 6 | no fault: the search's own refusal, not the parser's (C11) |
| `--sum stack` counts a single item as lacking | mcp-opus-2 5 | no fault: C93, documented in `--sum`'s help |

## Calls read

The runners' counts are theirs and never recomputed here. Read against
each report: cli-opus-1, calls 13, 21, 27, 28, 30, 31; mcp-opus-1,
calls 9, 10 (its row), 33, 37, 39; mcp-sonnet-1, call 19. Every other
fault that became a commit was reproduced in a scratch copy of the
pristine store on the release build, or found in the code; a parked
ask rests on the runner's calls as cited and was not reproduced one by
one. No further call file was opened. Every `calls.csv` was read whole
for its counts.

## The debriefs, routed

Each run's answer 5 (what to change first) and 7 (what we did not ask):

| Answer | Went to |
| --- | --- |
| Name what the trade answer leaves out, in plain words (cli-opus-1 5) | `efa8be17`, `35b43898` |
| The stale, partial store as the header's caveat; the sort and the filter one number (cli-sonnet-1 5) | `fcb7e577`, `f737a2bb` |
| Say what the site could show; reword fidelity (mcp-opus-1 5) | `35b43898`; the listing-state park |
| Every answer says what it could not see (mcp-sonnet-1 5) | `fcb7e577` |
| A site-visible field and row mark (cli-opus-2 5, mcp-opus-2 5) | the listing-state park, `decisions/search.md`; the price reason's words, the frontends citation park |
| The never-fetched listable (cli-sonnet-2 5) | the coverage-list park |
| A short answer (mcp-sonnet-2 5) | the frontends size park |
| "Which answer would you retract, or not bet on?" (mcp-opus-1 7, cli-opus-2 7, mcp-opus-2 7; cli-sonnet-1 7 and cli-sonnet-2 7 alike) | the seat skill's debrief |
| How fresh each item is (mcp-opus-1 7, mcp-sonnet-1 7) | the item-age park |
| The routes made counts cheap to check (cli-opus-1 7, cli-sonnet-1 7) | dropped: a strength, kept as it is |
| A size limit on `tabs` (mcp-sonnet-2 7) | the frontends size park |
