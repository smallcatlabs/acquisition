## Verdict: BROKEN

Removing the whole 102 KB governance reading path from every call would cut about **6%** of cost ($5,425 → $5,122). Most of the cost sits elsewhere: **68% is spent in calls made after their transcript had already passed 200k tokens of context.**

Your suspicion is also only half right. Contexts do not get big because sessions run very long. They get big because every session starts heavy (about 77k tokens, most of it the harness's tool schemas rather than the project's documents) and then adds about 2k tokens per call. A median session passes 200k at about call 48.

## A correction to `traces.md` first

The earlier trace counted transcript *events*, not API calls. One API call is written as two or three assistant events that share a message id and carry identical usage. On top of that, resumed or forked sessions copy earlier calls into their own file: 7,743 message ids appear in more than one file.

- **Unique API calls:** 56,213, not 137,560.
  - Opus: 46,025 (traces.md says 102,539).
  - Fable: 9,921. Sonnet: 241. Haiku: 26.
- **Context read:** 13.3 B tokens, not about 31.8 B.
- **The 217k average still stands**, because the numerator and the denominator were inflated by the same factor. My recount: Opus mean 220k, Fable mean 316k.

**Check by a second route.** I priced the deduplicated calls at per-token prices fitted from the sessions' own cost records (`prices.py`; Opus at $5 in, $25 out, $0.20 cache read, $5 cache write per million tokens). That gives **$5,425**, against the **$5,316** the sessions themselves recorded. The undeduplicated count prices to $7,001. A plain grep of all assistant lines gives 138,958 events and 56,233 unique message ids, which agrees.

## Cost per call by context size

Cost splits into cache reads 51%, cache writes 23% and output 25%. Re-reading context is half the bill, not all of it.

| Context per call | Share of calls | Share of cost | Share of context tokens |
|---|---|---|---|
| under 50k | 1.3% | 1.0% | 0.2% |
| 50–100k | 13.1% | 8.8% | 4.4% |
| 100–200k | 37.0% | 27.4% | 23.1% |
| 200–400k | 35.4% | 38.8% | 41.9% |
| over 400k | 13.1% | 23.9% | 30.4% |

- **Calls at or above 200k:** 62.7% of cost.
  - Second route, counting raw cache-read tokens instead of priced cost: 72.7%.
- **Calls after their transcript first passed 200k:** 31,316 calls (55.7%), 68.4% of cost, across 301 transcripts.
- **Compactions:** only 105, in 73 of 752 transcripts, all automatic. The median one fired at 351k tokens; the largest at 972k. With a 1M window, sessions simply keep growing.

## What fills the context

`analyse2.py` weights each item by its size times the number of later calls that re-read it, up to the next compaction. It explains about 63% of the tokens actually read; the rest is mainly thinking, which the transcripts do not store. Treat the rows as shares, not totals.

| Kind of content | Share |
|---|---|
| Shell reads (cat, grep, sed, find) | 26.8% |
| **Fixed prefix: system prompt plus tool schemas** | **23.8%** |
| The model's own tool-call inputs (commands, edit bodies) | 13.9% |
| git log, show and diff output | 4.5% |
| Read tool results | 4.5% |
| **Instruction files (AGENTS.md, CLAUDE.md, MEMORY.md)** | **4.5%** |
| **Skill listing** | **3.6%** |
| Human and meta text | 3.3% |
| gh, checks and bothy, cargo test, the gate, codex, acq | about 5% together |

A session's first call carries a median of **77.6k tokens**; a subagent's carries 38.8k. Mean bytes in that first call:

- **Tool schemas:** 123 KB. The Artifact tool alone is 37 KB; 51 tools in total.
- **System prompt:** 33 KB.
- **Instruction files:** 23 KB.
- **Skill listing:** 19 KB.
- **Everything else:** under 15 KB.

**The project's own documents are about 20% of the starting context; the harness is about 75%.** Governance files read later through the shell are part of the 26.8% shell-read row; `traces.md` puts about 38% of shell output in that area.

## Session shapes

There are 253 top-level sessions, with a median of 90 calls (p90 344, max 1,028). Subagents are 39% of calls and 28% of cost.

| Calls per session | Sessions | Share of cost |
|---|---|---|
| under 50 | 79 | 2.1% |
| 50–200 | 122 | 25.6% |
| 200–500 | 41 | 27.5% |
| over 500 | 11 | **16.6%** |

The ten largest sessions are $79 to $171 each, with mean context of 256k to 528k. Roles are my guess from keywords in each session's first prompt:

- **Builders:** 3 (late October, early November, 23–25 November; this last one is the largest at $171).
- **Seats:** 2 (4–5 November).
- **Audit or research:** 2.
- **Design:** 1.
- **Orchestrators:** 1 by keyword, Opus, 1,028 calls; plus 1 unclassified session (22–24 October) that ran 116 subagents.

**Subagents.** There are 396 subagent transcripts, 357 of them on Opus, with mean context 169k. They are not used only for reads:

- 63 made commits.
- 144 edited or built.
- 189 only read.

Subagents also pass 200k, at a median of call 51. Delegation does not by itself keep contexts small.

## Mechanical work on large models

These are calls whose only tool call was a gate run, a commit or push, a golden regeneration, a sleep or poll, Monitor, a review-file write, a `gh` command, or a check or bothy run:

- **7,643 calls**, 13.6% of calls and 14.6% of cost.
- **Mean context 273k**, higher than the average call. This work happens late in sessions.
- 7,613 of them ran on Opus or Fable.
- **Narrower set** (leaving out `gh` and the checks): 7.7% of calls, 8.2% of cost.

## Cost per landed product commit

A product commit is one touching `crates/acquisition-*/src/`, from `git log`.

| ISO week | Product commits | Cost | Per commit | Calls per commit |
|---|---|---|---|---|
| W42 | 91 | $531 | $5.8 | 61 |
| W43 | 155 | $1,024 | $6.6 | 74 |
| W44 | 293 | $1,441 | $4.9 | 49 |
| W45 | 161 | $900 | $5.6 | 57 |
| W46 | 66 | $595 | $9.0 | 86 |
| W47 | 70 | $411 | $5.9 | 62 |
| W48 (partial; the review week) | 45 | $523 | $11.6 | 124 |

Mean context per call stayed flat at 203k to 277k throughout. When cost per commit rises, it is because there are more calls per commit, not bigger contexts.

## Counterfactuals: arithmetic on recorded calls, not measurement

| Change | Cost | Change |
|---|---|---|
| Fresh session at 200k (prefix plus a 20k handoff), prefix re-written into the cache | $4,878 | −10% |
| Same, at 150k | $5,143 | −5% |
| Same, at 150k, prefix read from a shared cache and only the handoff written | $4,140 | **−24%** |
| Same, at 100k, prefix read from a shared cache | $6,366 | +17% |
| Mechanical calls on Sonnet at 30k context | $4,749 | −12% |
| Fixed prefix halved | $5,051 | −7% |
| 26k tokens (102 KB) of documents gone from every call | $5,122 | −6% |

**The cap moves the total most, but only if a restart is cheap.** With a 77k-token start, restarting often enough costs more than it saves. The heavy harness prefix and the long sessions are one problem, not two.

## Usage limits

- **No harness usage-limit events.** The 16 API errors are 529 overloads and connection failures.
- **The owner wrote about Opus or usage limits** four times on 22 October, the heaviest day to that point (4,605 calls). Activity then dropped to 757 and 215 calls, with a 25-hour idle gap.
- **A harness notice saying a limit was reached** appears on 27 October, the largest day of all (7,952 events, about $835 before the dedup fix).
- **Further owner mentions** on 4 and 5 November, followed by 29 hours idle.
- **Judgment:** limits probably bound the work in W43–W45. The 113-hour gap from 12 to 17 November carries no limit signal.

## What nobody named

1. **The harness costs more context than the governance.** Tool schemas average 123 KB in every call of every session, against 23 KB of instruction files. The Artifact schema alone (37 KB, about 10k tokens) costs about as much per call as AGENTS.md and MEMORY.md together. Trimming the tools loaded per session (no Artifact, iOS simulator, browser or Workflow schemas for a builder) is a cheaper lever than editing documents.
2. **Output is a quarter of the cost.** That includes Fable's thinking, at about $50 per million tokens on the fitted price. No document trim touches it.
3. **Cache writes are 23% of cost**, but almost all of that is new content entering the cache once per call (a 2k mean). Re-caching after idle gaps of 5 minutes or more costs only about $220, so idle waiting is not the leak.
4. **The real unit of cost is calls per landed change**: 49 in W44 against 86 to 124 in W46 and W48. A step that lands in fewer calls is the lever no context trim reaches. Gates, checks, commits and `gh` together are 7,643 of those calls.
5. **`traces.md` overstates call counts and tokens about 2.4 times** (events versus calls), and should be corrected before anyone cites its turn counts.

## Not measured

- **Thinking content:** not stored, so occupancy explains about 63% of the tokens read.
- **Byte-to-token ratio:** assumed at 3.6 bytes per token.
- **Long-context price tiers:** whether any apply.
- **Whether the system and tool prefix is shared in the cache across sessions.** This decides between the −5% and −24% cap results.
- **Sessions not in this directory:** cloud sessions and the VM guests.
- **Occupancy shares** come from per-file totals before the cross-file dedup, so copied history counts twice in the shares (not in the totals).
- **Session roles** are keyword guesses from the first prompt.

## Scripts

Run each with `python3 -I`:

- `<scratchpad>/reader-b/prices.py`: price fit from the cost records.
- `<scratchpad>/reader-b/extract.py`: one pass that writes `calls.jsonl` and `files.json`.
- `<scratchpad>/reader-b/analyse.py`: buckets, shapes, subagents, mechanical calls, counterfactuals, weeks.
- `<scratchpad>/reader-b/analyse2.py`: what fills the context, the prefix, errors, the corrected cap.
- `<scratchpad>/reader-b/analyse3.py`: first call, cache writes, days, check against the cost records.
- `<scratchpad>/reader-b/growth.py`: when contexts pass 200k, growth per call, second routes.
- `<scratchpad>/reader-b/cap2.py`: cap with the prefix read from a shared cache.
- `<scratchpad>/reader-b/limits.py` and `<scratchpad>/reader-b/limits2.py`: limit signals and idle gaps.

The commit table came from an inline `git log -- ':(glob)crates/acquisition-*/src/**'` joined with `weeks.json`.
