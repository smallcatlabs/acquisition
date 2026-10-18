# The re-sit — the record

Step 6 of `brainstorming-notes/38-the-answer-an-agent-reads.md`, run
under `PLAN.md` in this directory (deleted at the close, cited at its
last commit); the briefs `BRIEF-cli.md`, `BRIEF-mcp.md`, `TASKS.md` and
`DEBRIEF-QUESTIONS.md` beside it, at `35e7390f` after review 1. The
trade seat's record is `../seat/README.md`; this seat asks the same
person's questions again after note 38's steps 1 to 5, with the
person's question split in two and one task that turns on a
never-fetched tab, and counts what the plan says it owes.

## The worlds

Made 2026-10-18 at `35e7390f`, outside the repository under
`~/seat-worlds/trade-2026-10/re-sit/`. The store is the trade seat's
pristine copy of the owner's `GERWARIC` facts and intent files; the
copy itself had been removed at the trade seat's close (`teardown`
removes a world's store, the pristine's included), and was made again
2026-10-18 the same way (`setup --facts`, no daemon running, the
owner's files unchanged since 2026-10-15), its two files hashing the
same as the copy note 38's measurements were taken on
(`5f35c33f…`, `4165aa04…`). Every run's world is `setup --store` from
it and holds its brief (as `BRIEF.md`), `TASKS.md`, the tour (CLI:
README's "The tour", as `TOUR.md`, byte-identical to the trade seat's
second-round copy), its door, its binary under `bin/` and the store.

| Binary | sha256 |
| --- | --- |
| `acq` (release, `35e7390f`) | `45acc603ab787d14c54f84c8c8d021cd508663e318880d37df6bf8518a724d86` |
| `acq-mcp` (release, `35e7390f`) | `c3356772fa8a6ce8bc66292f527eb8e1463a3d4b3acf31bd72562ed0cee5ee55` |
| `copy-as-store` (release; made `pristine/`) | `6878430c6d35a5fc290f0f16b34a747099b5017a820a8a97ad57d2ebe3ed8012` |

A runner's startup context is the owner's global `CLAUDE.md` alone
(the launcher's header): at every launch of this seat its sha256 is
`e9c1329f56f2869c47aeed06cc7a8d08addea5bc95570ccb235cf863be013d0c`,
and it names nothing of Acquisition (review 1 read it).

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
| cli-opus | `claude-opus-5-5` | `393fea5a` | 44 | none | 50 + 4 | $2.65 | `calls-cli-opus.csv`, `REPORT-cli-opus.md`, `DEBRIEF-cli-opus.md` |
| mcp-opus | `claude-opus-5-5` | `20886c82` | 38 | none | 58 + 4 | $3.27 | `calls-mcp-opus.csv`, `REPORT-mcp-opus.md`, `DEBRIEF-mcp-opus.md` |

cli-opus: no denial; its harness spilled calls 9 and 10's joined output
to a file outside W, which the runner did not open — call 9 was run
again as call 11, and call 10 (`--describe trade`, 33,543 bytes) went
unread, named in its "Left out" and answer 1. No draft, no work file.

mcp-opus: two door commands denied by the rails — a `call … -` heredoc
and a shell loop over four explain paths — sent again as five drafts
(`call --from`) and single calls; the harness cut call 14's printout
(35,134 bytes, printed whole by the door) to about 2 KB, so the 69
rings the dexterity edit adds were counted, not listed, named in its
"Left out". No file opened outside W.
