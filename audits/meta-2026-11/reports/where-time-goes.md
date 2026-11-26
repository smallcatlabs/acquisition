## Verdict: BROKEN

**BROKEN.** Process weight does not set how much lands each week. The hours the machine runs do, and those hours follow the owner's presence.
**The deciding number:** product commits per active machine-hour stayed between 2.2 and 5.0 with no downward trend (W44 3.8, W45 3.6, W46 2.2, W47 5.0, W48 2.6). Over the same weeks active machine-hours fell from 78 to 41, 30 and 14, and are 17.5 for W48's first three days. Across weeks 42 to 48, product commits and active hours correlate at r = 0.96.
**Narrowing (my judgment):** process weight can only act inside the 51 active hours of the last two weeks. It cannot explain the 210 hours when nothing ran.

## Where a week's wall-clock goes

`wallclock.py` runs over `events_dedup.json`. Gaps are labelled per transcript, then a union per second is taken with priority model > tool > owner > other session > nothing. A gap under 5 minutes counts as agent work. A wait on the owner is capped at 8 hours. Times are local to the Mac.

| | Since 10-12 (1,080 h) | Last two weeks, 11-11 to 11-25 (360 h) |
| --- | --- | --- |
| (a) nothing running | 40.0% | **58.3%** |
| (b) agents working (model time) | 11.6% | 6.3% |
| (c) agents waiting on a tool | 12.9% | 7.9% |
| (d) sessions finished, waiting on the owner (nothing else running) | 33.8% | 24.5% |
| sessions waiting on another session | 1.7% | 3.0% |

In the last two weeks:
- **Days with no activity at all:** 11-13 to 11-16 and 11-22, 5 of 15 days. `git log` has no commits on those days either.
- **Daily averages:** 3.4 h active, 14.0 h with nothing running, and the owner present (sent a message) in 5.5 clock hours.
- **Concurrency:** peak 1.5 sessions and mean 0.77, averaged over all days including the dead ones.

**Which category to halve:**
- Halving (a), filled at the observed 2–5 commits per hour, roughly triples output.
- Halving (c), or process weight as a whole, can return at most part of 51 hours per fortnight.
- (d) is mostly long absences. Of the 90 h of reply gaps in the last two weeks, only 11 h are replies that came within 15 minutes.

### By week

From `weekly.py`. Product commits come from `git log -- '*/src/*.rs'`, which matches the brief's figures.

| ISO week | Active h (a+b) | Owner-wait h | Nothing h | Owner present, clock h/day | Product commits | Commits per active h |
| --- | --- | --- | --- | --- | --- | --- |
| 44 | 77.9 | 56 | 28 | 13.3 | 299 | 3.8 |
| 45 | 41.4 | 75 | 50 | 8.7 | 151 | 3.6 |
| 46 | 29.8 | 25 | 113 | 5.1 | 66 | 2.2 |
| 47 | 14.1 | 60 | 85 | 5.6 | 70 | 5.0 |
| 48 (3 days) | 17.5 | 21 | 33 | 9.7 | 45 | 2.6 |

The owner's present hours and active hours correlate at r = 0.91. Only 16% of agent-active time had no human message in the previous hour, and 19% in the last two weeks. Nearly all work is tended.

### Waiting on the owner

From `wallclock.py` and `turns.py`.
- **Finished turn to his next message:** 1,668 cases. Median 3.0 min, p75 8.9 min, p90 26 min; 89 gaps over 1 h.
- **Where the hours sit:** of 622 session-hours before a human message (summed, gaps up to 8 h), 438 h fall in just 116 gaps over 1 h.
- **How human messages were told apart:** by the transcript's `origin.kind == "human"`. Tool results, system reminders, task notifications, peer and coordinator prompts, side-session and "Background (from …" injections were classed separately.

### Inside-tool time

Hours summed over transcripts after removing copied events, with call counts.

| Kind | All time | Last two weeks |
| --- | --- | --- |
| nextest / test | 32.8 h (4,262) | 6.1 h |
| codex, background and foreground | 22.7 h | **7.5 h, the largest** |
| gh, including background `gh run watch` | 22.3 h | 5.9 h |
| full gate, including background | 16.9 h | 6.6 h |
| background subagents | 9.2 h | — |
| sleep / poll | 6.6 h | 1.1 h |
| docs, arch and trade checks; gate `--docs` | 8.0 h | 1.6 h |
| cargo build | 4.0 h | 0.9 h |

Tool wait against model time, as union hours: 139 to 125 overall and 28 to 23 in the last two weeks. That is about 53% tool wait inside active time.

### Stalls and loops

From `stalls.py` and `gatechains.py`.
- **The same command 3 or more times in a row:** 40 runs in all, mostly python. Worst cases:
  - `9c831ac2`, 11-01: a `gh` command 7 times over 50 minutes.
  - `399553f6`, 11-01: a docs/registry check 6 times.
  - `aec27f3b`, 11-03: a shell read 4 times over 30 minutes.
- **Red gate runs:** 31 of 229 full runs (`runs/gate/verdicts.tsv`). Red streaks before a green were 26 of length 1, one of 2 and one of 3, the longest on 11-11. There are no gate loops.
- **Sleep/poll:** no run of 5 or more consecutive calls.
- **Sessions over 300 assistant events:** 94, and 20 in the last two weeks. An event here is one content block, so this overstates turns. The largest are `a34d1a05` (11-10, 2,139), `d7b94266` (11-04, 1,923) and `a4d7711c` (11-23, 1,768).
- **Sessions with no successful `git commit`:** 75 of 256, including their subagents. Many are conversations, so this is not a stall count.

Stalls and loops are not where the time goes.

### Lead time per step

From `leadtime.py`: 17 of the 18 most recent closes; one with an unparseable slug was dropped.
- **Lead, first related commit to close:** median 0.9 h, range 0 to 315 h.
- **Commits per step:** median 8, range 1 to 20; 54% of related commits are brief, review or close commits.
- **Inside the lead:** the machine was active for a median 0.7 h.
- **The long leads are idle days, not work:**
  - The three form steps closed 11-23 had leads of 45 to 48 h, with 1.1 to 2.6 active hours inside them.
  - form-editor: 315 h lead, 33 active h, probably an over-early match.

Once the machine runs, a step takes about an hour.

## What nobody named

1. **The machine runs one session at a time.**
   - Mean concurrency over active minutes was 1.01 to 1.17 in every week.
   - Only 11 to 16% of active minutes had two or more top-level sessions, and the peak was 2 to 4 (`conc.py`).
   - The parallelism the method was built for is not being used, even when he is present.
2. **Most of his messages buy almost nothing; a few buy nearly everything.**
   - 64% of human messages are followed by under 5 minutes of autonomous work (median 1.5 min).
   - 13% start runs over 30 minutes, and those runs carry 235 of the 320 run-hours (73%) (`turns.py`).
   - Judgment: the throughput lever is how many long runs are started before he leaves, not the ceremony inside a run.
3. **Sessions sit finished while he is away.**
   - Spot read of session `8086ea79`: a finished turn at 05:13 UTC waited until 12:34, then came 1-to-8-minute exchanges, then a 4-hour gap.
   - There is a ready queue (112 of 256 sessions were opened by another session, 25 of 34 in W48), but nothing runs it unattended.
4. **Earlier transcript counts are inflated about 15–20%.**
   - Forked and resumed transcripts copy their parent's events: 45,857 of 221,593 events are duplicates (`dedup.py`).
   - `trace.py`'s 4,869 nextest calls become 4,262 after removing them. The cost and turn figures in `traces.md` carry the same inflation.
5. **In the last two weeks codex is the single largest tool wait (7.5 h).** It is mostly run in the background, which is when sessions end turns and wait.

## Checks and limits

- **Second routes for the top three numbers:**
  - Active hours: counting distinct minutes that hold any raw transcript line gives W44 60 h, W46 21 h and W47 13.3 h, against 78, 30 and 14. Same shape. This route is a lower bound, since it misses minutes between events.
  - nextest wall time before removing copies: 36.3 h from my script against 36.6 h from `trace.py`, written independently.
  - Coverage: 2,531 of 2,550 commits since 10-12 have a local transcript event within 15 minutes before them (`coverage.py`). The 19 without are 15 on 11-19 and 4 earlier.
- **The owner-wait gaps were read in the raw `.jsonl`** for sessions `3428cb60`, `69f1c7ac` and `8086ea79`.

**What I could not measure:**
- Cloud sessions and the VM guests are not in this directory, though the commit coverage suggests little is missing.
- Permission-prompt waits cannot be seen; they would show up as tool time.
- "Asking" versus "finished" turns cannot be told apart without reading text, which I did not keep.
- How much active time is ceremony (briefs, reviews, closes) as opposed to product work was not split.
- Lead-time matching uses slugs and issue numbers in commit subjects, which is heuristic.

## Scripts

All under `<scratchpad>/reader-a/`:
- `extract.py`
- `dedup.py`
- `wallclock.py`
- `weekly.py`
- `coverage.py`
- `conc.py`
- `turns.py`
- `stalls.py`
- `gatechains.py`
- `leadtime.py`
- `spot.py`

Outputs: `wc_all.txt` and `wc_2wk.txt` in the same folder. All read-only; no repo writes.
