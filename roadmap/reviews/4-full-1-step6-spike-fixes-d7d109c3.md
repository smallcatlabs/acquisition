`d7d109c3` — The fix-round done-when does not hold: browser discovery remains unbounded, and the requested regression pins are absent.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:1115`, shapes 2 and 3.** The deadline fix leaves browser discovery’s `urlopen()` and `json.load()` without a deadline, so the runner can still hang indefinitely. Concrete browser input: Chrome accepts `/json/list` but stalls before completing its response; execution never reaches the bounded debugging calls or failure recording. **Smallest fix:** bound discovery’s connection and complete response read by an absolute deadline, and report expiration as failure.

- **P3 — `audits/web-2026-11/spike/rig-1.patch:57`, `:225`, `:1004`.** All three fixes lack the executable regression pins explicitly requested by the prior review; their shared gap is that the commit retains descriptions of probes rather than checks. Concrete cases: a translated visible block, a debugging response arriving slowly in chunks, and a window pending at the result snapshot have no retained assertions protecting their corrected outcomes. **Smallest fix:** preserve repeatable fault checks alongside the rig that assert invalidation, deadline expiration and pending-request retention respectively.

Execution-dependent checks—including the gate, patch application, fault injections and measurements—are **not verified**, as instructed.

I checked the diff and affected callers statically. No goldens, references or retirement lines changed. The later judged runs and owner sitting are outside this fix round.

I would call the missing regression pins a process nitpick relative to the functional fixes; the unbounded wait is not a nitpick.

Brief, plan and prompt: nothing obstructed this review; no change suggested.
