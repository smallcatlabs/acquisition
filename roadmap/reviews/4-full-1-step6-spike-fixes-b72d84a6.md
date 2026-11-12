`b72d84a6` — The fix-round done-when does not hold: the position, deadline and per-window evidence fixes remain incomplete.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:58`, shape 2.** The measure uses layout offsets instead of rendered positions, so a visibly misplaced row can still escape invalidation. Concrete rendered-DOM fault: shift a visible block with `transform: translateY(24px)`; its rows’ `offsetTop` values remain unchanged, so they count as correctly placed. **Smallest fix:** use row bounding rectangles for visibility and displacement, including rows overflowing an otherwise offscreen segment, and pin this fault injection.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:972`, shapes 1 and 2.** The socket timeout bounds individual operations rather than a debugging call or run, leaving the promised deadlines unenforced. Concrete browser/CDP input: a response arriving in chunks less than 30 seconds apart keeps `_exact()` waiting beyond 30 seconds; the outer run deadline cannot interrupt it. Navigation also precedes the run timer. **Smallest fix:** propagate absolute monotonic call and run deadlines through every send/read, using their remaining time, and pin a partial-response stall.

- **P2 — `audits/web-2026-11/spike/rig-1.patch:230`, shape 1.** The per-window records still omit requests unfinished when `windows()` snapshots completed resource timings. Concrete loopback-response case: the final jump’s visible rows finish promptly, but a surrounding block remains pending beyond the 500 ms hold; that request disappears from the result despite the README promising every window asked. **Smallest fix:** record requests when issued, retain pending/aborted status, and attach completed timings when available; pin a response delayed past the snapshot.

Execution-dependent checks—builds, tests, gate, patch application, fault injections and measurements—are **not verified**, per the read-only instruction.

I would call none of these nitpicks: they affect the rig’s verdict, termination or promised evidence.

Brief: no change needed; the later judged runs and owner sitting are outside this fix round. Plan and prompt: nothing obstructed this review.
