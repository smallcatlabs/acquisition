## Most surprising thing

The cold-review loop only adds. Each finding gets answered with more mechanism, and in 4 episodes across these 12 sessions the complexity came out only when someone outside the loop stepped back (the owner, a sidebar, or the orchestrator). In none of the 4 builder sessions did a builder propose removing anything itself.

## The twelve sessions

Activity shares are tool-call shares from a keyword classifier, so they are approximate. Edits are undercounted because they are often folded into combined commands. Forked transcript files duplicate earlier ones, so I used the largest file of each fork group.

| Prefix | Dates | Kind | Asst turns | Owner msgs |
|---|---|---|---|---|
| 9a3787d4 | 11-11 to 11-12 (idle to 11-20) | Orchestrator: re-triage, serial workers | 1380 | 23 |
| a11c2f5a | 11-17 to 11-19 | Orchestrator: game release 3.30 | 789 | 31 |
| a4d7711c | 11-23 to 11-25 | Orchestrator: form series steps 3 to 9 | 1768 | 46 |
| 06bd26e1 | 11-12, 3h | Builder: GUI step 6.4c | 687 | 0 |
| 0d117731 | 11-21, 1.7h | Builder: form step 2 | 563 | 0 |
| 8967aea7 | 11-23, 36m | Builder: form step 6, "Add a column" | 254 | 0 |
| d99afed5 | 11-24, 38m | Builder: form step 9, link to the trade site | 296 | 0 |
| a85ecac9 | 11-21 | Design brief (Fable), then watched its workers | 243 | 7 |
| 06672fb0 | 11-23, 14m | Brief writer (Opus) | 112 | 0 |
| 94888d72 | 11-10, 6m | Cold review of instruction changes | 57 | 0 |
| 42c6057d | 11-10, 1.7h | Recorded the owner's answers, then review rounds | 266 | 0 |
| a34d1a05 | 11-11 to 11-12, 20h | Went badly in part: serial worker; #246 reversed | 2139 | 1 |

**9a3787d4.** Asked to re-triage issues after a sitting. Shares: records 23%, task-list and wrap-up calls 21%, code reading 12%, waiting 9%, messaging workers 8%. Stalled 11-11: a worker's review finished and nothing woke anyone for about 1.5h, until the owner asked "Are the workers still working?" The owner reversed #246 at 16:19 and asked for "fable" to look for a deeper pattern at 21:50. Left: road, issue and ruling edits, plus 6 worker sessions.

**a11c2f5a.** Asked to handle the 3.30 release. Shares: task-list calls 19%, records 14%, waiting 13%, code reading 13%. Went wrong twice:
- A sitting's capture problem took about 20 owner turns to fix, by walking the browser step by step.
- The orchestrator added a "closed for good" state the owner never asked for, built across 2,021 lines.

The owner said on 11-18 "we are getting lost in a rabbit hole", and on 11-19 "I'm concerned we have overbuilt" (answer: "Yes, strip it"). Left: 11 workers, ledger rows, rulings.

**a4d7711c.** Asked to take steps 3 to 9 of the form series. Shares: code reading 20%, task-list calls 20%, records 16%, waiting 11%. It also built some itself (gates, #307). 34 of the owner's 46 messages are five words or fewer ("Start the build", "Yes, write it"). Left: 24 workers and, by its own count, 15 closures (the nine form steps plus fixes and gate work) in 148 commits, 59% of which touched only governance documents.

**06bd26e1.** Builder. Shares: code reading 50%, build/test 21%, edits 7%. It sent its plan and waited for the orchestrator's word. Three review rounds; the orchestrator changed the design mid-round. It ran `acq` in a mock world once, to measure a row's size in bytes.

**0d117731.** Wrote a brief, ran 2 brief reviews, built, ran 3 build reviews, closed, then did #304. Shares: code reading 34%, build/test 18%, edits 13%. Answered a review with tick boxes that the owner later replaced with "The simpler rule" (a net removal). Its one screenshot of the page drew no comment. 17 commits.

**8967aea7.** Builder. Shares: code reading 42%, build/test 23%, edits 7%. Never opened the page it built. It checked through headless browser tests and "Goldens read". Clean in 36 minutes.

**d99afed5.** Builder. Shares: code reading 47%, build/test 22%. Asked the design session one classification question. Ran `acq search --describe` twice, to measure kilobytes.

**a85ecac9.** The brief took 12 minutes. The owner's one new idea (export searches as trade-site URLs) became §7. After that the session mostly relayed worker turns ("woke on its own, no nudge needed"). Its closing note to the owner included: "I have not run the gate or opened the pages myself".

**06672fb0.** Brief writer. Shares: code reading 42%, records 14%, reviews 11%. The brief ended at 7,994 of 8,000 bytes, after "Over the bound by 31 bytes; trimming."

**94888d72.** Ran Codex twice, saved both answers, answered both findings. Six minutes.

**42c6057d.** Shares: records 24%, build/test 17% (breaker suites). It amended an unpushed close that claimed results it had never run, which is good practice.

**a34d1a05.** About 10 issues in 20 hours, every one steered by orchestrator messages. #246 (a main account) went through 3 cold reviews, was reversed by the owner, then backed out in 4 commits. #209 went 3 rounds and became #261. It broke its own fixes 16 times to watch the tests fail.

## The seven answers

**1. What a visitor would find strange.**
- About a fifth of an orchestrator's tool calls are the app's task-list and wrap-up cards: 205 of 1,000 in a4d7711c, 172 of 816 in 9a3787d4.
- Builders mostly cycle through commit, push, Codex review, the review saved as its own commit, answer, re-review. In 0d117731 about half of the 17 commits are the brief, saved reviews and the close.
- Edits go through Python string-replace heredocs with asserts: 4,501 of them across all transcripts, against 670 Edit calls.
- Orchestrators poll for `codex exec` to exit in case a worker fails to wake: 117 wait calls in a4d7711c.

**2. Knowledge re-derived.** Across the 100 session files in the date range:
- `decisions/process.md` was read in 96, the road in 88.
- `which codex` was checked in 50.
- `web/mod.rs` was re-read in 49, in 470 calls of `sed` ranges.

This happens even though each worker's prompt already carries about 4.5 KB of background. Things learned the hard way:
- Workers not waking after a background review (11-11). Afterwards orchestrators set fallback watches every time.
- `\b` does not work in `git grep -E`.
- Edits failing after `fmt` moved the text.
- `preview_stop` wanting a different id.
- CI runner losses diagnosed from scratch each time (11-11, 11-25).

**3. How a session decides it is done.** Done means a close commit with the gate at exit 0, the reviews clean or stopped at three for the owner, the road rewritten, the issue closed and CI green. Final reports are built for checking, with headings like "How it was checked", "Gate" and "Cold reviews". In practice they are written for the reviewer and the next agent; the owner gets the summary. Done is a procedural state. The owner's page sitting on 11-24, after the series was "done", produced 14 stops and a six-item fix round.

**4. Does anyone use the product while building it?** Agents may not run real mode. Three of the four builders ran the mock product, but mostly to measure bytes; the fourth never ran it. One builder took a single screenshot and said nothing about it. To learn whether a change works, a builder runs the gate, reads golden HTML diffs, runs headless browser assertions, and breaks the fix on purpose. Real use is done separately:
- by the owner, at sittings and in his own refreshes;
- by blind seats (two page seats in a4d7711c, which found 4 faults).

**5. Hand-offs.** 49 workers in the range were started with hand-written "Background" prompts: median 4,468 characters, maximum 11,365. A worker's report reaches the orchestrator truncated ("first 400 of 3369 characters"), so the orchestrator calls `list_events`. What gets lost:
- The owner's reasoning. In a34d1a05 the worker never met the owner; the #246 reversal arrived second-hand, after three review rounds.
- A worker's design departures. These surfaced only when the orchestrator happened to read the saved reviews (a85ecac9).

**6. Where the rules help, and where they serve themselves.**

Better:
- The three-review stop (C176) forced escalation, which turned #209's three failures into #261. That gave a job a single ending, held by the compiler.
- Breaking a fix to see its test fail is real verification.
- The honest-claims norm works (42c6057d).

Serving the rule:
- Byte caps drive trimming. Trims appeared in 10 of the 12 sessions; the road was cut to fit its 6,000-byte cap.
- A reviewer's input it called unlikely itself (a stat id with a line break or padding) got built anyway.
- The orchestrator relays wording fixes to unpushed closes.
- 565 saved review files exist. In the sample, another session re-read one only when the orchestrator checked it the same day. That is my judgment, and it is inflated by forks.

**7. The candidate blind spots, tested.**
- **(a) Verifying by reading and tests, not by using the tool:** seen in all 4 builders. Strong.
- **(f) Orchestration done by hand:** seen in all 3 orchestrators and every worker prompt. Strong.
- **(e) Work shaped by what a diff reviewer can check:** seen in about 3 sessions.
- **(d) Constrained prose style costs effort:** seen, but the cost is moderate (trims and docs-check failures in about 6 sessions).
- **(b) Long sessions drift:** half true. Workers do start fresh. Orchestrators run 36 hours to 9 days with up to 3 compactions. a11c2f5a drifted, and 9a3787d4 asked itself whether a fresh session "with no stake in my earlier calls" was meant.
- **(c) Records written for readers who never come:** plausible, but these transcripts cannot show reads months later.

Two to add:
- **(g) The review ratchet.** Four episodes: #246, #209, C190's closed-for-good state, and the step-2 tick boxes. In each, every finding was answered by adding, and the cure came from outside the loop. The memory rule about repeated findings is applied after the fact ("I missed the pattern at the second review", a11c2f5a). I would test this one first.
- **(h) The owner can see the work only by asking.** He asked at least 6 status questions ("is anything happening", "summarize what we've done", "nothing is moving?"). The orchestrator is the only view of the work's state.

**What twelve sessions cannot show:** the reviewer's own transcripts, reads of the records after this window, and the owner's experience outside chat. The classifier's shares are approximate.

Scripts (in the scratchpad, not for commit): `outline.py`, `list.py`, `shares.py`, `uses.py`, `grepall.py`, `rederive.py` and `firstedit.py`, in <scratchpad>/reader-h/
