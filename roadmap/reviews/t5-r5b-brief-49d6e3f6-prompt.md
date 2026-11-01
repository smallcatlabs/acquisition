You are the outside reviewer of a BUILD BRIEF, not a code change, in the Acquisition Rust workspace: roadmap/briefs/t5-r5b.md as cut a third time at 49d6e3f6, round 5b of t5 (#141). Read it with `git show 49d6e3f6` and the file as it stands. This is a short confirming review: the second cold review (of 297a6566, saved verbatim at roadmap/reviews/t5-r5b-brief-297a6566.md with its prompt beside; the first review beside them) found eleven of twelve findings answered and raised four: the observer read must come after `DoorBuilder::run` returns (finish aborts the dispatcher without awaiting it), the observer functions are `Persisted::read_job` and `Persisted::read`, a sleep submit waits on the restore when an account is named (and 986/1127's apply admissions cannot run under a held restore), and the Goal's id check is the mock rows'. This cut answers all four, and the brief's foot says how. Read; do not build or run anything; the tree is read-only to you.

Your lens:
1. Are the four findings answered in substance? For the first: does `DoorBuilder::run` returning end the daemon (door.rs 452–520: the runtime built inside `run` and dropped at its end; the hook's `Arc<Daemon>` dropped with it; the dispatcher's and jobs' tasks cancelled by the runtime's drop), so that `Persisted::read_job(dir, id)` after `run` is C45's observer read; and is "a directory the test owns outside the closure" buildable (the test's `TestPath`/tempdir created before `Door::open`, the `JobDb` opened at `daemon_db_path(dir)` and passed to `.jobs_db`, the directory read after `run`).
2. Anything this cut introduced that contradicts another section, or a citation that does not match the tree.

Report in this form, nothing else:
- One line: the brief, and whether it is buildable as drawn.
- The four findings: one line each, answered / narrower (sound or not) / not answered.
- Findings, most severe first, each: the Do number, what is wrong in one sentence, the concrete reading (file:line), and the smallest change sufficient.
- What you did not check and why.
- Which findings you would call nitpicks.
- Whether round 5b may start as drawn, in one line.
