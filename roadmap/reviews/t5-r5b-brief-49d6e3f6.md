[roadmap/briefs/t5-r5b.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/t5-r5b.md) at `49d6e3f6`, unchanged in the working tree, is **buildable as drawn**.

1. **Answered.** `run` owns and drops its runtime; it removes the hook after the scenario, and runtime teardown cancels the dispatcher and job tasks (`door.rs:452–477`, `dispatch.rs:55`). A test-owned `TestPath::dir()` can survive `run`, with `JobDb::open(daemon_db_path(dir))` passed through `.jobs_db`; the subsequent observer read satisfies C45 (`harness.rs:714–721`, `door.rs:362`, `jobs.rs:113`).
2. **Answered.** `Persisted::read_job` and `Persisted::read` exist at the cited locations (`jobs.rs:450,513`).
3. **Answered.** Accountless sleep bypasses the restore wait; apply does not (`server.rs:273–281`, `daemon.rs:1668–1672`). Keeping 986/1127’s admissions together as a component test is sound.
4. **Answered.** The Goal now limits the id-1 check to mock rows, matching Do 2 (`t5-r5b.md:23,270–275`).

Findings, most severe first—both nonblocking:

- **Do 1/4/5, review-summary wording:** the first-review recap still describes observation “after `finish`,” leaving the superseded boundary in the text (`roadmap/briefs/t5-r5b.md:565`). Smallest change: replace it with “after `run` returns.”
- **Do 4 / done-when 1:** “986/1127’s admissions if kept” retains an obsolete choice after Do 4 mandates keeping them together (`roadmap/briefs/t5-r5b.md:356–363,478–479`); the Report also requests their “decision” at line 535. Smallest change: remove “if kept” and request the retained component test’s name.

What I did not check: compilation, execution, gates, CI, timings, or future implementation correctness, because this was a read-only confirming brief review.

Nitpicks: both findings above; neither undermines the four substantive corrections.

**Round 5b may start as drawn.**
