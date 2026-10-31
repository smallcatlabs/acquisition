You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/4-0-life.md as cut a fifth time at dfaa2b3c, the second build step of phase 4-0, the time seam. Read it with `git show dfaa2b3c` and the file as it stands. This is a short confirming review: the fourth cold review (of 10472714, saved verbatim at roadmap/reviews/4-0-life-brief-10472714.md with its prompt beside; the three earlier reviews beside them) found the third review's finding answered and raised one: a loop that performs `Judge` inline waits on `shared` (server.rs:674), so a signal's admission stop could wait behind a held lock where today `exit_armed` on another worker does not. This cut answers it: `Judge` runs on a task of its own and answers as an event, the loop waiting on no lock (Do 3, the new bullet), pinned by a new in-process test that holds `shared`, hands the loop a signal's winner event through the driver's channel and sees the rails stop before releasing the lock (Do 4). Read; do not build or run anything; the tree is read-only to you. Every line the brief cites was printed from the tree at dfaa2b3c.

Your lens:
1. Is the finding answered in substance? Is "no other effect the loop performs takes `shared`" true against the tree (stop_admitting at server.rs:563–567 and rails.rs:550–552; InFlight::of through try_lock at server.rs:584; Exit::arm at daemon.rs:229–250; leave at server.rs:616–662)? Can the new test be built as drawn (the driver's channel for events, a judge blocked on a held std Mutex while the loop's select stays live under a multi-thread test runtime)?
2. Anything in this cut that contradicts Do 1's inventory, Do 2's rows or the done-when lines; any sentence left standing that still says otherwise; any citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The finding: answered / narrower (sound or not) / not answered, one line.
- Findings, most severe first, each: the Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether step 2 may start as drawn, in one line.
