# The trade import's seat — the record

Step 8 of `search/TRADE-PLAN.md`, under `PLAN.md` beside this file. The
close (the plan's step 10) adds the runs, their call counts and trees,
the fate of every fault and the debriefs' answers routed.

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
| cli-opus | `claude-opus-5-5` | `b12c3dbb` | 34 | none | 46 + 2 | $2.27 | `calls-cli-opus.csv`, `REPORT-cli-opus.md`, `DEBRIEF-cli-opus.md` |
| mcp-opus | `claude-opus-5-5` | `493d51e7` | 39 | call 9 (`tabs`, 745,672 bytes) | 57 + 2 | $3.60 | `calls-mcp-opus.csv`, `REPORT-mcp-opus.md`, `DEBRIEF-mcp-opus.md` |
| cli-sonnet | `claude-sonnet-5-5` | `82f13d76` | 23 | none | 25 + 2 | $0.68 | `calls-cli-sonnet.csv`, `REPORT-cli-sonnet.md`, `DEBRIEF-cli-sonnet.md` |
| mcp-sonnet-0 | `claude-sonnet-5-5` | `c7a92eaa` | 12 | none | 20 | $0.43 | `calls-mcp-sonnet-0.csv`, `REPORT-mcp-sonnet-0.md` (stopped at T2) |
| mcp-sonnet | `claude-sonnet-5-5` | `a2b95231` | 23 | none | 37 + 2 | $0.92 | `calls-mcp-sonnet.csv`, `REPORT-mcp-sonnet.md`, `DEBRIEF-mcp-sonnet.md` |

cli-opus: one command denied by the rails (a shell line not starting
with the door), run again through it; one file read outside W — the
harness's own spill of calls 9 and 10's joined output, named in the
report's "Left out" and the debrief's answer 7.

mcp-opus: one door call denied by the rails — a heredoc whose JSON
body held escaped quotes and a `#`, refused by the harness's permission
check before the door ran — sent again inline (call 26). The harness
moved call 10's 31 KB result to a file the runner did not open, so call
10 went unread; the door's cut of call 9 is the one `head` row.

cli-sonnet: no denial; one file of its harness's (call 7's 39,716 bytes)
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

mcp-sonnet: no draft needed; the rails refused one chained command,
run again as its two parts; the harness clipped call 6, asked again as
call 7 with fewer rows.
