You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/4-0-life.md as cut a fourth time at 10472714, the second build step of phase 4-0, the time seam. Read it with `git show 10472714` and the file as it stands. This is a short confirming review: the third cold review (of 0d3c16dc, saved verbatim at roadmap/reviews/4-0-life-brief-0d3c16dc.md with its prompt beside; the two earlier reviews beside them) found the second review's four findings answered and raised one: the signal watcher calling `begin_exit` whole would take the rails' lock (rails.rs:551) and the log's (daemon.rs:1161) on the thread that must stay free for the second signal (start.rs:230). This cut answers it: the watcher swaps and nothing more, as today (start.rs:221), and the loop's first act on `Signal { won: true }` is `StopAdmitting(why)`, as `exit_armed` is today one runtime turn after the swap (server.rs:540–544); the stop's and idle's ingresses, on the main runtime, keep `begin_exit` whole as the admission fence (server.rs:550–567; exit.rs:223–240). Read; do not build or run anything; the tree is read-only to you. Every line the brief cites was printed from the tree at 10472714.

Your lens:
1. Is the finding answered in substance? Does the cut reproduce today's two paths exactly (the stop's synchronous fence; the signal's swap-then-spawned stop_admitting), and is the gap it names for the signal truly no wider than today's (start.rs:221–224 → exit_armed)?
2. Does the asymmetry (StopAdmitting as a machine effect for the signal's winner alone) contradict Do 1's inventory, Do 2's rows, or the done-when lines anywhere else in the brief? Name any sentence left standing that still says otherwise.
3. Any citation in the changed text that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The finding: answered / narrower (sound or not) / not answered, one line.
- Findings, most severe first, each: the Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether step 2 may start as drawn, in one line.
