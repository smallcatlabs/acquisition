`roadmap/briefs/t5-r3.md` at `6e421f2c` matches the working file and is buildable as drawn.

1. **Answered.** Do 1–2 restrict body fragments to job errors and halt words, and `challenge` / `Retry-After` to halt words (`roadmap/briefs/t5-r3.md:165–172,194–195`). This preserves the old exclusions (`dispatch.rs:821–838`), with persisted-file redaction retained by Do 4 (`t5-r3.md:226–228`). The journal may retain `cf-mitigated: challenge`, as the cited fixture, snapshot and journal code require (`harness.rs:493`, `policy.rs:427–444`, `rails.rs:839–840`). No exclusion is lost or applied to a surface that intentionally keeps the fragment.
2. **Answered.** Report requests “the step enum’s shape” (`roadmap/briefs/t5-r3.md:293–295`), consistent with Do 1’s step model.

Findings: none. This cut introduces no contradiction or mismatched citation found within the requested scope.

What I did not check: builds, tests, daemon execution, CI, executed twin wording, gate exits or timings; this was the requested read-only confirming review.

Nitpicks: none remaining; the former stale-column reference was a nitpick and is corrected.

Round 3 may start as drawn.
